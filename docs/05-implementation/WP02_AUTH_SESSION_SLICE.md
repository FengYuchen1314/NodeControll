# WP-02 密码登录与服务端会话纵切

## 1. 结论与边界

本纵切把首次初始化后的 Owner 凭据接成了可运行的最小登录闭环：浏览器通过同源密码登录取得两个 `__Host-` Cookie，Master 只在数据库保存带用途和版本的 HMAC；刷新页面后，前端用 `/me` 从服务端恢复身份；写请求同时经过 Origin、Host、CSRF cookie/header 和数据库 CSRF HMAC 校验；退出登录在一笔数据库事务中撤销当前 session、写安全事件并清除浏览器 Cookie。

登录不是无界的 Argon2 工作队列。初始化状态确认后，请求必须先取得进程内登录/Argon2 并发许可；许可耗尽立即返回 429，而且不会读取或写入 limiter bucket。许可从 limiter 之前一直持有到密码验证结束，因此并发请求触发的额度预检、bucket 写入、凭据读取与 Argon2 都受同一个 1～64 上限约束；验证结束后释放，不占用后续安全事件或 session 提交时间。取得许可后，repository 先只读检查 account、IP prefix、global 三个精确 bucket：已有封禁时不更新 blocked hit、不创建其他 scope 的 row；未命中才进入 account→IP→global 的权威事务。不存在的用户也验证固定 dummy PHC，HTTP 响应不区分不存在、密码错误和停用账号。

这不是完整 WP-02。公开 `ecd8dea` 基线在本文首次验收时尚未实现 TOTP、WebAuthn、恢复码、recent-auth 动作门、密码修改与透明 rehash、个人/API token、session 管理页、完整对象级 RBAC、用户 CRUD/删除生命周期、Turnstile 和浏览器 Playwright 安全套件。recent-auth、改密、登录透明 rehash 与 session 管理后来由 [C1](./WP02_C1_PASSWORD_RECENT_AUTH_SESSION_IMPLEMENTATION.md) 实现，并已绑定公开 `3f1bcb49…`、Actions artifact `9609917545` 和 fresh-checkout VPS run `20260826T135902729109375Z-p5`。其余能力仍未实现。`owner/admin/operator/support/auditor/member` 与 capability 基线只是后续授权骨架，不能据此宣称用户管理或 IDOR 矩阵已经完成。需求矩阵继续保持诚实的 `planned`，直到对应完整验收合同通过。

## 2. 数据与密码边界

### 2.1 `crates/domain`

| 类型或函数 | 责任 | 当前约束 |
|---|---|---|
| `UserRole::{as_str,parse}` | 在 domain 与数据库枚举之间做双向映射 | 只接受六个项目角色；未知字符串失败 |
| `UserStatus::{as_str,parse}` | 映射 active、suspended、disabled、pending_deletion | 登录仅允许 active |
| `CapabilityScope::{as_str,parse}` | 定义稳定 capability 字符串 | 未知 scope 不进入投影 |
| `BaselineCapabilities::for_role` | 生成每种角色的只读基线投影 | 当前不是对象关系授权判定器 |

### 2.2 `crates/identity`

| 类型或函数 | 责任 | 安全性质 |
|---|---|---|
| `SessionToken::generate` / `CsrfToken::generate` | 用操作系统 CSPRNG 生成 32-byte 随机值 | 分别编码为 `ncs1_`、`ncc1_` 加 64 位小写十六进制 |
| `parse_presented` | 校验浏览器提交的 token | 长度、版本前缀、字符集全部严格；错误不返回原值 |
| `SessionTokenPair::{generate,into_tokens}` | 保证 session 与 CSRF 一次成对创建、一次移交 | token 类型不实现可泄漏内容的 `Debug`，内部字符串离开作用域时清零 |
| `constant_time_bounded_string_equal` | 比较有上限的 CSRF cookie/header | 长度差也进入固定上限循环，避免直接字符串短路比较 |
| 安全文件读取辅助 | 单次打开 setup/root-key 文件 | Unix 使用 `O_NOFOLLOW/O_CLOEXEC/O_NONBLOCK`，要求 regular file、权限与 euid owner 合法并限制读取长度；Windows 拒绝 reparse point |

### 2.3 `crates/secrets`

`EnvelopeCipher::keyed_digest` 使用根密钥经 HKDF-SHA256 派生不同用途的 HMAC-SHA256 key。`KeyedDigestPurpose` 分开 `Session`、`Csrf`、`LoginAccount`、`LoginIp`、`LoginGlobal`，避免同一输入跨协议得到可关联 digest。返回值包含 `key_version`；数据库唯一键也包含版本，后续可以引入有限旧 key ring。`EnvelopeCipher::key_version` 给安全事件写入当前版本。`auth_sessions`、`login_rate_buckets` 和 `login_security_events` 不保存浏览器 token、明文规范化账号、原始 IP 或 User-Agent；用户主表仍以 `users.username_norm` 持久化规范化账号，用于唯一约束和登录查询。User-Agent 只保留 SHA-256，账号/IP/全局 bucket 使用 keyed digest。

当前 key rotation 仍缺旧 key ring 和 session 渐进迁移。直接切换根 key 会使旧 session 失效，这一行为必须在实现正式轮换工作流前保持明确。

### 2.4 `crates/persistence` 的认证枚举

| 类型 | 责任 | 当前约束 |
|---|---|---|
| `AuthLevel` | 保存 session 的认证等级 | 已支持 `password`、`mfa`、`phishing_resistant`、`recovery`；初始密码登录创建 `password`，rotation 保留原 session 的等级 |
| `AuthSessionStatus`、`SessionRevocationReason` | 描述服务端 session 生命周期 | status 为 active/revoked；logout、expired、security_policy、password_changed、user_revoked、administrator 等原因只用于 revoked row |
| `LoginSecurityReason` | 规范登录、近期认证、改密、撤销和限流事件 | 枚举值受数据库 CHECK 与 typed repository outcome 双重约束，`rate_limited` 不能经通用事件入口独立写入 |

## 3. 配置函数与启动接线

### 3.1 `crates/config`

| 配置/解析器 | 责任 | 拒绝条件 |
|---|---|---|
| `PublicOrigin::parse` | 固定浏览器对外访问的唯一 canonical origin | 外部地址必须 HTTPS；HTTP 只允许 localhost/loopback；拒绝路径、query、fragment、userinfo 和非规范形式 |
| `TrustedProxyCidr::parse` | 声明可以提供转发链的反向代理网段 | 拒绝 `/0`、host bits 非零和非规范 CIDR |
| `HttpConfig.public_origin` | 给 Origin/Host/Cookie 边界提供单一事实源 | 缺失或不合法时启动失败 |
| `HttpConfig.trusted_proxy_cidrs` | 限定 `X-Forwarded-For` 信任入口 | 未在列表内的 peer 所带转发头不会改变客户端身份 |
| `AuthConfig` | 配置 idle/absolute session、登录窗口/封禁、account/IP/global 阈值、Argon2 并发数和 digest key version | 零值、idle 不小于 absolute、封禁时长短于统计窗口、阈值/时长越界、并发数不在 1～64 等均失败 |

`AuthPolicy::validate` 在 application 构造时再次检查运行时不变量，避免测试替身或未来非文件配置绕过 config 层。`ControlPlaneApplication::new` 只有在 policy 合法时才创建 `Arc<Semaphore>`。Master 启动阶段还在 blocking thread 中生成 dummy PHC；生成失败不会启动一个会泄漏未知用户 timing 的服务。

### 3.2 `apps/master`

Master 把 typed config 映射为 `AuthPolicy` 和 `WebSecurityPolicy`，把数据库、root-key cipher、`PasswordService`、dummy PHC 与可选 setup capability 交给 `ControlPlaneApplication`。HTTP server 通过 `into_make_service_with_connect_info::<SocketAddr>()` 注入真实 peer 地址，API 不再把监听地址或伪造 header 当作客户端 IP。session touch 间隔取 idle 的四分之一并封顶 60 秒，减少每次 `/me` 都写库的压力。

## 4. 双数据库迁移与 repository

SQLite/PostgreSQL 的 `0003_auth_core.sql` 新增四组持久状态：

| 表 | 责任 |
|---|---|
| `user_auth_state` | 每个用户的 `auth_revision`；密码或高风险认证状态变化时可批量使旧 session 失效 |
| `auth_sessions` | token/CSRF HMAC、认证等级、idle/absolute 期限、最近使用、创建来源摘要、撤销状态与 revision |
| `login_rate_buckets` | scope + key version + bucket HMAC 唯一的共享固定窗口计数与封禁截止时间 |
| `login_security_events` | 只含 request ID、原因和不可逆摘要的认证与会话安全审计原语，覆盖登录、重新认证、改密、撤销、退出和限流 |

### 4.1 对外 repository 方法

| 方法 | 事务与语义 |
|---|---|
| `user_credentials_by_normalized_username` | 只投影登录需要的用户、PHC、状态、角色、强制改密和 auth revision；deleted/损坏记录不被冒充成有效凭据 |
| `reserve_login_attempt` | 先以一条只读 MAX 查询检查三个精确 HMAC bucket；已有封禁时返回最长 `Retry-After`，不更新或创建 row。未命中才在同一事务按 account→IP→global 更新，权威事务仍负责处理并发竞争 |
| `create_auth_session_with_optional_password_upgrade` | 生产登录入口；session、登录成功事件、account bucket 清理和可选透明 PHC 升级在同一事务中完成，并用 user/auth/password snapshot 做 CAS |
| `authenticate_session` | 用 token key version/HMAC 查找 active session，联表核对用户状态和 auth revision，再检查 absolute/idle；可选 CSRF 也在数据库比对；达到 touch interval 才延长 idle |
| `authenticate_session_read_only` | 在共享凭据读取路径中完成同一套 active/user/auth-revision/deadline 校验，但不 touch session；供需要稳定 snapshot 的近期认证与并发协议使用 |
| `list_user_sessions` | 返回指定用户的全量会话投影，不含 token/HMAC，也不按 active、期限或 auth revision 过滤 |
| `list_active_user_sessions` | 在数据库侧过滤 active、idle/absolute 期限和 auth revision；application 的会话管理列表使用这一查询 |
| `rotate_current_session` | reauth 成功后在一个事务内撤销旧凭据、签发 replacement，并继承 absolute expiry |
| `change_password_and_rotate` | 原子写入新 PHC、推进 auth revision、撤销全部旧 session 并创建唯一 replacement |
| `revoke_current_session_with_event` | 普通 logout 的原子撤销与事件入口；重复或已失效目标不制造事件 |
| `revoke_user_session_with_event` | 会话管理页 actor-aware DELETE；事务内重验调用方、近期认证、时间线与目标，再写 `session_revoked`/`user_revoked` |
| `logout_all_sessions_with_event` | `logout-all(false)`；撤销包括当前会话在内的全部活动 session，并写单条权威事件 |
| `logout_all_sessions_and_rotate` | `logout-all(true)`；撤销其他凭据并以新 token pair 替换当前 session |
| `record_login_security_event` | 写普通独立安全事件并明确拒绝 `rate_limited`；限流事件只能随 `reserve_login_attempt` 的 durable block transition 原子提交 |

### 4.2 bucket 状态函数

`next_rate_bucket_state` 处理窗口过期、计数递增和首次越限封禁；`rate_bucket_outcome` 把数据库计数收敛为 bounded `remaining_attempts`；`combine_rate_bucket_outcomes` 取最小剩余额度、最晚 reset 和最长有效封禁；`rate_bucket_is_limited` 决定是否短路下一层。PostgreSQL 对现有 bucket 使用 `FOR UPDATE`；SQLite 在写事务中执行同一合同。双数据库 contract test 使用相同 fixture，避免只在一种引擎上成立。

本纵切没有实现历史 bucket/过期 session 清理 job，也没有宣称跨地域数据库的全局强一致限流。SQLite 高并发 busy 行为和 PostgreSQL 热 bucket 吞吐仍要进入后续压力测试。

## 5. Application 登录与会话函数

### 5.1 `login`

执行顺序固定如下：

1. 从数据库确认 bootstrap 已完成；未完成返回稳定 409。
2. 用 `try_acquire_owned` 取得登录/Argon2 slot；满载立即 429，且此时尚未触碰 limiter。permit 持有到密码验证结束，随后在写失败事件或创建 session 前释放。
3. 尝试解析用户名。合法用户名用规范化值，非法或超长输入先收敛为 bounded subject，再生成 account HMAC。
4. 从可信 peer/代理链得到 canonical IP prefix，分别生成 IP 和 global HMAC。
5. repository 先只读检查三个精确 bucket；已有封禁时返回 bounded `Retry-After`，不查账号、不做 Argon2，也不扩张 bucket 集合。未封禁时才在权威事务中占用 account→IP→global 三层额度。
6. 按规范化用户名读取凭据。查不到或用户名非法时选 dummy PHC。
7. 实际 verify 只在 `spawn_blocking` 中运行；账号不存在、密码不符、非 active 统一返回 `INVALID_CREDENTIALS`，事件内部可以区分 inactive，但响应不区分。超过 1,024 bytes 的密码也映射为同一通用失败。
8. 成功后生成 token pair、用途隔离 HMAC、idle/absolute deadline 和来源摘要，在数据库原子写 session + success event + account bucket clear。
9. 只把原始 token 移交给 API；actor/session 投影不含 HMAC、PHC、IP 或 User-Agent。

### 5.2 `authenticate_credential`

函数严格解析 session token、计算带版本 HMAC，并构造 `SessionAuthentication`。repository 同时检查用户 active、删除状态、auth revision、session 状态和两类 deadline；失败统一收敛为 `SessionInvalid`。只读 `/me` 不要求 CSRF。达到 touch interval 时，idle deadline 只可延长到 absolute deadline以内。

### 5.3 `authenticate_mutating_credential`

写请求使用显式 `MutatingSessionCredential`，类型上不允许省略 CSRF。Application 严格解析两个 token 后只调用一次 repository；SQLite/PostgreSQL 都用同一条 session/user/auth-revision 约束查询得到 `SessionAuthenticationOutcome::{Authenticated, InvalidSession, InvalidCsrf}`。无效 session 映射 401，session 有效但 CSRF HMAC 不匹配映射 403。`InvalidCsrf` 会提交只读结果，不更新 `last_seen_at`、idle deadline 或 revision；不存在“第一次检查先 touch、第二次再发现 CSRF 错误”的窗口。

### 5.4 `current_actor` 与 `logout`

`current_actor` 只返回 server-side actor/session 投影。`logout` 先完成 session + CSRF 认证，再从当前请求生成审计摘要，调用 `revoke_current_session_with_event`；用户换网或 User-Agent 变化不会被硬绑定拒绝。`logout` 请求未携带 session，或 Cookie 结构合法但 token 格式错误、未知、撤销、过期时，返回显式成功的幂等 204，并由该成功响应清两枚 Cookie；重复目标 Cookie、超长 header 等 Cookie header 结构歧义返回 401 Problem，且该 Problem 不写 `Set-Cookie`。`/me` 等其他受保护 API 缺少有效 session 仍返回 401 Problem；有效 session 的 CSRF 失败则是 403。

## 6. HTTP 与浏览器安全边界

### 6.1 `crates/api::web_security`

| 函数 | 行为 |
|---|---|
| `validate_browser_origin` | 写请求必须有且只有一个 Origin 和 Host；Origin 与配置逐字匹配，Host 做 ASCII case-insensitive 精确 authority 匹配 |
| `validate_request_host` | `/me` 等只读浏览器端点仍核对 Host，阻止错误反代入口 |
| `resolve_client_network` | peer 不可信时忽略 XFF；peer 可信时要求非空、≤1,024 bytes、≤16 hops 的纯 IP 链，从右向左越过连续可信代理，停在第一个不可信地址 |
| `bounded_user_agent` | 只接受唯一且不超过 512 bytes 的值 |
| `security_cookie` | 所有 Cookie header 合计≤8 KiB、≤64 对；目标 cookie 必须唯一、非空且≤96 bytes，不做含糊 URL decode |
| `csrf_header_and_cookie` | CSRF cookie/header 都必须唯一、格式严格、常数时间相等 |
| `session_set_cookie` / `csrf_set_cookie` | 两者均 `__Host-`、`Secure`、`Path=/`、`SameSite=Lax`；session 额外 `HttpOnly` |
| `clear_session_cookie` / `clear_csrf_cookie` | 用相同 cookie 属性和 `Max-Age=0` 可靠覆盖删除 |

### 6.2 API 路由

| 路由 | 成功合同 | 主要失败 |
|---|---|---|
| `POST /api/v1/auth/login` | 200 actor/session envelope，两个 Set-Cookie，`Cache-Control: no-store` | 400/413/415/422 JSON 合同，401 通用凭据错误，403 Origin/Host，409 未初始化，429 限流/Argon2 满载，503 依赖失败 |
| `GET /api/v1/me` | 200 当前 actor/session，刷新页面可恢复 | 401 缺失/格式错/撤销/过期/用户状态变化；403 Host；503 依赖失败 |
| `POST /api/v1/auth/logout` | 204，撤销当前 session 并清两个 Cookie；缺 session 或结构合法但无效的 token 也幂等成功 | 401 Cookie header 结构歧义，403 Origin/Host/有效 session 的 CSRF，503 原子撤销失败 |

`Problem` 只可按本地 allowlist 附加普通响应头；`retry_after_seconds` 不进入 JSON/OpenAPI，只生成标准 `Retry-After`。任何 Problem，包括 `401 SESSION_INVALID`，都不得写 `Set-Cookie`。这是跨标签页安全边界：较早请求的迟到 401 不能清掉另一标签页刚轮换出的新 Cookie。只有路由明确选用的成功响应可以签发、轮换或清 Cookie；其中 logout、撤销当前 session 与 `logout-all(false)` 的权威 204 复用同一个清理响应。错误映射只使用本地固定 title/detail/code，不把 SQL、Argon2、header 解析器或用户输入错误原文返回浏览器。

OpenAPI 增加 session cookie 与 CSRF header security scheme；当前生成 SDK 暴露 `login`、`logout`、`logoutAll`、`reauthenticate`、`getCurrentActor`、`changeCurrentPassword`、`listCurrentSessions` 与 `revokeCurrentUserSession`。生成文件只由最终 Rust schema 在 VPS 导出后生成，不手工编辑。

## 7. Vue 3 / Vuetify 前端

### 7.1 session store

`useSessionStore` 的 actor、session 与凭据仍只存在内存，不启用 Pinia persistence plugin。浏览器存储只允许一个固定键 `nodecontroll:credential-coordination:v1`：它保存不含 token、CSRF、密码或用户资料的协调 journal；`sessionStorage` 必须为空。状态机为：

```text
unknown → loading → setup-required
                  → anonymous
                  → authenticated
                  → unavailable
                  → relogin-required
```

`credential-coordinator.ts` 用同源 Web Locks 串行化会轮换 Cookie 的 mutation，并让受保护读取在共享锁内完成请求、解析和运行时投影校验。journal 使用随机 epoch 和规范十进制 revision；mutation terminal 必须精确引用它实际观察到的 inflight，并按 `inflight → settled` 转换。受保护读取观察到 401 时，另以 `read-401/invalidated` 记录绑定原 base cursor，取得 exclusive 后只有 cursor 未变才持久化。持久 quarantine、BroadcastChannel 和 storage event 分别承担权威状态与唤醒，防止崩溃、丢事件与 ABA。损坏、revision 回滚或同值篡改、跳号、未观察到的 epoch 替换和未知 mutation 结果都会先关闭 actor/session 与受保护 DOM。journal 缺失时，fresh setup/anonymous 且无 CSRF Cookie 是合法初始态；已有 CSRF 或 authenticated/unavailable/relogin-required 投影时则进入隔离。只有结构完整且 generation 匹配的显式登录 200，或路由明确声明的权威清理 204，才能解除 sticky quarantine。

`refresh` 合并并发请求：先读 bootstrap，未初始化就不探测 `/me`；已初始化才在共享凭据锁内读 `/me`。明确 401 会按所观察的 session/cursor 发布并持久化条件失效；其他协议或网络失败转 unavailable，协调状态不可信则进入 relogin-required。`login` 只消费白名单 HTTP 状态和经过运行时结构校验的 200 投影，不渲染服务端 Problem 文本；等待 exclusive lease 时，自身 inflight 引起的 generation 精确 `+1` 才是正常推进，额外推进说明外部 mutation，401/429 也不能解除 quarantine。`logout` 从 `document.cookie` 读取唯一、严格格式的 CSRF token并显式写 header。账号、密码、session token 和 CSRF token都不进入 store 或 journal；密码由页面在请求结束后清空。

### 7.2 router 与页面

`createAppRouter` 支持 production history 和测试 memory history，并显式注入 Pinia。guard 首次导航等待 session 状态：未初始化只允许 `/setup`，匿名只允许 `/login`，已认证不能返回 setup/login。`safeRedirectPath` 只接受单斜杠开头的同站路径，拒绝 scheme-relative、反斜杠、编码后的双斜杠/反斜杠、控制字符、数组值和 guest 页面，避免登录后开放重定向。

`LoginPage` 使用浏览器密码管理器兼容的 autocomplete，错误文案只由本地枚举生成，登录成功用 `router.replace` 进入经过净化的站内目标；无论成功失败都在导航前清空密码。认证已成功但导航失败时显示独立提示、禁用再次登录并只允许重试导航，避免重复创建 session。`SetupPage` 在 Master 确认 initialized 后同步 session store 为 anonymous 并转到登录页；初始化写入已成功但 refetch 未确认时继续锁表单，避免重复写。`App.vue` 把 setup/login 放进 guest shell，把已认证页面放进标准 SaaS navigation/app bar；控制面状态无法确认，或受保护 route 与非 authenticated 状态不一致时，立即从 DOM 移除受保护内容并显示 fail-closed 门。即使退出成功后的路由跳转失败，旧页面也不会继续显示。

## 8. 测试合同与证据状态

代码内测试覆盖：token 格式/随机性/常数时间比较、安全文件边界、配置负例、角色/能力映射、双数据库 session 生命周期与原子 logout、三层 limiter 及短路、API cookie/header/Origin/Host/XFF 对抗输入、通用登录错误、前端 store 并发恢复/秘密不持久化、router guard/重定向、LoginPage 不可信 Problem 隔离和 SetupPage 初始化转场。

2026-08-26 的 VPS 候选树已经通过：Rust `fmt/check/test/clippy -D warnings/release build`，共 68 个 workspace test，SQLite 与固定 PostgreSQL 均执行；最终 Rust 导出的 OpenAPI 为 7 paths/8 operations，生成 OpenAPI/SDK 与工作树逐字节一致；Vue 通过 typecheck、零 warning lint、29/29 Vitest 和 341-module production build。真实 PostgreSQL 上完成 bootstrap→login→`/me`→logout→撤销拒绝，并在 Master 重启后再次完成同一会话闭环。重启前后日志按真实 setup token、root key、测试口令、PHC 与 token 前缀扫描，均为零命中。

公开 commit `ecd8deaecd6dcfad8fd365dada67e5fc487046ad` 随后完成正式验收。GitHub Actions run `32919113045` attempt 1、artifact `9589258880` 全绿；artifact 为 4,569,681 bytes，SHA-256 `1798c3746856f33deef36fe657ccd70294916ce0d6662d0c247827425ecc7b40`。fresh full clone 的 VPS run `20260826T013832936716270Z-p5` 为 `completed`，源码测试前后洁净；正式门重跑 68 个双库 Rust tests、29 个 Web tests、release/Web 重建和真实 Master smoke，并把 ELF、OpenAPI、Web dist、notices 与 Actions 逐字节比较。runtime log secret scan 通过，临时 secret/container/network 已清理。

以上 68/29/341 是 `ecd8dea` 基线的历史正式数字。历史 v4 C1 应用代码另以 226-file archive `0c2eb2a94256ee3b5795e9811b2833090989244c5d8370a9ce05beb8f5fabe5c` 在 VPS 固定 builder 中通过 78/78 Rust workspace all-targets tests、Clippy、release、9 文件 81/81 Vitest、362-module Web production build、生成目录 16 个物理文件零漂移，以及 SQLite/PostgreSQL 同合同 runtime smoke；逐文件清单 SHA-256 为 `772172f85e38d94ba67846f66252a588631ecb6a342566af1949357a5a7f61dc`。双页 HTTPS candidate v4 `20260826T112446112732525Z-wp02c-candidate-v4` 还验证了旧凭据 401 零 `Set-Cookie`、logout 503 quarantine、跨 reload 保持、显式登录恢复、迟到旧 cursor invalidation 不覆盖新状态，以及真实 logout 204；外层 validator 复算 33 个冻结文件、10,087,567 bytes 后通过。最新 v6 archive `e2a055...8cb58` 又覆盖 global virtual store=false、完整静态/Web、许可证闭包和两库 smoke。三者都是候选行为证据，不绑定随后形成的 Actions/VPS 编译边界修正，也不是 C1 的正式 provenance。

这个结论只覆盖本文纵切。需求矩阵仍保持 358 项 `planned`，因为 MFA、token、完整 RBAC/用户生命周期和对应产品验收还没有完成。

当前已确认的后续债务也不隐藏：`login_rate_buckets`、`login_security_events` 及撤销/过期 session 还没有 retention job，长期允许的失败会以有限速率永久增长数据库；root-key canary 还不是持久化的 keyring 指纹，格式正确但错误的根密钥不能仅靠启动时自加解密发现。C1 已把撤销时间钳制为 `max(created_at, now)`，并在一般认证、活动列表和 rotation 选择器拒绝 `now < last_seen_at`；WP02-E 仍需用可控时钟扩展完整回拨/跳跃矩阵。超长密码失败事件、亚毫秒 policy 输入、Windows ACL 与完整真实浏览器 Secure Cookie 套件也要在后续门中补齐。

## 9. 下一步

1. WP02-C1：recent-auth、透明密码 rehash、密码修改与 session rotation/管理已通过历史 v4 应用代码门、v6 门工具测试和真实双页 HTTPS candidate；代码与候选证据见 [C1 实现记录](./WP02_C1_PASSWORD_RECENT_AUTH_SESSION_IMPLEMENTATION.md)。先把最新 Actions/VPS 编译边界与账本纳入最终 freeze，再执行单父公开提交、Actions 同 SHA 制品和 fresh-clone 正式门。
2. WP02-C2～C7：持久化 key canary/keyring、一次性恢复码、统一 challenge、TOTP、WebAuthn、所有高危 use case guard 与真实浏览器/并发/故障注入验收。
3. WP02-D：personal/service token、六角色 RBAC、object relationship、字段投影、用户 CRUD/停用/软删、最后一个 Owner 保护和 IDOR 矩阵。
4. WP02-E：在 C1 双页凭据协调候选之上补齐 MFA/WebAuthn、token、RBAC/IDOR 的真实浏览器路径，以及 SQLite/PostgreSQL 重启恢复、并发/封禁/过期时钟、可信代理与 HTTPS/loopback Cookie 完整套件。
5. WP03 前先补过期/撤销 session、bucket 和安全事件的 durable retention job；再增加持久化 canary、HMAC 旧 key ring、认证指标和不含 secret 的安全审计查询面。

完成这些内容并通过 [WP-02 完成门](../04-rebuild/IMPLEMENTATION_PLAN.md#6-wp-02身份会话mfa角色与用户基础) 之前，不进入“完整身份系统已完成”的口径。
