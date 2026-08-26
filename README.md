现有模型能力无法完整实现，故暂时封存。

# NodeControll

NodeControll 是一个从零重写的、自托管的代理基础设施与订阅管理平台。项目以妙妙屋和妙妙屋 X 的公开源码、产品行为与功能文档为研究对象，原计划使用 Rust 后端、Vue 3 + Vuetify 前端和标准 sing-box 内核，完整实现妙妙屋 X 的普通与 PRO 功能。

目标形态不依赖商业激活、license key、PRO entitlement、官方授权服务、官方域名数据或闭源控制面。原 PRO 能力在设计中全部视为普通的自托管功能。

仓库目前包含较完整的研究、设计资料，以及一部分经过严格验证的工程与认证代码，但距离可部署的成品仍很远。请勿将当前版本用于生产环境，也不要向公网暴露或写入真实业务数据与长期凭据。

## 已完成

### 上游研究与重构设计

- 锁定并审阅妙妙屋上游源码，完成后端、前端、数据库、HTTP API、任务与主要数据流的逐文件、逐模块和逐函数索引。
- 整理妙妙屋现有功能，共形成 128 个稳定的功能单元。
- 审阅妙妙屋 X 的 58 个中文文档页面，整理 213 个功能与验收单元，并单独记录原 PRO 功能、授权耦合及社区版差距。
- 建立 358 项需求追踪矩阵，完成 Rust、Vue/Vuetify、数据库、OpenAPI、Agent、sing-box、订阅、流量、迁移、安全、部署和测试的总体设计。
- 文档分别保存在 [docs/01-upstream-source](docs/01-upstream-source)、[docs/02-upstream-features](docs/02-upstream-features)、[docs/03-mmwx-gap](docs/03-mmwx-gap) 和 [docs/04-rebuild](docs/04-rebuild)。

### 已写入仓库的代码

- Rust workspace、Master/Agent 程序骨架和 Vue 3 + Vuetify Web 工程。
- typed config、健康检查、readiness、版本信息、Problem Details、OpenAPI 3.1 合同与生成式前端 SDK。
- SQLite 与 PostgreSQL 双数据库持久层，migration 已推进到 0009_webauthn_credentials.sql。
- typed settings、原子文件对象、XChaCha20-Poly1305 密钥 canary、用途隔离的秘密记录与 root-key rotation 基础。
- Owner 初始化、Argon2id 密码、登录限流、服务端会话、CSRF/Origin/Host 校验、近期认证、自助改密、会话轮换与撤销。
- 恢复码、持久认证 challenge、TOTP 事务内核和 crash-safe durable handoff。
- WebAuthn 的 domain、加密 ceremony、凭据持久化、计数器与 BE/BS 状态、克隆嫌疑处理及双数据库事务内核。
- Setup、登录、账户安全等认证页面，以及响应式 SaaS 应用壳和一组共享 Vuetify 组件。
- GitHub Actions 正式构建、许可证归档、SBOM、制品 provenance 与 VPS 验收工具。

### 已取得的正式验证

公开提交 a245ee341f0bf622e8583ddc4a3614190520ccf2 是最后一个完成 GitHub Actions 生产构建和 fresh-clone VPS 制品验收的基线。该基线覆盖 C4 TOTP 核心及此前的认证、双数据库、OpenAPI、SDK、Web、许可证、浏览器和运行时合同；正式门记录为 109 项 Rust 测试、148 项 Web 测试，以及双数据库和真实浏览器流程验证。

当前仓库在此基线上继续加入了 C5 WebAuthn 核心、vendored OpenSSL 和额外并发/秘密边界修补。这些后续代码尚未完成同一提交上的全量 VPS 开发门与正式制品验收，因此只表示“代码已写入”，不表示“已经交付”。

详细历史记录见 [docs/00-project/PROGRESS.md](docs/00-project/PROGRESS.md)，各实现片的模块与限制见 [docs/05-implementation](docs/05-implementation)。

## 尚未完成

| 范围 | 当前缺口 |
| --- | --- |
| MFA 用户闭环 | TOTP 的 base32、otpauth URI、二维码、HTTP/OpenAPI/Vue 接线尚未完成；WebAuthn 也没有 HTTP API、前端调用和真实浏览器注册/认证互操作 |
| 身份与权限 | API token、完整 RBAC、对象级授权、用户生命周期、管理员用户管理尚未完成 |
| 任务与审计 | durable jobs、outbox/inbox、调度器、SSE、完整审计链和通知事件系统尚未完成 |
| Agent | enrollment、mTLS、协议版本、四种连接方式、远端任务、特权 helper、宿主观测均未实现 |
| sing-box | 官方制品管理、配置编译、生命周期、入站、节点、出站、路由、WARP、隧道与回滚均未实现 |
| 用户与流量 | 套餐、多个 entitlement、流量账本、连接观测、限速、并发/IP/设备策略均未实现 |
| 订阅系统 | 外部订阅源、parser/IR、provider、模板、规则、脚本沙箱和多客户端编码器均未实现 |
| 周边能力 | 证书、DNS、Nginx、测速、公开探针、Telegram、MCP、实例联合均未实现 |
| 迁移与运维 | 妙妙屋数据迁移、备份恢复、安装卸载、升级回滚、离线发布包均未完成 |
| 最终验收 | 358 项产品需求仍未逐项关闭；全量 E2E、安全、性能、故障注入、无障碍和长期运行验收未执行 |

P5 工程骨架没有收尾，P6 功能实现和 P7 系统验收也没有完成。需求追踪矩阵中的 358 项验收需求目前仍全部标记为 planned，implemented 和 verified 均为 0。现有页面和接口只覆盖认证与基础系统投影，不能替代妙妙屋或妙妙屋 X。

## 仓库目录

- [docs/00-project](docs/00-project)：范围、决策和历史进度。
- [docs/01-upstream-source](docs/01-upstream-source)：妙妙屋源码研究。
- [docs/02-upstream-features](docs/02-upstream-features)：妙妙屋功能目录。
- [docs/03-mmwx-gap](docs/03-mmwx-gap)：妙妙屋 X、PRO 与社区版差异。
- [docs/04-rebuild](docs/04-rebuild)：完整重构方案与 358 项需求追踪。
- [docs/05-implementation](docs/05-implementation)：已写代码的模块说明与已知限制。
- [apps](apps)：Rust Master/Agent 与 Vue Web。
- [crates](crates)：domain、application、API、配置、身份、持久化、对象存储与秘密模块。
- [openapi](openapi)：当前 OpenAPI 合同。
- [deploy/build](deploy/build)：固定构建镜像定义。

## 封存说明

本次封存保留源码、设计、研究结论、测试工具和历史证据，方便以后由维护者或其他实现者继续工作。仓库不承诺维护周期、兼容性、漏洞响应时限或可用发布版本。GitHub Actions 构建成功只能证明对应工程门通过，不能证明上表中的产品功能已经完成。

若恢复开发，应先从当前提交重新执行完整 VPS 开发门，再由 GitHub Actions 构建同一提交的生产制品，并以 fresh clone 完成正式验收；不得沿用旧提交或未闭合候选的通过结论。

## 构建与安全边界

正式生产编译只由 [.github/workflows/build.yml](.github/workflows/build.yml) 执行。历史开发测试与制品验收在维护者的私有 VPS 环境完成，本仓库不保存主机地址和 SSH 身份。

安全问题请按 [SECURITY.md](SECURITY.md) 说明报告。上游研究和第三方材料的边界见 [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)。

## 许可证

NodeControll 自研代码以 [GNU AGPL v3.0 only](LICENSE) 发布。第三方依赖、上游项目和外部文档仍适用各自的许可证与权利声明。

这里的“无商业授权依赖”只表示产品功能不受商业激活、PRO 权限或官方服务控制，不表示代码没有开源许可证，也不免除分发与网络服务中的法律义务。
