# TypeScript 分区 `components-layout`

导航栏、顶栏、用户菜单和应用外壳。

## `components/layout/nav-icon.tsx`

依赖：`lucide-react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–15 | const | `ANIME_NAV_ICONS` | 保存 'ANIME_NAV_ICONS' 的模块级常量、配置、路由或预计算值。 |  |
| 17–21 | interface | `NavIconProps` | 定义 'NavIconProps' 的数据契约、联合类型或组件属性。 |  |
| 23–39 | function | `NavIcon` | 渲染并协调 'NavIcon' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/layout/topbar.tsx`

依赖：`react`、`@tanstack/react-query`、`@tanstack/react-router`、`lucide-react`、`@/stores/auth-store`、`@/lib/profile`、`@/components/ui/button`、`@/components/ui/dropdown-menu`、`@/components/layout/nav-icon`、`@/components/theme-switch`、`./user-menu`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 31–47 | const | `baseNavLinks` | 保存 'baseNavLinks' 的模块级常量、配置、路由或预计算值。 |  |
| 49–90 | const | `adminNavLinks` | 保存 'adminNavLinks' 的模块级常量、配置、路由或预计算值。 |  |
| 92–308 | function | `Topbar` | 渲染并协调 'Topbar' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'Boolean'、'adminNavLinks.map'、'allNavLinks.map'、'baseNavLinks.map'、'useAuthStore'、'useCallback'、'useEffect'、'useQuery'、'useRef'、'useState' |
| 113–154 | function | `Topbar > useCallback.callback#2` | 封装 'useCallback.callback#2' Hook 的响应式状态、副作用和复用逻辑。 | 分支 3；循环 0；返回 3；await 0；调用 'Math.ceil'、'Math.min'、'setHideLogoText'、'setIconOnlyCount' |
| 156–173 | function | `Topbar > useEffect.callback#3` | 封装 'useEffect.callback#3' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'calculateIconOnlyCount'、'resizeObserver.observe'、'window.addEventListener' |
| 159–161 | function | `Topbar > useEffect.callback#3 > <anonymous#4>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'calculateIconOnlyCount' |
| 169–172 | function | `Topbar > useEffect.callback#3 > <anonymous#5>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'resizeObserver.disconnect'、'window.removeEventListener' |
| 200–228 | function | `Topbar > allNavLinks.map.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0 |
| 233–246 | function | `Topbar > baseNavLinks.map.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 266–281 | function | `Topbar > adminNavLinks.map.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 271–271 | function | `Topbar > adminNavLinks.map.callback#8 > onClick.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMobileMenuOpen' |

## `components/layout/user-menu.tsx`

依赖：`react`、`@tanstack/react-query`、`@tanstack/react-router`、`lucide-react`、`sonner`、`@/stores/auth-store`、`@/lib/api`、`@/lib/cookies`、`@/lib/handle-server-error`、`@/lib/profile`、`@/hooks/use-dialog-state`、`@/hooks/use-version-check`、`@/components/ui/avatar`、`@/components/ui/button`、`@/components/ui/dropdown-menu`、`@/components/ui/switch`、`@/components/backup-dialog`、`@/components/mmwx-dialog`、`@/components/sign-out-dialog`、`@/components/update-dialog`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 38–315 | function | `UserMenu` | 渲染并协调 'UserMenu' React 组件的状态、数据请求和用户交互。 | 分支 4；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.map'、'Boolean'、'displayName.slice'、'profile.avatar_url.trim'、'profile.email.trim'、'profile.role.toUpperCase'、'useAuthStore'、'useDialogState'、'useMutation'、'useQuery'、'useQueryClient'、'useState'、'useVersionCheck' |
| 58–67 | function | `UserMenu > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 69–71 | function | `UserMenu > refetchInterval` | 执行与 'refetchInterval' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0 |
| 76–79 | function | `UserMenu > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 80–83 | function | `UserMenu > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 84–87 | function | `UserMenu > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 92–95 | function | `UserMenu > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 96–99 | function | `UserMenu > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'toast.success' |
| 100–103 | function | `UserMenu > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 106–112 | function | `UserMenu > handleDebugToggle` | 处理与 'handleDebugToggle' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'disableDebugMutation.mutate'、'enableDebugMutation.mutate' |
| 182–182 | function | `UserMenu > onSelect.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.preventDefault' |
| 194–194 | function | `UserMenu > onClick.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 201–201 | function | `UserMenu > onSelect.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.preventDefault' |
| 209–233 | function | `UserMenu > <ArrayLiteralExpression>.map.callback#14` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'getCookie' |
| 213–224 | function | `UserMenu > <ArrayLiteralExpression>.map.callback#14 > onClick.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'getCookie'、'setCookie'、'window.location.reload' |
| 249–249 | function | `UserMenu > onClick.callback#16` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBackupDialogOpen' |
| 257–257 | function | `UserMenu > onClick.callback#17` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setUpdateDialogOpen' |
| 285–285 | function | `UserMenu > onClick.callback#18` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setMmwxDialogOpen' |
| 292–292 | function | `UserMenu > onClick.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setOpen' |
| 302–302 | function | `UserMenu > onOpenChange.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setOpen' |

