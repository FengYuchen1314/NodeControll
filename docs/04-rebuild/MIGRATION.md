# 妙妙屋/妙妙屋 X 导入、升级与回滚计划

## 1. 迁移原则

迁移不是直接把旧 SQLite 表改造成新表。NodeControll 使用“只读采集→规范化 staging→验证/预览→写入新库→行为对账→切换”的 importer；旧数据库、配置和生成文件在确认窗口内保持只读可回退。

硬规则：

- 原始实例先一致性快照并计算 manifest/hash；导入器绝不写旧库。
- 每次 import 有唯一 run、source fingerprint、工具版本和 row-level mapping；同一源可幂等重跑。
- secret 不出现在报告/日志；检测到明文后立即加密进入新 secret store，报告只写存在性和 fingerprint prefix。
- 不可确定的枚举、脚本、协议或引用进入 quarantine/diagnostic，不猜默认值。
- 新系统生成新 session、API token、Agent/federation identity；旧密码若算法可识别可暂存兼容验证并在首次登录 rehash。
- 所有 PRO 功能只作为普通 feature 导入；任何 license、官方域名、官方 server ID/激活 token 均不带入授权逻辑。

## 2. 支持的来源

| Source type | 发现方式 | 保真级别 |
|---|---|---|
| 妙妙屋社区版 SQLite | `data/traffic.db` + WAL/SHM、一致 backup API | 完整 schema 已按基线 26 表锁定 |
| 妙妙屋 data 目录 | rules/templates/generated/config/assets | 文件 hash + 安全 parser，按引用导入 |
| 妙妙屋备份包 | 先安全解包到隔离目录，再按上述两类 | 不信任 archive path/manifest |
| 妙妙屋 X | 用户提供的备份/DB/config；先执行 read-only detector | 公开源码/正式 schema 不全，必须 schema fingerprint adapter |
| 通用订阅/Clash/sing-box | URI/YAML/JSON/URL | 经 [SUBSCRIPTION_IR.md](./SUBSCRIPTION_IR.md) parser 导入 |
| NodeControll 旧版本 | versioned native backup | 完整自动 migration + restore rehearsal |

妙妙屋 X importer 不把文档描述当数据库事实。每个已发现 schema fingerprint 单独写 adapter、fixture、支持范围和拒绝条件；未知 fingerprint 只能生成 inventory，不能开始写目标库。

## 3. 迁移工作流和状态机

```text
created → inspecting → inspected → awaiting_confirmation
       → importing → validating → ready_to_cutover
       → cutover_running → completed
       ↘ failed / cancelled / rolled_back
```

1. `inspect`：检查 source type/version、SQLite integrity、文件、安全、行数、时间/大小、secret/外部依赖，不创建业务对象。
2. `plan`：生成对象/引用映射、冲突、转换、quarantine、不可迁移项、磁盘和预计窗口；用户下载报告。
3. `confirm`：owner recent-auth + source hash + target empty/merge strategy + typed phrase。
4. `import`：在新 target transaction/namespace 写入，逐批 checkpoint；异常可整体丢弃 staging。
5. `validate`：FK、domain invariant、订阅 golden、流量总和、用户/节点/源/模板计数、secret decrypt、登录兼容。
6. `rehearse`：只读启动新 Master，compile configs、生成订阅并和旧输出做 semantic diff；不连接/修改远端 Agent。
7. `cutover`：冻结旧写入、做增量/最终 snapshot、重跑差异、切换 reverse proxy；旧服务仅 loopback/read-only。
8. `observe`：至少 24 小时或用户设定窗口；health、登录、订阅、jobs、traffic、Agent 无关键异常后完成。
9. `rollback`：切回旧服务与原 snapshot；新库保留为失败证据，不反向写旧库。

## 4. 社区版 26 表映射

| 旧表 | 新目标 | 转换与风险 |
|---|---|---|
| `users` | `users`,`user_identities`,`password_credentials`,`recovery_codes` | username→UUID map；role 归一；旧 hash 识别/首次登录 rehash；TOTP/recovery 字段分别加密/hash |
| `sessions` | 默认不导入 | 切换后重新登录；避免复制可重放 bearer |
| `user_tokens` | `profile_tokens` 或 `api_tokens` | 按旧用途区分订阅短码/长期 token；只存新 hash，可保留原 token 以避免客户端全部失效，但必须 owner 明确选择并立即可轮换 |
| `user_settings` | `user_preferences`,`profiles`,`profile_nodes` 等 | 拆 JSON 数组/布尔；引用缺失 quarantine；系统行为字段迁入 profile policy |
| `user_subscriptions` | `profile_grants` | username/subscription integer ID 双映射，孤儿引用报告 |
| `nodes` | `nodes`,`node_tags`,`node_sources`,`node_links` | raw URI/parsed/clash 先转 IR；链式和 relay JSON 改 FK/ordered members；未识别协议禁用但保留原文 hash |
| `external_subscriptions` | `subscription_sources`,`source_revisions`,`source_items` | URL credential 抽 secret；流量 header 快照变 source metadata，不混入账本；schedule 归一 |
| `proxy_provider_configs` | `providers`,`provider_filters`,`profile_tokens` | source FK、include/exclude/health/override 拆 typed 配置；处理位置语义明确化 |
| `subscribe_files` | `profiles`,`profile_inputs`,`profile_nodes`,`profile_rules`,`profile_scripts`,`profile_tokens` | selected JSON 全部 join 表；short code 冲突；模板/流量/客户端设置拆分；生成 artifact 重建 |
| `subscription_links` | `profiles` 或 `legacy_links` quarantine | 与 subscribe_files 重复时按引用/短链/规则匹配合并；不能自动判断则保留两个 draft |
| `templates` | `templates`,`template_versions`,`profile_templates` | 内置/远程/用户模板标来源；远程 URL 经安全 fetch；内容 immutable version |
| `custom_rules` | `rule_libraries`,`rule_versions` | type/mode 映射；先 lint；非法内容 draft+diagnostic |
| `custom_rule_applications` | `profile_rules`,`migration_mappings` | 不复制缓存式 applied_content 为事实源；用 source/version hash 重建 |
| `rule_versions` | `rule_versions` | 保留时间/hash/内容；重新计算 canonical hash 并对比旧 hash |
| `override_scripts` | `subscription_scripts`,`script_versions` quarantine | 旧 JavaScript 不直接执行；默认 disabled，转换为人工审阅输入或重写 WASM；hook/order 保留 |
| `traffic_records` | `traffic_import_baselines` + `traffic_ledger_entries` | 只有按日 used/total/remaining 快照，不伪造 upload/download原始采样；建立 baseline 并标 `legacy_aggregate` |
| `speed_test_results` | `speed_test_runs`,`speed_test_samples` | 单结果映为 final sample，tester/node 字符串尽量映射；原始错误脱敏 |
| `speed_testers` | `testers`,`tester_pairings` | 不导入可用 bearer；创建 disabled tester，需重新配对 |
| `probe_configs` | `probe_settings` | type/address 变公开 projection source；外部 URL重验 SSRF policy |
| `probe_servers` | `probe_server_projections` | server_id 可能是外部字符串；不能映射则 draft placeholder，不曝光 |
| `system_config` | `instance_settings`,`notification_rules`,`assets`,`secret_records` | 37 列按 section schema；password/token/webhook 等敏感值加密；未知列进入只读 import report，不进通用 JSON |
| `system_settings` | `instance_settings` 或 migration marker | key allowlist；initialized 不覆盖目标已有状态 |
| `ip_bans` | `security_blocks` | IP canonicalize、expiry/released；过期项可只进历史事件 |
| `security_events` | `audit_entries`/`security_events` | append with `imported=true`；旧链无完整性保证，作为单独 epoch |
| `operation_logs` | `audit_entries` | method/path/status 转安全摘要；无 before/after，标 evidence grade legacy |
| `task_runs` | `job_history_imports` | 只作历史投影，不变成可重试 durable job |

所有旧 integer ID 保存到 `migration_mappings(run_id,source_type,source_table,source_key,target_type,target_id,source_hash,status,diagnostic)`，不把旧 ID 暴露为新资源 ID。

## 5. 文件与目录映射

| 旧内容 | 处理 |
|---|---|
| rules/templates | 只导入数据库或实际引用的文件；path canonicalize、大小/MIME/YAML 限制，内容变 immutable version |
| generated subscriptions | 不作为新事实源；用于 cutover semantic diff，随后由固定 IR 重新生成 |
| logos/backgrounds | magic decode/re-encode，随机 object ID，记录旧 hash；不复制可执行/未知文件 |
| config/env | 只识别登记 key；secret 进入 secret store；端口/URL/storage 用 plan 展示冲突 |
| SQLite WAL/SHM | 通过 SQLite backup API/checkpoint 得到一致 DB，不直接复制活动主文件 |
| tester/probe tokens | 默认废弃并重新配对；报告数量，不回显 |
| logs | 默认不导入；用户显式选择后加密归档为 legacy evidence，不进入新在线日志 |

安全 archive reader 拒绝绝对路径、`..`、symlink/hardlink/device、case collision、重复 entry 和压缩炸弹。导入缓存目录只在 task-scoped 根下，完成/失败后按策略可恢复删除。

## 6. 节点和订阅语义转换

旧 `nodes.raw_url`、`parsed_config`、`clash_config` 三者可能冲突。优先级不是简单覆盖：

1. 分别 parse 为候选 IR；
2. 比较 endpoint/protocol/credential fingerprint/transport/TLS；
3. 一致则合并展示元数据；
4. 不一致则用最近明确编辑来源（若有可靠 timestamp/action evidence），否则标 conflict，默认 disabled；
5. 用户在计划页选择候选，选择本身进入 audit。

妙妙屋的 relay/chain 映射为显式出站/路由或 profile group，不在 NodeIr 塞模糊数组。转换器检查循环、自引用、缺失节点和跨用户引用。

旧 subscribe file 的节点、标签、规则、脚本、模板和用户授权拆为 typed joins。发布预演以语义比较：节点协议/endpoint/TLS/transport、group/rule 顺序、DNS；不要求 YAML key 顺序/注释逐字相同。credential 差异只比较 keyed fingerprint，不写报告。

## 7. 密码、TOTP、token 和 URL secret

- 识别已知旧密码 hash prefix/参数；无法识别的账号进入 `password_reset_required`。兼容 verifier 仅用于一次登录，成功后 Argon2id rehash 并删除 legacy hash。
- TOTP secret 若旧库明文可解析，立即 AEAD；恢复码若旧版不可证明为 hash，全部废弃并要求 owner/user 重生。
- UI session/tester/Agent/MCP/federation token 不迁移。
- 订阅 token 为减少用户断流可选择保留：导入时只 hash；plan 明确它曾存在旧快照，建议 cutover 后滚动轮换。
- 外部订阅、DNS provider、Telegram webhook 等 URL/query/header 中 credential 先 parse/抽取到 `secret_records`；显示 URL 使用 redacted serialization。

## 8. 流量和账本迁移

社区版 `traffic_records` 是日聚合快照，不能重建逐用户/逐节点/上传下载原始事实。导入策略：

- 对每个旧日记录创建 `legacy_aggregate` baseline，保留 total/used/remaining 和原始 row hash；
- 若 `remaining != total-used`，不修数，标 inconsistency 并记录三者；目标投影按管理员确认的口径；
- 切换时以最终旧 snapshot 创建新 epoch opening baseline；新采集从零 delta 开始，图表分隔 epoch；
- 不把测速流量、订阅 source header 或 probe monthly total 混为用户账单；
- 任何人工校正用 append-only adjustment/reversal，带 reason/import run。

X 若提供 per-user/per-node ledger adapter，则验证非负、单调、reset epoch、重复 sample、单位和倍率，先入 raw measurement，再生成 ledger。没有 source evidence 的字段不推断。

## 9. 冲突与 merge 策略

目标默认要求空实例。需要 merge 时按对象显式策略：

| 对象 | 默认 |
|---|---|
| username/email | target wins，source 改名 draft，绝不把权限合并 |
| node/source/template/rule 名 | 允许重名但追加 origin badge；stable ID 不合并 |
| endpoint/protocol duplicate | 提示 exact fingerprint，可人工 merge；credential 不同不合并 |
| short/token path | target wins；source token 重新生成并输出客户端迁移清单 |
| settings/brand/security | target wins，source 值逐字段选择 |
| traffic/audit | 不 merge epoch/hash chain；并列 import origin |
| Agent/server | 创建 offline draft；必须重新 enrollment 后才执行 |

merge plan 是 immutable JSON + human report；确认后 body hash 绑定 import job，避免确认内容和执行内容不同。

## 10. 验证与对账

### 10.1 结构验证

- 旧 SQLite `quick_check/integrity_check`、每表 count、null/enum/time/JSON/foreign orphan inventory。
- 新库所有 FK/CHECK/unique、repository contract、secret decrypt、object hash、migration mapping 一一对应。
- 导入数满足 `source = imported + deliberately_skipped + quarantined`，无未解释差额。

### 10.2 行为验证

- owner/admin/member 登录与权限；旧密码兼容→rehash。
- 每种节点/protocol/transport parse→IR→目标 encoder golden；链/relay 无环。
- 每个 profile 按授权用户生成，比较节点集合、协议和策略；token revoke/expiry。
- 外部 source dry fetch（默认不联网可用 fixture；用户确认后实际同步）和 SSRF policy。
- traffic baseline/ledger 总和与每个 anomaly 报告。
- probe projection snapshot 不含私有字段；通知 config 不实际发消息直到启用。
- server drafts 不下发；重新 enrollment 后 compile→deploy smoke。

### 10.3 数量对账报告

报告固定包含 26 表行数、用户/角色/disabled、session/token 处理、节点按协议/有效/冲突、订阅/授权、source/provider/template/rule/script、probe/tester/speed、traffic日期/总和、security/audit/jobs、文件、secret 类型、quarantine 和所有 ID mapping 统计。

## 11. 切换、停机和回滚

推荐 blue/green：旧服务 A 继续对外，新服务 B 用旧快照 rehearsal。切换窗口：

1. 宣布维护并阻止旧管理写入；订阅下载可短暂继续 last-good。
2. SQLite checkpoint+backup，确认 source hash；最终 importer 或增量 adapter。
3. B 运行完整 validation/smoke；生成 rollback bundle。
4. reverse proxy 切 B；health/ready/login/subscription/one test user/new audit。
5. A 留在 loopback/read-only，不与 Agent 同时控制；监控关键 SLO。

触发自动/人工回滚：数据库/secret integrity、owner 无法登录、订阅大面积 5xx/空输出、Agent 错误部署、traffic ledger 失真、关键 security control 失败。回滚只切流量和 Agent ownership 回 A，不把 B 的写操作倒灌旧库；切换窗口中新建数据通过导出报告人工处理。

## 12. 原地版本升级

NodeControll migration 使用单调编号、checksum 和 compatibility range。启动默认只检查，不自动做不可逆大迁移；运维执行 `migrate plan`、backup、`migrate apply`。expand/contract：先加 nullable/双读写→backfill/checkpoint→新代码只读新列→后续 release 删除旧列。

SQLite migration 在新数据库文件重建并原子切换，避免长事务损坏唯一副本；PG 使用 transaction/online index/lock timeout 和可暂停 backfill。每个 migration 都有 empty/upgraded/large fixture、两数据库一致性、backup restore 和前一 release binary rollback compatibility 说明。

## 13. 实现包和完成定义

1. `migration-core`：source manifest、safe archive、run state、mapping、diagnostic、plan hash。
2. `mmw-sqlite-detector`：26 表/列/index fingerprint 和 integrity inventory。
3. 逐域 adapters：identity→nodes→profiles/sources→rules/templates/scripts→probe/speed→traffic→audit/settings/files。
4. X fingerprint registry：只在得到真实合法样本后增加 adapter/fixture，不凭文档虚构。
5. verifier/report：结构、行为、semantic diff、machine JSON + Markdown。
6. cutover CLI/runbook：freeze、final snapshot、health gate、switch/rollback。

完成定义：至少三个脱敏社区版 fixture（空、小型、复杂/损坏引用）和一个用户授权的真实副本在 VPS 完成 inspect/import/rehearse；原库 hash 不变；所有行有去向；secret scan 为零；订阅 semantic diff 的解释率 100%；备份、切换和回滚各成功演练一次。X adapter 只有在同样证据和演练满足后才可标“支持”。
