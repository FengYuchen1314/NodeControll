# 自托管部署、升级、备份与运维方案

## 1. 部署目标

NodeControll 必须在用户自己的服务器、域名、数据库和对象存储上完整运行；不开许可证服务器、不请求官方域名清单、不要求官方 Agent 中继。联网依赖仅是用户主动选择的 ACME/DNS、通知、外部订阅、规则和软件更新源。

支持三类拓扑：

| Profile | 组件 | 适用 |
|---|---|---|
| Single host | reverse proxy + Master + worker + SQLite + local object + Agent + sing-box | 个人/小型，最简单备份 |
| Split control/data | Master(+PG/object) 在控制机；每数据机 Agent + sing-box/Nginx/tc | 推荐生产形态，故障域清晰 |
| HA control | 2+ stateless Master/API/worker + PostgreSQL + S3-compatible object + LB；每机 Agent | 大规模，P7 压测后承诺规模 |

首版必须把 single host 与 split control/data 做成一等交付；HA 不以“容器可多开”冒充完成，需 scheduler lease、SSE、migration、object 和 failover 测试通过。

## 2. 制品与目录

发布制品：

- `nodecontroll-master` Rust 静态/尽量自包含二进制；包含编译后的 Vue assets 或由同版 Web image 提供。
- `nodecontroll-agent` Rust 二进制；不包含任意远控 shell。
- 官方 sing-box source build：stable track + 明确 full-feature preview track，版本/commit/build tags/hash/SBOM/source offer。
- OCI images：`master`, `agent`（可选；host network/privilege 需要显式）、`migrate`；全部 digest 和非 root 用户。
- Compose bundle、systemd units、Nginx/Caddy 示例、安装/升级/卸载/诊断 CLI。
- checksums、signature/provenance、SBOM、license/source 包、OpenAPI 和 migration manifest。

Linux FHS/native 目录：

```text
/usr/bin/nodecontroll-master
/usr/bin/nodecontroll-agent
/usr/lib/nodecontroll/sing-box/<version>/sing-box
/etc/nodecontroll/master.toml                 root:nodecontroll 0640
/etc/nodecontroll/agent.toml                  root:nodecontroll-agent 0640
/etc/nodecontroll/credentials/*               root-only/systemd credentials
/var/lib/nodecontroll/master/{db,objects,backups,tmp}
/var/lib/nodecontroll/agent/{state,artifacts,last-good,tmp}
/var/log/nodecontroll/                         optional; journald preferred
/run/nodecontroll/{master.sock,agent.sock}
```

版本目录不可由 Agent 任意命名；`current` symlink 只在签名/hash/preflight 后原子切换。临时目录与最终目录同 filesystem，便于 fsync+rename。

## 3. 端口与网络

默认监听：

| 端口/Socket | 主体 | 暴露 |
|---|---|---|
| `127.0.0.1:8080` 或 Unix socket | Master HTTP/API/Web | 只给 reverse proxy；可配置 |
| `127.0.0.1:9090` | Prometheus metrics | 默认仅本机/监控网 |
| `:7443` | Agent ingress（WS/HTTP 模式时复用 Master HTTPS 更佳） | mTLS/firewall allowlist |
| `/run/nodecontroll/agent.sock` | Master↔同机 Agent/local helper | Unix permissions |
| sing-box inbound ports | 用户流量面 | 按节点显式开放 |
| sing-box API | loopback/Unix/isolated namespace | 永不公网暴露 |
| Nginx 80/443 | ACME/site/subscription/transport | 按站点配置 |

reverse proxy 保留 WebSocket/HTTPUpgrade、SSE no-buffer、真实客户端 IP；Master 只信任配置的 proxy CIDR/hop。管理 UI 可与订阅域名分离；公开 probe 也建议独立 host/rate policy，但任何自有域名都可用。

出站网络按 purpose 文档化：ACME/DNS provider、Telegram、外部 sources/rules、制品更新、federation peer。默认不需连接 NodeControll 官方服务。离线安装支持将签名 release bundle/制品手工导入 allowlist。

## 4. Master 配置

配置优先级：compiled defaults < TOML file < `NODECONTROLL__SECTION__KEY` 非敏感 override < systemd credential/secret reference。启动输出最终非敏感配置和来源，不输出 secret。

关键配置域：

```toml
[http]
listen = "127.0.0.1:8080"
public_origin = "https://panel.example.com"
trusted_proxy_cidrs = ["127.0.0.1/32"]

[database]
url = "sqlite:///var/lib/nodecontroll/master/db/control.db"

[object_store]
kind = "filesystem"
root = "/var/lib/nodecontroll/master/objects"

[security]
master_key_credential = "nodecontroll-master-key"

[agent]
allowed_modes = ["websocket", "http", "pull", "local"]

[worker]
concurrency = 8
lease_ttl_seconds = 30
```

真实 schema 由 Rust typed config 产生并提供 `config check`。未知 key 默认启动失败；有替代名时给 migration diagnostic。public URL 不能从 Host header 自行推导安全 callback。

上面的片段是完整系统的目标配置域，不代表 P5 可执行文件已经接受其中全部 key。当前已落地的 bootstrap 配置是：

```toml
[http]
listen = "127.0.0.1:8080"
public_origin = "https://panel.example.com"
trusted_proxy_cidrs = ["127.0.0.1/32"]

[database]
url = "sqlite:///var/lib/nodecontroll/master/db/control.db?mode=rwc"
max_connections = 8
acquire_timeout_ms = 5000
statement_timeout_ms = 30000
lock_timeout_ms = 5000

[secrets]
root_key_file = "/etc/nodecontroll/credentials/master.root-key"
setup_token_file = "/etc/nodecontroll/credentials/master.setup-token"

[[secrets.previous_root_keys]]
key_version = 1
path = "/etc/nodecontroll/credentials/master.root-key.v1"

[bootstrap]
setup_token_ttl_seconds = 1800

[auth]
session_idle_seconds = 1800
session_absolute_seconds = 86400
recent_auth_seconds = 300
login_window_seconds = 300
login_block_seconds = 900
login_account_limit = 5
login_ip_limit = 50
login_global_limit = 10000
password_hash_concurrency = 4
digest_key_version = 2
```

`recent_auth_seconds` 的有效范围是 60～3600 秒，并且不得长于 absolute session lifetime。`login_block_seconds` 必须大于或等于 `login_window_seconds`；否则固定窗口尚未结束，封禁却先失效，计数和 `Retry-After` 会出现矛盾，Master 因此在启动前拒绝该配置。`password_hash_concurrency` 的有效范围是 1～64。`digest_key_version` 同时是当前 root/HMAC key version；`previous_root_keys` 最多 3 项，版本必须唯一、正数且严格小于 current。轮换时先保留旧 key 并以更高版本启动；持久 canary 成功解密后会 rewrap 到 current。确认旧版本 session/恢复码均已自然失效或完成替换后，才可从配置和 credential mount 删除旧 key。

首次启动前，安装器或操作者分别生成两个独立的 32-byte 随机值，以 64 位小写十六进制写入 owner-only regular file。root key 是长期数据密钥；setup token 只用于夺取首个 Owner，不能复用同一值。未初始化或 0001 legacy 数据库若不能安全读取 setup-token 文件，Master 在 bind HTTP 前失败；已经 Ready 的数据库不再要求该文件。浏览器通过 `x-nodecontroll-setup-token` header 提交，不放 URL、query、日志或 shell 参数。默认窗口是进程启动后 30 分钟、最大 60 分钟；过期且数据库仍未初始化时需要重启 Master。成功后数据库 latch 永久关闭 bootstrap，操作者应删除 setup-token 文件；Master 不自动删除部署者的 credential mount。反向代理必须使用 TLS，且不得记录该 header。

`public_origin` 是浏览器写请求的精确安全边界，只允许 scheme、host 和可选端口。除 `localhost` 与字面 loopback 地址外必须使用 HTTPS；它必须与浏览器地址栏、反向代理转发的 `Host` 完全一致。配置了 `trusted_proxy_cidrs` 后，受信代理必须发送合法 `X-Forwarded-For`，否则认证请求会失败关闭；未列入的来源即使伪造转发头也只按直连地址限流。

密码登录在任何 limiter 写入前先取得进程内并发许可；槽位满时立即返回带 `Retry-After` 的 429，不把 Tokio blocking pool 当作资源闸门，也不让过量请求扩张 bucket。取得许可后，repository 先只读检查 account、IP prefix、global 三个精确 bucket：已有封禁时不更新 blocked hit、不创建其他 scope 的 row；未命中才在权威事务中按 account→IP→global 占用额度。许可覆盖额度读写、凭据读取与 Argon2id verify，验证结束后释放，不占用安全事件或 session 提交时间。登录成功只向浏览器签发 `__Host-nodecontroll_session` 与 `__Host-nodecontroll_csrf`；数据库保存带版本的 HMAC，不保存原始 token。所有写操作要求同源 Origin/Host，并同时校验 CSRF cookie 与 header。会话有独立 idle 与 absolute deadline，服务端撤销后旧 cookie 不能恢复。

## 5. Agent 连接模式部署

四种模式有相同 task/result 语义，不是四套功能：

- `websocket`：Agent 主动建立长期 mTLS/WSS，适合 NAT；断线 outbox 重连。
- `http`：Master 对 Agent 发 mTLS HTTPS，要求 Agent 可达；firewall 只允许 Master。
- `pull`：Agent 周期长轮询 Master，适合严格出站网络；命令延迟由 interval 决定。
- `local`：同机 Unix socket；仍使用 server identity 和 task envelope，不绕过审计。

安装流程：Master 创建 server draft→一次性 enrollment token→用户在目标机运行带明确 Master URL/fingerprint 的 installer→installer 校验 release signature→创建系统用户/目录/unit→Agent 用 token+device key 兑换证书→capability inventory→UI 显示在线但尚未授权部署。

installer 不接受来自 URL 的任意 post-install shell。命令参数不出现在 process list；token 通过受限临时文件/stdin/systemd credential，兑换后立即删除。Master CA/fingerprint 必须可人工核对。

## 6. systemd 单元和权限

Master unit：`User=nodecontroll`、`NoNewPrivileges=true`、`PrivateTmp=true`、`ProtectSystem=strict`、`ProtectHome=true`、只写 Master state/log；SQLite 时限制单 active writer 进程。

Agent unit：`User=nodecontroll-agent`，只写 agent state/artifact。最小 privileged helper 单独 unit/socket；按实际能力授予 `CAP_NET_ADMIN/CAP_BPF` 或受限 sudo/systemd polkit，仅允许固定 tc/eBPF、unit 和目录操作。没有 capability 时功能报 unsupported，禁止回退 root Agent。

sing-box 用独立 `sing-box` 用户，配置/证书只读、state 可写；`AmbientCapabilities` 根据绑定低端口/TUN 精确配置。Nginx 沿用发行版用户；Agent 只能写 NodeControll 管理的 include 目录并调用 validate/reload helper。

所有 units 使用 restart backoff、start limit、OOM policy、file descriptor limit 和 journald identifier。Master readiness 不因一个 Agent 离线失败；数据库/migration/secret store 不可用则不 ready。

## 7. 容器部署

Compose 默认服务：`master`、可选 `postgres`、reverse proxy 示例；同机 Agent 推荐 native，因为 systemd/Nginx/tc/eBPF 与 host integration 更安全可控。若容器 Agent：

- 需要 host network 和明确 mount 的固定目录/socket；
- 只添加所需 capabilities，不使用 `privileged: true`；
- 不挂 Docker socket、宿主 `/` 或可写 `/etc`；
- privileged helper 仍在宿主并通过受限 Unix socket；
- 文档明确哪些功能（tc/eBPF/systemd）不可用/降级。

images multi-stage、read-only rootfs、non-root、tmpfs `/tmp`、固定 uid/gid、healthcheck、digest pin。配置/密钥用 secrets/file mounts，不 baked 到 image/env dump。

## 8. SQLite、PostgreSQL 与对象存储

SQLite：single host，WAL、foreign_keys=ON、busy timeout、max connection=1；DB/objects 必须在本地 POSIX filesystem，不放 NFS/SMB。定时 checkpoint 不替代 backup API。若 Master 多副本，禁止 SQLite。

PostgreSQL：HA/规模部署；TLS、专用 role/schema、最小权限、pool/statement/lock timeout；migration 使用 advisory lock。PITR/WAL/备份由用户数据库运维体系管理，NodeControll 仍提供逻辑 manifest/restore inspect。

filesystem object store 与数据库备份要保持 manifest 一致；S3-compatible 使用 versioning、SSE、private bucket、最小 access key、multipart abort lifecycle。公开订阅由 Master 鉴权后流式返回或签发极短的一次性内部 URL，不能让 bucket public。

## 9. Nginx、TLS 与域名

用户可使用任意自有域名或纯 IP（部分 TLS/客户端功能会受限）。站点管理只写 `/etc/nginx/nodecontroll.d/<site-id>.conf` 或配置的隔离根，模板字段 typed；禁止任意 `include`、Lua、shell 和越界 path。

部署步骤：render staging→path/upstream/domain/cert 引用检查→`nginx -t`→备份 active/hash→atomic rename→reload→HTTP/TLS health→失败 rollback+revalidate/reload。WS/HTTPUpgrade/header/timeout 按 transport profile 生成。

证书方式：ACME HTTP-01/DNS-01、导入、自管理 path。DNS provider credential 在 secret store；challenge record 精确 name/value、完成后 best-effort cleanup。续期 scheduler 有 jitter/lock/retry/到期告警。证书 private key 永不经普通 API 下载，除非 owner recent-auth + explicit export policy。

## 10. 官方 sing-box 制品与版本轨

基线和兼容策略见 [SINGBOX_COMPATIBILITY.md](./SINGBOX_COMPATIBILITY.md)。截至设计日：生产基线固定官方 stable `v1.13.19`；完整 per-user connection API 的开发预览固定官方 `v1.14.0-beta.17`，正式完整功能发布门要求对应 `1.14.x` stable 或把 beta 明确标 preview。

项目从官方 tag/commit 构建，默认官方 tags 加 `with_v2ray_api`，保存 Go version、module lock、commit、dirty=false、tags、binary hash、SBOM。Master release manifest 定义允许组合；Agent 不下载未经 allowlist 的版本。

升级：download staging→hash/signature→`version`/`check`→备份 binary/config→停止/原子切换→启动/health/traffic/connection checks→失败 last-good rollback。配置先按目标版本 compile；禁止 binary 先升、配置随后碰碰运气。

## 11. tc/eBPF 和内核前置条件

完整 per-user 平滑限速目标 Linux ≥5.10，具有 BTF、tc clsact/HTB/fq 和 Agent 所需权限。安装/diagnostic 检查 kernel、BTF、qdisc、interfaces、cgroup/namespace、loopback/Nginx path、offload。结果写 capability，不自动修改不相关 qdisc。

部署 classifier 前保存现状；只管理带 NodeControll handle/prefix 的 qdisc/class/filter；冲突先报告 impact plan。双向 flow map 来自 sing-box 1.14 官方 connection stream；map TTL/epoch/restart 行为受测。Hysteria2/UDP/多路复用等不能可靠分类时使用协议 native 限制或报告 degraded，绝不显示假成功。

卸载只删除带项目 owner marker 的 tc/eBPF 对象；不能递归清空接口全部 qdisc。

## 12. 备份、恢复和保留

备份 profile：

- 最小：DB + object manifest + settings + encrypted secrets；不含可重新下载的 core artifact。
- 完整：最小 + immutable artifacts/source revisions/audit checkpoint/license/source manifest。
- support bundle：不等于备份，不含 secrets/credential/full user data。

schedule、retention、destination、encryption 独立配置；备份成功必须完成 read-back hash，周期性恢复到隔离实例。备份 key 与数据分开；local backup 还需异机/离线副本才算灾备。

恢复遵循 [MIGRATION.md](./MIGRATION.md)：inspect→新目录/DB→migration→semantic validation→切换。RPO/RTO 默认不虚构，部署向导让用户选择并据此给出 schedule/容量建议；P7 实测后写出该 profile 的数值。

## 13. 升级、降级与兼容窗口

发布版本遵循 SemVer；每个 release note 包含 DB/API/Agent/core/config/backup 兼容矩阵、breaking/migration、downtime、rollback deadline。支持 N/N-1 Agent 与 Master 滚动窗口；不支持版本进入 `incompatible`，不派任务。

升级顺序：备份+restore inspect→下载/验证制品→Master migration expand→Master rolling/single restart→Agent 分批 canary→sing-box config compile/canary→全量→contract/traffic checks→contract migration 在后续 release。

canary 至少一个非关键 server 或用户指定 maintenance group；失败停止 rollout，不自动越过。binary downgrade 仅在 DB compatibility matrix 允许时；否则恢复升级前备份到新目录。

## 14. 可观测性与值班

Master/Agent structured JSON logs、trace/request/job/task IDs、Prometheus metrics、OpenTelemetry traces（可选 exporter）。详细指标和 SLO 见 `OBSERVABILITY.md`。

最小告警：Master not ready、DB/secret/object failure、job backlog/failure、Agent offline/clock/drift、core crash/reload rollback、traffic ingest gap/reset anomaly、limit degraded、certificate expiry/renew failure、source sync sustained failure、backup/restore rehearsal failure、disk/inode、security event/secret canary。

runbook 必须给出 detection→safe diagnosis→containment→recovery→verification，不建议直接编辑数据库/Agent state。diagnostics CLI 默认只读并 redact。

## 15. 安装前检查和容量

`nodecontroll doctor` 检查：arch/OS/kernel/time/DNS、CPU/RAM/disk/inode、端口、防火墙、reverse proxy、SQLite filesystem/PG、object permissions、systemd/capabilities/BTF/tc、existing sing-box/Nginx 冲突、outbound reachability 和 TLS public URL。

初始建议（待 P7 压测修正）：single host 2 vCPU/2 GiB/10 GiB 可作小型起点；启用本机编译、测速、长日志/细 traffic、多个 core 时需要更多资源。UI 必须根据实际 daily traffic rows/artifacts/log retention 给容量预测，而不是只列固定最低配置。

磁盘水位 80% warning、90% critical；critical 时暂停非关键 source/artifact/speed jobs和新 backup，但不能删 audit/账本/last-good。清理由 retention job 按对象类型执行并审计。

## 16. 卸载与数据可恢复性

卸载分：移除服务但保留数据（默认）、导出备份后移除、purge。purge 要显示并解析绝对目标目录、owner recent-auth/CLI typed confirmation；只删除 manifest 记录的 NodeControll 文件、unit、tc/eBPF owner objects，不碰用户其他 Nginx/site/qdisc。

Agent revoke 与 host uninstall 分开；Master 上先 revoke identity，host 再停止/删除服务。离线 Agent 无法由 Master 自动清宿主，UI 提供签名卸载说明但不声称已删除。

## 17. GitHub Actions 编译与 VPS 验收

正式可分发制品只由 GitHub Actions 编译。测试、数据库联调、部署演练和验收只在维护者私有 VPS 的 `/opt/nodecontroll` 执行；主机地址、SSH 用户和 key path 不进入仓库。本地工作区只编辑、审阅源码和文档，不提供编译或测试通过证据。Actions 中用于生成制品的 OpenAPI/SDK 一致性与 TypeScript 检查属于 build-integrity gate，不能替代 VPS 的静态和行为验收。

release candidate 从一次 `main` push 开始。Actions 对 clean checkout 使用锁定的 Rust/Node/pnpm 和固定 builder digest，完成 Rust release binaries、OpenAPI、typed Web SDK、Vue production dist、许可证与组件清单。payload 内的 `BUILD-METADATA` 记录 run ID、run attempt、commit、target、glibc、builder 和 source，`CONTENTS.sha256` 覆盖除清单自身外的全部文件。最终文件是单个 `nodecontroll-linux-x86_64-glibc2.36.tar.gz`；`upload-artifact` 使用 `archive: false`，GitHub artifact API 的 digest 因而直接对应 raw tar SHA-256。当前 verifier 只接受 `run_attempt=1`；失败后以新 push 产生新 run，不用 rerun 混合旧 attempt 的同名 artifact。

VPS 只接受与该 push 完全相同的 commit。checkout 必须位于 `/opt/nodecontroll/checkouts/<sha>`、是含独立 `.git` 的新 clone，初始 tracked/untracked 与 ignored 输入都为空；verifier 原子 claim 后不允许任何成功或失败重试复用。raw tar 位于 `/opt/nodecontroll/artifacts/github-actions/<sha>/`，进入 run 后先固化为 `provenance/` 下的 0444 快照；后续不再读取可变下载路径。verifier 从 GitHub API 核对仓库、workflow path、push/main、run conclusion/attempt、head SHA、run ID、artifact ID 与快照 digest。archive 在解包前检查规范路径与 `./` 前缀、重复/别名 member、声明父目录、类型、PAX/sparse 和压缩/单文件/解压总 size；解包后再检查 `BUILD-METADATA`、`CONTENTS.sha256`、license/Rust runtime notices/SBOM 和文件全集。Master/Agent 还要通过 x86-64 ELF、动态库 allowlist、解释器和最高 GLIBC 2.36 检查。

同一 fresh checkout 先在固定 Node image ID 下执行 `pnpm install --frozen-lockfile`。独立门禁随后从 fresh `node_modules/.pnpm` 枚举精确 name/version identity，并与本次 artifact inventory 声明的完整 npm identity 集合做双向相等检查；额外或缺失 identity 都失败。`node_modules`、`.pnpm` 与实际包根必须是非 symlink 目录，canonical realpath 必须留在各自上级边界内，因此脏 virtual store、symlink 和目录逃逸不能进入许可证闭包。

`pnpm-lock.yaml` 只接受审阅过的 v9 canonical 顶层顺序/全集、精确 `'9.0'` 版本值和 block-mapping sections；重复、quoted 或未知顶层 key、非规范顶层 YAML、重复 package/integrity 都拒绝。repository metadata 必须规范化为 absolute `http(s)`/`ssh`/`git` URI，非法或不安全值失败。CycloneDX `license.name` 仅保存收集器规范化后的包声明许可证字符串，不表示规范 SPDX `id` 判定；正文、来源、checksum 和精确 override 才是法律证据。

notices 重收集使用从固定 Node image ID 提取的 Node/pnpm runtime，并在固定 Rust image ID 中运行。容器断网，rootfs、source 和共享 Cargo cache 只读；run-scoped pnpm store 与本轮重建目录可写。Actions notices 与 VPS 结果按目录/文件全集、逐文件 size、SHA-256 和实际 bytes 双向比对。SBOM 另由固定 CycloneDX CLI 0.33.1 校验官方 v1.6 schema，CLI 完整 SHA-256 固定为 `bfc8b2538da86fe239bc53658bbb63c1c8c510a293c1e6891aa5bea5d3c58746`。这些拒绝路径已有发布前单项验证，较早公开基线也已完成正式 artifact/VPS verifier run；每个新 SHA 仍须以同 SHA Actions artifact 重新通过 provenance、archive、SBOM 与重收集门，当前 C1 SHA 尚未发生。

当前 P5 verifier 的落盘范围包括 schema 3 `manifest.json`、`commands.tsv`、逐阶段 `logs/`、GitHub API 与 raw-tar 快照 `provenance/`、本轮解包的 `compiled/`、C1 HTTPS rotation/logout 的冻结 `browser/` evidence、`checksums.txt`，以及成功时的 `COMPLETED_AT` 或阶段失败时的 `FAILED_STAGE`。runtime smoke 结束后先停止 Master、冻结最终日志，再扫描 setup token、root key、已知密码和 PHC 前缀；失败清理无法完成同等证明时留下 `SECRET_SCAN_FAILED`。JUnit、coverage、HTML reports、完整 Playwright traces/screenshots、安全、性能、Compose/native、Agent/core/Nginx/tc、备份与升级回滚证据尚未由当前脚本生成；这些目录和结论要等相应工作包真正实现后再加入。

部署完成仍要求 single-host Compose/native 和 split Master/Agent 各从零安装一次；任意自有域名、无官方服务运行；升级、Agent rotation、core rollback、backup restore、uninstall-preserve 都有可重复脚本、日志和验收报告。P5 的最小 provenance 与 smoke 不能替代这些发布门。
