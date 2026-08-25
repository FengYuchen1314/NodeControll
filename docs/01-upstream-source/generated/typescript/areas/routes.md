# TypeScript 分区 `routes`

TanStack Router 页面、加载/重定向守卫和页面内业务交互。

## `routes/404.tsx`

依赖：`@tanstack/react-router`、`@/components/ui/button`、`lucide-react`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–7 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 9–21 | function | `NotFoundPage` | 渲染并协调 'NotFoundPage' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |

## `routes/__root.tsx`

依赖：`react`、`@tanstack/react-query`、`@tanstack/react-router`、`@tanstack/react-query-devtools`、`@tanstack/react-router-devtools`、`@/stores/auth-store`、`@/components/ui/sonner`、`@/components/anime-starfield`、`@/components/debug-floating-viewer`、`@/components/navigation-progress`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–46 | function | `RootComponent` | 渲染并协调 'RootComponent' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'useAuthStore'、'useEffect'、'useState' |
| 16–25 | function | `RootComponent > useEffect.callback#2` | 封装 'useEffect.callback#2' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'checkMobile'、'window.addEventListener' |
| 17–19 | function | `RootComponent > useEffect.callback#2 > checkMobile` | 执行与 'checkMobile' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setIsMobile' |
| 24–24 | function | `RootComponent > useEffect.callback#2 > <anonymous#4>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'window.removeEventListener' |
| 48–66 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 52–57 | function | `notFoundComponent` | 执行与 'notFoundComponent' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 58–65 | function | `errorComponent` | 执行与 'errorComponent' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |

## `routes/change-password.tsx`

依赖：`@tanstack/react-router`、`react`、`@/stores/auth-store`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–13 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 6–11 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 15–28 | function | `ChangePasswordRedirect` | 执行与 'ChangePasswordRedirect' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'useAuthStore'、'useEffect'、'useNavigate' |
| 19–25 | function | `ChangePasswordRedirect > useEffect.callback#3` | 封装 'useEffect.callback#3' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'navigate' |

## `routes/custom-rules.index.tsx`

依赖：`@tanstack/react-router`、`@tanstack/react-query`、`react`、`lucide-react`、`@/components/data-table`、`@/components/data-table`、`@/components/ui/button`、`@/components/ui/card`、`@/components/ui/dialog`、`@/components/ui/alert-dialog`、`@/components/ui/input`、`@/components/ui/label`、`@/components/ui/textarea`、`@/components/ui/switch`、`@/components/ui/badge`、`@/components/ui/select`、`@/components/ui/tabs`、`sonner`、`@/lib/api`、`../config/custom-rules-templates`、`@/config/override-script-templates`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 51–53 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 55–64 | interface | `CustomRule` | 定义 'CustomRule' 的数据契约、联合类型或组件属性。 |  |
| 66–75 | interface | `OverrideScript` | 定义 'OverrideScript' 的数据契约、联合类型或组件属性。 |  |
| 77–89 | interface | `OverrideItem` | 定义 'OverrideItem' 的数据契约、联合类型或组件属性。 |  |
| 91–91 | type | `OverrideType` | 定义 'OverrideType' 的数据契约、联合类型或组件属性。 |  |
| 93–101 | interface | `FormData` | 定义 'FormData' 的数据契约、联合类型或组件属性。 |  |
| 103–106 | const | `HOOK_LABELS` | 保存 'HOOK_LABELS' 的模块级常量、配置、路由或预计算值。 |  |
| 108–120 | function | `ruleToItem` | 执行与 'ruleToItem' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 122–135 | function | `scriptToItem` | 执行与 'scriptToItem' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 137–947 | function | `OverrideManagementPage` | 渲染并协调 'OverrideManagementPage' React 组件的状态、数据请求和用户交互。 | 分支 15；循环 0；返回 1；await 0；调用 '<BinaryExpression>.map'、'Object.entries'、'Object.entries.map'、'allItems.filter'、'rules.map'、'scripts.map'、'useMutation'、'useQuery'、'useQueryClient'、'useState' |
| 159–162 | function | `OverrideManagementPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 167–170 | function | `OverrideManagementPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 181–181 | function | `OverrideManagementPage > allItems.filter.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 186–197 | function | `OverrideManagementPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 1；返回 1；await 2；调用 'api.post'、'api.put'、'rules.filter' |
| 189–189 | function | `OverrideManagementPage > mutationFn > rules.filter.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 198–201 | function | `OverrideManagementPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 202–204 | function | `OverrideManagementPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 208–219 | function | `OverrideManagementPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 1；返回 1；await 2；调用 'api.put'、'rules.filter' |
| 211–211 | function | `OverrideManagementPage > mutationFn > rules.filter.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 220–223 | function | `OverrideManagementPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 224–226 | function | `OverrideManagementPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 230–232 | function | `OverrideManagementPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.delete' |
| 233–236 | function | `OverrideManagementPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 237–239 | function | `OverrideManagementPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 243–255 | function | `OverrideManagementPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 1；返回 0；await 2；调用 'api.put'、'rules.filter'、'rules.find' |
| 244–244 | function | `OverrideManagementPage > mutationFn > rules.find.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 248–248 | function | `OverrideManagementPage > mutationFn > rules.filter.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 256–259 | function | `OverrideManagementPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 260–262 | function | `OverrideManagementPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 267–269 | function | `OverrideManagementPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'api.post' |
| 270–273 | function | `OverrideManagementPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 274–274 | function | `OverrideManagementPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 278–280 | function | `OverrideManagementPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'api.put' |
| 281–284 | function | `OverrideManagementPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 285–285 | function | `OverrideManagementPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 289–291 | function | `OverrideManagementPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'api.delete' |
| 292–295 | function | `OverrideManagementPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 296–296 | function | `OverrideManagementPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 300–308 | function | `OverrideManagementPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'api.put' |
| 309–312 | function | `OverrideManagementPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 315–327 | function | `OverrideManagementPage > resetForm` | 重置与 'resetForm' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingItem'、'setFormData'、'setSelectedTemplate' |
| 329–332 | function | `OverrideManagementPage > handleCreate` | 处理与 'handleCreate' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'resetForm'、'setIsDialogOpen' |
| 334–346 | function | `OverrideManagementPage > handleEdit` | 处理与 'handleEdit' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingItem'、'setFormData'、'setIsDialogOpen' |
| 348–351 | function | `OverrideManagementPage > handleDelete` | 处理与 'handleDelete' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setDeletingItem'、'setIsDeleteDialogOpen' |
| 353–362 | function | `OverrideManagementPage > handleDeleteConfirm` | 处理与 'handleDeleteConfirm' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'deleteRuleMutation.mutate'、'deleteScriptMutation.mutate'、'setDeletingItem'、'setIsDeleteDialogOpen' |
| 364–373 | function | `OverrideManagementPage > handleToggle` | 处理与 'handleToggle' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0；调用 'scripts.find'、'toggleRuleMutation.mutate'、'toggleScriptMutation.mutate' |
| 368–368 | function | `OverrideManagementPage > handleToggle > scripts.find.callback#40` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 375–433 | function | `OverrideManagementPage > handleSubmit` | 处理与 'handleSubmit' 对应的前端业务、状态或数据转换逻辑。 | 分支 6；循环 0；返回 4；await 0；调用 'createRuleMutation.mutate'、'createScriptMutation.mutate'、'formData.content.trim'、'formData.name.trim'、'resetForm'、'setIsDialogOpen'、'setIsRuleProviderConfirmOpen'、'setPendingRuleProviderData'、'toast.error'、'updateRuleMutation.mutate'、'updateScriptMutation.mutate' |
| 435–486 | function | `OverrideManagementPage > handleRuleProviderConfirm` | 处理与 'handleRuleProviderConfirm' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 1；await 4；调用 'allExistingLines.map'、'api.get'、'api.get.then'、'createRuleMutation.mutateAsync'、'existingRulesRules.forEach'、'filteredNewLines.join'、'latestRules.filter'、'newLines.filter'、'queryClient.invalidateQueries'、'resetForm'、'ruleContent.split'、'ruleContent.split.map'、'ruleContent.split.map.filter'、'setIsDialogOpen'、'setIsRuleProviderConfi… |
| 449–449 | function | `OverrideManagementPage > handleRuleProviderConfirm > api.get.then.callback#43` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 450–450 | function | `OverrideManagementPage > handleRuleProviderConfirm > latestRules.filter.callback#44` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 455–458 | function | `OverrideManagementPage > handleRuleProviderConfirm > existingRulesRules.forEach.callback#45` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'allExistingLines.push'、'lines.slice'、'rule.content.split'、'rule.content.split.map'、'rule.content.split.map.filter' |
| 456–456 | function | `OverrideManagementPage > handleRuleProviderConfirm > existingRulesRules.forEach.callback#45 > rule.content.split.map.callback#46` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'l.trim' |
| 456–456 | function | `OverrideManagementPage > handleRuleProviderConfirm > existingRulesRules.forEach.callback#45 > rule.content.split.map.filter.callback#47` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 459–459 | function | `OverrideManagementPage > handleRuleProviderConfirm > ruleContent.split.map.callback#48` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'l.trim' |
| 459–459 | function | `OverrideManagementPage > handleRuleProviderConfirm > ruleContent.split.map.filter.callback#49` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 460–460 | function | `OverrideManagementPage > handleRuleProviderConfirm > allExistingLines.map.callback#50` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'l.toLowerCase' |
| 461–461 | function | `OverrideManagementPage > handleRuleProviderConfirm > newLines.filter.callback#51` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'existingLinesLower.includes'、'l.toLowerCase' |
| 488–496 | function | `OverrideManagementPage > getTypeLabel` | 读取或计算与 'getTypeLabel' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 5；await 0 |
| 498–511 | function | `OverrideManagementPage > getTypeBadgeClass` | 读取或计算与 'getTypeBadgeClass' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 5；await 0 |
| 513–523 | function | `OverrideManagementPage > getModeOrHookLabel` | 读取或计算与 'getModeOrHookLabel' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 5；await 0 |
| 573–573 | function | `OverrideManagementPage > getRowKey.callback#55` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 578–578 | function | `OverrideManagementPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 583–587 | function | `OverrideManagementPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'getTypeBadgeClass'、'getTypeLabel' |
| 591–591 | function | `OverrideManagementPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'getModeOrHookLabel' |
| 595–606 | function | `OverrideManagementPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0 |
| 599–599 | function | `OverrideManagementPage > cell > onCheckedChange.callback#60` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleToggle' |
| 610–614 | function | `OverrideManagementPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 '<NewExpression>.toLocaleString' |
| 618–627 | function | `OverrideManagementPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 620–620 | function | `OverrideManagementPage > cell > onClick.callback#63` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleEdit' |
| 623–623 | function | `OverrideManagementPage > cell > onClick.callback#64` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleDelete' |
| 633–678 | function | `OverrideManagementPage > header` | 执行与 'header' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0；调用 '<NewExpression>.toLocaleString'、'getModeOrHookLabel'、'getTypeBadgeClass'、'getTypeLabel' |
| 646–649 | function | `OverrideManagementPage > header > onClick.callback#66` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'handleDelete' |
| 666–666 | function | `OverrideManagementPage > header > onCheckedChange.callback#67` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleToggle' |
| 680–685 | function | `OverrideManagementPage > actions` | 执行与 'actions' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 681–681 | function | `OverrideManagementPage > actions > onClick.callback#69` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleEdit' |
| 715–715 | function | `OverrideManagementPage > onCheckedChange.callback#70` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormData' |
| 722–725 | function | `OverrideManagementPage > onClick.callback#71` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'resetForm'、'setIsDialogOpen' |
| 741–741 | function | `OverrideManagementPage > onChange.callback#72` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormData' |
| 751–757 | function | `OverrideManagementPage > onValueChange.callback#73` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'setFormData'、'setSelectedTemplate' |
| 777–777 | function | `OverrideManagementPage > onValueChange.callback#74` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormData' |
| 793–793 | function | `OverrideManagementPage > onValueChange.callback#75` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormData' |
| 814–831 | function | `OverrideManagementPage > onValueChange.callback#76` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 1；await 0；调用 'Object.values'、'Object.values.some'、'setFormData'、'setSelectedTemplate' |
| 824–824 | function | `OverrideManagementPage > onValueChange.callback#76 > Object.values.some.callback#77` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 839–843 | function | `OverrideManagementPage > Object.entries.map.callback#78` | 渲染并协调 'Object.entries.map.callback#78' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 855–861 | function | `OverrideManagementPage > onValueChange.callback#79` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'hookTemplates.find'、'setFormData' |
| 857–857 | function | `OverrideManagementPage > onValueChange.callback#79 > hookTemplates.find.callback#80` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 859–859 | function | `OverrideManagementPage > onValueChange.callback#79 > setFormData.callback#81` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 867–871 | function | `OverrideManagementPage > <BinaryExpression>.map.callback#82` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 885–885 | function | `OverrideManagementPage > onChange.callback#83` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormData' |
| 936–936 | function | `OverrideManagementPage > onClick.callback#84` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleRuleProviderConfirm' |
| 939–939 | function | `OverrideManagementPage > onClick.callback#85` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleRuleProviderConfirm' |

## `routes/custom-rules.tsx`

依赖：`@tanstack/react-router`、`@/components/layout/topbar`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–6 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 8–15 | function | `CustomRulesLayout` | 渲染并协调 'CustomRulesLayout' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |

## `routes/generator.tsx`

依赖：`react`、`@tanstack/react-router`、`@tanstack/react-query`、`lucide-react`、`@/components/layout/topbar`、`@/stores/auth-store`、`@/lib/api`、`@/lib/utils`、`@/components/edit-nodes-dialog`、`@/components/mobile-edit-nodes-dialog`、`@/hooks/use-media-query`、`@/components/data-table`、`@/components/data-table`、`@/components/ui/button`、`@/components/ui/button-group`、`@/components/ui/input`、`@/components/ui/label`、`@/components/ui/textarea`、`@/components/clash-config-viewer`、`@/components/ui/checkbox`、`@/components/ui/dialog`、`@/components/ui/alert-dialog`、`@/components/ui/switch`、`@/components/ui/card`、`@/components/ui/badge`、`@/components/ui/select`、`sonner`、`@/components/twemoji`、`@/lib/sublink/clash-builder`、`@/components/custom-rules-editor`、`@/components/rule-selector`、`@/hooks/use-proxy-groups`、`@/lib/sublink/types`、`@/lib/sublink/types`、`@/lib/country-flag`、`@/lib/template-presets`、`@/lib/clash-validator`、`js-yaml`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 71–83 | interface | `ProxyProviderConfig` | 定义 'ProxyProviderConfig' 的数据契约、联合类型或组件属性。 |  |
| 86–90 | const | `YAML_DUMP_OPTIONS` | 保存 'YAML_DUMP_OPTIONS' 的模块级常量、配置、路由或预计算值。 |  |
| 93–107 | function | `preprocessYaml` | 执行与 'preprocessYaml' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'yamlStr.replace' |
| 98–105 | function | `preprocessYaml > yamlStr.replace.callback#2` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 '<RegularExpressionLiteral>.test'、'value.replace' |
| 110–121 | const | `PROTOCOL_COLORS` | 保存 'PROTOCOL_COLORS' 的模块级常量、配置、路由或预计算值。 |  |
| 124–127 | function | `getProtocolColor` | 读取或计算与 'getProtocolColor' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'protocol.toLowerCase'、'protocol.toLowerCase.split'、'protocol.toLowerCase.split[<key>].trim' |
| 130–158 | function | `ensureShortIdAsString` | 执行与 'ensureShortIdAsString' 对应的前端业务、状态或数据转换逻辑。 | 分支 6；循环 1；返回 3；await 0；调用 'Array.isArray'、'Object.entries'、'String'、'ensureShortIdAsString'、'obj.map' |
| 161–172 | function | `fixShortIdInYaml` | 执行与 'fixShortIdInYaml' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'result.replace' |
| 175–194 | function | `reorderProxyFields` | 执行与 'reorderProxyFields' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 2；返回 1；await 0；调用 'Object.entries'、'priorityKeys.includes' |
| 196–210 | type | `SavedNode` | 定义 'SavedNode' 的数据契约、联合类型或组件属性。 |  |
| 213–223 | interface | `Template` | 定义 'Template' 的数据契约、联合类型或组件属性。 |  |
| 225–225 | type | `TemplateFormData` | 定义 'TemplateFormData' 的数据契约、联合类型或组件属性。 |  |
| 227–235 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 228–233 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 237–3635 | function | `SubscriptionGeneratorPage` | 渲染并协调 'SubscriptionGeneratorPage' React 组件的状态、数据请求和用户交互。 | 分支 19；循环 0；返回 1；await 0；调用 'Array.from'、'Array.from.sort'、'Boolean'、'Math.ceil'、'Math.max'、'protocols.map'、'savedNodes.filter'、'sortedEnabledNodes.flatMap'、'sortedEnabledNodes.map'、'useAuthStore'、'useEffect'、'useMediaQuery'、'useMemo'、'useMutation'、'useProxyGroupCategories'、'useQuery'、'useQueryClient'、'useState' |
| 319–327 | function | `SubscriptionGeneratorPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 342–345 | function | `SubscriptionGeneratorPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 352–355 | function | `SubscriptionGeneratorPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 362–365 | function | `SubscriptionGeneratorPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 372–375 | function | `SubscriptionGeneratorPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 387–390 | function | `SubscriptionGeneratorPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 398–401 | function | `SubscriptionGeneratorPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 409–412 | function | `SubscriptionGeneratorPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 418–418 | function | `SubscriptionGeneratorPage > savedNodes.filter.callback#17` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 421–434 | function | `SubscriptionGeneratorPage > useMemo.callback#18` | 封装 'useMemo.callback#18' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 '<ArrayLiteralExpression>.sort'、'userConfig.node_order.forEach' |
| 427–427 | function | `SubscriptionGeneratorPage > useMemo.callback#18 > userConfig.node_order.forEach.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'orderMap.set' |
| 429–433 | function | `SubscriptionGeneratorPage > useMemo.callback#18 > <ArrayLiteralExpression>.sort.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'orderMap.get' |
| 437–454 | function | `SubscriptionGeneratorPage > useMemo.callback#21` | 封装 'useMemo.callback#21' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'dbTemplates.map'、'oldTemplates.map' |
| 440–444 | function | `SubscriptionGeneratorPage > useMemo.callback#21 > dbTemplates.map.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 448–452 | function | `SubscriptionGeneratorPage > useMemo.callback#21 > oldTemplates.map.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'filename.replace' |
| 457–461 | function | `SubscriptionGeneratorPage > useEffect.callback#24` | 封装 'useEffect.callback#24' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setSelectedTemplateUrl' |
| 465–468 | function | `SubscriptionGeneratorPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 469–474 | function | `SubscriptionGeneratorPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'resetTemplateForm'、'setIsTemplateFormDialogOpen'、'toast.success' |
| 475–477 | function | `SubscriptionGeneratorPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 482–485 | function | `SubscriptionGeneratorPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put' |
| 486–491 | function | `SubscriptionGeneratorPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'resetTemplateForm'、'setIsTemplateFormDialogOpen'、'toast.success' |
| 492–494 | function | `SubscriptionGeneratorPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 499–501 | function | `SubscriptionGeneratorPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.delete' |
| 502–507 | function | `SubscriptionGeneratorPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setDeletingTemplateId'、'setIsTemplateDeleteDialogOpen'、'toast.success' |
| 508–510 | function | `SubscriptionGeneratorPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 515–517 | function | `SubscriptionGeneratorPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.put' |
| 518–523 | function | `SubscriptionGeneratorPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingOldTemplate'、'setOldTemplateContent'、'setOldTemplateEditDialogOpen'、'toast.success' |
| 524–526 | function | `SubscriptionGeneratorPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 531–533 | function | `SubscriptionGeneratorPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.delete' |
| 534–539 | function | `SubscriptionGeneratorPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setDeletingOldTemplate'、'setIsOldTemplateDeleteDialogOpen'、'toast.success' |
| 540–542 | function | `SubscriptionGeneratorPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 547–552 | function | `SubscriptionGeneratorPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post'、'formData.append' |
| 553–556 | function | `SubscriptionGeneratorPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 557–559 | function | `SubscriptionGeneratorPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 564–567 | function | `SubscriptionGeneratorPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 568–574 | function | `SubscriptionGeneratorPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setIsOldTemplateRenameDialogOpen'、'setNewOldTemplateName'、'setRenamingOldTemplate'、'toast.success' |
| 575–577 | function | `SubscriptionGeneratorPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 581–591 | function | `SubscriptionGeneratorPage > resetTemplateForm` | 重置与 'resetTemplateForm' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingTemplate'、'setTemplateFormData' |
| 594–597 | function | `SubscriptionGeneratorPage > handleCreateTemplate` | 处理与 'handleCreateTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'resetTemplateForm'、'setIsTemplateFormDialogOpen' |
| 599–610 | function | `SubscriptionGeneratorPage > handleEditTemplate` | 处理与 'handleEditTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingTemplate'、'setIsTemplateFormDialogOpen'、'setTemplateFormData' |
| 612–615 | function | `SubscriptionGeneratorPage > handleDeleteTemplate` | 处理与 'handleDeleteTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setDeletingTemplateId'、'setIsTemplateDeleteDialogOpen' |
| 618–632 | function | `SubscriptionGeneratorPage > handleEditOldTemplate` | 处理与 'handleEditOldTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get'、'setEditingOldTemplate'、'setIsOldTemplateLoading'、'setOldTemplateContent'、'setOldTemplateEditDialogOpen'、'toast.error' |
| 634–640 | function | `SubscriptionGeneratorPage > handleSaveOldTemplate` | 处理与 'handleSaveOldTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'updateOldTemplateMutation.mutate' |
| 642–645 | function | `SubscriptionGeneratorPage > handleDeleteOldTemplate` | 处理与 'handleDeleteOldTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setDeletingOldTemplate'、'setIsOldTemplateDeleteDialogOpen' |
| 647–658 | function | `SubscriptionGeneratorPage > handleUploadOldTemplate` | 处理与 'handleUploadOldTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'document.createElement'、'input.click' |
| 651–656 | function | `SubscriptionGeneratorPage > handleUploadOldTemplate > <anonymous#54>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'uploadOldTemplateMutation.mutate' |
| 660–665 | function | `SubscriptionGeneratorPage > handleRenameOldTemplate` | 处理与 'handleRenameOldTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'filename.replace'、'setIsOldTemplateRenameDialogOpen'、'setNewOldTemplateName'、'setRenamingOldTemplate' |
| 667–673 | function | `SubscriptionGeneratorPage > handleConfirmRenameOldTemplate` | 处理与 'handleConfirmRenameOldTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'newOldTemplateName.trim'、'renameOldTemplateMutation.mutate' |
| 675–699 | function | `SubscriptionGeneratorPage > handlePreviewTemplate` | 处理与 'handlePreviewTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 1；调用 'api.post'、'setIsTemplatePreviewDialogOpen'、'setIsTemplatePreviewLoading'、'setTemplatePreviewContent'、'toast.error' |
| 701–724 | function | `SubscriptionGeneratorPage > handlePreviewSource` | 处理与 'handlePreviewSource' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 1；调用 'api.post'、'setIsSourcePreviewDialogOpen'、'setIsSourcePreviewLoading'、'setSourcePreviewContent'、'setSourcePreviewTitle'、'toast.error' |
| 726–759 | function | `SubscriptionGeneratorPage > handlePreviewSelectedSource` | 处理与 'handlePreviewSelectedSource' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 2；await 1；调用 'allTemplates.find'、'api.post'、'handleEditOldTemplate'、'setIsSourcePreviewDialogOpen'、'setIsSourcePreviewLoading'、'setSourcePreviewContent'、'setSourcePreviewTitle'、'toast.error' |
| 733–733 | function | `SubscriptionGeneratorPage > handlePreviewSelectedSource > allTemplates.find.callback#60` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 761–788 | function | `SubscriptionGeneratorPage > handleSubmitTemplate` | 处理与 'handleSubmitTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 2；await 0；调用 'createTemplateMutation.mutate'、'templateFormData.name.trim'、'templateFormData.rule_source.startsWith'、'templateFormData.rule_source.trim'、'toast.error'、'updateTemplateMutation.mutate' |
| 791–802 | function | `SubscriptionGeneratorPage > getAvailablePresets` | 读取或计算与 'getAvailablePresets' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'dbTemplates.map'、'filterPresets' |
| 792–792 | function | `SubscriptionGeneratorPage > getAvailablePresets > dbTemplates.map.callback#63` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 793–793 | function | `SubscriptionGeneratorPage > getAvailablePresets > dbTemplates.map.callback#64` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 795–796 | function | `SubscriptionGeneratorPage > getAvailablePresets > filterPresets` | 筛选与 'filterPresets' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'presets.filter' |
| 796–796 | function | `SubscriptionGeneratorPage > getAvailablePresets > filterPresets > presets.filter.callback#66` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'existingNames.has'、'existingUrls.has' |
| 805–814 | function | `SubscriptionGeneratorPage > handleTemplatePresetSelect` | 处理与 'handleTemplatePresetSelect' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'ALL_TEMPLATE_PRESETS.find'、'setTemplateFormData' |
| 806–806 | function | `SubscriptionGeneratorPage > handleTemplatePresetSelect > ALL_TEMPLATE_PRESETS.find.callback#68` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 817–817 | function | `SubscriptionGeneratorPage > sortedEnabledNodes.map.callback#69` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'n.protocol.toLowerCase' |
| 820–820 | function | `SubscriptionGeneratorPage > sortedEnabledNodes.flatMap.callback#70` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0 |
| 823–841 | function | `SubscriptionGeneratorPage > useMemo.callback#71` | 封装 'useMemo.callback#71' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'sortedEnabledNodes.filter' |
| 829–840 | function | `SubscriptionGeneratorPage > useMemo.callback#71 > sortedEnabledNodes.filter.callback#72` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 3；await 0；调用 'node.protocol.toLowerCase'、'nodeTags.some'、'selectedProtocols.has' |
| 837–837 | function | `SubscriptionGeneratorPage > useMemo.callback#71 > sortedEnabledNodes.filter.callback#72 > nodeTags.some.callback#73` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedTags.has' |
| 844–846 | function | `SubscriptionGeneratorPage > useEffect.callback#74` | 封装 'useEffect.callback#74' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeListPage' |
| 849–851 | function | `SubscriptionGeneratorPage > useEffect.callback#75` | 封装 'useEffect.callback#75' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setNodeListPage' |
| 853–856 | function | `SubscriptionGeneratorPage > useMemo.callback#76` | 封装 'useMemo.callback#76' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'filteredNodes.slice' |
| 858–863 | function | `SubscriptionGeneratorPage > useMemo.callback#77` | 封装 'useMemo.callback#77' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'Math.min' |
| 865–873 | function | `SubscriptionGeneratorPage > handleToggleNode` | 处理与 'handleToggleNode' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'newSet.add'、'newSet.delete'、'newSet.has'、'setSelectedNodeIds' |
| 875–881 | function | `SubscriptionGeneratorPage > handleToggleAll` | 处理与 'handleToggleAll' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'filteredNodes.map'、'setSelectedNodeIds' |
| 879–879 | function | `SubscriptionGeneratorPage > handleToggleAll > filteredNodes.map.callback#80` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 895–910 | function | `SubscriptionGeneratorPage > useMemo.callback#81` | 封装 'useMemo.callback#81' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'allProxies.filter'、'proxyGroups.forEach' |
| 902–906 | function | `SubscriptionGeneratorPage > useMemo.callback#81 > proxyGroups.forEach.callback#82` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'group.proxies.forEach' |
| 903–905 | function | `SubscriptionGeneratorPage > useMemo.callback#81 > proxyGroups.forEach.callback#82 > group.proxies.forEach.callback#83` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedNodes.add' |
| 909–909 | function | `SubscriptionGeneratorPage > useMemo.callback#81 > allProxies.filter.callback#84` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedNodes.has' |
| 913–1076 | function | `SubscriptionGeneratorPage > handleLoadTemplate` | 处理与 'handleLoadTemplate' 对应的前端业务、状态或数据转换逻辑。 | 分支 13；循环 0；返回 7；await 4；调用 'api.get'、'api.post'、'console.error'、'ensureShortIdAsString'、'fixShortIdInYaml'、'formatValidationIssues'、'proxies.map'、'selectedNodes.map'、'selectedNodes.map.filter'、'setClashConfig'、'setHasManuallyGrouped'、'setLoading'、'sortedEnabledNodes.filter'、'toast.error'、'toast.success'、'validateClashConfig'、'yaml.dump'、'yaml.load' |
| 948–948 | function | `SubscriptionGeneratorPage > handleLoadTemplate > sortedEnabledNodes.filter.callback#86` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 949–956 | function | `SubscriptionGeneratorPage > handleLoadTemplate > selectedNodes.map.callback#87` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 2；await 0；调用 'JSON.parse'、'console.error' |
| 956–956 | function | `SubscriptionGeneratorPage > handleLoadTemplate > selectedNodes.map.filter.callback#88` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 967–967 | function | `SubscriptionGeneratorPage > handleLoadTemplate > proxies.map.callback#89` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 982–982 | function | `SubscriptionGeneratorPage > handleLoadTemplate > proxies.map.callback#90` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'reorderProxyFields' |
| 1001–1001 | function | `SubscriptionGeneratorPage > handleLoadTemplate > proxies.map.callback#91` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'reorderProxyFields' |
| 1050–1050 | function | `SubscriptionGeneratorPage > handleLoadTemplate > validationResult.issues.filter.callback#92` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1078–1194 | function | `SubscriptionGeneratorPage > handleGenerate` | 处理与 'handleGenerate' 对应的前端业务、状态或数据转换逻辑。 | 分支 9；循环 0；返回 4；await 1；调用 'api.post'、'clashBuilder.build'、'console.error'、'customRules.filter'、'formatValidationIssues'、'selectedNodes.map'、'selectedNodes.map.filter'、'setClashConfig'、'setLoading'、'sortedEnabledNodes.filter'、'toast.error'、'toast.info'、'toast.success'、'toast.warning'、'validateClashConfig'、'validationResult.issues.filter'、'yaml.dump'、'yaml.lo… |
| 1087–1087 | function | `SubscriptionGeneratorPage > handleGenerate > sortedEnabledNodes.filter.callback#94` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 1088–1095 | function | `SubscriptionGeneratorPage > handleGenerate > selectedNodes.map.callback#95` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 2；await 0；调用 'JSON.parse'、'console.error' |
| 1095–1095 | function | `SubscriptionGeneratorPage > handleGenerate > selectedNodes.map.filter.callback#96` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1105–1105 | function | `SubscriptionGeneratorPage > handleGenerate > customRules.filter.callback#97` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'rule.name.trim' |
| 1162–1162 | function | `SubscriptionGeneratorPage > handleGenerate > validationResult.issues.filter.callback#98` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1196–1202 | function | `SubscriptionGeneratorPage > handleClear` | 处理与 'handleClear' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setClashConfig'、'setCustomRules'、'setSelectedCategories'、'setSelectedNodeIds'、'toast.info' |
| 1206–1218 | function | `SubscriptionGeneratorPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 1219–1228 | function | `SubscriptionGeneratorPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setSaveDialogOpen'、'setSubscribeDescription'、'setSubscribeFilename'、'setSubscribeName'、'toast.info'、'toast.success' |
| 1229–1232 | function | `SubscriptionGeneratorPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1235–1246 | function | `SubscriptionGeneratorPage > handleOpenSaveDialog` | 处理与 'handleOpenSaveDialog' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'setSaveDialogOpen'、'toast.error' |
| 1248–1285 | function | `SubscriptionGeneratorPage > handleSaveSubscribe` | 处理与 'handleSaveSubscribe' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 1；await 0；调用 'parseFloat'、'saveSubscribeMutation.mutate'、'statsServerIds.join'、'subscribeDescription.trim'、'subscribeFilename.trim'、'subscribeName.trim'、'toast.error' |
| 1288–1323 | function | `SubscriptionGeneratorPage > handleOpenGroupDialog` | 处理与 'handleOpenGroupDialog' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 '<AsExpression>.map'、'console.error'、'preprocessYaml'、'selectedNodes.map'、'setAllProxies'、'setGroupDialogOpen'、'setProxyGroups'、'sortedEnabledNodes.filter'、'toast.error'、'yaml.load' |
| 1304–1308 | function | `SubscriptionGeneratorPage > handleOpenGroupDialog > <AsExpression>.map.callback#106` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1311–1311 | function | `SubscriptionGeneratorPage > handleOpenGroupDialog > sortedEnabledNodes.filter.callback#107` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 1312–1312 | function | `SubscriptionGeneratorPage > handleOpenGroupDialog > selectedNodes.map.callback#108` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1325–1617 | function | `SubscriptionGeneratorPage > handleApplyGrouping` | 处理与 'handleApplyGrouping' 对应的前端业务、状态或数据转换逻辑。 | 分支 14；循环 4；返回 0；await 1；调用 'Object.entries'、'Object.keys'、'allMmwProviderNames.filter'、'api.get'、'console.error'、'data.nodes.forEach'、'data.nodes.map'、'mmwGroupsToAdd.push'、'nonMmwProviders.forEach'、'parsedConfig[<key>].filter'、'parsedConfig[<key>].findIndex'、'parsedConfig[<key>].map'、'preprocessYaml'、'proxyGroups.forEach'、'proxyGroups.map'、'proxyProviderCo… |
| 1332–1332 | function | `SubscriptionGeneratorPage > handleApplyGrouping > proxyProviderConfigs.filter.callback#110` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1333–1333 | function | `SubscriptionGeneratorPage > handleApplyGrouping > proxyProviderConfigs.filter.map.callback#111` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1337–1350 | function | `SubscriptionGeneratorPage > handleApplyGrouping > proxyGroups.forEach.callback#112` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'group.proxies.forEach'、'group.use.forEach' |
| 1340–1340 | function | `SubscriptionGeneratorPage > handleApplyGrouping > proxyGroups.forEach.callback#112 > group.use.forEach.callback#113` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedProviders.add' |
| 1344–1348 | function | `SubscriptionGeneratorPage > handleApplyGrouping > proxyGroups.forEach.callback#112 > group.proxies.forEach.callback#114` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'allMmwProviderNames.includes'、'usedProviders.add' |
| 1354–1354 | function | `SubscriptionGeneratorPage > handleApplyGrouping > proxyProviderConfigs.filter.callback#115` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedProviders.has' |
| 1357–1357 | function | `SubscriptionGeneratorPage > handleApplyGrouping > proxyProviderConfigs.filter.callback#116` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedProviders.has' |
| 1362–1362 | function | `SubscriptionGeneratorPage > handleApplyGrouping > allMmwProviderNames.filter.callback#117` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedProviders.has' |
| 1380–1420 | function | `SubscriptionGeneratorPage > handleApplyGrouping > proxyGroups.map.callback#118` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 1；await 0；调用 'group.proxies.filter'、'group.use.forEach' |
| 1383–1383 | function | `SubscriptionGeneratorPage > handleApplyGrouping > proxyGroups.map.callback#118 > group.proxies.filter.callback#119` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1390–1398 | function | `SubscriptionGeneratorPage > handleApplyGrouping > proxyGroups.map.callback#118 > group.use.forEach.callback#120` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'mmwGroupNames.push'、'newUse.push' |
| 1425–1425 | function | `SubscriptionGeneratorPage > handleApplyGrouping > data.nodes.map.callback#121` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1429–1429 | function | `SubscriptionGeneratorPage > handleApplyGrouping > parsedConfig[<key>].findIndex.callback#122` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1459–1465 | function | `SubscriptionGeneratorPage > handleApplyGrouping > parsedConfig[<key>].filter.callback#123` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'console.log'、'unusedMmwProviders.includes' |
| 1473–1482 | function | `SubscriptionGeneratorPage > handleApplyGrouping > data.nodes.forEach.callback#124` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'parsedConfig.proxies.findIndex'、'parsedConfig.proxies.push'、'reorderProxyFields' |
| 1476–1476 | function | `SubscriptionGeneratorPage > handleApplyGrouping > data.nodes.forEach.callback#124 > parsedConfig.proxies.findIndex.callback#125` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1488–1504 | function | `SubscriptionGeneratorPage > handleApplyGrouping > nonMmwProviders.forEach.callback#126` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 1512–1512 | function | `SubscriptionGeneratorPage > handleApplyGrouping > parsedConfig[<key>].map.callback#127` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1513–1523 | function | `SubscriptionGeneratorPage > handleApplyGrouping > parsedConfig[<key>].forEach.callback#128` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Array.isArray'、'group.proxies.forEach' |
| 1515–1521 | function | `SubscriptionGeneratorPage > handleApplyGrouping > parsedConfig[<key>].forEach.callback#128 > group.proxies.forEach.callback#129` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 '<ArrayLiteralExpression>.includes'、'groupNames.has'、'usedNodeNames.add' |
| 1528–1529 | function | `SubscriptionGeneratorPage > handleApplyGrouping > parsedConfig.proxies.filter.callback#130` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedNodeNames.has' |
| 1542–1546 | function | `SubscriptionGeneratorPage > handleApplyGrouping > savedNodes.forEach.callback#131` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeIDToName.set'、'nodeNameToChainID.set'、'nodeProtocolMap.set' |
| 1549–1554 | function | `SubscriptionGeneratorPage > handleApplyGrouping > parsedConfig.proxies.forEach.callback#132` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'nodeProtocolMap.get'、'protocol.includes' |
| 1557–1565 | function | `SubscriptionGeneratorPage > handleApplyGrouping > parsedConfig.proxies.forEach.callback#133` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'nodeIDToName.get'、'nodeNameToChainID.get' |
| 1570–1570 | function | `SubscriptionGeneratorPage > handleApplyGrouping > proxyGroups.some.callback#134` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1572–1572 | function | `SubscriptionGeneratorPage > handleApplyGrouping > group.proxies.filter.callback#135` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1573–1580 | function | `SubscriptionGeneratorPage > handleApplyGrouping > parsedConfig.proxies.map.callback#136` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 3；await 0；调用 'nodeNames.has'、'nodeProtocolMap.get'、'protocol.includes' |
| 1586–1586 | function | `SubscriptionGeneratorPage > handleApplyGrouping > parsedConfig.proxies.map.callback#137` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'reorderProxyFields' |
| 1620–1661 | function | `SubscriptionGeneratorPage > validateRulesNodes` | 校验与 'validateRulesNodes' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'Array.from'、'parsedConfig.proxies.map'、'parsedConfig[<key>].map'、'proxyGroupNames.add'、'rules.forEach' |
| 1622–1622 | function | `SubscriptionGeneratorPage > validateRulesNodes > parsedConfig[<key>].map.callback#139` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1623–1623 | function | `SubscriptionGeneratorPage > validateRulesNodes > parsedConfig.proxies.map.callback#140` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1634–1656 | function | `SubscriptionGeneratorPage > validateRulesNodes > rules.forEach.callback#141` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 2；await 0；调用 'missingNodes.add'、'parts[<key>].trim'、'proxyGroupNames.has'、'proxyNames.has'、'rule.split'、'toast' |
| 1664–1729 | function | `SubscriptionGeneratorPage > handleApplyReplacement` | 处理与 'handleApplyReplacement' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'Array.isArray'、'console.error'、'ensureShortIdAsString'、'fixShortIdInYaml'、'parsedConfig.proxies.map'、'parsedConfig[<key>].map'、'preprocessYaml'、'proxyGroupNames.add'、'rules.map'、'setClashConfig'、'setGroupDialogOpen'、'setHasManuallyGrouped'、'setMissingNodes'、'setMissingNodesDialogOpen'、'setPendingConfigAfterGrouping'、'toast.success… |
| 1668–1668 | function | `SubscriptionGeneratorPage > handleApplyReplacement > parsedConfig[<key>].map.callback#143` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1669–1669 | function | `SubscriptionGeneratorPage > handleApplyReplacement > parsedConfig.proxies.map.callback#144` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1678–1702 | function | `SubscriptionGeneratorPage > handleApplyReplacement > rules.map.callback#145` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 9；循环 0；返回 4；await 0；调用 'parts.join'、'parts[<key>].trim'、'proxyGroupNames.has'、'proxyNames.has'、'rule.split' |
| 1706–1706 | function | `SubscriptionGeneratorPage > handleApplyReplacement > parsedConfig.proxies.map.callback#146` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'reorderProxyFields' |
| 1732–1796 | function | `SubscriptionGeneratorPage > handleConfigureChainProxy` | 处理与 'handleConfigureChainProxy' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 0；await 0；调用 'Array.from'、'newGroups.map'、'newGroups.map.join'、'newGroups.push'、'proxyGroups.some'、'setProxyGroups'、'sortedEnabledNodes.forEach'、'toast.info'、'toast.success' |
| 1734–1734 | function | `SubscriptionGeneratorPage > handleConfigureChainProxy > proxyGroups.some.callback#148` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1735–1735 | function | `SubscriptionGeneratorPage > handleConfigureChainProxy > proxyGroups.some.callback#149` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1738–1738 | function | `SubscriptionGeneratorPage > handleConfigureChainProxy > sortedEnabledNodes.forEach.callback#150` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeIDToName.set' |
| 1743–1749 | function | `SubscriptionGeneratorPage > handleConfigureChainProxy > sortedEnabledNodes.forEach.callback#151` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'landingNodeNames.add'、'nodeIDToName.get'、'relayNodeNames.add' |
| 1771–1791 | function | `SubscriptionGeneratorPage > handleConfigureChainProxy > setProxyGroups.callback#152` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'newGroups.some'、'updatedGroups.map' |
| 1775–1775 | function | `SubscriptionGeneratorPage > handleConfigureChainProxy > setProxyGroups.callback#152 > newGroups.some.callback#153` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1776–1787 | function | `SubscriptionGeneratorPage > handleConfigureChainProxy > setProxyGroups.callback#152 > updatedGroups.map.callback#154` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 '<BinaryExpression>.filter' |
| 1779–1779 | function | `SubscriptionGeneratorPage > handleConfigureChainProxy > setProxyGroups.callback#152 > updatedGroups.map.callback#154 > <BinaryExpression>.filter.callback#155` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1792–1792 | function | `SubscriptionGeneratorPage > handleConfigureChainProxy > newGroups.map.callback#156` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1799–1817 | function | `SubscriptionGeneratorPage > generateProxyGroupYaml` | 生成与 'generateProxyGroupYaml' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 1；返回 1；await 0；调用 'lines.join'、'lines.push' |
| 1820–1881 | function | `SubscriptionGeneratorPage > insertProxiesIntoGroup` | 执行与 'insertProxiesIntoGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 7；循环 4；返回 2；await 0；调用 'groupMatch[<key>].trim'、'line.match'、'result.join'、'result.push'、'yamlStr.split' |
| 1884–1926 | function | `SubscriptionGeneratorPage > insertNewGroupsAfter` | 执行与 'insertNewGroupsAfter' 对应的前端业务、状态或数据转换逻辑。 | 分支 7；循环 2；返回 1；await 0；调用 'groupMatch[<key>].trim'、'line.match'、'result.join'、'result.push'、'result.splice'、'result[<key>].match'、'result[<key>].startsWith'、'yamlStr.split' |
| 1929–2086 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion` | 处理与 'handleAutoGroupByRegion' 对应的前端业务、状态或数据转换逻辑。 | 分支 20；循环 3；返回 2；await 0；调用 'Object.entries'、'existingGroupNames.has'、'extractRegionFromNodeName'、'findRegionGroupName'、'groups.find'、'groups.map'、'insertProxiesIntoGroup'、'newGroups.push'、'nodeNames.filter'、'nodes.filter'、'otherNodes.filter'、'otherNodes.push'、'preprocessYaml'、'regionNodes[<key>].push'、'selectedNodes.map'、'sortedEnabledNodes.filter'、'toast.e… |
| 1946–1946 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > sortedEnabledNodes.filter.callback#161` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 1947–1947 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > selectedNodes.map.callback#162` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1969–1969 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > groups.map.callback#163` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1972–1972 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > groups.find.callback#164` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1981–1981 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > groups.find.callback#165` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1983–1983 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > nodes.filter.callback#166` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'existingNodes.has' |
| 1992–1992 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > groups.find.callback#167` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1994–1994 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > otherNodes.filter.callback#168` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'existingNodes.has' |
| 2002–2002 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > nodeNames.filter.callback#169` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'existingAutoSelectNodes.has' |
| 2050–2050 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > newGroups.map.callback#170` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'generateProxyGroupYaml' |
| 2057–2057 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > groups.find.callback#171` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2059–2059 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > createdGroupNames.filter.callback#172` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'existingNodeSelectProxies.has' |
| 2070–2070 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > Object.entries.filter.callback#173` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2071–2071 | function | `SubscriptionGeneratorPage > handleAutoGroupByRegion > Object.entries.filter.map.callback#174` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2089–2101 | function | `SubscriptionGeneratorPage > handleRemoveProxy` | 处理与 'handleRemoveProxy' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyGroups' |
| 2090–2099 | function | `SubscriptionGeneratorPage > handleRemoveProxy > setProxyGroups.callback#176` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'groups.map' |
| 2091–2099 | function | `SubscriptionGeneratorPage > handleRemoveProxy > setProxyGroups.callback#176 > groups.map.callback#177` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'group.proxies.filter' |
| 2095–2095 | function | `SubscriptionGeneratorPage > handleRemoveProxy > setProxyGroups.callback#176 > groups.map.callback#177 > group.proxies.filter.callback#178` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2104–2115 | function | `SubscriptionGeneratorPage > handleRemoveGroup` | 处理与 'handleRemoveGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyGroups' |
| 2105–2114 | function | `SubscriptionGeneratorPage > handleRemoveGroup > setProxyGroups.callback#180` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'filteredGroups.map'、'groups.filter' |
| 2107–2107 | function | `SubscriptionGeneratorPage > handleRemoveGroup > setProxyGroups.callback#180 > groups.filter.callback#181` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2110–2113 | function | `SubscriptionGeneratorPage > handleRemoveGroup > setProxyGroups.callback#180 > filteredGroups.map.callback#182` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'group.proxies.filter' |
| 2112–2112 | function | `SubscriptionGeneratorPage > handleRemoveGroup > setProxyGroups.callback#180 > filteredGroups.map.callback#182 > group.proxies.filter.callback#183` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2118–2229 | function | `SubscriptionGeneratorPage > handleRenameGroup` | 处理与 'handleRenameGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 6；循环 0；返回 0；await 0；调用 'Array.isArray'、'console.error'、'ensureShortIdAsString'、'fixShortIdInYaml'、'parsedConfig[<key>].map'、'preprocessYaml'、'setClashConfig'、'setPendingConfigAfterGrouping'、'setProxyGroups'、'yaml.dump'、'yaml.load' |
| 2119–2132 | function | `SubscriptionGeneratorPage > handleRenameGroup > setProxyGroups.callback#185` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'groups.map' |
| 2121–2130 | function | `SubscriptionGeneratorPage > handleRenameGroup > setProxyGroups.callback#185 > groups.map.callback#186` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'group.proxies.map' |
| 2128–2128 | function | `SubscriptionGeneratorPage > handleRenameGroup > setProxyGroups.callback#185 > groups.map.callback#186 > group.proxies.map.callback#187` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 2140–2144 | function | `SubscriptionGeneratorPage > handleRenameGroup > parsedConfig[<key>].map.callback#188` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'group.proxies.map' |
| 2143–2143 | function | `SubscriptionGeneratorPage > handleRenameGroup > parsedConfig[<key>].map.callback#188 > group.proxies.map.callback#189` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 2149–2164 | function | `SubscriptionGeneratorPage > handleRenameGroup > parsedConfig[<key>].map.callback#190` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 3；await 0；调用 'parts.join'、'rule.split' |
| 2189–2193 | function | `SubscriptionGeneratorPage > handleRenameGroup > parsedConfig[<key>].map.callback#191` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'group.proxies.map' |
| 2192–2192 | function | `SubscriptionGeneratorPage > handleRenameGroup > parsedConfig[<key>].map.callback#191 > group.proxies.map.callback#192` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 2198–2211 | function | `SubscriptionGeneratorPage > handleRenameGroup > parsedConfig[<key>].map.callback#193` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 3；await 0；调用 'parts.join'、'rule.split' |
| 2232–2245 | function | `SubscriptionGeneratorPage > handleGroupDialogOpenChange` | 处理与 'handleGroupDialogOpenChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setGroupDialogOpen'、'setTimeout' |
| 2238–2241 | function | `SubscriptionGeneratorPage > handleGroupDialogOpenChange > setTimeout.callback#195` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setAllProxies'、'setProxyGroups' |
| 2279–2295 | function | `SubscriptionGeneratorPage > onClick.callback#196` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Array.from'、'Array.from.sort'、'currentIds.every'、'setSelectedNodeIds'、'setSelectedProtocols'、'setSelectedTags'、'sortedEnabledNodes.map' |
| 2281–2281 | function | `SubscriptionGeneratorPage > onClick.callback#196 > sortedEnabledNodes.map.callback#197` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2288–2288 | function | `SubscriptionGeneratorPage > onClick.callback#196 > currentIds.every.callback#198` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2299–2338 | function | `SubscriptionGeneratorPage > protocols.map.callback#199` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'protocol.toUpperCase'、'selectedProtocols.has'、'sortedEnabledNodes.filter' |
| 2300–2300 | function | `SubscriptionGeneratorPage > protocols.map.callback#199 > sortedEnabledNodes.filter.callback#200` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'n.protocol.toLowerCase' |
| 2307–2333 | function | `SubscriptionGeneratorPage > protocols.map.callback#199 > onClick.callback#201` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setSelectedNodeIds'、'setSelectedProtocols'、'setSelectedTags'、'sortedEnabledNodes.filter'、'sortedEnabledNodes.filter.map' |
| 2310–2310 | function | `SubscriptionGeneratorPage > protocols.map.callback#199 > onClick.callback#201 > sortedEnabledNodes.filter.callback#202` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'n.protocol.toLowerCase' |
| 2311–2311 | function | `SubscriptionGeneratorPage > protocols.map.callback#199 > onClick.callback#201 > sortedEnabledNodes.filter.map.callback#203` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2318–2322 | function | `SubscriptionGeneratorPage > protocols.map.callback#199 > onClick.callback#201 > setSelectedProtocols.callback#204` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'next.delete' |
| 2323–2327 | function | `SubscriptionGeneratorPage > protocols.map.callback#199 > onClick.callback#201 > setSelectedNodeIds.callback#205` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'protocolNodeIds.forEach' |
| 2325–2325 | function | `SubscriptionGeneratorPage > protocols.map.callback#199 > onClick.callback#201 > setSelectedNodeIds.callback#205 > protocolNodeIds.forEach.callback#206` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'next.delete' |
| 2330–2330 | function | `SubscriptionGeneratorPage > protocols.map.callback#199 > onClick.callback#201 > setSelectedProtocols.callback#207` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2331–2331 | function | `SubscriptionGeneratorPage > protocols.map.callback#199 > onClick.callback#201 > setSelectedNodeIds.callback#208` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2347–2363 | function | `SubscriptionGeneratorPage > onClick.callback#209` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Array.from'、'Array.from.sort'、'currentIds.every'、'setSelectedNodeIds'、'setSelectedProtocols'、'setSelectedTags'、'sortedEnabledNodes.map' |
| 2349–2349 | function | `SubscriptionGeneratorPage > onClick.callback#209 > sortedEnabledNodes.map.callback#210` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2356–2356 | function | `SubscriptionGeneratorPage > onClick.callback#209 > currentIds.every.callback#211` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2367–2406 | function | `SubscriptionGeneratorPage > tags.map.callback#212` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'selectedTags.has'、'sortedEnabledNodes.filter' |
| 2368–2368 | function | `SubscriptionGeneratorPage > tags.map.callback#212 > sortedEnabledNodes.filter.callback#213` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 '<ConditionalExpression>.includes' |
| 2375–2401 | function | `SubscriptionGeneratorPage > tags.map.callback#212 > onClick.callback#214` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setSelectedNodeIds'、'setSelectedProtocols'、'setSelectedTags'、'sortedEnabledNodes.filter'、'sortedEnabledNodes.filter.map' |
| 2378–2378 | function | `SubscriptionGeneratorPage > tags.map.callback#212 > onClick.callback#214 > sortedEnabledNodes.filter.callback#215` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 '<ConditionalExpression>.includes' |
| 2379–2379 | function | `SubscriptionGeneratorPage > tags.map.callback#212 > onClick.callback#214 > sortedEnabledNodes.filter.map.callback#216` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2386–2390 | function | `SubscriptionGeneratorPage > tags.map.callback#212 > onClick.callback#214 > setSelectedTags.callback#217` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'next.delete' |
| 2391–2395 | function | `SubscriptionGeneratorPage > tags.map.callback#212 > onClick.callback#214 > setSelectedNodeIds.callback#218` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'tagNodeIds.forEach' |
| 2393–2393 | function | `SubscriptionGeneratorPage > tags.map.callback#212 > onClick.callback#214 > setSelectedNodeIds.callback#218 > tagNodeIds.forEach.callback#219` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'next.delete' |
| 2398–2398 | function | `SubscriptionGeneratorPage > tags.map.callback#212 > onClick.callback#214 > setSelectedTags.callback#220` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2399–2399 | function | `SubscriptionGeneratorPage > tags.map.callback#212 > onClick.callback#214 > setSelectedNodeIds.callback#221` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2420–2423 | function | `SubscriptionGeneratorPage > onValueChange.callback#222` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'setNodeListPage'、'setNodeListPageSize' |
| 2429–2433 | function | `SubscriptionGeneratorPage > <ArrayLiteralExpression>.map.callback#223` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'String' |
| 2443–2443 | function | `SubscriptionGeneratorPage > onClick.callback#224` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeListPage' |
| 2453–2453 | function | `SubscriptionGeneratorPage > onClick.callback#225` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeListPage' |
| 2453–2453 | function | `SubscriptionGeneratorPage > onClick.callback#225 > setNodeListPage.callback#226` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Math.max' |
| 2458–2473 | function | `SubscriptionGeneratorPage > getPageNumbers.map.callback#227` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0 |
| 2469–2469 | function | `SubscriptionGeneratorPage > getPageNumbers.map.callback#227 > onClick.callback#228` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeListPage' |
| 2480–2480 | function | `SubscriptionGeneratorPage > onClick.callback#229` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeListPage' |
| 2480–2480 | function | `SubscriptionGeneratorPage > onClick.callback#229 > setNodeListPage.callback#230` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Math.min' |
| 2490–2490 | function | `SubscriptionGeneratorPage > onClick.callback#231` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeListPage' |
| 2502–2502 | function | `SubscriptionGeneratorPage > getRowKey.callback#232` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2505–2505 | function | `SubscriptionGeneratorPage > onRowClick.callback#233` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleToggleNode' |
| 2506–2506 | function | `SubscriptionGeneratorPage > rowClassName.callback#234` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 2512–2512 | function | `SubscriptionGeneratorPage > filteredNodes.every.callback#235` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 2516–2521 | function | `SubscriptionGeneratorPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 2519–2519 | function | `SubscriptionGeneratorPage > cell > onCheckedChange.callback#237` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleToggleNode' |
| 2526–2526 | function | `SubscriptionGeneratorPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 2531–2533 | function | `SubscriptionGeneratorPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'getProtocolColor'、'node.protocol.toUpperCase' |
| 2538–2552 | function | `SubscriptionGeneratorPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 1；await 0；调用 'JSON.parse' |
| 2557–2571 | function | `SubscriptionGeneratorPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0；调用 '<ConditionalExpression>.map' |
| 2559–2563 | function | `SubscriptionGeneratorPage > cell > <ConditionalExpression>.map.callback#242` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2577–2629 | function | `SubscriptionGeneratorPage > header` | 执行与 'header' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0；调用 '<ArrowFunction>'、'<ConditionalExpression>.map'、'getProtocolColor'、'node.protocol.toUpperCase'、'selectedNodeIds.has' |
| 2584–2584 | function | `SubscriptionGeneratorPage > header > onCheckedChange.callback#244` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleToggleNode' |
| 2595–2599 | function | `SubscriptionGeneratorPage > header > <ConditionalExpression>.map.callback#245` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2611–2625 | function | `SubscriptionGeneratorPage > header > <anonymous#246>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 1；await 0；调用 'JSON.parse' |
| 2645–2645 | function | `SubscriptionGeneratorPage > onClick.callback#247` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeListPage' |
| 2655–2655 | function | `SubscriptionGeneratorPage > onClick.callback#248` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeListPage' |
| 2655–2655 | function | `SubscriptionGeneratorPage > onClick.callback#248 > setNodeListPage.callback#249` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Math.max' |
| 2660–2675 | function | `SubscriptionGeneratorPage > getPageNumbers.map.callback#250` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0 |
| 2671–2671 | function | `SubscriptionGeneratorPage > getPageNumbers.map.callback#250 > onClick.callback#251` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeListPage' |
| 2682–2682 | function | `SubscriptionGeneratorPage > onClick.callback#252` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeListPage' |
| 2682–2682 | function | `SubscriptionGeneratorPage > onClick.callback#252 > setNodeListPage.callback#253` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Math.min' |
| 2692–2692 | function | `SubscriptionGeneratorPage > onClick.callback#254` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeListPage' |
| 2709–2709 | function | `SubscriptionGeneratorPage > onClick.callback#255` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setRuleMode' |
| 2716–2716 | function | `SubscriptionGeneratorPage > onClick.callback#256` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setRuleMode' |
| 2753–2757 | function | `SubscriptionGeneratorPage > allTemplates.map.callback#257` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2773–2773 | function | `SubscriptionGeneratorPage > onClick.callback#258` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTemplateManageDialogOpen' |
| 2782–2782 | function | `SubscriptionGeneratorPage > onClick.callback#259` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setOldTemplateManageDialogOpen' |
| 2792–2798 | function | `SubscriptionGeneratorPage > onClick.callback#260` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'toast.error' |
| 2832–2836 | function | `SubscriptionGeneratorPage > v3Templates.map.callback#261` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2852–2852 | function | `SubscriptionGeneratorPage > onClick.callback#262` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelectedV3Tags' |
| 2856–2875 | function | `SubscriptionGeneratorPage > tags.map.callback#263` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'selectedV3Tags.includes'、'sortedEnabledNodes.filter' |
| 2858–2858 | function | `SubscriptionGeneratorPage > tags.map.callback#263 > sortedEnabledNodes.filter.callback#264` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 '<ConditionalExpression>.includes' |
| 2864–2870 | function | `SubscriptionGeneratorPage > tags.map.callback#263 > onClick.callback#265` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setSelectedV3Tags' |
| 2866–2866 | function | `SubscriptionGeneratorPage > tags.map.callback#263 > onClick.callback#265 > setSelectedV3Tags.callback#266 > prev.filter.callback#267` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2866–2866 | function | `SubscriptionGeneratorPage > tags.map.callback#263 > onClick.callback#265 > setSelectedV3Tags.callback#266` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'prev.filter' |
| 2868–2868 | function | `SubscriptionGeneratorPage > tags.map.callback#263 > onClick.callback#265 > setSelectedV3Tags.callback#268` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2897–2901 | function | `SubscriptionGeneratorPage > onClick.callback#269` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'toast.error' |
| 2941–2941 | function | `SubscriptionGeneratorPage > onChange.callback#270` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTrafficLimit' |
| 2951–2969 | function | `SubscriptionGeneratorPage > probeServers.map.callback#271` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'statsServerIds.includes' |
| 2958–2964 | function | `SubscriptionGeneratorPage > probeServers.map.callback#271 > onClick.callback#272` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setStatsServerIds' |
| 2959–2962 | function | `SubscriptionGeneratorPage > probeServers.map.callback#271 > onClick.callback#272 > setStatsServerIds.callback#273` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'prev.filter' |
| 2961–2961 | function | `SubscriptionGeneratorPage > probeServers.map.callback#271 > onClick.callback#272 > setStatsServerIds.callback#273 > prev.filter.callback#274` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3066–3066 | function | `SubscriptionGeneratorPage > onChange.callback#275` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSubscribeName' |
| 3075–3075 | function | `SubscriptionGeneratorPage > onChange.callback#276` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSubscribeFilename' |
| 3087–3087 | function | `SubscriptionGeneratorPage > onChange.callback#277` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSubscribeDescription' |
| 3093–3093 | function | `SubscriptionGeneratorPage > onClick.callback#278` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSaveDialogOpen' |
| 3112–3112 | function | `SubscriptionGeneratorPage > savedNodes.filter.callback#279` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 3130–3130 | function | `SubscriptionGeneratorPage > savedNodes.filter.callback#280` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 3153–3157 | function | `SubscriptionGeneratorPage > missingNodes.map.callback#281` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3166–3166 | function | `SubscriptionGeneratorPage > onClick.callback#282` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setReplacementChoice' |
| 3173–3173 | function | `SubscriptionGeneratorPage > onClick.callback#283` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setReplacementChoice' |
| 3178–3195 | function | `SubscriptionGeneratorPage > <anonymous#284>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 2；await 0；调用 'parsedConfig[<key>].map'、'preprocessYaml'、'proxyGroupNames.map'、'yaml.load' |
| 3181–3181 | function | `SubscriptionGeneratorPage > <anonymous#284> > parsedConfig[<key>].map.callback#285` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3182–3191 | function | `SubscriptionGeneratorPage > <anonymous#284> > proxyGroupNames.map.callback#286` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 3186–3186 | function | `SubscriptionGeneratorPage > <anonymous#284> > proxyGroupNames.map.callback#286 > onClick.callback#287` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setReplacementChoice' |
| 3204–3204 | function | `SubscriptionGeneratorPage > onClick.callback#288` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMissingNodesDialogOpen' |
| 3236–3238 | function | `SubscriptionGeneratorPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 3242–3246 | function | `SubscriptionGeneratorPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'template.rule_source.split'、'template.rule_source.split.pop' |
| 3250–3285 | function | `SubscriptionGeneratorPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 3255–3255 | function | `SubscriptionGeneratorPage > cell > onClick.callback#292` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handlePreviewSource' |
| 3263–3263 | function | `SubscriptionGeneratorPage > cell > onClick.callback#293` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handlePreviewTemplate' |
| 3271–3271 | function | `SubscriptionGeneratorPage > cell > onClick.callback#294` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleEditTemplate' |
| 3279–3279 | function | `SubscriptionGeneratorPage > cell > onClick.callback#295` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleDeleteTemplate' |
| 3289–3289 | function | `SubscriptionGeneratorPage > getRowKey.callback#296` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3317–3318 | function | `SubscriptionGeneratorPage > onChange.callback#297` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTemplateFormData' |
| 3323–3355 | function | `SubscriptionGeneratorPage > <anonymous#298>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'available.acl4ssr.map'、'available.aethersailor.map'、'getAvailablePresets' |
| 3335–3339 | function | `SubscriptionGeneratorPage > <anonymous#298> > available.aethersailor.map.callback#299` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3345–3349 | function | `SubscriptionGeneratorPage > <anonymous#298> > available.acl4ssr.map.callback#300` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3366–3367 | function | `SubscriptionGeneratorPage > onChange.callback#301` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTemplateFormData' |
| 3385–3386 | function | `SubscriptionGeneratorPage > onCheckedChange.callback#302` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTemplateFormData' |
| 3393–3393 | function | `SubscriptionGeneratorPage > onClick.callback#303` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setIsTemplateFormDialogOpen' |
| 3421–3421 | function | `SubscriptionGeneratorPage > onClick.callback#304` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'deleteTemplateMutation.mutate' |
| 3501–3503 | function | `SubscriptionGeneratorPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 3507–3526 | function | `SubscriptionGeneratorPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 3512–3512 | function | `SubscriptionGeneratorPage > cell > onClick.callback#307` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleRenameOldTemplate' |
| 3520–3520 | function | `SubscriptionGeneratorPage > cell > onClick.callback#308` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleDeleteOldTemplate' |
| 3530–3530 | function | `SubscriptionGeneratorPage > getRowKey.callback#309` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3556–3556 | function | `SubscriptionGeneratorPage > onChange.callback#310` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setOldTemplateContent' |
| 3563–3563 | function | `SubscriptionGeneratorPage > onClick.callback#311` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setOldTemplateEditDialogOpen' |
| 3588–3588 | function | `SubscriptionGeneratorPage > onClick.callback#312` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'deleteOldTemplateMutation.mutate' |
| 3613–3613 | function | `SubscriptionGeneratorPage > onChange.callback#313` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNewOldTemplateName' |
| 3621–3621 | function | `SubscriptionGeneratorPage > onClick.callback#314` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setIsOldTemplateRenameDialogOpen' |

## `routes/index.tsx`

依赖：`react`、`@tanstack/react-query`、`@tanstack/react-router`、`recharts`、`lucide-react`、`@/components/layout/topbar`、`@/lib/api`、`@/stores/auth-store`、`@/components/ui/card`、`@/components/ui/progress`、`@/components/ui/skeleton`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 29–37 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 30–35 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 39–224 | function | `DashboardPage` | 渲染并协调 'DashboardPage' React 组件的状态、数据请求和用户交互。 | 分支 4；循环 0；返回 1；await 0；调用 'Array.from'、'Array.from.map'、'Boolean'、'cards.map'、'useAuthStore'、'useMemo'、'useQuery' |
| 43–47 | function | `DashboardPage > useMemo.callback#3` | 封装 'useMemo.callback#3' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 53–56 | function | `DashboardPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 62–62 | function | `DashboardPage > useMemo.callback#5` | 封装 'useMemo.callback#5' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 65–91 | function | `DashboardPage > useMemo.callback#6` | 封装 'useMemo.callback#6' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'formatMetric'、'formatPercentage' |
| 95–101 | function | `DashboardPage > useMemo.callback#7` | 封装 'useMemo.callback#7' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 '<BinaryExpression>.map' |
| 96–100 | function | `DashboardPage > useMemo.callback#7 > <BinaryExpression>.map.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'item.date.slice' |
| 112–127 | function | `DashboardPage > Array.from.map.callback#9` | 渲染并协调 'Array.from.map.callback#9' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 128–149 | function | `DashboardPage > cards.map.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Math.max'、'Math.min'、'Number.isNaN'、'numberFormatter.format' |
| 191–191 | function | `DashboardPage > tickFormatter.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'numberFormatter.format' |
| 197–197 | function | `DashboardPage > labelFormatter.callback#12 > chartData.find.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 197–197 | function | `DashboardPage > labelFormatter.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'chartData.find' |
| 198–198 | function | `DashboardPage > formatter.callback#14` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'numberFormatter.format' |
| 226–237 | function | `formatMetric` | 格式化与 'formatMetric' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'formatter.format' |
| 239–242 | function | `formatPercentage` | 格式化与 'formatPercentage' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'formatter.format' |

## `routes/login.tsx`

依赖：`react`、`react-hook-form`、`@tanstack/react-query`、`@tanstack/react-router`、`sonner`、`lucide-react`、`@/lib/api`、`@/stores/auth-store`、`@/components/ui/card`、`@/components/ui/input`、`@/components/ui/label`、`@/components/ui/button`、`@/components/ui/checkbox`、`@/components/ui/input-otp`、`@/lib/handle-server-error`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 28–36 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 29–34 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 38–42 | type | `LoginFormValues` | 定义 'LoginFormValues' 的数据契约、联合类型或组件属性。 |  |
| 44–50 | type | `SetupFormValues` | 定义 'SetupFormValues' 的数据契约、联合类型或组件属性。 |  |
| 52–81 | function | `LoginPage` | 渲染并协调 'LoginPage' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 3；await 0；调用 'useQuery' |
| 56–59 | function | `LoginPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 83–91 | type | `LoginResponse` | 定义 'LoginResponse' 的数据契约、联合类型或组件属性。 |  |
| 93–110 | function | `handleLoginSuccess` | 处理与 'handleLoginSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'auth.setAccessToken'、'navigate'、'queryClient.invalidateQueries'、'queryClient.setQueryData'、'toast.success' |
| 112–221 | function | `LoginView` | 渲染并协调 'LoginView' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 2；await 0；调用 'form.handleSubmit'、'form.register'、'form.watch'、'useAuthStore'、'useForm'、'useMutation'、'useNavigate'、'useQuery'、'useQueryClient'、'useState' |
| 120–120 | function | `LoginView > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get' |
| 131–134 | function | `LoginView > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 135–142 | function | `LoginView > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'form.reset'、'handleLoginSuccess'、'setTwoFactorToken' |
| 143–146 | function | `LoginView > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 149–155 | function | `LoginView > form.handleSubmit.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'login.mutate'、'toast.error' |
| 161–161 | function | `LoginView > onBack.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTwoFactorToken' |
| 162–162 | function | `LoginView > onSuccess.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleLoginSuccess' |
| 204–204 | function | `LoginView > onCheckedChange.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'form.setValue' |
| 223–254 | function | `TurnstileWidget` | 渲染并协调 'TurnstileWidget' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'useEffect'、'useRef' |
| 225–252 | function | `TurnstileWidget > useEffect.callback#15` | 封装 'useEffect.callback#15' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'document.createElement'、'document.head.appendChild'、'document.querySelector'、'existing.addEventListener'、'render'、'script.addEventListener' |
| 226–237 | function | `TurnstileWidget > useEffect.callback#15 > render` | 渲染与 'render' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'turnstile.render' |
| 233–233 | function | `TurnstileWidget > useEffect.callback#15 > render > expired-callback` | 执行与 'expired-callback' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onToken' |
| 234–234 | function | `TurnstileWidget > useEffect.callback#15 > render > error-callback` | 执行与 'error-callback' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onToken' |
| 251–251 | function | `TurnstileWidget > useEffect.callback#15 > <anonymous#19>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onToken' |
| 256–390 | function | `TwoFactorStep` | 渲染并协调 'TwoFactorStep' React 组件的状态、数据请求和用户交互。 | 分支 5；循环 0；返回 1；await 0；调用 'recoveryCode.trim'、'useMutation'、'useState' |
| 270–276 | function | `TwoFactorStep > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 277–277 | function | `TwoFactorStep > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onSuccess' |
| 278–282 | function | `TwoFactorStep > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'setOtpCode'、'toast.error' |
| 286–292 | function | `TwoFactorStep > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 293–296 | function | `TwoFactorStep > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onSuccess'、'toast.success' |
| 297–300 | function | `TwoFactorStep > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 317–317 | function | `TwoFactorStep > onChange.callback#27` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setRecoveryCode' |
| 320–324 | function | `TwoFactorStep > onKeyDown.callback#28` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'recoveryCode.trim'、'verifyRecovery.mutate' |
| 328–328 | function | `TwoFactorStep > onClick.callback#29` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'recoveryCode.trim'、'verifyRecovery.mutate' |
| 341–341 | function | `TwoFactorStep > onComplete.callback#30` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'verify2FA.mutate' |
| 358–358 | function | `TwoFactorStep > onClick.callback#31` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'verify2FA.mutate' |
| 377–381 | function | `TwoFactorStep > onClick.callback#32` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setOtpCode'、'setRecoveryCode'、'setUseRecovery' |
| 392–567 | function | `InitialSetupView` | 渲染并协调 'InitialSetupView' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'form.handleSubmit'、'form.register'、'useForm'、'useMutation'、'useQueryClient'、'useState' |
| 406–413 | function | `InitialSetupView > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 414–418 | function | `InitialSetupView > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'form.reset'、'queryClient.invalidateQueries'、'toast.success' |
| 419–422 | function | `InitialSetupView > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 426–432 | function | `InitialSetupView > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'api.post'、'formData.append' |
| 433–441 | function | `InitialSetupView > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setBackupFile'、'setTimeout'、'toast.success' |
| 438–440 | function | `InitialSetupView > onSuccess > setTimeout.callback#39` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'window.location.reload' |
| 442–445 | function | `InitialSetupView > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 448–450 | function | `InitialSetupView > form.handleSubmit.callback#41` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setup.mutate' |
| 545–545 | function | `InitialSetupView > onChange.callback#42` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBackupFile' |
| 550–550 | function | `InitialSetupView > onClick.callback#43` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'restoreBackup.mutate' |

## `routes/logs.tsx`

依赖：`react`、`@tanstack/react-query`、`@tanstack/react-router`、`lucide-react`、`sonner`、`@/components/layout/topbar`、`@/components/ui/badge`、`@/components/ui/button`、`@/components/ui/card`、`@/components/ui/input`、`@/components/ui/tabs`、`@/lib/api`、`@/lib/profile`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 16–27 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 17–25 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 1；调用 'context.queryClient.fetchQuery'、'redirect' |
| 29–45 | function | `LogsPage` | 渲染并协调 'LogsPage' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 47–60 | function | `SecurityPanel` | 渲染并协调 'SecurityPanel' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'bans.data.map'、'events.data.map'、'useMutation'、'useQuery'、'useQueryClient'、'useState' |
| 50–50 | function | `SecurityPanel > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get' |
| 51–51 | function | `SecurityPanel > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get' |
| 52–52 | function | `SecurityPanel > refresh > client.invalidateQueries.then.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'client.invalidateQueries' |
| 52–52 | function | `SecurityPanel > refresh` | 执行与 'refresh' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'client.invalidateQueries'、'client.invalidateQueries.then' |
| 53–53 | function | `SecurityPanel > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'api.post' |
| 53–53 | function | `SecurityPanel > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'refresh'、'setIP'、'toast.success' |
| 54–54 | function | `SecurityPanel > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'api.delete'、'encodeURIComponent' |
| 54–54 | function | `SecurityPanel > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'refresh'、'toast.success' |
| 56–56 | function | `SecurityPanel > onChange.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setIP' |
| 56–56 | function | `SecurityPanel > onClick.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'ban.mutate' |
| 56–56 | function | `SecurityPanel > onClick.callback#14` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'ban.mutate' |
| 57–57 | function | `SecurityPanel > bans.data.map.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'formatTime' |
| 57–57 | function | `SecurityPanel > bans.data.map.callback#15 > onClick.callback#16` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'unban.mutate' |
| 58–58 | function | `SecurityPanel > events.data.map.callback#17` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'formatTime' |
| 62–65 | function | `TaskPanel` | 渲染并协调 'TaskPanel' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'runs.data.map'、'useQuery' |
| 63–63 | function | `TaskPanel > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get' |
| 64–64 | function | `TaskPanel > onClick.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'runs.refetch' |
| 64–64 | function | `TaskPanel > runs.data.map.callback#21` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'formatTime' |
| 67–70 | function | `OperationPanel` | 渲染并协调 'OperationPanel' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'logs.data.map'、'useQuery' |
| 68–68 | function | `OperationPanel > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get' |
| 69–69 | function | `OperationPanel > logs.data.map.callback#25` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'formatTime' |
| 69–69 | function | `OperationPanel > onClick.callback#24` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'logs.refetch' |
| 72–72 | function | `formatTime` | 格式化与 'formatTime' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 '<NewExpression>.toLocaleString' |

## `routes/nodes.index.tsx`

依赖：`react`、`react-dom`、`@tanstack/react-router`、`@tanstack/react-query`、`sonner`、`@/components/layout/topbar`、`@/stores/auth-store`、`@/lib/api`、`@/lib/utils`、`@/components/ui/button`、`@/components/ui/textarea`、`@/components/ui/input`、`@/components/ui/label`、`@/components/ui/select`、`@/components/ui/card`、`@/components/ui/collapsible`、`@/components/ui/table`、`@/components/ui/switch`、`@/components/ui/badge`、`@/components/ui/checkbox`、`@/components/ui/dialog`、`@/components/ui/alert-dialog`、`@/components/ui/tooltip`、`@/components/ui/tabs`、`@/lib/proxy-types`、`js-yaml`、`lucide-react`、`@/components/ui/dropdown-menu`、`@/assets/icons/ip.svg`、`@/assets/icons/125.svg`、`@/assets/icons/250.svg`、`@/assets/icons/exchange.svg`、`@/lib/substore/producers/uri`、`@/lib/country-flag`、`@/components/twemoji`、`@/components/flag-emoji-picker`、`@/hooks/use-media-query`、`@/components/speedtest-dialog`、`@dnd-kit/core`、`@dnd-kit/sortable`、`@tanstack/react-virtual`、`@/components/external-sync-node-dialog`、`@/hooks/use-external-sync-selection`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–12 | const | `CLASH_DRAFT_KEY_PREFIX` | 保存 'CLASH_DRAFT_KEY_PREFIX' 的模块级常量、配置、路由或预计算值。 |  |
| 65–76 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 66–68 | function | `validateSearch` | 校验与 'validateSearch' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 69–74 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 78–95 | type | `ParsedNode` | 定义 'ParsedNode' 的数据契约、联合类型或组件属性。 |  |
| 97–109 | type | `TempNode` | 定义 'TempNode' 的数据契约、联合类型或组件属性。 |  |
| 111–123 | const | `PROTOCOL_COLORS` | 保存 'PROTOCOL_COLORS' 的模块级常量、配置、路由或预计算值。 |  |
| 125–125 | const | `PROTOCOLS` | 保存 'PROTOCOLS' 的模块级常量、配置、路由或预计算值。 |  |
| 128–140 | function | `isIpAddress` | 判断与 'isIpAddress' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'hostname.replace'、'ipv4Regex.test'、'ipv6Regex.test' |
| 143–164 | function | `reorderProxyConfig` | 执行与 'reorderProxyConfig' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 2；返回 2；await 0；调用 'Object.entries'、'priorityKeys.includes' |
| 167–188 | function | `DragHandle` | 渲染并协调 'DragHandle' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'cn'、'useSortable' |
| 191–198 | interface | `SortableTableRowProps` | 定义 'SortableTableRowProps' 的数据契约、联合类型或组件属性。 |  |
| 200–238 | const | `SortableTableRow` | 保存 'SortableTableRow' 的模块级常量、配置、路由或预计算值。 |  |
| 200–238 | function | `SortableTableRow` | 渲染并协调 'SortableTableRow' React 组件的状态、数据请求和用户交互。 | 分支 5；循环 0；返回 1；await 0；调用 'Boolean'、'cn'、'useSortable' |
| 209–209 | function | `SortableTableRow > animateLayoutChanges` | 执行与 'animateLayoutChanges' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 241–248 | interface | `SortableCardProps` | 定义 'SortableCardProps' 的数据契约、联合类型或组件属性。 |  |
| 250–288 | const | `SortableCard` | 保存 'SortableCard' 的模块级常量、配置、路由或预计算值。 |  |
| 250–288 | function | `SortableCard` | 渲染并协调 'SortableCard' React 组件的状态、数据请求和用户交互。 | 分支 5；循环 0；返回 1；await 0；调用 'Boolean'、'cn'、'useSortable' |
| 259–259 | function | `SortableCard > animateLayoutChanges` | 执行与 'animateLayoutChanges' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 291–296 | interface | `SortableTagButtonProps` | 定义 'SortableTagButtonProps' 的数据契约、联合类型或组件属性。 |  |
| 298–331 | const | `SortableTagButton` | 保存 'SortableTagButton' 的模块级常量、配置、路由或预计算值。 |  |
| 298–331 | function | `SortableTagButton` | 渲染并协调 'SortableTagButton' React 组件的状态、数据请求和用户交互。 | 分支 3；循环 0；返回 1；await 0；调用 'useSortable' |
| 334–378 | function | `DragOverlayContent` | 渲染并协调 'DragOverlayContent' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 3；await 0；调用 'firstNode.parsed.type.toUpperCase'、'node.parsed.type.toUpperCase' |
| 381–381 | const | `STORAGE_KEY_PROTOCOL` | 保存 'STORAGE_KEY_PROTOCOL' 的模块级常量、配置、路由或预计算值。 |  |
| 382–382 | const | `STORAGE_KEY_TAG` | 保存 'STORAGE_KEY_TAG' 的模块级常量、配置、路由或预计算值。 |  |
| 383–383 | const | `STORAGE_KEY_SELECTED_IDS` | 保存 'STORAGE_KEY_SELECTED_IDS' 的模块级常量、配置、路由或预计算值。 |  |
| 384–384 | const | `STORAGE_KEY_RENDER_MODE` | 保存 'STORAGE_KEY_RENDER_MODE' 的模块级常量、配置、路由或预计算值。 |  |
| 385–385 | const | `STORAGE_KEY_PAGE_SIZE` | 保存 'STORAGE_KEY_PAGE_SIZE' 的模块级常量、配置、路由或预计算值。 |  |
| 386–386 | const | `NODE_PAGE_SIZE_OPTIONS` | 保存 'NODE_PAGE_SIZE_OPTIONS' 的模块级常量、配置、路由或预计算值。 |  |
| 388–398 | function | `getStoredNodePageSize` | 读取或计算与 'getStoredNodePageSize' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'NODE_PAGE_SIZE_OPTIONS.includes'、'Number'、'localStorage.getItem' |
| 401–509 | function | `NodeListPagination` | 渲染并协调 'NodeListPagination' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 2；await 0；调用 'NODE_PAGE_SIZE_OPTIONS.map'、'String'、'cn'、'getPageNumbers'、'getPageNumbers.map' |
| 431–431 | function | `NodeListPagination > onValueChange.callback#14` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'onPageSizeChange' |
| 437–441 | function | `NodeListPagination > NODE_PAGE_SIZE_OPTIONS.map.callback#15` | 渲染并协调 'NODE_PAGE_SIZE_OPTIONS.map.callback#15' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0；调用 'String' |
| 454–454 | function | `NodeListPagination > onClick.callback#16` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onPageChange' |
| 464–464 | function | `NodeListPagination > onClick.callback#17` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Math.max'、'onPageChange' |
| 469–484 | function | `NodeListPagination > getPageNumbers.map.callback#18` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0 |
| 480–480 | function | `NodeListPagination > getPageNumbers.map.callback#18 > onClick.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onPageChange' |
| 491–491 | function | `NodeListPagination > onClick.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Math.min'、'onPageChange' |
| 501–501 | function | `NodeListPagination > onClick.callback#21` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onPageChange' |
| 512–521 | function | `getStoredFilterState` | 读取或计算与 'getStoredFilterState' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 2；await 0；调用 'localStorage.getItem' |
| 524–533 | function | `getStoredSelectedIds` | 读取或计算与 'getStoredSelectedIds' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'JSON.parse'、'localStorage.getItem' |
| 536–536 | type | `RenderMode` | 定义 'RenderMode' 的数据契约、联合类型或组件属性。 |  |
| 537–545 | function | `getStoredRenderMode` | 读取或计算与 'getStoredRenderMode' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'localStorage.getItem' |
| 547–7001 | function | `NodesPage` | 渲染并协调 'NodesPage' React 组件的状态、数据请求和用户交互。 | 分支 42；循环 0；返回 1；await 0；调用 'Boolean'、'Math.ceil'、'Math.max'、'useAuthStore'、'useCallback'、'useDeferredValue'、'useEffect'、'useExternalSyncSelection'、'useMediaQuery'、'useMemo'、'useMutation'、'useQuery'、'useQueryClient'、'useRef'、'useSearch'、'useSensor'、'useSensors'、'useState' |
| 565–571 | function | `NodesPage > useMemo.callback#26` | 封装 'useMemo.callback#26' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 1；返回 2；await 0；调用 '<RegularExpressionLiteral>.test'、'input.split' |
| 572–572 | function | `NodesPage > useMemo.callback#27` | 封装 'useMemo.callback#27' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 '<RegularExpressionLiteral>.test' |
| 577–577 | function | `NodesPage > useState.callback#28` | 封装 'useState.callback#28' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'getStoredFilterState' |
| 579–579 | function | `NodesPage > useState.callback#29` | 封装 'useState.callback#29' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'getStoredFilterState' |
| 597–601 | function | `NodesPage > useState.callback#30` | 封装 'useState.callback#30' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'localStorage.getItem' |
| 616–616 | function | `NodesPage > useState.callback#31` | 封装 'useState.callback#31' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'getStoredRenderMode' |
| 617–617 | function | `NodesPage > useState.callback#32` | 封装 'useState.callback#32' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'getStoredRenderMode' |
| 619–619 | function | `NodesPage > useState.callback#33` | 封装 'useState.callback#33' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'getStoredNodePageSize' |
| 626–626 | function | `NodesPage > useState.callback#34` | 封装 'useState.callback#34' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'getStoredSelectedIds' |
| 649–650 | function | `NodesPage > useState.callback#35` | 封装 'useState.callback#35' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'localStorage.getItem' |
| 684–686 | function | `NodesPage > useCallback.callback#36` | 封装 'useCallback.callback#36' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setUserAgent' |
| 688–690 | function | `NodesPage > useCallback.callback#37` | 封装 'useCallback.callback#37' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setCustomUserAgent' |
| 692–694 | function | `NodesPage > useCallback.callback#38` | 封装 'useCallback.callback#38' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setSubscriptionUrl' |
| 697–707 | function | `NodesPage > useCallback.callback#39` | 封装 'useCallback.callback#39' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelectedNodeIds' |
| 698–706 | function | `NodesPage > useCallback.callback#39 > setSelectedNodeIds.callback#40` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'newSet.add'、'newSet.delete'、'newSet.has' |
| 710–718 | function | `NodesPage > useCallback.callback#41` | 封装 'useCallback.callback#41' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'handleNodeSelect'、'target.closest' |
| 734–744 | function | `NodesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 749–753 | function | `NodesPage > useEffect.callback#43` | 封装 'useEffect.callback#43' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setNodeOrder' |
| 756–760 | function | `NodesPage > useEffect.callback#44` | 封装 'useEffect.callback#44' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'localStorage.setItem' |
| 762–766 | function | `NodesPage > useEffect.callback#45` | 封装 'useEffect.callback#45' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'localStorage.setItem' |
| 769–774 | function | `NodesPage > useEffect.callback#46` | 封装 'useEffect.callback#46' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setManualTag'、'setSubscriptionTag' |
| 777–781 | function | `NodesPage > useEffect.callback#47` | 封装 'useEffect.callback#47' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'Array.from'、'JSON.stringify'、'localStorage.setItem' |
| 784–788 | function | `NodesPage > useEffect.callback#48` | 封装 'useEffect.callback#48' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'localStorage.setItem' |
| 791–795 | function | `NodesPage > useEffect.callback#49` | 封装 'useEffect.callback#49' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'String'、'localStorage.setItem' |
| 797–801 | function | `NodesPage > useEffect.callback#50` | 封装 'useEffect.callback#50' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'String'、'localStorage.setItem' |
| 804–813 | function | `NodesPage > useEffect.callback#51` | 封装 'useEffect.callback#51' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setImportTab'、'setIsInputCardExpanded'、'setTimeout' |
| 809–811 | function | `NodesPage > useEffect.callback#51 > setTimeout.callback#52` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'subscriptionUrlInputRef.current.focus' |
| 828–833 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.put' |
| 834–836 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'queryClient.invalidateQueries' |
| 837–839 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 843–850 | function | `NodesPage > useCallback.callback#56` | 封装 'useCallback.callback#56' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'clearTimeout'、'setNodeOrder'、'setTimeout' |
| 846–849 | function | `NodesPage > useCallback.callback#56 > setTimeout.callback#57` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateNodeOrderMutation.mutate' |
| 853–859 | function | `NodesPage > useEffect.callback#58` | 封装 'useEffect.callback#58' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 854–858 | function | `NodesPage > useEffect.callback#58 > <anonymous#59>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'clearTimeout' |
| 864–873 | function | `NodesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 882–885 | function | `NodesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 889–889 | function | `NodesPage > useMemo.callback#62` | 封装 'useMemo.callback#62' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 890–894 | function | `NodesPage > useMemo.callback#63` | 封装 'useMemo.callback#63' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 1；返回 1；await 0；调用 'map.set' |
| 897–908 | function | `NodesPage > useEffect.callback#64` | 封装 'useEffect.callback#64' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'savedNodes.map'、'setSelectedNodeIds' |
| 899–899 | function | `NodesPage > useEffect.callback#64 > savedNodes.map.callback#65` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 900–907 | function | `NodesPage > useEffect.callback#64 > setSelectedNodeIds.callback#66` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'Array.from'、'Array.from.filter' |
| 901–901 | function | `NodesPage > useEffect.callback#64 > setSelectedNodeIds.callback#66 > Array.from.filter.callback#67` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'validIds.has' |
| 911–916 | function | `NodesPage > useEffect.callback#68` | 封装 'useEffect.callback#68' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'setRenderMode'、'setRenderModeInitialized' |
| 918–929 | function | `NodesPage > updateConfigName` | 更新与 'updateConfigName' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 3；await 0；调用 'JSON.parse'、'JSON.stringify' |
| 931–939 | function | `NodesPage > cloneProxyWithName` | 执行与 'cloneProxyWithName' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0 |
| 942–961 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 1；调用 'api.put'、'savedNodes.find'、'updateConfigName' |
| 943–943 | function | `NodesPage > mutationFn > savedNodes.find.callback#72` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 962–966 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setEditingNode'、'toast.success' |
| 967–969 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 976–979 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get'、'encodeURIComponent' |
| 980–983 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setResolvingIpFor'、'toast.error' |
| 988–991 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put' |
| 992–997 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setIpMenuState'、'setResolvingIpFor'、'toast.success' |
| 998–1001 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setResolvingIpFor'、'toast.error' |
| 1006–1009 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put' |
| 1010–1013 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 1014–1016 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1021–1026 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put' |
| 1027–1033 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'localStorage.removeItem'、'queryClient.invalidateQueries'、'setClashDialogOpen'、'toast.success' |
| 1034–1036 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1041–1046 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put' |
| 1047–1052 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setProbeBindingDialogOpen'、'setSelectedNodeForProbe'、'toast.success' |
| 1053–1055 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1059–1106 | function | `NodesPage > useCallback.callback#89` | 封装 'useCallback.callback#89' Hook 的响应式状态、副作用和复用逻辑。 | 分支 8；循环 0；返回 1；await 0；调用 'JSON.parse'、'JSON.stringify'、'dumpYAML'、'localStorage.getItem'、'localStorage.removeItem'、'reorderProxyConfig'、'setClashConfigError'、'setClashDialogOpen'、'setEditingClashConfig'、'setIsClashDraftRecoveryOpen'、'setJsonErrorLines' |
| 1109–1133 | function | `NodesPage > handleSaveClashConfig` | 处理与 'handleSaveClashConfig' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 2；await 0；调用 'JSON.parse'、'JSON.stringify'、'String'、'parseYAML'、'setClashConfigError'、'updateClashConfigMutation.mutate' |
| 1136–1181 | function | `NodesPage > handleClashConfigChange` | 处理与 'handleClashConfigChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 7；循环 0；返回 1；await 0；调用 'JSON.parse'、'String'、'errorMsg.includes'、'errorMsg.match'、'parseInt'、'parseYAML'、'setClashConfigError'、'setEditingClashConfig'、'setJsonErrorLines'、'value.substring'、'value.substring.split' |
| 1184–1191 | function | `NodesPage > useEffect.callback#92` | 封装 'useEffect.callback#92' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'Date.now'、'JSON.stringify'、'localStorage.setItem' |
| 1193–1208 | function | `NodesPage > handleRecoverClashDraft` | 处理与 'handleRecoverClashDraft' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 1；await 0；调用 'JSON.parse'、'String'、'parseYAML'、'setClashConfigError'、'setEditingClashConfig'、'setIsClashDraftRecoveryOpen'、'setJsonErrorLines' |
| 1210–1216 | function | `NodesPage > handleDiscardClashDraft` | 处理与 'handleDiscardClashDraft' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'localStorage.removeItem'、'setIsClashDraftRecoveryOpen' |
| 1219–1247 | function | `NodesPage > handleConfigFormatChange` | 处理与 'handleConfigFormatChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 2；await 0；调用 'JSON.parse'、'JSON.stringify'、'dumpYAML'、'localStorage.setItem'、'parseYAML'、'reorderProxyConfig'、'setClashConfigError'、'setConfigFormat'、'setEditingClashConfig'、'setJsonErrorLines' |
| 1250–1273 | function | `NodesPage > useCallback.callback#96` | 封装 'useCallback.callback#96' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 1；await 1；调用 'JSON.parse'、'String'、'URI_Producer'、'navigator.clipboard.writeText'、'producer.produce'、'setUriContent'、'setUriDialogOpen'、'toast.error'、'toast.success' |
| 1276–1315 | function | `NodesPage > handleTcping` | 处理与 'handleTcping' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 1；调用 'String'、'api.post'、'setTcpingNodeId'、'setTcpingResults' |
| 1281–1284 | function | `NodesPage > handleTcping > setTcpingResults.callback#98` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1293–1301 | function | `NodesPage > handleTcping > setTcpingResults.callback#99` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1303–1311 | function | `NodesPage > handleTcping > setTcpingResults.callback#100` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 1321–1395 | function | `NodesPage > handleBatchTcping` | 处理与 'handleBatchTcping' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 2；await 1；调用 'api.post'、'deferredFilteredNodes.filter'、'result.data.filter'、'selectedNodes.forEach'、'selectedNodes.map'、'setBatchTcpingLoading'、'setTcpingResults'、'toast.error'、'toast.success' |
| 1329–1329 | function | `NodesPage > handleBatchTcping > deferredFilteredNodes.filter.callback#102` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 1341–1344 | function | `NodesPage > handleBatchTcping > selectedNodes.forEach.callback#103` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'String' |
| 1345–1345 | function | `NodesPage > handleBatchTcping > setTcpingResults.callback#104` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1349–1353 | function | `NodesPage > handleBatchTcping > selectedNodes.map.callback#105` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1359–1368 | function | `NodesPage > handleBatchTcping > selectedNodes.forEach.callback#106` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'String' |
| 1369–1369 | function | `NodesPage > handleBatchTcping > setTcpingResults.callback#107` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1372–1372 | function | `NodesPage > handleBatchTcping > result.data.filter.callback#108` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1381–1389 | function | `NodesPage > handleBatchTcping > selectedNodes.forEach.callback#109` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'String' |
| 1390–1390 | function | `NodesPage > handleBatchTcping > setTcpingResults.callback#110` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1398–1434 | function | `NodesPage > handleResolveIp` | 处理与 'handleResolveIp' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 2；await 1；调用 'String'、'resolveIpMutation.mutateAsync'、'setIpMenuState'、'setResolvingIpFor'、'toast.error'、'updateNodeServerMutation.mutate'、'updateTempNodeServer' |
| 1437–1458 | function | `NodesPage > updateTempNodeServer` | 更新与 'updateTempNodeServer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setTempNodes'、'toast.success' |
| 1438–1456 | function | `NodesPage > updateTempNodeServer > setTempNodes.callback#113 > prev.map.callback#114` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 2；await 0 |
| 1438–1456 | function | `NodesPage > updateTempNodeServer > setTempNodes.callback#113` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'prev.map' |
| 1461–1477 | function | `NodesPage > restoreTempNodeServer` | 执行与 'restoreTempNodeServer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setTempNodes'、'toast.success' |
| 1462–1475 | function | `NodesPage > restoreTempNodeServer > setTempNodes.callback#116 > prev.map.callback#117` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 2；await 0 |
| 1462–1475 | function | `NodesPage > restoreTempNodeServer > setTempNodes.callback#116` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'prev.map' |
| 1481–1500 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 1；调用 'api.post'、'manualTag.trim'、'nodes.map'、'subscriptionTag.trim' |
| 1487–1496 | function | `NodesPage > mutationFn > nodes.map.callback#119` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'JSON.stringify'、'cloneProxyWithName' |
| 1501–1522 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'newNodes.map'、'queryClient.setQueryData'、'setInput'、'setNodeOrder'、'setTempNodes'、'toast.success'、'updateNodeOrderMutation.mutate' |
| 1504–1504 | function | `NodesPage > onSuccess > newNodes.map.callback#121` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1514–1517 | function | `NodesPage > onSuccess > queryClient.setQueryData.callback#122` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0 |
| 1523–1525 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1530–1543 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 1；调用 'api.put'、'savedNodes.find' |
| 1531–1531 | function | `NodesPage > mutationFn > savedNodes.find.callback#125` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1544–1546 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries' |
| 1547–1549 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1554–1556 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.delete' |
| 1557–1560 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 1561–1563 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1570–1572 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post' |
| 1573–1576 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 1577–1579 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1584–1598 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'api.put'、'savedNodes.find' |
| 1585–1585 | function | `NodesPage > mutationFn > savedNodes.find.callback#135` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1599–1604 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setTagManageInput'、'setTagManageSelectedTag'、'toast.success' |
| 1605–1607 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1612–1641 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'Promise.all'、'nodeIds.map' |
| 1615–1639 | function | `NodesPage > mutationFn > nodeIds.map.callback#139` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 8；循环 0；返回 2；await 0；调用 'Promise.resolve'、'api.put'、'newTags.filter'、'newTags.includes'、'newTags.map'、'newTags.push'、'savedNodes.find' |
| 1616–1616 | function | `NodesPage > mutationFn > nodeIds.map.callback#139 > savedNodes.find.callback#140` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1623–1623 | function | `NodesPage > mutationFn > nodeIds.map.callback#139 > newTags.map.callback#141` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 1625–1625 | function | `NodesPage > mutationFn > nodeIds.map.callback#139 > newTags.filter.callback#142` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1642–1652 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setBatchTagDialogOpen'、'setBatchTagInput'、'setBatchTagSelectedTag'、'setSelectedNodeIds'、'toast.success' |
| 1653–1655 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1660–1663 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 1664–1674 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setBatchRenameDialogOpen'、'setBatchRenameText'、'setFindText'、'setPrefixText'、'setReplaceText'、'setSelectedNodeIds'、'setSuffixText'、'toast.success' |
| 1675–1677 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1681–1684 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 1685–1689 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setSelectedNodeIds'、'toast.success' |
| 1690–1690 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1694–1805 | function | `NodesPage > useCallback.callback#151` | 封装 'useCallback.callback#151' Hook 的响应式状态、副作用和复用逻辑。 | 分支 12；循环 1；返回 1；await 3；调用 'Array.from'、'JSON.parse'、'api.get'、'api.put'、'console.error'、'countryCodeToFlag'、'encodeURIComponent'、'getGeoIPInfo'、'hasRegionEmoji'、'isIpAddress'、'parts.push'、'queryClient.invalidateQueries'、'savedNodes.find'、'setAddingRegionEmoji'、'toast.error'、'toast.info'、'toast.success'、'updateConfigName' |
| 1708–1708 | function | `NodesPage > useCallback.callback#151 > savedNodes.find.callback#152` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1808–1887 | function | `NodesPage > useCallback.callback#153` | 封装 'useCallback.callback#153' Hook 的响应式状态、副作用和复用逻辑。 | 分支 6；循环 0；返回 7；await 3；调用 'JSON.parse'、'api.get'、'api.put'、'console.error'、'countryCodeToFlag'、'encodeURIComponent'、'getGeoIPInfo'、'isIpAddress'、'queryClient.invalidateQueries'、'savedNodes.find'、'setAddingEmojiForNode'、'stripFlagEmoji'、'toast.error'、'toast.success'、'updateConfigName' |
| 1809–1809 | function | `NodesPage > useCallback.callback#153 > savedNodes.find.callback#154` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1890–1920 | function | `NodesPage > useCallback.callback#155` | 封装 'useCallback.callback#155' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 1；调用 'api.put'、'console.error'、'queryClient.invalidateQueries'、'savedNodes.find'、'setAddingEmojiForNode'、'stripFlagEmoji'、'toast.error'、'toast.success'、'updateConfigName' |
| 1891–1891 | function | `NodesPage > useCallback.callback#155 > savedNodes.find.callback#156` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1923–1971 | function | `NodesPage > useCallback.callback#157` | 封装 'useCallback.callback#157' Hook 的响应式状态、副作用和复用逻辑。 | 分支 5；循环 2；返回 2；await 0；调用 '<NonNullExpression>.push'、'JSON.parse'、'JSON.stringify'、'Object.keys'、'Object.keys.sort'、'configGroups.get'、'configGroups.has'、'configGroups.set'、'duplicates.push'、'setDuplicateDialogOpen'、'setDuplicateGroups'、'toast.info'、'toast.success' |
| 1974–2007 | function | `NodesPage > useCallback.callback#158` | 封装 'useCallback.callback#158' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 2；返回 2；await 1；调用 '<ArrayLiteralExpression>.sort'、'api.post'、'nodeIdsToDelete.push'、'queryClient.invalidateQueries'、'setDeletingDuplicates'、'setDuplicateDialogOpen'、'setDuplicateGroups'、'toast.error'、'toast.info'、'toast.success' |
| 1982–1982 | function | `NodesPage > useCallback.callback#158 > <ArrayLiteralExpression>.sort.callback#159` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 '<NewExpression>.getTime' |
| 2010–2057 | function | `NodesPage > useCallback.callback#160` | 封装 'useCallback.callback#160' Hook 的响应式状态、副作用和复用逻辑。 | 分支 3；循环 0；返回 2；await 1；调用 'Array.from'、'api.post'、'nodeOrder.forEach'、'nodesData.map'、'nodesData.map.filter'、'savedNodes.filter'、'savedNodes.filter.sort'、'setTempSubGenerating'、'setTempSubUrl'、'toast.error' |
| 2023–2023 | function | `NodesPage > useCallback.callback#160 > nodeOrder.forEach.callback#161` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'orderMap.set' |
| 2025–2025 | function | `NodesPage > useCallback.callback#160 > savedNodes.filter.callback#162` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeIdsSet.has' |
| 2026–2030 | function | `NodesPage > useCallback.callback#160 > savedNodes.filter.sort.callback#163` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'orderMap.get' |
| 2031–2037 | function | `NodesPage > useCallback.callback#160 > nodesData.map.callback#164` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 2；await 0；调用 'JSON.parse' |
| 2060–2068 | function | `NodesPage > useEffect.callback#165` | 封装 'useEffect.callback#165' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setTimeout' |
| 2063–2065 | function | `NodesPage > useEffect.callback#165 > setTimeout.callback#166` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'generateTempSubscription' |
| 2066–2066 | function | `NodesPage > useEffect.callback#165 > <anonymous#167>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'clearTimeout' |
| 2072–2105 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'JSON.parse'、'JSON.stringify'、'api.post' |
| 2106–2111 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setExchangeDialogOpen'、'setSourceNodeForExchange'、'toast.success' |
| 2112–2114 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 2119–2127 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put' |
| 2128–2136 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setExchangeDialogOpen'、'setRelayGroupMode'、'setRelayGroupName'、'setRelayGroupSelectedIds'、'setSourceNodeForExchange'、'toast.success' |
| 2137–2139 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 2144–2150 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put' |
| 2151–2154 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 2155–2157 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 2162–2181 | function | `NodesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 2182–2251 | function | `NodesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 0；await 1；调用 'api.post'、'console.log'、'data.proxies.map'、'queryClient.invalidateQueries'、'setCurrentTag'、'setSubscriptionTag'、'setTempNodes'、'subscriptionTag.trim'、'toast.success' |
| 2199–2220 | function | `NodesPage > onSuccess > data.proxies.map.callback#179` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'Math.random'、'Math.random.toString'、'Math.random.toString.substring'、'cloneProxyWithName'、'subscriptionTag.trim' |
| 2252–2254 | function | `NodesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 2258–2318 | function | `NodesPage > normalizeIndentation` | 规范化与 'normalizeIndentation' 对应的前端业务、状态或数据转换逻辑。 | 分支 8；循环 2；返回 2；await 0；调用 '<RegularExpressionLiteral>.test'、'Math.min'、'input.split'、'line.match'、'line.trim'、'lines.map'、'lines.map.join'、'normalized.split'、'normalizedLines.map'、'normalizedLines.map.join'、'normalizedLines[<key>].trim' |
| 2277–2280 | function | `NodesPage > normalizeIndentation > lines.map.callback#182` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'line.slice'、'line.trim' |
| 2302–2310 | function | `NodesPage > normalizeIndentation > normalizedLines.map.callback#183` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 3；await 0；调用 'l.match'、'l.slice'、'l.trim' |
| 2321–2379 | function | `NodesPage > parseYAMLProxies` | 解析与 'parseYAMLProxies' 对应的前端业务、状态或数据转换逻辑。 | 分支 7；循环 0；返回 4；await 0；调用 '<RegularExpressionLiteral>.test'、'Array.isArray'、'String'、'lines.join'、'normalizeIndentation'、'normalized.trim'、'parseYAML'、'toast.error'、'trimmed.includes'、'trimmed.split'、'trimmed.split.map'、'trimmed.split.map.filter'、'trimmed.split.map.join'、'trimmed.startsWith' |
| 2347–2353 | function | `NodesPage > parseYAMLProxies > trimmed.split.map.callback#185` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'l.startsWith'、'line.trim' |
| 2357–2357 | function | `NodesPage > parseYAMLProxies > trimmed.split.map.callback#186` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2381–2460 | function | `NodesPage > handleParse` | 处理与 'handleParse' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 2；返回 1；await 1；调用 'Math.random'、'Math.random.toString'、'Math.random.toString.substring'、'String'、'api.post'、'cloneProxyWithName'、'input.split'、'input.split.map'、'input.split.map.filter'、'lines.filter'、'manualTag.trim'、'parseYAMLProxies'、'parsed.push'、'setCurrentTag'、'setTempNodes'、'toast.error'、'toast.success'、'uriLines.join' |
| 2416–2416 | function | `NodesPage > handleParse > input.split.map.callback#188` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'l.trim' |
| 2417–2417 | function | `NodesPage > handleParse > input.split.map.filter.callback#189` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'l.startsWith' |
| 2418–2418 | function | `NodesPage > handleParse > lines.filter.callback#190` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'l.includes' |
| 2462–2468 | function | `NodesPage > handleSave` | 处理与 'handleSave' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'batchCreateMutation.mutate'、'toast.error' |
| 2470–2475 | function | `NodesPage > handleToggle` | 处理与 'handleToggle' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'savedNodes.find'、'toggleMutation.mutate' |
| 2471–2471 | function | `NodesPage > handleToggle > savedNodes.find.callback#193` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2477–2479 | function | `NodesPage > useCallback.callback#194` | 封装 'useCallback.callback#194' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'deleteMutation.mutate' |
| 2481–2484 | function | `NodesPage > useCallback.callback#195` | 封装 'useCallback.callback#195' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setTempNodes'、'toast.success' |
| 2482–2482 | function | `NodesPage > useCallback.callback#195 > setTempNodes.callback#196 > prev.filter.callback#197` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2482–2482 | function | `NodesPage > useCallback.callback#195 > setTempNodes.callback#196` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'prev.filter' |
| 2486–2488 | function | `NodesPage > useCallback.callback#198` | 封装 'useCallback.callback#198' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingNode' |
| 2490–2492 | function | `NodesPage > useCallback.callback#199` | 封装 'useCallback.callback#199' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingNode' |
| 2491–2491 | function | `NodesPage > useCallback.callback#199 > setEditingNode.callback#200` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 2494–2496 | function | `NodesPage > useCallback.callback#201` | 封装 'useCallback.callback#201' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingNode' |
| 2498–2528 | function | `NodesPage > useCallback.callback#202` | 封装 'useCallback.callback#202' Hook 的响应式状态、副作用和复用逻辑。 | 分支 4；循环 0；返回 4；await 0；调用 'editingNode.value.trim'、'setEditingNode'、'setTempNodes'、'toast.error'、'toast.success'、'updateNodeNameMutation.mutate' |
| 2515–2524 | function | `NodesPage > useCallback.callback#202 > setTempNodes.callback#203` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'prev.map' |
| 2516–2524 | function | `NodesPage > useCallback.callback#202 > setTempNodes.callback#203 > prev.map.callback#204` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'cloneProxyWithName' |
| 2530–2532 | function | `NodesPage > handleClearAll` | 处理与 'handleClearAll' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'clearAllMutation.mutate' |
| 2534–2556 | function | `NodesPage > handleFetchSubscription` | 处理与 'handleFetchSubscription' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 2；await 0；调用 'customUserAgent.trim'、'fetchSubscriptionMutation.mutate'、'subscriptionUrl.trim'、'toast.error' |
| 2559–2608 | function | `NodesPage > useMemo.callback#207` | 封装 'useMemo.callback#207' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.sort'、'nodeOrder.forEach'、'savedNodes.map'、'tempNodes.map' |
| 2561–2585 | function | `NodesPage > useMemo.callback#207 > savedNodes.map.callback#208` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 1；await 0；调用 'JSON.parse'、'cloneProxyWithName'、'n.id.toString'、'n.node_name.trim' |
| 2588–2594 | function | `NodesPage > useMemo.callback#207 > tempNodes.map.callback#209` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'cloneProxyWithName' |
| 2598–2598 | function | `NodesPage > useMemo.callback#207 > nodeOrder.forEach.callback#210` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'orderMap.set' |
| 2600–2604 | function | `NodesPage > useMemo.callback#207 > <ArrayLiteralExpression>.sort.callback#211` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'orderMap.get' |
| 2611–2628 | function | `NodesPage > useCallback.callback#212` | 封装 'useCallback.callback#212' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'displayNodes.filter'、'savedDisplayNodes.find'、'selectedNodeIds.has'、'setActiveId'、'setBatchDraggingIds' |
| 2619–2619 | function | `NodesPage > useCallback.callback#212 > displayNodes.filter.callback#213` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2620–2620 | function | `NodesPage > useCallback.callback#212 > savedDisplayNodes.find.callback#214` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2631–2699 | function | `NodesPage > useCallback.callback#215` | 封装 'useCallback.callback#215' Hook 的响应式状态、副作用和复用逻辑。 | 分支 8；循环 1；返回 5；await 0；调用 'arrayMove'、'debouncedSaveNodeOrder'、'displayNodes.filter'、'newOrder.push'、'savedDisplayNodes.filter'、'savedDisplayNodes.filter.map'、'savedDisplayNodes.find'、'savedDisplayNodes.findIndex'、'savedDisplayNodes.map'、'selectedNodeIds.has'、'setActiveId'、'setBatchDraggingIds' |
| 2645–2645 | function | `NodesPage > useCallback.callback#215 > displayNodes.filter.callback#216` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2646–2646 | function | `NodesPage > useCallback.callback#215 > savedDisplayNodes.find.callback#217` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2649–2649 | function | `NodesPage > useCallback.callback#215 > savedDisplayNodes.findIndex.callback#218` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2664–2664 | function | `NodesPage > useCallback.callback#215 > savedDisplayNodes.filter.callback#219` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 2665–2665 | function | `NodesPage > useCallback.callback#215 > savedDisplayNodes.filter.map.callback#220` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2668–2668 | function | `NodesPage > useCallback.callback#215 > savedDisplayNodes.filter.callback#221` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 2671–2671 | function | `NodesPage > useCallback.callback#215 > savedDisplayNodes.findIndex.callback#222` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2691–2691 | function | `NodesPage > useCallback.callback#215 > savedDisplayNodes.findIndex.callback#223` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2694–2694 | function | `NodesPage > useCallback.callback#215 > savedDisplayNodes.map.callback#224` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2702–2705 | function | `NodesPage > useCallback.callback#225` | 封装 'useCallback.callback#225' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setActiveId'、'setBatchDraggingIds' |
| 2708–2748 | function | `NodesPage > useCallback.callback#226` | 封装 'useCallback.callback#226' Hook 的响应式状态、副作用和复用逻辑。 | 分支 7；循环 1；返回 3；await 0；调用 '<ArrayLiteralExpression>.reverse'、'<ArrayLiteralExpression>.reverse.findIndex'、'currentOrder.filter'、'currentOrder.findIndex'、'currentOrder.push'、'debouncedSaveNodeOrder'、'orderSet.has'、'restItems.indexOf'、'restItems.slice'、'savedNodes.map' |
| 2714–2714 | function | `NodesPage > useCallback.callback#226 > savedNodes.map.callback#227` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2721–2721 | function | `NodesPage > useCallback.callback#226 > currentOrder.filter.callback#228` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'movingIds.has' |
| 2722–2722 | function | `NodesPage > useCallback.callback#226 > currentOrder.filter.callback#229` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'movingIds.has' |
| 2731–2731 | function | `NodesPage > useCallback.callback#226 > currentOrder.findIndex.callback#230` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'movingIds.has' |
| 2739–2739 | function | `NodesPage > useCallback.callback#226 > <ArrayLiteralExpression>.reverse.findIndex.callback#231` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'movingIds.has' |
| 2750–2766 | function | `NodesPage > useMemo.callback#232` | 封装 'useMemo.callback#232' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'nodes.filter' |
| 2755–2755 | function | `NodesPage > useMemo.callback#232 > nodes.filter.callback#233` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2760–2761 | function | `NodesPage > useMemo.callback#232 > nodes.filter.callback#234` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'node.dbNode.tags.includes' |
| 2771–2773 | function | `NodesPage > useEffect.callback#235` | 封装 'useEffect.callback#235' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodePage' |
| 2778–2782 | function | `NodesPage > useEffect.callback#236` | 封装 'useEffect.callback#236' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setNodePage' |
| 2784–2787 | function | `NodesPage > useMemo.callback#237` | 封装 'useMemo.callback#237' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'deferredFilteredNodes.slice' |
| 2789–2794 | function | `NodesPage > useMemo.callback#238` | 封装 'useMemo.callback#238' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'Math.min' |
| 2797–2800 | function | `NodesPage > useEffect.callback#239` | 封装 'useEffect.callback#239' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'tableVirtualListRef.current.scrollTo'、'virtualListRef.current.scrollTo' |
| 2806–2806 | function | `NodesPage > getScrollElement` | 读取或计算与 'getScrollElement' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 2807–2807 | function | `NodesPage > estimateSize` | 执行与 'estimateSize' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 2816–2816 | function | `NodesPage > getScrollElement` | 读取或计算与 'getScrollElement' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 2817–2817 | function | `NodesPage > estimateSize` | 执行与 'estimateSize' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 2823–2836 | function | `NodesPage > useMemo.callback#244` | 封装 'useMemo.callback#244' Hook 的响应式状态、副作用和复用逻辑。 | 分支 3；循环 0；返回 4；await 0；调用 'deferredFilteredNodes.filter'、'deferredFilteredNodes.find'、'selectedNodeIds.has' |
| 2826–2826 | function | `NodesPage > useMemo.callback#244 > deferredFilteredNodes.find.callback#245` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2831–2831 | function | `NodesPage > useMemo.callback#244 > deferredFilteredNodes.filter.callback#246` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 2838–2844 | function | `NodesPage > useMemo.callback#247` | 封装 'useMemo.callback#247' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 1；返回 1；await 0；调用 'displayNodes.filter' |
| 2841–2841 | function | `NodesPage > useMemo.callback#247 > displayNodes.filter.callback#248` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2846–2855 | function | `NodesPage > useMemo.callback#249` | 封装 'useMemo.callback#249' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'displayNodes.forEach' |
| 2848–2853 | function | `NodesPage > useMemo.callback#249 > displayNodes.forEach.callback#250` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 1；返回 0；await 0 |
| 2858–2872 | function | `NodesPage > useMemo.callback#251` | 封装 'useMemo.callback#251' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 '<ArrayLiteralExpression>.sort'、'Object.keys'、'Object.keys.filter' |
| 2859–2859 | function | `NodesPage > useMemo.callback#251 > Object.keys.filter.callback#252` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2864–2871 | function | `NodesPage > useMemo.callback#251 > <ArrayLiteralExpression>.sort.callback#253` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 4；await 0；调用 'tagOrder.indexOf' |
| 2875–2935 | function | `NodesPage > useCallback.callback#254` | 封装 'useCallback.callback#254' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 2；await 4；调用 'arrayMove'、'displayNodes.filter'、'newTagOrder.forEach'、'queryClient.invalidateQueries'、'savedDisplayNodes.forEach'、'setDraggingTag'、'setIsReorderingByTag'、'setNodeOrder'、'setTagOrder'、'sortedTags.indexOf'、'updateNodeOrderMutation.mutateAsync' |
| 2888–2888 | function | `NodesPage > useCallback.callback#254 > <anonymous#255>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'requestAnimationFrame' |
| 2897–2897 | function | `NodesPage > useCallback.callback#254 > displayNodes.filter.callback#256` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2901–2907 | function | `NodesPage > useCallback.callback#254 > savedDisplayNodes.forEach.callback#257` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'nodesByTag[<key>].push' |
| 2911–2918 | function | `NodesPage > useCallback.callback#254 > newTagOrder.forEach.callback#258` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodesInTag.forEach' |
| 2913–2917 | function | `NodesPage > useCallback.callback#254 > newTagOrder.forEach.callback#258 > nodesInTag.forEach.callback#259` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'newNodeOrder.push' |
| 2920–2924 | function | `NodesPage > useCallback.callback#254 > savedDisplayNodes.forEach.callback#260` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'newNodeOrder.includes'、'newNodeOrder.push' |
| 2938–2947 | function | `NodesPage > useMemo.callback#261` | 封装 'useMemo.callback#261' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'Array.from'、'Array.from.sort'、'savedNodes.forEach' |
| 2940–2945 | function | `NodesPage > useMemo.callback#261 > savedNodes.forEach.callback#262` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 1；返回 0；await 0；调用 't.trim'、'tags.add' |
| 2951–2963 | function | `NodesPage > useEffect.callback#263` | 封装 'useEffect.callback#263' Hook 的响应式状态、副作用和复用逻辑。 | 分支 3；循环 0；返回 1；await 0；调用 'setSelectedProtocol'、'setTagFilter' |
| 3014–3014 | function | `NodesPage > onChange.callback#264` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setInput' |
| 3024–3024 | function | `NodesPage > onClick.callback#265` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setEasterEggOpen' |
| 3039–3048 | function | `NodesPage > allUniqueTags.map.callback#266` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 3044–3044 | function | `NodesPage > allUniqueTags.map.callback#266 > onClick.callback#267` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setManualTag' |
| 3056–3056 | function | `NodesPage > onChange.callback#268` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setManualTag' |
| 3075–3075 | function | `NodesPage > onClick.callback#269` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleParse' |
| 3103–3103 | function | `NodesPage > onClick.callback#270` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setEasterEggOpen' |
| 3145–3154 | function | `NodesPage > allUniqueTags.map.callback#271` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 3150–3150 | function | `NodesPage > allUniqueTags.map.callback#271 > onClick.callback#272` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSubscriptionTag' |
| 3162–3162 | function | `NodesPage > onChange.callback#273` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSubscriptionTag' |
| 3203–3203 | function | `NodesPage > onValueChange.callback#274` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'setSubscriptionUpdateInterval' |
| 3273–3273 | function | `NodesPage > onClick.callback#275` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSpeedDialogMin'、'setSpeedDialogOpen' |
| 3281–3293 | function | `NodesPage > onClick.callback#276` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'api.post'、'toast.promise' |
| 3286–3289 | function | `NodesPage > onClick.callback#276 > success` | 执行与 'success' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'externalSyncSelection.present'、'queryClient.invalidateQueries' |
| 3290–3290 | function | `NodesPage > onClick.callback#276 > error` | 执行与 'error' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 3310–3316 | function | `NodesPage > onClick.callback#279` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'displayNodes.filter'、'selectedNodes.map'、'selectedNodes.map.join'、'setBatchRenameDialogOpen'、'setBatchRenameText' |
| 3312–3312 | function | `NodesPage > onClick.callback#279 > displayNodes.filter.callback#280` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 3313–3313 | function | `NodesPage > onClick.callback#279 > selectedNodes.map.callback#281` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3323–3323 | function | `NodesPage > onClick.callback#282` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBatchTagDialogOpen' |
| 3331–3331 | function | `NodesPage > onClick.callback#283` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Array.from'、'batchDisableSkipCertMutation.mutate' |
| 3356–3360 | function | `NodesPage > onClick.callback#284` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTempSubDialogOpen'、'setTempSubSingleNodeId'、'setTempSubUrl' |
| 3383–3400 | function | `NodesPage > onClick.callback#285` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Array.from'、'api.post'、'api.post.then'、'api.post.then.catch' |
| 3387–3396 | function | `NodesPage > onClick.callback#285 > api.post.then.callback#286` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setSelectedNodeIds'、'toast.success' |
| 3397–3399 | function | `NodesPage > onClick.callback#285 > api.post.then.catch.callback#287` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 3457–3457 | function | `NodesPage > onClick.callback#288` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelectedProtocol' |
| 3461–3474 | function | `NodesPage > PROTOCOLS.map.callback#289` | 渲染并协调 'PROTOCOLS.map.callback#289' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 2；await 0；调用 'protocol.toUpperCase' |
| 3469–3469 | function | `NodesPage > PROTOCOLS.map.callback#289 > onClick.callback#290` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelectedProtocol' |
| 3486–3503 | function | `NodesPage > onClick.callback#291` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Array.from'、'Array.from.sort'、'currentIds.every'、'displayNodes.filter'、'displayNodes.filter.filter'、'nodesToSelect.map'、'setSelectedNodeIds'、'setTagFilter' |
| 3490–3490 | function | `NodesPage > onClick.callback#291 > displayNodes.filter.callback#292` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3491–3491 | function | `NodesPage > onClick.callback#291 > displayNodes.filter.filter.callback#293` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'n.dbNode.protocol.toLowerCase' |
| 3492–3492 | function | `NodesPage > onClick.callback#291 > nodesToSelect.map.callback#294` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3498–3498 | function | `NodesPage > onClick.callback#291 > currentIds.every.callback#295` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3510–3510 | function | `NodesPage > onDragStart.callback#296` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setDraggingTag' |
| 3512–3512 | function | `NodesPage > onDragCancel.callback#297` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setDraggingTag' |
| 3518–3543 | function | `NodesPage > sortedTags.map.callback#298` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3524–3541 | function | `NodesPage > sortedTags.map.callback#298 > onClick.callback#299` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Array.from'、'Array.from.sort'、'currentIds.every'、'displayNodes.filter'、'displayNodes.filter.filter'、'nodesToSelect.map'、'setSelectedNodeIds'、'setTagFilter' |
| 3528–3528 | function | `NodesPage > sortedTags.map.callback#298 > onClick.callback#299 > displayNodes.filter.callback#300` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'n.dbNode.tags.includes' |
| 3529–3529 | function | `NodesPage > sortedTags.map.callback#298 > onClick.callback#299 > displayNodes.filter.filter.callback#301` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'n.dbNode.protocol.toLowerCase' |
| 3530–3530 | function | `NodesPage > sortedTags.map.callback#298 > onClick.callback#299 > nodesToSelect.map.callback#302` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3536–3536 | function | `NodesPage > sortedTags.map.callback#298 > onClick.callback#299 > currentIds.every.callback#303` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3559–3570 | function | `NodesPage > onClick.callback#304` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'clearTimeout'、'setSelectedNodeIds'、'setSortMode'、'updateNodeOrderMutation.mutate' |
| 3560–3560 | function | `NodesPage > onClick.callback#304 > setSortMode.callback#305` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3578–3578 | function | `NodesPage > onClick.callback#306` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setRenderMode' |
| 3578–3578 | function | `NodesPage > onClick.callback#306 > setRenderMode.callback#307` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 3616–3619 | function | `NodesPage > onPageSizeChange.callback#308` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodePage'、'setNodePageSize' |
| 3632–3632 | function | `NodesPage > paginatedNodes.map.callback#309` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3643–3997 | function | `NodesPage > paginatedNodes.map.callback#310` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 10；循环 0；返回 0；await 0；调用 '<ArrowFunction>'、'<ConditionalExpression>.map'、'Boolean'、'batchDraggingIds.has'、'node.dbNode.protocol.includes'、'node.dbNode.protocol.toUpperCase'、'node.parsed.type.toUpperCase'、'selectedNodeIds.has' |
| 3650–3650 | function | `NodesPage > paginatedNodes.map.callback#310 > <anonymous#311>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNodeSelect' |
| 3664–3672 | function | `NodesPage > paginatedNodes.map.callback#310 > onCheckedChange.callback#312` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'newSet.add'、'newSet.delete'、'setSelectedNodeIds' |
| 3697–3697 | function | `NodesPage > paginatedNodes.map.callback#310 > onClick.callback#313` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 3702–3702 | function | `NodesPage > paginatedNodes.map.callback#310 > onClick.callback#314` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditStart' |
| 3712–3715 | function | `NodesPage > paginatedNodes.map.callback#310 > onClick.callback#315` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setExchangeDialogOpen'、'setSourceNodeForExchange' |
| 3729–3733 | function | `NodesPage > paginatedNodes.map.callback#310 > onClick.callback#316` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'refetchProbeConfig'、'setProbeBindingDialogOpen'、'setSelectedNodeForProbe' |
| 3740–3811 | function | `NodesPage > paginatedNodes.map.callback#310 > <anonymous#317>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 8；循环 0；返回 3；await 0；调用 '<BinaryExpression>.toFixed'、'Math.round'、'String' |
| 3759–3759 | function | `NodesPage > paginatedNodes.map.callback#310 > <anonymous#317> > onClick.callback#318` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 3780–3780 | function | `NodesPage > paginatedNodes.map.callback#310 > <anonymous#317> > onClick.callback#319` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 3799–3799 | function | `NodesPage > paginatedNodes.map.callback#310 > <anonymous#317> > onClick.callback#320` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 3815–3815 | function | `NodesPage > paginatedNodes.map.callback#310 > onSelect.callback#321` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleSetNodeFlag' |
| 3816–3816 | function | `NodesPage > paginatedNodes.map.callback#310 > onAutoDetect.callback#322` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleAddSingleNodeEmoji' |
| 3828–3832 | function | `NodesPage > paginatedNodes.map.callback#310 > onClick.callback#323` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTempSubDialogOpen'、'setTempSubSingleNodeId'、'setTempSubUrl' |
| 3845–3845 | function | `NodesPage > paginatedNodes.map.callback#310 > onClick.callback#324` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 3848–3848 | function | `NodesPage > paginatedNodes.map.callback#310 > onChange.callback#325` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditChange' |
| 3849–3857 | function | `NodesPage > paginatedNodes.map.callback#310 > onKeyDown.callback#326` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'event.preventDefault'、'handleNameEditCancel'、'handleNameEditSubmit' |
| 3865–3865 | function | `NodesPage > paginatedNodes.map.callback#310 > onClick.callback#327` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditSubmit' |
| 3881–3890 | function | `NodesPage > paginatedNodes.map.callback#310 > <anonymous#328>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 2；await 0；调用 'node.dbNode.relay_group_node_ids.map'、'node.dbNode.relay_group_node_ids.map.filter'、'node.dbNode.relay_group_node_ids.map.filter.join'、'nodeIdToName.get' |
| 3887–3887 | function | `NodesPage > paginatedNodes.map.callback#310 > <anonymous#328> > node.dbNode.relay_group_node_ids.map.callback#329` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeIdToName.get' |
| 3889–3889 | function | `NodesPage > paginatedNodes.map.callback#310 > <anonymous#328> > onClick.callback#330` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'unbindRelayGroupMutation.mutate' |
| 3916–3923 | function | `NodesPage > paginatedNodes.map.callback#310 > <ConditionalExpression>.map.callback#331` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3917–3922 | function | `NodesPage > paginatedNodes.map.callback#310 > <ConditionalExpression>.map.callback#331 > onClick.callback#332` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'setTagManageDialogOpen'、'setTagManageInput'、'setTagManageNodeId'、'setTagManageSelectedTag' |
| 3934–3934 | function | `NodesPage > paginatedNodes.map.callback#310 > onClick.callback#333` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 3940–3948 | function | `NodesPage > paginatedNodes.map.callback#310 > onClick.callback#334` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'handleEditClashConfig'、'setClashDialogOpen' |
| 3959–3959 | function | `NodesPage > paginatedNodes.map.callback#310 > onClick.callback#335` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleCopyUri' |
| 3987–3987 | function | `NodesPage > paginatedNodes.map.callback#310 > onClick.callback#336` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'handleDelete'、'handleDeleteTemp' |
| 4033–4332 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 9；循环 0；返回 2；await 0；调用 '<ArrowFunction>'、'<ConditionalExpression>.map'、'cn'、'node.dbNode.protocol.includes'、'node.dbNode.protocol.toUpperCase'、'node.parsed.type.toUpperCase'、'selectedNodeIds.has' |
| 4055–4055 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > <anonymous#338>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNodeSelect' |
| 4066–4074 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onCheckedChange.callback#339` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'newSet.add'、'newSet.delete'、'setSelectedNodeIds' |
| 4075–4075 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onClick.callback#340` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 4100–4100 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onClick.callback#341` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 4103–4103 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onChange.callback#342` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditChange' |
| 4104–4112 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onKeyDown.callback#343` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'event.preventDefault'、'handleNameEditCancel'、'handleNameEditSubmit' |
| 4120–4120 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onClick.callback#344` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditSubmit' |
| 4136–4145 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > <anonymous#345>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 2；await 0；调用 'node.dbNode.relay_group_node_ids.map'、'node.dbNode.relay_group_node_ids.map.filter'、'node.dbNode.relay_group_node_ids.map.filter.join'、'nodeIdToName.get' |
| 4142–4142 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > <anonymous#345> > node.dbNode.relay_group_node_ids.map.callback#346` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeIdToName.get' |
| 4144–4144 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > <anonymous#345> > onClick.callback#347` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'unbindRelayGroupMutation.mutate' |
| 4152–4152 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onClick.callback#348` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 4157–4157 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onClick.callback#349` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditStart' |
| 4167–4170 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onClick.callback#350` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setExchangeDialogOpen'、'setSourceNodeForExchange' |
| 4181–4234 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > <anonymous#351>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 7；循环 0；返回 3；await 0；调用 '<BinaryExpression>.toFixed'、'Math.round'、'String' |
| 4197–4197 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > <anonymous#351> > onClick.callback#352` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 4212–4212 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > <anonymous#351> > onClick.callback#353` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 4225–4225 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > <anonymous#351> > onClick.callback#354` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 4255–4262 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > <ConditionalExpression>.map.callback#355` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4256–4261 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > <ConditionalExpression>.map.callback#355 > onClick.callback#356` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'setTagManageDialogOpen'、'setTagManageInput'、'setTagManageNodeId'、'setTagManageSelectedTag' |
| 4267–4267 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onClick.callback#357` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 4273–4281 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onClick.callback#358` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'handleEditClashConfig'、'setClashDialogOpen' |
| 4292–4292 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onClick.callback#359` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleCopyUri' |
| 4320–4320 | function | `NodesPage > rowVirtualizer.getVirtualItems.map.callback#337 > onClick.callback#360` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'handleDelete'、'handleDeleteTemp' |
| 4350–4350 | function | `NodesPage > paginatedNodes.map.callback#361` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4372–4800 | function | `NodesPage > paginatedNodes.map.callback#362` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 12；循环 0；返回 0；await 0；调用 '<ArrowFunction>'、'<ConditionalExpression>.map'、'Boolean'、'batchDraggingIds.has'、'node.dbNode.protocol.includes'、'node.dbNode.protocol.toUpperCase'、'node.parsed.type.toUpperCase'、'selectedNodeIds.has' |
| 4379–4379 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#363>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleRowClick' |
| 4410–4410 | function | `NodesPage > paginatedNodes.map.callback#362 > onChange.callback#364` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditChange' |
| 4411–4419 | function | `NodesPage > paginatedNodes.map.callback#362 > onKeyDown.callback#365` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'e.preventDefault'、'handleNameEditCancel'、'handleNameEditSubmit' |
| 4422–4422 | function | `NodesPage > paginatedNodes.map.callback#362 > onClick.callback#366` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 4428–4428 | function | `NodesPage > paginatedNodes.map.callback#362 > onClick.callback#367` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditSubmit' |
| 4462–4471 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#368>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 2；await 0；调用 'node.dbNode.relay_group_node_ids.map'、'node.dbNode.relay_group_node_ids.map.filter'、'node.dbNode.relay_group_node_ids.map.filter.join'、'nodeIdToName.get' |
| 4468–4468 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#368> > node.dbNode.relay_group_node_ids.map.callback#369` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeIdToName.get' |
| 4470–4470 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#368> > onClick.callback#370` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'unbindRelayGroupMutation.mutate' |
| 4494–4573 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#371>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 3；await 0；调用 'String'、'ipMenuState.ips.map'、'isIpAddress' |
| 4512–4512 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#371> > onClick.callback#372` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'restoreTempNodeServer' |
| 4521–4521 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#371> > onOpenChange.callback#373` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setIpMenuState' |
| 4537–4554 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#371> > ipMenuState.ips.map.callback#374` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4540–4550 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#371> > ipMenuState.ips.map.callback#374 > onClick.callback#375` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setIpMenuState'、'updateNodeServerMutation.mutate'、'updateTempNodeServer' |
| 4564–4564 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#371> > onClick.callback#376` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleResolveIp' |
| 4582–4582 | function | `NodesPage > paginatedNodes.map.callback#362 > onClick.callback#377` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'restoreNodeServerMutation.mutate' |
| 4593–4597 | function | `NodesPage > paginatedNodes.map.callback#362 > onClick.callback#378` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'refetchProbeConfig'、'setProbeBindingDialogOpen'、'setSelectedNodeForProbe' |
| 4604–4675 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#379>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 8；循环 0；返回 3；await 0；调用 '<BinaryExpression>.toFixed'、'Math.round'、'String' |
| 4623–4623 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#379> > onClick.callback#380` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 4644–4644 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#379> > onClick.callback#381` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 4663–4663 | function | `NodesPage > paginatedNodes.map.callback#362 > <anonymous#379> > onClick.callback#382` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 4684–4684 | function | `NodesPage > paginatedNodes.map.callback#362 > onClick.callback#383` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditStart' |
| 4694–4697 | function | `NodesPage > paginatedNodes.map.callback#362 > onClick.callback#384` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setExchangeDialogOpen'、'setSourceNodeForExchange' |
| 4708–4708 | function | `NodesPage > paginatedNodes.map.callback#362 > onSelect.callback#385` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleSetNodeFlag' |
| 4709–4709 | function | `NodesPage > paginatedNodes.map.callback#362 > onAutoDetect.callback#386` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleAddSingleNodeEmoji' |
| 4720–4727 | function | `NodesPage > paginatedNodes.map.callback#362 > <ConditionalExpression>.map.callback#387` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4721–4726 | function | `NodesPage > paginatedNodes.map.callback#362 > <ConditionalExpression>.map.callback#387 > onClick.callback#388` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'setTagManageDialogOpen'、'setTagManageInput'、'setTagManageNodeId'、'setTagManageSelectedTag' |
| 4742–4748 | function | `NodesPage > paginatedNodes.map.callback#362 > onClick.callback#389` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'handleEditClashConfig' |
| 4758–4758 | function | `NodesPage > paginatedNodes.map.callback#362 > onClick.callback#390` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleCopyUri' |
| 4791–4791 | function | `NodesPage > paginatedNodes.map.callback#362 > onClick.callback#391` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'handleDelete'、'handleDeleteTemp' |
| 4840–5061 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 7；循环 0；返回 2；await 0；调用 '<ArrowFunction>'、'<ConditionalExpression>.map'、'cn'、'node.dbNode.protocol.includes'、'node.dbNode.protocol.toUpperCase'、'node.parsed.type.toUpperCase'、'selectedNodeIds.has' |
| 4859–4859 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <anonymous#393>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNodeSelect' |
| 4866–4874 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onCheckedChange.callback#394` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'newSet.add'、'newSet.delete'、'setSelectedNodeIds' |
| 4875–4875 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onClick.callback#395` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 4899–4899 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onClick.callback#396` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 4900–4909 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <anonymous#397>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 2；await 0；调用 'node.dbNode.relay_group_node_ids.map'、'node.dbNode.relay_group_node_ids.map.filter'、'node.dbNode.relay_group_node_ids.map.filter.join'、'nodeIdToName.get' |
| 4906–4906 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <anonymous#397> > node.dbNode.relay_group_node_ids.map.callback#398` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeIdToName.get' |
| 4908–4908 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <anonymous#397> > onClick.callback#399` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'unbindRelayGroupMutation.mutate' |
| 4913–4913 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onClick.callback#400` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditStart' |
| 4917–4917 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onClick.callback#401` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setExchangeDialogOpen'、'setSourceNodeForExchange' |
| 4923–4923 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onSelect.callback#402` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleSetNodeFlag' |
| 4924–4924 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onAutoDetect.callback#403` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleAddSingleNodeEmoji' |
| 4937–4945 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <anonymous#404>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 3；await 0；调用 'String'、'isIpAddress' |
| 4942–4942 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <anonymous#404> > onClick.callback#405` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'restoreTempNodeServer' |
| 4944–4944 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <anonymous#404> > onClick.callback#406` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleResolveIp' |
| 4948–4948 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onClick.callback#407` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'refetchProbeConfig'、'setProbeBindingDialogOpen'、'setSelectedNodeForProbe' |
| 4953–4965 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <anonymous#408>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 6；循环 0；返回 3；await 0；调用 'Math.round'、'String' |
| 4959–4959 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <anonymous#408> > onClick.callback#409` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 4962–4962 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <anonymous#408> > onClick.callback#410` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 4964–4964 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <anonymous#408> > onClick.callback#411` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 4971–4978 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <ConditionalExpression>.map.callback#412` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4972–4977 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > <ConditionalExpression>.map.callback#412 > onClick.callback#413` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'setTagManageDialogOpen'、'setTagManageInput'、'setTagManageNodeId'、'setTagManageSelectedTag' |
| 4981–4981 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onClick.callback#414` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 4988–4995 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onClick.callback#415` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'handleEditClashConfig'、'setClashDialogOpen' |
| 5005–5005 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onClick.callback#416` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleCopyUri' |
| 5014–5018 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onClick.callback#417` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTempSubDialogOpen'、'setTempSubSingleNodeId'、'setTempSubUrl' |
| 5028–5028 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onClick.callback#418` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 5051–5051 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#392 > onClick.callback#419` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'handleDelete'、'handleDeleteTemp' |
| 5072–5072 | function | `NodesPage > paginatedNodes.map.callback#420` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5095–5622 | function | `NodesPage > paginatedNodes.map.callback#421` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 19；循环 0；返回 0；await 0；调用 '<ArrowFunction>'、'<ConditionalExpression>.map'、'Boolean'、'batchDraggingIds.has'、'editingClashConfig.config.split'、'editingClashConfig.config.split.map'、'node.dbNode.protocol.includes'、'node.dbNode.protocol.toUpperCase'、'node.parsed.type.toUpperCase'、'selectedNodeIds.has' |
| 5102–5102 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#422>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleRowClick' |
| 5132–5132 | function | `NodesPage > paginatedNodes.map.callback#421 > onChange.callback#423` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditChange' |
| 5133–5141 | function | `NodesPage > paginatedNodes.map.callback#421 > onKeyDown.callback#424` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'event.preventDefault'、'handleNameEditCancel'、'handleNameEditSubmit' |
| 5149–5149 | function | `NodesPage > paginatedNodes.map.callback#421 > onClick.callback#425` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditSubmit' |
| 5165–5174 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#426>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 2；await 0；调用 'node.dbNode.relay_group_node_ids.map'、'node.dbNode.relay_group_node_ids.map.filter'、'node.dbNode.relay_group_node_ids.map.filter.join'、'nodeIdToName.get' |
| 5171–5171 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#426> > node.dbNode.relay_group_node_ids.map.callback#427` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeIdToName.get' |
| 5173–5173 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#426> > onClick.callback#428` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'unbindRelayGroupMutation.mutate' |
| 5184–5184 | function | `NodesPage > paginatedNodes.map.callback#421 > onClick.callback#429` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNameEditStart' |
| 5194–5197 | function | `NodesPage > paginatedNodes.map.callback#421 > onClick.callback#430` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setExchangeDialogOpen'、'setSourceNodeForExchange' |
| 5208–5208 | function | `NodesPage > paginatedNodes.map.callback#421 > onSelect.callback#431` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleSetNodeFlag' |
| 5209–5209 | function | `NodesPage > paginatedNodes.map.callback#421 > onAutoDetect.callback#432` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleAddSingleNodeEmoji' |
| 5221–5228 | function | `NodesPage > paginatedNodes.map.callback#421 > <ConditionalExpression>.map.callback#433` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5222–5227 | function | `NodesPage > paginatedNodes.map.callback#421 > <ConditionalExpression>.map.callback#433 > onClick.callback#434` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'setTagManageDialogOpen'、'setTagManageInput'、'setTagManageNodeId'、'setTagManageSelectedTag' |
| 5257–5336 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#435>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 3；await 0；调用 'String'、'ipMenuState.ips.map'、'isIpAddress' |
| 5275–5275 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#435> > onClick.callback#436` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'restoreTempNodeServer' |
| 5284–5284 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#435> > onOpenChange.callback#437` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setIpMenuState' |
| 5300–5317 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#435> > ipMenuState.ips.map.callback#438` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5303–5313 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#435> > ipMenuState.ips.map.callback#438 > onClick.callback#439` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setIpMenuState'、'updateNodeServerMutation.mutate'、'updateTempNodeServer' |
| 5327–5327 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#435> > onClick.callback#440` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleResolveIp' |
| 5345–5345 | function | `NodesPage > paginatedNodes.map.callback#421 > onClick.callback#441` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'restoreNodeServerMutation.mutate' |
| 5356–5360 | function | `NodesPage > paginatedNodes.map.callback#421 > onClick.callback#442` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'refetchProbeConfig'、'setProbeBindingDialogOpen'、'setSelectedNodeForProbe' |
| 5367–5438 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#443>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 8；循环 0；返回 3；await 0；调用 '<BinaryExpression>.toFixed'、'Math.round'、'String' |
| 5386–5386 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#443> > onClick.callback#444` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 5407–5407 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#443> > onClick.callback#445` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 5426–5426 | function | `NodesPage > paginatedNodes.map.callback#421 > <anonymous#443> > onClick.callback#446` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 5454–5464 | function | `NodesPage > paginatedNodes.map.callback#421 > onOpenChange.callback#447` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setClashDialogOpen'、'setTimeout' |
| 5458–5462 | function | `NodesPage > paginatedNodes.map.callback#421 > onOpenChange.callback#447 > setTimeout.callback#448` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setClashConfigError'、'setEditingClashConfig'、'setJsonErrorLines' |
| 5471–5477 | function | `NodesPage > paginatedNodes.map.callback#421 > onClick.callback#449` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'handleEditClashConfig' |
| 5498–5498 | function | `NodesPage > paginatedNodes.map.callback#421 > onClick.callback#450` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleConfigFormatChange' |
| 5506–5506 | function | `NodesPage > paginatedNodes.map.callback#421 > onClick.callback#451` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleConfigFormatChange' |
| 5514–5525 | function | `NodesPage > paginatedNodes.map.callback#421 > editingClashConfig.config.split.map.callback#452` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'jsonErrorLines.includes' |
| 5530–5530 | function | `NodesPage > paginatedNodes.map.callback#421 > onChange.callback#453` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleClashConfigChange' |
| 5545–5545 | function | `NodesPage > paginatedNodes.map.callback#421 > onClick.callback#454` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setClashDialogOpen' |
| 5567–5567 | function | `NodesPage > paginatedNodes.map.callback#421 > onClick.callback#455` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleCopyUri' |
| 5576–5582 | function | `NodesPage > paginatedNodes.map.callback#421 > onClick.callback#456` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setTempSubDialogOpen'、'setTempSubSingleNodeId'、'setTempSubUrl' |
| 5613–5613 | function | `NodesPage > paginatedNodes.map.callback#421 > onClick.callback#457` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'handleDelete'、'handleDeleteTemp' |
| 5663–5955 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 9；循环 0；返回 2；await 0；调用 '<ArrowFunction>'、'<ConditionalExpression>.map'、'cn'、'node.dbNode.protocol.includes'、'node.dbNode.protocol.toUpperCase'、'node.parsed.type.toUpperCase'、'selectedNodeIds.has' |
| 5682–5682 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#459>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleNodeSelect' |
| 5689–5697 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onCheckedChange.callback#460` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'newSet.add'、'newSet.delete'、'setSelectedNodeIds' |
| 5698–5698 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onClick.callback#461` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 5723–5732 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#462>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 2；await 0；调用 'node.dbNode.relay_group_node_ids.map'、'node.dbNode.relay_group_node_ids.map.filter'、'node.dbNode.relay_group_node_ids.map.filter.join'、'nodeIdToName.get' |
| 5729–5729 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#462> > node.dbNode.relay_group_node_ids.map.callback#463` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeIdToName.get' |
| 5731–5731 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#462> > onClick.callback#464` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'unbindRelayGroupMutation.mutate' |
| 5740–5743 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onClick.callback#465` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'handleNameEditStart' |
| 5752–5756 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onClick.callback#466` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'setExchangeDialogOpen'、'setSourceNodeForExchange' |
| 5767–5767 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onSelect.callback#467` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleSetNodeFlag' |
| 5768–5768 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onAutoDetect.callback#468` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleAddSingleNodeEmoji' |
| 5779–5786 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <ConditionalExpression>.map.callback#469` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5780–5785 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <ConditionalExpression>.map.callback#469 > onClick.callback#470` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'setTagManageDialogOpen'、'setTagManageInput'、'setTagManageNodeId'、'setTagManageSelectedTag' |
| 5789–5789 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onClick.callback#471` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 5796–5828 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#472>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 3；await 0；调用 'String'、'ipMenuState.ips.map'、'isIpAddress' |
| 5803–5803 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#472> > onClick.callback#473` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'restoreTempNodeServer' |
| 5809–5809 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#472> > onOpenChange.callback#474` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setIpMenuState' |
| 5816–5820 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#472> > ipMenuState.ips.map.callback#475` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5817–5817 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#472> > ipMenuState.ips.map.callback#475 > onClick.callback#476` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setIpMenuState'、'updateNodeServerMutation.mutate'、'updateTempNodeServer' |
| 5824–5824 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#472> > onClick.callback#477` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleResolveIp' |
| 5831–5831 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onClick.callback#478` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'restoreNodeServerMutation.mutate' |
| 5837–5837 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onClick.callback#479` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'refetchProbeConfig'、'setProbeBindingDialogOpen'、'setSelectedNodeForProbe' |
| 5842–5870 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#480>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 8；循环 0；返回 3；await 0；调用 '<BinaryExpression>.toFixed'、'Math.round'、'String' |
| 5850–5850 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#480> > onClick.callback#481` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 5859–5859 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#480> > onClick.callback#482` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 5865–5865 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > <anonymous#480> > onClick.callback#483` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTcping' |
| 5875–5875 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onClick.callback#484` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 5882–5889 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onClick.callback#485` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'handleEditClashConfig'、'setClashDialogOpen' |
| 5899–5899 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onClick.callback#486` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleCopyUri' |
| 5908–5912 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onClick.callback#487` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTempSubDialogOpen'、'setTempSubSingleNodeId'、'setTempSubUrl' |
| 5922–5922 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onClick.callback#488` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 5945–5945 | function | `NodesPage > tableVirtualizer.getVirtualItems.map.callback#458 > onClick.callback#489` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'handleDelete'、'handleDeleteTemp' |
| 5979–5982 | function | `NodesPage > onPageSizeChange.callback#490` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodePage'、'setNodePageSize' |
| 5994–6003 | function | `NodesPage > onOpenChange.callback#491` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setClashDialogOpen'、'setTimeout' |
| 5997–6001 | function | `NodesPage > onOpenChange.callback#491 > setTimeout.callback#492` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setClashConfigError'、'setEditingClashConfig'、'setJsonErrorLines' |
| 6020–6020 | function | `NodesPage > onClick.callback#493` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleConfigFormatChange' |
| 6028–6028 | function | `NodesPage > onClick.callback#494` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleConfigFormatChange' |
| 6036–6047 | function | `NodesPage > editingClashConfig.config.split.map.callback#495` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'jsonErrorLines.includes' |
| 6052–6052 | function | `NodesPage > onChange.callback#496` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleClashConfigChange' |
| 6067–6067 | function | `NodesPage > onClick.callback#497` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setClashDialogOpen' |
| 6102–6105 | function | `NodesPage > onOpenChange.callback#498` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setProbeBindingDialogOpen'、'setProbeSearchQuery' |
| 6119–6119 | function | `NodesPage > onChange.callback#499` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProbeSearchQuery' |
| 6124–6124 | function | `NodesPage > probeConfig.servers.filter.callback#500` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'probeSearchQuery.toLowerCase'、's.name.toLowerCase'、's.name.toLowerCase.includes'、's.server_id.toLowerCase'、's.server_id.toLowerCase.includes' |
| 6125–6148 | function | `NodesPage > probeConfig.servers.filter.map.callback#501` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 6130–6137 | function | `NodesPage > probeConfig.servers.filter.map.callback#501 > onClick.callback#502` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'updateProbeBindingMutation.mutate' |
| 6154–6161 | function | `NodesPage > onClick.callback#503` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'updateProbeBindingMutation.mutate' |
| 6194–6194 | function | `NodesPage > onClick.callback#504` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setUriDialogOpen' |
| 6199–6206 | function | `NodesPage > onClick.callback#505` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'navigator.clipboard.writeText'、'navigator.clipboard.writeText.then'、'navigator.clipboard.writeText.then.catch' |
| 6200–6203 | function | `NodesPage > onClick.callback#505 > navigator.clipboard.writeText.then.callback#506` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setUriDialogOpen'、'toast.success' |
| 6203–6205 | function | `NodesPage > onClick.callback#505 > navigator.clipboard.writeText.then.catch.callback#507` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 6216–6228 | function | `NodesPage > onOpenChange.callback#508` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'setExchangeDialogOpen'、'setExchangeFilterText'、'setRelayGroupMode'、'setRelayGroupName'、'setRelayGroupSelectedIds' |
| 6241–6246 | function | `NodesPage > onClick.callback#509` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setRelayGroupMode'、'setRelayGroupSelectedIds' |
| 6252–6259 | function | `NodesPage > onClick.callback#510` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'setRelayGroupMode'、'setRelayGroupName' |
| 6266–6294 | function | `NodesPage > <anonymous#511>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 1；返回 1；await 0；调用 'Array.from'、'Array.from.map'、'existingGroups.entries'、'existingGroups.has'、'existingGroups.set' |
| 6278–6291 | function | `NodesPage > <anonymous#511> > Array.from.map.callback#512` | 渲染并协调 'Array.from.map.callback#512' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 0；await 0 |
| 6284–6287 | function | `NodesPage > <anonymous#511> > Array.from.map.callback#512 > onClick.callback#513` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setRelayGroupName'、'setRelayGroupSelectedIds' |
| 6298–6298 | function | `NodesPage > onChange.callback#514` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setRelayGroupName' |
| 6307–6307 | function | `NodesPage > onChange.callback#515` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setExchangeFilterText' |
| 6315–6382 | function | `NodesPage > <anonymous#516>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 1；await 0；调用 'exchangeFilterText.trim'、'filteredNodes.map'、'savedNodes.filter'、'savedNodes.filter.filter'、'savedNodes.filter.filter.filter' |
| 6317–6317 | function | `NodesPage > <anonymous#516> > savedNodes.filter.callback#517` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6318–6318 | function | `NodesPage > <anonymous#516> > savedNodes.filter.filter.callback#518` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'node.protocol.includes' |
| 6319–6327 | function | `NodesPage > <anonymous#516> > savedNodes.filter.filter.filter.callback#519` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'exchangeFilterText.toLowerCase'、'exchangeFilterText.trim'、'node.node_name.toLowerCase'、'node.node_name.toLowerCase.includes'、'node.protocol.toLowerCase'、'node.protocol.toLowerCase.includes'、'node.tag.toLowerCase'、'node.tag.toLowerCase.includes'、'node.tags.some' |
| 6325–6325 | function | `NodesPage > <anonymous#516> > savedNodes.filter.filter.filter.callback#519 > node.tags.some.callback#520` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 't.toLowerCase'、't.toLowerCase.includes' |
| 6331–6375 | function | `NodesPage > <anonymous#516> > filteredNodes.map.callback#521` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 0；await 0；调用 '<ConditionalExpression>.map'、'relayGroupSelectedIds.has' |
| 6336–6353 | function | `NodesPage > <anonymous#516> > filteredNodes.map.callback#521 > onClick.callback#522` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'createRelayNodeMutation.mutate'、'setRelayGroupSelectedIds' |
| 6338–6346 | function | `NodesPage > <anonymous#516> > filteredNodes.map.callback#521 > onClick.callback#522 > setRelayGroupSelectedIds.callback#523` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'next.add'、'next.delete'、'next.has' |
| 6368–6372 | function | `NodesPage > <anonymous#516> > filteredNodes.map.callback#521 > <ConditionalExpression>.map.callback#524` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6389–6397 | function | `NodesPage > onClick.callback#525` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Array.from'、'createRelayGroupMutation.mutate'、'relayGroupName.trim' |
| 6407–6410 | function | `NodesPage > onOpenChange.callback#526` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setTagManageDialogOpen'、'setTagManageInput'、'setTagManageNodeId'、'setTagManageSelectedTag' |
| 6417–6462 | function | `NodesPage > <anonymous#527>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 1；await 0；调用 'nodeTags.includes'、'nodeTags.map'、'savedNodes.find'、'tagManageInput.trim' |
| 6418–6418 | function | `NodesPage > <anonymous#527> > savedNodes.find.callback#528` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6423–6432 | function | `NodesPage > <anonymous#527> > nodeTags.map.callback#529` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 6428–6428 | function | `NodesPage > <anonymous#527> > nodeTags.map.callback#529 > onClick.callback#530` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTagManageInput'、'setTagManageSelectedTag' |
| 6438–6438 | function | `NodesPage > <anonymous#527> > onChange.callback#531` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTagManageInput' |
| 6443–6447 | function | `NodesPage > <anonymous#527> > onClick.callback#532` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 1；await 0；调用 'nodeTags.filter'、'updateNodeTagsMutation.mutate' |
| 6445–6445 | function | `NodesPage > <anonymous#527> > onClick.callback#532 > nodeTags.filter.callback#533` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6448–6452 | function | `NodesPage > <anonymous#527> > onClick.callback#534` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'nodeTags.map'、'updateNodeTagsMutation.mutate' |
| 6450–6450 | function | `NodesPage > <anonymous#527> > onClick.callback#534 > nodeTags.map.callback#535` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'tagManageInput.trim' |
| 6455–6458 | function | `NodesPage > <anonymous#527> > onClick.callback#536` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'tagManageInput.trim'、'updateNodeTagsMutation.mutate' |
| 6468–6471 | function | `NodesPage > onOpenChange.callback#537` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setBatchTagDialogOpen'、'setBatchTagInput'、'setBatchTagMode'、'setBatchTagSelectedTag' |
| 6480–6486 | function | `NodesPage > <AsExpression>.map.callback#538` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 0；await 0 |
| 6481–6483 | function | `NodesPage > <AsExpression>.map.callback#538 > onClick.callback#539` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBatchTagInput'、'setBatchTagMode'、'setBatchTagSelectedTag' |
| 6490–6513 | function | `NodesPage > <anonymous#540>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 1；返回 1；await 0；调用 '<ArrayLiteralExpression>.sort'、'<ConditionalExpression>.forEach'、'batchTags.map'、'savedNodes.find' |
| 6493–6493 | function | `NodesPage > <anonymous#540> > savedNodes.find.callback#541` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6494–6494 | function | `NodesPage > <anonymous#540> > <ConditionalExpression>.forEach.callback#542` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'tags.add' |
| 6501–6509 | function | `NodesPage > <anonymous#540> > batchTags.map.callback#543` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 6502–6506 | function | `NodesPage > <anonymous#540> > batchTags.map.callback#543 > onClick.callback#544` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'setBatchTagInput'、'setBatchTagSelectedTag' |
| 6522–6522 | function | `NodesPage > onChange.callback#545` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBatchTagInput' |
| 6532–6536 | function | `NodesPage > allUniqueTags.map.callback#546` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6533–6533 | function | `NodesPage > allUniqueTags.map.callback#546 > onClick.callback#547` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBatchTagInput' |
| 6543–6543 | function | `NodesPage > onClick.callback#548` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBatchTagDialogOpen' |
| 6553–6562 | function | `NodesPage > onClick.callback#549` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 0；await 0；调用 'Array.from'、'batchTagInput.trim'、'batchUpdateTagMutation.mutate' |
| 6591–6591 | function | `NodesPage > onChange.callback#550` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFindText' |
| 6604–6604 | function | `NodesPage > onChange.callback#551` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setReplaceText' |
| 6612–6622 | function | `NodesPage > onClick.callback#552` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'batchRenameText.split'、'batchRenameText.split.map'、'batchRenameText.split.map.join'、'setBatchRenameText'、'toast.error'、'toast.success' |
| 6617–6618 | function | `NodesPage > onClick.callback#552 > batchRenameText.split.map.callback#553` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'findText.replace'、'line.replace' |
| 6638–6638 | function | `NodesPage > onChange.callback#554` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setPrefixText' |
| 6650–6650 | function | `NodesPage > onChange.callback#555` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSuffixText' |
| 6657–6669 | function | `NodesPage > onClick.callback#556` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'batchRenameText.split'、'batchRenameText.split.map'、'batchRenameText.split.map.join'、'setBatchRenameText'、'setPrefixText'、'setSuffixText'、'toast.error'、'toast.success' |
| 6662–6663 | function | `NodesPage > onClick.callback#556 > batchRenameText.split.map.callback#557` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 6683–6683 | function | `NodesPage > onChange.callback#558` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBatchRenameText' |
| 6696–6703 | function | `NodesPage > onClick.callback#559` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBatchRenameDialogOpen'、'setBatchRenameText'、'setFindText'、'setPrefixText'、'setReplaceText'、'setSuffixText' |
| 6709–6731 | function | `NodesPage > onClick.callback#560` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 2；await 0；调用 'batchRenameMutation.mutate'、'batchRenameText.split'、'batchRenameText.split.map'、'batchRenameText.split.map.filter'、'displayNodes.filter'、'displayNodes.filter.map'、'nodeIds.map'、'toast.error' |
| 6710–6710 | function | `NodesPage > onClick.callback#560 > batchRenameText.split.map.callback#561` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'line.trim' |
| 6710–6710 | function | `NodesPage > onClick.callback#560 > batchRenameText.split.map.filter.callback#562` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6712–6712 | function | `NodesPage > onClick.callback#560 > displayNodes.filter.callback#563` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedNodeIds.has' |
| 6712–6712 | function | `NodesPage > onClick.callback#560 > displayNodes.filter.map.callback#564` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6725–6728 | function | `NodesPage > onClick.callback#560 > nodeIds.map.callback#565` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6747–6747 | function | `NodesPage > duplicateGroups.reduce.callback#566` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6751–6791 | function | `NodesPage > duplicateGroups.map.callback#567` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 '<ArrayLiteralExpression>.sort'、'<ArrayLiteralExpression>.sort.map' |
| 6763–6763 | function | `NodesPage > duplicateGroups.map.callback#567 > <ArrayLiteralExpression>.sort.callback#568` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 '<NewExpression>.getTime' |
| 6764–6788 | function | `NodesPage > duplicateGroups.map.callback#567 > <ArrayLiteralExpression>.sort.map.callback#569` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 5；循环 0；返回 0；await 0；调用 '<ConditionalExpression>.map'、'node.protocol.toUpperCase' |
| 6778–6782 | function | `NodesPage > duplicateGroups.map.callback#567 > <ArrayLiteralExpression>.sort.map.callback#569 > <ConditionalExpression>.map.callback#570` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6796–6799 | function | `NodesPage > onClick.callback#571` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setDuplicateDialogOpen'、'setDuplicateGroups' |
| 6809–6809 | function | `NodesPage > duplicateGroups.reduce.callback#572` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6818–6824 | function | `NodesPage > onOpenChange.callback#573` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setTempSubDialogOpen'、'setTempSubSingleNodeId'、'setTempSubUrl' |
| 6831–6831 | function | `NodesPage > savedNodes.find.callback#574` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6848–6848 | function | `NodesPage > onChange.callback#575` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parseInt'、'setTempSubMaxAccess' |
| 6862–6862 | function | `NodesPage > onChange.callback#576` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parseInt'、'setTempSubExpireSeconds' |
| 6880–6890 | function | `NodesPage > onClick.callback#577` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 1；调用 'navigator.clipboard.writeText'、'setTempSubDialogOpen'、'setTempSubSingleNodeId'、'setTempSubUrl'、'toast.error'、'toast.success' |
| 6905–6909 | function | `NodesPage > onClick.callback#578` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTempSubDialogOpen'、'setTempSubSingleNodeId'、'setTempSubUrl' |
| 6925–6925 | function | `NodesPage > onClick.callback#579` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleMoveNodes' |
| 6933–6933 | function | `NodesPage > onClick.callback#580` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleMoveNodes' |
| 6941–6941 | function | `NodesPage > onClick.callback#581` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleMoveNodes' |
| 6949–6949 | function | `NodesPage > onClick.callback#582` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleMoveNodes' |
| 6956–6956 | function | `NodesPage > onClick.callback#583` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelectedNodeIds'、'setSortMode' |
| 6979–6979 | function | `NodesPage > onMinimize.callback#584` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSpeedDialogMin' |
| 6980–6980 | function | `NodesPage > onClose.callback#585` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSpeedDialogMin'、'setSpeedDialogOpen' |
| 6993–6993 | function | `NodesPage > onClick.callback#586` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSpeedDialogMin' |

## `routes/nodes.tsx`

依赖：`@tanstack/react-router`、`@tanstack/react-router`、`@/stores/auth-store`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 7–15 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 8–13 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 17–19 | function | `NodesShell` | 渲染并协调 'NodesShell' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |

## `routes/probe.tsx`

依赖：`react`、`@tanstack/react-query`、`@tanstack/react-router`、`lucide-react`、`sonner`、`@/components/layout/topbar`、`@/lib/api`、`@/lib/handle-server-error`、`@/lib/profile`、`@/stores/auth-store`、`@/components/data-table`、`@/components/data-table`、`@/components/ui/button`、`@/components/ui/card`、`@/components/ui/input`、`@/components/ui/label`、`@/components/ui/select`、`@/components/ui/table`、`@/components/ui/alert-dialog`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 52–59 | type | `ServerForm` | 定义 'ServerForm' 的数据契约、联合类型或组件属性。 |  |
| 61–75 | type | `ProbeConfigResponse` | 定义 'ProbeConfigResponse' 的数据契约、联合类型或组件属性。 |  |
| 77–82 | const | `PROBE_TYPES` | 保存 'PROBE_TYPES' 的模块级常量、配置、路由或预计算值。 |  |
| 84–88 | const | `TRAFFIC_METHODS` | 保存 'TRAFFIC_METHODS' 的模块级常量、配置、路由或预计算值。 |  |
| 91–99 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 92–97 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 101–845 | function | `ProbeManagePage` | 渲染并协调 'ProbeManagePage' React 组件的状态、数据请求和用户交互。 | 分支 9；循环 0；返回 3；await 0；调用 '<ArrayLiteralExpression>.includes'、'Boolean'、'PROBE_TYPES.map'、'formState.servers.map'、'useAuthStore'、'useEffect'、'useMemo'、'useMutation'、'useQuery'、'useQueryClient'、'useState' |
| 112–115 | function | `ProbeManagePage > generateKey` | 生成与 'generateKey' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'Math.random'、'Math.random.toString'、'Math.random.toString.slice'、'crypto.randomUUID' |
| 128–131 | function | `ProbeManagePage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 136–162 | function | `ProbeManagePage > useEffect.callback#5` | 封装 'useEffect.callback#5' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 '<BinaryExpression>.map'、'<BinaryExpression>.toLowerCase'、'<BinaryExpression>.toLowerCase.trim'、'PROBE_TYPES.some'、'config.address.trim'、'setFormState' |
| 144–144 | function | `ProbeManagePage > useEffect.callback#5 > PROBE_TYPES.some.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 151–160 | function | `ProbeManagePage > useEffect.callback#5 > <BinaryExpression>.map.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Number'、'Number.isFinite' |
| 165–170 | function | `ProbeManagePage > useMemo.callback#8` | 封装 'useMemo.callback#8' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 174–184 | function | `ProbeManagePage > useMemo.callback#9` | 封装 'useMemo.callback#9' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 3；await 0；调用 'date.getTime'、'dateFormatter.format'、'isNaN'、'updatedAt.startsWith' |
| 187–210 | function | `ProbeManagePage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put'、'formState.address.trim'、'formState.servers.map' |
| 191–205 | function | `ProbeManagePage > mutationFn > formState.servers.map.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 '<ArrowFunction>'、'server.name.trim'、'server.server_id.trim' |
| 195–204 | function | `ProbeManagePage > mutationFn > formState.servers.map.callback#11 > <anonymous#12>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 3；await 0；调用 'Number'、'server.monthly_traffic_gb.trim' |
| 211–216 | function | `ProbeManagePage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'queryClient.setQueryData'、'toast.success' |
| 221–224 | function | `ProbeManagePage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.delete' |
| 225–236 | function | `ProbeManagePage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setFormState'、'toast.success' |
| 240–265 | function | `ProbeManagePage > handleServerChange` | 处理与 'handleServerChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormState' |
| 245–264 | function | `ProbeManagePage > handleServerChange > setFormState.callback#17` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 1；await 0；调用 'raw.trim'、'servers.splice' |
| 267–281 | function | `ProbeManagePage > handleAddServer` | 处理与 'handleAddServer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormState' |
| 268–280 | function | `ProbeManagePage > handleAddServer > setFormState.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'generateKey' |
| 283–289 | function | `ProbeManagePage > handleRemoveServer` | 处理与 'handleRemoveServer' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormState' |
| 284–288 | function | `ProbeManagePage > handleRemoveServer > setFormState.callback#21` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'servers.splice' |
| 291–291 | function | `ProbeManagePage > trimAddress` | 执行与 'trimAddress' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'formState.address.trim'、'formState.address.trim.replace' |
| 293–307 | function | `ProbeManagePage > fetchDstatusServers` | 从后端获取与 'fetchDstatusServers' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post'、'servers.map' |
| 300–306 | function | `ProbeManagePage > fetchDstatusServers > servers.map.callback#24` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'generateKey' |
| 309–323 | function | `ProbeManagePage > fetchNezhaServers` | 从后端获取与 'fetchNezhaServers' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post'、'servers.map' |
| 316–322 | function | `ProbeManagePage > fetchNezhaServers > servers.map.callback#26` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'generateKey' |
| 325–339 | function | `ProbeManagePage > fetchKomariServers` | 从后端获取与 'fetchKomariServers' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post'、'servers.map' |
| 332–338 | function | `ProbeManagePage > fetchKomariServers > servers.map.callback#28` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'generateKey' |
| 341–355 | function | `ProbeManagePage > fetchNezhaV0Servers` | 从后端获取与 'fetchNezhaV0Servers' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post'、'servers.map' |
| 348–354 | function | `ProbeManagePage > fetchNezhaV0Servers > servers.map.callback#30` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'generateKey' |
| 357–399 | function | `ProbeManagePage > handleSyncServers` | 处理与 'handleSyncServers' 对应的前端业务、状态或数据转换逻辑。 | 分支 7；循环 0；返回 3；await 4；调用 'console.error'、'fetchDstatusServers'、'fetchKomariServers'、'fetchNezhaServers'、'fetchNezhaV0Servers'、'formState.address.trim'、'setFormState'、'setSyncLoading'、'toast.error'、'toast.success'、'trimAddress' |
| 386–389 | function | `ProbeManagePage > handleSyncServers > setFormState.callback#32` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 401–449 | function | `ProbeManagePage > handleSubmit` | 处理与 'handleSubmit' 对应的前端业务、状态或数据转换逻辑。 | 分支 7；循环 1；返回 7；await 0；调用 '<ArrowFunction>'、'Number.isFinite'、'event.preventDefault'、'formState.address.trim'、'mutation.mutate'、'server.name.trim'、'server.server_id.trim'、'toast.error' |
| 432–441 | function | `ProbeManagePage > handleSubmit > <anonymous#34>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 3；await 0；调用 'Number'、'server.monthly_traffic_gb.trim' |
| 522–522 | function | `ProbeManagePage > onClick.callback#35` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'deleteMutation.mutate' |
| 552–553 | function | `ProbeManagePage > onValueChange.callback#36` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormState' |
| 553–553 | function | `ProbeManagePage > onValueChange.callback#36 > setFormState.callback#37` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 560–564 | function | `ProbeManagePage > PROBE_TYPES.map.callback#38` | 渲染并协调 'PROBE_TYPES.map.callback#38' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 573–574 | function | `ProbeManagePage > onChange.callback#39` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormState' |
| 574–574 | function | `ProbeManagePage > onChange.callback#39 > setFormState.callback#40` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 617–713 | function | `ProbeManagePage > formState.servers.map.callback#41` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 1；await 0；调用 'Number'、'Number.isFinite'、'TRAFFIC_METHODS.map' |
| 637–637 | function | `ProbeManagePage > formState.servers.map.callback#41 > onClick.callback#42` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleRemoveServer' |
| 657–658 | function | `ProbeManagePage > formState.servers.map.callback#41 > onValueChange.callback#43` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerChange' |
| 665–669 | function | `ProbeManagePage > formState.servers.map.callback#41 > TRAFFIC_METHODS.map.callback#44` | 渲染并协调 'TRAFFIC_METHODS.map.callback#44' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 688–693 | function | `ProbeManagePage > formState.servers.map.callback#41 > onChange.callback#45` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerChange' |
| 735–828 | function | `ProbeManagePage > formState.servers.map.callback#46` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 1；await 0；调用 'Number'、'Number.isFinite'、'TRAFFIC_METHODS.map' |
| 767–768 | function | `ProbeManagePage > formState.servers.map.callback#46 > onValueChange.callback#47` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerChange' |
| 775–779 | function | `ProbeManagePage > formState.servers.map.callback#46 > TRAFFIC_METHODS.map.callback#48` | 渲染并协调 'TRAFFIC_METHODS.map.callback#48' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 794–799 | function | `ProbeManagePage > formState.servers.map.callback#46 > onChange.callback#49` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerChange' |
| 821–821 | function | `ProbeManagePage > formState.servers.map.callback#46 > onClick.callback#50` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleRemoveServer' |

## `routes/rules.tsx`

依赖：`react`、`@tanstack/react-query`、`@tanstack/react-router`、`js-yaml`、`sonner`、`@/components/layout/topbar`、`@/lib/api`、`@/lib/handle-server-error`、`@/lib/profile`、`@/lib/utils`、`@/stores/auth-store`、`@/components/ui/card`、`@/components/ui/button`、`@/components/ui/badge`、`@/components/ui/scroll-area`、`@/components/ui/textarea`、`@/components/ui/separator`、`@/components/ui/skeleton`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 28–41 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 29–34 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 35–39 | function | `validateSearch` | 校验与 'validateSearch' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 43–417 | function | `RulesPage` | 渲染并协调 'RulesPage' React 组件的状态、数据请求和用户交互。 | 分支 10；循环 0；返回 3；await 0；调用 'Array.from'、'Array.from.map'、'Boolean'、'Route.useSearch'、'files.map'、'historyList.map'、'useAuthStore'、'useEffect'、'useMemo'、'useMutation'、'useQuery'、'useQueryClient'、'useState' |
| 62–67 | function | `RulesPage > useMemo.callback#4` | 封装 'useMemo.callback#4' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 73–83 | function | `RulesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 88–96 | function | `RulesPage > useEffect.callback#6` | 封装 'useEffect.callback#6' Hook 的响应式状态、副作用和复用逻辑。 | 分支 4；循环 0；返回 3；await 0；调用 'setSelectedFile' |
| 100–108 | function | `RulesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 1；调用 'api.get'、'encodeURIComponent' |
| 113–118 | function | `RulesPage > useEffect.callback#8` | 封装 'useEffect.callback#8' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setEditorValue'、'setIsDirty'、'setValidationError' |
| 122–133 | function | `RulesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 1；调用 'api.get'、'encodeURIComponent' |
| 139–144 | function | `RulesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put'、'encodeURIComponent' |
| 145–152 | function | `RulesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setIsDirty'、'setValidationError'、'toast.success' |
| 153–155 | function | `RulesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError' |
| 163–167 | function | `RulesPage > useMemo.callback#13` | 封装 'useMemo.callback#13' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 171–199 | function | `RulesPage > useEffect.callback#14` | 封装 'useEffect.callback#14' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 3；await 0；调用 'setTimeout'、'setValidationError' |
| 180–194 | function | `RulesPage > useEffect.callback#14 > setTimeout.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 1；await 0；调用 'editorValue.trim'、'parseYAML'、'setValidationError' |
| 196–198 | function | `RulesPage > useEffect.callback#14 > <anonymous#16>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'clearTimeout' |
| 201–207 | function | `RulesPage > handleSelectFile` | 处理与 'handleSelectFile' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setEditorValue'、'setIsDirty'、'setSelectedFile'、'setValidationError' |
| 209–222 | function | `RulesPage > handleSave` | 处理与 'handleSave' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'parseYAML'、'saveMutation.mutate'、'setValidationError'、'toast.error' |
| 224–229 | function | `RulesPage > handleReset` | 处理与 'handleReset' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setEditorValue'、'setIsDirty'、'setValidationError' |
| 278–280 | function | `RulesPage > Array.from.map.callback#20` | 渲染并协调 'Array.from.map.callback#20' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 286–303 | function | `RulesPage > files.map.callback#21` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 1；await 0；调用 'cn'、'labels.join' |
| 294–294 | function | `RulesPage > files.map.callback#21 > onClick.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleSelectFile' |
| 358–365 | function | `RulesPage > onChange.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setEditorValue'、'setIsDirty'、'setValidationError' |
| 383–385 | function | `RulesPage > Array.from.map.callback#24` | 渲染并协调 'Array.from.map.callback#24' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 392–406 | function | `RulesPage > historyList.map.callback#25` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'dateFormatter.format' |

## `routes/settings.tsx`

依赖：`react`、`react-hook-form`、`@tanstack/react-query`、`@tanstack/react-router`、`lucide-react`、`qrcode.react`、`sonner`、`@/stores/auth-store`、`@/lib/api`、`@/lib/cookies`、`@/lib/handle-server-error`、`@/lib/profile`、`@/components/ui/avatar`、`@/components/ui/button`、`@/components/ui/card`、`@/components/ui/dialog`、`@/components/ui/input`、`@/components/ui/input-otp`、`@/components/ui/label`、`@/components/layout/topbar`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 38–43 | type | `ProfileFormValues` | 定义 'ProfileFormValues' 的数据契约、联合类型或组件属性。 |  |
| 45–49 | type | `PasswordFormValues` | 定义 'PasswordFormValues' 的数据契约、联合类型或组件属性。 |  |
| 51–59 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 52–57 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 61–545 | function | `SettingsPage` | 渲染并协调 'SettingsPage' React 组件的状态、数据请求和用户交互。 | 分支 10；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.map'、'Boolean'、'displayName.slice'、'passwordForm.handleSubmit'、'passwordForm.register'、'profile.avatar_url.trim'、'profileForm.handleSubmit'、'profileForm.register'、'useAuthStore'、'useEffect'、'useForm'、'useMutation'、'useNavigate'、'useQuery'、'useQueryClient'、'useState' |
| 75–78 | function | `SettingsPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 92–101 | function | `SettingsPage > useEffect.callback#4` | 封装 'useEffect.callback#4' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'profileForm.reset' |
| 104–113 | function | `SettingsPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put'、'values.avatar_url.trim'、'values.email.trim'、'values.nickname.trim'、'values.username.trim' |
| 114–117 | function | `SettingsPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 118–121 | function | `SettingsPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 125–128 | function | `SettingsPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 129–132 | function | `SettingsPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.setQueryData'、'toast.success' |
| 133–136 | function | `SettingsPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 140–143 | function | `SettingsPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 144–148 | function | `SettingsPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 149–152 | function | `SettingsPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 157–160 | function | `SettingsPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 166–170 | function | `SettingsPage > useEffect.callback#15` | 封装 'useEffect.callback#15' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setShortCodeInput' |
| 173–178 | function | `SettingsPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post'、'code.trim' |
| 179–182 | function | `SettingsPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 183–185 | function | `SettingsPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError' |
| 197–203 | function | `SettingsPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 204–209 | function | `SettingsPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'auth.reset'、'navigate'、'passwordForm.reset'、'toast.success' |
| 210–213 | function | `SettingsPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 216–228 | function | `SettingsPage > profileForm.handleSubmit.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 2；await 0；调用 'toast.error'、'updateProfileMutation.mutate'、'values.username.trim' |
| 230–242 | function | `SettingsPage > passwordForm.handleSubmit.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 2；await 0；调用 'changePasswordMutation.mutate'、'toast.error'、'values.new_password.trim' |
| 356–381 | function | `SettingsPage > <ArrayLiteralExpression>.map.callback#24` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'getCookie' |
| 360–371 | function | `SettingsPage > <ArrayLiteralExpression>.map.callback#24 > onClick.callback#25` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'getCookie'、'setCookie'、'window.location.reload' |
| 400–400 | function | `SettingsPage > onChange.callback#26` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setShortCodeInput' |
| 406–406 | function | `SettingsPage > onClick.callback#27` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateShortCodeMutation.mutate' |
| 490–505 | function | `SettingsPage > onClick.callback#28` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 2；await 1；调用 'console.error'、'navigator.clipboard.writeText'、'toast.error'、'toast.success' |
| 513–513 | function | `SettingsPage > onClick.callback#29` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'resetTokenMutation.mutate' |
| 528–528 | function | `SettingsPage > onClick.callback#30` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'resetShortLinkMutation.mutate' |
| 547–881 | function | `TwoFactorCard` | 渲染并协调 'TwoFactorCard' React 组件的状态、数据请求和用户交互。 | 分支 5；循环 0；返回 1；await 0；调用 'recoveryCodes.map'、'useMutation'、'useQuery'、'useQueryClient'、'useState' |
| 568–571 | function | `TwoFactorCard > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 576–579 | function | `TwoFactorCard > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 580–584 | function | `TwoFactorCard > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setSetupStep'、'setTotpSecret'、'setTotpUrl' |
| 585–588 | function | `TwoFactorCard > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 592–595 | function | `TwoFactorCard > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 596–600 | function | `TwoFactorCard > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setRecoveryCodes'、'setSetupStep' |
| 601–605 | function | `TwoFactorCard > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'setVerifyCode'、'toast.error' |
| 609–611 | function | `TwoFactorCard > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post' |
| 612–617 | function | `TwoFactorCard > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setDisableCode'、'setDisableOpen'、'toast.success' |
| 618–622 | function | `TwoFactorCard > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'setDisableCode'、'toast.error' |
| 625–632 | function | `TwoFactorCard > resetSetup` | 重置与 'resetSetup' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setRecoveryCodes'、'setSetupPassword'、'setSetupStep'、'setTotpSecret'、'setTotpUrl'、'setVerifyCode' |
| 650–650 | function | `TwoFactorCard > onClick.callback#43` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setDisableOpen' |
| 657–660 | function | `TwoFactorCard > onClick.callback#44` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'resetSetup'、'setSetupOpen' |
| 670–675 | function | `TwoFactorCard > onOpenChange.callback#45` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'resetSetup'、'setSetupOpen' |
| 679–681 | function | `TwoFactorCard > onInteractOutside.callback#46` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'e.preventDefault' |
| 705–705 | function | `TwoFactorCard > onChange.callback#47` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSetupPassword' |
| 706–709 | function | `TwoFactorCard > onKeyDown.callback#48` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setupMutation.mutate' |
| 715–715 | function | `TwoFactorCard > onClick.callback#49` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setupMutation.mutate' |
| 735–735 | function | `TwoFactorCard > onClick.callback#50` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSetupStep' |
| 748–748 | function | `TwoFactorCard > onComplete.callback#51` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'verifySetupMutation.mutate' |
| 768–768 | function | `TwoFactorCard > onClick.callback#52` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'verifySetupMutation.mutate' |
| 778–782 | function | `TwoFactorCard > recoveryCodes.map.callback#53` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 787–796 | function | `TwoFactorCard > onClick.callback#54` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 1；调用 'navigator.clipboard.writeText'、'recoveryCodes.join'、'toast.error'、'toast.success' |
| 802–811 | function | `TwoFactorCard > onClick.callback#55` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'URL.createObjectURL'、'URL.revokeObjectURL'、'a.click'、'document.createElement'、'recoveryCodes.join' |
| 819–822 | function | `TwoFactorCard > onClick.callback#56` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'resetSetup'、'setSetupOpen' |
| 833–838 | function | `TwoFactorCard > onOpenChange.callback#57` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setDisableCode'、'setDisableOpen' |
| 853–853 | function | `TwoFactorCard > onComplete.callback#58` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'disableMutation.mutate' |
| 872–872 | function | `TwoFactorCard > onClick.callback#59` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'disableMutation.mutate' |

## `routes/subscribe-files.custom.tsx`

依赖：`@tanstack/react-router`、`@/stores/auth-store`、`@/components/ui/card`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–13 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 6–11 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 15–42 | function | `CustomProxyGroupPage` | 渲染并协调 'CustomProxyGroupPage' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |

## `routes/subscribe-files.index.tsx`

依赖：`react`、`@tanstack/react-router`、`@tanstack/react-query`、`js-yaml`、`sonner`、`date-fns`、`@/stores/auth-store`、`@/lib/api`、`@/lib/handle-server-error`、`@/hooks/use-media-query`、`@/components/data-table`、`@/components/data-table`、`@/components/ui/button`、`@/components/ui/input`、`@/components/ui/textarea`、`@/components/ui/card`、`@/components/ui/collapsible`、`@/components/ui/badge`、`@/components/ui/dialog`、`@/components/ui/alert-dialog`、`@/components/ui/tooltip`、`@/components/ui/label`、`@/components/ui/switch`、`@/components/ui/select`、`@/components/ui/progress`、`@/components/ui/checkbox`、`@/components/ui/tabs`、`@/components/ui/scroll-area`、`@/components/ui/calendar`、`@/components/ui/popover`、`@/lib/utils`、`lucide-react`、`lucide-react`、`@/components/edit-nodes-dialog`、`@/components/mobile-edit-nodes-dialog`、`@/components/twemoji`、`@/hooks/use-proxy-groups`、`@/lib/sublink/translations`、`@/lib/clash-validator`、`@/components/external-sync-node-dialog`、`@/hooks/use-external-sync-selection`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 44–52 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 45–50 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 54–74 | type | `SubscribeFile` | 定义 'SubscribeFile' 的数据契约、联合类型或组件属性。 |  |
| 76–80 | const | `TYPE_COLORS` | 保存 'TYPE_COLORS' 的模块级常量、配置、路由或预计算值。 |  |
| 82–86 | const | `TYPE_LABELS` | 保存 'TYPE_LABELS' 的模块级常量、配置、路由或预计算值。 |  |
| 88–104 | type | `ExternalSubscription` | 定义 'ExternalSubscription' 的数据契约、联合类型或组件属性。 |  |
| 106–128 | type | `ProxyProviderConfig` | 定义 'ProxyProviderConfig' 的数据契约、联合类型或组件属性。 |  |
| 131–134 | const | `PROXY_TYPES` | 保存 'PROXY_TYPES' 的模块级常量、配置、路由或预计算值。 |  |
| 138–152 | const | `REGION_CONFIGS` | 保存 'REGION_CONFIGS' 的模块级常量、配置、路由或预计算值。 |  |
| 155–168 | const | `PROTOCOL_CONFIGS` | 保存 'PROTOCOL_CONFIGS' 的模块级常量、配置、路由或预计算值。 |  |
| 171–178 | const | `IP_VERSION_OPTIONS` | 保存 'IP_VERSION_OPTIONS' 的模块级常量、配置、路由或预计算值。 |  |
| 181–193 | type | `OverrideForm` | 定义 'OverrideForm' 的数据契约、联合类型或组件属性。 |  |
| 196–208 | const | `defaultOverrideForm` | 保存 'defaultOverrideForm' 的模块级常量、配置、路由或预计算值。 |  |
| 211–228 | function | `overrideFormToJSON` | 执行与 'overrideFormToJSON' 对应的前端业务、状态或数据转换逻辑。 | 分支 12；循环 0；返回 1；await 0；调用 'JSON.stringify'、'Object.keys'、'parseInt' |
| 231–252 | function | `jsonToOverrideForm` | 执行与 'jsonToOverrideForm' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 3；await 0；调用 'JSON.parse'、'obj[<key>].toString' |
| 255–260 | function | `formatTraffic` | 格式化与 'formatTraffic' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 4；await 0；调用 '<BinaryExpression>.toFixed' |
| 263–265 | function | `formatTrafficGB` | 格式化与 'formatTrafficGB' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 '<BinaryExpression>.toFixed' |
| 267–6688 | function | `SubscribeFilesPage` | 渲染并协调 'SubscribeFilesPage' React 组件的状态、数据请求和用户交互。 | 分支 47；循环 0；返回 1；await 0；调用 '<ArrowFunction>'、'Boolean'、'String'、'aggregateForm.selected_tags.join'、'files.map'、'files.some'、'useAuthStore'、'useEffect'、'useExternalSyncSelection'、'useMediaQuery'、'useMemo'、'useMutation'、'useNavigate'、'useProxyGroupCategories'、'useQuery'、'useQueryClient'、'useRef'、'useState' |
| 281–284 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 293–298 | function | `SubscribeFilesPage > useMemo.callback#8` | 封装 'useMemo.callback#8' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 451–454 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 463–469 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 473–481 | function | `SubscribeFilesPage > useMemo.callback#11` | 封装 'useMemo.callback#11' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 1；返回 1；await 0；调用 'map.set' |
| 487–490 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 499–502 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 511–514 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 522–525 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 533–536 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 544–551 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 557–557 | function | `SubscribeFilesPage > files.some.callback#18` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 562–565 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 571–574 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 579–582 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 585–586 | function | `SubscribeFilesPage > useMemo.callback#22` | 封装 'useMemo.callback#22' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 '<BinaryExpression>.filter' |
| 586–586 | function | `SubscribeFilesPage > useMemo.callback#22 > <BinaryExpression>.filter.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 590–591 | function | `SubscribeFilesPage > useMemo.callback#24` | 封装 'useMemo.callback#24' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 '<BinaryExpression>.filter' |
| 591–591 | function | `SubscribeFilesPage > useMemo.callback#24 > <BinaryExpression>.filter.callback#25` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 596–607 | function | `SubscribeFilesPage > useMemo.callback#26` | 封装 'useMemo.callback#26' Hook 的响应式状态、副作用和复用逻辑。 | 分支 3；循环 2；返回 1；await 0；调用 'grouped[<key>].push' |
| 610–618 | function | `SubscribeFilesPage > useMemo.callback#27` | 封装 'useMemo.callback#27' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 2；返回 1；await 0；调用 'Array.from'、'Array.from.sort'、'tags.add' |
| 622–625 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 626–632 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setImportDialogOpen'、'setImportForm'、'toast.success' |
| 633–635 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 640–664 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 1；await 1；调用 'String'、'api.post'、'formData.append' |
| 665–672 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setUploadDialogOpen'、'setUploadFile'、'setUploadForm'、'toast.success' |
| 673–675 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 680–682 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.put' |
| 683–686 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries' |
| 687–689 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 692–697 | function | `SubscribeFilesPage > handleMoveUp` | 处理与 'handleMoveUp' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'newFiles.map'、'reorderMutation.mutate' |
| 696–696 | function | `SubscribeFilesPage > handleMoveUp > newFiles.map.callback#38` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 699–704 | function | `SubscribeFilesPage > handleMoveDown` | 处理与 'handleMoveDown' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'newFiles.map'、'reorderMutation.mutate' |
| 703–703 | function | `SubscribeFilesPage > handleMoveDown > newFiles.map.callback#40` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 708–710 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.delete' |
| 711–715 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 716–718 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 723–732 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 733–738 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setAggregateDialogOpen'、'setAggregateForm'、'toast.success' |
| 739–741 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 746–749 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put' |
| 750–757 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setEditMetadataDialogOpen'、'setEditingMetadata'、'setMetadataForm'、'toast.success' |
| 758–760 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 766–769 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 775–777 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post' |
| 778–781 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 782–784 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 789–791 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.delete' |
| 792–796 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 797–799 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 804–821 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.put' |
| 822–826 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 827–829 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 834–837 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 838–843 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'externalSyncSelection.present'、'queryClient.invalidateQueries'、'toast.success' |
| 844–846 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 852–856 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post'、'setSyncingSingleId' |
| 857–864 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'externalSyncSelection.present'、'queryClient.invalidateQueries'、'setSyncingSingleId'、'toast.success' |
| 865–868 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setSyncingSingleId'、'toast.error' |
| 873–903 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 2；调用 'api.post'、'console.warn' |
| 904–929 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setProxyProviderDialogOpen'、'setProxyProviderForm'、'toast.success' |
| 930–932 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 937–967 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 2；调用 'api.post'、'api.put'、'console.warn' |
| 968–973 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setEditingProxyProvider'、'setProxyProviderDialogOpen'、'toast.success' |
| 974–976 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 981–983 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.delete' |
| 984–987 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 988–990 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 995–1004 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 1；调用 'Promise.allSettled'、'ids.map'、'results.filter' |
| 998–998 | function | `SubscribeFilesPage > mutationFn > ids.map.callback#76` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'api.delete' |
| 1000–1000 | function | `SubscribeFilesPage > mutationFn > results.filter.callback#77` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1005–1010 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setBatchDeleteDialogOpen'、'setSelectedProxyProviderIds'、'toast.success' |
| 1011–1016 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setBatchDeleteDialogOpen'、'setSelectedProxyProviderIds'、'toast.error' |
| 1020–1025 | function | `SubscribeFilesPage > useMemo.callback#80` | 封装 'useMemo.callback#80' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'proxyProviderConfigs.filter' |
| 1024–1024 | function | `SubscribeFilesPage > useMemo.callback#80 > proxyProviderConfigs.filter.callback#81` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1028–1034 | function | `SubscribeFilesPage > handleSelectAllProxyProviders` | 处理与 'handleSelectAllProxyProviders' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'filteredProxyProviderConfigs.map'、'setSelectedProxyProviderIds' |
| 1030–1030 | function | `SubscribeFilesPage > handleSelectAllProxyProviders > filteredProxyProviderConfigs.map.callback#83` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1037–1047 | function | `SubscribeFilesPage > handleSelectProxyProvider` | 处理与 'handleSelectProxyProvider' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelectedProxyProviderIds' |
| 1038–1046 | function | `SubscribeFilesPage > handleSelectProxyProvider > setSelectedProxyProviderIds.callback#85` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'newSet.add'、'newSet.delete' |
| 1051–1058 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 1；调用 'api.put' |
| 1059–1062 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 1063–1065 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 1070–1204 | function | `SubscribeFilesPage > handleBatchCreateByRegion` | 处理与 'handleBatchCreateByRegion' 对应的前端业务、状态或数据转换逻辑。 | 分支 8；循环 1；返回 4；await 3；调用 'JSON.stringify'、'api.get'、'api.post'、'checkRegionHasNodes'、'proNamePrefix.trim'、'queryClient.invalidateQueries'、'results.filter'、'results.push'、'setProCreatingRegion'、'setProCreationResults'、'setProNamePrefix'、'toast.error'、'toast.success' |
| 1105–1123 | function | `SubscribeFilesPage > handleBatchCreateByRegion > checkRegionHasNodesLocal` | 执行与 'checkRegionHasNodesLocal' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 2；await 0；调用 'matchedNodes.filter' |
| 1113–1113 | function | `SubscribeFilesPage > handleBatchCreateByRegion > checkRegionHasNodesLocal > matchedNodes.filter.callback#91` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'filterRegex.test' |
| 1119–1119 | function | `SubscribeFilesPage > handleBatchCreateByRegion > checkRegionHasNodesLocal > matchedNodes.filter.callback#92` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'excludeRegex.test' |
| 1126–1146 | function | `SubscribeFilesPage > handleBatchCreateByRegion > checkRegionHasNodes` | 执行与 'checkRegionHasNodes' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 3；await 1；调用 'api.post'、'checkRegionHasNodesLocal'、'console.error' |
| 1195–1195 | function | `SubscribeFilesPage > handleBatchCreateByRegion > results.filter.callback#94` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1196–1196 | function | `SubscribeFilesPage > handleBatchCreateByRegion > results.filter.callback#95` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1208–1261 | function | `SubscribeFilesPage > handleBatchCreateByProtocol` | 处理与 'handleBatchCreateByProtocol' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 1；返回 2；await 1；调用 'JSON.stringify'、'api.post'、'proNamePrefix.trim'、'queryClient.invalidateQueries'、'results.filter'、'results.push'、'setProCreatingProtocol'、'setProCreationResults'、'setProNamePrefix'、'toast.error'、'toast.success' |
| 1257–1257 | function | `SubscribeFilesPage > handleBatchCreateByProtocol > results.filter.callback#97` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1264–1286 | function | `SubscribeFilesPage > handlePreviewProxyProvider` | 处理与 'handlePreviewProxyProvider' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 1；调用 'api.get'、'setPreviewConfigName'、'setPreviewContent'、'setPreviewDialogOpen'、'setPreviewLoading'、'toast.error' |
| 1289–1373 | function | `SubscribeFilesPage > generateProxyProviderYAML` | 生成与 'generateProxyProviderYAML' 对应的前端业务、状态或数据转换逻辑。 | 分支 14；循环 0；返回 2；await 0；调用 'JSON.parse'、'dumpYAML'、'form.exclude_type.join'、'form.header_user_agent.split'、'form.header_user_agent.split.map'、'overrideFormToJSON' |
| 1327–1327 | function | `SubscribeFilesPage > generateProxyProviderYAML > form.header_user_agent.split.map.callback#100` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 's.trim' |
| 1378–1386 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 1；调用 'api.get'、'encodeURIComponent' |
| 1394–1398 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 1；调用 'api.get'、'encodeURIComponent' |
| 1406–1409 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 1417–1427 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 1434–1438 | function | `SubscribeFilesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 1；调用 'api.get'、'encodeURIComponent' |
| 1445–1450 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put'、'encodeURIComponent' |
| 1451–1460 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setEditDialogOpen'、'setEditingFile'、'setEditorValue'、'setIsDirty'、'setValidationError'、'toast.success' |
| 1461–1463 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError' |
| 1468–1473 | function | `SubscribeFilesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put'、'encodeURIComponent' |
| 1474–1481 | function | `SubscribeFilesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setConfigContent'、'setEditConfigDialogOpen'、'setEditingConfigFile'、'toast.success' |
| 1482–1484 | function | `SubscribeFilesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError' |
| 1490–1495 | function | `SubscribeFilesPage > useEffect.callback#112` | 封装 'useEffect.callback#112' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setEditorValue'、'setIsDirty'、'setValidationError' |
| 1498–1518 | function | `SubscribeFilesPage > useEffect.callback#113` | 封装 'useEffect.callback#113' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'setTimeout' |
| 1501–1515 | function | `SubscribeFilesPage > useEffect.callback#113 > setTimeout.callback#114` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 1；await 0；调用 'editorValue.trim'、'parseYAML'、'setValidationError' |
| 1517–1517 | function | `SubscribeFilesPage > useEffect.callback#113 > <anonymous#115>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'clearTimeout' |
| 1521–1524 | function | `SubscribeFilesPage > useEffect.callback#116` | 封装 'useEffect.callback#116' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setConfigContent' |
| 1527–1568 | function | `SubscribeFilesPage > useEffect.callback#117` | 封装 'useEffect.callback#117' Hook 的响应式状态、副作用和复用逻辑。 | 分支 5；循环 0；返回 1；await 0；调用 'JSON.parse'、'JSON.stringify'、'console.error'、'localStorage.getItem'、'localStorage.removeItem'、'normalize'、'parseYAML'、'parsed[<key>].map'、'setIsNodesDraftRecoveryOpen'、'setProxyGroups'、'toast.error' |
| 1534–1540 | function | `SubscribeFilesPage > useEffect.callback#117 > parsed[<key>].map.callback#118` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Array.isArray' |
| 1551–1551 | function | `SubscribeFilesPage > useEffect.callback#117 > normalize > gs.map.callback#120` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1551–1551 | function | `SubscribeFilesPage > useEffect.callback#117 > normalize` | 规范化与 'normalize' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'gs.map' |
| 1571–1579 | function | `SubscribeFilesPage > useEffect.callback#121` | 封装 'useEffect.callback#121' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'Date.now'、'JSON.parse'、'JSON.stringify'、'localStorage.setItem'、'normalize' |
| 1573–1573 | function | `SubscribeFilesPage > useEffect.callback#121 > normalize > gs.map.callback#123` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1573–1573 | function | `SubscribeFilesPage > useEffect.callback#121 > normalize` | 规范化与 'normalize' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'gs.map' |
| 1581–1587 | function | `SubscribeFilesPage > handleRecoverNodesDraft` | 处理与 'handleRecoverNodesDraft' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setIsNodesDraftRecoveryOpen'、'setProxyGroups' |
| 1589–1595 | function | `SubscribeFilesPage > handleDiscardNodesDraft` | 处理与 'handleDiscardNodesDraft' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'localStorage.removeItem'、'setIsNodesDraftRecoveryOpen' |
| 1597–1603 | function | `SubscribeFilesPage > handleEdit` | 处理与 'handleEdit' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditDialogOpen'、'setEditingFile'、'setIsDirty'、'setValidationError' |
| 1605–1618 | function | `SubscribeFilesPage > handleSave` | 处理与 'handleSave' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'parseYAML'、'saveMutation.mutate'、'setValidationError'、'toast.error' |
| 1620–1625 | function | `SubscribeFilesPage > handleReset` | 处理与 'handleReset' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setEditorValue'、'setIsDirty'、'setValidationError' |
| 1627–1633 | function | `SubscribeFilesPage > handleImport` | 处理与 'handleImport' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'importMutation.mutate'、'toast.error' |
| 1636–1652 | function | `SubscribeFilesPage > handleCreateAggregate` | 处理与 'handleCreateAggregate' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'aggregateForm.description.trim'、'aggregateForm.filename.trim'、'aggregateForm.name.trim'、'createAggregateMutation.mutate'、'toast.error' |
| 1654–1664 | function | `SubscribeFilesPage > handleUpload` | 处理与 'handleUpload' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'toast.error'、'uploadMutation.mutate' |
| 1666–1668 | function | `SubscribeFilesPage > handleDelete` | 处理与 'handleDelete' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'deleteMutation.mutate' |
| 1670–1688 | function | `SubscribeFilesPage > handleEditMetadata` | 处理与 'handleEditMetadata' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 0；await 0；调用 'String'、'setEditMetadataDialogOpen'、'setEditingMetadata'、'setMetadataForm'、'setPickerMode' |
| 1690–1721 | function | `SubscribeFilesPage > handleUpdateMetadata` | 处理与 'handleUpdateMetadata' 对应的前端业务、状态或数据转换逻辑。 | 分支 7；循环 0；返回 3；await 0；调用 '<ArrowFunction>'、'metadataForm.filename.trim'、'metadataForm.name.trim'、'parseFloat'、'toast.error'、'updateMetadataMutation.mutate' |
| 1711–1715 | function | `SubscribeFilesPage > handleUpdateMetadata > <anonymous#135>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'endOfDay.setHours'、'endOfDay.toISOString' |
| 1723–1726 | function | `SubscribeFilesPage > handleEditConfig` | 处理与 'handleEditConfig' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditConfigDialogOpen'、'setEditingConfigFile' |
| 1728–1772 | function | `SubscribeFilesPage > handleSaveConfig` | 处理与 'handleSaveConfig' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 3；await 0；调用 'clashValidationResult.issues.filter'、'console.error'、'dumpYAML'、'formatValidationIssues'、'parseYAML'、'saveConfigMutation.mutate'、'toast.error'、'toast.warning'、'validateClashConfig' |
| 1762–1762 | function | `SubscribeFilesPage > handleSaveConfig > clashValidationResult.issues.filter.callback#138` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1774–1778 | function | `SubscribeFilesPage > handleEditNodes` | 处理与 'handleEditNodes' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditNodesDialogOpen'、'setEditingNodesFile'、'setShowAllNodes' |
| 1781–1820 | function | `SubscribeFilesPage > validateRulesNodes` | 校验与 'validateRulesNodes' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'Array.from'、'parsedConfig.proxies.map'、'parsedConfig[<key>].map'、'proxyGroupNames.add'、'rules.forEach' |
| 1783–1783 | function | `SubscribeFilesPage > validateRulesNodes > parsedConfig[<key>].map.callback#141` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1784–1784 | function | `SubscribeFilesPage > validateRulesNodes > parsedConfig.proxies.map.callback#142` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1795–1815 | function | `SubscribeFilesPage > validateRulesNodes > rules.forEach.callback#143` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 2；await 0；调用 'console.log'、'missingNodes.add'、'parts[<key>].trim'、'proxyGroupNames.has'、'proxyNames.has'、'rule.split' |
| 1823–1893 | function | `SubscribeFilesPage > handleApplyReplacement` | 处理与 'handleApplyReplacement' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 0；await 1；调用 'api.put'、'console.error'、'dumpYAML'、'encodeURIComponent'、'localStorage.removeItem'、'parseYAML'、'parsedConfig.proxies.map'、'parsedConfig[<key>].map'、'proxyGroupNames.add'、'queryClient.invalidateQueries'、'queryClient.setQueryData'、'rules.map'、'setConfigContent'、'setEditNodesDialogOpen'、'setMissingNodesDialogOpen'、'toast.error'、'toas… |
| 1827–1827 | function | `SubscribeFilesPage > handleApplyReplacement > parsedConfig[<key>].map.callback#145` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1828–1828 | function | `SubscribeFilesPage > handleApplyReplacement > parsedConfig.proxies.map.callback#146` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1837–1861 | function | `SubscribeFilesPage > handleApplyReplacement > rules.map.callback#147` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 9；循环 0；返回 4；await 0；调用 'parts.join'、'parts[<key>].trim'、'proxyGroupNames.has'、'proxyNames.has'、'rule.split' |
| 1895–2477 | function | `SubscribeFilesPage > handleSaveNodes` | 处理与 'handleSaveNodes' 对应的前端业务、状态或数据转换逻辑。 | 分支 40；循环 12；返回 3；await 2；调用 'Array.isArray'、'api.get'、'console.error'、'existingProxies.forEach'、'group.proxies.filter'、'nodeConfigs.sort'、'nodeOrder.forEach'、'nodesQuery.data.nodes.forEach'、'parseYAML'、'parsed.proxies.forEach'、'parsed.proxies.map'、'proxyGroups.forEach'、'proxyGroups.map'、'proxyGroups.some'、'proxyProviderConfigs.filter'、'proxyProviderConfigs.… |
| 1903–1920 | function | `SubscribeFilesPage > handleSaveNodes > reorderProxyProperties` | 执行与 'reorderProxyProperties' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 1；await 0；调用 'Object.keys'、'Object.keys.forEach'、'parseInt' |
| 1914–1918 | function | `SubscribeFilesPage > handleSaveNodes > reorderProxyProperties > Object.keys.forEach.callback#150` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 '<ArrayLiteralExpression>.includes' |
| 1927–1927 | function | `SubscribeFilesPage > handleSaveNodes > proxyProviderConfigs.filter.callback#151` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1928–1928 | function | `SubscribeFilesPage > handleSaveNodes > proxyProviderConfigs.filter.map.callback#152` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1933–1946 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroups.forEach.callback#153` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'group.proxies.forEach'、'group.use.forEach' |
| 1936–1936 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroups.forEach.callback#153 > group.use.forEach.callback#154` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedProviderNames.add' |
| 1940–1944 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroups.forEach.callback#153 > group.proxies.forEach.callback#155` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'allMmwProviderNames.includes'、'usedProviderNames.add' |
| 1950–1950 | function | `SubscribeFilesPage > handleSaveNodes > proxyProviderConfigs.filter.callback#156` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedProviderNames.has' |
| 1962–1964 | function | `SubscribeFilesPage > handleSaveNodes > resp.data.nodes.forEach.callback#157` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'mmwNodeNames.add' |
| 1973–1981 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroups.forEach.callback#158` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'group.proxies.forEach' |
| 1974–1980 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroups.forEach.callback#158 > group.proxies.forEach.callback#159` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 '<ArrayLiteralExpression>.includes'、'proxyGroups.some'、'usedNodeNames.add' |
| 1977–1977 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroups.forEach.callback#158 > group.proxies.forEach.callback#159 > proxyGroups.some.callback#160` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1990–2005 | function | `SubscribeFilesPage > handleSaveNodes > nodesQuery.data.nodes.forEach.callback#161` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'JSON.parse'、'console.error'、'nodeConfigs.push'、'nodeNameToIdMap.set'、'reorderProxyProperties'、'usedNodeNames.has' |
| 2012–2012 | function | `SubscribeFilesPage > handleSaveNodes > nodeOrder.forEach.callback#162` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'orderMap.set' |
| 2015–2023 | function | `SubscribeFilesPage > handleSaveNodes > nodeConfigs.sort.callback#163` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 1；await 0；调用 'nodeNameToIdMap.get'、'orderMap.get' |
| 2035–2043 | function | `SubscribeFilesPage > handleSaveNodes > existingProxies.forEach.callback#164` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'mmwNodeNames.has'、'reorderProxyProperties'、'updatedProxies.push'、'updatedProxies.some'、'usedNodeNames.has' |
| 2036–2036 | function | `SubscribeFilesPage > handleSaveNodes > existingProxies.forEach.callback#164 > updatedProxies.some.callback#165` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2060–2064 | function | `SubscribeFilesPage > handleSaveNodes > nodesQuery.data.nodes.forEach.callback#166` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeIDToName.set'、'nodeNameToChainID.set'、'nodeProtocolMap.set' |
| 2068–2076 | function | `SubscribeFilesPage > handleSaveNodes > parsed.proxies.forEach.callback#167` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'nodeIDToName.get'、'nodeNameToChainID.get' |
| 2081–2096 | function | `SubscribeFilesPage > handleSaveNodes > nodesQuery.data.nodes.forEach.callback#168` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 0；await 0；调用 'node.relay_group_node_ids.map'、'node.relay_group_node_ids.map.filter'、'parsed.proxies.find'、'relayGroupMap.has'、'relayGroupMap.set' |
| 2083–2083 | function | `SubscribeFilesPage > handleSaveNodes > nodesQuery.data.nodes.forEach.callback#168 > parsed.proxies.find.callback#169` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2089–2089 | function | `SubscribeFilesPage > handleSaveNodes > nodesQuery.data.nodes.forEach.callback#168 > node.relay_group_node_ids.map.callback#170` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeIDToName.get' |
| 2099–2108 | function | `SubscribeFilesPage > handleSaveNodes > relayGroupMap.forEach.callback#171` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parsed[<key>].push' |
| 2114–2114 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroups.some.callback#172` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2116–2116 | function | `SubscribeFilesPage > handleSaveNodes > group.proxies.filter.callback#173` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2117–2124 | function | `SubscribeFilesPage > handleSaveNodes > parsed.proxies.map.callback#174` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 3；await 0；调用 'nodeNames.has'、'nodeProtocolMap.get'、'protocol.includes' |
| 2130–2145 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroups.map.callback#175` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 1；await 0 |
| 2152–2152 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroupCategories.map.callback#176` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2157–2162 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroups.forEach.callback#177` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'categoryMap.get'、'selectedCategories.push' |
| 2169–2169 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroupCategories.find.callback#178` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2209–2209 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroupCategories.find.callback#179` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2225–2225 | function | `SubscribeFilesPage > handleSaveNodes > proxyGroupCategories.find.callback#180` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2240–2240 | function | `SubscribeFilesPage > handleSaveNodes > existingRules.findIndex.callback#181` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'r.startsWith' |
| 2257–2257 | function | `SubscribeFilesPage > handleSaveNodes > proxyProviderConfigs.filter.callback#182` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedProviderNames.has' |
| 2262–2262 | function | `SubscribeFilesPage > handleSaveNodes > allMmwProviderNames.filter.callback#183` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedProviderNames.has' |
| 2267–2273 | function | `SubscribeFilesPage > handleSaveNodes > parsed[<key>].filter.callback#184` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'console.log'、'unusedMmwProviders.includes' |
| 2291–2300 | function | `SubscribeFilesPage > handleSaveNodes > parsed.proxies.filter.callback#185` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 1；返回 2；await 0；调用 'console.log'、'proxyName.startsWith' |
| 2311–2342 | function | `SubscribeFilesPage > handleSaveNodes > parsed[<key>].map.callback#186` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 1；await 0；调用 'group.use.forEach' |
| 2318–2326 | function | `SubscribeFilesPage > handleSaveNodes > parsed[<key>].map.callback#186 > group.use.forEach.callback#187` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'mmwGroupNames.push'、'newUse.push' |
| 2347–2347 | function | `SubscribeFilesPage > handleSaveNodes > data.nodes.map.callback#188` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2351–2351 | function | `SubscribeFilesPage > handleSaveNodes > parsed[<key>].findIndex.callback#189` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2380–2389 | function | `SubscribeFilesPage > handleSaveNodes > data.nodes.forEach.callback#190` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'parsed.proxies.findIndex'、'parsed.proxies.push'、'reorderProxyProperties' |
| 2383–2383 | function | `SubscribeFilesPage > handleSaveNodes > data.nodes.forEach.callback#190 > parsed.proxies.findIndex.callback#191` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2396–2412 | function | `SubscribeFilesPage > handleSaveNodes > nonMmwProviders.forEach.callback#192` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 2437–2437 | function | `SubscribeFilesPage > handleSaveNodes > clashValidationResult.issues.filter.callback#193` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2479–2490 | function | `SubscribeFilesPage > handleRemoveNodeFromGroup` | 处理与 'handleRemoveNodeFromGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'proxyGroups.map'、'setProxyGroups' |
| 2480–2488 | function | `SubscribeFilesPage > handleRemoveNodeFromGroup > proxyGroups.map.callback#195` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'group.proxies.filter' |
| 2484–2484 | function | `SubscribeFilesPage > handleRemoveNodeFromGroup > proxyGroups.map.callback#195 > group.proxies.filter.callback#196` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2493–2504 | function | `SubscribeFilesPage > handleRemoveGroup` | 处理与 'handleRemoveGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyGroups' |
| 2494–2503 | function | `SubscribeFilesPage > handleRemoveGroup > setProxyGroups.callback#198` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'filteredGroups.map'、'groups.filter' |
| 2496–2496 | function | `SubscribeFilesPage > handleRemoveGroup > setProxyGroups.callback#198 > groups.filter.callback#199` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2499–2502 | function | `SubscribeFilesPage > handleRemoveGroup > setProxyGroups.callback#198 > filteredGroups.map.callback#200` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'group.proxies.filter' |
| 2501–2501 | function | `SubscribeFilesPage > handleRemoveGroup > setProxyGroups.callback#198 > filteredGroups.map.callback#200 > group.proxies.filter.callback#201` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2507–2560 | function | `SubscribeFilesPage > handleRenameGroup` | 处理与 'handleRenameGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0；调用 'Array.isArray'、'console.error'、'dumpYAML'、'parseYAML'、'parsed[<key>].map'、'queryClient.setQueryData'、'setConfigContent'、'setProxyGroups' |
| 2508–2521 | function | `SubscribeFilesPage > handleRenameGroup > setProxyGroups.callback#203` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'groups.map' |
| 2510–2519 | function | `SubscribeFilesPage > handleRenameGroup > setProxyGroups.callback#203 > groups.map.callback#204` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'group.proxies.map' |
| 2517–2517 | function | `SubscribeFilesPage > handleRenameGroup > setProxyGroups.callback#203 > groups.map.callback#204 > group.proxies.map.callback#205` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 2529–2544 | function | `SubscribeFilesPage > handleRenameGroup > parsed[<key>].map.callback#206` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 3；await 0；调用 'parts.join'、'rule.split' |
| 2563–2591 | function | `SubscribeFilesPage > useMemo.callback#207` | 封装 'useMemo.callback#207' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 3；await 0；调用 '<ArrayLiteralExpression>.sort'、'allNodeNames.filter'、'nodeOrder.forEach'、'proxyGroups.forEach'、'sortedNodes.map' |
| 2569–2569 | function | `SubscribeFilesPage > useMemo.callback#207 > nodeOrder.forEach.callback#208` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'orderMap.set' |
| 2571–2575 | function | `SubscribeFilesPage > useMemo.callback#207 > <ArrayLiteralExpression>.sort.callback#209` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'orderMap.get' |
| 2577–2577 | function | `SubscribeFilesPage > useMemo.callback#207 > sortedNodes.map.callback#210` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2585–2587 | function | `SubscribeFilesPage > useMemo.callback#207 > proxyGroups.forEach.callback#211` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'group.proxies.forEach' |
| 2586–2586 | function | `SubscribeFilesPage > useMemo.callback#207 > proxyGroups.forEach.callback#211 > group.proxies.forEach.callback#212` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedNodes.add' |
| 2590–2590 | function | `SubscribeFilesPage > useMemo.callback#207 > allNodeNames.filter.callback#213` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'usedNodes.has' |
| 2594–2625 | function | `SubscribeFilesPage > handleEditNodesDialogOpenChange` | 处理与 'handleEditNodesDialogOpenChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setEditNodesDialogOpen'、'setTimeout' |
| 2600–2621 | function | `SubscribeFilesPage > handleEditNodesDialogOpenChange > setTimeout.callback#215` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'console.error'、'parseYAML'、'parsed[<key>].map'、'setEditingNodesFile'、'setProxyGroups'、'setShowAllNodes' |
| 2607–2612 | function | `SubscribeFilesPage > handleEditNodesDialogOpenChange > setTimeout.callback#215 > parsed[<key>].map.callback#216` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Array.isArray' |
| 2672–2672 | function | `SubscribeFilesPage > onValueChange.callback#217` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'setUploadForm' |
| 2679–2683 | function | `SubscribeFilesPage > files.map.callback#218` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'String' |
| 2692–2692 | function | `SubscribeFilesPage > onCheckedChange.callback#219` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setUploadForm' |
| 2701–2701 | function | `SubscribeFilesPage > onChange.callback#220` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setUploadFile' |
| 2712–2712 | function | `SubscribeFilesPage > onChange.callback#221` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setUploadForm' |
| 2721–2721 | function | `SubscribeFilesPage > onChange.callback#222` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setUploadForm' |
| 2730–2730 | function | `SubscribeFilesPage > onChange.callback#223` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setUploadForm' |
| 2737–2737 | function | `SubscribeFilesPage > onClick.callback#224` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setUploadDialogOpen' |
| 2747–2752 | function | `SubscribeFilesPage > onOpenChange.callback#225` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setAggregateDialogOpen'、'setAggregateForm' |
| 2777–2777 | function | `SubscribeFilesPage > onChange.callback#226` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setAggregateForm' |
| 2786–2786 | function | `SubscribeFilesPage > onChange.callback#227` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setAggregateForm' |
| 2796–2796 | function | `SubscribeFilesPage > onChange.callback#228` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setAggregateForm' |
| 2806–2833 | function | `SubscribeFilesPage > <anonymous#229>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'Array.from'、'Array.from.sort'、'externalSubs.map'、'externalSubs.map.filter'、'options.map' |
| 2807–2807 | function | `SubscribeFilesPage > <anonymous#229> > externalSubs.map.callback#230` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2809–2809 | function | `SubscribeFilesPage > <anonymous#229> > Array.from.sort.callback#231` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'a.localeCompare' |
| 2813–2832 | function | `SubscribeFilesPage > <anonymous#229> > options.map.callback#232` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'aggregateForm.selected_tags.includes'、'externalNames.includes' |
| 2822–2827 | function | `SubscribeFilesPage > <anonymous#229> > options.map.callback#232 > onClick.callback#233` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'aggregateForm.selected_tags.filter'、'setAggregateForm' |
| 2824–2824 | function | `SubscribeFilesPage > <anonymous#229> > options.map.callback#232 > onClick.callback#233 > aggregateForm.selected_tags.filter.callback#234` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2845–2848 | function | `SubscribeFilesPage > onValueChange.callback#235` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setAggregateForm' |
| 2855–2859 | function | `SubscribeFilesPage > v3Templates.map.callback#236` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2868–2868 | function | `SubscribeFilesPage > onClick.callback#237` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setAggregateDialogOpen' |
| 2886–2886 | function | `SubscribeFilesPage > onClick.callback#238` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'navigate' |
| 2906–2906 | function | `SubscribeFilesPage > getRowKey.callback#239` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2912–2919 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 2923–2932 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0 |
| 2937–3089 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'getExpirationStatus' |
| 2938–2965 | function | `SubscribeFilesPage > cell > handleQuickExpire` | 处理与 'handleQuickExpire' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 0；await 0；调用 '<NewExpression>.toISOString'、'addDays'、'addDays.toISOString'、'days.toISOString'、'updateMetadataMutation.mutate' |
| 2959–2963 | function | `SubscribeFilesPage > cell > handleQuickExpire > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setCustomDateFileId'、'setExpirePopoverFileId'、'toast.success' |
| 2967–2990 | function | `SubscribeFilesPage > cell > getExpirationStatus` | 读取或计算与 'getExpirationStatus' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 5；await 0；调用 'differenceInCalendarDays'、'format'、'isPast'、'isToday' |
| 2997–2997 | function | `SubscribeFilesPage > cell > onOpenChange.callback#246` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setExpirePopoverFileId' |
| 3019–3019 | function | `SubscribeFilesPage > cell > onClick.callback#247` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleQuickExpire' |
| 3027–3027 | function | `SubscribeFilesPage > cell > onClick.callback#248` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleQuickExpire' |
| 3034–3034 | function | `SubscribeFilesPage > cell > onOpenChange.callback#249` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setCustomDateFileId' |
| 3049–3053 | function | `SubscribeFilesPage > cell > onSelect.callback#250` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'handleQuickExpire' |
| 3054–3054 | function | `SubscribeFilesPage > cell > disabled.callback#251` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3063–3078 | function | `SubscribeFilesPage > cell > onClick.callback#252` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateMetadataMutation.mutate' |
| 3073–3076 | function | `SubscribeFilesPage > cell > onClick.callback#252 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setExpirePopoverFileId'、'toast.success' |
| 3094–3259 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 1；await 0；调用 'cn'、'enabledCustomRules.map'、'enabledOverrideScripts.map' |
| 3121–3132 | function | `SubscribeFilesPage > cell > onClick.callback#255` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateMetadataMutation.mutate' |
| 3131–3131 | function | `SubscribeFilesPage > cell > onClick.callback#255 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.success' |
| 3141–3152 | function | `SubscribeFilesPage > cell > onClick.callback#257` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateMetadataMutation.mutate' |
| 3151–3151 | function | `SubscribeFilesPage > cell > onClick.callback#257 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.success' |
| 3160–3203 | function | `SubscribeFilesPage > cell > enabledCustomRules.map.callback#259` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'cn'、'ruleIds.includes' |
| 3168–3196 | function | `SubscribeFilesPage > cell > enabledCustomRules.map.callback#259 > onClick.callback#260` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 0；await 0；调用 'enabledCustomRules.filter'、'enabledCustomRules.filter.map'、'enabledOverrideScripts.map'、'ruleIds.filter'、'ruleIds.includes'、'updateMetadataMutation.mutate' |
| 3175–3175 | function | `SubscribeFilesPage > cell > enabledCustomRules.map.callback#259 > onClick.callback#260 > enabledCustomRules.filter.callback#261` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3175–3175 | function | `SubscribeFilesPage > cell > enabledCustomRules.map.callback#259 > onClick.callback#260 > enabledCustomRules.filter.map.callback#262` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3176–3176 | function | `SubscribeFilesPage > cell > enabledCustomRules.map.callback#259 > onClick.callback#260 > enabledOverrideScripts.map.callback#263` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3178–3178 | function | `SubscribeFilesPage > cell > enabledCustomRules.map.callback#259 > onClick.callback#260 > ruleIds.filter.callback#264` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3209–3252 | function | `SubscribeFilesPage > cell > enabledOverrideScripts.map.callback#265` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 1；await 0；调用 'cn'、'scriptIds.includes' |
| 3217–3245 | function | `SubscribeFilesPage > cell > enabledOverrideScripts.map.callback#265 > onClick.callback#266` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 0；await 0；调用 'enabledCustomRules.map'、'enabledOverrideScripts.filter'、'enabledOverrideScripts.filter.map'、'scriptIds.filter'、'scriptIds.includes'、'updateMetadataMutation.mutate' |
| 3224–3224 | function | `SubscribeFilesPage > cell > enabledOverrideScripts.map.callback#265 > onClick.callback#266 > enabledOverrideScripts.filter.callback#267` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3224–3224 | function | `SubscribeFilesPage > cell > enabledOverrideScripts.map.callback#265 > onClick.callback#266 > enabledOverrideScripts.filter.map.callback#268` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3225–3225 | function | `SubscribeFilesPage > cell > enabledOverrideScripts.map.callback#265 > onClick.callback#266 > enabledCustomRules.map.callback#269` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3227–3227 | function | `SubscribeFilesPage > cell > enabledOverrideScripts.map.callback#265 > onClick.callback#266 > scriptIds.filter.callback#270` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3266–3399 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'code.slice'、'subscriptionUsersQuery.data.map' |
| 3271–3279 | function | `SubscribeFilesPage > cell > onOpenChange.callback#272` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setCustomLinkFileId'、'setCustomLinkInput'、'setUserShortCodes' |
| 3300–3300 | function | `SubscribeFilesPage > cell > onChange.callback#273` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setCustomLinkInput' |
| 3309–3324 | function | `SubscribeFilesPage > cell > onClick.callback#274` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'customLinkInput.trim'、'updateMetadataMutation.mutate' |
| 3319–3322 | function | `SubscribeFilesPage > cell > onClick.callback#274 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'customLinkInput.trim'、'setCustomLinkFileId'、'toast.success' |
| 3334–3349 | function | `SubscribeFilesPage > cell > onClick.callback#276` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateMetadataMutation.mutate' |
| 3344–3347 | function | `SubscribeFilesPage > cell > onClick.callback#276 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setCustomLinkFileId'、'toast.success' |
| 3359–3388 | function | `SubscribeFilesPage > cell > subscriptionUsersQuery.data.map.callback#278` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0 |
| 3367–3367 | function | `SubscribeFilesPage > cell > subscriptionUsersQuery.data.map.callback#278 > onChange.callback#279` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setUserShortCodes' |
| 3367–3367 | function | `SubscribeFilesPage > cell > subscriptionUsersQuery.data.map.callback#278 > onChange.callback#279 > setUserShortCodes.callback#280` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3375–3381 | function | `SubscribeFilesPage > cell > subscriptionUsersQuery.data.map.callback#278 > onClick.callback#281` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 '<BinaryExpression>.trim'、'updateUserShortCodeMutation.mutate' |
| 3407–3485 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'cn'、'v3Templates.find'、'v3Templates.map' |
| 3408–3408 | function | `SubscribeFilesPage > cell > v3Templates.find.callback#283` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3433–3447 | function | `SubscribeFilesPage > cell > onClick.callback#284` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateMetadataMutation.mutate' |
| 3443–3445 | function | `SubscribeFilesPage > cell > onClick.callback#284 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.success' |
| 3452–3480 | function | `SubscribeFilesPage > cell > v3Templates.map.callback#286` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'cn' |
| 3461–3475 | function | `SubscribeFilesPage > cell > v3Templates.map.callback#286 > onClick.callback#287` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateMetadataMutation.mutate' |
| 3471–3473 | function | `SubscribeFilesPage > cell > v3Templates.map.callback#286 > onClick.callback#287 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.success' |
| 3493–3582 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 2；await 0；调用 'allNodeTags.map'、'cn' |
| 3522–3537 | function | `SubscribeFilesPage > cell > onClick.callback#290` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateMetadataMutation.mutate' |
| 3533–3535 | function | `SubscribeFilesPage > cell > onClick.callback#290 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.success' |
| 3542–3577 | function | `SubscribeFilesPage > cell > allNodeTags.map.callback#292` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'cn'、'selectedTags.includes' |
| 3553–3571 | function | `SubscribeFilesPage > cell > allNodeTags.map.callback#292 > onClick.callback#293` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'selectedTags.filter'、'updateMetadataMutation.mutate' |
| 3555–3555 | function | `SubscribeFilesPage > cell > allNodeTags.map.callback#292 > onClick.callback#293 > selectedTags.filter.callback#294` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3567–3569 | function | `SubscribeFilesPage > cell > allNodeTags.map.callback#292 > onClick.callback#293 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'toast.success' |
| 3589–3717 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 1；await 0；调用 '<ArrowFunction>'、'String'、'probeServers.map'、'subscribeTrafficMap.get' |
| 3594–3627 | function | `SubscribeFilesPage > cell > <anonymous#297>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'Math.max'、'Math.min'、'displayTraffic.limit_gb.toFixed'、'displayTraffic.used_gb.toFixed'、'percentage.toFixed'、'remainingGB.toFixed' |
| 3645–3645 | function | `SubscribeFilesPage > cell > ref.callback#298` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3653–3685 | function | `SubscribeFilesPage > cell > probeServers.map.callback#299` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 1；await 0；调用 'currentIds.includes'、'file.stats_server_ids.split'、'file.stats_server_ids.split.map'、'file.stats_server_ids.split.map.filter' |
| 3654–3654 | function | `SubscribeFilesPage > cell > probeServers.map.callback#299 > file.stats_server_ids.split.map.callback#300` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 's.trim' |
| 3662–3679 | function | `SubscribeFilesPage > cell > probeServers.map.callback#299 > onClick.callback#301` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'currentIds.filter'、'newIds.join'、'updateMetadataMutation.mutate' |
| 3664–3664 | function | `SubscribeFilesPage > cell > probeServers.map.callback#299 > onClick.callback#301 > currentIds.filter.callback#302` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3675–3677 | function | `SubscribeFilesPage > cell > probeServers.map.callback#299 > onClick.callback#301 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries' |
| 3695–3710 | function | `SubscribeFilesPage > cell > onClick.callback#304` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'parseFloat'、'updateMetadataMutation.mutate' |
| 3706–3708 | function | `SubscribeFilesPage > cell > onClick.callback#304 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries' |
| 3724–3733 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'dateFormatter.format' |
| 3738–3804 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'files.findIndex' |
| 3739–3739 | function | `SubscribeFilesPage > cell > files.findIndex.callback#308` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3746–3746 | function | `SubscribeFilesPage > cell > onClick.callback#309` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleMoveUp' |
| 3756–3756 | function | `SubscribeFilesPage > cell > onClick.callback#310` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleMoveDown' |
| 3765–3765 | function | `SubscribeFilesPage > cell > onClick.callback#311` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleEditMetadata' |
| 3773–3773 | function | `SubscribeFilesPage > cell > onClick.callback#312` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleEditConfig' |
| 3797–3797 | function | `SubscribeFilesPage > cell > onClick.callback#313` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleDelete' |
| 3812–3853 | function | `SubscribeFilesPage > header` | 执行与 'header' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 3832–3832 | function | `SubscribeFilesPage > header > onClick.callback#315` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 3846–3846 | function | `SubscribeFilesPage > header > onClick.callback#316` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleDelete' |
| 3857–3857 | function | `SubscribeFilesPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 3858–3858 | function | `SubscribeFilesPage > hidden` | 执行与 'hidden' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 3862–3862 | function | `SubscribeFilesPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 3866–3876 | function | `SubscribeFilesPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'dateFormatter.format' |
| 3880–3990 | function | `SubscribeFilesPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 1；await 0；调用 'cn'、'enabledCustomRules.map'、'enabledOverrideScripts.map' |
| 3901–3906 | function | `SubscribeFilesPage > value > onClick.callback#322` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateMetadataMutation.mutate' |
| 3905–3905 | function | `SubscribeFilesPage > value > onClick.callback#322 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.success' |
| 3914–3919 | function | `SubscribeFilesPage > value > onClick.callback#324` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateMetadataMutation.mutate' |
| 3918–3918 | function | `SubscribeFilesPage > value > onClick.callback#324 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.success' |
| 3927–3952 | function | `SubscribeFilesPage > value > enabledCustomRules.map.callback#326` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'cn'、'ruleIds.includes' |
| 3931–3945 | function | `SubscribeFilesPage > value > enabledCustomRules.map.callback#326 > onClick.callback#327` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 1；await 0；调用 'enabledCustomRules.filter'、'enabledCustomRules.filter.map'、'enabledOverrideScripts.map'、'ruleIds.filter'、'ruleIds.includes'、'updateMetadataMutation.mutate' |
| 3936–3936 | function | `SubscribeFilesPage > value > enabledCustomRules.map.callback#326 > onClick.callback#327 > enabledCustomRules.filter.callback#328` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3936–3936 | function | `SubscribeFilesPage > value > enabledCustomRules.map.callback#326 > onClick.callback#327 > enabledCustomRules.filter.map.callback#329` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3937–3937 | function | `SubscribeFilesPage > value > enabledCustomRules.map.callback#326 > onClick.callback#327 > enabledOverrideScripts.map.callback#330` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3939–3939 | function | `SubscribeFilesPage > value > enabledCustomRules.map.callback#326 > onClick.callback#327 > ruleIds.filter.callback#331` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3958–3983 | function | `SubscribeFilesPage > value > enabledOverrideScripts.map.callback#332` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 1；await 0；调用 'cn'、'scriptIds.includes' |
| 3962–3976 | function | `SubscribeFilesPage > value > enabledOverrideScripts.map.callback#332 > onClick.callback#333` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 1；await 0；调用 'enabledCustomRules.map'、'enabledOverrideScripts.filter'、'enabledOverrideScripts.filter.map'、'scriptIds.filter'、'scriptIds.includes'、'updateMetadataMutation.mutate' |
| 3967–3967 | function | `SubscribeFilesPage > value > enabledOverrideScripts.map.callback#332 > onClick.callback#333 > enabledOverrideScripts.filter.callback#334` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3967–3967 | function | `SubscribeFilesPage > value > enabledOverrideScripts.map.callback#332 > onClick.callback#333 > enabledOverrideScripts.filter.map.callback#335` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3968–3968 | function | `SubscribeFilesPage > value > enabledOverrideScripts.map.callback#332 > onClick.callback#333 > enabledCustomRules.map.callback#336` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3970–3970 | function | `SubscribeFilesPage > value > enabledOverrideScripts.map.callback#332 > onClick.callback#333 > scriptIds.filter.callback#337` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 3994–4137 | function | `SubscribeFilesPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'getExpirationStatus' |
| 3995–4022 | function | `SubscribeFilesPage > value > handleQuickExpire` | 处理与 'handleQuickExpire' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 0；await 0；调用 '<NewExpression>.toISOString'、'addDays'、'addDays.toISOString'、'days.toISOString'、'updateMetadataMutation.mutate' |
| 4016–4020 | function | `SubscribeFilesPage > value > handleQuickExpire > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setMobileCustomDateFileId'、'setMobileExpirePopoverFileId'、'toast.success' |
| 4024–4038 | function | `SubscribeFilesPage > value > getExpirationStatus` | 读取或计算与 'getExpirationStatus' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 5；await 0；调用 'differenceInCalendarDays'、'format'、'isPast'、'isToday' |
| 4045–4045 | function | `SubscribeFilesPage > value > onOpenChange.callback#342` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setMobileExpirePopoverFileId' |
| 4067–4067 | function | `SubscribeFilesPage > value > onClick.callback#343` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleQuickExpire' |
| 4075–4075 | function | `SubscribeFilesPage > value > onClick.callback#344` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleQuickExpire' |
| 4082–4082 | function | `SubscribeFilesPage > value > onOpenChange.callback#345` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setMobileCustomDateFileId' |
| 4097–4101 | function | `SubscribeFilesPage > value > onSelect.callback#346` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'handleQuickExpire' |
| 4102–4102 | function | `SubscribeFilesPage > value > disabled.callback#347` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4111–4126 | function | `SubscribeFilesPage > value > onClick.callback#348` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateMetadataMutation.mutate' |
| 4121–4124 | function | `SubscribeFilesPage > value > onClick.callback#348 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setMobileExpirePopoverFileId'、'toast.success' |
| 4140–4162 | function | `SubscribeFilesPage > actions` | 执行与 'actions' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 4146–4146 | function | `SubscribeFilesPage > actions > onClick.callback#351` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleEditMetadata' |
| 4156–4156 | function | `SubscribeFilesPage > actions > onClick.callback#352` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleEditConfig' |
| 4195–4195 | function | `SubscribeFilesPage > onClick.callback#353` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'navigate' |
| 4205–4205 | function | `SubscribeFilesPage > onClick.callback#354` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'syncExternalSubsMutation.mutate' |
| 4222–4222 | function | `SubscribeFilesPage > getRowKey.callback#355` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4228–4239 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'Math.round' |
| 4244–4255 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 4259–4286 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'nodes.map' |
| 4274–4278 | function | `SubscribeFilesPage > cell > nodes.map.callback#359` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4292–4423 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 9；循环 0；返回 3；await 0；调用 'Math.max'、'Math.min'、'formatTrafficGB'、'percentage.toFixed' |
| 4327–4337 | function | `SubscribeFilesPage > cell > onClick.callback#361` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 0；await 0；调用 'updateExternalSubMutation.mutate' |
| 4393–4403 | function | `SubscribeFilesPage > cell > onClick.callback#362` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 0；await 0；调用 'updateExternalSubMutation.mutate' |
| 4428–4434 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'dateFormatter.format' |
| 4438–4442 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'dateFormatter.format' |
| 4446–4496 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0 |
| 4451–4462 | function | `SubscribeFilesPage > cell > onClick.callback#366` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Boolean'、'setEditExternalSubDialogOpen'、'setEditExternalSubForm'、'setEditingExternalSub' |
| 4469–4469 | function | `SubscribeFilesPage > cell > onClick.callback#367` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'syncSingleExternalSubMutation.mutate' |
| 4489–4489 | function | `SubscribeFilesPage > cell > onClick.callback#368` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'deleteExternalSubMutation.mutate' |
| 4504–4603 | function | `SubscribeFilesPage > header` | 执行与 'header' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 1；await 0；调用 'Math.round'、'nodes.map' |
| 4521–4525 | function | `SubscribeFilesPage > header > nodes.map.callback#370` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4546–4558 | function | `SubscribeFilesPage > header > onClick.callback#371` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'Boolean'、'e.stopPropagation'、'setEditExternalSubDialogOpen'、'setEditExternalSubForm'、'setEditingExternalSub' |
| 4567–4570 | function | `SubscribeFilesPage > header > onClick.callback#372` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'syncSingleExternalSubMutation.mutate' |
| 4581–4581 | function | `SubscribeFilesPage > header > onClick.callback#373` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 4595–4595 | function | `SubscribeFilesPage > header > onClick.callback#374` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'deleteExternalSubMutation.mutate' |
| 4607–4607 | function | `SubscribeFilesPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 4611–4687 | function | `SubscribeFilesPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 8；循环 0；返回 2；await 0；调用 'Math.max'、'Math.min'、'formatTrafficGB' |
| 4656–4667 | function | `SubscribeFilesPage > value > onClick.callback#377` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'updateExternalSubMutation.mutate' |
| 4691–4691 | function | `SubscribeFilesPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'dateFormatter.format' |
| 4695–4695 | function | `SubscribeFilesPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'dateFormatter.format' |
| 4738–4738 | function | `SubscribeFilesPage > onClick.callback#380` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBatchDeleteDialogOpen' |
| 4752–4756 | function | `SubscribeFilesPage > onClick.callback#381` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProCreationResults'、'setProSelectedExternalSub'、'setProxyProviderProDialogOpen' |
| 4764–4788 | function | `SubscribeFilesPage > onClick.callback#382` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingProxyProvider'、'setProxyProviderDialogOpen'、'setProxyProviderForm'、'setSelectedExternalSub' |
| 4801–4812 | function | `SubscribeFilesPage > onClick.callback#383` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'proxyProviderConfigs.every'、'proxyProviderConfigs.map'、'setProxyProviderFilterSubId'、'setSelectedProxyProviderIds' |
| 4804–4804 | function | `SubscribeFilesPage > onClick.callback#383 > proxyProviderConfigs.map.callback#384` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4806–4806 | function | `SubscribeFilesPage > onClick.callback#383 > proxyProviderConfigs.every.callback#385` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedProxyProviderIds.has' |
| 4816–4841 | function | `SubscribeFilesPage > externalSubs.map.callback#386` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 2；await 0；调用 'proxyProviderConfigs.filter'、'subConfigs.every'、'subConfigs.map' |
| 4817–4817 | function | `SubscribeFilesPage > externalSubs.map.callback#386 > proxyProviderConfigs.filter.callback#387` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4819–4819 | function | `SubscribeFilesPage > externalSubs.map.callback#386 > subConfigs.map.callback#388` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4821–4821 | function | `SubscribeFilesPage > externalSubs.map.callback#386 > subConfigs.every.callback#389` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedProxyProviderIds.has' |
| 4827–4836 | function | `SubscribeFilesPage > externalSubs.map.callback#386 > onClick.callback#390` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setProxyProviderFilterSubId'、'setSelectedProxyProviderIds' |
| 4854–4854 | function | `SubscribeFilesPage > getRowKey.callback#391` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4860–4860 | function | `SubscribeFilesPage > filteredProxyProviderConfigs.every.callback#392` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedProxyProviderIds.has' |
| 4865–4871 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedProxyProviderIds.has' |
| 4868–4868 | function | `SubscribeFilesPage > cell > onCheckedChange.callback#394` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleSelectProxyProvider' |
| 4879–4881 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 4886–4893 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'externalSubs.find' |
| 4887–4887 | function | `SubscribeFilesPage > cell > externalSubs.find.callback#397` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 4898–4920 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 0；await 0 |
| 4905–4905 | function | `SubscribeFilesPage > cell > onClick.callback#399` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleProcessModeMutation.mutate' |
| 4927–4937 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0 |
| 4942–5124 | function | `SubscribeFilesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 4950–4950 | function | `SubscribeFilesPage > cell > onClick.callback#402` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handlePreviewProxyProvider' |
| 4963–5007 | function | `SubscribeFilesPage > cell > onClick.callback#403` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 6；循环 0；返回 0；await 0；调用 'Array.isArray'、'JSON.parse'、'config.exclude_type.split'、'config.exclude_type.split.map'、'externalSubs.find'、'headerObj[<key>].join'、'jsonToOverrideForm'、'setEditingProxyProvider'、'setProxyProviderDialogOpen'、'setProxyProviderForm'、'setSelectedExternalSub' |
| 4966–4966 | function | `SubscribeFilesPage > cell > onClick.callback#403 > externalSubs.find.callback#404` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5002–5002 | function | `SubscribeFilesPage > cell > onClick.callback#403 > config.exclude_type.split.map.callback#405` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 's.trim' |
| 5019–5095 | function | `SubscribeFilesPage > cell > onClick.callback#406` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 19；循环 0；返回 1；await 0；调用 'Array.isArray'、'JSON.parse'、'dumpYAML'、'externalSubs.find'、'headerObj[<key>].join'、'headerUserAgent.split'、'headerUserAgent.split.map'、'navigator.clipboard.writeText'、'setSelectedExternalSub'、'toast.success' |
| 5021–5021 | function | `SubscribeFilesPage > cell > onClick.callback#406 > externalSubs.find.callback#407` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5064–5064 | function | `SubscribeFilesPage > cell > onClick.callback#406 > headerUserAgent.split.map.callback#408` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 's.trim' |
| 5117–5117 | function | `SubscribeFilesPage > cell > onClick.callback#409` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'deleteProxyProviderMutation.mutate' |
| 5131–5261 | function | `SubscribeFilesPage > header` | 执行与 'header' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 0；await 0；调用 'selectedProxyProviderIds.has' |
| 5136–5136 | function | `SubscribeFilesPage > header > onCheckedChange.callback#411` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleSelectProxyProvider' |
| 5137–5137 | function | `SubscribeFilesPage > header > onClick.callback#412` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 5147–5150 | function | `SubscribeFilesPage > header > onClick.callback#413` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'toggleProcessModeMutation.mutate' |
| 5173–5176 | function | `SubscribeFilesPage > header > onClick.callback#414` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'handlePreviewProxyProvider' |
| 5185–5229 | function | `SubscribeFilesPage > header > onClick.callback#415` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 6；循环 0；返回 0；await 0；调用 'Array.isArray'、'JSON.parse'、'config.exclude_type.split'、'config.exclude_type.split.map'、'e.stopPropagation'、'externalSubs.find'、'headerObj[<key>].join'、'jsonToOverrideForm'、'setEditingProxyProvider'、'setProxyProviderDialogOpen'、'setProxyProviderForm'、'setSelectedExternalSub' |
| 5189–5189 | function | `SubscribeFilesPage > header > onClick.callback#415 > externalSubs.find.callback#416` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5224–5224 | function | `SubscribeFilesPage > header > onClick.callback#415 > config.exclude_type.split.map.callback#417` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 's.trim' |
| 5239–5239 | function | `SubscribeFilesPage > header > onClick.callback#418` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 5253–5253 | function | `SubscribeFilesPage > header > onClick.callback#419` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'deleteProxyProviderMutation.mutate' |
| 5265–5268 | function | `SubscribeFilesPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'externalSubs.find' |
| 5266–5266 | function | `SubscribeFilesPage > value > externalSubs.find.callback#421` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5272–5272 | function | `SubscribeFilesPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 5286–5295 | function | `SubscribeFilesPage > onOpenChange.callback#423` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setEditDialogOpen'、'setEditingFile'、'setEditorValue'、'setIsDirty'、'setValidationError' |
| 5338–5345 | function | `SubscribeFilesPage > onChange.callback#424` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setEditorValue'、'setIsDirty'、'setValidationError' |
| 5355–5355 | function | `SubscribeFilesPage > onClick.callback#425` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditDialogOpen' |
| 5363–5369 | function | `SubscribeFilesPage > onOpenChange.callback#426` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setEditMetadataDialogOpen'、'setEditingMetadata'、'setMetadataForm' |
| 5383–5383 | function | `SubscribeFilesPage > onChange.callback#427` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMetadataForm' |
| 5392–5392 | function | `SubscribeFilesPage > onChange.callback#428` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMetadataForm' |
| 5402–5402 | function | `SubscribeFilesPage > onChange.callback#429` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMetadataForm' |
| 5428–5428 | function | `SubscribeFilesPage > onSelect.callback#430` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMetadataForm' |
| 5447–5447 | function | `SubscribeFilesPage > v3Templates.find.callback#431` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5462–5462 | function | `SubscribeFilesPage > onClick.callback#432` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMetadataForm' |
| 5467–5481 | function | `SubscribeFilesPage > v3Templates.map.callback#433` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'cn' |
| 5476–5476 | function | `SubscribeFilesPage > v3Templates.map.callback#433 > onClick.callback#434` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMetadataForm' |
| 5498–5498 | function | `SubscribeFilesPage > onClick.callback#435` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setPickerMode' |
| 5505–5505 | function | `SubscribeFilesPage > onClick.callback#436` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setPickerMode' |
| 5515–5515 | function | `SubscribeFilesPage > onChange.callback#437` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMetadataForm' |
| 5539–5539 | function | `SubscribeFilesPage > onClick.callback#438` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMetadataForm' |
| 5544–5566 | function | `SubscribeFilesPage > allNodeTags.map.callback#439` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'cn'、'metadataForm.selected_tags.includes' |
| 5555–5560 | function | `SubscribeFilesPage > allNodeTags.map.callback#439 > onClick.callback#440` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'metadataForm.selected_tags.filter'、'setMetadataForm' |
| 5557–5557 | function | `SubscribeFilesPage > allNodeTags.map.callback#439 > onClick.callback#440 > metadataForm.selected_tags.filter.callback#441` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5586–5586 | function | `SubscribeFilesPage > onChange.callback#442` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMetadataForm' |
| 5598–5616 | function | `SubscribeFilesPage > probeServers.map.callback#443` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 1；await 0；调用 'metadataForm.stats_server_ids.split'、'metadataForm.stats_server_ids.split.map'、'metadataForm.stats_server_ids.split.map.filter'、'selectedIds.includes' |
| 5599–5599 | function | `SubscribeFilesPage > probeServers.map.callback#443 > metadataForm.stats_server_ids.split.map.callback#444` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 's.trim' |
| 5606–5611 | function | `SubscribeFilesPage > probeServers.map.callback#443 > onClick.callback#445` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'newIds.join'、'selectedIds.filter'、'setMetadataForm' |
| 5608–5608 | function | `SubscribeFilesPage > probeServers.map.callback#443 > onClick.callback#445 > selectedIds.filter.callback#446` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5632–5632 | function | `SubscribeFilesPage > onClick.callback#447` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditMetadataDialogOpen' |
| 5648–5654 | function | `SubscribeFilesPage > onOpenChange.callback#448` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setConfigContent'、'setEditConfigDialogOpen'、'setEditingConfigFile' |
| 5666–5666 | function | `SubscribeFilesPage > onClick.callback#449` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleEditNodes' |
| 5687–5687 | function | `SubscribeFilesPage > onChange.callback#450` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setConfigContent' |
| 5777–5777 | function | `SubscribeFilesPage > onClick.callback#451` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Array.from'、'batchDeleteProxyProviderMutation.mutate' |
| 5787–5793 | function | `SubscribeFilesPage > onOpenChange.callback#452` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setEditingProxyProvider'、'setProxyProviderDialogOpen'、'setSelectedExternalSub' |
| 5820–5823 | function | `SubscribeFilesPage > onChange.callback#453` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'externalSubs.find'、'setSelectedExternalSub' |
| 5821–5821 | function | `SubscribeFilesPage > onChange.callback#453 > externalSubs.find.callback#454` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number' |
| 5826–5828 | function | `SubscribeFilesPage > externalSubs.map.callback#455` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5837–5837 | function | `SubscribeFilesPage > onChange.callback#456` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5837–5837 | function | `SubscribeFilesPage > onChange.callback#456 > setProxyProviderForm.callback#457` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5847–5851 | function | `SubscribeFilesPage > <anonymous#458>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0 |
| 5859–5865 | function | `SubscribeFilesPage > onClick.callback#459` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'navigator.clipboard.writeText'、'toast.success' |
| 5881–5881 | function | `SubscribeFilesPage > onChange.callback#460` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5881–5881 | function | `SubscribeFilesPage > onChange.callback#460 > setProxyProviderForm.callback#461` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5893–5893 | function | `SubscribeFilesPage > onChange.callback#462` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5893–5893 | function | `SubscribeFilesPage > onChange.callback#462 > setProxyProviderForm.callback#463` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parseInt' |
| 5901–5901 | function | `SubscribeFilesPage > onChange.callback#464` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5901–5901 | function | `SubscribeFilesPage > onChange.callback#464 > setProxyProviderForm.callback#465` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5911–5911 | function | `SubscribeFilesPage > onChange.callback#466` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5911–5911 | function | `SubscribeFilesPage > onChange.callback#466 > setProxyProviderForm.callback#467` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parseInt' |
| 5926–5926 | function | `SubscribeFilesPage > onChange.callback#468` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5926–5926 | function | `SubscribeFilesPage > onChange.callback#468 > setProxyProviderForm.callback#469` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5935–5935 | function | `SubscribeFilesPage > onChange.callback#470` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5935–5935 | function | `SubscribeFilesPage > onChange.callback#470 > setProxyProviderForm.callback#471` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5948–5948 | function | `SubscribeFilesPage > onCheckedChange.callback#472` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5948–5948 | function | `SubscribeFilesPage > onCheckedChange.callback#472 > setProxyProviderForm.callback#473` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5958–5958 | function | `SubscribeFilesPage > onChange.callback#474` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5958–5958 | function | `SubscribeFilesPage > onChange.callback#474 > setProxyProviderForm.callback#475` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 5967–5967 | function | `SubscribeFilesPage > onChange.callback#476` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5967–5967 | function | `SubscribeFilesPage > onChange.callback#476 > setProxyProviderForm.callback#477` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parseInt' |
| 5976–5976 | function | `SubscribeFilesPage > onChange.callback#478` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5976–5976 | function | `SubscribeFilesPage > onChange.callback#478 > setProxyProviderForm.callback#479` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parseInt' |
| 5985–5985 | function | `SubscribeFilesPage > onChange.callback#480` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5985–5985 | function | `SubscribeFilesPage > onChange.callback#480 > setProxyProviderForm.callback#481` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parseInt' |
| 5992–5992 | function | `SubscribeFilesPage > onCheckedChange.callback#482` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 5992–5992 | function | `SubscribeFilesPage > onCheckedChange.callback#482 > setProxyProviderForm.callback#483` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6008–6008 | function | `SubscribeFilesPage > onClick.callback#484` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6008–6008 | function | `SubscribeFilesPage > onClick.callback#484 > setProxyProviderForm.callback#485` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6017–6017 | function | `SubscribeFilesPage > onClick.callback#486` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6017–6017 | function | `SubscribeFilesPage > onClick.callback#486 > setProxyProviderForm.callback#487` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6034–6034 | function | `SubscribeFilesPage > onChange.callback#488` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6034–6034 | function | `SubscribeFilesPage > onChange.callback#488 > setProxyProviderForm.callback#489` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6044–6044 | function | `SubscribeFilesPage > onChange.callback#490` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6044–6044 | function | `SubscribeFilesPage > onChange.callback#490 > setProxyProviderForm.callback#491` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6053–6079 | function | `SubscribeFilesPage > PROXY_TYPES.map.callback#492` | 渲染并协调 'PROXY_TYPES.map.callback#492' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'proxyProviderForm.exclude_type.includes' |
| 6062–6074 | function | `SubscribeFilesPage > PROXY_TYPES.map.callback#492 > onClick.callback#493` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6064–6067 | function | `SubscribeFilesPage > PROXY_TYPES.map.callback#492 > onClick.callback#493 > setProxyProviderForm.callback#494` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'prev.exclude_type.filter' |
| 6066–6066 | function | `SubscribeFilesPage > PROXY_TYPES.map.callback#492 > onClick.callback#493 > setProxyProviderForm.callback#494 > prev.exclude_type.filter.callback#495` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6069–6072 | function | `SubscribeFilesPage > PROXY_TYPES.map.callback#492 > onClick.callback#493 > setProxyProviderForm.callback#496` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6095–6098 | function | `SubscribeFilesPage > onCheckedChange.callback#497` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6095–6098 | function | `SubscribeFilesPage > onCheckedChange.callback#497 > setProxyProviderForm.callback#498` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6106–6109 | function | `SubscribeFilesPage > onCheckedChange.callback#499` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6106–6109 | function | `SubscribeFilesPage > onCheckedChange.callback#499 > setProxyProviderForm.callback#500` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6117–6120 | function | `SubscribeFilesPage > onCheckedChange.callback#501` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6117–6120 | function | `SubscribeFilesPage > onCheckedChange.callback#501 > setProxyProviderForm.callback#502` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6128–6131 | function | `SubscribeFilesPage > onCheckedChange.callback#503` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6128–6131 | function | `SubscribeFilesPage > onCheckedChange.callback#503 > setProxyProviderForm.callback#504` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6139–6142 | function | `SubscribeFilesPage > onCheckedChange.callback#505` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6139–6142 | function | `SubscribeFilesPage > onCheckedChange.callback#505 > setProxyProviderForm.callback#506` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6154–6157 | function | `SubscribeFilesPage > onChange.callback#507` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6154–6157 | function | `SubscribeFilesPage > onChange.callback#507 > setProxyProviderForm.callback#508` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6172–6175 | function | `SubscribeFilesPage > onChange.callback#509` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6172–6175 | function | `SubscribeFilesPage > onChange.callback#509 > setProxyProviderForm.callback#510` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6185–6188 | function | `SubscribeFilesPage > onChange.callback#511` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6185–6188 | function | `SubscribeFilesPage > onChange.callback#511 > setProxyProviderForm.callback#512` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6198–6201 | function | `SubscribeFilesPage > onValueChange.callback#513` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6198–6201 | function | `SubscribeFilesPage > onValueChange.callback#513 > setProxyProviderForm.callback#514` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6207–6211 | function | `SubscribeFilesPage > IP_VERSION_OPTIONS.map.callback#515` | 渲染并协调 'IP_VERSION_OPTIONS.map.callback#515' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 6226–6229 | function | `SubscribeFilesPage > onChange.callback#516` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6226–6229 | function | `SubscribeFilesPage > onChange.callback#516 > setProxyProviderForm.callback#517` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6239–6242 | function | `SubscribeFilesPage > onChange.callback#518` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderForm' |
| 6239–6242 | function | `SubscribeFilesPage > onChange.callback#518 > setProxyProviderForm.callback#519` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6259–6263 | function | `SubscribeFilesPage > onClick.callback#520` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'generateProxyProviderYAML'、'navigator.clipboard.writeText'、'toast.success' |
| 6276–6276 | function | `SubscribeFilesPage > onClick.callback#521` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderDialogOpen' |
| 6280–6328 | function | `SubscribeFilesPage > onClick.callback#522` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 5；循环 0；返回 1；await 0；调用 'JSON.stringify'、'Object.keys'、'createProxyProviderMutation.mutate'、'overrideFormToJSON'、'proxyProviderForm.exclude_type.join'、'proxyProviderForm.header_user_agent.split'、'proxyProviderForm.header_user_agent.split.map'、'toast.error'、'updateProxyProviderMutation.mutate' |
| 6284–6284 | function | `SubscribeFilesPage > onClick.callback#522 > proxyProviderForm.header_user_agent.split.map.callback#523` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 's.trim' |
| 6354–6358 | function | `SubscribeFilesPage > missingNodes.map.callback#524` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6366–6366 | function | `SubscribeFilesPage > onClick.callback#525` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setReplacementChoice' |
| 6373–6373 | function | `SubscribeFilesPage > onClick.callback#526` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setReplacementChoice' |
| 6378–6395 | function | `SubscribeFilesPage > <anonymous#527>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 2；await 0；调用 'parseYAML'、'parsedConfig[<key>].map'、'proxyGroupNames.map' |
| 6381–6381 | function | `SubscribeFilesPage > <anonymous#527> > parsedConfig[<key>].map.callback#528` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6382–6391 | function | `SubscribeFilesPage > <anonymous#527> > proxyGroupNames.map.callback#529` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 6386–6386 | function | `SubscribeFilesPage > <anonymous#527> > proxyGroupNames.map.callback#529 > onClick.callback#530` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setReplacementChoice' |
| 6405–6405 | function | `SubscribeFilesPage > onClick.callback#531` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMissingNodesDialogOpen' |
| 6430–6435 | function | `SubscribeFilesPage > onValueChange.callback#532` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'externalSubs.find'、'setProCreationResults'、'setProNamePrefix'、'setProSelectedExternalSub' |
| 6431–6431 | function | `SubscribeFilesPage > onValueChange.callback#532 > externalSubs.find.callback#533` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parseInt' |
| 6441–6445 | function | `SubscribeFilesPage > externalSubs.map.callback#534` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'sub.id.toString' |
| 6456–6456 | function | `SubscribeFilesPage > onChange.callback#535` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProNamePrefix' |
| 6501–6501 | function | `SubscribeFilesPage > proCreationResults.filter.callback#536` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6503–6513 | function | `SubscribeFilesPage > proCreationResults.map.callback#537` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 6520–6520 | function | `SubscribeFilesPage > onClick.callback#538` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyProviderProDialogOpen' |
| 6552–6555 | function | `SubscribeFilesPage > onClick.callback#539` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'navigator.clipboard.writeText'、'toast.success' |
| 6561–6561 | function | `SubscribeFilesPage > onClick.callback#540` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setPreviewDialogOpen' |
| 6569–6574 | function | `SubscribeFilesPage > onOpenChange.callback#541` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setEditExternalSubDialogOpen'、'setEditingExternalSub' |
| 6587–6587 | function | `SubscribeFilesPage > onChange.callback#542` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditExternalSubForm' |
| 6587–6587 | function | `SubscribeFilesPage > onChange.callback#542 > setEditExternalSubForm.callback#543` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6595–6595 | function | `SubscribeFilesPage > onValueChange.callback#544` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditExternalSubForm' |
| 6595–6595 | function | `SubscribeFilesPage > onValueChange.callback#544 > setEditExternalSubForm.callback#545` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6624–6624 | function | `SubscribeFilesPage > onCheckedChange.callback#546` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditExternalSubForm' |
| 6624–6624 | function | `SubscribeFilesPage > onCheckedChange.callback#546 > setEditExternalSubForm.callback#547` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 6634–6634 | function | `SubscribeFilesPage > onValueChange.callback#548` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditExternalSubForm' |
| 6634–6634 | function | `SubscribeFilesPage > onValueChange.callback#548 > setEditExternalSubForm.callback#549` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number' |
| 6653–6653 | function | `SubscribeFilesPage > onClick.callback#550` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditExternalSubDialogOpen' |
| 6657–6671 | function | `SubscribeFilesPage > onClick.callback#551` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setEditExternalSubDialogOpen'、'setEditingExternalSub'、'updateExternalSubMutation.mutate' |
| 6692–6769 | function | `NodePickerByTag` | 渲染并协调 'NodePickerByTag' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 1；返回 2；await 0；调用 '<BinaryExpression>.trim'、'Array.from'、'Array.from.map'、'arr.push'、'groups.entries'、'groups.get'、'groups.set' |
| 6710–6710 | function | `NodePickerByTag > setIds` | 设置与 'setIds' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onChange' |
| 6711–6716 | function | `NodePickerByTag > toggleNode` | 切换与 'toggleNode' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'Array.from'、's.add'、's.delete'、's.has'、'setIds' |
| 6717–6723 | function | `NodePickerByTag > toggleGroupAll` | 切换与 'toggleGroupAll' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'Array.from'、'members.every'、'members.forEach'、'setIds' |
| 6719–6719 | function | `NodePickerByTag > toggleGroupAll > members.every.callback#556` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 's.has' |
| 6720–6720 | function | `NodePickerByTag > toggleGroupAll > members.forEach.callback#557` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 's.delete' |
| 6721–6721 | function | `NodePickerByTag > toggleGroupAll > members.forEach.callback#558` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 's.add' |
| 6736–6765 | function | `NodePickerByTag > Array.from.map.callback#559` | 渲染并协调 'Array.from.map.callback#559' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'members.every'、'members.filter'、'members.map' |
| 6737–6737 | function | `NodePickerByTag > Array.from.map.callback#559 > members.every.callback#560` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selected.has' |
| 6744–6744 | function | `NodePickerByTag > Array.from.map.callback#559 > onClick.callback#561` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleGroupAll' |
| 6751–6751 | function | `NodePickerByTag > Array.from.map.callback#559 > members.filter.callback#562` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selected.has' |
| 6755–6761 | function | `NodePickerByTag > Array.from.map.callback#559 > members.map.callback#563` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selected.has' |
| 6757–6757 | function | `NodePickerByTag > Array.from.map.callback#559 > members.map.callback#563 > onCheckedChange.callback#564` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleNode' |

## `routes/subscribe-files.tsx`

依赖：`@tanstack/react-router`、`@/stores/auth-store`、`@/components/layout/topbar`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–13 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 6–11 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 15–22 | function | `SubscribeFilesLayout` | 渲染并协调 'SubscribeFilesLayout' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |

## `routes/subscription.index.tsx`

依赖：`react`、`@tanstack/react-query`、`@tanstack/react-router`、`qrcode.react`、`lucide-react`、`sonner`、`@/components/layout/topbar`、`@/lib/api`、`@/stores/auth-store`、`@/components/ui/card`、`@/components/ui/badge`、`@/components/ui/button`、`@/components/ui/dialog`、`@/components/ui/dropdown-menu`、`@/components/ui/tooltip`、`@/assets/icons/clash_color.png`、`@/assets/icons/stash_color.png`、`@/assets/icons/shadowrocket_color.png`、`@/assets/icons/surfboard_color.png`、`@/assets/icons/surge_color.png`、`@/assets/icons/surgeformac_icon_color.png`、`@/assets/icons/loon_color.png`、`@/assets/icons/quanx_color.png`、`@/assets/icons/egern_color.png`、`@/assets/icons/sing-box_color.png`、`@/assets/icons/v2ray_color.png`、`@/assets/icons/uri-color.svg`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 67–75 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 68–73 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 77–90 | type | `SubscribeFile` | 定义 'SubscribeFile' 的数据契约、联合类型或组件属性。 |  |
| 92–96 | const | `ICON_MAP` | 保存 'ICON_MAP' 的模块级常量、配置、路由或预计算值。 |  |
| 99–116 | const | `CLIENT_TYPES` | 保存 'CLIENT_TYPES' 的模块级常量、配置、路由或预计算值。 |  |
| 118–386 | function | `SubscriptionPage` | 渲染并协调 'SubscriptionPage' React 组件的状态、数据请求和用户交互。 | 分支 3；循环 0；返回 1；await 0；调用 'Boolean'、'subscribeFiles.map'、'useAuthStore'、'useMemo'、'useQuery'、'useState' |
| 126–129 | function | `SubscriptionPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 140–143 | function | `SubscriptionPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 151–156 | function | `SubscriptionPage > useMemo.callback#5` | 封装 'useMemo.callback#5' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 166–188 | function | `SubscriptionPage > buildSubscriptionURL` | 构建与 'buildSubscriptionURL' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 2；await 0；调用 'url.searchParams.set'、'url.toString' |
| 190–208 | function | `SubscriptionPage > handleCopy` | 处理与 'handleCopy' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 1；调用 'navigator.clipboard.writeText'、'setDisplayURLs'、'toast.error'、'toast.info'、'toast.success' |
| 192–192 | function | `SubscriptionPage > handleCopy > setDisplayURLs.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 229–355 | function | `SubscriptionPage > subscribeFiles.map.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 8；循环 0；返回 1；await 0；调用 'CLIENT_TYPES.map'、'buildSubscriptionURL'、'dateFormatter.format'、'encodeURIComponent' |
| 248–248 | function | `SubscriptionPage > subscribeFiles.map.callback#9 > onClick.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setQrValue' |
| 301–301 | function | `SubscriptionPage > subscribeFiles.map.callback#9 > onClick.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleCopy' |
| 321–333 | function | `SubscriptionPage > subscribeFiles.map.callback#9 > CLIENT_TYPES.map.callback#12` | 渲染并协调 'CLIENT_TYPES.map.callback#12' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'buildSubscriptionURL' |
| 326–326 | function | `SubscriptionPage > subscribeFiles.map.callback#9 > CLIENT_TYPES.map.callback#12 > onClick.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleCopy' |
| 361–365 | function | `SubscriptionPage > onOpenChange.callback#14` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setQrValue' |

## `routes/subscription.tsx`

依赖：`@tanstack/react-router`、`@tanstack/react-router`、`@/stores/auth-store`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 7–15 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 8–13 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 17–19 | function | `SubscriptionShell` | 渲染并协调 'SubscriptionShell' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |

## `routes/system-settings.tsx`

依赖：`react`、`@tanstack/react-query`、`@tanstack/react-router`、`sonner`、`@/components/layout/topbar`、`@/components/ui/card`、`@/components/ui/button`、`@/components/ui/label`、`@/components/ui/switch`、`@/components/ui/radio-group`、`@/components/ui/input`、`@/components/ui/checkbox`、`@/components/ui/tooltip`、`@/components/ui/popover`、`lucide-react`、`@/lib/api`、`@/lib/handle-server-error`、`@/stores/auth-store`、`@/hooks/use-proxy-groups`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 27–38 | interface | `NotifyConfig` | 定义 'NotifyConfig' 的数据契约、联合类型或组件属性。 |  |
| 40–75 | interface | `UserConfig` | 定义 'UserConfig' 的数据契约、联合类型或组件属性。 |  |
| 77–85 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 78–83 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 87–1212 | function | `SystemSettingsPage` | 渲染并协调 'SystemSettingsPage' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.map'、'Boolean'、'useAuthStore'、'useEffect'、'useMutation'、'useQuery'、'useQueryClient'、'useState'、'useSyncProxyGroupCategories' |
| 147–150 | function | `SystemSettingsPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 155–159 | function | `SystemSettingsPage > useEffect.callback#4` | 封装 'useEffect.callback#4' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setNotifyConfig' |
| 162–164 | function | `SystemSettingsPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.put' |
| 165–169 | function | `SystemSettingsPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setNotifyConfig'、'toast.success' |
| 170–173 | function | `SystemSettingsPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 177–179 | function | `SystemSettingsPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post' |
| 180–182 | function | `SystemSettingsPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.success' |
| 183–186 | function | `SystemSettingsPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 189–193 | function | `SystemSettingsPage > saveNotifyConfig` | 保存与 'saveNotifyConfig' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setNotifyConfig'、'updateNotifyMutation.mutate' |
| 197–200 | function | `SystemSettingsPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 205–242 | function | `SystemSettingsPage > useEffect.callback#13` | 封装 'useEffect.callback#13' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setAppendSubInfo'、'setCacheExpireMinutes'、'setClientCompatibilityMode'、'setEnableProbeBinding'、'setEnableProxyProvider'、'setEnableShortLink'、'setEnableSubInfoNodes'、'setForceSyncExternal'、'setKeepNodeName'、'setMatchRule'、'setNodeNameFilter'、'setProxyGroupsSourceUrl'、'setSilentMode'、'setSilentModeTimeout'、'setSubInfoExpirePrefix'、'… |
| 245–247 | function | `SystemSettingsPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.put' |
| 248–289 | function | `SystemSettingsPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setAppendSubInfo'、'setCacheExpireMinutes'、'setClientCompatibilityMode'、'setEnableProbeBinding'、'setEnableProxyProvider'、'setEnableShortLink'、'setEnableSubInfoNodes'、'setForceSyncExternal'、'setKeepNodeName'、'setMatchRule'、'setNodeNameFilter'、'setProxyGroupsSourceUrl'、'setSilentMode'、'setSilentModeTim… |
| 290–293 | function | `SystemSettingsPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 297–335 | function | `SystemSettingsPage > updateConfig` | 更新与 'updateConfig' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfigMutation.mutate' |
| 371–371 | function | `SystemSettingsPage > onCheckedChange.callback#18` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 393–393 | function | `SystemSettingsPage > onCheckedChange.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 413–413 | function | `SystemSettingsPage > onChange.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeNameFilter' |
| 414–414 | function | `SystemSettingsPage > onBlur.callback#21` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 438–438 | function | `SystemSettingsPage > onCheckedChange.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 449–452 | function | `SystemSettingsPage > onValueChange.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMatchRule'、'updateConfig' |
| 481–484 | function | `SystemSettingsPage > onValueChange.callback#24` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSyncScope'、'updateConfig' |
| 520–523 | function | `SystemSettingsPage > onCheckedChange.callback#25` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setKeepNodeName'、'updateConfig' |
| 545–545 | function | `SystemSettingsPage > onChange.callback#26` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parseInt'、'setCacheExpireMinutes' |
| 546–546 | function | `SystemSettingsPage > onBlur.callback#27` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 584–584 | function | `SystemSettingsPage > onCheckedChange.callback#28` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 607–607 | function | `SystemSettingsPage > onCheckedChange.callback#29` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 632–655 | function | `SystemSettingsPage > <ArrayLiteralExpression>.map.callback#30` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0 |
| 636–636 | function | `SystemSettingsPage > <ArrayLiteralExpression>.map.callback#30 > onClick.callback#31` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 677–677 | function | `SystemSettingsPage > onCheckedChange.callback#32` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 700–700 | function | `SystemSettingsPage > onCheckedChange.callback#33` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 723–723 | function | `SystemSettingsPage > onCheckedChange.callback#34` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 744–744 | function | `SystemSettingsPage > onChange.callback#35` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNotifyConfig' |
| 745–745 | function | `SystemSettingsPage > onBlur.callback#36` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'saveNotifyConfig' |
| 754–754 | function | `SystemSettingsPage > onChange.callback#37` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNotifyConfig' |
| 755–755 | function | `SystemSettingsPage > onBlur.callback#38` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'saveNotifyConfig' |
| 763–763 | function | `SystemSettingsPage > onClick.callback#39` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'testNotifyMutation.mutate' |
| 773–773 | function | `SystemSettingsPage > onCheckedChange.callback#40` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'saveNotifyConfig' |
| 781–781 | function | `SystemSettingsPage > onCheckedChange.callback#41` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'saveNotifyConfig' |
| 789–789 | function | `SystemSettingsPage > onCheckedChange.callback#42` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'saveNotifyConfig' |
| 797–797 | function | `SystemSettingsPage > onCheckedChange.callback#43` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'saveNotifyConfig' |
| 805–805 | function | `SystemSettingsPage > onCheckedChange.callback#44` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'saveNotifyConfig' |
| 813–813 | function | `SystemSettingsPage > onCheckedChange.callback#45` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'saveNotifyConfig' |
| 820–820 | function | `SystemSettingsPage > onChange.callback#46` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNotifyConfig' |
| 821–821 | function | `SystemSettingsPage > onBlur.callback#47` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'saveNotifyConfig' |
| 845–845 | function | `SystemSettingsPage > onCheckedChange.callback#48` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'saveNotifyConfig' |
| 868–868 | function | `SystemSettingsPage > onCheckedChange.callback#49` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 891–891 | function | `SystemSettingsPage > onCheckedChange.callback#50` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 913–927 | function | `SystemSettingsPage > <ArrayLiteralExpression>.map.callback#51` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 917–917 | function | `SystemSettingsPage > <ArrayLiteralExpression>.map.callback#51 > onClick.callback#52` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 953–953 | function | `SystemSettingsPage > onChange.callback#53` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parseInt'、'setSilentModeTimeout' |
| 954–954 | function | `SystemSettingsPage > onBlur.callback#54` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 979–979 | function | `SystemSettingsPage > onCheckedChange.callback#55` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 990–990 | function | `SystemSettingsPage > onChange.callback#56` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSubInfoExpirePrefix' |
| 991–991 | function | `SystemSettingsPage > onBlur.callback#57` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 1001–1001 | function | `SystemSettingsPage > onChange.callback#58` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSubInfoTrafficPrefix' |
| 1002–1002 | function | `SystemSettingsPage > onBlur.callback#59` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 1028–1028 | function | `SystemSettingsPage > onCheckedChange.callback#60` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSkipLocalIP'、'updateConfig' |
| 1040–1040 | function | `SystemSettingsPage > onCheckedChange.callback#61` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBlockUnknownSubUA'、'updateConfig' |
| 1056–1056 | function | `SystemSettingsPage > onChange.callback#62` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'setLoginRateMaxAttempts' |
| 1060–1060 | function | `SystemSettingsPage > onChange.callback#63` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'setLoginRateWindow' |
| 1064–1064 | function | `SystemSettingsPage > onChange.callback#64` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'setLoginRateLockDuration' |
| 1078–1078 | function | `SystemSettingsPage > onCheckedChange.callback#65` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBruteForceEnabled'、'updateConfig' |
| 1084–1084 | function | `SystemSettingsPage > onChange.callback#66` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'setBruteForceMaxFailures' |
| 1088–1088 | function | `SystemSettingsPage > onChange.callback#67` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'setBruteForceWindow' |
| 1092–1092 | function | `SystemSettingsPage > onChange.callback#68` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'setBruteForceBlockDuration' |
| 1107–1107 | function | `SystemSettingsPage > onCheckedChange.callback#69` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSubRateLimitEnabled'、'updateConfig' |
| 1113–1113 | function | `SystemSettingsPage > onChange.callback#70` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'setSubRateLimitMax' |
| 1117–1117 | function | `SystemSettingsPage > onChange.callback#71` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Number'、'setSubRateLimitWindow' |
| 1125–1138 | function | `SystemSettingsPage > onClick.callback#72` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateConfig' |
| 1164–1164 | function | `SystemSettingsPage > onChange.callback#73` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setProxyGroupsSourceUrl' |
| 1165–1169 | function | `SystemSettingsPage > onBlur.callback#74` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'proxyGroupsSourceUrl.trim'、'setProxyGroupsSourceUrl'、'updateConfig' |
| 1174–1184 | function | `SystemSettingsPage > onClick.callback#75` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'proxyGroupsSourceUrl.trim'、'syncProxyGroupsMutation.mutate' |
| 1177–1179 | function | `SystemSettingsPage > onClick.callback#75 > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.success' |
| 1180–1182 | function | `SystemSettingsPage > onClick.callback#75 > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError' |
| 1214–1267 | function | `TurnstileSettings` | 渲染并协调 'TurnstileSettings' React 组件的状态、数据请求和用户交互。 | 分支 3；循环 0；返回 1；await 0；调用 'useEffect'、'useMutation'、'useQuery'、'useState' |
| 1219–1223 | function | `TurnstileSettings > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get' |
| 1225–1229 | function | `TurnstileSettings > useEffect.callback#80` | 封装 'useEffect.callback#80' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setSecretKey'、'setSiteKey' |
| 1231–1234 | function | `TurnstileSettings > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'api.put'、'secretKey.trim'、'siteKey.trim' |
| 1235–1238 | function | `TurnstileSettings > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'settings.refetch'、'toast.success' |
| 1255–1255 | function | `TurnstileSettings > onChange.callback#83` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSiteKey' |
| 1259–1259 | function | `TurnstileSettings > onChange.callback#84` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSecretKey' |
| 1262–1262 | function | `TurnstileSettings > onClick.callback#85` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'save.mutate' |

## `routes/templates-v3.index.tsx`

依赖：`react`、`@tanstack/react-router`、`@tanstack/react-query`、`sonner`、`lucide-react`、`@/components/layout/topbar`、`@/stores/auth-store`、`@/lib/api`、`@/hooks/use-media-query`、`@/lib/utils`、`@/components/data-table`、`@/components/data-table`、`@/components/ui/button`、`@/components/ui/input`、`@/components/ui/textarea`、`@/components/ui/card`、`@/components/ui/dialog`、`@/components/ui/alert-dialog`、`@/components/ui/tabs`、`@/components/ui/scroll-area`、`@/components/ui/collapsible`、`@/components/ui/badge`、`@/components/template-v3/proxy-group-editor`、`@/components/template-v3/template-preview`、`@/components/template-v3/template-upload-dialog`、`@/components/ui/switch`、`@/components/ui/label`、`@/lib/template-v3-utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–14 | const | `TEMPLATE_DRAFT_KEY_PREFIX` | 保存 'TEMPLATE_DRAFT_KEY_PREFIX' 的模块级常量、配置、路由或预计算值。 |  |
| 52–60 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 53–58 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 62–941 | function | `TemplatesV3Page` | 渲染并协调 'TemplatesV3Page' React 组件的状态、数据请求和用户交互。 | 分支 19；循环 0；返回 1；await 0；调用 'cn'、'editingTemplateName.toLowerCase'、'editingTemplateName.toLowerCase.endsWith'、'formatTemplateForDisplay'、'getRegionProxyGroupNames'、'newTemplateName.trim'、'proxyGroups.map'、'useCallback'、'useEffect'、'useMediaQuery'、'useMutation'、'useQuery'、'useQueryClient'、'useRef'、'useState' |
| 108–111 | function | `TemplatesV3Page > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 115–115 | function | `TemplatesV3Page > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get' |
| 116–116 | function | `TemplatesV3Page > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get' |
| 118–121 | function | `TemplatesV3Page > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0；调用 'api.put'、'name.endsWith' |
| 122–122 | function | `TemplatesV3Page > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 123–123 | function | `TemplatesV3Page > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 126–126 | function | `TemplatesV3Page > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'api.put' |
| 127–127 | function | `TemplatesV3Page > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 128–128 | function | `TemplatesV3Page > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 134–137 | function | `TemplatesV3Page > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get'、'encodeURIComponent' |
| 144–158 | function | `TemplatesV3Page > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get'、'nodes.map'、'nodes.map.filter' |
| 148–157 | function | `TemplatesV3Page > queryFn > nodes.map.callback#14` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 3；await 0；调用 'JSON.parse' |
| 157–157 | function | `TemplatesV3Page > queryFn > nodes.map.filter.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 164–166 | function | `TemplatesV3Page > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.put'、'encodeURIComponent' |
| 167–180 | function | `TemplatesV3Page > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'localStorage.removeItem'、'queryClient.invalidateQueries'、'setEditingTemplateName'、'setIsDirty'、'setIsEditorOpen'、'setProxyGroups'、'setTemplateContent'、'toast.success' |
| 181–183 | function | `TemplatesV3Page > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 188–190 | function | `TemplatesV3Page > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.delete'、'encodeURIComponent' |
| 191–196 | function | `TemplatesV3Page > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setDeletingTemplateName'、'setIsDeleteDialogOpen'、'toast.success' |
| 197–199 | function | `TemplatesV3Page > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 204–210 | function | `TemplatesV3Page > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post'、'formData.append' |
| 211–215 | function | `TemplatesV3Page > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setIsUploadDialogOpen'、'toast.success' |
| 216–218 | function | `TemplatesV3Page > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 223–230 | function | `TemplatesV3Page > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post'、'formData.append' |
| 231–235 | function | `TemplatesV3Page > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setIsUploadDialogOpen'、'toast.success' |
| 236–238 | function | `TemplatesV3Page > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 243–245 | function | `TemplatesV3Page > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post' |
| 246–252 | function | `TemplatesV3Page > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setIsRenameDialogOpen'、'setNewTemplateName'、'setRenamingTemplate'、'toast.success' |
| 253–255 | function | `TemplatesV3Page > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 259–306 | function | `TemplatesV3Page > useEffect.callback#31` | 封装 'useEffect.callback#31' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'extractProxyGroups'、'extractTemplateVariables'、'groups.some'、'setEditorTab'、'setEnableRegionProxyGroups'、'setIsDirty'、'setProxyGroups'、'setTemplateContent'、'setTemplateVariables'、'setTimeout' |
| 277–277 | function | `TemplatesV3Page > useEffect.callback#31 > groups.some.callback#32` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 281–304 | function | `TemplatesV3Page > useEffect.callback#31 > setTimeout.callback#33` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 0；await 0；调用 'JSON.parse'、'extractProxyGroups'、'extractTemplateVariables'、'localStorage.getItem'、'localStorage.removeItem'、'setIsDraftRecoveryOpen'、'updateProxyGroups' |
| 309–319 | function | `TemplatesV3Page > useEffect.callback#34` | 封装 'useEffect.callback#34' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'generateProxyGroupsPreview'、'setPreviewContent' |
| 322–337 | function | `TemplatesV3Page > useEffect.callback#35` | 封装 'useEffect.callback#35' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'Date.now'、'JSON.stringify'、'localStorage.setItem'、'updateProxyGroups' |
| 340–345 | function | `TemplatesV3Page > useCallback.callback#36` | 封装 'useCallback.callback#36' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setTemplateContent'、'updateProxyGroups' |
| 348–357 | function | `TemplatesV3Page > handleTabChange` | 处理与 'handleTabChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0；调用 'extractProxyGroups'、'extractTemplateVariables'、'setEditorTab'、'setProxyGroups'、'setTemplateVariables'、'syncProxyGroupsToYaml' |
| 360–365 | function | `TemplatesV3Page > handleEdit` | 处理与 'handleEdit' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingTemplateName'、'setEditorTab'、'setIsEditorOpen'、'setPreviewContent' |
| 368–371 | function | `TemplatesV3Page > handleDelete` | 处理与 'handleDelete' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setDeletingTemplateName'、'setIsDeleteDialogOpen' |
| 374–378 | function | `TemplatesV3Page > handleRename` | 处理与 'handleRename' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setIsRenameDialogOpen'、'setNewTemplateName'、'setRenamingTemplate' |
| 381–419 | function | `TemplatesV3Page > handleListPreview` | 处理与 'handleListPreview' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 3；调用 '<BinaryExpression>.map'、'<BinaryExpression>.map.filter'、'api.get'、'api.post'、'encodeURIComponent'、'setListPreviewContent'、'setListPreviewLoading'、'setListPreviewOpen'、'setListPreviewTemplateContent'、'setListPreviewTemplateName'、'toast.error' |
| 396–405 | function | `TemplatesV3Page > handleListPreview > <BinaryExpression>.map.callback#42` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 3；await 0；调用 'JSON.parse' |
| 405–405 | function | `TemplatesV3Page > handleListPreview > <BinaryExpression>.map.filter.callback#43` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 422–429 | function | `TemplatesV3Page > handleSave` | 处理与 'handleSave' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'updateMutation.mutate'、'updateProxyGroups' |
| 432–438 | function | `TemplatesV3Page > handleCloseEditor` | 处理与 'handleCloseEditor' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'doCloseEditor'、'setIsCloseConfirmOpen' |
| 440–449 | function | `TemplatesV3Page > doCloseEditor` | 执行与 'doCloseEditor' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingTemplateName'、'setEnableRegionProxyGroups'、'setIsCloseConfirmOpen'、'setIsDirty'、'setIsEditorOpen'、'setPreviewContent'、'setProxyGroups'、'setTemplateContent' |
| 451–464 | function | `TemplatesV3Page > handleRecoverDraft` | 处理与 'handleRecoverDraft' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setEditorTab'、'setEnableRegionProxyGroups'、'setIsDirty'、'setIsDraftRecoveryOpen'、'setProxyGroups'、'setTemplateContent'、'setTemplateVariables'、'setTimeout' |
| 461–461 | function | `TemplatesV3Page > handleRecoverDraft > setTimeout.callback#48` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 466–472 | function | `TemplatesV3Page > handleDiscardDraft` | 处理与 'handleDiscardDraft' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'localStorage.removeItem'、'setIsDraftRecoveryOpen' |
| 478–500 | function | `TemplatesV3Page > handleRegionProxyGroupsToggle` | 处理与 'handleRegionProxyGroupsToggle' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'generateRegionProxyGroups'、'proxyGroups.filter'、'proxyGroups.filter.map'、'setEnableRegionProxyGroups'、'setIsDirty'、'setProxyGroups' |
| 486–486 | function | `TemplatesV3Page > handleRegionProxyGroupsToggle > proxyGroups.filter.callback#51` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'regionGroupNames.includes' |
| 491–491 | function | `TemplatesV3Page > handleRegionProxyGroupsToggle > proxyGroups.filter.callback#52` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'regionGroupNames.includes' |
| 492–497 | function | `TemplatesV3Page > handleRegionProxyGroupsToggle > proxyGroups.filter.map.callback#53` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'g.proxyOrder.filter' |
| 496–496 | function | `TemplatesV3Page > handleRegionProxyGroupsToggle > proxyGroups.filter.map.callback#53 > g.proxyOrder.filter.callback#54` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 503–510 | function | `TemplatesV3Page > handleProxyGroupChange` | 处理与 'handleProxyGroupChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setIsDirty'、'setProxyGroups' |
| 513–516 | function | `TemplatesV3Page > handleProxyGroupDelete` | 处理与 'handleProxyGroupDelete' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'proxyGroups.filter'、'setIsDirty'、'setProxyGroups' |
| 514–514 | function | `TemplatesV3Page > handleProxyGroupDelete > proxyGroups.filter.callback#57` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 519–525 | function | `TemplatesV3Page > handleProxyGroupMoveUp` | 处理与 'handleProxyGroupMoveUp' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setIsDirty'、'setProxyGroups' |
| 527–533 | function | `TemplatesV3Page > handleProxyGroupMoveDown` | 处理与 'handleProxyGroupMoveDown' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setIsDirty'、'setProxyGroups' |
| 536–539 | function | `TemplatesV3Page > handleAddProxyGroup` | 处理与 'handleAddProxyGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'createDefaultFormState'、'setIsDirty'、'setProxyGroups' |
| 542–559 | function | `TemplatesV3Page > handlePreview` | 处理与 'handlePreview' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 1；调用 'api.post'、'setIsPreviewLoading'、'setPreviewContent'、'toast.error'、'updateProxyGroups' |
| 562–565 | function | `TemplatesV3Page > handleYamlChange` | 处理与 'handleYamlChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setIsDirty'、'setTemplateContent' |
| 568–573 | function | `TemplatesV3Page > formatTemplateForDisplay` | 格式化与 'formatTemplateForDisplay' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'content.replace'、'content.replace.replace'、'content.replace.replace.replace' |
| 579–579 | function | `TemplatesV3Page > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 583–597 | function | `TemplatesV3Page > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'cn' |
| 585–585 | function | `TemplatesV3Page > cell > onClick.callback#66` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleEdit' |
| 588–588 | function | `TemplatesV3Page > cell > onClick.callback#67` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleListPreview' |
| 591–591 | function | `TemplatesV3Page > cell > onClick.callback#68` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'defaultMutation.mutate' |
| 592–592 | function | `TemplatesV3Page > cell > onClick.callback#69` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'visibilityMutation.mutate' |
| 593–593 | function | `TemplatesV3Page > cell > onClick.callback#70` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleDelete' |
| 613–613 | function | `TemplatesV3Page > onClick.callback#71` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setIsUploadDialogOpen' |
| 626–626 | function | `TemplatesV3Page > getRowKey.callback#72` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 629–629 | function | `TemplatesV3Page > header` | 执行与 'header' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 630–648 | function | `TemplatesV3Page > actions` | 执行与 'actions' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'cn' |
| 632–632 | function | `TemplatesV3Page > actions > onClick.callback#75` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleEdit' |
| 635–635 | function | `TemplatesV3Page > actions > onClick.callback#76` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleListPreview' |
| 638–638 | function | `TemplatesV3Page > actions > onClick.callback#77` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'defaultMutation.mutate' |
| 641–641 | function | `TemplatesV3Page > actions > onClick.callback#78` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'visibilityMutation.mutate' |
| 644–644 | function | `TemplatesV3Page > actions > onClick.callback#79` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleDelete' |
| 655–655 | function | `TemplatesV3Page > onOpenChange.callback#80` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleCloseEditor' |
| 740–756 | function | `TemplatesV3Page > proxyGroups.map.callback#81` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'proxyGroups.map'、'regionGroupNames.includes' |
| 745–745 | function | `TemplatesV3Page > proxyGroups.map.callback#81 > proxyGroups.map.callback#82` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 768–768 | function | `TemplatesV3Page > onChange.callback#83` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleYamlChange' |
| 799–799 | function | `TemplatesV3Page > onUpload.callback#84` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'uploadMutation.mutate' |
| 800–800 | function | `TemplatesV3Page > onCreate.callback#85` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'createMutation.mutate' |
| 816–816 | function | `TemplatesV3Page > onClick.callback#86` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setListPreviewOpen' |
| 868–868 | function | `TemplatesV3Page > onClick.callback#87` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'deleteMutation.mutate' |
| 887–887 | function | `TemplatesV3Page > onChange.callback#88` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNewTemplateName' |
| 892–892 | function | `TemplatesV3Page > onClick.callback#89` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setIsRenameDialogOpen' |
| 896–896 | function | `TemplatesV3Page > onClick.callback#90` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'renameMutation.mutate' |

## `routes/templates.index.tsx`

依赖：`@tanstack/react-router`、`@tanstack/react-query`、`react`、`lucide-react`、`@/components/data-table`、`@/components/data-table`、`@/components/ui/button`、`@/components/ui/card`、`@/components/ui/dialog`、`@/components/ui/alert-dialog`、`@/components/ui/input`、`@/components/ui/label`、`@/components/ui/switch`、`@/components/ui/badge`、`@/components/ui/select`、`sonner`、`@/lib/api`、`@/lib/template-presets`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 51–53 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 55–65 | interface | `Template` | 定义 'Template' 的数据契约、联合类型或组件属性。 |  |
| 67–67 | type | `TemplateFormData` | 定义 'TemplateFormData' 的数据契约、联合类型或组件属性。 |  |
| 69–595 | function | `TemplatesPage` | 渲染并协调 'TemplatesPage' React 组件的状态、数据请求和用户交互。 | 分支 3；循环 0；返回 1；await 0；调用 '<ArrowFunction>'、'ACL4SSR_PRESETS.map'、'useMutation'、'useQuery'、'useQueryClient'、'useState' |
| 90–93 | function | `TemplatesPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 98–101 | function | `TemplatesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 102–107 | function | `TemplatesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'resetForm'、'setIsDialogOpen'、'toast.success' |
| 108–110 | function | `TemplatesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 115–121 | function | `TemplatesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.put' |
| 122–127 | function | `TemplatesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'resetForm'、'setIsDialogOpen'、'toast.success' |
| 128–130 | function | `TemplatesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 135–137 | function | `TemplatesPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.delete' |
| 138–143 | function | `TemplatesPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setDeletingTemplateId'、'setIsDeleteDialogOpen'、'toast.success' |
| 144–146 | function | `TemplatesPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 149–159 | function | `TemplatesPage > resetForm` | 重置与 'resetForm' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingTemplate'、'setFormData' |
| 161–164 | function | `TemplatesPage > handleCreate` | 处理与 'handleCreate' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'resetForm'、'setIsDialogOpen' |
| 166–177 | function | `TemplatesPage > handleEdit` | 处理与 'handleEdit' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingTemplate'、'setFormData'、'setIsDialogOpen' |
| 179–182 | function | `TemplatesPage > handleDelete` | 处理与 'handleDelete' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setDeletingTemplateId'、'setIsDeleteDialogOpen' |
| 184–208 | function | `TemplatesPage > handlePreview` | 处理与 'handlePreview' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 1；调用 'api.post'、'setIsPreviewDialogOpen'、'setIsPreviewLoading'、'setPreviewContent'、'toast.error' |
| 210–221 | function | `TemplatesPage > handleSubmit` | 处理与 'handleSubmit' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'createMutation.mutate'、'formData.name.trim'、'toast.error'、'updateMutation.mutate' |
| 223–225 | function | `TemplatesPage > handlePresetSelect` | 处理与 'handlePresetSelect' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormData' |
| 228–239 | function | `TemplatesPage > handleTemplatePresetSelect` | 处理与 'handleTemplatePresetSelect' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'allPresets.find'、'setFormData' |
| 231–231 | function | `TemplatesPage > handleTemplatePresetSelect > allPresets.find.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 242–261 | function | `TemplatesPage > getAvailablePresets` | 读取或计算与 'getAvailablePresets' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'ACL4SSR_PRESETS.filter'、'Aethersailor_PRESETS.filter'、'templates.map' |
| 243–243 | function | `TemplatesPage > getAvailablePresets > templates.map.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 244–244 | function | `TemplatesPage > getAvailablePresets > templates.map.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 247–255 | function | `TemplatesPage > getAvailablePresets > filterPreset` | 筛选与 'filterPreset' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'existingNames.has'、'existingUrls.has' |
| 266–268 | function | `TemplatesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 272–276 | function | `TemplatesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0 |
| 280–290 | function | `TemplatesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'template.rule_source.split'、'template.rule_source.split.pop' |
| 294–298 | function | `TemplatesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0 |
| 302–306 | function | `TemplatesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 310–337 | function | `TemplatesPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 315–315 | function | `TemplatesPage > cell > onClick.callback#31` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handlePreview' |
| 323–323 | function | `TemplatesPage > cell > onClick.callback#32` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleEdit' |
| 331–331 | function | `TemplatesPage > cell > onClick.callback#33` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleDelete' |
| 360–360 | function | `TemplatesPage > getRowKey.callback#34` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 385–386 | function | `TemplatesPage > onChange.callback#35` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormData' |
| 396–433 | function | `TemplatesPage > <anonymous#36>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'available.acl4ssr.map'、'available.aethersailor.map'、'getAvailablePresets' |
| 414–418 | function | `TemplatesPage > <anonymous#36> > available.aethersailor.map.callback#37` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 424–428 | function | `TemplatesPage > <anonymous#36> > available.acl4ssr.map.callback#38` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 443–444 | function | `TemplatesPage > onValueChange.callback#39` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormData' |
| 462–463 | function | `TemplatesPage > onChange.callback#40` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormData' |
| 478–479 | function | `TemplatesPage > onChange.callback#41` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormData' |
| 489–493 | function | `TemplatesPage > ACL4SSR_PRESETS.map.callback#42` | 渲染并协调 'ACL4SSR_PRESETS.map.callback#42' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 511–512 | function | `TemplatesPage > onCheckedChange.callback#43` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormData' |
| 526–527 | function | `TemplatesPage > onCheckedChange.callback#44` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setFormData' |
| 534–534 | function | `TemplatesPage > onClick.callback#45` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setIsDialogOpen' |
| 559–559 | function | `TemplatesPage > onClick.callback#46` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'deleteMutation.mutate' |

## `routes/templates.tsx`

依赖：`@tanstack/react-router`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–6 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 8–10 | function | `TemplatesLayout` | 渲染并协调 'TemplatesLayout' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |

## `routes/users.tsx`

依赖：`react`、`@tanstack/react-query`、`@tanstack/react-router`、`sonner`、`@/components/layout/topbar`、`@/components/data-table`、`@/components/data-table`、`@/components/ui/button`、`@/components/ui/card`、`@/components/ui/dialog`、`@/components/ui/input`、`@/components/ui/label`、`@/components/ui/switch`、`@/components/ui/checkbox`、`@/components/ui/badge`、`@/lib/api`、`@/lib/handle-server-error`、`@/lib/profile`、`@/stores/auth-store`、`lucide-react`、`@/components/ui/popover`、`@/components/ui/tooltip`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 39–47 | const | `Route` | 保存 'Route' 的模块级常量、配置、路由或预计算值。 |  |
| 40–45 | function | `beforeLoad` | 执行与 'beforeLoad' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'redirect'、'useAuthStore.getState' |
| 49–57 | type | `UserRow` | 定义 'UserRow' 的数据契约、联合类型或组件属性。 |  |
| 59–62 | type | `ResetState` | 定义 'ResetState' 的数据契约、联合类型或组件属性。 |  |
| 64–71 | type | `CreateState` | 定义 'CreateState' 的数据契约、联合类型或组件属性。 |  |
| 73–77 | type | `SubscriptionManageState` | 定义 'SubscriptionManageState' 的数据契约、联合类型或组件属性。 |  |
| 79–88 | type | `SubscribeFile` | 定义 'SubscribeFile' 的数据契约、联合类型或组件属性。 |  |
| 90–93 | function | `generatePassword` | 生成与 'generatePassword' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'Array.from'、'Array.from.join' |
| 92–92 | function | `generatePassword > Array.from.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Math.floor'、'Math.random' |
| 95–1034 | function | `UsersPage` | 渲染并协调 'UsersPage' React 组件的状态、数据请求和用户交互。 | 分支 11；循环 0；返回 3；await 0；调用 'Boolean'、'generatePassword'、'subscriptionsQuery.data.map'、'useAuthStore'、'useEffect'、'useMemo'、'useMutation'、'useQuery'、'useQueryClient'、'useState' |
| 125–128 | function | `UsersPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 135–138 | function | `UsersPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 145–149 | function | `UsersPage > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 1；调用 'api.get' |
| 155–157 | function | `UsersPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post' |
| 158–161 | function | `UsersPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 166–172 | function | `UsersPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 173–181 | function | `UsersPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'navigator.clipboard.writeText'、'navigator.clipboard.writeText.catch'、'queryClient.invalidateQueries'、'setResetState'、'toast.success' |
| 179–179 | function | `UsersPage > onSuccess > navigator.clipboard.writeText.catch.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 182–184 | function | `UsersPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError' |
| 188–190 | function | `UsersPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post' |
| 191–195 | function | `UsersPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setDeleteUsername'、'toast.success' |
| 196–198 | function | `UsersPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError' |
| 202–221 | function | `UsersPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 2；调用 'api.post'、'api.put' |
| 222–231 | function | `UsersPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'generatePassword'、'navigator.clipboard.writeText'、'navigator.clipboard.writeText.catch'、'queryClient.invalidateQueries'、'setCreateOpen'、'setCreateState'、'toast.success' |
| 229–229 | function | `UsersPage > onSuccess > navigator.clipboard.writeText.catch.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 232–234 | function | `UsersPage > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError' |
| 238–242 | function | `UsersPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.put' |
| 243–247 | function | `UsersPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setSubscriptionManageState'、'toast.success' |
| 252–254 | function | `UsersPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post' |
| 255–259 | function | `UsersPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setRemarkEditState'、'toast.success' |
| 264–266 | function | `UsersPage > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post' |
| 267–273 | function | `UsersPage > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setCustomCodeEditUser'、'setCustomCodeInput'、'toast.success' |
| 277–293 | function | `UsersPage > toggleSubscriptionSelection` | 切换与 'toggleSubscriptionSelection' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setSubscriptionManageState' |
| 278–292 | function | `UsersPage > toggleSubscriptionSelection > setSubscriptionManageState.callback#28` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 5；循环 0；返回 4；await 0；调用 'prev.selectedIds.filter'、'prev.selectedIds.includes' |
| 290–290 | function | `UsersPage > toggleSubscriptionSelection > setSubscriptionManageState.callback#28 > prev.selectedIds.filter.callback#29` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 295–295 | function | `UsersPage > useMemo.callback#30` | 封装 'useMemo.callback#30' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 297–307 | function | `UsersPage > useEffect.callback#31` | 封装 'useEffect.callback#31' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'setSubscriptionManageState' |
| 301–306 | function | `UsersPage > useEffect.callback#31 > setSubscriptionManageState.callback#32` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0 |
| 366–369 | function | `UsersPage > onClick.callback#33` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'generatePassword'、'setCreateOpen'、'setCreateState' |
| 378–378 | function | `UsersPage > getRowKey.callback#34` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 384–384 | function | `UsersPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 390–390 | function | `UsersPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 395–395 | function | `UsersPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 401–413 | function | `UsersPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 408–408 | function | `UsersPage > cell > onClick.callback#39` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setRemarkEditState' |
| 418–480 | function | `UsersPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'code.slice' |
| 423–430 | function | `UsersPage > cell > onOpenChange.callback#41` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setCustomCodeEditUser'、'setCustomCodeInput' |
| 451–451 | function | `UsersPage > cell > onChange.callback#42` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.target.value.replace'、'setCustomCodeInput' |
| 460–460 | function | `UsersPage > cell > onClick.callback#43` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'customCodeMutation.mutate' |
| 470–470 | function | `UsersPage > cell > onClick.callback#44` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'customCodeMutation.mutate' |
| 485–488 | function | `UsersPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0 |
| 495–510 | function | `UsersPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 502–506 | function | `UsersPage > cell > onCheckedChange.callback#47` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'statusMutation.mutate' |
| 517–559 | function | `UsersPage > cell` | 执行与 'cell' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0 |
| 527–531 | function | `UsersPage > cell > onClick.callback#49` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'generatePassword'、'setResetState' |
| 539–544 | function | `UsersPage > cell > onClick.callback#50` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSubscriptionManageState' |
| 553–553 | function | `UsersPage > cell > onClick.callback#51` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setDeleteUsername' |
| 567–582 | function | `UsersPage > header` | 执行与 'header' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0 |
| 586–586 | function | `UsersPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 590–602 | function | `UsersPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 597–597 | function | `UsersPage > value > onClick.callback#55` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setRemarkEditState' |
| 606–624 | function | `UsersPage > value` | 执行与 'value' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0 |
| 614–618 | function | `UsersPage > value > onCheckedChange.callback#57` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'statusMutation.mutate' |
| 627–670 | function | `UsersPage > actions` | 执行与 'actions' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0 |
| 636–640 | function | `UsersPage > actions > onClick.callback#59` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'generatePassword'、'setResetState' |
| 649–654 | function | `UsersPage > actions > onClick.callback#60` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSubscriptionManageState' |
| 664–664 | function | `UsersPage > actions > onClick.callback#61` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setDeleteUsername' |
| 677–677 | function | `UsersPage > onOpenChange.callback#62` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setCreateOpen' |
| 689–698 | function | `UsersPage > onChange.callback#63` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setCreateState' |
| 690–698 | function | `UsersPage > onChange.callback#63 > setCreateState.callback#64` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0 |
| 709–710 | function | `UsersPage > onChange.callback#65` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setCreateState' |
| 710–710 | function | `UsersPage > onChange.callback#65 > setCreateState.callback#66` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 720–721 | function | `UsersPage > onChange.callback#67` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setCreateState' |
| 721–721 | function | `UsersPage > onChange.callback#67 > setCreateState.callback#68` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 731–732 | function | `UsersPage > onChange.callback#69` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setCreateState' |
| 732–732 | function | `UsersPage > onChange.callback#69 > setCreateState.callback#70` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 744–745 | function | `UsersPage > onChange.callback#71` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setCreateState' |
| 745–745 | function | `UsersPage > onChange.callback#71 > setCreateState.callback#72` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 755–781 | function | `UsersPage > subscriptionsQuery.data.map.callback#73` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'createState.subscriptionIds.includes' |
| 760–767 | function | `UsersPage > subscriptionsQuery.data.map.callback#73 > onCheckedChange.callback#74` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setCreateState' |
| 761–766 | function | `UsersPage > subscriptionsQuery.data.map.callback#73 > onCheckedChange.callback#74 > setCreateState.callback#75` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'prev.subscriptionIds.filter' |
| 764–764 | function | `UsersPage > subscriptionsQuery.data.map.callback#73 > onCheckedChange.callback#74 > setCreateState.callback#75 > prev.subscriptionIds.filter.callback#76` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 797–797 | function | `UsersPage > onClick.callback#77` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'createMutation.mutate' |
| 805–805 | function | `UsersPage > onOpenChange.callback#78` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setResetState' |
| 821–829 | function | `UsersPage > onChange.callback#79` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setResetState' |
| 822–828 | function | `UsersPage > onChange.callback#79 > setResetState.callback#80` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 844–844 | function | `UsersPage > onClick.callback#81` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'resetMutation.mutate' |
| 854–867 | function | `UsersPage > onOpenChange.callback#82` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'setSubscriptionManageState' |
| 858–865 | function | `UsersPage > onOpenChange.callback#82 > setSubscriptionManageState.callback#83` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0 |
| 884–925 | function | `UsersPage > subscriptionsQuery.data.map.callback#84` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'subscriptionManageState.selectedIds.includes' |
| 894–894 | function | `UsersPage > subscriptionsQuery.data.map.callback#84 > onClick.callback#85` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleSubscriptionSelection' |
| 895–903 | function | `UsersPage > subscriptionsQuery.data.map.callback#84 > onKeyDown.callback#86` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 1；await 0；调用 'event.preventDefault'、'toggleSubscriptionSelection' |
| 905–905 | function | `UsersPage > subscriptionsQuery.data.map.callback#84 > onClick.callback#87` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'event.stopPropagation' |
| 909–909 | function | `UsersPage > subscriptionsQuery.data.map.callback#84 > onCheckedChange.callback#88` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleSubscriptionSelection' |
| 941–948 | function | `UsersPage > onClick.callback#89` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'updateSubscriptionsMutation.mutate' |
| 956–956 | function | `UsersPage > onOpenChange.callback#90` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setDeleteUsername' |
| 984–984 | function | `UsersPage > onClick.callback#91` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'deleteMutation.mutate' |
| 992–992 | function | `UsersPage > onOpenChange.callback#92` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setRemarkEditState' |
| 1008–1011 | function | `UsersPage > onChange.callback#93` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setRemarkEditState' |
| 1009–1010 | function | `UsersPage > onChange.callback#93 > setRemarkEditState.callback#94` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 1025–1025 | function | `UsersPage > onClick.callback#95` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'remarkMutation.mutate' |

