# WP02-C4 TOTP 核心实现

本文记录 WP02-C4 的 TOTP domain、秘密材料、application 编排和 SQLite/PostgreSQL 事务实现。它建立在 C2 持久 keyring/恢复码与 C3 持久 challenge 之上。当前切片没有 HTTP 路由、OpenAPI、Vue enrollment 页面，也不是面向最终用户的完成声明；这些边界接通并通过同一公开提交的正式 Actions/VPS 门后，需求追踪状态才能改变。

## 1. 固定的安全轮廓

C4 将服务端 TOTP profile 收敛为一个不可由 Handler 改写的集合：

| 属性 | 固定值 |
|---|---|
| HMAC | SHA-1，兼容 RFC 4226/6238 authenticator profile |
| seed | 20 bytes CSPRNG |
| code | 6 个 ASCII 十进制数字 |
| period | 30 秒 |
| 验证窗口 | 当前 step 的前后各 1 个 step，即 `±1` |
| pending enrollment TTL | 30 秒至 1 小时，由 `TotpEnrollmentPolicy` 约束 |
| replay | 用户 active credential 的 `last_accepted_step` 单调递增 |

接受 `±1` 只处理设备与服务器之间的小幅时钟偏差，不表示可以回放。一次 step 成功后，旧 step 和相同 step 都不能再次推进持久状态。

## 2. 文件和模块职责

| 文件 | 职责 |
|---|---|
| `crates/domain/src/totp.rs` | 状态闭集、无秘密 credential 投影、固定 profile 与 enrollment TTL |
| `crates/secrets/src/lib.rs` | `TotpSeed`/`TotpCode` 的零化容器、CSPRNG、RFC 计算、窗口验证、keyring 加解密 |
| `crates/application/src/totp.rs` | 受控管理身份绑定、begin/activate/verify/disable 编排、C3 typed evidence 交接 |
| `crates/persistence/src/totp.rs` | 双库 repository、事务锁序、CAS replay、恢复码/auth/session 原子联动和共用合同 |
| `crates/persistence/migrations/*/0008_totp_credentials.sql` | credential 表、状态约束、pending/active 唯一索引及 TOTP seed coexistence 规则 |
| `crates/*/src/lib.rs` | 公开无秘密类型和受控 application/repository 边界 |

## 3. Domain 合同

### 3.1 `TotpCredentialStatus` 与 `TotpCredential`

状态只有 `pending`、`active`、`disabled`。`TotpCredential` 是可记录、可比较的无秘密投影，包含 credential ID、用户、secret record ID、状态时间、最后接受 step 和 revision；它不包含 seed、envelope、恢复码或明文 proof。

`TotpCredential::is_pending_at` 同时要求状态为 pending、时间非负且严格早于 expiry。到期边界使用半开区间，`now == pending_expires_at_ms` 已失效。

### 3.2 `TotpEnrollmentPolicy`

`new` 拒绝小于 30 秒或大于 1 小时的 pending TTL。`expires_at_ms` 拒绝负时间并检查整数溢出。application 计算 expiry 后，persistence 仍会验证时间字段；上层 policy 不是数据库条件的替代。

## 4. 秘密生命周期和算法

### 4.1 `TotpSeed` 与 `TotpCode`

`TotpSeed` 固定持有 20 bytes `Zeroizing` 数组；生成使用操作系统 CSPRNG。`TotpCode` 解析时要求精确 6 bytes 且全部为 ASCII digit，并以 `Zeroizing` 保存。二者都不实现 `Clone` 或 `Debug`，因此不会自然进入可克隆 DTO、结构化日志或调试输出。

`verify_totp_at_utc_ms`：

1. 拒绝负 UTC 毫秒；
2. 以 30 秒换算当前 counter；
3. 只计算前一、当前、后一 step，counter 为零时不下溢；
4. 常量时间比较 6 位结果；
5. 若候选 step 不大于 `last_accepted_step`，即使数字匹配也拒绝；
6. 返回实际命中的 step，而不是布尔值，交给 repository 做 CAS。

测试使用 RFC 4226/6238 向量验证截断与十进制格式，并覆盖相邻窗口、历史 step 拒绝和格式闭集。

### 4.2 keyring 与 AAD

seed 通过 C2 `Keyring::encrypt_totp_seed` 写为 XChaCha20-Poly1305 envelope。AAD 固定绑定：

- purpose `totp_seed`；
- owner kind `user`；
- owner ID；
- schema version `1`；
- envelope 自身 key version。

解密按记录中的 key version 选择 current/old key。换用户、换用途、换 schema、缺失旧 key或篡改 ciphertext 均失败。数据库只保存 envelope 和无秘密 credential 外键；服务完成验证后，局部 seed/code 按作用域 drop 并清零。

### 4.3 一次性明文结果

`BegunTotpEnrollment` 持有 seed，但不实现 `Clone`/`Debug`；未来 transport 必须把它转换为 base32/otpauth 后立即清理。`ActivatedTotpCredential` 同样只在一次性、不可调试结果中持有 8 个新恢复码。activation 事务成功以后无法从数据库还原这些明文；若响应丢失，已认证用户只能走恢复码重新生成流程。

## 5. 受控 application 边界

### 5.1 `TotpManagementBinding`

字段保持私有，也没有逐字段 public constructor。唯一外部 factory 是 `TotpManagementBinding::from_authenticated_session(&AuthenticatedSession)`：

- 输入必须来自现有 session authentication 投影；
- `force_password_change=true` 明确返回 `PasswordChangeRequired`；
- revoked/non-active projection 返回 `InvalidSession`；
- 成功时只捕获 user ID、actor session ID、user revision、auth revision 和精确 `recent_auth_at_ms`。

factory 不把快照当最终授权。`management_now` 再检查近期认证窗口和时钟回退；每个 repository 写事务还会锁定并重新读取 user/session，要求 active user、active session、未 revoke、未到 idle/absolute expiry、user/auth revision 与精确 recent-auth timestamp 全部一致。因此 factory 与数据库 revalidation 是两层不同职责，调用方不能用公开字段拼一个绕过 recent-auth 的 binding。

### 5.2 `begin_enrollment`

`TotpService::begin_enrollment` 的顺序为：

1. 取得受控 UTC 时间并验证 recent-auth；
2. 计算 bounded pending expiry；
3. 生成 seed；
4. 建立 user-owned `totp_seed` binding 并由 current key 加密；
5. 把 envelope、credential ID、session guard 交给 repository；
6. 只在 `Created` 时一次性返回 seed；已有 pending 返回 `AlreadyPending`，CAS/授权变化返回 `Stale`。

### 5.3 `activate_enrollment`

application 先读取当前 pending credential，精确匹配 credential ID/revision，再解密 seed，并用当前 UTC 时间验证首个 code。验证成功后生成 8 个恢复码，使用 `KeyedDigestPurpose::RecoveryCode` 和 current keyring 计算用途隔离 HMAC。plaintext 只保留在 one-shot outcome，repository 只收到 digest/key version。

最终成功不是数个独立写：credential swap、旧 seed tombstone、恢复码 replacement、auth revision 与 session 变更在同一个数据库事务中提交。任一唯一键、状态或 CAS 条件失败都会回滚全部效果。

### 5.4 `verify_challenge`

C4 不自己签发 bearer，也不接受 HTTP 层拼出的 challenge。它只接收 C3 已持久预留的 `AuthChallengeVerificationClaim`，并执行：

1. claim method 必须是 `totp`；
2. commit 时钟不能早于 claim 的持久 `reserved_at_ms`，也不能越过 challenge expiry；
3. code 窗口锚定 `reserved_at_ms`，而不是可能跨入下一 period 的 commit 时间；
4. active credential、secret envelope 与上次 step 从 repository 取得；
5. repository 以 credential revision、previous step、auth revision、可选 session 和 reservation window 做条件更新；
6. 只有 durable step advance 成功后，才调用 crate-private `VerifiedAuthChallengeEvidence::from_method_verifier` 产生 `mfa` evidence。

进程若在 step advance 后崩溃，该 code 已被烧毁，但 C3 verifier lease 可独立恢复；不会出现 evidence 已发出而 replay state 未持久化的窗口。时钟回退不会消费 replay 状态。

### 5.5 `disable`

disable 仍要求受控 binding、recent-auth、credential ID/revision。application 不把 stale repository 状态伪装为成功；数据库事务负责实际 tombstone、恢复码 invalidation、auth revision/session 联动。

## 6. Persistence 与事务边界

### 6.1 guard 锁序

SQLite 通过立即写事务串行化；PostgreSQL 使用显式 row lock。管理事务先验证 guard：

- user active 且 user revision 相等；
- user auth revision 相等；
- actor session 属于该 user、状态 active、没有 revoke 元数据；
- session auth revision 相等；
- session `recent_auth_at_ms` 精确相等；
- `now_ms` 仍在 idle/absolute expiry 之前。

revision 或 recent-auth 在 application 读取后发生变化时，repository fail closed。

### 6.2 begin 与 pending 清理

`begin_totp_enrollment` 在同一事务内：锁 guard、发现并禁用到期 pending、tombstone 其 secret、检查仍存活的 pending、插入新 secret 和 credential。两路并发 begin 由部分唯一索引和事务共同保证只有一个 `Created`，另一个得到 `AlreadyPending`。

active 与 pending 可以共存，这是换绑时维持可登录性的必要条件。generic secret helper 仍假设同 binding 只有一个 live secret，因此明确拒绝 `totp_seed`；TOTP repository 每次都通过 credential ID 选 seed，避免未来 rewrap 取到任意一条。

### 6.3 activation 原子 swap

`activate_totp_credential` 的一个事务完成：

1. 锁 guard 和目标 pending credential；
2. 精确检查 pending 状态、revision、expiry 与 seed 未 tombstone；
3. tombstone 旧 active seed，并把旧 active credential 改为 disabled；
4. 把 pending 改为 active，保存首个 accepted step；
5. 使旧 recovery set 失效并写入恰好 8 个新 digest，set version 单调加一；
6. 用户 auth revision 加一；
7. revoke sibling sessions；
8. actor session 提升为 MFA、同步新 auth revision/recent-auth，并推进 session revision；
9. commit 后才返回 activation 结果。

碰撞 recovery digest 等末端失败会让旧 active、旧 seed、旧恢复码和 revisions 保持原样。替换 enrollment 失败或过期也不会损伤 active credential。

### 6.4 replay CAS

`advance_totp_step` 不只比较数字。条件更新同时匹配 credential/user/status/revision、expected previous step、auth revision、可选 active session、reservation step window 和非回退 commit clock。两路同一快照并发推进只有一个 `Advanced`，另一个 `Stale`；成功后复制请求和旧 code 都不能再次推进。

### 6.5 disable 原子性

disable 事务 tombstone 该用户所有 pending/active TOTP seed，把对应 credential 全部 disabled，使 active recovery set 失效，增加 auth revision，revoke sibling sessions并更新当前 actor session。CAS 或后续数据库写失败时不允许留下“credential disabled 但 seed/recovery/session 未处理”的半状态。

## 7. Migration 0008 与原始不变量

两库 migration 都先调整 `secret_records_active_binding_uq`：除 `totp_seed` 外仍维持 generic one-live-binding 规则；TOTP 的 active+pending 由 credential 级唯一索引管理。

`totp_credentials` 具备：

- 外键到 user 与 secret record；
- `pending/active/disabled` 闭集；
- 各状态允许的时间字段和 `last_accepted_step` 组合；
- non-negative timestamps/step/revision；
- 每用户最多一个 pending、一个 active 的部分唯一索引；
- SQLite 的动态类型/规范 UUID 检查与 PostgreSQL native UUID/BIGINT 语义对齐。

repository raw-schema 合同还验证表中不存在 seed/recovery 明文字段，并直接尝试破坏状态约束，确保错误由数据库拒绝而不是只依赖 Rust decoder。

## 8. 测试合同

同一 `repository_contract` 会对 SQLite 和真实 PostgreSQL 运行，覆盖：

- migration 版本为 8、raw schema/索引/约束；
- active 与 pending 同时存在；
- 并发 begin 只有一个 winner；
- activation 末端失败完整回滚；
- replacement 失败/到期 pending 不损伤 active；
- replacement 成功原子 swap、旧 seed tombstone、恢复码版本递增、auth revision/session 联动；
- tombstoned envelope 不再可读；
- reservation 跨 30 秒边界仍按持久预留时间验证；
- stale auth revision、expired/revoked session 被拒绝；
- 并发 step CAS 只有一个 winner，随后 replay 拒绝；
- stale recent-auth 不能 disable；成功 disable 清除 active/recovery 并推进 auth revision。

Domain/secrets/application 测试另覆盖固定 profile、TTL 边界、RFC vectors、精确 6 位格式、`±1` window、owner/AAD/key rotation、binding factory 的 forced-password-change 拒绝、reservation time 跨步和 clock rollback。

这些是源码合同；只有固定镜像的 workspace all-target tests、Clippy `-D warnings`、真实 PostgreSQL、OpenAPI/SDK/Web/doc/secret scan 全部完成，才可在项目账本登记对应 run 证据。

## 9. 明确尚未实现的边界

- **HTTP/API**：没有 enrollment begin/activate/status/disable 或 C3 TOTP proof route，也没有 CSRF/recent-auth Handler 映射。
- **OpenAPI/SDK**：没有 TOTP request/response schema 或生成 SDK 方法；现有 OpenAPI 不应因 C4 core 产生漂移。
- **Vue**：没有账户安全页 enrollment/confirm/disable UI，也没有 seed 和恢复码的一次性交付状态机。
- **base32 与 otpauth**：核心只持有 20-byte seed；尚未实现 canonical base32、issuer/account label 规范化、`otpauth://` URI 或二维码。
- **credential-aware rewrap**：generic secret rotation 明确拒绝 `totp_seed`。未来 root-key rewrap 必须以 credential ID 遍历 active/pending seed，保留 owner/AAD/revision，并在双库事务中避免选择歧义。
- **可观测性与审计事件**：尚未接入 HTTP request ID、正式安全事件 taxonomy 与运行指标；任何未来日志都不得包含 seed、code、otpauth URI、envelope 或恢复码明文。
- **恢复与完整凭据编排**：C2 恢复码登录、C5 WebAuthn、C6 高危 use case 接线仍需与 C3 replacement-session transaction seam 一起完成。

在上述边界完成之前，C4 是可复用、可事务验证的认证方法内核，不是可对外宣称启用的 MFA 产品功能。
