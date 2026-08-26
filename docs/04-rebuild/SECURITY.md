# 安全模型、控制与验收

## 1. 基线与范围

目标验证基线为 [OWASP ASVS 5.0.0](https://owasp.org/www-project-application-security-verification-standard/) Level 2；Agent 远控、密钥、备份、MCP 和实例联合的高风险路径按更严格的项目控制验收。身份建议参考已于 2025-08-01 取代旧版的 [NIST SP 800-63B-4](https://pages.nist.gov/800-63-4/sp800-63b.html)，API 错误采用 [RFC 9457](https://datatracker.ietf.org/doc/html/rfc9457)。SSRF 控制以 [OWASP SSRF Prevention Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html) 为最低检查表。

安全目标：被盗的普通用户凭据不能管理服务器；一个 Agent 被攻陷不能横向接管 Master 或其他服务器；公开订阅/探针不能泄露控制面；外部 URL/模板/脚本/MCP 不能变成任意网络、文件或命令执行；备份和日志不能成为密钥副本；任何授权绕过、限制失效或远端执行都可审计和撤销。

## 2. 资产、参与者与信任边界

高价值资产：owner/admin 凭据、session/token、订阅 token、Agent CA/证书、server enrollment、用户协议 credential、TLS/ACME/DNS provider、WARP/SSH key、Telegram/MCP/federation 凭据、数据库、备份、sing-box/Nginx desired config、流量账本和审计链。

参与者：owner/admin/operator/support/auditor/member、匿名订阅者/探针访问者、Agent、tester、Telegram、MCP client、联合对端、外部 subscription/rule/ACME/DNS 服务、攻击者和被攻陷的远端服务器。

信任边界：

```text
Internet browser/client ─TLS─> Reverse proxy ─> Master HTTP
                                      │
Master DB/object/secret store <──── Master worker
                                      │ mTLS + signed envelope
                                      v
                              Agent ─local IPC─> sing-box/Nginx/tc

Untrusted external URLs <─ dedicated safe fetcher ─ Master/Agent
MCP/Telegram/Federation ─ distinct credentials/scopes ─ Master adapters
```

Master 与 Agent、Agent 与 sing-box、Master 与外部 fetcher 都是边界，不因同机部署而自动信任。SQLite 文件和宿主 root 权限不属于应用租户隔离；单实例管理员是可信运维主体，但仍受最小权限、再认证和审计约束。

## 3. 威胁清单与强制控制

| ID | 威胁 | 核心控制 | 必须通过的测试 |
|---|---|---|---|
| SEC-001 | 密码填充/账号枚举 | Argon2id、通用响应、IP+账号双限速、MFA/WebAuthn、session rotation | timing bucket、enumeration、rate-limit distributed test |
| SEC-002 | CSRF/session 固定 | HttpOnly/Secure/SameSite、CSRF token、Origin、login rotation | cross-site form/fetch、old cookie replay |
| SEC-003 | IDOR/越权 | handler 后 domain authorization：scope+relationship+state | 每端点不同用户/角色 object matrix |
| SEC-004 | secret 在 API/log/审计泄露 | Secret newtype、禁止 Debug、DTO projection、central redact | canary secret scan 全部 artifacts/logs/errors |
| SEC-005 | 外部订阅/规则 SSRF | scheme/host/CIDR allowlist、DNS+peer+redirect 重验、隔离 fetcher | IPv4/6/decimal/redirect/rebinding/metadata corpus |
| SEC-006 | YAML/压缩/解析 DoS | bytes/depth/alias/ratio/item/time/fuel limits | bombs/fuzz/slow body/resource ceiling |
| SEC-007 | Agent 任意命令/RCE | 无 shell API；typed allowlist；签名任务；local privilege separation | schema smuggling、argument injection、stale/replayed task |
| SEC-008 | Agent 横向移动 | 每设备 mTLS identity、server-bound audience、短轮换、无 Master DB credential | Agent A 冒充 B、CA rotation、revoked cert |
| SEC-009 | 恶意/被劫持 core 制品 | 官方 allowlist、TLS、hash、signature/attestation、pinned version、last-good | hash/signature mismatch、rollback、wrong arch |
| SEC-010 | 配置注入/secret 错配 | typed config compiler、semantic validate、atomic file mode、no raw shell | malformed Nginx/sing-box、symlink/path traversal |
| SEC-011 | 流量/速率/并发绕过 | server-side effective policy、official connection feed + tc、degraded fail-policy | reconnect/UDP/WS/loopback/Agent restart/multi-server |
| SEC-012 | 订阅 token 泄露/缓存串户 | hashed random token、route log redaction、principal cache key、revoke check | token in referer/log、cross-user cache、expiry/revoke |
| SEC-013 | 模板/脚本逃逸 | pure templates、sandbox WASM、no WASI network/fs、fuel/memory | filesystem/network/env/clock attempts、infinite loop |
| SEC-014 | 文件上传/恢复利用 | magic+size、non-executable object store、safe archive reader、manifest/hash | polyglot、zip slip、symlink、oversize、schema downgrade |
| SEC-015 | MCP prompt/工具滥用 | tool allowlist、resource scope、danger confirm、no hidden tool expansion | untrusted text instruction、confused deputy、replay confirm |
| SEC-016 | Telegram webhook/Mini App 伪造 | secret header、initData signature/age、user binding、nonce | forged/stale initData、chat swap、duplicate update |
| SEC-017 | 联合对端扩大权限/重放 | mutual pin、signed envelope、audience/scope/expiry/nonce、projection allowlist | replay、wrong audience、peer revoke、malicious projection |
| SEC-018 | 公开探针信息泄露/滥用 | public_id、fixed projection、rate/quota、network target policy | hidden fields snapshot、target SSRF/scan、high-cardinality DoS |
| SEC-019 | 备份泄密/回滚攻击 | AEAD、separate passphrase/KMS、signed manifest、version check、inspect first | wrong key/tamper/truncate/old schema/restore rehearsal |
| SEC-020 | 审计删除/伪造 | append-only、hash chain/checkpoint/export signature、restricted retention | row edit/delete/gap detection、clock/order anomaly |
| SEC-021 | 依赖/构建供应链 | lockfiles、digest images、SBOM、license/source、vuln+provenance gates | reproducibility、tampered dependency、unknown license |
| SEC-022 | XSS/content injection | Vue escaping、sanitize restricted markdown、CSP nonce/hash、no unsafe HTML | stored names/templates/log ANSI/markdown payloads |
| SEC-023 | trusted proxy/IP spoof | explicit proxy CIDR/hop、Forwarded parse、direct peer fallback | forged XFF, multi-hop, IPv6 normalization |
| SEC-024 | 删除/恢复误操作 | dependency impact plan、typed confirmation、re-auth、job/rollback | concurrent delete、partial Agent failure、restore after revoke |

## 4. 身份、密码和会话

- 密码用 Argon2id，参数在 VPS 目标级校准为约 250–500ms、合理内存并写入 hash；每次登录按 hash 参数透明升级。长度允许至少 64 字符，支持 paste/password manager，不规定字符组成；检查常见/泄漏密码使用本地可更新拒绝集，不向外部发送密码。
- 空实例 bootstrap 由本机一次性 setup capability 授权，是没有既有身份时的唯一例外。实例建立后，新增/晋升/转移 owner、改密码、关闭 MFA、生成恢复码、secret export、restore、CA/peer rotation 都需要 recent authentication。WebAuthn 优先，TOTP 可用，恢复码一次性 hash 保存；精确 rotation 与 challenge 合同见 [WP02_C_AUTHENTICATION_SECURITY_CONTRACT.md](../05-implementation/WP02_C_AUTHENTICATION_SECURITY_CONTRACT.md)。
- 登录失败不透露账号/MFA 是否存在。rate limiter 使用规范化账号 keyed bucket + IP/prefix bucket；反向代理 IP 只信任配置 CIDR。
- 浏览器 session credential 使用 256 bit CSPRNG，数据库只存带用途和版本的 HMAC；内部 session row ID 是不可充当凭据的 UUIDv7。登录、提权和密码修改都 rotation。absolute + idle expiry；撤销在服务端实时检查，不使用无法即时撤销的长寿命浏览器 JWT。
- 两枚 cookie 都使用 `__Host-` 前缀、Secure、Path=/、SameSite=Lax；session cookie 为 HttpOnly，CSRF cookie 则供同源前端读取并复制到 header。跨站部署若必须 `None`，需要 HTTPS、明确 Origin allowlist 和更严格 CSRF。
- 同源标签页共享 Cookie 但不共享 Pinia 状态。所有凭据 mutation 用 Web Locks exclusive 覆盖完整请求与响应校验，受保护读取用 shared；唯一 localStorage journal 只保存非秘密协调元数据，包括协议版本、epoch、规范十进制 `baseSeq/seq`、op/sender ID、operation、phase 与 disposition，BroadcastChannel/storage event 只唤醒。未知结果、journal 损坏、revision 回滚或同值篡改、跳号和未观察到的 terminal 都先关闭受保护 DOM。journal 缺失时，fresh setup/anonymous 且无 CSRF Cookie 是合法初始态；已有凭据迹象或 authenticated/unavailable/relogin-required 投影时必须隔离。显式登录成功或权威清理 204 才能恢复。
- 任何 Problem，包括旧凭据的 `401 SESSION_INVALID`，都不得写 `Set-Cookie`；迟到错误响应不能清掉另一个标签页刚轮换出的新 Cookie。设置、轮换和清除 Cookie 只允许来自路由明确声明的成功响应。
- personal/service token 使用高熵随机值、前缀识别、仅 hash、scope/CIDR/expiry；创建一次性回显。token 永远不能恢复明文。

## 5. 授权与资源隔离

每个 application use case 接收 `ActorContext`，在 repository 查询之前或同一事务内检查：credential status、global scope、resource relationship、effective policy、resource lifecycle、recent-auth。禁止 handler 先加载任意对象再靠前端过滤。

support 默认不可读完整源 IP/credential/artifact；auditor 不可执行；operator 不可管理 owner、根密钥或联合 trust。member 的对象查询必须以 `owner_principal_id`/entitlement join 限定，不接受客户端传入 user ID 作为授权证据。

批量命令逐项授权，返回 per-item result；任何一项无权时默认整批拒绝，除非端点明确声明 partial semantics。导出、搜索和 metrics 也应用同样字段级 projection。

## 6. Secret 管理与密码学

- 主密钥不在数据库。默认从权限 0600 root-owned key file/systemd credential 读取；可选外部 KMS。数据库备份和主密钥必须分开。
- `secret_records` 使用成熟 AEAD（AES-256-GCM 或 XChaCha20-Poly1305，最终实现锁定一种），每记录随机 nonce；AAD 包含 instance、purpose、owner type/id、schema/key version，防换位。
- data encryption key versioned；轮换采用双读单写，后台 rewrap，完成后销毁旧 key 前先备份/恢复演练。
- `Secret<T>`/`ExposedSecret<T>` 类型阻止默认 Debug/Serialize/Clone；只有最小执行函数在窄作用域 expose，随后 zeroize best-effort。Rust crash dump/core dump 默认禁用。
- 比较 token/hash 使用 constant-time。随机全部由 OS CSPRNG；禁止自研密码算法。
- API、OpenAPI example、metrics label、tracing span、job event、Agent error、support bundle 都经过结构化 redaction；不以正则作为唯一防线。

## 7. Master–Agent 与宿主权限

Agent enrollment token 一次性、短 TTL、server-bound；兑换后发独立 mTLS certificate，Master pin device ID/public key。envelope 还有 protocol version、server/task/audience、sequence、issued/expiry、body hash、签名；双层设计减少代理/TLS 终止配置错误的影响。

Agent 使用专用系统用户，默认无登录 shell。需要的特权拆为最小 helper/capability：管理指定 systemd units、写固定目录、Nginx validate/reload、tc/eBPF attach；不授予任意 sudo。path 使用预打开目录/`openat2` 语义、防 symlink；所有临时文件同目录创建、fsync、rename。

任务类型固定 schema，字段带长度/enum/path/URL/hash 校验。Agent 不执行从 Master 传来的 shell command、systemd unit name、任意下载 URL或任意文件路径。stdout/stderr 受大小限制并 redact。

Agent offline 时任务排队有 expiry；恢复后拒绝 stale desired revision。重复 task ID 返回原结果；顺序/冲突由 resource lease 控制。卸载/丢失/盗用可从 Master revoke，Agent 本地检测证书无效后进入 fail-safe，不自动信任新 Master。

## 8. sing-box、Nginx 与速率控制

- 只使用官方 sing-box source/tag/commit 构建，记录 build tags、Go toolchain、hash、SBOM、GPLv3/source offer。制品 allowlist 由项目 release manifest 提供，不允许用户把 URL 当二进制源。
- config compiler 输出到 staging，先 schema/semantic check 和 `sing-box check`；Nginx 用结构化生成 + `nginx -t`。通过后原子替换；保留 last-good 和 mode/owner/hash。失败不覆盖 active。
- reload 被视为可能中断（稳定 1.13 SIGHUP 会重建实例），UI/任务显示影响；部署前 flush 计量并在后续 epoch 对账。
- tc/eBPF 只挂目标 interfaces/qdisc/class，保存 previous state 并可原子回滚。Agent 报 `enforced/degraded/unsupported/unknown`，Master/UI 绝不把不支持显示成生效。
- 连接关闭、速率、并发和 IP 限制都以 server-side principal mapping 为准；协议无可靠 user mapping 时策略决定拒绝部署或显式降级，不根据节点显示名猜用户。

## 9. 网络请求、SSRF 与 webhook

所有外部获取经统一 `SafeHttpClient` 或隔离 fetcher；调用者必须声明 purpose policy。URL canonicalize 后仅允许登记 scheme/port；域名解析的每个地址和实际 peer 都过 CIDR policy，每次 redirect 重验；阻断 loopback、unspecified、link-local、multicast、benchmark、carrier-grade NAT、文档网段、云 metadata 和 IPv4-mapped IPv6 绕过。私网源需要管理员显式 CIDR allowlist。

限制 redirect、connect/total timeout、header/body、压缩比、响应类型、DNS 答案数和并发。默认不转发 credential/Authorization 到不同 origin；禁止环境代理，除非 purpose 显式配置并受同样目标策略。

出站 webhook 由管理员 allowlist endpoint，签名、timestamp、delivery ID；重试不改变 body/signature basis。入站 webhook 使用 opaque path + provider secret/signature、body limit、timestamp/replay cache，先验证原始 bytes 再解析。

公开 speed/probe 不接受任意 URL/IP/port；只选择管理员登记 target 或严格 allowlist。否则它会变成扫描器。

## 10. 输入、输出、浏览器和文件

- Rust 所有 JSON/query/path/header 有长度、字符和 collection 上限；SQL 使用 bind，不拼列/排序；命令不经过 shell。
- Vue 默认转义；Markdown 用固定 parser + sanitizer + allowlist，禁 script/style/iframe/event attributes/javascript/data URL。日志 ANSI 转义或 strip。
- CSP 默认 `default-src 'self'; object-src 'none'; base-uri 'none'; frame-ancestors 'none'`，script 只 self+nonce/hash；连接、图片按部署明确扩展。配合 HSTS、nosniff、Referrer-Policy、Permissions-Policy、严格 CORS。
- 文件名只作显示；存储用随机 object ID。校验 magic/扩展/MIME/大小，图像解码/re-encode 或隔离；对象目录不可执行且不由 Web server 直接列出。
- ZIP/TAR 恢复拒绝 absolute、`..`、symlink/hardlink/device、重复路径、case collision、压缩炸弹；先解到新临时目录和新 DB，完成 manifest/hash/schema/semantic 检查后进入受控恢复。
- CSV/表格导出前对 `= + - @` 等公式前缀进行目标安全编码，并在文档说明。

## 11. Telegram、MCP 与实例联合

Telegram Bot 不接受“管理员聊天即管理员”的隐式规则。Telegram identity 先由已认证站内用户生成短 pairing code 绑定；命令映射到站内 actor/scope。Mini App initData 按官方算法、bot token、auth_date 和 nonce 验证，然后换短期站内 session。

MCP 将所有 tool 调用视为不可信自动化：tool schema 参数严格；resource scope 固定；读与写分 token；delete/restore/rotate/deploy/traffic adjustment/close connection 等需要 out-of-band UI confirmation 或管理员显式设定的窄 policy。来自节点名、日志、订阅内容的文本不能改变 tool/权限。

联合对端不是同一信任域。每个 peer 独立 key、endpoint pin、scope 和 expiry；message 带 sender/recipient/audience/type/version/id/issued/expires/nonce/body hash/signature。只共享显式 projection，credential 采用重新签发/opaque capability，不直接复制 owner/Agent/数据库密钥。revoke 后已有导入引用进入 revoked/last-known，不静默永久可用。

## 12. 流量、连接与隐私

默认最少收集：流量账务需要 user/node/server/time bucket 和 bytes，不需要长期保存完整目标域名。连接历史 source IP 可按运营需要配置短期保留和 prefix 匿名化；公开 probe 永不输出用户/连接/IP 原始数据。

审计记录 actor、action、resource、outcome、reason、request/job ID、before/after 安全摘要和来源网络，不记录 secret/full config。metrics label 禁 user/server/node 动态 ID，详细维度进受权查询表。

用户删除：立即 revoke 凭据/session/token，个人资料软删/按 policy 匿名化；账本和审计保留不可变 pseudonymous principal ID。每类数据在设置中显示保留期与清理 job 状态。

## 13. 备份、恢复与灾难控制

备份内容有 manifest：产品/schema/version、每文件 path/type/size/hash、创建者/时间、加密算法/key metadata（无密钥）、依赖制品引用。先建立一致数据库快照，再收集对象；流式 AEAD，加密后再上传。

默认备份不含 Master root key；用户必须保管独立 passphrase/key。恢复先 `inspect`：认证、解密、hash、路径、schema upgrade route、磁盘需求、实例冲突。真正 restore 需要 maintenance mode、owner recent-auth、当前备份和明确回滚点；在新目录/DB 完成 migration/验证后切换。

恢复后强制决定：保留还是轮换 sessions/tokens/Agent/federation/webhook credentials；默认撤销浏览器 sessions 和一次性 enrollment。每个 release 的 VPS 门必须完成一次自动备份→新实例恢复→核心 smoke，而非只测试“备份命令成功”。

## 14. 审计完整性与告警

审计 append-only；每条 `entry_hash = H(previous_hash | canonical_entry)`，定期生成独立签名 checkpoint。数据库 superuser 仍可删除整表，因此 checkpoint/export 应支持写入外部 append target；UI 明确区分“链一致”与“外部锚定”。

安全事件：登录/MFA/token/secret/CA/peer 变更、权限/套餐/流量 adjustment、远端 service/config/binary、Nginx/cert、backup/restore、MCP dangerous tool、probe abuse、redaction failure。高危失败和 repeated deny 进入告警去重/quiet-hour 例外策略。

系统时钟异常、Agent sequence rollback、审计 gap、制品 hash mismatch、恢复 manifest mismatch 不允许被归类为普通 warning。

## 15. 供应链与发布

- Rust/Node/Go module lock/pin；容器用 digest；公开 Actions 负责编译 release，VPS 以无漂移私有依赖输入运行测试和制品验收，保存工具链/version/digest。
- 生成 CycloneDX/SPDX SBOM、第三方许可证、sing-box source tag/commit/source offer。GPL sing-box 作为独立进程/制品，不隐藏修改；目标是不维护 fork。
- 依赖漏洞扫描需要 severity、reachable assessment、owner、deadline 和 waiver；禁止无人负责的永久 ignore。
- release artifact checksum/signature/provenance；部署只认项目 release manifest allowlist。
- secret scanning 覆盖 git、构建上下文、镜像 layers、前端 dist、source maps、support bundle 和测试输出。
- reproducible target 至少二次干净容器构建 hash 可比较；无法逐字节一致的部分记录原因和 provenance。

## 16. 安全开发与响应门

每个 feature PR/阶段包回答：资产/actor、输入/外部调用、权限、secret/PII、远端副作用、失败/回滚、audit、abuse limit、测试。数据库 migration 和 API schema 有单独 security review。

VPS 发布门：

1. ASVS 5.0 L2 映射无无主项；本文件 SEC-001～024 全有自动或人工证据；
2. SAST、dependency/container scan、secret scan、SBOM/license 通过；
3. auth/authz/CSRF/CORS/CSP、SSRF corpus、archive/upload、Agent replay/impersonation、MCP/Telegram/federation signature 测试通过；
4. canary secrets 扫日志/API/audit/artifact/backup/support bundle为零泄露；
5. 备份恢复演练、CA/token/secret rotation 和 Agent/core last-good rollback 通过；
6. 独立攻击面复核处理所有 critical/high，medium 有明确接受人和期限。

疑似泄露的响应顺序：隔离/撤销（session/token/Agent/peer/provider）→保存审计和镜像证据→确认范围→轮换关联 secret→恢复 last-good→通知受影响管理员/用户→补测试与公开 advisory。不得为了“保留现场”继续让已知凭据有效。
