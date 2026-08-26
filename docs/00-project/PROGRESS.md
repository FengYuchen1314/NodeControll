# NodeControll 实施进度

> 本文档是项目推进的权威事实源。每次实现、验证、架构变更和风险发现都必须更新。时间使用 Asia/Shanghai。

## 当前状态

- 当前阶段：P5，WP02-C1 已完成公开提交级验收；C2+C3 已通过合并态 VPS 开发门并进入本地主线，尚待含 WP04 的主线组合门与公开 Actions 正式制品门；WP04-A 共享 SaaS 组件已通过独立 VPS 门，WP04-B 应用壳层正在实现；P0～P4 已完成。
- 总体状态：进行中。
- 当前上游基线：`iluobei/miaomiaowu@0b47f10c52aee10b9f759a593ca5f61a823cbb72`（`main`，2026-08-25 获取）。
- 妙妙屋 X 文档基线：`https://miaomiaowux.com/docs/tutorial` 及同站文档页，2026-08-25 开始抓取。
- 本地工作区：维护者私有工作区（公开文档不记录操作系统账户路径）。
- 远端测试根目录：`/opt/nodecontroll`；主机地址和 SSH 身份只存在于维护者私有配置，不进入仓库。
- 新系统工作名：NodeControll；最终命名尚未锁定，不影响内部模块边界。

## 阶段看板

| 阶段 | 目标 | 状态 | 完成定义 |
|---|---|---|---|
| P0 | 环境、上游、文档证据基线 | 已完成 | 上游提交锁定；X 文档目录完整；VPS 工具链可复现 |
| P1 | 妙妙屋逐函数源码解剖 | 已完成 | 每个源文件、导出/内部函数、数据表、API、任务均有说明 |
| P2 | 妙妙屋功能说明 | 已完成 | 管理端、订阅端、探针、用户、模板等行为和限制完整 |
| P3 | 妙妙屋 X/PRO 差异分析 | 已完成 | 文档功能逐项有证据、差异、优先级和验收条件 |
| P4 | 重构总体设计 | 已完成 | 架构、数据、API、UI、内核、迁移、安全、测试无待定大项 |
| P5 | 工程骨架 | 进行中 | Rust/Vue/Vuetify、数据库、CI、远端构建链路通过 |
| P6 | 核心与扩展功能实现 | 未开始 | 规划功能逐项通过自动化测试 |
| P7 | 系统验收和交付 | 未开始 | E2E、性能、安全、升级/回滚、备份恢复全部验收 |

## 已完成内容与代码说明

### 2026-08-27 00:26 — C2+C3 合并态 31/31 门通过并合入本地主线

- 最终不可变候选绑定源码 `e5f38be7b9f75fd8a39a75ce8d445da673015cef`，目录为 `/opt/nodecontroll/dev/e5f38be-c2c3-20260826T161714Z-527e6805`。三笔最小源码修补分别把 challenge SQL 改为 SQLx 可证明为 `'static` 的双后端字面量、把 SQLite/PostgreSQL 不同 `QueryResult` 只归一为共同的 `rows_affected: u64`、修正 migration 版本与大枚举布局，并让 completion-time 生产路径真实读取持久 reservation 时间。测试目标会实际调用 method-verifier 构造器，因此 lint expectation 只挂在非 test target；生产目标仍会在 C4 接入后以 unfulfilled expectation 提醒删除临时标记，没有改成 `allow`。
- 失败过程没有被抹掉。首个候选由 12 处动态 SQL lifetime 错误和三处双库返回类型错误阻断；后续候选又发现 migration 断言仍停在 6、Clippy 大变体/未使用 seam，以及 SQLite `immutable=1` 忽略 WAL 的门脚本假设。WAL 门随后改为先证明运行态 WAL，再 checkpoint 到独立验证副本、关闭连接并只读重开。最终复核还发现旧 secret scan 同时使用互斥的 `grep -E/-F`，会把工具 `rc=2` 错当成无命中；该轮没有冒充通过，而是重新分离扫描输入和输出日志，并规定 `rc=0` 为泄漏失败、`rc=1` 为无命中、`rc>=2` 为扫描器故障失败，再从同一源码新归档全跑。
- 最终 31/31 门 `overall_rc=0`：Rust fmt/check/workspace all-targets test/Clippy/debug Master 均通过，共 92 项测试；真实 PostgreSQL 18.6 与 SQLite repository contracts、0006/0007 migration/typed-secret guard、canary wrong/old key、恢复码整体替换和并发单次消费均执行。运行时 OpenAPI 与 tracked 文件精确一致，为 13 paths/15 operations；SDK 重生成零漂移。Web typecheck、零 warning ESLint 和 12 files/111 tests 全绿；设计矩阵仍为 358 条、0 broken links，sanitizer 零修改。
- SQLite 与 PostgreSQL Master smoke 输出逐字一致：bootstrap 只返回恰好八枚规范恢复码且 `no-store`，缺 CSRF 不改状态，有效替换令版本精确 `+1` 且新旧集合不重叠；持久态均为 migration 7、set 1 replaced、set 2 active、8 remaining。SQLite Master 停止删除后 WAL 从 1,380,232 bytes checkpoint 为 0，`busy=0`，关闭连接后的只读重开验证通过。严格 secret scan 覆盖 37 个输入文件，canonical recovery-code 正则、四个运行时秘密和固定密码六项全部得到明确 `grep rc=1`。
- archive、source pre/post manifest、`evidence.txt`、logs manifest、evidence-files manifest 和 gate script 的 SHA-256 依次为 `6510609a0619debe9725102f5a3ca2dd4c30b6d0bfbcf01ff6752c21d6c481ec`、`56efc974ec63a1565b89237b89bf9b2cb9b25821ba46f32dfbfb843e84cc37d9`、`800f4207142e7d6f78efbe1f94d4b5d9e8f27b94d4721996b4d191eedb423403`、`d913d16f96ec128f632d7785d14b8029391c8cec1a7fe87238d213201a7c2fea`、`cc30a48d305c10b66b26fc8e5af64cf6930875dde41b6ba70bf3ac9f8ef984cf`、`257c2bb6c0371fa3d74abfbf0894daaaf76fdc014aec0625fa759a963dc79dc2`；扫描输入 manifest 为 `d8b88ec06d3cfecb2106d09cb087c878e15eaf361f68fa25d1727aba3d90dbe7`。
- 三笔经过验证的修补已按顺序合入本地主线为 `8018c5a…`、`1565454…`、`b99ec4e…`。VPS 未执行 release 或 Vite production build，具名容器、网络、PG 匿名卷与 build scratch 已精确清理；这仍是开发门，不冒充公开 Actions 制品验收。下一步先把 WP04-B 及必要合同修补收口，再对精确主线运行组合开发门，之后才显式推送公开 `main`。

### 2026-08-27 00:02 — WP04-A SaaS 共享组件通过独立 VPS 门并合入主线

- 新增 `ResourceHeader`、`StatusChip`、`DangerDialog`、`SecretField`、`DesiredReportedDiff`、`PolicyExplainer` 和 `SafeDisplayValueView`，公共入口只暴露展示合同，不复制尚不存在的 API DTO。主题现在同时注册 light/dark SaaS token；组件统一使用 Vuetify 语义色，在 360px 下保留标题动作、状态证据、危险确认、秘密输入、desired/reported/last-good 和策略来源的完整可访问结构。
- `DangerDialog` 在发出危险动作前先取得本地单次提交锁。同步失败、旧错误或一次普通 pending 往返都不会自行解锁；父页面必须提供新的 `retryRevision` 作为明确 terminal failure 信号后才能重试。`SecretField` 不持久化输入，只保存 reveal 布尔值，并在 visibility/pagehide/unmount 时重新遮蔽；现有值只显示“已配置”，不会从服务端回填。差异与策略组件只接受 `text/empty/redacted` 判别值，夹带到 redacted 对象中的原文不会被渲染。
- 分支最终提交 `0a652d1c59d53029f8d60cd5188e555e499fbb74` 的 fresh VPS run 为 `20260826T155301Z-wp04-saas-v5`。固定 Node 24.19.0/pnpm 11.24.0 下，16 个 OpenAPI 生成文件零漂移，typecheck、零 warning ESLint、18 个文件/122 项 Vitest、OpenAPI、79 篇设计文档/0 broken links 和 sanitizer 全绿；按边界没有运行 production build。archive/source、source manifest、generated manifest 和 evidence manifest SHA-256 分别为 `812daa151aea06c6f60ebd199e71804683c4656334eb540c1566fe663224fdb9`、`8b827b76cf9b963f3ecf050c10096324840d035dfa74860603456b230f9dfade`、`b0158df8fde2c31d8d491c211b51f209c55a20c471de4dca3e4b7bfd163e39ba`、`20eba2c7b3aae2dca8769a7db1ca5ec73e2bd7872d5b2343d55fd09706794f19`。
- 八笔经过验证的提交已按顺序合入本地主线，形成 `df3595d…` 至 `7fee4b0…`；实现索引冲突只合并了 C3 与 WP04 两条独立文档链接，没有修改代码。AppShell、command palette、权限导航、DataTable、JobDrawer、MetricChart、真实 API 页面、i18n、axe/visual/Playwright 和性能预算仍属后续 WP04，不把本批组件称为完整界面。

### 2026-08-26 23:28 — C2 后端独立门通过；`334c8ea…` 合并态门已启动

- C2 后端第三个全新候选绑定源码提交 `2ddb143e0af9514b943c324b2718a820c620a28a`，archive SHA-256 为 `8a3be5d6b45802d3517d04ad4bee62d30348f8f745255681051b6ba750e953af`；候选目录是 `/opt/nodecontroll/dev/2ddb143-c2-20260826T151845Z-2718dab6`。上传前后 source manifest 同为 `22f4171924e31b3d356061e9f851b620e3364120afc80bccd588fff6fbdb565e`，证明门禁没有在候选内原地改源码。
- 固定 Rust、Node 和 PostgreSQL 18.6 镜像下，Rust fmt/check/workspace all-targets tests/Clippy `-D warnings`、SQLite 与真实 PostgreSQL repository contracts、OpenAPI、358/358 需求文档、链接及 sanitizer 全部以 `rc=0` 结束。test log、logs manifest 和 evidence SHA-256 分别为 `025dd2084e036d26af8e6046cbd8317f366e608e6600e9eb5c532230e06f26ba`、`14d389a528430df90f95ec0b67036f40a3b39b3ba7018620deff186e4b88db50`、`1440321274302712b7a0ce6175f242a8dd79c7a01f1b1a946d4252d7beab1b5f`。
- 通过项明确覆盖 0006 迁移防护、错误/旧 key canary、首次 bootstrap 恰好八枚恢复码、整组原子替换和并发单次消费。VPS 没有执行 release 或 Web production build；本轮具名容器、网络、PostgreSQL 匿名卷及约 4.5 GB build scratch 已精确删除，源码、日志与校验文件保留。OpenAPI 对恢复码字符串长度/正则及部分计数上界的静态约束仍偏宽，已登记为后续合同硬化项；Web 运行时目前执行更严格的规范校验。
- C3 在随后互审中补出一个只读、crate-private 的 `AuthChallengeVerificationClaim::reserved_at_ms()`，使 TOTP/WebAuthn verifier 能以服务端持久 reservation 时间判定挑战窗口，而不允许 HTTP 层伪造时间。该修补已作为本地主线 `334c8ea…` 的最后一笔提交。
- 现已从精确、干净的本地主线 `334c8ea2af42068fd69d215c2df868c3f1d4225f` 启动新的不可变 VPS 集成候选。门禁将同时覆盖 C3 双库迁移/合同、完整 Rust 与 Web 开发门、运行时 OpenAPI/SDK 零漂移、文档闭包，并在 SQLite 与 PostgreSQL 两个真实 Master 上执行扩展恢复码 smoke；任何秘密都不得进入日志。本轮同样禁止在 VPS 编译 release/Vite production，结果返回前不登记 C3 或合并态通过，也不推送公开 `main`。
- 首个 `334c8ea…` 集成候选的 fmt 已通过，但 workspace check/test 以 `rc=101` 失败：`crates/persistence/src/auth_challenge.rs` 有 12 处把非 `'static` `&str` 传给 SQLx 0.9 `query` 的 E0277，三个双数据库测试 helper 又把 SQLite 与 PostgreSQL 的不同 `QueryResult` 放在同一 match 返回值，触发 E0308。这是确定的源码错误，候选已判失败；门脚本会继续收集其余结果并精确清理。修复必须从原 SHA 的独立 worktree 开始，把动态 SQL 改为后端各自的静态字面量，并只把不同 QueryResult 的 `rows_affected()` 归一成共同整数；不得泄漏动态字符串、放宽 SQLx 或在失败候选原地改动。新提交形成后另建不可变候选全量重跑。

### 2026-08-26 23:24 — C2 Web 独立门通过；C2 后端第三候选与 C3 合并态待验

- C2 后端第一次精确候选以 `dea2b96…` 为源码基线，在隔离 Rust/PostgreSQL 环境中真实执行。Node 侧 OpenAPI、文档和源码门通过；Rust 门准确拦下两项源码问题：`crates/secrets/src/lib.rs` 有两处 `rustfmt` 漂移，一条负向测试用 `Result<bool, SecretError> == Ok(false)`，而含 I/O 错误的 `SecretError` 不应为了测试派生 `PartialEq`。修补改用嵌套 `matches!(…, Ok(false))`，两处格式由固定 VPS builder 产出；后端分支提交为 `9b2f5f63…`，合入本地主线后是 `d5b6ee8…`。
- 第二候选的 fmt/check 和 Node 三门通过，SQLite/PostgreSQL 却在同一既有 session 列表断言失败：恢复码合同先创建并撤销了一条 session，随后 `list_user_sessions` 会按设计返回所有状态，污染了下一个合同的 `len == 1` 前提。修补没有放宽旧断言，而是在恢复码合同完成 revoke 后按 `session.id + owner.id + revoked` 精确删除自己的测试夹具、断言只删一行并确认 owner 暂无 session；三个 Clippy `type_complexity` 则收敛成私有 `RecoveryReplacementSnapshot`，没有增加 `allow`。分支提交 `2ddb143…` 已合入本地主线为 `0fb6562…`，第三个全新候选正在重跑，结果返回前不登记后端门通过。
- C2 Web 主体已以 `e8b821e…` 合入本地主线。一次性明文只有在 terminal journal 成功 settle、持久化并广播后才交给页面；失败会清空 staged/transfer 数组并进入 quarantine。bootstrap 改成 status-first：只有结构正确、可证明未提交的 Problem 4xx 可以重新开放表单；畸形/空 5xx、意外 2xx 和无法验证的 201 都锁定并只做 GET 对账，不自动重放。恢复码响应限定为恰好 8 枚、每枚 8 组小写十六进制；运输层拒绝重定向，以流式读取强制 64 KiB 上限，并只接受精确成功状态、媒体类型与 `no-store`。
- VPS 从合入态 OpenAPI 重生成了 recovery-code SDK：tracked 生成文件对应分支提交 `3000941…`，安全运输层随后以生成请求/响应类型和字面路径取代临时手写类型，提交 `ba5b117…`；两者已分别合入本地主线为 `4fc0a34…`、`d04f157…`。生成客户端仍会跟随重定向并先做无界 `response.text()`，所以这里只复用合同，不删除专门的受限运输层。随后五笔修补把 terminal disposition 改为显式赋值、收紧浏览器 lint、在清空后销毁一次性对话框实例，并让账户安全页测试经真实 `RouterView` 挂载，避免“测试通过但离页 guard 没有注册”的假覆盖；它们已合入本地主线 `96c3f89…` 至 `40c0ecb…`。
- C2 Web 最终精确候选为 `31c112e9…`，archive SHA-256 `d66b64e34b396e4b0b2025e7aa64139f114a2770537b011e69b0e2c42096d1fc`。固定 Node 24.19.0/pnpm 11.24.0 fresh install 下，OpenAPI 为 13 paths/15 operations，16 个生成文件前后 manifest 同为 `418cadbc…ca4b`；typecheck、零 warning ESLint、12 files/111 tests、358/358 文档、0 broken links 和 sanitizer 全绿，最终没有 Router guard warning。测试日志 SHA-256 为 `aad478f9…a45e`，final-gates manifest 为 `0f7d9821…bd1f`。按发布边界没有执行 production build；VPS 的 store、工作副本、临时文件和容器已清理，只保留最终源码、日志和 hash。
- 本地主线 `f4786c5…` 扩展了 `tools/smoke_master.mjs` 的 C2 运行时合同：bootstrap 必须只返回一次 8 枚规范恢复码并带 `no-store`；活动会话可读取无秘密摘要；缺失 CSRF 的再生成必须 403 且 set 版本、余量和创建时间不变；有效再生成必须精确 200、版本加一、旧组整体替换，随后 GET 与新组一致。脚本在完成结构与不重合校验后立刻覆写响应对象中的明文数组，只输出布尔结论，不把恢复码写入日志。这笔只做了静态 diff 审计，等待后续合并态 VPS runtime smoke。
- C3 修正版以源码提交 `591a1fa13…` 完成，已合入本地主线为 `5d32d18…`。它把 proof 前 attempt reservation、同用户/目的单 active limiter、method/assurance 矩阵、opaque bearer、网络/UA 精确绑定和 replacement-session 事务 seam 放进同一模型。进一步审阅发现 proof 已提交为 `rotation_pending` 后、替换 session 前若进程退出，原先只有一次性内存 claim，会卡到 TTL；修版增加持久 handoff lease、受 bearer/context/revision 约束的 resume，以及超时后只释放 handoff 的恢复路径，并用 CAS 保证并发 resume 只有一个 transaction claim。源码合同覆盖四路 proof/resume、最后 slot 成功、两层 lease 崩溃恢复、exhausted 发行绕过、上下文/认证版本/session 失效和 raw schema 破坏；独立实现说明见 `WP02_C3_AUTH_CHALLENGE_IMPLEMENTATION.md`。它尚未经过 cargo 或双库门，不能登记为通过。
- 当前公开 `origin/main` 仍停在已正式验收的 `3f1bcb49…`。上述本地提交没有推送，GitHub Actions 也没有为它们生成正式 production artifact；只有 C2/C3 合并态在 VPS 通过开发门后才会显式推送 `HEAD:refs/heads/main`，随后由公开 Actions 进行唯一正式编译。

### 2026-08-26 22:40 — WP-02-C2 后端进入本地主线；Web/C3 互审阻断正在修正

- C2 后端候选提交 `7bc89f04c1eb67fb2388471cd66503f3f5575ef1` 已合入本地主线，形成 `dea2b96…`。合并只在实现索引里遇到一处文字冲突：保留 C1 已通过的正式证据，同时追加 C2 待验收状态；13 个代码、锁文件、迁移与 OpenAPI 路径和候选提交逐字节一致。该提交尚未推送，不能称为公开实现。
- `crates/secrets` 现有 typed `NCSECRET2` AAD、持久 root-key canary、current 加最多 3 枚旧 key 的有限 keyring，并新增 256-bit、严格小写十六进制、自动清零且没有 `Debug/Clone` 的 challenge bearer。session、CSRF、登录 bucket、恢复码和 challenge 各用独立 HKDF/HMAC purpose；旧记录只能按自身 key version 走有限旧 key 验证。
- SQLite/PostgreSQL `0006_secret_recovery.sql` 增加 typed `secret_records`、恢复码 set 与单码记录。非空的旧式无 owner/schema secret 表会让迁移原子失败，不猜测归属。bootstrap 在原事务里创建 8 枚 128-bit 恢复码；数据库只保存用途隔离 HMAC 和版本，明文只在 201 响应出现一次。GET 只读摘要；POST 绑定 Origin/Host、session、double-submit CSRF、近期认证和强制改密限制，并在事务内重验 user/auth/session 时间线后整体替换旧组；同一码并发消费只有一次成功。
- 后端分支的静态检查记录为：`cargo metadata --locked --no-deps`、13 paths/15 operations OpenAPI、358/358 需求文档与 76 份 authored docs、0 broken links、diff check 和 20 个提交 blob 的 LF 审计均通过。它没有在本机编译或运行测试。主线精确 SHA 的 Rust fmt/check/test/Clippy、真实 PostgreSQL 与文档/OpenAPI VPS 候选已经另行启动，结果未返回前不登记通过。
- C2 Web 的独立审阅阻止了原候选直接合并：terminal coordinator settle 失败后可能把一次性明文重新放入 Pinia；bootstrap 遇畸形 5xx 或意外 2xx 时可能开放重放；页面卸载、BFCache、同 URL 离开再返回、非流式 64 KiB 上限、过宽恢复码格式和迟到 generation 分支也缺少闭包。前端分支正在改为 terminal 成功后才交付明文、status-first 不重放、operation ownership/pagehide 清理、流式限额、严格 8×32-hex 分组格式和对应负向测试。
- C3 早期 challenge 候选同样未合并。互审发现并发同 revision 猜测只记一次、可用新 challenge 绕过 attempt budget、method/assurance 可错配、客户端摘要只存不验，以及 rotation completion 可脱离 replacement-session 事务。修正版采用 proof 前原子 claim/attempt reservation、单 active issuance、真实 context 条件、不可伪造 evidence 与 method-assurance 矩阵，并只保留能与 session replacement 共事务组合的 port seam；它将直接复用 C2 的 challenge bearer，不复制密码学实现。

### 2026-08-26 22:01 — WP-02-C1 公开 Actions 制品与 fresh-checkout VPS 正式门通过

- C1 的公开实现提交为 `d200c033b81ebabfe0c99c50572cc46186ba5329`，随后用 `3f1bcb49da5743a0a4585a9635a27437000c8011` 修复 VPS 验证器里一处未定义的 Python `phase` 局部变量。两个提交都只接在公开 `main` 上，旧的私有 `master` 历史没有进入公开分支；当前正式证据绑定后一个 SHA。
- GitHub Actions `Build` run `32976849583`、attempt `1` 在 3 分 13 秒内完成唯一的正式生产编译。Rust 二进制、运行时 OpenAPI、Vue production Web、许可证正文、依赖清单和 CycloneDX 1.6 SBOM 被打成 artifact `9609917545`。原始 gzip 共 4,776,366 bytes，SHA-256 为 `81a272e006bb7016f5837ca065c5dc26ac612c44bc5c2bbb5745c2279933d657`；Actions 同时验证了生成 SDK 和 tracked source 在打包前后零漂移。
- VPS 从公开仓库新建 `/opt/nodecontroll/checkouts/3f1bcb49da5743a0a4585a9635a27437000c8011`，本地 `main`、`origin/main` 和远端 `main` 均精确指向该 SHA，共 226 个 tracked files。正式 run `20260826T135902729109375Z-p5` 于 `2026-08-26T14:01:31Z` 完成，未在 VPS 重建 release/Web/notices；它只校验 Actions 产物并执行测试。
- 制品门核对了 1,575 个规范 archive members、893 个 packaged files、650 个锁定组件、858 份许可证证据和 20/20 个精确 override；CycloneDX schema、两个 ELF 的 glibc 解释器、七行 `BUILD-METADATA`、包内 `CONTENTS.sha256`、OpenAPI 与 Web 静态资源均通过。fresh pnpm virtual store 的 428 个实际 npm identity 与 artifact inventory 双向相等。
- Rust 1.98.0 在 SQLite 与真实 PostgreSQL 18 合同下通过 78/78 workspace all-targets tests、`cargo fmt` 和 Clippy `-D warnings`。Web 重新生成 4 个顶层 SDK 输出，typecheck、零 warning lint 和 9 files/81 tests 全绿。runtime smoke 覆盖 bootstrap、并行 session、失败/成功 reauth、sibling 保留、逐会话撤销、改密 replacement、旧密码拒绝、keep-current rotation、logout-all、CSRF 与普通 logout，打包运行时 OpenAPI 与 12-path 源合同逐字节相同。
- 双页 HTTPS Playwright 门再次证明旧 Cookie 在 rotation 后返回 401 且零 `Set-Cookie`，logout 503 会关闭两页受保护 DOM 并将 quarantine 保持到 reload，显式登录可恢复，新 epoch 不接受旧 cursor 的迟到 invalidation，最终权威 logout 204 才清 Cookie。冻结证据为 33 files、10,087,550 bytes；gate SHA-256 `459eabe176e90e20ac9bda5845cb943c176a505ecc2aace96b62f1f38c96577c`，browser closure SHA-256 `ff9fb087c90d4553358a8df82a6e6f00c8f9385807e1f8d47502ce8cc5d1958b`。
- 正式 run 的 `manifest.json`、`checksums.txt`、`commands.tsv` SHA-256 分别为 `91beb06087fc5876c9ffde3062c2c9bebbf06b65aa5a90325f2cde8ef2a5b8fc`、`4e874fcbf71f64ad67c930f8947a4e019b77325f93f630d9cc02596a8acdd112`、`7f7854df23adde5f9082978f2913c43ac98284bcdf49e5a66a87346d8c78b0be`。完成后无残留测试容器、网络、临时秘密、占用端口或 verifier lock。
- PostgreSQL 官方镜像声明的数据卷没有被旧 cleanup 命令带走，宿主审计发现本 run 留下一枚 66,318,301-byte 匿名卷。确认创建时间属于该 run、无容器引用且处于 dangling 状态后，已按精确 volume ID 删除；独立 host audit 的 JSON/checksum SHA-256 为 `393f911fdda1c54491d9a59e79f28ee452ec8f11c0b81776dd547dd4fd037200`、`4333d38e64556a233ed63906803a3f34924013f3528263a5f2f380752d688e6d`。后续提交会把 PostgreSQL cleanup 改为同时移除匿名卷，免去宿主补扫。
- WP02-C1 至此收口。C2 的持久化 root-key canary/keyring、恢复码组与一次性回显，C3 的统一 challenge，C4 TOTP、C5 WebAuthn、C6 高危 use case 接入和 C7 完整对抗矩阵仍在推进；358 项产品需求保持 `planned`，不会用一个认证纵切冒充妙妙屋 X 全功能完成。

### 2026-08-26 — WP-02-C1 历史应用与门工具候选记录

- 在已正式验收的密码登录/session 基线上实现 C1：登录成功可透明升级旧 Argon2 PHC；`POST /api/v1/auth/reauth` 成功只轮换当前 session，失败 proof 不 touch；`POST /api/v1/me/password` 在一个事务中写新 PHC、清 `force_password_change`、推进 `auth_revision`、撤销全部旧 session 并签发唯一 replacement；活动 session 可以列出、逐个撤销、退出其他或全部退出。rotation 全部继承原 absolute expiry，session/CSRF token 同时更换且无 grace period。
- application 增加显式 `AuthenticatedAction` allowlist。强制改密时，现有受保护用例只允许自身身份读取、近期认证、改密、管理自身 session 和退出；声明为 `ProductAccess` 的调用会返回 `PASSWORD_CHANGE_REQUIRED`。普通产品 API 尚未进入后续工作包，C6 接入时还要逐个声明动作并修正正常 `ProductAccess` 的 touch 策略。Vue router 另有 `allowDuringPasswordChange`/`requiresRecentAuth` guard，`App.vue` 再做 fail-closed DOM gate；这两层只负责界面收口，不替代后端授权。
- 身份库把 PHC 解析限定为资源有界的 Argon2 算法/版本/`m/t/p`/salt/output 组合；接受旧 PHC 的上界已经收紧到当前策略 `m=19456,t=2,p=1,salt=16,output=32`，同时保留有界低成本旧值。未知账号、当前 PHC 和低成本旧 PHC 统一执行“current dummy 校准→所选凭据验证/条件升级→动态 deadline padding”；过长输入也完成两次受限工作后强制失败，permit 覆盖完整计划。停用账号不在失败响应上生成成功 rehash，避免泄漏正确旧密码。新 PHC 每次使用随机 salt，不会得到“相同升级值”。登录事务用 user revision、auth revision、改密时间和旧 PHC 做 CAS；若并发赢家已经写入另一条符合当前策略且仍能验证同一口令的 PHC，loser 可继续创建自己的 session；并发改密或账号状态变化则拒绝建立 session。
- SQLite/PostgreSQL 新增 `0004_recent_auth_password` 与 `0005_session_rotation_timeline`。repository 增加 read-only session authentication、current rotation、改密全撤销+replacement、logout-all 两种事务、active session 查询和 actor-aware 逐会话撤销；每类写事务复核与其安全语义相符的稳定 snapshot。普通 touch 会推进 session revision，`logout-all(false)` 与逐会话 DELETE 因而有意不对这项易变值做 CAS；DELETE 在同一事务锁定并重验调用 session、用户、user/auth revision、recent-auth 和时间线，再决定目标更新与事件。普通 touch 后仍成功，调用方并发 rotation/revoke、认证版本变化、用户停用/变更或时间回拨则拒绝且不改目标。一般认证、active 列表和 rotation 当前会话选择器也统一为 `last_seen_at_ms <= now`，墙钟早于 session 时间线时 fail closed；撤销时间仍取 `max(created_at, now)`。
- API 增加 reauth、改密、session 列表/撤销、logout-all。认证响应、204 与 Problem 均为 `Cache-Control: no-store`；Cookie `Max-Age` 取配置值与剩余 absolute lifetime 的较小值。跨标签页审阅已废止“`SESSION_INVALID` 自动清 Cookie”：所有 Problem 现在都零 `Set-Cookie`，避免旧请求的迟到 401 清掉新 rotation；只有路由显式允许的成功响应可签发、轮换或清 Cookie。OpenAPI 明确两枚 Cookie 是两个独立 `Set-Cookie` header field。安全审阅后把撤销端点 operationId 改为 `revokeCurrentUserSession`；2026-08-26 已重新导出 12 paths/13 operations 的 OpenAPI，SHA-256 为 `b30934dac8c52d1cdbae0dca470e2ba3b4a44785b8f63ccd4b0484df73254596`。固定 Node/pnpm 生成器报告 4 个顶层输出项，递归展开 `apps/web/src/api/generated` 后共有 16 个物理文件并逐字节零漂移。该 hash 代表本次导出输入，不替代尚待完成的提交级正式验证。
- Pinia 用单个 actor/session snapshot 接受 rotation。重新认证、改密或 logout-all 遇到传输中断、畸形 200 或任何 5xx 时，客户端都不重放原 mutation，也不再用 `/me` 的 session ID 猜测成功；数据库 `COMMIT` 报错本身无法证明事务未提交，因此即使响应是 `503 AUTHENTICATION_UNAVAILABLE` 也必须读取最新 CSRF 尝试幂等退出。只有明确 204 才解除本地隔离；401、5xx 或 cleanup 传输失败都保持 sticky `relogin-required`，不会把未知结果假报成登出成功。账户安全页用 deadline timer 与 `visibilitychange` 同步 recent-auth UI。
- 会话列表本身只读取粗粒度时间线，不要求 recent-auth；逐会话 DELETE 则以服务端 freshness 为权威。提交前前端终审发现 DELETE 的 `403 RECENT_AUTH_REQUIRED` 曾被泛化成 `request-rejected`，本地时钟落后或边界竞态时页面无法进入 step-up。store 现在先映射稳定 code，页面只跳转一次且不重放 DELETE；新增 store snapshot/单次调用和页面导航回归，已进入下述 226-file 精确快照并通过 VPS 门。
- 早期开发候选 `/opt/nodecontroll/dev/wp02c-c1-20260826t0342z-003/source` 曾在固定 VPS builder 中通过 Rust fmt/check、73 个 SQLite/PostgreSQL workspace tests、Clippy `-D warnings`；Vue typecheck、零 warning lint、49/49 Vitest 和 361-module production build；OpenAPI 12/13、文档 358/358/0 broken links、source sanitizer，以及 SQLite/PostgreSQL 真实 Master smoke。它覆盖双 session、失败/成功 reauth、sibling 保留、列表/撤销、改密、旧密码/session 失效、新密码/replacement 生效、logout-all keep/clear、Max-Age 与 Problem status/type。该历史候选还验证过现在已废止的 Problem 自动清 Cookie 行为，不能代表现行跨标签页合同。
- 该早期候选的 SQLite/PostgreSQL runtime log SHA-256 分别为 `6f2e2288dbe8446fde262d55dbcc1df432f9edf4ce0b078402dcafb453489fe5`、`8a32697f7833ee110af062ebfd6e9b68bdbe360ac285b98375db70e3935bf31f`，按当时真实测试 secret 扫描均为零命中。之后的安全审阅又移除了未知 logout-all 的错误成功推断，补了一般 session 墙钟回拨 fail-closed、`account_hmac` 仓储校验、独立 `session_revoked` 审计语义、明确的 operationId，并开始仓库化真实 HTTPS Playwright 门；这些后续更改尚不在上述统计和 hash 内。
- 早期候选目录在初始上传后做过增量同步，初始 archive hash 不能代表最终源码，也没有 run manifest/命令日志；当时只能登记“早期开发候选曾全绿”。后续证据见本节末尾：v4 只作为历史应用代码与运行时证据，v6 则绑定随后形成的门工具候选；两者仍不等于公开提交、Actions 制品或 fresh-clone formal provenance。C2～C7、WP02-D/E 和 358 项产品需求仍未完成。
- 新一轮迭代候选以初始 source archive SHA-256 `cf13577fa4742017150642b4072e306645ee477c73c27a6ed22fbfd94c83adb9` 上传到 `/opt/nodecontroll/dev/wp02c-c1-20260826t0630z-001/source`；之后只用于增量查错，因此这个 archive hash 同样不代表候选目录后续状态。Playwright 1.62.0 的锁项只在 VPS 的 Node 24.19.0 / pnpm 11.24.0 builder 中解析，依赖闭包由 461 项变为 465 项并通过仓库供应链策略。较早增量状态观察到 73/73 Rust tests；并发修正和约束语义测试加入后，新的 fresh PostgreSQL/SQLite run 为 74/74，Rust fmt 与 Clippy `-D warnings` 也通过。Vue typecheck、52/52 Vitest、361-module build、OpenAPI/SDK/文档验证仍是本候选已观察结果，但这些增量数字不是提交级 provenance。
- 两项 P1 后审修正又进入同一增量候选：登录端消除低成本旧 PHC 错密路径与 current/dummy 的工作量差异，逐会话 DELETE 则把调用方资格复核移入目标事务。225 个源码路径的同步 archive SHA-256 为 `033b7c0f938f51d57d55806e152c80ab64c9034287bffd14cd3047f51711e6db`；它只描述该次覆盖输入，不描述含缓存与日志的整个增量目录。固定 Rust 1.98 builder 随后通过 `cargo fmt --all -- --check`、78/78 workspace all-targets tests（同一合同覆盖 SQLite 与真实 PostgreSQL）、Clippy `-D warnings` 和 release bins。Master/Agent SHA-256 分别为 `45e4f98d0bdced80a26b143e860a8217a8471e6d6c7d2e99dfacd5a06d1f9f71`、`a72215e93f6634794f8e35ef8fb52845f4a1c975b40bfa446cd20dcf3fd3b0cd`。这仍是增量开发验证，不替代全新精确 run root、公开提交或正式制品。
- P1 后的独立锁图审阅又发现 PostgreSQL 同用户 A/B session 互删可能各持有 actor 行后等待对方。稳态认证写现统一为 `user_auth_state → users → auth_sessions`：actor-aware DELETE、登录/透明 rehash、touch、rotation、改密和两种 logout-all 都先取得用户级 auth-state barrier；touch 的第一次 token 查询只用于找候选 user ID，随后在锁内重新执行完整授权校验。普通 logout 仍只锁 session，之后不再取 auth-state/user 锁。共享双库合同加入 A/B 互删、touch/DELETE 和 touch/rotation；互删只允许一方成功，恰好一条事件，最终一条 active、另一条 `user_revoked`，任何 SQL deadlock、lock-timeout 或 revision conflict 都会失败。
- 登录 limiter 同时构造与 reservation 同源的候选 `rate_limited` 事件，repository 在写前核对时间、request ID、key version、账号/IP HMAC 和 UA hash；account/IP/global 只有首次进入 durable block 时在同一事务写一条事件。已有 block、并发 follower 与 Argon2 semaphore 饱和均零 durable 写；事件 ID 冲突会回滚 bucket。通用事件入口现拒绝独立 `rate_limited`，避免绕过事务不变量；双库合同还锁定 `blocked_until-1`、精确等于截止点和下一周期再次封禁。包含这一边界修正的 225-path 增量 archive SHA-256 为 `e96e469399a7a1e12c83e6be96ff81043717397dbde56b0ec868d1ee9e067112`；固定 VPS Rust/PostgreSQL builder 已通过格式检查和 78/78 workspace all-targets tests。该快照仍待迁移失败回滚、Clippy/release 与全新正式 run，不是发布 provenance。
- C1 迁移回滚门随后补齐四条独立合同。SQLite 分别在 0003→0004、0004→0005 的新表复制晚阶段用外键阻断 `DROP TABLE`，逐项核对版本、完整行、原表 DDL、显式索引和无 staging 残留；移除阻断后用同一 embedded migrator 重跑，并验证新 reason、索引和 `webauthn` 转换。PostgreSQL 0004 以 poison row 让新 CHECK 的回扫失败，0005 以只拦截 `webauthn → phishing_resistant` 的 trigger 在前置约束修改后失败；两者都精确比较回滚前后的行、索引、全部约束定义及 validated 状态，再关闭旧连接池，以新进程连接池完成重试。审阅同时发现两条旧回滚合同的 PostgreSQL 分支也暗含“同池恰好复用持锁连接”的假设；它们已采用同样的重启边界，避免 SQLx 失败路径遗留 session advisory lock 所造成的偶发等待或超时。
- 上述迁移阶段代码在写入本条记录前形成了 225-path 精确 source archive；本地与 VPS 逐文件 SHA-256 清单均为 225 项且零差异，archive SHA-256 为 `c31ab821ed0df8e73f4e6fd554fd6bc9586d8bdedf3a21df5ab7889b17e34344`。固定 Rust 1.98.0 与真实 PostgreSQL builder 通过 `cargo fmt --all -- --check`、78/78 `cargo test --locked --workspace --all-targets`、Clippy `-D warnings` 和 `cargo build --locked --release --workspace --bins`。test/Clippy/release 日志 SHA-256 依次为 `a2378d55dc76d2d5c0689310edb0a152d85496b7e87cec82b8caff64be9b64ff`、`e9c20416b6b668b7eebb642818c4e8d190cd9002cd9decabec95202840f26628`、`e00e793be3c4f60bce84db79b6382bb194622acd994e1ef9c5d7fd68b196f30d`；Master/Agent 为 `f0ba91073af186dcf356c2dd659b904ea0732bc5c1d9486ca7686642d957330f`、`a72215e93f6634794f8e35ef8fb52845f4a1c975b40bfa446cd20dcf3fd3b0cd`。测试 schema 清理后无残留，测试容器和专用网络也已按精确名称删除。该 archive 生成于本条账本更新之前，且仍属于增量目录；它不包含下一条前端收口修正，也不替代新的 run root、公开 commit、Actions 同 SHA 制品或 fresh-clone formal provenance。
- 公开提交前的前端审阅发现两处相连的 fail-open。其一，store/API 允许直接撤销当前 session，若响应在服务端提交后丢失，旧实现会保留 authenticated snapshot；`revokeSession` 现在请求前冻结“目标是否当前 session”，未知结果据此执行幂等 fail-safe logout，其他 session 的未知结果不误伤当前登录。其二，较早发出的 reauth/改密/keep-current 响应可能在普通 logout 或另一条 fail-safe logout 之后晚到，再次调用 `acceptAuthenticated`。Pinia 身份快照现带单调 generation：每次身份/状态替换都推进；refresh 与 rotation 只接纳各自请求 generation 仍匹配的投影。login 等待 exclusive lease 时，以自身写入 inflight 造成的精确 `+1` 为正常；若 generation 被额外推进，即使随后收到 401/429，也保持 sticky quarantine。只有结构完整、运行时校验通过且 generation 仍匹配的 200 登录投影可以恢复。晚到 rotation 200 进入 `outcome-unknown` 并再做一次 cleanup，不能复活受保护 DOM。普通 logout 与 `requireReloginAfterUnknownMutation` 在网络请求前立即关闭投影；后续跨标签页审阅又把未知 cleanup 从临时 unavailable 收紧为持久 `relogin-required`，只有显式成功登录或权威清理 204 可以恢复。此前固定 Node builder 的 63/63 结果属于这一后续收紧之前的增量点；现行测试数字见下方最新记录。根 README 的完成状态和三处过期的 425 npm identity 文案也已纠正，现行门改为与每次 artifact inventory 的完整集合双向相等，历史 run 数字保留。
- 一次静态审计曾把 PostgreSQL 0003 的自动约束名判断反了；把 0005 改成删除 `auth_sessions_authenticated_at_ms_check` 后，真正的 fresh PostgreSQL 立即以“约束不存在”拒绝迁移。随后在只应用 0001～0004 的探针数据库查询 `pg_constraint`，确认跨列认证时间约束是 `auth_sessions_check`，status/revocation 配对约束是 `auth_sessions_check7`。0005 因此恢复为删除并重建前者，后者始终保留，并新增语义约束测试避免以后再次依赖名称猜测。另一次并发审计发现 `logout-all(false)` 对旧 session revision 做 CAS 时，会把合法的并发 touch 误判成 session 失效；该 CAS 已移除，双库测试保存旧 snapshot、触发独立 touch，再证明全量退出仍撤销两条 session 且只写一条审计事件。该候选在当时仍须通过真实 HTTPS 与正式制品门才可收口；正式结果见本节顶部的新记录。
- 真实 HTTPS Playwright 门已经进入仓库。独立审计发现并修正了 server 清理可悬挂、12～15 字符密码可能绕过明文扫描、六类证据路径可被无关文件冒充、扫描后目标仍可变化等闭包问题；七类目标现绑定到 run root 的规范路径，并采用 behavior-ready → 外层停 Master/备份 SQLite/生成 dump/冻结目标 → scan-ready → 浏览器持有秘密完成扫描的两阶段协议。第一次真实 VPS 行为已到 behavior-ready，但冻结快照仍保持 WAL journal，dump 的只读连接产生 `-wal/-shm`；门拒绝 sidecar，排查又超过 120 秒，浏览器按合同超时失败。修正为快照先切回 DELETE journal、dump 使用 `immutable=1`，并在 dump 后再次拒绝 sidecar；该失败 run 及其测试秘密随后按精确路径删除。
- 第二次候选 run `20260826T052551853287390Z-wp02c-candidate` 已完成真实 Chromium HTTPS 登录与 reauth rotation：旧 session 为 401，旧 CSRF 为 403，新 CSRF 通过后错误 proof 为 403 且零 auth `Set-Cookie`、Cookie 与 session projection 不变，新 session 为 200；两枚 Cookie 均为 Secure/SameSite=Lax，session 为 HttpOnly、CSRF 非 HttpOnly，rotation 不延长 absolute lifetime；DOM/URL/console/request/storage 均无凭据。门扫描并二次稳定性比对 33 个文件、10,027,952 bytes，覆盖 ELF、SQLite、dump、OpenAPI、Master log、TLS/attestation 与 production Web；宿主 validator 重算七个 tree hash 并通过。evidence SHA-256 为 `0fed24fab2edcaa0fce5d89c8291970f6b5785c2be0c4f8866ee2a894dad4bfe`，run checksums SHA-256 为 `3d3dcf30c488ff547536f88929d8c4df389c330657cf6414e23a05b4929b6088`；测试秘密、live DB、容器和网络已删除，只保留冻结的无明文秘密候选证据。该 run 的 `sourceRevision` 只是增量树的公开基线 `ecd8dea…`，不是当前未提交工作树的完整绑定，因此不能当作正式 provenance。
- 独立后审继续收紧宿主边界：validator 现在精确绑定 `/evidence`、三枚握手文件、七类目标的 file/directory 类型、规范路径、互不重叠关系、根身份和所有叶文件只读位；同时拒绝最终及临时 SQLite WAL/SHM/journal。失败 secret scan 会先精确删除污染日志或 evidence，再留下无秘密 marker；正常完成态还必须证明 marker、临时目录、容器和网络全部不存在。加固后的 validator 在 VPS 对上述真实证据重新计算仍为 33 files、10,027,952 bytes；两个隔离副本负向回放分别把 `compiled/web/index.html` 恢复写位、增加 `browser/database-journal`，均被硬拒绝，临时副本随后删除。
- 加固后的第二版 HTTPS 候选 run `20260826T063126155736337Z-wp02c-candidate-v2` 还覆盖普通 logout 的两段故障语义：注入 503 时不得清 Cookie 或伪造成功，但必须清内存投影并关闭受保护 DOM；随后真实 204 logout 清 Cookie，旧凭据再请求为 401。冻结证据仍为 33 个文件，共 10,028,822 bytes；evidence SHA-256 `0bb41ac31d1713338caa4dad33f0d65f4133f6668c062f9eb5dfcf647d75755f`，checksums SHA-256 `7b8670567fbe1be9c5b19ea093d7885362d64afb18e5bf905084b19b6b33c5f0`。除可写叶文件和未声明 journal 外，受控负向 run 还把解码后的 32-byte root key 放入 production Web 目标，门以 `secret material found in scanned artifact` 拒绝。负向副本、测试秘密、容器和网络已经删除；该 v2 run 仍绑定公开基线而非当前未提交树，所以只能作为门本身的开发证据。
- `tools/vps_verify.sh` 已接入固定 Playwright 镜像、Node 24.19 runtime、fresh file SQLite、SAN TLS、两阶段冻结、外层 tree-hash validator、只读 closure、checksums、清理闭包与 manifest schema 3；`tools/verify_auth_e2e_evidence.mjs` 负责宿主重算。完整 formal E2E 仍须等当前树形成公开 commit、Actions 产出同 SHA artifact 后，在 fresh full clone 上执行。
- 跨标签页凭据协调已从“只靠本标签页 generation”升级为持久协议：唯一 localStorage journal 使用随机 epoch 和规范十进制 revision。凭据 mutation 只允许与实际观察到的 inflight 精确相连的 `inflight → settled`；受保护读取的 401 则绑定 observed session 和 base cursor，释放 shared 后只有 cursor 未变才独立持久化 `invalidated`。Web Locks 的 shared/exclusive 临界区覆盖完整 SDK 响应解析与运行时 shape 校验；BroadcastChannel/storage 只唤醒。journal 损坏、同 epoch revision 回滚或同值篡改、跳号、未观察到的 terminal/epoch 替换、crash 遗留 inflight 与未知结果都会进入 sticky quarantine。journal 缺失时，若仍能观察到 CSRF Cookie，或状态是 authenticated、unavailable、relogin-required，同样隔离；fresh setup/anonymous 且无 CSRF Cookie 时则是合法初始态。显式登录可以建立新 epoch，普通受保护 mutation 不可绕过。Pinia actor/session 和受管理会话列表继续只在内存中，journal 不含凭据或用户投影，`sessionStorage` 保持为空。
- 同一轮后端收紧删除了 `Problem.clear_session_cookies`：所有 Problem 现在不写 `Set-Cookie`，显式成功清理响应仍清两枚 Cookie。历史 v4 226-file 应用代码候选在固定 Rust 1.98.0 与 fresh PostgreSQL 中通过格式检查、78/78 workspace all-targets tests、Clippy `-D warnings` 和 release；fmt/test/Clippy/release 日志 SHA-256 分别为 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`、`73ddaff852fe9eacbec25764779d38ed6a4bfa686e9c8f4d874ac8be108e3f24`、`2bc683fb4e4f4f21ee9ee39e2b8ecce2efe20d3b1be57bda5316aa519ab3d30a`、`6b1c8cbe0bd9cb3b12baa322a9182419af04da4050d3affc7c9798b1b46e30b5`。release exporter 与仓库 OpenAPI 同为 `b30934dac8c52d1cdbae0dca470e2ba3b4a44785b8f63ccd4b0484df73254596`。固定 Node 24.19.0/pnpm 11.24.0 门通过生成器 4 个顶层输出项、目录内 16 个物理文件零漂移、typecheck、零 warning lint、9 文件 81/81 Vitest、362-module production build 与产物校验；typecheck/lint/test/build/artifact 日志 SHA-256 分别为 `aacdfedc485449151e8f0cedfb8264a7d15047256a7b3c3a4a9e848e28e7b178`、`b543827ce56a63357c629028555ecec2cc964dda33dbbf2a9a2872b1a2ab20f8`、`ec10a8d768f65e49bbd7ee162f10de04764dc7a3375fc58a8664a7c89c2b5203`、`c7318b67329e3309f741bc0eace71c68a77d5d3f66a18a226dfa1de9f4fab1ca`、`26aac325c5633ddbeb822af5aab515fa423cad1315bf405545726c07a5eee7f3`。这些 hash 只绑定 v4；Vitest 覆盖畸形真值 200、锁服务失败、读写锁顺序、401 跨 store 条件失效、late join、epoch 替换、revision 回滚/同值篡改/跳号、queued login 外部推进和持久 quarantine。
- 双页 HTTPS 候选 run `20260826T112446112732525Z-wp02c-candidate-v4` 已在固定 Playwright 1.62.0、外置 Node 24.19.0 与 Chromium 151.0.7922.34 上通过。它用同一 browser context 的两个真实页面验证：旧凭据 `/me` 401 零 `Set-Cookie` 且不改旧 Cookie；peer logout 503 关闭两页受保护 DOM，quarantine 跨 reload 保留且不自动 `/me`/login；显式登录恢复两页；绑定旧 cursor 的迟到 invalidation 已实际送达，但不得覆盖新 journal/session/DOM；只有真实 logout 204 清两枚 Cookie。冻结扫描覆盖 33 个文件、10,087,567 bytes，evidence SHA-256 为 `9db50f56137bd905310771e4d1fb82bffdada29780df4a2ce4c3a9297586fe5f`，gate/validator SHA-256 分别为 `459eabe176e90e20ac9bda5845cb943c176a505ecc2aace96b62f1f38c96577c`、`c2df67a7f2c2a469a3a70f933046e9cc04b142bf94efdd04f67a205c16e63327`。当前 validator 又在断网只读容器中独立复算 33 个目标，日志 SHA-256 为 `55ff874ab73ac6b97bb1169522edc08392a564d34065ca0d8916188da43a8550`；测试秘密、live DB 和容器均已删除，browser 证据子树以 0400/0500 封存，七类扫描目标及叶文件均已去除写位。此前 v3 的两次启动分别因 Node runtime 挂载错误和镜像 ID 形态错误在业务前拒绝，第三次因人工分段超过 scan-ready 窗口而 fail-closed；失败资源均按精确路径删除。该浏览器 run 仍以 `ecd8dea…` 作为未提交增量树的公开基线，不是当前完整 tree 的提交级绑定；公开提交、Actions 同 SHA 制品和 fresh-clone formal provenance 仍待完成。
- 冻结源码的提交前审计又发现 `tools/smoke_master.mjs::expectSessionInvalid` 仍沿用已经废止的“401 清 Cookie”断言。它会让现行正确后端在正式 runtime smoke 中被误报失败。该 helper 已改为要求 generic `SESSION_INVALID` 的 `Set-Cookie` 数量严格为零；`logout-all(false)` 与当前 session logout 的显式成功 204 仍分别要求恰好两枚清理 Cookie。修正后的脚本先由锁定 Prettier 规范化，再随历史 v4 应用代码候选在 SQLite 与 PostgreSQL 分别跑通同一合同。最新 v6 门工具候选又使用与 v4 逐字节相同的 Rust/runtime/smoke 输入重跑两库；两个 smoke 输出仍逐字节相同，SHA-256 都是 `2a4549206f74fec53374e4d92c175135b1310b7fb638cffc9f666eddf50dac14`，新 SQLite/PostgreSQL Master 日志分别为 `c21fa4438eb8040ec8928f3eb8c86bccb6040756baf2e386ce927fb2ba9a1408`、`b63d17c7f9d821f62e162117eeced01f4cc31821519c22ead518531aa8414386`。随机 root key、setup token、PostgreSQL 密码和 smoke 口令均零命中，fixture、数据库、容器与网络已删除。
- 历史 v4 应用代码候选位于 VPS `/opt/nodecontroll/dev/wp02c-c1-freeze-20260826t1130z-004`。源码归档严格包含 226 个工作树文件，archive SHA-256 为 `0c2eb2a94256ee3b5795e9811b2833090989244c5d8370a9ce05beb8f5fabe5c`，逐文件清单 SHA-256 为 `772172f85e38d94ba67846f66252a588631ecb6a342566af1949357a5a7f61dc`；31 份日志清单 SHA-256 为 `320b1e2039ea0675b758c7b6c763f888c10aae3f8f7b9d880a057ec56532ab89`。v4 直接归档工作树原始字节并完成当时的 Rust、Node 与双库 runtime 门，但早于 pnpm global virtual store、Actions/VPS 编译边界、formal verifier 和文档修正，只能作为历史应用代码与运行时证据。
- 随后的 v6 门工具候选同样严格包含 226 个文件，archive SHA-256 为 `e2a055daf353da1f6500ba643b7ae75516e900976e02bae3536e44a818a8cb58`，逐文件清单 SHA-256 为 `ee9084efb429f50dbe9e4f8b1a3e133b8f58bb40d2d197a3931ea4d3cf099ec6`。它在 VPS 以 fresh pnpm store 通过 465 项供应链策略和 428 包安装，现场断言 global virtual store 为 `false`；文档/sanitizer/OpenAPI、`bash -n`、拓扑 YAML、16 个生成文件零漂移、typecheck、零 warning lint、9 文件 81/81 Vitest、362-module Web 诊断构建和 artifact 检查均通过。全新 Cargo 输入下两次断网许可证收集逐字节一致，共 650 个组件、858 份证据，fresh virtual store 的 428 个 npm identity 与 inventory 双向相等；SQLite/PostgreSQL 同合同 smoke 均通过并完成 secret/resource 清理。v6 之后又按最新目标把正式 release/Web 编译唯一收敛到公开 GitHub Actions、VPS 只保留测试与同 SHA 制品验收，因此 v6 也不是最终 pre-push tree；公开 commit、attempt 1 Actions 和 fresh-clone formal 仍待执行。

### 2026-08-26 — WP-02 密码登录/服务端会话纵切通过公开 Actions + VPS 正式门

- 新增 `nodecontroll-application`，把 bootstrap、登录、当前身份和退出从 HTTP/SQL 细节中分离；Domain 补齐六角色、用户状态、capability、认证等级与 session 生命周期枚举。SQLite/PostgreSQL `0003_auth_core` 同步增加 `user_auth_state`、`auth_sessions`、`login_rate_buckets`、`login_security_events`，repository contract 在两库使用同一 fixture。
- 登录在任何 limiter 写入前取得 1～64 的进程内许可，许可覆盖三层 bucket、凭据读取与 Argon2 验证。已有 account/IP/global 封禁先由一条只读查询拒绝，不更新 blocked hit，也不会随着轮换账号或 IP 扩张 row；未命中仍由 account→IP→global 固定锁序事务处理并发。不存在用户验证 dummy PHC，停用、未知、错密对外统一 401。
- 原始 session/CSRF token 只进入 `Secure`、`__Host-` Cookie；数据库保存用途隔离、带 key version 的 HMAC。`/me` 检查 active 用户、`auth_revision`、idle/absolute deadline；写请求同时检查 canonical Origin/Host、严格 Cookie/header CSRF 与数据库 CSRF HMAC。mutating auth 已收敛为单次 typed repository outcome，错误 CSRF 不 touch session；logout 撤销与安全事件在同一事务中提交。
- Vue 增加内存 Pinia session 状态机、登录页、路由 guard、SaaS authenticated shell 和 fail-closed 控制面门。重定向拒绝外站、反斜杠、编码控制字符和 guest route。Setup 成功前会清空 setup token/密码；登录成功但导航失败时清空密码、禁用再次登录并只允许重试导航；退出后即使路由失败，旧受保护 DOM 也会立即消失。
- 固定 Rust builder 的候选预检通过 `cargo fmt --check`、`cargo check --locked --workspace --all-targets`、68 个双库 workspace test、Clippy `-D warnings` 和 release bins；精确工具链仍是 rustc/cargo 1.98.0、rustfmt 1.9.0、clippy 0.1.98。最终 OpenAPI SHA-256 为 `e2d8316e7f8c7543328d03044a24bf621461144fa2043f3b4562fc1c29af6280`，共 7 paths/8 operations；VPS 生成 SDK 后 typecheck、零 warning lint、6 个文件 29/29 Vitest 和 341-module production build 全绿。
- 真实 PostgreSQL runtime 已完成一次性 bootstrap、重复初始化拒绝、登录、刷新恢复、退出、撤销后 401；Master 以同一数据库和 root key 重启后，再次登录/恢复/退出也通过。重启前后日志分别为 205/376 bytes，SHA-256 `63f0035d22a8e9d58443d1d60c495a12e8c56ae7e4544bd396aa5766f4256347` 与 `979649eacfc8d555e09996a8625bb9322642e907a385b57b2923060c8ece66d1`；按真实 setup token、root key、测试口令、PHC、session/CSRF 前缀扫描均无命中。临时 Master/PostgreSQL 容器、网络与秘密文件已删除，只保留不含秘密的候选日志。
- 发布前审计修正 verifier 边界：runtime log 先写 `.capturing`，捕获成功后才原子替换并扫描；捕获失败、secret fixture 缺失或扫描异常都会留下失败 marker。扫描器改由 Python 按文件路径在进程内读取秘密，秘密正文不再进入子进程 argv。Cargo fetch、test、release 和许可证重建容器都显式固定 `RUSTUP_HOME`，与断网只读工具链预检一致。`bash -n` 与精确版本预检已经在 VPS 通过。
- 公开 commit `ecd8deaecd6dcfad8fd365dada67e5fc487046ad` 已只推送到 `main`。Actions run `32919113045` attempt 1、job `98028915315` 在 3 分 18 秒内全绿；artifact `9589258880` 为 4,569,681 bytes，GitHub API、本机下载和 VPS commit-scoped 文件的 SHA-256 均为 `1798c3746856f33deef36fe657ccd70294916ce0d6662d0c247827425ecc7b40`。公开仓库仍是 `public`、唯一分支 `main`、0 tag；临时 write deploy key 在远端确认 SHA 后已删除，VPS 私钥、bundle 和临时 push clone 均已清理。
- fresh full clone 上的正式 run `20260826T013832936716270Z-p5` 从 `01:38:33Z` 到 `01:42:24Z` 完成。manifest `status=completed`、无 `FAILED_STAGE`/`SECRET_SCAN_FAILED`，并记录 full clone、ignored inputs absent、one-time claim、`source_checkout_clean_after_tests=true`。正式门核对 211 个 tracked blob/mode、1,561 个规范 archive member、882 个 package 文件、647 个锁定组件、852 份许可证证据、20/20 overrides、CycloneDX 1.6，以及 856 个 notices 文件逐字节复现；Cargo/pnpm lock SHA-256 分别为 `5e2a9b4df3113f7dc274fd5546bc2d65f59fa6e499b48761b527e94f8a98ffa9`、`554d9932aa59b372164df94c5e3eed6d2bd1270c1f48e402d54a75a91ad1aef7`。
- 正式 run 重新执行 68 个 SQLite/PostgreSQL Rust tests、零 warning Clippy、断网 release build、OpenAPI 7 paths/8 operations、Web type/lint/29 tests/341-module build；Master/Agent ELF、OpenAPI、完整 Web dist 和 notices 与 Actions 逐字节一致。runtime smoke 的 bootstrap、重复初始化拒绝、登录、`/me` 恢复、退出与撤销后 `SESSION_INVALID` 全部通过，request ID 唯一；最终 Master log SHA-256 为 `fc251e6300a5e1da26717f2bf33df0cd37311246736048ac9b66d1ba7a64a2e2`，secret scan 通过。run 结束后 root key/setup token、容器和网络均不存在。
- 这只把该 SHA 的密码登录/服务端会话纵切绑定到公开制品，不代表完整 WP-02。该正式 run 当时没有覆盖 MFA/WebAuthn/recovery/recent-auth、token、完整 RBAC/用户生命周期、浏览器 E2E，以及 bucket/event/session retention 和持久化 key canary；358 项需求继续全部保持 `planned`。后续 C1 开发候选状态以上方新记录为准。

### 2026-08-26 — 首个公开 SHA 通过完整 Actions + VPS 正式门

- commit `190492823da766d7446375f05b517d6359fb0d72` 的 Actions run `32905833325` attempt 1 全绿，artifact `9584823840` 为 4,165,197 bytes；GitHub API digest、本机 raw payload 和 VPS commit-scoped 文件的 SHA-256 都是 `6ed502ab5cc94b6ac3c19404654e28462864a8e7fb5f14682f287a4b2129f65a`。
- 一次 SCP 在 3,899,392 bytes 处中断；不完整文件没有进入 verifier，而是移到 VPS 临时目录保留。完整文件先传到 `.uploading`，核对 size/hash 后才原子改成正式名称。新 archive 预检仍是 676 个 `0755` 目录、868 个 `0644` 普通文件和两个 `0755` ELF。
- fresh full clone 上的正式 run `20260825T223046660411478Z-p5` 于 `22:30:46Z`～`22:34:23Z` 完成，manifest `status=completed`、无 `failed_stage`，并记录 `source_checkout_clean_after_tests=true`。Rust/Node/PostgreSQL builder 分别固定为 `6ab618...1613`、`066286...afcb`、`1c59e2...e1af`。
- 正式门逐项通过：GitHub provenance；196 个 tracked blob/mode；1546 个 archive member；869 个 package 文件；645 个锁定组件、844 份许可证证据、20/20 overrides 和 CycloneDX 1.6；34 个 Rust test、零 warning clippy；Master/Agent ELF 与 VPS release 逐字节一致；OpenAPI 在提交、Actions 与 VPS 三方一致；848 个 notices 文件逐字节复现；Web typecheck、lint、13 个 Vitest、324-module production build 与 Actions `dist` 精确一致。
- 真实 Master smoke 验证了 liveness、readiness、一次性 bootstrap、重复初始化 `ALREADY_INITIALIZED`、版本/API 身份、4 条 OpenAPI path 和唯一 request ID；runtime OpenAPI 与 package 合同一致，日志 secret scan 通过。这个结果只完成当前工程骨架、foundation/bootstrap slice 的公开发布门，不代表 WP-02 身份/session/MFA/RBAC 或 358 项产品需求已经完成；正式 run 的矩阵仍为 `planned=358, implemented=0, verified=0`。

### 2026-08-26 — mode 合同通过；ELF loader allowlist 缺项被正式门拒绝

- commit `332b204b47d418513e4f9e5850921f744762038a` 的 Actions run `32904404331` attempt 1 全绿，job `97985122463` 用时 3 分 1 秒。artifact `9584322069` 为 4,165,204 bytes，GitHub API digest 与下载 payload SHA-256 都是 `f31986a99336a80000d5d8345afceec6d5509f96c60dccc6805e1d2f39c262ea`。
- VPS 预检确认 archive 有 676 个 `0755` 目录、868 个 `0644` 普通文件，只有 Master/Agent 两个 ELF 为 `0755`。上一份 artifact 相对新 exact-mode 合同共有 19 个 evidence mode 需要归一化：14 个可写的 `0666` 文件触发了当时的正式拒绝，另有 5 个不应执行的 `0755` evidence；新包中两类都已消失。两个 ELF 都只包含 `/cargo-home/registry`，不再包含 `/usr/local/cargo`、Actions workspace、runner home 或 VPS target 路径。
- fresh full clone 上的正式 run `20260825T221048400560496Z-p5` 通过了公开 checkout、196 个 tracked blob/mode、GitHub run/artifact provenance、CycloneDX CLI、1546 个规范 archive member、869 个 package 文件、645 个锁定组件、844 份证据、20/20 overrides 和 CycloneDX schema，随后在 `actions-elf-check` fail-closed。
- 失败原因已经缩小到动态库 allowlist：Master 为 `__tls_get_addr@GLIBC_2.3` 合法声明了 `DT_NEEDED ld-linux-x86-64.so.2`；固定 builder 的 loader SONAME 与程序解释器都是这个精确名称，`ldd` 也已解析成功。verifier 原先只允许 libc/libm/libgcc 等库，漏了 loader。本轮只加入该精确 SONAME，未知或未解析的动态库仍然拒绝；同时不再用无法传播 `readelf` 失败状态的 process substitution，并明确拒绝空 `DT_NEEDED` 集合。

### 2026-08-26 — Actions 首次全绿；正式 VPS verifier 拒绝不安全 mode

- push SHA `7fd836d9fc73a66fe89ebbd3da131506cbe2f7b8` 的 Actions run `32901899767` attempt 1 在 2 分 49 秒内全部通过：Rust release、OpenAPI 导出/校验、Node/pnpm、公开分析边界、依赖安装、645 组件/844 证据收集、SDK/typecheck/Web production、generated drift、tracked source/工作树闭包、确定性 package 和 artifact 上传。artifact `9583470546` 为 4,165,254 bytes，GitHub API 与下载 payload 的 SHA-256 都是 `405626249d443586fce002d58dfc106455134afaa661de45b37ff10ad5e00039`。
- 这份 artifact 没有被登记为发布门通过。VPS fresh full clone 上的正式 run `20260825T214246348535211Z-p5` 已先验证公共 `origin/main`、196 个 tracked blob/mode、GitHub run/artifact provenance 和固定 CycloneDX CLI，随后在 `actions-archive-members` fail-closed：Rust sysroot 的 14 个证据文件继承了 `0666`，违反包内普通文件不得 group/world writable 的合同。
- 收集器现在在所有内容与 checksum 生成后递归拒绝 symlink/特殊文件，并把生成目录统一为 `0755`、普通文件统一为 `0644`；证据 bytes 与 SHA-256 不变。下一份 Actions artifact 会在 VPS formal run 前先检查 archive mode 分布。
- 同时提前消除尚未走到的 ELF 可复现性阻断：成功 artifact 的 ELF 含 `/usr/local/cargo/registry/...`，而 VPS release 使用 `/cargo-home/registry/...`。Actions 的全新空 `CARGO_HOME` 改为 `/cargo-home`，与 VPS 的 source mount 绝对路径一致；固定 image 中该路径已在 VPS 验证为初始不存在且可以创建为空目录。不使用放宽 `cmp` 或事后改写二进制的办法。

### 2026-08-26 — 公开根提交已发布；Actions 前置环境修复中

- 公开仓库已建立为 `FengYuchen1314/NodeControll`，`main` 的首个 commit 是无父根提交 `607f54b652ec7b3525852cbc4c65441743b8ddce`。远端只有 `main`，没有 tag；author/committer 均使用 GitHub noreply 地址。旧的 5 个私有工作提交没有推送，因为其中保留过 VPS 地址、本机路径、原始网页归档和全文提取。公开根树只有 196 个审计文件，不含 `docs/03-mmwx-gap/evidence/raw` 或 `extracted`。
- 本地到 GitHub 的 HTTPS push 遇到连接重置，改由 VPS 使用一次性 write deploy key 推送只含 `main` 的完整 Git bundle。推送后已从 GitHub 删除该 deploy key，并删除 VPS 上的私钥、bundle 与临时 clone；该凭据不可恢复，也没有进入 commit 或 run artifact。
- 首次 Actions run `32901202453` 对应根 SHA、event=`push`、attempt=1，但在第一个源码 pinning step 因 job container 与 host checkout 的 UID 不同触发 Git `dubious ownership`，尚未进入 Rust 编译，也没有 artifact。修复不放宽源码验证：workflow 改用进程级 `GIT_CONFIG_COUNT/KEY/VALUE`，把唯一 `safe.directory` 固定为 `${{ github.workspace }}`，不写全局 Git config。下一次 run 通过前，正式编译和 VPS artifact verifier 仍是未完成状态。
- 第二次 push run `32901554308` 已证明上述 ownership 修复有效：checkout 与 source-pinning step 通过。下一步 `actions/setup-python` 因 Debian 12 job container 不在其 3.13.7 二进制 manifest 中失败，仍未编译、无 artifact。固定 Rust image 本身已在 VPS 断网容器中确认提供 `/usr/bin/python3` 3.11.2；源码校验器不需要 3.13 特性，因此移除不适用于该 container 的 setup action，改为核对 image 内置的精确版本。该运行时受完整 job image digest 约束，不通过 apt 或浮动下载引入。

### 2026-08-26 — 发布前源码、依赖与双端重建门禁冻结

- 最终临时许可证重收集在 VPS `/opt/nodecontroll/tmp/collector-final.<run>` 连续运行两次；两轮均为 645 components、844 evidence files、npm 425、override 20/20、`issues=[]`、`warnings=[]`，目录和文件内容逐字节一致。6 个 SQLx 0.9.0 包各有 MIT 与 Apache-2.0 的完整固定上游证据。CycloneDX CLI 0.33.1 已按固定 SHA-256 校验后通过官方 1.6 schema。该目录没有正式 run manifest，也没有同 SHA Actions artifact，只是发布前证据，不能登记为正式制品门通过。
- Cargo 组件 PURL 只保留规范化 `repository_url`；`Cargo.lock` checksum 作为 CycloneDX `hashes` 写入，不再使用非标准 PURL checksum qualifier。应用组件版本改为从 workspace `Cargo.toml` 读取。收集器拒绝空白证据、相对 pointer stub、越界 realpath、共享但 spec 不完全一致的 override，以及 stale/unused override。
- 新增独立源码校验器：按固定 commit tree 逐个核对 tracked blob bytes 与 mode，不信任工作树 index；即使变更被 `skip-worktree` 或 `assume-unchanged` 隐藏，bytes/mode 篡改 fixture 仍会被拒绝。所有 commit/tree/blob 读取显式禁用 Git replacement objects；伪造 replacement ref 不会改变被验证对象。
- VPS 正式 verifier 的 Cargo registry/git 输入与 pnpm store 都使用 run-scoped 私有闭包。Cargo 先联网 `fetch --locked`，随后只读挂载并只生成测试 target；pnpm 在私有 store 中以 `--ignore-scripts --package-import-method=copy` 安装，再冻结输入清单。Node 测试工具只在隔离 scratch source 中运行，除两棵 `node_modules` 外拒绝额外路径。正式 checkout 还必须是公共仓库唯一 `origin` 的非 shallow、非 partial standalone full clone，本地 `main`、`origin/main`、HEAD 与 Actions push SHA 同指一个 commit。
- 正式 release/OpenAPI/Web 现在只由公开 Actions 构建。VPS verifier 直接验证 Actions 的 archive、ELF、OpenAPI、Web 和 notices/SBOM，然后运行 Rust/Web tests、Master/Agent smoke、runtime OpenAPI、双数据库合同和真实浏览器 E2E；不再重建 release、`dist` 或正式 notices。Actions 工作流仍在构建、打包前后重复核对源码 verifier、replacement refs 和真实工作树闭包。
- `Cargo.lock` 曾因 `nodecontroll-identity` 已引用 `hex`、`sha2` 而落后于 manifest。VPS 上 `cargo metadata --locked` 先按预期失败；隔离离线刷新确认只需给该 workspace package 补入这两个已锁定依赖，因此只提交这两行最小变更，没有接受 `cargo generate-lockfile` 提议的无关传递依赖漂移。fresh pnpm install 另由独立 inventory verifier 确认 425 个已安装 identity 与 notices 双向相等。
- 诚实边界：公开无父 `main`、首次 GitHub Actions artifact 和正式 VPS verifier 尚未执行；358 条产品需求仍全部是 `planned`。WP-02 目前只完成 bootstrap 纵切，登录、session、MFA、recovery code、RBAC、用户管理和浏览器 E2E 都未完成。

### 2026-08-26 — 许可证重收集口径纠正，正式 artifact gate 待执行

- 02:35 记录的 647 components/846 evidence 来自旧扫描口径：它枚举安装目录时纳入了 `.pnpm` 中两个已不在当前依赖图内的 stale package。该计数已撤回，不再作为许可证闭包或发布门通过证据。
- 收集器现从 fresh `pnpm install --frozen-lockfile` 产生的 active pnpm reachable graph 取 npm 组件，只收入实际安装且可达的 package；当前 OS/CPU/libc 未安装的 platform-specific optional dependency 不计为分发组件。Cargo/npm 组件仍须有实际许可证/notice；最终共 20 个精确 override，同时核对 ecosystem/name/version/license、锁文件 integrity、固定上游 revision、本地证据 bytes/SHA-256、realpath 边界及 stale/unused 闭包。Rust 1.98.0 标准库运行时继续从当前 `rustc --print sysroot` 收入 README、标准库版权页和 `licenses/` 全集。
- 2026-08-26 发布前 v10 临时双跑（无正式 run manifest）两轮均为 645 components、844 evidence files，其中 npm 425、platform-specific optional excluded 36、null lock integrity 0、override 14/14、Rust sysroot evidence 14，`issues=[]`、`warnings=[]`；两轮输出目录逐字节一致。这是 SQLx 证据补齐前的历史记录，已由上节 override 20/20 的最终临时双跑替代。
- fresh install 后另有独立 npm 实体闭包门：直接枚举 `node_modules/.pnpm` 中安装包的精确 name/version identity，并与 notices 中 425 个 npm identity 做集合双向相等检查。脏 virtual store 中的额外包、清单有但未安装的包都会拒绝；`node_modules`、`.pnpm` 和包根必须是非 symlink 目录，canonical realpath 必须分别留在 checkout、`node_modules` 和 virtual store 内。
- `pnpm-lock.yaml` 按审阅过的 v9 canonical 顶层结构独立解析：顶层顺序和全集固定为 `lockfileVersion/settings/importers/packages/snapshots`，版本值必须精确为 `'9.0'`，其余 section 使用 block mapping。重复、quoted 或未知顶层 key、非规范 YAML 顶层语法和重复 package/integrity 均拒绝。依赖 repository metadata 必须规范化为 absolute `http(s)`/`ssh`/`git` URI；不能安全规范化或含非法 credential/path 的值拒绝。
- CycloneDX 1.6 除内部字段闭包外，还由固定 CycloneDX CLI `0.33.1` 校验官方 schema；下载文件的固定 SHA-256 为 `bfc8b2538da86fe239bc53658bbb63c1c8c510a293c1e6891aa5bea5d3c58746`，版本或 hash 不符时不运行。
- CycloneDX 1.6 的 `components[].licenses[].license.name` 保存收集器规范化后的包声明许可证字符串，包括复合表达式；它不是已确认 SPDX `id` 的声明。许可证正文、来源、checksum、锁文件 integrity 和精确 override 共同构成法律证据。
- 许可证收集器的断网双跑属于历史发布前诊断；正式 notices/SBOM 只由 Actions 生成。VPS 对 artifact 内 inventory、许可证正文/checksum、CycloneDX schema 和 override 闭包做只读验证，并把 fresh `.pnpm` 的实际 npm identity 与 inventory 双向比对，不再生成第二份正式 payload。该时点仍须等待同 SHA Actions artifact；正式 provenance 后来已由本节顶部记录的 run 补齐。

### 2026-08-26 02:35 — SetupPage 对抗性组件预检；许可证旧计数已撤回

- SetupPage 现在只消费白名单 Problem code 和 JSON pointer，不渲染服务端 `detail/title/message/request_id`；字段白名单使用 `Map`，避免 `__proto__`/`constructor` 命中原型链。成功、失败和网络拒绝都在 refetch/显示错误前清空 setup token、密码与确认密码；409 重新读取 bootstrap，429 只显示解析后且不超过 3,600 秒的延迟。
- Vuetify 组件测试在 Vitest 中显式内联样式依赖，并为 jsdom 提供最小 `ResizeObserver` 测试替身。VPS 观察 `vue-tsc --noEmit`、ESLint `--max-warnings=0` 均通过；完整 Web 测试为 2 个文件、13/13 用例，其中 SetupPage 11 项覆盖 header/body 边界、确认密码门、秘密清理、403/409/429、字段定位、原型键、网络 reject 与不可信回显。
- SetupPage 结果仍只是发布前预检：尚未创建公开无父提交、尚无同 SHA GitHub Actions 制品，也没有运行正式 artifact provenance/runtime verifier；登录/session/MFA/RBAC 与浏览器 E2E 仍未实现，358 项需求状态保持 planned。该时点的许可证 647/846 计数已由上节纠正，不能继续引用为通过结论。

### 2026-08-26 02:10 — Bootstrap 抢占阻断已修复，VPS 预检通过

- 未初始化 Master 现在必须从私有 regular file 载入 32-byte 随机 setup token。进程只保留 digest；默认有效窗口 1,800 秒、配置上限 3,600 秒，成功提交后消费。POST 通过 `x-nodecontroll-setup-token` 传递；缺失、错误、过期或已消费统一返回 403。无效 capability 和字段错误在占用 2 秒尝试间隔与 Argon2 前拒绝，429 增加 `Retry-After: 2`。数据库 `ready` latch 仍是跨重启和多副本的一次性边界。
- Legacy bootstrap 在创建 Owner 前锁内读取 `subscription.behavior`：真正缺失才补默认；schema 不为 1 或 JSON 不能按 typed setting 解析时返回 `InconsistentBootstrapState` 并回滚。SQLite/PG 合同新增 missing、wrong-schema、unknown-field JSON 三条路径。
- API 合同新增必填 setup-token header、403 Problem、请求字段 pattern/length 与 write-only password；SetupPage 增加 token、密码确认和 header 发送。runtime smoke 合同扩为 14 个唯一服务器 request ID，并加入无 token→立即正确 token、响应/最终日志 token/password/root-key/PHC 扫描。
- VPS 预检已观察：`cargo test --workspace --all-targets` 全通过，随后新增的 setup-token symlink 拒绝使 identity 单测增至 6 项并单独通过（API 8、config 3、domain 6、identity 6、Master 1、object store 3、persistence 3、secrets 4；其中 persistence 在真实 PostgreSQL 18.6 和 SQLite 双跑）；workspace Clippy 与新增 identity 目标的 `-D warnings` 均通过；OpenAPI 在 VPS 重新导出，Web SDK 重新生成，Vue typecheck、lint 与当时已有的 2 个 Vitest 通过。SetupPage component 后来已在 02:35 补齐；浏览器 E2E 与 runtime smoke 仍要等待后续测试和 Actions 同 SHA 制品。
- 正式证据链增加 Actions run ID/attempt 内嵌与校验、raw tar 只读快照、一次性且无 ignored 输入的 checkout claim、最终 Master 日志冻结后扫描、失败路径标记。VPS 实测当前 Docker image store 对 PostgreSQL 固定引用的 `.Id` 与 RepoDigest 都是 `sha256:1c59e2...d7e1af`；verifier 同时检查两者。
- 14 个缺许可证正文的精确依赖版本已有 20 份固定来源证据和 hash override；active pnpm reachable graph、stale/unused 检查与 Rust std/toolchain runtime notices 已在后续发布前临时双跑中得到当前口径结果。由于没有正式 run manifest 和同 SHA Actions artifact，这不是 recollection gate 通过结论。

### 2026-08-26 01:33 — P5.2 身份初始化纵切与正式制品链待验状态

- 新增 `nodecontroll-identity`、双库 `0002_identity` migration 和 Owner/instance 原子 bootstrap。当前可完成 Argon2id 密码写入、空库初始化、0001 历史实例补 Owner、singleton latch 关闭与异常状态 fail-closed；登录、session、MFA、recovery code、RBAC 和用户管理仍未实现。
- Bootstrap API/OpenAPI/SDK/Setup UI 已接通。GET/POST 对不一致数据库状态统一返回 `BOOTSTRAP_STATE_INCONSISTENT` 503；Problem media type、request ID 对齐、密码/PHC 不回显、Unicode scalar 与 UTF-8 byte 边界已经写入测试或 smoke 合同。SetupPage component test 后来已在 02:35 通过；真实浏览器 E2E 仍缺失。
- 正式编译链改为公开 GitHub Actions 生成单个 glibc 2.36 raw tar；VPS verifier 只接受同一 `main` push SHA 的 fresh checkout 与 GitHub run/artifact API 可核验制品，并检查 raw tar digest、包内全集、BUILD-METADATA、ELF/GLIBC、许可证证据后再执行双库、Web 和 runtime smoke。
- 第三方依赖收集器的早期 VPS 预跑发现 646 个 Cargo/npm 组件中有 14 个安装包只声明许可证而未携带正文。生成 declaration notice 的临时做法不满足分发门；精确 source revision/hash override 已补齐，最终组件口径以后续 active pnpm reachable graph 的 645/844 临时双跑为准。
- 这一阶段仍无对应的公开 Actions artifact，也未完成正式 `tools/vps_verify.sh` run；后续虽已补齐许可证临时双跑和 SetupPage component test，正式 provenance/runtime 结论依旧未产生。因此不把本纵切记为 WP-02 完成，不更新 358 项需求状态。实现与缺口见 `docs/05-implementation/WP02_IDENTITY_BOOTSTRAP_SLICE.md`。

### 2026-08-25 23:45 — P5.1 typed settings、对象存储与 secret canary 通过

- Domain 新增 schema v1 的 `SubscriptionBehaviorSettings`、external sync/client compatibility 显式枚举；persistence 在 SQLite/PG 双实现 create/update/revision-conflict，旧 revision 更新不会覆盖新值。
- 新增 `nodecontroll-object-store`：lowercase SHA-256 内容地址、固定两级 key、size preflight、同目录 create-new temp、file+directory fsync、atomic rename、dedupe read-back；storage key不能由调用者注入。测试覆盖幂等、限额和磁盘篡改。
- 新增 `nodecontroll-secrets`：用户自有 32-byte root key 文件、Unix 0600检查、Zeroizing、XChaCha20-Poly1305/OS nonce、length-prefixed purpose+owner AAD、key version 和 canary。Master bind 前加载/canary，readiness 独立显示 `secret_store`；客户端只见稳定错误 code。
- `tools/vps_verify.sh` 每轮生成临时 0600 test key、只读 mount 到 Master、结束 trap 删除；历史 VPS run `20260825T154501Z-p5` 已确认临时 key 不存在。其 manifest 对应 pre-public private baseline, intentionally unpublished；该 run ID 仅保留为发布前测试记录，不暗示存在可解析的公开 commit。
- 该 run exit 0：Rust 21 tests、fmt/Clippy；typed settings 在 SQLite+真实 PG 18.6 双跑；OpenAPI/docs/Web/runtime 继续全绿。Cargo lock `b40ca8...e3520`。
- 诚实边界：settings 尚无认证 API/UI；object 尚未接 DB metadata/ref/quota/S3/streaming；secret 尚未持久化/轮换/rewrap。358 项仍为 planned，不把基础库测试升级为产品需求 verified。
- 逐函数与算法说明：`docs/05-implementation/WP01_STORAGE_SECRET_SLICE.md`。下一步是 Owner/instance 原子 bootstrap、settings API、secret/content metadata repository。

### 2026-08-25 23:24 — P5.1 typed config、SQLite/PG 双跑与真实 readiness 通过

- 新增 `nodecontroll-config`：defaults < TOML < `NODECONTROLL__...`，嵌套 `deny_unknown_fields`；loopback 默认；数据库 URL 用 `SecretString`，诊断只显示 `[REDACTED]`。`--check-config` 在连接前只解析配置/DSN，VPS 用只读 worktree 验证不会创建目标 SQLite 文件。
- 新增 `nodecontroll-persistence` 与 SQLite/PostgreSQL 两套 `0001_foundation` migration，建立 instances/settings/secret/content object/reference 基础表。SQLite 强制单 connection、FK/WAL/NORMAL/busy timeout；PG pool 每连接设置 statement/lock timeout。
- 实例 repository 对两库运行同一 contract：empty/migrate、初始化判定、UUIDv7 fixture 写读、重复 bootstrap 拒绝、原值不覆盖。VPS 固定 Docker Official PostgreSQL 18.6 digest `1c59e2...d7e1af`，不再允许 PG 测试因无 DSN 静默跳过。
- `FoundationProbe` 保持 API 与 SQLx 解耦；Master 启动必须先 config→connect→migrate，再 bind。`readyz` 实时 `SELECT 1`；`bootstrap` 读取实例表；未知路由返回 `application/problem+json`/`ROUTE_NOT_FOUND`；成功 envelope 带传播后的 request ID。
- Vue 新增 `/setup` 只读初始化投影，System 增加 15 秒 readiness 投影；OpenAPI/SDK 扩到 4 paths/4 operation IDs。没有 Owner transaction 前刻意不提供临时写按钮。
- 历史 VPS run `20260825T152835Z-p5` exit 0：Rust 14 tests（其中 SQLite+真实 PG contract）、fmt/Clippy；配置零写入检查；OpenAPI/doc；Web type/lint/Vitest 2/2/build 297 modules；真实 runtime 6 个 request IDs 唯一。其 source revision 属于 pre-public private baseline, intentionally unpublished；保留 run ID 不表示存在对应公开 commit。完整说明：`docs/05-implementation/WP01_FOUNDATION_SLICE.md`。
- 诚实边界：这是 WP-01 的第一纵切，不是 WP-01 完成。settings/secret/object 目前只有 schema；typed repository、AEAD/object adapter、Owner 原子 bootstrap 尚未实现。358 条产品项仍全部 planned。
- 下一步：继续 WP-01 typed settings、内容寻址对象、secret canary 与 bootstrap transaction；随后 P5.2 Agent handshake。

### 2026-08-25 22:55 — P5.0 首个 Rust/Vue 纵切与统一 VPS 验证通过

- 工具链不按 `latest` 猜测：VPS 从官方 manifest/registry 查询 Rust 1.98.0、Node LTS 24.19.0、pnpm 11.24.0、Vue 3.5.41、Vuetify 4.1.11、Vite 8.2.2。TypeScript 7 超出 typescript-eslint 范围；6.0.3 又在真实 vue-tsc 中与 Hey API/Vuetify/Router/i18n 声明不兼容，最终锁 5.9.3。
- 固定 builder：Rust base `e536cf...001d5` → builder ID `6ab618...1613`；Node base `a9f5f7...29df` → builder ID `066286...afcb`。`tools/vps_verify.sh` 在任何阶段前校验 image ID，防标签漂移。
- Rust workspace 已建立 `apps/master`、`apps/agent`、`crates/domain`、`crates/api`。Domain 当前实现 UUIDv7 `EntityId`、checked `Revision`/`ByteCount`；Master 实现 loopback 默认监听、graceful shutdown、JSON tracing、request ID、`healthz/readyz/system/version` 和运行时 OpenAPI；Agent 明确只处于 `skeleton-not-enrolled`。
- Vue/Vuetify 已建立 SaaS shell、Router、Pinia、TanStack Query、i18n、theme、dashboard/system 页面。Rust `utoipa` 导出 OpenAPI 3.1，再由 Hey API 0.99.0 生成 TypeScript fetch SDK；System 页面未手写 response type。
- pnpm 11 supply-chain 设置已显式化：24 小时 maturity strict、block exotic transitive、strict peer/engine、未审阅 build script 即失败；只允许已检查的 `vue-demi` postinstall，明确拒绝 `@parcel/watcher` build。严格安装先后发现不存在的 `@eslint/js` 推断版本、缺失 `vue-eslint-parser` 和 deprecated fetch runtime，均修正后通过。
- 首个统一 VPS run `20260825T145357Z-p5`（`/opt/nodecontroll/artifacts/test-runs/...`）exit 0：Cargo 6/6、Clippy零 warning；OpenAPI 3.1/3 paths/3 operation IDs；docs 358/358/0断链；Web type/lint、Vitest 2/2、Vite 295 modules；主 JS gzip 111.95 KiB；真实 Master smoke 的 health/ready/version/OpenAPI 和 4 个唯一 request ID 均通过。
- 已知边界：P5 尚在进行。无数据库、认证、job、Agent enrollment/transport 或 sing-box；`readyz` 还是 skeleton；358 个产品项仍为 planned 358/implemented 0/verified 0。详细逐文件/函数和失败修正见 `docs/05-implementation/WP00_ENGINEERING_SKELETON.md`。
- 下一步 P5.1：typed config、SQLite/PG migration/repository、instance/settings/secret/object、真实 readiness 和 Problem Details；P5.2 才做 Agent protocol/enrollment handshake。

### 2026-08-25 22:22 — 完成 P4 无授权全功能重构设计

- `docs/04-rebuild` 形成 16 份设计入口/正文：总体架构、官方 sing-box 兼容性、Agent v1 协议、Rust 模块/函数、SQLite+PostgreSQL 数据模型、HTTP/事件 API、订阅 typed IR、Vue 3+Vuetify SaaS UX、安全、可观测性、部署、迁移、测试和 WP-00～WP-21 实施计划。
- sing-box 官方源码基线锁定 stable `v1.13.19@b5ebaa1fc0f2b94256180b95468e73ef53caa27d` 与 preview `v1.14.0-beta.17@c82b9b8dc92e1495968a1e0835644e4ad6fc303b`。确认标准 transport 无 XHTTP，1.13 SIGHUP 会重建实例；1.14 official API 提供带 user/source/bytes 的连接流和关闭/选择能力，但标准内核没有通用动态用户 CRUD 或通用限速器。
- 目标不维护 sing-box fork：从官方 source/tag 构建默认 tags + `with_v2ray_api`。用户连接事件来自官方 1.14 API；Linux tc/eBPF + HTB/fq 执行平滑限速，unsupported/degraded 必须真实报告。稳定版/预览版轨、reload epoch、计量 flush、last-good 和协议逐项互操作均进入测试门。
- 原 PRO 行为全部归入普通工作包：内核/容器模式 WP-07，实时/自动限速与连接追踪 WP-11，测速 WP-15，实例联合 WP-18，品牌 WP-04；NOLIC-001～007 在 WP-20 做断公网和仓库/制品/网络捕获验收。
- `REQUIREMENTS_TRACEABILITY.md` 为社区版 128、X 213、PRO/去授权 17 项逐一生成 `NC-<source-id>`，共 358 行；每项已有 primary WP、设计合同和计划测试，但状态全部保持 `planned`，尚未把设计当实现。
- `tools/validate_design_docs.mjs` 已在 VPS `/opt/nodecontroll/worktree` 使用锁定 `node@sha256:d32cdf619f63fe0471182d08996dd516c6275bb5fd31ae06e55a570bd9e1ad43` 只读容器运行：`source_requirements=358`、`trace_rows=358`、`design_documents=16`、`planned=358`、`implemented=0`、`verified=0`、`broken_links=0`，exit 0。
- P5 从 WP-00 开始：先锁定 Rust stable/Node/pnpm/Vue/Vuetify 等官方版本，创建 workspace、GitHub Actions 编译链、VPS 测试证据流水线和最小 Rust Master/Agent + Vue shell；正式编译在 Actions，测试在私有 VPS。

### 2026-08-25 21:15 — 完成 P3 妙妙屋 X/PRO 差异分析

- 对 sitemap 中 58/58 个中文页面完成阅读台账。公开仓库保留来源 URL、抓取时间、字节数、SHA-256、标题树和原创分析；规范化正文与 gzip 原始 HTML 只留在被 Git 忽略的维护者审阅归档中。`DOC_EVIDENCE_AUDIT.md` 同时记录公开 X 仓库无主程序源码、Agent 仓库不可访问以及 8 个文档矛盾/不确定项。
- `X_FEATURE_CATALOG.md` 将 X 文档拆成 213 个唯一 `MMWX-*` 验收单元，覆盖平台/部署、Agent 四种连接、服务器与内核、21 个协议组合、节点/出站/路由、用户套餐与账本、订阅模板、证书/Nginx、探针、TGBot、MCP 和实例联合。
- `PRO_FEATURES.md` 确认 10 类原 PRO 行为、7 条必须删除的授权耦合。节点测速被判定为“社区版已有核心实现、X 扩展 UI/配对后加授权门槛”，没有误记为完全新增。
- `DIFFERENCE_MATRIX.md` 沿用社区版全部 128 个 `MMW-*` ID 域，对照 213 个 X 单元，逐域给出继承/扩展/新增/改语义/原 PRO/待内核核验、目标验收、F0～F3 优先级及复杂度。
- 当前 `docs` 中人工撰写 Markdown 的相对链接检查为 0 个断链；证据正文中保留 6 个官网根路径链接，属于抓取原文而非本地断链。

### 2026-08-25 21:02 — 完成 P2 妙妙屋功能说明

- `docs/02-upstream-features/FEATURE_CATALOG.md` 已按五类参与者和十个业务域整理 128 个稳定 `MMW-*` 能力 ID；每一项均记录入口、实际行为以及源码约束/外部依赖。
- 功能目录同时给出最小使用路径、外部订阅 + proxy-provider 路径、流量探针路径，以及社区版明确未实现或实现不完整的边界。
- 能力结论以源码、HTTP 路由、数据库和 VPS 构建为准；远程 tester、X Agent 等仓库外组件没有被误记为已完成。
- 文档相对链接复核为 0 个断链。P3 将沿用 `MMW-*` ID，与 `MMWX-*` 和 NodeControll 验收 ID 建立逐项映射。

### 2026-08-25 20:54 — 完成 P1 逐函数源码解剖

- 前端 TypeScript AST 索引已在 VPS 修正并重跑。首版把 `createFileRoute(...)({...})` 的外层柯里化调用也算作路由，现只接受带字符串 literal 的文件路由：23 个文件路由 + 1 个根上下文路由，共 24 个。新归档 SHA-256 为 `e2b31d270742280312a28ef41c12ed5ec0b6362b8275cdb38ff56a3890bc8c38`。
- `docs/01-upstream-source` 的公开结构化分析当前有 43 个文件、总计 1,640,037 字节，其中 41 个 Markdown；内部相对 Markdown 链接检查为 0 个断链。含源码签名的早期审阅稿已移入被 Git 忽略的私有归档。
- 完成人工校对文档：
  - `REPOSITORY.md`：根目录、依赖、Docker/Compose、GitHub Actions、持久目录和 18 个 shell 函数/顶层脚本逐项说明。
  - `BACKEND.md`：启动 16 步、请求中间件、18 个 Go 包、每个非测试 Handler 文件、后台任务、测试意图和约束。
  - `FRONTEND.md`：入口/provider、认证/API、24 条路由、20 个业务组件、布局/V3/UI/Hook/Context/config、两套订阅构建模块和复杂度。
  - `DATABASE.md`：26 张表的每列、索引、外键、关系和 SQLite PRAGMA；确认源码未显式执行 `PRAGMA foreign_keys=ON`。
  - `HTTP_API.md`：87 个顶层注册及其真实 method/子路径/鉴权/业务行为。
  - `DATA_FLOWS.md`：初始化登录、节点、外部同步、provider、订阅发布、模板规则、流量、测速、备份恢复和跨资源一致性。
- 机器索引提供逐函数追踪：Go 120 文件/1,473 符号；TypeScript 135 文件/3,666 声明，其中函数/方法/闭包 3,263 个。公开版逐项记录文件、行号、原创作用说明、主要调用和控制流证据，不复制源码签名、常量字面量或表达式正文。

### 2026-08-25 20:29 — 完成 P0 基线并验证上游构建

- 妙妙屋 X 中文文档 sitemap 的 58 个页面已全部审阅：
  - 证据索引：`docs/03-mmwx-gap/evidence/PAGE_INDEX.md`。
  - 结构化清单：`docs/03-mmwx-gap/evidence/manifest.json`。
  - 维护者私有审阅归档保存规范化正文和 gzip 压缩原始 HTML；HTML 总计 3,784,930 字节，归档路径被 Git 忽略。
  - 公开仓库逐页记录来源、抓取时间、HTML 字节数与 SHA-256、标题层级和 PRO 明文次数，不分发网页镜像。
- 使用 VPS 上的固定容器生成逐函数源码索引：
  - Go：`golang@sha256:28d89ee9cc0ff9fec75c82ca201e6bf7fdf9a679d4b7b24dfa04f2bb766bb468`。
  - Go 结果：120 个文件、1,032 个具名函数/方法、113 个匿名闭包、190 个类型、138 个常量/变量；共 1,473 个符号。
  - TypeScript：`node@sha256:d32cdf619f63fe0471182d08996dd516c6275bb5fd31ae06e55a570bd9e1ad43` + TypeScript 5.9.3。
  - TypeScript 结果：135 个文件、3,263 个函数/方法/闭包、403 个顶层声明、24 条真实路由、225 个静态可识别 API 调用。
  - 公开索引为每个函数记录文件/行号、原创作用说明、主要调用、分支/循环/返回/并发或 await 证据；后续人工文档负责校正自动推断语义。源码签名只用于私有审阅，不在仓库中发布。
- 额外锁定公开 `iluobei/miaomiaowuX@074de299588d7077d4ba62aeabecd503de5baed8`：当前仅 14 个安装/规则/README 文件，无主控实现源码。
- `iluobei/mmw-agent` 在抓取当日 GitHub API 返回 404，无法克隆；旧搜索缓存仍可看到 README，但只能作为时效性较弱的辅助证据，不能标为源码验证。
- 按上游 Dockerfile 顺序在 VPS 验证构建：
  - 前端镜像：`node@sha256:2cf067cfed83d5ea958367df9f966191a942351a2df77d6f0193e162b5febfc0`（`node:20-slim`）。
  - `npm ci` 安装 457 个包；`npm run build:only` 成功，Vite 转换 3,327 模块并生成 176 个 `internal/web/dist` 文件。
  - 首次 `go test ./...` 因仓库没有提交 `internal/web/dist` 而使 `go:embed dist/*` 失败；这属于必须先构建前端的顺序约束。
  - 构建前端后重跑 `go test ./...`，全部包可编译；`internal/handler` 有 2 个稳定失败测试：`TestBatchCreate_DefaultsEnabled`、`TestCreate_DefaultsEnabled`。两者构造的节点缺少当前存储层必填的 `protocol`，收到 HTTP 400；其余有测试的包通过。
- VPS 容量复核：根盘剩余约 30 GiB；Go 缓存 732 MiB、npm 缓存 97 MiB、上游 `node_modules` 459 MiB、前端构建产物 16 MiB。

### 2026-08-25 20:16 — 建立项目文档骨架

- 创建根 `README.md`，明确完全自托管、无许可证依赖、Rust + Vue 3 + Vuetify + sing-box 的最终方向。
- 创建分层文档目录约定，后续源码解剖、产品功能、差异、设计与实现说明不混写。
- 创建本进度文档，规定它为持续更新的权威状态源。
- 创建 `.gitignore`，隔离上游 Git 元数据、下载中间产物、依赖和构建输出。

### 2026-08-25 20:14 — 完成首份源码文本快照

- VPS 已成功浅克隆 `https://github.com/iluobei/miaomiaowu.git` 到 `/opt/nodecontroll/upstream/miaomiaowu`。
- 锁定提交：`0b47f10c52aee10b9f759a593ca5f61a823cbb72`。
- 远端 Git 工作树洁净：`main...origin/main`，共 385 个已跟踪文件。
- 为绕过本地 GitHub 慢链路，生成 303 个关键文本文件的审计快照：
  - 远端文件：`/opt/nodecontroll/cache/miaomiaowu-text-0b47f10.tar.gz`
  - SHA-256：`0bb3321aa27130f17f4996ac96cacf48de2fb0d71f4e405fe5ff1b8f08408081`
  - 本地校验通过后展开至 `upstream/miaomiaowu`。
- 初步识别上游技术栈：Go 后端（120 个 `.go` 文件）和 React/TypeScript 前端（93 个 `.tsx`、45 个 `.ts` 文件），并非本次目标技术栈。
- 初步识别后端包：`auth`、`captcha`、`handler`、`logger`、`notify`、`patches`、`proxygroups`、`scriptengine`、`speedtest`、`storage`、`taskrun`、`util`、`validator`、`version`、`web`。

### 2026-08-25 20:02 — 验证 VPS

- SSH key 非交互登录成功；host、系统用户和 key 路径由维护者私有配置保存。
- 系统：Debian 系 Linux 6.1，x86_64，8 vCPU，约 11 GiB RAM，无 swap。
- 根盘：197 GiB，总剩余约 34 GiB；后续必须限制 Docker/Rust/Node 缓存增长。
- 已安装：Git 2.39.5、Docker 29.7.2。
- 未安装：Rust/Cargo、Node/npm/pnpm、sqlite3 CLI。计划使用固定版本容器完成所有编译与测试，避免污染宿主机。

## 验证记录

| 时间 | 范围 | 命令/方法 | 结果 |
|---|---|---|---|
| 2026-08-25 20:02 | SSH | 使用维护者私有 host/user/key 配置执行 BatchMode 连接 | 通过；连接细节不进入公开仓库 |
| 2026-08-25 20:03 | 上游引用 | `git ls-remote ... HEAD refs/heads/main` | 两者均为 `0b47f10...` |
| 2026-08-25 20:05 | 远端克隆 | `git clone --depth 1 --single-branch ...` | 通过，工作树洁净 |
| 2026-08-25 20:14 | 本地快照 | SHA-256 + 解包后文件统计 | 通过，303 文件、3,965,869 字节 |
| 2026-08-25 20:18 | X 文档证据 | sitemap + 4 并发 HTTP 抓取 + 每页 SHA-256 | 通过，58/58 中文页 |
| 2026-08-25 20:20 | Go AST 索引 | VPS `golang:1.26-alpine` 容器 | 通过，120 文件/1,473 符号 |
| 2026-08-25 20:23 | TypeScript AST 索引 | VPS `node:24-alpine` + TypeScript 5.9.3 | 通过，135 文件/3,666 声明 |
| 2026-08-25 20:28 | 上游前端 | VPS `npm ci && npm run build:only` | 通过，3,327 模块/176 文件 |
| 2026-08-25 20:29 | 上游后端 | VPS `go test ./...`（前端构建后） | 编译通过；2 个已知上游测试失败 |
| 2026-08-25 20:45 | TS 路由索引校正 | VPS Node 24 + TypeScript 5.9 AST 重跑、归档 SHA-256 校验 | 通过，24 条真实路由 |
| 2026-08-25 20:54 | P1 文档完整性 | 41 个 Markdown 相对链接目标检查 | 通过，0 个断链 |
| 2026-08-25 21:15 | P2 功能目录 | `MMW-*` 唯一 ID 统计 + 相对链接检查 | 通过，128 项 |
| 2026-08-25 21:15 | P3 文档覆盖/目录 | manifest 计数、`MMWX-*` 唯一 ID 统计 | 通过，58/58 页、213 项 |
| 2026-08-25 21:15 | P3 文档完整性 | 排除只读证据正文后的 Markdown 相对链接检查 | 通过，0 个断链 |
| 2026-08-25 23:24 | P5.1 双库/系统纵切 | VPS `tools/vps_verify.sh` + PG 18.6 | 通过，14 Rust tests、4 API paths、2 Web tests、runtime smoke |
| 2026-08-25 23:45 | P5.1 存储/密钥纵切 | VPS `20260825T154501Z-p5` | 通过，21 Rust tests、SQLite+PG settings、object、AEAD、runtime key cleanup |
| 2026-08-26 02:35 | 第三方许可证旧扫描 | VPS 固定 Rust/Node builder 双次扫描 | 旧 647/846 受 stale `.pnpm` 污染，计数与“闭包通过”结论均已撤回 |
| 2026-08-26（发布前 v10 临时双跑） | 第三方许可证历史候选 | fresh pnpm install + active reachable graph；无正式 run manifest | 645 components、844 evidence、npm 425、platform optional excluded 36、null integrity 0、14/14 overrides、Rust evidence 14；已被最终临时双跑替代 |
| 2026-08-26（发布前门禁单项验证） | npm/lock/repository/SBOM 闭包 | fresh virtual store 与负向输入、固定 CycloneDX CLI | 425 npm identity 精确相等；脏 store、symlink/越界、非 canonical v9 lock、非法 repository 和无效 SBOM schema 均按门禁拒绝；无正式 run manifest |
| 2026-08-26（发布前最终临时双跑） | 第三方许可证最终候选 | 私有 run-scoped Cargo/pnpm 输入，两次隔离收集，CycloneDX CLI 0.33.1 | 两轮逐字节一致：645 components、844 evidence、npm 425、20/20 overrides；6 个 SQLx 包均有双许可证完整证据；无正式 run manifest |
| 2026-08-26（发布前负向 fixture） | tracked source provenance | commit blob/mode、index-hidden changes、replacement refs | 正向 2 blobs 通过；被 `skip-worktree`/`assume-unchanged` 隐藏的内容变更和 mode 篡改均拒绝；replacement ref 被禁用 |
| 2026-08-26（发布前依赖闭包） | fresh pnpm inventory | 私有空 store、断共享缓存安装后独立枚举 | 安装 425 个 package identity，与收集器 inventory 精确双向相等 |
| 2026-08-26 02:35 | SetupPage Web 预检 | VPS Vue typecheck、ESLint、Vitest | 通过，2 files、13/13 tests；SetupPage 11 项 |
| 2026-08-26（公开根提交） | public projection | root commit、GitHub branches/tags/commit API、敏感字符串与 tree mode 扫描 | 通过；`607f54b...` 无父、196 files、仅 `main`、无 tag、无私有路径/凭据/原始 X 网页全文 |
| 2026-08-26（Actions attempt 1） | 正式编译 | run `32901202453` | 失败；container ownership 在首个 source-pinning step 拒绝，未编译、无 artifact；修复待下一 run 验证 |
| 2026-08-26（第二次 push run） | Actions 前置环境 | run `32901554308` | source pinning 已通过；setup-python 不支持 Debian container 而失败，未编译、无 artifact；改用固定 image 自带 Python 3.11.2 |
| 2026-08-26（第三次 push run） | GitHub Actions 正式编译 | run `32901899767`、artifact `9583470546` | Actions 全绿；artifact 4,165,254 bytes、SHA-256 `405626...0039`；尚未通过 VPS verifier |
| 2026-08-26（首次正式 artifact run） | VPS provenance/archive | run `20260825T214246348535211Z-p5` | provenance、source、CycloneDX CLI 通过；`actions-archive-members` 因 14 个 sysroot evidence 为 `0666` 而拒绝，状态 failed |
| 2026-08-26（mode/Cargo 修复 push） | GitHub Actions 正式编译 | run `32904404331`、artifact `9584322069` | Actions 全绿；mode 与嵌入路径预检通过；artifact SHA-256 `f31986...c262ea` |
| 2026-08-26（第二次正式 artifact run） | VPS archive/license/ELF | run `20260825T221048400560496Z-p5` | archive、package/license/SBOM 通过；`actions-elf-check` 因 allowlist 漏列合法 loader SONAME 而拒绝，状态 failed |
| 2026-08-26（ELF verifier 修复 push） | GitHub Actions 正式编译 | run `32905833325`、artifact `9584823840` | attempt 1 全绿；artifact 4,165,197 bytes、SHA-256 `6ed502...29f65a` |
| 2026-08-26（第三次正式 artifact run） | 完整 VPS 发布门 | run `20260825T223046660411478Z-p5` | completed；provenance、archive/license/SBOM、34 Rust tests、clippy、ELF/OpenAPI/Web 精确复现、13 Web tests、runtime smoke 全部通过 |
| 2026-08-26（C1 HTTPS v2 候选） | rotation/logout/证据闭包 | run `20260826T063126155736337Z-wp02c-candidate-v2` | 通过，33 files/10,028,822 bytes；覆盖 rotation、错误 proof、logout 503 fail-closed、真实 204、旧凭据 401；可写叶文件、额外 journal、解码 root-key 三类负向输入均拒绝；尚非当前树 provenance |
| 2026-08-26（C1 P1 增量候选） | 登录 timing + actor-aware DELETE | archive `033b7c...e6db`，固定 Rust/PostgreSQL builder | fmt、78/78 双库 workspace tests、Clippy `-D warnings`、release bins 通过；仍待全新精确 run root、公开 Actions 与 fresh-clone 正式门 |
| 2026-08-26（C1 迁移回滚增量候选） | 原子限流事件、统一 PostgreSQL 锁序、0004/0005 失败回滚 | archive `c31ab8...e34344`，固定 Rust/PostgreSQL builder | 225/225 源码清单一致；fmt、78/78 双库 workspace tests、Clippy `-D warnings`、release bins 通过；测试 schema、容器、网络已清理；仍非提交级 provenance |
| 2026-08-26（C1 前端协调增量） | typecheck/lint、Vitest、Web production build | 固定 Node 24.19.0/pnpm 11.24.0 builder | 9 files、81/81 Vitest，test log `ec10a8...5203`；362 modules，build log `c7318b...b1ca`；生成器 4 个顶层输出项、目录内 16 个物理文件零漂移，artifact 校验通过 |
| 2026-08-26（C1 双页协调候选） | Web Locks/journal、运行时投影校验、双页 HTTPS | run `20260826T112446112732525Z-wp02c-candidate-v4` | 真实双页 gate 扫描 33 files/10,087,567 bytes，宿主 validator 复算通过；仍非提交级 provenance |
| 2026-08-26（C1 历史应用代码候选 v4） | 226-file archive、Rust/Node、OpenAPI/SDK、SQLite/PostgreSQL smoke | archive `0c2eb2...abe5c`；log manifest `320b1e...2ab89` | 78/78 双库 Rust、Clippy/release、81/81 Web、362 modules、两库 runtime smoke 全绿；这是历史应用代码/运行时证据，不是当前完整输入 |
| 2026-08-26（C1 门工具候选 v6） | pnpm 拓扑、静态/Web、许可证闭包、SQLite/PostgreSQL smoke | archive `e2a055...8cb58`；source manifest `ee9084...99ec6` | global virtual store=false；16 个生成文件零漂移；81/81 Web；650 个许可证组件、428 npm identity 闭合；两库 smoke 全绿且资源清理通过；后续 Actions/VPS 边界和账本修改尚待最终 freeze |

## 风险与约束

- VPS 根盘最近一次只读观测为约 94%（176 GiB/197 GiB，剩余约 13 GiB）；Rust registry、前端依赖和 Docker layer 必须使用可清理、可度量的专用缓存，只按已核对的精确路径回收候选资源，不做宽泛 prune。
- 本地到 GitHub/VPS 的大文件下载速度很慢；公开 GitHub Actions 是正式 release/Web 的唯一编译环境，VPS 是测试和同 SHA 制品运行验收环境，本地只保留文本源码和必要资产。
- 妙妙屋 X 文档为动态站点，必须同时保存站点目录、页面证据和抓取日期，避免文档更新后差异结论失去可追溯性。
- “实现所有功能”以最终功能矩阵的验收条件为准；所有原 PRO 能力一律视为普通自托管能力，不引入商业许可证、官方授权或功能锁。项目和第三方的开源许可证/版权声明仍依法保留。
- 代理内核从 Xray/Mihomo 语义迁移到 sing-box 时存在协议、统计、限速和动态配置能力差异，必须在设计阶段逐项验证，不做名称替换式迁移。

## 下一步

1. 把“正式编译只在公开 Actions、VPS 只测试/运行验收”的 verifier 与本次账本修正纳入最终 226-file freeze，完成静态门和 staged-tree 预检；随后只推送单父 `main`，等待 attempt 1 Actions 同 SHA 制品，再用 fresh standalone full clone 完成 VPS formal provenance。通过后并行推进 C2 keyring/recovery、C3 challenge 与 C4 TOTP 核心，不得把 C1 候选误记为完整身份系统。
2. 完成 P5.2 Agent protocol/enrollment handshake，再按 WP-03～WP-20 推进后端、Vue/Vuetify 页面与 SingBox 集成。
3. 按 358 条追踪矩阵逐项实现、测试并更新状态；不得用 schema、路由占位或页面壳替代产品行为验收。
