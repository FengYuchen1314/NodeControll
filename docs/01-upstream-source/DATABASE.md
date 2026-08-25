# 妙妙屋 SQLite 数据库解剖

> 基线：`iluobei/miaomiaowu@0b47f10c52aee10b9f759a593ca5f61a823cbb72`。本页不是只抄静态 DDL：在 VPS 隔离实例中执行完整启动迁移后，再从生成数据库只读导出。机器可读证据见 `generated/database-schema.json`。

## 数据库运行方式

- 驱动为 `modernc.org/sqlite`，默认路径 `data/traffic.db`。
- 连接池上限强制为 1，因此所有 Repository 查询串行复用单连接；实现简单但会限制高并发写入。
- 启动设置 WAL、`busy_timeout=5000`、`synchronous=NORMAL`、64 MiB journal limit。
- 备份前执行 `wal_checkpoint(TRUNCATE)`；后台任务优先 TRUNCATE，繁忙时回退 PASSIVE。
- 源码未执行 `PRAGMA foreign_keys=ON`。虽然 8 条外键写进 schema，SQLite 默认连接不会强制它们；实际级联主要不能依赖数据库保证，这是重构迁移时必须修正的完整性缺口。
- 迁移没有版本表，而是在单个 `migrate()` 中反复 `CREATE TABLE IF NOT EXISTS`、读取 `PRAGMA table_info`、逐列 `ALTER TABLE`，并对少数历史结构重建表。可重复运行，但难以审计迁移版本和失败恢复点。

## 领域总览

| 表 | 领域 | 列 | 索引 | 外键 | 作用 |
|---|---|---:|---:|---:|---|
| `custom_rule_applications` | 订阅生成 | 8 | 3 | 2 | 记录某条自定义规则已向某订阅文件应用的内容与哈希，用于幂等/追踪。 |
| `custom_rules` | 订阅生成 | 8 | 3 | 0 | 自定义 DNS、rules、rule-providers 等规则片段及追加/替换模式。 |
| `external_subscriptions` | 订阅来源 | 16 | 3 | 1 | 用户导入的外部机场订阅、流量头、过期时间和自动更新策略。 |
| `ip_bans` | 安全 | 8 | 2 | 0 | 暴力探测和人工封禁状态；支持临时、永久、释放和重启恢复。 |
| `nodes` | 节点 | 17 | 4 | 0 | 规范化节点主表，同时保留原始 URI、解析 JSON、Clash JSON、标签和链式代理关系。 |
| `operation_logs` | 审计 | 7 | 2 | 0 | 管理员变更请求的操作者、方法、路径、状态码和来源 IP。 |
| `override_scripts` | 订阅生成 | 9 | 2 | 0 | 按用户保存的 JavaScript pre-save/post-fetch 覆写脚本及顺序。 |
| `probe_configs` | 探针 | 5 | 0 | 0 | 单例探针数据源类型和地址。 |
| `probe_servers` | 探针 | 9 | 1 | 1 | 探针面板内选中的服务器、流量口径、月流量和展示顺序。 |
| `proxy_provider_configs` | 订阅来源 | 23 | 2 | 1 | 把外部订阅转成 Clash proxy-provider 时的过滤、健康检查、覆写和处理位置。 |
| `rule_versions` | 订阅生成 | 6 | 1 | 0 | 规则 YAML 文件的不可变版本历史。 |
| `security_events` | 安全 | 8 | 3 | 0 | 登录失败、短链探测、封禁/解封等追加式安全事件流。 |
| `sessions` | 身份 | 4 | 3 | 0 | 管理 UI 登录会话；启动时回填到内存 TokenStore。 |
| `speed_test_results` | 测速 | 12 | 1 | 0 | 本地 Mihomo 或远程 tester 的节点下载速度、延迟、出口 IP 和状态。 |
| `speed_testers` | 测速 | 6 | 1 | 0 | 远程测速器身份、令牌哈希和最后在线时间。 |
| `subscribe_files` | 订阅产品 | 21 | 4 | 0 | 对外发布的订阅文件元数据、短码、模板、选中节点/标签/规则/脚本和流量设置。 |
| `subscription_links` | 订阅产品 | 9 | 2 | 0 | 较旧的订阅定义/规则入口模型，保存名称、规则文件、客户端按钮和短链。 |
| `system_config` | 系统 | 37 | 0 | 0 | 固定 id=1 的全局产品、安全、通知和输出行为配置。 |
| `system_settings` | 系统 | 3 | 1 | 0 | 键值型系统状态；用于初始化标记等不适合强类型单例表的设置。 |
| `task_runs` | 可观测性 | 6 | 2 | 0 | 后台任务名称、开始时间、耗时、状态和节流后的详情。 |
| `templates` | 订阅生成 | 9 | 2 | 0 | V2/远程模板定义、规则来源、代理开关和 include-all 行为。 |
| `traffic_records` | 流量 | 5 | 1 | 0 | 按日保存聚合流量上限、已用和剩余快照，供 30 日趋势图。 |
| `user_settings` | 身份 | 22 | 1 | 1 | 每用户的同步、模板、缓存、探针绑定、节点顺序、调试和短链偏好。 |
| `user_subscriptions` | 授权 | 3 | 3 | 2 | 用户与可访问订阅文件的多对多授权表。 |
| `user_tokens` | 订阅授权 | 5 | 3 | 0 | 长期订阅 token、系统短码与自定义用户短码；不同于 UI 会话。 |
| `users` | 身份 | 13 | 1 | 0 | 用户账户、密码哈希、角色、启用状态、资料、备注和 TOTP 恢复信息。 |

## 关系与完整性

- `custom_rule_applications.custom_rule_id` → `custom_rules.id`，删除策略 `CASCADE`。
- `custom_rule_applications.subscribe_file_id` → `subscribe_files.id`，删除策略 `CASCADE`。
- `external_subscriptions.username` → `users.username`，删除策略 `CASCADE`。
- `probe_servers.config_id` → `probe_configs.id`，删除策略 `CASCADE`。
- `proxy_provider_configs.external_subscription_id` → `external_subscriptions.id`，删除策略 `CASCADE`。
- `user_settings.username` → `users.username`，删除策略 `CASCADE`。
- `user_subscriptions.subscription_id` → `subscribe_files.id`，删除策略 `CASCADE`。
- `user_subscriptions.username` → `users.username`，删除策略 `CASCADE`。
- `nodes.chain_proxy_node_id` 和 `nodes.relay_group_node_ids` 是逻辑关系，没有数据库外键；删除节点后由 Go 代码遍历并修剪 relay 成员。
- 多个 `subscribe_files.selected_*` 字段把 ID 数组编码进 TEXT，数据库无法保证引用存在，也无法高效反向查询。
- `subscription_links` 与 `subscribe_files` 是并存的两套订阅概念；前者偏规则/按钮入口，后者是当前文件、模板、节点和授权聚合根。重构时需要显式统一或定义边界。

## 完整字段目录

### `custom_rule_applications`

领域：订阅生成。记录某条自定义规则已向某订阅文件应用的内容与哈希，用于幂等/追踪。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `subscribe_file_id` | `INTEGER` | 是 | `—` | 0 | 关联的 subscribe_file 标识。 |
| `custom_rule_id` | `INTEGER` | 是 | `—` | 0 | 关联的 custom_rule 标识。 |
| `rule_type` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `rule_mode` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `applied_content` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `content_hash` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `applied_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 业务事件时间。 |

索引：
- `idx_custom_rule_applications_rule`：`(custom_rule_id)`，非唯一。
- `idx_custom_rule_applications_file`：`(subscribe_file_id)`，非唯一。
- `sqlite_autoindex_custom_rule_applications_1`：`(subscribe_file_id, custom_rule_id, rule_type)`，唯一。

外键声明（注意运行时未显式开启 SQLite 外键强制）：
- `custom_rule_id` → `custom_rules.id`，ON DELETE `CASCADE`。
- `subscribe_file_id` → `subscribe_files.id`，ON DELETE `CASCADE`。

### `custom_rules`

领域：订阅生成。自定义 DNS、rules、rule-providers 等规则片段及追加/替换模式。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `name` | `TEXT` | 是 | `—` | 0 | 用户可见名称。 |
| `type` | `TEXT` | 是 | `—` | 0 | 业务类型判别值。 |
| `mode` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `content` | `TEXT` | 是 | `—` | 0 | 原始规则、模板或脚本文本。 |
| `enabled` | `INTEGER` | 是 | `1` | 0 | 启用开关，SQLite INTEGER 布尔值。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |

索引：
- `idx_custom_rules_enabled`：`(enabled)`，非唯一。
- `idx_custom_rules_type`：`(type)`，非唯一。
- `sqlite_autoindex_custom_rules_1`：`(name, type)`，唯一。

### `external_subscriptions`

领域：订阅来源。用户导入的外部机场订阅、流量头、过期时间和自动更新策略。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `username` | `TEXT` | 是 | `—` | 0 | 用户主键/所有者。 |
| `name` | `TEXT` | 是 | `—` | 0 | 用户可见名称。 |
| `url` | `TEXT` | 是 | `—` | 0 | 源地址或对外地址。 |
| `node_count` | `INTEGER` | 是 | `0` | 0 | 计数值。 |
| `last_sync_at` | `TIMESTAMP` | 否 | `—` | 0 | 最近成功同步时间。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |
| `upload` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `download` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `total` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `expire` | `TIMESTAMP` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `user_agent` | `TEXT` | 是 | `'clash-meta/2.4.0'` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `traffic_mode` | `TEXT` | 是 | `'both'` | 0 | 流量计算口径：上传、下载或两者。 |
| `auto_update` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `update_interval_minutes` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |

索引：
- `idx_external_subscriptions_url`：`(url)`，非唯一。
- `idx_external_subscriptions_username`：`(username)`，非唯一。
- `sqlite_autoindex_external_subscriptions_1`：`(username, url)`，唯一。

外键声明（注意运行时未显式开启 SQLite 外键强制）：
- `username` → `users.username`，ON DELETE `CASCADE`。

### `ip_bans`

领域：安全。暴力探测和人工封禁状态；支持临时、永久、释放和重启恢复。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `ip` | `TEXT` | 否 | `—` | 1 | 来源或被封禁 IP。 |
| `reason` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `banned_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 业务事件时间。 |
| `expires_at` | `TIMESTAMP` | 否 | `—` | 0 | 失效时间。 |
| `permanent` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `fail_count` | `INTEGER` | 是 | `0` | 0 | 计数值。 |
| `released_at` | `TIMESTAMP` | 否 | `—` | 0 | 业务事件时间。 |
| `actor` | `TEXT` | 是 | `''` | 0 | 操作发起者。 |

索引：
- `idx_ip_bans_active`：`(released_at, expires_at)`，非唯一。
- `sqlite_autoindex_ip_bans_1`：`(ip)`，唯一。

### `nodes`

领域：节点。规范化节点主表，同时保留原始 URI、解析 JSON、Clash JSON、标签和链式代理关系。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `username` | `TEXT` | 是 | `—` | 0 | 用户主键/所有者。 |
| `raw_url` | `TEXT` | 是 | `—` | 0 | 导入时的原始代理 URI。 |
| `node_name` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `protocol` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `parsed_config` | `TEXT` | 是 | `—` | 0 | 解析器得到的协议 JSON。 |
| `clash_config` | `TEXT` | 是 | `—` | 0 | 用于订阅生成的 Clash/Mihomo 代理 JSON。 |
| `enabled` | `INTEGER` | 是 | `1` | 0 | 启用开关，SQLite INTEGER 布尔值。 |
| `tag` | `TEXT` | 是 | `'手动输入'` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |
| `original_server` | `TEXT` | 否 | `—` | 0 | 改写服务器地址前的原始地址，用于恢复。 |
| `probe_server` | `TEXT` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `tags` | `TEXT` | 是 | `'[]'` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `chain_proxy_node_id` | `INTEGER` | 否 | `—` | 0 | 单节点链式代理引用；未声明数据库外键。 |
| `relay_group_name` | `TEXT` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `relay_group_node_ids` | `TEXT` | 否 | `—` | 0 | 中转/relay 组成员 ID 的 JSON 数组。 |

索引：
- `idx_nodes_tag`：`(tag)`，非唯一。
- `idx_nodes_enabled`：`(enabled)`，非唯一。
- `idx_nodes_protocol`：`(protocol)`，非唯一。
- `idx_nodes_username`：`(username)`，非唯一。

### `operation_logs`

领域：审计。管理员变更请求的操作者、方法、路径、状态码和来源 IP。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `actor` | `TEXT` | 是 | `''` | 0 | 操作发起者。 |
| `method` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `path` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `status` | `INTEGER` | 是 | `—` | 0 | 执行状态。 |
| `ip` | `TEXT` | 是 | `''` | 0 | 来源或被封禁 IP。 |

索引：
- `idx_operation_logs_actor_at`：`(actor, at)`，非唯一。
- `idx_operation_logs_at`：`(at)`，非唯一。

### `override_scripts`

领域：订阅生成。按用户保存的 JavaScript pre-save/post-fetch 覆写脚本及顺序。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `username` | `TEXT` | 是 | `—` | 0 | 用户主键/所有者。 |
| `name` | `TEXT` | 是 | `—` | 0 | 用户可见名称。 |
| `hook` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `content` | `TEXT` | 是 | `—` | 0 | 原始规则、模板或脚本文本。 |
| `enabled` | `INTEGER` | 是 | `1` | 0 | 启用开关，SQLite INTEGER 布尔值。 |
| `sort_order` | `INTEGER` | 是 | `0` | 0 | 显式展示/执行顺序。 |
| `created_at` | `TIMESTAMP` | 否 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 否 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |

索引：
- `idx_override_scripts_hook`：`(hook)`，非唯一。
- `idx_override_scripts_username`：`(username)`，非唯一。

### `probe_configs`

领域：探针。单例探针数据源类型和地址。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `probe_type` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `address` | `TEXT` | 是 | `—` | 0 | 远端服务地址。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |

### `probe_servers`

领域：探针。探针面板内选中的服务器、流量口径、月流量和展示顺序。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `config_id` | `INTEGER` | 是 | `—` | 0 | 关联的 config 标识。 |
| `server_id` | `TEXT` | 是 | `—` | 0 | 关联的 server 标识。 |
| `name` | `TEXT` | 是 | `—` | 0 | 用户可见名称。 |
| `traffic_method` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `monthly_traffic_bytes` | `INTEGER` | 是 | `0` | 0 | 字节数。 |
| `position` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |

索引：
- `sqlite_autoindex_probe_servers_1`：`(config_id, server_id)`，唯一。

外键声明（注意运行时未显式开启 SQLite 外键强制）：
- `config_id` → `probe_configs.id`，ON DELETE `CASCADE`。

### `proxy_provider_configs`

领域：订阅来源。把外部订阅转成 Clash proxy-provider 时的过滤、健康检查、覆写和处理位置。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `username` | `TEXT` | 是 | `—` | 0 | 用户主键/所有者。 |
| `external_subscription_id` | `INTEGER` | 是 | `—` | 0 | 关联的 external_subscription 标识。 |
| `name` | `TEXT` | 是 | `—` | 0 | 用户可见名称。 |
| `type` | `TEXT` | 是 | `'http'` | 0 | 业务类型判别值。 |
| `interval` | `INTEGER` | 否 | `3600` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `proxy` | `TEXT` | 否 | `'DIRECT'` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `size_limit` | `INTEGER` | 否 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `header` | `TEXT` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `health_check_enabled` | `INTEGER` | 否 | `1` | 0 | 功能开关或该功能的参数。 |
| `health_check_url` | `TEXT` | 否 | `'https://www.gstatic.com/generate_204'` | 0 | 功能开关或该功能的参数。 |
| `health_check_interval` | `INTEGER` | 否 | `300` | 0 | 功能开关或该功能的参数。 |
| `health_check_timeout` | `INTEGER` | 否 | `5000` | 0 | 功能开关或该功能的参数。 |
| `health_check_lazy` | `INTEGER` | 否 | `1` | 0 | 功能开关或该功能的参数。 |
| `health_check_expected_status` | `INTEGER` | 否 | `204` | 0 | 功能开关或该功能的参数。 |
| `filter` | `TEXT` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `exclude_filter` | `TEXT` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `exclude_type` | `TEXT` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `geo_ip_filter` | `TEXT` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `override` | `TEXT` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `process_mode` | `TEXT` | 否 | `'client'` | 0 | provider 在客户端侧还是妙妙屋服务端预处理。 |
| `created_at` | `TIMESTAMP` | 否 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 否 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |

索引：
- `idx_proxy_provider_configs_external_subscription_id`：`(external_subscription_id)`，非唯一。
- `idx_proxy_provider_configs_username`：`(username)`，非唯一。

外键声明（注意运行时未显式开启 SQLite 外键强制）：
- `external_subscription_id` → `external_subscriptions.id`，ON DELETE `CASCADE`。

### `rule_versions`

领域：订阅生成。规则 YAML 文件的不可变版本历史。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `filename` | `TEXT` | 是 | `—` | 0 | 磁盘文件名。 |
| `version` | `INTEGER` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `content` | `TEXT` | 是 | `—` | 0 | 原始规则、模板或脚本文本。 |
| `created_by` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |

索引：
- `sqlite_autoindex_rule_versions_1`：`(filename, version)`，唯一。

### `security_events`

领域：安全。登录失败、短链探测、封禁/解封等追加式安全事件流。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `ip` | `TEXT` | 是 | `—` | 0 | 来源或被封禁 IP。 |
| `kind` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `path` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `username` | `TEXT` | 是 | `''` | 0 | 用户主键/所有者。 |
| `detail` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `actor` | `TEXT` | 是 | `''` | 0 | 操作发起者。 |

索引：
- `idx_sec_events_kind_at`：`(kind, at)`，非唯一。
- `idx_sec_events_ip`：`(ip)`，非唯一。
- `idx_sec_events_at`：`(at)`，非唯一。

### `sessions`

领域：身份。管理 UI 登录会话；启动时回填到内存 TokenStore。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `token` | `TEXT` | 否 | `—` | 1 | 令牌明文（按表的用途解释）。 |
| `username` | `TEXT` | 是 | `—` | 0 | 用户主键/所有者。 |
| `expires_at` | `TIMESTAMP` | 是 | `—` | 0 | 失效时间。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |

索引：
- `idx_sessions_expires_at`：`(expires_at)`，非唯一。
- `idx_sessions_username`：`(username)`，非唯一。
- `sqlite_autoindex_sessions_1`：`(token)`，唯一。

### `speed_test_results`

领域：测速。本地 Mihomo 或远程 tester 的节点下载速度、延迟、出口 IP 和状态。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `node_id` | `INTEGER` | 是 | `—` | 0 | 关联的 node 标识。 |
| `node_name` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `source` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `down_mbps` | `REAL` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `latency_ms` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `test_bytes` | `INTEGER` | 是 | `0` | 0 | 字节数。 |
| `status` | `TEXT` | 是 | `'running'` | 0 | 执行状态。 |
| `error` | `TEXT` | 否 | `''` | 0 | 失败错误文本。 |
| `egress_ip` | `TEXT` | 否 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `tested_by` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |

索引：
- `idx_speed_test_node`：`(node_id)`，非唯一。

### `speed_testers`

领域：测速。远程测速器身份、令牌哈希和最后在线时间。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `name` | `TEXT` | 是 | `—` | 0 | 用户可见名称。 |
| `token_hash` | `TEXT` | 是 | `—` | 0 | 不可逆令牌哈希。 |
| `created_by` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `last_seen` | `TIMESTAMP` | 否 | `—` | 0 | 最后心跳/在线时间。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |

索引：
- `sqlite_autoindex_speed_testers_1`：`(token_hash)`，唯一。

### `subscribe_files`

领域：订阅产品。对外发布的订阅文件元数据、短码、模板、选中节点/标签/规则/脚本和流量设置。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `name` | `TEXT` | 是 | `—` | 0 | 用户可见名称。 |
| `description` | `TEXT` | 否 | `—` | 0 | 可选说明。 |
| `url` | `TEXT` | 是 | `—` | 0 | 源地址或对外地址。 |
| `type` | `TEXT` | 是 | `—` | 0 | 业务类型判别值。 |
| `filename` | `TEXT` | 是 | `—` | 0 | 磁盘文件名。 |
| `expire_at` | `TIMESTAMP` | 否 | `—` | 0 | 产品配置的到期时间。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |
| `file_short_code` | `TEXT` | 是 | `''` | 0 | 短码或验证码相关值。 |
| `custom_short_code` | `TEXT` | 是 | `''` | 0 | 用户自定义值。 |
| `raw_output` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `traffic_limit` | `REAL` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `stats_server_ids` | `TEXT` | 是 | `''` | 0 | 用于该订阅流量统计的探针服务器 ID 编码。 |
| `auto_sync_custom_rules` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `template_filename` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `selected_tags` | `TEXT` | 是 | `'[]'` | 0 | 生成订阅时选择的节点标签 JSON。 |
| `selected_node_ids` | `TEXT` | 是 | `'[]'` | 0 | 显式选中的节点 ID JSON。 |
| `selected_custom_rule_ids` | `TEXT` | 是 | `'[]'` | 0 | 绑定的自定义规则 ID JSON。 |
| `selected_override_script_ids` | `TEXT` | 是 | `'[]'` | 0 | 绑定的覆写脚本 ID JSON。 |
| `sort_order` | `INTEGER` | 是 | `0` | 0 | 显式展示/执行顺序。 |

索引：
- `idx_subscribe_files_custom_short_code`：`(custom_short_code)`，唯一。
- `idx_subscribe_files_file_short_code`：`(file_short_code)`，唯一。
- `idx_subscribe_files_type`：`(type)`，非唯一。
- `sqlite_autoindex_subscribe_files_1`：`(name)`，唯一。

### `subscription_links`

领域：订阅产品。较旧的订阅定义/规则入口模型，保存名称、规则文件、客户端按钮和短链。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `name` | `TEXT` | 是 | `—` | 0 | 用户可见名称。 |
| `type` | `TEXT` | 是 | `''` | 0 | 业务类型判别值。 |
| `description` | `TEXT` | 否 | `—` | 0 | 可选说明。 |
| `rule_filename` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `buttons` | `TEXT` | 是 | `'[]'` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |
| `short_url` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |

索引：
- `idx_subscription_links_short_url`：`(short_url)`，唯一。
- `sqlite_autoindex_subscription_links_1`：`(name)`，唯一。

### `system_config`

领域：系统。固定 id=1 的全局产品、安全、通知和输出行为配置。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `proxy_groups_source_url` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |
| `client_compatibility_mode` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `silent_mode` | `INTEGER` | 是 | `0` | 0 | 静默模式总开关：正常情况下伪装 404。 |
| `silent_mode_timeout` | `INTEGER` | 是 | `15` | 0 | 订阅访问/启动后临时恢复 UI 的分钟数。 |
| `enable_sub_info_nodes` | `INTEGER` | 是 | `0` | 0 | 是否把到期和剩余流量合成为提示节点。 |
| `sub_info_expire_prefix` | `TEXT` | 是 | `'📅过期时间'` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `sub_info_traffic_prefix` | `TEXT` | 是 | `'⌛剩余流量'` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `enable_override_scripts` | `INTEGER` | 是 | `0` | 0 | 功能开关或该功能的参数。 |
| `enable_short_link` | `INTEGER` | 是 | `1` | 0 | 功能开关或该功能的参数。 |
| `enable_sub_traffic_header` | `INTEGER` | 是 | `1` | 0 | 是否输出 Subscription-Userinfo 等流量响应头。 |
| `subscription_output_format` | `TEXT` | 是 | `'yaml'` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `notify_enabled` | `INTEGER` | 是 | `0` | 0 | 功能开关或该功能的参数。 |
| `telegram_bot_token` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `telegram_chat_id` | `TEXT` | 是 | `''` | 0 | 关联的 telegram_chat 标识。 |
| `notify_subscribe_fetch` | `INTEGER` | 是 | `1` | 0 | 功能开关或该功能的参数。 |
| `notify_login` | `INTEGER` | 是 | `1` | 0 | 功能开关或该功能的参数。 |
| `notify_ip_ban` | `INTEGER` | 是 | `1` | 0 | 功能开关或该功能的参数。 |
| `notify_silent_mode` | `INTEGER` | 是 | `1` | 0 | 功能开关或该功能的参数。 |
| `notify_daily_traffic` | `INTEGER` | 是 | `0` | 0 | 功能开关或该功能的参数。 |
| `notify_expiry` | `INTEGER` | 是 | `1` | 0 | 功能开关或该功能的参数。 |
| `notify_daily_traffic_time` | `TEXT` | 是 | `'08:00'` | 0 | 功能开关或该功能的参数。 |
| `enable_two_factor` | `INTEGER` | 是 | `0` | 0 | 功能开关或该功能的参数。 |
| `login_rate_max_attempts` | `INTEGER` | 是 | `5` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `login_rate_window` | `INTEGER` | 是 | `60` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `login_rate_lock_duration` | `INTEGER` | 是 | `60` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `brute_force_enabled` | `INTEGER` | 是 | `1` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `brute_force_max_failures` | `INTEGER` | 是 | `5` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `brute_force_window` | `INTEGER` | 是 | `1440` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `brute_force_block_duration` | `INTEGER` | 是 | `1440` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `sub_rate_limit_enabled` | `INTEGER` | 是 | `1` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `sub_rate_limit_max` | `INTEGER` | 是 | `30` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `sub_rate_limit_window` | `INTEGER` | 是 | `120` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `skip_local_ip` | `INTEGER` | 是 | `1` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `block_unknown_subscription_ua` | `INTEGER` | 是 | `0` | 0 | 是否拒绝未知订阅客户端 UA。 |

### `system_settings`

领域：系统。键值型系统状态；用于初始化标记等不适合强类型单例表的设置。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `key` | `TEXT` | 否 | `—` | 1 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `value` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |

索引：
- `sqlite_autoindex_system_settings_1`：`(key)`，唯一。

### `task_runs`

领域：可观测性。后台任务名称、开始时间、耗时、状态和节流后的详情。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `task_name` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `started_at` | `TIMESTAMP` | 是 | `—` | 0 | 业务事件时间。 |
| `duration_ms` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `status` | `TEXT` | 是 | `—` | 0 | 执行状态。 |
| `detail` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |

索引：
- `idx_task_runs_started`：`(started_at)`，非唯一。
- `idx_task_runs_name_started`：`(task_name, started_at)`，非唯一。

### `templates`

领域：订阅生成。V2/远程模板定义、规则来源、代理开关和 include-all 行为。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `id` | `INTEGER` | 否 | `—` | 1 | 行主键。 |
| `name` | `TEXT` | 是 | `—` | 0 | 用户可见名称。 |
| `category` | `TEXT` | 是 | `'clash'` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `template_url` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `rule_source` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `use_proxy` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `enable_include_all` | `INTEGER` | 是 | `0` | 0 | 功能开关或该功能的参数。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |

索引：
- `idx_templates_category`：`(category)`，非唯一。
- `sqlite_autoindex_templates_1`：`(name)`，唯一。

### `traffic_records`

领域：流量。按日保存聚合流量上限、已用和剩余快照，供 30 日趋势图。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `date` | `TEXT` | 否 | `—` | 1 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `total_limit` | `INTEGER` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `total_used` | `INTEGER` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `total_remaining` | `INTEGER` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |

索引：
- `sqlite_autoindex_traffic_records_1`：`(date)`，唯一。

### `user_settings`

领域：身份。每用户的同步、模板、缓存、探针绑定、节点顺序、调试和短链偏好。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `username` | `TEXT` | 否 | `—` | 1 | 用户主键/所有者。 |
| `force_sync_external` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |
| `match_rule` | `TEXT` | 是 | `'node_name'` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `cache_expire_minutes` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `sync_traffic` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `enable_probe_binding` | `INTEGER` | 是 | `0` | 0 | 功能开关或该功能的参数。 |
| `sync_scope` | `TEXT` | 是 | `'saved_only'` | 0 | 外部订阅同步到已保存节点还是更大范围。 |
| `keep_node_name` | `INTEGER` | 是 | `1` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `custom_rules_enabled` | `INTEGER` | 是 | `0` | 0 | 用户自定义值。 |
| `enable_short_link` | `INTEGER` | 是 | `0` | 0 | 功能开关或该功能的参数。 |
| `template_version` | `TEXT` | 是 | `'v2'` | 0 | 用户选择的模板系统版本。 |
| `enable_proxy_provider` | `INTEGER` | 是 | `0` | 0 | 功能开关或该功能的参数。 |
| `node_order` | `TEXT` | 是 | `'[]'` | 0 | 用户自定义节点顺序 JSON。 |
| `debug_enabled` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `debug_log_path` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `debug_started_at` | `TIMESTAMP` | 否 | `—` | 0 | 业务事件时间。 |
| `node_name_filter` | `TEXT` | 是 | `'剩余\|流量\|到期\|订阅\|时间\|重置'` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `append_sub_info` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `default_template_filename` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `default_surge_template_filename` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |

索引：
- `sqlite_autoindex_user_settings_1`：`(username)`，唯一。

外键声明（注意运行时未显式开启 SQLite 外键强制）：
- `username` → `users.username`，ON DELETE `CASCADE`。

### `user_subscriptions`

领域：授权。用户与可访问订阅文件的多对多授权表。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `username` | `TEXT` | 是 | `—` | 1 | 用户主键/所有者。 |
| `subscription_id` | `INTEGER` | 是 | `—` | 2 | 关联的 subscription 标识。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |

索引：
- `idx_user_subscriptions_subscription_id`：`(subscription_id)`，非唯一。
- `idx_user_subscriptions_username`：`(username)`，非唯一。
- `sqlite_autoindex_user_subscriptions_1`：`(username, subscription_id)`，唯一。

外键声明（注意运行时未显式开启 SQLite 外键强制）：
- `subscription_id` → `subscribe_files.id`，ON DELETE `CASCADE`。
- `username` → `users.username`，ON DELETE `CASCADE`。

### `user_tokens`

领域：订阅授权。长期订阅 token、系统短码与自定义用户短码；不同于 UI 会话。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `username` | `TEXT` | 否 | `—` | 1 | 用户主键/所有者。 |
| `token` | `TEXT` | 是 | `—` | 0 | 令牌明文（按表的用途解释）。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |
| `user_short_code` | `TEXT` | 是 | `''` | 0 | 短码或验证码相关值。 |
| `custom_user_short_code` | `TEXT` | 是 | `''` | 0 | 用户自定义值。 |

索引：
- `idx_user_tokens_custom_user_short_code`：`(custom_user_short_code)`，唯一。
- `idx_user_tokens_user_short_code`：`(user_short_code)`，唯一。
- `sqlite_autoindex_user_tokens_1`：`(username)`，唯一。

### `users`

领域：身份。用户账户、密码哈希、角色、启用状态、资料、备注和 TOTP 恢复信息。

| 列 | 类型 | 非空 | 默认值 | 主键序号 | 语义 |
|---|---|---|---|---:|---|
| `username` | `TEXT` | 否 | `—` | 1 | 用户主键/所有者。 |
| `password_hash` | `TEXT` | 是 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `email` | `TEXT` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `nickname` | `TEXT` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `avatar_url` | `TEXT` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `role` | `TEXT` | 是 | `'user'` | 0 | 用户角色。 |
| `is_active` | `INTEGER` | 是 | `1` | 0 | 账户启用状态。 |
| `created_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 创建时间。 |
| `updated_at` | `TIMESTAMP` | 是 | `CURRENT_TIMESTAMP` | 0 | 最后更新时间。 |
| `remark` | `TEXT` | 否 | `—` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `totp_secret` | `TEXT` | 是 | `''` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `totp_enabled` | `INTEGER` | 是 | `0` | 0 | 对应模型的持久化字段；精确读写行为见 storage 函数索引。 |
| `recovery_codes` | `TEXT` | 是 | `'[]'` | 0 | 一次性恢复码哈希的 JSON 数组。 |

索引：
- `sqlite_autoindex_users_1`：`(username)`，唯一。

