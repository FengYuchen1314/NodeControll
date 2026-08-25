# 妙妙屋 Go 后端解剖

> 基线：`iluobei/miaomiaowu@0b47f10c52aee10b9f759a593ca5f61a823cbb72`。逐函数、逐类型和逐变量的行号级索引见 [`generated/go/README.md`](generated/go/README.md)，本页负责解释模块边界、运行顺序和业务协作关系。

## 1. 总体形态

后端是一个 Go 单体服务：`cmd/server/main.go` 直接创建 `http.ServeMux`、Repository、认证状态和各业务 Handler，并启动多个 goroutine。没有独立 service/domain 层，多数业务编排位于 `internal/handler`，SQLite SQL 位于 `internal/storage`，前端构建结果通过 `go:embed` 打进同一二进制。

运行时主要数据面如下：

```text
HTTP / WebSocket / SSE
        │
        ▼
CORS → SilentMode → OperationAudit → ServeMux → RequireToken/RequireAdmin → Handler
                                                    │
               ┌────────────────────────────────────┼─────────────────────────┐
               ▼                                    ▼                         ▼
        TrafficRepository                    文件系统 YAML             外部 HTTP/WS
        SQLite/WAL 单连接                subscribes/rules/logs     探针/订阅/GitHub/TG
```

关键特征：

- 传输层使用标准库 `net/http`，路由由精确/前缀字符串匹配和 Handler 内部的 `r.Method`、`r.URL.Path` 分派完成。
- `TrafficRepository` 实际承担 Repository、迁移器和大量领域操作三种职责。
- `handler` 包有 83 个文件，是最大的耦合点；它同时处理 HTTP、节点解析、YAML AST、订阅生成、缓存、调度和外部系统适配。
- 单进程内存状态包括 UI 会话、2FA 临时令牌、登录限速、订阅限速、短链暴力防护、静默模式活跃窗口、代理集合缓存、远程测速器 WebSocket 和通知器。
- 可持久化状态主要在 SQLite；规则、订阅和日志正文仍分散在文件系统。

## 2. 启动和停止时序

`main()` 的依赖与副作用顺序如下；顺序不是偶然的，后续步骤依赖前一步产生的表、文件或内存状态。

1. 初始化结构化日志，并启动每天一次、保留 7 天的文件日志清理 goroutine。
2. 从 `PORT` 读取监听端口，默认 `8080`。
3. 打开 `data/traffic.db`，设置 WAL/超时/同步级别，执行全部 schema 迁移。
4. 创建认证 `Manager`、24 小时 `TokenStore` 和 5 分钟 `TwoFactorPendingStore`。
5. 从 `sessions` 表回填未过期 UI 会话到内存，并清理数据库过期会话。
6. 将二进制内嵌的默认订阅和规则模板准备到 `subscribes/`、`rule_templates/`；再对历史错误 DNS 片段做精确幂等补丁。
7. 读取 `system_config`，从用户地址或内置回退地址拉取代理组目录；失败时以空数组启动，而不是阻断服务。
8. 扫描磁盘订阅 YAML，把缺失的 `subscribe_files` 元数据补入数据库。
9. 用系统配置初始化 Telegram 通知器、未知订阅 UA 拦截开关和代理集合缓存。
10. 启动代理集合首次预热和定时同步器。
11. 创建流量、用户、登录限速、Turnstile 等共享依赖，并注册全部 HTTP 端点。
12. 根路径 Handler 依次尝试临时订阅、短链和 SPA；疑似短链枚举会进入暴力探测计数。
13. 包装全局中间件：最内层审计、其外静默模式、最外 CORS。
14. 启动 `http.Server`，仅设置 5 秒 `ReadHeaderTimeout`。
15. 启动 WAL checkpoint、数据库日志清理、封禁清理、每日流量、通知、外部订阅自动更新等 goroutine。
16. 收到 SIGINT/SIGTERM 后取消各任务 Context，并给 HTTP Server 10 秒优雅关闭窗口。

启动有一个明确构建前置条件：`internal/web/handler.go` 使用 `//go:embed dist/*`，而仓库不提交 `dist`。必须先执行前端构建，把 Vite 输出写到 `internal/web/dist`，Go 包才能编译。

## 3. 请求管线与身份边界

### 3.1 CORS

`cmd/server/cors.go` 从 `ALLOWED_ORIGINS` 读取逗号分隔来源；默认 `*`。中间件根据请求 `Origin` 回显允许来源，补充 methods/headers，并直接结束 OPTIONS 预检。允许来源和是否全开放在启动时计算一次。

### 3.2 静默模式

`SilentModeManager` 位于 CORS 内、审计外。开启后：

- 服务启动后的 `silent_mode_timeout` 分钟内全局放行；
- 订阅端点、proxy-provider、临时订阅和数据库中已知短链始终允许；
- 成功获取订阅后刷新用户/IP/全局活跃窗口，在超时前允许管理 UI；
- 其他请求返回伪装响应，意图隐藏服务指纹；
- 相关短链集合有内存缓存，订阅/短码变更后需要显式失效。

### 3.3 操作审计

`OperationAuditMiddleware` 只记录 `/api/admin/` 下的 POST/PUT/PATCH/DELETE。它用包装的 `ResponseWriter` 捕获最终状态码，并在响应后写 `operation_logs`。GET、普通用户写操作、订阅获取和根路径不进入该表。

### 3.4 UI 会话认证

- 客户端把 UI 会话放在自定义头 `MM-Authorization`；SSE 可退回 `?token=`。
- `TokenStore` 以加密随机数签发 URL-safe token，保存用户名和过期时间；内存读写有锁。
- 会话同时持久化到 `sessions`，服务重启后恢复。
- `RequireToken` 只检查 TokenStore；`RequireAdmin` 再从数据库读取用户并要求 `role == admin`。
- 用户密码使用 bcrypt；账户状态和角色在用户 Repository 中维护。
- TOTP setup/verify/disable 与一次性恢复码由 `auth/totp.go` 和 `handler/two_factor.go` 协作；登录中间态 token 默认 5 分钟。

### 3.5 订阅长期凭据

UI 会话和订阅授权不是同一套 token。`user_tokens` 为每个用户保存长期订阅 token、系统生成用户短码和可选自定义短码；`subscribe_files` 另有文件短码。对外短链通常由“文件短码 + 用户短码”组合解析，并再次检查用户对订阅文件的授权。

## 4. 包级职责

| 包 | 文件 | 作用与边界 |
|---|---:|---|
| `cmd/server` | 2 | 依赖组装、路由注册、全局中间件、后台任务生命周期和优雅关闭。 |
| `internal/auth` | 4 | bcrypt 密码、会话 token、角色中间件、TOTP/恢复码和 storage 适配。 |
| `internal/captcha` | 1 | 从数据库读取 Turnstile 配置，并调用 Cloudflare siteverify。 |
| `internal/handler` | 83 | HTTP/WS/SSE 适配和大部分业务逻辑；详细文件表见后文。 |
| `internal/logger` | 2 | `slog` 文本日志、敏感参数清洗、临时 debug 文件和轮转/清理。 |
| `internal/notify` | 3 | 通知事件模型、事件开关和 Telegram Bot API 发送。 |
| `internal/patches` | 1 | 用 YAML 语义等价比较匹配已知错误 DNS 块，只替换明确历史版本。 |
| `internal/proxygroups` | 4 | 代理组目录 URL 解析、HTTP 拉取、默认值规范化和并发安全内存快照。 |
| `internal/scriptengine` | 2 | goja JavaScript VM；支持 post-fetch、pre-save-nodes、console 和 produce，默认 5 秒超时。 |
| `internal/speedtest` | 2 | 下载/缓存 Mihomo、检查版本与 Snell 支持，启动临时内核并测延迟/下载/出口 IP。 |
| `internal/storage` | 9 | SQLite/WAL、迁移、26 张表和所有数据访问；详见 [`DATABASE.md`](DATABASE.md)。 |
| `internal/taskrun` | 1 | 包装后台任务，记录耗时/结果，并按任务类型节流成功记录。 |
| `internal/util` | 1 | YAML Node 转换和代理字段稳定排序。 |
| `internal/validator` | 1 | Clash 配置、代理、代理组、引用和循环依赖校验，并重排输出字段。 |
| `internal/version` | 1 | 编译版本常量 `0.8.3`。 |
| `internal/web` | 1 | 嵌入 Vite dist，静态文件命中时直接返回，其他非 API 路径 fallback 到 SPA index。 |
| `rule_templates` | 1 | 嵌入默认规则模板并以“不覆盖用户文件”方式落盘。 |
| `subscribes` | 1 | 嵌入默认订阅文件并准备订阅目录。 |

每个包内的完整函数、方法、匿名闭包、类型和全局值见 [`generated/go/packages`](generated/go/packages)。

## 5. `internal/handler` 文件职责

### 5.1 身份、用户和初始化

| 文件 | 作用 |
|---|---|
| `setup.go` | 查询首次初始化状态，创建首个管理员账户；初始化前端由此判断展示 setup。 |
| `auth.go` | 获取可信客户端 IP，处理用户名/密码/Turnstile 登录，并提供管理员修改主凭据入口。 |
| `password.go` | 已登录用户修改自己的密码。 |
| `profile.go` | 读取和更新昵称、邮箱、头像等资料。 |
| `users.go` | 管理员列表、创建、删除、启停、重置密码、备注和自定义短码。 |
| `user_subscriptions.go` | 管理员读取/覆盖某用户可访问的订阅文件集合。 |
| `user_token.go` | 读取或重置用户长期订阅 token。 |
| `user_settings.go` | 读写每用户同步、模板、缓存、探针绑定、调试等偏好。 |
| `user_config.go` | 聚合用户与系统层配置；校验代理组源 URL，并把运行时配置推送给相关模块。 |
| `user_default_template.go` | 读写用户默认 Clash/Surge 模板。 |
| `two_factor.go` | 2FA 登录、恢复码登录、setup、确认启用、状态和禁用。 |
| `turnstile_settings.go` | 管理员读取/更新 Turnstile 开关、site key 和 secret。 |

### 5.2 安全、限速、审计和调试

| 文件 | 作用 |
|---|---|
| `rate_limiter.go` | 登录失败滑动窗口、锁定时间和本地 IP 例外；进程内状态。 |
| `brute_force.go` | 短链枚举失败计数、临时/永久 IP 封禁、数据库恢复、清理和管理操作。 |
| `silent_mode.go` | 404 伪装、订阅激活窗口、已知短链缓存和允许路径判断。 |
| `subscription_rate.go` | 按 IP 限制订阅请求频率并周期清理内存桶。 |
| `subscription_ua_guard.go` | 可选拒绝未知订阅客户端 User-Agent。 |
| `client_ua.go` | 把 UA 归类为 Clash、Surge、Loon、sing-box 等客户端，用于自动输出格式。 |
| `ip_helpers.go` | 判断 loopback、私网、链路本地等地址，供安全例外使用。 |
| `ssrf_safe_fetch.go` | 限制 http/https、解析 DNS 并拒绝私网/保留 IP，自定义 Dialer 防止 DNS rebinding。 |
| `tls_fingerprint.go` | 获取对端证书并计算 SHA-256 指纹。 |
| `operation_audit.go` | 捕获管理员变更请求结果并落审计表。 |
| `operation_logs.go` | 查询操作日志。 |
| `security_logs.go` | 查询安全事件和封禁；支持管理员封禁、永久化、解封。 |
| `task_logs.go` | 查询后台任务运行记录和任务类型列表。 |
| `debug.go` | 开启/关闭临时 debug 日志、自动关闭计时、状态、tail 和下载。 |

### 5.3 节点、外部订阅和 proxy-provider

| 文件 | 作用 |
|---|---|
| `nodes.go` | 节点 CRUD、批量创建/删除/重命名、server 改写/恢复、证书跳过开关、URI/订阅解析、探针绑定和链式代理字段。 |
| `proxy_parser.go` | 调用独立 `mmwX-plugins/proxyparser` 解析代理 URI 与 V2Ray base64 订阅。 |
| `v2ray_parser.go` | 宽容 base64 解码辅助。 |
| `external_subscriptions.go` | 外部订阅 CRUD、自动更新间隔规范化、节点预览和过滤命中检查。 |
| `external_sync.go` | 手动/单条/确认式同步，节点选择暂存，流量头解析，名称过滤，更新 YAML，定时自动同步。 |
| `proxy_provider.go` | proxy-provider 配置 CRUD、缓存刷新/状态/节点 API，以及 client/MMW 两种处理模式互转。 |
| `proxy_provider_cache.go` | 并发安全内存缓存、失败退避、定时调度、worker 上限、首次预热和节点预览。 |
| `proxy_provider_serve.go` | 对外 provider 端点；下载、过滤、GeoIP、覆写、顺序稳定化、缓存生成和刷新。 |
| `yaml_sync_manager.go` | 单节点/批量新增、更新、删除时协调多个订阅 YAML。 |
| `yaml_sync.go` | 在 YAML AST 中更新代理字段、组成员、规则引用、顶层顺序和短 ID 风格。 |
| `yaml_utils.go` | YAML marshal、缩进、Unicode 转义、显式 string tag 和 scalar 转换。 |

### 5.4 订阅文件、生成、规则和模板

| 文件 | 作用 |
|---|---|
| `subscription.go` | 对外订阅核心：鉴权、解析短码、同步外部来源、运行脚本、选择节点/标签、应用规则/模板、注入链式代理、排序去重，并转换 Clash/Surge/Loon/JSON。 |
| `subscription_admin.go` | 较旧 `subscription_links` 管理、规则文件落盘/清理和用户可见订阅列表。 |
| `subscribe_files.go` | 当前订阅文件聚合根：创建、导入、上传、内容编辑、元数据、顺序、聚合、模板重生成、批量刷新和未引用代理修剪。 |
| `subscribe_files_list.go` | 已登录用户仅查看自己有权访问的订阅文件。 |
| `short_link.go` | 解析组合短码、转交订阅 Handler，并提供文件/用户短码重置。 |
| `temp_subscription.go` | 进程内临时订阅，随机 8 位 code、过期清理和 `/t/{code}` 访问。 |
| `rules.go` | 规则文件列表、读取、更新和历史版本；严格清理文件名。 |
| `rules_metadata.go` | 获取规则文件的最新版本元数据。 |
| `rule_templates.go` | V3/文件模板列表、读取、上传、更新、重命名、删除、所有权和公开性。 |
| `templates.go` | 数据库 V2 模板 CRUD、远程内容获取、V1/V2 兼容转换和代理组成员补全。 |
| `template_v3.go` | V3 模板处理/预览、标签筛选、代理与 relay 注入、V2 转换、订阅分析、地域筛选和 Surge 模板分支。 |
| `custom_rules.go` | 自定义规则 CRUD。 |
| `apply_custom_rules.go` | 把 DNS/rules/rule-providers 片段按追加/替换策略合并进 YAML，去重并自动补缺失代理组。 |
| `override_scripts.go` | JavaScript 覆写脚本 CRUD 和顺序响应。 |
| `proxy_groups.go` | 读取代理组目录内存快照，或由管理员触发远程重新同步。 |
| `clash_snell_filter.go` | 对不支持 Snell v6 的 Clash 客户端剔除相关代理和组引用。 |

### 5.5 探针、流量、测速与通知

| 文件 | 作用 |
|---|---|
| `probe_admin.go` | Nezha v1/v0、DStatus、Komari 探针配置 CRUD、服务器列表和流量口径校验。 |
| `probe_sync.go` | 分别通过 HTTP/WebSocket 拉取各探针服务器清单并归一化。 |
| `traffic_summary.go` | 聚合探针与外部订阅的上限/使用/剩余，保存每日快照，输出摘要/SSE，并构造逐服务器流量。 |
| `speedtest.go` | 创建本地/远程节点测速任务、tester 注册/吊销/轮换、结果查询。 |
| `speedtester_ws.go` | 认证远程 tester WebSocket，维护在线连接并派发任务/接收结果。 |
| `tcping.go` | 单个和批量 TCP connect 延迟探测。 |
| `notify_global.go` | 保存进程级 Notifier 单例。 |
| `notify_config.go` | 通知配置读写和测试消息。 |
| `notify_scheduler.go` | 定时发送每日流量与订阅过期通知。 |

### 5.6 运维

| 文件 | 作用 |
|---|---|
| `backup.go` | checkpoint 后打包数据库、订阅、规则等；恢复时校验/解压并覆盖对应数据。 |
| `update.go` | 查询 GitHub release、版本比较、下载重试/进度、备份和替换当前二进制；提供普通 JSON 和 SSE 两套接口。 |
| `notify_config.go` | 除业务通知外，也承担运行时 Notifier 配置热更新。 |

## 6. 后台任务与共享状态

| 任务 | 触发 | 状态位置 | 作用 |
|---|---|---|---|
| 文件日志清理 | 启动后每 24 小时 | 文件系统 | 删除 7 天前 debug/log 文件。 |
| WAL checkpoint | 每 5 分钟 | SQLite | 优先 TRUNCATE，繁忙则 PASSIVE；任务记录做 5 分钟节流。 |
| 数据库日志清理 | 启动立即 + 每 24 小时 | SQLite | 安全/操作日志保留 90 天，任务记录 30 天。 |
| 暴力封禁清理 | Context 生命周期 | 内存 + `ip_bans` | 移除到期封禁并记录安全事件。 |
| 流量采集 | 启动立即 + 每 24 小时 | 探针/外订阅 + `traffic_records` | 汇总并持久化每日快照。 |
| 代理集合同步 | 扫描 15 秒、配置重载 5 分钟 | 内存 cache + 外部订阅 | 按到期和失败状态并发刷新。 |
| 外部订阅更新 | 每 1 分钟检查 | SQLite + YAML | 对启用 auto-update 且到期的来源执行同步。 |
| 通知调度 | Context 生命周期 | SQLite + Telegram | 每日流量、到期等事件。 |
| Debug 自动关闭 | 用户启用时 | 内存 timer + `user_settings` | 到期后关闭文件日志。 |

这些任务在一个进程内用多个 goroutine 运行，没有分布式锁。单实例部署符合上游假设；若直接横向扩容，会重复同步、重复通知并争用同一文件系统。

## 7. 测试文件与覆盖意图

`internal/handler` 的测试集中覆盖复杂配置变换和安全边界：

- `brute_force_persistence_test.go`：人工封禁落库、重启恢复和解封。
- `chain_proxy_test.go`、`inject_relay_groups_test.go`、`relay_group*_test.go`：链式代理、relay 组注入、更新和删除修剪。
- `clash_snell_filter_test.go`：客户端不兼容协议过滤。
- `external_sync_selection_test.go`：同步节点选择确认。
- `node_enabled_default_test.go`：节点缺省启用；当前 2 个用例因缺少必填 `protocol` 而失败。
- `nodes_skipcert_test.go`：批量关闭证书跳过。
- `override_script_test.go`：goja hook、异常、超时、produce 和对象转换。
- `proxy_parser_test.go`：URI/V2Ray 解析桥接。
- `ssrf_safe_fetch_test.go`：私网/保留地址阻断。
- `subscribe_filename_test.go`：订阅文件名清理。
- `surge_template_test.go`：Surge 模板注入。
- `template_v3_rule_providers_test.go`、`templates_v2_groups_test.go`：模板规则和代理组兼容。
- `tls_fingerprint_test.go`：证书指纹。

此外 `storage` 覆盖批量节点重名和删除 relay 成员，`proxygroups` 覆盖默认值/URL 推断/Store 更新，`scriptengine` 有较完整的运行时单元测试。

## 8. 重要实现约束与重构提示

- `handler` 与 `storage` 体量过大，HTTP、业务、配置转换和外部 I/O 缺少接口边界；Rust 重构不能按文件逐字翻译，应先拆领域服务与端口适配器。
- SQLite 声明了外键却未显式开启 `PRAGMA foreign_keys=ON`；现有数据可能含孤儿行，迁移前必须做一致性审计。
- 多个关系以 JSON/TEXT ID 数组存储；新 schema 应正规化关联表，同时保留迁移兼容层。
- UI 会话、订阅 token、文件短码、用户短码和临时订阅 code 是五类不同凭据，重构时必须分别建模、设定轮换/过期/审计策略。
- 中间件顺序决定静默模式是否记录审计、CORS 是否暴露响应头；新实现必须用端到端测试锁定。
- 多数后台任务假定单主控；未来支持高可用时需要任务租约或数据库 advisory-lock 等机制。
- 上游只有 `ReadHeaderTimeout`，没有 `ReadTimeout`、`WriteTimeout`、`IdleTimeout` 和统一请求体限制；新系统需按流式订阅/SSE/WS 特性分别配置。
- 自更新会替换运行中的二进制；容器化自托管产品更适合“检查版本 + 生成升级命令/镜像标签 + 可回滚迁移”，不能照搬覆盖文件。

