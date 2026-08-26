# NodeControll 实施进度

> 本文档是项目推进的权威事实源。每次实现、验证、架构变更和风险发现都必须更新。时间使用 Asia/Shanghai。

## 当前状态

- 当前阶段：P5，工程骨架与远端可复现构建；P0～P4 已完成。
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

### 2026-08-26 — WP-02 密码登录/服务端会话候选树通过 VPS 预检

- 新增 `nodecontroll-application`，把 bootstrap、登录、当前身份和退出从 HTTP/SQL 细节中分离；Domain 补齐六角色、用户状态、capability、认证等级与 session 生命周期枚举。SQLite/PostgreSQL `0003_auth_core` 同步增加 `user_auth_state`、`auth_sessions`、`login_rate_buckets`、`login_security_events`，repository contract 在两库使用同一 fixture。
- 登录在任何 limiter 写入前取得 1～64 的进程内许可，许可覆盖三层 bucket、凭据读取与 Argon2 验证。已有 account/IP/global 封禁先由一条只读查询拒绝，不更新 blocked hit，也不会随着轮换账号或 IP 扩张 row；未命中仍由 account→IP→global 固定锁序事务处理并发。不存在用户验证 dummy PHC，停用、未知、错密对外统一 401。
- 原始 session/CSRF token 只进入 `Secure`、`__Host-` Cookie；数据库保存用途隔离、带 key version 的 HMAC。`/me` 检查 active 用户、`auth_revision`、idle/absolute deadline；写请求同时检查 canonical Origin/Host、严格 Cookie/header CSRF 与数据库 CSRF HMAC。mutating auth 已收敛为单次 typed repository outcome，错误 CSRF 不 touch session；logout 撤销与安全事件在同一事务中提交。
- Vue 增加内存 Pinia session 状态机、登录页、路由 guard、SaaS authenticated shell 和 fail-closed 控制面门。重定向拒绝外站、反斜杠、编码控制字符和 guest route。Setup 成功前会清空 setup token/密码；登录成功但导航失败时清空密码、禁用再次登录并只允许重试导航；退出后即使路由失败，旧受保护 DOM 也会立即消失。
- 固定 Rust builder 的候选预检通过 `cargo fmt --check`、`cargo check --locked --workspace --all-targets`、68 个双库 workspace test、Clippy `-D warnings` 和 release bins；精确工具链仍是 rustc/cargo 1.98.0、rustfmt 1.9.0、clippy 0.1.98。最终 OpenAPI SHA-256 为 `e2d8316e7f8c7543328d03044a24bf621461144fa2043f3b4562fc1c29af6280`，共 7 paths/8 operations；VPS 生成 SDK 后 typecheck、零 warning lint、6 个文件 29/29 Vitest 和 341-module production build 全绿。
- 真实 PostgreSQL runtime 已完成一次性 bootstrap、重复初始化拒绝、登录、刷新恢复、退出、撤销后 401；Master 以同一数据库和 root key 重启后，再次登录/恢复/退出也通过。重启前后日志分别为 205/376 bytes，SHA-256 `63f0035d22a8e9d58443d1d60c495a12e8c56ae7e4544bd396aa5766f4256347` 与 `979649eacfc8d555e09996a8625bb9322642e907a385b57b2923060c8ece66d1`；按真实 setup token、root key、测试口令、PHC、session/CSRF 前缀扫描均无命中。临时 Master/PostgreSQL 容器、网络与秘密文件已删除，只保留不含秘密的候选日志。
- 发布前审计修正 verifier 边界：runtime log 先写 `.capturing`，捕获成功后才原子替换并扫描；捕获失败、secret fixture 缺失或扫描异常都会留下失败 marker。扫描器改由 Python 按文件路径在进程内读取秘密，秘密正文不再进入子进程 argv。Cargo fetch、test、release 和许可证重建容器都显式固定 `RUSTUP_HOME`，与断网只读工具链预检一致。`bash -n` 与精确版本预检已经在 VPS 通过。
- 这仍不是正式发布证据：候选目录没有 commit-scoped manifest，也没有绑定公开 SHA、Actions artifact 或 fresh-checkout run。下一步是提交并只推送公开 `main`，等待 Actions 生成同 SHA 制品，再运行完整 `tools/vps_verify.sh`。MFA/WebAuthn/recovery/recent-auth、token、完整 RBAC/用户生命周期、浏览器 E2E，以及 bucket/event/session retention 和持久化 key canary 仍未完成；358 项需求继续全部保持 `planned`。

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
- VPS 正式 verifier 的 Cargo registry/git 输入与 pnpm store 都改为 run-scoped 私有闭包。Cargo 先联网 `fetch --locked`，随后只读挂载；Rust test 与 release 使用不同 target，release 阶段断网。pnpm 先在私有 store 中 `--ignore-scripts --package-import-method=copy` 安装，再冻结输入清单；Node 工具只在隔离 scratch source 中运行，并按阶段拒绝 allowlist 外的工作树额外路径。正式 checkout 还必须是公共仓库唯一 `origin` 的非 shallow、非 partial standalone full clone，本地 `main`、`origin/main`、HEAD 与 Actions push SHA 同指一个 commit。
- 正式 verifier 现在不只测试 Actions 产物：它还在 VPS 固定 builder 中重新编译 Rust release、导出 OpenAPI、构建 Web production，然后分别与 Actions 两个 ELF、OpenAPI 和完整 `dist` 树逐字节比较。Actions 工作流在构建、打包前后重复核对源码 verifier、replacement refs 和真实工作树闭包。
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
- 重收集器使用从固定 Node image ID 提取的 Node/pnpm runtime，并在固定 Rust image ID 中运行；network none、rootfs/source/本轮私有 Cargo 输入只读，只有 run-scoped pnpm runtime store 和本轮 notices 输出可写。上述闭包与负向拒绝路径已做发布前单项验证，但正式 recollection gate 仍等待同 SHA GitHub Actions artifact；届时 VPS 还须把重建 notices 与 Actions payload 按目录/文件全集及逐文件 size、SHA-256、实际 bytes 双向比对。当前不能登记为 provenance 或发布验收通过。

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

## 风险与约束

- VPS 根盘已使用约 83%；Rust registry、前端依赖和 Docker layer 必须使用可清理、可度量的专用缓存。
- 本地到 GitHub/VPS 的大文件下载速度很慢；以 VPS 为上游镜像和唯一编译测试环境，本地保留文本源码和必要资产。
- 妙妙屋 X 文档为动态站点，必须同时保存站点目录、页面证据和抓取日期，避免文档更新后差异结论失去可追溯性。
- “实现所有功能”以最终功能矩阵的验收条件为准；所有原 PRO 能力一律视为普通自托管能力，不引入许可证检查。
- 代理内核从 Xray/Mihomo 语义迁移到 sing-box 时存在协议、统计、限速和动态配置能力差异，必须在设计阶段逐项验证，不做名称替换式迁移。

## 下一步

1. 补齐 WP-02 登录/session/MFA/RBAC 与 SetupPage 浏览器 E2E，并把对应需求从 planned 更新为 implemented/verified；不得把当前 bootstrap slice 误记为完整身份系统。
2. 完成 P5.2 Agent protocol/enrollment handshake，再按 WP-03～WP-20 推进后端、Vue/Vuetify 页面与 SingBox 集成。
3. 按 358 条追踪矩阵逐项实现、测试并更新状态；不得用 schema、路由占位或页面壳替代产品行为验收。
