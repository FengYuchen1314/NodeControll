# 妙妙屋前端源码说明

> 分析基线：`iluobei/miaomiaowu@0b47f10c52aee10b9f759a593ca5f61a823cbb72`。本章描述人工校对后的模块关系和业务语义；每个 TypeScript/TSX 声明、函数、方法及匿名回调的文件、行号、原创作用说明、调用和控制流证据见 [`generated/typescript/README.md`](generated/typescript/README.md)。公开索引不复制源码签名或表达式正文。

## 1. 技术基线与构建产物

前端位于上游仓库的 `miaomiaowu/` 子目录，包版本为 `0.8.3`，使用 React 19.2、TypeScript 5.9、Vite 7、TanStack Router/Query、Zustand、Tailwind CSS 4 和 Radix/shadcn 组件。表单、图表、拖拽、虚拟列表、二维码、YAML 与通知分别由 React Hook Form、Recharts、dnd-kit、TanStack Virtual、qrcode.react、js-yaml 和 Sonner 承担。

构建链为：

1. `tsc -b` 做项目引用和类型检查；
2. `vite build` 按文件路由自动分包；
3. 输出到 Go 包的 `internal/web/dist`；
4. Go 的 `internal/web/embed.go` 用 `go:embed dist/*` 把 SPA 嵌入最终二进制；
5. 完整 `npm run build` 还调用 `scripts/inject-site-config.js` 注入站点配置。

`vite.config.ts` 将 `base` 设为 `./`，开发时代理 `/api` 和 `/t/` 到 `VITE_API_URL` 或 `http://localhost:8080`。生产构建保留 source map，这方便排错，但会暴露可读源码结构并增加交付体积，部署时应由明确的调试策略控制。

VPS 实测 `npm ci && npm run build:only` 成功：457 个依赖包、3,327 个转换模块、176 个输出文件。仓库本身没有提交 `internal/web/dist`，所以直接执行后端测试会先因嵌入目录不存在而失败。

## 2. 应用启动与全局状态

### 2.1 `main.tsx`

入口按以下顺序装配应用：

```text
React StrictMode
└─ QueryClientProvider
   └─ ThemeProvider
      └─ FontProvider
         └─ DirectionProvider
            └─ RouterProvider
```

`QueryClient` 的默认行为是缓存 10 秒、生产环境窗口重新聚焦时刷新、开发环境不重试、生产环境最多重试三次，401/403 不重试。全局 mutation 错误交给 `handleServerError`；query 遇到 401 时清空认证并导航到首页，500 也导航到首页。路由器使用生成的 `routeTree.gen.ts`，默认按链接意图预加载，路由上下文只注入 `QueryClient`。

这里有两个值得保留到重构验收中的行为差异：Axios 响应拦截器把 401 强制跳到 `/login`，而 QueryCache 的 401 分支导航到 `/`；500 也被当作页面导航事件，而不是在原页面提供可恢复错误状态。

### 2.2 `routes/__root.tsx`

根路由承载所有页面共有的：

- 顶部导航进度条；
- 登录后才显示的星空主题背景；
- 子路由出口；
- 调试日志浮窗；
- 根据 768 px 断点切换位置的全局 Toast；
- 仅开发模式启用的 Query 与 Router Devtools；
- 统一 404 和未捕获路由错误页。

### 2.3 `stores/auth-store.ts` 与 Cookie

全局 Zustand store 只保存一个 `accessToken`。它在初始化时从 `traffic_info_access_token` Cookie 反序列化，设置或清除 token 时同步写 Cookie。Cookie 默认保留七天；代码没有将它设为 HttpOnly，因为 token 由 JavaScript 读取并放入自定义请求头。

该状态并不是完整用户会话：角色、昵称、邮箱、调试状态等由 React Query 从 `/api/user/profile` 等端点获取。退出或 401 时 `reset` 只删除 token。

### 2.4 `lib/api.ts`

共享 Axios 实例的地址选择规则是：

- 优先使用 `VITE_API_BASE_URL`；
- 生产配置若错误地写成 `http://localhost:8080`，主动退回同源；
- 本机浏览器默认连接当前协议下的 `localhost:8080`；
- 非本机默认连接当前页面同源。

请求拦截器把 token 放入 `MM-Authorization`。响应拦截器在 401 时清会话并硬跳转 `/login`；若 404 带 `X-Silent-Mode: true`，硬跳转 `/404`。`withCredentials` 为 false，身份依赖自定义 header 而不是 Cookie 自动携带。

## 3. 路由、权限与页面职责

源码中共有 23 个文件路由和 1 个根上下文路由。除 `/login`、`/404` 与根路由外，页面守卫大多只检查本地 token 是否非空。`/logs` 是唯一在前端守卫中读取 profile 并验证管理员角色的页面；其他管理页主要依赖导航隐藏与后端 `/api/admin/*` 授权，不能把前端守卫当作安全边界。

| 路由 | 页面/布局 | 主要职责 | 前端守卫 |
|---|---|---|---|
| `/` | `DashboardPage` | 当前用户流量汇总、使用进度、账户状态与入口卡片 | 无 token → `/login` |
| `/login` | `LoginPage` | 初始化状态探测、首次管理员创建/备份恢复、密码登录、Turnstile、2FA/恢复码 | 有 token → `/` |
| `/change-password` | `ChangePasswordRedirect` | 兼容旧入口并引导到设置中的密码修改 | 无 token → `/` |
| `/settings` | `SettingsPage` | 个人资料、密码、订阅 token、短链、自定义短码、2FA、界面偏好 | 无 token → `/` |
| `/subscription` | `SubscriptionShell` | 用户订阅页的无 UI 父布局 | 无 token → `/` |
| `/subscription/` | `SubscriptionPage` | 列出用户可用订阅文件，生成带 token 的订阅 URL、复制和二维码 | 无 token → `/` |
| `/generator` | `SubscriptionGeneratorPage` | 选择节点/代理集合/模板/规则，预览并生成多客户端订阅或临时订阅 | 无 token → `/login` |
| `/nodes` | `NodesShell` | 节点管理子路由出口 | 无 token → `/` |
| `/nodes/` | `NodesPage` | 节点 CRUD、批量编辑/标签/启停/排序、导入解析、远端同步、DNS/TCPing/测速、探针绑定、临时订阅 | 无 token → `/` |
| `/subscribe-files` | `SubscribeFilesLayout` | 订阅管理的 Topbar 与子路由出口 | 无 token → `/` |
| `/subscribe-files/` | `SubscribeFilesPage` | 订阅文件、聚合文件、外部订阅、代理集合、内容编辑、规则编辑、批量区域/协议集合及缓存刷新 | 无 token → `/` |
| `/subscribe-files/custom` | `CustomProxyGroupPage` | 代理组自定义入口；当前页面仅显示“功能开发中” | 无 token → `/` |
| `/templates` | `TemplatesLayout` | 旧模板管理子路由出口 | 无独立守卫 |
| `/templates/` | `TemplatesPage` | 旧版数据库模板 CRUD、预览和转换 | 无独立守卫，依赖父层/后端 |
| `/templates-v3/` | `TemplatesV3Page` | 文件模板上传、重命名、可见性、默认模板、结构化编辑与带节点预览 | 无 token → `/` |
| `/custom-rules` | 父路由 | 覆写管理子路由出口 | 父路由本身无业务 UI |
| `/custom-rules/` | `OverrideManagementPage` | 自定义规则与 JS 覆写脚本 CRUD、启停、排序/应用、内置模板 | 无 token → `/` |
| `/rules` | `RulesPage` | 规则文件选择、查看、编辑、保存和版本历史 | 无 token → `/` |
| `/probe` | `ProbeManagePage` | 探针服务器、探针配置、同步与连接状态管理 | 无 token → `/` |
| `/users` | `UsersPage` | 用户创建/删除/启停/重置密码/备注/自定义短码/订阅授权 | 无 token → `/` |
| `/system-settings` | `SystemSettingsPage` | 系统、通知、定时同步、静默模式、备份恢复、Turnstile 等全局配置 | 无 token → `/` |
| `/logs` | `LogsPage` | 操作日志、任务运行、安全事件、IP 封禁四类运维面板 | profile 必须为管理员 |
| `/404` | 独立静默页 | 静默模式的伪装/不可见页面 | 无 |

完整路由位置见 [`generated/typescript/routes.md`](generated/typescript/routes.md)。

## 4. 页面数据访问模式

页面主要使用 `useQuery` 读取、`useMutation` 写入，并在成功后用 `invalidateQueries` 或局部 state 更新。代码中可静态识别 225 个 `/api` 调用点，逐调用的 HTTP 方法、URL、所在函数和行号见 [`generated/typescript/api-calls.md`](generated/typescript/api-calls.md)。

主要 query-key 域包括 profile、traffic、nodes、subscriptions、templates、rules、probe、users、config、security、tasks 和 update。没有一个集中定义的 query-key 工厂；同一资源存在 `profile` 与 `user-profile` 等不同 key，容易造成重复请求或失效不完整。

页面直接定义大量请求/响应 interface，并直接拼动态 URL。前端没有生成式 API SDK，也没有统一 schema 校验层；运行时主要相信服务端 JSON 形状。大多数写操作依靠全局 Toast 反馈，少量页面同时保留局部 error state。

## 5. 业务组件

| 文件 | 作用 |
|---|---|
| `anime-starfield.tsx` | 根据登录态渲染主题星空背景与装饰动画。 |
| `backup-dialog.tsx` | 备份导出、文件选择、恢复确认和恢复结果交互。 |
| `clash-config-viewer.tsx` | 展示/格式化生成的 Clash YAML，支持查看与复制。 |
| `confirm-dialog.tsx` | 通用危险操作确认框。 |
| `custom-rules-editor.tsx` | 规则文本编辑与模板选择。 |
| `data-table.tsx` / `data-table.types.ts` | 通用表格、列定义、选择/分页等类型与渲染约定。 |
| `debug-floating-viewer.tsx` | 轮询调试状态和尾部日志，提供关闭调试能力。 |
| `edit-nodes-dialog.tsx` | 桌面端节点批量表单；涵盖协议字段、传输/TLS、标签、校验和保存。 |
| `mobile-edit-nodes-dialog.tsx` | 移动端简化节点批量编辑流程。 |
| `external-sync-node-dialog.tsx` | 从外部订阅选择节点并映射为本地同步配置。 |
| `flag-emoji-picker.tsx` | 国家/地区旗帜选择与节点名前缀处理。 |
| `mmwx-dialog.tsx` | 妙妙屋 X 推广/说明弹窗；不是社区版业务内核。 |
| `navigation-progress.tsx` | 路由跳转顶部进度条。 |
| `rule-selector.tsx` | 预定义规则集与自定义规则选择。 |
| `sign-out-dialog.tsx` | 退出确认并清理本地认证。 |
| `speedtest-dialog.tsx` | 发起单节点测速、轮询任务、展示指标和历史结果。 |
| `theme-switch.tsx` | 明暗主题切换。 |
| `twemoji.tsx` | Twemoji 资源渲染封装。 |
| `update-dialog.tsx` | 查询 GitHub/后端版本信息并呈现升级说明或升级动作。 |

### 5.1 布局组件

`layout/topbar.tsx` 按 profile 的 `is_admin` 动态组合普通导航与管理导航，并根据可用宽度逐步隐藏 Logo 文字、再把尾部导航收缩为纯图标；移动端改用下拉菜单。`layout/user-menu.tsx` 承载设置、调试开关、版本检查、更新和退出。`layout/nav-icon.tsx` 统一导航图标状态。

### 5.2 V3 模板组件

`keyword-filter-input` 在关键字与正则过滤器之间转换；`proxy-group-editor` 编辑代理组类型、节点/集合引用、过滤器和排序；`proxy-group-select`、`proxy-type-select` 提供受约束选择器；`template-preview` 展示 YAML 和结构化预览；`template-upload-dialog` 支持文件、URL、外部订阅和 V2 模板转换入口。

### 5.3 UI 基础层

`components/ui` 的 33 个文件是 Radix/shadcn 风格的无业务组件：alert、alert-dialog、avatar、badge、button/button-group、calendar、card、checkbox、collapsible、command、dialog、dropdown-menu、form、input/input-otp、label、popover、progress、radio-group、scroll-area、select、separator、sheet、sidebar、skeleton、sonner、switch、table、tabs、textarea、tooltip，以及一个 Kanban 封装。它们负责可访问性、样式变体和组合 API，不应该承载权限或数据访问逻辑。

## 6. Hook、Context 与配置模块

| 模块 | 作用 |
|---|---|
| `use-dialog-state` | 用统一 API 管理一个或多个对话框的开闭状态。 |
| `use-external-sync-selection` | 维护外部订阅节点选择、全选和清空。 |
| `use-media-query` / `use-mobile` | 监听媒体查询与 768 px 移动断点。 |
| `use-node-drag-drop` | 节点与代理组之间的 dnd-kit 拖放和排序。 |
| `use-proxy-groups` | 拉取、缓存并触发同步代理组分类。 |
| `use-version-check` | 调 GitHub Releases API 比较当前 `0.8.3` 与最新版本。 |
| `theme-provider` | 把主题写入 DOM/localStorage，并响应系统主题。 |
| `font-provider` | 在 OPlus Sans、JetBrains Mono、系统字体间切换。 |
| `direction-provider` | 提供 LTR/RTL 方向上下文。 |
| `custom-rules-templates` | 内置规则提供者片段和规则模板。 |
| `override-script-templates` | 内置 JavaScript 覆写脚本模板。 |
| `fonts` | 字体枚举和类型。 |

`use-version-check` 从浏览器直接请求 GitHub API，版本检测因此受外网可达性和 GitHub 限流影响。`country-flag.ts` 也包含从浏览器请求外部 GeoIP 服务的逻辑和硬编码 token；重构时必须移到可配置、可审计的服务端适配器。

## 7. 订阅构建与格式生产模块

前端包含两套相关但边界不同的实现。

### 7.1 `lib/sublink`

该目录面向生成器页面的 Clash 配置组装：

- `types.ts` 定义看板、代理、TLS/传输、自定义规则、Clash 配置、规则集和代理组分类；
- `clash-config.ts` 提供默认 Clash 配置和规则集下载基址；
- `clash-builder.ts` 的 `ClashConfigBuilder` 累积代理、代理组、规则提供者和规则并输出配置；
- `predefined-rules.ts` 把选中的规则分类展开为规则行；
- `proxy-groups.ts` 从后端读取/同步代理组分类，并生成预设映射；
- `translations.ts` 将内部出站/分类名称翻译为展示名称；
- `utils.ts` 放置生成过程中使用的清洗、去重和格式辅助函数。

### 7.2 `lib/substore/producers`

该目录把统一代理对象序列化为不同客户端格式。`index.ts` 注册大小写和历史别名，统一调度以下 producer：

| Producer | 输出目标与职责 |
|---|---|
| `clash.ts` | 传统 Clash 代理对象。 |
| `clashmeta.ts` | Clash.Meta/Mihomo 扩展字段、IP 版本与新协议映射。 |
| `sing-box.ts` | sing-box outbound JSON，处理 TLS、Reality、传输、分片等字段。 |
| `surge.ts` | Surge iOS 代理行。 |
| `surgemac.ts` | Surge Mac 兼容差异。 |
| `loon.ts` | Loon 代理行与插件语法。 |
| `qx.ts` | Quantumult X 节点语法。 |
| `shadowrocket.ts` | Shadowrocket URI/配置。 |
| `stash.ts` | Stash YAML 代理对象。 |
| `surfboard.ts` | Surfboard 代理行。 |
| `egern.ts` | Egern 配置对象。 |
| `uri.ts` | 按协议生成标准/客户端 URI。 |
| `v2ray.ts` | V2Ray base64 列表。 |
| `utils.ts` | 共享编码、字段清理、TLS/传输转换和兼容辅助。 |
| `index.ts` 内部 `JSON_Producer` | 返回内部对象或格式化 JSON，并注册所有别名。 |

这些 producer 是从 Sub-Store 风格代码移植的宽松 `any` 模型。不同文件重复实现协议分支和字段兼容，未知字段通常被忽略；新系统不能把它们直接当作 sing-box 的权威 schema。应先进入强类型的规范化节点模型，再由版本化 exporter 生成各客户端格式。

### 7.3 模板工具

`template-v3-utils.ts` 定义 V3 代理组模型、保留标记、表单↔配置转换、模板解析/序列化、区域代理组生成和预览。`clash-validator.ts` 校验代理、代理组引用和循环依赖，可给出字段重排后的修正版。`template-presets.ts` 提供 ACL4SSR/Aethersailor 预设。`proxy-types.ts` 是节点/Clash 代理的共享轻量类型。

## 8. 复杂度与重构风险

### 8.1 页面单体化

最大三个页面分别约 7,001、6,769 和 3,635 行：`nodes.index.tsx`、`subscribe-files.index.tsx`、`generator.tsx`。它们同时包含领域类型、查询、mutation、转换、对话框状态和大段 JSX。`edit-nodes-dialog.tsx` 也约 2,024 行。结果是：

- 页面内闭包数量非常多，自动索引统计的 3,263 个函数中大部分来自 JSX/数组/Hook 回调；
- 同一资源的读取和修改散落在数十个 mutation 中；
- 节点协议字段在桌面编辑、移动编辑、导入、生成器和多个 producer 之间重复；
- 很难为领域逻辑写脱离 DOM 的单元测试。

### 8.2 权限表达不一致

Topbar 会隐藏管理导航，但多数管理路由只验证“有 token”，不验证管理员。安全性最终依赖后端，这一点是正确底线，但前端会让普通用户先进入页面再收到 403。重构需用路由 meta 统一声明 `anonymous/user/admin`，后端继续独立强制 RBAC。

### 8.3 数据契约和身份存储

请求/响应类型分散且没有运行时 schema；自定义 header token 放在可被脚本读取的 Cookie 中；部分订阅 URL又把用户 token 放入 query。新系统需区分管理会话、API token、订阅凭证和一次性链接，做到可撤销、可轮换、作用域化，且日志统一脱敏。

### 8.4 外部依赖与生成正确性

浏览器直接访问 GitHub 和 GeoIP 服务会造成部署行为不确定。多客户端 producer 通过宽松对象转换，缺少按目标版本的兼容测试。重构必须建立协议 fixture、golden file 和目标客户端/sing-box 配置校验测试，而不是仅比较 UI 输出字符串。

## 9. 覆盖性索引

人工章节解释模块边界；机器索引用于达到逐函数追踪粒度：

- [`generated/typescript/areas/routes.md`](generated/typescript/areas/routes.md)：24 个路由文件内每个页面、Hook 回调、事件处理器和集合回调；
- [`generated/typescript/areas/components.md`](generated/typescript/areas/components.md)：20 个业务组件；
- [`generated/typescript/areas/components-layout.md`](generated/typescript/areas/components-layout.md)：3 个布局组件；
- [`generated/typescript/areas/components-template-v3.md`](generated/typescript/areas/components-template-v3.md)：6 个 V3 模板组件；
- [`generated/typescript/areas/components-ui.md`](generated/typescript/areas/components-ui.md)：33 个基础 UI 模块；
- [`generated/typescript/areas/hooks.md`](generated/typescript/areas/hooks.md)、[`context.md`](generated/typescript/areas/context.md)、[`config.md`](generated/typescript/areas/config.md)：Hook、Context 和预设；
- [`generated/typescript/areas/lib.md`](generated/typescript/areas/lib.md)：32 个 API、校验、模板、订阅与 producer 模块；
- [`generated/typescript/areas/stores.md`](generated/typescript/areas/stores.md)：认证状态；
- [`generated/typescript/areas/root.md`](generated/typescript/areas/root.md)：入口、类型与生成路由树。

自动索引中的“作用”是基于 AST 和命名的可复查推断；业务结论以本章、后端文档和数据流文档为准。
