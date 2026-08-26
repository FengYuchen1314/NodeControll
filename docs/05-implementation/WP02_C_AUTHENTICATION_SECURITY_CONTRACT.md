# WP02-C 认证安全合同

## 1. 文档状态

本文冻结 WP02-C 的实现语义，解决总设计里尚未钉死的近期认证、密码轮换、TOTP、恢复码和 WebAuthn 边界。它不是完成声明。每个纵向切片只有在公开提交、GitHub Actions 和指定 VPS 的 SQLite/PostgreSQL 共用合同全部通过后，才能在进度表中标记完成。

当前顺序如下：

1. C1：密码近期认证、透明 Argon2id 升级、自助改密码、会话查看/撤销/全退出；
2. C2：持久化 root-key canary/keyring、typed secret record、恢复码组和 bootstrap 一次性回显；
3. C3：统一认证 challenge；
4. C4：TOTP；
5. C5：WebAuthn；
6. C6：把近期认证 guard 接到所有高危 use case；
7. C7：真实浏览器 E2E、并发、故障注入和完整安全证据。

完成 C1 不等于完成 WP02-C，更不等于完成 WP02。

C1 的历史 v4 应用代码、v6 门工具候选和真实双页 HTTPS candidate 已通过各自 VPS 测试；v6 之后形成的“正式编译只在公开 Actions、VPS 只测试和运行验收同 SHA 制品”边界尚待最终 freeze。公开单父提交、Actions 同 SHA 制品与 fresh-clone formal provenance 同样尚未完成，因此这里仍不把 C1 标成正式完成。

## 2. 不再留给实现临时决定的事项

### 2.1 bootstrap 与首位 owner

空实例没有既有身份，无法要求用户先用 WebAuthn、TOTP 或密码重新认证。`POST /bootstrap` 因此由本机生成、短时有效、一次性消费的 setup capability 授权，是唯一初始化例外。它必须在一个事务里创建实例、首位 owner 和首组恢复码；任何一步失败都不得留下半初始化状态。

实例建立后，新增 owner、把其他用户晋升为 owner、转移 owner 身份或修改 owner 的安全凭据，都必须经过服务端近期认证检查。公开网络上不得存在“官方授权”“官方域名数据”或远程后门作为替代路径。

### 2.2 认证方法与保证级别

认证方法和会话保证级别分开建模：

- 方法：`password`、`totp`、`webauthn`、`recovery_code`；
- 保证级别：`password`、`mfa`、`phishing_resistant`、`recovery`。

数据库当前的 `auth_level` 暂时承载保证级别。C3 引入 challenge 时补独立的 `authentication_method`，不再用一个枚举同时表达“用了什么”和“可信到什么程度”。恢复码只能作为已经通过密码阶段后的第二因素后备；它不是默认的忘记密码入口，也不能绕开账户枚举防护。

### 2.3 两类 rotation

近期认证和密码修改使用两种不同事务，禁止复用同一个“全退出”快捷路径。

| 事件 | 全局 `auth_revision` | 受影响会话 | 新会话保证级别 | absolute expiry |
|---|---:|---|---|---|
| 密码近期认证/后续 MFA step-up | 不变 | 只撤销当前会话 | 不得低于原会话；由实际证明更新 | 继承原截止时间，不延长 |
| 修改或重置密码 | 加一 | 撤销该用户全部活动会话 | 当前浏览器获得唯一 replacement | 继承原当前会话截止时间，不延长 |
| logout-all，`keep_current=true` | 加一 | 撤销全部旧会话 | 当前浏览器获得唯一 replacement | 不延长 |
| logout-all，`keep_current=false` | 加一 | 撤销全部会话 | 无 | 不适用 |

每次 rotation 同时更换 session token 和 CSRF token。数据库只保存用途隔离、带 key version 的 HMAC。事务提交前不得发 Cookie；提交后旧 token 立即失效，没有 grace period。客户端没收到响应时只能重新登录，不能让旧 token 短时复活，也不能仅凭 `/me` 返回了不同 session ID 就推断目标 mutation 已提交。

系统墙钟回拨不能阻止撤销。`revoked_at` 写入 `max(created_at, now)`；一般认证、活动列表和 rotation 当前会话选择器在 `now < last_seen_at` 时都 fail closed。反复 step-up、改密码或保留当前会话都不能借机延长 absolute expiry。

## 3. C1 合同

### 3.1 密码验证与透明 rehash

Argon2 工作在 `password_hash_concurrency` 的共享许可内。请求必须先取得许可，再读取/写入共享 limiter；许可覆盖完整 blocking 工作，包括 current-policy 校准、所选凭据验证、可选成功 rehash 和 deadline padding，闭包结束后才释放。不存在账号仍选择固定 current-policy dummy PHC；登录响应不透露账号、停用状态或 MFA 状态。

每次三层 limiter 预留都同时提交一枚候选 `rate_limited` 事件，候选与 reservation 必须共享时间、request ID、账号/IP HMAC、UA hash 和 key version。repository 在任何 bucket 写入前逐字段核对；account、IP 或 global 只有首次进入有效 block 的事务才能写一条事件。已有 block 的只读 preflight、并发 follower、非 transition 429 和进程内 Argon2 semaphore 饱和都不得产生 durable 事件。通用安全事件入口必须拒绝 `rate_limited`，事件插入失败必须让同一事务内的 bucket 计数与 block 一起回滚。`now < blocked_until_ms` 仍封禁；`now == blocked_until_ms` 重新开放，后续再次真正越过阈值才写下一周期事件。

密码登录的外部可见工作计划固定为三步：先用 dummy PHC 校准一次当前策略成本，再验证真实或 dummy 凭据，最后补齐以实测成本计算的共同 deadline。未知账号、当前 PHC 和资源有界的低成本旧 PHC 不能选择不同 plan 或 timing bucket。超出输入上界的值改用固定受限字符串完成相同两次 Argon2 和 padding，最终强制失败；密码库错误也只能在完整计划结束后返回。接受的旧 PHC 上限不得高于当前策略，防止数据库内容制造不受控 CPU/内存消耗。

成功验证旧 PHC 后，如果算法版本、内存、迭代、并行度或输出长度不等于当前 policy，应用在同一 blocking 工作内生成新 PHC。数据库登录事务重新检查：

- 用户 ID、活动状态和未删除状态；
- `auth_revision`；
- `password_changed_at_ms`；
- 预期旧 PHC 和用户 revision。

只有旧密码状态仍是同一个 snapshot 时才升级 PHC。透明 rehash 不修改 `password_changed_at_ms`，不推进 `auth_revision`，不撤销既有会话，也不伪装成用户主动改密码。并发登录已先完成相同升级时，可以跳过重复写入并继续创建 session；并发改密码或重置导致认证版本/改密时间变化时，本次登录不得创建 session。rehash、session、安全事件和账号 limiter 清理必须在同一事务中提交。

C1 的透明 rehash 只发生在正常密码登录；密码 reauth 只验证 proof，不顺带维护 PHC。未来若扩展到 reauth，必须复用同等的 snapshot CAS、事件和失败回滚合同，不能做成独立的尽力而为更新。

### 3.2 近期认证

默认 freshness 为 300 秒，可在 60～3600 秒内配置，且不得长于 absolute session lifetime。服务端使用 session 的 `recent_auth_at_ms` 判定；浏览器拿到的 `recent_auth_expires_at_ms` 只用于界面预判，不是授权证据。`now < recent_auth_at_ms` 视为无效，不能用饱和减法把回拨时间误判为“刚认证”。

C1 的 `POST /api/v1/auth/reauth` 使用带方法判别的请求：

```json
{"method":"password","password":"..."}
```

它要求活动 session、同源 Origin、严格 Host、double-submit CSRF 和共享密码限流。证明失败返回 `403 REAUTHENTICATION_FAILED`，保留旧 session；成功则在一个事务中撤销当前 session、写安全事件、清账号 limiter，并创建新 session/CSRF。其他设备不受影响。后续 TOTP/WebAuthn 通过同一 method-neutral 状态机扩展，不能让浏览器自动重放原高危 mutation。

### 3.3 自助改密码

`POST /api/v1/me/password` 的 body 只有 `new_password`；确认字段只存在浏览器内，不发送到 API。端点先验证当前 session、Origin/CSRF 和 freshness，再在有界 Argon2 工作中检查新密码、拒绝与当前密码相同的值并生成新 PHC。

数据库事务固定完成：

1. 重验活动当前会话、用户状态、认证版本和旧密码 snapshot；
2. 更新 PHC，清除 `force_password_change`，推进用户 revision；
3. 单调更新 `password_changed_at_ms` 和 `updated_at_ms`；
4. 推进全局 `auth_revision`；
5. 以 `password_changed` 撤销该用户全部活动会话；
6. 插入当前浏览器的唯一 replacement session 和新 CSRF；
7. 写不含密码/PHC/token 的安全事件并清账号 limiter。

任一步失败必须完整回滚。并发两次改密码只能有一个提交；失败请求在 winner 提交后得到失效 session，不能覆盖新密码。

### 3.4 会话 API

- `GET /api/v1/me/sessions` 只返回活动会话的粗粒度投影：ID、是否当前、保证级别、创建/最后使用/idle/absolute/recent-auth 截止时间。不得返回 token、HMAC、原始 User-Agent 或完整 IP。
- `DELETE /api/v1/me/sessions/{id}` 要求近期认证且只作用于当前用户。调用 session 的有效性复核、目标撤销和安全事件必须在同一数据库事务；复核包含 session ID/状态/期限、用户 active/未删除、user/auth revision 和 recent-auth snapshot，但不比较普通 touch 会推进的 session revision。调用者仍有效时，未知或已经撤销的目标是 204；调用方并发 rotation/revoke、用户或认证状态变化、时间线回拨必须拒绝且不改目标、不写事件。如果首次撤销的是调用者当前 session，响应清两枚 Cookie，随后用这枚已撤销凭据重放会先得到 `401 SESSION_INVALID`。客户端在请求前冻结目标是否为当前 session；若无法确认这次 current-target mutation 的结果，必须先关闭本地身份，再用最新 CSRF 尝试一次幂等 logout，不得继续显示受保护 DOM，也不得重放 DELETE。非当前目标的未知结果不应撤销仍有效的当前登录。
- 客户端所有身份快照替换都必须推进单调 generation。refresh、reauth、改密和 keep-current rotation 只能接纳请求起点 generation 仍匹配的 projection。login 在等待 exclusive lease 前记录起点；取得 lease 后，自身 inflight 写入导致的精确 `+1` 是唯一正常推进，随后以该 `loginGeneration` 校验响应。等待期间若还有外部推进，即使返回 401/429，也必须保留 sticky quarantine；只有结构完整、运行时校验通过且 generation 匹配的 200 登录投影可以恢复。logout、session-invalid、未知 mutation cleanup 或另一个已接纳身份一旦推进 generation，旧 200 不得重新打开身份。晚到的 rotation success 仍可能改变 Cookie，因此不能只丢弃 body；它必须按结果未知再次执行 fail-safe logout。普通 logout 与 fail-safe helper 都要在第一个网络 await 之前关闭受保护 DOM。
- `POST /api/v1/auth/logout-all` 必须显式提交 `keep_current`。`true` 返回 replacement 投影和撤销数；`false` 返回 204 并清 Cookie。两种模式都需要近期认证，且安全事件与状态变化在同一事务。

PostgreSQL 的同用户稳态认证写统一使用 `user_auth_state → users → auth_sessions` 的行锁顺序。touch 不得先锁 session 再取得 auth-state；它可以先无锁读取 token 对应的候选 user ID，但取得 auth-state barrier 和用户锁后必须重新执行完整 token、用户、auth revision、CSRF 与时间线校验，再锁 session。actor-aware DELETE、登录/透明 rehash、rotation、改密和两种 logout-all 都服从同一顺序。只锁 session 的普通 logout 之后不得再取得 auth-state 或用户锁。

### 3.5 强制改密

`force_password_change=true` 时，后端 use-case allowlist、router guard 和 App DOM gate 三层都只允许：读取自身会话、近期认证、修改密码、查看/撤销自身会话和退出。不能只隐藏导航按钮。改密成功的 replacement projection 必须返回 `force_password_change=false`；浏览器先原子接受新 actor/session、清空密码 DOM，再跳转到经过净化的原路径。导航失败只能重试导航，不能再次提交密码。

当前 capability 投影里的 `credentials:manage` 只是 C1 粗粒度 UI 能力，不能自动授权未来 TOTP、WebAuthn 或恢复码管理。C6 接入真实产品 use case 时，每个受保护入口都必须声明 action；正常、未被强制改密的 `ProductAccess` 还必须保持既定 idle-touch 语义。

### 3.6 同源标签页与 Cookie 协调

session/CSRF Cookie 是同一 origin 的共享可变状态，Pinia 却是每个 browsing context 的局部状态。客户端因此必须满足以下合同：

- 只允许一个 localStorage 键 `nodecontroll:credential-coordination:v1`。值是严格校验的非秘密 journal，包含协议版本、随机 epoch/op/sender ID、规范十进制 `baseSeq/seq`、operation、phase 与 disposition；不得包含 token、CSRF、密码、actor、session 投影或用户资料，`sessionStorage` 始终为空。
- Web Locks 的稳定锁名覆盖整个凭据请求，包括 SDK 读取响应体、运行时 shape 校验与本地状态提交。rotation/login/logout/改密/撤销等用 exclusive；`/me` 与 session 列表用 shared。拿不到锁、锁超时或协调能力缺失都 fail closed。
- exclusive mutation 必须先把 `inflight/quarantine` 持久化、回读并本地分发，再发送网络请求；释放锁前必须持久化 `settled/reconcile` 或 sticky `settled/quarantine`。每条持久写恰好推进一个 revision，terminal 必须引用它实际观察到的 inflight。
- BroadcastChannel 和 storage event 只是提示；标签页恢复可见时重新读取权威 journal。损坏、revision 回滚或同值篡改、跳号、未观察到的 terminal/epoch 替换、crash 后遗留 inflight 或未知结果都关闭 actor/session、会话列表和受保护 DOM。journal 缺失时，fresh setup/anonymous 且没有 CSRF Cookie 是合法初始态；已有 CSRF Cookie，或当前状态为 authenticated、unavailable、relogin-required 时必须隔离。只有显式登录成功，或得到权威 204 的显式清理操作，才能建立新 epoch 或解除 quarantine。
- 共享读得到 401 时，先发布绑定“所观察 session ID + base epoch/seq”的瞬时失效消息；释放 shared 后再取 exclusive，并且只有 cursor 仍相同才持久化 `invalidated`。旧标签页迟到的 401/消息不能覆盖新登录或 rotation。
- 任意 Problem 响应都不得包含 `Set-Cookie`。成功响应是否设置/轮换/清除 Cookie 由路由的显式 allowlist 决定；无效旧凭据的通用 401 只拒绝该请求，不能清掉此刻可能已经属于更新会话的浏览器 Cookie。

Web 平台无法让一个标签页同步修改另一个标签页的 DOM。若后者正被数秒的同步 JavaScript 长任务占满，它只能在事件循环恢复并消费 storage/BroadcastChannel 后关闭旧画面；阻塞期间它同样不能处理用户交互或发出请求，Cookie 请求仍受 Web Lock 排他。这个 P2 平台边界不能通过等待所有标签页 ACK 消除，因为冻结、崩溃或休眠标签页可能永不确认。

## 4. C2～C5 的固定边界

### 4.1 secret 与恢复码

现有 `secret_records` 物理表不等于凭据安全已经完成。C2 需要 typed repository、业务 owner、purpose/schema/key-version AAD、持久化 canary 和有限旧 key ring。错误但格式合法的 root key 必须在启动时被持久 canary 拒绝。

恢复码以 code set 管理。每次生成 8 个、每个至少 128 bit 随机熵；显示格式允许分组连字符，服务端只做明确的大小写/分隔符规范化。数据库保存用途隔离 HMAC、key version、set version、创建和消费时间，不保存明文。再生成整组时旧组在同一事务失效；同一码并发消费只能一次成功。明文只在 bootstrap 或再生成响应出现一次，响应 `Cache-Control: no-store`，GET 永不恢复。

### 4.2 challenge

C3 的 `auth_challenges` 至少绑定：opaque token HMAC、purpose、用户、可选当前 session、`auth_revision` snapshot、允许方法、过期时间、attempt/max、状态、消费时间、客户端网络/UA 摘要和 revision。密码登录需要 MFA 时不创建 provisional browser session；只有 challenge 最终成功才签发正式 session。challenge 失败不能 touch 旧 session，也不能通过不断新建 challenge 绕过共享 account/IP/global 限流。

### 4.3 TOTP

C4 固定使用 6 位、30 秒、HMAC-SHA-1 的广泛兼容配置；只接受当前 step 前后各一个 step，服务端以受控 UTC 时间计算。enrollment secret 用专用 purpose/AAD 加密，状态从 pending 开始，只有首次 code 验证成功才激活并生成恢复码。`last_accepted_step` 必须用条件更新原子推进，同一时间步顺序或并发重放只能一次成功。时钟倒退不能重新接受已经记录的 step。

### 4.4 WebAuthn

C5 支持一个用户多凭据。credential 记录至少包含 credential ID、COSE public key、user handle、sign counter、AAGUID、transports、UV、backup eligibility/state、昵称、状态、创建/最后使用时间和 revision。registration/authentication ceremony 精确绑定 public origin 推导的 RP ID、Origin、challenge、用户/session、purpose 和 TTL。默认要求 user verification；attestation 默认 `none`，不建立官方元数据依赖。逐凭据重命名/撤销使用资源 ID 和 revision，不用含糊的集合级 DELETE。

## 5. 稳定 Problem 语义

| code | HTTP | 状态变化 |
|---|---:|---|
| `INVALID_CREDENTIALS` | 401 | 登录未建立 session |
| `SESSION_INVALID` | 401 | Problem 零 `Set-Cookie`；只拒绝该请求，客户端按所观察 cursor 失效本地投影，不能清除可能已被其他标签页轮换的新 Cookie |
| `REAUTHENTICATION_FAILED` | 403 | 保留旧 session |
| `RECENT_AUTH_REQUIRED` | 403 | 目标 mutation 未执行 |
| `PASSWORD_CHANGE_REQUIRED` | 403 | 目标受保护 use case 未执行；当前 session 仅保留强制改密 allowlist |
| `CSRF_INVALID` / `BROWSER_ORIGIN_INVALID` | 403 | mutation 未执行 |
| `PASSWORD_POLICY_REJECTED` / `PASSWORD_UNCHANGED` | 422 | mutation 未执行 |
| `LOGIN_RATE_LIMITED` | 429 | 带 bounded `Retry-After` |
| `AUTHENTICATION_UNAVAILABLE` | 503 | mutation 客户端不得据此断言未提交；rotation 按结果未知、fail-safe logout 和重新登录处理 |

challenge/TOTP/WebAuthn 后续增加 `AUTH_CHALLENGE_INVALID`、`AUTH_CHALLENGE_STALE` 和方法专用的本地可映射错误；响应不带 SQL、密码学库错误或账号存在性信息。

## 6. VPS 验收门

C1 的同一套 repository contract 必须分别在 SQLite 和真实 PostgreSQL 跑过：

- 空库和从 0003 升级 migration；CHECK、索引和事件枚举等价；SQLite/PostgreSQL 的 0004、0005 各自用晚阶段失败夹具证明 version、行、索引、表定义/约束和 validated 状态完整回滚，再从明确的进程重启边界重试成功；
- 当前 session rotation：旧 token/CSRF 立即失效、sibling 保留、absolute 不延长；
- 改密码：全部旧 session 失效、唯一 replacement、PHC/force flag/auth revision 正确；
- 透明 rehash：弱/当前参数、并发 rehash、与改密码竞态、写失败回滚；
- logout-all 的 keep/clear 两种事务和安全事件；
- actor-aware 逐会话撤销：旧 snapshot 后普通 touch 成功；actor rotation/revoke、认证版本推进、用户停用/变更和墙钟回拨拒绝且不写目标/事件；
- PostgreSQL 同用户 A/B session 互删与 touch/DELETE、touch/rotation 交错：不得 deadlock 或 revision conflict，终态和事件数必须满足单一线性化顺序；
- 三层 limiter：四并发只产生一次首次封禁事件，已有 block/follower 零写，候选字段不匹配在任何写前拒绝，事件冲突使 bucket 一起回滚，`blocked_until_ms` 边界产生下一周期事件；
- 未知账号、当前 PHC、低成本旧 PHC 的固定登录工作计划，过长输入和错误延后；
- 墙钟回拨撤销、错误 event、重复请求、revision 饱和和并发 winner；
- OpenAPI 与生成 TypeScript 客户端完全同步；
- Vue typecheck、lint、组件/路由/store 测试；
- 真实双页 HTTPS 浏览器抓取旧 Cookie，证明 rotation 后旧凭据 401 且零 `Set-Cookie`、新凭据可用；注入 logout 503 后两页关闭受保护 DOM 并跨 reload 保持 quarantine，显式登录恢复两页，绑定旧 cursor 的迟到 invalidation 不得覆盖新状态，最终权威 logout 204 才清 Cookie；
- 日志、数据库 dump、OpenAPI、构建目录、测试 artifact 和浏览器 storage/URL 的 secret scan。

所有编译和测试仍只在维护者私有配置指定的 VPS 一次性、可追溯 run root 内执行。验收主机地址和 SSH 身份不写入公开仓库；本机只允许源码编辑、格式化、diff 和 Git 元数据操作。
