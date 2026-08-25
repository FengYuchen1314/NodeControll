# Go 包 `auth`

用户认证、密码、会话令牌、角色授权和两步验证上下文。

## `internal/auth/manager.go`

依赖：`context`、`errors`、`strings`、`sync`、`golang.org/x/crypto/bcrypt`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–17 | type | `Credentials` | 定义 'Credentials' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 19–23 | type | `Manager` | 定义 'Manager' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 25–32 | function | `NewManager` | 创建并初始化与 'new manager' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'errors.New' |
| 34–57 | function | `(*Manager).Authenticate` | *Manager 的方法，执行与 'authenticate' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'bcrypt.CompareHashAndPassword'、'errors.Is'、'm.repo.GetUser'、'strings.TrimSpace' |
| 59–88 | function | `(*Manager).Update` | *Manager 的方法，更新与 'update' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 5；goroutine 0；调用 'bcrypt.GenerateFromPassword'、'errors.New'、'm.mu.Lock'、'm.mu.Unlock'、'm.repo.RenameUser'、'm.repo.UpdateUserPassword'、'string' |
| 90–131 | function | `(*Manager).ChangePassword` | *Manager 的方法，执行与 'change password' 对应的业务或基础设施操作。 | 分支 9；循环 0；返回 9；goroutine 0；调用 'bcrypt.CompareHashAndPassword'、'bcrypt.GenerateFromPassword'、'errors.Is'、'errors.New'、'm.mu.Lock'、'm.mu.Unlock'、'm.repo.GetUser'、'm.repo.UpdateUserPassword'、'string'、'strings.TrimSpace' |
| 133–144 | function | `(*Manager).Credentials` | *Manager 的方法，执行与 'credentials' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'm.mu.RLock'、'm.mu.RUnlock'、'm.repo.GetUser' |
| 147–149 | function | `(*Manager).User` | *Manager 的方法，执行与 'user' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'm.repo.GetUser' |
| 152–178 | function | `(*Manager).ValidatePassword` | *Manager 的方法，校验与 'validate password' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 7；goroutine 0；调用 'bcrypt.CompareHashAndPassword'、'errors.Is'、'errors.New'、'm.repo.GetUser'、'strings.TrimSpace' |

## `internal/auth/repository_adapter.go`

依赖：`context`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 9–11 | type | `RepositoryAdapter` | 定义 'RepositoryAdapter' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 14–16 | function | `NewRepositoryAdapter` | 创建并初始化与 'new repository adapter' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 19–30 | function | `(*RepositoryAdapter).GetUser` | *RepositoryAdapter 的方法，查询或读取与 'get user' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'a.repo.GetUser' |

## `internal/auth/token_store.go`

依赖：`context`、`crypto/rand`、`encoding/base64`、`encoding/json`、`errors`、`io`、`net/http`、`strings`、`sync`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 16–19 | type | `session` | 定义 'session' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 21–21 | type | `contextKey` | 定义 'contextKey' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 24–24 | const | `userContextKey` | 定义 'userContextKey' 的不可变协议值、默认值或枚举成员。 |  |
| 27–27 | const | `AuthHeader` | 定义 'AuthHeader' 的不可变协议值、默认值或枚举成员。 |  |
| 29–33 | type | `TokenStore` | 定义 'TokenStore' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 35–43 | function | `NewTokenStore` | 创建并初始化与 'new token store' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'make' |
| 45–47 | function | `(*TokenStore).Issue` | *TokenStore 的方法，判断与 'issue' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 's.IssueWithTTL' |
| 50–72 | function | `(*TokenStore).IssueWithTTL` | *TokenStore 的方法，判断与 'issue with ttl' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'Add'、'errors.New'、'randomToken'、's.mu.Lock'、's.mu.Unlock'、'strings.TrimSpace'、'time.Now' |
| 74–77 | function | `(*TokenStore).Validate` | *TokenStore 的方法，校验与 'validate' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 's.Lookup' |
| 79–88 | function | `(*TokenStore).Revoke` | *TokenStore 的方法，执行与 'revoke' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'delete'、's.mu.Lock'、's.mu.Unlock'、'strings.TrimSpace' |
| 90–94 | function | `(*TokenStore).RevokeAll` | *TokenStore 的方法，执行与 'revoke all' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'make'、's.mu.Lock'、's.mu.Unlock' |
| 97–112 | function | `(*TokenStore).LoadSession` | *TokenStore 的方法，加载与 'load session' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'After'、's.mu.Lock'、's.mu.Unlock'、'strings.TrimSpace'、'time.Now' |
| 115–129 | function | `(*TokenStore).UpdateUsername` | *TokenStore 的方法，更新与 'update username' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 1；goroutine 0；调用 's.mu.Lock'、's.mu.Unlock'、'strings.TrimSpace' |
| 132–152 | function | `(*TokenStore).Lookup` | *TokenStore 的方法，执行与 'lookup' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'After'、'delete'、's.mu.Lock'、's.mu.Unlock'、'strings.TrimSpace'、'time.Now' |
| 154–156 | function | `ContextWithUsername` | 执行与 'context with username' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'context.WithValue' |
| 159–165 | function | `UsernameFromContext` | 执行与 'username from context' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'ctx.Value' |
| 168–173 | function | `UsernameOrDefault` | 执行与 'username or default' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'UsernameFromContext' |
| 175–181 | function | `randomToken` | 执行与 'random token' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'base64.RawURLEncoding.EncodeToString'、'io.ReadFull'、'make' |
| 183–199 | function | `RequireToken` | 执行与 'require token' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'ContextWithUsername'、'Get'、'WriteUnauthorizedResponse'、'http.HandlerFunc'、'next.ServeHTTP'、'r.Context'、'r.Header.Get'、'r.URL.Query'、'r.WithContext'、'store.Lookup'、'strings.TrimSpace' |
| 184–198 | closure | `RequireToken.closure#1` | 供 RequireToken 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'ContextWithUsername'、'Get'、'WriteUnauthorizedResponse'、'next.ServeHTTP'、'r.Context'、'r.Header.Get'、'r.URL.Query'、'r.WithContext'、'store.Lookup'、'strings.TrimSpace' |
| 202–204 | type | `UserRepository` | 定义 'UserRepository' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 207–211 | type | `User` | 定义 'User' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 214–234 | function | `RequireAdmin` | 执行与 'require admin' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'RequireToken'、'Set'、'UsernameFromContext'、'http.HandlerFunc'、'next.ServeHTTP'、'r.Context'、'repo.GetUser'、'w.Header'、'w.Write'、'w.WriteHeader' |
| 215–233 | closure | `RequireAdmin.closure#1` | 供 RequireAdmin 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'Set'、'UsernameFromContext'、'next.ServeHTTP'、'r.Context'、'repo.GetUser'、'w.Header'、'w.Write'、'w.WriteHeader' |
| 236–247 | function | `WriteUnauthorizedResponse` | 执行与 'write unauthorized response' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'Encode'、'Set'、'json.NewEncoder'、'w.Header'、'w.WriteHeader' |

## `internal/auth/totp.go`

依赖：`crypto/rand`、`crypto/sha256`、`encoding/base64`、`encoding/hex`、`fmt`、`strings`、`sync`、`time`、`github.com/pquerna/otp`、`github.com/pquerna/otp/totp`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 17–22 | function | `GenerateTOTPKey` | 生成与 'generate totp key' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'totp.Generate' |
| 24–26 | function | `ValidateTOTPCode` | 校验与 'validate totp code' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'totp.Validate' |
| 28–42 | function | `GenerateRecoveryCodes` | 生成与 'generate recovery codes' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0；调用 'fmt.Errorf'、'hex.EncodeToString'、'make'、'rand.Read'、'sha256.Sum256'、'strings.ToLower' |
| 44–56 | function | `ValidateRecoveryCode` | 校验与 'validate recovery code' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0；调用 'append'、'hex.EncodeToString'、'len'、'make'、'sha256.Sum256'、'strings.ToLower'、'strings.TrimSpace' |
| 58–62 | type | `twoFactorPending` | 定义 'twoFactorPending' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 64–68 | type | `TwoFactorPendingStore` | 定义 'TwoFactorPendingStore' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 70–75 | function | `NewTwoFactorPendingStore` | 创建并初始化与 'new two factor pending store' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'make' |
| 77–92 | function | `(*TwoFactorPendingStore).Issue` | *TwoFactorPendingStore 的方法，判断与 'issue' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'Add'、'base64.RawURLEncoding.EncodeToString'、'make'、'rand.Read'、's.mu.Lock'、's.mu.Unlock'、'time.Now' |
| 94–102 | function | `(*TwoFactorPendingStore).Validate` | *TwoFactorPendingStore 的方法，校验与 'validate' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'After'、's.mu.RLock'、's.mu.RUnlock'、'time.Now' |
| 104–108 | function | `(*TwoFactorPendingStore).Consume` | *TwoFactorPendingStore 的方法，执行与 'consume' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'delete'、's.mu.Lock'、's.mu.Unlock' |

