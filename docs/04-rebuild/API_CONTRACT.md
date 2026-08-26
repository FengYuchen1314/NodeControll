# HTTP、事件与控制面 API 合同

## 1. 合同边界

本文件定义浏览器、订阅客户端、Telegram Mini App、MCP adapter、远程 tester 与第三方自动化所见的稳定边界。Master 与 Agent 之间的协议以 [AGENT_PROTOCOL.md](./AGENT_PROTOCOL.md) 为准；Master 对 sing-box 的管理不是公开 API。

API 前缀为 `/api/v1`，内容类型默认 `application/json; charset=utf-8`。破坏性变更只进入新 major；同一 major 只能增加可选字段、枚举值和端点。浏览器不得依赖未知字段被剔除，服务端不得静默接受未知的安全敏感字段。

## 2. 通用请求与响应

### 2.1 标识、时间、数值

- ID 是 canonical UUIDv7 string；公开订阅 token、enrollment token 和 secret 不复用数据库 ID。
- 时间字段以 `_at` 结尾，使用 RFC 3339 UTC；持续时间以 `_ms` 结尾；流量以 `_bytes` 结尾并序列化为十进制字符串，避免 JavaScript 53 位精度损失。
- 速率是整数 `bytes_per_second` 或 `mbps`，金额/倍率使用整数最小单位或 numerator/denominator，不使用浮点账务。
- 所有资源返回 `revision`、`created_at`、`updated_at`；可编辑资源返回 `ETag: "<id>:<revision>"`。

### 2.2 Envelope

成功单体响应：

```json
{"data":{"id":"019...","revision":4},"meta":{"request_id":"req_..."}}
```

列表响应：

```json
{
  "data": [],
  "page": {"limit": 50, "next_cursor": null, "has_more": false},
  "meta": {"request_id": "req_..."}
}
```

失败响应采用 Problem Details 语义，并为表单提供稳定字段错误：

```json
{
  "type": "urn:nodecontroll:problem:revision-conflict",
  "title": "Revision conflict",
  "status": 409,
  "code": "REVISION_CONFLICT",
  "detail": "Resource changed since it was loaded",
  "request_id": "req_...",
  "errors": [{"pointer":"/name","code":"already_exists","message":"Name is in use"}]
}
```

`detail/message` 可本地化但 `code` 永远稳定。不得把 SQL、栈、远端 stderr、密钥或完整上游响应写入客户端错误。

### 2.3 查询、分页和筛选

- 列表统一支持 `limit`（默认 50，最大 200）和不透明 `cursor`；排序必须包含 ID 作为稳定尾键。
- 文本搜索使用 `q`；状态使用重复 query（如 `status=online&status=degraded`）；时间窗用 `from`/`to`。
- 明确允许的 `sort` 枚举才可排序，不把列名原样拼接为 SQL。
- 导出是 job，不通过无限分页拼装；小型列表可 `?format=csv`，上限 10,000 行并记录 audit。

### 2.4 幂等、并发和异步任务

- POST 创建、执行、导入和支付式账本操作接受 `Idempotency-Key`；同一 actor + route + key 保留 24 小时，body hash 不同返回 409。
- PATCH/PUT/DELETE 必须有 `If-Match`；缺失返回 428，revision 不一致返回 409。明确的 append-only 命令（流量调整、撤销）用幂等键而不是 revision。
- 预计超过 2 秒、涉及 Agent、文件、批处理或外部网络的操作返回 `202` 和 `job_id`。job 状态是 `queued/running/waiting/succeeded/failed/cancelled/expired`。
- 删除默认是可恢复软删除；密钥、会话、Agent 凭据立即 revoke。真正 purge 单独端点，要求二次确认和审计权限。

### 2.5 状态码

| 状态 | 含义 |
|---|---|
| 200/201/202/204 | 查询或命令成功/已创建/异步接收/无正文 |
| 400 | JSON 或跨字段校验失败 |
| 401/403 | 未认证/已认证但 scope 或资源关系不足 |
| 404 | 不存在，或调用者不应获知其存在 |
| 409 | revision、幂等、唯一性或当前状态冲突 |
| 410 | 订阅、分享、邀请等凭据已撤销/过期 |
| 412/428 | 前置条件失败/缺少 `If-Match` |
| 422 | 语法有效但配置无法编译/协议组合不受支持 |
| 429 | 登录、公开订阅、探针、MCP 或管理调用限速 |
| 502/503/504 | 受控外部依赖错误/不可用/超时；携带可重试提示 |

## 3. 认证、角色和 scope

### 3.1 凭据

| 表面 | 凭据 | 保护 |
|---|---|---|
| 管理 SPA | HttpOnly、Secure、SameSite=Lax session cookie + CSRF double submit | 登录/TOTP/WebAuthn，session rotation，origin check |
| 自动化 API | `Authorization: Bearer nc_...` personal/service token | 只存 hash；scope、IP/CIDR、到期、last-used |
| 订阅下载 | path token 或 `Authorization`；可选 username/password | constant-time hash，速率限制，撤销与到期 |
| tester/traffic ingest | 独立配对 token + request signature/nonce | 不接受管理员 cookie/token |
| Telegram | Bot webhook secret；Mini App `initData` 验签 | user/chat 绑定、短 TTL、重放防护 |
| MCP | 独立 OAuth/API token | tool allowlist、危险操作显式确认 |
| federation | 双向实例凭据 + 签名 envelope | 对端 pin、scope、expiry、replay window |

### 3.2 内置角色

- `owner`：实例所有权、密钥轮换、恢复、联合和所有授权。
- `admin`：除所有权/根密钥外的日常管理。
- `operator`：服务器、内核、节点、路由、证书、站点和任务。
- `support`：用户/订阅排障，默认不可见凭据明文和系统密钥。
- `auditor`：只读设置、指标、任务和审计。
- `member`：只访问自己的套餐、流量、设备、订阅和公开节点投影。

细粒度 scope 采用 `resource:verb`，如 `servers:read`、`servers:execute`、`users:write`、`traffic:adjust`、`secrets:rotate`。授权必须同时检查 scope、对象所属关系和当前资源状态；前端隐藏按钮不算授权。

## 4. 身份、实例与用户端点

| Method/path | 作用 | 特殊合同 |
|---|---|---|
| `GET /bootstrap` | 是否已初始化、公开品牌、受支持登录方式 | 不泄露管理员/数据库信息 |
| `POST /bootstrap` | 首位 owner、实例和恢复码 | 仅空实例；一次性原子事务 |
| `POST /auth/login` | 密码第一阶段 | 可能返回 `challenge_id`，响应恒定化防枚举 |
| `POST /auth/reauth` | 对当前会话补做近期认证 | C1 支持密码；成功只轮换当前 session/CSRF，不延长 absolute expiry，不影响 sibling |
| `POST /auth/challenges/{id}/verify` | TOTP/WebAuthn/恢复码 | 成功 rotation session |
| `POST /auth/logout` / `POST /auth/logout-all` | 撤销当前/全部 session | logout-all 保留当前与否须显式字段 |
| `GET/PATCH /me` | 当前资料、语言、时区 | email/username 变更需重新认证 |
| `POST /me/password` | 自助修改当前密码 | 要求近期认证；推进 auth revision，撤销全部旧会话，仅给当前浏览器创建 replacement |
| `GET /me/sessions`、`DELETE /me/sessions/{id}` | 查看活动会话、撤销本人的指定会话 | 删除要求近期认证；只返回 ID、保证级别和粗粒度时间，不返回 token、HMAC、原始 IP/UA |
| `GET/POST/DELETE /me/totp`、`GET/POST/DELETE /me/webauthn` | MFA 生命周期 | setup secret 只返回一次 |
| `GET/POST/DELETE /me/tokens` | 个人 token | 创建只回显一次 token |
| `GET /instance`、`PATCH /instance` | 名称、品牌、locale、公开 URL | 资产上传走 object API |
| `GET/PATCH /settings/{section}` | 分区设置 | schema-versioned DTO，敏感设置引用 secret |
| `GET/POST /assets`、`GET/PATCH/DELETE /assets/{id}` | logo/favicon/background | MIME magic、尺寸、quota 校验 |
| `GET/POST /users`、`GET/PATCH/DELETE /users/{id}` | 用户生命周期 | 删除时撤销订阅/session；账本保留 |
| `POST /users/{id}/restore` / `POST /users/{id}/purge` | 恢复/清除 | purge 是高危异步 job |
| `POST /users/{id}/reset-password` | 管理员触发一次性重置 | 不返回新密码；用户下次登录设置 |
| `GET/POST/DELETE /users/{id}/tokens` | 管理用户 API token | 明文只创建时返回 |
| `GET /users/{id}/effective-policy` | 解释最终套餐/策略/限制 | 返回来源链和冲突解决结果 |

C1 的近期认证、改密和 logout-all 精确 rotation 语义以 [WP02-C 认证安全合同](../05-implementation/WP02_C_AUTHENTICATION_SECURITY_CONTRACT.md) 为准。认证方法与保证级别不能混用：方法是 `password/totp/webauthn/recovery_code`，会话保证级别是 `password/mfa/phishing_resistant/recovery`。`force_password_change=true` 时，后端 use case、router guard 和 App DOM gate 使用同一白名单；角色 projection 不能继续暴露普通产品 scope。

## 5. 套餐、entitlement 与流量账本

| Method/path | 作用 |
|---|---|
| `GET/POST /packages`、`GET/PATCH/DELETE /packages/{id}` | 套餐 CRUD、流量/日期/设备/节点/速率/并发限制 |
| `POST /packages/{id}/clone`、`POST /packages/reorder` | 克隆与稳定排序 |
| `GET/POST /users/{id}/entitlements`、`PATCH/DELETE /users/{id}/entitlements/{eid}` | 一个用户多套餐绑定与覆盖项 |
| `POST /entitlements/{id}/pause`、`resume`、`reset-cycle` | 生命周期命令；均要求 reason |
| `GET /traffic/summary`、`GET /traffic/series` | 实例/服务器/用户/节点按时间聚合 |
| `GET /traffic/records` | 原始来源记录，只允许诊断权限和受限时间窗 |
| `GET /users/{id}/traffic`、`GET /servers/{id}/traffic` | 对象投影与 billing/raw 分离 |
| `GET /traffic/ledger` | append-only 账本查询 |
| `POST /traffic/adjustments` | 增减、清零、baseline；必须 reason/idempotency |
| `POST /traffic/ledger/{id}/reverse` | 追加冲正，不修改原记录 |
| `GET /connections/live`、`GET /connections/history` | 当前/历史连接，按权限脱敏源 IP |
| `POST /connections/{id}/close` | 关闭一条连接并记录执行者/原因 |

`traffic/series` 的 grain 只允许 `minute/hour/day/month`。账单流量不能由可变 UI 配置回写原始 measurement；倍率和 adjustment 都是独立 ledger entry。

## 6. 服务器、Agent 和 sing-box 内核

| Method/path | 作用 | 结果 |
|---|---|---|
| `GET/POST /servers`、`GET/PATCH/DELETE /servers/{id}` | 服务器聚合根 | 删除需要无活跃共享/任务或显式 cascade plan |
| `POST /servers/{id}/enrollment-tokens` | 生成一次性 Agent 安装令牌 | 明文仅返回一次，默认 15 分钟 |
| `GET /servers/{id}/status` | online/health/capability/drift/版本/时钟 | reported，不用 last-login 推测 |
| `GET /servers/{id}/metrics`、`GET /servers/{id}/logs` | 系统、内核和 Agent 观测 | 日志 cursor，服务端强制 redact |
| `GET /servers/{id}/capabilities` | 内核/OS/build tags/API/eBPF 能力 | UI 所有动作据此禁用/解释 |
| `POST /servers/{id}/probe` | 延迟/端口/公网 IP/出口探测 | 异步 job |
| `POST /servers/{id}/commands/{start|stop|restart|reload}` | service 生命周期 | restart/reload 都生成 task；reload 报 disruption |
| `GET /servers/{id}/core`、`PATCH /servers/{id}/core` | pinned channel/version/build tags | 不接受任意下载 URL |
| `POST /servers/{id}/core/install`、`upgrade`、`rollback` | 签名制品安装 | checksum/signature/last-good/health gate |
| `GET /servers/{id}/configs` | revision 历史与部署结果 | secret redacted |
| `POST /servers/{id}/configs/compile` | 只编译、lint、兼容性诊断 | 不改变 desired/reported |
| `POST /servers/{id}/configs/deploy` | CAS 生成 desired revision | 异步、preflight、last-good rollback |
| `POST /servers/{id}/configs/{rev}/rollback` | 回滚到已知 revision | 新建 revision，不篡改历史 |
| `GET /servers/{id}/drift` | desired/reported 文件/hash/capability 差异 | 明确 unknown/degraded |
| `POST /servers/{id}/agent/rotate` | 轮换 mTLS 身份 | grace 双证书窗口 |

不提供“任意 shell”公开端点。所有 Agent task 都来自 allowlist command schema；安装、Nginx、证书和内核操作各有独立 DTO 与权限。

## 7. 入站、节点、出站、路由与隧道

| Method/path | 作用 |
|---|---|
| `GET/POST /servers/{sid}/inbounds`、`GET/PATCH/DELETE /inbounds/{id}` | 入站 CRUD；protocol-specific `settings` 由 discriminated schema 校验 |
| `POST /inbounds/{id}/validate` | 端口、TLS、transport、build/capability、用户模式校验 |
| `GET/POST /inbounds/{id}/principals`、`PATCH/DELETE /inbound-principals/{id}` | 绑定 VLESS/VMess/Trojan/Hy2/AnyTLS/Snell/SSM 身份 |
| `POST /inbounds/{id}/rotate-credentials` | 凭据轮换与 overlap 窗口 |
| `GET/POST /nodes`、`GET/PATCH/DELETE /nodes/{id}` | 可发布节点和显示元数据 |
| `POST /nodes/bulk`、`POST /nodes/reorder` | 批量启停/移动/标签与顺序 |
| `POST /nodes/{id}/test`、`GET /nodes/{id}/test-runs` | 节点测速、连通/延迟/IP 测试 |
| `GET/POST /servers/{sid}/outbounds`、`GET/PATCH/DELETE /outbounds/{id}` | direct/proxy/selector/urltest/WARP 等出站 |
| `POST /outbounds/{id}/select` | selector 切换或 Agent scheduler 决策 |
| `GET/POST /servers/{sid}/route-rules`、`GET/PATCH/DELETE /route-rules/{id}` | first-match 路由规则 |
| `POST /route-rules/reorder`、`POST /route-rules/validate` | 原子排序、shadow/unreachable 检查 |
| `GET/POST /route-rule-sets`、`POST /route-rule-sets/{id}/sync` | remote/local rule-set，安全 fetch |
| `GET/POST /tunnels`、`GET/PATCH/DELETE /tunnels/{id}` | 入站到出站/远端服务器隧道 |
| `POST /tunnels/{id}/test` | 两端 capability、路径和回环检测 |
| `GET/POST /warp-profiles`、`POST /warp-profiles/{id}/refresh` | WARP 身份、密钥引用和 endpoint 刷新 |
| `GET/POST /user-routes`、`PATCH/DELETE /user-routes/{id}` | 私有用户路由节点、流量配额和到期 |

任何会改变 sing-box 的写操作只修改 Master desired state；只有显式 deploy 或启用的 debounce auto-deploy 才创建 Agent task。compile diagnostic 要精确指出 JSON pointer、目标服务器、sing-box 版本和替代方案；XHTTP 不得伪装成 sing-box HTTP transport。

## 8. 订阅、外部源、provider、模板和规则

| Method/path | 作用 |
|---|---|
| `GET/POST /sources`、`GET/PATCH/DELETE /sources/{id}` | 外部订阅源 CRUD；URL credential 存 secret |
| `POST /sources/{id}/sync`、`GET /sources/{id}/runs` | 条件请求、解析、diff、原子激活 |
| `GET /sources/{id}/items`、`POST /sources/{id}/preview` | 规范化结果与未提交预览 |
| `GET/POST /profiles`、`GET/PATCH/DELETE /profiles/{id}` | 订阅文件聚合根 |
| `POST /profiles/{id}/publish`、`GET /profiles/{id}/versions` | 固定输入生成 immutable artifact |
| `GET/POST /profiles/{id}/tokens`、`POST /profile-tokens/{id}/rotate` | 下载 token 生命周期 |
| `GET/POST /providers`、`GET/PATCH/DELETE /providers/{id}` | proxy-provider 配置/过滤/链接 |
| `POST /providers/{id}/preview` | proxy-provider 输出预览 |
| `GET/POST /templates`、`GET/PATCH/DELETE /templates/{id}` | 内置/用户模板，版本化 |
| `POST /templates/{id}/render`、`POST /templates/{id}/lint` | 受限上下文渲染、schema lint |
| `GET/POST /rule-libraries`、`POST /rule-libraries/{id}/sync` | 规则目录与远程更新 |
| `GET/POST /subscription-scripts`、`POST /subscription-scripts/{id}/test` | sandbox 转换脚本、resource limits |
| `POST /generate/preview` | 不保存的 IR→目标客户端预览 |
| `GET /client-capabilities` | 各客户端/格式/版本可表达能力 |

公开下载端点不在 `/api/v1`：`GET /sub/{token}`、`GET /provider/{token}`。它们支持 ETag、`If-None-Match`、gzip、客户端 UA capability 协商和可配置文件名；不得在响应 header 泄露内部 source URL。输出流水线见 [SUBSCRIPTION_IR.md](./SUBSCRIPTION_IR.md)。

## 9. 证书、Nginx 站点、测速与探针

| Method/path | 作用 |
|---|---|
| `GET/POST /certificates`、`GET/PATCH/DELETE /certificates/{id}` | ACME/import/self-managed 证书 |
| `POST /certificates/{id}/issue`、`renew`、`deploy` | DNS-01/HTTP-01 任务和受控部署 |
| `GET /certificates/{id}/events` | challenge、续期、部署、过期历史（脱敏） |
| `GET/POST /servers/{sid}/sites`、`GET/PATCH/DELETE /sites/{id}` | Nginx 站点、反代、WS/HTTPUpgrade |
| `POST /sites/{id}/validate`、`deploy`、`rollback` | `nginx -t` 后原子切换/回滚 |
| `GET/POST /speed-targets`、`PATCH/DELETE /speed-targets/{id}` | 测速目标和地域/运营商元数据 |
| `POST /speed-tests`、`GET /speed-tests/{id}` | server/node/public tester 测速 job |
| `GET /speed-tests/{id}/samples` | 分阶段样本，不只最终平均值 |
| `GET/PATCH /probe/settings` | 公开字段、匿名化、缓存和 rate limit |
| `GET /probe/servers`、`GET /probe/servers/{public_id}` | 经 allowlist 的公开 projection |
| `POST /probe/tests`、`GET /probe/tests/{id}` | 公开配额内的远程测试 |

公开 probe 永远使用随机 `public_id`，不返回服务器 SSH、Agent、内网 IP、用户、连接或原始日志。管理员预览端点可显示“公开将看到什么”，但走同一 projection 函数。

## 10. Telegram、通知、MCP 与实例联合

| Method/path | 作用 |
|---|---|
| `GET/PATCH /notifications/settings` | 渠道、阈值、quiet hours、dedupe |
| `GET/POST /notification-rules`、`PATCH/DELETE /notification-rules/{id}` | 事件路由与模板 |
| `GET /notification-deliveries`、`POST /notification-deliveries/{id}/retry` | 投递审计/重试 |
| `POST /integrations/telegram/connect`、`disconnect` | Bot/token secret 和 webhook 生命周期 |
| `POST /integrations/telegram/webhook/{opaque}` | Telegram 入站；验证 secret header |
| `POST /integrations/telegram/miniapp/session` | 验签 initData 并换最小权限短 session |
| `GET/POST /mcp/clients`、`PATCH/DELETE /mcp/clients/{id}` | MCP client、tool allowlist、scope |
| `POST /mcp/clients/{id}/rotate` | token rotation；明文一次性 |
| `GET /mcp/tools`、`POST /mcp/invocations/{id}/confirm` | 26+ tool catalog 与危险调用确认 |
| `GET/POST /peers`、`GET/PATCH/DELETE /peers/{id}` | 实例联合配对和 trust policy |
| `POST /peers/{id}/handshake`、`rotate`、`revoke` | 双向凭据生命周期 |
| `GET/POST /shares`、`PATCH/DELETE /shares/{id}` | 分享服务器/节点/能力投影 |
| `GET /peers/{id}/shares`、`POST /peer-shares/{id}/import` | 发现并导入 immutable reference |
| `POST /federation/v1/envelopes` | 对端签名消息入口 | replay/expiry/schema/signature 后才入库 |

“联合”只分享显式 allowlist projection；不建立全局账号、不把对端当管理员、不依赖官方目录或授权服务器。任一实例可离线独立工作。

## 11. Jobs、审计、备份与运维

| Method/path | 作用 |
|---|---|
| `GET /jobs`、`GET /jobs/{id}`、`POST /jobs/{id}/cancel`、`retry` | durable job 查询和控制 |
| `GET /jobs/{id}/events` | 状态/步骤/安全摘要；cursor pagination |
| `GET /events/stream` | 管理端 SSE；带类型/对象过滤和 resume ID |
| `GET /audit`、`GET /audit/{id}`、`POST /audit/export` | append-only 审计与签名导出 |
| `POST /backups`、`GET /backups`、`GET /backups/{id}` | 加密备份 job 和 manifest |
| `POST /restores/inspect` | 离线校验版本、hash、迁移路径，不改状态 |
| `POST /restores` | 新维护窗口恢复；需要 owner 再认证和确认短语 |
| `GET /system/health` | auth 后完整依赖健康 |
| `GET /system/version` | UI/API/DB schema/Agent/core compatibility |
| `GET /system/licenses` | 本项目和所带第三方许可证/source offer |
| `GET /system/diagnostics`、`POST /system/diagnostics/export` | 脱敏环境和 support bundle |

公开的 `/healthz` 只返回进程 liveness，`/readyz` 仅部署网络可见。Prometheus `/metrics` 默认独立监听地址并要求网络/凭据保护。

## 12. 实时事件合同

管理 UI 使用 SSE，不以轮询推导任务完成。事件 envelope：

```json
{
  "id":"evt_019...",
  "occurred_at":"2026-08-25T13:00:00Z",
  "type":"server.reported_state.changed",
  "resource":{"type":"server","id":"019...","revision":42},
  "summary":{"status":"degraded"}
}
```

SSE 支持 `Last-Event-ID`，服务端保留至少 15 分钟 resume window；超过窗口返回 control event `resync_required`。事件只包含 UI 刷新所需的安全摘要，完整对象仍通过 REST 获取。订阅消费者必须按 event ID 去重。

已保留的事件族：`job.*`、`server.*`、`agent.*`、`core.*`、`traffic.*`、`connection.*`、`certificate.*`、`site.*`、`source.*`、`profile.*`、`notification.*`、`security.*` 和 `federation.*`。

## 13. OpenAPI 与 DTO 生成规则

- Rust handler DTO 使用 `serde` + `utoipa`（或最终选定的单一生成器）作为 OpenAPI 3.1 事实源；domain entity 绝不直接序列化为 API。
- CI/VPS 检查 OpenAPI deterministic、无未描述 2xx/4xx、示例通过 schema、operation ID 唯一。
- TypeScript client 从锁定的 OpenAPI 生成，生成物单独目录、禁止手改；前端 query keys 以 operation ID + normalized args 构造。
- Protocol-specific DTO 必须使用 `type` discriminator 和 `oneOf`，禁止无约束 `settings: object` 成为永久逃生口。
- secret 字段拆为 `secret_ref`、`is_configured`、`last_rotated_at`；任何 GET 都不回显 secret。创建时一次性返回的 secret 置于独立 `one_time_secret`，并设置 `Cache-Control: no-store`。

## 14. 合同测试门

每个端点至少验证：成功、未认证、scope 不足、对象越权、schema 错误、revision/幂等冲突、资源不存在、审计记录和 secret redaction。异步命令还要验证 job 重试/取消/Agent offline/重复回执；公开端点验证速率限制、缓存、token revoke/expiry 和信息最小化。

发布门包括：OpenAPI backward compatibility diff、Rust/TypeScript round-trip、SQLite/PG repository 双实现、浏览器 CSRF/CORS、MCP/Telegram/federation signature golden vectors、订阅客户端 fixture、SSE resume，以及所有错误 code 稳定性快照。
