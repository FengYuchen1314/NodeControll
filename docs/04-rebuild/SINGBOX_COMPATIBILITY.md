# 标准 sing-box 能力基线与兼容决策

> 核验时间：2026-08-25（Asia/Shanghai）。只使用 SagerNet 官方文档、官方仓库和官方 GitHub Release API。远端只读源码位于 VPS `/opt/nodecontroll/upstream/sing-box`。

## 1. 固定基线

| 轨道 | tag/commit | 状态 | 用途 |
|---|---|---|---|
| 稳定 | `v1.13.19` / `b5ebaa1fc0f2b94256180b95468e73ef53caa27d` | 2026-08-25 GitHub latest release | 核心协议、V2Ray stats、Clash API 与回退基线 |
| 全功能预览 | `v1.14.0-beta.17` / `c82b9b8dc92e1495968a1e0835644e4ad6fc303b` | 官方预发布 | Snell inbound、sing-box API 用户连接流、全功能开发验收 |
| 发布要求 | `>=1.14.0` stable | 尚未发布（核验日） | 正式 release 默认内核；若仍未发布，beta lane 必须清楚标为 preview，但功能测试不能跳过 |

官方发布页：<https://github.com/SagerNet/sing-box/releases>。官方配置文档默认会提前展示 1.14 字段，因此不能看到页面就误认为 1.13 已具备。

NodeControll 不修改 sing-box 源码。构建使用官方 tag 和默认 build tags，并额外启用官方的 `with_v2ray_api` build tag；这是启用上游已提供模块，不是私有 fork。sing-box 作为独立进程/制品分发，保留 GPL-3.0-or-later 许可证、源码 tag 与制品 hash；Rust 控制面不链接其代码。

## 2. X 的 21 个组合到标准 sing-box

| X 组合 | 标准 sing-box 映射 | 判定 |
|---|---|---|
| VLESS TCP + TLS/REALITY，含 Vision | `vless` inbound + shared TLS/Reality；`users.flow=xtls-rprx-vision` | 原生 |
| VLESS WebSocket TLS | `vless` + V2Ray `ws` transport + TLS | 原生 |
| VLESS gRPC REALITY | `vless` + `grpc` transport + TLS Reality；标准 gRPC 需相应构建支持 | 原生，需构建/互通测试 |
| VLESS XHTTP REALITY | sing-box 没有 Xray XHTTP；有 `http`、`ws`、`grpc`、`httpupgrade`、`quic` transport | 不做伪等价；目标提供标准 HTTP/HTTPUpgrade 方案，导入 XHTTP 给迁移诊断 |
| Trojan TCP/gRPC + TLS/REALITY | `trojan` inbound 支持 shared TLS 和 V2Ray transport | 原生，客户端矩阵门控 |
| VMess TCP/WS ± TLS | `vmess` inbound + shared TLS/transport | 原生 |
| Shadowsocks AEAD/2022 | `shadowsocks` inbound；2022/multi-user 按官方 method/key schema | 原生 |
| Hysteria2 UDP TLS | `hysteria2` inbound，自带 users、带宽、混淆、TLS/QUIC字段 | 原生 |
| AnyTLS TCP TLS | `anytls` inbound，自 1.12 起 | 原生 |
| AnyTLS TCP REALITY | AnyTLS 使用 shared TLS，官方 shared TLS inbound 含 Reality server fields | schema 原生；必须先做 sing-box 双端互通，第三方客户端另行门控 |
| Snell v4/v5 | 1.14 `snell` inbound version 5；官方说明 v5 wire 等价 v4，不单独提供 v4 server，支持 HTTP obfs | 1.14 原生、版本门控 |
| Snell v6 | 1.14 `snell` inbound version 6，支持 multi-user、PSK/userkey 与 shaping mode | 1.14 原生、版本门控 |

官方入口：[inbound 列表](https://sing-box.sagernet.org/configuration/inbound/)、[VLESS](https://sing-box.sagernet.org/configuration/inbound/vless/)、[Trojan](https://sing-box.sagernet.org/configuration/inbound/trojan/)、[Hysteria2](https://sing-box.sagernet.org/configuration/inbound/hysteria2/)、[AnyTLS](https://sing-box.sagernet.org/configuration/inbound/anytls/)、[Snell](https://sing-box.sagernet.org/configuration/inbound/snell/)、[TLS/Reality](https://sing-box.sagernet.org/configuration/shared/tls/)、[V2Ray transport](https://sing-box.sagernet.org/configuration/shared/v2ray-transport/)。

## 3. 配置检查与重载事实

- 官方 CLI 提供 `sing-box check`、`format` 和 `merge`；Agent 每次应用前必须运行固定版本的 `check`。
- 官方 systemd unit 通过 `SIGHUP` reload。1.13.19 源码的真实行为是：先解析/检查新配置，然后 cancel 旧实例、`Close()`，再创建新实例；它不是原地热改对象。
- 因此 reload 可能中断活动连接，V2Ray 内存计数也会重置。目标必须合并短时间内的多次改动，应用前采集计数、生成 revision/epoch，失败则原子回滚上一个配置再 reload。
- 除 Shadowsocks 的标准 SSM API 外，1.13/1.14 没有通用 VLESS/VMess/Trojan/Hy2/AnyTLS/Snell 动态用户 CRUD。用户/凭据变更归入配置 reconcile，不能声称零中断动态注入。

## 4. 统计、连接与限速

### 4.1 官方可用能力

| 能力 | 1.13.19 | 1.14 beta | 目标使用方式 |
|---|---|---|---|
| V2Ray gRPC stats | 有，但官方 release 默认未带 `with_v2ray_api` | 有 | 自构建官方 tag + build tag；按明确 inbound/outbound/user 统计 uplink/downlink |
| Clash API | 官方默认 build tags 含 `with_clash_api` | 有 | 1.13 的整体连接/流量与 close API 回退；只监听 loopback + secret |
| 用户连接事件 | Clash JSON 故意不序列化 `metadata.User` | sing-box API `SubscribeConnections` 包含 user、source、inbound、outbound、增量和 close | 全功能 lane 使用 1.14 gRPC API，Agent 订阅并落本地 outbox |
| 动态关闭连接 | Clash API 可按 connection ID 关闭 | sing-box API 原生 CloseConnection/CloseAll | 执行并发上限、禁用/到期和管理员操作 |
| 速度限制 | 除协议特定字段外无通用逐用户限速 | 仍无通用逐用户 token bucket | 由 Agent 的 Linux tc/eBPF shaper 执行，sing-box 保持官方原版 |

V2Ray API 文档：<https://sing-box.sagernet.org/configuration/experimental/v2ray-api/>；1.14 sing-box API：<https://sing-box.sagernet.org/configuration/service/api/>。

### 4.2 逐用户限速的标准内核方案

NodeControll 不往 sing-box 注入私有调度器。Agent 订阅 1.14 connection events，拿到认证后的 `user`、inbound 与 source tuple，将双向 flow key 映射到 `(principal_id,node_id)`；Linux tc eBPF classifier 给流量设置 HTB class，HTB/fq 根据 effective Mbps 做平滑整形。这样协议认证/转发仍完全由官方 sing-box 完成，Vision/AnyTLS/WS 等加密流量在外层 socket 上统一整形。

限制与验收：

- 认证完成前的少量握手流量进入默认 class；事件到达后该连接余下流量进入用户 class。
- WebSocket 经 Nginx 时在 loopback 的 sing-box→Nginx 流上整形；Agent 同时附着物理接口与 loopback。
- Hysteria2/UDP 必须实测同一 QUIC 会话的 source tuple 与 user 映射；不通过时使用协议原生带宽作为降级，UI 标记实际执行器。
- full feature 支持线为 Linux kernel 5.10+、有 BTF/TC；不满足时不能假报“限速已生效”，reported capability 必须为 degraded/unsupported。
- 连接数上限不是“设备数”：Agent 对用户 active event 计数，超限关闭最新连接并上报 reason；IP/设备策略另建模型。

## 5. 路由和负载均衡

- 标准 route 是顺序规则 + final；目标 IR 保持 first-match，并编译到官方 route rule/action/rule-set。[官方 Route](https://sing-box.sagernet.org/configuration/route/)、[Rule Action](https://sing-box.sagernet.org/configuration/route/rule_action/)。
- `selector` 可切换当前出站，`urltest` 原生实现最低延迟候选。[官方 outbounds](https://sing-box.sagernet.org/configuration/outbound/)。
- X 的 `leastPing` 映射 `urltest`；`random`、`roundRobin`、`leastLoad` 没有同名原生 per-connection 算法。Agent 用 1.14 connection event + 官方 API `SelectOutbound` 驱动 selector：按新连接轮转/随机，或按活动连接/吞吐周期选择最小负载。已建立连接不迁移，选择动作影响后续连接。
- 路由 IR 先做可达性/遮蔽/悬空引用检查，再生成 JSON；不能表达的 Xray `attrs` 表达式拒绝并指出字段，不运行任意表达式。

## 6. 版本与能力协商

Agent 启动时上报：

- sing-box semver、commit、build tags、binary SHA-256；
- 支持的 inbound/outbound/transport/TLS/route/API schema；
- OS/kernel/BTF/tc/eBPF、systemd/OpenRC、Nginx、网卡与权限；
- `core.stats.user`、`core.connections.user`、`enforcement.speed.user`、`protocol.snell.server` 等细粒度 capability。

Master 只允许创建所有目标 Agent 都能表达的配置，或要求管理员选择部分部署；任何版本不满足都在 dry-run 阶段失败，不把错误留给远端 reload。

## 7. 已锁定的 ADR 结论

1. sing-box 是独立官方进程，不嵌入 Rust、不维护 fork。
2. 数据库保存领域 IR 和 revision，sing-box JSON只是 Agent 可再生物化产物。
3. 1.13.19 是回退/核心轨道；全功能研发固定 1.14.0-beta.17，正式默认等待/升级到 1.14 stable。
4. 逐用户统计优先 V2Ray stats + 1.14 connection stream；每次 reload 是新 epoch。
5. 通用速度限制由 Rust Agent + Linux tc/eBPF 执行，生效状态必须可证明。
6. XHTTP 不伪装成 sing-box HTTP；提供迁移诊断与标准传输替代。
