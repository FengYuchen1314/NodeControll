# 需求追踪矩阵

> 本表把上游社区版、妙妙屋 X 文档和原 PRO/去授权要求逐项映射到 NodeControll。当前为设计基线；实现后必须填写实现路径、测试 run 和 verified 状态。来源名称和验收语义以链接的原目录为准，不能通过修改本表降低要求。

## 1. 覆盖统计

| 来源 | 验收项 | 来源文档 |
|---|---:|---|
| 妙妙屋社区版 | 128 | [功能目录](../02-upstream-features/FEATURE_CATALOG.md) |
| 妙妙屋 X | 213 | [X 功能目录](../03-mmwx-gap/X_FEATURE_CATALOG.md) |
| PRO + 去授权 | 17 | [PRO/去授权](../03-mmwx-gap/PRO_FEATURES.md) |
| **合计** | **358** | 每一项恰好一行 |

状态只允许：`planned`（设计已分配）、`implemented`（代码和最小测试存在）、`verified`（目标 VPS 验收 run 通过）、`deferred-blocked`（有外部阻断、证据和用户确认）。P4 完成时全部是 planned；这不代表功能已实现。

## 2. 机器检查规则

- `Target ID` 是永久 ID：`NC-<source-id>`；不得复用或删除，需求变化追加说明。
- `Primary WP` 仅表示主交付包；跨包依赖由设计列和 [实施计划](./IMPLEMENTATION_PLAN.md) 描述。
- `Implementation` 在编码时填仓库相对路径与关键 symbol；`Last VPS run` 填 test run ID/commit。
- coverage checker 验证 128 + 213 + 17 = 358、Target ID 唯一、WP/设计/测试非空、verified 有实现和 run。

## 3. 全量矩阵

| Source ID | Target ID | 能力 | Primary WP | 设计合同 | 计划测试 | Implementation | Last VPS run | 状态 |
|---|---|---|---|---|---|---|---|---|
| MMW-AUTH-001 | NC-MMW-AUTH-001 | 首次安装状态 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-002 | NC-MMW-AUTH-002 | 创建首个管理员 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-003 | NC-MMW-AUTH-003 | 由备份初始化 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-004 | NC-MMW-AUTH-004 | 密码登录 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-005 | NC-MMW-AUTH-005 | 登录限速 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-006 | NC-MMW-AUTH-006 | Cloudflare Turnstile | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-007 | NC-MMW-AUTH-007 | TOTP 两步验证 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-008 | NC-MMW-AUTH-008 | UI 会话重启恢复 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-009 | NC-MMW-AUTH-009 | 修改个人密码 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-010 | NC-MMW-AUTH-010 | 个人资料 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-011 | NC-MMW-AUTH-011 | 用户 CRUD | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-012 | NC-MMW-AUTH-012 | 用户备注 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-013 | NC-MMW-AUTH-013 | 用户订阅授权 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-014 | NC-MMW-AUTH-014 | 长期订阅 token | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-AUTH-015 | NC-MMW-AUTH-015 | 用户/文件短码 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-TRAFFIC-001 | NC-MMW-TRAFFIC-001 | 总流量摘要 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMW-TRAFFIC-002 | NC-MMW-TRAFFIC-002 | 30 天趋势 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMW-TRAFFIC-003 | NC-MMW-TRAFFIC-003 | 外部订阅流量 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMW-TRAFFIC-004 | NC-MMW-TRAFFIC-004 | 订阅文件独立流量 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMW-TRAFFIC-005 | NC-MMW-TRAFFIC-005 | 订阅响应头 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMW-TRAFFIC-006 | NC-MMW-TRAFFIC-006 | 信息节点 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMW-NODE-001 | NC-MMW-NODE-001 | 手工节点导入 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-002 | NC-MMW-NODE-002 | 订阅 URL 导入 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-003 | NC-MMW-NODE-003 | 节点 CRUD | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-004 | NC-MMW-NODE-004 | 批量创建/删除/清空 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-005 | NC-MMW-NODE-005 | 批量重命名 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-006 | NC-MMW-NODE-006 | 去重节点 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-007 | NC-MMW-NODE-007 | 标签 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-008 | NC-MMW-NODE-008 | 启停与排序 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-009 | NC-MMW-NODE-009 | 协议专有字段 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-010 | NC-MMW-NODE-010 | server 改写与恢复 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-011 | NC-MMW-NODE-011 | 链式代理 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-012 | NC-MMW-NODE-012 | 探针绑定 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-013 | NC-MMW-NODE-013 | TCPing | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-014 | NC-MMW-NODE-014 | 节点测速 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-015 | NC-MMW-NODE-015 | 临时订阅 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-016 | NC-MMW-NODE-016 | URI 复制 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-NODE-017 | NC-MMW-NODE-017 | YAML 自动同步 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMW-EXT-001 | NC-MMW-EXT-001 | 外部订阅 CRUD | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-EXT-002 | NC-MMW-EXT-002 | 手动同步全部/单条 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-EXT-003 | NC-MMW-EXT-003 | 自动更新 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-EXT-004 | NC-MMW-EXT-004 | 节点 include/exclude | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-EXT-005 | NC-MMW-EXT-005 | 节点选择确认 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-EXT-006 | NC-MMW-EXT-006 | 匹配策略 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-EXT-007 | NC-MMW-EXT-007 | 同步范围 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-EXT-008 | NC-MMW-EXT-008 | 保留本地名称 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-EXT-009 | NC-MMW-EXT-009 | 获取订阅时强制同步 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-EXT-010 | NC-MMW-EXT-010 | 流量/到期后缀 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-001 | NC-MMW-SUB-001 | 上传订阅文件 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-002 | NC-MMW-SUB-002 | 从 URL/配置导入 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-003 | NC-MMW-SUB-003 | 订阅文件 CRUD | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-004 | NC-MMW-SUB-004 | 正文编辑 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-005 | NC-MMW-SUB-005 | 聚合订阅 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-006 | NC-MMW-SUB-006 | 文件排序 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-007 | NC-MMW-SUB-007 | 用户可见订阅页 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-008 | NC-MMW-SUB-008 | 组合短链 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-009 | NC-MMW-SUB-009 | UA 自动格式 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-010 | NC-MMW-SUB-010 | Clash/Mihomo 输出 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-011 | NC-MMW-SUB-011 | Surge 输出 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-012 | NC-MMW-SUB-012 | Loon 输出 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-013 | NC-MMW-SUB-013 | JSON 输出 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-014 | NC-MMW-SUB-014 | 其他客户端 producers | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-015 | NC-MMW-SUB-015 | Snell 兼容过滤 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-016 | NC-MMW-SUB-016 | 无效凭据伪装内容 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-017 | NC-MMW-SUB-017 | 订阅频率限制 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-SUB-018 | NC-MMW-SUB-018 | 旧 subscription links | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-PP-001 | NC-MMW-PP-001 | Provider 配置 CRUD | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-PP-002 | NC-MMW-PP-002 | 名称/GeoIP 过滤 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-PP-003 | NC-MMW-PP-003 | 字段覆写 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-PP-004 | NC-MMW-PP-004 | 客户端/MMW 模式 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-PP-005 | NC-MMW-PP-005 | 对外 provider URL | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-PP-006 | NC-MMW-PP-006 | 手动刷新/预览 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-PP-007 | NC-MMW-PP-007 | 自动缓存调度 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-PP-008 | NC-MMW-PP-008 | 批量按地域/协议创建 | [WP-12](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-GEN-001 | NC-MMW-GEN-001 | 可视化节点选择 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-GEN-002 | NC-MMW-GEN-002 | 拖拽代理组 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-GEN-003 | NC-MMW-GEN-003 | 预定义规则分类 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-GEN-004 | NC-MMW-GEN-004 | 生成预览 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-GEN-005 | NC-MMW-GEN-005 | 保存为订阅 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-TPL-001 | NC-MMW-TPL-001 | V2 数据库模板 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-TPL-002 | NC-MMW-TPL-002 | V3 文件模板 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-TPL-003 | NC-MMW-TPL-003 | 模板所有权/公开性 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-TPL-004 | NC-MMW-TPL-004 | 默认模板 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-TPL-005 | NC-MMW-TPL-005 | 可视化代理组编辑 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-TPL-006 | NC-MMW-TPL-006 | V2→V3 转换 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-TPL-007 | NC-MMW-TPL-007 | 订阅分析建模板 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-TPL-008 | NC-MMW-TPL-008 | 带标签/节点预览 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-RULE-001 | NC-MMW-RULE-001 | 规则文件编辑 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-RULE-002 | NC-MMW-RULE-002 | 自定义 DNS/rules/provider | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-RULE-003 | NC-MMW-RULE-003 | 追加/替换和去重 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-RULE-004 | NC-MMW-RULE-004 | JavaScript 覆写 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-RULE-005 | NC-MMW-RULE-005 | 内置规则/脚本模板 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-RULE-006 | NC-MMW-RULE-006 | Clash 配置校验 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMW-PROBE-001 | NC-MMW-PROBE-001 | Nezha v1 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMW-PROBE-002 | NC-MMW-PROBE-002 | Nezha v0 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMW-PROBE-003 | NC-MMW-PROBE-003 | DStatus | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMW-PROBE-004 | NC-MMW-PROBE-004 | Komari | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMW-PROBE-005 | NC-MMW-PROBE-005 | 节点/文件绑定 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMW-SPEED-001 | NC-MMW-SPEED-001 | 本地 Mihomo 测速 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMW-SPEED-002 | NC-MMW-SPEED-002 | 远程 tester | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMW-SPEED-003 | NC-MMW-SPEED-003 | 测速历史 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMW-NOTIFY-001 | NC-MMW-NOTIFY-001 | Telegram 通知 | [WP-16](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) / [DATA](./DATA_MODEL.md) | E2E-009 + TEST §10/11 | — | — | planned |
| MMW-NOTIFY-002 | NC-MMW-NOTIFY-002 | 事件通知 | [WP-16](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) / [DATA](./DATA_MODEL.md) | E2E-009 + TEST §10/11 | — | — | planned |
| MMW-SEC-001 | NC-MMW-SEC-001 | 静默模式 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-SEC-002 | NC-MMW-SEC-002 | 短链暴力防护 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-SEC-003 | NC-MMW-SEC-003 | 手动 IP 封禁 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-SEC-004 | NC-MMW-SEC-004 | 未知订阅 UA 阻断 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-SEC-005 | NC-MMW-SEC-005 | 本地 IP 例外 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-SEC-006 | NC-MMW-SEC-006 | SSRF 防护 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-SEC-007 | NC-MMW-SEC-007 | 安全事件日志 | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMW-OPS-001 | NC-MMW-OPS-001 | 管理操作审计 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-OPS-002 | NC-MMW-OPS-002 | 后台任务日志 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-OPS-003 | NC-MMW-OPS-003 | 临时 debug 日志 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-OPS-004 | NC-MMW-OPS-004 | 数据库 WAL 维护 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-OPS-005 | NC-MMW-OPS-005 | 日志保留清理 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-OPS-006 | NC-MMW-OPS-006 | 备份下载 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-OPS-007 | NC-MMW-OPS-007 | 备份恢复 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-OPS-008 | NC-MMW-OPS-008 | 版本检查 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-OPS-009 | NC-MMW-OPS-009 | 应用内更新 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-UI-001 | NC-MMW-UI-001 | 明暗主题 | [WP-04](./IMPLEMENTATION_PLAN.md) | [UX](./FRONTEND_UX.md) / [API](./API_CONTRACT.md) | TEST §9 + E2E-015 | — | — | planned |
| MMW-UI-002 | NC-MMW-UI-002 | 字体切换 | [WP-04](./IMPLEMENTATION_PLAN.md) | [UX](./FRONTEND_UX.md) / [API](./API_CONTRACT.md) | TEST §9 + E2E-015 | — | — | planned |
| MMW-UI-003 | NC-MMW-UI-003 | 响应式导航 | [WP-04](./IMPLEMENTATION_PLAN.md) | [UX](./FRONTEND_UX.md) / [API](./API_CONTRACT.md) | TEST §9 + E2E-015 | — | — | planned |
| MMW-UI-004 | NC-MMW-UI-004 | 桌面/移动节点编辑 | [WP-04](./IMPLEMENTATION_PLAN.md) | [UX](./FRONTEND_UX.md) / [API](./API_CONTRACT.md) | TEST §9 + E2E-015 | — | — | planned |
| MMW-DEPLOY-001 | NC-MMW-DEPLOY-001 | 单二进制 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-DEPLOY-002 | NC-MMW-DEPLOY-002 | Docker/Compose | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-DEPLOY-003 | NC-MMW-DEPLOY-003 | systemd 安装器 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-DEPLOY-004 | NC-MMW-DEPLOY-004 | 便携 nohup 安装器 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMW-DEPLOY-005 | NC-MMW-DEPLOY-005 | Windows 二进制 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-001 | NC-MMWX-PLAT-001 | Master-Agent 架构 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-002 | NC-MMWX-PLAT-002 | 首次安装链路 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-003 | NC-MMWX-PLAT-003 | 直接安装 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-004 | NC-MMWX-PLAT-004 | Docker 安装 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-005 | NC-MMWX-PLAT-005 | SQLite 主控 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-006 | NC-MMWX-PLAT-006 | PostgreSQL 18 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-007 | NC-MMWX-PLAT-007 | amd64/arm64 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-008 | NC-MMWX-PLAT-008 | 主/订阅域名 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-009 | NC-MMWX-PLAT-009 | HTTPS 反代 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-010 | NC-MMWX-PLAT-010 | Cloudflare Tunnel | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-011 | NC-MMWX-PLAT-011 | 在线更新 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-012 | NC-MMWX-PLAT-012 | SQLite ZIP 备份 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-013 | NC-MMWX-PLAT-013 | PostgreSQL 独立备份 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-014 | NC-MMWX-PLAT-014 | SQLite 启动修复 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-015 | NC-MMWX-PLAT-015 | 从妙妙屋迁移 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-016 | NC-MMWX-PLAT-016 | 空库迁移前置 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-017 | NC-MMWX-PLAT-017 | 智能认领 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-018 | NC-MMWX-PLAT-018 | 迁移回滚 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-019 | NC-MMWX-PLAT-019 | 更新日志 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-PLAT-020 | NC-MMWX-PLAT-020 | 无许可证自托管 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| MMWX-AGENT-001 | NC-MMWX-AGENT-001 | 服务器登记 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-002 | NC-MMWX-AGENT-002 | 一服务器一 token | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-003 | NC-MMWX-AGENT-003 | WebSocket 模式 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-004 | NC-MMWX-AGENT-004 | HTTP 模式 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-005 | NC-MMWX-AGENT-005 | Pull 模式 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-006 | NC-MMWX-AGENT-006 | Auto 模式 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-007 | NC-MMWX-AGENT-007 | systemd 部署 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-008 | NC-MMWX-AGENT-008 | OpenRC 部署 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-009 | NC-MMWX-AGENT-009 | 无 init 回退 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-010 | NC-MMWX-AGENT-010 | 配置文件/环境变量 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-011 | NC-MMWX-AGENT-011 | 在线状态 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-012 | NC-MMWX-AGENT-012 | 系统指标 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-013 | NC-MMWX-AGENT-013 | 实时网络 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-014 | NC-MMWX-AGENT-014 | 批量升级 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-015 | NC-MMWX-AGENT-015 | token 轮换 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-016 | NC-MMWX-AGENT-016 | 主控热重连 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-017 | NC-MMWX-AGENT-017 | 任务执行边界 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-AGENT-018 | NC-MMWX-AGENT-018 | 嵌入/外置内核模式 | [WP-05](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [ARCH](./ARCHITECTURE.md) / [SEC](./SECURITY.md) | E2E-002/013 + TEST §5 | — | — | planned |
| MMWX-CORE-001 | NC-MMWX-CORE-001 | 系统网卡计数 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-002 | NC-MMWX-CORE-002 | 内核协议计数 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-003 | NC-MMWX-CORE-003 | 服务器数据源选择 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-004 | NC-MMWX-CORE-004 | 本次开机流量 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-005 | NC-MMWX-CORE-005 | 域名延迟 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-006 | NC-MMWX-CORE-006 | 服务扫描 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-007 | NC-MMWX-CORE-007 | 安装内核 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-008 | NC-MMWX-CORE-008 | 卸载内核 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-009 | NC-MMWX-CORE-009 | 启动/停止/重启 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-010 | NC-MMWX-CORE-010 | 配置路径发现 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-011 | NC-MMWX-CORE-011 | 完整配置查看 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-012 | NC-MMWX-CORE-012 | 配置编辑 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-013 | NC-MMWX-CORE-013 | 配置生效 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-014 | NC-MMWX-CORE-014 | 运行配置过滤 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-015 | NC-MMWX-CORE-015 | 内核版本状态 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-CORE-016 | NC-MMWX-CORE-016 | 变更串行化 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| MMWX-IN-001 | NC-MMWX-IN-001 | 可视化入站向导 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-002 | NC-MMWX-IN-002 | 端口冲突检测 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-003 | NC-MMWX-IN-003 | 凭据生成 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-004 | NC-MMWX-IN-004 | 入站↔节点同步 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-005 | NC-MMWX-IN-005 | VLESS TCP REALITY | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-006 | NC-MMWX-IN-006 | VLESS TCP REALITY Vision | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-007 | NC-MMWX-IN-007 | VLESS TCP TLS | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-008 | NC-MMWX-IN-008 | VLESS TCP TLS Vision | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-009 | NC-MMWX-IN-009 | VLESS WebSocket TLS | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-010 | NC-MMWX-IN-010 | VLESS gRPC REALITY | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-011 | NC-MMWX-IN-011 | VLESS XHTTP REALITY | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-012 | NC-MMWX-IN-012 | Trojan TCP TLS | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-013 | NC-MMWX-IN-013 | Trojan TCP REALITY | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-014 | NC-MMWX-IN-014 | Trojan gRPC REALITY | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-015 | NC-MMWX-IN-015 | VMess TCP 无安全层 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-016 | NC-MMWX-IN-016 | VMess TCP TLS | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-017 | NC-MMWX-IN-017 | VMess WS 无安全层 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-018 | NC-MMWX-IN-018 | VMess WS TLS | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-019 | NC-MMWX-IN-019 | Shadowsocks AEAD | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-020 | NC-MMWX-IN-020 | Shadowsocks 2022 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-021 | NC-MMWX-IN-021 | Hysteria2 UDP TLS | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-022 | NC-MMWX-IN-022 | AnyTLS TCP TLS | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-023 | NC-MMWX-IN-023 | AnyTLS TCP REALITY | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-024 | NC-MMWX-IN-024 | Snell v4/v5 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-025 | NC-MMWX-IN-025 | Snell v6 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-026 | NC-MMWX-IN-026 | 废弃 H2/Trojan flow | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-IN-027 | NC-MMWX-IN-027 | 客户端格式转换 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-NODE-001 | NC-MMWX-NODE-001 | 自动节点 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-NODE-002 | NC-MMWX-NODE-002 | 外部节点 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-NODE-003 | NC-MMWX-NODE-003 | 多标签 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-NODE-004 | NC-MMWX-NODE-004 | 排序 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-NODE-005 | NC-MMWX-NODE-005 | 全用户 URI 视图 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-NODE-006 | NC-MMWX-NODE-006 | 地址切换/恢复 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-NODE-007 | NC-MMWX-NODE-007 | 端口转发复用 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-NODE-008 | NC-MMWX-NODE-008 | Tunnel 入站 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-NODE-009 | NC-MMWX-NODE-009 | Tunnel 清理 | [WP-08](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-003 + TEST §6.2 | — | — | planned |
| MMWX-OUT-001 | NC-MMWX-OUT-001 | Direct/Block | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-OUT-002 | NC-MMWX-OUT-002 | 代理出站 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-OUT-003 | NC-MMWX-OUT-003 | Tunnel 出站 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-OUT-004 | NC-MMWX-OUT-004 | WARP 注册 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-OUT-005 | NC-MMWX-OUT-005 | WARP v4/v6 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-OUT-006 | NC-MMWX-OUT-006 | WARP+ | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-OUT-007 | NC-MMWX-OUT-007 | WARP 刷新 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-OUT-008 | NC-MMWX-OUT-008 | WARP 卸载 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-001 | NC-MMWX-ROUTE-001 | first-match | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-002 | NC-MMWX-ROUTE-002 | 条件 AND | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-003 | NC-MMWX-ROUTE-003 | 节点专属/全局 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-004 | NC-MMWX-ROUTE-004 | Catch-all 检测 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-005 | NC-MMWX-ROUTE-005 | 路由拖排 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-006 | NC-MMWX-ROUTE-006 | 域名/IP/协议/端口规则 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-007 | NC-MMWX-ROUTE-007 | 来源/网络/用户/属性 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-008 | NC-MMWX-ROUTE-008 | 快捷规则 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-009 | NC-MMWX-ROUTE-009 | 防送中 WARP 规则 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-010 | NC-MMWX-ROUTE-010 | Balancer random | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-011 | NC-MMWX-ROUTE-011 | Balancer round-robin | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-012 | NC-MMWX-ROUTE-012 | Balancer least-ping | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-013 | NC-MMWX-ROUTE-013 | Balancer least-load | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-014 | NC-MMWX-ROUTE-014 | 节点级路由出站 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-015 | NC-MMWX-ROUTE-015 | 用户级私有路由出站 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-016 | NC-MMWX-ROUTE-016 | 用户路由配额 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-ROUTE-017 | NC-MMWX-ROUTE-017 | 自动暂停/恢复 | [WP-09](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [API](./API_CONTRACT.md) / [DATA](./DATA_MODEL.md) | E2E-004 + TEST §6.3 | — | — | planned |
| MMWX-USER-001 | NC-MMWX-USER-001 | admin/user 角色 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-USER-002 | NC-MMWX-USER-002 | 用户 CRUD/状态 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-USER-003 | NC-MMWX-USER-003 | 密码/JWT | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-USER-004 | NC-MMWX-USER-004 | 订阅 token | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-USER-005 | NC-MMWX-USER-005 | 多套餐 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-USER-006 | NC-MMWX-USER-006 | 多用户同端口 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-001 | NC-MMWX-PKG-001 | 套餐模板/实例 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-002 | NC-MMWX-PKG-002 | 节点/标签选择 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-003 | NC-MMWX-PKG-003 | 流量限额 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-004 | NC-MMWX-PKG-004 | 到期时间 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-005 | NC-MMWX-PKG-005 | 独立凭据 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-006 | NC-MMWX-PKG-006 | 节点计费倍率 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-007 | NC-MMWX-PKG-007 | 单/双向计费 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-008 | NC-MMWX-PKG-008 | 套餐默认速度 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-009 | NC-MMWX-PKG-009 | 套餐逐节点速度 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-010 | NC-MMWX-PKG-010 | 用户全局覆盖 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-011 | NC-MMWX-PKG-011 | 用户逐节点覆盖 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-012 | NC-MMWX-PKG-012 | 限速优先级 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-013 | NC-MMWX-PKG-013 | 并发连接上限 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-014 | NC-MMWX-PKG-014 | 规则推送 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-015 | NC-MMWX-PKG-015 | 自动限速 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-016 | NC-MMWX-PKG-016 | 自动解除 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-PKG-017 | NC-MMWX-PKG-017 | 自动停用 | [WP-10](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [DATA](./DATA_MODEL.md) / [UX](./FRONTEND_UX.md) | E2E-005/015 + TEST §4.1 | — | — | planned |
| MMWX-TRAFFIC-001 | NC-MMWX-TRAFFIC-001 | 入站/出站/用户三维 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMWX-TRAFFIC-002 | NC-MMWX-TRAFFIC-002 | 原始与计费流量 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMWX-TRAFFIC-003 | NC-MMWX-TRAFFIC-003 | 每日账本 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMWX-TRAFFIC-004 | NC-MMWX-TRAFFIC-004 | 重置 baseline | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMWX-TRAFFIC-005 | NC-MMWX-TRAFFIC-005 | 手工调整 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMWX-TRAFFIC-006 | NC-MMWX-TRAFFIC-006 | 路由出站归属 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMWX-TRAFFIC-007 | NC-MMWX-TRAFFIC-007 | 外部订阅口径 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| MMWX-SUB-001 | NC-MMWX-SUB-001 | 订阅文件管理 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-002 | NC-MMWX-SUB-002 | 12+ 客户端格式 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-003 | NC-MMWX-SUB-003 | UA/显式格式选择 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-004 | NC-MMWX-SUB-004 | 生成流水线 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-005 | NC-MMWX-SUB-005 | 模板叠加 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-006 | NC-MMWX-SUB-006 | V3 模板 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-007 | NC-MMWX-SUB-007 | include-all-proxies | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-008 | NC-MMWX-SUB-008 | include-all-providers | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-009 | NC-MMWX-SUB-009 | proxy/providers/both | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-010 | NC-MMWX-SUB-010 | select/url-test/fallback/load-balance | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-011 | NC-MMWX-SUB-011 | relay/dialer 链 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-012 | NC-MMWX-SUB-012 | 隐藏/图标等展示属性 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-013 | NC-MMWX-SUB-013 | 自定义分流规则 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-014 | NC-MMWX-SUB-014 | 外部订阅/provider | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SUB-015 | NC-MMWX-SUB-015 | 代理组配置同步 | [WP-13](./IMPLEMENTATION_PLAN.md) | [IR](./SUBSCRIPTION_IR.md) / [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) | E2E-006 + TEST §8 | — | — | planned |
| MMWX-SPEED-001 | NC-MMWX-SPEED-001 | 主控本地测速 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-SPEED-002 | NC-MMWX-SPEED-002 | 远程家用测速端 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-SPEED-003 | NC-MMWX-SPEED-003 | 单/8 线程吞吐 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-SPEED-004 | NC-MMWX-SPEED-004 | 真连接延迟 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-SPEED-005 | NC-MMWX-SPEED-005 | 下载测速 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-SPEED-006 | NC-MMWX-SPEED-006 | 出口 IP | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-SPEED-007 | NC-MMWX-SPEED-007 | 批量异步/历史 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-CERT-001 | NC-MMWX-CERT-001 | ACME DNS-01 | [WP-14](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-008 + TEST §11 | — | — | planned |
| MMWX-CERT-002 | NC-MMWX-CERT-002 | DNS 提供商 | [WP-14](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-008 + TEST §11 | — | — | planned |
| MMWX-CERT-003 | NC-MMWX-CERT-003 | SAN/通配符 | [WP-14](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-008 + TEST §11 | — | — | planned |
| MMWX-CERT-004 | NC-MMWX-CERT-004 | 自动续期 | [WP-14](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-008 + TEST §11 | — | — | planned |
| MMWX-CERT-005 | NC-MMWX-CERT-005 | 自动部署 Agent | [WP-14](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-008 + TEST §11 | — | — | planned |
| MMWX-CERT-006 | NC-MMWX-CERT-006 | PEM 下载 | [WP-14](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-008 + TEST §11 | — | — | planned |
| MMWX-CERT-007 | NC-MMWX-CERT-007 | Webhook/Certimate | [WP-14](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-008 + TEST §11 | — | — | planned |
| MMWX-SITE-001 | NC-MMWX-SITE-001 | Nginx 探测 | [WP-14](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-008 + TEST §11 | — | — | planned |
| MMWX-SITE-002 | NC-MMWX-SITE-002 | 静态网站 | [WP-14](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-008 + TEST §11 | — | — | planned |
| MMWX-SITE-003 | NC-MMWX-SITE-003 | 反向代理 | [WP-14](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-008 + TEST §11 | — | — | planned |
| MMWX-SITE-004 | NC-MMWX-SITE-004 | 端口检查 | [WP-14](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-008 + TEST §11 | — | — | planned |
| MMWX-SITE-005 | NC-MMWX-SITE-005 | 安全删除 | [WP-14](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-008 + TEST §11 | — | — | planned |
| MMWX-SET-001 | NC-MMWX-SET-001 | 外订阅同步策略 | [WP-01](./IMPLEMENTATION_PLAN.md) | [ARCH](./ARCHITECTURE.md) / [DATA](./DATA_MODEL.md) / [API](./API_CONTRACT.md) | TEST §4.2/4.3 | — | — | planned |
| MMWX-SET-002 | NC-MMWX-SET-002 | 静默模式 | [WP-01](./IMPLEMENTATION_PLAN.md) | [ARCH](./ARCHITECTURE.md) / [DATA](./DATA_MODEL.md) / [API](./API_CONTRACT.md) | TEST §4.2/4.3 | — | — | planned |
| MMWX-SET-003 | NC-MMWX-SET-003 | 短链接 | [WP-01](./IMPLEMENTATION_PLAN.md) | [ARCH](./ARCHITECTURE.md) / [DATA](./DATA_MODEL.md) / [API](./API_CONTRACT.md) | TEST §4.2/4.3 | — | — | planned |
| MMWX-SET-004 | NC-MMWX-SET-004 | 客户端兼容模式 | [WP-01](./IMPLEMENTATION_PLAN.md) | [ARCH](./ARCHITECTURE.md) / [DATA](./DATA_MODEL.md) / [API](./API_CONTRACT.md) | TEST §4.2/4.3 | — | — | planned |
| MMWX-SET-005 | NC-MMWX-SET-005 | 覆写脚本 | [WP-01](./IMPLEMENTATION_PLAN.md) | [ARCH](./ARCHITECTURE.md) / [DATA](./DATA_MODEL.md) / [API](./API_CONTRACT.md) | TEST §4.2/4.3 | — | — | planned |
| MMWX-SET-006 | NC-MMWX-SET-006 | 模板版本/序列化 | [WP-01](./IMPLEMENTATION_PLAN.md) | [ARCH](./ARCHITECTURE.md) / [DATA](./DATA_MODEL.md) / [API](./API_CONTRACT.md) | TEST §4.2/4.3 | — | — | planned |
| MMWX-SET-007 | NC-MMWX-SET-007 | 订阅响应头/信息节点 | [WP-01](./IMPLEMENTATION_PLAN.md) | [ARCH](./ARCHITECTURE.md) / [DATA](./DATA_MODEL.md) / [API](./API_CONTRACT.md) | TEST §4.2/4.3 | — | — | planned |
| MMWX-SET-008 | NC-MMWX-SET-008 | 品牌/主题 | [WP-01](./IMPLEMENTATION_PLAN.md) | [ARCH](./ARCHITECTURE.md) / [DATA](./DATA_MODEL.md) / [API](./API_CONTRACT.md) | TEST §4.2/4.3 | — | — | planned |
| MMWX-NOTIFY-001 | NC-MMWX-NOTIFY-001 | Telegram 通知 | [WP-16](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) / [DATA](./DATA_MODEL.md) | E2E-009 + TEST §10/11 | — | — | planned |
| MMWX-PROBE-001 | NC-MMWX-PROBE-001 | 内置公开探针 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-PROBE-002 | NC-MMWX-PROBE-002 | 外置 Worker 探针 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-PROBE-003 | NC-MMWX-PROBE-003 | 状态快照 API | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-PROBE-004 | NC-MMWX-PROBE-004 | 实时 WS | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-PROBE-005 | NC-MMWX-PROBE-005 | 历史 series | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-PROBE-006 | NC-MMWX-PROBE-006 | 公开扩展字段 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| MMWX-SEC-001 | NC-MMWX-SEC-001 | Cloudflare Turnstile | [WP-02](./IMPLEMENTATION_PLAN.md) | [RUST](./RUST_BACKEND.md) / [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-001/015 + SEC auth | — | — | planned |
| MMWX-TG-001 | NC-MMWX-TG-001 | 内嵌 Telegram Bot | [WP-16](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) / [DATA](./DATA_MODEL.md) | E2E-009 + TEST §10/11 | — | — | planned |
| MMWX-TG-002 | NC-MMWX-TG-002 | 用户命令 | [WP-16](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) / [DATA](./DATA_MODEL.md) | E2E-009 + TEST §10/11 | — | — | planned |
| MMWX-TG-003 | NC-MMWX-TG-003 | 管理员命令 | [WP-16](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) / [DATA](./DATA_MODEL.md) | E2E-009 + TEST §10/11 | — | — | planned |
| MMWX-TG-004 | NC-MMWX-TG-004 | 邀请码 | [WP-16](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) / [DATA](./DATA_MODEL.md) | E2E-009 + TEST §10/11 | — | — | planned |
| MMWX-TG-005 | NC-MMWX-TG-005 | 每日通知 | [WP-16](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) / [DATA](./DATA_MODEL.md) | E2E-009 + TEST §10/11 | — | — | planned |
| MMWX-TG-006 | NC-MMWX-TG-006 | Mini App | [WP-16](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) / [DATA](./DATA_MODEL.md) | E2E-009 + TEST §10/11 | — | — | planned |
| MMWX-MCP-001 | NC-MMWX-MCP-001 | Streamable HTTP `/mcp` | [WP-17](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-009 + TEST §11 | — | — | planned |
| MMWX-MCP-002 | NC-MMWX-MCP-002 | Scoped API token | [WP-17](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-009 + TEST §11 | — | — | planned |
| MMWX-MCP-003 | NC-MMWX-MCP-003 | 26 个工具 | [WP-17](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-009 + TEST §11 | — | — | planned |
| MMWX-MCP-004 | NC-MMWX-MCP-004 | 高危确认 | [WP-17](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-009 + TEST §11 | — | — | planned |
| MMWX-MCP-005 | NC-MMWX-MCP-005 | 极端接口不暴露 | [WP-17](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [SEC](./SECURITY.md) | E2E-009 + TEST §11 | — | — | planned |
| MMWX-SHARE-001 | NC-MMWX-SHARE-001 | 分享 token | [WP-18](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-010 + TEST §5/11 | — | — | planned |
| MMWX-SHARE-002 | NC-MMWX-SHARE-002 | 消费方接入 | [WP-18](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-010 + TEST §5/11 | — | — | planned |
| MMWX-SHARE-003 | NC-MMWX-SHARE-003 | 最小权限 `/api/child` | [WP-18](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-010 + TEST §5/11 | — | — | planned |
| MMWX-SHARE-004 | NC-MMWX-SHARE-004 | 入站前缀 | [WP-18](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-010 + TEST §5/11 | — | — | planned |
| MMWX-SHARE-005 | NC-MMWX-SHARE-005 | 禁止服务控制/配置读取 | [WP-18](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-010 + TEST §5/11 | — | — | planned |
| MMWX-SHARE-006 | NC-MMWX-SHARE-006 | 禁止二次分享 | [WP-18](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-010 + TEST §5/11 | — | — | planned |
| MMWX-SHARE-007 | NC-MMWX-SHARE-007 | ECDH/HTTPS 传输 | [WP-18](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-010 + TEST §5/11 | — | — | planned |
| PRO-001 | NC-PRO-001 | Agent 内嵌代理内核 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| PRO-002 | NC-PRO-002 | 实时用户/节点限速 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| PRO-003 | NC-PRO-003 | XTLS Vision 限速钩子 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| PRO-004 | NC-PRO-004 | 自动限速/解除 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| PRO-005 | NC-PRO-005 | 在线用户、IP 与连接追踪 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| PRO-006 | NC-PRO-006 | 最大并发连接限制 | [WP-11](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [RUST](./RUST_BACKEND.md) / [OBS](./OBSERVABILITY.md) | E2E-005/013 + TEST §7 | — | — | planned |
| PRO-007 | NC-PRO-007 | 节点测速工作台 | [WP-15](./IMPLEMENTATION_PLAN.md) | [API](./API_CONTRACT.md) / [UX](./FRONTEND_UX.md) / [SEC](./SECURITY.md) | E2E-007 + TEST §10 | — | — | planned |
| PRO-008 | NC-PRO-008 | 分享服务器 | [WP-18](./IMPLEMENTATION_PLAN.md) | [AGENT](./AGENT_PROTOCOL.md) / [SEC](./SECURITY.md) / [API](./API_CONTRACT.md) | E2E-010 + TEST §5/11 | — | — | planned |
| PRO-009 | NC-PRO-009 | 自定义品牌 | [WP-04](./IMPLEMENTATION_PLAN.md) | [UX](./FRONTEND_UX.md) / [API](./API_CONTRACT.md) | TEST §9 + E2E-015 | — | — | planned |
| PRO-010 | NC-PRO-010 | 内嵌 Agent Docker 开关 | [WP-07](./IMPLEMENTATION_PLAN.md) | [SINGBOX](./SINGBOX_COMPATIBILITY.md) / [DEPLOY](./DEPLOYMENT.md) | E2E-002/013/014 + TEST §6 | — | — | planned |
| NOLIC-001 | NC-NOLIC-001 | `mmwxlicense.com` 或其他官方激活服务 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| NOLIC-002 | NC-NOLIC-002 | 机器 ID 申请、绑定或硬件指纹 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| NOLIC-003 | NC-NOLIC-003 | 签名 feature flags/许可证套餐 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| NOLIC-004 | NC-NOLIC-004 | 许可证给出的服务器、节点、用户额度 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| NOLIC-005 | NC-NOLIC-005 | 官方域名、订阅域名库或远程配置为必要条件 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| NOLIC-006 | NC-NOLIC-006 | 分享双方必须持证 | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |
| NOLIC-007 | NC-NOLIC-007 | License UI、到期降级、license badge | [WP-20](./IMPLEMENTATION_PLAN.md) | [DEPLOY](./DEPLOYMENT.md) / [MIGRATION](./MIGRATION.md) / [SEC](./SECURITY.md) | E2E-012/014 + TEST §13 | — | — | planned |

## 4. 变更与关闭规则

实现某项时先把状态改为 `implemented`，填实际模块/函数和最小 VPS run；阶段全量/真实互操作通过后才改 `verified`。若一个 source ID 包含多个可独立失败的行为，实施包可在代码测试中派生 `NC-.../a,b` 子验收，但父行只有全部子项通过才能 verified。

任何 sing-box 标准不支持、X 文档矛盾或 X 私有源码不可见的问题，必须保留源要求并在实现/测试列写兼容策略；不能删行或把“不支持”直接标 verified。原 PRO 行只有在普通用户、离线自托管、无授权路径真实通过后才能关闭；NOLIC 行还需仓库/制品静态扫描和断公网动态捕获证据。

