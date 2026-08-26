# WP-04-B 响应式应用壳与跨域展示组件

本片把现有 `setup`、`login`、`dashboard`、`system`、`reauth`、`profile/security`、`profile/security/password` 真实路由装入统一的 SaaS 应用壳，并增加 capability、表格和任务展示基础件。它不增加业务 API、不伪造资源数据，也不代表旧业务页面已经全部完成中英文翻译。

## 1. 身份边界与路由投影

`App.vue` 仍是受保护 DOM 的唯一外层闸门，顺序固定为：session resolving、resolution unavailable、未认证 protected route、强制改密、capability deny、guest layout、authenticated shell。处于 quarantine、强制改密或 capability 被收回时，`AppShell` 会被卸载，drawer、账户名、页面标题、命令面板和原业务组件都不留在 DOM。

`router/route-names.ts` 是 shell 使用的 route name 联合类型；`router/capabilities.ts` 是当前后端已发布 capability scope 的拼写合同；`router/access.ts` 只执行 all-of 精确字符串匹配。`router/index.ts` 保持 setup → anonymous/relogin → forced password → recent auth → capability → guest 的既有优先级。Dashboard 是无需额外 capability 的 authenticated 安全落点；System 需要 `system:read`；账户安全页需要 `sessions:read + credentials:manage`；改密页需要 `credentials:manage`；reauth 不加 capability，避免阻断 step-up。

`shell/navigation.ts` 是 drawer、mobile drawer、account menu 和 command palette 的单一 registry。它只解析真实 route name，并再次读取 route meta 与当前 actor capabilities；缺 route、缺 capability、guest route 或强制改密期间不允许的 route 都默认剔除。角色字符串只经已知六角色本地化后显示，未知角色显示通用名称，永不参与授权推导。后端授权仍是最终边界，隐藏菜单不是安全控制的替代品。

## 2. AppShell 与可访问性

`AppShell.vue` 提供 desktop permanent drawer/rail、360px temporary drawer、top bar、账户/主题/语言菜单、skip link、可聚焦 main 和 route live announcement。路由变化后关闭移动 drawer 与 command palette，并把焦点移到 main。全局 `Ctrl/Cmd+K` listener 只在 shell 存活时注册，卸载时移除。

`CommandPalette.vue` 只接收已经授权的展示项，不读取 session 或业务 API。它支持初始搜索焦点、focus retention、Esc、上下箭头、Enter、过滤和空结果。提交期间禁用重复动作；router abort/rejection 时保持面板打开并显示本地通用错误，成功后才关闭。Vuetify dialog 是唯一 dialog role，不再制造嵌套 dialog。

`stores/ui-preferences.ts` 只允许持久化 `light|dark|system` 与 `zh-CN|en` 两组枚举；不保存 capability、actor、route、搜索词或资源名，Storage 被阻止时静默回安全默认值。`use-shell-preferences.ts` 在根 App 初始化，立即解析 system theme、同步 Vuetify、`html.lang` 和 `color-scheme`，并严格成对注册/清理 `matchMedia` listener。

## 3. 跨域共享组件

| 文件 | 合同 |
|---|---|
| `CapabilityGuard.vue` | `hide` 从结构删除；`disable` 同时使用 disabled fieldset、`inert`、捕获事件和 `aria-hidden` 阻断链接/自定义 click，原因留在可访问树；`explain` 只显示拒绝说明 |
| `AppDataTable.vue` | 纯 props/slots 的 loading、error、empty、stale、selection、desktop table 与 360px card；重复/空 column key 或 row key 直接显示配置错误；selected keys 只保留当前 row 集合 |
| `JobChip.vue` | 展示 canonical `queued/running/waiting/succeeded/failed/cancelled/expired`，有限进度钳制到 0–100；交互只发出 job id，不发 API |
| `JobDrawer.vue` | 只读任务事实与步骤；没有 retry/cancel；message 必须是 `SafeDisplayValue`，redacted 分支不会读取附带的隐藏 `text` |

所有新增组件的可见默认、状态与 ARIA 文案由调用者提供的 typed labels 决定，因此可以直接接入当前中英文 locale，而不在组件内固化某一种语言。`JobPresentation` 是纯 UI 展示合同，不复制 OpenAPI DTO；未来业务 adapter 必须把 API job 明确映射为安全投影，并在进入 `JobDrawer` 前清除日志、secret 和未审核错误正文。

## 4. 测试边界

新增源码测试覆盖：

- direct navigation capability deny、路由 meta invariant、dashboard 无 capability 回落环；
- session quarantine、退出 preflight 失败、强制改密和动态 capability 缩减时敏感 DOM 立即消失；
- drawer/command/account 单一权限投影、缺 route fail-closed、强制改密投影；
- 360px drawer、skip link、main focus、route announcement、Ctrl/Cmd+K、Esc、授权搜索与 navigation abort；
- theme/locale allowlist、Storage 异常、system theme 初值/change 与 listener cleanup；
- CapabilityGuard inert、AppDataTable 全状态/重复 key/stale selection/mobile DOM、Job canonical states/progress 与 redacted message。

Node/pnpm install、OpenAPI generation、typecheck、零 warning lint、Vitest、文档 validator 和 sanitizer 只允许在 VPS 的 fresh immutable archive 中运行。本地不运行编译或测试，也不运行 production build。

## 5. 尚未覆盖

- MetricChart 留到有真实指标 projection 与可访问图表合同的后续片；
- 当前只保证应用壳中英文，旧 setup/login/security/system 页面仍有历史中文硬编码；
- JobDrawer 尚未接入后端 job endpoint，也没有假数据 adapter；
- 本片没有增加资源 CRUD、对象级授权或新 API；
- 浏览器级视觉回归与完整真实路由 E2E 仍属于后续验收，production Web build 仍只允许 Actions。

## 6. VPS 候选门证据

实现候选 `5bfa15b3ee18b61503aa3c30ed194d9c204315a0` 由 `git archive` 生成唯一源码包，并在 VPS 的固定 builder `sha256:06628671caed76e73560464d4ce47cacb202fcf28d090c0d24f2ead1cc23afcb` 中以 Node `v24.19.0`、pnpm `11.24.0` 和 run-scoped 空 store 完成 fresh frozen install；未运行 production build。

- run：`20260826T170138Z-wp04-app-shell-v5`
- archive SHA-256：`8678fdd807c155fc52ffdd9cfd94be63191971670664f4fe8fef3bbbf2bf7f5f`
- source manifest：281 files，生成前后完全相同，SHA-256 `58ea6916bc63dd3042ae672c7695a863b301f7b68e1076c69999fca647501136`
- generated manifest：16 files，生成前后完全相同，SHA-256 `b0158df8fde2c31d8d491c211b51f209c55a20c471de4dca3e4b7bfd163e39ba`
- evidence manifest SHA-256：`696f08a175d615429b54c0182c0af6b03ba4bc8df835da61631fb7002eec5d8a`

门禁结果：OpenAPI SDK generator zero-drift；Vue typecheck 通过；ESLint 零 warning；Vitest `27/27` files、`148/148` tests；OpenAPI validator 为 3.1.0、13 paths、15 operations；文档 validator 为 358/358 trace rows、16 design documents、80 authored documents、0 broken links；sanitizer 为 0 files changed；不存在 `apps/web/dist`。

关键日志 SHA-256：generator `3f2ddcaed69115700bed6b8fb01ba25d73aaea116ff9addf91d64eede322398c`，typecheck `a47392ecb7cad530f3fb24433d02ab40cc5b98523be4c2b859cc80b91a1c9ea9`，lint `0596cdf27bfaa95796264046fc05b1a818227055fe896e07dd6911ebb67d4592`，Vitest `3f286f93cd29e164cf9dd97c7e4171c242281684ae0a68f76110a73359b8eb39`，OpenAPI `d9dc0729e874073cd4b922f47e4f29e3b9b55f25505701dcb7bc9b69014b61d3`，文档 `f8e2d1e0e6a94c9b7dcf7b10fba04b21c544435c6d0216ac0cd8df452498e724`，sanitizer `93d300cbea2f1c12cff3baef2943bae1b380ee8ca53a0b34e6ed949e69237cc3`。

run-scoped pnpm store、上传 archive、gate script 和 named container 已精确清除。为同一依赖锁下的后续 OpenAPI 定向生成，门禁成功后暂时保留只读的 root `node_modules`（280,128,403 bytes）与 Web `node_modules`（103,700 bytes）；两棵树的非 symlink 文件和目录均无写位，它们不是交付制品，复用完成后必须按精确路径删除。
