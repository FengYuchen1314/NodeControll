# 妙妙屋源码解剖

本目录记录 `iluobei/miaomiaowu@0b47f10c52aee10b9f759a593ca5f61a823cbb72` 的静态分析。最终文档会覆盖：

- 根目录、构建、发布、安装和嵌入资源；
- Go 后端的每个包、文件、类型、常量、变量、函数和方法；
- React/TypeScript 前端的每个路由、组件、Hook、Store、类型和工具函数；
- HTTP API、认证/授权、中间件、数据库表/迁移、后台任务、配置和文件格式；
- 关键业务数据流、错误处理、并发模型、安全边界和已知限制；
- 自动生成的逐函数索引与人工校对的模块说明。

## 人工校对文档

- [仓库、构建与交付](REPOSITORY.md)
- [后端源码说明](BACKEND.md)
- [前端源码说明](FRONTEND.md)
- [SQLite 数据模型](DATABASE.md)
- [HTTP API 说明](HTTP_API.md)
- [关键数据流](DATA_FLOWS.md)
- [HTTP 路由注册索引](generated/http-routes.md)
- [Go 逐符号索引](generated/go/README.md)
- [TypeScript/TSX 逐声明与逐函数索引](generated/typescript/README.md)

HTTP API 语义和端到端数据流仍在补充，进度以 `docs/00-project/PROGRESS.md` 为准。
