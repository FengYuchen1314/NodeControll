# TypeScript 分区 `root`

应用入口、生成路由树和顶层类型。

## `main.tsx`

依赖：`react`、`react-dom/client`、`axios`、`@tanstack/react-query`、`@tanstack/react-router`、`sonner`、`@/stores/auth-store`、`@/lib/handle-server-error`、`./context/direction-provider`、`./context/font-provider`、`./context/theme-provider`、`./routeTree.gen`、`./styles/index.css`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 21–69 | const | `queryClient` | 保存 'queryClient' 的模块级常量、配置、路由或预计算值。 |  |
| 24–35 | function | `retry` | 执行与 'retry' 对应的前端业务、状态或数据转换逻辑。 | 分支 3；循环 0；返回 3；await 0；调用 '<ArrayLiteralExpression>.includes'、'console.log' |
| 40–48 | function | `onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 2；循环 0；返回 0；await 0；调用 'handleServerError'、'toast.error' |
| 52–67 | function | `onError` | 执行与 'onError' 对应的前端业务、状态或数据转换逻辑。 | 分支 4；循环 0；返回 0；await 0；调用 'router.navigate'、'toast.error'、'useAuthStore.getState'、'useAuthStore.getState.auth.reset' |
| 72–77 | const | `router` | 保存 'router' 的模块级常量、配置、路由或预计算值。 |  |
| 87–87 | const | `rootElement` | 保存 'rootElement' 的模块级常量、配置、路由或预计算值。 |  |

## `routeTree.gen.ts`

> 此文件由工具生成；仍纳入符号清单，但重构时不手工移植。

依赖：`./routes/__root`、`./routes/users`、`./routes/templates`、`./routes/system-settings`、`./routes/subscription`、`./routes/subscribe-files`、`./routes/settings`、`./routes/rules`、`./routes/probe`、`./routes/nodes`、`./routes/logs`、`./routes/login`、`./routes/generator`、`./routes/custom-rules`、`./routes/change-password`、`./routes/404`、`./routes/index`、`./routes/templates.index`、`./routes/templates-v3.index`、`./routes/subscription.index`、`./routes/subscribe-files.index`、`./routes/nodes.index`、`./routes/custom-rules.index`、`./routes/subscribe-files.custom`。

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 36–40 | const | `UsersRoute` | 保存 'UsersRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 39–39 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 41–45 | const | `TemplatesRoute` | 保存 'TemplatesRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 44–44 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 46–50 | const | `SystemSettingsRoute` | 保存 'SystemSettingsRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 49–49 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 51–55 | const | `SubscriptionRoute` | 保存 'SubscriptionRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 54–54 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 56–60 | const | `SubscribeFilesRoute` | 保存 'SubscribeFilesRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 59–59 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 61–65 | const | `SettingsRoute` | 保存 'SettingsRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 64–64 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 66–70 | const | `RulesRoute` | 保存 'RulesRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 69–69 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 71–75 | const | `ProbeRoute` | 保存 'ProbeRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 74–74 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 76–80 | const | `NodesRoute` | 保存 'NodesRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 79–79 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 81–85 | const | `LogsRoute` | 保存 'LogsRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 84–84 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 86–90 | const | `LoginRoute` | 保存 'LoginRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 89–89 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 91–95 | const | `GeneratorRoute` | 保存 'GeneratorRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 94–94 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 96–100 | const | `CustomRulesRoute` | 保存 'CustomRulesRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 99–99 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 101–105 | const | `ChangePasswordRoute` | 保存 'ChangePasswordRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 104–104 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 106–110 | const | `R404Route` | 保存 'R404Route' 的模块级常量、配置、路由或预计算值。 |  |
| 109–109 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 111–115 | const | `IndexRoute` | 保存 'IndexRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 114–114 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 116–120 | const | `TemplatesIndexRoute` | 保存 'TemplatesIndexRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 119–119 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 121–125 | const | `TemplatesV3IndexRoute` | 保存 'TemplatesV3IndexRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 124–124 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 126–130 | const | `SubscriptionIndexRoute` | 保存 'SubscriptionIndexRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 129–129 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 131–135 | const | `SubscribeFilesIndexRoute` | 保存 'SubscribeFilesIndexRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 134–134 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 136–140 | const | `NodesIndexRoute` | 保存 'NodesIndexRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 139–139 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 141–145 | const | `CustomRulesIndexRoute` | 保存 'CustomRulesIndexRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 144–144 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 146–150 | const | `SubscribeFilesCustomRoute` | 保存 'SubscribeFilesCustomRoute' 的模块级常量、配置、路由或预计算值。 |  |
| 149–149 | function | `getParentRoute` | 读取或计算与 'getParentRoute' 对应的前端业务、状态或数据转换逻辑。 | 分支 0；循环 0；返回 0；await 0 |
| 152–176 | interface | `FileRoutesByFullPath` | 定义 'FileRoutesByFullPath' 的数据契约、联合类型或组件属性。 |  |
| 177–196 | interface | `FileRoutesByTo` | 定义 'FileRoutesByTo' 的数据契约、联合类型或组件属性。 |  |
| 197–222 | interface | `FileRoutesById` | 定义 'FileRoutesById' 的数据契约、联合类型或组件属性。 |  |
| 223–295 | interface | `FileRouteTypes` | 定义 'FileRouteTypes' 的数据契约、联合类型或组件属性。 |  |
| 296–314 | interface | `RootRouteChildren` | 定义 'RootRouteChildren' 的数据契约、联合类型或组件属性。 |  |
| 482–484 | interface | `CustomRulesRouteChildren` | 定义 'CustomRulesRouteChildren' 的数据契约、联合类型或组件属性。 |  |
| 486–488 | const | `CustomRulesRouteChildren` | 保存 'CustomRulesRouteChildren' 的模块级常量、配置、路由或预计算值。 |  |
| 490–492 | const | `CustomRulesRouteWithChildren` | 保存 'CustomRulesRouteWithChildren' 的模块级常量、配置、路由或预计算值。 |  |
| 494–496 | interface | `NodesRouteChildren` | 定义 'NodesRouteChildren' 的数据契约、联合类型或组件属性。 |  |
| 498–500 | const | `NodesRouteChildren` | 保存 'NodesRouteChildren' 的模块级常量、配置、路由或预计算值。 |  |
| 502–502 | const | `NodesRouteWithChildren` | 保存 'NodesRouteWithChildren' 的模块级常量、配置、路由或预计算值。 |  |
| 504–507 | interface | `SubscribeFilesRouteChildren` | 定义 'SubscribeFilesRouteChildren' 的数据契约、联合类型或组件属性。 |  |
| 509–512 | const | `SubscribeFilesRouteChildren` | 保存 'SubscribeFilesRouteChildren' 的模块级常量、配置、路由或预计算值。 |  |
| 514–516 | const | `SubscribeFilesRouteWithChildren` | 保存 'SubscribeFilesRouteWithChildren' 的模块级常量、配置、路由或预计算值。 |  |
| 518–520 | interface | `SubscriptionRouteChildren` | 定义 'SubscriptionRouteChildren' 的数据契约、联合类型或组件属性。 |  |
| 522–524 | const | `SubscriptionRouteChildren` | 保存 'SubscriptionRouteChildren' 的模块级常量、配置、路由或预计算值。 |  |
| 526–528 | const | `SubscriptionRouteWithChildren` | 保存 'SubscriptionRouteWithChildren' 的模块级常量、配置、路由或预计算值。 |  |
| 530–532 | interface | `TemplatesRouteChildren` | 定义 'TemplatesRouteChildren' 的数据契约、联合类型或组件属性。 |  |
| 534–536 | const | `TemplatesRouteChildren` | 保存 'TemplatesRouteChildren' 的模块级常量、配置、路由或预计算值。 |  |
| 538–540 | const | `TemplatesRouteWithChildren` | 保存 'TemplatesRouteWithChildren' 的模块级常量、配置、路由或预计算值。 |  |
| 542–560 | const | `rootRouteChildren` | 保存 'rootRouteChildren' 的模块级常量、配置、路由或预计算值。 |  |
| 561–563 | const | `routeTree` | 保存 'routeTree' 的模块级常量、配置、路由或预计算值。 |  |

## `vite-env.d.ts`

| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|

