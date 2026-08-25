# 妙妙屋 HTTP API 说明

> 基线：`iluobei/miaomiaowu@0b47f10c52aee10b9f759a593ca5f61a823cbb72`。本文按 Handler 的实际方法分派整理；87 条 `ServeMux` 顶层注册及源码行号见 [`generated/http-routes.md`](generated/http-routes.md)，前端 225 个静态调用点见 [`generated/typescript/api-calls.md`](generated/typescript/api-calls.md)。

## 1. 协议约定

- 管理 UI 会话通过 `MM-Authorization: <session-token>` 传递；会话默认 24 小时，SSE 端点可从 `?token=` 读取。
- 管理员端点由 `RequireAdmin` 包装；普通用户端点由 `RequireToken` 包装。个别历史命名为 `/api/admin/*` 的模板读取端点只要求登录，再由 Handler 做 owner/public/admin 判断。
- 对外订阅、proxy-provider、短链、临时订阅和测速 tester 各自使用独立凭据，不接受 UI 会话替代。
- JSON 错误通常为 `{"error":"..."}`，但部分早期 Handler 使用 `http.Error` 返回纯文本；成功响应也没有统一 envelope。
- 方法不匹配返回 405，并在部分 Handler 设置 `Allow`。
- CORS 默认允许任意来源，可用 `ALLOWED_ORIGINS` 收紧。全局仅设置 5 秒 `ReadHeaderTimeout`。
- 管理员 POST/PUT/PATCH/DELETE 会进入 `operation_logs`；普通用户写操作、GET 和订阅访问不会进入该审计表。

## 2. 公开初始化、验证码与登录

| 方法 | 路径 | 请求/行为 | 响应 |
|---|---|---|---|
| GET | `/api/setup/status` | 查询是否尚无用户 | `{needs_setup}` |
| POST | `/api/setup/init` | 首次创建管理员；body 含 `username/password/nickname/email/avatar_url`，已有用户时拒绝 | 新管理员资料 |
| POST | `/api/setup/restore-backup` | 初始化阶段上传备份并恢复 | 恢复状态；实现会校验归档内容 |
| GET | `/api/captcha/config` | 读取登录页所需 Turnstile 公开配置 | enabled/site key，不泄露 secret |
| POST | `/api/login` | `username/password/remember_me/turnstile_token`；登录限速和 Turnstile 后校验 bcrypt | 普通登录返回会话和用户资料；启用 TOTP 时返回 `requires_2fa` 和五分钟中间 token |
| POST | `/api/login/2fa` | `two_factor_token/code` | 消费中间 token 并签发 UI 会话 |
| POST | `/api/login/recovery` | `two_factor_token/recovery_code` | 消费恢复码、关闭当前 TOTP 并签发 UI 会话 |

初始化不是长期注册入口：`/api/setup/init` 会再次检查用户总数，确保只有空库可执行。

## 3. 当前用户与会话内设置

| 方法 | 路径 | 作用 |
|---|---|---|
| GET | `/api/user/profile` | 返回用户名、昵称、邮箱、头像、角色和 `is_admin`。 |
| POST | `/api/user/password` | 校验旧密码后修改当前用户密码。 |
| PUT | `/api/user/settings` | 更新当前用户界面/同步/调试等偏好。 |
| GET / PUT | `/api/user/config` | 读取或更新聚合配置；包含系统配置和用户偏好，并触发运行时配置刷新。 |
| GET / POST | `/api/user/token` | 读取或重新生成当前用户长期订阅 token。 |
| POST | `/api/user/short-link` | 重置用户系统短码，并使短链缓存失效。 |
| GET / POST | `/api/user/custom-short-code` | 查询或设置自己的自定义短码。 |
| GET | `/api/user/default-template` | 获取用户默认 V3 模板。 |
| PUT | `/api/user/default-template` | 设置模板类型/文件名；Handler 验证可见性。 |
| GET | `/api/user/2fa/status` | 返回 `{enabled}`。 |
| POST | `/api/user/2fa/setup` | 用当前密码确认后生成 TOTP secret 与 otpauth URL。 |
| POST | `/api/user/2fa/verify-setup` | 校验首次 TOTP code，启用 2FA，并仅此次返回 8 个明文恢复码。 |
| POST | `/api/user/2fa/disable` | 用当前 TOTP code 关闭 2FA。 |
| POST | `/api/user/debug/enable` | 开启当前用户临时 debug 日志。 |
| POST | `/api/user/debug/disable` | 关闭 debug。 |
| GET | `/api/user/debug/status` | 返回启用状态、文件与自动关闭信息。 |
| GET | `/api/user/debug/tail?lines=N` | 读取日志尾部，前端默认 200 行。 |
| GET | `/api/user/debug/download` | 下载当前调试日志。 |

## 4. 管理员用户与授权

| 方法 | 路径 | 作用 |
|---|---|---|
| PUT | `/api/admin/credentials` | 修改主凭据后撤销全部 UI 会话。 |
| GET | `/api/admin/users` | 列出用户和状态。 |
| POST | `/api/admin/users/create` | 创建用户；前端可随后分配订阅文件。 |
| POST | `/api/admin/users/delete` | 按 username 删除用户及相关授权。 |
| POST | `/api/admin/users/status` | 启用/禁用账户。 |
| POST | `/api/admin/users/reset-password` | 重置指定用户密码。 |
| POST | `/api/admin/users/remark` | 更新管理员备注。 |
| POST | `/api/admin/users/custom-short-code` | 为指定用户设置自定义短码。 |
| GET | `/api/admin/users/{username}/subscriptions` | 获取用户可见订阅文件 ID。 |
| PUT | `/api/admin/users/{username}/subscriptions` | 覆盖用户与订阅文件关联。 |

## 5. 节点

`/api/admin/nodes` 使用一个前缀 Handler。节点对象包含名称、协议、服务器/端口、原始 URI、启停、排序、标签、探针服务器绑定、链式代理和同步元数据；协议专有配置还可存为结构化 JSON。

| 方法 | 路径 | 作用 |
|---|---|---|
| GET | `/api/admin/nodes` | 列出当前用户范围内节点。 |
| POST | `/api/admin/nodes` | 创建单节点；当前存储层要求 `protocol`。 |
| POST | `/api/admin/nodes/batch` | 批量创建并处理重名、缺省启用和 YAML 同步。 |
| POST | `/api/admin/nodes/fetch-subscription` | 安全拉取外部订阅并解析节点。 |
| POST | `/api/admin/nodes/parse-uris` | 解析 URI/V2Ray base64 文本，不保存。 |
| PUT / PATCH | `/api/admin/nodes/{id}` | 更新通用字段，并维护 relay/订阅 YAML 引用。 |
| DELETE | `/api/admin/nodes/{id}` | 删除节点并清理引用。 |
| PUT | `/api/admin/nodes/{id}/probe-binding` | 更新探针服务器绑定。 |
| PUT | `/api/admin/nodes/{id}/server` | 临时或同步式改写服务器地址。 |
| PUT | `/api/admin/nodes/{id}/restore-server` | 恢复原服务器。 |
| PUT | `/api/admin/nodes/{id}/config` | 更新协议专有配置。 |
| POST | `/api/admin/nodes/clear` | 清空节点。 |
| POST | `/api/admin/nodes/batch-delete` | 按 ID 批量删除。 |
| POST | `/api/admin/nodes/batch-rename` | 批量重命名。 |
| POST | `/api/admin/nodes/batch-disable-skip-cert` | 批量关闭跳过证书验证。 |
| GET | `/api/dns/resolve?hostname=` | 解析主机名，供节点服务器规范化。 |
| POST | `/api/admin/tcping` | 单地址 TCP connect 延迟探测。 |
| POST | `/api/admin/tcping/batch` | 批量 TCPing。 |

这些动作由同一个 Handler 的 method/path switch 分派；后续重构应为每个静态动作和 `/{id}` 参数路由分别注册并做路由级回归测试。

## 6. 外部订阅与 proxy-provider

| 方法 | 路径 | 作用 |
|---|---|---|
| GET / POST / PUT / DELETE | `/api/user/external-subscriptions[?id=]` | 当前用户的外部订阅 CRUD；保存 URL、过滤、自动更新、流量同步等设置。 |
| GET | `/api/user/external-subscriptions/nodes?id=` | 拉取并返回可选择节点。 |
| POST | `/api/user/external-subscriptions/check-filter` | 判断过滤器是否能命中节点。 |
| POST | `/api/admin/sync-external-subscriptions` | 批量同步到期或选中外部订阅；可 force。 |
| POST | `/api/admin/sync-external-subscription?id=` | 同步单个来源。 |
| POST | `/api/admin/sync-external-subscriptions/confirm` | 提交交互式节点选择确认。 |
| GET / POST / PUT / DELETE | `/api/user/proxy-provider-configs[?id=]` | 当前用户 provider 配置 CRUD，可按 external subscription 过滤。 |
| POST | `/api/user/proxy-provider-cache/refresh?id=` | 强制刷新指定 provider 缓存。 |
| GET | `/api/user/proxy-provider-cache/status?id=` | 查询缓存时间、错误和退避状态。 |
| GET | `/api/user/proxy-provider-nodes?id=` | 获取 provider 规范化节点列表。 |
| GET | `/api/proxy-provider/{config-id}?token=` | 对外提供 Clash proxy-provider YAML；按长期 token 鉴权并应用过滤、GeoIP、脚本和缓存。 |

外部 URL 拉取使用 SSRF 防护版本的 HTTP 客户端；代理集合缓存位于进程内，配置元数据位于 SQLite。

## 7. 订阅文件、短链与对外订阅

### 7.1 当前 `subscribe_files`

| 方法 | 路径 | 作用 |
|---|---|---|
| GET / POST | `/api/admin/subscribe-files` | 列表或创建元数据/文件。 |
| PUT / PATCH / DELETE | `/api/admin/subscribe-files/{id}` | 修改或删除。 |
| PUT | `/api/admin/subscribe-files/reorder` | 保存显示/处理顺序。 |
| POST | `/api/admin/subscribe-files/import` | 从已有配置导入。 |
| POST | `/api/admin/subscribe-files/upload` | multipart 上传。 |
| POST | `/api/admin/subscribe-files/create-from-config` | 由生成配置创建。 |
| POST | `/api/admin/subscribe-files/create-aggregate` | 创建聚合文件并保存成员关系。 |
| GET | `/api/admin/subscribe-files/{id}/users` | 查询获授权用户。 |
| GET / PUT | `/api/admin/subscribe-files/{filename}/content` | 读取/覆盖文件正文。 |
| GET | `/api/subscribe-files` | 普通用户列出自己获授权的订阅文件。 |
| GET | `/api/subscriptions` | 返回适合订阅页展示的可用订阅清单。 |

### 7.2 旧 `subscription_links`

`GET/POST /api/admin/subscriptions` 与 `PUT|PATCH|DELETE /api/admin/subscriptions/{short-code}` 管理旧式上传订阅链接。该模型与 `subscribe_files` 并存，重构迁移必须识别两类记录而不能简单按路径更名。

### 7.3 对外访问

| 方法 | 路径 | 鉴权与行为 |
|---|---|---|
| GET | `/api/clash/subscribe?...` | 用用户长期 token、文件/用户短码等参数鉴权；按文件、用户、UA 和格式参数生成最终内容。 |
| GET | `/{file-code}{user-code}` | 根 Handler 尝试解析组合短链，检查授权后转交订阅生成。 |
| POST | `/api/admin/temp-subscription` | 创建随机 8 位、带过期时间的进程内临时订阅。 |
| GET | `/t/{code}` | 访问未过期临时订阅。 |

最终订阅 Handler 还负责 UA 格式识别、未知 UA 守卫、IP 限速、暴力探测记录、外部同步、覆写脚本、规则/模板、relay 注入、排序去重和 Snell 兼容过滤。因此它不是简单的静态文件下载端点。

## 8. 规则、覆写和模板

| 方法 | 路径 | 作用 |
|---|---|---|
| GET | `/api/admin/rules/` | 列出规则文件。 |
| GET / PUT | `/api/admin/rules/{filename}` | 读取或更新规则正文。 |
| GET | `/api/admin/rules/{filename}/history` | 查询版本历史。 |
| GET | `/api/admin/rules/latest` | 最新规则版本元数据。 |
| GET / POST | `/api/admin/custom-rules` | 列表/创建自定义规则。 |
| GET / PUT / DELETE | `/api/admin/custom-rules/{id}` | 单条读取、更新、启停或删除。 |
| POST | `/api/admin/apply-custom-rules` | 把 DNS/rules/rule-providers 规则按策略合入目标 YAML。 |
| GET / POST | `/api/admin/override-scripts` | 列表/创建 JavaScript 覆写脚本。 |
| GET / PUT / DELETE | `/api/admin/override-scripts/{id}` | 读取/更新/排序/启停/删除脚本。 |
| GET / POST | `/api/admin/templates` | 旧 V2 数据库模板列表/创建。 |
| GET / PUT / DELETE | `/api/admin/templates/{id}` | 旧模板单条 CRUD。 |
| POST | `/api/admin/templates/convert` | V1/V2/目标配置转换与代理组补全。 |
| POST | `/api/admin/templates/fetch-source` | SSRF 防护地获取模板 URL。 |
| GET | `/api/admin/rule-templates` | 列出当前用户可见文件模板。 |
| POST | `/api/admin/rule-templates/upload` | 上传模板并记录 owner。 |
| POST | `/api/admin/rule-templates/rename` | 重命名所有者模板。 |
| PUT | `/api/admin/rule-templates/visibility` | 管理员切换公开状态。 |
| GET / PUT / DELETE | `/api/admin/rule-templates/{filename}` | 按 owner/public/admin 规则查看或修改。 |
| GET | `/api/admin/template-v3/` | 列出 V3 模板。 |
| POST | `/api/admin/template-v3/process` | 向已保存模板注入代理并生成。 |
| POST | `/api/admin/template-v3/preview` | 用提交的模板正文和代理预览。 |
| POST | `/api/admin/template-v3/preview-with-tags` | 按标签筛选代理后预览。 |
| POST | `/api/admin/template-v3/convert-v2` | 旧模板转 V3。 |
| POST | `/api/admin/template-v3/analyze-subscription` | 拉取/分析订阅结构以辅助建模板。 |
| GET | `/api/admin/template-v3/region-filters` | 返回内置地域过滤器。 |
| GET | `/api/proxy-groups` | 普通登录用户读取当前代理组分类快照。 |
| POST | `/api/admin/proxy-groups/sync` | 管理员触发远程同步分类。 |

## 9. 探针、流量与测速

| 方法 | 路径 | 作用 |
|---|---|---|
| GET / PUT / DELETE | `/api/admin/probe-config` | 获取、保存或移除 Nezha v1/v0、DStatus、Komari 探针配置。 |
| POST | `/api/admin/probe-sync` | 立即同步探针服务器与流量状态。 |
| GET | `/api/traffic/summary` | 聚合当前用户的探针流量与启用同步的外部订阅流量。 |
| GET | `/api/traffic/subscribe` | 返回每个订阅文件和探针总计的使用/限额。 |
| POST | `/api/admin/speedtest/run` | 创建节点测速；可走本地 Mihomo 或远程 tester。 |
| GET | `/api/admin/speedtest/results?node_id=&limit=` | 查询测速历史。 |
| GET | `/api/admin/speedtest/mihomo-status` | 查询本地 Mihomo 可用/版本状态。 |
| GET | `/api/admin/speedtest/testers` | 列出远程 tester 与在线状态。 |
| POST | `/api/admin/speedtest/testers/create` | 创建 tester 并仅此次返回 token。 |
| POST | `/api/admin/speedtest/testers/revoke` | 吊销 tester。 |
| POST | `/api/admin/speedtest/testers/rotate-token` | 轮换 token。 |
| GET/Upgrade | `/api/speedtest/tester/ws` | tester token 鉴权后升级 WebSocket，接收任务并回传结果。 |

## 10. 运维、安全、备份和更新

| 方法 | 路径 | 作用 |
|---|---|---|
| GET | `/api/admin/operations` | 分页查询管理员变更审计。 |
| GET | `/api/admin/tasks/types` | 返回后台任务机器名与中文名。 |
| GET | `/api/admin/tasks/runs?task=&status=&limit=&offset=` | 查询任务运行记录。 |
| GET | `/api/admin/security/events` | 查询登录/暴力探测等安全事件。 |
| GET / POST | `/api/admin/security/bans` | 列出或人工创建 IP 封禁。 |
| DELETE | `/api/admin/security/bans/{ip}` | 解封。 |
| GET / PUT | `/api/admin/security/turnstile` | 读取或更新 Turnstile 配置；GET 应隐藏 secret。 |
| GET / PUT | `/api/admin/notify-config` | 读取或更新 Telegram/事件开关配置。 |
| POST | `/api/admin/notify-config/test` | 发送测试通知。 |
| GET | `/api/admin/backup/download` | checkpoint 后打包数据库与受管文件并下载。 |
| POST | `/api/admin/backup/restore` | 上传并恢复备份。 |
| GET | `/api/admin/update/check` | 查询 GitHub Release 与当前版本差异。 |
| POST | `/api/admin/update/apply` | 备份、下载并替换本机二进制。 |
| GET/POST + SSE | `/api/admin/update/apply-sse` | 流式返回升级进度；支持 query token。 |

## 11. API 设计问题与重构输入

- 路径存在新旧资源并存：`subscriptions`、`subscribe-files`、`clash/subscribe` 表达三个不同概念；新 API 必须用领域名拆清“订阅定义、用户授权、发布端点、临时发布”。
- `/api/admin/rule-templates` 实际允许普通用户访问自有/公开模板，路径与中间件语义冲突。
- 单个 Handler 内手写 path/method 分派，部分宽泛条件可能遮蔽后续特例；Rust 路由应在框架层声明精确方法和参数，并生成 OpenAPI。
- body 大小、分页、过滤、错误码和 envelope 不统一；文件上传以外也缺少统一限制。
- API token 进入 query 和 URL，容易被访问日志、Referer 或浏览器历史记录留存；新系统要提供不含明文长期 token 的一次性展示和日志脱敏。
- WebSocket、SSE、静态下载和普通 JSON 需要不同超时策略，不能继续共享几乎无超时的默认 Server。
- 当前 API 文档来自源码事实，不代表新系统应复制其路径。重构兼容层应保留迁移所需旧入口，主 API 使用版本化 `/api/v1`、一致错误格式和显式 RBAC/作用域。
