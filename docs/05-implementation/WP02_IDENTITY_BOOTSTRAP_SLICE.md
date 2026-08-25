# WP-02 身份纵切：Owner 凭据与一次性控制面初始化

## 1. 交付边界与当前结论

本纵切实现了一个可实际写入数据库的首次初始化闭环：公开页面读取初始化状态，部署者同时提交短时一次性 setup token、实例名、Owner 用户名和密码，Master 先校验部署者能力，再在受限的阻塞线程中生成 Argon2id 密码哈希，最后由 SQLite 或 PostgreSQL 的同一事务创建或接管实例、创建首个 Owner、写入所需默认设置并关闭 bootstrap latch。旧的 `0001_foundation` 数据库已有实例时，`0002_identity` 会进入 `LegacyNeedsOwner`；后续初始化保留旧实例和兼容的已有设置并创建 Owner。固定的 `subscription.behavior` 设置缺失时补入默认值；schema version 或 typed JSON 不兼容时整笔 fail closed。

这仍只是 [WP-02 规划](../04-rebuild/IMPLEMENTATION_PLAN.md#6-wp-02身份会话mfa角色与用户基础) 的第一个纵切，不是身份系统完成。当前没有可用的密码登录端点、登录 challenge、浏览器 session、恢复码、MFA/WebAuthn、CSRF/Origin 校验、用户管理、密码重置、API token 或 RBAC。`GET /api/v1/bootstrap` 在初始化后仍返回空的 `login_methods`，避免把已存密码凭据误报成可用登录方式。[需求追踪矩阵](../04-rebuild/REQUIREMENTS_TRACEABILITY.md) 因此不应仅凭本纵切把 `MMW-AUTH-*` 标为 `implemented` 或 `verified`。

本文说明的是当前工作树，而不是一个已经发布的版本。此前的 [WP-01 基础纵切](./WP01_FOUNDATION_SLICE.md) 和 [WP-01 存储/密钥纵切](./WP01_STORAGE_SECRET_SLICE.md) 仍是它的前置基础。

## 2. 模块与函数责任

### 2.1 Domain：输入不变量与身份值对象

实现位于 [`crates/domain/src/lib.rs`](../../crates/domain/src/lib.rs)。

| 类型/函数 | 当前责任 | 不变量或诚实边界 |
|---|---|---|
| `InstanceName::parse` | trim 首尾空白并建立实例名值对象 | 1～80 个 Unicode scalar；拒绝 control character；数据库接收的是 trim 后的值 |
| `Username::parse` | 建立登录名值对象 | 3～32 个 ASCII 字节；仅字母、数字、`_`、`-`、`.`；由于限定 ASCII，字节数也等于字符数 |
| `Username::as_str` | 返回保留用户原始大小写的显示/存储值 | 不做 Unicode 或 locale 归一化 |
| `Username::normalized` | 生成唯一性键 | 只做 ASCII lowercase；例如 `Owner` 与 `owner` 冲突 |
| `PasswordHash::parse` | 把持久化前后的 PHC 字符串约束为可接受的 Argon2id 哈希 | 见本小节下文；它不是密码验证函数 |
| `PasswordHash::as_str` | 在 identity/persistence 边界窄范围暴露 PHC | `PasswordHash` 故意没有 `Debug`/`Serialize` |
| `PrincipalLabel::parse` | 建立可安全用于机器映射的稳定 principal label | 1～80 个 ASCII 字节；字符集同用户名；数据库全局唯一 |
| `UserRole::as_str` | 把当前 domain role 映射为数据库值 | domain 目前只实现 `Owner`；数据库为后续角色预留更多枚举不等于 RBAC 已实现 |
| `UserAccount` | 聚合首个用户写入所需字段 | 包含凭据哈希，因此没有派生 `Debug`/`Serialize`；当前没有读取、修改或认证 repository |

Master 自动生成 `principal_label = "usr_" + owner UUIDv7`，而不是接受公开请求中的 label。`UserAccount` 初始值固定为 `role=Owner`、`status=active`（由 SQL 写入）、`force_password_change=false`、`revision=0`。这套 label 当前只解决稳定机器身份和数据库唯一性，尚未接入对象关系授权。

`PasswordHash::parse` 不只检查 `$argon2id$` 前缀。它使用 PHC parser 并要求：

- 字符串非空且不超过 512 字节；
- 算法严格为 `argon2id`、版本严格为 `19`；
- 参数恰好只有 `m/t/p` 三项；
- `m` 为 8,192～262,144 KiB、`t` 为 1～10、`p` 为 1～8；
- Base64 salt 解码后至少 8 字节，输出严格为 32 字节。

这使未来登录读取旧哈希时不会接受无界内存/迭代参数。数据库自身只约束 `password_hash` 长度为 1～512；绕过 application 直接写 SQL 仍可放入语义无效的 PHC，所以任何凭据读取路径仍必须再次调用 `PasswordHash::parse`。

### 2.2 Identity：密码策略、setup capability、哈希与验证

实现位于 [`crates/identity/src/lib.rs`](../../crates/identity/src/lib.rs)，crate 入口没有 HTTP、SQL 或配置依赖。

| 函数 | 责任 | 当前参数/行为 |
|---|---|---|
| `PasswordService::recommended` | 构造当前固定参数的 password service | Argon2id v19；`m=19,456 KiB`、`t=2`、`p=1`、输出 32 字节 |
| `PasswordService::validate` | 在占用限速槽和 Argon2 资源前执行新密码策略 | 与 `hash` 的输入策略相同，但不生成 salt 或 PHC |
| `PasswordService::hash` | 校验新密码、生成随机 salt、计算 PHC、交回 domain 再校验 | 16-byte OS CSPRNG salt；salt 临时数组用 `Zeroizing`；输出含算法、版本、参数、salt 与 hash |
| `PasswordService::verify` | 对输入密码与已校验的 `PasswordHash` 做 Argon2 verify | 只先执行 1,024-byte 资源上限，不重施“新密码”最短长度/control-character 策略，以便未来兼容旧凭据；当前没有调用它的登录端点 |
| `validate_password` | 新密码策略 | 至少 12 个 Unicode scalar；不允许 control character；先执行字节上限 |
| `validate_password_resource_bound` | 在 Argon2 工作前限制攻击者输入 | 最多 1,024 个 UTF-8 字节，不是 1,024 个 Unicode 字符 |
| `PasswordError` | 区分策略错误、随机源/编码/参数/运算/存储错误 | Master 只把策略三类映射为公开 400；运行和存储错误统一降为 503，不泄露内部原因 |
| `SetupCapability::from_file` | 从仅 owner 可读、非 symlink 的 regular file 载入 32-byte 随机 token，并只保留 SHA-256 digest | 文件严格是 64 位小写十六进制，可带一个结尾 LF；TTL 必须大于 0，配置层上限 3,600 秒；默认 1,800 秒 |
| `SetupCapability::authorize` | 检查未消费、未过期、格式正确，并对固定长度 digest 做不提前返回的比较 | token 不进入 tracing span；无效 capability 不占用 bootstrap 的 Argon2/全局尝试间隔 |
| `SetupCapability::consume` | 数据库事务成功后把本进程 capability 标为已消费 | 数据库 `ready` latch 是跨重启、跨副本的一次性边界；内存标记是同进程快速拒绝层 |

当前参数是硬编码的安全基线，还没有实现 [安全设计](../04-rebuild/SECURITY.md#4-身份密码和会话) 要求的 VPS 目标级 250～500 ms 校准、按旧 PHC 参数透明 rehash、本地泄漏密码拒绝集或可运维的参数升级策略。`verify` 已提供底层能力，但不能据此声称登录、恒定外观错误或账号枚举防护已经完成。

密码从 `BootstrapRequest` 反序列化后会立即通过 `std::mem::take` 移入 `Zeroizing<String>`；`BootstrapCommand`、`PasswordHash` 和 `UserAccount` 均不提供默认 debug 输出。HTTP tracing span 只记录 method/path，不记录请求 body。这里仍是 best-effort：Axum body buffer、Serde 分配、Argon2 实现内部以及 allocator 已释放区域不保证被全部擦除，生产环境还需要 core-dump 策略和结构化日志审计。

### 2.3 Persistence：显式状态、历史升级与原子写入

实现位于 [`crates/persistence/src/lib.rs`](../../crates/persistence/src/lib.rs)，schema 分别位于 [`SQLite 0002`](../../crates/persistence/migrations/sqlite/0002_identity.sql) 与 [`PostgreSQL 0002`](../../crates/persistence/migrations/postgres/0002_identity.sql)。

#### `0002_identity` schema 变更

两套 migration 都创建 `users`，包含 UUID、原始/归一用户名、PHC、role/status、principal label、force-change、revision、创建/软删时间。`users_username_norm_active_uq` 是带 `deleted_at_ms IS NULL` 条件的唯一索引：所有未软删用户的归一用户名都唯一，无论其 status 是 active、disabled 还是 suspended；软删后才允许复用。principal label 则始终全局唯一。PostgreSQL 使用原生 UUID；SQLite 由 application 写入 UUID 字符串，数据库只做长度与粗粒度 GLOB 形状/字符检查，不等价于完整 UUID parser，也不约束 UUID version。

SQLite 因不能原地增加目标外键，会把 `instance_settings` 重建并复制旧数据；PostgreSQL 会先把旧的非空 `updated_by` 清为 `NULL`，再增加外键。两者最终都让 `instance_settings.updated_by` 引用 `users(id) ON DELETE SET NULL`。旧 `0001` 的 `updated_by` 只是没有身份外键的自由值，因此 migration 不把它冒充为新用户 ID。

`control_plane_bootstrap` 是 singleton row：`singleton_key=1`，状态只能是 `pending/ready`，`ready` 必须带 `instance_id`，该 ID 引用实例且 `ON DELETE RESTRICT`。migration 用当前 `instances.singleton_key=1` 回填可空的 `instance_id`，因此同一份 SQL 同时支持空库和已有 0001 实例。

#### 状态分类

`Database::bootstrap_state` 一次读取 latch 以及“是否有实例、是否有用户、是否有未删除 active owner”，再由 `classify_bootstrap_record` 严格分类：

| latch / 数据组合 | Domain 状态 | 对外含义 |
|---|---|---|
| `pending`、无 latch instance ID、无实例、无用户、无 active owner | `Uninitialized` | 可创建新实例、Owner 和默认设置 |
| `pending`、有 latch instance ID、有实例、无用户、无 active owner | `LegacyNeedsOwner` | 0001 历史实例待补 Owner；公开投影仍为未初始化 |
| `ready`、有 latch instance ID、有实例、有用户、有 active owner | `Ready` | 首次初始化关闭 |
| 任何其他组合 | `InconsistentBootstrapState` | 不猜测修复；GET/POST 返回稳定 503，等待 operator recovery |

singleton latch row 完全缺失也显式映射为 `InconsistentBootstrapState`，而不是把 SQL `RowNotFound` 混入一般依赖错误。`Database::is_initialized` 只有在状态严格为 `Ready` 时才返回 `true`，不再把“实例表有一行”误当成“可登录的控制面已完成初始化”。

#### 函数与事务步骤

| 函数 | 责任 |
|---|---|
| `Database::bootstrap_state` | 对 SQLite/PG 执行等价状态查询并严格分类 |
| `Database::is_initialized` | 把状态收敛为公开 boolean；inconsistent 不降级为 false |
| `Database::bootstrap_control_plane` | 先检查非负时间、revision 数据库表示和 settings JSON，再分派双库实现 |
| `bootstrap_sqlite` | 在一个 SQLite transaction 内 claim latch、创建/接管资源并 finalize |
| `bootstrap_postgres` | 使用 PostgreSQL 原生 UUID/JSONB 占位符执行相同语义 |
| `classify_bootstrap_record` | 只接受上述三个合法数据组合，防止部分数据被静默视为可重试 |
| `Database::save_subscription_settings` | 本纵切新增必填 `actor_id`；成功 insert/update 都把实际用户写入 `updated_by`，不存在的用户不能通过外键完成写入 |
| `Database::active_owner_count` | 测试/诊断当前 active、未删除 Owner 数量；不是用户列表 API |

`bootstrap_sqlite` / `bootstrap_postgres` 的顺序相同：

1. 开启 transaction；
2. 第一条语句执行 `UPDATE control_plane_bootstrap SET status=status ... WHERE status='pending'`，用 singleton row 获得数据库级写锁；
3. 在同一 transaction 内读取并分类 latch 与真实表状态；
4. `Uninitialized` 插入请求生成的新实例；`LegacyNeedsOwner` 读取并保留 latch 指向的旧实例 ID；`Ready` 返回 `AlreadyInitialized`；
5. 插入 Owner。只有 `username_norm` 的未软删 partial-index 冲突通过精确 conflict target 变成 `IdentityConflict`，不会解析供应商错误字符串，也不会把 UUID/principal-label 等内部约束错误误报为用户名冲突；
6. 新实例插入 schema-v1 默认 `subscription.behavior` 设置并把真实 Owner ID 写入 `updated_by`；legacy 实例在锁内读取该 key：缺失时以 Owner 为 actor 插入，schema-v1 且能反序列化为 `SubscriptionBehaviorSettings` 时连同 migration 置空的历史 `updated_by` 原样保留，错误 schema 或 typed JSON 则返回 inconsistent 并回滚 Owner；
7. 条件更新 latch 为 `ready` 并写入实际持久化的 instance ID；受影响行数不是 1 即报 inconsistent；
8. 最后 commit，并返回实际 instance ID。

fresh 路径的新实例、Owner、默认设置和 latch finalize 属于同一事务；legacy 路径的同一事务只包含新 Owner、可能缺失的默认设置和 latch finalize，旧实例/设置早已存在且不会被重写。Owner trigger、设置插入或 finalize 失败都会回滚本次写入；legacy 路径失败后旧实例/设置仍保留并继续处于 `LegacyNeedsOwner`。密码哈希在事务外先计算，避免数据库事务在 Argon2 工作期间持锁。

#### 并发的两层防线

- Master 进程内的 `Mutex<Option<Instant>>` 在整个 `initialize` 调用期间持有 guard，同一进程只会同时运行一个 bootstrap 密码哈希/写事务。
- 每个 Master 内的 SQLite pool 强制单连接；不同进程仍依赖 SQLite 文件写锁。PostgreSQL 的条件 no-op `UPDATE` 会锁 singleton latch row。不同 Master 进程/副本即使同时完成哈希，也只能有一个事务把 `pending` 变为 `ready`，其他事务在锁等待成功后得到 `AlreadyInitialized`（异常数据为 inconsistent）；若超过 SQLite busy timeout 或 PostgreSQL lock/statement timeout，则返回 unavailable，但仍不能覆盖首个结果。

数据库 latch 才是跨进程正确性边界；内存 mutex 不是分布式锁。当前 PostgreSQL 多副本仍可能在落到数据库锁之前分别执行一次昂贵哈希，后续需要共享限速与部署拓扑约束。

### 2.4 API：公开 bootstrap port 与 Problem Details

实现位于 [`crates/api/src/lib.rs`](../../crates/api/src/lib.rs)，生成合同位于 [`openapi/nodecontroll-v1.json`](../../openapi/nodecontroll-v1.json)。

| 类型/函数 | 责任 |
|---|---|
| `FoundationProbe` | API 与 Master/persistence 的 object-safe port；定义 readiness、状态读取和初始化命令 |
| `BootstrapCommand` | 把原始字段传给 application adapter；密码与 header 中的 setup token 都是 `Zeroizing<String>` |
| `BootstrapOutcome` | 只返回 instance/owner ID，不返回 PHC 或 recovery secret |
| `BootstrapServiceError` | 稳定 application error 集：字段错误、setup capability 无效、already initialized、identity conflict、inconsistent、rate limited、unavailable；Master 的初始化前置状态读取也保留专用 inconsistent 分支 |
| `BootstrapRequest` | `deny_unknown_fields` 的 JSON DTO；OpenAPI 声明实例名/用户名边界与 pattern，并把密码标为 write-only password、12～1,024 长度的保守合同 |
| `get_bootstrap` | 公开读取严格数据库状态；密码登录端点实现前始终不宣称可用登录方式 |
| `initialize_control_plane` | 从专用 header 读取 setup token，尽快移动/清空 DTO 中的密码，调用 port；成功返回 201 envelope |
| `bootstrap_problem` / `bootstrap_projection_problem` / `validation_problem` | 让 GET/POST 的 inconsistent 状态共享专用 Problem，并把其他错误映射为带 request ID 的稳定 Problem Details；不拼接 SQL、PHC、用户名或密码 |
| `router` | 注册 GET/POST 同路径、method/path trace 与全局 16 KiB body limit；丢弃客户端自带的 `x-request-id`，再生成并传播服务器 UUIDv4 |

HTTP 合同如下：

| 请求 | 当前成功/失败 |
|---|---|
| `GET /api/v1/bootstrap` | 200：`initialized/product/login_methods/setup_capability_required` + `api_version/request_id`；依赖或状态异常为 503 |
| `POST /api/v1/bootstrap` | 必填 `x-nodecontroll-setup-token`；201：`instance_id/owner_id`；字段策略错误 400；capability 缺失、错误、过期或已消费为 403；已完成或初始身份冲突 409；进程内限速 429 并带 `Retry-After: 2`；存储/随机/哈希依赖错误或 inconsistent 状态 503 |

项目声明的 400/403/409/413/415/422/429/503 错误响应在 runtime 和 OpenAPI 都使用 `application/problem+json`；409 描述同时覆盖 already initialized 与初始身份冲突。字段错误带 JSON pointer（`/instance_name`、`/username`、`/password`）。Axum JSON rejection 会收敛为稳定 Problem：语法错误 400、超过全局 16 KiB body limit 为 413、非 JSON media type 为 415、缺字段/未知字段/错误类型为 422；响应不复述 extractor 的原始错误或请求正文。密码业务上限仍是 1,024 UTF-8 bytes。

### 2.5 Master application adapter

实现位于 [`apps/master/src/main.rs`](../../apps/master/src/main.rs)。

`main` 在数据库 migration 后先读取严格 bootstrap 状态。未初始化或 legacy 数据库必须成功读取 setup-token regular file，校验私有权限和 64 位小写十六进制格式，并按 `bootstrap.setup_token_ttl_seconds` 建立短时 capability，之后才 bind HTTP；已经 Ready 的数据库不再要求该文件。`DatabaseProbe::initialize` 先读数据库状态，已经 `Ready` 时直接返回 409；不一致状态为专用 503。未初始化请求必须先通过 capability，再完成实例名、用户名和密码的廉价校验，之后才锁住进程级 mutex、占用 2 秒尝试间隔并执行 Argon2。无 token、错误 token 和字段无效请求不会消耗该间隔。

mutex guard 持有到哈希和数据库提交结束，同一进程不会并行计算两次 bootstrap Argon2。数据库事务成功后 capability 才在内存中消费；失败请求仍可在间隔后重试。成功后的数据库 `ready` latch 跨进程和重启拒绝重复初始化。配置默认 TTL 是 30 分钟、上限 60 分钟；过期后需要重新启动尚未初始化的 Master 来重新建立窗口，不能通过延长客户端请求绕过。429 明确返回 `Retry-After: 2`。

`map_password_error` 只公开映射 `TooShort/TooLong/ControlCharacter → InvalidPassword`；随机源不可用、Argon2 参数/运算错误和存储 PHC 异常统一成为 unavailable。`main` 在 HTTP bind 前完成 migration、secret root-key canary 和 `PasswordService::recommended` 构造，避免服务启动后才发现这些基础依赖不可用。

### 2.6 Vue 3 + Vuetify 与运行时 smoke

页面实现位于 [`apps/web/src/views/SetupPage.vue`](../../apps/web/src/views/SetupPage.vue)，调用由 OpenAPI 生成的 [`sdk.gen.ts`](../../apps/web/src/api/generated/sdk.gen.ts)。

- TanStack Query 调用 `getBootstrapState`；页面不自行猜测初始化状态。当前公开 projection 仍把空库 `Uninitialized` 与历史库 `LegacyNeedsOwner` 都表示成 `initialized=false`，因此页面使用“完成所需初始化写入”的中性文案，不声称每次都会新建实例。
- 未初始化时显示 setup token、实例名、Owner 用户名、密码与本地确认输入；token 必须是 64 位小写十六进制并通过专用 header 发送。成功或失败都立即清空 token、密码和确认字段，避免一次性 capability 与凭据继续驻留在可见表单；成功后再读取服务器状态。
- 已初始化时关闭表单，明确说明重复请求会被 Master 拒绝；结果文案并列说明空库与历史库两种语义，不猜测本次实际走了哪条路径，也明确说明登录/session 尚未启用。
- 页面不再用 UTF-16 `String.length` 或原生 password `minlength/maxlength` 猜测密码策略，而是用 `Array.from` 计 Unicode scalar、用 `TextEncoder` 计 UTF-8 bytes，并拒绝孤立 surrogate 与 Unicode control character；实例名也按 trim 后的 Unicode scalar 计数。button 和 form submit handler 共用同一 `canSubmit`，回车提交不能绕过提示层校验。服务端 domain 校验仍是最终权威边界。
- 错误 UI 只使用 HTTP status、白名单 Problem code 与白名单 JSON pointer；服务端 `detail/title/message/request_id` 不进入 DOM。字段错误使用本地固定文案，403 指向 setup-token 操作，409 自动重新读取状态，429 只显示解析后且不超过 3,600 秒的 `Retry-After`。pointer/code 表使用 `Map`，原型链键不会被当成白名单成员。

[`tools/smoke_master.mjs`](../../tools/smoke_master.mjs) 已定义真实运行时合同：空库 GET 为 false 且要求 capability；四类 JSON/body rejection 分别为稳定 400/413/415/422 Problem；缺 token 的语义有效请求先得到 403，紧接着带正确 token 的请求仍能 201，证明无效 capability 未占用尝试间隔；再次 GET 为 true 且不再要求 capability；重复 POST 返回 `ALREADY_INITIALIZED`。十四个响应都必须带互不重复的服务器 UUIDv4；脚本向首个请求注入客户端自选 ID，并要求响应不能沿用。带 envelope/Problem request ID 的十一个 body 还必须与各自 header 一致；所有响应的 JSON body 与 headers 都不得出现 setup token、四类提交密码材料或 `$argon2id$`。脚本还要求登录端点实现前 `login_methods` 保持空、检查 readiness、runtime OpenAPI 的九个 bootstrap Problem media type、重复 409 与 404 的 `application/problem+json`。VPS verifier 另行扫描 Master runtime log，不允许 token、三种已知密码或 PHC 前缀出现。

## 3. 安全控制与威胁边界

### 3.1 当前已有控制

- Bootstrap 状态投影公开，但写操作必须持有部署者从私有文件读取的短 TTL 一次性 capability；数据库 latch 而非 UI 或单进程 token 状态决定跨副本能否执行。
- 首个写入是单事务，数据库状态组合不合法时 fail closed，不自动删表或猜测修复。
- Argon2 在 `spawn_blocking` 中运行，不阻塞 Tokio async worker；输入和 PHC 参数均有资源上限。
- salt 来自 OS CSPRNG；密码不落明文数据库，项目自身的 bootstrap success/Problem 和统一 JSON extractor rejection 都不回显 password/PHC。正式 runtime secret-scan 仍需由同 SHA Actions 制品的 VPS verifier 留证。
- 进程内串行化 + 2 秒最小间隔降低单进程并发 CPU 放大；16 KiB body limit 限制请求内存。
- username case-insensitive 唯一；principal label 由服务器生成；本次新 bootstrap 生成的 instance、public 和 owner ID 使用 UUIDv7。历史数据库中的 ID 只要求能按对应数据库类型读取，不据此声称也是 v7。
- 显式 method/path tracing、稳定 request ID 和内部错误收敛降低日志意外泄密面。

### 3.2 尚未解决的攻击面

| 风险 | 当前限制 | 后续必须实现 |
|---|---|---|
| setup token 生命周期 | token 只从私有 regular file 读取、进程只存 digest、默认 30 分钟且成功后消费；文件本身不会由 Master 自动删除，重启会为仍未初始化的数据库重开 TTL | 安装器以原子权限创建并在成功后提示操作者安全删除 token 文件；补审计、显式轮换/恢复命令与多副本部署说明 |
| CPU/请求滥用 | capability 先行且无效字段不占槽；仍只有每进程全局 2 秒间隔，无 per-IP/account bucket，重启清零，多副本不共享 | trusted-proxy-aware IP/prefix + normalized-account 分层 limiter、共享/持久策略和指标；登录 limiter 与 bootstrap 分离 |
| 跨站初始化 | 没有 Origin/CSRF 检查；未初始化端点无 session | 明确 bootstrap Origin 策略和浏览器威胁模型；认证后实施 cookie/CSRF/Origin 全合同 |
| 大/畸形 JSON | router 有 16 KiB limit，字段有业务上限；JSON/shape/media/size rejection 已统一为 400/422/415/413 Problem | 继续补 content-length/chunked/JSON depth/duplicate field/UTF-8 边界和真实反向代理回归 |
| 密码质量 | 只有长度、control 和资源上限；固定 Argon2 参数 | VPS 校准、常见/泄漏密码本地拒绝集、参数版本与透明 rehash；不把密码发往外部服务 |
| 敏感内存 | API command 和 salt best-effort zeroize | crash/core dump 禁用、全链路 redaction 测试、检查 allocator/错误路径与前端内存生命周期 |
| 数据库旁路 | DB 只检查 PHC 字符串长度 | 凭据读取始终 domain parse；限制 DB 权限；备份/诊断/审计不得输出 PHC |
| 历史升级恢复 | 能识别 `LegacyNeedsOwner` 和 inconsistent，但没有 operator repair CLI | 提供只读诊断、备份前置与显式 break-glass repair；禁止文档指导手改 latch |

当前 rate limiter 仅保护 bootstrap，不是登录限速，也没有账号枚举 timing bucket。不存在 session cookie，所以现在也没有可声称通过的 session fixation、logout、absolute/idle expiry、revocation、recent-auth、CSRF 或 CORS 安全性质。

## 4. 数据库与迁移测试合同

[`crates/persistence/src/lib.rs`](../../crates/persistence/src/lib.rs) 中的测试没有用“migration 文件存在”代替升级验证：

| 合同 | SQLite 与 PostgreSQL 当前测试步骤 |
|---|---|
| fresh repository | 执行完整 migration；确认 `Uninitialized`；安装 Owner failure trigger，证明失败后 instance/owner/latch 全回滚；移除 trigger 后让两个 bootstrap future 并发竞争，严格要求一个成功、一个 `AlreadyInitialized`、最终只有一个 active owner；再验证设置 actor/revision，重复初始化后重新读取 instance/settings/actor 证明未覆盖；最后删除 latch 并要求专用 inconsistent error |
| 0001→0002 legacy | 用 migrator `run_to(1)` 真实停在 0001；插入旧实例和旧设置；运行完整 migrator；要求状态为 `LegacyNeedsOwner`；先分别把已有设置改成 schema 2 和带未知字段的 JSON，均须返回 inconsistent、owner=0 且 latch 仍 pending；恢复有效设置后再强制 Owner 插入失败，证明旧 instance/settings 不变；最后补 Owner，返回旧 instance ID、旧设置不变、历史 `updated_by` 仍为 `NULL`、状态变为 Ready |
| 0001→0002 legacy 缺设置 | 在独立数据库/schema 停在 0001，插入实例后删掉固定 setting；升级并 bootstrap 后必须补 schema-v1 默认值、actor 为新 Owner、revision=0，并返回旧 instance ID |
| 0002 DDL 原子性 | 停在 0001 后预建冲突的 latch table，迫使 0002 后段失败；要求 migration version 仍为 1 且 `users` 不残留；移除冲突后完整 migration 可重跑并进入 `Uninitialized` |

PostgreSQL 测试要求 `NODECONTROLL_TEST_POSTGRES_URL`；环境变量缺失会 panic，而不是把 PostgreSQL gate 静默标成 skipped。测试在四个独立 schema 中运行 fresh、legacy-existing-setting、legacy-missing-setting 和 migration-rollback 合同并清理。SQLite 对应合同分别使用新的 memory database。触发器只存在于测试 fixture，用于制造“实例已写、Owner 写入失败”的中途错误。

## 5. 已观察验证证据

本轮合同修正前的直接前序工作树曾在 VPS 做过一次不带正式 run-id 的预检，观察到：

- `cargo fmt --all -- --check` 通过；
- `cargo test --workspace --all-targets` 通过，其中真实 PostgreSQL 18 persistence gate 与 SQLite gate 均执行了 fresh、真实 0001→0002 legacy、0002 DDL rollback、业务 rollback 和并发 latch 合同；
- `cargo clippy --workspace --all-targets -- -D warnings` 通过；
- OpenAPI 校验为 4 paths / 5 operations；文档追踪仍为 358 项且相对链接 0 broken；
- 当时的 Web typecheck、lint 和 2 个 Vitest 测试通过；该次预检确实只覆盖 `formatStartedAt`。后续 02:35 预检已经加入 SetupPage 组件测试，见下文。

上述早期预检本身不能证明后来的 inconsistent 映射、OpenAPI media type、Unicode UI、smoke 强断言和新增 repository 断言。其后同一发布前工作树又在 VPS 观察到 workspace Rust tests、SQLite/真实 PostgreSQL contract、Clippy、OpenAPI 导出/SDK 生成、Vue typecheck 与 lint 通过；identity setup-capability 单测为 6 项。02:35 再次观察 Vue typecheck、ESLint zero warning 和完整 Vitest 2 files/13 tests 通过，其中 SetupPage 11 项覆盖 header/body 边界、确认门、成功/失败/网络拒绝清密、403/409/429、字段 pointer、原型键和不可信回显。

这些仍都没有正式 artifact run-id，且未完成“GitHub Actions 编译制品 → VPS 完整 verifier/runtime smoke”的最终链路。本文不声称 bootstrap runtime smoke、Actions release artifact 或提交 provenance 已验收。较早的 VPS run `20260825T155714Z-p5` 曾通过 identity crate 的 2 个密码测试和当时的真实 PostgreSQL identity/bootstrap contract，但它早于后续 API、latch、legacy/DDL rollback、rate/body-limit 加固，只能作为早期基线，不能替代当前工作树的正式 full run。

源码中已存在但仍需在正式完整 run 留证的测试/检查包括：

- `PasswordService` 的 hash→verify、错误密码 false、短密码/control/1,025-byte 拒绝；setup capability 的严格编码、私有文件权限、TTL 非零、正确/错误 token 和成功后消费；
- domain 对完整且有界的 Argon2id PHC、错误算法、超大内存参数和缺失 output 的拒绝；
- API handler 的公开最小 bootstrap 投影、201 ID envelope、稳定 inconsistent Problem 和 OpenAPI Problem media type；这些 unit tests 使用 `TestProbe`，不证明真实 Argon2/SQL transaction；
- Master 单元测试检查前置状态读取保留 inconsistent、其他 persistence error 收敛为 unavailable；真实 HTTP 仍由 smoke 负责；
- persistence 双库 contract 新增 legacy 历史 setting actor 保持 `NULL`、缺失 setting 自动补值、错误 schema/typed JSON fail closed、重复 bootstrap 后 settings/actor 不变、缺失 latch 返回 inconsistent 的断言；
- `tools/smoke_master.mjs` 的真实 SQLite in-memory HTTP capability 拒绝/首次写入、四类 extractor rejection、状态持久、重复 409、十四个服务器 UUIDv4 强唯一、客户端 request ID 不可信、所有响应 token/password/PHC 不回显、runtime log secret scan 和 runtime OpenAPI media type；
- SetupPage component test 已在 VPS 通过 11 项；真实浏览器、反向代理、键盘/无障碍和 Master 联调仍需要后续 Playwright E2E。

正式 run 完成后，应在权威进度文档记录 run-id、commit、Actions run/artifact hash、VPS manifest、真实 PostgreSQL image digest 与 runtime smoke 结果；本文不预填尚不存在的证据。

## 6. 未实现清单与下一退出门

### 6.1 身份与凭据

- 首次 bootstrap recovery codes：生成、只显示一次、hash 存储、确认步骤、原子消费；当前 POST 只返回两个 ID。
- 密码登录与通用外观失败、账号/IP 双限速、Turnstile 可选集成、Argon2 参数校准/rehash。
- 修改/重置密码、禁用用户、软删/恢复、凭据与数据面联动撤销。
- TOTP、WebAuthn、MFA challenge、recovery code 再生成与 recent-auth。
- legacy password verifier/首次登录迁移策略。

### 6.2 会话与 Web 安全

- 至少 256-bit 随机 session、数据库仅存 token hash、idle/absolute expiry、rotation/revoke/logout-all。
- `__Host-` Secure/HttpOnly/SameSite cookie、CSRF token、Origin allowlist、trusted proxy 与真实 client IP。
- `/auth/login`、challenge、logout、`/me/sessions`、profile/security UI 和刷新后 session 恢复。
- 统一 JSON/body extractor Problem、CSP、安全 headers、审计事件与 redaction regression suite。

### 6.3 授权与用户生命周期

- domain/application 层的 `owner/admin/operator/support/auditor/member` 角色模型；当前只有 Owner 可构造。
- `ActorContext`、scope、resource relationship、字段 projection、IDOR 与批量授权语义。
- 用户 CRUD、备注、状态、profile/preferences、service/personal token、CIDR/expiry/rotation。
- 对 Owner 删除/降权的最后 Owner 约束、session/token/订阅/数据面级联策略。

本纵切的下一退出门不是“表中已有 Owner”，而是 [WP-02 完成门](../04-rebuild/IMPLEMENTATION_PLAN.md#6-wp-02身份会话mfa角色与用户基础)：E2E-001、角色/IDOR 矩阵、CSRF/session/token/MFA 安全套件全部通过，并证明 API、日志、OpenAPI example、审计和支持包都不会回显 secret。
