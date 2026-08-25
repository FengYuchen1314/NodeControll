# 参与 NodeControll

NodeControll 尚在按工作包逐步实现，当前不是可部署产品。开始修改前先阅读 `docs/00-project/PROGRESS.md`、对应工作包和 `docs/04-rebuild/REQUIREMENTS_TRACEABILITY.md`，不要把计划中的能力写成已经完成。

## 变更要求

- Rust、Node、pnpm 和直接依赖版本必须保持精确锁定；升级需同时说明官方来源和兼容性影响。
- API 先修改 Rust/utoipa 合同，再生成并提交 OpenAPI 与 Web SDK；生成物漂移会让 Actions 失败。
- 数据库变更必须同时提供 SQLite 与 PostgreSQL forward migration，并覆盖新装、历史升级和事务失败路径。
- 不提交 `.env`、密钥、真实主机地址、SSH 身份、数据库、日志、备份或第三方网页全文。
- 新业务能力必须链接稳定需求 ID、补实现文档和测试；只有完整验收后才能把 trace 状态改为 `implemented` 或 `verified`。

## 构建与测试边界

Pull request 会触发 GitHub Actions 的 release 编译、OpenAPI/SDK 漂移检查和 Web production build。维护者在私有、一次性的 VPS 测试环境对同一 commit 与 Actions 制品执行 Rust/前端/双数据库/运行时门禁；仓库不保存该环境的连接信息。

贡献者可以在自有隔离环境运行相同工具做预检，但本项目进度文档只能引用维护者保存的 VPS run artifact。不要把本地成功结果写成权威验收。

## 提交 Pull Request

说明问题、设计边界、关联需求 ID、迁移影响和测试范围。UI 变更附截图；协议、数据迁移、安全或破坏性操作需给出失败与恢复路径。保持 PR 聚焦，不顺带格式化或重写无关文件。
