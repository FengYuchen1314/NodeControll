# NodeControll 重构设计

本目录是 P4 设计基线。它把社区版 128 项能力、妙妙屋 X 213 项能力和 17 项 PRO/去授权约束转成可编码、可测试、可追踪的目标合同。

## 阅读顺序

1. [总体架构](./ARCHITECTURE.md)：进程、信任域、模块化单体和关键数据流。
2. [sing-box 标准兼容性](./SINGBOX_COMPATIBILITY.md)：官方版本/源码/API/协议差异、reload、流量与限速方案。
3. [Agent 协议](./AGENT_PROTOCOL.md)：enrollment、mTLS、四种连接、durable task、desired/reported。
4. [Rust 后端模块](./RUST_BACKEND.md) 与 [数据模型](./DATA_MODEL.md)：crate、use case/函数和 SQLite/PG 表/invariant。
5. [API 合同](./API_CONTRACT.md) 与 [订阅 IR](./SUBSCRIPTION_IR.md)：浏览器/公开/集成边界、解析生成流水线。
6. [前端 UX](./FRONTEND_UX.md)：Vue 3/Vuetify SaaS 信息架构、页面、组件、状态和无障碍。
7. [安全](./SECURITY.md)、[可观测性](./OBSERVABILITY.md)、[部署](./DEPLOYMENT.md) 与 [迁移](./MIGRATION.md)。
8. [测试计划](./TEST_PLAN.md) 与 [实施计划](./IMPLEMENTATION_PLAN.md)：VPS-only 验收和 WP-00～WP-21。
9. [需求追踪矩阵](./REQUIREMENTS_TRACEABILITY.md)：358 项逐项设计/实施/测试/run/status。

## 设计结论

- Rust Master/Agent + Vue 3/Vuetify，Master 先做模块化单体，远端副作用全部 durable job/outbox。
- sing-box 使用官方 tag/commit 独立制品，不维护私有 fork；生产 stable 与需要 1.14 API 的完整功能轨清晰分开。
- per-user 连接事件使用官方 API；Linux tc/eBPF 做可验证平滑限速，不支持时报告 degraded/unsupported，绝不伪装生效。
- 所有订阅输入进入一套 typed IR；模板纯函数、自定义转换使用无网络/文件的受限 WASM。
- SQLite 和 PostgreSQL 从第一张表起共用 repository contract；备份、迁移、API、Agent 协议都有版本合同。
- 原 PRO 能力全部普通自托管；没有许可证、机器激活、官方域名/额度/分享许可依赖。

文档结构检查由 `tools/validate_design_docs.mjs` 在目标 VPS 的固定 Node 容器执行。`planned` 只表示已分配设计和工作包；必须有代码路径、VPS run 和验收证据才能变成 `verified`。
