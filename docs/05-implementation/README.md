# NodeControll 实现说明

本目录随代码实现持续更新，记录每个新模块的职责、公共接口、关键算法、配置、数据迁移、测试覆盖和运维方式。

## 实施记录

- [WP-00/P5.0 工具链与首个工程纵切](./WP00_ENGINEERING_SKELETON.md)：Rust/Vue 版本决策、builder、现有每个函数/模块、OpenAPI SDK、失败修正和 VPS run `20260825T145357Z-p5`。
- [WP-01/P5.1 配置、双数据库与公开系统投影](./WP01_FOUNDATION_SLICE.md)：typed config、SQLite/PostgreSQL 18 migrations/repository contract、真实 readiness/bootstrap、Problem Details、Vue setup/system 和提交级 VPS run `20260825T152835Z-p5`。
- [WP-01/P5.1 存储与密钥纵切](./WP01_STORAGE_SECRET_SLICE.md)：typed settings revision contract、atomic content-addressed filesystem objects、XChaCha20-Poly1305 root-key canary 和提交级 VPS run `20260825T154501Z-p5`。
- [WP-02/P5.2 身份初始化纵切](./WP02_IDENTITY_BOOTSTRAP_SLICE.md)：Owner/instance 原子 bootstrap、Argon2id 密码、一次性 setup secret、SQLite/PostgreSQL 一致状态判定、OpenAPI/SDK 与 Setup UI；当前代码尚待正式 Actions 制品驱动的 VPS 验证。

功能只有在 [需求追踪矩阵](../04-rebuild/REQUIREMENTS_TRACEABILITY.md) 填入实现路径、VPS run 并标记 `verified` 后才算完成。当前工程纵切用于建立质量门，不代表 358 项产品能力已经实现。
