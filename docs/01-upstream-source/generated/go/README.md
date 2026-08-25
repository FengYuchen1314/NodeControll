# Go 源码符号总览

> 自动生成索引；公开版只保留符号、行号、原创作用说明、调用和控制流证据，不公开源码签名或常量字面量。

- 文件：120
- 包：18
- 具名函数/方法：1032
- 匿名闭包：113
- 类型：190
- 常量/变量：138

| 包 | 文件数 | 模块作用 | 详细索引 |
|---|---:|---|---|
| `auth` | 4 | 用户认证、密码、会话令牌、角色授权和两步验证上下文。 | [auth](packages/auth.md) |
| `captcha` | 1 | Cloudflare Turnstile 配置读取与服务端验证码校验。 | [captcha](packages/captcha.md) |
| `handler` | 83 | HTTP/WebSocket/SSE 适配层以及多数业务编排逻辑。 | [handler](packages/handler.md) |
| `logger` | 2 | 结构化日志、日志文件轮转和历史日志清理。 | [logger](packages/logger.md) |
| `main` | 2 | 组合全部基础设施与 HTTP 端点，启动和优雅停止单体服务。 | [main](packages/main.md) |
| `notify` | 3 | Telegram 等外部通知的格式化、发送和开关控制。 | [notify](packages/notify.md) |
| `patches` | 1 | 对历史配置文件做幂等、精确匹配的数据修补。 | [patches](packages/patches.md) |
| `proxygroups` | 4 | 代理组远程配置的拉取、验证、内存缓存和查询。 | [proxygroups](packages/proxygroups.md) |
| `ruletemplates` | 1 | 随二进制嵌入并落盘默认规则模板。 | [ruletemplates](packages/ruletemplates.md) |
| `scriptengine` | 2 | 基于 goja 的 JavaScript 覆写脚本执行沙箱与对象转换。 | [scriptengine](packages/scriptengine.md) |
| `speedtest` | 2 | Mihomo/远程测试器驱动的节点测速模型和执行能力。 | [speedtest](packages/speedtest.md) |
| `storage` | 9 | SQLite 建表、迁移、Repository 方法和持久化数据模型。 | [storage](packages/storage.md) |
| `subscribes` | 1 | 随二进制嵌入并准备默认订阅配置文件。 | [subscribes](packages/subscribes.md) |
| `taskrun` | 1 | 后台任务运行记录、状态和可观测性封装。 | [taskrun](packages/taskrun.md) |
| `util` | 1 | 跨模块复用的网络、时间、字符串和文件工具。 | [util](packages/util.md) |
| `validator` | 1 | 配置、节点和请求数据的语义校验。 | [validator](packages/validator.md) |
| `version` | 1 | 构建版本、更新通道和版本比较。 | [version](packages/version.md) |
| `web` | 1 | 嵌入式前端静态资源和 SPA fallback HTTP Handler。 | [web](packages/web.md) |
