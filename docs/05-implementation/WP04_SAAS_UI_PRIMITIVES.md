# WP-04 SaaS 共享界面基础片

## 1. 本批范围

本批建立六个与业务 API 解耦的共享组件，以及一个安全展示值组件：

| 组件 | 当前职责 |
|---|---|
| `ResourceHeader` | 页级标题、面包屑、状态、revision、主动作、次级动作与更多操作 |
| `StatusChip` | 以图标、文字和语义颜色显示状态，并暴露判断来源和时间 |
| `DangerDialog` | 显示影响与依赖，要求逐字输入对象名，可选原因，提交期间失败关闭 |
| `SecretField` | 新建或替换秘密值、显式清除、一次性显示提示和瞬时遮蔽 |
| `DesiredReportedDiff` | 并列显示 desired、reported、last-good，区分一致、等待、偏差和未知 |
| `PolicyExplainer` | 显示 effective 值、候选来源、优先级、范围、时间范围与采用结果 |
| `SafeDisplayValueView` | 只渲染 `text`、`empty`、`redacted` 三种受限展示值 |

这些组件没有连接业务页面，也没有请求 API。测试中的对象名和状态仅为组件输入 fixture，不会出现在运行时页面。组件不导入 generated client，不声明请求或响应 DTO，不包含付费、品牌授权或官方数据依赖。

## 2. 文件与接口

### 2.1 公共类型

`apps/web/src/components/saas/types.ts` 是本批唯一的 TypeScript 展示合同：

- `ResourceBreadcrumb` 接受 Vue Router 的 `RouteLocationRaw`，不复制路由字符串协议；
- `StatusTone` 固定为 `neutral/info/success/warning/error`；
- `SafeDisplayValue` 是带判别字段的联合类型。`redacted` 分支没有原始值字段，`empty` 与 `text` 也不能被误判为敏感值；
- `DesiredReportedField` 描述三路展示值、状态和证据；
- `PolicyContributor` 描述候选值、优先级、范围、时间范围和采用结果。

`apps/web/src/components/saas/index.ts` 是公共导出入口。业务层应从这里引用组件和展示类型，不应让组件依赖某个尚未存在的 API shape。以后接入 generated client 时，由页面 adapter 把响应映射成这些展示类型。

### 2.2 `ResourceHeader`

props 承载标题、描述、面包屑、revision 和主动作状态；`eyebrow`、`status`、`actions`、`overflow` slots 承载调用方内容。主动作只发出 `primaryAction`，组件不预判 mutation 结果。

页面标题使用唯一 `h1`。面包屑是带 `aria-label` 的 `nav/ol`；更多动作是有名称的 icon button。599px 以下切为单列身份区，动作区使用两列网格，主动作占可用宽度，面包屑可滚动而不挤压页面。

### 2.3 `StatusChip`

`tone` 只决定 Vuetify 语义色和默认图标，`label` 始终可见。`source` 为必填证据；`observedAt` 和 `description` 可选。组件把三者合并为 tooltip 和 `aria-label`，chip 可获得键盘焦点。颜色不是唯一状态信号。

### 2.4 `DangerDialog`

`objectName` 必须非空，输入内容与它进行区分大小写的精确比较。`reasonRequired` 打开后，只有非空白原因可以提交。`impactSummary` 是必填退路，调用方也可以用 `impact`、`dependencies`、`details` slots 提供结构化内容。

提交函数先设置本地 `submissionLocked`，再发出一次 `confirm`。因此父组件尚未来得及把 `pending` 设为 `true` 时，第二次点击、回车或关闭也会被拒绝。`pending` 或本地锁定期间 dialog 为 persistent，取消按钮不可用。对象名、原因和本地锁在关闭、重新打开和卸载时清空。

父层以 `retryRevision` 明确授权一次失败后的重试。若请求在 transport preflight 或同步校验阶段失败，从未经历 `pending=true`，仅更新 `errorMessage` 不会解锁；父层必须在错误状态稳定后推进 revision。组件只在本次提交已经锁定、revision 与提交时不同且当前不是 pending 时允许重试。已有错误、无关重渲染、`pending` 从 true 回到 false 或重复 revision 均不能开放第二次提交。成功后父层应关闭 dialog；只有已经确认失败的请求才能推进 `retryRevision`。

组件不执行近期认证，不创建 job，也不把错误当成功。接入危险业务动作时，页面仍须按 `impact → reason/re-auth/confirm → job → terminal` 完成状态机，并通过 `errorMessage` 显示持久错误。

### 2.5 `SecretField`

组件接收受控的 `modelValue`，自身只保存 `revealed: boolean`，不会把值复制到 store、Web Storage、BroadcastChannel、日志或 toast。输入默认是 password，显示按钮具有动态 `aria-label` 和 `aria-pressed`。浏览器进入隐藏状态、触发 `pagehide` 或组件卸载时，字段立即回到遮蔽状态。

`mode` 明确区分 create/replace；`configured` 只显示“已配置”，不会请求或回填现有值；`clearRequested` 使用单独的受控布尔值表达清除意图。选择清除后，组件同时发出空字符串，避免旧草稿继续由表单提交。

父页面仍拥有 `modelValue` 的生命周期，离开页面时必须清空对应表单状态。剪贴板写入及超时清理尚未放入本组件；在统一安全 clipboard 能覆盖失败、权限拒绝和计时器清理前，不提供复制按钮。

### 2.6 `DesiredReportedDiff`

组件只接受 `DesiredReportedField[]`。每行固定展示字段、状态、desired、reported、last-good 五列；最近正常值缺失时显示“未设置”。`StatusChip` 把 `match/pending/drift/unknown` 映射为可读状态，同时展示证据来源和时间。

`redactedRaw` slot 的名字和展开前提示明确要求调用方先脱敏。组件没有 raw object、JSON 或 YAML prop。`SafeDisplayValueView` 按 `kind` 读取字段；运行时对象即使在 `redacted` 分支夹带额外 `text` 属性，也不会渲染该属性。

桌面使用语义 table roles 和五列网格。599px 以下每个字段变成独立卡片，列名在每个值前重新显示，所有列仍留在 DOM 中，不以横向大表替代移动布局。

### 2.7 `PolicyExplainer`

组件显示一个 effective 值和按数字优先级从高到低排列的贡献列表。相同优先级保持输入顺序。每项包含来源名称、scope、priority、候选值、适用时间范围，以及 applied/excluded/overridden 状态。适用范围直接显示，并作为状态解释的一部分；它不会传给 `observedAt`，不会被标成观测时间。`effective` 和 `contributor` slots 允许调用方替换布局，slot props 仍只包含 UI 展示合同。

贡献列表使用有名称的 `ol/li`。599px 以下由四列压成两列，来源和值各自占满一行。敏感候选值复用 `SafeDisplayValueView`，不会从 redacted 分支读取附加原文。

## 3. 主题、响应式与无障碍

`apps/web/src/plugins/vuetify.ts` 注册 `nodecontrollLight` 和 `nodecontrollDark`。组件颜色全部引用 Vuetify 的 `primary`、`surface`、`surface-variant`、`outline` 和五种语义色，没有组件级十六进制状态色。

本批的窄屏断点统一为 599px，覆盖 360px 目标。标题动作、危险操作按钮、秘密值状态、差异卡片和策略来源均有窄屏布局。测试在 360px viewport 下确认 `ResourceHeader` 的主动作、次级动作和更多动作仍位于可访问树中；真正的像素级布局、200% zoom 和 light/dark visual snapshot 留给 WP-04 gallery/Playwright 门。

组件使用原生 heading、navigation、form、table、list、status、alertdialog 语义；图标按钮具有名称；状态不只依赖颜色；tooltip 激活器可获得键盘焦点。Vuetify dialog 负责焦点 trap 和返回，后续 axe 与屏幕阅读器人工矩阵仍是 WP-04 完成门的一部分。

## 4. 测试源码

当前有 11 个组件行为测试：

- `ResourceHeader.test.ts`：标题/面包屑/slot/动作，以及 360px 下动作可达；
- `StatusChip.test.ts`：图标、文字、证据 tooltip、键盘焦点和双主题注册；
- `DangerDialog.test.ts`：区分大小写的对象名、必填原因、form submit、本地单次锁、pending 关闭拒绝，以及同步失败必须由新终态 revision 才能解锁；
- `SecretField.test.ts`：默认遮蔽、显式显示、pagehide 再遮蔽、替换/清除事件，以及无 Web Storage/console 写入；
- `DesiredReportedDiff.test.ts`：语义状态、证据、空态、脱敏原始差异入口和夹带字段不泄露；
- `PolicyExplainer.test.ts`：优先级顺序、effective 值、适用时间证据不冒充观测时间，以及夹带字段不泄露。

这些是待 VPS 执行的测试源码数量，不是本机运行声明。最终 typecheck、零 warning lint 和 Vitest 结果只接受从同一提交 `git archive` 上传到固定 Node/pnpm builder 的 fresh install；禁止在本地执行，也不运行 production build。

## 5. 尚未覆盖

本批没有提前伪造以下能力：

- responsive app shell、command palette、permission route projection；
- AppDataTable、JobChip/JobDrawer、MetricChart、CapabilityGuard、AuditDiff、QrDialog；
- 真实 WP-01～03 页面接入、generated API adapter、job/recent-auth 协调；
- system 主题跟随、用户主题偏好、品牌色对比校验；
- 中英文 i18n 接入；
- gallery/Storybook、axe、visual snapshot、Playwright、屏幕阅读器与性能预算。

这些项目仍属于 WP-04，不能因本批组件源码存在而标记完成。
