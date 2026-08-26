# NodeControll 实现说明

本目录随代码实现持续更新，记录每个新模块的职责、公共接口、关键算法、配置、数据迁移、测试覆盖和运维方式。

## 实施记录

- [WP-00/P5.0 工具链与首个工程纵切](./WP00_ENGINEERING_SKELETON.md)：Rust/Vue 版本决策、builder、现有每个函数/模块、OpenAPI SDK、失败修正和 VPS run `20260825T145357Z-p5`。
- [WP-01/P5.1 配置、双数据库与公开系统投影](./WP01_FOUNDATION_SLICE.md)：typed config、SQLite/PostgreSQL 18 migrations/repository contract、真实 readiness/bootstrap、Problem Details、Vue setup/system 和提交级 VPS run `20260825T152835Z-p5`。
- [WP-01/P5.1 存储与密钥纵切](./WP01_STORAGE_SECRET_SLICE.md)：typed settings revision contract、atomic content-addressed filesystem objects、XChaCha20-Poly1305 root-key canary 和提交级 VPS run `20260825T154501Z-p5`。
- [WP-02/P5.2 身份初始化纵切](./WP02_IDENTITY_BOOTSTRAP_SLICE.md)：Owner/instance 原子 bootstrap、Argon2id 密码、一次性 setup secret、SQLite/PostgreSQL 一致状态判定、OpenAPI/SDK 与 Setup UI；该纵切已经作为公开基线的一部分通过 Actions 制品驱动的 VPS 正式门，后续状态以密码登录纵切为准。
- [WP-02/P5.3 密码登录与服务端会话纵切](./WP02_AUTH_SESSION_SLICE.md)：共享登录限流、全路径并发闸门、HMAC 会话、idle/absolute 期限、Origin/Host/CSRF、可信代理、登录/恢复/退出前端状态机；公开 SHA 已通过 Actions + fresh-checkout VPS 正式门。MFA、token、完整 RBAC 与用户生命周期尚未进入本纵切。
- [WP-02-C 认证安全合同](./WP02_C_AUTHENTICATION_SECURITY_CONTRACT.md)：冻结近期认证、透明 rehash、自助改密码、两类 session rotation、恢复码、TOTP 和 WebAuthn 的状态机与双数据库验收门。该文档是后续纵切的约束，不是完成声明。
- [WP-02-C1 密码近期认证、改密与会话管理实现](./WP02_C1_PASSWORD_RECENT_AUTH_SESSION_IMPLEMENTATION.md)：逐模块记录 recent-auth、透明 Argon2 rehash、改密、session rotation/管理、API、Vue 状态机、双库事务与测试证据。公开 `3f1bcb49…`、Actions run `32976849583`/artifact `9609917545` 与 fresh-checkout VPS run `20260826T135902729109375Z-p5` 已完成正式验收。

功能只有在 [需求追踪矩阵](../04-rebuild/REQUIREMENTS_TRACEABILITY.md) 填入实现路径、VPS run 并标记 `verified` 后才算完成。当前工程纵切用于建立质量门，不代表 358 项产品能力已经实现。
