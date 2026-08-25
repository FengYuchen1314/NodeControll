# 妙妙屋 → 妙妙屋 X/PRO → NodeControll 差异矩阵

> 社区版基线：`iluobei/miaomiaowu@0b47f10...`，能力源见 [`../02-upstream-features/FEATURE_CATALOG.md`](../02-upstream-features/FEATURE_CATALOG.md)。X 基线为 2026-08-25 的 58 页公开文档，能力源见 [`X_FEATURE_CATALOG.md`](X_FEATURE_CATALOG.md)。X 无可审计主程序源码，因此“X”列表示文档宣称。

## 1. 判定口径

| 标记 | 含义 |
|---|---|
| `继承` | X 文档基本沿用妙妙屋行为，目标需要兼容或迁移。 |
| `扩展` | 社区版已有核心能力，X 增加数据模型、UI、远端执行或边界。 |
| `新增` | 社区版源码中没有对应领域能力。 |
| `改语义` | 名称相似但计量、内核、身份或生命周期不同，不能直接复用模型。 |
| `原PRO` | X 以许可证门控；目标无条件作为普通功能交付。 |
| `降风险` | 目标保留用户结果，但必须替换不安全/模糊机制。 |
| `待内核核验` | 是否能由标准 sing-box 原生表达需以官方能力/真实互通为准。 |

优先级使用 `F0`（工程/安全底座）、`F1`（首个完整闭环）、`F2`（X 全功能）、`F3`（联合与生态）；复杂度为 `S/M/L/XL`。

## 2. 身份、用户与授权

| 能力 | 妙妙屋 | X/PRO | 差异 | NodeControll 决策与验收 | 优先级/复杂度 |
|---|---|---|---|---|---|
| 首次初始化 | `MMW-AUTH-001..003`：空用户表初始化、建首管、可从备份恢复 | X 安装/迁移向导延续首管与恢复语义 | 继承+扩展 | setup token 只在空库有效；创建首管/恢复互斥；E2E 覆盖重放与并发初始化 | F0/M |
| 密码登录 | `MMW-AUTH-004` bcrypt + 持久 UI session | 文档称密码 + JWT | 改语义 | Argon2id、短 access、可撤销 refresh/session；不把长期 JWT 当不可撤销会话 | F0/L |
| 登录防护 | `MMW-AUTH-005` IP+用户名进程内限速 | X 有 Turnstile，未证明持久限速 | 扩展+降风险 | Redis 非必需；DB/内存分层 token bucket、渐进锁定、审计，Turnstile 仅附加 | F0/M |
| Turnstile | `MMW-AUTH-006` 已实现 | `MMWX-SEC-001` 延续 | 继承 | 可选 provider；断网/故障策略可配置，不绕过基本限速 | F1/S |
| TOTP/恢复码 | `MMW-AUTH-007` 已实现 | X FAQ 有本地重置管理员并关闭 2FA | 继承+降风险 | TOTP secret 加密、恢复码 hash；重置需本机 CLI/一次性审计，不提供公开远程后门 | F1/M |
| 会话恢复/撤销 | `MMW-AUTH-008` DB 回填内存 token store | X 文档未给细节 | 继承+改语义 | session 是数据库事实源，设备/UA/最后使用/撤销/全登出测试 | F0/M |
| 个人资料/密码 | `MMW-AUTH-009..010` | X 用户体系继承 | 继承 | 修改密码可选择撤销其他会话；email 与内核 principal 分离 | F1/S |
| 用户 CRUD | `MMW-AUTH-011..012` | `MMWX-USER-001..003` + 套餐/状态 | 扩展 | 软删除保留账本；所有写入有 actor/revision/audit | F1/M |
| 角色 | 仅 admin/user | X 仍为 admin/user；MCP/TG/分享另有 token scope | 扩展 | RBAC 至少 admin/user，服务令牌使用 scopes；后端策略统一 | F0/L |
| 订阅授权 | `MMW-AUTH-013` 文件 allowlist | X 主要由多套餐决定节点/订阅 | 改语义 | 支持文件授权和套餐实例并存，最终权限可 explain | F1/L |
| 用户订阅 token | `MMW-AUTH-014` 长 token 明文查询使用 | `MMWX-USER-004` 可重置 | 继承+降风险 | 只存 hash、scope/audience、轮换、最后使用；日志/Referer 脱敏 | F0/M |
| 短码 | `MMW-AUTH-015` 多类短码 | X 延续短链开关 | 继承 | 类型化 credential，唯一索引、撤销、碰撞/枚举防护 | F1/M |
| 多套餐 | 社区版没有套餐实例 | `MMWX-USER-005`、`PKG-001..007` | 新增 | user↔package_instance 多对多；每实例凭据、周期、baseline、节点快照 | F1/XL |
| 同端口多用户 | 社区版节点订阅不管理服务端 client | X 用 client identity/email 区分 | 新增+改语义 | principal 使用不可变 UUID；email 只作为 sing-box/Xray 兼容 label | F1/L |
| 邀请注册 | 社区版无公开注册 | X TGBot 有邀请码 | 新增 | 次数/有效期/默认角色/套餐模板/滥用控制 | F2/M |

## 3. 流量、套餐、限额与实时执行

| 能力 | 妙妙屋 | X/PRO | 差异 | NodeControll 决策与验收 | 优先级/复杂度 |
|---|---|---|---|---|---|
| 总量/30 天看板 | `MMW-TRAFFIC-001..002` 每日记录 | X 增加服务器/用户/套餐/节点多维页面 | 扩展 | 同一 metrics/ledger 查询层，时间范围/时区/来源清晰 | F1/L |
| 外订阅流量 | `MMW-TRAFFIC-003` 解析 header | `MMWX-TRAFFIC-007` 仍为独立来源 | 继承 | source=`external_subscription`，绝不无依据混入内核计费 | F1/M |
| 文件独立流量 | `MMW-TRAFFIC-004` 文件限额/探针绑定 | X 用套餐作为核心配额 | 改语义 | 保留文件维度兼容迁移；新建优先套餐实例 | F2/L |
| 订阅 header/信息节点 | `MMW-TRAFFIC-005..006` | `MMWX-SET-007` 延续 | 继承 | 与账本同一 snapshot，缓存不串用户 | F1/M |
| 系统网卡计数 | 社区版主要从探针导入 | `MMWX-CORE-001,003,004` Agent 本机 raw counter | 新增 | interface/boot_id/counter 保存；baseline 而非清零 | F1/L |
| 内核流量 | 社区版无服务端内核 | `MMWX-CORE-002` | 新增 | inbound/outbound/principal/tag/revision 维度唯一、采样去重 | F1/XL |
| 三维归属 | 社区版无入站用户计量 | `MMWX-TRAFFIC-001,006` | 新增 | 多跳流量只能计费一次；route user 可反查原套餐 | F1/XL |
| 原始/计费值 | 社区版没有正式不可变账本 | `MMWX-TRAFFIC-002` 倍率后计费 | 新增 | raw event + immutable ledger + derived aggregate，倍率版本化 | F0/XL |
| 单/双向计费 | 社区版总量口径有限 | `MMWX-PKG-007` | 新增 | 方向枚举和公式固化，table-driven tests | F1/M |
| 每日账本 | 社区版 `traffic_records` 日聚合 | `MMWX-TRAFFIC-003` 多维日账本 | 扩展 | late data/rebuild/version/idempotency 测试 | F1/L |
| reset/调整 | 社区版重置语义分散 | `MMWX-TRAFFIC-004..005` baseline+adjustment | 扩展+降风险 | append-only event，reason/actor；从不覆盖原计数 | F1/L |
| 套餐流量/到期 | 社区版只有用户/文件授权 | `MMWX-PKG-003..005` | 新增 | 周期、时区、无限、宽限、独立凭据全建模 | F1/XL |
| 节点倍率 | 社区版没有计费倍率 | `MMWX-PKG-006` | 新增 | raw/billed 同显；变更不重算既往账 | F1/L |
| 速度继承 | 社区版无数据面限速 | `MMWX-PKG-008..012` 原 PRO | 新增+原PRO | 有效策略纯函数：用户节点 > 用户全局 > 套餐节点 > 套餐默认 > unlimited | F2/XL |
| 并发上限 | 社区版无 | X 字段称设备数，实际 connections | 新增+原PRO+改语义 | `max_connections` 与设备/IP policy 分列，真实并发测试 | F2/XL |
| 实时策略推送 | 社区版无 Agent | `MMWX-PKG-014` 内嵌 Xray WS push | 新增+原PRO | snapshot/revision/ack/reconcile；断线恢复后必达 | F2/XL |
| 自动限速/解除 | 社区版仅展示限额 | `MMWX-PKG-015..016` | 新增+原PRO | 阈值、持续、迟滞、恢复、手工覆盖状态机，不振荡 | F2/XL |
| 超限/到期停用 | 社区版可停用户但非自动策略 | `MMWX-PKG-017` | 扩展 | reasoned state + reconcile + 通知，恢复一致 | F1/L |
| 在线/IP/连接追踪 | 社区版探针不是会话追踪 | PRO-005/006 | 新增+原PRO | 聚合上报、隐私保留期、按主体授权查看 | F2/XL |

## 4. 节点、外部订阅与测速

| 能力 | 妙妙屋 | X/PRO | 差异 | NodeControll 决策与验收 | 优先级/复杂度 |
|---|---|---|---|---|---|
| URI/订阅导入 | `MMW-NODE-001..002` | X 继承节点/订阅体系 | 继承 | parser fuzz + SSRF + preview/commit 两阶段 | F1/L |
| 节点 CRUD/批量 | `MMW-NODE-003..006` | X 同时区分托管入站节点和外部节点 | 扩展 | `origin_kind` 决定可编辑/级联；删除影响预览 | F1/L |
| 标签/排序 | `MMW-NODE-007..008` | `MMWX-NODE-003..004` | 继承+扩展 | 多标签规范化、稳定排序、套餐动态选择 | F1/M |
| 协议字段 | `MMW-NODE-009` 客户端配置字段 | X 入站先生成服务端再转换客户端 | 改语义 | 单一 protocol IR 含 server/client views，禁止两套漂移表单 | F1/XL |
| 地址改写/恢复 | `MMW-NODE-010` | `MMWX-NODE-006..007` 增加转发复用 | 扩展 | original/resolved/override 分列，端口转发引用计数 | F2/L |
| 客户端链式代理 | `MMW-NODE-011` Clash dialer/relay | X 新增服务器侧 outbound/routing/tunnel | 改语义 | client chain 与 server route 两种图分开，均做环检测 | F1/XL |
| 探针绑定 | `MMW-NODE-012` | X Agent 自带服务器监控并另有公开探针 | 扩展 | 不再用自由文本 server IDs；FK+source type | F1/M |
| TCPing | `MMW-NODE-013` | X 节点测速包含真连接延迟 | 扩展 | 保存 probe method，不能把 TCPing 当代理可用性 | F1/M |
| 节点测速 | `MMW-NODE-014`,`MMW-SPEED-001..003` 已有本机/远程/历史 | X `MMWX-SPEED-001..007` 工作台并设 PRO | 扩展+原PRO | 复用业务结果但重写 Rust task/executor；普通功能，无 license | F1/XL |
| 临时订阅 | `MMW-NODE-015` 进程内、重启丢失 | X 文档未强调 | 继承+降风险 | DB 存 hash/expiry/scope，清理任务和限速 | F2/M |
| URI 管理视图 | `MMW-NODE-016` | `MMWX-NODE-005` 所有用户/节点 | 扩展 | secret 再认证、分页、审计 | F2/M |
| YAML 自动同步 | `MMW-NODE-017` DB/文件无共同事务 | X 入站事件总线同步节点 | 扩展+降风险 | DB 是事实源；outbox/materializer 原子发布文件 | F0/XL |
| 外订阅 CRUD/同步 | `MMW-EXT-001..003` | X 系统设置延续 | 继承 | ETag、任务租约、失败保留 last-good、手动/定时同服务层 | F1/L |
| 过滤/选择 | `MMW-EXT-004..005` | X 延续 | 继承 | regex 安全，selection session 持久/过期 | F1/M |
| 匹配/范围/保名 | `MMW-EXT-006..008` | X 迁移/认领进一步使用 server:port | 扩展 | stable fingerprint + confidence，歧义人工确认 | F1/L |
| GET 前同步 | `MMW-EXT-009` 可强制 | X 设置延续 | 降风险 | 不阻塞订阅关键路径；stale-while-revalidate + freshness header | F2/M |
| 名称后缀 | `MMW-EXT-010` | X 延续 | 继承 | display name 派生，不写回 identity name | F2/S |
| Tunnel | 社区版只有客户端链 | `MMWX-NODE-008..009` dokodemo-door | 新增+待内核核验 | 用 sing-box 可表达模型/兼容执行器；环路/清理 E2E | F2/XL |

## 5. 订阅文件、生成器、模板、规则与 provider

| 能力 | 妙妙屋 | X/PRO | 差异 | NodeControll 决策与验收 | 优先级/复杂度 |
|---|---|---|---|---|---|
| 文件导入/CRUD/正文 | `MMW-SUB-001..004` | `MMWX-SUB-001` 继承 | 继承+降风险 | 对象/元数据 DB 化或内容寻址；编辑走 revision，不直接绕模型 | F1/L |
| 聚合/排序/复制 | `MMW-SUB-005..007` | X 继承 | 继承 | DAG 无环、稳定 order、copy-on-write secret policy | F1/M |
| 公开/私有/用户授权 | `MMW-SUB-008..009` | X 套餐 token 为主 | 改语义 | policy explain：公开、文件 ACL、套餐 entitlement 三层 | F1/L |
| 短链/标准 URL | `MMW-SUB-010..011` | X 延续 | 继承 | canonical URL builder，可信 origin，token 脱敏 | F1/M |
| token/短码访问 | `MMW-SUB-012..013` | X 用户/套餐 token | 扩展 | credential type/audience 不能混用 | F0/L |
| UA/format | `MMW-SUB-014..015` | X 宣称 12+ 格式 `MMWX-SUB-002..003` | 扩展 | 显式格式优先，每 producer golden/schema/client parse | F1/XL |
| 缓存/header/content disposition | `MMW-SUB-016..018` | X 继承 | 继承+降风险 | user-scoped cache key，ETag，no-store secret views，安全文件名 | F1/M |
| 生成器流水线 | `MMW-GEN-001..005` | `MMWX-SUB-004..005` | 继承+扩展 | typed IR + deterministic pipeline + explain trace | F1/XL |
| 模板 CRUD/默认 | `MMW-TPL-001..004` | X V3 延续 | 继承 | schema/version/revision，默认资源可离线替换 | F1/L |
| 模板代理组/DSL/脚本 | `MMW-TPL-005..008` | X V3 字段更完整 | 扩展+降风险 | 结构化 DSL 优先；脚本严格 sandbox/配额/禁网 | F2/XL |
| 规则集/模板/订阅 | `MMW-RULE-001..006` | X custom-rules 延续 | 继承 | source hash/signature、解析诊断、规则引用 FK、离线包 | F1/L |
| Provider CRUD/缓存 | `MMW-PP-001..005` | `MMWX-SUB-009,014` | 继承+扩展 | SSRF/token scope/cache revision/last-good | F1/L |
| Provider 覆写/过滤/GeoIP | `MMW-PP-006..008` | X 延续 | 继承 | typed transform pipeline + fuzz/golden | F2/L |
| 远程预设同步 | 社区版已有默认模板/资源但更新路径分散 | `MMWX-SUB-015` | 扩展+降风险 | 任意管理员 URL、本地导入、签名/hash/rollback；无官方域名强依赖 | F2/L |
| 自定义品牌 | 社区版主题/字体 | X 对品牌设 PRO | 扩展+原PRO | 名称/logo/favicon/色彩/背景全普通可用，资产本地化 | F2/M |

## 6. Agent 与 sing-box 内核编排（X 的最大净新增）

| 能力 | 妙妙屋 | X/PRO | 差异 | NodeControll 决策与验收 | 优先级/复杂度 |
|---|---|---|---|---|---|
| 服务器登记/token | 无 | `MMWX-AGENT-001..002,015` | 新增 | token hash/一次显示/rotation overlap/audience | F0/L |
| WS/HTTP/Pull/Auto | 无 | `MMWX-AGENT-003..006` | 新增 | 任务 envelope、租约、idempotency、sequence、模式切换状态机 | F0/XL |
| 多 init 部署 | 无 | `MMWX-AGENT-007..009` | 新增 | systemd/OpenRC/容器；非 root/沙箱；真实 VM 测试 | F1/L |
| Agent 配置/重连 | 无 | `MMWX-AGENT-010,016` | 新增 | schema、secret redact、backoff、不停已有数据面 | F0/L |
| 在线/指标/网络 | 社区版探针是独立 HTTP 数据源 | `MMWX-AGENT-011..013` | 新增+改语义 | heartbeat + metrics series；缺失非 0；boot/interface 处理 | F1/XL |
| 批量升级 | 无 | `MMWX-AGENT-014` | 新增 | signed artifact、canary、并发、rollback、架构匹配 | F2/XL |
| 受限任务执行 | 无 | `MMWX-AGENT-017` | 新增 | 无任意 shell；typed capability allowlist + audit | F0/XL |
| managed/external | 无 | `MMWX-AGENT-018`，内嵌为 PRO | 新增+原PRO | Agent 自带标准 sing-box；external 模式只对可安全管理能力开放 | F1/XL |
| 网卡/内核双口径 | 探针数据 | `MMWX-CORE-001..005` | 新增 | raw/source/baseline 模型 | F1/XL |
| 扫描/安装/卸载 | 无 | `MMWX-CORE-006..008` | 新增 | discovery/claim 分离、hash、备份、高危确认 | F1/XL |
| 启停/状态/版本 | 无 | `MMWX-CORE-009,015` | 新增 | desired/current/health/revision，最终状态确认 | F1/L |
| 配置查看/编辑 | 社区版只生成客户端 YAML | `MMWX-CORE-010..014` | 新增+改语义 | secret mask、schema dry-run、snapshot、atomic apply、rollback | F1/XL |
| 同服务器串行化 | 社区版单进程但无远程配置竞争 | `MMWX-CORE-016` | 新增 | per-server writer/lease + optimistic config revision | F0/L |
| Xray→sing-box | 不适用 | X 所有内核模型围绕 Xray/fork | 改语义+待内核核验 | 建独立 desired IR 和 sing-box compiler；不能保存 Xray JSON为核心模型 | F0/XL |

## 7. 入站协议矩阵

| 组合组 | 妙妙屋 | X/PRO | 差异 | NodeControll 决策与验收 | 优先级/复杂度 |
|---|---|---|---|---|---|
| 入站向导/端口/凭据 | 无服务端管理 | `MMWX-IN-001..004` | 新增 | capability schema、Agent 实际端口检查、CSPRNG、事务同步节点 | F1/XL |
| VLESS TCP TLS/REALITY/Vision | 仅可导入客户端节点 | `MMWX-IN-005..008` | 新增+改语义 | 标准 sing-box server/client 双配置互通；版本门控 Vision | F1/XL |
| VLESS WS/gRPC | 仅客户端字段 | `MMWX-IN-009..010` | 新增 | 反代、HTTP2、path/service/REALITY E2E | F1/L |
| VLESS XHTTP REALITY | 社区版可含相似客户端字段 | `MMWX-IN-011` Xray 组合 | 待内核核验 | 标准 sing-box 不等价则明确兼容方案，不生成假配置 | F2/XL |
| Trojan TLS | 客户端节点 | `MMWX-IN-012` | 新增 | sing-box/Mihomo/Shadowrocket 互通 | F1/L |
| Trojan REALITY/gRPC REALITY | 客户端模型能力有限 | `MMWX-IN-013..014` | 待内核核验 | 以官方 sing-box 支持为准，无法原生则隔离适配 | F2/XL |
| VMess TCP/WS ± TLS | 客户端节点 | `MMWX-IN-015..018` | 新增 | 四组合 server/client/producer 测试 | F1/L |
| Shadowsocks AEAD/2022 | 客户端节点 | `MMWX-IN-019..020` | 新增 | method/key/multi-user/URI 互通 | F1/L |
| Hysteria2 | 客户端节点 | `MMWX-IN-021` Xray fork | 改语义 | 直接使用标准 sing-box Hysteria2，QUIC/bandwidth/auth/cert E2E | F1/XL |
| AnyTLS TLS | 客户端可能已有解析 | `MMWX-IN-022` Xray fork | 改语义 | 直接使用标准 sing-box AnyTLS，producer/客户端互通 | F2/L |
| AnyTLS REALITY | 无 | `MMWX-IN-023` X 文档自称客户端不支持 | 改语义+已核验 schema | 官方 AnyTLS 复用含 Reality 的 TLS schema；先做 sing-box 双端互通，再按 producer 客户端矩阵开放 | F2/L |
| Snell v4/v5/v6 服务端 | 社区版主要客户端转换 | `MMWX-IN-024..025` Xray fork | 改语义+版本门控 | 官方 sing-box 1.14 已加入 v5/v6 server，v5 wire 等价 v4；开发固定官方 beta，发布升级官方稳定版，不维护私有 fork | F2/XL |
| 废弃组合迁移 | 社区版可保存旧节点 | `MMWX-IN-026` | 扩展 | import diagnostics + replacement guidance，不静默丢字段 | F2/M |
| 入站→客户端 producer | 社区版已有多 producer | `MMWX-IN-027` | 扩展 | protocol IR 单源，格式 golden tests | F1/XL |

## 8. 出站、路由、WARP 和用户路由出站

| 能力 | 妙妙屋 | X/PRO | 差异 | NodeControll 决策与验收 | 优先级/复杂度 |
|---|---|---|---|---|---|
| direct/block | 客户端模板有 DIRECT/REJECT | `MMWX-OUT-001` 服务端出站 | 改语义 | client policy 与 server outbound 分离 | F1/M |
| 代理/隧道出站 | 客户端 relay/dialer | `MMWX-OUT-002..003` | 新增 | server graph、健康和环检测 | F1/XL |
| WARP 生命周期 | 无 | `MMWX-OUT-004..008` | 新增 | WireGuard 标准配置、凭据 secret、幂等/注销/回滚；CF 非系统强依赖 | F2/XL |
| first-match/AND/default | 客户端规则语义依目标格式 | `MMWX-ROUTE-001..002` | 新增+改语义 | sing-box route IR + simulator/compiler parity | F1/XL |
| 专属/全局/catch-all | 无服务端路由 | `MMWX-ROUTE-003..004` | 新增 | stable inbound ID + unreachable analysis | F1/L |
| 拖排/系统规则 | 无服务端路由 | `MMWX-ROUTE-005` | 新增 | revision/set/locked system rules/rollback | F1/L |
| 完整匹配字段 | 客户端规则支持部分相似字段 | `MMWX-ROUTE-006..007` | 改语义+待核验 | capability matrix；不可表达字段拒绝并解释 | F1/XL |
| 快捷规则 | 社区版规则模板 | `MMWX-ROUTE-008..009` | 扩展 | 本地可编辑/版本化，无官方域名数据强依赖 | F2/M |
| random/roundRobin | 客户端 proxy group | `MMWX-ROUTE-010..011` 服务端 balancer | 改语义+待核验 | 用 sing-box 可表达机制或 Agent 调度扩展，行为压测 | F2/XL |
| leastPing/leastLoad | 客户端 url-test/load-balance | `MMWX-ROUTE-012..013` observatory | 改语义+待核验 | 健康指标/失败窗口/选路行为可观测 | F2/XL |
| 节点级路由出站 | 社区版只有客户端链 | `MMWX-ROUTE-014` | 新增 | catch-all route + lifecycle/reference E2E | F2/L |
| 用户私有路由出站 | 无 | `MMWX-ROUTE-015..017` | 新增 | principal/client/route/subscription/traffic 原子链；配额/暂停 reconcile | F2/XL |

## 9. 证书、网站、公开探针与外部集成

| 能力 | 妙妙屋 | X/PRO | 差异 | NodeControll 决策与验收 | 优先级/复杂度 |
|---|---|---|---|---|---|
| ACME DNS/供应商 | 无服务器证书管理 | `MMWX-CERT-001..003` | 新增 | adapter、DNS-01 状态机、secret vault、SAN验证 | F2/XL |
| 续期/Agent 部署 | 无 | `MMWX-CERT-004..005` | 新增 | lock/retry/alert、0600 原子部署、reload rollback | F2/XL |
| 下载 PEM | 无 | `MMWX-CERT-006` | 新增+高风险 | re-auth、审计、no-store、默认不展示 key | F2/M |
| Certimate webhook | 无 | `MMWX-CERT-007` | 新增 | scoped token、body/cert-key validation、幂等 | F2/L |
| Nginx 发现/静态/反代 | 无 Agent 站点管理 | `MMWX-SITE-001..004` | 新增 | 类型化站点模型、path/port/loop/SSRF 安全 | F2/XL |
| 安全删除站点 | 无 | `MMWX-SITE-005` | 新增+降风险 | ownership marker + DB ID + realpath，绝不删非托管配置 | F2/L |
| 社区探针服务器 | `MMW-PROBE-001..005` 登录/状态/历史/隐藏 | X Agent 指标 + public probe API | 改语义 | 支持 Agent 原生指标与兼容导入；public projection 独立 | F1/XL |
| 外置 Worker 探针 | 社区版无此公开网关 | `MMWX-PROBE-002` | 新增 | 可替换/可自托管网关，origin token rotation | F2/L |
| 公共 snapshot/WS/series | 社区版探针更多是管理内数据源 | `MMWX-PROBE-001,003..005` | 新增 | allowlist schema、rate limit、WS backpressure、range limits | F2/L |
| 可选公开字段 | `MMW-PROBE-*` 有部分 | `MMWX-PROBE-006` 更广 | 扩展 | 每字段公开开关，missing != zero，URL 清理 | F2/M |
| Telegram 通知 | `MMW-NOTIFY-001..002` 通知配置 | X 内嵌 bot/命令/Mini App | 扩展 | 通知先 F1；账号绑定、命令、Mini App initData 在 F2 | F1-F2/XL |
| MCP | 无 scoped API/MCP | `MMWX-MCP-001..005` 26 tools | 新增 | scopes、allowlist、同 service auth、高危两阶段 intent | F3/XL |
| 分享服务器 | 无 | `MMWX-SHARE-001..007` 原 PRO | 新增+原PRO | 双方自托管、mTLS/身份钉扎、最小权限、禁止转授、契约/E2E | F3/XL |

## 10. 运维、安全、部署和 UI

| 能力 | 妙妙屋 | X/PRO | 差异 | NodeControll 决策与验收 | 优先级/复杂度 |
|---|---|---|---|---|---|
| 日志/系统信息 | `MMW-OPS-001..002` | X 远程服务器/Agent 进一步扩展 | 扩展 | structured log、trace/correlation、secret redaction、分页/导出 | F0/L |
| 维护任务 | `MMW-OPS-003` | X 多类任务更多 | 扩展 | durable job state/lease/retry/cancel/actor/audit | F0/XL |
| 备份/恢复 | `MMW-OPS-004..005` | `MMWX-PLAT-012..014` SQLite+PG/资源 | 扩展 | manifest/hash/encryption option/restore drill | F0/XL |
| 更新检查/应用内更新 | `MMW-OPS-006,009` 可覆写容器内 binary | X 主控/Agent 更新 | 扩展+降风险 | signed immutable artifacts，不在容器 volume 自我替换镜像；支持 rollback | F2/XL |
| 配置/订阅清理 | `MMW-OPS-007..008` | X 多资源生命周期 | 扩展 | referential preview + soft delete + async GC | F2/L |
| CORS/安全 header | `MMW-SEC-001..002` | X 文档未详细 | 继承+增强 | strict origin、CSP、HSTS、frame/permissions policy，浏览器 E2E | F0/M |
| SSRF | `MMW-SEC-003` 已保护外拉 | X 外订阅、webhook、反代、WARP 等扩大面 | 扩展 | 单一 egress policy，DNS rebinding/redirect/IPv6/metadata tests | F0/L |
| 路径安全 | `MMW-SEC-004` | X 证书/Nginx/Agent 配置扩大面 | 扩展 | realpath/allowlist/ownership marker，无任意文件 API | F0/L |
| 限流/body/恢复模式 | `MMW-SEC-005..007` | X 增加 public/MCP/联邦端点 | 扩展 | endpoint policy matrix、durable limiter、break-glass local-only | F0/L |
| SaaS UI | `MMW-UI-001..004` 猫咪主题、两套节点编辑、移动不完整 | X 文档有管理面板但设计非目标约束 | 全面重构 | Vue3+Vuetify；统一 responsive form、导航、data table、a11y、i18n | F0-2/XL |
| 单二进制 | `MMW-DEPLOY-001` Go embed SPA | X Master/Agent 分离 | 改语义 | Rust Master 可 embed Vue dist；Agent 单独静态链接/容器制品 | F1/L |
| Docker/systemd/portable/Windows | `MMW-DEPLOY-002..005` | X 主控/Agent Linux 为主，文档含多 arch | 扩展+取舍 | 首发 Linux amd64/arm64 + Docker/systemd/OpenRC；Windows tester 后续，控制面不承诺 Windows 服务端 | F1-2/XL |
| SQLite/PostgreSQL | 社区版仅 SQLite | X 支持两者 | 扩展 | SQLx migrations/queries 双库 CI；SQLite 单写和 PG 并发路径分别测试 | F0/XL |
| 迁移妙妙屋 | 无需自迁 | X 五步迁移 | 新增 | `MMWX-PLAT-015..018` dry-run/report/rollback 全实现 | F2/XL |
| 去许可证 | 社区版无官方许可证 | X PRO/机器 ID/额度/官方激活 | 原PRO+降风险 | 全部 `PRO-*` 普通可用；静态/断网/E2E 验证 `NOLIC-*` | F0/M |

## 11. 完整覆盖索引

下表证明两个能力目录的 ID 域均已被比较；详细到单项的行为/验收仍以相应目录行及上表映射为准。

| 来源域 | ID 范围 | 本矩阵位置 |
|---|---|---|
| 妙妙屋身份 | `MMW-AUTH-001..015` | §2 |
| 妙妙屋流量 | `MMW-TRAFFIC-001..006` | §3 |
| 妙妙屋节点 | `MMW-NODE-001..017` | §4 |
| 妙妙屋外订阅 | `MMW-EXT-001..010` | §4 |
| 妙妙屋订阅文件 | `MMW-SUB-001..018` | §5 |
| 妙妙屋生成器 | `MMW-GEN-001..005` | §5 |
| 妙妙屋模板 | `MMW-TPL-001..008` | §5 |
| 妙妙屋规则 | `MMW-RULE-001..006` | §5 |
| 妙妙屋 provider | `MMW-PP-001..008` | §5 |
| 妙妙屋探针/测速 | `MMW-PROBE-001..005`、`MMW-SPEED-001..003` | §4、§9 |
| 妙妙屋通知 | `MMW-NOTIFY-001..002` | §9 |
| 妙妙屋运维/安全/UI/部署 | `MMW-OPS-001..009`、`MMW-SEC-001..007`、`MMW-UI-001..004`、`MMW-DEPLOY-001..005` | §10 |
| X 平台/Agent/内核 | `MMWX-PLAT-*`、`AGENT-*`、`CORE-*` | §6、§10 |
| X 入站/节点/出站/路由 | `MMWX-IN-*`、`NODE-*`、`OUT-*`、`ROUTE-*` | §4、§7、§8 |
| X 用户/套餐/账本 | `MMWX-USER-*`、`PKG-*`、`TRAFFIC-*` | §2、§3 |
| X 订阅/测速 | `MMWX-SUB-*`、`SPEED-*` | §4、§5 |
| X 证书/站点/设置 | `MMWX-CERT-*`、`SITE-*`、`SET-*`、`NOTIFY-*` | §9、§10 |
| X 探针/安全/TG/MCP/分享 | `MMWX-PROBE-*`、`SEC-*`、`TG-*`、`MCP-*`、`SHARE-*` | §9 |

## 12. 结论

1. 妙妙屋最成熟的部分是节点/订阅/模板/provider 转换流水线；应迁移行为和测试样例，不迁移 Go/React 技术结构。
2. X 的最大净新增是 Master-Agent + 服务端内核编排 + 套餐/用户数据面计量执行；这是重构的核心风险，不是 UI 工作。
3. X 的“内嵌 Xray”不能原样迁移：目标以标准 sing-box 为事实内核，需要独立能力编译层、版本探测和兼容隔离。
4. 所有原 PRO 能力均保留用户结果并去除授权；节点测速属于社区已有能力的扩展，不应重复造一个互不兼容体系。
5. 分享服务器、MCP、证书私钥和远程内核控制构成最高安全风险面，必须在 F0 的身份、审计、任务和 secret 底座之上实现。
