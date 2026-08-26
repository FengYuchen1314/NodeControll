# WP-02-C2 持久密钥 canary 与恢复码实现

> 候选基线：`d200c033b81ebabfe0c99c50572cc46186ba5329` 之上的 `codex/wp02-c2-backend`。本文记录代码合同，不记录尚未发生的公开 Actions 或 VPS 通过结论。

## 1. 交付边界

C2 把 WP-01 的进程内 AEAD round-trip 升级为数据库持久安全边界，并完成恢复码的生成、保存、摘要、整组替换和单次消费：

- root key 仍只来自 owner-only regular file；配置加载一枚 current key 和最多 3 枚严格更旧的 key，不存在授权服务、许可证或官方域名依赖；
- Master 完成数据库迁移后、绑定 HTTP 端口前，创建或解密 system-owned 持久 canary。格式合法但不匹配持久数据的 key 会使启动失败；旧 key 解密成功时，canary 在事务中 rewrap 到 current；
- `secret_records` 由自由字符串升级为 typed owner/purpose/schema record。AEAD AAD 同时包含 purpose、owner type/id、schema version 和 key version，不能把密文换位或篡改版本；
- bootstrap 的 instance、Owner、认证状态、默认设置、恢复码 set v1 和永久 latch 在同一数据库事务提交。任何一步失败都不留下部分恢复码；
- 每组恰好 8 个恢复码，每个来自 16 byte CSPRNG。展示是八组四位十六进制；输入只接受该格式或 32 位连续十六进制，大小写归一，不接受空白或其他分隔符；
- 数据库只保存 `RecoveryCode` 专用 HKDF/HMAC-SHA-256、digest key version、set version、position、创建/消费时间，不保存明文；
- 重生成事务重新锁定 user/auth/session/recent-auth snapshot，先把旧 active set 标为 replaced，再插入单调新版本；同一码消费是 `consumed_at_ms IS NULL` 的条件更新，并发调用只有一个影响一行。

## 2. 代码与迁移

| 层 | 实现 |
|---|---|
| 密码学 | `crates/secrets/src/lib.rs`：`SecretBinding`、`SecretPurpose`、`SecretOwnerKind`、`Keyring`、`RecoveryCode`、`KeyedDigestPurpose::RecoveryCode`；另为 C3 提供 `AuthChallengeToken`、current-key 生成/摘要和按记录 key version 验证 API，使用独立 HKDF context |
| 配置/启动 | `crates/config/src/lib.rs` 的 `previous_root_keys` 校验；`apps/master/src/main.rs` 在 bind 前调用 `initialize_root_key_canary` |
| 数据库 | SQLite/PostgreSQL `0006_secret_recovery.sql`；`ensure/rotate/active_secret_record`、bootstrap-with-recovery、summary、replace 和 conditional consume repository |
| 应用 | `ControlPlaneApplication` 生成一次性码、兼容旧 key session digest、recent-auth 管理入口与后续恢复流程可复用的 consume boundary |
| HTTP/OpenAPI | `POST /api/v1/bootstrap`、`GET/POST /api/v1/me/recovery-codes`；Rust OpenAPI 与 `openapi/nodecontroll-v1.json` 同步 |

0006 遇到非空的旧式、无 owner/schema 的 `secret_records` 会整笔迁移失败。实现不猜测业务 owner，也不把旧 `NCSECRET1` ciphertext 静默包装成看似 typed 的记录；操作者必须先按来源显式解密/重加密或移除废弃记录。空表升级保持原子。

## 3. HTTP 合同

`POST /api/v1/bootstrap` 成功返回 201：

```json
{
  "data": {
    "instance_id": "019...",
    "owner_id": "019...",
    "one_time_recovery_codes": ["0123-4567-89ab-cdef-0123-4567-89ab-cdef"]
  },
  "meta": {"api_version": "v1", "request_id": "..."}
}
```

数组固定 8 项；示例只展示形状。响应带 `Cache-Control: no-store`。数据库提交失败、并发 bootstrap 失败或 capability 无效时都不回显码。

`GET /api/v1/me/recovery-codes` 只返回：

```json
{"data":{"set_version":1,"total_count":8,"remaining_count":8,"created_at_ms":1777777777000},"meta":{"api_version":"v1","request_id":"..."}}
```

GET 需要有效 session，不接受也不返回 code。没有 active set 时返回 `409 RECOVERY_CODES_UNAVAILABLE`，不会为 legacy 用户偷偷生成无法交付的明文。

`POST /api/v1/me/recovery-codes` 需要有效 session、同源 Origin/Host、double-submit CSRF 和服务端 recent-auth；`force_password_change` 期间拒绝。成功返回 `{set_version,one_time_recovery_codes:string[8],created_at_ms}`，带 `Cache-Control: no-store`。旧 set 与新 set 的切换是单事务；结果未知时客户端不得自行假设旧码仍有效。

## 4. key rotation 和失败语义

`auth.digest_key_version` 是 current version。`[[secrets.previous_root_keys]]` 只用于有限迁移窗口：最多 3 项、不能重复、必须小于 current。新 session、login bucket、恢复码和 secret envelope 始终使用 current；读 session、消费恢复码和解密 secret 会按记录中的版本选择 key。未知版本 fail closed。

持久 canary 是固定 typed binding：`purpose=root_key_canary`、`owner_type=system`、nil owner UUID、schema v1。AAD 还包含 envelope key version。数据库唯一索引保证并发启动不能产生两枚 active canary；CAS rotation 冲突时重新读取并验证胜者。

C2 不创建 C3 的 challenge 表或 HTTP 流程，但已经提供 32-byte CSPRNG、严格 64 位小写十六进制且由 `Zeroizing` 持有的 `AuthChallengeToken`。`Keyring::generate_auth_challenge` 始终用 current key 产生 token 与可持久摘要；`verify_auth_challenge` 按摘要记录的 key version 选择有限旧 key。`AuthChallenge` 与 session、CSRF、login bucket、恢复码分别派生 purpose key，后续不得复用 `RecoveryCode` digest。

## 5. 测试源码与待验收

候选测试源码覆盖：typed AAD/错误 owner、有限 keyring/旧版本 decrypt、恢复码格式和 purpose separation、challenge 256-bit 格式/current-key 生成/旧版本验证；双库 fresh migration、持久 canary 错 key 拒绝、bootstrap set v1、整组替换、旧 set 失效、摘要计数，以及同一码两个并发 consume 恰好一真一假；API bootstrap 一次性字段、`no-store` 和 OpenAPI path/schema。

本地按合同不运行编译或测试。正式 release 只由公开 GitHub Actions 编译；VPS 只对同 SHA Actions artifact 做 provenance、运行和测试验收。建议门命令见本任务交付说明，只有完整 run 写入证据后才能把需求状态从 `implemented` 提升为 `verified`。
