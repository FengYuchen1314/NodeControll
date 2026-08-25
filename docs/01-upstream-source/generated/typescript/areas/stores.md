# TypeScript 分区 `stores`

Zustand 全局状态，主要承载认证会话。

## `stores/auth-store.ts`

依赖：`zustand`、`@/lib/cookies`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–4 | const | `TOKEN_COOKIE` | 保存 'TOKEN_COOKIE' 的模块级常量、配置、路由或预计算值。 |  |
| 6–12 | interface | `AuthState` | 定义 'AuthState' 的数据契约、联合类型或组件属性。 |  |
| 14–40 | function | `create.callback#1` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'JSON.parse'、'getCookie' |
| 14–40 | const | `useAuthStore` | 保存 'useAuthStore' 的模块级常量、配置、路由或预计算值。 |  |
| 21–29 | function | `create.callback#1 > setAccessToken` | 设置与 'setAccessToken' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'set' |
| 22–29 | function | `create.callback#1 > setAccessToken > set.callback#3` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 1；循环 0；返回 1；await 0；调用 'JSON.stringify'、'removeCookie'、'setCookie' |
| 30–37 | function | `create.callback#1 > reset` | 重置与 'reset' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0；调用 'set' |
| 31–37 | function | `create.callback#1 > reset > set.callback#5` | 供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。 | 分支 0；循环 0；返回 1；await 0；调用 'removeCookie' |

