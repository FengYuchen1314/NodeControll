# WP02-C3 持久认证 Challenge 实现

本文记录统一认证 challenge 的本地实现。源码候选最初提交为 `591a1fa13d8f2ab61b9ef5827947d509b6ddeddb`，合入当前本地主线后对应 `5d32d18…`。它建立了 C4 TOTP、C5 WebAuthn 和恢复码登录共同使用的状态机，但尚未接 HTTP 路由，也没有经过 Rust 编译、SQLite/PostgreSQL 运行测试、公开 GitHub Actions 或正式 VPS 验收。

因此，本页描述的是已经落入源码的合同，不是 `verified` 声明。当前公开 `origin/main` 仍是 C1 正式基线，需求追踪矩阵暂不改变。

## 1. 这条切片解决什么

C3 不负责验证具体的 TOTP 数字或 WebAuthn assertion。它负责证明“谁可以开始验证、一次失败计几次、哪一个并发请求有权提交结果、成功后是否必须轮换 session”。具体方法验证器只能在取得 challenge claim 后运行。

本次实现固定了六个此前容易被业务 Handler 各自解释的边界：

1. challenge bearer 是 256-bit CSPRNG 随机值，数据库只存用途隔离 HMAC 和 key version；
2. proof 验证前先持久预留 attempt，四个并发请求只能有一个进入真正验证器；
3. 同一用户、同一 purpose 同时只有一个开放 challenge，耗尽后在原 TTL 内仍占 limiter 槽；
4. proof 方法与 assurance 分开建模，数据库、application evidence 和 verifier 三层都拒绝越级组合；
5. bearer、客户端网络摘要、User-Agent 摘要、用户认证版本和可选 session 生命周期共同决定访问资格；
6. 恢复码等需要 replacement session 的成功结果不能用独立 API 标记完成，只能交给“建新 session、写事件、消费 challenge”同事务端口。

## 2. 文件与模块职责

| 文件 | 主要职责 |
|---|---|
| `crates/domain/src/auth_challenge.rs` | 方法、assurance、purpose、状态、rotation 状态、TTL/attempt policy 和无秘密 challenge 投影 |
| `crates/application/src/auth_challenge.rs` | opaque command/claim/evidence，challenge 服务编排，持久化 port，以及 replacement-session 原子事务 seam |
| `crates/persistence/src/auth_challenge.rs` | SQLite/PostgreSQL 同语义 repository、CAS/lease、状态刷新、行解码和共用双库合同 |
| `crates/persistence/migrations/*/0007_auth_challenges.sql` | `auth_challenges`、允许方法集合、状态/组合约束和开放槽唯一索引 |
| `crates/secrets/src/lib.rs` | C2 已提供的 `AuthChallengeToken`、生成器和 keyring HMAC 验证；C3 只复用，不复制密码学实现 |
| `crates/domain/src/lib.rs`、`crates/application/src/lib.rs`、`crates/persistence/src/lib.rs` | 模块公开与 repository contract 接线 |

## 3. Domain 类型

### 3.1 `AuthenticationMethod`

规范值只有 `password`、`totp`、`webauthn`、`recovery_code`。`parse` 拒绝未知字符串，`as_str` 负责稳定数据库编码。

`permits_assurance` 是第一层防越级矩阵：

| method | 可产生的 assurance |
|---|---|
| `password` | `password` |
| `totp` | `mfa` |
| `webauthn` | `mfa`、`phishing_resistant` |
| `recovery_code` | `recovery` |

WebAuthn 是否真正达到 phishing-resistant 仍由 C5 验证器根据 ceremony 属性决定；这个矩阵只排除某种方法永远不可能产生的标签。

### 3.2 `AuthenticationAssurance`

规范值为 `password`、`mfa`、`phishing_resistant`、`recovery`。它描述证明结果和未来 session 的保证级别，不复用 method 枚举。

### 3.3 `AuthChallengePurpose`

当前 purpose 为 `login`、`reauthenticate`、`sensitive_action`、`credential_enrollment`。开放槽唯一索引按 `(user_id, purpose)` 工作，不把 method 放进唯一键；攻击者不能为同一登录目的反复换 method 重置猜测预算。

### 3.4 状态与 rotation

`AuthChallengeStatus` 包含：

| 状态 | 含义 |
|---|---|
| `pending` | 可预留下一次 proof attempt |
| `verification_pending` | 某一 claim 已独占验证器 lease |
| `rotation_pending` | proof 已验证，但 replacement-session 事务尚未完成 |
| `consumed` | 不需要 rotation 的成功 challenge 已消费 |
| `exhausted` | 最后一个 attempt 已失败或其验证 lease 超时 |
| `expired` | challenge TTL 到期 |
| `invalidated` | 用户、认证版本或绑定 session 已失效 |

`AuthChallengeRotationState` 独立记录 `not_required`、`required`、`pending`、`completed`。这样“证明是否成功”和“会话轮换是否完成”不会压进一个含糊状态。

`AuthChallenge::remaining_attempts` 使用饱和减法，只做投影；是否允许新尝试仍由数据库条件更新决定。`verification_in_progress` 与 `rotation_transaction_in_progress` 分别暴露两层 lease，不回显 claim ID、token HMAC 或客户端摘要。

### 3.5 `AuthChallengePolicy`

policy 同时约束 challenge TTL、单次 verifier/transaction lease 和最大 attempt。构造时拒绝：

- 非正 TTL 或 lease；
- lease 长于总 TTL；
- attempt 为零或超过数据库 `i32` 上限；
- 时间相加溢出或负时间起点。

repository 写 lease 时还会把截止时间截到 challenge 自身 expiry，application policy 不取代数据库最终边界。

## 4. Application 边界

### 4.1 签发与呈现

`IssueAuthChallengeCommand` 带 purpose、用户、可选 session、`auth_revision` snapshot、允许方法、是否要求 rotation、客户端绑定和受控创建时间。

`AuthChallengeService::issue`：

1. 用 policy 计算 expiry；
2. 调 C2 keyring 生成不可 `Clone`、不可 `Debug`、drop 时清零的 bearer；
3. 只把 bearer 的 key version/HMAC 放进 `NewAuthChallenge`；
4. 交给 repository 原子创建；
5. 只有 `Created` 才一次性返回 `IssuedAuthChallenge { challenge, token }`。

同用户/purpose 已有开放槽返回 `AlreadyPending`；用户、认证版本或绑定 session 不可用返回统一 `Unauthorized`。明文 token 从不进入持久化 model 或可克隆结果。

`PresentAuthChallengeCommand` 包含 challenge ID、opaque bearer、可信 HTTP 边界算出的客户端摘要和当前时间。`authorize` 先按 ID、上下文、时间和主体状态只读取得 digest，再用 keyring 常量时间验证 bearer，最后构造仅在 application 内流转的 `AuthChallengeAccess`。

### 4.2 proof attempt claim

`reserve_attempt` 接受调用方观察到的 revision 和准备运行的 method。repository 先做状态刷新，再以单条条件更新完成：

- `pending → verification_pending`；
- `attempts_used += 1`；
- 写随机 claim ID、method、开始/截止时间；
- `revision += 1`。

只有 CAS winner 得到 `AuthChallengeVerificationClaim`。claim 字段私有，HTTP DTO 不能自己拼出 claim ID 或 revision；独立的 `reserved_at_ms` 不会在重试时被新的 access 时间覆盖。loser 得到 `Stale`，不得继续验证 proof。

最后一个 attempt 预留后，`attempts_used == max_attempts` 但仍保持 `verification_pending`。这一 slot 的正确 proof 可以成功；只有错误 proof 或 lease 超时才进入 `exhausted`。

`reject_attempt` 只能提交与 claim ID、method、revision、上下文和未过期 lease 全部相同的失败。非最后 slot 回到 `pending`；最后 slot 进入 `exhausted`。迟到 verifier、复制 claim 或错误 method 都只得到 `Stale`。

### 4.3 typed evidence

`VerifiedAuthChallengeEvidence` 没有 public 任意构造器。C4/C5 等同 crate 方法验证器取得 claim 后，才可调用 crate-private `from_method_verifier`；该函数再次执行 method/assurance 矩阵。

`accept_verified_method` 在提交前第三次检查矩阵，然后按 claim CAS：

- 无需 rotation：写 verified method/assurance/consumed time，进入 `consumed`；
- 需要 rotation：写 verified method/assurance，进入 `rotation_pending`，不写 consumed time。

持久层还在 SQL CHECK 中编码同一矩阵。即使未来 application 出错，数据库也不能保存 `totp + phishing_resistant` 或 `recovery_code + mfa`。

### 4.4 replacement-session transaction claim

proof 成功后不能直接公开 `complete_rotation`。application 会继续调用 `reserve_rotation_claim`，为 `rotation_pending` 行取得第二层持久 lease，并返回 `AuthChallengeRotationTransactionClaim`。它只向 infrastructure 暴露：

- 已授权 access；
- 随机 transaction claim ID；
- expected revision；
- 无秘密 challenge 投影。

`AuthChallengeRotationTransactionPort::replace_session_and_consume_atomically` 是唯一完成 seam。实现者必须在一个数据库事务内创建 replacement session、写安全事件，并以 claim ID/revision 消费 challenge。C3 故意没有为 `Database` 实现一个可脱离 session 创建的完成方法。

若进程在 proof 已提交后退出，数据库会保留 `rotation_pending`。调用方重新读取当前投影后，可用 bearer、相同上下文和观察到的 revision 调 `resume_rotation`。并发 resume 通过 CAS 只有一个得到 transaction claim；进程在第二层 lease 内再次退出时，lease 到期只清 handoff claim 并推进 revision，proof 仍保持已验证，之后可以再次恢复。

## 5. Persistence 操作

### 5.1 创建与只读授权

`create_auth_challenge` 在事务内先刷新同 user/purpose 的过期、主体失效和超时 lease，再检查 active user、精确 `auth_revision` 和可选 active session。部分唯一索引处理真正并发；插入成功后再写去重、规范排序的允许方法集合。

`auth_challenge_token_digest` 只返回 `KeyedDigest`，不返回 challenge 状态或 HMAC 字节的可序列化包装。查询同时匹配客户端上下文、开放状态、创建/到期时间、active user/auth revision 和可选 session 时间线。

`auth_challenge` 使用已授权 access，先在事务内刷新状态，再读取无秘密投影和规范方法集合。decoder 拒绝未知枚举、重复/非规范方法、越界 attempt、claim/status 不一致和 method/assurance 错配。

### 5.2 状态刷新

`refresh_access_sqlite/postgres` 对单个 bearer 绑定行工作；`refresh_user_purpose_sqlite/postgres` 在签发新 challenge 前处理该用户和 purpose 的旧行。顺序固定为：

1. 总 TTL 到期：开放状态转 `expired`；
2. 用户、认证版本或 session 无效：转 `invalidated`；
3. proof lease 到期：没有 method-specific terminal handoff 时清 claim，未耗尽回 `pending`，已耗尽进 `exhausted`；exact TOTP terminal 可把同一 claim 固定到总 challenge/session/auth 生命周期，供崩溃恢复；
4. rotation handoff lease 到期：只清 transaction claim，保持 `rotation_pending`。

每次自动刷新都推进 revision，并把 `updated_at_ms` 写成调用方受控当前时间。旧 claim 因 revision 不再匹配，不能在刷新后迟到提交。

### 5.3 CAS 写操作

| repository 方法 | 原子效果 |
|---|---|
| `reserve_auth_challenge_attempt` | 预占一次猜测预算和唯一 verifier lease |
| `resume_auth_challenge_attempt` | 以原 bearer/context 恢复 exact claim；普通 claim 仍受 verifier lease 限制，method terminal 可跨短 lease |
| `record_auth_challenge_failure` | 只让 claim owner 提交失败，决定 retryable/exhausted |
| `begin_auth_challenge_consumption` | 只让 claim owner提交 typed 成功，进入 consumed/rotation_pending |
| `reserve_auth_challenge_rotation` | 为 replacement-session 事务预占可恢复 handoff lease |

四个写入口都重新匹配 token digest、三元客户端上下文、revision、状态、claim/method、时间边界、active user/auth revision 和可选 session。application 的检查不是 repository 条件的替代品。

## 6. 数据库结构与不变量

`auth_challenges` 保存 token digest、主体、purpose、状态、两层 lease 共用的 claim 字段、验证结果、客户端摘要和 revision。`auth_challenge_methods` 以 `(challenge_id, method)` 为主键保存允许方法。

两库共享以下约束：

- token HMAC、网络 HMAC、UA hash 必须是 32 bytes；
- context 只能全空或 key version/network/UA 三者全有；
- `attempts_used` 始终在 `0..=max_attempts`；
- claim ID、attempt method、开始/截止时间必须按状态整组出现或整组为空；
- verified method 与 assurance 必须同时有或同时无，且满足固定矩阵；
- consumed time 只在最终 consumed 状态出现；
- 每个状态允许的 rotation、attempt、claim 和验证字段组合由一个总 CHECK 封闭。

SQLite 额外检查整数/文本/BLOB 的动态类型，并要求 ID 是规范小写、带连字符的 UUID 文本；PostgreSQL 使用 native UUID、BYTEA 和 BIGINT/INTEGER。两份 migration 的状态枚举、矩阵和索引语义保持同态。

`auth_challenges_user_purpose_open_uq` 只覆盖 `pending`、`verification_pending`、`rotation_pending`、`exhausted`。`exhausted` 故意保持到原 expiry 才释放唯一槽，避免客户端立刻签发新 bearer 重置 attempt budget；`consumed`、`expired` 和 `invalidated` 不再占槽。

## 7. 已写测试合同

Domain 测试覆盖规范词汇、状态闭集、policy 边界/溢出和完整 method/assurance 矩阵。Application fake-port 测试覆盖：

- bearer 只以 digest 进入持久 model；
- reserve claim 与 typed evidence 的传递；
- 错配 assurance 无法构造 evidence；
- 并发 crash resume 只产生一个 rotation transaction claim。

Persistence 的同一个 `auth_challenge_contract` 会由 SQLite 和真实 PostgreSQL repository suite 共同调用，源码覆盖：

- 四路同 revision proof reservation 只有一个 winner；
- 最后 attempt 正确 proof 可以成功，错误或超时才 exhausted；
- proof lease 崩溃恢复与旧 claim 失效；
- 四路 rotation resume 只有一个 winner，handoff lease 到期可再次恢复；
- 同 user/purpose 并发签发、expired replacement 和 exhausted 再签发阻断；
- 错误 context 在 token lookup、load 和 reserve 各层都失败；
- auth revision 变化、session idle expiry 和 session revoke 自动 invalidation；
- 原始 SQL 写入破坏 context/matrix/state 约束时由数据库拒绝。

这些测试目前只是源码。只有固定 VPS 上的 `cargo fmt --check`、workspace all-target tests、真实 PostgreSQL、Clippy `-D warnings` 和后续合并态门全部通过后，才能把本节改为实际证据。

## 8. 尚未完成的集成

- C4 已提供 TOTP enrollment/verification 与 durable terminal handoff；尚待 HTTP transport 把原 bearer/context resume 接到真实请求生命周期；
- C5 需要提供 WebAuthn ceremony、credential counter 与真实 assurance 判断；
- 恢复码登录要把 C2 单次消费接到 C3 claim，成功后走 rotation transaction seam；
- C6 要把 challenge issue/verify 接入登录、近期认证、凭据 enrollment 和全部高危 use case；
- API/OpenAPI、生成 SDK、Vue challenge 页面和真实浏览器多标签页流程尚未实现；
- replacement-session transaction port 只有合同，没有具体基础设施实现；实现时必须复用现有 session 锁序和事件语义。

在这些接线和验收完成前，C3 只是后续认证方法的可靠内核，不是用户可见功能。
