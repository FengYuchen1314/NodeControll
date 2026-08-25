# 妙妙屋仓库、构建与交付说明

> 基线：`iluobei/miaomiaowu@0b47f10c52aee10b9f759a593ca5f61a823cbb72`。本章覆盖 Go/TS 之外的根目录、配置、脚本、容器与发布流程。

## 1. 仓库布局

| 路径 | 内容与作用 |
|---|---|
| `cmd/server/` | Go 进程入口、依赖装配、路由、中间件与 CORS。 |
| `internal/` | 后端私有包；业务集中在 `handler`，数据集中在 `storage`。 |
| `miaomiaowu/` | React/Vite 前端项目、站点元数据和构建后注入脚本。 |
| `rule_templates/` | 两个默认 V3 YAML 模板及 Go embed/落盘逻辑。 |
| `subscribes/` | 默认订阅目录、占位 YAML、说明和 embed/准备逻辑。 |
| `configs/` | 完整/精简代理组目录 JSON 的源配置。 |
| `proxy_groups/` | 运行/发布用代理组 JSON 副本。 |
| `scripts/` | 版本同步、Git hook 安装和人工 release 脚本。 |
| `.github/workflows/` | 二进制 Release 与 GHCR 镜像流水线。 |
| `Dockerfile` / `docker-compose.yml` / `docker-entrypoint.sh` | 三阶段容器构建、持久卷、权限降级和运行。 |
| `build.sh` | 本地前后端打包。 |
| `install.sh` | Debian/Ubuntu systemd 安装、更新和卸载。 |
| `quick-install.sh` | 当前目录下载二进制并 nohup 运行的简化安装器。 |
| `test-docker-embed.sh` | 手工构建镜像并检查模板目录。 |
| `token_invalid.yaml` | 无效订阅凭据时可返回的伪装配置。 |

源码快照中还存在 `internal/handler/topbar.tsx`，它不属于 Go 包也没有进入前端 `src` 构建，是一个位置异常的遗留 TSX 文件；不能把它当作生效导航实现，实际文件是 `miaomiaowu/src/components/layout/topbar.tsx`。

## 2. Go 模块与依赖

模块名是本地式 `miaomiaowu`，声明 Go `1.26`。直接依赖职责如下：

| 依赖 | 用途 |
|---|---|
| `MMWOrg/mmwX-plugins/proxyparser` | 解析多协议代理 URI/订阅，社区版代码仍依赖这个独立公开模块。 |
| `dop251/goja` | JavaScript 覆写脚本 VM。 |
| `google/uuid` | UUID 生成。 |
| `gorilla/websocket` | 探针和远程测速器 WebSocket。 |
| `pquerna/otp` | TOTP key、二维码 URI 和验证码。 |
| `x/crypto` | bcrypt 等密码学实现。 |
| `yaml.v3` | YAML AST 解析和稳定写回。 |
| `modernc.org/sqlite` | 纯 Go SQLite driver。 |

`Dockerfile` 注释声称“CGO needed for SQLite WAL”并设置 `CGO_ENABLED=1`，但正式 GitHub 二进制 workflow 使用 `CGO_ENABLED=0`，且 driver 是 modernc 纯 Go 实现。该注释与实际发布链不一致。

## 3. 前端工程配置

| 文件 | 作用 |
|---|---|
| `package.json` | 版本权威源 `0.8.3`、npm scripts 与依赖。 |
| `package-lock.json` | Docker/CI 的 `npm ci` 锁；仓库同时保留 `pnpm-lock.yaml`，但正式链不用 pnpm。 |
| `vite.config.ts` | React SWC、Tailwind、TanStack Router 插件、别名、代理与输出目录。 |
| `tsconfig.json` / `tsconfig.app.json` / `tsconfig.node.json` | 浏览器和 Vite 配置的 TypeScript project references。 |
| `eslint.config.js` | ESLint flat config。 |
| `knip.config.ts` | 未使用导出/文件检查。 |
| `components.json` | shadcn 组件生成配置。 |
| `index.html` | SPA HTML 与社交元标签模板。 |
| `site.json` | 站点名称、描述、URL、favicon、预览图和主题色。 |
| `scripts/inject-site-config.js` | 构建后用正则修改 dist `index.html` 中的 title/meta。 |

`inject-site-config.js` 没有函数声明，而是模块顶层脚本：计算自身目录、同步读取 `site.json` 和构建产物、依次替换 title/description/favicon/OpenGraph/Twitter/theme-color，写回并输出摘要。它依赖 HTML 属性顺序和空格与正则一致，模板稍改就可能静默漏替换。

## 4. 容器构建与启动

### 4.1 Dockerfile

1. `node:20-slim` 复制 package files、执行 `npm ci`，再复制前端并 `npm run build`。
2. `golang:1.26-bookworm` 安装 git/gcc/libc，下载 Go modules，复制全仓库，再从前端 stage 复制 `internal/web/dist`，构建 `/app/server`。
3. `debian:bookworm-slim` 安装 CA、时区、gosu、wget，建立 UID/GID 1000 的 `appuser`，复制 server、规则模板和 entrypoint。
4. 暴露 `/app/data` 与 `/app/subscribes` 卷、8080 端口，healthcheck 用 wget 请求根页。

镜像 `VOLUME` 没有声明 `/app/rule_templates`，但 compose 与 README 都挂载它；仅用裸 `docker run` 时用户需要显式挂载才能持久化模板改动。

### 4.2 `docker-entrypoint.sh`

该脚本没有 shell 函数，按顺序执行：读取 `PUID/PGID`；必要时重建 appuser；创建并递归 chown 三个持久目录；若 `/app/data/server` 是可执行文件则优先使用它（支持容器内自更新）；设置 `DOCKER=1`；最后 `gosu appuser` exec 二进制。

每次启动递归 chown 大目录可能延长启动时间。允许数据卷中的二进制覆盖镜像二进制也削弱不可变镜像和供应链可追溯性。

### 4.3 Compose

Compose 使用 `ghcr.io/iluobei/miaomiaowu:latest`、宿主 root 启动 entrypoint、映射 8080，挂载 `data/subscribes/rule_templates`，并设置 `DATABASE_PATH` 与 `LOG_LEVEL`。`version: 3.8` 在新 Compose 中已属兼容字段。

## 5. Shell 脚本逐函数说明

### 5.1 `install.sh`

| 函数 | 作用与副作用 |
|---|---|
| `echo_info` | 绿色 INFO 输出。 |
| `echo_warn` | 黄色 WARN 输出；当前主流程很少调用。 |
| `echo_error` | 红色 ERROR 输出。 |
| `check_root` | 要求 EUID 0，否则退出。 |
| `check_architecture` | 把 x86_64/amd64 映射到 `mmw-linux-amd64`，aarch64/arm64 映射到 ARM64，其他退出。 |
| `install_dependencies` | `apt-get update` 并安装 wget/curl/systemd。 |
| `download_binary` | 从固定 GitHub tag 下载架构资产到 `/tmp`。 |
| `install_binary` | chmod 后移动到 `/usr/local/bin/mmw`。 |
| `create_directories` | 创建 `/etc/mmw`；`DATA_DIR` 与 `CONFIG_DIR` 实际是同一路径。 |
| `create_systemd_service` | 交互或从 `PORT` 取端口，直接生成 root 用户运行的 service unit。 |
| `start_service` | enable/start，等待 2 秒后检查 active。 |
| `show_status` | 从 unit 解析端口并打印访问/维护命令。 |
| `update_service` | 停服务、备份二进制、下载替换、写 `.version`、可改端口、重启；失败回滚二进制。 |
| `uninstall_service` | 停用 unit、删除二进制，并按交互/`KEEP_DATA` 决定是否递归删除 `/etc/mmw`。 |
| `main` | 根据首参 `update/uninstall/其他` 选择流程。 |

脚本将 service 设为 root，虽启用 `NoNewPrivileges` 和 `PrivateTmp`，仍比容器 entrypoint 的 appuser 权限更大。升级回滚只覆盖二进制，不回滚数据库迁移。

### 5.2 `quick-install.sh`

| 函数 | 作用与副作用 |
|---|---|
| `install` | 下载当前目录 `mmw`、创建 `data`、保存 `.version/.port`，用 nohup 后台启动。 |
| `update` | 用 `pgrep/pkill -f ./mmw` 停进程、备份、下载替换、复用/询问端口并 nohup。 |
| `uninstall` | 用同样进程匹配停止，删除程序/日志；可删除相对 `data/` 与 `subscribes/`。 |
| `main` | 按 `update/uninstall/其他` 分派。 |

它没有 systemd 监管、PID file、checksum/signature 校验或健康检查；`pkill -f` 可能匹配同目录外的命令行，不适合作为目标系统安装方式。

### 5.3 无函数式构建/发布脚本

- `build.sh`：同步版本、删除 `build`、必要时 `npm install`、构建前端、交叉构建 Linux/Windows amd64、复制可选 data/subscribes/config。它直接 `rm -rf build`，且只构建文件列表 `main.go cors.go`；正式 workflow 用 `./cmd/server` 更可靠。
- `scripts/sync-version.sh`：以前端 package version 为源，用 `sed` 同步 Go 常量、两个安装器和前端版本检查常量。
- `scripts/release.sh`：从上一 tag 收集 commit，npm bump、同步版本、改 README、commit/tag/push，再用 `gh release create`。它会直接改变并推送 main，属于维护者人工工具。
- `scripts/install-hooks.sh`：写入 `.git/hooks/post-commit`；commit message 含 `[release]` 时执行 release 脚本。
- `test-docker-embed.sh`：构建固定测试镜像、后台启动 8081、列模板、停止容器；若中途失败没有 trap 保证清理。

## 6. GitHub Actions

### 6.1 `build.yml`

tag `v*` 或手动触发。`lint-and-test` 安装 Go 1.26/Node 20、验证 modules、`npm ci`，然后运行前端 lint/format/knip；三项都 `continue-on-error`，而且该 job 名称虽然叫 test，却没有执行 `go test` 或前端测试。后续构建 Linux amd64/arm64 与 Windows amd64，使用 `CGO_ENABLED=0`；tag 时生成 SHA-256 并发布。另有同 workflow 的 Docker job。

### 6.2 `docker-ghcr.yml`

main、`codex/**`、tag 或手动触发。普通分支默认 amd64，tag 默认 amd64+arm64；通过 Buildx/GHA cache 推 GHCR，关闭 provenance。它与 `build.yml` 的 Docker job 功能重复，tag 可能触发两次镜像构建。

## 7. 运行目录与持久性

| 路径/状态 | 是否应持久 | 说明 |
|---|---|---|
| `data/traffic.db`、WAL/SHM | 是 | SQLite 权威数据。 |
| `subscribes/*.yaml` | 是 | 订阅正文/发布输入。 |
| `rule_templates/*` | 是 | 用户模板和默认模板副本。 |
| `rules/*` 与历史 | 是 | 规则正文与版本。 |
| debug/log 文件 | 可选 | 运维证据，当前有定时清理。 |
| `data/server` | 上游支持但不建议 | 容器内自更新产物，会覆盖镜像 server。 |
| UI session、限速桶、选择会话、缓存、tester WS | 否/部分 | 重启丢失；sessions 和永久 ban 有数据库恢复。 |

## 8. 可复现性与发布缺口

- Docker 基础镜像和 GitHub Actions action 使用浮动 tag，没有在仓库锁 digest；本次审计另行记录了实际测试 digest。
- Release 二进制有 SHA-256，但安装脚本不下载/验证 checksums；也没有签名/SBOM/provenance。
- lint/format/knip 失败不会阻断，Go 测试未进入 CI；当前两个稳定失败测试因此不会阻止发布。
- 前端同时提交 npm/pnpm 锁，实际工具口径不唯一。
- 默认代理组 URL、GeoIP、GitHub、Telegram、Turnstile、探针和订阅源都是运行时外部依赖；离线部署只能使用部分功能。
- 新项目的发布链需锁工具链和镜像 digest，远端执行 `cargo test/clippy`、Vue typecheck/lint/unit/E2E、sing-box config check、迁移测试，生成 checksums、SBOM 和签名后才允许产物发布。
