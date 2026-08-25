# TypeScript 分区 `components-template-v3`

V3 模板编辑、预览、筛选与代理组控件。

## `components/template-v3/keyword-filter-input.tsx`

依赖：`@/components/ui/input`、`@/components/ui/label`、`@/components/ui/badge`、`@/lib/template-v3-utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–14 | interface | `KeywordFilterInputProps` | 定义 'KeywordFilterInputProps' 的数据契约、联合类型或组件属性。 |  |
| 16–62 | function | `KeywordFilterInput` | 渲染并协调 'KeywordFilterInput' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'keywordsToRegex' |
| 39–45 | function | `KeywordFilterInput > onChange.callback#2` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'onChange'、'onVariableCleared' |

## `components/template-v3/proxy-group-editor.tsx`

依赖：`@/components/ui/button`、`@/components/ui/input`、`@/components/ui/label`、`@/components/ui/switch`、`@/components/ui/badge`、`@/components/ui/select`、`@/components/ui/collapsible`、`@/components/ui/tooltip`、`lucide-react`、`react`、`./keyword-filter-input`、`./proxy-type-select`、`./proxy-group-select`、`@/lib/template-v3-utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 22–35 | interface | `ProxyGroupEditorProps` | 定义 'ProxyGroupEditorProps' 的数据契约、联合类型或组件属性。 |  |
| 37–43 | const | `GROUP_TYPE_LABELS` | 保存 'GROUP_TYPE_LABELS' 的模块级常量、配置、路由或预计算值。 |  |
| 45–416 | function | `ProxyGroupEditor` | 渲染并协调 'ProxyGroupEditor' React 组件的状态、数据请求和用户交互。 | 分支 5；循环 0；返回 1；await 0；调用 '<ArrayLiteralExpression>.includes'、'<RegularExpressionLiteral>.test'、'Object.entries'、'Object.entries.map'、'Object.keys'、'PROXY_GROUP_TYPES.map'、'allGroupNames.filter'、'allGroupNames.filter.map'、'hasProxyNodes'、'hasProxyProviders'、'useState' |
| 62–67 | function | `ProxyGroupEditor > updateField` | 更新与 'updateField' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onChange' |
| 102–102 | function | `ProxyGroupEditor > onClick.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'setShowRelayPicker' |
| 112–112 | function | `ProxyGroupEditor > onClick.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'setShowRelayPicker' |
| 121–121 | function | `ProxyGroupEditor > onClick.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'onMoveUp' |
| 131–131 | function | `ProxyGroupEditor > onClick.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'onMoveDown' |
| 140–140 | function | `ProxyGroupEditor > onClick.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'e.stopPropagation'、'onDelete' |
| 157–157 | function | `ProxyGroupEditor > onClick.callback#8` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 164–164 | function | `ProxyGroupEditor > allGroupNames.filter.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 164–175 | function | `ProxyGroupEditor > allGroupNames.filter.map.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0 |
| 171–171 | function | `ProxyGroupEditor > allGroupNames.filter.map.callback#10 > onClick.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'updateField' |
| 188–188 | function | `ProxyGroupEditor > onChange.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 197–197 | function | `ProxyGroupEditor > onValueChange.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 203–207 | function | `ProxyGroupEditor > PROXY_GROUP_TYPES.map.callback#14` | 渲染并协调 'PROXY_GROUP_TYPES.map.callback#14' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 220–227 | function | `ProxyGroupEditor > onCheckedChange.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onChange' |
| 234–241 | function | `ProxyGroupEditor > onCheckedChange.callback#16` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'onChange' |
| 248–255 | function | `ProxyGroupEditor > onCheckedChange.callback#17` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'onChange' |
| 263–263 | function | `ProxyGroupEditor > onCheckedChange.callback#18` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 271–271 | function | `ProxyGroupEditor > onCheckedChange.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 283–283 | function | `ProxyGroupEditor > onChange.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 284–284 | function | `ProxyGroupEditor > allGroupNames.filter.callback#21` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 306–311 | function | `ProxyGroupEditor > Object.entries.map.callback#22` | 渲染并协调 'Object.entries.map.callback#22' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 324–324 | function | `ProxyGroupEditor > onChange.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 325–325 | function | `ProxyGroupEditor > onVariableCleared.callback#24` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 333–333 | function | `ProxyGroupEditor > onChange.callback#25` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 334–334 | function | `ProxyGroupEditor > onVariableCleared.callback#26` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 346–346 | function | `ProxyGroupEditor > onChange.callback#27` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 352–352 | function | `ProxyGroupEditor > onChange.callback#28` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 364–364 | function | `ProxyGroupEditor > onChange.callback#29` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 374–374 | function | `ProxyGroupEditor > onChange.callback#30` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parseInt'、'updateField' |
| 384–384 | function | `ProxyGroupEditor > onChange.callback#31` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'parseInt'、'updateField' |
| 398–398 | function | `ProxyGroupEditor > onChange.callback#32` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |
| 406–406 | function | `ProxyGroupEditor > onCheckedChange.callback#33` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'updateField' |

## `components/template-v3/proxy-group-select.tsx`

依赖：`react`、`@/components/ui/label`、`@/components/ui/button`、`@/components/ui/popover`、`@/components/ui/command`、`lucide-react`、`@/lib/utils`、`@/lib/template-v3-utils`、`@dnd-kit/core`、`@dnd-kit/sortable`、`@dnd-kit/utilities`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 41–51 | interface | `ProxyGroupSelectProps` | 定义 'ProxyGroupSelectProps' 的数据契约、联合类型或组件属性。 |  |
| 53–56 | interface | `SortableItemProps` | 定义 'SortableItemProps' 的数据契约、联合类型或组件属性。 |  |
| 59–65 | const | `SPECIAL_MARKERS` | 保存 'SPECIAL_MARKERS' 的模块级常量、配置、路由或预计算值。 |  |
| 67–69 | function | `isSpecialMarker` | 判断与 'isSpecialMarker' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'SPECIAL_MARKERS.has' |
| 71–80 | function | `markerDisplayName` | 执行与 'markerDisplayName' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 6；await 0 |
| 82–91 | function | `markerBgClass` | 执行与 'markerBgClass' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 5；await 0 |
| 93–124 | function | `SortableItem` | 渲染并协调 'SortableItem' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'CSS.Transform.toString'、'cn'、'isSpecialMarker'、'markerBgClass'、'markerDisplayName'、'useSortable' |
| 117–117 | function | `SortableItem > onClick.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onRemove' |
| 126–133 | function | `DragOverlayItem` | 渲染并协调 'DragOverlayItem' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn'、'markerBgClass'、'markerDisplayName' |
| 135–297 | function | `ProxyGroupSelect` | 渲染并协调 'ProxyGroupSelect' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'JSON.stringify'、'availableGroups.map'、'displayItems.map'、'ensureMarkers'、'internalOrder.filter'、'setTimeout'、'useEffect'、'useRef'、'useSensor'、'useSensors'、'useState' |
| 153–157 | function | `ProxyGroupSelect > useEffect.callback#8` | 封装 'useEffect.callback#8' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'setInternalOrder' |
| 166–169 | function | `ProxyGroupSelect > handleDragStart` | 处理与 'handleDragStart' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setActiveId' |
| 171–180 | function | `ProxyGroupSelect > handleDragOver` | 处理与 'handleDragOver' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0；调用 'arrayMove'、'internalOrder.indexOf'、'setInternalOrder' |
| 182–189 | function | `ProxyGroupSelect > handleDragEnd` | 处理与 'handleDragEnd' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'JSON.stringify'、'onChange'、'setActiveId' |
| 191–197 | function | `ProxyGroupSelect > handleSelect` | 处理与 'handleSelect' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'onChange'、'value.filter'、'value.includes' |
| 193–193 | function | `ProxyGroupSelect > handleSelect > value.filter.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 199–201 | function | `ProxyGroupSelect > handleRemove` | 处理与 'handleRemove' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onChange'、'value.filter' |
| 200–200 | function | `ProxyGroupSelect > handleRemove > value.filter.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 204–210 | function | `ProxyGroupSelect > internalOrder.filter.callback#16` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 4；循环 0；返回 5；await 0 |
| 213–243 | function | `ProxyGroupSelect > ensureMarkers` | 执行与 'ensureMarkers' 对应的前端业务、状态或数据转换逻辑。 | 分支 10；循环 0；返回 1；await 0；调用 'result.filter'、'result.includes'、'result.push' |
| 231–231 | function | `ProxyGroupSelect > ensureMarkers > result.filter.callback#18` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 234–234 | function | `ProxyGroupSelect > ensureMarkers > result.filter.callback#19` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 237–237 | function | `ProxyGroupSelect > ensureMarkers > result.filter.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 240–240 | function | `ProxyGroupSelect > ensureMarkers > result.filter.callback#21` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 249–249 | function | `ProxyGroupSelect > setTimeout.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onChange' |
| 260–262 | function | `ProxyGroupSelect > displayItems.map.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 283–288 | function | `ProxyGroupSelect > availableGroups.map.callback#24` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'cn'、'value.includes' |
| 284–284 | function | `ProxyGroupSelect > availableGroups.map.callback#24 > onSelect.callback#25` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleSelect' |

## `components/template-v3/proxy-type-select.tsx`

依赖：`react`、`@/components/ui/button`、`@/components/ui/checkbox`、`@/components/ui/label`、`@/components/ui/badge`、`@/components/ui/popover`、`@/components/ui/scroll-area`、`lucide-react`、`@/lib/template-v3-utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 11–16 | interface | `ProxyTypeSelectProps` | 定义 'ProxyTypeSelectProps' 的数据契约、联合类型或组件属性。 |  |
| 18–97 | function | `ProxyTypeSelect` | 渲染并协调 'ProxyTypeSelect' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 1；await 0；调用 'PROXY_TYPES.map'、'useState'、'value.slice'、'value.slice.join' |
| 26–32 | function | `ProxyTypeSelect > handleToggle` | 处理与 'handleToggle' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'onChange'、'value.filter'、'value.includes' |
| 28–28 | function | `ProxyTypeSelect > handleToggle > value.filter.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 34–40 | function | `ProxyTypeSelect > handleSelectAll` | 处理与 'handleSelectAll' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'onChange' |
| 78–90 | function | `ProxyTypeSelect > PROXY_TYPES.map.callback#5` | 渲染并协调 'PROXY_TYPES.map.callback#5' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0；调用 'value.includes' |
| 82–82 | function | `ProxyTypeSelect > PROXY_TYPES.map.callback#5 > onClick.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleToggle' |
| 86–86 | function | `ProxyTypeSelect > PROXY_TYPES.map.callback#5 > onCheckedChange.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'handleToggle' |

## `components/template-v3/template-preview.tsx`

依赖：`@/components/ui/button`、`@/components/ui/card`、`@/components/ui/scroll-area`、`lucide-react`、`sonner`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 7–13 | interface | `TemplatePreviewProps` | 定义 'TemplatePreviewProps' 的数据契约、联合类型或组件属性。 |  |
| 15–75 | function | `TemplatePreview` | 渲染并协调 'TemplatePreview' React 组件的状态、数据请求和用户交互。 | 分支 3；循环 0；返回 1；await 0 |
| 22–29 | function | `TemplatePreview > handleCopy` | 处理与 'handleCopy' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'navigator.clipboard.writeText'、'toast.error'、'toast.success' |

## `components/template-v3/template-upload-dialog.tsx`

依赖：`react`、`@/components/ui/button`、`@/components/ui/input`、`@/components/ui/label`、`@/components/ui/textarea`、`@/components/ui/dialog`、`@/components/ui/tabs`、`@/components/ui/select`、`lucide-react`、`sonner`、`@/lib/template-v3-utils`、`@/lib/api`、`@/config/custom-rules-templates`、`@/lib/template-presets`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 16–20 | interface | `UserTemplate` | 定义 'UserTemplate' 的数据契约、联合类型或组件属性。 |  |
| 22–27 | interface | `SubscribeFile` | 定义 'SubscribeFile' 的数据契约、联合类型或组件属性。 |  |
| 29–35 | interface | `TemplateUploadDialogProps` | 定义 'TemplateUploadDialogProps' 的数据契约、联合类型或组件属性。 |  |
| 37–754 | function | `TemplateUploadDialog` | 渲染并协调 'TemplateUploadDialog' React 组件的状态、数据请求和用户交互。 | 分支 8；循环 0；返回 1；await 0；调用 'ALL_TEMPLATE_PRESETS.map'、'Object.entries'、'Object.entries.map'、'importUrl.trim'、'subscribeFiles.map'、'useEffect'、'useState'、'userTemplates.map' |
| 69–73 | function | `TemplateUploadDialog > useEffect.callback#2` | 封装 'useEffect.callback#2' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'fetchUserTemplates' |
| 76–80 | function | `TemplateUploadDialog > useEffect.callback#3` | 封装 'useEffect.callback#3' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'fetchSubscriptions' |
| 82–92 | function | `TemplateUploadDialog > fetchUserTemplates` | 从后端获取与 'fetchUserTemplates' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get'、'setIsFetchingTemplates'、'setUserTemplates'、'toast.error' |
| 94–104 | function | `TemplateUploadDialog > fetchSubscriptions` | 从后端获取与 'fetchSubscriptions' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 1；调用 'api.get'、'setIsFetchingSubscriptions'、'setSubscribeFiles'、'toast.error' |
| 106–117 | function | `TemplateUploadDialog > resetForm` | 重置与 'resetForm' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'setAnalysisPreview'、'setImportUrl'、'setNewTemplateName'、'setPasteContent'、'setSelectedDnsPreset'、'setSelectedFile'、'setSelectedSubscription'、'setSelectedV2Template'、'setTab'、'setUrlPreview' |
| 119–122 | function | `TemplateUploadDialog > handleClose` | 处理与 'handleClose' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'onOpenChange'、'resetForm' |
| 124–134 | function | `TemplateUploadDialog > handleFileChange` | 处理与 'handleFileChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 0；调用 'file.name.toLowerCase'、'lower.endsWith'、'setSelectedFile'、'toast.error' |
| 136–178 | function | `TemplateUploadDialog > handleSubmit` | 处理与 'handleSubmit' 对应的前端业务、状态或数据转换逻辑。 | 分支 13；循环 0；返回 7；await 0；调用 '<RegularExpressionLiteral>.test'、'createBlankTemplate'、'handleFromSubscription'、'handleFromUrl'、'handleV2Import'、'name.endsWith'、'newTemplateName.trim'、'onCreate'、'onUpload'、'pasteContent.trim'、'resetForm'、'toast.error' |
| 180–211 | function | `TemplateUploadDialog > handleFromSubscription` | 处理与 'handleFromSubscription' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 2；await 1；调用 'api.post'、'name.endsWith'、'newTemplateName.trim'、'onCreate'、'resetForm'、'setIsAnalyzing'、'toast.error' |
| 213–237 | function | `TemplateUploadDialog > handleAnalyzePreview` | 处理与 'handleAnalyzePreview' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 1；await 1；调用 'api.post'、'setAnalysisPreview'、'setIsAnalyzing'、'setNewTemplateName'、'subscribeFiles.find'、'toast.error' |
| 228–228 | function | `TemplateUploadDialog > handleAnalyzePreview > subscribeFiles.find.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 239–265 | function | `TemplateUploadDialog > handleFromUrl` | 处理与 'handleFromUrl' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 2；await 1；调用 'fetchUrlContent'、'importUrl.trim'、'name.endsWith'、'newTemplateName.trim'、'onCreate'、'resetForm'、'setIsFetchingUrl'、'toast.error' |
| 267–282 | function | `TemplateUploadDialog > handleUrlPreview` | 处理与 'handleUrlPreview' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 1；调用 'fetchUrlContent'、'importUrl.trim'、'setIsFetchingUrl'、'setUrlPreview'、'toast.error' |
| 284–290 | function | `TemplateUploadDialog > fetchUrlContent` | 从后端获取与 'fetchUrlContent' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 1；调用 'api.post' |
| 292–356 | function | `TemplateUploadDialog > handleV2Import` | 处理与 'handleV2Import' 对应的前端业务、状态或数据转换逻辑。 | 分支 7；循环 0；返回 5；await 2；调用 'ALL_TEMPLATE_PRESETS.find'、'api.post'、'generateV3TemplateFromConversion'、'name.endsWith'、'newTemplateName.trim'、'onCreate'、'resetForm'、'selectedV2Template.replace'、'selectedV2Template.startsWith'、'setIsConverting'、'toast.error'、'userTemplates.find' |
| 308–308 | function | `TemplateUploadDialog > handleV2Import > userTemplates.find.callback#17` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 't.id.toString' |
| 316–316 | function | `TemplateUploadDialog > handleV2Import > ALL_TEMPLATE_PRESETS.find.callback#18` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 359–446 | function | `TemplateUploadDialog > generateV3TemplateFromConversion` | 生成与 'generateV3TemplateFromConversion' 对应的前端业务、状态或数据转换逻辑。 | 分支 11；循环 5；返回 1；await 0；调用 'Object.entries'、'Object.keys'、'dnsPreset.content.split'、'lines.join'、'lines.push' |
| 449–468 | function | `TemplateUploadDialog > handleV2TemplateSelect` | 处理与 'handleV2TemplateSelect' 对应的前端业务、状态或数据转换逻辑。 | 分支 5；循环 0；返回 0；await 0；调用 'ALL_TEMPLATE_PRESETS.find'、'setNewTemplateName'、'setSelectedV2Template'、'userTemplates.find'、'value.replace'、'value.startsWith' |
| 454–454 | function | `TemplateUploadDialog > handleV2TemplateSelect > userTemplates.find.callback#21` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 't.id.toString' |
| 460–460 | function | `TemplateUploadDialog > handleV2TemplateSelect > ALL_TEMPLATE_PRESETS.find.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 480–480 | function | `TemplateUploadDialog > onValueChange.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setTab' |
| 529–529 | function | `TemplateUploadDialog > onChange.callback#24` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNewTemplateName' |
| 537–537 | function | `TemplateUploadDialog > onChange.callback#25` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setPasteContent' |
| 549–549 | function | `TemplateUploadDialog > onChange.callback#26` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNewTemplateName' |
| 563–566 | function | `TemplateUploadDialog > onChange.callback#27` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setImportUrl'、'setUrlPreview' |
| 575–575 | function | `TemplateUploadDialog > onChange.callback#28` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNewTemplateName' |
| 621–625 | function | `TemplateUploadDialog > userTemplates.map.callback#29` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 630–634 | function | `TemplateUploadDialog > ALL_TEMPLATE_PRESETS.map.callback#30` | 渲染并协调 'ALL_TEMPLATE_PRESETS.map.callback#30' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 644–644 | function | `TemplateUploadDialog > onChange.callback#31` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNewTemplateName' |
| 656–660 | function | `TemplateUploadDialog > Object.entries.map.callback#32` | 渲染并协调 'Object.entries.map.callback#32' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0 |
| 686–690 | function | `TemplateUploadDialog > subscribeFiles.map.callback#33` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 699–699 | function | `TemplateUploadDialog > onChange.callback#34` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'setNewTemplateName' |

