# WP02-C5 WebAuthn 核心实现

## 1. 本纵切边界

本纵切实现 WebAuthn/passkey 的 domain、secret、application 与 SQLite/PostgreSQL persistence 核心，并把成功证明接到 C3 的 typed `VerifiedAuthChallengeEvidence` seam。它不包含 HTTP DTO/router、OpenAPI、生成 SDK、Vue 页面或浏览器互操作门；这些仍是后续工作，不能把本提交描述成用户可用的完整 WebAuthn 功能。

核心安全合同如下：

- registration 使用 attestation `none`、user verification `required`；authentication 同样要求 UV；
- RP ID 只从 typed public HTTPS origin 的精确 host 派生，不接受独立的、更宽 RP ID 配置；请求 Origin 经同一个 type canonicalize 后做精确相等比较；
- 不启用 subdomain、any-port 或 alternate-origin 放宽，不接厂商 metadata、官方域名、许可证或在线设备信誉服务；
- challenge、origin、RP ID、signature 与 UV 的验证全部交给维护中的 Rust WebAuthn 库；NodeControll 不解析或实现 COSE、CBOR、签名或 attestation；
- raw attestation object、`clientDataJSON`、`authenticatorData` 只存在于无 `Debug`/`Clone` 的 finish command 中，直接传给库，绝不保存或日志输出；
- ceremony 是加密、typed、TTL、单次消费的持久状态；成功、明确失败、过期都会清除 ciphertext columns；
- authentication 只有在库完整验证、credential/UV/user handle 再核对、counter/BE/BS CAS 提交成功之后，才调用 crate-private evidence constructor 产生 phishing-resistant evidence。

## 2. 依赖与真实 API

workspace 只精确锁定版本并关闭 default features；危险能力不随
`webauthn-rs.workspace = true` 自动扩散：

```toml
webauthn-rs = { version = "=0.5.5", default-features = false }
url = "=2.5.8"
```

只有 `nodecontroll-application` 的直接 dependency 显式开启：

```toml
webauthn-rs = { workspace = true, features = [
  "danger-allow-state-serialisation",
  "danger-credential-internals",
] }
```

依赖来源与核验入口：

- [`webauthn-rs` 官方仓库](https://github.com/kanidm/webauthn-rs)；
- [`Webauthn` 0.5.5 API](https://docs.rs/webauthn-rs/0.5.5/webauthn_rs/struct.Webauthn.html)；
- [`WebauthnBuilder` 0.5.5 API](https://docs.rs/webauthn-rs/0.5.5/webauthn_rs/struct.WebauthnBuilder.html)；
- [`AuthenticationResult` 0.5.5 API](https://docs.rs/webauthn-rs-core/0.5.5/webauthn_rs_core/proto/struct.AuthenticationResult.html)；
- [WebAuthn Level 3 credential ID definition and attested credential data (§6.5.1)](https://www.w3.org/TR/webauthn-3/)；
- [WebAuthn Level 3 credential backup state contract](https://www.w3.org/TR/webauthn-3/#sctn-credential-backup)；
- [PostgreSQL 17 transaction isolation / Read Committed](https://www.postgresql.org/docs/17/transaction-iso.html)；
- [`url` 2.5.8 API](https://docs.rs/url/2.5.8/url/struct.Url.html)。

源码审阅确认 `start_passkey_registration` 固定 `AttestationConveyancePreference::None`、`UserVerificationPolicy::Required`，允许同步 authenticator；`start_passkey_authentication` 固定 UV required；两个 `finish_*` 负责 challenge/origin/RP/signature/UV 等 WebAuthn 验证。库为兼容旧数据提供 backup-eligibility upgrade 开关，但 NodeControll 依据 WebAuthn Level 3 把 BE 当作注册期不变量，在 finish 后和 persistence command 两层要求 result BE 与 stored BE 精确相等。`WebauthnBuilder` 默认 `allow_subdomains=false`、`allow_any_port=false`，实现没有调用任何放宽方法。

两个 `danger-*` feature 的用途被严格限制：

1. `danger-allow-state-serialisation` 只用于把库拥有的 concrete `PasskeyRegistration` / `PasskeyAuthentication` 转成 bytes，立刻按 ceremony ID 与专用 purpose/AAD 加密；没有 `serde_json::Value` 或任意 JSON state escape hatch。
2. `danger-credential-internals` 只做两件事：把已由数据库解封并与 projection 完整核对的 `Passkey` 验证副本 counter 归零；读取/更新库拥有的 credential projection 后重新加密。它不用于读取 raw response、实现签名/COSE/CBOR 或绕开 finish 验证。

关闭默认 `attestation` feature 是有意的：C5 不建立 CA/metadata 信任链，也不依赖网络 metadata。默认 attestation-none 流程会把 credential 的 `AttestationMetadata` 归为 `None`，因此 AAGUID 列为 nullable；实现不会为了填 AAGUID 自行解析 raw CBOR。将来若产品需要 attested authenticator allowlist，必须作为独立安全设计，不得悄悄改变本合同。

## 3. Domain 与 public origin

`crates/domain/src/webauthn.rs` 提供：

- `WebAuthnOrigin`：只接受 pathless HTTPS domain origin；拒绝账号信息、query、fragment、IP literal 与前后空白；通过 `url` canonicalize，并从 exact domain 得到 RP ID；
- `WebAuthnCredentialId`：按 WebAuthn Level 3 接受 opaque 16..1023-byte ID，业务代码不解释；
- `WebAuthnUserHandle`：1..64 bytes，当前固定为用户 UUID 的 16 raw bytes，不使用 username；
- `WebAuthnAaguid`：16-byte local value，不触发外部 lookup；
- `WebAuthnTransport`：`usb/nfc/ble/internal/hybrid/test` UX hint；`test` 来自 webauthn-rs 0.5.5/Windows 兼容枚举。该版本尚不识别 Level 3 `smart-card`，因此当前把它当未知 transport 忽略；支持它需要先升级并审阅依赖，transport 仍不参与安全判断；
- `WebAuthnNickname`：trim 后 1..80 chars，拒绝 control chars；
- `WebAuthnCredentialStatus`：`active/revoked/clone_suspected`；
- `WebAuthnCredential`：credential ID、user handle、nullable AAGUID、transports、UV、BE、BS、counter、nickname、status、created/last-used/revoked/clone-suspected、同步 counter anomaly 时间与 revision。

`WebAuthnOrigin` 是唯一配置与请求比较入口。即使 RP host 相同，scheme 或显式 non-default port 不同也不是同一 origin；应用在调用库之前先做 exact equality，库随后再使用同一个 exact origin 验证 client data。

## 4. Typed secret 边界

`crates/secrets/src/lib.rs` 增加三组 schema v1：

| Purpose | Owner | Plaintext concrete type |
|---|---|---|
| `webauthn_registration_state` | `webauthn_ceremony` | `PasskeyRegistration` serialization |
| `webauthn_authentication_state` | `webauthn_ceremony` | `PasskeyAuthentication` serialization |
| `webauthn_credential_material` | `webauthn_credential` | `Passkey` serialization |

`Keyring::encrypt/decrypt_webauthn_*` 使用专用 purpose、owner kind、owner UUID 和 schema version 形成 AAD。plaintext deserialize/serialize buffer 用 `Zeroizing<Vec<u8>>` 持有。generic singleton `secret_records` API 明确拒绝这三类 purpose，防止绕过 ceremony/credential transaction lifecycle。

没有 raw response type 的加密 helper，也没有 arbitrary JSON helper。错误对外只映射 typed application error，不拼接或输出库错误和 plaintext。

### 4.1 库日志硬边界

源码审阅同时发现 `webauthn-rs-core` 0.5.5 的 tracing 点可能格式化 parser input。为此不是依赖默认 `RUST_LOG` 或“调用方不记录”，而是同时设置两层不可绕过边界：

1. 每个同步 `start_*` / `finish_*` 调用都在 thread-local `NoSubscriber` 内执行；库返回错误离开该边界时立即丢弃具体 error/source，只把固定的 operation + `library_rejected` reason 交给自有 target 审计；
2. production master subscriber 只有一个 JSON output layer，immutable allowlist 只接受十个 `nodecontroll_*` crate target 及其 `::` 子模块，再与环境 `EnvFilter` 做 AND。任何 `webauthn_rs`、`webauthn_rs_core`、`webauthn_rs_proto` 顶层或子 target 的更具体 `RUST_LOG=...=trace` directive 都无法越过 allowlist。`tracing-subscriber` 的 `tracing-log` default feature 被关闭，不存在第二个 log bridge；若 hardened subscriber 已被别的 subscriber 抢先占用，master 启动失败。

合同测试用恶意顶层/子 target directive 注入带 attestation/client/authenticator secret marker 的事件，断言输出只含自有 stable audit；源码合同断言 binary 没有第二个 subscriber/LogTracer，WebAuthn 边界不记录 library error/source/response。

## 5. 数据库 schema

双库 migration 均为 `0009_webauthn_credentials.sql`，建立：

### `webauthn_credentials`

全局 unique opaque `credential_id`；16-byte `user_handle`；nullable AAGUID；`user_verified` 必须 true；`backup_state => backup_eligible`；unsigned-32 范围 counter；nickname/status/timestamps/revision；material schema/key/nonce/ciphertext/AAD hash；可选 `backup_counter_anomaly_at_ms` 只允许用于 BE credential。

material 仍由库拥有且加密。表里没有 COSE key column，也没有 attestation/client/authenticator raw data column。

### `webauthn_credential_transports`

以 internal credential row ID 为外键保存去重 transport hints；删除 credential row 时 cascade。transport 不参与 proof decision。

### `webauthn_ceremonies`

同一 typed table 承载两类互斥 shape：

- registration：必须绑定 user、actor session、purpose=`credential_enrollment`、exact RP/origin、user revision、auth revision、recent-auth timestamp、created/expires/revision；不得有 C3 claim 字段；
- authentication：必须绑定 C3 auth challenge ID、claim ID、purpose、user、optional session、auth revision、reserved time、verification expiry、完整 client context digest、exact RP/origin；不得有 registration revision/recent-auth 字段。

pending row 必须同时有完整 encrypted-state columns，consumed/rejected/expired row 必须有不晚于 ceremony expiry 的 `finished_at_ms` 且所有 encrypted-state columns 为 NULL。partial unique index 保证同一 user/session 同时只有一个 pending registration；authentication claim ID 全局 unique。

## 6. Registration 事务

所有 management command 的 binding 只能通过 `WebAuthnManagementBinding::from_authenticated_session` 从 ordinary authenticated-session projection 生成；字段不可由 transport 逐个拼装。factory 明确拒绝 `force_password_change`、non-active/revoked session，并捕获 user/session/user revision/auth revision/exact recent-auth timestamp、canonical username 与 principal label。registration 的 RP account name/display name只取该私有投影，HTTP DTO 不能替用户制造 authenticator 上显示的账户标签；当前策略是 username→`user.name`、principal label→`user.displayName`。repository 仍在事务中重新锁定和校验 revision/session lifetime。

`WebAuthnService::begin_registration`：

1. exact Origin 与 controlled clock/recent-auth 检查；
2. persistence 重新检查 active user/session、user/auth revision、recent-auth timestamp、session idle/absolute 时间，拒绝 `force_password_change`；
3. 解密并核对当前 credential `Passkey`，只把 credential IDs 交给库作 exclude list；
4. 调用 `start_passkey_registration`；
5. concrete state serialize 到 zeroizing bytes、按 ceremony ID 加密；
6. `Database::begin_webauthn_registration` 以 partial unique + revision binding 写入单一 pending ceremony。

`WebAuthnService::finish_registration`：

1. 按 ceremony ID/revision、management guard 与 exact origin 读取仍有效 state；
2. 解密为 concrete `PasskeyRegistration`，调用库 `finish_passkey_registration`；
3. 再断言 UV、BE/BS 与 attestation-none，不读取 raw payload；transport 只在库成功后从 response 的 typed hint list 投影；
4. concrete `Passkey` serialize/encrypt；
5. 加密与官方 finish 返回后再次读取受控时钟；第二次时间不得倒退，且必须仍在 recent-auth 与 ceremony TTL 内，所有 terminal 时间都用这次 fresh sample；
6. `Database::complete_webauthn_registration` 在单一事务内：全局 duplicate CAS、插入 credential/transports、推进 auth revision、撤销同用户其他 active sessions、把 actor session 更新到新 auth revision/`phishing_resistant` 并刷新 recent-auth、消费并清空 ceremony state。PostgreSQL completion 与 begin/rename/revoke 共用 canonical auth-state → user/actor-session lock prefix，再单独锁 ceremony，避免反序。若 credential unique CAS 判定为重复，事务保持 credential/transports/auth revision/session 零写，但把已锁定的 exact ceremony 原子标为 rejected、清除 encrypted state 后提交；这样重复响应不能把用户困在 `one_pending` 到 TTL，也不能复用旧 challenge。

除上述 duplicate-burn 终态外，任何 stale、actor-session update 或 ceremony consume 失败都整体回滚。明确无效 proof 会把 registration ceremony 标为 rejected 并清空 state，不能重复试同一 challenge。

## 7. C3 authentication 与 crash resume

C3 `AuthChallengeVerificationClaim` 现在携带 durable reservation 的 actual `reserved_at_ms` 与 min(challenge expiry, verifier lease) `verification_expires_at_ms`。普通 pending verifier 的 `AuthChallengeService::resume_verification_claim` 仍严格要求 verifier lease；finish 必须再次提交原 opaque challenge bearer 与同一 client context，persistence 重新验证 token HMAC、claim ID、method、challenge revision、user/session/auth revision，再重建 private claim。数据库 ceremony ID 或 claim ID 本身不是 capability。只有下文已经在 lease 内提交的 terminal WebAuthn handoff 能把同一 claim 的 C3 收尾延长到 enclosing challenge expiry。

`WebAuthnService::begin_authentication` 只接受 method=`webauthn` 的 private C3 claim，形成 `WebAuthnChallengeBinding`，并要求 active credential。每个 credential material 必须先由数据库 envelope 解封，再与 DB projection 的 credential ID、UV、BE、BS、counter、attestation-none 完整相等；只有私有 `DatabasePasskey` wrapper 能进入 `counter_normalized_verifier_copy`。

counter 归零只发生在交给 `start_passkey_authentication` 的一次性验证副本中，用来避免库的通用 counter policy 在 synced passkey 场景提前拒绝、从而让应用失去按 BE/BS 做 CAS 的机会。原 `Passkey` material 从不归零；finish result 也绝不直接信任。后续顺序不可交换：

1. exact origin + C3 bearer/claim/context resume；
2. exact encrypted ceremony 与 selected credential load；
3. 库 `finish_passkey_authentication` 完整验证 challenge/origin/RP/signature/UV；
4. 核对 result credential ID、UV、optional user handle、`BS=>BE`、BE 与注册记录精确相等；
5. 对原数据库 `Passkey` 应用官方 `update_credential`；
6. 在 terminal reject/clone/success 前第二次读取受控时钟；它不得早于 preflight sample，且必须严格早于 exact verifier expiry；
7. 按 credential revision + old counter + old BE/BS + ceremony revision 做事务 CAS，所有 terminal time 使用 fresh sample；
8. CAS winner 消费/清空 ceremony，之后才创建 phishing-resistant evidence。

库失败或结果字段不一致会先原子 burn ceremony；只有 burn winner 返回 typed rejected claim，避免 stale loser 误伤并发 winner 的 C3 claim。

### 7.1 C3 commit-point crash recovery

WebAuthn credential/counter CAS 与 C3 的 `accept_verified_method` / `reject_attempt` 是两个 repository commit point，不能假装一次函数返回消除了中间崩溃窗口。authentication ceremony 的 terminal row 因而同时是 durable handoff：

- counter/material commit 成功后为 `consumed`，proof failure commit 后为 `rejected`；两者都清除 encrypted library state并把 revision 精确推进一次；
- authentication begin/commit/reject/clone 在修改 ceremony 或 credential 前，PostgreSQL 先以 user auth-state 行作为单用户事务 gate，再以 `FOR NO KEY UPDATE` 锁 user/optional session，最后按 claim/reserved time/user/session/auth revision/context 锁定并重新验证 exact C3 row；SQLite 通过同事务 writer lock。`NO KEY UPDATE` 仍阻断 status/delete，却与 C3 challenge INSERT 的 FK `KEY SHARE` 相容，因此 C3 的 stale-challenge → principal 顺序不会和 C5 的 principal → challenge 形成死锁。lease-expiry refresh 要么先赢而整个 WebAuthn transaction 返回 stale，要么等待 terminal row 提交后看到 durable pin，不会出现 counter/ceremony 已变而 claim 被并发清除的半状态；
- PostgreSQL `READ COMMITTED` 下，单条等待中的 `UPDATE` 可能看到 concurrent target-row version，却仍不能看到该语句 snapshot 之后写入的其他 terminal row。因此 refresh 先以独立 statement 锁 exact challenge（user/purpose 批量路径按 UUID 排序锁定），terminal 检查与 UPDATE 再由后续 statement 的新 snapshot 执行；revoke/clone 对将批量 invalidated 的 challenge 也先按同一 UUID 顺序锁定，统一为 auth-state → user/session → challenges → credential → ceremony 的顺序；
- retry 必须再次提交原 C3 opaque bearer。authorize 先重新验证 token HMAC 与 exact client context；terminal handoff lookup 再精确匹配 caller-observed ceremony revision + 1、claim/reserved time/user/session/auth revision/context/RP/origin 和仍处于同一 `verification_pending` claim 的 C3 row；
- 当 verifier lease 已过但 enclosing challenge 尚未过期时，C3 refresh 只对数据库中 exact terminal WebAuthn row 保留原 claim；resume 与 C3 success/failure transition 也分别只接受 `consumed`/`rejected` 对应 handoff。普通 verifier、pending ceremony、错误 terminal status 或已失效 user/session/auth revision 都不能利用这条放宽；
- exact `consumed` handoff 可幂等重建同一 phishing-resistant evidence，exact `rejected` handoff可幂等重建同一 rejected claim，从而让 C3 consumption/failure 在进程崩溃或响应丢失后继续；错误 bearer/context 或 C3 已推进时均不能读取 handoff。
- terminal row 已由原事务证明 `reserved <= created <= finished <= verifier expiry`。恢复时若可信 wall clock 回拨到 reserved 与 immutable terminal timestamp 之间，不再用 `created/finished <= current now` 否决既成 handoff；仍要求 current now 不早于 reservation、未超过 enclosing challenge/session expiry，并重新验证 bearer、exact claim/context、user/session/auth revision 与 RP/origin。pending pre-terminal 路径仍严格 fail closed。

因此 raw proof 仍是一次验证、counter CAS 仍是单 winner，但“库提交成功/失败后、C3 尚未落库”的窗口不会在 verifier lease 到期时 orphan。SQLite/PostgreSQL 共用合同在 `verification_expires_at_ms + 1` 分别模拟 success/failure crash retry、C3 transition 单 winner与 wrong-claim 不可见，并在 expiry 边界并发 terminal commit/refresh，断言只允许“完整 terminal + pinned claim”或“完整 refresh + credential 未变”两种结果。

## 8. Counter、BE/BS 与 clone suspected

counter policy 使用库的 verified `AuthenticationResult`，但由 persistence CAS 落实：

- 非 BE credential：如果 stored/result 任一 counter 非零，result 必须严格大于 stored；否则进入 clone-suspected transaction，不生成 evidence；`0 -> 0` 按 WebAuthn 无 counter 信号处理；
- BE credential：允许合法同步导致的相等或回退，持久 counter 取 `max(stored,result)`，不误杀 synced passkey；当任一 counter 非零而 result 非递增时写 `backup_counter_anomaly_at_ms`，保留本地审计信号；persistence 从 official result 的 `observed_sign_counter` 独立重算 persisted counter/anomaly，拒绝调用方省略或伪造该信号；
- `backup_state=true` 必须同时 `backup_eligible=true`；result BE 必须与注册时 stored BE 精确相等，false→true 与 true→false 都拒绝，persistence command 也不能改变 BE；
- normal commit 同时 CAS revision、old counter、old BE 与 old BS，concurrent/replay 只有一个 winner。

clone-suspected transaction 原子执行：credential status/revision 改为 `clone_suspected`、burn WebAuthn ceremony、invalidate 精确 C3 challenge、推进 user auth revision、撤销该用户所有 active sessions并 invalidates 其他 open challenges。任一 CAS 失败整笔回滚。

## 9. Rename 与 revoke

`rename_webauthn_credential` 以 internal credential ID + expected credential revision 更新 active credential；application 在任何数据库调用前要求 command 的 typed request origin 与配置 public origin 精确相等，同时重新核对 management user/session/user revision/auth revision/recent-auth/idle/absolute/force-password-change guard。

`revoke_webauthn_credential` 同样先执行 exact typed Origin guard，再使用 credential ID + revision CAS，并在同一事务内：标记 revoked/time、burn 该用户 pending WebAuthn ceremonies、invalidate open auth challenges、推进 auth revision、撤销其他 sessions、把 actor session迁移到新 auth revision。cleanup 时，已过期但尚未惰性刷新到 DB 的 pending ceremony 写为 `expired` 且 `finished_at_ms=expires_at_ms`；仍有效的才写为 `rejected` 且 finished time 为事务时间，始终满足 schema 的 terminal timestamp 约束。stale target 或 session guard 不会留下半完成状态。未来 HTTP adapter 仍必须复用 C1 Host/Origin/CSRF middleware；application 自身的 Origin guard不是省略 CSRF 的理由。

## 10. 合同测试与静态门

`crates/persistence/src/webauthn_contract.rs` 对 SQLite memory DB 与独立 PostgreSQL schema 运行同一个 repository contract，覆盖：

- registration 并发单 winner、wrong origin、重复 credential 的零 credential 写 + ceremony burn + 立即重新 begin、replay、晚于 TTL 的惰性 expiry，以及 finish 与 revoke 的 canonical-lock one-winner race；另以 stale C3 challenge replacement 并发 revoke/clone，固定 PostgreSQL FK `KEY SHARE` 不得形成交叉死锁；
- exact challenge/claim/user/session/auth revision/client context/RP-origin 错配；
- authentication ceremony/credential counter revision CAS、concurrent single winner 与 replay；
- success/failure commit-point 后跨 verifier lease及 terminal 后 wall-clock rollback 的 exact bearer/context durable handoff retry，wrong claim 不可见，C3 transition 单 winner；
- verifier-expiry refresh 与 terminal commit 并发时，exact claim lock 保证 terminal-pin 或 clean stale 二选一；
- registration/authentication 在 `expiry + 1` 清理时把 finished time 固定到 stored expiry；revoke 同时清理 stale pending 与 live pending ceremony；
- non-BE clone-suspected 全 session revoke/auth revision；
- BE/BS synced non-increment acceptance、monotonic max、anomaly audit、BE 双向变化拒绝与 `BS=>BE`；
- rename/revoke revision 与 session/auth-revision effect；
- raw schema columns absence、credential ID 15/16/1023/1024 byte 边界、UV=true 与 `BS=>BE` raw constraint。

application unit contract固定 controlled management-binding factory及 canonical labels、rename/revoke exact Origin guard、preflight/terminal 两次 controlled-clock 边界、counter matrix、BE 双向不变量与 `BS=>BE`。domain tests固定 origin/credential ID 15/16/1023/1024/nickname bounds。secrets tests固定 purpose/owner/schema binding。C3 persistence/application tests覆盖 bearer+context crash resume。master telemetry contract覆盖恶意 `RUST_LOG`、秘密不出日志和单 subscriber/source seam。仓库继续 `unsafe_code=forbid`、`unwrap_used/todo/dbg_macro=deny`。

按任务约束，本地没有运行 compile、test 或 formatter。本提交只做 `git diff --check`、schema/secret/source 静态扫描；精确 SHA 的 Rust fmt/Clippy/SQLite/真实 PostgreSQL 测试必须由后续 VPS gate 产生证据，未取得 run 前不得标成 verified。

## 11. 尚未实现与风险

- HTTP request/response DTO、Origin header adapter、C3 bearer transport、Problem Details 映射尚缺；raw finish DTO 必须继续禁止 Debug/logging，并设严格 body size。
- C6 的第一项必须是 authenticated management `list_credentials` application/repository port，只返回 non-secret `WebAuthnCredential` projection并执行 exact Origin/session guard；HTTP 不得直接调用现有 internal `active_*` 查询，因为那些查询为 verifier/registration 返回 encrypted material。C5 当前尚未提供安全 list API。
- OpenAPI/SDK、Vue credential management 与 browser ceremony、Playwright/真实 authenticator interoperability 尚缺。
- attestation none 无可信 AAGUID；当前 nullable 是诚实状态，不提供设备型号/信誉判断。
- 两个 `danger-*` 都是审阅敏感能力，当前只在 application crate 显式开启；升级 `webauthn-rs` 时必须重新审查 serialized state/credential schema、counter semantics 与 features，不能仅放宽 semver或把 features 移回 workspace dependency。
- encrypted library state/material 与 0.5.5 representation 绑定；未来升级需要显式 schema migration/兼容读取，不能用 untyped JSON 修补。
- authentication WebAuthn ceremony commit 与随后 C3 evidence consumption 仍是两个持久 commit point，但 terminal handoff 已覆盖中间 crash/retry；后续 HTTP adapter 必须在收到 `Verified`/`Rejected` 后调用 C3 accept/reject，不能把 handoff 结果直接当最终登录响应。
