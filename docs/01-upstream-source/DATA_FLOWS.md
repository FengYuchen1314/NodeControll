# 妙妙屋关键数据流

> 基线：`iluobei/miaomiaowu@0b47f10c52aee10b9f759a593ca5f61a823cbb72`。本章把前端、HTTP Handler、SQLite、文件系统、内存状态与外部服务串成端到端链路；逐函数实现见 Go/TypeScript 自动索引。

## 1. 首次初始化与登录

### 1.1 首次初始化

```text
LoginPage
  → GET /api/setup/status
  → users 是否为空
  ├─ 否：显示普通登录
  └─ 是：POST /api/setup/init 或 POST /api/setup/restore-backup
          → 再次检查 users 为空
          → bcrypt 密码 / 创建首个 admin
          → 返回资料，前端进入登录
```

关键一致性边界是 Handler 内的第二次“用户为空”检查；UI 的状态查询只是显示逻辑。恢复备份是首次安装的替代分支，它会直接引入数据库和受管文件，因此重构必须在恢复前验证格式、版本、路径与完整性。

### 1.2 密码 + 可选 2FA 登录

1. `LoginView` 读取 Turnstile 公共配置并提交用户名、密码、remember-me 和验证码 token。
2. `NewLoginHandler` 以 `CF-Connecting-IP → X-Forwarded-For[0] → X-Real-IP → RemoteAddr` 的优先级提取客户端 IP。
3. 若 Turnstile 启用，服务端向 Cloudflare 验证；随后登录限速器按 IP + username 检查失败窗口。
4. `auth.Manager` 从 `users` 读取 bcrypt hash 并校验密码；失败会更新进程内限速状态和安全日志。
5. 若用户启用 TOTP，`TwoFactorPendingStore` 签发五分钟中间 token，前端切换到验证码/恢复码步骤。
6. TOTP 成功后消费中间 token；恢复码成功后还会关闭当前 TOTP，迫使用户重新设置。
7. `issueLoginSession` 向内存 `TokenStore` 签发 UI token，并写入 `sessions` 以便重启恢复；登录前还确保 `user_tokens` 中存在长期订阅 token。
8. 前端把 UI token 存进可读 Cookie，Axios 后续写入 `MM-Authorization`。

失败与副作用：登录失败不产生会话；会话持久化失败与 token 内存签发的先后关系需要通过实现测试确认。更新管理员主凭据会 `RevokeAll`，但普通用户改密码后的会话撤销语义应在重构中明确。

## 2. 节点导入、创建与 YAML 同步

### 2.1 预解析

节点可以来自手输字段、代理 URI/V2Ray base64 或外部订阅：

```text
NodesPage
  ├─ POST /api/admin/nodes/parse-uris
  │    → proxyparser / 宽容 base64 解码
  │    → 规范化协议、server、port、name、raw_url、config
  └─ POST /api/admin/nodes/fetch-subscription
       → SSRF 安全拉取
       → 解码 YAML/base64/URI
       → 返回候选节点，不立即落库
```

前端在桌面或移动编辑器中补协议字段、标签、启停、证书跳过、链式代理和探针绑定，再提交单条或 batch 创建。

### 2.2 持久化与文件副作用

1. `nodesHandler` 校验节点名、协议和必填字段；缺少 `protocol` 返回 400，这也是当前两个上游测试失败的原因。
2. `storage` 写 `nodes`，处理同用户名下重名和缺省 enabled。
3. `YAMLSyncManager` 根据用户设置和订阅文件关联，把新增/更新/删除同步到对应 YAML AST。
4. `yaml_sync.go` 更新 `proxies`、代理组成员、relay/dialer 引用和字段顺序；删除节点时修剪悬空成员。
5. 节点 server 改写会保留可恢复原值；专用 restore 端点恢复。
6. Handler 返回节点记录；前端使 nodes/config 等 query 失效并重新拉取。

SQLite 与 YAML 不是同一事务资源。如果数据库成功而文件写入失败，存在部分提交窗口；新系统需要把“规范化节点真相”放在数据库，配置文件作为可重建发布产物，并用 outbox/job 状态追踪生成。

## 3. 外部订阅同步

### 3.1 配置

`external_subscriptions` 保存 URL、名称过滤/排除、自动更新间隔、是否同步流量、节点命名前缀、目标订阅文件和最近同步状态。用户可 CRUD；管理员可以触发批量、单条或需要确认的同步。

### 3.2 同步流水线

```text
manual / scheduled trigger
  → 找出当前用户实际引用的外部订阅
  → HTTP GET 远端内容
  → 读取 Subscription-Userinfo 流量头
  → 解析 YAML/base64/URI 为代理候选
  → include/exclude 正则过滤 + 名称前缀
  → 与已有节点/目标文件做匹配
  ├─ 直接模式：新增、更新、删除/保留并写 YAML
  └─ 选择模式：候选存进内存 selection session
                 → 返回 selection_id
                 → POST confirm 提交选择
                 → 应用所选候选
  → 更新 last_sync/last_error/流量/过期时间
  → 失效订阅内容和 provider 缓存
```

`storeExternalSyncSelection` 的确认会话位于进程内；服务重启或请求落到另一实例会丢失。定时器每分钟扫描到期配置，在同一进程并发执行，没有跨实例租约。

### 3.3 流量头

`ParseTrafficInfoHeader` 识别常见的 `upload/download/total/expire`；解析结果更新外部订阅记录。订阅展示和总流量汇总会读取这些字段，因此同步既改变节点，也改变计费/展示口径。

## 4. proxy-provider 发布与缓存

### 4.1 配置与请求

用户把一个外部订阅绑定成一个或多个 `proxy_provider_configs`，配置 include/exclude、GeoIP、字段覆写、输出名称、缓存间隔和 client/MMW 处理模式。外部客户端访问：

```text
GET /api/proxy-provider/{config-id}?token={user-token}
```

`NewProxyProviderServeHandler` 的步骤是：

1. 暴力防护检查 IP/path；
2. 校验方法、config ID、用户长期 token 和 config 所有权；
3. 查 `ProxyProviderCache`；未过期则直接返回；
4. 缓存缺失/过期时读取外部订阅配置并拉取内容；
5. 预处理 base64/URI/YAML，定位 `proxies`；
6. 应用名称 include/exclude、GeoIP 条件、协议/字段覆写；
7. 规范化代理字段顺序并生成 proxy-provider YAML；
8. 把正文、节点预览、生成/过期时间和错误状态写入进程内 cache；
9. 返回内容；失败时更新暴力/缓存失败状态。

### 4.2 后台刷新

启动时 `InitProxyProviderCacheOnStartup` 预热配置。`StartProxyProviderCacheSync` 周期收集到期任务，用有上限的 worker 刷新；连续失败进入退避，成功清零失败状态。它另有短周期的外部订阅 DB cache，减少同源查询。

缓存不持久化，重启会形成集中预热流量；多实例各自拉取同一外部来源。Rust 设计应把发布快照按内容 hash 持久化，并用任务租约、ETag/Last-Modified 与 stale-if-error 控制刷新。

## 5. 最终订阅发布

### 5.1 鉴权入口

最终内容可从 `/api/clash/subscribe`、组合短链或内部转交进入 `SubscriptionHandler`。入口先执行：

- IP 暴力封禁与订阅频率限制；
- 用户长期 token、系统短码/自定义短码和文件短码解析；
- 用户状态与 `user_subscriptions` 授权检查；
- 可选未知 User-Agent 拒绝；
- 静默模式活跃窗口刷新。

无效 token 可返回配置化的伪装内容，而不是始终返回 JSON 401。

### 5.2 内容流水线

```text
resolve user + subscribe_file
  → 如配置则同步被引用的 external subscriptions
  → 选择内容来源
     ├─ 直接读取受管 YAML
     ├─ V3 模板 + 全部/选定标签节点
     └─ 聚合文件/旧 subscription link
  → 合并 MMW proxy providers
  → 运行 post-fetch JavaScript 覆写（5 秒上限）
  → 注入 relay/dialer proxy 组与 legacy 兼容字段
  → 按 node_order 排序 + 去重 + 统一字段顺序
  → 依据 UA/format 转换
     ├─ Clash/Mihomo YAML
     ├─ Surge
     ├─ Loon
     └─ JSON
  → 对旧 Clash 过滤不兼容 Snell v6
  → 添加 Subscription-Userinfo / profile headers
  → 返回配置正文
```

`generateFromTemplate` 和 `generateFromSelectedTags` 是动态生成的两个主分支；`convertSubscription` 再处理目标客户端。流量来自探针与外部订阅聚合，过期时间和剩余量还可作为特殊 info 节点注入内容。

### 5.3 失败策略

- 外部同步、模板、脚本、YAML 解析、代理引用或格式转换任一阶段都可中断请求；
- 部分外部数据失败会回退已有文件/缓存，部分直接报错，策略没有统一类型表达；
- 生成发生在请求路径，复杂配置会把外部延迟和 CPU 直接转嫁给订阅客户端；
- 长期 token 常出现在 query，需避免写入访问日志。

新系统应把编辑态、已验证版本和已发布快照分离：变更后异步生成并通过 sing-box `check`/目标 exporter 校验，客户端 GET 只读取最近成功的不可变快照。

## 6. 模板与规则编辑

### 6.1 V3 文件模板

1. 管理员或普通用户上传模板，文件落到 `rule_templates/`，owner/public 元数据写 SQLite。
2. `RuleTemplatesHandler` 用 admin、owner 和 public 三者判定查看/修改权限。
3. 前端 `template-v3-utils` 把 YAML 中的代理组转成表单状态；用户修改组类型、节点/代理集合占位、正则和顺序。
4. 预览请求把模板正文与选定代理送到 `TemplateV3Handler`；服务端注入 proxies、proxy-groups、rule-providers、relay，并返回生成正文。
5. 用户可把一个模板设为默认，订阅生成时读取。

模板正文与 owner 元数据分处文件系统和数据库，同样存在部分提交风险。

### 6.2 自定义规则与 JS 覆写

自定义规则记录定义要修改的 DNS、rules、rule-providers 片段及追加/替换策略。`apply_custom_rules` 以 YAML AST 合并、去重并补代理组。覆写脚本由 goja 执行，暴露 `post-fetch`、`pre-save-nodes`、console 和 produce；VM 默认五秒超时。启用顺序决定最终内容，应当是发布版本的一部分并进入审计。

## 7. 探针与流量汇总

### 7.1 探针同步

管理员保存 Nezha v1/v0、DStatus 或 Komari 地址/凭据。`ProbeSyncHandler` 按类型走不同 HTTP 或 WebSocket 协议，归一化为 server ID、名称、上下行、上限和在线状态，写入 `probe_servers`。节点与订阅文件通过 ID 数组绑定探针服务器。

### 7.2 流量读取

`TrafficSummaryHandler`：

1. 读取当前用户允许的 probe server IDs；
2. 按探针类型请求总计或批量服务器数据；
3. 将 net-in/net-out 和周期上限归一为 bytes；
4. 叠加当前用户启用 `sync_traffic` 的外部订阅用量；
5. 返回 total/used/remaining，并为每个订阅文件计算其绑定服务器或独立限额；
6. 定时任务每日把快照写入 `traffic_records`，通知任务读取逐服务器数据发送 Telegram。

探针不可用时总计分支可退回外部订阅流量。不同探针的周期、重置和上下行定义并不天然一致，重构数据模型必须保存原始读数、来源时间和归一化规则。

## 8. 测速任务

```text
SpeedtestDialog → POST /api/admin/speedtest/run
  → 读取 node + 生成临时代理配置
  ├─ local：下载/校验 Mihomo → 启临时内核 → 测延迟/下载/出口 IP
  └─ remote：选择在线 tester → WebSocket 下发任务 → tester 回传结果
  → 写 speed_test_results
  → 前端轮询结果并绘制历史
```

远程 tester 创建时只显示一次 token；数据库保存 token hash，在线连接位于进程内 map。吊销或轮换后应关闭旧连接。当前测速依赖 Mihomo，与目标 sing-box 内核不同；重构必须重新定义“内核配置可用性检查”和“实际链路吞吐测速”的边界。

## 9. 备份、恢复与升级

备份先执行 WAL checkpoint，再打包 SQLite 与订阅、规则、模板等目录。恢复接收归档、检查文件路径/内容后覆盖运行数据。上游自更新会查询 GitHub Release、下载对应资产、在更新前备份并替换当前二进制；另有 SSE 输出进度。

风险点：

- 数据库和文件树需要同一恢复点，但没有显式 manifest/hash/schema 版本协议；
- 运行中的 goroutine 和打开的数据库连接会观察到恢复中间态；
- 覆盖二进制不适合容器化部署，也不能原子回滚 schema。

目标系统应生成带 manifest、版本、hash、数据库一致快照和加密选项的备份；恢复采用停写/维护模式、临时目录校验、原子切换和迁移 dry-run。升级只提供镜像/二进制版本检查、兼容性检查和可回滚部署步骤。

## 10. 跨资源一致性清单

| 变化 | 需要同步/失效的状态 |
|---|---|
| 用户禁用/删除 | UI sessions、订阅 token/短码、订阅授权、内存调试 timer、相关缓存。 |
| 订阅 token/短码轮换 | 短链集合、暴力防护已知路径、provider/订阅 URL 展示。 |
| 节点更新/删除 | `nodes`、订阅 YAML、relay/代理组引用、发布快照、测速上下文。 |
| 外部订阅更新 | 外订阅记录、节点/YAML、流量字段、订阅内容 cache、provider cache。 |
| 模板/规则/脚本更新 | 文件/元数据、默认引用、所有受影响发布物、审计。 |
| 探针配置更新 | 运行时 client、server 列表、节点/文件绑定、流量快照与通知。 |
| 系统配置更新 | proxy group store、通知器、UA guard、silent mode、缓存调度器。 |

这些关系目前由 Handler 中的顺序调用维持。Rust 重构的核心不是复制调用顺序，而是把变化建模为事务内领域事件/outbox，再让幂等 worker 完成可重试副作用。
