# WP-01 基础纵切：配置、双数据库与公开系统投影

## 1. 本次交付边界

本纵切把 P5.0 的静态骨架推进到真实持久层：Master 在监听端口前加载 typed config、解析数据库 URL、建立连接并执行版本化 migration；SQLite 和 PostgreSQL 18 跑同一份实例 repository contract；readiness 与 bootstrap 不再返回硬编码成功。HTTP 错误开始使用稳定 Problem Details；Vue 增加只读初始化页和数据库依赖状态。

这是 pre-public private baseline, intentionally unpublished（发布前私有基线，按设计不公开）时的边界。随后 [WP-01 存储与密钥纵切](./WP01_STORAGE_SECRET_SLICE.md) 已增加 typed setting repository、filesystem object adapter 和 AEAD secret canary；Owner 原子 bootstrap、secret/content metadata repository、S3 与资产 API仍未实现。因此需求追踪矩阵仍保持 358 项 `planned`，没有把“表已创建”或基础端口误标成对应产品能力完成。

## 2. 依赖与固定版本

| 组件 | 版本/制品 | 用途 |
|---|---|---|
| SQLx | `0.9.0` | Tokio runtime、SQLite、PostgreSQL、migration macro、UUID |
| config | `0.15.25` | defaults、TOML、`NODECONTROLL__...` override 和 typed deserialize |
| secrecy | `0.10.3` | 数据库 URL 内存包装与显式暴露 |
| async-trait | `0.1.92` | API 的 object-safe foundation probe port |
| PostgreSQL | official `18.6-bookworm@sha256:1c59e2...d7e1af` | 每轮 VPS 双库 contract；不使用宿主数据库 |

版本来自 2026-08-25 crates.io 官方元数据与 Docker Official Image manifest。所有 Cargo 依赖继续精确锁定，`Cargo.lock` 由 VPS Rust builder 生成。

## 3. `nodecontroll-config`

### 3.1 数据结构

| 类型/函数 | 责任 | 安全或校验边界 |
|---|---|---|
| `MasterConfig` | 聚合 HTTP 与数据库启动配置 | `deny_unknown_fields`；未知 section/key 启动失败 |
| `HttpConfig` | `SocketAddr` 监听地址 | 默认 `127.0.0.1:8080`，避免默认暴露公网 |
| `DatabaseConfig` | DSN、pool 和 acquire/statement/lock timeout | URL 是 `SecretString`；连接数和 timeout 必须大于 0 |
| `DatabaseConfig::url` | 只在连接适配器边界显式取得 DSN | 调用点可审计；日志不使用此返回值 |
| `DatabaseConfig::redacted_url` | 诊断/配置检查投影 | 永远返回 `[REDACTED]` |
| `load` | defaults → 可选 TOML → `NODECONTROLL__SECTION__KEY` | 配置文件显式传入才 required；deserialize/semantic error 不降级 |

`NODECONTROLL_CONFIG` 只选择 TOML 路径，不进入 typed setting tree。`--check-config` 在数据库连接前调用 `Database::validate_url`，只做结构/DSN 解析；VPS 用 read-only worktree 和一个不存在的 SQLite 路径验证它不会创建数据库或运行 migration。

### 3.2 测试

- 默认监听必须是 loopback，默认数据库必须是 SQLite；
- URL 的公开诊断值只能是 `[REDACTED]`；
- TOML 中加入 `http.unknown` 必须返回 `ConfigError::Load`，不能悄悄忽略拼写错误。

## 4. `nodecontroll-domain` 增量

| 类型/函数 | 责任 |
|---|---|
| `EntityId::from_uuid` / `Display` | 数据库 UUID 与 domain newtype 的显式双向边界 |
| `Revision::from_value` | 从校验后的非负数据库整数恢复 revision |
| `InstanceName::parse` | trim；拒绝空、超过 80 个 Unicode scalar、control character |
| `Instance` | `id/public_id/name/created_at_ms/revision` 的基础聚合投影 |

SQLite row 使用 canonical UUID text，PostgreSQL row 使用原生 UUID；转换只发生在 persistence adapter。API DTO 或 SQL row 没有进入 domain crate。

## 5. `nodecontroll-persistence`

### 5.1 连接和 migration

| 函数 | SQLite | PostgreSQL |
|---|---|---|
| `Database::validate_url` | 解析 `SqliteConnectOptions` | 解析 `PgConnectOptions` |
| `Database::connect` | 强制 pool=1、foreign keys、WAL、NORMAL sync、busy timeout | bounded pool；每条连接用 `set_config` 设置 statement/lock timeout |
| `Database::migrate` | embedded `migrations/sqlite` | embedded `migrations/postgres` |
| `Database::probe` | 在当前 pool 执行 `SELECT 1` | 同左 |
| `Database::engine` | 稳定值 `sqlite` | 稳定值 `postgres` |

首个 migration 在两库创建：

- `instances`：singleton check、UUID/public UUID、name、非负时间/revision；
- `instance_settings`：复合主键、schema version、JSON、revision、更新时间；
- `secret_records`：purpose/key version/nonce/ciphertext/AAD hash/rotation/tombstone；
- `content_objects`：SHA-256、size、MIME、backend/storage key、ref count；
- `content_references`：owner/purpose 唯一引用与 object FK。

两套 SQL 按数据库原生类型分别书写：SQLite UUID/JSON 为受约束 text，PostgreSQL 使用 `uuid/jsonb/bytea`；共同字段都约束非负整数、长度、枚举和 FK。migration 是 forward-only schema contract；回滚由备份/恢复与版本兼容策略承担，不伪造不安全 down migration。

### 5.2 实例 repository

| 函数 | 作用与不变量 |
|---|---|
| `is_initialized` | 读取 `instances` 行数；只有 `>0` 才初始化 |
| `bootstrap_instance` | 拒绝负时间/越界 revision；事务插入；已有实例或并发 unique conflict 均失败 |
| `instance` | 两库 row 分别解码，之后统一进入 `decode_instance` 做 UUID/name/revision domain 校验 |
| `duration_millis_string` | checked `u128→u64` 后生成 PostgreSQL timeout 参数 |
| `is_unique_violation` | 使用 SQLx database error classification，不匹配供应商错误字符串 |

repository contract 的顺序是：空库 migration → `initialized=false` → 写一个 UUIDv7 fixture → `initialized=true` → 完整读取相等 → 第二次 bootstrap 被拒绝 → 原实例未被覆盖。

## 6. API 与 Master 启动

### 6.1 启动序列

`apps/master::main` 当前执行：telemetry → config path/load → 可选零写入 config check → DB connect → migration → bind listener → 构造 API state → serve/graceful shutdown。连接或 migration 失败时不会打开 HTTP 端口。

`DatabaseProbe` 是 Master adapter，实现 API 定义的 `FoundationProbe`；API crate 因此不依赖 SQLx/persistence。`database_ready` 和 `is_initialized` 把内部错误映射为稳定 `DATABASE_UNAVAILABLE`，不会向客户端返回 DSN、SQL 或 driver error。

### 6.2 HTTP 合同

| Endpoint | 当前行为 |
|---|---|
| `GET /healthz` | 只说明进程/HTTP handler 活着，不访问数据库 |
| `GET /readyz` | 每次真实 `SELECT 1`；成功 200，失败 503；返回依赖名/status/code |
| `GET /api/v1/bootstrap` | 从实例表读取 `initialized`；空库不暴露登录方式 |
| `GET /api/v1/system/version` | version/start time + API version + request ID |
| 未匹配路由 | 404、`application/problem+json`、稳定 `ROUTE_NOT_FOUND` 和 request ID |

`Problem` 已包含 type/title/status/code/detail/request_id/字段错误数组，type 使用不需要解析官方域名的 `urn:nodecontroll:problem:*`。完整业务错误映射、认证与 revision/idempotency middleware 仍属于 WP-02/03。

## 7. Vue 3 + Vuetify

- `/setup` 调用生成的 `getBootstrapState`，显示真实初始化状态；
- 页面明确只读，不在 Owner/user transaction 尚不存在时提供临时写按钮；
- `/system` 同时调用 `getSystemVersion` 与 `getReadiness`，15 秒刷新依赖状态；
- 所有 response type 和 operation function 都来自 Rust OpenAPI → Hey API SDK；本次 OpenAPI 为 3.1、4 paths、4 个唯一 operation IDs。

## 8. VPS 验证证据

历史统一 run：`/opt/nodecontroll/artifacts/test-runs/20260825T152835Z-p5`，开始 `2026-08-25T15:28:36Z`，完成 `15:29:11Z`，exit 0。该 run 来自发布前私有基线，不是公开 Actions artifact 验收。

- Rust：API 5、config 2、domain 4、persistence 3，共 14 个测试通过；fmt 和 Clippy `-D warnings` 通过；
- 双库：SQLite memory 与真实 PostgreSQL 18.6 都执行相同实例 contract；PG test 用时约 0.24 秒；
- 配置检查：read-only worktree 成功，输出 URL 为 `[REDACTED]`，目标 DB 文件确认不存在；
- API：OpenAPI 3.1、4 paths/4 operation IDs，runtime smoke 覆盖 health/readiness/bootstrap/version/OpenAPI/404 Problem，6 个 request ID 唯一；
- Web：typecheck、ESLint、Vitest 2/2、Vite 297 modules；主入口 gzip 112.07 KiB；
- 文档追踪：358 source/358 trace、断链 0；状态仍是 planned 358；
- run 清理：临时 Master、PostgreSQL 容器和专用 Docker network 都不存在；
- lock：Cargo SHA-256 `3aa625085c994d7954a0037f8b18c6ba25cd29bdc166d4eb837786d235c4d487`，pnpm SHA-256 `554d9932aa59b372164df94c5e3eed6d2bd1270c1f48e402d54a75a91ad1aef7`。

manifest 当时记录了内部 source revision、builder、PostgreSQL image 和两个 lock hash。该 source 属于 pre-public private baseline, intentionally unpublished，公开文档不再给出不可解析的私有 SHA；现有 run ID 只用于定位历史 VPS 日志，不能当作公开 commit 或正式发布 provenance。

## 9. 下一纵切

后续结果见 [WP-01 存储与密钥纵切](./WP01_STORAGE_SECRET_SLICE.md)。当前下一步是 bootstrap application transaction、settings API、secret/content metadata repository 与 `/instance` projection；之后进入 WP-02 身份、安全 middleware、Owner 创建和 session。
