# GitHub Actions 编译链

`build.yml` 只负责可复现的编译与静态生成，不承担测试。单元测试、集成测试、数据库测试和运行时烟雾测试继续只在项目指定 VPS 上由 `tools/vps_verify.sh` 执行。

## 触发与权限

- `main` 分支 push、所有 pull request 和手工 `workflow_dispatch` 会触发构建。
- `GITHUB_TOKEN` 只有 `contents: read` 权限。
- checkout 禁用凭据持久化；工作流不写仓库、不发布 Release、不部署。
- Job container 通过进程级 Git config 把唯一 `safe.directory` 固定为 `${{ github.workspace }}`，解决 host checkout 与 container 用户不同导致的 ownership 拒绝；不改全局 Git 配置文件。
- 同一 ref 的新构建会取消尚未完成的旧构建。

## 固定版本

| 组件 | 固定值 | 用途 |
|---|---:|---|
| Runner | `ubuntu-24.04` | GitHub 托管的 x86-64 构建环境 |
| Job container | `rust@sha256:e536cf...001d5` | Debian 12/glibc 2.36 ABI 基线；与 VPS runtime 兼容 |
| Rust | `1.98.0` | workspace release binaries 与 OpenAPI exporter |
| Node.js | `24.19.0` | OpenAPI 校验、SDK 生成和 Web 构建 |
| pnpm | `11.24.0` | frozen-lock 安装与 workspace scripts |
| CycloneDX CLI | `0.33.1`，SHA-256 `bfc8b253...c58746` | 仅由正式 VPS verifier 按官方 CycloneDX 1.6 schema 校验 SBOM |
| `actions/checkout` | `de0fac2e4500dabe0009e67214ff5f5447ce83dd` (`v6.0.2`) | 只读检出 |
| `actions/setup-python` | `a309ff8b426b58ec0e2a45f0f869d46889d02405` (`v6.2.0`) | 安装精确 Python 3.13.7，运行独立源码校验器 |
| `actions/setup-node` | `820762786026740c76f36085b0efc47a31fe5020` (`v7.0.0`) | 安装精确 Node 版本 |
| `actions/upload-artifact` | `043fb46d1a93c77aae656e7c1c64a875d1fc6a0a` (`v7.0.1`) | 上传不可变构建产物 |

Action 的标签仅作为行尾审计注释，真正执行的是 40 位不可变 commit SHA。更新 SHA 时必须从对应 GitHub 官方仓库的 release/tag API 解析并复核完整提交，不得直接改用 `@main` 或 `@vN`。

官方来源：

- <https://github.com/actions/checkout/releases>
- <https://github.com/actions/setup-python/releases>
- <https://github.com/actions/setup-node/releases>
- <https://github.com/actions/upload-artifact/releases>

## 构建顺序

1. 核对 `HEAD == GITHUB_SHA`、拒绝 replacement refs，并从该 commit 的 blob 只提取一次独立源码校验器。校验器固定为 `0500`，记录 SHA-256；之后每次调用前都复核文件类型、属主、权限和摘要。它按 commit tree 逐个核对 tracked blob bytes/mode，不依赖 index 的“干净”结论。
2. 在固定 digest 的官方 Rust 1.98.0 Bookworm 容器内验证 glibc 2.36，并执行 `cargo build --locked --workspace --bins --release`。不能直接在 Ubuntu 24.04 host 上链接，否则 glibc 2.39 二进制无法在 Debian 12 测试环境运行。
3. 直接运行已编译的 `export-openapi`，把 Rust 路由契约写入 `openapi/nodecontroll-v1.json`。
4. 使用仓库内校验器检查 OpenAPI 3.1、必需端点与唯一 `operationId`。
5. 先用 `npm install --global --ignore-scripts` 安装固定 pnpm，再以 `pnpm install --frozen-lockfile --ignore-scripts` 安装依赖。`pnpm-workspace.yaml` 中的 strict peer、engine、依赖成熟期及 exotic dependency 限制继续生效；Actions 不执行任何依赖生命周期脚本。
6. 从 `Cargo.lock`、`pnpm-lock.yaml`、精确 override 目录和 Rust 1.98.0 sysroot 收集许可证正文、来源证据、CycloneDX 与 checksum；缺失、空白/pointer 证据、篡改、stale/unused、锁文件不一致或任何 warning 都会中止。
7. 从刚生成的 OpenAPI 生成 Web SDK，执行 Vue TypeScript typecheck，并构建生产版 Vite 静态文件。
8. `git diff --exit-code` 检查 OpenAPI 和 SDK 是否与仓库提交一致。若 Rust 契约变更却没有提交相应生成物，构建失败。
9. 打包前后都重跑 tracked-source 校验，拒绝 replacement refs，并用 NUL-safe 文件系统遍历核对真实工作树闭包。只有 `target`、根和 Web 的 `node_modules`、`artifacts`、`apps/web/dist` 五棵明确目录树可以作为未跟踪构建输出；allowlist 根本身必须是真实非 symlink 目录。
10. 将项目许可证、第三方 notices/SBOM、ABI/build metadata、两个 Rust 可执行文件、OpenAPI 和 Web `dist` 以提交时间戳制作确定性 tar 包，生成 SHA-256 校验和，并保留 14 天。

工作流故意不启用跨运行依赖缓存，避免来自其他 ref 的可变缓存进入受保护构建。Ubuntu 托管 runner 的系统镜像由 GitHub 维护，无法像容器镜像一样按 digest 固定；因此产物发布前仍须以 VPS 的固定 digest builder 完成完整测试门禁。

CycloneDX 1.6 中的 `components[].licenses[].license.name` 保存收集器规范化后的包声明许可证字符串，包括复合表达式；这里的 `name` 不表示收集器已把该声明判定为规范 SPDX `id`。实际许可证正文、checksum、锁文件 integrity 和精确 override 才组成分发证据闭包。

## VPS 制品复核

正式 verifier 只接受目标公开仓库 `main` 的 fresh standalone full checkout：唯一 `origin` 必须是规范 HTTPS 公共仓库地址，本地 `main` 与 `origin/main` 必须同指 `HEAD`；同时拒绝 shallow、partial-clone promisor、alternates、grafts 和 replacement refs。GitHub workflow run、artifact API、commit-scoped artifact 路径和 raw tar digest 也必须指向这个 push SHA。检出后立即从 commit blob 提取独立源码校验器，逐个核对 tracked bytes/mode；即使修改被 `skip-worktree`/`assume-unchanged` 隐藏也会被发现。测试结束会重复核对源码，并保留最终日志；run 目录由 VPS 宿主 root 管理，不宣称外部不可篡改存储。

依赖输入不再使用跨运行共享缓存。Cargo 使用本轮私有 `CARGO_HOME` 联网执行一次 `cargo fetch --locked`，记录 registry/git 全集后只读挂载；test 和 release 使用不同的 run-scoped target，release 构建断网。pnpm 使用本轮私有空 store，以 `--ignore-scripts --package-import-method=copy` 安装并记录输入全集；后续 Node 工具在从 commit archive 创建的隔离 scratch source 中运行，工具自身的可写元数据目录也是本轮临时目录。闭包内记录的条目在后续阶段发生字节、mode、symlink 或路径变化都会失败。

安装完成后由独立门禁枚举 fresh `node_modules/.pnpm` 中的精确 name/version identity，并与 artifact inventory 的 425 个 npm identity 做集合双向相等检查；脏 store 中额外的 name/version identity 和清单有但未安装的 identity 都会失败。`node_modules`、`.pnpm` 与实际包根必须是非 symlink 目录，canonical realpath 不能离开 checkout、`node_modules` 或 virtual store。隔离 Node source 还按 dependencies/build 两阶段检查额外路径闭包：前者只容许两个 `node_modules`，后者再容许 `apps/web/dist`。

`pnpm-lock.yaml` 的独立解析器只接受审阅过的 v9 canonical 顶层顺序与全集、精确 `'9.0'` 版本值和 block-mapping sections；重复、quoted 或未知顶层 key、非规范顶层 YAML、重复 package/integrity 均拒绝。收集器输出的组件 repository 必须规范化为 absolute `http(s)`/`ssh` URI；可安全转换的 `git://` 来源会改写为 HTTPS，非法或不安全值不能进入 inventory/SBOM。

notices 重建使用从固定 Node image ID 提取的 Node/pnpm runtime，并在固定 Rust image ID 中运行。容器采用 `--network none`、只读 rootfs/source/私有 Cargo 输入；可写 bind mount 只有 run-scoped pnpm runtime store 和本轮重建输出，另有同样不跨运行的 `/tmp` 与 `/cargo-home` tmpfs scratch。Actions notices 与 VPS 结果按目录和文件全集双向核对，再逐文件比较 size、SHA-256 和实际 bytes；任一侧多文件、少文件或内容不同都会失败。

VPS 还会在固定 builder 中独立执行完整 Rust release build、OpenAPI export 和 Web production build。两份 VPS ELF 分别与 Actions ELF 做逐字节 `cmp`，VPS OpenAPI 与提交文件及 Actions payload 三方逐字节相等，VPS `dist` 与 Actions Web 目录按树和内容精确相等。这里保留“可复现”作为硬门；若构建路径造成字节差异，应修正编译输入映射，不能放宽比较。

artifact 中的 `bom.cdx.json` 还要通过 CycloneDX CLI 0.33.1 的官方 v1.6 schema 校验。CLI 由固定 HTTPS URL 获取，执行前核对完整 SHA-256 `bfc8b2538da86fe239bc53658bbb63c1c8c510a293c1e6891aa5bea5d3c58746` 和版本输出。

raw tar 在解包前还要检查 member 路径是否规范且带 `./` 前缀，拒绝重复或别名路径、符号链接、PAX/sparse 以及普通文件和目录以外的类型，并限制压缩包、member 和解压总大小。上述拒绝路径已有发布前单项验证；重收集、SBOM schema 和 archive 门只有在同 SHA Actions artifact 存在后才能形成正式结论，目前尚无正式 Actions artifact/VPS verifier run。

## 明确不在 Actions 执行的内容

工作流不得添加以下命令或其等价物：

- `cargo test`、`cargo nextest`；
- `vitest`、`pnpm web:test`；
- PostgreSQL/SQLite 集成测试；
- Master/Agent 运行时 smoke 或端到端测试。

这些验证的权威结果来自 VPS 的 `/opt/nodecontroll/artifacts/test-runs/<run-id>`。
