# 妙妙屋现有功能目录

> 基线：`iluobei/miaomiaowu@0b47f10c52aee10b9f759a593ca5f61a823cbb72`（应用版本 `0.8.3`）。这里的“已实现”以源码、路由、数据表和 VPS 构建结果为准，不按宣传文案推断。源码入口可沿 [`../01-upstream-source/HTTP_API.md`](../01-upstream-source/HTTP_API.md) 和 [`../01-upstream-source/DATA_FLOWS.md`](../01-upstream-source/DATA_FLOWS.md) 追踪。

## 1. 产品定位与角色

妙妙屋是一个单实例、自托管的个人/小团队代理节点与订阅管理面板。它管理节点、外部订阅、Clash/Surge 模板、规则、用户授权、流量探针和订阅发布，但不在社区版中直接管理 Xray/sing-box 服务端入站。

| 角色 | 可见能力 |
|---|---|
| 未登录访问者 | 首次初始化、登录、2FA；用长期 token/短码读取被授权订阅；访问临时订阅或 proxy-provider。 |
| 普通用户 | 流量看板、个人设置、自己的订阅链接、公开/自有模板；部分外部订阅/provider 数据由后端按 owner 限制。 |
| 管理员 | 节点、订阅文件、生成器、模板、规则、探针、用户、系统、安全、日志、备份与更新。 |
| 外部订阅客户端 | 依据 token、短码、UA 和 format 获取转换后的配置。 |
| 远程测速器 | 用独立 tester token 建 WebSocket，接收测速任务并返回结果。 |

Topbar 会按 `profile.is_admin` 隐藏管理导航；安全边界仍由后端 `RequireAdmin` 和资源 owner 检查建立。

## 2. 功能全量矩阵

“边界/依赖”列记录功能在社区版中的实际限制，而不是计划中的改进。

### 2.1 初始化、身份与用户

| ID | 功能 | 角色/入口 | 已实现行为 | 边界/依赖 |
|---|---|---|---|---|
| MMW-AUTH-001 | 首次安装状态 | 公开 `/login` | 检查用户表是否为空，切换初始化/登录界面 | 只以用户数判断 |
| MMW-AUTH-002 | 创建首个管理员 | 首次初始化 | 用户名、密码、昵称、邮箱、头像；再次检查空库 | 不提供公开注册 |
| MMW-AUTH-003 | 由备份初始化 | 首次初始化 | 上传并恢复备份后接管旧实例数据 | 归档格式与当前版本耦合 |
| MMW-AUTH-004 | 密码登录 | 所有用户 | bcrypt 校验、remember-me、签发并持久化 UI session | 前端 token 位于 JS 可读 Cookie |
| MMW-AUTH-005 | 登录限速 | 登录端点 | 按 IP + 用户统计失败、锁定、成功清零 | 限速状态主要在进程内 |
| MMW-AUTH-006 | Cloudflare Turnstile | 登录/系统设置 | 管理员配置 site key/secret，登录时服务端验证 | 依赖 Cloudflare 外网 |
| MMW-AUTH-007 | TOTP 两步验证 | 个人设置/登录 | 密码确认、secret/QR、首次 code、8 个恢复码、禁用 | 恢复码登录会关闭当前 TOTP |
| MMW-AUTH-008 | UI 会话重启恢复 | 后端 | sessions 落 SQLite，启动时回填未过期 token | TokenStore 仍为单进程内存索引 |
| MMW-AUTH-009 | 修改个人密码 | 个人设置 | 校验旧密码后更新 bcrypt hash | 会话撤销语义不突出 |
| MMW-AUTH-010 | 个人资料 | 个人设置 | 展示/维护昵称、邮箱、头像和角色信息 | UI 页面更新能力以实际 API 为准 |
| MMW-AUTH-011 | 用户 CRUD | 管理员 `/users` | 列表、新增、删除、启停、重置密码 | 管理员角色模型仅 admin/user |
| MMW-AUTH-012 | 用户备注 | 管理员 `/users` | 管理员维护备注 | 仅管理用途 |
| MMW-AUTH-013 | 用户订阅授权 | 管理员 `/users` | 为每个用户覆盖可访问订阅文件集合 | 关系表持久化 |
| MMW-AUTH-014 | 长期订阅 token | 个人设置 | 首次自动创建、显示、复制、重置 | 常出现在 query URL |
| MMW-AUTH-015 | 用户/文件短码 | 个人/系统设置 | 系统短码、自定义短码、文件短码和组合短链 | 多类凭据概念容易混淆 |

### 2.2 流量看板

| ID | 功能 | 入口 | 已实现行为 | 边界/依赖 |
|---|---|---|---|---|
| MMW-TRAFFIC-001 | 总流量摘要 | `/` | 展示 limit/used/remaining 与百分比 | 数据来自探针和/或外订阅 |
| MMW-TRAFFIC-002 | 30 天趋势 | `/` | 查询 `traffic_records` 最近 30 天并绘制 AreaChart | 每日采集；非实时曲线 |
| MMW-TRAFFIC-003 | 外部订阅流量 | 系统设置 | 解析 `Subscription-Userinfo` 并叠加 | 来源需提供规范 header |
| MMW-TRAFFIC-004 | 订阅文件独立流量 | 订阅管理 | 按文件 traffic_limit 或绑定探针 server IDs 汇总 | 服务器 ID 以 JSON/TEXT 数组保存 |
| MMW-TRAFFIC-005 | 订阅响应头 | 系统设置 | 可写 `Subscription-Userinfo` 给客户端 | 可关闭以减少外部请求延迟 |
| MMW-TRAFFIC-006 | 信息节点 | 系统设置 | 可把剩余流量/到期时间作为特殊节点注入订阅 | 前缀可配置，属于展示技巧 |

### 2.3 节点管理

| ID | 功能 | 入口 | 已实现行为 | 边界/依赖 |
|---|---|---|---|---|
| MMW-NODE-001 | 手工节点导入 | `/nodes` | 粘贴 URI/base64，解析后批量预览/编辑/保存 | 必须得到支持的协议与必填字段 |
| MMW-NODE-002 | 订阅 URL 导入 | `/nodes` | SSRF 安全拉取远端、解析候选，不立即落库 | 外部网络和格式正确性 |
| MMW-NODE-003 | 节点 CRUD | `/nodes` | 单条新增、字段编辑、启停、删除 | 当前无软删除/版本 |
| MMW-NODE-004 | 批量创建/删除/清空 | `/nodes` | 批量保存、选中删除、全部清空并确认 | 文件同步可能与 DB 部分失败 |
| MMW-NODE-005 | 批量重命名 | `/nodes` | 按规则批量修改节点名 | 同步 YAML 引用 |
| MMW-NODE-006 | 去重节点 | `/nodes` | 识别重复并选择删除 | 判定口径由页面/Handler 实现 |
| MMW-NODE-007 | 标签 | `/nodes` | 单条/批量标签、标签过滤、按标签生成 | 标签存 JSON/TEXT |
| MMW-NODE-008 | 启停与排序 | `/nodes` | enabled、drag/drop 顺序，生成时按 node_order | 多处配置需同步顺序 |
| MMW-NODE-009 | 协议专有字段 | 节点编辑器 | TLS/Reality/传输/skip-cert/指纹等配置 | 桌面/移动编辑实现不完全同构 |
| MMW-NODE-010 | server 改写与恢复 | `/nodes` | DNS 解析、临时改写、保存原值、恢复 | 原值与同步状态需一致 |
| MMW-NODE-011 | 链式代理 | `/nodes`/生成器 | dialer-proxy、relay 组注入和引用修剪 | 基于 Clash/Mihomo 语义 |
| MMW-NODE-012 | 探针绑定 | `/nodes` | 绑定一个/多个探针 server IDs | 需启用功能并先同步探针 |
| MMW-NODE-013 | TCPing | `/nodes` | 单个和批量 TCP connect 延迟 | 不代表代理链路可用 |
| MMW-NODE-014 | 节点测速 | `/nodes` | 延迟、下载、出口 IP、历史 | 本地依赖下载 Mihomo，或远程 tester |
| MMW-NODE-015 | 临时订阅 | `/nodes` | 选择节点后生成随机 8 位、限时 `/t/{code}` | 仅进程内，重启丢失 |
| MMW-NODE-016 | URI 复制 | `/nodes` | 展示/复制原始或生成 URI | 输出兼容取决于 producer |
| MMW-NODE-017 | YAML 自动同步 | 节点变更 | 更新多个订阅文件的 proxies/组/relay 引用 | SQLite 与文件系统无共同事务 |

### 2.4 外部订阅同步

| ID | 功能 | 入口 | 已实现行为 | 边界/依赖 |
|---|---|---|---|---|
| MMW-EXT-001 | 外部订阅 CRUD | `/subscribe-files` | URL、更新间隔、过滤、前缀、流量同步等 | owner 范围由后端控制 |
| MMW-EXT-002 | 手动同步全部/单条 | 订阅管理 | 拉取、解析、匹配、写节点/YAML、记录结果 | 远端失败策略不完全统一 |
| MMW-EXT-003 | 自动更新 | 后台任务 | 每分钟扫描到期、按间隔同步 | 单实例 goroutine，无租约 |
| MMW-EXT-004 | 节点 include/exclude | 外订阅配置 | 名称正则过滤并可预检查命中 | 错误正则阻止同步 |
| MMW-EXT-005 | 节点选择确认 | 节点/同步对话框 | 新候选先存 selection session，再确认 | 会话仅内存 |
| MMW-EXT-006 | 匹配策略 | 系统设置 | 名称、server+port、协议+server+port | 错误口径会重复或错配节点 |
| MMW-EXT-007 | 同步范围 | 系统设置 | 仅已保存节点或全部节点 | 会影响数据库增长 |
| MMW-EXT-008 | 保留本地名称 | 系统设置 | 更新远端字段时可保留已有 node name | 依赖稳定匹配 |
| MMW-EXT-009 | 获取订阅时强制同步 | 系统设置 | 对外 GET 前刷新引用来源 | 显著增加响应延迟/失败面 |
| MMW-EXT-010 | 流量/到期后缀 | 系统设置 | 同步时把余量和天数附加到节点名 | 名称会随流量变化 |

### 2.5 订阅文件与发布

| ID | 功能 | 入口 | 已实现行为 | 边界/依赖 |
|---|---|---|---|---|
| MMW-SUB-001 | 上传订阅文件 | `/subscribe-files` | multipart 上传配置并建立元数据 | 文件名严格清理 |
| MMW-SUB-002 | 从 URL/配置导入 | `/subscribe-files` | 导入外部内容或生成配置 | SSRF 与格式校验 |
| MMW-SUB-003 | 订阅文件 CRUD | `/subscribe-files` | 名称、描述、短码、公开/授权、内容等 | 元数据与正文分离 |
| MMW-SUB-004 | 正文编辑 | `/subscribe-files` | 读取/覆盖 YAML，编辑节点、规则和代理组 | 直接改文件可绕过高层模型 |
| MMW-SUB-005 | 聚合订阅 | `/subscribe-files` | 选择多个文件创建 aggregate | 聚合引用存在元数据中 |
| MMW-SUB-006 | 文件排序 | `/subscribe-files` | 保存展示/处理顺序 | order 数值维护 |
| MMW-SUB-007 | 用户可见订阅页 | `/subscription` | 列授权文件，生成 URL、复制和二维码 | 需要长期 token |
| MMW-SUB-008 | 组合短链 | 根路径 | 文件短码 + 用户短码解析并检查授权 | 枚举失败进入暴力防护 |
| MMW-SUB-009 | UA 自动格式 | 对外 GET | 识别 Clash/Surge/Loon/sing-box 等 UA | 可选择阻断未知 UA |
| MMW-SUB-010 | Clash/Mihomo 输出 | 对外 GET | YAML、排序、去重、规则和模板注入 | 核心语义偏 Clash |
| MMW-SUB-011 | Surge 输出 | 对外 GET | Clash 转 Surge 或 Surge 模板分支 | 协议支持是目标客户端子集 |
| MMW-SUB-012 | Loon 输出 | 对外 GET | 生成 Loon 配置 | 同上 |
| MMW-SUB-013 | JSON 输出 | 对外 GET | YAML AST 转稳定/紧凑 JSON | 非强类型 schema |
| MMW-SUB-014 | 其他客户端 producers | 前端生成 | QX、Shadowrocket、Stash、Surfboard、Egern、URI、V2Ray、sing-box 等 | 多为宽松 Sub-Store 转换代码 |
| MMW-SUB-015 | Snell 兼容过滤 | 对外 GET | 老 Clash UA 自动剔除 Snell v6 和组引用 | 特定客户端兼容补丁 |
| MMW-SUB-016 | 无效凭据伪装内容 | 对外 GET | 可返回 `token_invalid.yaml` 风格内容 | 行为与标准 401 不一致 |
| MMW-SUB-017 | 订阅频率限制 | 对外 GET | 按 IP 窗口限制 | 内存桶，重启清零 |
| MMW-SUB-018 | 旧 subscription links | 管理 API | 上传、更新、删除旧链接模型 | 与当前 subscribe_files 并存 |

### 2.6 proxy-provider

| ID | 功能 | 入口 | 已实现行为 | 边界/依赖 |
|---|---|---|---|---|
| MMW-PP-001 | Provider 配置 CRUD | `/subscribe-files` | 绑定外订阅、名称、处理模式、刷新间隔 | 需开启功能 |
| MMW-PP-002 | 名称/GeoIP 过滤 | Provider 配置 | include、exclude、GeoIP 规则 | GeoIP 调用外部服务/缓存有限 |
| MMW-PP-003 | 字段覆写 | Provider 配置 | 用 JSON 覆盖节点字段 | 宽松对象模型可能产生非法组合 |
| MMW-PP-004 | 客户端/MMW 模式 | Provider 配置 | 选择由客户端处理或服务端生成 | 行为需结合目标模板 |
| MMW-PP-005 | 对外 provider URL | `/api/proxy-provider/{id}` | token 鉴权后返回 `proxies:` YAML | query token 可能进日志 |
| MMW-PP-006 | 手动刷新/预览 | 订阅管理 | 刷缓存、看节点和最终输出 | 缓存在内存 |
| MMW-PP-007 | 自动缓存调度 | 后台 | 预热、到期 worker、失败退避、stale 状态 | 多实例重复刷新；重启全失 |
| MMW-PP-008 | 批量按地域/协议创建 | 订阅管理 | 根据外订阅候选和过滤器生成多个 provider | 依赖内置正则与命中预检 |

### 2.7 生成器、模板与规则

| ID | 功能 | 入口 | 已实现行为 | 边界/依赖 |
|---|---|---|---|---|
| MMW-GEN-001 | 可视化节点选择 | `/generator` | 按标签/搜索选节点和 provider | 页面约 3,635 行，状态复杂 |
| MMW-GEN-002 | 拖拽代理组 | `/generator` | 把节点/集合拖入代理组并排序 | Clash 组语义 |
| MMW-GEN-003 | 预定义规则分类 | `/generator` | 从远程代理组目录选择规则分类 | 首次外部同步失败则空目录 |
| MMW-GEN-004 | 生成预览 | `/generator` | 输出 YAML/客户端格式，校验与复制 | 生成逻辑分前后端两处 |
| MMW-GEN-005 | 保存为订阅 | `/generator` | 将当前配置建立为订阅文件 | 进入文件+元数据双写 |
| MMW-TPL-001 | V2 数据库模板 | `/templates/` | CRUD、远端取源、预览和转换 | 遗留模型 |
| MMW-TPL-002 | V3 文件模板 | `/templates-v3/` | YAML/Surge 文件上传、读取、修改、重命名、删除 | 正文在文件系统 |
| MMW-TPL-003 | 模板所有权/公开性 | V3 模板 | owner 可改，public 可读，admin 全权 | `/api/admin` 路径命名误导 |
| MMW-TPL-004 | 默认模板 | 模板页 | 每用户选择默认 Clash/Surge 模板 | 引用文件名，重命名需协调 |
| MMW-TPL-005 | 可视化代理组编辑 | V3 模板 | 组类型、正则、节点/provider/区域占位、顺序 | Surge 模板只显示代码模式 |
| MMW-TPL-006 | V2→V3 转换 | 模板上传 | 服务端分析旧模板并生成 V3 | 转换可能需要人工校对 |
| MMW-TPL-007 | 订阅分析建模板 | 模板上传 | 拉取订阅并分析代理组/规则结构 | 外部 URL 风险与格式依赖 |
| MMW-TPL-008 | 带标签/节点预览 | 模板页/生成器 | 注入实际节点、relay、rules/provider 后预览 | 预览不等于客户端实际运行 |
| MMW-RULE-001 | 规则文件编辑 | `/rules` | 文件列表、正文、保存、历史 | 规则正文仍是文件 |
| MMW-RULE-002 | 自定义 DNS/rules/provider | `/custom-rules` | CRUD、分类、启停、应用顺序 | 合并策略需理解 YAML AST |
| MMW-RULE-003 | 追加/替换和去重 | 发布链 | 合并 DNS/rules/rule-providers 并补代理组 | 顺序会改变匹配结果 |
| MMW-RULE-004 | JavaScript 覆写 | `/custom-rules` | post-fetch/pre-save-nodes、console、produce | goja VM 5 秒；脚本有高权限数据访问 |
| MMW-RULE-005 | 内置规则/脚本模板 | 编辑器 | 快速填充常用规则与脚本 | 需随客户端规则语义维护 |
| MMW-RULE-006 | Clash 配置校验 | 前端/后端 | 代理/组/引用/环路、字段排序和修正建议 | 不是 sing-box schema 校验 |

### 2.8 探针、测速和通知

| ID | 功能 | 入口 | 已实现行为 | 边界/依赖 |
|---|---|---|---|---|
| MMW-PROBE-001 | Nezha v1 | `/probe` | 配置地址/凭据，同步服务器/流量 | 远端 API 版本依赖 |
| MMW-PROBE-002 | Nezha v0 | `/probe` | HTTP/WS 兼容同步与逐服务器流量 | 遗留接口差异多 |
| MMW-PROBE-003 | DStatus | `/probe` | 服务器/流量归一化 | 外部面板可达性 |
| MMW-PROBE-004 | Komari | `/probe` | 批量 API 同步和汇总 | 外部面板可达性 |
| MMW-PROBE-005 | 节点/文件绑定 | `/probe`、节点/文件 | 选 server IDs 作为流量口径 | ID 数组非正规化 |
| MMW-SPEED-001 | 本地 Mihomo 测速 | 节点测速 | 下载/缓存内核、临时配置、延迟/下载/出口 IP | 与目标 sing-box 内核不一致 |
| MMW-SPEED-002 | 远程 tester | 测速管理 | 创建/吊销/轮换 token，WS 在线与任务派发 | tester 实现不在本仓库 |
| MMW-SPEED-003 | 测速历史 | 对话框 | 每节点最近结果表/图 | 默认最多读取 100 条 |
| MMW-NOTIFY-001 | Telegram 通知 | 系统设置 | Bot token/chat id、测试、全局开关 | 依赖 Telegram API |
| MMW-NOTIFY-002 | 事件通知 | 系统设置 | 订阅获取、登录、IP 封禁、静默、到期、每日流量 | 每事件可开关 |

### 2.9 安全、隐身与运维

| ID | 功能 | 入口 | 已实现行为 | 边界/依赖 |
|---|---|---|---|---|
| MMW-SEC-001 | 静默模式 | 系统设置 | 默认伪装 404；启动或订阅激活后按时长开放管理面 | 单实例内存窗口 |
| MMW-SEC-002 | 短链暴力防护 | 根路径/日志 | 失败计数、临时/永久 ban、SQLite 恢复 | 代理头信任需正确配置 |
| MMW-SEC-003 | 手动 IP 封禁 | `/logs` | IPv4/IPv6 临时/永久封禁和解封 | 对已建立连接作用有限 |
| MMW-SEC-004 | 未知订阅 UA 阻断 | 系统设置 | 可拒绝不识别客户端 | 自定义客户端可能误伤 |
| MMW-SEC-005 | 本地 IP 例外 | 系统设置 | 限速/防护可跳过本地地址 | 反代来源判断很关键 |
| MMW-SEC-006 | SSRF 防护 | URL 抓取 | 限 http/https，拒私网/保留地址并防 DNS rebinding | 会阻止合法内网订阅/模板 |
| MMW-SEC-007 | 安全事件日志 | `/logs` | 事件、IP、路径/详情、时间 | 保留期 90 天 |
| MMW-OPS-001 | 管理操作审计 | `/logs` | 记录 `/api/admin` 变更方法、路径、状态、actor、IP | 不记录 GET/普通用户写操作 |
| MMW-OPS-002 | 后台任务日志 | `/logs` | 类型、状态、耗时、详情，按任务筛选 | 成功记录可能节流 |
| MMW-OPS-003 | 临时 debug 日志 | 用户菜单 | 开启/自动关闭、tail、下载 | 文件在本机，用户级设置+timer |
| MMW-OPS-004 | 数据库 WAL 维护 | 后台 | 定时 checkpoint，繁忙时降级 PASSIVE | SQLite 单连接 |
| MMW-OPS-005 | 日志保留清理 | 后台 | 文件 7 天；安全/操作 90 天；任务 30 天 | 固定策略为主 |
| MMW-OPS-006 | 备份下载 | 用户菜单/系统 | checkpoint 后打包 DB、订阅、规则/模板 | 格式缺少正式 manifest 协议 |
| MMW-OPS-007 | 备份恢复 | 初始化/登录后 | 上传、校验、覆盖数据 | 运行态恢复原子性有限 |
| MMW-OPS-008 | 版本检查 | 用户菜单 | 浏览器/后端查询 GitHub Releases | GitHub 可达性与限流 |
| MMW-OPS-009 | 应用内更新 | 用户菜单 | 备份、下载、替换二进制，JSON/SSE 进度 | 容器中写 `data/server` 覆盖镜像 |

### 2.10 界面与部署

| ID | 功能 | 已实现行为 | 边界/依赖 |
|---|---|---|---|
| MMW-UI-001 | 明暗主题 | 跟随/切换主题并持久化 | 上游采用像素/猫咪主题，不是标准 SaaS |
| MMW-UI-002 | 字体切换 | OPlus Sans、JetBrains Mono、系统字体 | 资源需随前端提供 |
| MMW-UI-003 | 响应式导航 | 桌面自动收缩文字，移动下拉菜单 | README 明示移动端“不完全适配” |
| MMW-UI-004 | 桌面/移动节点编辑 | 两套节点编辑对话框 | 字段容易漂移 |
| MMW-DEPLOY-001 | 单二进制 | SPA、Go 和默认模板嵌入一个二进制 | 构建必须先生成前端 dist |
| MMW-DEPLOY-002 | Docker/Compose | amd64/arm64 镜像、持久卷、PUID/PGID 降权 | 镜像可被 data/server 覆盖 |
| MMW-DEPLOY-003 | systemd 安装器 | Debian/Ubuntu 下载、安装、更新、卸载 | root 服务、无 checksum 验证 |
| MMW-DEPLOY-004 | 便携 nohup 安装器 | 当前目录运行和简单更新 | 无服务监管/健康检查 |
| MMW-DEPLOY-005 | Windows 二进制 | CI 交叉构建 amd64 | 运行时文件/更新路径需人工管理 |

## 3. 核心使用路径

### 3.1 最小可用路径

1. 部署并创建首个管理员；
2. 在节点管理粘贴 URI 或订阅 URL，预览并保存；
3. 在模板 V3 上传/选择模板，或直接使用已有订阅配置；
4. 在生成器选择节点、规则和代理组，预览后保存为订阅；
5. 创建普通用户并授权订阅文件；
6. 用户在订阅页复制带长期 token 的 URL给客户端。

探针、外部自动同步、provider、Telegram、静默模式、脚本覆写、远程测速都不是最小路径的前置条件。

### 3.2 外部订阅 + provider 路径

1. 在订阅管理添加外部订阅并配置自动更新/过滤；
2. 预览节点，选择同步为本地节点，或创建 proxy-provider 配置；
3. 配置 provider include/exclude/GeoIP/覆写与缓存间隔；
4. 在模板代理组中引用 provider；
5. 对外发布的配置携带 provider URL，客户端再取 provider YAML。

这条路径包含两个外部 GET：主订阅和 provider。任一长期 token 泄露都需要轮换；provider 缓存和外部订阅更新时间会影响一致性。

## 4. 明确未实现或不完整

- 社区版没有 Xray/sing-box 服务端入站管理、用户级内核客户端动态下发或节点服务器 Agent 编排。
- `/subscribe-files/custom` 明确显示功能开发中，不应计为自定义代理组完成品。
- 远程测速 tester 与 X Agent 主程序不在该仓库，只有主控侧协议与存储。
- 没有组织/租户、多管理员细粒度 RBAC、OAuth/OIDC/LDAP、API scope 或审计导出。
- 没有 PostgreSQL/MySQL、高可用、任务租约或共享缓存；运行模型是 SQLite + 本地文件 + 单实例内存。
- 没有正式 OpenAPI/生成 SDK；错误 envelope、分页和 body 限制不统一。
- 没有 sing-box 内核生命周期、配置热重载、入站用户流量/设备/限速管理。
- CI 名为 lint-and-test，但未运行 Go tests，前端 lint/format/knip 也不阻断；仓库当前有两个稳定失败的节点默认值测试。

## 5. 证据等级

| 等级 | 含义 | 本文使用方式 |
|---|---|---|
| S | 源码 + 路由/数据模型确认 | 绝大多数功能；可追到 Handler/页面/表。 |
| B | VPS 构建/测试确认 | 前端构建、Go 编译、schema 迁移。 |
| R | README 宣称，源码存在对应实现 | 30 天流量、支持探针等。 |
| U | 仅 UI 占位或外部组件缺失 | 自定义代理组、远程 tester 完整实现等，明确不计完成。 |

后续 X 差异矩阵会继续沿用这些 `MMW-*` ID，避免“名字相似就算同功能”。
