# WP02-C1 密码近期认证、改密与会话管理实现

## 1. 状态与范围

本文记录 WP02-C1 当前代码的真实边界。历史 v4 226-file 应用代码候选已在指定 VPS 通过 78/78 Rust 双库门、9 文件 81/81 Vitest、362-module production build、OpenAPI/生成 SDK 零漂移和 SQLite/PostgreSQL 同合同 smoke；双页 HTTPS candidate v4 也已通过并由 validator 重算。后续 v6 226-file 门工具候选 archive SHA-256 为 `e2a055daf353da1f6500ba643b7ae75516e900976e02bae3536e44a818a8cb58`：fresh pnpm 明确关闭 global virtual store，静态/OpenAPI/生成目录 16 个物理文件/Web 门、650 组件许可证闭包和两库 smoke 均通过。v6 之后又按最新目标把正式 release/Web 编译唯一收敛到公开 GitHub Actions、VPS 只保留测试和同 SHA 制品验收，因此两批证据都不绑定最终提交树。公开提交、Actions attempt 1 制品和 fresh-clone 正式验收仍未完成，本页不能作为正式发布门通过证明。

C1 只覆盖：

- 密码近期认证与当前 session rotation；
- 登录时透明升级旧 Argon2 PHC；
- 自助修改密码、推进 `auth_revision`、撤销全部旧 session 并换发唯一 replacement；
- 活动 session 列表、逐个撤销、退出其他 session、退出全部 session；
- 强制改密期间的后端 use-case allowlist、router guard 和 DOM fail-closed gate；
- 对应的 OpenAPI、生成 SDK、Vue 页面、SQLite/PostgreSQL 共用合同和 runtime smoke。

恢复码、持久化 key canary/keyring、统一 challenge、TOTP、WebAuthn 和所有后续高危业务 use case 接入属于 C2～C6，仍未实现。C1 的真实 HTTPS 门已扩展为双页协调合同：旧凭据 401 零 `Set-Cookie`、logout 503/204、quarantine 跨 reload、显式登录恢复、迟到旧 cursor 失效不覆盖新状态，以及七类冻结目标的 secret scan；candidate v4 已通过，提交级 formal 仍须用公开同 SHA 制品重跑。C7 会在此基础上扩展完整浏览器、并发与故障注入矩阵。完整语义由 [WP02-C 认证安全合同](./WP02_C_AUTHENTICATION_SECURITY_CONTRACT.md) 约束。

## 2. 请求到数据库的调用链

```text
Vue page
  -> Pinia session store
  -> OpenAPI generated SDK
  -> Axum handler / browser security boundary
  -> ControlPlane application use case
  -> identity/password + envelope HMAC
  -> Database typed operation
  -> SQLite or PostgreSQL transaction
```

高危写请求必须同时满足 Host/Origin、session Cookie、CSRF Cookie/header、服务端 session 状态和近期认证。浏览器计算的 `recentAuthValid` 只负责提前导航；最终授权始终由 application 使用数据库读出的 `recent_auth_at_ms` 判断。

## 3. 配置与 Master 装配

| 位置 | 函数/字段 | 作用 |
|---|---|---|
| `crates/config/src/lib.rs` | `AuthConfig::recent_auth_seconds` | 近期认证窗口，默认 300 秒；只接受 60～3600 秒，且不得超过 absolute session lifetime |
| `load` 的 auth 校验 | recent-auth 边界检查 | TOML 与 `NODECONTROLL__...` 覆盖后统一 fail-fast，非法配置不会启动网络监听 |
| `apps/master/src/main.rs` | `auth_policy` | 把秒数转换为 `Duration`，连同 idle、absolute、touch 和三层登录限流装配进 `AuthPolicy` |
| `apps/master/src/main.rs` | `main` | 仍只组合配置、数据库、cipher、密码服务、application 与 API；不在入口复制认证判断 |

`AuthPolicy::validate` 再做一次 application 层防御性验证。测试或其他装配点即使绕过配置加载，也不能构造 recent-auth 长于 absolute lifetime 的策略。

## 4. Domain 与密码实现

### 4.1 `crates/domain/src/lib.rs`

| 类型/函数 | 作用 |
|---|---|
| `PasswordHash::parse` | 只接受完整且资源有界的 Argon2 PHC：算法限定 Argon2id/i/d，版本限定 16/19，参数只能有 `m/t/p`，内存 8,192～19,456 KiB，迭代 1～2，并行度固定为 1，salt 8～16 bytes，输出 16～32 bytes |
| `PasswordHash::as_str` | 只读暴露已校验 PHC；类型不实现会泄漏正文的调试输出 |
| `BaselineCapabilities::for_forced_password_change` | 强制改密时返回最小 capability 集，不复用角色的完整能力 |

PHC parser 的上界很重要：登录必须兼容旧参数，但不能让数据库里任意 PHC 诱导密码库分配无界资源。

### 4.2 `crates/identity/src/lib.rs`

| 类型/函数 | 作用 |
|---|---|
| `PasswordVerification::{verified,into_upgraded_hash}` | 把“密码是否正确”和“是否需要新 PHC”绑定为一次验证结果，调用方不能在失败密码上请求 rehash |
| `PasswordService::validate` | 对新密码执行 enrollment policy；当前规则为至少 12 个 Unicode scalar、最多 1024 UTF-8 bytes且无控制字符 |
| `PasswordService::verify` | 对已有 PHC 验证密码；只执行资源上界，不拿新密码的长度下界拒绝合法旧密码 |
| `PasswordService::verify_with_upgrade` | 验证成功后比较当前 Argon2id 策略，必要时生成新的随机 salt/PHC |
| `PasswordService::needs_rehash` | 比较算法、版本、`m/t/p`、解码后的 salt 长度和输出长度；不比较随机 salt 内容 |
| `PasswordService::hash_resource_bounded` | 给透明 rehash 使用；允许仍然合法但不满足当前 enrollment 长度的旧密码升级，避免把有效旧账号锁死 |
| `LoginPasswordWorkStep` / `LoginPasswordWorkPlan` | 冻结所有密码登录共有的三步计划：用 current-policy dummy 校准、验证所选凭据并按条件升级、补齐校准 deadline；调用方只能取得固定 plan，不能按账号路径自定义步骤 |
| `PasswordService::execute_login_work_plan` | 对未知账号、当前 PHC 和受限旧 PHC 执行同一计划；校准耗时为 `C` 时，目标为 `2C + clamp(C/2, 25ms, 1s)`，不把慢机器的 `C` 向下截断；校准或验证错误也要等所选验证与 padding 完成后才返回 |
| `login_verification_input` | 超过 1024 UTF-8 bytes 的输入改用固定、受限的校准字符串完成两次 Argon2 与 padding，结果强制失败且不能产生升级 PHC |

密码值由 `Zeroizing<String>` 承载；API DTO、application command 和阻塞 Argon2 task 都不把密码写入日志、错误或 store。`ControlPlaneApplication::new` 还会拒绝不符合当前策略的 dummy PHC。并发 semaphore permit 被移入 blocking closure，覆盖校准、所选验证、可选成功升级和 padding，闭包结束后才释放。

当前透明升级只接在正常 `login`。密码 reauth 使用 `PasswordService::verify`，不会顺带改写 PHC；这避免把“近期证明”和“凭据维护”混为一个未冻结的事务。若以后要在 reauth 中升级，必须先补同等级 CAS、事件和回滚合同。

## 5. Application use case

### 5.1 类型与公共边界

| 类型/函数 | 作用 |
|---|---|
| `AuthServiceError` | 增加 `InvalidProof`、`InvalidNewPassword`、`PasswordUnchanged`、`RecentAuthRequired`、`PasswordChangeRequired` 等稳定业务错误 |
| `AuthenticatedAction` | 所有消费浏览器 session 的 use case 必须声明动作类别 |
| `AuthenticatedAction::allowed_during_forced_password_change` | 只放行读取自身身份、重新认证、改密码、管理自身 session 与退出 |
| `ReauthenticateCommand` | 绑定 mutating credential 与一次性密码证明 |
| `ChangePasswordCommand` | 绑定 mutating credential 与新密码，不接收浏览器确认字段 |
| `LogoutAllCommand` | 强制调用方显式选择 `keep_current` |
| `LogoutAllOutcome` | 区分返回 replacement 的 200 与全部退出的 204 |
| `UserSessionProjection` | 在粗粒度 session 投影上增加 `is_current`，不包含网络或凭据材料 |
| `RevokeSessionCommand/Outcome` | 将目标 session 限定到当前用户，并通知 API 是否需要清当前 Cookie |
| `ControlPlane` | 新增 `reauthenticate`、`change_password`、`logout_all`、`list_sessions`、`revoke_session` 五个 use case |

### 5.2 投影、freshness 与 rotation helper

| 函数 | 作用 |
|---|---|
| `actor_projection` | 从已认证数据库快照构造 actor；强制改密时只投影受限能力 |
| `session_projection` | 返回已有 session 的时间线，并把 `recent_auth_at + window` 截断到 absolute expiry |
| `new_session_projection` | 对尚未重新读取的事务内 replacement 使用同一投影规则 |
| `recent_auth_is_valid` | 使用严格 `now >= recent_auth_at && now - recent_auth_at < window`；等于截止时间即过期，墙钟回拨也失败 |
| `prepare_session_rotation` | 生成全新 session/CSRF token 与用途隔离 HMAC，更新客户端摘要；继承原 absolute expiry，不借 rotation 延长总寿命 |
| `security_event` | 只记录 request ID、reason、账号/IP HMAC 与 UA hash，不记录明文账号凭据 |
| `rotated_login_outcome` | 原子组合新 actor/session/token；改密成功时把 `force_password_change` 投影改为 false |

`prepare_session_rotation` 由调用方显式传入两项证明时间。密码 reauth 把 `authenticated_at_ms`、`recent_auth_at_ms` 都更新到当前时间；改密码和保留当前的 logout-all 则继承已有证明时间。

### 5.3 session 认证入口

| 函数 | 作用 |
|---|---|
| `authenticate_credential` | 解析 session/可选 CSRF token，计算 HMAC，调用 touch 或 read-only repository 认证，再执行强制改密 allowlist |
| `authenticate_mutating_credential` | 把 mutating credential 转成带 CSRF 的认证请求，固定使用 read-only 模式 |

读取 `/me` 和 session 列表可以按 touch interval 更新最后活动时间。重新认证、改密码、撤销和 logout-all 在进入真正事务前只读认证；失败的 proof、freshness 或 CSRF 不改变 session revision、last-seen 或 idle deadline。

repository 要求 `last_seen_at_ms <= now_ms`；因为合法 session 已满足 `created_at_ms <= last_seen_at_ms`，一般认证、活动列表和所有 rotation 当前会话选择器在墙钟早于 session 时间线时都会 fail closed。`ProductAccess` 目前只是后续业务 use case 的动作类别；C6 接入真实产品 API 前还要重构 touch 决策，避免正常用户的产品读取被错误地固定为 read-only。

### 5.4 `login`

`login` 的顺序是：初始化检查 → 取得 Argon2 并发许可 → 三层共享限流预留 → 读取用户并选择真实 PHC，或为未知账号选择 current-policy dummy → 执行固定三步密码工作计划 → 生成 session/event → `create_auth_session_with_optional_password_upgrade`。未知账号、当前 PHC 和低成本旧 PHC 的错误密码都进入同一 plan 与 timing bucket；停用账号即使旧密码正确也只验证、不生成额外 current-cost rehash，避免失败响应泄漏“密码正确”的时序信号。只有活动账号的成功旧密码登录可以透明升级。

限流预留同时携带一枚候选 `rate_limited` 事件。repository 在写任何 bucket 前核对 reason、时间、key version、request ID、账号/IP HMAC 与 UA hash；account、IP 或 global 只有从未封禁状态首次进入 durable block 时，才在同一个 bucket 事务里写一条事件。已有 block 的 preflight、并发 follower 和进程内 Argon2 semaphore 饱和都不写 durable 事件。通用 `record_login_security_event` 明确拒绝 `rate_limited`，调用方不能绕过这个原子入口伪造独立限流记录。事件 ID 冲突会连同本次 bucket 更新一起回滚；`now == blocked_until_ms` 重新开放，下一次真实进入 block 才产生新周期事件。

数据库事务用用户 revision、`auth_revision`、`password_changed_at_ms` 和旧 PHC 做 compare-and-swap：

- snapshot 未变时，rehash、session、安全事件和账号 limiter 清理一起提交；
- 另一条并发登录先完成同一透明升级时，loser 在其余 credential/auth snapshot 未变、user revision 精确推进一次且 PHC 已变化时识别 winner；随机 salt 使两条候选 PHC 无需逐字节相同，loser 仍可建立自己的 session；
- 并发改密、停用、删除或认证版本变化时，不建立 session；
- 透明 rehash 不改 `password_changed_at_ms`，不推进 `auth_revision`，也不撤销 sibling session。

### 5.5 `reauthenticate`

`reauthenticate` 先只读认证当前 session，再走与登录共享的 Argon2 semaphore 和 account/IP/global limiter。错误密码记录通用失败事件，返回 `InvalidProof`，旧 session 保持不变。成功路径调用 `rotate_current_session`：

1. 锁定并复核用户 revision、当前 session revision/状态/期限和认证版本；
2. 以 `rotation` 原因撤销当前 session；
3. 插入唯一 replacement 与 `reauthentication_succeeded` 事件；
4. 清账号 limiter；
5. 在同一事务提交后才由 API 发两枚新 Cookie。

replacement 把 `authenticated_at_ms`、`recent_auth_at_ms` 更新为当前时间，absolute expiry 继承旧 session。其他 session 不受影响。

这个路径只证明方法 `password`。初始密码登录只创建 `auth_level=password`，rotation 则保留当前 session row 的既有等级；未来高保证 session 的方法/保证级别分离由 C3 完成，不能把“保留原 auth level”解释成原来的 MFA 或 WebAuthn 刚被重新证明。

### 5.6 `change_password`

`change_password` 的授权顺序是 session/CSRF → recent-auth → 当前用户 snapshot → 有界 Argon2 新密码验证、同密码拒绝与新 PHC。它随后调用 `change_password_and_rotate`，在单个事务中：

1. compare-and-swap 复核用户 revision、当前 session、旧 `auth_revision` 和 recent-auth snapshot；
2. 写新 PHC、单调的 `password_changed_at_ms=max(旧值, 当前时间)`、`force_password_change=false`；
3. 推进 `auth_revision`；
4. 以 `password_changed` 撤销该用户全部旧 session；
5. 插入使用新 revision 的唯一 replacement；
6. 写安全事件并清账号 limiter。

replacement 继承当前 session 的 `authenticated_at_ms`、`recent_auth_at_ms` 和 absolute expiry。并发改密只有一个 winner，loser 不得覆盖新密码。

### 5.7 session 管理 use case

| 函数 | 作用 |
|---|---|
| `current_actor` | 认证并按 touch interval 更新当前 session，返回 actor/session 投影 |
| `list_sessions` | 只列当前用户、active、未过 idle/absolute 且 `auth_revision` 仍匹配的 session |
| `revoke_session` | 要求 recent-auth；把调用 session 的稳定认证快照交给 actor-aware 事务，事务复核调用方仍有效后才撤销同一用户的目标，以安全事件 `session_revoked` 和状态原因 `user_revoked` 记录独立审计语义 |
| `logout` | 未知/已失效 session 幂等成功；有效 session 的撤销与 logout 事件同事务 |
| `logout_all(keep_current=false)` | 要求 recent-auth；推进认证版本、撤销包括当前在内的全部 session、写事件，不创建 replacement |
| `logout_all(keep_current=true)` | 要求 recent-auth；推进认证版本、撤销全部旧 session并插入唯一 replacement，继承 absolute 与证明时间 |

两类 logout-all 都使用条件更新复核当前 session。`revoked_at_ms` 写为 `max(created_at_ms, now_ms)`，墙钟回拨不能阻止撤销记录满足数据库约束。

指定 session 的 DELETE 在调用者仍持有有效当前 session 时，对未知或已撤销目标返回 204。SQLite 事务先以 `user_auth_state` 同值更新取得写锁；PostgreSQL 的稳态认证写统一遵守 `user_auth_state → users → auth_sessions`：DELETE 先锁用户级 auth-state barrier 并核对 auth revision，再锁用户行复核 active/deleted/user revision，最后锁 actor session 复核状态、期限、auth revision 与 recent-auth，随后才更新目标。touch 第一次 token 查询只取得候选 user ID；取得同一 barrier 和用户锁后，会按 token 重新做完整授权校验并锁 session，第一次读取不能作为授权 snapshot。登录/rehash、rotation、改密和两种 logout-all 也使用同一锁序；普通 logout 只锁 session，之后不再取得 auth-state 或用户锁，因此不形成逆序等待。A/B session 互删只能有一个事务撤销对方，loser 在取得 barrier 后发现 actor 已失效；双库合同要求恰好一条事件、一个 active 和一个 `user_revoked`，不接受 deadlock、lock-timeout 或 SQL 错误。事务刻意不比较会被普通 touch 推进的 session revision，所以旧 snapshot 后的无害 touch 仍可撤销；并发 rotation/revoke、认证版本变化、用户停用/修改或墙钟回拨则拒绝，且目标与事件都不变。若第一次删除的正是调用者当前 session，响应会清 Cookie；随后用这枚已撤销凭据重放请求会先在认证边界得到 `401 SESSION_INVALID`，不属于目标资源幂等语义。

## 6. Persistence 与迁移

### 6.1 新增/扩展数据类型

| 类型 | 作用 |
|---|---|
| `UserCredentials` | 增加 `user_revision`、`auth_revision` 与 `password_changed_at_ms`，给 CAS 提供完整 snapshot |
| `AuthLevel` | 保证级别固定为 `password/mfa/phishing_resistant/recovery`；旧 `webauthn` 数据迁移为 `phishing_resistant` |
| `PasswordChangeRotation` | 用参数对象传递 user ID、当前 session ID、预期 user revision、新 PHC、replacement、事件与时间；新 auth revision 在 replacement 中给出，不把会随 touch 变化的 session revision 当作改密授权 CAS |
| `UserSessionRevocation` | 绑定 user、actor session、target session、预期 user/auth revision、recent-auth snapshot、事件和当前时间，作为逐会话 DELETE 的完整事务输入 |
| `LoginAttemptReservation` | 绑定三层 limiter 摘要、UA hash、request ID、统一时间与策略；与候选 `rate_limited` 事件逐字段核对后才允许写 bucket |
| `LogoutAllResult` | 返回撤销数、最新认证版本与是否保留当前 |
| `PasswordChangeResult` | 返回 replacement 摘要、撤销数与新认证版本 |
| `LoginSecurityReason` | 增加 `reauthentication_succeeded`、`password_changed` 与用户主动管理会话所用的 `session_revoked` |

### 6.2 repository 函数

| 函数 | 事务/查询职责 |
|---|---|
| `user_credentials_by_normalized_username` | 联结 `users` 与 `user_auth_state`，读取密码和两类 revision snapshot |
| `create_auth_session_with_optional_password_upgrade` | 原子完成条件 rehash、登录 session、成功事件和 limiter 清理 |
| `upgrade_password_hash_if_current` | 仅供 repository contract test 使用的私有 CAS helper；生产登录走上一行的一体事务，不能绕开 credential/auth snapshot |
| `rotate_current_session` | recent-auth 成功的当前 session 单点 rotation |
| `change_password_and_rotate` | 改密、推进认证版本、全撤销、唯一 replacement 与事件的一体事务 |
| `authenticate_session` | 认证并在满足 touch interval 时条件更新 last-seen/idle/revision |
| `authenticate_session_read_only` | 完成相同合法性/CSRF 检查但不 touch；高危 mutation 的前置失败不留写痕迹 |
| `logout_all_sessions_with_event` | `keep_current=false` 的条件全撤销与事件事务 |
| `logout_all_sessions_and_rotate` | `keep_current=true` 的认证版本 CAS、全撤销、replacement 与事件事务 |
| `list_active_user_sessions` | 数据库侧过滤 status、期限和 auth revision，固定按创建时间/ID倒序 |
| `revoke_user_session_with_event` | actor-aware 逐会话撤销；先在同一事务重验调用方资格，再按 `(target_session_id, user_id, active)` 撤销，只有实际状态变化才写事件 |
| `revoke_current_session_with_event` | 普通 logout 专用；只按 `(user_id, session_id)` 撤销，和 logout 安全事件同事务，不承载逐会话管理授权 |
| `reserve_login_attempt` | account→IP→global 固定顺序预留；首次进入 durable block 时把 bucket 与唯一 `rate_limited` 事件原子提交 |
| `record_login_security_event` | 写普通独立安全事件；拒绝 `rate_limited`，防止绕过 limiter transition 事务 |

SQLite 与 PostgreSQL 分支由同一公开函数验证输入，再分别进入 `*_sqlite`/`*_postgres` 实现。rotation helper 统一检查：replacement 必须是新 ID、新 token/CSRF HMAC、initial revision、同一用户、正确事件类型、正确客户端摘要、时间线不越过旧 absolute expiry。

### 6.3 migration

| 文件 | 作用 |
|---|---|
| `*/0004_recent_auth_password.sql` | 扩展安全事件 reason 白名单，允许近期认证成功与改密事件 |
| SQLite `0005_session_rotation_timeline.sql` | 以新表复制方式重建 `auth_sessions`，有意把 `authenticated_at_ms >= created_at_ms` 放宽为非负，使 replacement 能继承更早的认证证明；其余时间线仍受约束，并把旧 `webauthn` 迁移为 `phishing_resistant` |
| PostgreSQL `0005_session_rotation_timeline.sql` | 重建 0003 自动命名的跨列认证时间约束 `auth_sessions_check`，使 replacement 可继承早于自身创建时间的认证时刻；保留 status/revocation 配对的 `auth_sessions_check7`，并迁移 auth-level 与撤销原因枚举 |

当前迁移合同覆盖 fresh schema、0004→0005 的 session 时间线升级、两库 0005 约束拒绝，以及 pool 重连后的 session 持久性。新增的失败注入又逐条锁定以下行为：

- SQLite 0003→0004 在复制既有 `login_security_events` 后，由外键引用令旧表 `DROP TABLE` 晚失败；事务回滚后，migration version 仍为 3，八字段事件行、原表 DDL、两个显式索引、阻断行全部不变，`login_security_events_new` 不得残留。移除阻断后重跑到 4，索引可被显式使用，两种新 reason 可写，未知 reason 仍被 CHECK 拒绝。
- SQLite 0004→0005 同样在 session 新表复制后晚失败；回滚保留完整 session/event、原表 DDL 和索引，version 仍为 4，`auth_sessions_new` 不得残留。重跑到 5 时只把旧 `webauthn` 转成 `phishing_resistant`，其余字段和事件不变。
- PostgreSQL 0004 用一条不满足新 reason CHECK 的历史 poison row 触发验证失败；version、事件、索引以及全部约束定义和 `convalidated` 状态必须逐项回到 0003。删除 poison 后在新连接池重跑，最终 CHECK 必须 validated，新 reason 可写、未知值不可写。
- PostgreSQL 0005 的 test-only trigger 只在 `webauthn → phishing_resistant` 更新时失败，因此能证明前置约束变更也在同一事务回滚。删除 trigger/function 后以新连接池重跑，session 只发生预期转换，三个新显式 CHECK 都已 validated，fixture function 不得残留。

SQLx 0.9 的迁移 apply 失败会在 session advisory lock 解锁前返回。测试若直接复用原 pool，取回同一物理连接时 lock 会重入并残留一层，取到另一连接时则可能等待至超时。四条 PostgreSQL 失败后重试分支——新增两条、既有两条——都明确关闭旧 pool，再从原 `connect_options` 建立保留隔离 `search_path` 与超时参数的新 pool；这既模拟生产进程重启，也去掉了连接选择这一隐含测试前提。SQLite 内存库不能重连，仍在原单连接 pool 中完成修复后的重跑。

## 7. HTTP API 与 Cookie

| 方法与路径 | 成功响应 | 关键失败 |
|---|---|---|
| `POST /api/v1/auth/reauth` | 200 actor/session；两个独立 `Set-Cookie` header 轮换 session/CSRF | 401 session；403 proof/Origin/CSRF；429 limiter |
| `POST /api/v1/me/password` | 200 actor/session/revoked count；换发两枚 Cookie | 403 recent-auth；422 policy/unchanged；401 session |
| `GET /api/v1/me/sessions` | 200 粗粒度活动 session 列表 | 401 session；400 metadata；403 Host |
| `DELETE /api/v1/me/sessions/{id}` / `revokeCurrentUserSession` | 204；撤销当前 session 时同时清 Cookie | 400 非 canonical UUID；401 当前调用会话无效；403 recent-auth/Origin/CSRF；503 认证依赖不可用 |
| `POST /api/v1/auth/logout-all` | keep=true 为 200 replacement；false 为 204并清 Cookie | 403 recent-auth/Origin/CSRF；401 session |

`ReauthenticateRequest.method` 当前只接受 `password`，但请求形状已经为 C3 的 method-neutral challenge 留出判别字段。受保护 JSON DTO 全部 `deny_unknown_fields`，密码字段用自定义 deserializer 进入 `Zeroizing<String>`。

`remaining_cookie_max_age_seconds_at` 使用 `min(configured_max_age, floor((absolute-now)/1000))`。rotation 不会把浏览器 Cookie 的 Max-Age 延长到 session absolute deadline 以后。所有认证成功、204 和 Problem 响应都设置 `Cache-Control: no-store`。Problem，包括 `SESSION_INVALID`，一律零 `Set-Cookie`；否则旧标签页迟到的 401 可能清掉另一标签页刚轮换的新 Cookie。签发、轮换或清 Cookie 只能来自路由显式选择的成功响应；logout、撤销当前 session 与 `logout-all(false)` 的 204 复用清理响应。

如果 absolute lifetime 只剩不足一个完整秒，向下取整会得到 `Max-Age=0`。这是刻意的 fail-closed 边界：不能为了让 replacement 在浏览器里多活一秒而越过服务端截止时间；客户端随后需要重新登录。

OpenAPI 由 Rust `utoipa` 导出。安全审阅后的版本仍为 12 paths/13 operations，撤销端点 operationId 为 `revokeCurrentUserSession`，SHA-256 是 `b30934dac8c52d1cdbae0dca470e2ba3b4a44785b8f63ccd4b0484df73254596`；固定 Node/pnpm 生成器报告 4 个顶层输出项，递归展开输出目录后共有 16 个物理文件且逐字节零漂移。`Set-Cookie` 描述明确表示 session 与 CSRF 是两个独立 header field，生成 SDK 不手工修改。正式 release/OpenAPI 只由公开 Actions 构建和导出，VPS 对同 SHA 制品运行测试与契约验收。

## 8. Vue/Vuetify 实现

### 8.1 `stores/session.ts`

| 函数/状态 | 作用 |
|---|---|
| `SessionSnapshot` | 用单个 discriminated snapshot 原子保存 status 或 actor+session；每次替换推进单调 generation，旧异步响应不能覆盖 logout 或更新后的身份 |
| `syncRecentAuthClock` / `recentAuthValid` | 仅做 UI 预判，使用严格 `< recent_auth_expires_at_ms` |
| `acceptAuthenticated` | 一次替换 actor/session 并同步时钟；所有非 authenticated 状态同时清空受管理会话列表 |
| `credential-coordinator.ts` | 以 Web Locks、唯一非秘密 localStorage journal、随机 epoch 与逐次 revision 协调所有同源标签页；BroadcastChannel/storage 只唤醒，不取代持久记录 |
| `acquireCredentialMutation` | exclusive 锁覆盖 inflight 持久化、完整 SDK 请求/响应解析、运行时投影校验和 terminal 持久化；只有精确 cursor 可开始受保护 mutation |
| `withCredentialReadLock` | shared 锁覆盖 `/me`/session-list 请求与验证；回调业务错误原样透出，锁/存储/协议错误单独 fail closed |
| `publishCredentialInvalidation` / `persistCredentialInvalidation` | 401 先发布绑定 observed session 与 base cursor 的瞬时失效；释放 shared 后取 exclusive，cursor 未变化才持久化 invalidated，迟到 401 不能覆盖新 epoch |
| `reauthenticate` | 每次请求前重新读取 CSRF Cookie；只消费白名单状态、本地 Problem code 和经过运行时校验的成功投影 |
| `changePassword` | 成功原子接受 replacement；传输结果未知时抛 `outcome-unknown`，不自动重放；畸形 200 同样视为未知 |
| `listSessions` | 在 shared 锁内验证活动 session 列表、唯一 current 与当前 ID；新读取开始先清旧列表，验证成功后才一次提交 |
| `revokeSession` | 请求前冻结目标是否为当前 session 并跟踪逐 ID pending；当前目标成功时转 anonymous，结果未知时以幂等 logout 关闭本地身份；其他 session 的未知结果不误伤当前登录 |
| `logoutAll` | 处理 keep/clear 两种权威结果；未知结果不重放，也不按 session ID 猜测事务是否提交 |
| `requireReloginAfterUnknownMutation` | 先进入持久 `relogin-required`，读取最新 CSRF 后只尝试一次幂等 logout；只有权威 204 才转 anonymous，401/5xx/传输失败都保持 sticky quarantine |

reauth、改密、两种 logout-all，以及直接撤销当前 session 的传输中断或任何 5xx 都进入 `outcome-unknown`：

- 不重发原 mutation；
- 不调用 `/me`、不比较新旧 session ID，也不向用户宣称提交或未提交；
- 只用浏览器此刻的 CSRF 尝试幂等 logout，以覆盖“服务端已 rotation 但响应丢失”和“请求根本未提交”两种状态；
- 只有 logout 权威返回 204 时转 anonymous；401、5xx、畸形响应或传输失败都不能证明共享 Cookie 已被清理，必须保持 `relogin-required`，直到显式登录成功或后续显式清理 204。

每个凭据 mutation 在网络请求前先持久化 `inflight/quarantine`，使本标签页和已调度的其他标签页立即关闭 actor/session、会话列表与受保护 DOM。释放 exclusive lock 前再写 `settled/reconcile` 或 `settled/quarantine`；写入失败、revision/epoch 不连续、未观察到对应 inflight、journal 损坏与协调能力缺失都 fail closed。journal 缺失不是无条件异常：fresh setup/anonymous 且无 CSRF Cookie 是合法初始态；仍有 CSRF，或投影为 authenticated、unavailable、relogin-required 时则必须隔离。唯一 localStorage 键是 `nodecontroll:credential-coordination:v1`，其中不含凭据或身份投影；`sessionStorage` 保持为空。显式登录可从 absent/corrupt/quarantine 建立新 epoch，普通受保护 mutation 不可以。

浏览器无法在另一个标签页被同步长任务阻塞时立刻修改其 DOM；该标签页只能在事件循环恢复后消费通知并关屏，阻塞期间也无法交互或发请求。凭据请求仍受 Web Lock 排他。等待全部标签页 ACK 会被冻结或崩溃标签页永久阻塞，因此这项 P2 平台边界保留并写入安全合同。

rotation mutation 的任何 5xx 都按 `outcome-unknown` 处理，包括带稳定 code 的 `503 AUTHENTICATION_UNAVAILABLE`。原因不是不信任服务端 Problem，而是数据库 `COMMIT` 报错可能发生在提交已经落盘、确认却丢失之后；当前 persistence 没有把“可证明提交前失败”和“提交结果未知”分成两个稳定错误。纯查询的 503 仍可作为普通 unavailable。

### 8.2 router 与 App gate

| 位置 | 作用 |
|---|---|
| `router/meta.d.ts` | 增加 `allowDuringPasswordChange` 与 `requiresRecentAuth` |
| `accessRedirect` | 强制改密时把普通功能导向改密页；recent-auth 过期时把高危页面导向 reauth |
| `safeRedirectPath` | 继续拒绝外站、双斜杠、反斜杠、多层编码控制字符和 guest loop |
| `App.vue` | 除 router guard 外再做受保护 route 与强制改密 DOM gate；状态不一致时先移除业务 DOM |

后端 allowlist 才是授权边界，router 和 DOM gate 负责减少错误操作与敏感内容残留。三层检查不能互相替代。

### 8.3 页面

| 页面 | 关键行为 |
|---|---|
| `ReauthenticatePage.vue` | 当前密码只在一次请求内存在；成功后清空并锁提交，导航失败只能重试导航 |
| `ChangePasswordPage.vue` | 浏览器内检查确认字段、scalar 下界和 UTF-8 byte 上界；服务端仍做最终策略；未知结果锁表单、清密码并要求重新登录 |
| `ProfileSecurityPage.vue` | 展示粗粒度 session 时间线，带确认 dialog 的逐个撤销/退出其他/全部退出；不显示 IP、原始 UA 或 token |
| `ProfileSecurityPage.vue` recent-auth timer | 按服务端 deadline 设置 timer，并在 tab 恢复可见时重新同步，避免后台 timer throttling 让 UI 长期显示“仍新鲜” |

## 9. 测试与开发候选证据

### 9.1 已观察的 VPS 候选结果

较早的开发候选位于 `/opt/nodecontroll/dev/wp02c-c1-20260826t0342z-003/source`。该目录在初次上传后做过逐文件同步，因此初始 archive SHA 不能代表最终树；它只用于迭代验证，不是提交级 provenance。下面的数字只描述该较早候选，不覆盖本页前述的安全审阅后修正。

已通过：

- Rust `cargo fmt --check`、workspace all-targets check、73 个 SQLite/PostgreSQL tests、Clippy `-D warnings`；
- Vue typecheck、ESLint `--max-warnings=0`、49/49 Vitest、361-module production build；
- Rust 导出 OpenAPI 12 paths/13 operations，工作树 OpenAPI SHA-256 为 `e0470bf8e827f663932034e6c28efd6994f4e0b850d07686664d762a50a88cda`；
- OpenAPI SDK 在 VPS 用锁定 `@hey-api/openapi-ts@0.99.0` 生成；
- 文档校验为 358 requirements、358 trace rows、0 broken links；
- SQLite 与 PostgreSQL 真实 Master smoke 均覆盖双 session、失败/成功 reauth、sibling 保留、列表/逐个撤销、改密、旧密码与旧 session 失效、新密码与 replacement 生效、logout-all keep/clear、最终 logout；
- 该较早 smoke 当时还检查 `SESSION_INVALID` 清两枚 Cookie；这项历史行为已被跨标签页审阅废止，不能作为现行合同或正式门证据。现行门必须改为 Problem 零 `Set-Cookie`、旧浏览器 Cookie 不被迟到 401 改写，并由显式成功清理响应单独验证删除；其余 Max-Age、Problem HTTP/body status 与非空 `type` 断言保留；
- SQLite runtime log SHA-256 `6f2e2288dbe8446fde262d55dbcc1df432f9edf4ce0b078402dcafb453489fe5`，PostgreSQL runtime log SHA-256 `8a32697f7833ee110af062ebfd6e9b68bdbe360ac285b98375db70e3935bf31f`，按真实测试 secret 扫描均为零命中。

固定 builder 仍是 Rust image `sha256:6ab6185f9998fe126309ed033570b3828808212bb3c4f7edbf88f98892881613`、Node image `sha256:06628671caed76e73560464d4ce47cacb202fcf28d090c0d24f2ead1cc23afcb`、PostgreSQL image `sha256:1c59e2c3c818eaa0f0628f695b36e7c9e362d6b219b36a54a32df645cbd7e1af`。

新一轮 `/opt/nodecontroll/dev/wp02c-c1-20260826t0630z-001/source` 迭代候选先观察到 73/73 Rust tests；并发修正和约束语义测试加入后，fresh PostgreSQL/SQLite run 为 74/74。登录 timing 与 actor-aware DELETE 两项 P1 修正加入后，225 个源码路径的同步 archive SHA-256 为 `033b7c0f938f51d57d55806e152c80ab64c9034287bffd14cd3047f51711e6db`；固定 Rust 1.98 builder 通过格式检查、78/78 workspace all-targets tests、Clippy `-D warnings` 和 release bins，Master/Agent SHA-256 分别是 `45e4f98d0bdced80a26b143e860a8217a8471e6d6c7d2e99dfacd5a06d1f9f71`、`a72215e93f6634794f8e35ef8fb52845f4a1c975b40bfa446cd20dcf3fd3b0cd`。Vue typecheck、57/57 Vitest、361-module build、OpenAPI 12/13 和文档 358/358/0 broken links 也已在该目录的不同增量点观察到。一次静态约束名误判被 fresh migration fail-closed，随后通过真实 catalog 查询纠正；`logout-all(false)` 被无害 touch 打断，以及逐会话 DELETE 在调用方并发失效后仍执行的竞态，都由双库合同锁定。这些结果仍只描述增量候选，精确源码快照与正式制品门未完成，不能升级为发布证据。

统一 PostgreSQL 锁序和 `rate_limited` 原子审计加入后，首次 VPS 编译发现两处 `tokio::join!` 借用临时认证对象的测试生命周期错误；改成显式局部绑定后，SQLite 与真实 PostgreSQL 的共享合同通过。独立限流审阅随后发现通用安全事件入口仍能单独写 `rate_limited`，这一生产 API 缺口已在写前拒绝，并补 `blocked_until_ms` 严格边界。含该修正的较早 225-path archive SHA-256 是 `e96e469399a7a1e12c83e6be96ff81043717397dbde56b0ec868d1ee9e067112`。

迁移失败合同和 advisory-lock 重试修正完成后，在本段证据账本更新前又生成一份 225-path 精确 archive。VPS 与本地逐文件清单都是 225 项且零差异，archive SHA-256 为 `c31ab821ed0df8e73f4e6fd554fd6bc9586d8bdedf3a21df5ab7889b17e34344`；`crates/persistence/src/lib.rs` 的同步 SHA-256 为 `aa35cd84c18f2eeb4ddbf9b595c61fddb9707381ce71d7110e3e5db0e7d83431`。固定 Rust 1.98 builder 通过格式检查、78/78 workspace all-targets tests、Clippy `-D warnings` 和 release bins；test/Clippy/release 日志 SHA-256 分别为 `a2378d55dc76d2d5c0689310edb0a152d85496b7e87cec82b8caff64be9b64ff`、`e9c20416b6b668b7eebb642818c4e8d190cd9002cd9decabec95202840f26628`、`e00e793be3c4f60bce84db79b6382bb194622acd994e1ef9c5d7fd68b196f30d`；Master/Agent SHA-256 为 `f0ba91073af186dcf356c2dd659b904ea0732bc5c1d9486ca7686642d957330f`、`a72215e93f6634794f8e35ef8fb52845f4a1c975b40bfa446cd20dcf3fd3b0cd`。这份 archive 是下一项前端收口前的迁移检查点，仍不能替代新的 run root、公开提交与正式 provenance。

公开提交前的一轮审阅补上两处客户端 fail-closed。`revokeSession` 的公共边界可以直接收到当前 session ID，旧实现若在服务端已撤销后丢失响应，会继续保留 authenticated snapshot；函数现在于任何 await 之前冻结 current-target，未知时调用 `requireReloginAfterUnknownMutation`。继续审阅又复现了晚到响应复活身份：较早的 reauth/改密/keep-current 200 可以在 logout 或另一条 fail-safe logout 关闭身份后才执行 `acceptAuthenticated`。store 现用单调 snapshot generation 使所有身份替换失效旧请求；refresh、reauth、改密和 keep-current 只接纳请求 generation 仍匹配的 projection。login 等待 exclusive lease 时，自身 inflight 造成的精确 `+1` 才是正常推进；若还有外部推进，随后即使得到 401/429 也不能解除 sticky quarantine。晚到 rotation 200 转成结果未知并再次 cleanup。普通 logout 和 fail-safe helper 都在发出网络请求前关闭投影；只有结构完整且 generation 匹配的显式登录 200，或权威清理 204，才恢复。该轮固定 Node builder 的 63/63 Vitest 与 361-module build 是后续协调协议之前的历史增量结果；现行数字见进度账本。

真实 HTTPS 门第一次已完成浏览器行为，但冻结的 WAL 快照在 dump 时生成 `-wal/-shm`，随后又超过 120 秒握手上限；门正确失败，失败 run 与秘密已删除。修正为 snapshot 切回 DELETE journal、dump 以 `immutable=1` 打开并在 dump 后再次拒绝 sidecar 后，第二次候选 run `20260826T052551853287390Z-wp02c-candidate` 通过：旧 session 401、旧 CSRF 403、新 CSRF 通过后错误 proof 403 且零 auth `Set-Cookie`/Cookie 与 session projection 不变、新 session 200；Cookie、absolute lifetime、浏览器 storage/DOM/URL/console/request、TLS SAN/key pair 都满足合同。门对七类冻结目标执行两次一致扫描，共 33 files、10,027,952 bytes；宿主 validator 重算 tree hash 通过。evidence SHA-256 是 `0fed24fab2edcaa0fce5d89c8291970f6b5785c2be0c4f8866ee2a894dad4bfe`，checksums SHA-256 是 `3d3dcf30c488ff547536f88929d8c4df389c330657cf6414e23a05b4929b6088`。测试秘密、live DB、容器和网络均已删除。该 run 只绑定公开基线 SHA，不绑定当前未提交增量树，所以仍是开发证据。

宿主 validator 后审又补了三层独立约束：固定 `/evidence` 与握手文件的精确规范路径；核对七类目标的类型、只读叶文件、根身份和互不重叠关系；拒绝最终及临时 SQLite WAL/SHM/journal。secret scan 失败时不再只写 marker，而是先删除被判定污染的日志或 evidence；完成态另外证明 marker、临时目录、容器和网络都已消失。新版 validator 在 VPS 对同一成功证据回放仍精确得到 33 files、10,027,952 bytes。两个隔离负向副本分别恢复一个 Web 叶文件的写位、增加 `database-journal`，均被拒绝；副本随后按已核对的精确路径删除。这些是验证器回归，不改变该候选尚未绑定当前源码的性质。

第二版候选 run `20260826T063126155736337Z-wp02c-candidate-v2` 继续验证普通 logout：注入 503 时 Cookie 保持、不得出现 auth `Set-Cookie`，但内存投影和受保护 DOM fail closed；真实 204 随后清 Cookie，旧凭据再请求为 401。冻结证据为 33 files、10,028,822 bytes，evidence SHA-256 `0bb41ac31d1713338caa4dad33f0d65f4133f6668c062f9eb5dfcf647d75755f`，checksums SHA-256 `7b8670567fbe1be9c5b19ea093d7885362d64afb18e5bf905084b19b6b33c5f0`。负向门除可写 Web 叶文件和额外 `database-journal` 外，还在受控副本中放入解码后的 32-byte root key，secret scan 按设计拒绝；负向目录与秘密随后删除。这个 run 同样只绑定公开基线，不能替代当前树的 formal 复跑。

双页协调版候选 run `20260826T112446112732525Z-wp02c-candidate-v4` 已在固定 Playwright 1.62.0、外置 Node 24.19.0 和 Chromium 151.0.7922.34 上通过。两个同 context 真实页面共同验证 peer logout 503 后 protected DOM 关闭、settled/quarantine journal 跨 reload 原样保留、reload 零自动 `/me`/login、显式 200 login 恢复两页，以及绑定旧 `baseEpoch/baseSeq` 的合法迟到 invalidation 实际送达却不改变新 journal/session/DOM；旧原始 Cookie `/me` 401 明确为零 `Set-Cookie`，只有最终真实 logout 204 调用双 Cookie 清理断言。浏览器显式以已 realpath、检查 metadata 并核对 SHA-256 的 canonical Chromium executablePath 启动。门冻结并扫描 33 files、10,087,567 bytes；evidence SHA-256 为 `9db50f56137bd905310771e4d1fb82bffdada29780df4a2ce4c3a9297586fe5f`，gate/validator SHA-256 为 `459eabe176e90e20ac9bda5845cb943c176a505ecc2aace96b62f1f38c96577c`、`c2df67a7f2c2a469a3a70f933046e9cc04b142bf94efdd04f67a205c16e63327`；断网只读宿主 validator 独立复算通过。测试秘密、live DB 和容器已删除，browser 证据子树以 0400/0500 封存，七类扫描目标及叶文件均已去除写位。它仍以 `ecd8dea…` 作为当前未提交增量树的公开基线，不是完整 Git tree 绑定，因此不能替代公开提交后的 formal provenance。

历史 v4 应用代码门使用 `/opt/nodecontroll/dev/wp02c-c1-freeze-20260826t1130z-004`。archive 严格包含 226 个工作树文件，SHA-256 为 `0c2eb2a94256ee3b5795e9811b2833090989244c5d8370a9ce05beb8f5fabe5c`；远端逐文件清单 SHA-256 为 `772172f85e38d94ba67846f66252a588631ecb6a342566af1949357a5a7f61dc`，31 份日志清单 SHA-256 为 `320b1e2039ea0675b758c7b6c763f888c10aae3f8f7b9d880a057ec56532ab89`。固定 Rust 门再次得到 78/78，fmt/test/Clippy/release 日志 SHA-256 依次为 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`、`73ddaff852fe9eacbec25764779d38ed6a4bfa686e9c8f4d874ac8be108e3f24`、`2bc683fb4e4f4f21ee9ee39e2b8ecce2efe20d3b1be57bda5316aa519ab3d30a`、`6b1c8cbe0bd9cb3b12baa322a9182419af04da4050d3affc7c9798b1b46e30b5`。release exporter 与仓库 OpenAPI 都是 `b30934dac8c52d1cdbae0dca470e2ba3b4a44785b8f63ccd4b0484df73254596`；这些 hash 只绑定 v4，不绑定之后的门工具与文档修正。

同一历史 v4 archive 的固定 Node 门通过生成器 4 个顶层输出项递归展开后的 16 个物理文件零漂移、typecheck、零 warning lint、9 文件 81/81 Vitest、362-module Vite build 和 Web artifact 检查。typecheck/lint/test/build/artifact 日志 SHA-256 依次为 `aacdfedc485449151e8f0cedfb8264a7d15047256a7b3c3a4a9e848e28e7b178`、`b543827ce56a63357c629028555ecec2cc964dda33dbbf2a9a2872b1a2ab20f8`、`ec10a8d768f65e49bbd7ee162f10de04764dc7a3375fc58a8664a7c89c2b5203`、`c7318b67329e3309f741bc0eace71c68a77d5d3f66a18a226dfa1de9f4fab1ca`、`26aac325c5633ddbeb822af5aab515fa423cad1315bf405545726c07a5eee7f3`。当前 validator 对上述 v4 浏览器 evidence 的再验日志 SHA-256 为 `55ff874ab73ac6b97bb1169522edc08392a564d34065ca0d8916188da43a8550`。

最新 v6 门工具候选 archive `e2a055daf353da1f6500ba643b7ae75516e900976e02bae3536e44a818a8cb58` 以 fresh pnpm/Cargo 输入通过 global virtual store=false、静态/OpenAPI/YAML、16 个生成文件零漂移、typecheck、lint、81/81 Vitest、Web artifact、两次断网许可证闭包重建和两库 smoke；许可证 inventory 为 Cargo 221、npm 428、Rust toolchain 1，共 650 个组件和 858 份证据。v6 与 v4 的 Rust、OpenAPI、smoke、E2E gate/validator 输入逐字节相同；测试秘密、数据库、容器和网络均已删除。它仍早于最新 Actions/VPS 编译边界与本次文档修正，不能冒充最终提交树。

修正 `expectSessionInvalid` 后，SQLite 与 PostgreSQL 分别完成同一份 Master smoke；两份 smoke 输出 SHA-256 都是 `2a4549206f74fec53374e4d92c175135b1310b7fb638cffc9f666eddf50dac14`，Master 日志分别为 `54d92aa6b978d8b48b78a8ac72c3950bad7167d3eac2c1323bfe423e9cbd4c06`、`1030887b5e83c1c40b758dddf3bba14e745cb5ae4b52c8229b7c7ef540264c8c`。PostgreSQL 前两次只在业务请求前失败：第一次被配置门拒绝非回环 HTTP origin，第二次证明 internal Docker network 不允许宿主机访问发布端口；第三次改用独立 bridge 并只发布 `127.0.0.1:18086` 后通过。三轮容器、网络和随机 fixture 都按精确名称清理，成功与失败日志对真实 root key、setup token、数据库密码、四个 smoke 口令和 Argon2 PHC 前缀均零命中。

### 9.2 验证中发现并修正的问题

- application test 缺少 `UserCredentials` import，补齐后才进入双库合同；
- SQLite legacy fixture 的 `username_norm` 与新规范化读取不一致，升级 helper 现在显式修正；
- PostgreSQL 自动约束名不能按列名猜测：在只应用 0001～0004 的 fresh 探针库查询 `pg_constraint` 后，确认跨列认证时间约束是 `auth_sessions_check`，status/revocation 配对是 `auth_sessions_check7`；一次错误修改被 fresh migration 立即拒绝，0005 已恢复为只重建前者并保留后者；
- PostgreSQL restart test 复用了已被合同撤销的 session，改为专用持久 session marker；
- Clippy 报告改密函数参数过多和测试复杂类型，分别引入 `PasswordChangeRotation` 与类型别名；
- jsdom 缺少 `VisualViewport`，测试 setup 增加最小替身；
- Vuetify dialog 在 teleport 后出现同名按钮，组件测试改为在 dialog 内精确定位；
- 前端浏览器全局引用统一为 `globalThis`，满足零 warning lint；
- `logoutAll(true)` 曾用新旧 session ID 推断事务结果；审阅证明另一个 tab 的无关 rotation 会制造假阳性，现已删除该启发式，所有未知 rotation 结果都 fail-safe logout 并要求重新登录；
- 一般 session 最初只检查 idle/absolute deadline，没有拒绝 `now < last_seen_at_ms`；认证、列表与 rotation 选择器现统一在墙钟回拨时 fail closed；
- 逐会话撤销最初复用 `logout` 审计原因，后又短暂误用 `administrator` 状态原因；现固定为安全事件 `session_revoked` 与状态原因 `user_revoked` 的配对，并保留 `administrator` 给未来真正的管理员动作；
- `logout_all_sessions_and_rotate` 最初没有在公开仓储入口强制事件带账号 HMAC，现已补验证及共享双库合同；
- 逐会话撤销的前端最初把服务端 `403 RECENT_AUTH_REQUIRED` 与普通 403 一起压成 `request-rejected`；客户端时钟落后或请求跨过 freshness 截止点时无法 step-up。store 现在先识别稳定 code，保留当前 snapshot 且不重放 DELETE，账户安全页只导航到一次 reauth；会话列表仍按合同不要求 recent-auth；
- 会话 DELETE 的 operationId 原为容易误读的 `revokeCurrentSession`，现改为 `revokeCurrentUserSession`；OpenAPI/SDK 已在 VPS 重建，正式提交还要执行生成漂移与制品三方一致性门；
- PostgreSQL 0005 的约束名曾在静态审计中被误判；错误改成删除不存在的 `auth_sessions_authenticated_at_ms_check` 后，fresh migration fail closed。真实 catalog 证明原 `auth_sessions_check` 正是要放宽的认证时间约束，status/revocation 配对是 `auth_sessions_check7`；迁移已恢复并补语义约束回归；
- `logout-all(false)` 曾把认证快照的 session revision 作为精确 CAS；并发 GET touch 只更新 last-seen 也会令全量退出失败。事务现复核 session ID、用户、状态、期限、recent-auth 与 auth revision，不再把无害 touch 当成凭据失效，并以双库旧 snapshot 回归锁定；
- 登录错密最初直接验证所选 PHC；受限但较便宜的旧 PHC 因而会比 unknown/current dummy 更早返回，泄漏账号或 hash 代际。登录现固定执行 current-policy 校准、所选验证和动态 padding，dummy 策略在 application 构造时校验，过长输入与错误也没有廉价路径；
- 逐会话 DELETE 最初只在进入 persistence 前认证调用 session；并发 rotation/revoke、用户停用或认证版本推进可能在目标事务前让调用方失效。新事务把 actor/user/auth-state 复核和目标撤销放在同一锁边界，同时排除普通 touch revision，双库矩阵证明失效 actor 不改目标、不写事件；
- PostgreSQL actor-aware DELETE 最初用联合 `FOR UPDATE` 锁 actor/user/auth-state，再更新 target；A/B 互删可能各持有 actor 后互等。稳态写现统一为 auth-state barrier→用户→session，touch 在 barrier 后重新做完整校验；互删与 touch/DELETE、touch/rotation 共享合同不接受 SQL deadlock 或 revision conflict；
- `rate_limited` 枚举和数据库 CHECK 最初存在，但 limiter 没有原子写事件；补上 transition 事务后，通用事件 API 仍能绕过它单独写同名事件。该入口现显式拒绝，只有 account/IP/global 首次进入 durable block 的内部事务可写；并发 follower、已有 block、事件冲突回滚和截止点重开均由双库合同锁定；
- PostgreSQL migration rollback test 最初在失败后直接复用原 pool。SQLx 的失败路径可能留下 session advisory lock；同连接重入会掩盖泄漏，另一连接则可能等待至超时。四条 PostgreSQL 失败后重试分支现统一以“关闭旧 pool、保留连接参数建立新 pool”模拟进程重启，fixture cleanup 对已关闭旧 pool 的二次关闭保持幂等；
- `revokeSession` 最初只在收到 204 后处理当前 session；若提交后的响应丢失，公共 store 调用可以留下已失效的 authenticated snapshot。函数现于请求前冻结 current-target；该目标结果未知时执行一次 fail-safe logout 并立即关闭受保护 DOM，非当前目标不登出；
- Pinia 最初没有 auth-operation generation；较早的 rotation/refresh 成功响应能在 logout 后晚到并重新写入 authenticated snapshot。所有 snapshot replacement 现推进 generation，异步 projection 只做条件接纳；stale rotation 触发 cleanup，普通/fail-safe logout 又在 await 前关闭 DOM；
- 浏览器证据第一次冻结仍以 WAL snapshot 做普通只读 dump，SQLite 因此创建 sidecar；外层现先将快照切回 DELETE journal，再用 `immutable=1` dump，并在 dump 后二次拒绝 sidecar；
- 宿主 validator 原先只检查目标根写位，目录中的叶文件可被恢复写位而不触发外层重算；现在逐文件检查，并绑定目标类型、规范路径、互斥关系及临时/final SQLite sidecar；VPS 正负回放均已覆盖；
- cleanup 原先会在日志 secret scan 失败后保留污染文件并吞掉清理结果；现在先删除精确目标、写无秘密失败 marker，完成态再验证 marker、临时根、容器和网络全部不存在；
- OpenAPI 原先把两枚 Cookie 写得像单一 header，描述已明确为两个独立 `Set-Cookie` field；
- smoke 增加 Max-Age、Problem status/type、所有 Problem 零 `Set-Cookie`，以及显式成功清理响应恰好两枚清 Cookie 的协议断言；
- coordinator 起初只比较 journal 的 epoch/revision。现对同 revision 逐字段等值、同 epoch 回滚、`baseSeq` 连续性、未观察 inflight 的 terminal 和未观察 settled epoch 替换分别 fail closed；测试覆盖篡改、跳号、late join 与显式新 epoch 恢复；
- queued login 起初把自身 inflight 推进也当成外部 generation 变化，或反过来允许等待锁期间的外部 mutation 被 401/429 清掉 quarantine。现记录 `startingGeneration` 和 lease 后的 `loginGeneration`，只允许自身精确 `+1`，额外推进保持隔离；
- 浏览器 attestation 起初只记录 canonical Chromium 路径和 hash，却仍由 Playwright 默认解析 executable。现在 `chromium.launch` 显式传入同一个 `executablePath`，把实际执行文件与证据绑定。
- 第一次尝试给当前代码生成提交前归档时，Windows `git archive` 受 `core.autocrlf=true` 影响，把未强制 EOL 的文本导出为 CRLF；关键文件 hash 在编译前就不匹配，该 v3 输入因此被拒绝。历史 v4 随后改为直接归档工作树原始字节，并同时核对 archive hash、226 个文件、关键文件 hash 与 VPS 逐文件清单。
- PostgreSQL runtime smoke 第一次使用 `http://master:8080`，被配置层按设计拒绝非回环 HTTP origin；第二次把 Master 和数据库放在 internal network，宿主回环发布端口因此不可达。两次都没有进入 smoke 业务请求，清理与 secret scan 后，第三次改用独立普通 bridge、仅发布宿主 `127.0.0.1` 才通过。
- runtime smoke 的 `expectSessionInvalid` 一度仍要求通用 401 清两枚 Cookie，与现行跨标签页合同相反。现在它严格要求零 `Set-Cookie`；只有 `logout-all(false)` 和当前 session logout 的显式 204 继续调用双 Cookie 清理断言。

这些失败都是候选阶段发现的问题，不应从记录中删除；它们说明相应的回归测试和 fail-closed 分支为什么存在。

## 10. 尚未完成的门与已知边界

历史 v4 应用代码门与 v6 门工具候选测试已经完成；最终 Actions/VPS 边界修正仍须进入新的 freeze。在 C1 可标为正式完成前，还必须：

1. 把正式编译仅由公开 Actions 执行、VPS 仅测试和运行同 SHA 制品的 verifier/文档修正纳入最终 226-file archive，并完成静态与 staged-tree 预检；
2. 只把通过预检、无秘密的单父提交推到公开仓库 `main`；
3. 等待该 push 的 GitHub Actions attempt 1 产生同 SHA raw artifact，并按 commit-scoped 路径固化；
4. 从公开 `origin/main` 建立无 tracked/untracked/ignored 输入的 fresh standalone full clone；
5. 在这份 fresh clone 上运行 `tools/vps_verify.sh`，使用 Actions 二进制重复双库测试、Master/Agent smoke、双页 HTTPS rotation、旧 Cookie 零 `Set-Cookie`、logout 503/204、quarantine/recovery/stale-invalidation、许可证/SBOM校验和 secret scan；VPS 不重建正式 release/Web；
6. 把 commit、run、artifact、hash 与正式 VPS manifest 回填本页和总进度。

仍存在但不属于 C1 的债务：session/bucket/security-event retention、持久化 root-key canary 与旧 key ring、完整 Secure Cookie/HTTPS/可信代理浏览器矩阵、可控时钟与更多并发故障注入、MFA/WebAuthn/recovery。C1 的最小 HTTPS rotation 已在增量候选通过，但提交级 formal 复跑仍是本轮必过门，不能推迟到这些完整矩阵。需求矩阵仍保持 358 项 `planned`，不能用本纵切的工程测试数替代产品需求验收。
