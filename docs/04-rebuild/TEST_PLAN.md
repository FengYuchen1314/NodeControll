# 编译、测试与系统验收计划

## 1. 唯一验收环境与证据

正式可分发制品只由 GitHub Actions 编译。Actions 可以执行生成制品所必需的锁文件检查、OpenAPI/SDK 一致性检查和 TypeScript 编译检查；这些步骤只保证编译输入与输出自洽，不作为测试通过证据。格式、静态、单元、集成、E2E、性能和安全验收均在维护者私有 VPS 的 `/opt/nodecontroll` 执行。主机地址和 SSH 身份由私有配置注入，本地工作区只编辑、审阅，不产生验收结论。

P5 当前的 `tools/vps_verify.sh` 实际写入以下内容：

```text
/opt/nodecontroll/artifacts/test-runs/<run-id>/
  manifest.json       # schema v3、source SHA、一次性 checkout/ignored 标记、builder/lock/artifact/run attempt、browser gate、host、started_at
  commands.tsv        # 每个已执行 stage 的开始命令与结束状态
  logs/               # 每个 stage 的 stdout/stderr；Master 容器日志单独保存
  provenance/         # GitHub run/artifact API 原始 JSON，以及本轮 0444 raw-tar 快照
  compiled/           # 本轮从已核验 raw tar 解出，并以只读 mount 消费的制品
  browser/            # C1 HTTPS rotation/logout 的冻结 DB/dump/log/TLS 证书、握手与无秘密 evidence
  checksums.txt
  COMPLETED_AT        # 全部当前阶段通过后才出现
  FAILED_STAGE        # 某个 run_stage 失败时出现
  SECRET_SCAN_FAILED  # 仅失败清理路径无法证明最终 runtime log 无 secret 时出现
```

`manifest.json` 启动时写入 `status=running`；正常收尾原子更新为 `completed`，异常退出更新为 `failed`。成功还必须同时存在 `COMPLETED_AT`，某个 `run_stage` 失败时另有 `FAILED_STAGE`；直接前置检查失败不伪造阶段名。C1 已加入真实 Playwright HTTPS 行为与冻结 evidence，但没有把通用 screenshots/traces 冒充现有报告。JUnit、coverage、HTML reports 和完整 Playwright trace/video/screenshot 矩阵仍是后续工作包目标，只有 runner 真正落盘并纳入 checksum 后才加入目录合同。

成功 run 的 ID、commit、GitHub run/artifact ID、raw tar SHA 和实际覆盖的 requirements IDs 才能写入 `docs/00-project/PROGRESS.md`。失败记录保留完整 stage 日志、fixture/seed 和固定 image digest，不得只摘录最后一行。

### 1.1 正式证据链

一轮正式验收必须把以下对象锁到同一 commit：

1. GitHub Actions run 来自允许的公开仓库和 `.github/workflows/build.yml`，事件为 `push`，分支为 `main`，状态 `completed/success`；run 的 `head_sha` 必须等于 VPS checkout 的完整 HEAD。当前证据格式只接受首次执行 `run_attempt=1`，失败后必须新 push，不能用同一 run ID 的 rerun 混入旧 attempt artifact。
2. Actions 使用 clean checkout、锁定 Rust/Node/pnpm 与 builder digest，编译 Rust release binaries、导出 OpenAPI、生成 SDK 和 Vue production dist。Actions 中的合同检查属于制品生成门，不替代 VPS 静态验收。
3. `upload-artifact` 以 `archive: false` 上传单个 `nodecontroll-linux-x86_64-glibc2.36.tar.gz`。GitHub artifact API 的 `sha256:` digest 必须等于 VPS 上 raw tar 的 SHA-256；artifact ID 还必须归属于上述 run，不能只凭文件名或下载目录判断来源。verifier 先把外部文件固化成 run 目录内 0444 快照，后续 hash、API 对照、成员检查和解包只读该快照，消除下载者替换原路径的窗口。
4. VPS checkout 位于 `/opt/nodecontroll/checkouts/<sha>`，artifact 位于 `/opt/nodecontroll/artifacts/github-actions/<sha>/`。checkout 必须是带独立 `.git` 目录的新 clone，tracked/untracked 状态为空且测试前没有任何 ignored 文件；verifier 原子创建 `.git/nodecontroll-verifier.claim`，成功或失败后都禁止复用。测试后再检查源码 drift。verifier 拒绝 archive 路径穿越、符号链接和非常规文件，再解压到本轮 `compiled/`。
5. `BUILD-METADATA` 的 run ID、run attempt、commit、target、glibc、builder、source 必须各出现一次并符合 API/基线；`CONTENTS.sha256` 必须覆盖除清单自身外的全部 payload，不能有未列文件或缺失文件。许可证、Rust runtime notices、第三方 notices、SBOM/依赖清单也属于 payload。
6. Master 与 Agent 必须是可执行的 x86-64 ELF；动态库只能来自 allowlist，`readelf` 报告的最高 GLIBC version 不得超过 2.36。制品名称中的 glibc 基线是部署约束，不代表任意 Linux 发行版都兼容。
7. 通过上述 provenance 门后，VPS 才运行 fmt/clippy、Rust tests、双数据库合同、OpenAPI/docs/Web 静态与单元检查，以及直接消费 Actions Master/Agent/OpenAPI/Web 的 artifact smoke。Master 停止后先冻结最终容器日志，再扫描 setup token、root key、已知密码和 PHC 前缀；失败 cleanup 无法完成同等扫描时必须留下 `SECRET_SCAN_FAILED`。任何本地或其他 commit 的 binary/dist 都不能补充为正式结果。

P5 verifier 已实现上述最小链路，并已有较早公开基线的成功提交级证据；每个新 SHA 仍必须重新取得 main push、Actions run 和对应 VPS `COMPLETED_AT`，不能沿用旧 run。C1 已加入最小 HTTPS Playwright rotation/logout 门；完整浏览器矩阵、Compose/native、Agent handshake、sing-box/Nginx/tc、安全、备份、升级回滚和性能报告仍由后续工作包补齐。

## 2. 测试金字塔与门

| 层 | 范围 | 每次变更 | 阶段/发布 |
|---|---|---|---|
| format/static | rustfmt/clippy/deny、TS format/eslint/type、OpenAPI/lint、docs links | 是 | 是 |
| unit/property | domain、parser/encoder、policy、config、UI components | 是 | 是 |
| repository/integration | SQLite+PG、object、jobs、HTTP、Agent protocol | 相关变更 | 全量 |
| contract/compat | OpenAPI、protobuf、backup、Agent N/N-1、core/client matrix | 相关变更 | 全量 |
| component system | Master/Agent/sing-box/Nginx/tc/source mocks | smoke | 全量 |
| browser E2E/a11y/visual | 关键用户旅程、移动端、权限、失败恢复 | smoke | 全量 |
| security/resilience | threat corpus、fault/restart/backup/rollback | 相关 smoke | 全量 |
| performance/soak | API、publish、traffic、connections、fleet、24h | 否 | milestone/release |

单元覆盖率不作为唯一质量指标。初始 line/branch 门由 P5 baseline 建立（核心 domain/parser/security 目标更高），随后只能通过有说明的 waiver 下调；需求完成必须有行为测试和 traceability，不因覆盖率数字高而通过。

## 3. 可复现 VPS 测试拓扑

测试网络通过 Compose/namespace 创建：

- `master-sqlite`、`master-pg-a/b`、PostgreSQL、S3-compatible object；
- 至少两个 Agent：WS/pull（其余模式合同复用并单测），独立 server identity；
- 官方 sing-box stable 和目标 1.14 track；Nginx；privileged test namespace 中 tc/eBPF；
- fake ACME/DNS/Telegram/MCP/federation/source servers；
- Playwright browser；traffic clients/servers/iperf-like deterministic fixtures；
- network fault proxy（delay/drop/reset/duplicate/429/5xx）和 clock abstraction。

不向真实 Telegram/DNS/ACME 或外部订阅发送常规 CI 请求。真实互操作用手工受控 profile、专用测试凭据和明确清理。

VPS 宿主上的其他进程/项目不在测试清理范围。所有网络、volume、process、tc 对象带 run ID；teardown 只处理 manifest 解析后的绝对路径/owner marker。

## 4. Rust 后端测试

### 4.1 Domain/unit/property

- newtype/enum/bytes/time/revision checked arithmetic、serialization、unknown enum/API evolution。
- password/session/token/MFA/recovery/re-auth、authorization scope+relationship+state 矩阵。
- package/entitlement effective policy：多个套餐、override、时间窗、pause/reset、最小/最大/并集/交集规则。
- raw traffic→dedupe→delta→epoch→multiplier→ledger→aggregate；reset/wrap/out-of-order/duplicate/negative/adjust/reverse。
- config desired/reported/last-good、revision/CAS、compile diagnostics、deployment state machine。
- route first-match/order/shadow/cycle、selector scheduler、tunnel/WARP capability。
- certificate/site/source/profile/job/federation 状态机的非法 transition。
- canonical JSON/CBOR/hash/signature/idempotency/audit chain deterministic。

property tests 使用固定失败 seed 输出；checked arithmetic 和 parser 不允许 panic。时间、随机、DNS、HTTP、process 通过 port/adapters 注入，不在 unit test 访问真实时钟/网络。

### 4.2 Repository contract

同一套 contract 对 SQLite/PG 执行：CRUD、unique/FK/check、soft delete/revoke、revision conflict、pagination/order、transaction rollback、concurrent update、job lease、outbox claim、ledger append、audit hash chain。

每个 migration：empty latest、逐历史版本升级、失败 rollback、新旧 binary compatibility、SQLite foreign_keys/WAL、PG lock timeout。test fixture 包括 non-ASCII、最大 bytes、clock boundary 和 orphan legacy import。

### 4.3 HTTP/API

对 [API_CONTRACT.md](./API_CONTRACT.md) 每个 operation 自动生成基础矩阵：schema success、401、403、object IDOR、400/422、404、If-Match 428/409、Idempotency replay/body conflict、rate limit、audit、redaction。

额外：cookie/CSRF/Origin/CORS/trusted proxy、pagination cursor tamper、ETag/304、SSE resume/dedupe/resync、multipart/body/time limits、problem details、OpenAPI response conformance、unknown fields。

## 5. Agent 协议与远端执行

协议 golden vectors 覆盖 protobuf/canonical signature、mTLS enrollment/rotation/revoke、sequence/expiry/audience/body hash。四种 transport 跑同一 compliance suite：connect/heartbeat/capability、task dispatch/ack/progress/result、file chunks、backpressure/reconnect/resume。

必须注入：duplicate/out-of-order/delayed/lost envelope、Master/Agent restart、network partition、full outbox/disk、clock skew、certificate grace/expiry、N/N-1 protocol、task cancellation race、resource lease conflict。

每种 privileged task 有：valid、schema boundary、argument/path/symlink injection、wrong server/revision、stale/cancel、helper permission denied、partial operation、last-good rollback。确认 Agent 永不执行任意 shell/URL/path。

两个 Agent 的隔离测试：A token/cert/task/file/result 不可用于 B；被 revoke A 不能重 enrollment/claim pending；被攻陷 simulated Agent 不能读取 Master secret/其他 server data。

## 6. sing-box、协议和连接测试

### 6.1 Build/compat

- 从官方固定 tag/commit、clean tree、固定 Go image 构建 default tags + `with_v2ray_api`；记录 binary hash/version/tags/SBOM/license/source。
- stable `1.13.19` 和目标 `1.14` track 分别跑 config schema/feature matrix；1.14 stable 发布前替换 beta pin并重跑。
- 不支持字段/组合在 compile 阶段给 422 diagnostic；XHTTP 不映射成 HTTP，Snell version/AnyTLS 起始版本正确。

### 6.2 Protocol matrix

对支持的每一 server 入站组合：生成 config→`sing-box check`→启动→真实客户端握手→双向 TCP/UDP（适用时）→TLS/Reality/transport→流量/user attribution→关闭/reload。至少覆盖 Shadowsocks、VMess、VLESS Vision、Trojan、Hysteria2、AnyTLS、Snell（版本允许时），及 WS/HTTP/HTTPUpgrade/gRPC/QUIC 的合法交集。

证书有效/过期/SNI/ALPN、IPv4/6、DNS、特殊 credential、端口冲突、错误 client、重连/多路复用、大 payload/半关闭都进入矩阵。测试输出从实际 config capability 自动枚举，禁止手工漏组合。

### 6.3 API/reload/last-good

验证 V2Ray stats non-reset/delta/user/inbound/outbound、1.14 connection stream/user/source/bytes/close/select outbound、Clash connection/status 辅助接口。kill/restart/reload 产生 epoch，部署前 flush；SIGHUP disruption 和连接影响有实测证据。

invalid config/binary/crash/health timeout 必须自动保留/恢复 last-good；desired/reported/UI/job outcome 一致。旧连接与 selector scheduler 切换行为记录。

## 7. tc/eBPF、流量和限制测试

在隔离 network namespace/VM 能力中，不修改 VPS 其他接口：

- mapping：TCP/UDP、IPv4/6、direct inbound、Nginx WS/HTTPUpgrade loopback、NAT、connection close/reuse、Agent/core restart、map TTL/epoch。
- 限速：单用户/多用户/多连接/多节点、上/下行、burst、长期平均、公平性、动态改速/取消、并发/源 IP/设备限制。
- protocol：VLESS/Trojan/VMess/Hy2/AnyTLS/Snell 等可归属性；多路复用/QUIC/Hysteria UDP path。
- fault：classifier/helper/permission/BTF/qdisc conflict/map full；状态必须 degraded/unsupported，不得显示 enforced。
- cleanup/rollback：只删除 run owner objects，恢复 previous qdisc；重复 apply/remove 幂等。

流量对账：客户端发送已知字节、sing-box API、tc counters、raw measurement、ledger、aggregate 比较容差与原因；upload/download、reset/reload、倍率、baseline/adjustment/reversal、周期边界/时区、跨服务器均验证。测速流量按配置排除/计入要可解释。

## 8. 订阅 IR、来源与客户端互操作

按照 [SUBSCRIPTION_IR.md](./SUBSCRIPTION_IR.md)：

- parser fixture/golden/property/fuzz：URI/base64/Clash/Meta/sing-box/Surge/QX/Loon/Stash；畸形/YAML/zip/Unicode/IPv6。
- normalize/fingerprint/dedupe/name/order/filter/package/private node/capability diagnostics。
- templates pure sandbox、WASM no fs/network/env/clock、fuel/memory/time/output；secret handle isolation。
- encoder byte deterministic + round-trip/schema lint；每个 target profile 的 unsupported/warning/fatal。
- source staging/304/diff/atomic activate/last-good/concurrent sync；SSRF redirect/rebinding/metadata/private/IPv6 corpus。
- artifact cache user isolation、ETag/304、token revoke/expiry/rotate、UA profile、gzip、filename/header/log redaction。

真实客户端互操作表记录客户端名/版本/OS、profile、协议组合、import、connect、DNS/route、结果和 artifact hash。没有自动 linter 的格式至少两个实际客户端版本或 decoder round-trip + 一个实际版本。

## 9. Vue/Vuetify 测试

- Type/lint/build、generated client 与 OpenAPI sync、i18n missing/unused、bundle budget。
- shared components 全状态：loading/empty/error/stale/permission/capability/mobile/dark/keyboard。
- form schema/conditional/secret semantics、API JSON pointer errors、409 three-way conflict、422 compile diagnostic。
- route guards/back-forward/deep link/refresh/query state/cancel；SSE disconnect/resume/job recovery。
- visual snapshots：中文/英文、light/dark、1440/768/390/360；只对稳定 fixture，动态时间/ID固定。
- axe WCAG 2.2 AA 自动检查 + 键盘 checklist + NVDA/VoiceOver 手工 milestone。
- performance：route chunks、LCP/INP/CLS、large table/log、low network/CPU。

Playwright page objects 以 role/label 查询，不依赖 CSS 实现。失败保留 trace/video/screenshot/console/network（token redacted）。

## 10. 端到端旅程

| E2E | 旅程/验收 |
|---|---|
| E2E-001 | 首次 setup→owner→恢复码确认→logout/login/MFA/WebAuthn/session revoke |
| E2E-002 | 创建服务器→WS enrollment→capability→core install→compile/deploy→reported in sync |
| E2E-003 | 创建每类入站/节点→真实客户端 connect→traffic/user attribution→subscription download |
| E2E-004 | 出站/selector/urltest/route reorder/WARP/tunnel→真实路径/失败诊断 |
| E2E-005 | 用户+多套餐→effective policy→订阅节点集→流量/速率/并发/IP/设备限制 |
| E2E-006 | external source sync→diff→provider/profile→template/rule/WASM→多客户端输出 |
| E2E-007 | node/server speed test→samples→history→公开 probe projection/quota |
| E2E-008 | ACME fake DNS→证书→Nginx site validate/deploy→WS traffic→renew/rollback |
| E2E-009 | Telegram pairing/Mini App/通知；MCP read/write/危险确认/revoke |
| E2E-010 | 两个实例 peer→share/import→断线/rotate/revoke，无官方服务 |
| E2E-011 | 妙妙屋 snapshot inspect→import→semantic diff→cutover→rollback |
| E2E-012 | backup→损坏检测→新实例 restore→session/credential decision→service smoke |
| E2E-013 | Master/Agent/core/DB/object/network故障→告警→last-good/retry→一致恢复 |
| E2E-014 | upgrade N-1→N（Master/Agent/core/config/DB）→canary→rollback compatibility |
| E2E-015 | owner/admin/operator/support/auditor/member 全 UI/API IDOR/字段投影矩阵 |

每个旅程同时链接 `MMW-*`、`MMWX-*`、`PRO-*`/`NOLIC-*` 和目标 `NC-*` 需求。

## 11. 安全测试

按 [SECURITY.md](./SECURITY.md) SEC-001～SEC-024：

- auth/authz/session/token/MFA/CSRF/CORS/CSP/XSS/trusted proxy/rate abuse；
- SQL/command/path/header/template/log/CSV 注入；upload/archive/restore；
- SafeHttp SSRF全编码/IP/redirect/rebinding/slow/oversize；
- Agent impersonation/replay/task smuggling/file/symlink/helper；制品 hash/signature/SBOM；
- secret canaries 自动放入各类 secret，扫描 API/log/trace/metrics/audit/job/task/artifact/frontend dist/source map/backup/support bundle；
- Telegram/MCP/federation signature/replay/confused deputy/untrusted content；公开 probe scan/PII；
- SAST、dependency/container vulnerability、license、secret、IaC/config scan；
- 独立 manual review/penetration 在 RC，所有 critical/high 关闭。

security test 失败可能产生敏感 exploit fixture，artifact 权限/保留受限；报告公开部分不含可用 token/内网细节。

## 12. 性能、容量和耐久

### 12.1 场景

建立小/中/目标上限数据集（P5 初值，P7 校准）：用户 100/10k/100k；节点 100/10k/100k；服务器 10/1k/5k；live connections 1k/50k/目标；traffic records/day；profiles/source nodes/artifact sizes。

压测：API read/write/pagination/search、dashboard traffic series、source parse/publish、并发 subscription downloads/cache miss、job/Agent heartbeat/task fanout、traffic ingest/aggregation、SSE clients、audit/ledger、backup/migration。

记录 throughput、p50/95/99、errors、CPU/RSS/FD、DB lock/pool/IO、object bytes、queue lag、network、artifact and cost per operation。结果与 [OBSERVABILITY.md](./OBSERVABILITY.md) 初始 SLO/前端预算比较。

### 12.2 Soak 与泄漏

至少 24h milestone soak：稳定 traffic/connection churn、source schedules、downloads、jobs/SSE、Agent reconnect；注入周期 core reload和一 Agent flap。检查 RSS/FD/task/map/WAL/table/object/log增长、duplicate ledger、stuck leases、sequence/epoch、cleanup。

HA profile 另做 Master kill、PG failover、worker lease takeover、SSE reconnect、duplicate delivery exact-once-effect。未通过前文档标 experimental。

## 13. 备份、迁移和灾难测试

- SQLite online backup/WAL busy、PG logical/PITR adapter、object manifest；加密 read-back。
- wrong key、bit flip、truncate、missing/extra object、zip slip/symlink/old/new schema、磁盘不足。
- 恢复到全新路径/DB，所有行/对象/hash/secret/订阅/core smoke；sessions/Agent/peer rotation policy。
- 社区版 empty/small/complex/corrupt fixture 26 表全对账；旧 DB hash 不变；idempotent rerun。
- blue/green freeze/final delta/proxy switch/Agent ownership/rollback；新写不反灌。
- 每个正式 release 至少一次前版→当前升级和允许窗口内 rollback。

## 14. 需求追踪与缺陷门

`REQUIREMENTS_TRACEABILITY.md` 的每个需求必须指向设计、实现路径、测试 ID、最后 run 和状态。状态只允许 `planned/implemented/verified/deferred-blocked`；“页面存在”不等于 verified。

发布阻断：任何未解释需求；critical/high security；数据丢失/账本错误/secret 泄露；flaky test；E2E/backup/rollback/core protocol matrix失败；SLO/budget超标无 owner；OpenAPI/protobuf/backup incompatibility；license/source缺失。

flaky test 不允许简单 rerun 变绿。隔离时要登记 owner、issue、seed/频率、最晚修复阶段；涉及 auth、ledger、migration、Agent/core、security 的 flaky 不能隔离发布。

## 15. 每阶段完成证据

| 阶段 | 必要证据 |
|---|---|
| P5 skeleton | main push Actions release build 与 raw tar/API provenance；同 SHA clean VPS checkout；SQLite/PG、API/Vue、OpenAPI 与编译制品 smoke；Agent handshake 未实现前 P5 不完成 |
| P6 每个 work package | unit/repository/API/UI/E2E、trace IDs、migration、docs/requirements/progress、无新 security debt |
| Feature complete | 358+ source constraints 全 mapped；E2E-001～015；protocol/client matrix；license-free offline self-host |
| P7 RC | main push 的完整 Actions release 制品；同 SHA clean VPS 全门；security、24h soak、performance、backup/restore、upgrade/rollback、native+Compose、split topology |

测试完成定义是“固定 main commit 的 Actions 制品经过 provenance 核验后，可在 VPS 一键复现行为和失败语义”，不是仅保留终端截图或人工点击结论。
