# WP02-C2 Web 恢复码闭环实现

本文记录 WP02-C2 Web 端实现与集成边界。最初的页面候选以公开提交 `d200c033b81ebabfe0c99c50572cc46186ba5329` 为基线；集成提交 `e8b821e7e0bee75fe5974b6786925da23da974f4` 已合入后端和 OpenAPI。本轮 follow-up 重新生成 SDK，并把首次初始化回显、账户安全页状态读取和整组再生成绑定到同一份生成合同。所有 Node 检查都在指定 VPS 执行，本机没有运行编译或测试，也没有执行 production build。Web 门通过不等同于整包运行时或正式制品已经 `verified`。

## 1. 接口边界

当前 OpenAPI 和生成 SDK 已包含 bootstrap 一次性恢复码、恢复码状态与再生成端点。`apps/web/src/api/recovery-codes.ts` 直接引用生成的 request body、成功响应 data、response meta 和 literal path 类型，不再维护重复的临时合同。现有 generated client 仍会先执行无上限 `response.text()`，且 fetch 默认允许跟随重定向，所以不能直接用生成函数替掉这里的有界流式读取和 `redirect: error`。生成 transport 具备同等安全保证后，才可删除这层包装。

| 请求 | 成功合同 | Web 端约束 |
|---|---|---|
| `POST /api/v1/bootstrap` | 201；`data.instance_id`、`data.owner_id`、`data.one_time_recovery_codes: string[8]` | setup token 只进 header；明文响应必须带 `Cache-Control: no-store` |
| `GET /api/v1/me/recovery-codes` | 200；`data.set_version/total_count/remaining_count/created_at_ms` | 只读元数据，不接收明文，不需要 CSRF |
| `POST /api/v1/me/recovery-codes` | 仅 200；`data.set_version/created_at_ms/one_time_recovery_codes: string[8]` | 需要 CSRF 和近期认证；明文响应必须带 `Cache-Control: no-store`；旧组由服务端原子失效 |

adapter 使用同源 Cookie、`cache: no-store`、`redirect: error` 和精确 envelope 校验。响应 body 通过 `ReadableStream` 逐块累计；一旦超过 64 KiB，reader 立即 cancel，不再先把无界响应读入字符串。恢复码必须恰好八枚，每枚都是八组四位十六进制、由七个连字符分隔的 39 字符串；唯一性比较会去掉分隔符并忽略大小写。秘密响应缺少 `no-store`、字段多缺、类型错误、格式不规范或存在重复码时，调用方只能得到“响应不可安全接受”，不会渲染其中的任何值。

错误只在媒体类型为 `application/problem+json`、body 是有界合法 JSON 时进入 Problem 分支。GET/POST 的 401 表示当前 session 无效；再生成的 403 `RECENT_AUTH_REQUIRED` 触发一次显式 step-up，其他已知 4xx 只显示本地固定说明；网络错误、5xx 和不可信成功载荷属于结果未知。Web 端不读取或展示服务端 `title/detail`。

## 2. 首次初始化

`SetupPage.vue` 在初始化请求成功后把八枚恢复码复制到组件局部内存，随即覆写 adapter 响应中的原数组；mutation 返回值保持为空，避免 Vue Query cache 留下明文。页面同时清空 setup token、Owner 密码和确认密码，再重新读取 bootstrap 状态。数据库状态确认前表单保持锁定；用户可以先保存恢复码，但“已保存，继续”按钮仍不可用。状态确认成功后，用户必须主动勾选保存确认并继续，页面才会先覆写并清空数组，再跳转登录页。

只有媒体类型、Problem 必填字段、HTTP status 与白名单 code 全部匹配，并且能够证明请求未提交的 400/403/409/413/415/422/429，才允许用户修正后再次提交。`ALREADY_INITIALIZED` 只进入状态核对，不开放重放。空 body、畸形 JSON、未知或错配 Problem、5xx、意外 2xx，以及 201 未通过秘密校验，都按结果未知处理：页面锁定表单并重新读取 Master 状态，绝不自动重放 bootstrap。若数据库已经初始化但恢复码响应不可用，页面只提示用户登录后重新生成，不展示服务端无效载荷。

`OneTimeRecoveryCodes.vue` 是 bootstrap 和再生成共用的展示组件。下载只由用户点击触发：组件现场构造文本 `Blob`，点击临时隐藏链接，随后在 microtask 中撤销 object URL。下载失败只显示固定本地说明，不把恢复码送入 toast 或异常文本。

## 3. 账户安全页

`ProfileSecurityPage.vue` 新增恢复码卡片。GET 状态只展示剩余数、总数、组版本和创建时间。重新生成前先检查当前 session 的近期认证期限，再显示“旧组立即失效”的确认框；本地检查过期或服务端返回 `RECENT_AUTH_REQUIRED` 时，只跳转 `/reauth?redirect=/profile/security` 一次，不自动重放 POST。

再生成响应先停留在 session action 的局部数组。只有 credential coordinator 已把 terminal revision 成功写入 localStorage、广播并完成 `settle(reconcile)`，action 才把明文返回页面；terminal 写入、广播或 CAS 失败时，两份临时数组都先覆写，页面只收到 `outcome-unknown`。随后明文才可进入 `one-time-recovery` Pinia store。该 store 没有持久化插件，仅用于 credential mutation 暂时关闭受保护 DOM 时跨组件重挂载交接结果。用户确认保存后，store 先覆写并清空明文，再关闭持久对话框。任何 credential journal 损坏、reset、新的 inflight 或 quarantine 都会立即清空这份内存。

一次性 store 还持有每次再生成的不可序列化 ownership token，但 token 不含恢复码或身份材料。组件 unmount 会立即覆写明文；如果 unmount 只是 credential gate 在同一路由关闭并重新打开受保护 DOM，token 可让已通过 terminal settle 的旧 action 把结果交给新组件。接收时还必须保持原始 router location 对象身份，不以路径字符串代替页面访问身份。真正的 `onBeforeRouteLeave`、route 变化或 `pagehide`（包括组件暂时卸载期间进入 BFCache）会同时撤销 token 和覆写 store，因此“离开后又返回同一路径”的旧结果也只能清空返回数组，不能靠 `route.fullPath` 相等重新取得写入权。

再生成属于 credential coordinator 的 exclusive operation：

1. 发送请求前写入 `inflight/quarantine` journal；
2. 已知 4xx 恢复原认证投影，并以 `settled/reconcile` 收口；
3. 401 按现有会话失效合同关闭认证投影；
4. 网络错误、5xx 或不可信成功响应按结果未知处理，只尝试一次 fail-safe logout，绝不重放生成请求；
5. 只有权威 logout 204 可以解除 quarantine，否则保持 `relogin-required`。

协调 journal 只记录操作名 `regenerate-recovery-codes` 和非秘密 revision 元数据。恢复码不会进入 localStorage、sessionStorage、BroadcastChannel、session Pinia 状态、日志或 toast。

## 4. 文件职责

| 文件 | 职责 |
|---|---|
| `apps/web/src/api/generated/*` | 从当前 OpenAPI 生成的恢复码 response、operation 和 literal path 合同；禁止手改 |
| `apps/web/src/api/recovery-codes.ts` | 复用生成类型与路径，并保留有界流式读取、拒绝重定向和秘密响应运行时校验 |
| `apps/web/src/components/security/OneTimeRecoveryCodes.vue` | 一次性明文展示、显式下载和保存确认 |
| `apps/web/src/stores/one-time-recovery.ts` | 仅内存的再生成结果交接和隔离事件清理 |
| `apps/web/src/lib/credential-coordinator.ts` | 注册恢复码再生 exclusive operation；不改变 journal schema |
| `apps/web/src/stores/session.ts` | 状态 GET、再生成 mutation、CSRF、generation/cursor、401 与结果未知处理 |
| `apps/web/src/views/SetupPage.vue` | bootstrap 回显、状态确认、降级与离页清理 |
| `apps/web/src/views/ProfileSecurityPage.vue` | 恢复码元数据和 recent-auth 再生成交互 |

## 5. 测试源码与 Web 验收

新增或扩展的测试源码覆盖：

- adapter 对 `no-store`、八枚唯一恢复码、精确数据结构和 GET/POST header 的校验；
- canonical 39 字符格式矩阵、错误媒体类型、空/畸形错误 body，以及超限 stream 的即时 cancel；
- 下载只能由显式点击触发，object URL 会撤销，确认门不会提前开放；
- setup 清空提交凭据、只显示一次、reconcile 前不可离页，以及空/畸形 5xx、未知 4xx 和意外 2xx 均锁定且不重放；
- profile 只显示元数据、近期认证前置、服务端 recent-auth 边界只调用一次，并覆盖 unmount、离开后返回同一路径和 pagehide/BFCache 的明文清理；
- session store 成功路径不把明文写入 Pinia 或浏览器存储，stale success/401/403 统一进入结果未知，terminal settle 失败不把明文交给调用者。

集成 follow-up 通过 `git archive` 把唯一候选送到 `185.99.135.224`，在固定 Node 24.19.0、pnpm 11.24.0 builder 内 fresh install。生成器对 16 个物理文件实现逐字节零漂移，typecheck、`eslint --max-warnings=0` 和 12 个文件的 111 项 Vitest 全部通过；OpenAPI validator、设计文档 validator 和上游公开内容 sanitizer 同时通过。任务明确禁止 production build，本轮没有执行。整包后端 runtime smoke、正式生产制品和公开 Actions 仍需按总验证流程独立闭环。
