# TypeScript 分区 `components`

跨页面复用的业务组件和交互对话框。

## `components/anime-starfield.tsx`

依赖：`react`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–15 | const | `COLORS` | 保存 'COLORS' 的模块级常量、配置、路由或预计算值。 |  |
| 17–17 | function | `rand` | 执行与 'rand' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'Math.random' |
| 18–18 | function | `pick` | 执行与 'pick' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'Math.floor'、'Math.random' |
| 20–27 | type | `Star` | 定义 'Star' 的数据契约、联合类型或组件属性。 |  |
| 29–29 | let | `gid` | 保存 'gid' 的模块级常量、配置、路由或预计算值。 |  |
| 30–39 | function | `makeStar` | 执行与 'makeStar' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'Math.round'、'Number'、'pick'、'rand'、'rand.toFixed' |
| 41–41 | const | `SPAWN_MS` | 保存 'SPAWN_MS' 的模块级常量、配置、路由或预计算值。 |  |
| 42–42 | const | `MAX` | 保存 'MAX' 的模块级常量、配置、路由或预计算值。 |  |
| 44–80 | function | `AnimeStarfield` | 渲染并协调 'AnimeStarfield' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'stars.map'、'useEffect'、'useState' |
| 47–56 | function | `AnimeStarfield > useEffect.callback#5` | 封装 'useEffect.callback#5' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'setInterval' |
| 48–54 | function | `AnimeStarfield > useEffect.callback#5 > setInterval.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setStars' |
| 49–53 | function | `AnimeStarfield > useEffect.callback#5 > setInterval.callback#6 > setStars.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'makeStar'、'prev.slice' |
| 55–55 | function | `AnimeStarfield > useEffect.callback#5 > <anonymous#8>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'clearInterval' |
| 58–59 | function | `AnimeStarfield > remove` | 移除与 'remove' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setStars' |
| 59–59 | function | `AnimeStarfield > remove > setStars.callback#10 > prev.filter.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 59–59 | function | `AnimeStarfield > remove > setStars.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'prev.filter' |
| 63–77 | function | `AnimeStarfield > stars.map.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 75–75 | function | `AnimeStarfield > stars.map.callback#12 > onAnimationEnd.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'remove' |

## `components/backup-dialog.tsx`

依赖：`react`、`@tanstack/react-query`、`lucide-react`、`sonner`、`@/lib/api`、`@/components/ui/dialog`、`@/components/ui/button`、`@/components/ui/input`、`@/components/ui/label`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 17–20 | interface | `BackupDialogProps` | 定义 'BackupDialogProps' 的数据契约、联合类型或组件属性。 |  |
| 22–126 | function | `BackupDialog` | 渲染并协调 'BackupDialog' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'useMutation'、'useState' |
| 27–48 | function | `BackupDialog > handleDownload` | 处理与 'handleDownload' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 '<NewExpression>.toISOString'、'<NewExpression>.toISOString.replace'、'<NewExpression>.toISOString.replace.slice'、'api.get'、'document.body.appendChild'、'document.createElement'、'link.click'、'link.remove'、'link.setAttribute'、'setIsDownloading'、'toast.error'、'toast.success'、'window.URL.createObjectURL'、'window.URL.revokeObjectURL' |
| 52–58 | function | `BackupDialog > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'api.post'、'formData.append' |
| 59–67 | function | `BackupDialog > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onOpenChange'、'setBackupFile'、'setTimeout'、'toast.success' |
| 64–66 | function | `BackupDialog > onSuccess > setTimeout.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'window.location.reload' |
| 68–70 | function | `BackupDialog > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 105–105 | function | `BackupDialog > onChange.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setBackupFile' |
| 109–109 | function | `BackupDialog > onClick.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'restoreMutation.mutate' |

## `components/clash-config-viewer.tsx`

依赖：`react`、`@tanstack/react-virtual`、`lucide-react`、`sonner`、`@/components/ui/button`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–13 | type | `ClashConfigViewerProps` | 定义 'ClashConfigViewerProps' 的数据契约、联合类型或组件属性。 |  |
| 15–15 | const | `LINE_HEIGHT` | 保存 'LINE_HEIGHT' 的模块级常量、配置、路由或预计算值。 |  |
| 17–17 | const | `AUTO_VIEW_LINES` | 保存 'AUTO_VIEW_LINES' 的模块级常量、配置、路由或预计算值。 |  |
| 19–19 | const | `AUTO_VIEW_CHARS` | 保存 'AUTO_VIEW_CHARS' 的模块级常量、配置、路由或预计算值。 |  |
| 21–25 | function | `formatBytes` | 格式化与 'formatBytes' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 3；await 0；调用 '<BinaryExpression>.toFixed' |
| 27–190 | const | `ClashConfigViewer` | 保存 'ClashConfigViewer' 的模块级常量、配置、路由或预计算值。 |  |
| 27–190 | function | `ClashConfigViewer` | 渲染并协调 'ClashConfigViewer' React 组件的状态、数据请求和用户交互。 | 分支 6；循环 0；返回 1；await 0；调用 'cn'、'formatBytes'、'lineCount.toLocaleString'、'useCallback'、'useEffect'、'useMemo'、'useRef'、'useState'、'useVirtualizer'、'virtualizer.getTotalSize'、'virtualizer.getVirtualItems'、'virtualizer.getVirtualItems.map' |
| 33–33 | function | `ClashConfigViewer > useMemo.callback#3` | 封装 'useMemo.callback#3' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'value.split' |
| 45–48 | function | `ClashConfigViewer > useEffect.callback#4` | 封装 'useEffect.callback#4' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'setDraft' |
| 52–52 | function | `ClashConfigViewer > getScrollElement` | 读取或计算与 'getScrollElement' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 53–53 | function | `ClashConfigViewer > estimateSize` | 执行与 'estimateSize' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 57–64 | function | `ClashConfigViewer > useCallback.callback#7` | 封装 'useCallback.callback#7' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'requestAnimationFrame'、'setDraft'、'setMode' |
| 61–63 | function | `ClashConfigViewer > useCallback.callback#7 > requestAnimationFrame.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'textareaRef.current.focus' |
| 66–70 | function | `ClashConfigViewer > useCallback.callback#9` | 封装 'useCallback.callback#9' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onChange'、'setMode' |
| 72–76 | function | `ClashConfigViewer > useCallback.callback#10` | 封装 'useCallback.callback#10' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setDraft'、'setMode' |
| 78–88 | function | `ClashConfigViewer > useCallback.callback#11` | 封装 'useCallback.callback#11' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 1；调用 'navigator.clipboard.writeText'、'setCopied'、'toast.error'、'toast.success'、'window.setTimeout' |
| 84–84 | function | `ClashConfigViewer > useCallback.callback#11 > window.setTimeout.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setCopied' |
| 90–102 | function | `ClashConfigViewer > useCallback.callback#13` | 封装 'useCallback.callback#13' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'URL.createObjectURL'、'URL.revokeObjectURL'、'a.click'、'a.remove'、'document.body.appendChild'、'document.createElement'、'toast.success' |
| 150–164 | function | `ClashConfigViewer > virtualizer.getVirtualItems.map.callback#14` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 177–177 | function | `ClashConfigViewer > onChange.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setDraft' |

## `components/confirm-dialog.tsx`

依赖：`@/lib/utils`、`@/components/ui/alert-dialog`、`@/components/ui/button`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 13–26 | type | `ConfirmDialogProps` | 定义 'ConfirmDialogProps' 的数据契约、联合类型或组件属性。 |  |
| 28–67 | function | `ConfirmDialog` | 渲染并协调 'ConfirmDialog' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'cn' |

## `components/custom-rules-editor.tsx`

依赖：`react`、`@/components/ui/collapsible`、`@/lib/sublink/types`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–8 | interface | `CustomRulesEditorProps` | 定义 'CustomRulesEditorProps' 的数据契约、联合类型或组件属性。 |  |
| 10–202 | function | `CustomRulesEditor` | 渲染并协调 'CustomRulesEditor' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'useState' |

## `components/data-table.tsx`

依赖：`react`、`@/components/ui/card`、`@/components/ui/table`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–21 | type | `DataTableColumn` | 定义 'DataTableColumn' 的数据契约、联合类型或组件属性。 |  |
| 26–35 | type | `DataTableCardField` | 定义 'DataTableCardField' 的数据契约、联合类型或组件属性。 |  |
| 40–75 | type | `DataTableProps` | 定义 'DataTableProps' 的数据契约、联合类型或组件属性。 |  |
| 102–235 | function | `DataTable` | 渲染并协调 'DataTable' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'columns.map'、'data.map' |
| 113–130 | function | `DataTable > renderCellContent` | 渲染与 'renderCellContent' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 4；await 0；调用 'String'、'column.accessor'、'column.cell' |
| 144–181 | function | `DataTable > data.map.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'getRowKey'、'mobileCard.actions'、'mobileCard.fields.map'、'mobileCard.header'、'rowClassName' |
| 148–148 | function | `DataTable > data.map.callback#3 > onClick.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onRowClick' |
| 159–169 | function | `DataTable > data.map.callback#3 > mobileCard.fields.map.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'field.hidden'、'field.value' |
| 175–175 | function | `DataTable > data.map.callback#3 > onClick.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 191–199 | function | `DataTable > columns.map.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 213–228 | function | `DataTable > data.map.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'columns.map'、'getRowKey'、'rowClassName' |
| 217–217 | function | `DataTable > data.map.callback#8 > onClick.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onRowClick' |
| 219–226 | function | `DataTable > data.map.callback#8 > columns.map.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'renderCellContent' |

## `components/data-table.types.ts`

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|

## `components/debug-floating-viewer.tsx`

依赖：`react`、`@tanstack/react-query`、`lucide-react`、`sonner`、`@/components/ui/button`、`@/components/ui/sheet`、`@/components/ui/badge`、`@/lib/api`、`@/stores/auth-store`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 11–11 | const | `AUTO_CLOSE_SECONDS` | 保存 'AUTO_CLOSE_SECONDS' 的模块级常量、配置、路由或预计算值。 |  |
| 13–20 | function | `formatElapsed` | 格式化与 'formatElapsed' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 3；await 0；调用 'Math.floor' |
| 22–181 | function | `DebugFloatingViewer` | 渲染并协调 'DebugFloatingViewer' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 2；await 0；调用 'Boolean'、'useAuthStore'、'useCallback'、'useEffect'、'useMutation'、'useQuery'、'useQueryClient'、'useRef'、'useState' |
| 32–41 | function | `DebugFloatingViewer > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 43–43 | function | `DebugFloatingViewer > refetchInterval` | 执行与 'refetchInterval' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0 |
| 48–51 | function | `DebugFloatingViewer > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 57–60 | function | `DebugFloatingViewer > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 61–81 | function | `DebugFloatingViewer > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 1；调用 'api.get'、'data.download_url.split'、'document.body.appendChild'、'document.createElement'、'link.click'、'link.remove'、'link.setAttribute'、'queryClient.invalidateQueries'、'setSheetOpen'、'toast.error'、'window.URL.createObjectURL'、'window.URL.revokeObjectURL' |
| 84–87 | function | `DebugFloatingViewer > useCallback.callback#8` | 封装 'useCallback.callback#8' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'disableMutation.mutate'、'toast.success' |
| 90–107 | function | `DebugFloatingViewer > useEffect.callback#9` | 封装 'useEffect.callback#9' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 '<NewExpression>.getTime'、'setElapsed'、'setInterval'、'tick' |
| 96–103 | function | `DebugFloatingViewer > useEffect.callback#9 > tick` | 执行与 'tick' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'Date.now'、'Math.floor'、'disableMutation.mutate'、'formatElapsed'、'setElapsed'、'toast.info' |
| 106–106 | function | `DebugFloatingViewer > useEffect.callback#9 > <anonymous#11>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'clearInterval' |
| 110–114 | function | `DebugFloatingViewer > useEffect.callback#12` | 封装 'useEffect.callback#12' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0 |
| 116–120 | function | `DebugFloatingViewer > handleScroll` | 处理与 'handleScroll' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0 |
| 128–128 | function | `DebugFloatingViewer > onClick.callback#14` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSheetOpen' |
| 164–164 | function | `DebugFloatingViewer > onClick.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSheetOpen' |

## `components/edit-nodes-dialog.tsx`

依赖：`react`、`lucide-react`、`@/components/twemoji`、`@dnd-kit/core`、`@dnd-kit/sortable`、`@dnd-kit/utilities`、`@/components/ui/dialog`、`@/components/ui/card`、`@/components/ui/button`、`@/components/ui/input`、`@/components/ui/select`、`@/components/ui/popover`、`@/hooks/use-proxy-groups`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 32–39 | const | `PROXY_SERVICE_EMOJIS` | 保存 'PROXY_SERVICE_EMOJIS' 的模块级常量、配置、路由或预计算值。 |  |
| 41–50 | interface | `ProxyGroup` | 定义 'ProxyGroup' 的数据契约、联合类型或组件属性。 |  |
| 52–57 | interface | `Node` | 定义 'Node' 的数据契约、联合类型或组件属性。 |  |
| 60–60 | type | `DragItemType` | 定义 'DragItemType' 的数据契约、联合类型或组件属性。 |  |
| 62–69 | interface | `DragItemData` | 定义 'DragItemData' 的数据契约、联合类型或组件属性。 |  |
| 71–74 | interface | `ActiveDragItem` | 定义 'ActiveDragItem' 的数据契约、联合类型或组件属性。 |  |
| 77–77 | const | `SPECIAL_NODES` | 保存 'SPECIAL_NODES' 的模块级常量、配置、路由或预计算值。 |  |
| 80–80 | const | `DragStateContext` | 保存 'DragStateContext' 的模块级常量、配置、路由或预计算值。 |  |
| 83–88 | interface | `ProxyTypeSelectorProps` | 定义 'ProxyTypeSelectorProps' 的数据契约、联合类型或组件属性。 |  |
| 90–188 | const | `ProxyTypeSelector` | 保存 'ProxyTypeSelector' 的模块级常量、配置、路由或预计算值。 |  |
| 90–188 | function | `ProxyTypeSelector` | 渲染并协调 'ProxyTypeSelector' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'allGroups.filter'、'allGroups.filter.map'、'types.map' |
| 98–123 | function | `ProxyTypeSelector > handleTypeSelect` | 处理与 'handleTypeSelect' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 0；await 0；调用 'onChange'、'onClose'、'types.find' |
| 99–99 | function | `ProxyTypeSelector > handleTypeSelect > types.find.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 127–137 | function | `ProxyTypeSelector > types.map.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 133–133 | function | `ProxyTypeSelector > types.map.callback#4 > onClick.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleTypeSelect' |
| 144–144 | function | `ProxyTypeSelector > onValueChange.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onChange'、'onClose' |
| 162–171 | function | `ProxyTypeSelector > onValueChange.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'onChange'、'onClose' |
| 178–178 | function | `ProxyTypeSelector > allGroups.filter.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 178–182 | function | `ProxyTypeSelector > allGroups.filter.map.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 191–211 | const | `DroppableAllGroupsZone` | 保存 'DroppableAllGroupsZone' 的模块级常量、配置、路由或预计算值。 |  |
| 191–211 | function | `DroppableAllGroupsZone` | 渲染并协调 'DroppableAllGroupsZone' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'useDroppable' |
| 214–234 | const | `DroppableRemoveFromAllZone` | 保存 'DroppableRemoveFromAllZone' 的模块级常量、配置、路由或预计算值。 |  |
| 214–234 | function | `DroppableRemoveFromAllZone` | 渲染并协调 'DroppableRemoveFromAllZone' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'useDroppable' |
| 237–239 | interface | `DroppableAvailableZoneProps` | 定义 'DroppableAvailableZoneProps' 的数据契约、联合类型或组件属性。 |  |
| 241–257 | const | `DroppableAvailableZone` | 保存 'DroppableAvailableZone' 的模块级常量、配置、路由或预计算值。 |  |
| 241–257 | function | `DroppableAvailableZone` | 渲染并协调 'DroppableAvailableZone' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'useDroppable' |
| 260–268 | interface | `DraggableGroupTitleProps` | 定义 'DraggableGroupTitleProps' 的数据契约、联合类型或组件属性。 |  |
| 270–325 | const | `DraggableGroupTitle` | 保存 'DraggableGroupTitle' 的模块级常量、配置、路由或预计算值。 |  |
| 270–325 | function | `DraggableGroupTitle` | 渲染并协调 'DraggableGroupTitle' React 组件的状态、数据请求和用户交互。 | 分支 3；循环 0；返回 1；await 0；调用 'useDraggable' |
| 301–301 | function | `DraggableGroupTitle > onChange.callback#14` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onEditingValueChange' |
| 302–305 | function | `DraggableGroupTitle > onKeyDown.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'onCancelEdit'、'onSubmitEdit' |
| 317–317 | function | `DraggableGroupTitle > onClick.callback#16` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onStartEdit' |
| 328–342 | interface | `SortableCardProps` | 定义 'SortableCardProps' 的数据契约、联合类型或组件属性。 |  |
| 344–526 | const | `SortableCard` | 保存 'SortableCard' 的模块级常量、配置、路由或预计算值。 |  |
| 344–526 | function | `SortableCard` | 渲染并协调 'SortableCard' React 组件的状态、数据请求和用户交互。 | 分支 13；循环 0；返回 1；await 0；调用 '<BinaryExpression>.filter'、'<BinaryExpression>.filter.map'、'<BinaryExpression>.map'、'CSS.Transform.toString'、'useContext'、'useDroppable'、'useSortable'、'useState' |
| 397–400 | function | `SortableCard > ref.callback#18` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setDropRef'、'setNodeRef' |
| 461–461 | function | `SortableCard > onChange.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onGroupTypeChange' |
| 462–462 | function | `SortableCard > onClose.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTypePopoverOpen' |
| 470–473 | function | `SortableCard > onClick.callback#21` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'onRemoveGroup' |
| 485–485 | function | `SortableCard > <BinaryExpression>.filter.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 485–485 | function | `SortableCard > <BinaryExpression>.filter.map.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 486–486 | function | `SortableCard > <BinaryExpression>.map.callback#24` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 491–502 | function | `SortableCard > <BinaryExpression>.map.callback#25` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'mmwProviderNames.has' |
| 505–513 | function | `SortableCard > <BinaryExpression>.map.callback#26` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 511–511 | function | `SortableCard > <BinaryExpression>.map.callback#26 > onRemove.callback#27` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onRemoveUseItem' |
| 516–516 | function | `SortableCard > <BinaryExpression>.filter.callback#28` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 529–532 | interface | `DraggableAvailableNodeProps` | 定义 'DraggableAvailableNodeProps' 的数据契约、联合类型或组件属性。 |  |
| 534–566 | const | `DraggableAvailableNode` | 保存 'DraggableAvailableNode' 的模块级常量、配置、路由或预计算值。 |  |
| 534–566 | function | `DraggableAvailableNode` | 渲染并协调 'DraggableAvailableNode' React 组件的状态、数据请求和用户交互。 | 分支 3；循环 0；返回 1；await 0；调用 'useContext'、'useDraggable' |
| 569–571 | interface | `DraggableProxyProviderProps` | 定义 'DraggableProxyProviderProps' 的数据契约、联合类型或组件属性。 |  |
| 573–600 | const | `DraggableProxyProvider` | 保存 'DraggableProxyProvider' 的模块级常量、配置、路由或预计算值。 |  |
| 573–600 | function | `DraggableProxyProvider` | 渲染并协调 'DraggableProxyProvider' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'useDraggable' |
| 603–606 | interface | `DraggableAvailableHeaderProps` | 定义 'DraggableAvailableHeaderProps' 的数据契约、联合类型或组件属性。 |  |
| 608–640 | const | `DraggableAvailableHeader` | 保存 'DraggableAvailableHeader' 的模块级常量、配置、路由或预计算值。 |  |
| 608–640 | function | `DraggableAvailableHeader` | 渲染并协调 'DraggableAvailableHeader' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'useDraggable' |
| 643–649 | interface | `SortableProxyProps` | 定义 'SortableProxyProps' 的数据契约、联合类型或组件属性。 |  |
| 651–766 | const | `SortableProxy` | 保存 'SortableProxy' 的模块级常量、配置、路由或预计算值。 |  |
| 651–766 | function | `SortableProxy` | 渲染并协调 'SortableProxy' React 组件的状态、数据请求和用户交互。 | 分支 10；循环 0；返回 2；await 0；调用 'CSS.Transform.toString'、'useContext'、'useSortable' |
| 718–718 | function | `SortableProxy > onPointerDown.callback#33` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 719–719 | function | `SortableProxy > onTouchStart.callback#34` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 720–723 | function | `SortableProxy > onClick.callback#35` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'onRemove' |
| 754–754 | function | `SortableProxy > onPointerDown.callback#36` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 755–755 | function | `SortableProxy > onTouchStart.callback#37` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 756–759 | function | `SortableProxy > onClick.callback#38` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'onRemove' |
| 769–774 | interface | `SortableUseItemProps` | 定义 'SortableUseItemProps' 的数据契约、联合类型或组件属性。 |  |
| 776–852 | const | `SortableUseItem` | 保存 'SortableUseItem' 的模块级常量、配置、路由或预计算值。 |  |
| 776–852 | function | `SortableUseItem` | 渲染并协调 'SortableUseItem' React 组件的状态、数据请求和用户交互。 | 分支 5；循环 0；返回 1；await 0；调用 'CSS.Transform.toString'、'useContext'、'useSortable' |
| 840–840 | function | `SortableUseItem > onPointerDown.callback#40` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 841–841 | function | `SortableUseItem > onTouchStart.callback#41` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 842–845 | function | `SortableUseItem > onClick.callback#42` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'onRemove' |
| 854–889 | interface | `EditNodesDialogProps` | 定义 'EditNodesDialogProps' 的数据契约、联合类型或组件属性。 |  |
| 891–2024 | function | `EditNodesDialog` | 渲染并协调 'EditNodesDialog' React 组件的状态、数据请求和用户交互。 | 分支 5；循环 0；返回 1；await 0；调用 'Array.from'、'Array.from.map'、'Math.floor'、'Math.max'、'Math.min'、'React.useCallback'、'React.useRef'、'SPECIAL_NODES.map'、'filteredAvailableNodes.map'、'proxyGroups.map'、'proxyProviderConfigs.map'、'uniqueTags.map'、'useMemo'、'useProxyGroupCategories'、'useSensor'、'useSensors'、'useState'、'withScrollPreservation' |
| 917–926 | function | `EditNodesDialog > useMemo.callback#44` | 封装 'useMemo.callback#44' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'proxyGroupCategories.map' |
| 919–922 | function | `EditNodesDialog > useMemo.callback#44 > proxyGroupCategories.map.callback#45` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 929–932 | function | `EditNodesDialog > useState.callback#46` | 封装 'useState.callback#46' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'Number'、'localStorage.getItem' |
| 937–940 | function | `EditNodesDialog > handleColumnsChange` | 处理与 'handleColumnsChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'String'、'localStorage.setItem'、'setTotalColumns' |
| 948–952 | function | `EditNodesDialog > useMemo.callback#48` | 封装 'useMemo.callback#48' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'newGroupName.trim' |
| 955–958 | function | `EditNodesDialog > useMemo.callback#49` | 封装 'useMemo.callback#49' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'proxyGroups.some' |
| 957–957 | function | `EditNodesDialog > useMemo.callback#49 > proxyGroups.some.callback#50` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 976–985 | function | `EditNodesDialog > useMemo.callback#51` | 封装 'useMemo.callback#51' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'Array.from'、'Array.from.sort'、'allNodes.forEach' |
| 978–983 | function | `EditNodesDialog > useMemo.callback#51 > allNodes.forEach.callback#52` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 1；返回 0；await 0；调用 't.trim'、'tags.add' |
| 988–994 | function | `EditNodesDialog > useMemo.callback#53` | 封装 'useMemo.callback#53' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'allNodes.forEach' |
| 990–992 | function | `EditNodesDialog > useMemo.callback#53 > allNodes.forEach.callback#54` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 'map.set' |
| 997–1003 | function | `EditNodesDialog > useMemo.callback#55` | 封装 'useMemo.callback#55' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'proxyProviderConfigs.filter'、'proxyProviderConfigs.filter.map' |
| 1000–1000 | function | `EditNodesDialog > useMemo.callback#55 > proxyProviderConfigs.filter.callback#56` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1001–1001 | function | `EditNodesDialog > useMemo.callback#55 > proxyProviderConfigs.filter.map.callback#57` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1006–1026 | function | `EditNodesDialog > useMemo.callback#58` | 封装 'useMemo.callback#58' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'filtered.filter'、'nodeNameFilter.toLowerCase'、'nodeNameFilter.toLowerCase.trim'、'nodeNameFilter.trim' |
| 1012–1013 | function | `EditNodesDialog > useMemo.callback#58 > filtered.filter.callback#59` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeName.toLowerCase'、'nodeName.toLowerCase.includes' |
| 1019–1022 | function | `EditNodesDialog > useMemo.callback#58 > filtered.filter.callback#60` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'nodeTagMap.get'、'tags.includes' |
| 1044–1052 | function | `EditNodesDialog > React.useCallback.callback#61` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'closestCenter'、'pointerWithin' |
| 1055–1067 | function | `EditNodesDialog > handleDragStart` | 处理与 'handleDragStart' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'String'、'setActiveDragItem' |
| 1070–1475 | function | `EditNodesDialog > handleDragEnd` | 处理与 'handleDragEnd' 对应的前端业务、状态或数据转换逻辑。 | 分支 32；循环 0；返回 13；await 0；调用 'String'、'arrayMove'、'getTargetGroupName'、'group.proxies.indexOf'、'group.use.indexOf'、'nodeId.replace'、'onProxyGroupsChange'、'onRemoveNodeFromGroup'、'proxyGroups.find'、'proxyGroups.findIndex'、'proxyGroups.map'、'restoreAvailableNodesScroll'、'setActiveDragItem' |
| 1083–1089 | function | `EditNodesDialog > handleDragEnd > restoreAvailableNodesScroll` | 执行与 'restoreAvailableNodesScroll' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'requestAnimationFrame' |
| 1084–1088 | function | `EditNodesDialog > handleDragEnd > restoreAvailableNodesScroll > requestAnimationFrame.callback#65` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 1098–1112 | function | `EditNodesDialog > handleDragEnd > getTargetGroupName` | 读取或计算与 'getTargetGroupName' 对应的前端业务、状态或数据转换逻辑。 | 分支 7；循环 0；返回 7；await 0；调用 'overId.includes'、'overId.replace'、'overId.startsWith'、'proxyGroups.find' |
| 1108–1108 | function | `EditNodesDialog > handleDragEnd > getTargetGroupName > proxyGroups.find.callback#67` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'overId.startsWith' |
| 1115–1127 | function | `EditNodesDialog > handleDragEnd > getInsertIndex` | 读取或计算与 'getInsertIndex' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 3；await 0 |
| 1130–1146 | function | `EditNodesDialog > handleDragEnd > getUseInsertIndex` | 读取或计算与 'getUseInsertIndex' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 3；await 0；调用 'Math.max'、'Math.min' |
| 1158–1163 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#70` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'group.proxies.filter'、'group.proxies.includes' |
| 1160–1160 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#70 > group.proxies.filter.callback#71` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1167–1172 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#72` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'group.proxies.includes' |
| 1179–1189 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#73` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'getInsertIndex'、'group.proxies.includes'、'newProxies.splice' |
| 1205–1211 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#74` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'group.proxies.filter' |
| 1206–1206 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#74 > group.proxies.filter.callback#75` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodeNamesToRemove.has' |
| 1215–1223 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#76` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'nodeNames.filter' |
| 1218–1218 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#76 > nodeNames.filter.callback#77` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'existingNodes.has' |
| 1227–1241 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#78` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 2；await 0；调用 'getInsertIndex'、'newProxies.splice'、'nodeNames.filter' |
| 1231–1231 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#78 > nodeNames.filter.callback#79` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'existingNodes.has' |
| 1265–1270 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#80` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'group.proxies.filter'、'group.proxies.includes' |
| 1267–1267 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#80 > group.proxies.filter.callback#81` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1277–1277 | function | `EditNodesDialog > handleDragEnd > proxyGroups.find.callback#82` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1286–1291 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#83` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'arrayMove' |
| 1297–1302 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#84` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'group.proxies.includes' |
| 1311–1324 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#85` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 3；await 0；调用 'getInsertIndex'、'group.proxies.filter'、'group.proxies.includes'、'newProxies.splice' |
| 1314–1314 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#85 > group.proxies.filter.callback#86` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1339–1344 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#87` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'group.proxies.includes' |
| 1348–1357 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#88` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'getInsertIndex'、'group.proxies.includes'、'newProxies.splice' |
| 1367–1367 | function | `EditNodesDialog > handleDragEnd > proxyGroups.findIndex.callback#89` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1368–1368 | function | `EditNodesDialog > handleDragEnd > proxyGroups.findIndex.callback#90` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1385–1391 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#91` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'currentUse.includes' |
| 1395–1407 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#92` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 2；await 0；调用 'currentUse.includes'、'getUseInsertIndex'、'newUse.splice' |
| 1423–1423 | function | `EditNodesDialog > handleDragEnd > proxyGroups.find.callback#93` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1440–1445 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#94` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0；调用 'arrayMove' |
| 1451–1466 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#95` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 3；await 0；调用 'currentUse.includes'、'getUseInsertIndex'、'group.use.filter'、'newUse.splice' |
| 1454–1454 | function | `EditNodesDialog > handleDragEnd > proxyGroups.map.callback#95 > group.use.filter.callback#96` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1478–1488 | function | `EditNodesDialog > withScrollPreservation` | 执行与 'withScrollPreservation' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0 |
| 1479–1487 | function | `EditNodesDialog > withScrollPreservation > <anonymous#98>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'fn'、'requestAnimationFrame' |
| 1482–1486 | function | `EditNodesDialog > withScrollPreservation > <anonymous#98> > requestAnimationFrame.callback#99` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 1492–1496 | function | `EditNodesDialog > withScrollPreservation.callback#100` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'onRemoveNodeFromGroup' |
| 1502–1506 | function | `EditNodesDialog > withScrollPreservation.callback#101` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'onRemoveGroup' |
| 1511–1529 | function | `EditNodesDialog > handleRenameGroupInternal` | 处理与 'handleRenameGroupInternal' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 2；await 0；调用 'newName.trim'、'onRenameGroup'、'proxyGroups.find'、'setEditingGroupName'、'setEditingGroupValue' |
| 1519–1519 | function | `EditNodesDialog > handleRenameGroupInternal > proxyGroups.find.callback#103` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1531–1534 | function | `EditNodesDialog > startEditingGroup` | 执行与 'startEditingGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingGroupName'、'setEditingGroupValue' |
| 1536–1539 | function | `EditNodesDialog > cancelEditingGroup` | 执行与 'cancelEditingGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingGroupName'、'setEditingGroupValue' |
| 1541–1545 | function | `EditNodesDialog > submitEditingGroup` | 执行与 'submitEditingGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'handleRenameGroupInternal' |
| 1548–1561 | function | `EditNodesDialog > handleAddGroup` | 处理与 'handleAddGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'onProxyGroupsChange'、'setAddGroupDialogOpen'、'setNewGroupName'、'setSelectedEmoji' |
| 1563–1575 | function | `EditNodesDialog > handleQuickSelect` | 处理与 'handleQuickSelect' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'match[<key>].trim'、'name.match'、'name.slice'、'name.slice.trim'、'setNewGroupName'、'setSelectedEmoji' |
| 1578–1583 | function | `EditNodesDialog > React.useCallback.callback#109` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onProxyGroupsChange'、'proxyGroups.map' |
| 1579–1580 | function | `EditNodesDialog > React.useCallback.callback#109 > proxyGroups.map.callback#110` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 1586–1595 | function | `EditNodesDialog > React.useCallback.callback#111` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onProxyGroupsChange'、'proxyGroups.map' |
| 1587–1593 | function | `EditNodesDialog > React.useCallback.callback#111 > proxyGroups.map.callback#112` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 2；await 0；调用 '<BinaryExpression>.filter' |
| 1589–1589 | function | `EditNodesDialog > React.useCallback.callback#111 > proxyGroups.map.callback#112 > <BinaryExpression>.filter.callback#113` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1598–1600 | function | `EditNodesDialog > useMemo.callback#114` | 封装 'useMemo.callback#114' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 1637–1637 | function | `EditNodesDialog > Array.from.callback#115` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1637–1647 | function | `EditNodesDialog > Array.from.map.callback#116` | 渲染并协调 'Array.from.map.callback#116' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 0；await 0 |
| 1643–1643 | function | `EditNodesDialog > Array.from.map.callback#116 > onClick.callback#117` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleColumnsChange' |
| 1654–1654 | function | `EditNodesDialog > proxyGroups.map.callback#118` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1658–1675 | function | `EditNodesDialog > proxyGroups.map.callback#119` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1687–1687 | function | `EditNodesDialog > onClick.callback#120` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setAddGroupDialogOpen' |
| 1705–1705 | function | `EditNodesDialog > onClick.callback#121` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onShowAllNodesChange' |
| 1738–1738 | function | `EditNodesDialog > onChange.callback#122` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNodeNameFilter' |
| 1750–1754 | function | `EditNodesDialog > uniqueTags.map.callback#123` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1777–1783 | function | `EditNodesDialog > filteredAvailableNodes.map.callback#124` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1794–1799 | function | `EditNodesDialog > proxyProviderConfigs.map.callback#125` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1811–1817 | function | `EditNodesDialog > SPECIAL_NODES.map.callback#126` | 渲染并协调 'SPECIAL_NODES.map.callback#126' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 1866–1908 | function | `EditNodesDialog > <anonymous#127>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'group.proxies.slice'、'group.proxies.slice.map'、'proxyGroups.find' |
| 1867–1867 | function | `EditNodesDialog > <anonymous#127> > proxyGroups.find.callback#128` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1886–1894 | function | `EditNodesDialog > <anonymous#127> > group.proxies.slice.map.callback#129` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 1918–1924 | function | `EditNodesDialog > onOpenChange.callback#130` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setAddGroupDialogOpen'、'setNewGroupName'、'setSelectedEmoji' |
| 1949–1960 | function | `EditNodesDialog > allServiceEmojis.map.callback#131` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 1956–1956 | function | `EditNodesDialog > allServiceEmojis.map.callback#131 > onClick.callback#132` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelectedEmoji' |
| 1967–1967 | function | `EditNodesDialog > onClick.callback#133` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelectedEmoji' |
| 1977–1977 | function | `EditNodesDialog > onChange.callback#134` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNewGroupName' |
| 1978–1980 | function | `EditNodesDialog > onKeyDown.callback#135` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'handleAddGroup' |
| 1992–2007 | function | `EditNodesDialog > proxyGroupCategories.map.callback#136` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'proxyGroups.some' |
| 1994–1994 | function | `EditNodesDialog > proxyGroupCategories.map.callback#136 > proxyGroups.some.callback#137` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 2001–2001 | function | `EditNodesDialog > proxyGroupCategories.map.callback#136 > onClick.callback#138` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleQuickSelect' |
| 2013–2013 | function | `EditNodesDialog > onClick.callback#139` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setAddGroupDialogOpen' |

## `components/external-sync-node-dialog.tsx`

依赖：`react`、`@/components/ui/badge`、`@/components/ui/button`、`@/components/ui/checkbox`、`@/components/ui/dialog`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–15 | interface | `ExternalSyncCandidate` | 定义 'ExternalSyncCandidate' 的数据契约、联合类型或组件属性。 |  |
| 17–21 | interface | `ExternalSyncSelection` | 定义 'ExternalSyncSelection' 的数据契约、联合类型或组件属性。 |  |
| 23–57 | function | `ExternalSyncNodeDialog` | 渲染并协调 'ExternalSyncNodeDialog' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'Boolean'、'groups.map'、'useMemo' |
| 30–34 | function | `ExternalSyncNodeDialog > useMemo.callback#2` | 封装 'useMemo.callback#2' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 1；返回 1；await 0；调用 'grouped.entries'、'grouped.get'、'grouped.set' |
| 36–41 | function | `ExternalSyncNodeDialog > toggle` | 切换与 'toggle' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'next.add'、'next.delete'、'onSelectionChange' |
| 42–42 | function | `ExternalSyncNodeDialog > onOpenChange.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onCancel' |
| 46–46 | function | `ExternalSyncNodeDialog > onCheckedChange.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'onSelectionChange'、'selection.nodes.map' |
| 46–46 | function | `ExternalSyncNodeDialog > onCheckedChange.callback#5 > selection.nodes.map.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 50–52 | function | `ExternalSyncNodeDialog > groups.map.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'nodes.map' |
| 51–51 | function | `ExternalSyncNodeDialog > groups.map.callback#7 > nodes.map.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'selection.selectedIds.has' |
| 51–51 | function | `ExternalSyncNodeDialog > groups.map.callback#7 > nodes.map.callback#8 > onCheckedChange.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggle' |

## `components/flag-emoji-picker.tsx`

依赖：`react`、`lucide-react`、`@/components/ui/popover`、`@/components/ui/button`、`@/components/twemoji`、`@/lib/country-flag`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–15 | interface | `FlagEmojiPickerProps` | 定义 'FlagEmojiPickerProps' 的数据契约、联合类型或组件属性。 |  |
| 17–54 | function | `FlagEmojiPicker` | 渲染并协调 'FlagEmojiPicker' React 组件的状态、数据请求和用户交互。 | 分支 3；循环 0；返回 1；await 0；调用 'FLAG_OPTIONS.map'、'useState' |
| 28–28 | function | `FlagEmojiPicker > <anonymous#2>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 33–33 | function | `FlagEmojiPicker > <anonymous#3>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 35–35 | function | `FlagEmojiPicker > onClick.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onAutoDetect'、'setOpen' |
| 40–49 | function | `FlagEmojiPicker > FLAG_OPTIONS.map.callback#5` | 渲染并协调 'FLAG_OPTIONS.map.callback#5' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0；调用 'countryCodeToFlag' |
| 44–44 | function | `FlagEmojiPicker > FLAG_OPTIONS.map.callback#5 > onClick.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'countryCodeToFlag'、'onSelect'、'setOpen' |

## `components/mmwx-dialog.tsx`

依赖：`lucide-react`、`@/components/ui/dialog`、`@/components/ui/scroll-area`、`@/components/ui/button`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 11–24 | const | `TABLE_ROWS` | 保存 'TABLE_ROWS' 的模块级常量、配置、路由或预计算值。 |  |
| 26–33 | const | `EXCLUSIVE_FEATURES` | 保存 'EXCLUSIVE_FEATURES' 的模块级常量、配置、路由或预计算值。 |  |
| 35–46 | const | `SHARED_FEATURES` | 保存 'SHARED_FEATURES' 的模块级常量、配置、路由或预计算值。 |  |
| 48–147 | function | `MmwxDialog` | 渲染并协调 'MmwxDialog' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'EXCLUSIVE_FEATURES.map'、'SHARED_FEATURES.map'、'TABLE_ROWS.map' |
| 89–100 | function | `MmwxDialog > TABLE_ROWS.map.callback#2` | 渲染并协调 'TABLE_ROWS.map.callback#2' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 0；await 0 |
| 112–117 | function | `MmwxDialog > EXCLUSIVE_FEATURES.map.callback#3` | 渲染并协调 'EXCLUSIVE_FEATURES.map.callback#3' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 127–132 | function | `MmwxDialog > SHARED_FEATURES.map.callback#4` | 渲染并协调 'SHARED_FEATURES.map.callback#4' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |

## `components/mobile-edit-nodes-dialog.tsx`

依赖：`react`、`lucide-react`、`@dnd-kit/core`、`@dnd-kit/sortable`、`@dnd-kit/utilities`、`@/components/ui/sheet`、`@/components/ui/popover`、`@/components/ui/select`、`@/components/ui/card`、`@/components/ui/button`、`@/components/ui/input`、`@/components/ui/checkbox`、`@/components/ui/badge`、`@/components/twemoji`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 41–50 | interface | `ProxyGroup` | 定义 'ProxyGroup' 的数据契约、联合类型或组件属性。 |  |
| 52–57 | interface | `Node` | 定义 'Node' 的数据契约、联合类型或组件属性。 |  |
| 60–60 | const | `SPECIAL_NODES` | 保存 'SPECIAL_NODES' 的模块级常量、配置、路由或预计算值。 |  |
| 62–75 | interface | `MobileEditNodesDialogProps` | 定义 'MobileEditNodesDialogProps' 的数据契约、联合类型或组件属性。 |  |
| 78–82 | interface | `SortableNodeItemProps` | 定义 'SortableNodeItemProps' 的数据契约、联合类型或组件属性。 |  |
| 84–129 | function | `SortableNodeItem` | 渲染并协调 'SortableNodeItem' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'CSS.Transform.toString'、'useSortable' |
| 95–95 | function | `SortableNodeItem > animateLayoutChanges` | 执行与 'animateLayoutChanges' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 131–856 | function | `MobileEditNodesDialog` | 渲染并协调 'MobileEditNodesDialog' React 组件的状态、数据请求和用户交互。 | 分支 3；循环 0；返回 1；await 0；调用 'SPECIAL_NODES.map'、'allTags.map'、'filteredAvailableNodes.map'、'proxyGroups.find'、'proxyGroups.map'、'proxyProviderConfigs.map'、'useMemo'、'useSensor'、'useSensors'、'useState' |
| 170–199 | function | `MobileEditNodesDialog > handleDragEnd` | 处理与 'handleDragEnd' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 5；await 0；调用 'String'、'activeIdStr.slice'、'activeIdStr.startsWith'、'arrayMove'、'isNaN'、'onProxyGroupsChange'、'overIdStr.slice'、'overIdStr.startsWith'、'parseInt'、'proxyGroups.find'、'proxyGroups.map' |
| 175–175 | function | `MobileEditNodesDialog > handleDragEnd > proxyGroups.find.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 192–197 | function | `MobileEditNodesDialog > handleDragEnd > proxyGroups.map.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0 |
| 202–209 | function | `MobileEditNodesDialog > useMemo.callback#7` | 封装 'useMemo.callback#7' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'Array.from'、'Array.from.sort'、'allNodes.forEach' |
| 204–207 | function | `MobileEditNodesDialog > useMemo.callback#7 > allNodes.forEach.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 1；返回 0；await 0；调用 'tags.add' |
| 212–230 | function | `MobileEditNodesDialog > useMemo.callback#9` | 封装 'useMemo.callback#9' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'allNodes.filter'、'allNodes.filter.map'、'proxyGroups.find' |
| 213–213 | function | `MobileEditNodesDialog > useMemo.callback#9 > proxyGroups.find.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 217–228 | function | `MobileEditNodesDialog > useMemo.callback#9 > allNodes.filter.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 6；循环 0；返回 4；await 0；调用 'currentProxies.has'、'node.node_name.toLowerCase'、'node.node_name.toLowerCase.includes'、'nodeTags.includes'、'searchQuery.toLowerCase' |
| 229–229 | function | `MobileEditNodesDialog > useMemo.callback#9 > allNodes.filter.map.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 233–241 | function | `MobileEditNodesDialog > toggleGroup` | 切换与 'toggleGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'newExpanded.add'、'newExpanded.delete'、'newExpanded.has'、'setExpandedGroups' |
| 244–247 | function | `MobileEditNodesDialog > startEditGroupName` | 执行与 'startEditGroupName' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingGroupName'、'setEditingGroupNewName' |
| 250–256 | function | `MobileEditNodesDialog > confirmRename` | 执行与 'confirmRename' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'editingGroupNewName.trim'、'onRenameGroup'、'setEditingGroupName'、'setEditingGroupNewName' |
| 259–262 | function | `MobileEditNodesDialog > cancelRename` | 执行与 'cancelRename' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingGroupName'、'setEditingGroupNewName' |
| 265–270 | function | `MobileEditNodesDialog > openEditSheet` | 执行与 'openEditSheet' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setCurrentEditingGroup'、'setEditSheetOpen'、'setSearchQuery'、'setSelectedTag' |
| 273–278 | function | `MobileEditNodesDialog > closeEditSheet` | 执行与 'closeEditSheet' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setCurrentEditingGroup'、'setEditSheetOpen'、'setSearchQuery'、'setSelectedTag' |
| 281–285 | function | `MobileEditNodesDialog > isNodeInCurrentGroup` | 判断与 'isNodeInCurrentGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'group.proxies.includes'、'proxyGroups.find' |
| 283–283 | function | `MobileEditNodesDialog > isNodeInCurrentGroup > proxyGroups.find.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 288–292 | function | `MobileEditNodesDialog > isProviderInCurrentGroup` | 判断与 'isProviderInCurrentGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'group.use.includes'、'proxyGroups.find' |
| 290–290 | function | `MobileEditNodesDialog > isProviderInCurrentGroup > proxyGroups.find.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 295–314 | function | `MobileEditNodesDialog > toggleNodeInGroup` | 切换与 'toggleNodeInGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 2；await 0；调用 'group.proxies.filter'、'group.proxies.indexOf'、'onProxyGroupsChange'、'proxyGroups.findIndex' |
| 298–298 | function | `MobileEditNodesDialog > toggleNodeInGroup > proxyGroups.findIndex.callback#24` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 307–307 | function | `MobileEditNodesDialog > toggleNodeInGroup > group.proxies.filter.callback#25` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 317–340 | function | `MobileEditNodesDialog > toggleProviderInGroup` | 切换与 'toggleProviderInGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 2；await 0；调用 'onProxyGroupsChange'、'proxyGroups.findIndex'、'useArray.filter'、'useArray.indexOf' |
| 320–320 | function | `MobileEditNodesDialog > toggleProviderInGroup > proxyGroups.findIndex.callback#27` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 330–330 | function | `MobileEditNodesDialog > toggleProviderInGroup > useArray.filter.callback#28` | 封装 'useArray.filter.callback#28' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 343–352 | function | `MobileEditNodesDialog > addNewGroup` | 添加与 'addNewGroup' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onProxyGroupsChange'、'setExpandedGroups' |
| 363–390 | function | `MobileEditNodesDialog > handleGroupTypeChange` | 处理与 'handleGroupTypeChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onProxyGroupsChange'、'proxyGroups.map'、'proxyTypes.find' |
| 364–364 | function | `MobileEditNodesDialog > handleGroupTypeChange > proxyTypes.find.callback#31` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 365–388 | function | `MobileEditNodesDialog > handleGroupTypeChange > proxyGroups.map.callback#32` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 2；await 0 |
| 393–399 | function | `MobileEditNodesDialog > handleStrategyChange` | 处理与 'handleStrategyChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onProxyGroupsChange'、'proxyGroups.map' |
| 394–397 | function | `MobileEditNodesDialog > handleStrategyChange > proxyGroups.map.callback#34` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 2；await 0 |
| 402–414 | function | `MobileEditNodesDialog > handleDialerProxyGroupChange` | 处理与 'handleDialerProxyGroupChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onProxyGroupsChange'、'proxyGroups.map' |
| 403–412 | function | `MobileEditNodesDialog > handleDialerProxyGroupChange > proxyGroups.map.callback#36` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 2；await 0 |
| 417–419 | function | `MobileEditNodesDialog > getTypeLabel` | 读取或计算与 'getTypeLabel' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'proxyTypes.find' |
| 418–418 | function | `MobileEditNodesDialog > getTypeLabel > proxyTypes.find.callback#38` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 434–661 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 0；await 0；调用 '<BinaryExpression>.map'、'expandedGroups.has'、'getTypeLabel'、'group.proxies.map'、'proxyGroups.filter'、'proxyGroups.filter.map'、'proxyTypes.map' |
| 445–445 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > onChange.callback#40` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setEditingGroupNewName' |
| 470–470 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > onClick.callback#41` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleGroup' |
| 484–484 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > onClick.callback#42` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onRemoveGroup' |
| 500–500 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > onClick.callback#43` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'startEditGroupName' |
| 505–505 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > onOpenChange.callback#44` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setTypePopoverGroup' |
| 519–529 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > proxyTypes.map.callback#45` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 525–525 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > proxyTypes.map.callback#45 > onClick.callback#46` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'handleGroupTypeChange'、'setTypePopoverGroup' |
| 536–536 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > onValueChange.callback#47` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleStrategyChange'、'setTypePopoverGroup' |
| 554–554 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > onValueChange.callback#48` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleDialerProxyGroupChange'、'setTypePopoverGroup' |
| 561–561 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > proxyGroups.filter.callback#49` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 561–565 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > proxyGroups.filter.map.callback#50` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 576–576 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > onClick.callback#51` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'openEditSheet' |
| 585–585 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > onClick.callback#52` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleGroup' |
| 613–613 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > onDragEnd.callback#53` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleDragEnd' |
| 616–616 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > group.proxies.map.callback#54` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 619–626 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > group.proxies.map.callback#55` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 624–624 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > group.proxies.map.callback#55 > onRemove.callback#56` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onRemoveNodeFromGroup' |
| 630–654 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > <BinaryExpression>.map.callback#57` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 640–649 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > <BinaryExpression>.map.callback#57 > onClick.callback#58` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onProxyGroupsChange'、'proxyGroups.map' |
| 641–647 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > <BinaryExpression>.map.callback#57 > onClick.callback#58 > proxyGroups.map.callback#59` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 2；await 0；调用 '<BinaryExpression>.filter' |
| 643–643 | function | `MobileEditNodesDialog > proxyGroups.map.callback#39 > <BinaryExpression>.map.callback#57 > onClick.callback#58 > proxyGroups.map.callback#59 > <BinaryExpression>.filter.callback#60` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 676–676 | function | `MobileEditNodesDialog > onClick.callback#61` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onOpenChange' |
| 679–679 | function | `MobileEditNodesDialog > onClick.callback#62` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onOpenChange'、'onSave' |
| 703–703 | function | `MobileEditNodesDialog > onChange.callback#63` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSearchQuery' |
| 714–714 | function | `MobileEditNodesDialog > onClick.callback#64` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelectedTag' |
| 718–727 | function | `MobileEditNodesDialog > allTags.map.callback#65` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0 |
| 723–723 | function | `MobileEditNodesDialog > allTags.map.callback#65 > onClick.callback#66` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelectedTag' |
| 740–767 | function | `MobileEditNodesDialog > filteredAvailableNodes.map.callback#67` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 1；await 0；调用 '<ConditionalExpression>.map'、'allNodes.find'、'isNodeInCurrentGroup' |
| 741–741 | function | `MobileEditNodesDialog > filteredAvailableNodes.map.callback#67 > allNodes.find.callback#68` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 750–750 | function | `MobileEditNodesDialog > filteredAvailableNodes.map.callback#67 > onClick.callback#69` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleNodeInGroup' |
| 754–754 | function | `MobileEditNodesDialog > filteredAvailableNodes.map.callback#67 > onCheckedChange.callback#70` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleNodeInGroup' |
| 755–755 | function | `MobileEditNodesDialog > filteredAvailableNodes.map.callback#67 > onClick.callback#71` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 759–763 | function | `MobileEditNodesDialog > filteredAvailableNodes.map.callback#67 > <ConditionalExpression>.map.callback#72` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 775–800 | function | `MobileEditNodesDialog > proxyProviderConfigs.map.callback#73` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'isProviderInCurrentGroup' |
| 785–785 | function | `MobileEditNodesDialog > proxyProviderConfigs.map.callback#73 > onClick.callback#74` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleProviderInGroup' |
| 789–789 | function | `MobileEditNodesDialog > proxyProviderConfigs.map.callback#73 > onCheckedChange.callback#75` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleProviderInGroup' |
| 790–790 | function | `MobileEditNodesDialog > proxyProviderConfigs.map.callback#73 > onClick.callback#76` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 810–830 | function | `MobileEditNodesDialog > SPECIAL_NODES.map.callback#77` | 渲染并协调 'SPECIAL_NODES.map.callback#77' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'isNodeInCurrentGroup' |
| 818–818 | function | `MobileEditNodesDialog > SPECIAL_NODES.map.callback#77 > onClick.callback#78` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleNodeInGroup' |
| 822–822 | function | `MobileEditNodesDialog > SPECIAL_NODES.map.callback#77 > onCheckedChange.callback#79` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleNodeInGroup' |
| 823–823 | function | `MobileEditNodesDialog > SPECIAL_NODES.map.callback#77 > onClick.callback#80` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation' |
| 842–842 | function | `MobileEditNodesDialog > proxyGroups.find.callback#81` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 843–843 | function | `MobileEditNodesDialog > proxyGroups.find.callback#82` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 845–845 | function | `MobileEditNodesDialog > proxyGroups.find.callback#83` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |

## `components/navigation-progress.tsx`

依赖：`react`、`@tanstack/react-router`、`react-top-loading-bar`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–25 | function | `NavigationProgress` | 渲染并协调 'NavigationProgress' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'useEffect'、'useRef'、'useRouterState' |
| 9–15 | function | `NavigationProgress > useEffect.callback#2` | 封装 'useEffect.callback#2' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'ref.current.complete'、'ref.current.continuousStart' |

## `components/rule-selector.tsx`

依赖：`react`、`lucide-react`、`@/components/ui/label`、`@/components/ui/checkbox`、`@/components/ui/select`、`@/components/ui/collapsible`、`@/components/ui/button`、`@/components/ui/tooltip`、`@/hooks/use-proxy-groups`、`@/lib/sublink/types`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 27–32 | interface | `RuleSelectorProps` | 定义 'RuleSelectorProps' 的数据契约、联合类型或组件属性。 |  |
| 34–206 | function | `RuleSelector` | 渲染并协调 'RuleSelector' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'categories.map'、'useEffect'、'useProxyGroupCategories'、'useState' |
| 49–66 | function | `RuleSelector > useEffect.callback#2` | 封装 'useEffect.callback#2' Hook 的响应式状态、副作用和复用逻辑。 | 分支 5；循环 0；返回 0；await 0；调用 'categories.filter'、'categories.filter.map'、'categories.map'、'onCategoriesChange'、'setInitialized' |
| 54–54 | function | `RuleSelector > useEffect.callback#2 > categories.filter.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'c.presets.includes' |
| 54–54 | function | `RuleSelector > useEffect.callback#2 > categories.filter.map.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 56–56 | function | `RuleSelector > useEffect.callback#2 > categories.filter.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'c.presets.includes' |
| 56–56 | function | `RuleSelector > useEffect.callback#2 > categories.filter.map.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 58–58 | function | `RuleSelector > useEffect.callback#2 > categories.map.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 69–93 | function | `RuleSelector > useEffect.callback#8` | 封装 'useEffect.callback#8' Hook 的响应式状态、副作用和复用逻辑。 | 分支 6；循环 0；返回 2；await 0；调用 'categories.filter'、'categories.filter.map'、'categories.map'、'onCategoriesChange'、'setPrevRuleSet' |
| 85–85 | function | `RuleSelector > useEffect.callback#8 > categories.filter.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'c.presets.includes' |
| 85–85 | function | `RuleSelector > useEffect.callback#8 > categories.filter.map.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 87–87 | function | `RuleSelector > useEffect.callback#8 > categories.filter.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'c.presets.includes' |
| 87–87 | function | `RuleSelector > useEffect.callback#8 > categories.filter.map.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 89–89 | function | `RuleSelector > useEffect.callback#8 > categories.map.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 95–106 | function | `RuleSelector > handleCategoryToggle` | 处理与 'handleCategoryToggle' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'categories.map'、'categories.map.filter'、'onCategoriesChange'、'selectedCategories.filter'、'selectedCategories.includes' |
| 97–97 | function | `RuleSelector > handleCategoryToggle > selectedCategories.filter.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 102–102 | function | `RuleSelector > handleCategoryToggle > categories.map.callback#16` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 103–103 | function | `RuleSelector > handleCategoryToggle > categories.map.filter.callback#17` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'newCategories.includes' |
| 108–114 | function | `RuleSelector > handleRuleSetChange` | 处理与 'handleRuleSetChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onRuleSetChange'、'setIsOpen' |
| 182–198 | function | `RuleSelector > categories.map.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'selectedCategories.includes' |
| 186–186 | function | `RuleSelector > categories.map.callback#19 > onClick.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleCategoryToggle' |
| 191–191 | function | `RuleSelector > categories.map.callback#19 > onCheckedChange.callback#21` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |

## `components/sign-out-dialog.tsx`

依赖：`@tanstack/react-router`、`@tanstack/react-query`、`@/stores/auth-store`、`@/components/confirm-dialog`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–9 | interface | `SignOutDialogProps` | 定义 'SignOutDialogProps' 的数据契约、联合类型或组件属性。 |  |
| 11–39 | function | `SignOutDialog` | 渲染并协调 'SignOutDialog' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'useAuthStore'、'useNavigate'、'useQueryClient' |
| 16–25 | function | `SignOutDialog > handleSignOut` | 处理与 'handleSignOut' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'auth.reset'、'navigate'、'queryClient.removeQueries' |

## `components/speedtest-dialog.tsx`

依赖：`react`、`@tanstack/react-query`、`sonner`、`lucide-react`、`@/lib/api`、`@/components/ui/badge`、`@/components/ui/button`、`@/components/ui/input`、`@/components/ui/label`、`@/components/ui/checkbox`、`@/components/ui/dialog`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–24 | const | `PROTOCOL_COLORS` | 保存 'PROTOCOL_COLORS' 的模块级常量、配置、路由或预计算值。 |  |
| 26–35 | function | `relTime` | 执行与 'relTime' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 4；await 0；调用 '<NewExpression>.getTime'、'Date.now'、'Math.floor' |
| 38–38 | const | `RUNNING_TIMEOUT_MS` | 保存 'RUNNING_TIMEOUT_MS' 的模块级常量、配置、路由或预计算值。 |  |
| 39–44 | function | `isStaleRunning` | 判断与 'isStaleRunning' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 3；await 0；调用 '<NewExpression>.getTime'、'Date.now' |
| 46–59 | function | `useLatestSpeedResults` | 封装 'useLatestSpeedResults' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'useQuery' |
| 49–54 | function | `useLatestSpeedResults > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 1；返回 1；await 1；调用 'api.get' |
| 56–57 | function | `useLatestSpeedResults > refetchInterval` | 执行与 'refetchInterval' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'Object.values'、'Object.values.some' |
| 57–57 | function | `useLatestSpeedResults > refetchInterval > Object.values.some.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 61–80 | function | `SpeedCell` | 渲染并协调 'SpeedCell' React 组件的状态、数据请求和用户交互。 | 分支 4；循环 0；返回 5；await 0；调用 'Number'、'Number.toFixed'、'isStaleRunning' |
| 83–139 | function | `LatencyCell` | 渲染并协调 'LatencyCell' React 组件的状态、数据请求和用户交互。 | 分支 5；循环 0；返回 5；await 0；调用 'isStaleRunning' |
| 141–144 | function | `EgressIPCell` | 渲染并协调 'EgressIPCell' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 2；await 0 |
| 146–404 | function | `SpeedTestDialog` | 渲染并协调 'SpeedTestDialog' React 组件的状态、数据请求和用户交互。 | 分支 6；循环 0；返回 1；await 0；调用 'rows.map'、'testers.map'、'useEffect'、'useLatestSpeedResults'、'useMemo'、'useQuery'、'useQueryClient'、'useState' |
| 156–163 | function | `SpeedTestDialog > useState.callback#11` | 封装 'useState.callback#11' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'Number'、'isNaN'、'localStorage.getItem' |
| 164–166 | function | `SpeedTestDialog > useState.callback#12` | 封装 'useState.callback#12' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'localStorage.getItem' |
| 175–175 | function | `SpeedTestDialog > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get' |
| 181–181 | function | `SpeedTestDialog > useEffect.callback#14` | 封装 'useEffect.callback#14' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'String'、'localStorage.setItem' |
| 182–182 | function | `SpeedTestDialog > useEffect.callback#15` | 封装 'useEffect.callback#15' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'String'、'localStorage.setItem' |
| 183–187 | function | `SpeedTestDialog > useEffect.callback#16` | 封装 'useEffect.callback#16' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setSource'、'testers.some' |
| 184–184 | function | `SpeedTestDialog > useEffect.callback#16 > testers.some.callback#17` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 191–201 | function | `SpeedTestDialog > useMemo.callback#18` | 封装 'useMemo.callback#18' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 '<BinaryExpression>.map' |
| 192–200 | function | `SpeedTestDialog > useMemo.callback#18 > <BinaryExpression>.map.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 '<BinaryExpression>.toLowerCase'、'JSON.parse'、'Number' |
| 203–217 | function | `SpeedTestDialog > runTest` | 执行与 'runTest' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 1；await 1；调用 'Promise.all'、'nodeIds.map'、'queryClient.invalidateQueries'、'rows.find'、'toast.error'、'toast.success' |
| 209–209 | function | `SpeedTestDialog > runTest > nodeIds.map.callback#21` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'api.post' |
| 211–211 | function | `SpeedTestDialog > runTest > rows.find.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 220–220 | function | `SpeedTestDialog > toggleAll > rows.map.callback#24` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 220–220 | function | `SpeedTestDialog > toggleAll` | 切换与 'toggleAll' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'rows.map'、'setSelected' |
| 221–226 | function | `SpeedTestDialog > toggleOne` | 切换与 'toggleOne' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setSelected' |
| 222–226 | function | `SpeedTestDialog > toggleOne > setSelected.callback#26` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'next.add'、'next.delete'、'next.has' |
| 229–229 | function | `SpeedTestDialog > onOpenChange.callback#27` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'onClose'、'setAutoRotateTesterId'、'setHistoryNode'、'setManageTesters' |
| 233–238 | function | `SpeedTestDialog > onInteractOutside.callback#28` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 2；await 0；调用 'e.preventDefault'、'onMinimize'、'setAutoRotateTesterId'、'setHistoryNode'、'setManageTesters' |
| 239–244 | function | `SpeedTestDialog > onEscapeKeyDown.callback#29` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 2；await 0；调用 'e.preventDefault'、'onMinimize'、'setAutoRotateTesterId'、'setHistoryNode'、'setManageTesters' |
| 247–247 | function | `SpeedTestDialog > onBack.callback#30` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setHistoryNode' |
| 249–249 | function | `SpeedTestDialog > onBack.callback#31` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setAutoRotateTesterId'、'setManageTesters' |
| 259–259 | function | `SpeedTestDialog > onClick.callback#32` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setSource' |
| 262–281 | function | `SpeedTestDialog > testers.map.callback#33` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 0；await 0 |
| 267–275 | function | `SpeedTestDialog > testers.map.callback#33 > onClick.callback#34` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'setAutoRotateTesterId'、'setManageTesters'、'setSource' |
| 283–283 | function | `SpeedTestDialog > onClick.callback#35` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setThreads' |
| 286–286 | function | `SpeedTestDialog > onClick.callback#36` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setThreads' |
| 291–291 | function | `SpeedTestDialog > onClick.callback#37` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'Array.from'、'runTest' |
| 296–296 | function | `SpeedTestDialog > onClick.callback#38` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setManageTesters' |
| 323–353 | function | `SpeedTestDialog > rows.map.callback#39` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'isStaleRunning'、'r.protocol.toUpperCase'、'selected.has' |
| 329–329 | function | `SpeedTestDialog > rows.map.callback#39 > onCheckedChange.callback#40` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleOne' |
| 339–339 | function | `SpeedTestDialog > rows.map.callback#39 > onProbe.callback#41` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'runTest' |
| 343–343 | function | `SpeedTestDialog > rows.map.callback#39 > onClick.callback#42` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setHistoryNode' |
| 346–346 | function | `SpeedTestDialog > rows.map.callback#39 > onClick.callback#43` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'runTest' |
| 360–395 | function | `SpeedTestDialog > rows.map.callback#44` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'isStaleRunning'、'r.protocol.toUpperCase'、'selected.has' |
| 366–366 | function | `SpeedTestDialog > rows.map.callback#44 > onCheckedChange.callback#45` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toggleOne' |
| 379–379 | function | `SpeedTestDialog > rows.map.callback#44 > onProbe.callback#46` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'runTest' |
| 385–385 | function | `SpeedTestDialog > rows.map.callback#44 > onClick.callback#47` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setHistoryNode' |
| 388–388 | function | `SpeedTestDialog > rows.map.callback#44 > onClick.callback#48` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'runTest' |
| 406–467 | function | `HistoryView` | 渲染并协调 'HistoryView' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'rows.map'、'useQuery' |
| 409–409 | function | `HistoryView > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get' |
| 410–410 | function | `HistoryView > refetchInterval > <BinaryExpression>.some.callback#52` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 410–410 | function | `HistoryView > refetchInterval` | 执行与 'refetchInterval' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 '<BinaryExpression>.some' |
| 423–423 | function | `HistoryView > onClick.callback#53` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'refetch' |
| 442–460 | function | `HistoryView > rows.map.callback#54` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 0；await 0；调用 '<NewExpression>.toLocaleString'、'Number'、'Number.toFixed'、'relTime' |
| 470–613 | function | `TestersView` | 渲染并协调 'TestersView' React 组件的状态、数据请求和用户交互。 | 分支 4；循环 0；返回 1；await 0；调用 '<BinaryExpression>.map'、'useEffect'、'useMutation'、'useQuery'、'useQueryClient'、'useState' |
| 478–478 | function | `TestersView > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get' |
| 482–486 | function | `TestersView > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post'、'name.trim' |
| 487–487 | function | `TestersView > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'qc.invalidateQueries'、'setNewCred'、'toast.success' |
| 488–488 | function | `TestersView > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 491–491 | function | `TestersView > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.post' |
| 492–492 | function | `TestersView > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'qc.invalidateQueries'、'toast.success' |
| 493–493 | function | `TestersView > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 497–500 | function | `TestersView > mutationFn` | 执行与 'mutationFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 501–501 | function | `TestersView > onSuccess` | 执行与 'onSuccess' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'qc.invalidateQueries'、'setNewCred'、'toast.success' |
| 502–502 | function | `TestersView > onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.error' |
| 505–512 | function | `TestersView > useEffect.callback#66` | 封装 'useEffect.callback#66' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 '<BinaryExpression>.find'、'rotateMut.mutate' |
| 507–507 | function | `TestersView > useEffect.callback#66 > <BinaryExpression>.find.callback#67` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 514–514 | function | `TestersView > copy` | 执行与 'copy' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'navigator.clipboard.writeText'、'navigator.clipboard.writeText.then' |
| 514–514 | function | `TestersView > copy > navigator.clipboard.writeText.then.callback#69` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'toast.success' |
| 514–514 | function | `TestersView > copy > navigator.clipboard.writeText.then.callback#70` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 547–547 | function | `TestersView > onChange.callback#71` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setName' |
| 549–549 | function | `TestersView > onClick.callback#72` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'createMut.mutate' |
| 564–564 | function | `TestersView > onClick.callback#73` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'copy' |
| 569–569 | function | `TestersView > onClick.callback#74` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'copy' |
| 574–574 | function | `TestersView > onClick.callback#75` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'copy' |
| 584–608 | function | `TestersView > <BinaryExpression>.map.callback#76` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0 |
| 596–596 | function | `TestersView > <BinaryExpression>.map.callback#76 > onClick.callback#77` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'rotateMut.mutate' |
| 603–603 | function | `TestersView > <BinaryExpression>.map.callback#76 > onClick.callback#78` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'revokeMut.mutate' |

## `components/theme-switch.tsx`

依赖：`react`、`lucide-react`、`@/context/theme-provider`、`@/components/ui/button`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–45 | function | `ThemeSwitch` | 渲染并协调 'ThemeSwitch' React 组件的状态、数据请求和用户交互。 | 分支 4；循环 0；返回 1；await 0；调用 'useEffect'、'useTheme' |
| 11–15 | function | `ThemeSwitch > useEffect.callback#2` | 封装 'useEffect.callback#2' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 0；await 0；调用 'document.querySelector'、'metaThemeColor.setAttribute' |
| 18–26 | function | `ThemeSwitch > cycleTheme` | 执行与 'cycleTheme' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0；调用 'setTheme' |

## `components/twemoji.tsx`

依赖：`react`、`twemoji`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–7 | interface | `TwemojiProps` | 定义 'TwemojiProps' 的数据契约、联合类型或组件属性。 |  |
| 10–10 | const | `parseCache` | 保存 'parseCache' 的模块级常量、配置、路由或预计算值。 |  |
| 17–46 | const | `Twemoji` | 保存 'Twemoji' 的模块级常量、配置、路由或预计算值。 |  |
| 17–46 | function | `Twemoji` | 渲染并协调 'Twemoji' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'useMemo' |
| 18–38 | function | `Twemoji > useMemo.callback#2` | 封装 'useMemo.callback#2' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 2；await 0；调用 'String'、'document.createElement'、'parseCache.get'、'parseCache.has'、'parseCache.set'、'twemoji.parse' |

## `components/update-dialog.tsx`

依赖：`react`、`@tanstack/react-query`、`lucide-react`、`sonner`、`@/lib/api`、`@/components/ui/dialog`、`@/components/ui/button`、`@/components/ui/progress`、`@/stores/auth-store`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 15–45 | function | `processInline` | 执行与 'processInline' 对应的前端业务、状态或数据转换逻辑。 | 分支 6；循环 1；返回 1；await 0；调用 'parts.push'、'pattern.exec'、'text.slice' |
| 47–62 | function | `ReleaseNotes` | 渲染并协调 'ReleaseNotes' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'text.split'、'text.split.map' |
| 50–59 | function | `ReleaseNotes > text.split.map.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 3；循环 0；返回 4；await 0；调用 '<RegularExpressionLiteral>.test'、'line.replace'、'line.trim'、'processInline' |
| 74–77 | interface | `UpdateDialogProps` | 定义 'UpdateDialogProps' 的数据契约、联合类型或组件属性。 |  |
| 79–86 | interface | `UpdateInfo` | 定义 'UpdateInfo' 的数据契约、联合类型或组件属性。 |  |
| 88–92 | interface | `UpdateProgress` | 定义 'UpdateProgress' 的数据契约、联合类型或组件属性。 |  |
| 94–100 | const | `STEPS` | 保存 'STEPS' 的模块级常量、配置、路由或预计算值。 |  |
| 102–360 | function | `UpdateDialog` | 渲染并协调 'UpdateDialog' React 组件的状态、数据请求和用户交互。 | 分支 3；循环 0；返回 1；await 0；调用 'STEPS.findIndex'、'STEPS.map'、'useAuthStore'、'useCallback'、'useQuery'、'useRef'、'useState' |
| 116–119 | function | `UpdateDialog > queryFn` | 执行与 'queryFn' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.get' |
| 126–196 | function | `UpdateDialog > useCallback.callback#6` | 封装 'useCallback.callback#6' Hook 的响应式状态、副作用和复用逻辑。 | 分支 9；循环 2；返回 2；await 2；调用 'JSON.parse'、'buffer.split'、'decoder.decode'、'fetch'、'line.slice'、'line.startsWith'、'lines.pop'、'reader.read'、'response.body.getReader'、'setIsUpdating'、'setTimeout'、'setUpdateProgress'、'toast.error'、'toast.success' |
| 168–170 | function | `UpdateDialog > useCallback.callback#6 > setTimeout.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'window.location.reload' |
| 199–201 | function | `UpdateDialog > handleOpenChange` | 处理与 'handleOpenChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onOpenChange' |
| 206–206 | function | `UpdateDialog > STEPS.findIndex.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 257–289 | function | `UpdateDialog > STEPS.map.callback#10` | 渲染并协调 'STEPS.map.callback#10' React 组件的状态、数据请求和用户交互。 | 分支 4；循环 0；返回 1；await 0 |
| 326–326 | function | `UpdateDialog > onClick.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'window.open' |
| 347–347 | function | `UpdateDialog > onClick.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'refetch' |

