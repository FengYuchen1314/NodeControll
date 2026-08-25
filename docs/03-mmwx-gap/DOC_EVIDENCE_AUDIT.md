# 妙妙屋 X 文档证据审计

> 审计基线：2026-08-25 20:17:42（Asia/Shanghai）。本文件回答“读了哪些、能证明什么、不能证明什么”。页面逐条哈希见 [`evidence/PAGE_INDEX.md`](evidence/PAGE_INDEX.md)，机器可读元数据见 [`evidence/manifest.json`](evidence/manifest.json)。

## 1. 覆盖结论

- Sitemap 中发现的 58 个中文文档页面已全部抓取，覆盖率 `58/58 = 100%`。
- 审阅时曾保存规范化正文与原始 HTML，以便逐页核对；公开仓库不再分发第三方网页全文，只保留源 URL、抓取时间、HTML 字节数、完整 SHA-256、标题树和原创分析。
- 审阅批次原始 HTML 合计 3,784,930 字节。哈希用于标识当时所读版本，不表示站点当前内容仍相同。
- 8 页含字面量 `PRO`，共 30 次：`embedded-xray` 8、`system-settings` 6、`node-ratelimit` 5、`faq-node-management` 3、`share-server` 3、`comparison` 2、`node-speedtest` 2、`install-agent` 1。字面量出现不等于独立 PRO 能力，分类见 [`PRO_FEATURES.md`](PRO_FEATURES.md)。

## 2. 58 页阅读台账

“已提炼”表示页面正文已进入能力目录或差异结论；FAQ/概览中的重复叙述会作为交叉证据，不会重复计成能力。

| 文档域 | 页面（slug） | 数量 | 审计状态 |
|---|---|---:|---|
| 产品定位与上手 | `index`、`about`、`features`、`comparison`、`quick-start`、`tutorial` | 6 | 已提炼 |
| 部署、升级、备份与迁移 | `system-requirements`、`install-direct`、`install-docker`、`install-agent`、`install-external-probe`、`cloudflare-tunnel`、`update`、`backup-restore`、`upgrade-from-mmw`、`changelog` | 10 | 已提炼 |
| Master/Agent、服务器与内核 | `remote-servers`、`xray-service`、`xray-inbounds`、`xray-outbounds`、`xray-routing`、`xray-system-config`、`nodes`、`website-management` | 8 | 已提炼 |
| 协议 | `protocol-matrix`、`protocol-vless`、`protocol-vmess`、`protocol-trojan`、`protocol-shadowsocks`、`protocol-hysteria2`、`protocol-anytls`、`protocol-snell` | 8 | 已提炼 |
| 用户、套餐、订阅与高级能力 | `users`、`packages`、`traffic-accounting`、`certificates`、`generator`、`subscribe-files`、`templates`、`custom-rules`、`routed-outbound`、`share-server`、`embedded-xray`、`node-ratelimit`、`node-speedtest`、`system-settings` | 14 | 已提炼 |
| 外部接口与集成 | `probe-api`、`tool-cloudflare-turnstile`、`tool-mmwx-tgbot`、`mcp` | 4 | 已提炼 |
| FAQ 交叉核验 | `faq`、`faq-carpool`、`faq-common-ops`、`faq-install-deploy`、`faq-node-management`、`faq-protocol-inbound`、`faq-server-management`、`faq-sub-client` | 8 | 已交叉核验 |
| **总计** |  | **58** | **58/58** |

## 3. 证据等级

| 等级 | 能证明什么 | 不能证明什么 |
|---|---|---|
| `X-DOC` | 官方站点在抓取时公开宣称的 UI、行为、配置和限制 | 闭源程序确实按文档实现、没有隐藏限制或缺陷 |
| `X-ARTIFACT` | 公开 `miaomiaowuX` 仓库中的安装脚本/规则/README 内容 | Master、Agent 和 PRO 内核的源码实现 |
| `MMW-SOURCE` | 社区版妙妙屋锁定提交中的真实代码、路由、表和前端行为 | X 闭源分支如何复用或改写该代码 |
| `TARGET` | NodeControll 的规划要求和最终自动化验收约束 | 尚未实现阶段的完成状态 |

本目录关于 X 的结论默认是 `X-DOC`。公开仓库 `iluobei/miaomiaowuX@074de299588d7077d4ba62aeabecd503de5baed8` 只有安装/规则/README 等 14 个文件；`iluobei/mmw-agent` 在审计日由 GitHub API 返回 404。因此不能输出“X 每个函数的源码分析”，也不会把文档行为伪装为源码事实。

## 4. 文档内部矛盾与不确定项

| 编号 | 现象 | 处理方式 |
|---|---|---|
| AUD-001 | `system-requirements` 宣称 amd64/arm64、最低约 128 MB；`faq-install-deploy` 又写 x86/amd64、最低约 512 MB。 | 记为文档冲突；目标同时构建 linux/amd64 与 linux/arm64，按实际压测给出资源下限。 |
| AUD-002 | 协议矩阵标题/叙述称 19 种，表格实际可数到 21 行组合。 | 不采用宣传数字，以逐组合验收用例为准。 |
| AUD-003 | `templates` 的部分说明保留未解析 i18n key。 | 只采纳正文中可读字段与示例，并用妙妙屋源码交叉验证模板语义。 |
| AUD-004 | X 文档把节点测速列为 PRO，但社区版妙妙屋源码已实现主控本机 Mihomo 测速、远程 tester 与历史。 | 判为“社区已有、X 重新包装/扩展并设授权门槛”，不是纯新增能力。 |
| AUD-005 | 文档称 Agent 本身不校验签名，但 Docker 内嵌内核切换又要求 PRO，且实现源码不可见。 | 只记录外部可见授权行为；目标完全移除许可证、机器 ID、额度签名和授权域名依赖。 |
| AUD-006 | X 的 AnyTLS/Snell 来自 Xray fork/cherry-pick，而用户要求目标使用标准 sing-box。 | 官方核验后确认 sing-box 1.12+ 原生 AnyTLS 入站，1.14 新增 Snell v5/v6 入站；目标直接使用官方 schema，但 Snell 必须做稳定版/预发布版门控，XHTTP 仍无同名等价传输。 |
| AUD-007 | `device_limit` 在节点限速文档中实际代表“并发连接数”，并非稳定设备身份。 | 目标数据模型拆成 `max_connections` 与真正的设备/IP 会话策略，避免名称误导。 |
| AUD-008 | 流量存在系统网卡、Xray、用户、套餐、外部订阅等多套口径。 | 目标保存原始计数、来源和派生账本；任何页面都必须标注口径，不直接覆盖累计值。 |

## 5. 可复核方式

1. 从 [`evidence/PAGE_INDEX.md`](evidence/PAGE_INDEX.md) 选择页面并打开官方源 URL。
2. 用 [`evidence/manifest.json`](evidence/manifest.json) 的抓取时间、完整 SHA-256 与标题树识别本次审阅基线；公开仓库不提供网页镜像。
3. 从 [`X_FEATURE_CATALOG.md`](X_FEATURE_CATALOG.md) 的证据列回到官方页面，再从 [`DIFFERENCE_MATRIX.md`](DIFFERENCE_MATRIX.md) 检查社区版与目标验收映射。
4. 若官网更新，另建带时间戳的元数据批次并明确差异，不覆盖旧哈希。
