# TypeScript 分区 `context`

主题、字体、方向等 React Context。

## `context/direction-provider.tsx`

依赖：`react`、`@radix-ui/react-direction`、`@/lib/cookies`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–5 | type | `Direction` | 定义 'Direction' 的数据契约、联合类型或组件属性。 |  |
| 7–7 | const | `DEFAULT_DIRECTION` | 保存 'DEFAULT_DIRECTION' 的模块级常量、配置、路由或预计算值。 |  |
| 8–8 | const | `DIRECTION_COOKIE_NAME` | 保存 'DIRECTION_COOKIE_NAME' 的模块级常量、配置、路由或预计算值。 |  |
| 9–9 | const | `DIRECTION_COOKIE_MAX_AGE` | 保存 'DIRECTION_COOKIE_MAX_AGE' 的模块级常量、配置、路由或预计算值。 |  |
| 11–16 | type | `DirectionContextType` | 定义 'DirectionContextType' 的数据契约、联合类型或组件属性。 |  |
| 18–18 | const | `DirectionContext` | 保存 'DirectionContext' 的模块级常量、配置、路由或预计算值。 |  |
| 20–52 | function | `DirectionProvider` | 渲染并协调 'DirectionProvider' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'useEffect'、'useState' |
| 22–22 | function | `DirectionProvider > useState.callback#2` | 封装 'useState.callback#2' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'getCookie' |
| 25–28 | function | `DirectionProvider > useEffect.callback#3` | 封装 'useEffect.callback#3' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'htmlElement.setAttribute' |
| 30–33 | function | `DirectionProvider > setDir` | 设置与 'setDir' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 '_setDir'、'setCookie' |
| 35–38 | function | `DirectionProvider > resetDir` | 重置与 'resetDir' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 '_setDir'、'removeCookie' |
| 55–61 | function | `useDirection` | 封装 'useDirection' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'useContext' |

## `context/font-provider.tsx`

依赖：`react`、`@/config/fonts`、`@/lib/cookies`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–5 | type | `Font` | 定义 'Font' 的数据契约、联合类型或组件属性。 |  |
| 7–7 | const | `FONT_COOKIE_NAME` | 保存 'FONT_COOKIE_NAME' 的模块级常量、配置、路由或预计算值。 |  |
| 8–8 | const | `FONT_COOKIE_MAX_AGE` | 保存 'FONT_COOKIE_MAX_AGE' 的模块级常量、配置、路由或预计算值。 |  |
| 10–14 | type | `FontContextType` | 定义 'FontContextType' 的数据契约、联合类型或组件属性。 |  |
| 16–16 | const | `FontContext` | 保存 'FontContext' 的模块级常量、配置、路由或预计算值。 |  |
| 18–49 | function | `FontProvider` | 渲染并协调 'FontProvider' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'useEffect'、'useState' |
| 19–22 | function | `FontProvider > useState.callback#2` | 封装 'useState.callback#2' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'fonts.includes'、'getCookie' |
| 24–34 | function | `FontProvider > useEffect.callback#3` | 封装 'useEffect.callback#3' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'applyFont' |
| 25–31 | function | `FontProvider > useEffect.callback#3 > applyFont` | 执行与 'applyFont' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'root.classList.add'、'root.classList.forEach' |
| 27–29 | function | `FontProvider > useEffect.callback#3 > applyFont > root.classList.forEach.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 0；await 0；调用 'cls.startsWith'、'root.classList.remove' |
| 36–39 | function | `FontProvider > setFont` | 设置与 'setFont' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 '_setFont'、'setCookie' |
| 41–44 | function | `FontProvider > resetFont` | 重置与 'resetFont' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 '_setFont'、'removeCookie' |
| 52–58 | function | `useFont` | 封装 'useFont' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'useContext' |

## `context/theme-provider.tsx`

依赖：`react`、`@/lib/cookies`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–4 | type | `Theme` | 定义 'Theme' 的数据契约、联合类型或组件属性。 |  |
| 5–5 | type | `ResolvedTheme` | 定义 'ResolvedTheme' 的数据契约、联合类型或组件属性。 |  |
| 7–7 | const | `DEFAULT_THEME` | 保存 'DEFAULT_THEME' 的模块级常量、配置、路由或预计算值。 |  |
| 8–8 | const | `THEME_COOKIE_NAME` | 保存 'THEME_COOKIE_NAME' 的模块级常量、配置、路由或预计算值。 |  |
| 9–9 | const | `THEME_COOKIE_MAX_AGE` | 保存 'THEME_COOKIE_MAX_AGE' 的模块级常量、配置、路由或预计算值。 |  |
| 11–15 | type | `ThemeProviderProps` | 定义 'ThemeProviderProps' 的数据契约、联合类型或组件属性。 |  |
| 17–23 | type | `ThemeProviderState` | 定义 'ThemeProviderState' 的数据契约、联合类型或组件属性。 |  |
| 25–31 | const | `initialState` | 保存 'initialState' 的模块级常量、配置、路由或预计算值。 |  |
| 29–29 | function | `setTheme` | 设置与 'setTheme' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 30–30 | function | `resetTheme` | 重置与 'resetTheme' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 33–33 | const | `ThemeContext` | 保存 'ThemeContext' 的模块级常量、配置、路由或预计算值。 |  |
| 35–101 | function | `ThemeProvider` | 渲染并协调 'ThemeProvider' React 组件的状态、数据请求和用户交互。 | 分支 0；循环 0；返回 1；await 0；调用 'useEffect'、'useMemo'、'useState' |
| 42–42 | function | `ThemeProvider > useState.callback#4` | 封装 'useState.callback#4' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'getCookie' |
| 46–53 | function | `ThemeProvider > useMemo.callback#5` | 封装 'useMemo.callback#5' Hook 的响应式状态、副作用和复用逻辑。 | 分支 2；循环 0；返回 2；await 0；调用 'window.matchMedia' |
| 55–76 | function | `ThemeProvider > useEffect.callback#6` | 封装 'useEffect.callback#6' Hook 的响应式状态、副作用和复用逻辑。 | 分支 0；循环 0；返回 1；await 0；调用 'applyTheme'、'mediaQuery.addEventListener'、'window.matchMedia' |
| 59–62 | function | `ThemeProvider > useEffect.callback#6 > applyTheme` | 执行与 'applyTheme' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'root.classList.add'、'root.classList.remove' |
| 64–69 | function | `ThemeProvider > useEffect.callback#6 > handleChange` | 处理与 'handleChange' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0；调用 'applyTheme' |
| 75–75 | function | `ThemeProvider > useEffect.callback#6 > <anonymous#9>` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 0；await 0；调用 'mediaQuery.removeEventListener' |
| 78–81 | function | `ThemeProvider > setTheme` | 设置与 'setTheme' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 '_setTheme'、'setCookie' |
| 83–86 | function | `ThemeProvider > resetTheme` | 重置与 'resetTheme' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 '_setTheme'、'removeCookie' |
| 104–110 | function | `useTheme` | 封装 'useTheme' Hook 的响应式状态、副作用和复用逻辑。 | 分支 1；循环 0；返回 1；await 0；调用 'useContext' |

