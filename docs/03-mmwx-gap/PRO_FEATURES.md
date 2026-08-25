# 妙妙屋 X 原 PRO 能力与去授权要求

> 目标原则：下列能力全部作为 NodeControll 普通自托管功能交付。不得出现许可证密钥、机器 ID 激活、官方签名、官方授权域名、按许可证限制服务器/节点/用户数，拥有方与消费方也不需要向任何第三方取授权。

## 1. 经文档明确确认的 PRO 能力

| ID | 原 PRO 能力 | X 文档行为 | 社区版现状 | NodeControll 必须实现 |
|---|---|---|---|---|
| PRO-001 | Agent 内嵌代理内核 | Xray 以库形式运行在 Agent 进程；主控可切换内嵌/外置模式，运行时注入调度器和统计。 | 无 Agent/内嵌服务端内核。 | Agent 自带受版本管理的标准 sing-box；支持托管模式和导入既有 sing-box 配置的外置模式，均无授权检查。 |
| PRO-002 | 实时用户/节点限速 | 限速规则经 WebSocket 推到内嵌 Xray；持续与突发桶控制下载速度。 | 无服务端数据面限速。 | 控制面策略版本化下发；Agent 在可验证的数据面执行，报告实际生效版本、拒绝原因和计量指标。 |
| PRO-003 | XTLS Vision 限速钩子 | X 的自定义 Xray fork 对 Vision 流量插入限速。 | 无。 | 不复刻私有 Xray hook；用 sing-box 可支持的连接/用户策略实现同等用户结果，并以真实吞吐测试验收。 |
| PRO-004 | 自动限速/解除 | 用户接近或超过套餐阈值时自动降速，条件解除后恢复。 | 只有流量展示/限额，无内核执行。 | 阈值、窗口、迟滞、恢复条件、人工覆盖均显式建模；任务幂等且可审计。 |
| PRO-005 | 在线用户、IP 与连接追踪 | 内嵌内核按入站/用户维护在线、活动 IP 和连接数。 | 探针不是用户连接追踪。 | Agent 上报匿名化/可配置保留期的会话聚合；页面可追到用户、节点、服务器和时间窗口。 |
| PRO-006 | 最大并发连接限制 | 文档字段叫“客户端数/设备数”，实际按活动连接数限制。 | 无。 | 命名为 `max_connections`；并另设可选的设备/IP 策略，防止语义混淆。 |
| PRO-007 | 节点测速工作台 | 主控自动取得 Mihomo，支持单/8 线程、延迟、出口 IP、批量、异步进度和历史；远程家用 tester 反连。 | 已有本地 Mihomo、远程 tester、历史等核心能力，但 X UI/配对流程更完整。 | 保留并扩展为普通功能；测速执行器可插拔，配对 token 一次显示、任务串行/限并发、结果留痕。 |
| PRO-008 | 分享服务器 | 拥有方发分享 token，消费方无需 Agent 即可创建带前缀的入站/节点；不能服务控制、改配置或二次分享。双方都被要求持证。 | 无跨实例服务器共享。 | 建立实例间最小权限联合协议；token 只存 hash、可撤销，细粒度 scope/配额/审计，禁止二次转授；双方完全自托管。 |
| PRO-009 | 自定义品牌 | 登录壁纸、品牌等需要对应 PRO feature。 | 有主题/字体，没有完整租户品牌。 | 普通管理员可配置名称、logo、favicon、主题色、登录背景和外部链接；资产本地存储，提供恢复默认。 |
| PRO-010 | 内嵌 Agent Docker 开关 | Agent 文档称 Docker 内嵌模式需开启 PRO；Agent 本身又称不做签名校验。 | 无。 | 统一开源配置，不存在激活开关；镜像启动即能选择 managed/external 模式。 |

证据：[`embedded-xray`](https://miaomiaowux.com/docs/embedded-xray)、[`node-ratelimit`](https://miaomiaowux.com/docs/node-ratelimit)、[`node-speedtest`](https://miaomiaowux.com/docs/node-speedtest)、[`share-server`](https://miaomiaowux.com/docs/share-server)、[`system-settings`](https://miaomiaowux.com/docs/system-settings)、[`install-agent`](https://miaomiaowux.com/docs/install-agent)。

## 2. 含 PRO 字样但不是新增授权能力的内容

| 页面 | 原因 | 结论 |
|---|---|---|
| `comparison` | 对内嵌 Xray 和 PRO 版本做总览。 | 归并到 PRO-001～006，不重复计数。 |
| `faq-node-management` | UI 截图/按钮旁提到 PRO 路由或功能。 | 能力按路由出站、测速、共享等实际章节计数。 |
| `node-ratelimit` 演示表中的“香港 PRO”等 | 示例节点/套餐名称。 | 不是授权机制。 |
| `probe-api` 的 `license_badge` | 公开探针可展示许可证铭牌。 | 目标删除许可证含义；可用普通 `instance_badge` 展示自定义实例标签。 |

## 3. 必须从目标中删除的授权耦合

| ID | 禁止项 | 目标验收 |
|---|---|---|
| NOLIC-001 | `mmwxlicense.com` 或其他官方激活服务 | 全仓库静态扫描不得出现运行时依赖；断网安装后全部功能可用。 |
| NOLIC-002 | 机器 ID 申请、绑定或硬件指纹 | 不生成授权用途硬件指纹；节点身份只用于安全配对且可轮换。 |
| NOLIC-003 | 签名 feature flags/许可证套餐 | 功能开关均由本地管理员控制，数据库迁移不会因许可证降级。 |
| NOLIC-004 | 许可证给出的服务器、节点、用户额度 | 仅管理员自定资源配额与系统容量保护，不接受第三方额度。 |
| NOLIC-005 | 官方域名、订阅域名库或远程配置为必要条件 | 默认规则、模板、Geo 资源可选择任意 URL或本地文件；离线导入可完成初始化。 |
| NOLIC-006 | 分享双方必须持证 | 联合协议只校验双方自签/管理员信任的实例身份和授权 scope。 |
| NOLIC-007 | License UI、到期降级、license badge | UI 不呈现许可证页；升级后不因任何外部状态停止功能。 |

## 4. 安全边界不是“许可证”的替代品

去授权不等于取消安全控制。Agent 配对、跨实例分享、MCP 高危写操作、证书私钥、订阅 token 仍必须使用本地可审计的身份、scope、密钥轮换、确认与撤销机制。资源上限也仍需存在，但用途只能是管理员套餐、滥用防护或容量保护，并且完全由本实例决定。
