# TypeScript 分区 `components-ui`

基于 Radix UI/Tailwind 的无业务基础 UI 封装。

## `components/ui/alert-dialog.tsx`

依赖：`react`、`@radix-ui/react-alert-dialog`、`@/lib/utils`、`@/components/ui/button`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–10 | function | `AlertDialog` | 渲染并协调 'AlertDialog' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 12–18 | function | `AlertDialogTrigger` | 渲染并协调 'AlertDialogTrigger' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 20–26 | function | `AlertDialogPortal` | 渲染并协调 'AlertDialogPortal' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 28–42 | function | `AlertDialogOverlay` | 渲染并协调 'AlertDialogOverlay' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 44–61 | function | `AlertDialogContent` | 渲染并协调 'AlertDialogContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 63–74 | function | `AlertDialogHeader` | 渲染并协调 'AlertDialogHeader' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 76–90 | function | `AlertDialogFooter` | 渲染并协调 'AlertDialogFooter' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 92–103 | function | `AlertDialogTitle` | 渲染并协调 'AlertDialogTitle' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 105–116 | function | `AlertDialogDescription` | 渲染并协调 'AlertDialogDescription' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 118–128 | function | `AlertDialogAction` | 渲染并协调 'AlertDialogAction' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'buttonVariants'、'cn' |
| 130–140 | function | `AlertDialogCancel` | 渲染并协调 'AlertDialogCancel' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'buttonVariants'、'cn' |

## `components/ui/alert.tsx`

依赖：`react`、`class-variance-authority`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–19 | const | `alertVariants` | 保存 'alertVariants' 的模块级常量、配置、路由或预计算值。 |  |
| 21–34 | function | `Alert` | 渲染并协调 'Alert' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'alertVariants'、'cn' |
| 36–47 | function | `AlertTitle` | 渲染并协调 'AlertTitle' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 49–63 | function | `AlertDescription` | 渲染并协调 'AlertDescription' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/avatar.tsx`

依赖：`react`、`@radix-ui/react-avatar`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–19 | function | `Avatar` | 渲染并协调 'Avatar' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 21–32 | function | `AvatarImage` | 渲染并协调 'AvatarImage' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 34–48 | function | `AvatarFallback` | 渲染并协调 'AvatarFallback' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/badge.tsx`

依赖：`react`、`@radix-ui/react-slot`、`class-variance-authority`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–25 | const | `badgeVariants` | 保存 'badgeVariants' 的模块级常量、配置、路由或预计算值。 |  |
| 27–43 | function | `Badge` | 渲染并协调 'Badge' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'badgeVariants'、'cn' |

## `components/ui/button-group.tsx`

依赖：`react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–21 | interface | `ButtonGroupProps` | 定义 'ButtonGroupProps' 的数据契约、联合类型或组件属性。 |  |
| 35–69 | const | `ButtonGroup` | 保存 'ButtonGroup' 的模块级常量、配置、路由或预计算值。 |  |
| 36–68 | function | `React.forwardRef.callback#1` | 渲染并协调 'React.forwardRef.callback#1' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/button.tsx`

依赖：`react`、`@radix-ui/react-slot`、`class-variance-authority`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–36 | const | `buttonVariants` | 保存 'buttonVariants' 的模块级常量、配置、路由或预计算值。 |  |
| 38–57 | function | `Button` | 渲染并协调 'Button' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'buttonVariants'、'cn' |

## `components/ui/calendar.tsx`

依赖：`react`、`lucide-react`、`react-day-picker`、`@/lib/utils`、`@/components/ui/button`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 11–170 | function | `Calendar` | 渲染并协调 'Calendar' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'buttonVariants'、'cn'、'getDefaultClassNames' |
| 36–37 | function | `Calendar > formatMonthDropdown` | 格式化与 'formatMonthDropdown' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'date.toLocaleString' |
| 125–134 | function | `Calendar > Root` | 渲染并协调 'Root' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 135–154 | function | `Calendar > Chevron` | 渲染并协调 'Chevron' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 3；await 0；调用 'cn' |
| 156–164 | function | `Calendar > WeekNumber` | 渲染并协调 'WeekNumber' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 172–208 | function | `CalendarDayButton` | 渲染并协调 'CalendarDayButton' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'React.useEffect'、'React.useRef'、'cn'、'day.date.toLocaleDateString'、'getDefaultClassNames' |
| 181–183 | function | `CalendarDayButton > React.useEffect.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'ref.current.focus' |

## `components/ui/card.tsx`

依赖：`react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–15 | function | `Card` | 渲染并协调 'Card' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 17–28 | function | `CardHeader` | 渲染并协调 'CardHeader' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 30–38 | function | `CardTitle` | 渲染并协调 'CardTitle' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 40–48 | function | `CardDescription` | 渲染并协调 'CardDescription' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 50–61 | function | `CardAction` | 渲染并协调 'CardAction' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 63–71 | function | `CardContent` | 渲染并协调 'CardContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 73–81 | function | `CardFooter` | 渲染并协调 'CardFooter' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/checkbox.tsx`

依赖：`react`、`@radix-ui/react-checkbox`、`lucide-react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–27 | function | `Checkbox` | 渲染并协调 'Checkbox' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/collapsible.tsx`

依赖：`@radix-ui/react-collapsible`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–7 | function | `Collapsible` | 渲染并协调 'Collapsible' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 9–18 | function | `CollapsibleTrigger` | 渲染并协调 'CollapsibleTrigger' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 20–29 | function | `CollapsibleContent` | 渲染并协调 'CollapsibleContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |

## `components/ui/command.tsx`

依赖：`react`、`cmdk`、`lucide-react`、`@/lib/utils`、`@/components/ui/dialog`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 13–27 | function | `Command` | 渲染并协调 'Command' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 29–58 | function | `CommandDialog` | 渲染并协调 'CommandDialog' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 60–80 | function | `CommandInput` | 渲染并协调 'CommandInput' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 82–96 | function | `CommandList` | 渲染并协调 'CommandList' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 98–108 | function | `CommandEmpty` | 渲染并协调 'CommandEmpty' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 110–124 | function | `CommandGroup` | 渲染并协调 'CommandGroup' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 126–137 | function | `CommandSeparator` | 渲染并协调 'CommandSeparator' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 139–153 | function | `CommandItem` | 渲染并协调 'CommandItem' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 155–169 | function | `CommandShortcut` | 渲染并协调 'CommandShortcut' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/dialog.tsx`

依赖：`react`、`@radix-ui/react-dialog`、`lucide-react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–12 | function | `Dialog` | 渲染并协调 'Dialog' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 14–18 | function | `DialogTrigger` | 渲染并协调 'DialogTrigger' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 20–24 | function | `DialogPortal` | 渲染并协调 'DialogPortal' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 26–30 | function | `DialogClose` | 渲染并协调 'DialogClose' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 32–46 | function | `DialogOverlay` | 渲染并协调 'DialogOverlay' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 48–81 | function | `DialogContent` | 渲染并协调 'DialogContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 83–91 | function | `DialogHeader` | 渲染并协调 'DialogHeader' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 93–104 | function | `DialogFooter` | 渲染并协调 'DialogFooter' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 106–117 | function | `DialogTitle` | 渲染并协调 'DialogTitle' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 119–130 | function | `DialogDescription` | 渲染并协调 'DialogDescription' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/dropdown-menu.tsx`

依赖：`react`、`@radix-ui/react-dropdown-menu`、`lucide-react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–10 | function | `DropdownMenu` | 渲染并协调 'DropdownMenu' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 12–18 | function | `DropdownMenuPortal` | 渲染并协调 'DropdownMenuPortal' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 20–29 | function | `DropdownMenuTrigger` | 渲染并协调 'DropdownMenuTrigger' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 31–49 | function | `DropdownMenuContent` | 渲染并协调 'DropdownMenuContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 51–57 | function | `DropdownMenuGroup` | 渲染并协调 'DropdownMenuGroup' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 59–80 | function | `DropdownMenuItem` | 渲染并协调 'DropdownMenuItem' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 82–106 | function | `DropdownMenuCheckboxItem` | 渲染并协调 'DropdownMenuCheckboxItem' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 108–117 | function | `DropdownMenuRadioGroup` | 渲染并协调 'DropdownMenuRadioGroup' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 119–141 | function | `DropdownMenuRadioItem` | 渲染并协调 'DropdownMenuRadioItem' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 143–161 | function | `DropdownMenuLabel` | 渲染并协调 'DropdownMenuLabel' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 163–174 | function | `DropdownMenuSeparator` | 渲染并协调 'DropdownMenuSeparator' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 176–190 | function | `DropdownMenuShortcut` | 渲染并协调 'DropdownMenuShortcut' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 192–196 | function | `DropdownMenuSub` | 渲染并协调 'DropdownMenuSub' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 198–220 | function | `DropdownMenuSubTrigger` | 渲染并协调 'DropdownMenuSubTrigger' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 222–236 | function | `DropdownMenuSubContent` | 渲染并协调 'DropdownMenuSubContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/form.tsx`

依赖：`react`、`react-hook-form`、`@radix-ui/react-label`、`@radix-ui/react-slot`、`@/lib/utils`、`@/components/ui/label`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 16–16 | const | `Form` | 保存 'Form' 的模块级常量、配置、路由或预计算值。 |  |
| 18–23 | type | `FormFieldContextValue` | 定义 'FormFieldContextValue' 的数据契约、联合类型或组件属性。 |  |
| 25–27 | const | `FormFieldContext` | 保存 'FormFieldContext' 的模块级常量、配置、路由或预计算值。 |  |
| 29–40 | function | `FormField` | 渲染并协调 'FormField' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 42–63 | function | `useFormField` | 封装 'useFormField' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'React.useContext'、'getFieldState'、'useFormContext'、'useFormState' |
| 65–67 | type | `FormItemContextValue` | 定义 'FormItemContextValue' 的数据契约、联合类型或组件属性。 |  |
| 69–71 | const | `FormItemContext` | 保存 'FormItemContext' 的模块级常量、配置、路由或预计算值。 |  |
| 73–85 | function | `FormItem` | 渲染并协调 'FormItem' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'React.useId'、'cn' |
| 87–102 | function | `FormLabel` | 渲染并协调 'FormLabel' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn'、'useFormField' |
| 104–120 | function | `FormControl` | 渲染并协调 'FormControl' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'useFormField' |
| 122–133 | function | `FormDescription` | 渲染并协调 'FormDescription' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn'、'useFormField' |
| 135–153 | function | `FormMessage` | 渲染并协调 'FormMessage' React 组件的状态、数据请求和用户交互。 | 分支 2；循环 0；返回 2；await 0；调用 'String'、'cn'、'useFormField' |

## `components/ui/input-otp.tsx`

依赖：`react`、`input-otp`、`lucide-react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–24 | function | `InputOTP` | 渲染并协调 'InputOTP' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 26–34 | function | `InputOTPGroup` | 渲染并协调 'InputOTPGroup' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 36–64 | function | `InputOTPSlot` | 渲染并协调 'InputOTPSlot' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'React.useContext'、'cn' |
| 66–72 | function | `InputOTPSeparator` | 渲染并协调 'InputOTPSeparator' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |

## `components/ui/input.tsx`

依赖：`react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–18 | function | `Input` | 渲染并协调 'Input' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/label.tsx`

依赖：`react`、`@radix-ui/react-label`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 7–21 | function | `Label` | 渲染并协调 'Label' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/popover.tsx`

依赖：`react`、`@radix-ui/react-popover`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–9 | function | `Popover` | 渲染并协调 'Popover' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 11–15 | function | `PopoverTrigger` | 渲染并协调 'PopoverTrigger' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 17–37 | function | `PopoverContent` | 渲染并协调 'PopoverContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 39–43 | function | `PopoverAnchor` | 渲染并协调 'PopoverAnchor' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |

## `components/ui/progress.tsx`

依赖：`react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–7 | interface | `ProgressProps` | 定义 'ProgressProps' 的数据契约、联合类型或组件属性。 |  |
| 9–34 | const | `Progress` | 保存 'Progress' 的模块级常量、配置、路由或预计算值。 |  |
| 10–33 | function | `React.forwardRef.callback#1` | 渲染并协调 'React.forwardRef.callback#1' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'Math.max'、'Math.min'、'cn' |

## `components/ui/radio-group.tsx`

依赖：`react`、`@radix-ui/react-radio-group`、`lucide-react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–17 | function | `RadioGroup` | 渲染并协调 'RadioGroup' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 19–40 | function | `RadioGroupItem` | 渲染并协调 'RadioGroupItem' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/scroll-area.tsx`

依赖：`react`、`@radix-ui/react-scroll-area`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–8 | interface | `ScrollAreaProps` | 定义 'ScrollAreaProps' 的数据契约、联合类型或组件属性。 |  |
| 10–35 | function | `ScrollArea` | 渲染并协调 'ScrollArea' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 37–62 | function | `ScrollBar` | 渲染并协调 'ScrollBar' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/select.tsx`

依赖：`react`、`@radix-ui/react-select`、`lucide-react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–10 | function | `Select` | 渲染并协调 'Select' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 12–16 | function | `SelectGroup` | 渲染并协调 'SelectGroup' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 18–22 | function | `SelectValue` | 渲染并协调 'SelectValue' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 24–48 | function | `SelectTrigger` | 渲染并协调 'SelectTrigger' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 50–83 | function | `SelectContent` | 渲染并协调 'SelectContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 85–96 | function | `SelectLabel` | 渲染并协调 'SelectLabel' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 98–120 | function | `SelectItem` | 渲染并协调 'SelectItem' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 122–133 | function | `SelectSeparator` | 渲染并协调 'SelectSeparator' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 135–151 | function | `SelectScrollUpButton` | 渲染并协调 'SelectScrollUpButton' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 153–169 | function | `SelectScrollDownButton` | 渲染并协调 'SelectScrollDownButton' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/separator.tsx`

依赖：`react`、`@radix-ui/react-separator`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–23 | function | `Separator` | 渲染并协调 'Separator' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/shadcn-io/kanban/index.tsx`

依赖：`@dnd-kit/core`、`@dnd-kit/core`、`@dnd-kit/sortable`、`@dnd-kit/utilities`、`react`、`react-dom`、`tunnel-rat`、`@/components/ui/card`、`@/components/ui/scroll-area`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 36–36 | const | `t` | 保存 't' 的模块级常量、配置、路由或预计算值。 |  |
| 40–44 | type | `KanbanItemProps` | 定义 'KanbanItemProps' 的数据契约、联合类型或组件属性。 |  |
| 46–49 | type | `KanbanColumnProps` | 定义 'KanbanColumnProps' 的数据契约、联合类型或组件属性。 |  |
| 51–58 | type | `KanbanContextProps` | 定义 'KanbanContextProps' 的数据契约、联合类型或组件属性。 |  |
| 60–64 | const | `KanbanContext` | 保存 'KanbanContext' 的模块级常量、配置、路由或预计算值。 |  |
| 66–70 | type | `KanbanBoardProps` | 定义 'KanbanBoardProps' 的数据契约、联合类型或组件属性。 |  |
| 72–89 | function | `KanbanBoard` | 渲染并协调 'KanbanBoard' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'cn'、'useDroppable' |
| 91–94 | type | `KanbanCardProps` | 定义 'KanbanCardProps' 的数据契约、联合类型或组件属性。 |  |
| 96–147 | function | `KanbanCard` | 渲染并协调 'KanbanCard' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'CSS.Transform.toString'、'cn'、'useContext'、'useSortable' |
| 149–153 | type | `KanbanCardsProps` | 定义 'KanbanCardsProps' 的数据契约、联合类型或组件属性。 |  |
| 155–177 | function | `KanbanCards` | 渲染并协调 'KanbanCards' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn'、'data.filter'、'filteredData.map'、'useContext' |
| 161–161 | function | `KanbanCards > data.filter.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 162–162 | function | `KanbanCards > filteredData.map.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 179–179 | type | `KanbanHeaderProps` | 定义 'KanbanHeaderProps' 的数据契约、联合类型或组件属性。 |  |
| 181–183 | function | `KanbanHeader` | 渲染并协调 'KanbanHeader' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 0；await 0；调用 'cn' |
| 185–197 | type | `KanbanProviderProps` | 定义 'KanbanProviderProps' 的数据契约、联合类型或组件属性。 |  |
| 199–338 | function | `KanbanProvider` | 渲染并协调 'KanbanProvider' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn'、'columns.map'、'createPortal'、'useSensor'、'useSensors'、'useState' |
| 221–227 | function | `KanbanProvider > handleDragStart` | 处理与 'handleDragStart' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'data.find'、'onDragStart'、'setActiveCardId' |
| 222–222 | function | `KanbanProvider > handleDragStart > data.find.callback#9` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 229–261 | function | `KanbanProvider > handleDragOver` | 处理与 'handleDragOver' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 2；await 0；调用 'arrayMove'、'columns.find'、'data.find'、'newData.findIndex'、'onDataChange'、'onDragOver' |
| 236–236 | function | `KanbanProvider > handleDragOver > data.find.callback#11` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 237–237 | function | `KanbanProvider > handleDragOver > data.find.callback#12` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 246–246 | function | `KanbanProvider > handleDragOver > columns.find.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 251–251 | function | `KanbanProvider > handleDragOver > newData.findIndex.callback#14` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 252–252 | function | `KanbanProvider > handleDragOver > newData.findIndex.callback#15` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 263–282 | function | `KanbanProvider > handleDragEnd` | 处理与 'handleDragEnd' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'arrayMove'、'newData.findIndex'、'onDataChange'、'onDragEnd'、'setActiveCardId' |
| 276–276 | function | `KanbanProvider > handleDragEnd > newData.findIndex.callback#17` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 277–277 | function | `KanbanProvider > handleDragEnd > newData.findIndex.callback#18` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 285–289 | function | `KanbanProvider > onDragStart` | 执行与 'onDragStart' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'data.find' |
| 286–286 | function | `KanbanProvider > onDragStart > data.find.callback#20` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 290–295 | function | `KanbanProvider > onDragOver` | 执行与 'onDragOver' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'columns.find'、'data.find' |
| 291–291 | function | `KanbanProvider > onDragOver > data.find.callback#22` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 292–292 | function | `KanbanProvider > onDragOver > columns.find.callback#23` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 296–301 | function | `KanbanProvider > onDragEnd` | 执行与 'onDragEnd' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'columns.find'、'data.find' |
| 297–297 | function | `KanbanProvider > onDragEnd > data.find.callback#25` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 298–298 | function | `KanbanProvider > onDragEnd > columns.find.callback#26` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 302–306 | function | `KanbanProvider > onDragCancel` | 执行与 'onDragCancel' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'data.find' |
| 303–303 | function | `KanbanProvider > onDragCancel > data.find.callback#28` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 326–326 | function | `KanbanProvider > columns.map.callback#29` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'children' |

## `components/ui/sheet.tsx`

依赖：`react`、`@radix-ui/react-dialog`、`lucide-react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 6–8 | function | `Sheet` | 渲染并协调 'Sheet' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 10–14 | function | `SheetTrigger` | 渲染并协调 'SheetTrigger' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 16–20 | function | `SheetClose` | 渲染并协调 'SheetClose' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 22–26 | function | `SheetPortal` | 渲染并协调 'SheetPortal' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 28–42 | function | `SheetOverlay` | 渲染并协调 'SheetOverlay' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 44–79 | function | `SheetContent` | 渲染并协调 'SheetContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 81–89 | function | `SheetHeader` | 渲染并协调 'SheetHeader' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 91–99 | function | `SheetFooter` | 渲染并协调 'SheetFooter' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 101–112 | function | `SheetTitle` | 渲染并协调 'SheetTitle' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 114–125 | function | `SheetDescription` | 渲染并协调 'SheetDescription' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/sidebar.tsx`

依赖：`react`、`@radix-ui/react-slot`、`class-variance-authority`、`lucide-react`、`@/lib/utils`、`@/hooks/use-mobile`、`@/components/ui/button`、`@/components/ui/input`、`@/components/ui/separator`、`@/components/ui/sheet`、`@/components/ui/skeleton`、`@/components/ui/tooltip`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 25–25 | const | `SIDEBAR_COOKIE_NAME` | 保存 'SIDEBAR_COOKIE_NAME' 的模块级常量、配置、路由或预计算值。 |  |
| 26–26 | const | `SIDEBAR_COOKIE_MAX_AGE` | 保存 'SIDEBAR_COOKIE_MAX_AGE' 的模块级常量、配置、路由或预计算值。 |  |
| 27–27 | const | `SIDEBAR_WIDTH` | 保存 'SIDEBAR_WIDTH' 的模块级常量、配置、路由或预计算值。 |  |
| 28–28 | const | `SIDEBAR_WIDTH_MOBILE` | 保存 'SIDEBAR_WIDTH_MOBILE' 的模块级常量、配置、路由或预计算值。 |  |
| 29–29 | const | `SIDEBAR_WIDTH_ICON` | 保存 'SIDEBAR_WIDTH_ICON' 的模块级常量、配置、路由或预计算值。 |  |
| 30–30 | const | `SIDEBAR_KEYBOARD_SHORTCUT` | 保存 'SIDEBAR_KEYBOARD_SHORTCUT' 的模块级常量、配置、路由或预计算值。 |  |
| 32–40 | type | `SidebarContextProps` | 定义 'SidebarContextProps' 的数据契约、联合类型或组件属性。 |  |
| 42–42 | const | `SidebarContext` | 保存 'SidebarContext' 的模块级常量、配置、路由或预计算值。 |  |
| 44–51 | function | `useSidebar` | 封装 'useSidebar' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'React.useContext' |
| 53–149 | function | `SidebarProvider` | 渲染并协调 'SidebarProvider' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'React.useCallback'、'React.useEffect'、'React.useMemo'、'React.useState'、'cn'、'useIsMobile' |
| 74–84 | function | `SidebarProvider > React.useCallback.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 2；循环 0；返回 0；await 0；调用 '_setOpen'、'setOpenProp'、'value' |
| 89–91 | function | `SidebarProvider > React.useCallback.callback#4` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'setOpen'、'setOpenMobile' |
| 90–90 | function | `SidebarProvider > React.useCallback.callback#4 > setOpen.callback#6` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 90–90 | function | `SidebarProvider > React.useCallback.callback#4 > setOpenMobile.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 94–107 | function | `SidebarProvider > React.useEffect.callback#7` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'window.addEventListener' |
| 95–103 | function | `SidebarProvider > React.useEffect.callback#7 > handleKeyDown` | 处理与 'handleKeyDown' 对应的前端业务、状态或数据转换逻辑。 | 分支 1；循环 0；返回 0；await 0；调用 'event.preventDefault'、'toggleSidebar' |
| 106–106 | function | `SidebarProvider > React.useEffect.callback#7 > <anonymous#9>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'window.removeEventListener' |
| 114–122 | function | `SidebarProvider > React.useMemo.callback#10` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0 |
| 151–251 | function | `Sidebar` | 渲染并协调 'Sidebar' React 组件的状态、数据请求和用户交互。 | 分支 6；循环 0；返回 3；await 0；调用 'cn'、'useSidebar' |
| 253–277 | function | `SidebarTrigger` | 渲染并协调 'SidebarTrigger' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn'、'useSidebar' |
| 267–270 | function | `SidebarTrigger > onClick.callback#13` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'onClick'、'toggleSidebar' |
| 279–307 | function | `SidebarRail` | 渲染并协调 'SidebarRail' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn'、'useSidebar' |
| 309–321 | function | `SidebarInset` | 渲染并协调 'SidebarInset' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 323–335 | function | `SidebarInput` | 渲染并协调 'SidebarInput' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 337–346 | function | `SidebarHeader` | 渲染并协调 'SidebarHeader' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 348–357 | function | `SidebarFooter` | 渲染并协调 'SidebarFooter' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 359–371 | function | `SidebarSeparator` | 渲染并协调 'SidebarSeparator' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 373–385 | function | `SidebarContent` | 渲染并协调 'SidebarContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 387–396 | function | `SidebarGroup` | 渲染并协调 'SidebarGroup' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 398–417 | function | `SidebarGroupLabel` | 渲染并协调 'SidebarGroupLabel' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'cn' |
| 419–440 | function | `SidebarGroupAction` | 渲染并协调 'SidebarGroupAction' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'cn' |
| 442–454 | function | `SidebarGroupContent` | 渲染并协调 'SidebarGroupContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 456–465 | function | `SidebarMenu` | 渲染并协调 'SidebarMenu' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 467–476 | function | `SidebarMenuItem` | 渲染并协调 'SidebarMenuItem' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 478–498 | const | `sidebarMenuButtonVariants` | 保存 'sidebarMenuButtonVariants' 的模块级常量、配置、路由或预计算值。 |  |
| 500–548 | function | `SidebarMenuButton` | 渲染并协调 'SidebarMenuButton' React 组件的状态、数据请求和用户交互。 | 分支 3；循环 0；返回 2；await 0；调用 'cn'、'sidebarMenuButtonVariants'、'useSidebar' |
| 550–580 | function | `SidebarMenuAction` | 渲染并协调 'SidebarMenuAction' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'cn' |
| 582–602 | function | `SidebarMenuBadge` | 渲染并协调 'SidebarMenuBadge' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 604–640 | function | `SidebarMenuSkeleton` | 渲染并协调 'SidebarMenuSkeleton' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'React.useMemo'、'cn' |
| 612–614 | function | `SidebarMenuSkeleton > React.useMemo.callback#31` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'Math.floor'、'Math.random' |
| 642–655 | function | `SidebarMenuSub` | 渲染并协调 'SidebarMenuSub' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 657–669 | function | `SidebarMenuSubItem` | 渲染并协调 'SidebarMenuSubItem' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 671–701 | function | `SidebarMenuSubButton` | 渲染并协调 'SidebarMenuSubButton' React 组件的状态、数据请求和用户交互。 | 分支 1；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/skeleton.tsx`

依赖：`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 3–11 | function | `Skeleton` | 渲染并协调 'Skeleton' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/sonner.tsx`

依赖：`sonner`、`@/context/theme-provider`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–24 | function | `Toaster` | 渲染并协调 'Toaster' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'useTheme' |

## `components/ui/switch.tsx`

依赖：`react`、`@radix-ui/react-switch`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–26 | function | `Switch` | 渲染并协调 'Switch' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/table.tsx`

依赖：`react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–17 | function | `Table` | 渲染并协调 'Table' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 19–27 | function | `TableHeader` | 渲染并协调 'TableHeader' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 29–37 | function | `TableBody` | 渲染并协调 'TableBody' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 39–50 | function | `TableFooter` | 渲染并协调 'TableFooter' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 52–63 | function | `TableRow` | 渲染并协调 'TableRow' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 65–76 | function | `TableHead` | 渲染并协调 'TableHead' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 78–89 | function | `TableCell` | 渲染并协调 'TableCell' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 91–102 | function | `TableCaption` | 渲染并协调 'TableCaption' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/tabs.tsx`

依赖：`react`、`@radix-ui/react-tabs`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–16 | function | `Tabs` | 渲染并协调 'Tabs' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 18–32 | function | `TabsList` | 渲染并协调 'TabsList' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 34–48 | function | `TabsTrigger` | 渲染并协调 'TabsTrigger' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |
| 50–61 | function | `TabsContent` | 渲染并协调 'TabsContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/textarea.tsx`

依赖：`react`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–15 | function | `Textarea` | 渲染并协调 'Textarea' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

## `components/ui/tooltip.tsx`

依赖：`react`、`@radix-ui/react-tooltip`、`@/lib/utils`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 7–18 | function | `TooltipProvider` | 渲染并协调 'TooltipProvider' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 20–28 | function | `Tooltip` | 渲染并协调 'Tooltip' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 30–34 | function | `TooltipTrigger` | 渲染并协调 'TooltipTrigger' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0 |
| 36–58 | function | `TooltipContent` | 渲染并协调 'TooltipContent' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'cn' |

