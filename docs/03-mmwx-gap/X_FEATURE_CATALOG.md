# 妙妙屋 X 文档能力目录

> 本目录只描述 2026-08-25 抓取的 58 页文档所宣称的能力，证据等级为 `X-DOC`，不是闭源源码审计。每项分配稳定 `MMWX-*` ID，后续重构、实现和验收不得只按页面名称笼统勾选。证据完整性见 [`DOC_EVIDENCE_AUDIT.md`](DOC_EVIDENCE_AUDIT.md)。

## 1. 产品、部署、存储与迁移

| ID | 能力 | 文档行为/边界 | NodeControll 验收焦点 | 证据 |
|---|---|---|---|---|
| MMWX-PLAT-001 | Master-Agent 架构 | 一个主控统一编排多台远程服务器，Agent 执行系统与 Xray 操作。 | 控制面与节点执行面接口版本化；节点断线不破坏既有数据面。 | [about](https://miaomiaowux.com/docs/about) |
| MMWX-PLAT-002 | 首次安装链路 | 安装 Master、添加远程服务器、部署 Agent、安装内核、创建入站、生成节点与用户订阅。 | 全新空机 E2E 一次跑通，任何步骤可恢复/重试。 | [quick-start](https://miaomiaowux.com/docs/quick-start) |
| MMWX-PLAT-003 | 直接安装 | Linux 脚本安装主控，默认端口 12889，数据位于 `/etc/mmwx/data`。 | 提供校验和、非交互参数、systemd 沙箱和卸载保留数据选项。 | [install-direct](https://miaomiaowux.com/docs/install-direct) |
| MMWX-PLAT-004 | Docker 安装 | 主控用 host network；挂载 data/subscribes/rule_templates；可配 PostgreSQL 数据卷。 | Compose 健康检查、固定镜像 digest、非 root、卷备份验证。 | [install-docker](https://miaomiaowux.com/docs/install-docker) |
| MMWX-PLAT-005 | SQLite 主控 | 小型/默认部署使用 SQLite。 | WAL/备份/迁移/完整性检查有测试；单实例写入约束明确。 | [install-direct](https://miaomiaowux.com/docs/install-direct) |
| MMWX-PLAT-006 | PostgreSQL 18 | Docker 部署可选 PostgreSQL 18。 | 同一领域模型支持 PostgreSQL；迁移、并发锁和集成测试双库运行。 | [install-docker](https://miaomiaowux.com/docs/install-docker) |
| MMWX-PLAT-007 | amd64/arm64 | 系统要求页称主控和 Agent 支持两种架构。 | 发布 linux/amd64、linux/arm64 镜像/二进制并在 VPS/模拟架构验证。 | [system-requirements](https://miaomiaowux.com/docs/system-requirements) |
| MMWX-PLAT-008 | 主/订阅域名 | 主控 UI/API 与订阅可使用不同域名。 | base URL、可信代理和公开订阅 origin 分离，反代测试覆盖。 | [tutorial](https://miaomiaowux.com/docs/tutorial) |
| MMWX-PLAT-009 | HTTPS 反代 | 文档给出 Nginx、Caddy 等发布方法。 | 正确处理 WebSocket、forwarded headers、secure cookie 和 HSTS。 | [tutorial](https://miaomiaowux.com/docs/tutorial) |
| MMWX-PLAT-010 | Cloudflare Tunnel | 主控可只监听回环并经 Tunnel 发布，无需开放入站端口。 | origin/WS 都能通过 Tunnel；不将 Cloudflare 设为强依赖。 | [cloudflare-tunnel](https://miaomiaowux.com/docs/cloudflare-tunnel) |
| MMWX-PLAT-011 | 在线更新 | 主控与 Agent 有版本更新/升级路径。 | 签名制品、阶段发布、失败回滚、数据库兼容窗口和版本审计。 | [update](https://miaomiaowux.com/docs/update) |
| MMWX-PLAT-012 | SQLite ZIP 备份 | 导出数据库、订阅资源和证书；新备份不再使用密码。 | 生成 manifest/hash；私钥归档明确加密选项；恢复前验证。 | [backup-restore](https://miaomiaowux.com/docs/backup-restore) |
| MMWX-PLAT-013 | PostgreSQL 独立备份 | 应用 ZIP 不包含 PG 数据，需另用 `pg_dump`。 | UI/CLI 明确拆分；恢复演练同时覆盖对象存储资源与数据库。 | [backup-restore](https://miaomiaowux.com/docs/backup-restore) |
| MMWX-PLAT-014 | SQLite 启动修复 | 启动发现完整性异常时尝试 `.backup` 恢复。 | 不静默覆盖；隔离损坏库、保存诊断、只从校验通过的备份恢复。 | [backup-restore](https://miaomiaowux.com/docs/backup-restore) |
| MMWX-PLAT-015 | 从妙妙屋迁移 | 五步向导、六个管理员端点，导入 10 张表，保留短链并认领远端节点。 | 可重复 dry-run、映射报告、冲突策略、事务边界和回滚快照。 | [upgrade-from-mmw](https://miaomiaowux.com/docs/upgrade-from-mmw) |
| MMWX-PLAT-016 | 空库迁移前置 | X 数据库必须为空，先部署相关 Agent。 | 前置检查自动完成；不满足时阻止写入并给出可执行修复。 | [upgrade-from-mmw](https://miaomiaowux.com/docs/upgrade-from-mmw) |
| MMWX-PLAT-017 | 智能认领 | 根据 server:port 等关联旧节点与已扫描入站，避免重复创建。 | 匹配结果分置信度；歧义必须人工确认，留存映射表。 | [upgrade-from-mmw](https://miaomiaowux.com/docs/upgrade-from-mmw) |
| MMWX-PLAT-018 | 迁移回滚 | 导入失败可恢复 X 备份并重新启动旧妙妙屋。 | 自动生成回滚包和步骤，E2E 演练 RPO/RTO。 | [upgrade-from-mmw](https://miaomiaowux.com/docs/upgrade-from-mmw) |
| MMWX-PLAT-019 | 更新日志 | 文档按版本记录修复。 | 制品版本、数据库 schema、Agent/API 兼容性和变更日志相互可追踪。 | [changelog](https://miaomiaowux.com/docs/changelog) |
| MMWX-PLAT-020 | 无许可证自托管 | X 存在机器 ID/许可证/额度；目标要求全部合并为普通功能。 | 断网部署可用全部功能，满足 `NOLIC-001..007`。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |

## 2. Agent、连接与远程服务器

| ID | 能力 | 文档行为/边界 | NodeControll 验收焦点 | 证据 |
|---|---|---|---|---|
| MMWX-AGENT-001 | 服务器登记 | 主控创建服务器记录并生成唯一 Agent token。 | token 只显示一次、库中只存 hash，关联 server id/audience。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-AGENT-002 | 一服务器一 token | 复用 token 会产生重连/写入冲突。 | 数据库唯一约束与明确错误；轮换可无损完成。 | [install-agent](https://miaomiaowux.com/docs/install-agent) |
| MMWX-AGENT-003 | WebSocket 模式 | Agent 主动保持 WS，适合实时控制与推送。 | ping/pong、退避、背压、消息序号、重放保护和断点续传。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-AGENT-004 | HTTP 模式 | 以 HTTP 请求/响应执行远端操作。 | 幂等 key、超时、任务查询、结果签名与审计。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-AGENT-005 | Pull 模式 | Agent 轮询主控取任务，适应不能长连环境。 | 长轮询游标、租约、重复领取保护、可调间隔。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-AGENT-006 | Auto 模式 | 自动在 WS/HTTP/Pull 间选择/回退。 | 状态机可观测；切换不重复执行有副作用任务。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-AGENT-007 | systemd 部署 | 一键脚本识别 systemd 并创建服务。 | 最小权限 unit、启动依赖、日志轮转、升级原子替换。 | [install-agent](https://miaomiaowux.com/docs/install-agent) |
| MMWX-AGENT-008 | OpenRC 部署 | Alpine 等环境可创建 OpenRC 服务。 | 在对应容器/VM 验证启停、重启与日志。 | [install-agent](https://miaomiaowux.com/docs/install-agent) |
| MMWX-AGENT-009 | 无 init 回退 | 无 systemd/OpenRC 时直接后台运行。 | 明确“不受监管”风险；容器模式用前台 PID 1。 | [install-agent](https://miaomiaowux.com/docs/install-agent) |
| MMWX-AGENT-010 | 配置文件/环境变量 | Agent 地址、token、连接模式等可由 YAML/env 配置。 | 优先级固定、secret 不出日志、配置 schema 校验。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |
| MMWX-AGENT-011 | 在线状态 | 主控展示 Agent 在线/离线及连接信息。 | 心跳时间、连接模式、最近错误、版本均可查询。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-AGENT-012 | 系统指标 | Agent 上报 CPU、内存、磁盘、负载和网络等。 | 单位、采样周期、缺失值和保留策略明确。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-AGENT-013 | 实时网络 | 服务器页显示实时上下行。 | 单调计数转速率，处理重启/溢出/网卡变更。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-AGENT-014 | 批量升级 | 主控对选中/全部 Agent 批量升级并展示结果。 | 并发上限、架构匹配、签名验证、金丝雀和回滚。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-AGENT-015 | token 轮换 | 服务器 token 可重新生成。 | 旧/新 token 短暂重叠、确认新连接后撤销旧 token。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-AGENT-016 | 主控热重连 | Agent 断线自动退避重连。 | 抖动、上限、DNS/证书错误分类；既有 sing-box 不被停掉。 | [install-agent](https://miaomiaowux.com/docs/install-agent) |
| MMWX-AGENT-017 | 任务执行边界 | Agent 代主控执行内核、证书、Nginx、系统信息等操作。 | 使用类型化 allowlist，不提供任意 shell API；每次操作带主体和审计。 | [features](https://miaomiaowux.com/docs/features) |
| MMWX-AGENT-018 | 嵌入/外置内核模式 | X 支持内嵌库和外置 Xray 二进制。 | sing-box managed/external 两模式能力差异在 UI/API 中显式呈现。 | [embedded-xray](https://miaomiaowux.com/docs/embedded-xray) |

## 3. 服务器流量、探测与代理内核生命周期

| ID | 能力 | 文档行为/边界 | NodeControll 验收焦点 | 证据 |
|---|---|---|---|---|
| MMWX-CORE-001 | 系统网卡计数 | 从 `/proc/net/dev` 读取物理网卡 RX/TX，包含非代理流量，不含部分虚拟网卡。 | 保存接口名和 raw counter；明确排除项，处理 reboot baseline。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-CORE-002 | 内核协议计数 | Xray 维度只统计由内核转发的入/出站流量。 | sing-box 指标以 inbound/outbound/user 维度归一化。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-CORE-003 | 服务器数据源选择 | 管理员可在系统网卡与 Xray 口径间切换。 | 历史记录保留 source；切换建立新 baseline，不改写旧值。 | [remote-servers](https://miaomiaowux.com/docs/remote-servers) |
| MMWX-CORE-004 | 本次开机流量 | 页面直接展示当前 boot 的网卡累计，不等同计费周期。 | 显示 boot id/采样时间，不混入套餐账本。 | [traffic-accounting](https://miaomiaowux.com/docs/traffic-accounting) |
| MMWX-CORE-005 | 域名延迟 | 远程服务器可探测域名/目标延迟。 | 每次结果含 server、target、method、超时和时间。 | [features](https://miaomiaowux.com/docs/features) |
| MMWX-CORE-006 | 服务扫描 | Agent 扫描现有 Xray 服务/配置以供接管。 | 只读发现与认领分离；不确定配置不自动改写。 | [xray-service](https://miaomiaowux.com/docs/xray-service) |
| MMWX-CORE-007 | 安装内核 | 主控远程触发安装 Xray。 | 改为安装固定/可选 sing-box 版本，校验签名/hash、架构和磁盘。 | [xray-service](https://miaomiaowux.com/docs/xray-service) |
| MMWX-CORE-008 | 卸载内核 | 可远程卸载服务。 | 高危二次确认；默认保留配置/备份；共享/活跃入站阻止误删。 | [xray-service](https://miaomiaowux.com/docs/xray-service) |
| MMWX-CORE-009 | 启动/停止/重启 | 服务控制与状态反馈。 | 类型化任务、最终状态确认、超时不误报成功。 | [xray-service](https://miaomiaowux.com/docs/xray-service) |
| MMWX-CORE-010 | 配置路径发现 | 展示/维护运行配置文件。 | realpath allowlist，禁止越界读写；跟踪 checksum/revision。 | [xray-service](https://miaomiaowux.com/docs/xray-service) |
| MMWX-CORE-011 | 完整配置查看 | 可查看入站完整 JSON和系统配置。 | secret 默认遮罩，下载/显示需额外权限并记审计。 | [xray-inbounds](https://miaomiaowux.com/docs/xray-inbounds) |
| MMWX-CORE-012 | 配置编辑 | UI 可修改完整内核配置。 | schema 校验、dry-run、差异预览、版本快照、原子替换。 | [xray-system-config](https://miaomiaowux.com/docs/xray-system-config) |
| MMWX-CORE-013 | 配置生效 | 路由变更后自动重启 Xray。 | 优先 sing-box 可用的 reload；否则受控重启并做健康回滚。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-CORE-014 | 运行配置过滤 | 入站列表隐藏 API 入站和空 tag 运行时入站。 | 系统托管对象明确标记并只读，不能仅靠 tag 空值猜测。 | [xray-inbounds](https://miaomiaowux.com/docs/xray-inbounds) |
| MMWX-CORE-015 | 内核版本状态 | 服务卡片体现安装/运行与版本。 | desired/current version、配置 revision、健康状态可观测。 | [xray-service](https://miaomiaowux.com/docs/xray-service) |
| MMWX-CORE-016 | 变更串行化 | 同一服务器配置/控制操作需要有序。 | 每 server 单写者/租约，乐观并发控制，冲突返回 revision。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |

## 4. 入站与协议组合

| ID | 能力/组合 | 文档行为 | NodeControll 验收焦点 | 证据 |
|---|---|---|---|---|
| MMWX-IN-001 | 可视化入站向导 | 协议→传输→安全→端口→凭据→预览创建。 | Vue 表单由能力 schema 驱动；后端再次校验组合。 | [xray-inbounds](https://miaomiaowux.com/docs/xray-inbounds) |
| MMWX-IN-002 | 端口冲突检测 | 创建前自动检测监听冲突。 | 检查托管配置和 Agent 实际监听；提交时再次校验。 | [xray-inbounds](https://miaomiaowux.com/docs/xray-inbounds) |
| MMWX-IN-003 | 凭据生成 | UUID/密码等由向导自动生成。 | CSPRNG、协议长度约束、仅在必要时回显。 | [xray-inbounds](https://miaomiaowux.com/docs/xray-inbounds) |
| MMWX-IN-004 | 入站↔节点同步 | 创建入站自动生成节点，删除入站自动删除配对节点。 | 领域事务/outbox 保证最终一致；外部节点不被误删。 | [xray-inbounds](https://miaomiaowux.com/docs/xray-inbounds) |
| MMWX-IN-005 | VLESS TCP REALITY | 矩阵组合 1。 | 生成有效 sing-box server/client；连通性与错误参数测试。 | [protocol-matrix](https://miaomiaowux.com/docs/protocol-matrix) |
| MMWX-IN-006 | VLESS TCP REALITY Vision | 矩阵组合 2，flow=`xtls-rprx-vision`。 | 仅在 sing-box 官方支持的版本启用；真实连通/吞吐验收。 | [protocol-vless](https://miaomiaowux.com/docs/protocol-vless) |
| MMWX-IN-007 | VLESS TCP TLS | 矩阵组合 3。 | 证书部署、SNI、ALPN 和客户端转换测试。 | [protocol-matrix](https://miaomiaowux.com/docs/protocol-matrix) |
| MMWX-IN-008 | VLESS TCP TLS Vision | 矩阵组合 4。 | flow 能力探测与版本门控。 | [protocol-vless](https://miaomiaowux.com/docs/protocol-vless) |
| MMWX-IN-009 | VLESS WebSocket TLS | 矩阵组合 5。 | path/header/反代/WSS E2E。 | [protocol-matrix](https://miaomiaowux.com/docs/protocol-matrix) |
| MMWX-IN-010 | VLESS gRPC REALITY | 矩阵组合 6。 | service name、HTTP/2、REALITY 客户端互通。 | [protocol-matrix](https://miaomiaowux.com/docs/protocol-matrix) |
| MMWX-IN-011 | VLESS XHTTP REALITY | 矩阵组合 7；X 文档要求 xhttp headers/mode 转换。 | sing-box 不支持时不得伪装；以能力适配结论决定替代/兼容插件。 | [protocol-matrix](https://miaomiaowux.com/docs/protocol-matrix) |
| MMWX-IN-012 | Trojan TCP TLS | 矩阵组合 8。 | password/SNI/cert 与 Mihomo、sing-box 客户端互通。 | [protocol-trojan](https://miaomiaowux.com/docs/protocol-trojan) |
| MMWX-IN-013 | Trojan TCP REALITY | 矩阵组合 9。 | 依官方 sing-box 支持矩阵门控；不沿用 Xray 私有 JSON。 | [protocol-trojan](https://miaomiaowux.com/docs/protocol-trojan) |
| MMWX-IN-014 | Trojan gRPC REALITY | 矩阵组合 10。 | 同上，并覆盖 gRPC 参数。 | [protocol-matrix](https://miaomiaowux.com/docs/protocol-matrix) |
| MMWX-IN-015 | VMess TCP 无安全层 | 矩阵组合 11。 | UUID/alterId/加密兼容和风险提示。 | [protocol-vmess](https://miaomiaowux.com/docs/protocol-vmess) |
| MMWX-IN-016 | VMess TCP TLS | 矩阵组合 12。 | TLS/客户端格式互通。 | [protocol-vmess](https://miaomiaowux.com/docs/protocol-vmess) |
| MMWX-IN-017 | VMess WS 无安全层 | 矩阵组合 13。 | path/header/反代 E2E。 | [protocol-vmess](https://miaomiaowux.com/docs/protocol-vmess) |
| MMWX-IN-018 | VMess WS TLS | 矩阵组合 14。 | WSS E2E。 | [protocol-vmess](https://miaomiaowux.com/docs/protocol-vmess) |
| MMWX-IN-019 | Shadowsocks AEAD | AES-256-GCM，矩阵组合 15。 | method/password 校验及 URI/Clash/sing-box 转换。 | [protocol-shadowsocks](https://miaomiaowux.com/docs/protocol-shadowsocks) |
| MMWX-IN-020 | Shadowsocks 2022 | 2022-blake3-aes-256-gcm，矩阵组合 16。 | key 长度/编码、multi-user 能力与客户端互通。 | [protocol-shadowsocks](https://miaomiaowux.com/docs/protocol-shadowsocks) |
| MMWX-IN-021 | Hysteria2 UDP TLS | 矩阵组合 17，Xray fork 中用 hysteria/version 2。 | 使用标准 sing-box Hysteria2 schema；QUIC、带宽、认证、证书 E2E。 | [protocol-hysteria2](https://miaomiaowux.com/docs/protocol-hysteria2) |
| MMWX-IN-022 | AnyTLS TCP TLS | 矩阵组合 18，文档称 Mihomo/sing-box 通用。 | 使用标准 sing-box AnyTLS 并验证 client/server 版本。 | [protocol-anytls](https://miaomiaowux.com/docs/protocol-anytls) |
| MMWX-IN-023 | AnyTLS TCP REALITY | 矩阵组合 19；X 文档同时称其客户端不支持。 | 官方 sing-box AnyTLS 使用共享 TLS schema，而该 schema含 Reality；先做 sing-box↔sing-box 互通，再按客户端矩阵开放，不能照搬 Xray JSON。 | [protocol-matrix](https://miaomiaowux.com/docs/protocol-matrix) |
| MMWX-IN-024 | Snell v4/v5 | 每用户独立 PSK，可加 obfs。 | 官方 sing-box 1.14 新增 Snell v5 入站，文档说明其 wire protocol 等价 v4且支持 HTTP obfs；用官方版本门控和互通测试交付。 | [protocol-snell](https://miaomiaowux.com/docs/protocol-snell) |
| MMWX-IN-025 | Snell v6 | 共享 PSK + clientID、隐藏 salt 与整形。 | 官方 sing-box 1.14 原生 Snell v6 入站/multi-user；在 1.14 稳定前作为 pinned upstream preview lane，绝不维护私有 fork。 | [protocol-snell](https://miaomiaowux.com/docs/protocol-snell) |
| MMWX-IN-026 | 废弃 H2/Trojan flow | X 明确 H2 迁到 XHTTP，Trojan flow 被移除。 | 导入时给迁移诊断，不生成已废弃组合。 | [protocol-matrix](https://miaomiaowux.com/docs/protocol-matrix) |
| MMWX-IN-027 | 客户端格式转换 | 入站转换成 Mihomo/Clash 代理字段，处理 SNI、REALITY、Hy2 等差异。 | 建协议无损中间模型；每 producer 使用 golden tests。 | [xray-inbounds](https://miaomiaowux.com/docs/xray-inbounds) |

## 5. 节点、出站、路由、WARP 与隧道

| ID | 能力 | 文档行为/边界 | NodeControll 验收焦点 | 证据 |
|---|---|---|---|---|
| MMWX-NODE-001 | 自动节点 | 托管入站自动生成节点并保持配对。 | 记录 origin/inbound id，不靠名称或 server:port 猜测。 | [nodes](https://miaomiaowux.com/docs/nodes) |
| MMWX-NODE-002 | 外部节点 | 可创建不对应本机入站的节点。 | 与托管节点分型，删除/同步策略隔离。 | [nodes](https://miaomiaowux.com/docs/nodes) |
| MMWX-NODE-003 | 多标签 | 节点可设置多个标签供套餐/订阅选取。 | 标签规范化、批量操作与动态选择规则。 | [nodes](https://miaomiaowux.com/docs/nodes) |
| MMWX-NODE-004 | 排序 | 节点顺序影响管理和订阅。 | 稳定稀疏排序/重排事务。 | [nodes](https://miaomiaowux.com/docs/nodes) |
| MMWX-NODE-005 | 全用户 URI 视图 | 管理员可查看所有用户、所有节点的 protocol URI。 | 权限/secret 显示审计，大数据分页和按用户筛选。 | [nodes](https://miaomiaowux.com/docs/nodes) |
| MMWX-NODE-006 | 地址切换/恢复 | 在域名与解析 IP 等地址之间切换并保留原值。 | 明确 source/current/override，幂等恢复。 | [nodes](https://miaomiaowux.com/docs/nodes) |
| MMWX-NODE-007 | 端口转发复用 | 可按域名/IP复用端口转发映射。 | 唯一约束、冲突检测、引用计数和生命周期。 | [nodes](https://miaomiaowux.com/docs/nodes) |
| MMWX-NODE-008 | Tunnel 入站 | `dokodemo-door` 把端口转到节点/固定目标，统一聚合管理。 | 以 sing-box direct/redirect/tun 合法模型实现；健康、环路、目标校验。 | [nodes](https://miaomiaowux.com/docs/nodes) |
| MMWX-NODE-009 | Tunnel 清理 | 删除 tunnel 同时清配套节点。 | 预览影响范围、事务/outbox、失败可重试。 | [nodes](https://miaomiaowux.com/docs/nodes) |
| MMWX-OUT-001 | Direct/Block | 默认 freedom/blackhole。 | 映射到 sing-box direct/block，系统 tag 不可误删。 | [xray-outbounds](https://miaomiaowux.com/docs/xray-outbounds) |
| MMWX-OUT-002 | 代理出站 | VLESS/VMess/Trojan/SS 作为落地。 | 统一引用已有节点/手填出站并验证链路。 | [xray-outbounds](https://miaomiaowux.com/docs/xray-outbounds) |
| MMWX-OUT-003 | Tunnel 出站 | 隧道可作为路由目标。 | 拓扑和循环检测、端到端状态。 | [xray-outbounds](https://miaomiaowux.com/docs/xray-outbounds) |
| MMWX-OUT-004 | WARP 注册 | Agent 不依赖 wgcf，独立注册账号并写 `warp.json`。 | 凭据 secret 存储、API 失败退避、条款提示和可替代 WireGuard 配置。 | [xray-outbounds](https://miaomiaowux.com/docs/xray-outbounds) |
| MMWX-OUT-005 | WARP v4/v6 | 自动注入两个 WireGuard 出站。 | 标准 sing-box wireguard endpoints、幂等 tag 和 IPv4/IPv6 E2E。 | [xray-outbounds](https://miaomiaowux.com/docs/xray-outbounds) |
| MMWX-OUT-006 | WARP+ | 接受 license key 升级账户。 | 该 key 仅是 Cloudflare 服务凭据，不是项目许可证；加密存储/删除。 | [xray-outbounds](https://miaomiaowux.com/docs/xray-outbounds) |
| MMWX-OUT-007 | WARP 刷新 | 重新拉取并幂等替换同 tag 出站。 | revision 和健康检查，失败保留上个可用配置。 | [xray-outbounds](https://miaomiaowux.com/docs/xray-outbounds) |
| MMWX-OUT-008 | WARP 卸载 | 注销账号、删本地文件、移除出站。 | 高危确认、引用检查、部分失败可恢复。 | [xray-outbounds](https://miaomiaowux.com/docs/xray-outbounds) |
| MMWX-ROUTE-001 | first-match | 自上而下首个命中，未命中走默认出站。 | UI 模拟器/后端编译器与 sing-box 规则语义一致。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-ROUTE-002 | 条件 AND | 单条规则的 domain/IP/protocol 等共同满足。 | 可测试的 IR；空字段不提交。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-ROUTE-003 | 节点专属/全局 | 有 inboundTag 为专属，无则全局。 | target inbound id 稳定映射，重命名不失联。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-ROUTE-004 | Catch-all 检测 | 只有 inbound/outbound 的规则遮蔽后续全局/默认，UI 提示。 | 静态分析 unreachable rules，阻止或警告错误顺序。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-ROUTE-005 | 路由拖排 | 整体 set 保存并自动重启，API 规则固定最前。 | optimistic revision、系统规则锁定、失败回滚。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-ROUTE-006 | 域名/IP/协议/端口规则 | 支持 geosite/domain/full/regexp、geoip/CIDR、protocol、目标/源端口。 | 映射 sing-box route rule；对已弃字段出迁移错误。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-ROUTE-007 | 来源/网络/用户/属性 | 支持 source、network、user、attrs 等条件。 | 按标准 sing-box 可表达性分级；不可表达项显式拒绝。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-ROUTE-008 | 快捷规则 | BT、CN IP、OpenAI、内网、EMBY、TikTok 等预设。 | 预设是可查看/版本化模板，不从官方域名强依赖下载。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-ROUTE-009 | 防送中 WARP 规则 | Google/Meta 走 warp-v4。 | 管理员可编辑数据源/目标；无 WARP 时不可创建悬空规则。 | [xray-outbounds](https://miaomiaowux.com/docs/xray-outbounds) |
| MMWX-ROUTE-010 | Balancer random | 随机选出站。 | 映射标准 sing-box selector/urltest 能力或自有调度语义测试。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-ROUTE-011 | Balancer round-robin | 轮询出站。 | 若 sing-box 无等价项，Agent 编译器必须明确能力/替代策略。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-ROUTE-012 | Balancer least-ping | observatory 选最低延迟。 | 健康检查 URL、间隔、容差、失败窗口可配置。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-ROUTE-013 | Balancer least-load | 按负载选出站。 | 定义负载指标和一致性；无指标时降级可观测。 | [xray-routing](https://miaomiaowux.com/docs/xray-routing) |
| MMWX-ROUTE-014 | 节点级路由出站 | 整个父节点/入站所有流量走指定落地。 | 生成 catch-all 路由，引用关系和禁用/删除级联正确。 | [routed-outbound](https://miaomiaowux.com/docs/routed-outbound) |
| MMWX-ROUTE-015 | 用户级私有路由出站 | 普通用户为自己创建子节点，生成专属 client/email 和 route.user。 | 每用户凭据/路由/订阅原子创建，账单正确归属原用户。 | [routed-outbound](https://miaomiaowux.com/docs/routed-outbound) |
| MMWX-ROUTE-016 | 用户路由配额 | 默认最多保有 2 个，每日最多 5 次变更；管理员可开关。 | 本地可配、原子计数、时区定义、管理员豁免显式。 | [routed-outbound](https://miaomiaowux.com/docs/routed-outbound) |
| MMWX-ROUTE-017 | 自动暂停/恢复 | 用户禁用、过期、超限时私有路由暂停，恢复条件后启用。 | 策略状态机、reason、审计和幂等 reconcile。 | [routed-outbound](https://miaomiaowux.com/docs/routed-outbound) |

## 6. 用户、套餐、流量与执行策略

| ID | 能力 | 文档行为/边界 | NodeControll 验收焦点 | 证据 |
|---|---|---|---|---|
| MMWX-USER-001 | admin/user 角色 | 管理员管理系统，普通用户取订阅和查看自己数据。 | RBAC 后端强制；以后可扩 scope，不依赖前端隐藏。 | [users](https://miaomiaowux.com/docs/users) |
| MMWX-USER-002 | 用户 CRUD/状态 | 创建、编辑、启停、删除用户。 | 乐观并发、软删除/保留账本、高危确认。 | [users](https://miaomiaowux.com/docs/users) |
| MMWX-USER-003 | 密码/JWT | 文档称密码登录并使用 JWT。 | 短期 access + 可撤销 refresh/session，Argon2id，CSRF 策略。 | [users](https://miaomiaowux.com/docs/users) |
| MMWX-USER-004 | 订阅 token | 每用户 token 访问订阅，可重置。 | hash 存储、scope/audience、轮换、最后使用时间。 | [users](https://miaomiaowux.com/docs/users) |
| MMWX-USER-005 | 多套餐 | 一个用户可绑定多个套餐实例。 | 实例独立周期、节点、凭据、baseline 和状态。 | [users](https://miaomiaowux.com/docs/users) |
| MMWX-USER-006 | 多用户同端口 | 通过内核 client identity/email 区分同一入站的用户流量。 | 稳定内部 principal id，不把可改邮箱作为主键。 | [faq-carpool](https://miaomiaowux.com/docs/faq-carpool) |
| MMWX-PKG-001 | 套餐模板/实例 | 套餐定义可给用户产生独立实例。 | template revision 与实例快照分离，变更传播策略明确。 | [packages](https://miaomiaowux.com/docs/packages) |
| MMWX-PKG-002 | 节点/标签选择 | 套餐选择节点或标签。 | 静态选择与动态标签查询分离，预览命中。 | [packages](https://miaomiaowux.com/docs/packages) |
| MMWX-PKG-003 | 流量限额 | 套餐有流量上限和周期。 | bytes 整数、时区/周期、无限值、结转规则明确。 | [packages](https://miaomiaowux.com/docs/packages) |
| MMWX-PKG-004 | 到期时间 | 套餐实例可独立到期。 | 精确 instant，宽限期/暂停 reason 有审计。 | [packages](https://miaomiaowux.com/docs/packages) |
| MMWX-PKG-005 | 独立凭据 | 每个套餐实例可有独立订阅/内核 client 身份。 | 泄露只轮换该实例，不影响用户其他套餐。 | [packages](https://miaomiaowux.com/docs/packages) |
| MMWX-PKG-006 | 节点计费倍率 | 节点可配置倍率，影响计费流量。 | 同时保存 raw 和 billed；倍率版本化，历史不回写。 | [packages](https://miaomiaowux.com/docs/packages) |
| MMWX-PKG-007 | 单/双向计费 | 可只计上/下行或双向。 | 方向定义写入账本，UI 清楚显示公式。 | [traffic-accounting](https://miaomiaowux.com/docs/traffic-accounting) |
| MMWX-PKG-008 | 套餐默认速度 | 套餐层设置默认 Mbps。 | unlimited 用 null，不用魔法数；值域校验。 | [node-ratelimit](https://miaomiaowux.com/docs/node-ratelimit) |
| MMWX-PKG-009 | 套餐逐节点速度 | 节点覆盖套餐默认速度。 | 继承来源可解释。 | [node-ratelimit](https://miaomiaowux.com/docs/node-ratelimit) |
| MMWX-PKG-010 | 用户全局覆盖 | 用户级速度覆盖套餐。 | 优先级与冲突在 UI/API 返回 effective policy。 | [node-ratelimit](https://miaomiaowux.com/docs/node-ratelimit) |
| MMWX-PKG-011 | 用户逐节点覆盖 | 最高优先级逐用户逐节点设置。 | 唯一约束、批量编辑、有效期。 | [node-ratelimit](https://miaomiaowux.com/docs/node-ratelimit) |
| MMWX-PKG-012 | 限速优先级 | `用户节点 > 用户全局 > 套餐节点 > 套餐默认 > 不限`。 | 单一纯函数和表驱动测试覆盖所有 fallback。 | [node-ratelimit](https://miaomiaowux.com/docs/node-ratelimit) |
| MMWX-PKG-013 | 并发连接上限 | 与速度使用同一继承结构；文档误称设备/客户端数。 | 字段改名 `max_connections`，真实连接压测。 | [node-ratelimit](https://miaomiaowux.com/docs/node-ratelimit) |
| MMWX-PKG-014 | 规则推送 | Agent 建连及策略变化时推送服务器所有有效限速。 | snapshot + revision + ack；漏推可 reconcile。 | [node-ratelimit](https://miaomiaowux.com/docs/node-ratelimit) |
| MMWX-PKG-015 | 自动限速 | 接近/达到阈值自动调整。 | 可配置阈值/迟滞/持续时间；任务不会震荡。 | [packages](https://miaomiaowux.com/docs/packages) |
| MMWX-PKG-016 | 自动解除 | 条件恢复后自动取消临时限速。 | 保存原始 policy，不以覆盖写丢配置。 | [packages](https://miaomiaowux.com/docs/packages) |
| MMWX-PKG-017 | 自动停用 | 超限/到期可停用户或套餐。 | 原因、来源、恢复规则和通知一致。 | [packages](https://miaomiaowux.com/docs/packages) |
| MMWX-TRAFFIC-001 | 入站/出站/用户三维 | Xray 按 inbound、outbound、user/email 统计。 | sing-box 指标映射到稳定 ID；采样去重。 | [faq-carpool](https://miaomiaowux.com/docs/faq-carpool) |
| MMWX-TRAFFIC-002 | 原始与计费流量 | raw 与倍率后 billed 分开。 | 不可变 ledger，所有调整另记 event。 | [traffic-accounting](https://miaomiaowux.com/docs/traffic-accounting) |
| MMWX-TRAFFIC-003 | 每日账本 | 按天聚合用户/套餐/节点用量。 | 时区、迟到数据、重算版本和唯一键。 | [traffic-accounting](https://miaomiaowux.com/docs/traffic-accounting) |
| MMWX-TRAFFIC-004 | 重置 baseline | 套餐/服务器重置不删除 raw counter，而移动基线。 | baseline event 可审计、可回放。 | [traffic-accounting](https://miaomiaowux.com/docs/traffic-accounting) |
| MMWX-TRAFFIC-005 | 手工调整 | 管理员可补加/扣减计费量。 | append-only adjustment，必须 reason/actor。 | [traffic-accounting](https://miaomiaowux.com/docs/traffic-accounting) |
| MMWX-TRAFFIC-006 | 路由出站归属 | 通过用户标识反查，私有落地流量仍归属套餐。 | 多 hop 不重复计费；trace id/维度测试。 | [routed-outbound](https://miaomiaowux.com/docs/routed-outbound) |
| MMWX-TRAFFIC-007 | 外部订阅口径 | 外部源流量是另一来源，页面显示规则不同。 | source enum，禁止和本机账本无依据相加。 | [traffic-accounting](https://miaomiaowux.com/docs/traffic-accounting) |

## 7. 订阅、模板、规则与测速

| ID | 能力 | 文档行为/边界 | NodeControll 验收焦点 | 证据 |
|---|---|---|---|---|
| MMWX-SUB-001 | 订阅文件管理 | 继承妙妙屋上传、编辑、公开/授权、短链等能力。 | 覆盖所有 `MMW-SUB-*`，文件与元数据一致性。 | [subscribe-files](https://miaomiaowux.com/docs/subscribe-files) |
| MMWX-SUB-002 | 12+ 客户端格式 | Clash/Mihomo、Shadowrocket、Surge、Stash、Surfboard、V2Ray、sing-box、QX 等。 | 每格式 schema/golden/客户端解析测试，绝不只改 UA。 | [generator](https://miaomiaowux.com/docs/generator) |
| MMWX-SUB-003 | UA/显式格式选择 | 按 URL/UA 输出目标客户端配置。 | 显式参数优先，未知 UA 使用安全默认并返回诊断 header。 | [generator](https://miaomiaowux.com/docs/generator) |
| MMWX-SUB-004 | 生成流水线 | 用户→套餐→节点→模板→规则→格式转换。 | 每阶段纯模型化并有快照测试，可输出 explain trace。 | [generator](https://miaomiaowux.com/docs/generator) |
| MMWX-SUB-005 | 模板叠加 | 模板与用户节点/规则合并。 | 冲突/覆盖优先级和引用完整性固定。 | [generator](https://miaomiaowux.com/docs/generator) |
| MMWX-SUB-006 | V3 模板 | 代理组、provider、规则提供器等结构化模板。 | JSON schema、版本、导入 dry-run 和未知字段保留策略。 | [templates](https://miaomiaowux.com/docs/templates) |
| MMWX-SUB-007 | include-all-proxies | 新节点自动进入相应组。 | 不产生重复/悬空引用，过滤禁用节点。 | [templates](https://miaomiaowux.com/docs/templates) |
| MMWX-SUB-008 | include-all-providers | 外部 provider 自动进入组。 | provider scope/鉴权 URL 正确。 | [templates](https://miaomiaowux.com/docs/templates) |
| MMWX-SUB-009 | proxy/providers/both | 组可只含节点、只含 providers 或两者。 | 组合 matrix golden tests。 | [templates](https://miaomiaowux.com/docs/templates) |
| MMWX-SUB-010 | select/url-test/fallback/load-balance | 模板代理组类型及 URL/interval/tolerance。 | 字段只对相关类型生效，客户端兼容检查。 | [templates](https://miaomiaowux.com/docs/templates) |
| MMWX-SUB-011 | relay/dialer 链 | 用 `dialer-proxy-group` 表达链式代理。 | 图拓扑无环，目标客户端不支持时明确报错。 | [templates](https://miaomiaowux.com/docs/templates) |
| MMWX-SUB-012 | 隐藏/图标等展示属性 | 模板组支持 hidden/icon 等。 | producer 能力表决定保留/丢弃并记录 warning。 | [templates](https://miaomiaowux.com/docs/templates) |
| MMWX-SUB-013 | 自定义分流规则 | 支持规则类型、策略与自定义配置。 | parser/schema/冲突提示；危险 regex/脚本受限。 | [custom-rules](https://miaomiaowux.com/docs/custom-rules) |
| MMWX-SUB-014 | 外部订阅/provider | 系统设置开启后可在订阅文件中配置外部代理集合。 | SSRF 防护、缓存、ETag、token 隔离和失败回退。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |
| MMWX-SUB-015 | 代理组配置同步 | 从远程同步预设组和规则选择器。 | URL 可由管理员替换；签名/hash、版本/回滚、离线包。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |
| MMWX-SPEED-001 | 主控本地测速 | 自动取得 Mihomo，从主控网络测试。 | 二进制固定/hash；后续可用 sing-box 执行器，隔离进程。 | [node-speedtest](https://miaomiaowux.com/docs/node-speedtest) |
| MMWX-SPEED-002 | 远程家用测速端 | tester 反向 WSS，无需公网 IP。 | 一次性配对 token、设备撤销、mTLS/短期 session。 | [node-speedtest](https://miaomiaowux.com/docs/node-speedtest) |
| MMWX-SPEED-003 | 单/8 线程吞吐 | 单流反映拥塞，多线程逼近链路上限。 | 线程、时长、字节、URL写入结果；服务端限并发。 | [node-speedtest](https://miaomiaowux.com/docs/node-speedtest) |
| MMWX-SPEED-004 | 真连接延迟 | Cloudflare 204 三次取最佳两次平均。 | method/URL/样本全部保存，失败分类。 | [node-speedtest](https://miaomiaowux.com/docs/node-speedtest) |
| MMWX-SPEED-005 | 下载测速 | 文档示例 gstatic，约 8 秒。 | 测试源可配置，默认源不是强依赖；有最大流量/时长。 | [node-speedtest](https://miaomiaowux.com/docs/node-speedtest) |
| MMWX-SPEED-006 | 出口 IP | 经代理回显实际出口。 | IP source/ASN 可选，隐私保留期。 | [node-speedtest](https://miaomiaowux.com/docs/node-speedtest) |
| MMWX-SPEED-007 | 批量异步/历史 | 任务立即返回、约 1.5 秒轮询、历史可回看，来源内串行。 | SSE/WS 或轮询均可；持久状态机、取消/超时、串行锁。 | [node-speedtest](https://miaomiaowux.com/docs/node-speedtest) |

## 8. 证书、网站、设置与通知

| ID | 能力 | 文档行为/边界 | NodeControll 验收焦点 | 证据 |
|---|---|---|---|---|
| MMWX-CERT-001 | ACME DNS-01 | 申请根域/通配符证书。 | ACME account/订单状态机、重试、挑战清理。 | [certificates](https://miaomiaowux.com/docs/certificates) |
| MMWX-CERT-002 | DNS 提供商 | Cloudflare、阿里云、腾讯云、Namesilo。 | provider adapter、最小权限凭据、联调/模拟测试。 | [certificates](https://miaomiaowux.com/docs/certificates) |
| MMWX-CERT-003 | SAN/通配符 | 根域和 wildcard 可共同申请。 | 域名规范化、授权域检查、证书 SAN 验证。 | [certificates](https://miaomiaowux.com/docs/certificates) |
| MMWX-CERT-004 | 自动续期 | 到期前自动申请新证书。 | 锁/退避/提前窗口/失败告警；旧证书保留到部署成功。 | [certificates](https://miaomiaowux.com/docs/certificates) |
| MMWX-CERT-005 | 自动部署 Agent | 证书/私钥传播到远端并供内核/站点使用。 | 加密传输、原子权限 0600、引用目标 allowlist、reload 回滚。 | [certificates](https://miaomiaowux.com/docs/certificates) |
| MMWX-CERT-006 | PEM 下载 | 管理员下载证书与私钥。 | 额外确认/再认证、审计、no-store。 | [certificates](https://miaomiaowux.com/docs/certificates) |
| MMWX-CERT-007 | Webhook/Certimate | API token 接收 PEM，更新/创建并部署。 | scoped token、body 限制、证书/私钥匹配、幂等。 | [certificates](https://miaomiaowux.com/docs/certificates) |
| MMWX-SITE-001 | Nginx 探测 | Agent 检查是否安装及服务管理器。 | 只读能力发现，不假设包管理器。 | [website-management](https://miaomiaowux.com/docs/website-management) |
| MMWX-SITE-002 | 静态网站 | 上传/配置站点目录并生成 Nginx 站点。 | 路径隔离、压缩包穿越防护、owner 权限、原子发布。 | [website-management](https://miaomiaowux.com/docs/website-management) |
| MMWX-SITE-003 | 反向代理 | 创建 upstream 反代站点。 | URL/端口/WS/TLS schema，SSRF/loop 检测。 | [website-management](https://miaomiaowux.com/docs/website-management) |
| MMWX-SITE-004 | 端口检查 | 创建站点前检查冲突。 | Agent 实际监听 + Nginx config 双检查。 | [website-management](https://miaomiaowux.com/docs/website-management) |
| MMWX-SITE-005 | 安全删除 | 只删除由系统管理的配置。 | ownership marker + 数据库 id + realpath 三重校验。 | [website-management](https://miaomiaowux.com/docs/website-management) |
| MMWX-SET-001 | 外订阅同步策略 | 流量、名称后缀、include/exclude、匹配范围、缓存等。 | 覆盖社区版 `MMW-EXT-*`，设置作用域/默认值可追踪。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |
| MMWX-SET-002 | 静默模式 | 暂停公开访问，并可短时恢复。 | 管理员/API 不被锁死；时间到自动恢复，状态审计。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |
| MMWX-SET-003 | 短链接 | 开关短码发布。 | 短码随机/冲突/撤销/速率限制。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |
| MMWX-SET-004 | 客户端兼容模式 | 调整订阅输出兼容性。 | 具体行为按 producer capability version 化，不用模糊总开关。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |
| MMWX-SET-005 | 覆写脚本 | 生成后执行自定义变换。 | 沙箱、CPU/内存/时间、禁网默认、版本与失败回退。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |
| MMWX-SET-006 | 模板版本/序列化 | 选择模板系统和 YAML/JSON等输出。 | schema 迁移、预览差异、producer 测试。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |
| MMWX-SET-007 | 订阅响应头/信息节点 | 输出流量 header 与余量/到期伪节点。 | 值与账本同源，缓存策略不泄露其他用户。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |
| MMWX-SET-008 | 品牌/主题 | 默认主题、壁纸、自定义品牌。 | 所有品牌能力普通可用，资产自托管、a11y 对比度。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |
| MMWX-NOTIFY-001 | Telegram 通知 | 配置 bot token/chat id，选择通知事件。 | secret 加密、测试发送、模板/重试/去重/失败队列。 | [system-settings](https://miaomiaowux.com/docs/system-settings) |

## 9. 公开探针、Telegram、MCP、安全与实例联合

| ID | 能力 | 文档行为/边界 | NodeControll 验收焦点 | 证据 |
|---|---|---|---|---|
| MMWX-PROBE-001 | 内置公开探针 | 同源页面展示选中服务器状态。 | 明确公开字段 allowlist，不返回内部 id/IP/token/config。 | [probe-api](https://miaomiaowux.com/docs/probe-api) |
| MMWX-PROBE-002 | 外置 Worker 探针 | Cloudflare Worker 代理公开探针；`PROBE_TOKEN` 保护源接口。 | 外置实现可替换/自托管；secret 比较常数时间，可轮换。 | [install-external-probe](https://miaomiaowux.com/docs/install-external-probe) |
| MMWX-PROBE-003 | 状态快照 API | `GET /api/public/probe-servers` 返回当前状态/一小时摘要。 | OpenAPI schema、缓存/ETag、缺失字段为 null/omit 而非 0。 | [probe-api](https://miaomiaowux.com/docs/probe-api) |
| MMWX-PROBE-004 | 实时 WS | `/api/public/probe-ws` 每 5 秒推快照。 | 连接/订阅上限、心跳、背压和 origin 策略。 | [probe-api](https://miaomiaowux.com/docs/probe-api) |
| MMWX-PROBE-005 | 历史 series | `/api/public/probe-series` 返回延迟/系统历史。 | 时间范围、bucket、分页、最大点数。 | [probe-api](https://miaomiaowux.com/docs/probe-api) |
| MMWX-PROBE-006 | 公开扩展字段 | 状态、系统、流量、延迟、回程、续费、提供商等可选字段。 | 每字段单独公开开关；URL 清理，许可证铭牌改实例 badge。 | [probe-api](https://miaomiaowux.com/docs/probe-api) |
| MMWX-SEC-001 | Cloudflare Turnstile | 登录前可启用人机验证。 | provider 可选、服务端校验、故障策略和本地限速仍存在。 | [tool-cloudflare-turnstile](https://miaomiaowux.com/docs/tool-cloudflare-turnstile) |
| MMWX-TG-001 | 内嵌 Telegram Bot | 配置后热启动/重启，无需独立服务。 | bot 生命周期/health，token 不日志。 | [tool-mmwx-tgbot](https://miaomiaowux.com/docs/tool-mmwx-tgbot) |
| MMWX-TG-002 | 用户命令 | 用户查询套餐、流量、订阅等。 | TG 账号需安全绑定用户，可撤销。 | [tool-mmwx-tgbot](https://miaomiaowux.com/docs/tool-mmwx-tgbot) |
| MMWX-TG-003 | 管理员命令 | 管理员查看状态、用户、服务器并执行有限管理。 | admin allowlist + 高危确认 + 审计。 | [tool-mmwx-tgbot](https://miaomiaowux.com/docs/tool-mmwx-tgbot) |
| MMWX-TG-004 | 邀请码 | Bot 支持邀请/注册路径。 | 一次性/次数/有效期、角色固定为普通用户、滥用限速。 | [tool-mmwx-tgbot](https://miaomiaowux.com/docs/tool-mmwx-tgbot) |
| MMWX-TG-005 | 每日通知 | 定时推送用户/管理员摘要。 | 时区、幂等、失败重试和退订。 | [tool-mmwx-tgbot](https://miaomiaowux.com/docs/tool-mmwx-tgbot) |
| MMWX-TG-006 | Mini App | Telegram initData 免普通 Web 登录，区分管理员/用户视图。 | 严格验证 initData 签名/时间窗/replay；权限仍由 API 强制。 | [tool-mmwx-tgbot](https://miaomiaowux.com/docs/tool-mmwx-tgbot) |
| MMWX-MCP-001 | Streamable HTTP `/mcp` | AI Agent 通过 MCP 连接主控。 | 标准协议版本协商、限流、超时、结构化错误。 | [mcp](https://miaomiaowux.com/docs/mcp) |
| MMWX-MCP-002 | Scoped API token | 管理员生成可选择工具权限的令牌。 | hash 存储、scope、到期、撤销、最后使用/IP策略。 | [mcp](https://miaomiaowux.com/docs/mcp) |
| MMWX-MCP-003 | 26 个工具 | 覆盖服务器、用户、套餐、流量、节点等读写。 | 每工具 JSON schema、同 REST service 层权限、契约测试。 | [mcp](https://miaomiaowux.com/docs/mcp) |
| MMWX-MCP-004 | 高危确认 | 写操作/危险动作带显式确认 flag。 | 两阶段 intent token，不能只信任布尔值；审计输入摘要。 | [mcp](https://miaomiaowux.com/docs/mcp) |
| MMWX-MCP-005 | 极端接口不暴露 | 部分删除/破坏性端点不进入工具集。 | MCP allowlist；任意 shell/私钥下载/系统卸载永不暴露。 | [mcp](https://miaomiaowux.com/docs/mcp) |
| MMWX-SHARE-001 | 分享 token | 拥有方为服务器建多个 token，token 可撤销且服务端存 hash。 | secret 一次显示、scope/配额/到期/受众绑定。 | [share-server](https://miaomiaowux.com/docs/share-server) |
| MMWX-SHARE-002 | 消费方接入 | 消费方不部署 Agent，登记拥有方 endpoint/token。 | 双方实例身份钉扎、健康/兼容版本检查。 | [share-server](https://miaomiaowux.com/docs/share-server) |
| MMWX-SHARE-003 | 最小权限 `/api/child` | 只开放状态与消费方自己的入站/节点操作。 | 资源 ownership 在服务端强制，租户前缀不是唯一边界。 | [share-server](https://miaomiaowux.com/docs/share-server) |
| MMWX-SHARE-004 | 入站前缀 | 消费方创建的 tag 自动加固定前缀防冲突。 | 前缀+全局唯一 ID；不可冒充拥有方系统 tag。 | [share-server](https://miaomiaowux.com/docs/share-server) |
| MMWX-SHARE-005 | 禁止服务控制/配置读取 | 消费方不能安装、停启、编辑完整配置或直接连 Agent。 | API 根本不发相应 capability，越权集成测试。 | [share-server](https://miaomiaowux.com/docs/share-server) |
| MMWX-SHARE-006 | 禁止二次分享 | 消费方不能把收到的服务器再分享。 | 授权链只允许 origin owner 签发，服务端检查 provenance。 | [share-server](https://miaomiaowux.com/docs/share-server) |
| MMWX-SHARE-007 | ECDH/HTTPS 传输 | 文档宣称 HTTPS + ECDH 端到端保护并有 HTTPS token fallback。 | 使用标准 TLS 1.3/mTLS 或应用层信封；协议威胁模型与互操作测试。 | [share-server](https://miaomiaowux.com/docs/share-server) |

## 10. 数量与使用规则

- 本目录共列出 **213 个唯一 `MMWX-*` 验收单元**（按表格 ID 计）。这不是营销功能数，而是把组合、边界和安全条件拆成可测试项。
- 同一能力可由多个页面描述，但只分配一个主 ID；FAQ 只用于补充限制。
- `MMWX-IN-023..025` 已按 2026-08-25 官方资料校正：AnyTLS 是 1.12+ 标准入站，Snell 服务端进入 1.14；实际开放仍由内核版本与客户端互通矩阵决定，不沿用 Xray 私有 fork。
- 原 PRO 能力由 [`PRO_FEATURES.md`](PRO_FEATURES.md) 的 `PRO-*`/`NOLIC-*` 约束覆盖，不能以“没有 Xray 内嵌库”作为删除限速、追踪或分享能力的理由。
