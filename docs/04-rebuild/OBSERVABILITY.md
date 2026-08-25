# 可观测性、SLO 与故障诊断

## 1. 观测模型

系统必须回答四个问题：用户请求是否成功；后台/远端任务是否真正落地；desired 与 reported 是否一致；流量/限制/账本是否连续可信。只收集 CPU/HTTP 500 无法回答后两项。

所有组件使用统一关联字段：`request_id`（HTTP）、`trace_id/span_id`、`job_id`、`task_id`、`server_id`、`resource_type/id`、`config_revision`、`traffic_epoch`。字段进入 structured log/trace 和受权限查询，但高基数字段不进入 Prometheus label。

三种事实分开：

- operational state：当前 health/reported/queue，允许覆盖更新；
- domain history：job/task/config/cert/sync/connection/ledger 等持久记录；
- telemetry：metrics/logs/traces，有保留期，不能替代审计/账本。

## 2. 健康端点

| Probe | 条件 | 不检查 |
|---|---|---|
| `/healthz` liveness | event loop/进程能响应 | DB、Agent、外部网络 |
| `/readyz` readiness | config/secret、DB 可读写、migration compatible、critical worker lease | 任一 Agent 离线、外部 source/TG 失败 |
| startup | 必需目录/key/DB/schema/OpenAPI/assets 一致 | 远端服务器全在线 |
| server health | heartbeat、clock、Agent/core/service、disk/capability/drift/ingest | 只用单个 boolean |

readiness 返回机器 code 和依赖摘要但不泄露 DSN/path/secret；公网 reverse proxy 通常只暴露 liveness 的最小响应。

## 3. 日志合同

JSON 字段：timestamp UTC、level、component/module、event、message、request/trace/job/task、安全 actor/resource ID、outcome/error code、duration。message 给人读，查询和告警依赖稳定 `event/code`。

禁止：password/token/session/cookie/Authorization、subscription URL/token、协议 credential、private key、完整 config/body、Telegram initData、MCP bearer、数据库 DSN。URL 结构化为 scheme/host/path-template，query 全部丢弃或 allowlist。

Master、Agent、sing-box、Nginx 日志进入各自 source。sing-box/Nginx 原文先限长、去 ANSI 和 central redact；Agent 回传用 cursor/bytes quota，不能因日志淹没 task channel。debug 临时开启有 TTL/audit，仍不解除 secret redaction。

## 4. 指标目录

### 4.1 HTTP 与认证

- `nodecontroll_http_requests_total{route,method,status_class}`
- `nodecontroll_http_request_duration_seconds{route,method}`
- `nodecontroll_http_in_flight_requests`
- `nodecontroll_auth_attempts_total{method,outcome,reason_class}`
- `nodecontroll_auth_sessions_active`、`rate_limit_decisions_total{surface,outcome}`

route 是模板而非原始 path；不含 user/IP/token。

### 4.2 Jobs、outbox 与 Agent

- `jobs_total{kind,outcome}`、`job_duration_seconds{kind}`、`jobs_ready{kind}`、`job_oldest_ready_seconds{kind}`
- `outbox_pending{destination}`、`outbox_oldest_seconds{destination}`
- `agents_by_state{state,mode}`、`agent_heartbeat_age_seconds`（聚合 histogram）
- `agent_tasks_total{kind,outcome,error_class}`、`agent_task_duration_seconds{kind}`
- `agent_protocol_messages_total{direction,type,outcome}`、`agent_clock_offset_seconds`

单 server 详情来自数据库/日志，不用 `server_id` label 制造高基数。

### 4.3 Core、配置与限制

- `core_process_state{version_track,state}`、`core_restarts_total{reason}`
- `config_compiles_total{outcome,error_class}`、`config_deploys_total{outcome}`、`config_drift_resources{kind}`
- `connections_current{protocol}`、`connections_total{protocol,outcome}`
- `traffic_ingest_bytes_total{direction,source}`、`traffic_ingest_lag_seconds{source}`、`traffic_epoch_changes_total{reason}`
- `limit_principals_by_state{kind,state}`、`limit_enforcement_errors_total{kind,error_class}`
- `tc_map_entries`、`tc_classifier_misses_total{reason}`、`tc_dropped_or_shaped_bytes_total{direction}`

用户/节点账务数不作为 metric label；详细 series 由 `traffic_aggregates` 查询。

### 4.4 订阅与外部网络

- `source_syncs_total{format,outcome,error_class}`、`source_sync_duration_seconds{format}`、`source_items{state}`
- `safe_http_requests_total{purpose,outcome,deny_reason}`、`safe_http_response_bytes{purpose}`
- `subscription_publishes_total{target,outcome}`、`subscription_publish_duration_seconds{target}`
- `subscription_artifact_bytes{target}`、`subscription_cache_requests_total{target,outcome}`
- `subscription_downloads_total{target,status_class}`、`subscription_download_bytes_total{target}`
- `parser_diagnostics_total{format,severity,code}`、`script_runs_total{outcome,reason}`

### 4.5 证书、站点、测速与集成

- `certificates_by_expiry_window{window}`、`certificate_operations_total{kind,outcome,provider}`
- `site_deploys_total{outcome}`、`site_validation_duration_seconds`
- `speed_tests_total{kind,outcome}`、`speed_test_duration_seconds{kind}`、`speed_test_queue_depth`
- `notification_deliveries_total{channel,outcome}`、`notification_queue_oldest_seconds{channel}`
- `mcp_invocations_total{tool_class,outcome,confirmation}`
- `federation_messages_total{direction,type,outcome}`、`federation_peers_by_state{state}`
- `backups_total{kind,outcome}`、`backup_age_seconds`、`restore_rehearsal_age_seconds`

provider/tool names 需 bounded registry；用户自定义文本不能成为 label。

### 4.6 进程和存储

Rust process/runtime、CPU/RSS/FD、DB pool/latency/locks、SQLite WAL/PG connectivity、object operation/bytes、disk/inode 由 bounded metrics 提供。对象/表名使用 enum。不得暴露 SQL statement/raw query label。

## 5. Trace 与跨组件传播

OpenTelemetry spans 覆盖 HTTP→use case→repository/job enqueue；worker→job steps→Agent task；Agent→compile/file/service/core API；source fetch→parse→publish。异步通过 trace link，不伪造长达数小时的单个 open span。

HTTP 接受/生成 W3C trace context，但匿名/第三方 header 不能决定 sampling/security。Master→Agent envelope 带 trace context 且签名；Agent 不把外部请求 trace 直接转进 privileged helper。

span attribute allowlist，request body/config/URI/credential 不入 span。错误记录稳定 code/class，完整安全摘要回 domain event；外部 exporter 默认关闭且用户自行配置。

## 6. 初始 SLI/SLO

以下是工程目标，必须在 P7 用 VPS 负载和故障测试校准，不是当前已达到的承诺：

| 能力 | SLI | 初始目标/窗口 |
|---|---|---|
| 已认证 API 可用性 | 非预期 5xx/timeout 之外成功请求 | 99.9% / 30d |
| 订阅下载可用性 | 有效 token 返回 2xx/304 或明确 last-good | 99.95% / 30d |
| API 延迟 | 非导出 GET/command accept p95 | 300ms/500ms |
| Job 接收 | accepted job 5s 内被 worker claim | 99%（依赖 online capacity） |
| 在线 Agent task | 非长任务从 dispatch 到 ack | p95 < 5s WS/HTTP；pull 按 interval |
| 流量新鲜度 | online core sample 到可查询聚合 | p95 < 60s |
| 限制状态可信 | 有限制 principal 为 enforced 或明确 degraded | 100%，禁止 unknown 当 enforced |
| 证书安全窗 | 到期前 7d 无未处理 critical | 100% |
| 备份 | 计划备份按期且 read-back 成功 | 99%，并看 RPO |
| 恢复演练 | 最近成功年龄 | ≤ 30d（生产建议） |

维护窗口、用户错误 4xx、Agent 明确离线、外部 provider outage 分别统计，不从 SLI 随意排除。排除规则固定代码/事件，记录 error budget。

## 7. Dashboard

1. Control plane：RPS/latency/errors/readiness、DB/object、worker/job/outbox、resource saturation。
2. Fleet：Agent states/modes/versions/clock/heartbeat、core versions/crash/drift、server resource/disk。
3. Traffic enforcement：ingest lag/epochs/deltas、connections、limits states、tc misses/degraded。
4. Subscriptions：source sync/error/diff、parse diagnostics、publish/cache/download、artifact sizes。
5. Edge：cert expiry/renew、Nginx site deploy/validation、public probe/speed abuse/error。
6. Integrations/security：auth/rate limit/security events、Telegram/MCP/federation delivery、backup/restore。

UI dashboard 使用业务 API projection，不直接查询 Prometheus；Grafana/外部 dashboard 用 metrics。每个 panel 链到 runbook 和 filtered job/log view。

## 8. 告警与抑制

page 级：Master 不 ready、DB/secret不可用、订阅大面积失败、Agent fleet 异常、core crash loop、limit enforcement false success、traffic gap、证书近到期、backup/restore损坏、制品/签名/审计完整性异常。

ticket 级：单 source/notification/peer 持续失败、disk 80%、job retry 增多、版本过旧、规则/模板 warning。单 server/node/user问题按对象通知，不触发全局 page。

告警有 `for`、dedupe key、severity、owner、runbook、silence limit。maintenance job 自动附 maintenance context，但 security/integrity/disk critical 不被静默。通知渠道失败有独立 fallback/本地持久告警。

## 9. Runbook 模板

每个告警文档包含：用户影响、触发公式、常见/危险原因、只读诊断、需要的 scope、临时 containment、恢复、数据一致性验证、回滚和升级条件。命令只引用 typed CLI，不建议执行任意 SQL、删除 state、清空 qdisc 或跳过签名。

首批 runbooks：Master not-ready、SQLite busy/WAL/PG pool、job lease stuck、Agent offline/clock/cert、core crash/reload rollback、config drift、traffic epoch/gap、tc degraded、source SSRF deny/parser failure、订阅 last-good、ACME/DNS、Nginx rollback、disk full、backup/restore、audit gap、secret suspected leak。

## 10. 保留、成本与隐私

默认建议：debug log 3d、info 14d、operational metrics 30d、trace 7d sampled、connection detail 7d、raw traffic 30d、aggregates 按账务/管理员 policy、audit/ledger更长且可归档。实际设置在部署向导中明确，cleanup job 可观测/审计。

采样：error/security/remote mutation spans 100%，普通读请求 tail/head bounded；不得因用户 ID sampling。日志预算按组件 bytes/sec，超限先 drop debug 并发出 counter，不阻塞核心 task/traffic ledger。

## 11. 故障注入和验收

- kill/restart Master/worker/Agent/sing-box/Nginx；DB lock/PG failover/object timeout/disk 90%；时钟偏移/网络分区/消息重复乱序。
- source slow/redirect/rebinding/oversize；Telegram/DNS/ACME/MCP/federation 429/5xx/timeout。
- core reload epoch、Agent upgrade、tc helper失败、BTF/permission缺失。
- 验证用户可见状态、指标、log/trace correlation、告警、runbook、恢复和无 secret。

完成定义：所有关键 user journey 可从 request/job/task/config/traffic epoch 关联；每个 SLI有可计算 query和测试；所有 page alert 在故障注入时触发并在恢复后清除；canary secret 扫描为零；高基数/日志量在压测预算内。
