# Rust 后端模块与函数级实施设计

> 本文件是新代码的模块合同。函数名在实现时可因 Rust 类型推导微调，但职责、transaction boundary 和错误语义不能漂移；实现进度文档将逐项链接真实文件/测试。

## 1. 技术选择

| 领域 | 选择 | 原因/约束 |
|---|---|---|
| async/runtime | Tokio | Axum/Tonic/SQLx生态统一；所有阻塞压缩/crypto/进程等待显式隔离 |
| HTTP | Axum + Tower | middleware、typed state、WS/SSE；handler 保持薄层 |
| OpenAPI | utoipa（或同等编译期 schema，骨架时锁版） | DTO 与 schema 同源，契约 diff 阻断 |
| DB | SQLx + SQLite/PostgreSQL 双 adapter | 显式 SQL/migration；所有 query 双库集成测试 |
| Agent wire | Prost + Tonic core types；WS/HTTP envelope共用 | Protobuf 版本化、Rust/TS tooling |
| serialization | Serde JSON/YAML | 领域 IR 与 producer；不直接反序列化任意 JSON到内核 |
| identity | UUIDv7 newtypes、time crate UTC | 可排序 ID、避免裸 String 混用 |
| auth | Argon2id、HMAC-SHA256、TOTP、AEAD | 密码/token/secret用途分离 |
| errors | thiserror domain/application；RFC 9457 problem response | 不向用户泄露 SQL/path/secret |
| tracing | tracing + OpenTelemetry-compatible fields | trace/task/agent/job贯穿 |
| jobs | 自建 DB durable queue | 默认不要求 Redis/Kafka |
| eBPF | Aya + tc/netlink | Rust 用户态/程序；完整功能 Linux 5.10+ |

依赖版本只在 VPS 骨架阶段从官方/registry 锁定，`Cargo.lock` 对应用仓库提交；`cargo-deny` 检查 license/advisory/source/duplicate。

## 2. 启动顺序

`nodecontroll-master serve`：

1. `Config::load()` 合并 CLI→env→config file→defaults，保留来源用于 doctor；
2. `validate_static_config()` 检查 origin、目录、DB URL、trusted proxy、key provider，不连接网络；
3. 初始化 secret redaction 和 tracing（此时不打印 secret 值）；
4. `Database::connect()`，SQLite 执行 WAL/busy timeout/foreign_keys，PG设置 statement timeout；
5. `MigrationGuard::verify_or_apply()`，阻止多个不兼容版本同时迁移；
6. `Keyring::open()`，解密 canary 验证主密钥正确；
7. `ObjectStore::verify_layout()` 与 owned temp cleanup；
8. 构建 repositories、domain services、application services；
9. `Scheduler::recover_leases()`、outbox/job workers 启动但等待 readiness；
10. `AgentHub::recover_sessions()` 清旧 ownership lease；
11. 建 Axum router、OpenAPI route、public/admin/control rate policies；
12. bind listener 后置 readiness=true；
13. shutdown 先 readiness=false，停止领取任务，drain HTTP/WS，再释放 leases/flush telemetry。

任何步骤失败进程退出非 0，错误只含 reason code和安全上下文。迁移失败不尝试启动旧 schema。

## 3. Domain 基础类型

### 3.1 `identity`

| 函数/类型 | 职责 |
|---|---|
| `EntityId<T>::new()` | 生成 UUIDv7，phantom type 防止 UserId/NodeId混用 |
| `EntityId<T>::parse()` | canonical UUID校验，拒绝 nil/非 v7（迁移 legacy id 用单独 converter） |
| `Revision::initial()/next()` | 单调 `u64`，溢出返回 domain error |
| `ContentHash::sha256(bytes)` | 统一 lowercase hex/bytes表示和常数时间比较 |
| `UtcInstant::from_unix_ms()` | 范围检查；数据库统一 UTC，展示层选择时区 |
| `ByteCount`,`Mbps`,`Percentage`,`Port` | checked constructors、无魔法负值/unlimited |

### 3.2 `state`

| 函数 | 职责 |
|---|---|
| `transition(current,event)` | 纯状态机；非法转换返回包含 allowed transitions 的错误 |
| `reconcile_condition(old,new,now)` | 状态不变保留 `since`，变化才更新时间 |
| `is_effectively_enabled(entity,parents,now)` | 合并删除/禁用/到期/套餐状态，返回 reason chain |
| `ensure_revision(expected,actual)` | API/worker optimistic concurrency 公用 |

## 4. Application transaction 模板

每个 command use case 执行固定顺序：

```rust
pub async fn execute(&self, actor: &Actor, cmd: Command) -> AppResult<View> {
    self.authorizer.require(actor, Action::X, &cmd.scope())?;
    let validated = self.validator.validate(cmd).await?;
    self.tx.run(|uow| async move {
        let current = uow.repo.load_for_update(validated.id).await?;
        ensure_revision(validated.expected_revision, current.revision)?;
        let (updated, events) = current.apply(validated)?;
        uow.repo.save(&updated).await?;
        uow.outbox.append_all(events).await?;
        uow.audit.append(AuditDraft::from_change(actor, &current, &updated)).await?;
        Ok(updated.into_view())
    }).await
}
```

Query use case 不写 audit/outbox，不开启 write transaction；所有列表必须 cursor pagination、上限和 stable sort。

## 5. 身份与安全函数

### 5.1 `auth/password.rs`

| 函数 | 行为 |
|---|---|
| `PasswordPolicy::validate(candidate,context)` | 长度/已知弱密码/与用户名相似；不强制易预测字符类别 |
| `PasswordHasher::hash(secret)` | Argon2id 参数来自配置并在 VPS benchmark；生成随机 salt |
| `PasswordHasher::verify_and_upgrade(hash,secret)` | 验证资源有界的旧 Argon2 PHC；算法、版本、m/t/p、输出或 salt 落后时生成当前 Argon2id PHC，登录事务内做 snapshot CAS |
| `LoginService::authenticate(input,request_meta)` | 归一用户名→分层限速→Turnstile→password→TOTP→session；错误外观统一 |
| `LoginThrottle::record_failure/success()` | IP prefix、username hash 和全局三桶；返回 retry_after |

### 5.2 `auth/session.rs`

| 函数 | 行为 |
|---|---|
| `SessionService::create(user,remember,meta)` | opaque 256-bit token，只存 HMAC；access/absolute expiry |
| `SessionService::authenticate(cookie)` | HMAC lookup、status/expiry/user enabled，节流更新 last_seen |
| `SessionService::rotate(session)` | privilege/password变化后防 fixation |
| `SessionService::revoke(id/revoke_all_except)` | 事务撤销并发安全；推 security event |
| `CsrfService::issue/verify()` | same-site cookie + header token，所有 cookie-auth写请求必须验证 |
| `RecentAuth::verify_password(session,proof)` | 共享 Argon2 limiter；成功只替换当前 session/CSRF，继承 absolute expiry，sibling 不变 |
| `ChangeCurrentPassword::execute(session,new_password)` | 服务端检查 freshness；原子更新 PHC/auth revision、撤销全部旧会话并给当前浏览器唯一 replacement |
| `ListActiveSessions::query(actor,now)` | 只投影未过 idle/absolute 且 auth revision 当前有效的本人会话，不返回来源原文或凭据摘要 |
| `LogoutAll::execute(session,keep_current)` | 事务内对发起 session 做 CAS；可选 replacement 继承原证明时间，普通 mutation 不能续期 recent-auth |

### 5.3 `auth/totp.rs` 与 credential

| 函数 | 行为 |
|---|---|
| `TotpService::begin_enrollment(user,password)` | 生成 encrypted pending secret和 QR payload，不立即启用 |
| `confirm_enrollment(code)` | 窗口校验、启用、生成 8 个 hash recovery codes |
| `verify(code_or_recovery)` | TOTP replay step防护；recovery 原子 consume，默认不自动关闭 TOTP |
| `disable(re_auth)` | 需要 password/TOTP或 local break-glass，撤销 recovery |
| `OpaqueToken::generate()` | 256-bit；显示一次 |
| `TokenStore::hash/lookup/rotate/revoke()` | token type+audience+scope+subject绑定 |

### 5.4 `auth/authorization.rs`

| 函数 | 行为 |
|---|---|
| `Authorizer::decide(actor,action,resource)` | role+scope+ownership+resource state，返回 allow/deny reason |
| `require_admin/require_scope/require_owner()` | application 层组合器 |
| `explain_entitlement(user,resource)` | 文件 ACL、套餐、公开状态的可解释权限链 |
| `sanitize_view(actor,entity)` | secret/private fields 由 view builder剔除，不由 handler手工删 |

## 6. 用户与套餐函数

### 6.1 `users`

| use case | 职责/副作用 |
|---|---|
| `CreateUser::execute` | 唯一用户名/email、密码 hash、role、可选套餐；发 UserCreated |
| `UpdateUserProfile::execute` | 昵称/email/备注/品牌字段；不改变 principal label |
| `SetUserStatus::execute` | disabled/enabled + reason；触发 session撤销、套餐/route reconcile |
| `ChangePassword::execute` | old password/re-auth、rehash、session policy |
| `AdminResetPassword::execute` | high-risk audit，生成一次性临时密码/强制下次修改 |
| `DeleteUser::execute` | 软删除、撤销凭据/session、暂停数据面；账本不删 |
| `RotateSubscriptionToken::execute` | 新 token返回一次；旧 token grace可配置 |
| `ListUsers::query` | cursor/filter/status/package/traffic summary；禁止 N+1 |

### 6.2 `packages`

| 函数 | 职责 |
|---|---|
| `CreatePackageTemplate` | 验证周期/流量/计费/默认限速和节点选择器 |
| `RevisePackageTemplate` | 新 revision；选择是否传播到现有实例，先返回 impact preview |
| `InstantiatePackageForUser` | snapshot template、生成 principal credential/baseline、entitlement outbox |
| `RenewPackageInstance` | append renewal event，计算新窗口，不覆盖旧周期 |
| `Pause/Resume/ExpirePackage` | reasoned transition + data-plane reconcile |
| `AssignNodes/Tags` | 静态 ID与动态 tag selector，返回 effective node set/diff |
| `effective_speed_policy(user,node,package)` | 实现五级优先级，返回值+来源+revision |
| `effective_billing_policy(instance,node,direction)` | 倍率、计费方向、周期与 billing point |
| `EvaluatePackageState::run(now)` | 超限/到期/阈值/迟滞；输出 policy events，幂等 |

## 7. 节点、服务器与内核函数

### 7.1 `servers`

| 函数 | 职责 |
|---|---|
| `RegisterServer` | server entity + enrollment grant，一次显示 token |
| `UpdateServerMetadata` | 名称、地区、provider、renewal、public projection |
| `RotateEnrollment/DeviceCredential` | grant/cert 生命周期，旧连接 fencing |
| `SetServerConnectionMode` | callback验证、模式前置、desired connection policy |
| `Disable/DeleteServer` | 影响预览；disable保留数据面策略可选，delete延迟 GC |
| `AdoptDiscoveredService` | discovery snapshot→claim plan→确认→managed resource |
| `SelectTrafficSource` | 新 baseline epoch，不改历史 source |
| `ReconcileServer` | desired/report/capability diff→最新 job graph |

### 7.2 `nodes`

| 函数 | 职责 |
|---|---|
| `ParseNodeInput` | URI/base64/YAML格式探测→NodeIR candidates+diagnostics，纯函数/fuzz |
| `FetchAndPreviewNodes` | SSRF-safe fetch→parse→filter→selection session |
| `CreateExternalNode` | 验证 client IR、fingerprint去重、order/tag |
| `CreateManagedInbound` | server capability/port/cert/credentials→inbound + paired node + config revision |
| `UpdateNode` | origin-kind字段策略、impact preview、revision |
| `DeleteNode` | paired inbound/tunnel/route/package/template引用图处理 |
| `BatchCreate/Rename/Tag/Enable/Delete` | 每项结果+原子/partial模式显式；默认全事务 |
| `ReorderNodes` | stable ordered IDs、完整集合/partial move校验 |
| `SwitchAddress/RestoreAddress` | original/resolved/override有类型状态，不覆盖原值 |
| `NodeFingerprint::compute` | protocol+canonical endpoint+credential-safe fingerprint；secret用HMAC |
| `ConvertInboundToClientIR` | server IR→客户端无损中间模型，producer前唯一转换入口 |
| `CreateTunnel/DeleteTunnel` | topology/cycle/port/ref validation + paired lifecycle |

### 7.3 `core`

| 函数 | 职责 |
|---|---|
| `BuildServerConfigIR::execute(server_id)` | 查询 managed in/out/route/users/certs/API→deterministic IR |
| `ValidateCapabilities::execute(ir,reported)` | 返回逐 path supported/warning/error，不只 bool |
| `CompileSingBox::compile(ir,target_version)` | stable canonical JSON + artifact references + secret manifest |
| `PlanConfigChange(old,new)` | semantic diff、reload impact、affected users/nodes |
| `RequestApplyConfig` | 更新 desired revision + dedup reconcile job |
| `HandleApplyResult` | fencing revision、写 reported/conditions、SSE/audit |
| `RequestCoreControl` | start/stop/reload/restart/install/upgrade/rollback typed jobs |

## 8. 路由、出站与 WARP 函数

| 函数 | 职责 |
|---|---|
| `CreateOutbound/UpdateOutbound/DeleteOutbound` | typed direct/block/proxy/wireguard/selector/urltest；引用图 |
| `CompileWarpAccount` | Agent 返回 WireGuard material→secret ref + v4/v6 endpoint IR |
| `CreateRouteRule` | 条件/action schema、normalize values、first-match order |
| `MoveRouteRule` | revision + sparse/stable order；系统规则锁定 |
| `AnalyzeRouteTable` | unreachable/catch-all shadow/empty group/cycle/capability diagnostics |
| `SimulateRoute(metadata)` | 与 compiler同一 matcher，返回每条 pass/fail和 final outbound |
| `CreateBalancer` | strategy/candidates/probe参数；编译 selector/urltest/scheduler plan |
| `ReconcileBalancerSelection` | 1.14 events→random/round-robin/least-load selection，幂等API task |
| `CreateNodeRoutedOutbound` | parent inbound→catch-all route+child node refs |
| `CreateUserRoutedOutbound` | quota→new principal credential→user route→subscription child node，单事务 |
| `EvaluateRoutedOutboundState` | 用户/套餐禁用/到期/超限时 pause/resume，reasoned reconcile |

## 9. 外部订阅、模板与发布函数

### 9.1 fetch/sync

| 函数 | 职责 |
|---|---|
| `EgressPolicy::resolve_and_validate(url)` | scheme/port/DNS/redirect/IP range/rebinding/allowlist |
| `SafeHttpClient::fetch(request,budget)` | timeout/size/decompression ratio/ETag/content-type/redirect逐跳检查 |
| `SyncExternalSubscription` | fetch→parse→filter→match plan→apply/last-good→traffic header |
| `MatchNodeCandidates` | fingerprint/name/endpoint策略，输出 confidence/ambiguity |
| `ScheduleExternalSyncs` | next_due + jitter + lease；同 source 单飞 |
| `MaterializeProvider` | source snapshot→transform/filter/Geo→content hash/cache metadata |

### 9.2 template/rule

| 函数 | 职责 |
|---|---|
| `ValidateTemplate` | schema、组名唯一、引用、type fields、placeholder、DAG无环 |
| `MergeTemplate` | base + selected nodes/providers/rules，deterministic conflict policy |
| `CompileProxyGroups` | include-all/filters/order/dialer chain/capability warning |
| `FetchRuleSource` | safe fetch/local object→parse→hash→last-good |
| `CompileRules` | custom + rule templates + provider refs，target producer语义 |
| `RunTransformScript` | WASM/JS sandbox adapter，fuel/memory/time/no network/default deny |

### 9.3 publish

| 函数 | 职责 |
|---|---|
| `AuthenticateSubscriptionRequest` | token/shortcode/type/audience/expiry/user/file/package/silent mode |
| `BuildSubscriptionIR` | entitlement snapshot→nodes/groups/rules/providers/info，同 revision key |
| `DetectOutputFormat` | explicit query/path > file policy > safe UA map > default |
| `ProduceSubscription` | producer registry、format capability/warnings/golden deterministic |
| `BuildSubscriptionHeaders` | ETag/cache/content-disposition/userinfo/warnings，无 header injection |
| `SubscriptionCache::get_or_build` | subject+revision+format scoped singleflight；secret-safe eviction |
| `CreateTemporarySubscription` | DB hash code、selection snapshot、expiry/max uses/rate policy |

## 10. Traffic/limits 函数

| 函数 | 职责 |
|---|---|
| `IngestMetricBatch` | agent/epoch/seq唯一、解压预算、schema/time skew、raw append |
| `CounterDelta::derive(previous,current)` | monotonic delta；下降/boot/revision开启新 epoch |
| `ResolveTrafficPrincipal(label,server,inbound)` | stable label mapping；unknown进入隔离队列而非丢弃 |
| `AttributeTraffic` | raw dimension→package/node/billing point；multi-hop去重 |
| `AppendLedgerEntries` | raw/billed/direction/multiplier revision，唯一 source event |
| `ApplyTrafficAdjustment` | signed amount+reason/actor；append-only |
| `ResetTrafficBaseline` | baseline event，不删/改 raw/ledger |
| `RebuildAggregates(range,version)` | shadow表计算→checksum→切换；可重入 |
| `QueryTrafficSummary/Series/Breakdown` | 明确 source/raw/billed/timezone/bucket，限制点数 |
| `EvaluateEnforcementPolicies` | speed/connection/IP/overlimit effective desired policies |
| `HandleEnforcementReport` | per policy applied/degraded/reason/capability/revision |

## 11. 证书、站点、测速与公开探针函数

### 11.1 certificates

| 函数 | 职责 |
|---|---|
| `CreateAcmeAccount` | directory/email/key secret ref，接受条款时间 |
| `CreateCertificateOrder` | domain/SAN/wildcard/provider/target验证，workflow启动 |
| `PresentDnsChallenge/CleanupDnsChallenge` | provider adapter，最小权限secret，传播轮询 |
| `FinalizeAndValidateCertificate` | chain/domain/time/key match/PEM limits |
| `PlanCertificateDeployment` | server targets/path/mode/reload service/ownership |
| `RenewDueCertificates` | distributed lease+jitter+expiry priority+failure alert |
| `UploadCertificateWebhook` | scoped token、PEM/key match、idempotency、deploy plan |

### 11.2 sites

| 函数 | 职责 |
|---|---|
| `CreateStaticSite/CreateReverseProxy` | typed model、domain/port/path/upstream/WS/TLS校验 |
| `RenderNginxSite` | canonical include-only config、ownership marker/hash |
| `PlanSiteChange/DeleteSite` | impact/ref/path ownership，nginx -t + rollback job |
| `PublishStaticArtifact` | archive traversal/symlink/size/file-count检查，content address stage |

### 11.3 speed/probe

| 函数 | 职责 |
|---|---|
| `PairTester` | one-time grant→tester identity/capabilities |
| `CreateSpeedTestRun` | source/nodes/test types/threads/budget→parent+child jobs |
| `DispatchNextSpeedTask` | per source serial semaphore、cancel/deadline |
| `RecordSpeedResult` | latency samples/speed/bytes/duration/exit IP/executor version/history |
| `BuildPublicProbeSnapshot` | per-field allowlist/projection，missing保持 null/omit |
| `QueryProbeSeries` | allowed metric/range/bucket/max points |
| `StreamProbeSnapshots` | bounded broadcast、5s tick/变化合并、慢客户端断开 |

## 12. Telegram、MCP 与实例联合函数

### 12.1 Telegram

| 函数 | 职责 |
|---|---|
| `ValidateTelegramInitData` | HMAC签名、auth_date窗口、replay nonce、bot audience |
| `BindTelegramAccount` | re-auth/one-time binding code，TG id唯一 |
| `HandleBotUpdate` | update id去重、command parse→同 application service |
| `AuthorizeBotCommand` | user/admin role + action；不信任 command菜单 |
| `BuildDailyDigest` | 用户/管理员不同 projection，timezone+dedupe |

### 12.2 MCP

| 函数 | 职责 |
|---|---|
| `McpSession::initialize` | protocol/capability/tool list negotiation |
| `ToolRegistry::list/call` | compile-time allowlist、JSON schema验证、scope mapping |
| `CreateHighRiskIntent` | 规范化参数 hash、actor/tool/expiry→一次性 intent token |
| `ConfirmAndExecuteIntent` | token+相同参数+session/re-auth，原子 consume |
| `MapAppErrorToMcp` | 结构化、安全、可恢复信息；不泄露内部日志 |

### 12.3 federation

| 函数 | 职责 |
|---|---|
| `CreateInstanceIdentity/RotateIdentity` | self-hosted CA/Ed25519 identity与pin |
| `IssueShareGrant` | owner server/scopes/prefix/quota/expiry；token HMAC，只显示一次 |
| `AcceptShareGrant` | TLS/pin/instance compatibility确认，保存 peer projection |
| `AuthorizeChildRequest` | grant+origin ownership+resource prefix+method scope；禁止转授 |
| `CreateChildInbound/DeleteChildResource` | owner transaction→正常 Agent reconcile，标 consumer owner |
| `RevokeShareGrant` | 拒绝新请求，按策略 pause/delete consumer resources，审计 |
| `SyncPeerProjection` | cursor/ETag、只读状态，不让消费方伪造 Agent report |

## 13. Jobs、outbox、audit 与 notifications

| 函数 | 职责 |
|---|---|
| `JobQueue::enqueue_unique` | type+idempotency唯一，已有 terminal返回结果/新 revision supersede |
| `JobQueue::lease_next` | `FOR UPDATE SKIP LOCKED` PG；SQLite短 write tx，capability/priority/due过滤 |
| `JobQueue::heartbeat/complete/fail/cancel` | lease token fencing、attempt/backoff/dead-letter |
| `Scheduler::tick` | schedule definitions + DB clock + jitter + singleton lease |
| `Workflow::advance` | step state/compensation，crash后从数据库恢复 |
| `OutboxDispatcher::dispatch_batch` | lock rows→handler→dedupe receipt→mark；业务 handler幂等 |
| `AuditService::append` | action/resource/before-after hash/安全 diff/request meta，append-only |
| `NotificationRouter::route` | event→channel/user preference/template/dedupe |
| `DeliveryWorker::send` | timeout/retry/provider result/dead-letter；不阻塞业务 |

## 14. API delivery 规则

Handler 只允许：extract/parse DTO → 调 application query/command → map view/status。禁止 handler 直接调用 repository、Agent hub 或 SDK。

| 通用函数 | 职责 |
|---|---|
| `request_context_middleware` | request/trace id、client IP可信代理解析、deadline |
| `session_auth_middleware` | session/API token二选一 actor；公共 route显式无 actor |
| `csrf_middleware` | 仅 cookie-auth unsafe method |
| `rate_limit_middleware` | route policy+actor/IP/token key |
| `idempotency_middleware` | unsafe create/action端点 request hash/response replay |
| `body_limit/decompression_guard` | route type大小与 ratio限制 |
| `problem_response` | RFC 9457 type/title/status/detail/instance/code/fields/trace_id |
| `etag/if_match` | entity revision强制；冲突返回 current revision |
| `cursor_page` | opaque signed cursor、limit默认/最大、stable keys |

## 15. 配置与 feature flags

系统配置按 scope 分：immutable startup、instance setting、user preference、integration secret。所有 setting 有 typed key、schema version、default、sensitivity、restart/reconcile effect。功能开关只控制本地行为，绝不由签名许可证载入。

`Config::load()` 优先级：CLI > `NC_*` env > YAML/TOML file > default；`doctor config --explain` 显示来源但 secret 只显示 `<set>`。未知 key 默认错误；deprecated key 给迁移期限。

## 16. 测试与质量门

- 每个纯 policy/state/IR compiler 使用 table/property tests；parser 使用 cargo-fuzz corpus；
- application tests 用 fake ports 断言 transaction/outbox/audit，不依赖 HTTP；
- repository contract suite 对 SQLite/PG 两 adapter 跑同一用例；
- API contract 测 status/problem/schema/ETag/idempotency/auth，不用 snapshot掩盖字段；
- Agent executor 在 rootless fake 和专用 privileged VPS container/VM分别测试；
- `cargo fmt --check`、Clippy `-D warnings`、nextest、doc tests、deny/audit、MSRV/locked build均在 VPS；
- 禁止新增 `unwrap/expect/panic!` 于请求/worker/Agent输入路径；测试和不可达 invariant 需 lint allow+说明；
- 关键 cryptographic/session/token/route/ledger模块要求 mutation/property test 或等价强化验证。
