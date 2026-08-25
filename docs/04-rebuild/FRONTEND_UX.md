# Vue 3 + Vuetify SaaS 前端与交互设计

## 1. 产品体验原则

界面采用标准 SaaS 管理台：固定应用壳、清晰层级、可搜索数据表、右侧详情抽屉、可复核的多步高危操作、任务中心和统一空/错/加载态。风格克制，不复制妙妙屋的页面结构，也不使用需要官方品牌/授权的素材。

五条硬约束：

1. desired、reported、effective 三种状态不能混成一个“已启用”开关；
2. 远端操作先返回 job，UI 持续展示进度，刷新/换页后仍可恢复；
3. 不支持、降级、等待部署和执行失败具有不同图标、文案和颜色；
4. secret 创建时只回显一次，普通详情永远只显示“已配置/最后轮换”；
5. 所有桌面能力在 360px 宽度仍可完成，表格在移动端切卡片/关键列，而不是横向塞满。

## 2. 技术边界

- Vue 3 Composition API + TypeScript strict；单文件组件使用 `<script setup>`。
- Vuetify 提供 tokens、布局、组件和无障碍基础；自定义 CSS 使用 design token，不散落 magic color/spacing。
- Vue Router 使用路由级动态 import、typed route metadata 和守卫。
- TanStack Query 管 server state、cache、mutation、retry；Pinia 只管 session shell、local preferences、wizard drafts 等 client state。
- OpenAPI 自动生成 typed client；组件不能手写 API path/response interface。
- VeeValidate + schema adapter（最终固定实现）负责表单，API field error 通过 JSON pointer 映射。
- vue-i18n；首发简体中文与英文，所有可见文本、aria label、通知和日期格式均走 i18n。
- Vitest + Testing Library 做行为测试；Playwright 做关键 E2E；Storybook（或轻量等价 gallery）维护共享组件状态。

依赖具体版本在 P5 由 VPS 查询官方 registry、锁定 lockfile 和镜像 digest；文档不写漂移的 `latest`。

## 3. 应用壳与全局导航

桌面布局：左侧 256px navigation drawer、顶部 context bar、主内容 max-width/全宽按页面选择、右侧可选 inspector drawer。移动端左导航变 modal drawer，context/action 收入 sticky bottom action 或 overflow。

导航按 capability 和 scope 动态投影，但直接访问仍由后端授权：

```text
总览
资源
  服务器
  入站与节点
  出站与路由
  隧道与 WARP
用户与策略
  用户
  套餐
  流量与连接
订阅
  订阅文件
  外部来源
  Proxy Provider
  模板、规则与脚本
运维
  证书
  网站
  测速
  公开探针
  任务中心
  日志与审计
集成
  通知与 Telegram
  MCP
  实例联合
设置
```

顶部 context bar 提供全局搜索/command palette、当前服务器/用户上下文、health badge、任务队列、通知和个人菜单。任何 health badge 可打开原因，不用颜色作为唯一信息。

## 4. 路由与页面清单

### 4.1 认证与个人区域

| Route | 页面 | 核心行为 |
|---|---|---|
| `/setup` | 首次初始化 | 实例、owner、恢复码下载/确认；完成后永久关闭 |
| `/login` | 登录 | 密码→MFA/WebAuthn challenge；防枚举通用错误 |
| `/recover` | 恢复码/一次性密码 | 不展示账号是否存在 |
| `/profile` | 资料 | 昵称、语言、时区、重新认证 |
| `/profile/security` | 安全 | TOTP/WebAuthn、sessions、tokens、恢复码 |
| `/me/subscription` | 用户订阅中心 | 套餐、流量、到期、设备、客户端下载和二维码 |
| `/me/connections` | 我的连接 | 当前连接和允许的关闭动作 |

### 4.2 总览与状态

`/dashboard` 包含时间范围、总流量/账单流量、服务器 health、在线 Agent、内核 drift、用户/节点、失败 jobs、证书到期、连接、最近审计和待处理告警。每个卡片有数据来源和更新时间；无权限卡片不渲染；无数据与采集失败分开。

`/activity` 是跨资源事件流，可按 type/severity/actor/resource 过滤并深链到详情。

### 4.3 服务器工作台

| Route | 内容 |
|---|---|
| `/servers` | 服务器表/卡：online、mode、OS、Agent/core version、health、drift、流量、标签 |
| `/servers/new` | 创建 + Agent 四种连接模式引导 + 一次性 enrollment |
| `/servers/:id/overview` | CPU/内存/磁盘/网络、公网 IP、时间、服务状态、capability 和近期操作 |
| `/servers/:id/agent` | 连接、证书、heartbeat、capability、outbox、轮换和诊断 |
| `/servers/:id/core` | sing-box channel/version/build tags、install/upgrade/rollback、license/source |
| `/servers/:id/inbounds` | 入站、端口/TLS/transport、principal、有效连接、配置状态 |
| `/servers/:id/outbounds` | 出站、selector/urltest、WARP 和 health |
| `/servers/:id/routes` | first-match 可排序路由、rule-set、shadow/unreachable diagnostics |
| `/servers/:id/config` | desired/reported/last-good diff、compile、deploy、rollback timeline |
| `/servers/:id/sites` | Nginx 站点、证书、upstream、validate/deploy |
| `/servers/:id/metrics` | 系统/流量/连接图与原始采集健康 |
| `/servers/:id/logs` | Agent/core/Nginx 可筛选流式日志、下载脱敏 bundle |

服务器标题区固定显示 `Online/Offline/Degraded`、`In sync/Pending/Drift`、最近心跳和当前 job。start/stop/restart/reload 不做瞬时乐观成功；按钮转 job chip。

### 4.4 节点、出站、路由和隧道

| Route | 内容 |
|---|---|
| `/nodes` | 跨服务器节点表、tags、发布状态、测速、用户可见性、批量动作 |
| `/nodes/new`、`/nodes/:id` | protocol/transport/TLS/Reality 分步配置与实时 capability 诊断 |
| `/outbounds` | 跨服务器出站、选择器、健康和路由引用 |
| `/routes` | 服务器过滤后的路由规则、排序、命中/阴影解释器 |
| `/tunnels` | 两端拓扑、状态、回环/MTU/capability 测试 |
| `/warp` | WARP profiles、key 状态、endpoint、refresh 和引用 |
| `/user-routes` | 私有节点/出站、用户绑定、独立配额、到期 |

协议编辑器由 schema registry 驱动。第一步选择目标服务器和协议后，后续字段由 server capability + core version 决定；已存在但新版不支持的字段进入只读迁移态，不能悄悄丢弃。

### 4.5 用户、套餐、流量和连接

| Route | 内容 |
|---|---|
| `/users` | 用户、状态、多套餐、周期、用量、连接、最后活动与批量操作 |
| `/users/:id/overview` | identity、effective policy、用量、设备、订阅和事件 |
| `/users/:id/access` | session/token/MFA/订阅 token；secret 安全动作 |
| `/users/:id/traffic` | raw/billed/baseline/adjustment 分层图与 ledger |
| `/users/:id/routes` | 私有路由和配额 |
| `/packages` | 套餐卡/表、克隆、排序、使用人数和冲突提示 |
| `/packages/:id` | 周期、流量、节点、协议、设备、速率、并发、IP 策略 |
| `/traffic` | 实例/服务器/用户/节点 series 和账本 drilldown |
| `/connections` | live/history；按权限展示/脱敏源 IP，批量关闭需 reason |

`EffectivePolicyPanel` 展示最终值、贡献来源和冲突规则。例如速率显示 `10 Mbps ← 套餐 A(20) ∩ 用户覆盖(10)`，不能只显示结果。

### 4.6 订阅工作台

| Route | 内容 |
|---|---|
| `/subscriptions` | profile、格式、用户/套餐、最近 publish、artifact 和下载健康 |
| `/subscriptions/:id` | 输入→选择→模板→输出 pipeline、版本、token、预览与诊断 |
| `/sources` | URL、格式、schedule、ETag、last sync、diff/error |
| `/sources/:id` | 安全 URL、runs、items、staged preview、手动 sync |
| `/providers` | proxy-provider、filters、输出格式和 token |
| `/templates` | 内置/自定义、fork、版本、编辑、lint、render diff |
| `/rules` | rule libraries、remote sync、引用和版本 |
| `/scripts` | WASM module、权限、hash、测试 fixture、执行指标 |
| `/generator` | 不保存的输入/目标/模板预览，展示降级 diagnostics |
| `/client-capabilities` | 客户端/版本×协议/字段矩阵 |

订阅详情使用可视化 pipeline stepper；每步都能查看项目数、变化、warning/fatal 和 immutable input hash。预览默认遮蔽 credential，只有重新认证且有 scope 才能一次性下载完整 artifact。

### 4.7 证书、站点、测速和公开探针

| Route | 内容 |
|---|---|
| `/certificates` | 域名、issuer/challenge、服务器、到期、自动续期和事件 |
| `/certificates/:id` | DNS provider secret ref、issue/renew/deploy timeline |
| `/sites` | server/site/domain/upstream/cert/status 列表 |
| `/sites/:id` | 结构化 Nginx 配置、generated diff、validate/deploy/rollback |
| `/speed-tests` | 节点/服务器/公开测试 runs、样本、地域/运营商比较 |
| `/speed-targets` | 目标、配额、地区、健康和维护 |
| `/probe` | 公开 projection 设置、预览、tester 配对、限速和 cache |

测速结果明确显示测试点、时间、协议、并发、样本量、失败率和单位；不把单次峰值包装成稳定带宽。

### 4.8 运维、集成和设置

| Route | 内容 |
|---|---|
| `/jobs`、`/jobs/:id` | durable jobs、step、attempt、Agent task、retry/cancel |
| `/logs` | Master 聚合日志，secret redaction，server/source filter |
| `/audit` | actor/action/resource/before-after 摘要和签名导出 |
| `/backups` | 备份、manifest、加密目标、inspect/restore rehearsal |
| `/notifications` | rules、channels、quiet hours、deliveries |
| `/integrations/telegram` | Bot/webhook/Mini App/命令/scope 配置 |
| `/integrations/mcp` | clients、26+ tools、scope、confirm policy、invocations |
| `/federation` | peers、trust、shares、imports、revoke/rotate |
| `/settings/general` | 名称、URL、locale、branding |
| `/settings/security` | password/session/MFA/CORS/trusted proxy/rate limits |
| `/settings/storage` | SQLite/PG 信息、object store、retention（不可显示 secret） |
| `/settings/system` | versions、compatibility、license/source、diagnostics |

## 5. 共享组件和模式

| 组件 | 合同 |
|---|---|
| `AppDataTable` | server pagination/filter/sort、URL 同步、列偏好、mobile card、empty/error skeleton |
| `ResourceHeader` | breadcrumb、状态、revision、主要动作、overflow；不重复页面标题 |
| `StatusChip` | icon+文字+颜色，tooltip 说明判断来源/时间 |
| `JobChip/JobDrawer` | 从 mutation 202 接管，SSE 更新、重连、retry/cancel、深链 |
| `DesiredReportedDiff` | desired/reported/last-good 三列 semantic + raw diff、secret redacted |
| `CapabilityGuard` | scope + capability + state 的 disabled reason；仅 UX，不替代后端 |
| `SecretField` | create/update clear semantics、clipboard timeout、一次性 reveal banner |
| `DangerDialog` | 影响摘要、依赖、typed confirmation、reason、re-auth、job outcome |
| `ProtocolEditor` | discriminator schema、跨字段 validation、server capability diagnostic |
| `PolicyExplainer` | effective value、来源、优先级和时间范围 |
| `MetricChart` | timezone、unit、null/gap、sampling、accessible table alternative |
| `AuditDiff` | JSON pointer-level before/after，敏感字段恒为 redacted |
| `QrDialog` | 屏幕防旁观提示、过期、复制、下载与禁止 analytics payload |

Toast 只用于操作接收/轻量完成；失败的持久任务必须在 job center 和对象页保留，不能仅弹 4 秒消失。不可逆或影响在线流量的操作必须先展示 impact plan。

## 6. Server state、缓存和实时更新

- Query key 由 generated API operation + canonical args 构成；列表 mutation 不手工猜 cache，优先 invalidation 或使用响应资源 revision 精确更新。
- SSE event 只触发 invalidation/小型状态 patch，不能被当作完整对象；按 event ID 去重。
- route loader 预取首屏关键 query；切换 route 时取消不再使用请求。
- offline 不允许提交远端动作；已有页面进入 stale banner，显示最后成功时间。
- mutation 统一状态：`idle → submitting → accepted(job) → terminal`。刷新后 job store 从 API 恢复，不将运行态只存在内存。
- 409 revision conflict 打开三方视图：我加载的版本、服务端现值、我的改动；用户选择 reload 或重应用，不自动覆盖。

## 7. 表单与配置编辑

简单资源用单页表单；跨资源或危险操作用 stepper：范围→配置→验证/影响→确认→job。Autosave 只用于无副作用的草稿，不用于 desired state 或 secret。

协议字段 schema 同时包含：label/help、type/default、required/conditional、sensitive、server capability predicate、client export capability、JSON pointer 和 migration hint。前端初验提升体验，后端编译/校验是最终事实源。

原始 JSON/YAML 高级编辑器只是结构化编辑器的另一视图：切换时 round-trip、展示字段丢失预警；保存前必须 compile。绝不提供任意 Nginx/shell 模板直通 Agent；Nginx 高级片段使用受限 grammar/allowlist。

## 8. SaaS 视觉系统

基础 token：4px spacing grid；内容间距 16/24/32；圆角 8（control）/12（card/dialog）；边框优先于重阴影；正文最小 14px；monospace 只用于 ID/config/log。支持 light/dark/system 和高对比。

语义颜色限定 `info/success/warning/error/neutral`，状态另带 icon/text。图表使用可区分色板、pattern/marker 和表格替代。品牌可配置 logo、primary 色和登录背景，但对比度不合格时拒绝保存或自动选安全 on-color。

dashboard 卡片可自定义顺序/隐藏，但关键 security warning 不允许永久隐藏；保存为用户偏好，不改变全局数据。

## 9. 响应式与无障碍

- 目标 WCAG 2.2 AA；键盘可完成所有操作，焦点可见，dialog 正确 trap/restore。
- landmarks、唯一 H1、语义 table/list/form；icon button 必须 aria-label；实时 job 使用克制的 `aria-live`。
- 颜色对比、200% zoom、reduced motion、屏幕阅读器（NVDA/VoiceOver 至少一种桌面+移动组合）进入发布门。
- 拖拽排序有键盘上移/下移和序号编辑替代；拓扑图始终有可操作列表替代。
- 图表 tooltip 可键盘访问，时间序列可下载/展开为表格。
- 数据表移动端优先显示 identity/status/primary metric/actions；其余进入详情，禁止仅靠 hover。

## 10. 性能预算

- 首次匿名/login JS gzip ≤ 180 KiB；认证后 shell 首屏总 JS gzip ≤ 350 KiB，不含按需 Monaco/chart chunks。
- LCP p75 ≤ 2.5s、INP p75 ≤ 200ms、CLS ≤ 0.1（典型 4G/中端设备基准）。
- 路由 chunk 独立；Monaco、地图/拓扑、大型图表仅进入相应页面加载。
- 大表 server pagination；超过 200 行不在 DOM 全量渲染。日志采用窗口化并暂停自动滚动。
- bundle analyzer 在 VPS CI 执行，超过 budget 失败并产出 artifact。

## 11. 测试层次与验收

1. 纯函数：format、policy explanation、capability、route/query normalization。
2. component：所有共享组件的 loading/empty/error/permission/mobile/keyboard 状态。
3. mock API integration：每个 route 成功与关键失败；409、422、429、Agent offline、job retry。
4. Playwright：setup/login/MFA、Agent enrollment、节点→compile→deploy、用户套餐→订阅、traffic limit、证书/站点、备份 inspect、MCP dangerous confirm、federation revoke。
5. visual regression：light/dark、中文/英文、desktop/mobile、关键协议表单和 dashboard。
6. axe + keyboard + screen reader manual checklist；Lighthouse/INP/bundle budget。

页面完成定义：路由可深链/刷新；scope/capability/错误可解释；请求可取消；空/加载/失败/部分降级完整；移动端和键盘可完成；i18n 无漏项；行为与 E2E 通过；文档/截图和需求追踪已更新。
