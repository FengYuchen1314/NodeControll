# NodeControll

NodeControll 是一个正在重构中的、完全自托管的代理基础设施与订阅管理平台。目标是以 Rust 后端、Vue 3 + Vuetify 前端和 sing-box 标准内核，实现妙妙屋与妙妙屋 X（包括原 PRO 功能）的完整能力，不依赖商业激活、license key、PRO entitlement、官方授权服务、官方域名数据或闭源服务。

P0～P4 的源码研究、功能差异和重构设计已经完成，当前正在 P5 构建工程骨架。Rust Master/Agent、OpenAPI 3.1、Vue/Vuetify SaaS 壳、typed config、SQLite/PostgreSQL 18 repository、typed settings/filesystem object/AEAD canary，以及 Owner 初始化、密码登录、服务端 session、近期认证、自助改密和自身会话管理纵切已经实现；较早的密码登录/session 基线通过了公开 Actions 与正式 VPS 制品门，当前 C1 扩展已通过增量候选验证，仍待同 SHA Actions 制品和 fresh-clone 正式验收。MFA、RBAC、Agent enrollment 和业务功能尚未完成，项目不能视为可发布产品。权威进度见 [`docs/00-project/PROGRESS.md`](docs/00-project/PROGRESS.md)。

## 文档结构

- `docs/00-project`：项目范围、决策、进度和验收状态。
- `docs/01-upstream-source`：妙妙屋源码的模块、文件、函数、数据流和接口解剖。
- `docs/02-upstream-features`：妙妙屋现有产品功能与行为。
- `docs/03-mmwx-gap`：妙妙屋 X（含 PRO）功能证据与差异矩阵。
- `docs/04-rebuild`：Rust/Vue/Vuetify/sing-box 重构架构和实施计划。
- `docs/05-implementation`：新代码的模块说明、测试和运维手册。
- `apps`：Rust Master/Agent 与 Vue 3/Vuetify Web 应用。
- `crates`：无框架 domain 与后续 API、持久化、任务、协议适配模块。
- `deploy/build`：固定官方 digest 派生的 VPS 测试/验收镜像。
- `openapi`：由 Rust `utoipa` 导出的 API 3.1 合同。
- `third_party/dependency-license-overrides`：依赖归档缺少许可证文件时使用的精确版本、来源 revision 与 hash 审阅证据。
- `upstream`：本地/远端上游研究快照，不作为新系统实现代码，也不纳入本仓库提交。

## 构建纪律

正式编译由 GitHub Actions 的 [`.github/workflows/build.yml`](.github/workflows/build.yml) 完成；单元、双数据库、Web 和运行时测试只在项目维护者配置的私有一次性 VPS 环境执行。本地工作区只负责编辑和审阅，不保存测试主机地址或 SSH 身份。VPS 统一入口是 `tools/vps_verify.sh`，每次运行在私有测试根目录保存 builder ID、lock hash、逐阶段日志、checksums 和完成时间。

## 参与与安全

项目仍处于不可发布的重构阶段。提交变更前请阅读 [`CONTRIBUTING.md`](CONTRIBUTING.md)；安全问题请按 [`SECURITY.md`](SECURITY.md) 私下报告，不要在公开 issue 中披露利用细节。上游研究资料的边界见 [`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md)。

## 许可证

NodeControll 自研代码以 [GNU AGPL v3.0 only](LICENSE) 发布。通过网络向用户提供修改版服务时，需遵守许可证第 13 节的对应源码提供义务。第三方依赖、上游项目和外部文档仍各自适用其权利声明；本许可证不替代它们。

本文所说的“无许可证/无授权”只指产品功能不受商业激活、license key、PRO entitlement 或官方服务控制，不表示代码没有开源许可证，也不免除分发与网络服务中的法律义务。AGPL 许可证和第三方 legal notices 是开源法律凭证，不是功能开关、额度或付费授权。业务模型中的本地用户 `entitlement` 仅表示管理员分配的订阅与资源权限，也不是 PRO 授权。
