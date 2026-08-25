# WP-00/P5.0：工具链与首个工程纵切

## 1. 本次完成边界

本次交付了可编译、可测试、可启动的最小纵切：Rust domain crate；Axum Master 的 liveness/readiness/version/OpenAPI；独立 Agent binary；Rust OpenAPI→TypeScript SDK；Vue 3 + Vuetify SaaS 壳；固定 VPS builder 和一键证据脚本。

它没有数据库、用户登录、durable jobs、Agent enrollment/transport 或 sing-box 控制，所以不关闭任何 `MMW-*`/`MMWX-*`/`PRO-*` 产品需求。追踪矩阵仍是 `planned=358, implemented=0, verified=0`。

## 2. 版本和供应链基线

版本在 VPS 直接查询官方 Rust manifest、Node index、npm registry 和 crates.io API：

| 项 | 锁定值 | 说明 |
|---|---|---|
| Rust | 1.98.0，manifest 2026-08-20 | `rust-toolchain.toml`，edition 2024 |
| Rust base | `rust@sha256:e536cf...001d5` | 官方 `rust:1.98.0-bookworm` digest |
| Rust builder | `sha256:6ab618...1613` | 在 base 上只安装 clippy/rustfmt |
| Node | 24.19.0 LTS Krypton | 2026-08-03 release |
| Node base | `node@sha256:a9f5f7...29df` | 官方 `node:24.19.0-bookworm-slim` digest |
| Node builder | `sha256:066286...afcb` | 只全局安装 pnpm 11.24.0 |
| Vue/Vuetify/Vite | 3.5.41 / 4.1.11 / 8.2.2 | 精确 npm versions |
| TypeScript | 5.9.3 | 6.0.3 实际不兼容当前 Vue/Vuetify/Hey API 声明文件，见失败记录 |
| OpenAPI codegen | `@hey-api/openapi-ts` 0.99.0 | exact pin；fetch client 已内嵌，不安装 deprecated runtime package |

`pnpm-workspace.yaml` 开启 strict peer/engine、阻断 transitive exotic sources、显式 24 小时 maturity strict gate。此次官方查询后未满 24 小时的 15 个 exact packages 有带日期例外。依赖 build script 只批准 `vue-demi`；`@parcel/watcher` 明确 false，任何新脚本默认使安装失败。

`Cargo.lock` 和 `pnpm-lock.yaml` 都由 VPS 生成并同步回仓库。workspace Rust direct dependencies exact pin；后续依赖升级必须重跑 compatibility 和完整 VPS 门。

## 3. Rust workspace 与函数说明

### 3.1 `crates/domain`

该 crate 无 async/HTTP/SQL/filesystem 依赖，后续业务不与框架耦合。

| Symbol | 当前作用 | 验证 |
|---|---|---|
| `EntityId::new` | 使用 OS randomness/time 生成 UUIDv7 | 测试 version=7 |
| `EntityId::into_uuid` | 在 adapter 边界取底层 UUID | const、无格式猜测 |
| `Default for EntityId` | 等价于新 UUIDv7，不产生 nil ID | 复用 `new` |
| `Revision::initial` | aggregate 初始乐观并发 revision 0 | unit |
| `Revision::next` | checked `u64` 加一，耗尽返回 typed `RevisionError` | normal + MAX |
| `Revision::value` | 只读取得数值，用于 persistence/API adapter | const |
| `ByteCount::new/value` | 非负字节 newtype 边界 | unit |
| `ByteCount::checked_add` | 禁止流量/账本溢出绕回 | normal + MAX |

workspace lint 禁 `unsafe`、未使用 must-use；Clippy 额外 deny `dbg!`、`todo!` 和 `unwrap`。后续 eBPF FFI 若确需 unsafe，必须单独 crate/模块调整并写安全不变量，不能放宽全仓库。

### 3.2 `crates/api`

| Symbol | 当前作用 | 后续演进 |
|---|---|---|
| `AppState::new` | 固定进程启动 UTC RFC3339 和 build version | WP-01 加 config/DB/object/health deps |
| `healthz` | 仅返回进程 liveness `ok` | 不检查 Agent/外部网络 |
| `readyz` | 当前 skeleton 返回 ready | WP-01 接入 config/secret/DB/migration gate |
| `system_version` | 返回 `{data,meta}` v1 envelope，含产品/版本/启动时间 | 加 schema/Agent/core compatibility |
| `openapi` | `utoipa` 单一事实源，导出 OpenAPI 3.1 | 每端点必须有 operation ID/response schema |
| `openapi_json` | 运行时规范端点；序列化异常返回可诊断最小文档 | 后续不泄露内部错误 |
| `router` | 注册 3 个业务/健康路径和规范路径；设置/传播 UUID request ID；结构化 trace span | WP-02 加安全 middleware/Problem Details |

`openapi` unit 验证三个 skeleton path；`tools/validate_openapi.mjs` 在 VPS 验证 3.1、必需 path、operation ID 存在且唯一。

### 3.3 `apps/master`

| Symbol | 作用 |
|---|---|
| `main` | 初始化 JSON tracing；解析 `NODECONTROLL_HTTP_LISTEN`（默认 loopback 8080）；绑定 socket；构造 state；运行 Axum graceful server |
| `init_telemetry` | 使用 `RUST_LOG` EnvFilter 或 info，输出 JSON；初始化冲突不 panic |
| `shutdown_signal` | Unix SIGTERM/跨平台 Ctrl-C graceful shutdown |
| `export-openapi::main` | 从同一 Rust schema pretty serialize 规范到 stdout；VPS 写入 `openapi/nodecontroll-v1.json` |

运行日志当前只有 timestamp/level/message/address/version；smoke 中没有 secret/body。Master 默认只监听 loopback，测试显式使用 18080，结束后容器被删除。

### 3.4 `apps/agent`

`main` 当前只输出 machine-readable product/version/`skeleton-not-enrolled`。独立 binary 已证明 workspace/build/release边界存在，但没有协议或特权操作。它不应被称为“Agent 已实现”；WP-05/06 才会增加 enrollment、mTLS、四种 transport、local SQLite、task registry 和 helper。

## 4. Vue/Vuetify 代码说明

### 4.1 组合根

`src/main.ts` 依次安装 Pinia（client state）、TanStack Vue Query（server state）、i18n、Router 和 Vuetify。没有把 server response 手写进 Pinia。

`src/plugins/vuetify.ts` 建立首个 SaaS light theme 和语义 success/warning/error/info。`src/plugins/i18n.ts` 只有 skeleton 中英文本；业务页面新增时必须同步两种 locale。

### 4.2 应用壳和页面

- `App.vue`：Vuetify application、navigation drawer、top bar、route content；显示明确 `P5 · Skeleton`，避免误认为生产系统。
- `DashboardPage.vue`：只展示已经存在的四个工程基础，不展示虚构业务指标；tuple 有显式 readonly 类型以满足 `noUncheckedIndexedAccess`。
- `SystemPage.vue`：TanStack Query 调用生成的 `getSystemVersion`；显示产品/版本/API/启动时间；错误是持久 alert。
- `formatStartedAt`：非法日期返回 `—`，合法 RFC3339 按 locale 格式化；Vitest 两项覆盖。
- `main.scss`：系统字体、1280px 内容宽、品牌/序号 token、reduced-motion；无远程字体/品牌资源依赖。

移动 drawer 目前没有完整 toggle、无认证/权限/全局任务壳；属于 P5/WP-04 后续，不能关闭 UI 需求。

### 4.3 OpenAPI SDK

`openapi-ts.config.ts` 读取 Rust 生成的 `openapi/nodecontroll-v1.json`，清空并重建 `src/api/generated`，文件头明确禁止手改。生成器默认内嵌 fetch client；曾安装的 `@hey-api/client-fetch@0.13.1` 已因官方 deprecation 信息删除。

`SystemPage` 直接导入生成 operation，response shape 来自 Rust schema。生产 build 顺序固定 `generate → vue-tsc → vite`，因此 stale/missing spec 或 SDK 会失败。

TypeScript 使用 strict + `noUncheckedIndexedAccess`。`skipLibCheck=true` 是有边界的兼容决定：只跳第三方 `.d.ts` 内 Vuetify/Vue/Hey API 的相互声明冲突，应用和生成 SDK 的 `.ts` 仍检查；后续生态兼容后尝试恢复 false。

## 5. Builder 与正式证据链

`deploy/build/Dockerfile.rust`/`.node` 的 `FROM` 都是官方 digest。VPS test builder 的 image ID 固定在 `tools/vps_verify.sh`；标签被重建或替换时，脚本会在测试开始前失败。正式 release binary 和 Vue dist 不再由 VPS builder 生成，而由 `.github/workflows/build.yml` 对 `main` push 的 clean checkout 编译。

Actions 固定 Rust 1.98.0、x86-64 GNU target、glibc 2.36、Node 24.19.0 和 pnpm 11.24.0，输出一个 `nodecontroll-linux-x86_64-glibc2.36.tar.gz`。包内包含 Master/Agent、OpenAPI、Web dist、license/notices/SBOM、`BUILD-METADATA` 和 `CONTENTS.sha256`；上传采用 `archive: false`，GitHub artifact digest 直接对应 raw tar SHA-256。OpenAPI/SDK drift 和 TypeScript 检查是制品生成门，不计作 VPS 验收。

当前 `tools/vps_verify.sh` 的正式入口执行以下检查：

1. 要求 cwd 为 `/opt/nodecontroll/checkouts/<完整 SHA>` 的 standalone clone，HEAD、目录名和 `/opt/nodecontroll/artifacts/github-actions/<SHA>/` 下的 raw tar 对应同一 commit；初始 tracked/untracked 与 ignored 输入均为空，并原子写入 `.git/nodecontroll-verifier.claim`，无论成功失败都不得复用 checkout。
2. 读取 GitHub run/artifact API，核对固定仓库与 workflow、`push/main`、成功 conclusion、head SHA、run/artifact ID；当前只接受 `run_attempt=1`。外部 raw tar 先复制为 run 目录内 0444 快照，API digest、后续 hash 和解包只消费该快照。
3. archive 在解包前必须通过 member 规范路径与 `./` 前缀、重复/别名路径、声明父目录、类型和 size 门；符号链接、PAX/sparse、普通文件/目录以外类型、超限压缩包/单文件/解压总量均被拒绝。解包到本轮 `compiled/` 后再验证 `BUILD-METADATA` 中的 run ID/attempt/commit/build baseline、`CONTENTS.sha256`、许可证、Rust runtime notices、第三方 notices、SBOM 和文件全集。
4. `pnpm-lock.yaml` 必须符合审阅过的 v9 canonical 顶层顺序和全集，`lockfileVersion` 精确为 `'9.0'`，其余 section 使用 block mapping。重复、quoted 或未知顶层 key、非规范 YAML 顶层语法、重复 package/integrity 均拒绝。组件 repository 只保留可规范化的 absolute `http(s)`/`ssh`/`git` URI；非法或不安全值失败。
5. fresh checkout 在固定 Node image ID 下执行 frozen pnpm install。独立 inventory gate 枚举 fresh `node_modules/.pnpm` 中的精确 name/version identity，与 artifact inventory 的 425 个 npm identity 做集合双向相等检查；脏 store 的额外包和缺失包都失败。`node_modules`、`.pnpm` 和实际包根必须是非 symlink 目录，realpath 分别受 checkout、`node_modules` 与 virtual store containment 限制。
6. 许可证重收集使用从固定 Node image ID 提取的 Node/pnpm runtime，在固定 Rust image ID、network none、只读 rootfs/source/共享 Cargo cache 环境运行；只有 run-scoped pnpm store 与空的本轮输出目录可写。重建 notices 与 Actions notices 按两侧目录/文件全集、逐文件 size、SHA-256 和实际 bytes 比对。CycloneDX 1.6 SBOM 另由固定 CLI 0.33.1 和 SHA-256 `bfc8b2538da86fe239bc53658bbb63c1c8c510a293c1e6891aa5bea5d3c58746` 校验官方 schema。
7. 使用 `file`、`readelf`、`ldd` 检查 Master/Agent 的 x86-64 ELF、解释器、动态库 allowlist 和最高 GLIBC 2.36，再直接运行 Agent artifact smoke、Master config/runtime smoke、runtime OpenAPI 对比及 Web artifact 引用检查；并执行 cargo fmt/test/clippy、真实 PostgreSQL 18 与 SQLite 合同、OpenAPI/docs/publication boundary、SDK drift、Web type/lint/unit。VPS 不再生成正式 Rust binary 或 Vue dist。
8. runtime smoke 后先停止 Master、冻结最终日志，再扫描 setup token、随机 root key、测试密码和 PHC 前缀。当前落盘内容为 `manifest.json`、`commands.tsv`、逐阶段 `logs/`、GitHub API 与 raw-tar 快照 `provenance/`、本轮 `compiled/`、`checksums.txt`；manifest 从 `running` 原子收尾为 `completed` 或 `failed`。全部阶段通过才写 `COMPLETED_AT`，`run_stage` 失败时写 `FAILED_STAGE`；失败 cleanup 无法证明最终日志无 secret 时写 `SECRET_SCAN_FAILED`。

JUnit、coverage、reports、Playwright traces、安全、性能和部署演练目录尚未由 P5 verifier 生成，不能列入本阶段已有证据。脚本只清理本轮容器、network、临时 root key 与 setup token，不删除 cache、已 claim checkout、下载的 Actions artifact、旧 run 或其他任务资源。

当前收集器把规范化后的依赖包许可证声明写入 CycloneDX `license.name`，不把复合表达式冒充已确认的 SPDX `id`。法律证据仍以随包或精确 override 收入的正文、来源和 checksum 为准。上述闭包和负向拒绝路径已有发布前单项验证；许可证重收集门尚无同 SHA Actions artifact 可供对照，因此本节描述的是 verifier 合同与临时验证结果，不是正式通过结论。

## 6. 失败与修正记录

下表记录初版 WP-00 VPS builder 和旧 verifier 的调通过程，其中的 VPS build 属于历史预检，不是当前正式 release 编译路径。

| 失败 | 根因 | 修正 |
|---|---|---|
| Rust fmt check | 初写没有经过固定 rustfmt | VPS 格式化并同步机械结果 |
| Clippy `double_must_use` | `Router` 自带 must-use，函数又重复标注 | 删除冗余属性；重跑零警告 |
| 首次 smoke binary missing | test harness/export binary 不等于普通 Master binary | 统一门显式 `cargo build --workspace --bins` |
| `@eslint/js@10.9.1` 不存在 | 错把 eslint 与配置包版本视为同步 | registry 核验后锁 10.0.1 |
| pnpm ignored builds | pnpm 11 默认拒绝未审阅 install scripts | review metadata；只允 vue-demi、拒 parcel watcher |
| peer missing | strict peer 发现 `vue-eslint-parser` 未声明 | registry 核验并显式锁 10.4.1 |
| deprecated fetch client | 0.73 起 client 已内嵌 generator | 删除 runtime dependency |
| TypeScript 6 大量 ecosystem errors | peer 声明宽于真实 `.d.ts` 兼容 | 锁 5.9.3；不以 skipLibCheck 掩盖项目源码 |
| 第三方 `.d.ts` 冲突 | Vuetify 4/Vue/i18n/DOM 声明内部兼容问题 | `skipLibCheck=true`，项目/生成 `.ts` 门保留 |

## 7. 早期 VPS 预检证据

以下 run 发生在公开 GitHub Actions release artifact 和当前 provenance verifier 建立之前，只保留为 WP-00 工程骨架的历史预检。它证明当时源码可以在固定 VPS builder 中编译、测试和启动，不满足“同一 main push commit → Actions raw tar/API digest → VPS 消费该制品”的正式证据链，也不能替代后续提交级 run。

统一 run：`/opt/nodecontroll/artifacts/test-runs/20260825T145357Z-p5`，14:53:57Z～14:54:28Z，exit 0。

结果：

- Rust tests：domain 3 + API 3，6/6；所有 targets compile；Clippy `-D warnings` 通过。
- OpenAPI：3.1.0，3 paths/3 unique operation IDs。
- 设计校验：358 source/358 trace、16 design docs、0 broken links；状态仍 planned 358。
- Web：typecheck、ESLint zero warning；Vitest 1 file/2 tests；Vite 295 modules。
- dist：main JS 324.63 KiB / gzip 111.95 KiB，低于 skeleton shell 预算；主 CSS gzip 34.36 KiB。
- runtime：health `ok`、ready `ready`、NodeControll 0.1.0、API v1、3 OpenAPI paths、4/4 unique request IDs。
- artifact：manifest、15 stage logs、commands、checksums、runtime log、COMPLETED_AT；smoke container 已清除。

lock hashes：Cargo `e96afd7f...4159`；pnpm `554d9932...aef7`。

## 8. 下一纵切

WP-01/P5.1：typed Master config；SQLite/PG repository 基础与 versioned migration；instances/settings/secret/object最小模型；readiness 真实依赖；API Problem Details；前端 setup/system projection。随后 P5.2 才建立 Agent protocol skeleton/enrollment handshake，满足 P5 退出门。
