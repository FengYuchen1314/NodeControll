# WP-01 存储与密钥纵切

## 1. 交付结论

本纵切完成三个可独立验证的基础端口：`SubscriptionBehaviorSettings` 的 SQLite/PostgreSQL 乐观并发 repository；本地 filesystem 的 SHA-256 内容寻址对象 adapter；用户自有 root key 驱动的 XChaCha20-Poly1305 envelope cipher/canary。Master readiness 现在同时检查数据库和 secret store；不存在授权、官方域名、远程 key service 或联网初始化。

这仍不是产品功能完成：settings 尚无认证 API/UI；object adapter 尚未和 `content_objects` metadata transaction、quota、S3/GC/上传 API 相连；secret envelope 尚未写入 `secret_records`，也没有 keyring/rewrap/backup。对应 358 条产品需求继续保持 `planned`。

来源：pre-public private baseline, intentionally unpublished（发布前私有基线，按设计不公开）。历史 VPS run：`/opt/nodecontroll/artifacts/test-runs/20260825T154501Z-p5`；保留该 run ID 不表示存在可解析的公开 commit。

## 2. Typed subscription settings

### 2.1 Domain

| 类型 | 值/字段 | 语义 |
|---|---|---|
| `ExternalSyncStrategy` | `scheduled/on_request/manual` | 外部订阅何时刷新；不把 bool 混用成三态 |
| `ClientCompatibilityMode` | `strict/legacy` | 客户端兼容策略显式化 |
| `SubscriptionBehaviorSettings` | sync、silent、short link、compat、response header、info node | `deny_unknown_fields`、schema v1；脚本/模板引用刻意不塞进 JSON |

所有类型用固定 `snake_case` serialization；新增枚举值或字段需要 schema version/migration，不允许把未知输入静默转成默认值。

### 2.2 Persistence functions

| 函数 | 行为 |
|---|---|
| `subscription_settings(instance_id)` | 按固定 key `subscription.behavior` + schema 1 读取；PG 显式投影 `jsonb::text`；两库都 deserialize 成 domain type |
| `save_subscription_settings(..., None, actor_id, ...)` | 只允许首次 insert，revision=0，并记录修改人；unique conflict 映射 `RevisionConflict` |
| `save_subscription_settings(..., Some(expected), actor_id, ...)` | checked `Revision::next`，`UPDATE ... WHERE revision=expected`，同时记录修改人；affected rows 不是 1 即冲突 |

repository contract 在 SQLite 与真实 PG 18.6 中依次验证：不存在 → 创建 → 完整读取 → revision 0 更新到 1 → 用旧 revision 更新失败 → 已保存值不回滚。PG/SQLite 使用各自 placeholder/UUID/JSON 类型，业务结果相同。

## 3. Filesystem object store

### 3.1 公共合同

| 类型/函数 | 责任 |
|---|---|
| `StoredObject` | `sha256,size_bytes,storage_key` 不可变 metadata |
| `ObjectStore::put/get` | application 可替换端口；当前 adapter 为 filesystem |
| `FilesystemObjectStore::open` | 建根目录并 canonicalize；拒绝 0 byte size limit |
| `path_for_hash` | 只接受 64 字符 lowercase SHA-256；生成 `sha256/aa/bb/<hash>`，调用者不能提供路径 |
| `verify_existing` | 每次 get 及 dedupe 命中都复算 size/hash |
| `put` | 入站 size 先限额；hash；同目录 `create_new` temp；write/flush/fsync；atomic rename；目录 fsync；竞争命中再做完整校验 |
| `get` | storage key 必须由 hash 重新推导且完全相等；读取后校验 hash/size |
| `sync_directory` | `spawn_blocking` 执行目录 `fsync`，不阻塞 Tokio executor thread |

失败路径会尽力移除精确 temp file；不会递归删除 object root。相同内容重复 put 返回相同 metadata；磁盘内容被篡改返回 `IntegrityMismatch`，不会把坏数据继续交给调用者。

当前限制：API 仍接收内存 byte slice、get 整体读入内存；大对象 streaming/multipart 在资产/备份 WP 接入。S3 adapter、DB metadata/ref-count transaction、symlink/openat2 强化和垃圾回收尚未交付。

## 4. Envelope secret cipher

### 4.1 Key 与算法

- `SecretsConfig.root_key_file` 默认 `nodecontroll.key`，可由 TOML/`NODECONTROLL__SECRETS__ROOT_KEY_FILE` 指向用户自己的文件；
- 文件必须是 regular file、恰好 64 个 lowercase hex；Unix group/other 任一权限位存在即拒绝，推荐/测试 mode `0600`；
- 32-byte key 存放在 `Arc<Zeroizing<[u8;32]>>`，读取的编码字符串和解密 plaintext 也用 `Zeroizing`；
- 算法为 `XChaCha20Poly1305`，24-byte nonce 每次由 OS `getrandom` 取得，不从时间/UUID派生；
- AAD 是带版本头和两个 big-endian length prefix 的 purpose+owner，避免字符串拼接歧义；数据库保存其 SHA-256 便于 owner/purpose 错配先行诊断。

### 4.2 函数

| 函数 | 责任/失败语义 |
|---|---|
| `EnvelopeCipher::from_key_file` | key version>0、metadata/mode/hex 校验；不打印 key/path contents |
| `from_hex` | 测试/受控 provider 构造；拒绝 uppercase、短 key 和 version 0 |
| `encrypt` | 校验 binding、生成 nonce、AEAD encrypt；返回 version/nonce/ciphertext/aad_hash |
| `decrypt` | 先检查 key version 与 AAD hash，再 AEAD authenticate；owner/purpose/tamper 都失败 |
| `canary` | 固定非敏感 plaintext 做随机 nonce encrypt/decrypt round-trip，只返回成功/稳定错误 |
| `associated_data` | 空 binding/超过 u32 长度拒绝；格式为 `NCSECRET1\0 + len + purpose + len + owner` |

Master 在 bind 前加载 key 并执行一次 canary；`/readyz` 每次再验证 `secret_store`，与 `database` 分开报告。API 只得到 `SECRET_STORE_UNAVAILABLE`，不会收到 cipher/IO 细节。

本段记录的是 WP-01 当时的实现边界，现已由 [WP-02-C2 实现](./WP02_C2_SECRET_RECOVERY_IMPLEMENTATION.md) 接续：数据库已有 typed `secret_records` repository、持久化 root-key canary，以及当前 key 加最多 3 枚旧 key 的有限 keyring；启动时可用旧 key 解密并原子 rewrap canary。HSM/KMS/TPM provider 仍未实现。非 Unix 平台目前只检查 regular file，不宣称完成 ACL 验证。

## 5. VPS 验证

run `20260825T154501Z-p5`：开始 `2026-08-25T15:45:01Z`，完成 `15:45:36Z`，exit 0。manifest 的 source revision 对应发布前私有基线，按设计不公开；该记录是历史 VPS 预检，不是公开 Actions artifact 验收。

- Rust 21 tests：API 5、config 2、domain 4、object 3、persistence 3、secret 4；fmt/Clippy `-D warnings` 全绿；
- object：content-addressed/idempotent/round-trip、pre-write size limit、on-disk corruption 三类；
- settings：SQLite 与真实 PG 18.6 同一 create/update/conflict contract；
- secret：round-trip、owner/purpose mismatch、ciphertext tamper、canary/key format、Unix 0644 reject/0600 accept；
- runtime：临时 root key 在 `/opt/nodecontroll/tmp` 生成、只读 mount，readiness 返回 database+secret_store ready；trap 后文件确认不存在；
- OpenAPI 4 paths、Web type/lint/Vitest 2/2/build、runtime smoke、358 trace/71 authored docs 由统一脚本继续覆盖；
- Cargo lock SHA-256 `b40ca8ea6f6b99e7e8444d03af86c3a3fb3d97ea20d6d98386715207aa7e3520`；pnpm lock 未变。

## 6. 下一步

本节也是历史下一步记录；Owner bootstrap、密码/session、C1 与 C2 已有后续实现说明。当前未完成项以 [项目进度](../00-project/PROGRESS.md) 和需求追踪矩阵为准。S3 adapter 可在资产/备份纵切接入，但 filesystem contract 从现在起必须保持兼容。
