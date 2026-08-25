# TypeScript 分区 `hooks`

可复用 React Hook、响应式状态和拖拽/媒体查询行为。

## `hooks/use-dialog-state.tsx`

依赖：`react`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 9–18 | function | `useDialogState` | 封装 'useDialogState' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'useState' |
| 14–15 | function | `useDialogState > setOpen` | 设置与 'setOpen' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 '_setOpen' |
| 15–15 | function | `useDialogState > setOpen > _setOpen.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |

## `hooks/use-external-sync-selection.ts`

依赖：`react`、`@tanstack/react-query`、`sonner`、`@/components/external-sync-node-dialog`、`@/lib/api`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–28 | function | `useExternalSyncSelection` | 封装 'useExternalSyncSelection' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'useMutation'、'useQueryClient'、'useState' |
| 11–16 | function | `useExternalSyncSelection > present` | 执行与 'present' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'nodes.map'、'setSelection' |
| 14–14 | function | `useExternalSyncSelection > present > nodes.map.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 18–18 | function | `useExternalSyncSelection > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post' |
| 19–24 | function | `useExternalSyncSelection > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries'、'setSelection'、'toast.success' |
| 25–25 | function | `useExternalSyncSelection > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 27–27 | function | `useExternalSyncSelection > cancel` | 执行与 'cancel' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelection' |
| 27–27 | function | `useExternalSyncSelection > confirm` | 执行与 'confirm' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'confirm.mutate' |
| 27–27 | function | `useExternalSyncSelection > setSelectedIds` | 设置与 'setSelectedIds' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelection' |
| 27–27 | function | `useExternalSyncSelection > setSelectedIds > setSelection.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |

## `hooks/use-media-query.ts`

依赖：`react`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–25 | function | `useMediaQuery` | 封装 'useMediaQuery' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'useEffect'、'useState' |
| 6–22 | function | `useMediaQuery > useEffect.callback#2` | 封装 'useEffect.callback#2' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'media.addEventListener'、'setMatches'、'window.matchMedia' |
| 13–15 | function | `useMediaQuery > useEffect.callback#2 > listener` | 执行与 'listener' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setMatches' |
| 21–21 | function | `useMediaQuery > useEffect.callback#2 > <anonymous#4>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'media.removeEventListener' |

## `hooks/use-mobile.tsx`

依赖：`react`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–3 | const | `MOBILE_BREAKPOINT` | 保存 'MOBILE_BREAKPOINT' 的模块级常量、配置、路由或预计算值。 |  |
| 5–19 | function | `useIsMobile` | 封装 'useIsMobile' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'React.useEffect'、'React.useState' |
| 8–16 | function | `useIsMobile > React.useEffect.callback#2` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'mql.addEventListener'、'setIsMobile'、'window.matchMedia' |
| 10–12 | function | `useIsMobile > React.useEffect.callback#2 > onChange` | 执行与 'onChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setIsMobile' |
| 15–15 | function | `useIsMobile > React.useEffect.callback#2 > <anonymous#4>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'mql.removeEventListener' |

## `hooks/use-node-drag-drop.ts`

依赖：`react`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–7 | interface | `ProxyGroup` | 定义 'ProxyGroup' 的数据契约、联合类型或组件属性。 |  |
| 9–14 | interface | `DraggedNode` | 定义 'DraggedNode' 的数据契约、联合类型或组件属性。 |  |
| 16–20 | interface | `UseNodeDragDropOptions` | 定义 'UseNodeDragDropOptions' 的数据契约、联合类型或组件属性。 |  |
| 22–176 | function | `useNodeDragDrop` | 封装 'useNodeDragDrop' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'useState' |
| 31–38 | function | `useNodeDragDrop > handleDragStart` | 处理与 'handleDragStart' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setDraggedNode' |
| 40–44 | function | `useNodeDragDrop > handleDragEnd` | 处理与 'handleDragEnd' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setActiveGroupTitle'、'setDragOverGroup'、'setDraggedNode' |
| 46–48 | function | `useNodeDragDrop > handleDragEnterGroup` | 处理与 'handleDragEnterGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setDragOverGroup' |
| 50–52 | function | `useNodeDragDrop > handleDragLeaveGroup` | 处理与 'handleDragLeaveGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setDragOverGroup' |
| 54–143 | function | `useNodeDragDrop > handleDrop` | 处理与 'handleDrop' 对应的前端业务、状态或数据转换逻辑。 | 分支 11；循环 0；返回 3；await 0；调用 'handleDragEnd'、'nodesToAdd.forEach'、'onProxyGroupsChange'、'updatedGroups.findIndex'、'updatedGroups.forEach'、'updatedGroups[<key>].proxies.filter'、'updatedGroups[<key>].proxies.includes'、'updatedGroups[<key>].proxies.push'、'updatedGroups[<key>].proxies.splice' |
| 64–75 | function | `useNodeDragDrop > handleDrop > updatedGroups.forEach.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodesToAdd.forEach' |
| 65–74 | function | `useNodeDragDrop > handleDrop > updatedGroups.forEach.callback#7 > nodesToAdd.forEach.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'group.proxies.includes'、'group.proxies.push'、'specialNodesToFilter.includes' |
| 78–83 | function | `useNodeDragDrop > handleDrop > updatedGroups.forEach.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'group.proxies.includes'、'group.proxies.push' |
| 96–96 | function | `useNodeDragDrop > handleDrop > updatedGroups.findIndex.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 99–99 | function | `useNodeDragDrop > handleDrop > updatedGroups[<key>].proxies.filter.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 106–106 | function | `useNodeDragDrop > handleDrop > updatedGroups.findIndex.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 111–120 | function | `useNodeDragDrop > handleDrop > nodesToAdd.forEach.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'specialNodesToFilter.includes'、'updatedGroups[<key>].proxies.includes'、'updatedGroups[<key>].proxies.push' |
| 145–162 | function | `useNodeDragDrop > handleDropToAvailable` | 处理与 'handleDropToAvailable' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'handleDragEnd'、'onProxyGroupsChange'、'updatedGroups.findIndex'、'updatedGroups[<key>].proxies.filter' |
| 153–153 | function | `useNodeDragDrop > handleDropToAvailable > updatedGroups.findIndex.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 156–156 | function | `useNodeDragDrop > handleDropToAvailable > updatedGroups[<key>].proxies.filter.callback#16` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |

## `hooks/use-proxy-groups.ts`

依赖：`@tanstack/react-query`、`@/lib/sublink/proxy-groups`、`@/lib/sublink/types`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 9–18 | function | `useProxyGroupCategories` | 封装 'useProxyGroupCategories' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'useQuery' |
| 24–34 | function | `useSyncProxyGroupCategories` | 封装 'useSyncProxyGroupCategories' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'useMutation'、'useQueryClient' |
| 28–28 | function | `useSyncProxyGroupCategories > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'syncProxyGroupCategories' |
| 29–32 | function | `useSyncProxyGroupCategories > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'queryClient.invalidateQueries' |

## `hooks/use-version-check.ts`

依赖：`@tanstack/react-query`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–3 | const | `CURRENT_VERSION` | 保存 'CURRENT_VERSION' 的模块级常量、配置、路由或预计算值。 |  |
| 4–4 | const | `GITHUB_API_URL` | 保存 'GITHUB_API_URL' 的模块级常量、配置、路由或预计算值。 |  |
| 6–10 | interface | `GitHubRelease` | 定义 'GitHubRelease' 的数据契约、联合类型或组件属性。 |  |
| 12–29 | function | `compareVersions` | 执行与 'compareVersions' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 1；返回 3；await 0；调用 'Math.max'、'cleanCurrent.split'、'cleanCurrent.split.map'、'cleanLatest.split'、'cleanLatest.split.map'、'current.replace'、'latest.replace' |
| 31–55 | function | `fetchLatestVersion` | 从后端获取与 'fetchLatestVersion' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 2；调用 'compareVersions'、'console.error'、'data.tag_name.replace'、'fetch'、'response.json' |
| 57–73 | function | `useVersionCheck` | 封装 'useVersionCheck' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'useQuery' |

