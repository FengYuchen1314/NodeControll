# Go 包 `handler`

HTTP/WebSocket/SSE 适配层以及多数业务编排逻辑。

## `internal/handler/apply_custom_rules.go`

依赖：`bytes`、`context`、`encoding/json`、`errors`、`fmt`、`miaomiaowu/internal/logger`、`net/http`、`strings`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`、`miaomiaowu/internal/validator`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 20–22 | type | `applyCustomRulesRequest` | 定义 'applyCustomRulesRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 24–27 | type | `applyCustomRulesResponse` | 定义 'applyCustomRulesResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 30–93 | function | `NewApplyCustomRulesHandler` | 创建并初始化与 'new apply custom rules handler' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 8；goroutine 0；调用 'Decode'、'Encode'、'Set'、'applyCustomRulesToYaml'、'auth.UsernameFromContext'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'json.NewEncoder'、'panic'、'r.Context'、'repo.GetUserSettings'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 35–92 | closure | `NewApplyCustomRulesHandler.closure#1` | 供 NewApplyCustomRulesHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 7；循环 0；返回 7；goroutine 0；调用 'Decode'、'Encode'、'Set'、'applyCustomRulesToYaml'、'auth.UsernameFromContext'、'errors.New'、'fmt.Errorf'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.GetUserSettings'、'string'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 95–192 | function | `applyCustomRulesToYamlFiltered` | 应用与 'apply custom rules to yaml filtered' 对应的业务或基础设施操作。 | 分支 18；循环 4；返回 12；goroutine 0；调用 'append'、'applyDNSRuleToNode'、'applyRuleProvidersRuleToNode'、'applyRulesRuleToNode'、'autoAddMissingProxyGroups'、'fmt.Errorf'、'fmt.Sprintf'、'len'、'logger.Info'、'repo.ListEnabledCustomRules'、'tempBuf.Bytes'、'tempEncoder.Encode'、'tempEncoder.SetIndent'、'validator.ValidateClashConfig'、'yaml.NewEncoder'、'yaml.Unmarshal' |
| 195–197 | function | `applyCustomRulesToYaml` | 应用与 'apply custom rules to yaml' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'applyCustomRulesToYamlFiltered' |
| 200–244 | function | `removeDuplicateRulesCaseInsensitive` | 移除与 'remove duplicate rules case insensitive' 对应的业务或基础设施操作。 | 分支 5；循环 2；返回 1；goroutine 0；调用 'append'、'extractRuleKey'、'isMatchRule'、'logger.Info'、'make'、'strings.ToLower' |
| 247–260 | function | `extractRuleKey` | 执行与 'extract rule key' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 2；goroutine 0 |
| 263–271 | function | `isMatchRule` | 判断与 'is match rule' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'strings.HasPrefix'、'strings.ToUpper'、'strings.TrimSpace' |
| 275–330 | function | `removeDuplicateNodesBasedOnNewRules` | 移除与 'remove duplicate nodes based on new rules' 对应的业务或基础设施操作。 | 分支 7；循环 2；返回 1；goroutine 0；调用 'append'、'extractRuleKey'、'isMatchRule'、'logger.Info'、'make'、'strings.HasPrefix'、'strings.ToLower'、'strings.ToUpper'、'strings.TrimSpace' |
| 333–352 | function | `extractRuleSetRules` | 执行与 'extract rule set rules' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 2；goroutine 0；调用 'append'、'strings.HasPrefix'、'strings.ToUpper'、'strings.TrimSpace' |
| 356–466 | function | `autoAddMissingProxyGroups` | 执行与 'auto add missing proxy groups' 对应的业务或基础设施操作。 | 分支 14；循环 4；返回 3；goroutine 0；调用 'append'、'findFieldNode'、'len'、'logger.Info'、'make'、'strings.Split'、'strings.TrimSpace' |
| 469–528 | function | `extractProxyGroupsFromRulesContent` | 执行与 'extract proxy groups from rules content' 对应的业务或基础设施操作。 | 分支 12；循环 3；返回 2；goroutine 0；调用 'append'、'len'、'make'、'strings.Split'、'strings.TrimSpace'、'yaml.Unmarshal' |
| 531–543 | function | `findFieldNode` | 查找与 'find field node' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 3；goroutine 0；调用 'len' |
| 546–568 | function | `applyDNSRuleToNode` | 应用与 'apply dns rule to node' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 1；goroutine 0；调用 'findFieldNode'、'len'、'setFieldNode'、'yaml.Unmarshal' |
| 571–684 | function | `applyRulesRuleToNode` | 应用与 'apply rules rule to node' 对应的业务或基础设施操作。 | 分支 18；循环 1；返回 2；goroutine 0；调用 'append'、'extractRuleSetRules'、'findFieldNode'、'len'、'make'、'removeDuplicateNodesBasedOnNewRules'、'setFieldNode'、'strings.HasPrefix'、'strings.ToUpper'、'strings.TrimSpace'、'yaml.Unmarshal' |
| 687–739 | function | `applyRuleProvidersRuleToNode` | 应用与 'apply rule providers rule to node' 对应的业务或基础设施操作。 | 分支 10；循环 0；返回 2；goroutine 0；调用 'findFieldNode'、'len'、'mergeMapNodes'、'setFieldNode'、'yaml.Unmarshal' |
| 742–763 | function | `setFieldNode` | 设置与 'set field node' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 2；goroutine 0；调用 'append'、'len' |
| 766–793 | function | `mergeMapNodes` | 执行与 'merge map nodes' 对应的业务或基础设施操作。 | 分支 3；循环 2；返回 1；goroutine 0；调用 'append'、'len' |

## `internal/handler/auth.go`

依赖：`encoding/json`、`errors`、`net/http`、`strings`、`time`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/captcha`、`miaomiaowu/internal/logger`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 16–21 | type | `loginRequest` | 定义 'loginRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 23–32 | type | `loginResponse` | 定义 'loginResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 34–37 | type | `credentialsRequest` | 定义 'credentialsRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 41–64 | function | `GetClientIP` | 查询或读取与 'get client ip' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 4；goroutine 0；调用 'len'、'r.Header.Get'、'strings.LastIndex'、'strings.Split'、'strings.TrimSpace' |
| 66–157 | function | `NewLoginHandler` | 创建并初始化与 'new login handler' 对应的业务或基础设施操作。 | 分支 16；循环 0；返回 12；goroutine 0；调用 'Decode'、'Format'、'GetClientIP'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'len'、'logger.Warn'、'manager.Authenticate'、'panic'、'r.Context'、'rateLimiter.Check'、'rateLimiter.RecordFailure'、'strings.TrimSpace'、'turnstile.Verify'、'writeError' |
| 71–156 | closure | `NewLoginHandler.closure#1` | 供 NewLoginHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 15；循环 0；返回 11；goroutine 0；调用 'Decode'、'Format'、'GetClientIP'、'errors.New'、'json.NewDecoder'、'len'、'logger.Warn'、'manager.Authenticate'、'r.Context'、'rateLimiter.Check'、'rateLimiter.RecordFailure'、'rateLimiter.RecordSuccess'、'strings.TrimSpace'、'time.Now'、'turnstile.Verify'、'writeError' |
| 159–194 | function | `NewCredentialsHandler` | 创建并初始化与 'new credentials handler' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'json.NewEncoder'、'manager.Update'、'panic'、'r.Context'、'strings.TrimSpace'、'tokens.RevokeAll'、'w.Header'、'w.WriteHeader'、'writeError' |
| 164–193 | closure | `NewCredentialsHandler.closure#1` | 供 NewCredentialsHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 4；循环 0；返回 4；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'manager.Update'、'r.Context'、'strings.TrimSpace'、'tokens.RevokeAll'、'w.Header'、'w.WriteHeader'、'writeError' |

## `internal/handler/backup.go`

依赖：`archive/zip`、`encoding/json`、`errors`、`fmt`、`io`、`net/http`、`os`、`path/filepath`、`strings`、`time`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 20–56 | function | `NewBackupDownloadHandler` | 创建并初始化与 'new backup download handler' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'Format'、'Set'、'addDirToZip'、'errors.New'、'fmt.Errorf'、'fmt.Sprintf'、'http.HandlerFunc'、'panic'、'repo.Checkpoint'、'time.Now'、'w.Header'、'writeBackupError'、'zip.NewWriter'、'zipWriter.Close' |
| 25–55 | closure | `NewBackupDownloadHandler.closure#1` | 供 NewBackupDownloadHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 4；循环 0；返回 4；goroutine 0；调用 'Format'、'Set'、'addDirToZip'、'errors.New'、'fmt.Errorf'、'fmt.Sprintf'、'repo.Checkpoint'、'time.Now'、'w.Header'、'writeBackupError'、'zip.NewWriter'、'zipWriter.Close' |
| 60–109 | function | `NewBackupRestoreHandler` | 创建并初始化与 'new backup restore handler' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'Set'、'errors.New'、'extractBackup'、'file.Close'、'fmt.Errorf'、'http.HandlerFunc'、'http.MaxBytesReader'、'io.Copy'、'os.CreateTemp'、'os.Remove'、'panic'、'r.FormFile'、'tempFile.Close'、'tempFile.Name'、'w.Header'、'writeBackupError' |
| 65–108 | closure | `NewBackupRestoreHandler.closure#1` | 供 NewBackupRestoreHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'Encode'、'Set'、'errors.New'、'extractBackup'、'file.Close'、'fmt.Errorf'、'http.MaxBytesReader'、'io.Copy'、'os.CreateTemp'、'os.Remove'、'r.FormFile'、'tempFile.Close'、'tempFile.Name'、'w.Header'、'w.WriteHeader'、'writeBackupError' |
| 113–174 | function | `NewSetupRestoreBackupHandler` | 创建并初始化与 'new setup restore backup handler' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 8；goroutine 0；调用 'errors.New'、'file.Close'、'fmt.Errorf'、'http.HandlerFunc'、'http.MaxBytesReader'、'io.Copy'、'len'、'os.CreateTemp'、'os.Remove'、'panic'、'r.Context'、'r.FormFile'、'repo.ListUsers'、'tempFile.Close'、'tempFile.Name'、'writeBackupError' |
| 118–173 | closure | `NewSetupRestoreBackupHandler.closure#1` | 供 NewSetupRestoreBackupHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 7；循环 0；返回 7；goroutine 0；调用 'Set'、'errors.New'、'extractBackup'、'file.Close'、'fmt.Errorf'、'http.MaxBytesReader'、'io.Copy'、'len'、'os.CreateTemp'、'os.Remove'、'r.Context'、'r.FormFile'、'repo.ListUsers'、'tempFile.Close'、'tempFile.Name'、'writeBackupError' |
| 177–221 | function | `addDirToZip` | 添加与 'add dir to zip' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 9；goroutine 0；调用 'file.Close'、'filepath.Join'、'filepath.Rel'、'filepath.Walk'、'info.IsDir'、'info.Name'、'io.Copy'、'os.Open'、'strings.HasPrefix'、'zip.FileInfoHeader'、'zipWriter.CreateHeader' |
| 178–220 | closure | `addDirToZip.closure#1` | 供 addDirToZip 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 7；循环 0；返回 8；goroutine 0；调用 'file.Close'、'filepath.Join'、'filepath.Rel'、'info.IsDir'、'info.Name'、'io.Copy'、'os.Open'、'strings.HasPrefix'、'zip.FileInfoHeader'、'zipWriter.CreateHeader' |
| 224–295 | function | `extractBackup` | 执行与 'extract backup' 对应的业务或基础设施操作。 | 分支 12；循环 2；返回 8；goroutine 0；调用 'IsDir'、'destFile.Close'、'errors.New'、'f.FileInfo'、'f.Open'、'filepath.Dir'、'fmt.Errorf'、'io.Copy'、'os.Create'、'os.MkdirAll'、'reader.Close'、'srcFile.Close'、'strings.Contains'、'strings.HasPrefix'、'zip.OpenReader' |
| 297–303 | function | `writeBackupError` | 执行与 'write backup error' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'Encode'、'Set'、'err.Error'、'json.NewEncoder'、'w.Header'、'w.WriteHeader' |

## `internal/handler/brute_force.go`

依赖：`context`、`fmt`、`net/http`、`sync`、`time`、`miaomiaowu/internal/logger`、`miaomiaowu/internal/notify`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 15–15 | var | `globalBruteForceProtector` | 保存 'globalBruteForceProtector' 的包级共享状态、配置或预计算值。 |  |
| 17–23 | type | `bruteForceRecord` | 定义 'bruteForceRecord' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 25–38 | type | `BruteForceProtector` | 定义 'BruteForceProtector' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 41–45 | function | `(*BruteForceProtector).SetRepo` | *BruteForceProtector 的方法，设置与 'set repo' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'p.mu.Lock'、'p.mu.Unlock' |
| 47–51 | function | `(*BruteForceProtector).getRepo` | *BruteForceProtector 的方法，查询或读取与 'get repo' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'p.mu.RLock'、'p.mu.RUnlock' |
| 56–66 | function | `NewBruteForceProtector` | 创建并初始化与 'new brute force protector' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 70–80 | function | `NewBruteForceProtectorWithConfig` | 创建并初始化与 'new brute force protector with config' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'time.Duration' |
| 84–88 | function | `(*BruteForceProtector).SetSkipLocalIP` | *BruteForceProtector 的方法，设置与 'set skip local ip' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'p.mu.Lock'、'p.mu.Unlock' |
| 91–96 | function | `(*BruteForceProtector).shouldSkip` | *BruteForceProtector 的方法，执行与 'should skip' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'IsLocalOrPrivateIP'、'p.mu.RLock'、'p.mu.RUnlock' |
| 99–106 | function | `(*BruteForceProtector).UpdateConfig` | *BruteForceProtector 的方法，更新与 'update config' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'p.mu.Lock'、'p.mu.Unlock'、'time.Duration' |
| 108–112 | function | `(*BruteForceProtector).getConfig` | *BruteForceProtector 的方法，查询或读取与 'get config' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'p.mu.RLock'、'p.mu.RUnlock' |
| 114–116 | function | `GetBruteForceProtector` | 查询或读取与 'get brute force protector' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 118–156 | function | `(*BruteForceProtector).IsBlocked` | *BruteForceProtector 的方法，判断与 'is blocked' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'Round'、'String'、'logger.Info'、'logger.Warn'、'now.Before'、'p.attempts.Delete'、'p.attempts.Load'、'p.getConfig'、'p.shouldSkip'、'rec.blockUntil.IsZero'、'rec.blockUntil.Sub'、'time.Now' |
| 158–231 | function | `(*BruteForceProtector).RecordFailure` | *BruteForceProtector 的方法，执行与 'record failure' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 5；goroutine 1；调用 'GetNotifier'、'fmt.Sprintf'、'logger.Warn'、'n.Send'、'now.Add'、'now.Before'、'now.Sub'、'p.attempts.Load'、'p.attempts.Store'、'p.getConfig'、'p.persistBan'、'p.recordEvent'、'p.shouldSkip'、'rec.blockUntil.Format'、'rec.blockUntil.IsZero'、'time.Now' |
| 233–235 | function | `(*BruteForceProtector).RecordSuccess` | *BruteForceProtector 的方法，执行与 'record success' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'p.attempts.Delete' |
| 238–246 | function | `(*BruteForceProtector).recordEvent` | *BruteForceProtector 的方法，执行与 'record event' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'context.Background'、'p.getRepo'、'repo.InsertSecurityEvent' |
| 249–275 | function | `(*BruteForceProtector).persistBan` | *BruteForceProtector 的方法，执行与 'persist ban' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 1；goroutine 0；调用 'context.Background'、'fmt.Sprintf'、'p.getRepo'、'repo.InsertSecurityEvent'、'repo.UpsertIPBan'、'time.Now' |
| 278–288 | function | `(*BruteForceProtector).BanIP` | *BruteForceProtector 的方法，执行与 'ban ip' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'logger.Warn'、'now.Add'、'p.attempts.Store'、'p.getConfig'、'p.persistBan'、'time.Now' |
| 291–300 | function | `(*BruteForceProtector).UnbanIP` | *BruteForceProtector 的方法，执行与 'unban ip' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'context.Background'、'logger.Info'、'p.attempts.Delete'、'p.getRepo'、'repo.InsertSecurityEvent'、'repo.ReleaseIPBan' |
| 304–326 | function | `(*BruteForceProtector).RestoreFromDB` | *BruteForceProtector 的方法，执行与 'restore from db' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 2；goroutine 0；调用 'logger.Info'、'logger.Warn'、'p.attempts.Store'、'p.getRepo'、'repo.ListRestorableIPBans' |
| 330–355 | function | `(*BruteForceProtector).StartCleanup` | *BruteForceProtector 的方法，启动与 'start cleanup' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 3；goroutine 0；调用 'ctx.Done'、'now.After'、'now.Sub'、'p.attempts.Delete'、'p.attempts.Range'、'rec.blockUntil.IsZero'、'ticker.Stop'、'time.NewTicker' |
| 338–352 | closure | `StartCleanup.closure#1` | 供 StartCleanup 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 4；循环 0；返回 2；goroutine 0；调用 'now.After'、'now.Sub'、'p.attempts.Delete'、'rec.blockUntil.IsZero' |
| 358–361 | type | `StatusRecorder` | 定义 'StatusRecorder' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 363–366 | function | `(*StatusRecorder).WriteHeader` | *StatusRecorder 的方法，执行与 'write header' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'r.ResponseWriter.WriteHeader' |

## `internal/handler/brute_force_persistence_test.go`

依赖：`context`、`path/filepath`、`testing`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 11–49 | function | `TestBruteForceBanPersistsAndRestores` | 执行与 'test brute force ban persists and restores' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 0；goroutine 0；调用 'NewBruteForceProtector'、'context.Background'、'filepath.Join'、'first.BanIP'、'first.SetRepo'、'first.SetSkipLocalIP'、'repo.Close'、'repo.ListActiveIPBans'、'restored.IsBlocked'、'restored.RestoreFromDB'、'restored.SetRepo'、'restored.SetSkipLocalIP'、'restored.UnbanIP'、'storage.NewTrafficRepository'、't.Fatal'、't.TempDir' |

## `internal/handler/chain_proxy_test.go`

依赖：`encoding/json`、`testing`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–78 | function | `TestChainProxyInjection` | 执行与 'test chain proxy injection' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 0；goroutine 0；调用 'buildProxiesFromNodes'、'findProxyByName'、'int64'、'len'、't.Error'、't.Errorf'、't.Fatal'、't.Fatalf' |
| 82–106 | function | `TestChainProxyInjection_TargetNotFound` | 执行与 'test chain proxy injection_ target not found' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'buildProxiesFromNodes'、'int64'、'len'、't.Error'、't.Fatalf' |
| 110–146 | function | `TestChainProxyInjection_DisabledTarget` | 执行与 'test chain proxy injection_ disabled target' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'buildProxiesFromNodes'、'int64'、'len'、't.Errorf'、't.Fatalf' |
| 150–196 | function | `TestChainProxyInjection_WithTagFilter` | 执行与 'test chain proxy injection_ with tag filter' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'buildProxiesFromNodes'、'int64'、'len'、't.Errorf'、't.Fatalf' |
| 200–236 | function | `TestChainProxyInjection_ChainIDOverridesLegacy` | 执行与 'test chain proxy injection_ chain id overrides legacy' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'buildProxiesFromNodes'、'findProxyByName'、'int64'、't.Errorf'、't.Fatal' |
| 240–264 | function | `TestChainProxyInjection_NilChainID_PreservesExisting` | 执行与 'test chain proxy injection_ nil chain id_ preserves existing' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'buildProxiesFromNodes'、'len'、't.Errorf'、't.Fatalf' |
| 268–305 | function | `TestMigrateChainProxyNodes_ExtractsDialerProxy` | 执行与 'test migrate chain proxy nodes_ extracts dialer proxy' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 0；goroutine 0；调用 'delete'、'json.Marshal'、'json.Unmarshal'、't.Error'、't.Errorf'、't.Fatalf' |
| 309–339 | function | `buildProxiesFromNodes` | 构建与 'build proxies from nodes' 对应的业务或基础设施操作。 | 分支 5；循环 2；返回 1；goroutine 0；调用 'append'、'json.Unmarshal'、'len'、'make'、'node.HasAnyTag' |
| 341–348 | function | `findProxyByName` | 查找与 'find proxy by name' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0 |

## `internal/handler/clash_snell_filter.go`

依赖：`strconv`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 20–98 | function | `filterSnellV6FromClashYAML` | 执行与 'filter snell v6 from clash yaml' 对应的业务或基础设施操作。 | 分支 15；循环 6；返回 6；goroutine 0；调用 'MarshalYAMLWithIndent'、'RemoveUnicodeEscapeQuotes'、'append'、'isSnellV6ProxyNode'、'len'、'make'、'string'、'yaml.Unmarshal'、'yamlMapScalar' |
| 101–110 | function | `isSnellV6ProxyNode` | 判断与 'is snell v6 proxy node' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'strconv.Atoi'、'yamlMapScalar' |
| 113–120 | function | `yamlMapScalar` | 执行与 'yaml map scalar' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0；调用 'len' |

## `internal/handler/clash_snell_filter_test.go`

依赖：`strings`、`testing`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 9–56 | function | `TestFilterSnellV6FromClashYAML` | 执行与 'test filter snell v6 from clash yaml' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 0；goroutine 0；调用 'filterSnellV6FromClashYAML'、'string'、'strings.Contains'、't.Errorf' |
| 59–69 | function | `TestFilterSnellV6NoOpWhenAbsent` | 执行与 'test filter snell v6 no op when absent' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'filterSnellV6FromClashYAML'、'string'、't.Error'、't.Errorf' |

## `internal/handler/client_ua.go`

依赖：`net/http`、`strings`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–11 | type | `clientUARule` | 定义 'clientUARule' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 13–25 | var | `clientUARules` | 保存 'clientUARules' 的包级共享状态、配置或预计算值。 |  |
| 27–37 | function | `detectClientTypeFromUA` | 执行与 'detect client type from ua' 对应的业务或基础设施操作。 | 分支 1；循环 2；返回 2；goroutine 0；调用 'strings.Contains'、'strings.ToLower'、'strings.TrimSpace' |
| 41–47 | function | `resolveClientType` | 解析或求解与 'resolve client type' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'Get'、'detectClientTypeFromUA'、'r.Header.Get'、'r.URL.Query'、'strings.EqualFold'、'strings.TrimSpace' |

## `internal/handler/custom_rules.go`

依赖：`encoding/json`、`errors`、`net/http`、`strconv`、`strings`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 16–22 | type | `customRuleRequest` | 定义 'customRuleRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 24–33 | type | `customRuleResponse` | 定义 'customRuleResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 35–68 | function | `NewCustomRulesHandler` | 创建并初始化与 'new custom rules handler' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 4；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'handleCreateCustomRule'、'handleListCustomRules'、'http.HandlerFunc'、'panic'、'r.Context'、'repo.GetUser'、'strings.TrimSpace'、'writeError' |
| 40–67 | closure | `NewCustomRulesHandler.closure#1` | 供 NewCustomRulesHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 4；循环 0；返回 3；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'handleCreateCustomRule'、'handleListCustomRules'、'r.Context'、'repo.GetUser'、'strings.TrimSpace'、'writeError' |
| 70–119 | function | `NewCustomRuleHandler` | 创建并初始化与 'new custom rule handler' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 6；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'handleDeleteCustomRule'、'handleGetCustomRule'、'handleUpdateCustomRule'、'http.HandlerFunc'、'panic'、'r.Context'、'repo.GetUser'、'strconv.ParseInt'、'strings.TrimPrefix'、'strings.TrimSpace'、'writeError' |
| 75–118 | closure | `NewCustomRuleHandler.closure#1` | 供 NewCustomRuleHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 6；循环 0；返回 5；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'handleDeleteCustomRule'、'handleGetCustomRule'、'handleUpdateCustomRule'、'r.Context'、'repo.GetUser'、'strconv.ParseInt'、'strings.TrimPrefix'、'strings.TrimSpace'、'writeError' |
| 121–147 | function | `handleListCustomRules` | 处理与 'handle list custom rules' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 1；goroutine 0；调用 'Encode'、'Get'、'Set'、'append'、'json.NewEncoder'、'len'、'make'、'r.Context'、'r.URL.Query'、'repo.ListCustomRules'、'rule.CreatedAt.Format'、'rule.UpdatedAt.Format'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 149–174 | function | `handleGetCustomRule` | 处理与 'handle get custom rule' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'Encode'、'Set'、'errors.Is'、'errors.New'、'json.NewEncoder'、'r.Context'、'repo.GetCustomRule'、'rule.CreatedAt.Format'、'rule.UpdatedAt.Format'、'w.Header'、'w.WriteHeader'、'writeError' |
| 176–233 | function | `handleCreateCustomRule` | 处理与 'handle create custom rule' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 5；goroutine 0；调用 'Decode'、'Encode'、'Set'、'err.Error'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.CreateCustomRule'、'rule.CreatedAt.Format'、'rule.UpdatedAt.Format'、'strings.Contains'、'w.Header'、'w.WriteHeader'、'writeError'、'yaml.Unmarshal' |
| 235–296 | function | `handleUpdateCustomRule` | 处理与 'handle update custom rule' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 6；goroutine 0；调用 'Decode'、'Encode'、'Set'、'err.Error'、'errors.Is'、'errors.New'、'json.NewDecoder'、'r.Context'、'repo.UpdateCustomRule'、'rule.CreatedAt.Format'、'rule.UpdatedAt.Format'、'strings.Contains'、'w.Header'、'w.WriteHeader'、'writeError'、'yaml.Unmarshal' |
| 298–309 | function | `handleDeleteCustomRule` | 处理与 'handle delete custom rule' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'errors.Is'、'errors.New'、'r.Context'、'repo.DeleteCustomRule'、'w.WriteHeader'、'writeError' |

## `internal/handler/debug.go`

依赖：`bytes`、`context`、`errors`、`fmt`、`io`、`net/http`、`os`、`path/filepath`、`strconv`、`strings`、`sync`、`time`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/logger`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 22–22 | const | `debugAutoCloseSeconds` | 定义 'debugAutoCloseSeconds' 的不可变协议值、默认值或枚举成员。 |  |
| 24–30 | type | `debugHandler` | 定义 'debugHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 33–42 | function | `NewDebugHandler` | 创建并初始化与 'new debug handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'logger.NewLogManager'、'panic' |
| 44–69 | function | `(*debugHandler).ServeHTTP` | *debugHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'h.handleDisable'、'h.handleDownload'、'h.handleEnable'、'h.handleStatus'、'h.handleTail'、'methodNotAllowed'、'r.Context'、'strings.Trim'、'strings.TrimPrefix'、'strings.TrimSpace'、'writeError' |
| 72–133 | function | `(*debugHandler).handleEnable` | *debugHandler 的方法，处理与 'handle enable' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 5；goroutine 0；调用 'errors.Is'、'fmt.Errorf'、'h.logManager.CreateLogFile'、'h.repo.GetUserSettings'、'h.repo.UpsertUserSettings'、'h.startAutoCloseTimer'、'logger.DisableDebug'、'logger.EnableDebug'、'logger.Info'、'now.Format'、'r.Context'、'respondJSON'、'time.Now'、'writeError' |
| 136–146 | function | `(*debugHandler).startAutoCloseTimer` | *debugHandler 的方法，启动与 'start auto close timer' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'h.autoClose'、'h.autoCloseTimer.Stop'、'h.mu.Lock'、'h.mu.Unlock'、'time.AfterFunc'、'time.Duration' |
| 143–145 | closure | `startAutoCloseTimer.closure#1` | 供 startAutoCloseTimer 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'h.autoClose' |
| 149–156 | function | `(*debugHandler).stopAutoCloseTimer` | *debugHandler 的方法，停止与 'stop auto close timer' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'h.autoCloseTimer.Stop'、'h.mu.Lock'、'h.mu.Unlock' |
| 159–180 | function | `(*debugHandler).autoClose` | *debugHandler 的方法，执行与 'auto close' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'context.Background'、'h.mu.Lock'、'h.mu.Unlock'、'h.repo.GetUserSettings'、'h.repo.UpsertUserSettings'、'logger.DisableDebug'、'logger.Error'、'logger.Info' |
| 183–223 | function | `(*debugHandler).handleDisable` | *debugHandler 的方法，处理与 'handle disable' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'filepath.Base'、'fmt.Errorf'、'fmt.Sprintf'、'h.repo.GetUserSettings'、'h.repo.UpsertUserSettings'、'h.stopAutoCloseTimer'、'logger.DisableDebug'、'logger.Info'、'r.Context'、'respondJSON'、'writeError' |
| 226–273 | function | `(*debugHandler).handleStatus` | *debugHandler 的方法，处理与 'handle status' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 3；goroutine 0；调用 'Seconds'、'duration.Seconds'、'errors.Is'、'formatDuration'、'formatFileSize'、'h.logManager.GetLogFileSize'、'h.repo.GetUserSettings'、'h.repo.UpsertUserSettings'、'int'、'logger.DisableDebug'、'logger.Info'、'r.Context'、'respondJSON'、'time.Since'、'writeError' |
| 276–347 | function | `(*debugHandler).handleDownload` | *debugHandler 的方法，处理与 'handle download' 对应的业务或基础设施操作。 | 分支 9；循环 0；返回 8；goroutine 1；调用 'Get'、'Set'、'errors.New'、'file.Close'、'filepath.Base'、'filepath.Join'、'h.repo.GetUserSettings'、'os.IsNotExist'、'os.Open'、'os.Stat'、'r.Context'、'r.URL.Query'、'strings.HasPrefix'、'strings.HasSuffix'、'w.Header'、'writeError' |
| 339–346 | closure | `handleDownload.closure#1` | 供 handleDownload 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'h.logManager.DeleteLogFile'、'logger.Error'、'logger.Info'、'time.Sleep' |
| 350–379 | function | `(*debugHandler).handleTail` | *debugHandler 的方法，处理与 'handle tail' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 2；goroutine 0；调用 'Get'、'h.logManager.GetLogFileSize'、'h.repo.GetUserSettings'、'int64'、'r.Context'、'r.URL.Query'、'respondJSON'、'strconv.Atoi'、'tailFile' |
| 382–438 | function | `tailFile` | 执行与 'tail file' 对应的业务或基础设施操作。 | 分支 6；循环 2；返回 5；goroutine 0；调用 'buf.Bytes'、'buf.Reset'、'buf.Write'、'bytes.Count'、'bytes.Join'、'bytes.Split'、'f.Close'、'f.ReadAt'、'f.Stat'、'int64'、'len'、'make'、'os.Open'、'stat.Size'、'string' |
| 440–451 | function | `formatFileSize` | 执行与 'format file size' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0；调用 'float64'、'fmt.Sprintf'、'int64' |
| 453–469 | function | `formatDuration` | 执行与 'format duration' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'd.Seconds'、'fmt.Sprintf'、'int' |

## `internal/handler/dns.go`

依赖：`errors`、`net`、`net/http`、`strings`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 10–10 | type | `dnsHandler` | 定义 'dnsHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 13–15 | function | `NewDNSHandler` | 创建并初始化与 'new dns handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 17–28 | function | `(*dnsHandler).ServeHTTP` | *dnsHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'h.handleResolve'、'methodNotAllowed'、'strings.Trim'、'strings.TrimPrefix' |
| 30–82 | function | `(*dnsHandler).handleResolve` | *dnsHandler 的方法，处理与 'handle resolve' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 4；goroutine 0；调用 'Get'、'append'、'err.Error'、'errors.New'、'ip.String'、'ip.To4'、'len'、'net.LookupIP'、'net.ParseIP'、'net.SplitHostPort'、'r.URL.Query'、'respondJSON'、'writeBadRequest'、'writeError' |

## `internal/handler/external_subscriptions.go`

依赖：`encoding/json`、`errors`、`miaomiaowu/internal/logger`、`net/http`、`strconv`、`strings`、`time`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 16–23 | type | `externalSubscriptionRequest` | 定义 'externalSubscriptionRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 25–41 | type | `externalSubscriptionResponse` | 定义 'externalSubscriptionResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 44–53 | function | `normalizeUpdateIntervalMinutes` | 规范化与 'normalize update interval minutes' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0 |
| 55–72 | function | `resolveAutoUpdateSettings` | 解析或求解与 'resolve auto update settings' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 2；goroutine 0；调用 'normalizeUpdateIntervalMinutes' |
| 74–102 | function | `toExternalSubscriptionResponse` | 执行与 'to external subscription response' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'sub.CreatedAt.Format'、'sub.Expire.Format'、'sub.LastSyncAt.Format'、'sub.UpdatedAt.Format' |
| 104–129 | function | `NewExternalSubscriptionsHandler` | 创建并初始化与 'new external subscriptions handler' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 2；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'handleCreateExternalSubscription'、'handleDeleteExternalSubscription'、'handleListExternalSubscriptions'、'handleUpdateExternalSubscription'、'http.HandlerFunc'、'panic'、'r.Context'、'strings.TrimSpace'、'writeError' |
| 109–128 | closure | `NewExternalSubscriptionsHandler.closure#1` | 供 NewExternalSubscriptionsHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'handleCreateExternalSubscription'、'handleDeleteExternalSubscription'、'handleListExternalSubscriptions'、'handleUpdateExternalSubscription'、'r.Context'、'strings.TrimSpace'、'writeError' |
| 131–146 | function | `handleListExternalSubscriptions` | 处理与 'handle list external subscriptions' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 1；goroutine 0；调用 'Encode'、'Set'、'append'、'json.NewEncoder'、'len'、'make'、'r.Context'、'repo.ListExternalSubscriptions'、'toExternalSubscriptionResponse'、'w.Header'、'w.WriteHeader'、'writeError' |
| 148–294 | function | `handleCreateExternalSubscription` | 处理与 'handle create external subscription' 对应的业务或基础设施操作。 | 分支 22；循环 0；返回 9；goroutine 0；调用 'Decode'、'ParseTrafficInfoHeader'、'client.Do'、'errors.New'、'http.NewRequestWithContext'、'json.NewDecoder'、'logger.Info'、'r.Context'、'req.Header.Set'、'resp.Body.Close'、'resp.Header.Get'、'retryReq.Header.Set'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace'、'writeError' |
| 296–380 | function | `handleUpdateExternalSubscription` | 处理与 'handle update external subscription' 对应的业务或基础设施操作。 | 分支 11；循环 0；返回 10；goroutine 0；调用 'Decode'、'Get'、'Set'、'errors.Is'、'errors.New'、'json.NewDecoder'、'r.Context'、'r.URL.Query'、'repo.GetExternalSubscription'、'repo.UpdateExternalSubscription'、'resolveAutoUpdateSettings'、'strconv.ParseInt'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 382–405 | function | `handleDeleteExternalSubscription` | 处理与 'handle delete external subscription' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 4；goroutine 0；调用 'Get'、'errors.Is'、'errors.New'、'r.Context'、'r.URL.Query'、'repo.DeleteExternalSubscription'、'strconv.ParseInt'、'w.WriteHeader'、'writeError' |
| 408–467 | function | `NewExternalSubscriptionNodesHandler` | 创建并初始化与 'new external subscription nodes handler' 对应的业务或基础设施操作。 | 分支 7；循环 1；返回 8；goroutine 0；调用 'Get'、'Set'、'auth.UsernameFromContext'、'errors.Is'、'errors.New'、'fetchSubscriptionNodes'、'http.HandlerFunc'、'len'、'logger.Info'、'make'、'r.Context'、'r.URL.Query'、'repo.GetExternalSubscription'、'strconv.ParseInt'、'strings.TrimSpace'、'writeError' |
| 409–466 | closure | `NewExternalSubscriptionNodesHandler.closure#1` | 供 NewExternalSubscriptionNodesHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 7；循环 1；返回 7；goroutine 0；调用 'Get'、'Set'、'auth.UsernameFromContext'、'errors.Is'、'errors.New'、'fetchSubscriptionNodes'、'len'、'logger.Info'、'make'、'r.Context'、'r.URL.Query'、'repo.GetExternalSubscription'、'strconv.ParseInt'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 470–520 | function | `NewExternalSubscriptionCheckFilterHandler` | 创建并初始化与 'new external subscription check filter handler' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 7；goroutine 0；调用 'Decode'、'Encode'、'Set'、'auth.UsernameFromContext'、'checkFilterMatches'、'errors.Is'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.GetExternalSubscription'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 471–519 | closure | `NewExternalSubscriptionCheckFilterHandler.closure#1` | 供 NewExternalSubscriptionCheckFilterHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'Decode'、'Encode'、'Set'、'auth.UsernameFromContext'、'checkFilterMatches'、'errors.Is'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.GetExternalSubscription'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |

## `internal/handler/external_sync.go`

依赖：`bytes`、`context`、`crypto/rand`、`encoding/hex`、`encoding/json`、`errors`、`fmt`、`io`、`miaomiaowu/internal/logger`、`net/http`、`os`、`regexp`、`strconv`、`strings`、`sync`、`time`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`、`miaomiaowu/internal/util`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 28–28 | const | `defaultNodeNameFilterPattern` | 定义 'defaultNodeNameFilterPattern' 的不可变协议值、默认值或枚举成员。 |  |
| 30–30 | const | `externalSyncSelectionTTL` | 定义 'externalSyncSelectionTTL' 的不可变协议值、默认值或枚举成员。 |  |
| 32–40 | type | `externalSyncCandidate` | 定义 'externalSyncCandidate' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 42–46 | type | `externalSyncSelectionSession` | 定义 'externalSyncSelectionSession' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 48–51 | var | `externalSyncSelections` | 保存 'externalSyncSelections' 的包级共享状态、配置或预计算值。 |  |
| 53–56 | type | `manualExternalSyncResult` | 定义 'manualExternalSyncResult' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 58–64 | function | `randomExternalSyncID` | 执行与 'random external sync id' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'hex.EncodeToString'、'make'、'rand.Read' |
| 66–95 | function | `storeExternalSyncSelection` | 执行与 'store external sync selection' 对应的业务或基础设施操作。 | 分支 4；循环 2；返回 4；goroutine 0；调用 'delete'、'externalSyncSelections.Lock'、'externalSyncSelections.Unlock'、'len'、'make'、'now.Add'、'now.After'、'randomExternalSyncID'、'time.Now' |
| 97–119 | function | `applyNodeNameFilterToProxies` | 应用与 'apply node name filter to proxies' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 2；goroutine 0；调用 'append'、'filterRegex.MatchString'、'len'、'logger.Info'、'make' |
| 122–201 | function | `syncExternalSubscriptionsManual` | 同步与 'sync external subscriptions manual' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 4；goroutine 0；调用 'append'、'fmt.Errorf'、'len'、'logger.Info'、'newSSRFSafeHTTPClient'、'repo.GetUserSettings'、'repo.ListExternalSubscriptions'、'repo.UpdateExternalSubscription'、'syncSingleExternalSubscriptionWithSelection'、'time.Now' |
| 204–311 | function | `syncExternalSubscriptions` | 同步与 'sync external subscriptions' 对应的业务或基础设施操作。 | 分支 10；循环 2；返回 5；goroutine 0；调用 'append'、'fmt.Errorf'、'getUsedExternalSubscriptionURLs'、'len'、'logger.Info'、'newSSRFSafeHTTPClient'、'repo.GetUserSettings'、'repo.ListExternalSubscriptions'、'repo.UpdateExternalSubscription'、'syncSingleExternalSubscription'、'time.Now' |
| 314–358 | function | `getUsedExternalSubscriptionURLs` | 查询或读取与 'get used external subscription ur ls' 对应的业务或基础设施操作。 | 分支 7；循环 2；返回 3；goroutine 0；调用 'fmt.Errorf'、'fmt.Sprintf'、'logger.Info'、'make'、'os.ReadFile'、'repo.ListSubscribeFiles'、'yaml.Unmarshal' |
| 362–365 | function | `syncSingleExternalSubscription` | 同步与 'sync single external subscription' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'syncSingleExternalSubscriptionWithSelection' |
| 367–808 | function | `syncSingleExternalSubscriptionWithSelection` | 同步与 'sync single external subscription with selection' 对应的业务或基础设施操作。 | 分支 69；循环 8；返回 10；goroutine 1；调用 'client.Do'、'fmt.Errorf'、'http.NewRequestWithContext'、'io.LimitReader'、'io.ReadAll'、'logger.Info'、'parseAndUpdateTrafficInfo'、'req.Header.Set'、'resp.Body.Close'、'resp.Header.Get'、'strings.Contains'、'strings.ToLower'、'trafficReq.Header.Set'、'trafficResp.Body.Close'、'trafficResp.Header.Get'、'validateFetchURL' |
| 813–849 | function | `ParseTrafficInfoHeader` | 解析与 'parse traffic info header' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 1；goroutine 0；调用 'len'、'strconv.ParseInt'、'strings.Split'、'strings.SplitN'、'strings.TrimSpace'、'time.Unix' |
| 853–931 | function | `parseAndUpdateTrafficInfo` | 解析与 'parse and update traffic info' 对应的业务或基础设施操作。 | 分支 12；循环 1；返回 0；goroutine 0；调用 'expireTime.Format'、'float64'、'int64'、'len'、'logger.Info'、'repo.UpdateExternalSubscription'、'strconv.ParseFloat'、'strconv.ParseInt'、'strings.Split'、'strings.SplitN'、'strings.TrimSpace'、'sub.Expire.Format'、'time.Unix' |
| 934–937 | type | `SyncExternalSubscriptionsHandler` | 定义 'SyncExternalSubscriptionsHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 940–945 | function | `NewSyncExternalSubscriptionsHandler` | 创建并初始化与 'new sync external subscriptions handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 948–951 | type | `SyncSingleExternalSubscriptionHandler` | 定义 'SyncSingleExternalSubscriptionHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 954–959 | function | `NewSyncSingleExternalSubscriptionHandler` | 创建并初始化与 'new sync single external subscription handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 961–1072 | function | `(*SyncSingleExternalSubscriptionHandler).ServeHTTP` | *SyncSingleExternalSubscriptionHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 11；循环 1；返回 8；goroutine 0；调用 'Encode'、'Get'、'Set'、'auth.UsernameFromContext'、'h.repo.GetUserSettings'、'h.repo.ListExternalSubscriptions'、'http.Error'、'json.NewEncoder'、'logger.Info'、'newSSRFSafeHTTPClient'、'r.Context'、'r.URL.Query'、'strconv.ParseInt'、'syncSingleExternalSubscriptionWithSelection'、'w.Header'、'w.WriteHeader' |
| 1074–1113 | function | `(*SyncExternalSubscriptionsHandler).ServeHTTP` | *SyncExternalSubscriptionsHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 4；goroutine 0；调用 'Encode'、'Set'、'auth.UsernameFromContext'、'fmt.Errorf'、'fmt.Sprintf'、'http.Error'、'json.NewEncoder'、'logger.Info'、'r.Context'、'storeExternalSyncSelection'、'syncExternalSubscriptionsManual'、'w.Header'、'w.WriteHeader'、'writeError' |
| 1115–1118 | type | `confirmExternalSyncRequest` | 定义 'confirmExternalSyncRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 1120–1172 | function | `NewConfirmExternalSyncHandler` | 创建并初始化与 'new confirm external sync handler' 对应的业务或基础设施操作。 | 分支 8；循环 1；返回 6；goroutine 0；调用 'After'、'Decode'、'append'、'auth.UsernameFromContext'、'errors.New'、'externalSyncSelections.Lock'、'externalSyncSelections.Unlock'、'http.Error'、'http.HandlerFunc'、'json.NewDecoder'、'len'、'make'、'r.Context'、'strings.TrimSpace'、'time.Now'、'writeError' |
| 1121–1171 | closure | `NewConfirmExternalSyncHandler.closure#1` | 供 NewConfirmExternalSyncHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 8；循环 1；返回 5；goroutine 0；调用 'After'、'Decode'、'append'、'auth.UsernameFromContext'、'delete'、'errors.New'、'externalSyncSelections.Lock'、'externalSyncSelections.Unlock'、'http.Error'、'json.NewDecoder'、'len'、'make'、'r.Context'、'strings.TrimSpace'、'time.Now'、'writeError' |
| 1177–1270 | function | `syncProxyProviderNodesToYAML` | 同步与 'sync proxy provider nodes to yaml' 对应的业务或基础设施操作。 | 分支 12；循环 4；返回 5；goroutine 0；调用 'GetProxyProviderCache'、'RefreshProxyProviderCache'、'append'、'cache.Get'、'cache.IsExpired'、'copyMapForSync'、'fmt.Errorf'、'fmt.Sprintf'、'len'、'logger.Info'、'make'、'repo.ListProxyProviderConfigsBySubscription'、'repo.ListSubscribeFiles'、'strings.Index'、'updateYAMLFileWithProxyProviderNodes' |
| 1274–1624 | function | `updateYAMLFileWithProxyProviderNodes` | 更新与 'update yaml file with proxy provider nodes' 对应的业务或基础设施操作。 | 分支 42；循环 16；返回 9；goroutine 0；调用 'append'、'encoder.Close'、'encoder.Encode'、'encoder.SetIndent'、'fmt.Errorf'、'fmt.Sprintf'、'len'、'logger.Info'、'make'、'os.ReadFile'、'sanitizeExplicitStringTags'、'strings.HasPrefix'、'util.GetNodeFieldValue'、'util.ReorderProxyFieldsToNode'、'yaml.NewEncoder'、'yaml.Unmarshal' |
| 1627–1648 | function | `copyMapForSync` | 执行与 'copy map for sync' 对应的业务或基础设施操作。 | 分支 2；循环 2；返回 1；goroutine 0；调用 'copyMapForSync'、'len'、'make' |
| 1652–1676 | function | `buildSubInfoSuffix` | 构建与 'build sub info suffix' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 2；goroutine 0；调用 'Hours'、'append'、'fmt.Sprintf'、'formatTrafficShort'、'int'、'len'、'strings.Join'、'time.Until' |
| 1678–1687 | function | `formatTrafficShort` | 执行与 'format traffic short' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'float64'、'fmt.Sprintf' |
| 1691–1714 | function | `StartExternalSubscriptionAutoUpdateScheduler` | 启动与 'start external subscription auto update scheduler' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 2；goroutine 0；调用 'ctx.Done'、'logger.Info'、'runExternalSubscriptionAutoUpdates'、'ticker.Stop'、'time.NewTicker' |
| 1716–1794 | function | `runExternalSubscriptionAutoUpdates` | 运行与 'run external subscription auto updates' 对应的业务或基础设施操作。 | 分支 10；循环 1；返回 2；goroutine 0；调用 'cancel'、'context.WithTimeout'、'logger.Info'、'make'、'newSSRFSafeHTTPClient'、'now.Sub'、'repo.GetUserSettings'、'repo.ListAllExternalSubscriptions'、'repo.UpdateExternalSubscription'、'syncSingleExternalSubscription'、'time.Duration'、'time.Now' |

## `internal/handler/external_sync_selection_test.go`

依赖：`testing`、`time`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 10–34 | function | `TestStoreExternalSyncSelectionUsesOpaqueIDs` | 执行与 'test store external sync selection uses opaque i ds' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 0；goroutine 0；调用 'delete'、'externalSyncSelections.Lock'、'externalSyncSelections.Unlock'、'len'、'session.ExpiresAt.After'、'storeExternalSyncSelection'、't.Fatal'、't.Fatalf'、'time.Now' |

## `internal/handler/inject_relay_groups_test.go`

依赖：`context`、`path/filepath`、`testing`、`miaomiaowu/internal/storage`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 13–20 | function | `relayTestRepo` | 执行与 'relay test repo' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'filepath.Join'、'storage.NewTrafficRepository'、't.Fatalf'、't.Helper'、't.TempDir' |
| 22–29 | function | `mustNode` | 执行与 'must node' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'context.Background'、'repo.CreateNode'、't.Fatalf'、't.Helper' |
| 33–43 | function | `rootMappingFromYAML` | 执行与 'root mapping from yaml' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'len'、't.Fatal'、't.Fatalf'、't.Helper'、'yaml.Unmarshal' |
| 45–58 | function | `proxyNamesOf` | 执行与 'proxy names of' 对应的业务或基础设施操作。 | 分支 2；循环 2；返回 1；goroutine 0；调用 'len'、'make'、'yamlMapGet' |
| 60–81 | function | `findGroupProxies` | 查找与 'find group proxies' 对应的业务或基础设施操作。 | 分支 3；循环 4；返回 2；goroutine 0；调用 'append'、'len'、'yamlMapGet' |
| 87–153 | function | `TestInjectRelayGroups_BackfillsAndDropsDisabled` | 执行与 'test inject relay groups_ backfills and drops disabled' 对应的业务或基础设施操作。 | 分支 7；循环 2；返回 0；goroutine 0；调用 'context.Background'、'findGroupProxies'、'injectRelayGroups'、'len'、'mustNode'、'proxyNamesOf'、'relayTestRepo'、'rootMappingFromYAML'、't.Error'、't.Errorf'、't.Fatal'、'yamlMapGet' |
| 157–196 | function | `TestInjectRelayGroups_SourceNotInSubscription` | 执行与 'test inject relay groups_ source not in subscription' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'context.Background'、'findGroupProxies'、'injectRelayGroups'、'mustNode'、'proxyNamesOf'、'relayTestRepo'、'rootMappingFromYAML'、't.Error' |

## `internal/handler/ip_helpers.go`

依赖：`net`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 7–16 | function | `IsLocalOrPrivateIP` | 判断与 'is local or private ip' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'ip.IsLinkLocalUnicast'、'ip.IsLoopback'、'ip.IsPrivate'、'ip.IsUnspecified'、'net.ParseIP' |

## `internal/handler/node_enabled_default_test.go`

依赖：`context`、`net/http`、`net/http/httptest`、`strings`、`testing`、`miaomiaowu/internal/auth`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–21 | function | `postNodes` | 执行与 'post nodes' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'auth.ContextWithUsername'、'context.Background'、'h.ServeHTTP'、'httptest.NewRecorder'、'httptest.NewRequest'、'req.WithContext'、'strings.NewReader'、't.Helper' |
| 25–51 | function | `TestBatchCreate_DefaultsEnabled` | 执行与 'test batch create_ defaults enabled' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 0；goroutine 0；调用 'NewNodesHandler'、'context.Background'、'len'、'postNodes'、'rec.Body.String'、'relayTestRepo'、'repo.ListNodes'、't.Errorf'、't.Fatalf'、't.TempDir' |
| 54–81 | function | `TestCreate_DefaultsEnabled` | 执行与 'test create_ defaults enabled' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 0；goroutine 0；调用 'NewNodesHandler'、'context.Background'、'postNodes'、'rec.Body.String'、'rec2.Body.String'、'relayTestRepo'、'repo.ListNodes'、't.Error'、't.Fatalf'、't.TempDir' |

## `internal/handler/nodes.go`

依赖：`crypto/tls`、`encoding/json`、`errors`、`fmt`、`io`、`miaomiaowu/internal/logger`、`net/http`、`net/url`、`regexp`、`strconv`、`strings`、`time`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 24–40 | function | `convertNilToEmptyStringInMap` | 转换与 'convert nil to empty string in map' 对应的业务或基础设施操作。 | 分支 5；循环 2；返回 0；goroutine 0；调用 'convertNilToEmptyStringInMap' |
| 43–52 | function | `safeURLDecode` | 执行与 'safe url decode' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'url.QueryUnescape' |
| 56–109 | function | `decodeProxyURLFields` | 执行与 'decode proxy url fields' 对应的业务或基础设施操作。 | 分支 15；循环 1；返回 0；goroutine 0；调用 'safeURLDecode' |
| 111–130 | function | `applyNodeNameFilterToClashProxies` | 应用与 'apply node name filter to clash proxies' 对应的业务或基础设施操作。 | 分支 2；循环 2；返回 2；goroutine 0；调用 'append'、'applyNodeNameFilterToProxies'、'len'、'make' |
| 132–136 | type | `nodesHandler` | 定义 'nodesHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 139–149 | function | `NewNodesHandler` | 创建并初始化与 'new nodes handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'NewYAMLSyncManager'、'panic' |
| 151–194 | function | `(*nodesHandler).ServeHTTP` | *nodesHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'h.handleBatchCreate'、'h.handleClearAll'、'h.handleCreate'、'h.handleDelete'、'h.handleFetchSubscription'、'h.handleList'、'h.handleParseURIs'、'h.handleRestoreServer'、'h.handleUpdate'、'h.handleUpdateConfig'、'h.handleUpdateProbeBinding'、'h.handleUpdateServer'、'strings.HasSuffix'、'strings.Trim'、'strings.TrimPrefix'、'strings.TrimSuffix' |
| 196–212 | function | `(*nodesHandler).handleList` | *nodesHandler 的方法，处理与 'handle list' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'auth.UsernameFromContext'、'convertNodes'、'errors.New'、'h.repo.ListNodes'、'r.Context'、'respondJSON'、'writeError' |
| 214–303 | function | `(*nodesHandler).handleCreate` | *nodesHandler 的方法，处理与 'handle create' 对应的业务或基础设施操作。 | 分支 11；循环 0；返回 8；goroutine 0；调用 'Decode'、'auth.UsernameFromContext'、'errors.New'、'fmt.Sprintf'、'h.repo.CheckNodeNameExists'、'json.NewDecoder'、'json.Unmarshal'、'logger.Info'、'r.Context'、'req.parseChainProxyNodeID'、'req.parseEnabled'、'req.resolvedEnabled'、'string'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |
| 305–359 | function | `(*nodesHandler).handleBatchCreate` | *nodesHandler 的方法，处理与 'handle batch create' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 5；goroutine 0；调用 'Decode'、'append'、'auth.UsernameFromContext'、'convertNodes'、'errors.New'、'h.repo.BatchCreateNodes'、'json.NewDecoder'、'len'、'make'、'n.resolvedEnabled'、'r.Context'、'respondJSON'、'writeBadRequest'、'writeError' |
| 361–506 | function | `(*nodesHandler).handleUpdate` | *nodesHandler 的方法，处理与 'handle update' 对应的业务或基础设施操作。 | 分支 28；循环 0；返回 10；goroutine 0；调用 'Decode'、'auth.UsernameFromContext'、'errors.Is'、'errors.New'、'fmt.Sprintf'、'h.repo.CheckNodeNameExists'、'h.repo.GetNode'、'json.NewDecoder'、'logger.Info'、'r.Context'、'req.parseChainProxyNodeID'、'req.parseEnabled'、'strconv.ParseInt'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |
| 508–594 | function | `(*nodesHandler).handleUpdateServer` | *nodesHandler 的方法，处理与 'handle update server' 对应的业务或基础设施操作。 | 分支 17；循环 0；返回 6；goroutine 0；调用 'Decode'、'auth.UsernameFromContext'、'errors.Is'、'errors.New'、'h.repo.GetNode'、'h.repo.UpdateNode'、'h.yamlSyncManager.SyncNode'、'json.Marshal'、'json.NewDecoder'、'json.Unmarshal'、'r.Context'、'respondJSON'、'strconv.ParseInt'、'string'、'writeBadRequest'、'writeError' |
| 596–670 | function | `(*nodesHandler).handleRestoreServer` | *nodesHandler 的方法，处理与 'handle restore server' 对应的业务或基础设施操作。 | 分支 13；循环 0；返回 5；goroutine 0；调用 'auth.UsernameFromContext'、'convertNode'、'errors.Is'、'errors.New'、'h.repo.GetNode'、'h.repo.UpdateNode'、'h.yamlSyncManager.SyncNode'、'json.Marshal'、'json.Unmarshal'、'r.Context'、'respondJSON'、'strconv.ParseInt'、'string'、'writeBadRequest'、'writeError' |
| 672–752 | function | `(*nodesHandler).handleUpdateConfig` | *nodesHandler 的方法，处理与 'handle update config' 对应的业务或基础设施操作。 | 分支 12；循环 1；返回 7；goroutine 0；调用 'Decode'、'auth.UsernameFromContext'、'err.Error'、'errors.Is'、'errors.New'、'fmt.Sprintf'、'h.repo.GetNode'、'h.repo.UpdateNode'、'h.yamlSyncManager.SyncNode'、'json.NewDecoder'、'json.Unmarshal'、'r.Context'、'respondJSON'、'strconv.ParseInt'、'writeBadRequest'、'writeError' |
| 754–798 | function | `(*nodesHandler).handleDelete` | *nodesHandler 的方法，处理与 'handle delete' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 4；goroutine 1；调用 'RefreshAllTemplateSubscriptions'、'auth.UsernameFromContext'、'errors.Is'、'errors.New'、'h.repo.DeleteNode'、'h.repo.GetNode'、'h.yamlSyncManager.DeleteNode'、'r.Context'、'respondJSON'、'strconv.ParseInt'、'writeBadRequest'、'writeError' |
| 800–816 | function | `(*nodesHandler).handleClearAll` | *nodesHandler 的方法，处理与 'handle clear all' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 1；调用 'RefreshAllTemplateSubscriptions'、'auth.UsernameFromContext'、'errors.New'、'h.repo.DeleteAllUserNodes'、'r.Context'、'respondJSON'、'writeError' |
| 818–880 | function | `(*nodesHandler).handleBatchDelete` | *nodesHandler 的方法，处理与 'handle batch delete' 对应的业务或基础设施操作。 | 分支 9；循环 2；返回 3；goroutine 1；调用 'Decode'、'RefreshAllTemplateSubscriptions'、'append'、'auth.UsernameFromContext'、'errors.New'、'h.repo.DeleteNode'、'h.repo.GetNode'、'h.yamlSyncManager.BatchDeleteNodes'、'json.NewDecoder'、'len'、'make'、'r.Context'、'respondJSON'、'writeBadRequest'、'writeError' |
| 882–983 | function | `(*nodesHandler).handleBatchRename` | *nodesHandler 的方法，处理与 'handle batch rename' 对应的业务或基础设施操作。 | 分支 13；循环 1；返回 3；goroutine 0；调用 'Decode'、'append'、'auth.UsernameFromContext'、'convertNode'、'errors.New'、'h.repo.GetNode'、'h.repo.UpdateNode'、'h.yamlSyncManager.BatchSyncNodes'、'json.Marshal'、'json.NewDecoder'、'json.Unmarshal'、'len'、'r.Context'、'string'、'writeBadRequest'、'writeError' |
| 985–1031 | function | `(*nodesHandler).handleBatchDisableSkipCert` | *nodesHandler 的方法，处理与 'handle batch disable skip cert' 对应的业务或基础设施操作。 | 分支 7；循环 1；返回 2；goroutine 0；调用 'Decode'、'append'、'auth.UsernameFromContext'、'convertNode'、'disableSkipCertVerifyInJSON'、'errors.New'、'h.repo.GetNode'、'h.repo.UpdateNode'、'h.yamlSyncManager.BatchSyncNodes'、'json.NewDecoder'、'len'、'logger.Info'、'r.Context'、'respondJSON'、'writeBadRequest'、'writeError' |
| 1033–1048 | function | `disableSkipCertVerifyInJSON` | 执行与 'disable skip cert verify in json' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'isTruthySkipCert'、'json.Marshal'、'json.Unmarshal'、'string'、'strings.TrimSpace' |
| 1050–1059 | function | `isTruthySkipCert` | 判断与 'is truthy skip cert' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 3；goroutine 0；调用 'strings.EqualFold'、'strings.TrimSpace' |
| 1061–1075 | type | `nodeRequest` | 定义 'nodeRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 1079–1081 | function | `(*nodeRequest).hasEnabled` | *nodeRequest 的方法，判断是否具有与 'has enabled' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'string' |
| 1084–1088 | function | `(*nodeRequest).parseEnabled` | *nodeRequest 的方法，解析与 'parse enabled' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'json.Unmarshal'、'r.hasEnabled' |
| 1092–1101 | function | `(*nodeRequest).resolvedEnabled` | *nodeRequest 的方法，解析或求解与 'resolved enabled' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'json.Unmarshal'、'r.hasEnabled' |
| 1103–1105 | function | `(*nodeRequest).hasChainProxyNodeID` | *nodeRequest 的方法，判断是否具有与 'has chain proxy node id' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 1107–1116 | function | `(*nodeRequest).parseChainProxyNodeID` | *nodeRequest 的方法，解析与 'parse chain proxy node id' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'json.Unmarshal'、'string' |
| 1118–1135 | type | `nodeDTO` | 定义 'nodeDTO' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 1137–1164 | function | `convertNode` | 转换与 'convert node' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0 |
| 1166–1172 | function | `convertNodes` | 转换与 'convert nodes' 对应的业务或基础设施操作。 | 分支 0；循环 1；返回 1；goroutine 0；调用 'append'、'convertNode'、'len'、'make' |
| 1174–1446 | function | `(*nodesHandler).handleFetchSubscription` | *nodesHandler 的方法，处理与 'handle fetch subscription' 对应的业务或基础设施操作。 | 分支 37；循环 2；返回 12；goroutine 0；调用 'Decode'、'auth.UsernameFromContext'、'client.Do'、'err.Error'、'errors.New'、'h.repo.GetUserSettings'、'http.NewRequest'、'httpReq.Header.Set'、'json.NewDecoder'、'logger.Info'、'r.Context'、'regexp.Compile'、'resp.Body.Close'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |
| 1451–1487 | function | `(*nodesHandler).handleParseURIs` | *nodesHandler 的方法，处理与 'handle parse ur is' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 4；goroutine 0；调用 'Decode'、'ParseV2raySubscription'、'auth.UsernameFromContext'、'convertNilToEmptyStringInMap'、'decodeProxyURLFields'、'err.Error'、'errors.New'、'json.NewDecoder'、'len'、'r.Context'、'respondJSON'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |
| 1490–1529 | function | `(*nodesHandler).handleUpdateProbeBinding` | *nodesHandler 的方法，处理与 'handle update probe binding' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'Decode'、'auth.UsernameFromContext'、'convertNode'、'errors.Is'、'errors.New'、'h.repo.GetNode'、'h.repo.UpdateNodeProbeServer'、'json.NewDecoder'、'r.Context'、'respondJSON'、'strconv.ParseInt'、'writeBadRequest'、'writeError' |

## `internal/handler/nodes_skipcert_test.go`

依赖：`testing`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–17 | function | `TestDisableSkipCertVerifyInJSON` | 执行与 'test disable skip cert verify in json' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 0；goroutine 0；调用 'disableSkipCertVerifyInJSON'、't.Fatal' |

## `internal/handler/notify_config.go`

依赖：`encoding/json`、`errors`、`net/http`、`strings`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/notify`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–25 | type | `notifyConfigResponse` | 定义 'notifyConfigResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 27–38 | type | `notifyConfigRequest` | 定义 'notifyConfigRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 41–64 | function | `NewNotifyConfigHandler` | 创建并初始化与 'new notify config handler' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'handleGetNotifyConfig'、'handleNotifyTest'、'handleUpdateNotifyConfig'、'http.HandlerFunc'、'r.Context'、'strings.HasSuffix'、'strings.TrimSpace'、'writeError' |
| 42–63 | closure | `NewNotifyConfigHandler.closure#1` | 供 NewNotifyConfigHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 3；循环 0；返回 2；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'handleGetNotifyConfig'、'handleNotifyTest'、'handleUpdateNotifyConfig'、'r.Context'、'strings.HasSuffix'、'strings.TrimSpace'、'writeError' |
| 66–94 | function | `handleGetNotifyConfig` | 处理与 'handle get notify config' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'Encode'、'Set'、'json.NewEncoder'、'len'、'r.Context'、'repo.GetSystemConfig'、'strings.Repeat'、'w.Header'、'writeError' |
| 96–149 | function | `handleUpdateNotifyConfig` | 处理与 'handle update notify config' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 3；goroutine 0；调用 'Decode'、'Encode'、'GetNotifier'、'Set'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'n.UpdateConfig'、'r.Context'、'repo.GetSystemConfig'、'repo.UpdateSystemConfig'、'strings.Contains'、'w.Header'、'writeError' |
| 151–165 | function | `handleNotifyTest` | 处理与 'handle notify test' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'Encode'、'GetNotifier'、'Set'、'errors.New'、'json.NewEncoder'、'n.SendTest'、'r.Context'、'w.Header'、'writeError' |

## `internal/handler/notify_global.go`

依赖：`miaomiaowu/internal/notify`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–5 | var | `globalNotifier` | 保存 'globalNotifier' 的包级共享状态、配置或预计算值。 |  |
| 8–10 | function | `InitNotifier` | 执行与 'init notifier' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'notify.New' |
| 13–15 | function | `GetNotifier` | 查询或读取与 'get notifier' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |

## `internal/handler/notify_scheduler.go`

依赖：`context`、`fmt`、`strings`、`time`、`miaomiaowu/internal/logger`、`miaomiaowu/internal/notify`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–48 | function | `StartNotifyScheduler` | 启动与 'start notify scheduler' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 1；goroutine 2；调用 'GetNotifier'、'ctx.Done'、'n.GetConfig'、'now.Format'、'sendDailyTrafficNotification'、'sendExpiryNotification'、'ticker.Stop'、'time.NewTicker' |
| 50–92 | function | `sendDailyTrafficNotification` | 执行与 'send daily traffic notification' 对应的业务或基础设施操作。 | 分支 6；循环 2；返回 2；goroutine 0；调用 'b.String'、'b.WriteString'、'fmt.Fprintf'、'len'、'logger.Warn'、'n.Send'、'th.FetchTrafficSummaryForNotify' |
| 94–125 | function | `sendExpiryNotification` | 执行与 'send expiry notification' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 2；goroutine 0；调用 'Hours'、'append'、'f.ExpireAt.After'、'f.ExpireAt.Before'、'f.ExpireAt.Sub'、'fmt.Sprintf'、'int'、'len'、'logger.Warn'、'n.Send'、'now.Add'、'repo.ListSubscribeFiles'、'strings.Join'、'time.Now' |

## `internal/handler/operation_audit.go`

依赖：`context`、`net/http`、`strings`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–15 | type | `auditResponseWriter` | 定义 'auditResponseWriter' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 17–20 | function | `(*auditResponseWriter).WriteHeader` | *auditResponseWriter 的方法，执行与 'write header' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'w.ResponseWriter.WriteHeader' |
| 22–34 | function | `OperationAuditMiddleware` | 执行与 'operation audit middleware' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'GetClientIP'、'context.Background'、'http.HandlerFunc'、'next.ServeHTTP'、'r.Header.Get'、'repo.InsertOperationLog'、'strings.HasPrefix'、'strings.TrimSpace'、'tokens.Lookup' |
| 23–33 | closure | `OperationAuditMiddleware.closure#1` | 供 OperationAuditMiddleware 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'GetClientIP'、'context.Background'、'next.ServeHTTP'、'r.Header.Get'、'repo.InsertOperationLog'、'strings.HasPrefix'、'strings.TrimSpace'、'tokens.Lookup' |

## `internal/handler/operation_logs.go`

依赖：`net/http`、`strconv`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 10–28 | function | `NewOperationLogHandler` | 创建并初始化与 'new operation log handler' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'Get'、'http.HandlerFunc'、'methodNotAllowed'、'r.Context'、'r.URL.Query'、'repo.ListOperationLogs'、'respondJSON'、'strconv.Atoi'、'writeError' |
| 11–27 | closure | `NewOperationLogHandler.closure#1` | 供 NewOperationLogHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 3；循环 0；返回 2；goroutine 0；调用 'Get'、'methodNotAllowed'、'r.Context'、'r.URL.Query'、'repo.ListOperationLogs'、'respondJSON'、'strconv.Atoi'、'writeError' |

## `internal/handler/override_script_test.go`

依赖：`context`、`strings`、`testing`、`miaomiaowu/internal/scriptengine`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 15–60 | function | `TestRunPostFetchScript_ModifiesDNS` | 执行与 'test run post fetch script_ modifies dns' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 0；goroutine 0；调用 'context.Background'、'h.runPostFetchScript'、'len'、't.Errorf'、't.Fatalf'、'yaml.Unmarshal' |
| 63–108 | function | `TestRunPostFetchScript_FilterProxies` | 执行与 'test run post fetch script_ filter proxies' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 0；goroutine 0；调用 'context.Background'、'h.runPostFetchScript'、'len'、't.Fatalf'、'yaml.Unmarshal' |
| 111–156 | function | `TestRunPostFetchScript_AddRules` | 执行与 'test run post fetch script_ add rules' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 0；goroutine 0；调用 'context.Background'、'h.runPostFetchScript'、'len'、't.Errorf'、't.Fatalf'、'yaml.Unmarshal' |
| 160–217 | function | `TestRunPostFetchScript_ClashConfigModified` | 执行与 'test run post fetch script_ clash config modified' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 0；goroutine 0；调用 'context.Background'、'h.runPostFetchScript'、't.Errorf'、't.Fatalf'、'yaml.Unmarshal' |
| 220–236 | function | `TestRunPostFetchScript_ScriptError` | 执行与 'test run post fetch script_ script error' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'context.Background'、'err.Error'、'h.runPostFetchScript'、'strings.Contains'、't.Errorf'、't.Fatal' |
| 239–246 | function | `TestRunPostFetchScript_InvalidYAML` | 执行与 'test run post fetch script_ invalid yaml' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'context.Background'、'h.runPostFetchScript'、't.Fatal' |
| 250–322 | function | `TestPreSaveNodes_ModifiesNodes` | 执行与 'test pre save nodes_ modifies nodes' 对应的业务或基础设施操作。 | 分支 11；循环 0；返回 0；goroutine 0；调用 'context.Background'、'len'、'proxiesToYamlNode'、'scriptengine.RunPreSaveNodes'、'string'、'strings.Contains'、't.Errorf'、't.Fatalf'、'yaml.Marshal'、'yaml.Unmarshal'、'yamlNodeToProxies' |
| 325–384 | function | `TestPreSaveNodes_FilterNodes` | 执行与 'test pre save nodes_ filter nodes' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 0；goroutine 0；调用 'context.Background'、'len'、'proxiesToYamlNode'、'scriptengine.RunPreSaveNodes'、'string'、'strings.Contains'、't.Errorf'、't.Fatalf'、'yaml.Marshal'、'yaml.Unmarshal'、'yamlNodeToProxies' |
| 387–425 | function | `TestPreSaveNodes_ModifyServerAndPort` | 执行与 'test pre save nodes_ modify server and port' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 0；goroutine 0；调用 'context.Background'、'scriptengine.RunPreSaveNodes'、't.Errorf'、't.Fatalf'、'yaml.Unmarshal'、'yamlNodeToProxies' |
| 428–442 | function | `TestPreSaveNodes_ScriptErrorSkips` | 执行与 'test pre save nodes_ script error skips' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'context.Background'、'err.Error'、'scriptengine.RunPreSaveNodes'、'strings.Contains'、't.Errorf'、't.Fatal' |
| 445–504 | function | `TestPreSaveNodes_RoundTrip` | 执行与 'test pre save nodes_ round trip' 对应的业务或基础设施操作。 | 分支 5；循环 2；返回 0；goroutine 0；调用 'context.Background'、'len'、'proxiesToYamlNode'、'scriptengine.RunPreSaveNodes'、'string'、'strings.Contains'、't.Errorf'、't.Fatalf'、'yaml.Marshal'、'yaml.Unmarshal'、'yamlNodeToProxies' |

## `internal/handler/override_scripts.go`

依赖：`encoding/json`、`errors`、`net/http`、`strconv`、`strings`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–20 | type | `overrideScriptRequest` | 定义 'overrideScriptRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 22–31 | type | `overrideScriptResponse` | 定义 'overrideScriptResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 33–196 | function | `NewOverrideScriptsHandler` | 创建并初始化与 'new override scripts handler' 对应的业务或基础设施操作。 | 分支 24；循环 1；返回 19；goroutine 0；调用 'Set'、'auth.UsernameFromContext'、'errors.New'、'http.HandlerFunc'、'len'、'panic'、'r.Context'、'repo.GetOverrideScript'、'repo.GetUser'、'strconv.ParseInt'、'strings.Split'、'strings.Trim'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 38–195 | closure | `NewOverrideScriptsHandler.closure#1` | 供 NewOverrideScriptsHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 23；循环 1；返回 18；goroutine 0；调用 'Encode'、'Set'、'auth.UsernameFromContext'、'errors.New'、'json.NewEncoder'、'len'、'r.Context'、'repo.GetOverrideScript'、'repo.GetUser'、'strconv.ParseInt'、'strings.Split'、'strings.Trim'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 198–209 | function | `toOverrideScriptResponse` | 执行与 'to override script response' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 's.CreatedAt.Format'、's.UpdatedAt.Format' |

## `internal/handler/password.go`

依赖：`encoding/json`、`errors`、`net/http`、`strings`、`miaomiaowu/internal/auth`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–15 | type | `changePasswordRequest` | 定义 'changePasswordRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 17–62 | function | `NewPasswordHandler` | 创建并初始化与 'new password handler' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 7；goroutine 0；调用 'Decode'、'Encode'、'Set'、'auth.UsernameFromContext'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'json.NewEncoder'、'len'、'manager.ChangePassword'、'panic'、'r.Context'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 22–61 | closure | `NewPasswordHandler.closure#1` | 供 NewPasswordHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'Decode'、'Encode'、'Set'、'auth.UsernameFromContext'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'len'、'manager.ChangePassword'、'r.Context'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |

## `internal/handler/probe_admin.go`

依赖：`encoding/json`、`errors`、`math`、`net/http`、`strconv`、`strings`、`time`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 15–17 | type | `probeConfigHandler` | 定义 'probeConfigHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 19–27 | type | `probeServerPayload` | 定义 'probeServerPayload' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 29–35 | type | `probeConfigPayload` | 定义 'probeConfigPayload' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 37–46 | type | `probeConfigUpdateRequest` | 定义 'probeConfigUpdateRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 48–54 | function | `NewProbeConfigHandler` | 创建并初始化与 'new probe config handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'panic' |
| 56–67 | function | `(*probeConfigHandler).ServeHTTP` | *probeConfigHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'h.handleDelete'、'h.handleGet'、'h.handleUpdate'、'methodNotAllowed' |
| 69–90 | function | `(*probeConfigHandler).handleGet` | *probeConfigHandler 的方法，处理与 'handle get' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'convertProbeConfigResponse'、'errors.Is'、'h.repo.GetProbeConfig'、'r.Context'、'respondJSON'、'writeError' |
| 92–192 | function | `(*probeConfigHandler).handleUpdate` | *probeConfigHandler 的方法，处理与 'handle update' 对应的业务或基础设施操作。 | 分支 10；循环 3；返回 9；goroutine 0；调用 'Decode'、'append'、'formatServerError'、'getAllowedProbeTypes'、'getAllowedTrafficMethods'、'h.repo.UpsertProbeConfig'、'http.MaxBytesReader'、'int64'、'json.NewDecoder'、'len'、'make'、'math.Round'、'r.Context'、'strings.ToLower'、'strings.TrimSpace'、'writeBadRequest' |
| 194–206 | function | `(*probeConfigHandler).handleDelete` | *probeConfigHandler 的方法，处理与 'handle delete' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'h.repo.ClearAllStatsServerIDs'、'h.repo.DeleteProbeConfig'、'r.Context'、'respondJSON'、'writeError' |
| 208–231 | function | `convertProbeConfigResponse` | 转换与 'convert probe config response' 对应的业务或基础设施操作。 | 分支 0；循环 1；返回 1；goroutine 0；调用 'append'、'float64'、'len'、'make'、'math.Round' |
| 233–235 | function | `formatServerError` | 执行与 'format server error' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'strconv.Itoa' |
| 237–244 | function | `getAllowedProbeTypes` | 查询或读取与 'get allowed probe types' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 246–252 | function | `getAllowedTrafficMethods` | 查询或读取与 'get allowed traffic methods' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |

## `internal/handler/probe_sync.go`

依赖：`bytes`、`context`、`encoding/json`、`errors`、`fmt`、`io`、`miaomiaowu/internal/logger`、`math`、`net/http`、`net/url`、`strconv`、`strings`、`time`、`github.com/gorilla/websocket`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 23–26 | type | `probeSyncHandler` | 定义 'probeSyncHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 28–31 | type | `probeSyncRequest` | 定义 'probeSyncRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 33–38 | type | `probeSyncServer` | 定义 'probeSyncServer' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 40–42 | type | `probeSyncResponse` | 定义 'probeSyncResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 44–51 | function | `NewProbeSyncHandler` | 创建并初始化与 'new probe sync handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'panic' |
| 53–102 | function | `(*probeSyncHandler).ServeHTTP` | *probeSyncHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'Decode'、'h.fetchDstatusServers'、'h.fetchKomariServers'、'h.fetchNezhaServers'、'h.fetchNezhaV0Servers'、'http.MaxBytesReader'、'json.NewDecoder'、'len'、'logger.Info'、'methodNotAllowed'、'r.Context'、'strings.ToLower'、'strings.TrimRight'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |
| 104–251 | function | `(*probeSyncHandler).fetchNezhaServers` | *probeSyncHandler 的方法，从外部获取与 'fetch nezha servers' 对应的业务或基础设施操作。 | 分支 19；循环 1；返回 10；goroutine 0；调用 'base.ResolveReference'、'cancel'、'conn.Close'、'conn.SetReadDeadline'、'context.WithTimeout'、'fmt.Errorf'、'fmt.Sprintf'、'io.ReadAll'、'logger.Info'、'resp.Body.Close'、'string'、'strings.ToLower'、'strings.TrimSpace'、'target.String'、'url.Parse'、'websocket.DefaultDialer.DialContext' |
| 253–403 | function | `(*probeSyncHandler).fetchDstatusServers` | *probeSyncHandler 的方法，从外部获取与 'fetch dstatus servers' 对应的业务或基础设施操作。 | 分支 16；循环 3；返回 7；goroutine 0；调用 'base.ResolveReference'、'bytes.NewReader'、'decoder.UseNumber'、'fmt.Errorf'、'h.client.Do'、'http.NewRequestWithContext'、'io.ReadAll'、'json.NewDecoder'、'len'、'logger.Info'、'resp.Body.Close'、'resp.Header.Get'、'string'、'strings.TrimSpace'、'target.String'、'url.Parse' |
| 405–524 | function | `(*probeSyncHandler).fetchNezhaV0Servers` | *probeSyncHandler 的方法，从外部获取与 'fetch nezha v0 servers' 对应的业务或基础设施操作。 | 分支 17；循环 1；返回 7；goroutine 0；调用 'base.ResolveReference'、'bytes.NewReader'、'decoder.Decode'、'decoder.UseNumber'、'fmt.Errorf'、'h.client.Do'、'http.NewRequestWithContext'、'io.ReadAll'、'json.NewDecoder'、'len'、'logger.Info'、'resp.Body.Close'、'string'、'strings.TrimSpace'、'target.String'、'url.Parse' |
| 526–614 | function | `(*probeSyncHandler).fetchKomariServers` | *probeSyncHandler 的方法，从外部获取与 'fetch komari servers' 对应的业务或基础设施操作。 | 分支 9；循环 1；返回 7；goroutine 0；调用 'base.ResolveReference'、'bytes.NewReader'、'decoder.UseNumber'、'fmt.Errorf'、'h.client.Do'、'http.NewRequestWithContext'、'io.ReadAll'、'json.NewDecoder'、'len'、'logger.Info'、'resp.Body.Close'、'resp.Header.Get'、'string'、'strings.TrimSpace'、'target.String'、'url.Parse' |
| 616–753 | function | `(*probeSyncHandler).fetchNezhaV0ServersViaWebSocket` | *probeSyncHandler 的方法，从外部获取与 'fetch nezha v0 servers via web socket' 对应的业务或基础设施操作。 | 分支 18；循环 1；返回 9；goroutine 0；调用 'Add'、'base.ResolveReference'、'bytes.TrimSpace'、'cancel'、'conn.Close'、'conn.ReadMessage'、'conn.SetReadDeadline'、'context.WithTimeout'、'fmt.Errorf'、'len'、'logger.Info'、'resp.Body.Close'、'strings.ToLower'、'target.String'、'time.Now'、'websocket.DefaultDialer.DialContext' |

## `internal/handler/profile.go`

依赖：`encoding/json`、`errors`、`net/http`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–19 | type | `profileResponse` | 定义 'profileResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 21–21 | var | `errUnauthorized` | 保存 'errUnauthorized' 的包级共享状态、配置或预计算值。 |  |
| 23–53 | function | `NewProfileHandler` | 创建并初始化与 'new profile handler' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'Encode'、'Set'、'auth.UsernameFromContext'、'http.HandlerFunc'、'json.NewEncoder'、'panic'、'r.Context'、'repo.GetUser'、'w.Header'、'writeError' |
| 28–52 | closure | `NewProfileHandler.closure#1` | 供 NewProfileHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'Encode'、'Set'、'auth.UsernameFromContext'、'json.NewEncoder'、'r.Context'、'repo.GetUser'、'w.Header'、'writeError' |

## `internal/handler/proxy_groups.go`

依赖：`encoding/json`、`errors`、`fmt`、`io`、`net/http`、`strings`、`time`、`miaomiaowu/internal/proxygroups`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 18–20 | type | `proxyGroupsHandler` | 定义 'proxyGroupsHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 23–28 | function | `NewProxyGroupsHandler` | 创建并初始化与 'new proxy groups handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'panic' |
| 30–51 | function | `(*proxyGroupsHandler).ServeHTTP` | *proxyGroupsHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 1；goroutine 0；调用 'Set'、'h.store.Snapshot'、'http.Error'、'syncedAt.Format'、'syncedAt.IsZero'、'w.Header'、'w.Write' |
| 54–57 | type | `proxyGroupsSyncHandler` | 定义 'proxyGroupsSyncHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 59–61 | type | `proxyGroupsSyncRequest` | 定义 'proxyGroupsSyncRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 63–67 | type | `proxyGroupsSyncResponse` | 定义 'proxyGroupsSyncResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 71–79 | function | `NewProxyGroupsSyncHandler` | 创建并初始化与 'new proxy groups sync handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'panic' |
| 81–132 | function | `(*proxyGroupsSyncHandler).ServeHTTP` | *proxyGroupsSyncHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 5；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.Is'、'fmt.Errorf'、'h.repo.GetSystemConfig'、'h.store.Update'、'http.Error'、'json.NewDecoder'、'json.NewEncoder'、'proxygroups.FetchConfig'、'r.Context'、'strings.TrimSpace'、'time.Now'、'w.Header'、'writeError' |

## `internal/handler/proxy_parser.go`

依赖：`github.com/MMWOrg/mmwX-plugins/proxyparser`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 9–11 | function | `ParseProxyURL` | 解析与 'parse proxy url' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'proxyparser.Parse' |
| 14–16 | function | `ParseV2raySubscription` | 解析与 'parse v2ray subscription' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'proxyparser.ParseSubscription' |

## `internal/handler/proxy_parser_test.go`

依赖：`encoding/base64`、`net/url`、`strings`、`testing`、`github.com/MMWOrg/mmwX-plugins/proxyparser/substore`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–45 | function | `TestPinnedPeerCertSha256V2RayRoundTrip` | 执行与 'test pinned peer cert sha256v2 ray round trip' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 0；goroutine 0；调用 'Get'、'GetProducer'、'ParseProxyURL'、'base64.StdEncoding.DecodeString'、'producer.Produce'、'string'、'strings.TrimSpace'、'substore.GetDefaultFactory'、't.Fatalf'、'uri.Query'、'url.Parse' |
| 47–71 | function | `TestAnyTLSStashAndQuantumultXOutput` | 执行与 'test any tls stash and quantumult x output' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 0；goroutine 0；调用 'GetProducer'、'producer.Produce'、'strings.Contains'、'substore.GetDefaultFactory'、't.Fatalf' |

## `internal/handler/proxy_provider.go`

依赖：`bytes`、`context`、`encoding/json`、`errors`、`fmt`、`miaomiaowu/internal/logger`、`net/http`、`os`、`strconv`、`strings`、`time`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`、`miaomiaowu/internal/util`、`miaomiaowu/internal/validator`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 24–47 | type | `proxyProviderConfigRequest` | 定义 'proxyProviderConfigRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 49–72 | type | `proxyProviderConfigResponse` | 定义 'proxyProviderConfigResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 74–99 | function | `NewProxyProviderConfigsHandler` | 创建并初始化与 'new proxy provider configs handler' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 2；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'handleCreateProxyProviderConfig'、'handleDeleteProxyProviderConfig'、'handleListProxyProviderConfigs'、'handleUpdateProxyProviderConfig'、'http.HandlerFunc'、'panic'、'r.Context'、'strings.TrimSpace'、'writeError' |
| 79–98 | closure | `NewProxyProviderConfigsHandler.closure#1` | 供 NewProxyProviderConfigsHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'handleCreateProxyProviderConfig'、'handleDeleteProxyProviderConfig'、'handleListProxyProviderConfigs'、'handleUpdateProxyProviderConfig'、'r.Context'、'strings.TrimSpace'、'writeError' |
| 101–132 | function | `handleListProxyProviderConfigs` | 处理与 'handle list proxy provider configs' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 2；goroutine 0；调用 'Encode'、'Get'、'Set'、'append'、'errors.New'、'len'、'make'、'r.Context'、'r.URL.Query'、'repo.ListProxyProviderConfigs'、'repo.ListProxyProviderConfigsBySubscription'、'strconv.ParseInt'、'toProxyProviderConfigResponse'、'w.Header'、'w.WriteHeader'、'writeError' |
| 134–233 | function | `handleCreateProxyProviderConfig` | 处理与 'handle create proxy provider config' 对应的业务或基础设施操作。 | 分支 14；循环 0；返回 6；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.CreateProxyProviderConfig'、'repo.GetExternalSubscription'、'strings.TrimSpace'、'time.Now'、'toProxyProviderConfigResponse'、'w.Header'、'w.WriteHeader'、'writeError' |
| 235–346 | function | `handleUpdateProxyProviderConfig` | 处理与 'handle update proxy provider config' 对应的业务或基础设施操作。 | 分支 16；循环 0；返回 7；goroutine 1；调用 'Decode'、'Get'、'Set'、'errors.New'、'json.NewDecoder'、'logger.Info'、'r.Context'、'r.URL.Query'、'repo.GetProxyProviderConfig'、'repo.UpdateProxyProviderConfig'、'strconv.ParseInt'、'strings.TrimSpace'、'syncProxyProviderModeChange'、'time.Now'、'w.Header'、'writeError' |
| 348–378 | function | `handleDeleteProxyProviderConfig` | 处理与 'handle delete proxy provider config' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 4；goroutine 0；调用 'Delete'、'Get'、'GetProxyProviderCache'、'err.Error'、'errors.New'、'logger.Info'、'r.Context'、'r.URL.Query'、'repo.DeleteProxyProviderConfig'、'repo.GetProxyProviderConfig'、'strconv.ParseInt'、'w.WriteHeader'、'writeError' |
| 380–405 | function | `toProxyProviderConfigResponse` | 执行与 'to proxy provider config response' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'config.CreatedAt.Format'、'config.UpdatedAt.Format' |
| 408–488 | function | `NewProxyProviderCacheRefreshHandler` | 创建并初始化与 'new proxy provider cache refresh handler' 对应的业务或基础设施操作。 | 分支 11；循环 0；返回 11；goroutine 0；调用 'Encode'、'Get'、'Set'、'auth.UsernameFromContext'、'errors.New'、'http.HandlerFunc'、'json.NewEncoder'、'panic'、'r.Context'、'r.URL.Query'、'repo.GetProxyProviderConfig'、'strconv.ParseInt'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 413–487 | closure | `NewProxyProviderCacheRefreshHandler.closure#1` | 供 NewProxyProviderCacheRefreshHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 10；循环 0；返回 10；goroutine 0；调用 'Encode'、'Get'、'RefreshProxyProviderCache'、'Set'、'auth.UsernameFromContext'、'errors.New'、'json.NewEncoder'、'r.Context'、'r.URL.Query'、'repo.GetExternalSubscription'、'repo.GetProxyProviderConfig'、'strconv.ParseInt'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 491–574 | function | `NewProxyProviderNodesHandler` | 创建并初始化与 'new proxy provider nodes handler' 对应的业务或基础设施操作。 | 分支 12；循环 0；返回 10；goroutine 0；调用 'Encode'、'Get'、'Set'、'auth.UsernameFromContext'、'errors.New'、'http.HandlerFunc'、'json.NewEncoder'、'panic'、'r.Context'、'r.URL.Query'、'repo.GetProxyProviderConfig'、'strconv.ParseInt'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 496–573 | closure | `NewProxyProviderNodesHandler.closure#1` | 供 NewProxyProviderNodesHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 11；循环 0；返回 9；goroutine 0；调用 'Encode'、'Get'、'GetProxyProviderCache'、'Set'、'auth.UsernameFromContext'、'cache.Get'、'errors.New'、'json.NewEncoder'、'r.Context'、'r.URL.Query'、'repo.GetProxyProviderConfig'、'strconv.ParseInt'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 577–615 | function | `NewProxyProviderCacheStatusHandler` | 创建并初始化与 'new proxy provider cache status handler' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 4；goroutine 0；调用 'Encode'、'GetProxyProviderCache'、'Set'、'auth.UsernameFromContext'、'cache.GetCacheStatus'、'errors.New'、'http.HandlerFunc'、'make'、'panic'、'r.Context'、'repo.ListProxyProviderConfigs'、'strconv.FormatInt'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 582–614 | closure | `NewProxyProviderCacheStatusHandler.closure#1` | 供 NewProxyProviderCacheStatusHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 4；循环 1；返回 3；goroutine 0；调用 'Encode'、'GetProxyProviderCache'、'Set'、'auth.UsernameFromContext'、'cache.GetCacheStatus'、'errors.New'、'json.NewEncoder'、'make'、'r.Context'、'repo.ListProxyProviderConfigs'、'strconv.FormatInt'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader'、'writeError' |
| 621–773 | function | `syncProxyProviderModeChange` | 同步与 'sync proxy provider mode change' 对应的业务或基础设施操作。 | 分支 23；循环 4；返回 1；goroutine 0；调用 'GetProxyProviderCache'、'append'、'cache.Get'、'context.Background'、'fileUsesProxyProvider'、'fmt.Sprintf'、'logger.Info'、'os.IsNotExist'、'os.ReadFile'、'os.Stat'、'repo.ListSubscribeFiles'、'strings.Index'、'syncClientToMMW'、'syncMMWToClient'、'yaml.NewEncoder'、'yaml.Unmarshal' |
| 776–853 | function | `fileUsesProxyProvider` | 执行与 'file uses proxy provider' 对应的业务或基础设施操作。 | 分支 14；循环 6；返回 7；goroutine 0；调用 'len'、'util.GetNodeFieldValue' |
| 861–893 | function | `syncClientToMMW` | 同步与 'sync client to mmw' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 4；goroutine 0；调用 'RefreshProxyProviderCache'、'append'、'copyMapForProvider'、'fmt.Errorf'、'len'、'make'、'repo.GetExternalSubscription'、'updateYAMLNodeForMMW' |
| 900–1101 | function | `syncMMWToClient` | 同步与 'sync mmw to client' 对应的业务或基础设施操作。 | 分支 28；循环 11；返回 4；goroutine 0；调用 'append'、'createProxyProviderYAMLNode'、'len'、'logger.Info'、'make'、'util.GetNodeFieldValue' |
| 1104–1364 | function | `updateYAMLNodeForMMW` | 更新与 'update yaml node for mmw' 对应的业务或基础设施操作。 | 分支 31；循环 15；返回 5；goroutine 0；调用 'append'、'len'、'logger.Info'、'make'、'strings.HasPrefix'、'util.GetNodeFieldValue'、'util.ReorderProxyFieldsToNode' |
| 1367–1412 | function | `createProxyProviderYAMLNode` | 创建与 'create proxy provider yaml node' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'append'、'fmt.Sprintf'、'strconv.Itoa' |
| 1415–1430 | function | `copyMapForProvider` | 执行与 'copy map for provider' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 1；goroutine 0；调用 'copy'、'copyMapForProvider'、'len'、'make' |

## `internal/handler/proxy_provider_cache.go`

依赖：`context`、`miaomiaowu/internal/logger`、`sync`、`time`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 13–22 | type | `CacheEntry` | 定义 'CacheEntry' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 25–28 | type | `ProxyProviderCache` | 定义 'ProxyProviderCache' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 31–33 | var | `proxyProviderCache` | 保存 'proxyProviderCache' 的包级共享状态、配置或预计算值。 |  |
| 36–38 | function | `GetProxyProviderCache` | 查询或读取与 'get proxy provider cache' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 41–46 | function | `(*ProxyProviderCache).Get` | *ProxyProviderCache 的方法，查询或读取与 'get' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'c.mu.RLock'、'c.mu.RUnlock' |
| 49–54 | function | `(*ProxyProviderCache).Set` | *ProxyProviderCache 的方法，设置与 'set' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'c.mu.Lock'、'c.mu.Unlock'、'logger.Info' |
| 58–64 | function | `(*ProxyProviderCache).UpdateInterval` | *ProxyProviderCache 的方法，更新与 'update interval' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'c.mu.Lock'、'c.mu.Unlock' |
| 67–72 | function | `(*ProxyProviderCache).Delete` | *ProxyProviderCache 的方法，删除与 'delete' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'c.mu.Lock'、'c.mu.Unlock'、'delete'、'logger.Info' |
| 75–84 | function | `(*ProxyProviderCache).IsExpired` | *ProxyProviderCache 的方法，判断与 'is expired' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'time.Duration'、'time.Since' |
| 87–107 | function | `(*ProxyProviderCache).GetCacheStatus` | *ProxyProviderCache 的方法，查询或读取与 'get cache status' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'c.IsExpired'、'c.mu.RLock'、'c.mu.RUnlock'、'entry.FetchedAt.Format' |
| 110–125 | function | `(*ProxyProviderCache).GetAllCacheStatus` | *ProxyProviderCache 的方法，查询或读取与 'get all cache status' 对应的业务或基础设施操作。 | 分支 0；循环 1；返回 1；goroutine 0；调用 'c.IsExpired'、'c.mu.RLock'、'c.mu.RUnlock'、'entry.FetchedAt.Format'、'make' |
| 128–133 | function | `(*ProxyProviderCache).Clear` | *ProxyProviderCache 的方法，执行与 'clear' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'c.mu.Lock'、'c.mu.Unlock'、'logger.Info'、'make' |
| 136–187 | function | `InitProxyProviderCacheOnStartup` | 执行与 'init proxy provider cache on startup' 对应的业务或基础设施操作。 | 分支 6；循环 2；返回 2；goroutine 0；调用 'RefreshProxyProviderCache'、'context.Background'、'logger.Info'、'repo.GetExternalSubscription'、'repo.ListProxyProviderConfigs'、'repo.ListUsers' |
| 192–192 | const | `proxyProviderScanInterval` | 定义 'proxyProviderScanInterval' 的不可变协议值、默认值或枚举成员。 |  |
| 195–195 | const | `proxyProviderReloadInterval` | 定义 'proxyProviderReloadInterval' 的不可变协议值、默认值或枚举成员。 |  |
| 197–197 | const | `proxyProviderRetryBase` | 定义 'proxyProviderRetryBase' 的不可变协议值、默认值或枚举成员。 |  |
| 199–199 | const | `proxyProviderRetryMax` | 定义 'proxyProviderRetryMax' 的不可变协议值、默认值或枚举成员。 |  |
| 201–201 | const | `proxyProviderWorkerLimit` | 定义 'proxyProviderWorkerLimit' 的不可变协议值、默认值或枚举成员。 |  |
| 203–203 | const | `defaultProxyInterval` | 定义 'defaultProxyInterval' 的不可变协议值、默认值或枚举成员。 |  |
| 205–205 | const | `nodeLogPreviewSize` | 定义 'nodeLogPreviewSize' 的不可变协议值、默认值或枚举成员。 |  |
| 207–207 | const | `refreshOperationTimeout` | 定义 'refreshOperationTimeout' 的不可变协议值、默认值或枚举成员。 |  |
| 209–209 | const | `configLoadTimeout` | 定义 'configLoadTimeout' 的不可变协议值、默认值或枚举成员。 |  |
| 213–218 | type | `proxyProviderSyncState` | 定义 'proxyProviderSyncState' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 221–226 | type | `scheduledProxyConfig` | 定义 'scheduledProxyConfig' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 230–233 | type | `dbSubscriptionCacheEntry` | 定义 'dbSubscriptionCacheEntry' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 237–250 | type | `proxyProviderCacheSyncer` | 定义 'proxyProviderCacheSyncer' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 254–261 | function | `StartProxyProviderCacheSync` | 启动与 'start proxy provider cache sync' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'newProxyProviderCacheSyncer'、'syncer.run' |
| 264–274 | function | `newProxyProviderCacheSyncer` | 创建并初始化与 'new proxy provider cache syncer' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'GetProxyProviderCache'、'make' |
| 277–307 | function | `(*proxyProviderCacheSyncer).run` | *proxyProviderCacheSyncer 的方法，运行与 'run' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 1；goroutine 0；调用 'ctx.Done'、'logger.Info'、'proxyProviderReloadInterval.String'、'proxyProviderScanInterval.String'、'reloadTicker.Stop'、's.reloadConfigs'、's.runSyncCycle'、's.wg.Wait'、'scanTicker.Stop'、'time.NewTicker' |
| 311–327 | function | `(*proxyProviderCacheSyncer).runSyncCycle` | *proxyProviderCacheSyncer 的方法，运行与 'run sync cycle' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 1；goroutine 0；调用 'len'、's.clearSubscriptionCache'、's.collectDueConfigs'、's.launchWorker' |
| 330–362 | function | `(*proxyProviderCacheSyncer).collectDueConfigs` | *proxyProviderCacheSyncer 的方法，执行与 'collect due configs' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 1；goroutine 0；调用 'append'、'len'、'logger.Info'、'now.Before'、's.ensureStateLocked'、's.mu.Lock'、's.mu.Unlock'、's.shouldRefreshLocked'、'state.blockUntil.IsZero'、'time.Now' |
| 365–392 | function | `(*proxyProviderCacheSyncer).shouldRefreshLocked` | *proxyProviderCacheSyncer 的方法，执行与 'should refresh locked' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'entry.FetchedAt.Add'、'now.Before'、's.cache.Get'、'time.Duration' |
| 395–414 | function | `(*proxyProviderCacheSyncer).launchWorker` | *proxyProviderCacheSyncer 的方法，执行与 'launch worker' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 1；调用 'ctx.Done'、's.markTaskFinished'、's.refreshSingle'、's.wg.Add'、's.wg.Done' |
| 406–413 | closure | `launchWorker.closure#1` | 供 launchWorker 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 's.markTaskFinished'、's.refreshSingle'、's.wg.Done' |
| 407–411 | closure | `launchWorker.closure#2` | 供 launchWorker 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 's.markTaskFinished'、's.wg.Done' |
| 417–473 | function | `(*proxyProviderCacheSyncer).refreshSingle` | *proxyProviderCacheSyncer 的方法，执行与 'refresh single' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 3；goroutine 0；调用 'RefreshProxyProviderCache'、'cancel'、'context.WithTimeout'、'ctx.Err'、'logger.Info'、'logger.Warn'、'makeNodePreview'、's.getOrFetchSubscription'、's.recordFailure'、's.recordSuccess' |
| 476–499 | function | `(*proxyProviderCacheSyncer).recordFailure` | *proxyProviderCacheSyncer 的方法，执行与 'record failure' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'Add'、'delay.String'、'logger.Info'、's.ensureStateLocked'、's.mu.Lock'、's.mu.Unlock'、'time.Now' |
| 502–509 | function | `(*proxyProviderCacheSyncer).recordSuccess` | *proxyProviderCacheSyncer 的方法，执行与 'record success' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 's.ensureStateLocked'、's.mu.Lock'、's.mu.Unlock' |
| 512–516 | function | `(*proxyProviderCacheSyncer).markTaskFinished` | *proxyProviderCacheSyncer 的方法，执行与 'mark task finished' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'delete'、's.mu.Lock'、's.mu.Unlock' |
| 519–559 | function | `(*proxyProviderCacheSyncer).reloadConfigs` | *proxyProviderCacheSyncer 的方法，执行与 'reload configs' 对应的业务或基础设施操作。 | 分支 3；循环 3；返回 1；goroutine 0；调用 'cancel'、'context.WithTimeout'、'delete'、'len'、'logger.Info'、'logger.Warn'、'make'、's.cache.Delete'、's.cache.UpdateInterval'、's.mu.Lock'、's.mu.Unlock'、's.repo.ListMMWProxyProviderConfigs' |
| 562–569 | function | `(*proxyProviderCacheSyncer).ensureStateLocked` | *proxyProviderCacheSyncer 的方法，执行与 'ensure state locked' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0 |
| 573–600 | function | `(*proxyProviderCacheSyncer).getOrFetchSubscription` | *proxyProviderCacheSyncer 的方法，查询或读取与 'get or fetch subscription' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 's.repo.GetExternalSubscription'、's.subCacheMu.Lock'、's.subCacheMu.RLock'、's.subCacheMu.RUnlock'、's.subCacheMu.Unlock'、'time.Now' |
| 603–612 | function | `(*proxyProviderCacheSyncer).clearSubscriptionCache` | *proxyProviderCacheSyncer 的方法，执行与 'clear subscription cache' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'len'、'logger.Info'、'make'、's.subCacheMu.Lock'、's.subCacheMu.Unlock' |
| 615–621 | function | `makeNodePreview` | 执行与 'make node preview' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'append'、'len' |

## `internal/handler/proxy_provider_serve.go`

依赖：`bytes`、`context`、`encoding/json`、`errors`、`fmt`、`io`、`miaomiaowu/internal/logger`、`net`、`net/http`、`regexp`、`strconv`、`strings`、`sync`、`time`、`miaomiaowu/internal/scriptengine`、`miaomiaowu/internal/storage`、`miaomiaowu/internal/util`、`github.com/MMWOrg/mmwX-plugins/proxyparser`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 28–28 | const | `ipInfoToken` | 定义 'ipInfoToken' 的不可变协议值、默认值或枚举成员。 |  |
| 30–33 | type | `geoIPResponse` | 定义 'geoIPResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 35–35 | var | `geoIPCache` | 保存 'geoIPCache' 的包级共享状态、配置或预计算值。 |  |
| 38–38 | const | `subscriptionCacheTTL` | 定义 'subscriptionCacheTTL' 的不可变协议值、默认值或枚举成员。 |  |
| 40–43 | type | `subscriptionCacheEntry` | 定义 'subscriptionCacheEntry' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 45–45 | var | `subscriptionCache` | 保存 'subscriptionCache' 的包级共享状态、配置或预计算值。 |  |
| 48–48 | var | `overrideScriptRepo` | 保存 'overrideScriptRepo' 的包级共享状态、配置或预计算值。 |  |
| 51–53 | function | `InvalidateSubscriptionContentCache` | 执行与 'invalidate subscription content cache' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'subscriptionCache.Delete' |
| 56–101 | function | `getGeoIPCountryCode` | 查询或读取与 'get geo ip country code' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'Decode'、'client.Get'、'fmt.Sprintf'、'geoIPCache.Load'、'geoIPCache.Store'、'ips.String'、'json.NewDecoder'、'len'、'logger.Info'、'net.LookupIP'、'net.ParseIP'、'resp.Body.Close'、'strings.ToUpper' |
| 105–217 | function | `NewProxyProviderServeHandler` | 创建并初始化与 'new proxy provider serve handler' 对应的业务或基础设施操作。 | 分支 19；循环 0；返回 13；goroutine 0；调用 'Get'、'GetBruteForceProtector'、'GetClientIP'、'bfp.IsBlocked'、'errors.New'、'http.HandlerFunc'、'http.NotFound'、'len'、'panic'、'r.Header.Get'、'r.URL.Query'、'strconv.ParseInt'、'strings.CutPrefix'、'strings.Split'、'strings.Trim'、'writeError' |
| 112–216 | closure | `NewProxyProviderServeHandler.closure#1` | 供 NewProxyProviderServeHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 18；循环 0；返回 12；goroutine 0；调用 'Get'、'GetBruteForceProtector'、'GetClientIP'、'bfp.IsBlocked'、'errors.New'、'http.NotFound'、'len'、'r.Context'、'r.Header.Get'、'r.URL.Query'、'repo.ValidateUserToken'、'strconv.ParseInt'、'strings.CutPrefix'、'strings.Split'、'strings.Trim'、'writeError' |
| 220–271 | function | `fetchSubscriptionContent` | 从外部获取与 'fetch subscription content' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 6；goroutine 0；调用 'client.Do'、'fmt.Errorf'、'http.NewRequest'、'io.ReadAll'、'logger.Info'、'req.Header.Set'、'resp.Body.Close'、'subscriptionCache.Delete'、'subscriptionCache.Load'、'subscriptionCache.Store'、'time.Now'、'time.Since' |
| 275–290 | var | `base64FeatureStrings` | 保存 'base64FeatureStrings' 的包级共享状态、配置或预计算值。 |  |
| 295–318 | function | `preprocessSubscriptionContent` | 执行与 'preprocess subscription content' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 6；goroutine 0；调用 'fmt.Errorf'、'len'、'logger.Info'、'proxyparser.Preprocess'、'yaml.Marshal' |
| 322–409 | function | `FetchAndFilterProxiesYAML` | 从外部获取与 'fetch and filter proxies yaml' 对应的业务或基础设施操作。 | 分支 11；循环 1；返回 6；goroutine 0；调用 'applyFiltersToNode'、'applyOverridesToNode'、'context.Background'、'fetchSubscriptionContent'、'findProxiesNode'、'fmt.Errorf'、'len'、'logger.Info'、'overrideScriptRepo.GetSystemConfig'、'overrideScriptRepo.ListOverrideScripts'、'preprocessSubscriptionContent'、'proxiesToYamlNode'、'reorderProxiesNode'、'scriptengine.RunPreSaveNodes'、'ya… |
| 412–434 | function | `findProxiesNode` | 查找与 'find proxies node' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 4；goroutine 0；调用 'findProxiesNode'、'len' |
| 437–481 | function | `fetchSubscriptionNodeNames` | 从外部获取与 'fetch subscription node names' 对应的业务或基础设施操作。 | 分支 6；循环 2；返回 5；goroutine 0；调用 'append'、'fetchSubscriptionContent'、'findProxiesNode'、'fmt.Errorf'、'len'、'preprocessSubscriptionContent'、'yaml.Unmarshal' |
| 484–487 | type | `NodeInfo` | 定义 'NodeInfo' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 490–541 | function | `fetchSubscriptionNodes` | 从外部获取与 'fetch subscription nodes' 对应的业务或基础设施操作。 | 分支 8；循环 2；返回 5；goroutine 0；调用 'append'、'fetchSubscriptionContent'、'findProxiesNode'、'fmt.Errorf'、'len'、'preprocessSubscriptionContent'、'yaml.Unmarshal' |
| 545–627 | function | `checkFilterMatches` | 检查与 'check filter matches' 对应的业务或基础设施操作。 | 分支 14；循环 2；返回 4；goroutine 0；调用 'excludeRegex.MatchString'、'fetchSubscriptionNodes'、'filterRegex.MatchString'、'fmt.Errorf'、'getGeoIPCountryCode'、'len'、'logger.Info'、'make'、'regexp.Compile'、'strings.Split'、'strings.ToUpper'、'strings.TrimSpace' |
| 630–640 | function | `reorderProxiesNode` | 执行与 'reorder proxies node' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 1；goroutine 0；调用 'util.ReorderProxyNode' |
| 643–753 | function | `applyFiltersToNode` | 应用与 'apply filters to node' 对应的业务或基础设施操作。 | 分支 17；循环 3；返回 2；goroutine 0；调用 'append'、'excludeRegex.MatchString'、'filterRegex.MatchString'、'getGeoIPCountryCode'、'len'、'logger.Info'、'make'、'regexp.Compile'、'strings.Split'、'strings.ToLower'、'strings.ToUpper'、'strings.TrimSpace'、'util.GetNodeFieldValue' |
| 756–777 | function | `applyOverridesToNode` | 应用与 'apply overrides to node' 对应的业务或基础设施操作。 | 分支 3；循环 2；返回 2；goroutine 0；调用 'json.Unmarshal'、'logger.Info'、'util.SetNodeField' |
| 780–813 | function | `yamlNodeToProxies` | 执行与 'yaml node to proxies' 对应的业务或基础设施操作。 | 分支 3；循环 4；返回 2；goroutine 0；调用 'append'、'len'、'make' |
| 816–827 | function | `proxiesToYamlNode` | 执行与 'proxies to yaml node' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 1；goroutine 0；调用 'append'、'len'、'yaml.Marshal'、'yaml.Unmarshal' |
| 830–841 | function | `createEmptyCacheEntry` | 创建与 'create empty cache entry' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'time.Now' |
| 844–909 | function | `RefreshProxyProviderCache` | 执行与 'refresh proxy provider cache' 对应的业务或基础设施操作。 | 分支 7；循环 1；返回 4；goroutine 0；调用 'FetchAndFilterProxiesYAML'、'GetProxyProviderCache'、'append'、'cache.Set'、'createEmptyCacheEntry'、'fmt.Errorf'、'len'、'logger.Info'、'make'、'string'、'time.Now'、'yaml.Unmarshal' |

## `internal/handler/rate_limiter.go`

依赖：`errors`、`sync`、`time`、`miaomiaowu/internal/logger`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–12 | var | `ErrRateLimited` | 保存 'ErrRateLimited' 的包级共享状态、配置或预计算值。 |  |
| 14–14 | var | `globalLoginRateLimiter` | 保存 'globalLoginRateLimiter' 的包级共享状态、配置或预计算值。 |  |
| 16–18 | function | `GetLoginRateLimiter` | 查询或读取与 'get login rate limiter' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 20–24 | type | `attemptInfo` | 定义 'attemptInfo' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 26–34 | type | `LoginRateLimiter` | 定义 'LoginRateLimiter' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 36–45 | function | `NewLoginRateLimiter` | 创建并初始化与 'new login rate limiter' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 47–56 | function | `NewLoginRateLimiterWithConfig` | 创建并初始化与 'new login rate limiter with config' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'time.Duration' |
| 58–64 | function | `(*LoginRateLimiter).UpdateConfig` | *LoginRateLimiter 的方法，更新与 'update config' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'l.mu.Lock'、'l.mu.Unlock'、'time.Duration' |
| 66–70 | function | `(*LoginRateLimiter).getConfig` | *LoginRateLimiter 的方法，查询或读取与 'get config' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'l.mu.RLock'、'l.mu.RUnlock' |
| 72–76 | function | `(*LoginRateLimiter).SetSkipLocalIP` | *LoginRateLimiter 的方法，设置与 'set skip local ip' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'l.mu.Lock'、'l.mu.Unlock' |
| 78–83 | function | `(*LoginRateLimiter).shouldSkipIP` | *LoginRateLimiter 的方法，执行与 'should skip ip' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'IsLocalOrPrivateIP'、'l.mu.RLock'、'l.mu.RUnlock' |
| 85–109 | function | `(*LoginRateLimiter).Check` | *LoginRateLimiter 的方法，检查与 'check' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 3；goroutine 0；调用 'l.checkAttempts'、'l.shouldSkipIP'、'logger.Warn'、'time.Now' |
| 111–141 | function | `(*LoginRateLimiter).checkAttempts` | *LoginRateLimiter 的方法，检查与 'check attempts' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'info.lockUntil.IsZero'、'l.getConfig'、'now.Add'、'now.After'、'now.Before'、'now.Sub'、'store.Delete'、'store.Load' |
| 143–152 | function | `(*LoginRateLimiter).RecordFailure` | *LoginRateLimiter 的方法，执行与 'record failure' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'l.recordAttempt'、'l.shouldSkipIP'、'time.Now' |
| 154–177 | function | `(*LoginRateLimiter).recordAttempt` | *LoginRateLimiter 的方法，执行与 'record attempt' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'l.getConfig'、'now.Sub'、'store.Load'、'store.Store' |
| 179–184 | function | `(*LoginRateLimiter).RecordSuccess` | *LoginRateLimiter 的方法，执行与 'record success' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'l.accountAttempts.Delete'、'l.ipAttempts.Delete' |

## `internal/handler/relay_group_fix_test.go`

依赖：`context`、`net/http`、`net/http/httptest`、`strings`、`testing`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 18–61 | function | `TestNodeUpdate_PartialPayloadPreservesEnabled` | 执行与 'test node update_ partial payload preserves enabled' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 0；goroutine 0；调用 'NewNodesHandler'、'auth.ContextWithUsername'、'context.Background'、'h.ServeHTTP'、'httptest.NewRecorder'、'httptest.NewRequest'、'int64ToString'、'mustNode'、'rec.Body.String'、'relayTestRepo'、'repo.ListNodes'、'req.WithContext'、'strings.NewReader'、't.Fatal'、't.Fatalf'、't.TempDir' |
| 64–89 | function | `TestNodeUpdate_ExplicitDisableStillWorks` | 执行与 'test node update_ explicit disable still works' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 0；goroutine 0；调用 'NewNodesHandler'、'auth.ContextWithUsername'、'context.Background'、'h.ServeHTTP'、'httptest.NewRecorder'、'httptest.NewRequest'、'int64ToString'、'mustNode'、'rec.Body.String'、'relayTestRepo'、'repo.ListNodes'、'req.WithContext'、'strings.NewReader'、't.Error'、't.Fatalf'、't.TempDir' |
| 93–101 | function | `TestAnyToYAMLNode_StringSlice` | 执行与 'test any to yaml node_ string slice' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'anyToYAMLNode'、'len'、't.Errorf'、't.Fatalf' |
| 105–122 | function | `TestInjectRelayGroupsIntoTemplate_PopulatesMembers` | 执行与 'test inject relay groups into template_ populates members' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 0；goroutine 0；调用 'injectRelayGroupsIntoTemplate'、'strings.Contains'、't.Errorf'、't.Fatalf' |

## `internal/handler/relay_group_test.go`

依赖：`encoding/json`、`testing`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–100 | function | `buildRelayData` | 构建与 'build relay data' 对应的业务或基础设施操作。 | 分支 14；循环 5；返回 3；goroutine 0；调用 'append'、'buildProxyConfig'、'json.Unmarshal'、'len'、'make'、'node.HasAnyTag' |
| 24–39 | closure | `buildRelayData.closure#1` | 供 buildRelayData 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 4；循环 0；返回 2；goroutine 0；调用 'json.Unmarshal'、'len' |
| 105–155 | function | `TestRelayGroup_UnderlyingNodeBackfilled` | 执行与 'test relay group_ underlying node backfilled' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 0；goroutine 0；调用 'buildRelayData'、'findProxyByName'、'len'、't.Errorf'、't.Fatalf' |
| 159–198 | function | `TestRelayGroup_DisabledMemberDropped` | 执行与 'test relay group_ disabled member dropped' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 0；goroutine 0；调用 'buildRelayData'、'findProxyByName'、'len'、't.Error'、't.Errorf'、't.Fatalf' |
| 202–233 | function | `TestRelayGroup_MemberAlreadyInProxies_NoDuplicate` | 执行与 'test relay group_ member already in proxies_ no duplicate' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 0；goroutine 0；调用 'buildRelayData'、't.Errorf' |

## `internal/handler/rule_templates.go`

依赖：`encoding/json`、`fmt`、`io`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`、`net/http`、`os`、`path/filepath`、`strings`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 15–17 | type | `RuleTemplatesHandler` | 定义 'RuleTemplatesHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 19–21 | function | `NewRuleTemplatesHandler` | 创建并初始化与 'new rule templates handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 23–27 | function | `(*RuleTemplatesHandler).isAdmin` | *RuleTemplatesHandler 的方法，判断与 'is admin' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'auth.UsernameFromContext'、'h.repo.GetUser'、'r.Context' |
| 29–35 | function | `(*RuleTemplatesHandler).canView` | *RuleTemplatesHandler 的方法，判断是否允许与 'can view' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'auth.UsernameFromContext'、'h.isAdmin'、'h.repo.GetRuleTemplateOwner'、'h.repo.IsRuleTemplatePublic'、'r.Context' |
| 37–43 | function | `(*RuleTemplatesHandler).canModify` | *RuleTemplatesHandler 的方法，判断是否允许与 'can modify' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'auth.UsernameFromContext'、'h.isAdmin'、'h.repo.GetRuleTemplateOwner'、'r.Context' |
| 45–103 | function | `(*RuleTemplatesHandler).ServeHTTP` | *RuleTemplatesHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 6；goroutine 0；调用 'h.canModify'、'h.canView'、'h.handleDeleteTemplate'、'h.handleGetTemplate'、'h.handleListTemplates'、'h.handleRenameTemplate'、'h.handleUpdateTemplate'、'h.handleUploadTemplate'、'h.handleVisibility'、'http.Error'、'strings.TrimPrefix' |
| 105–134 | function | `(*RuleTemplatesHandler).handleListTemplates` | *RuleTemplatesHandler 的方法，处理与 'handle list templates' 对应的业务或基础设施操作。 | 分支 2；循环 2；返回 1；goroutine 0；调用 'Encode'、'Set'、'append'、'entry.IsDir'、'entry.Name'、'h.canView'、'h.repo.IsRuleTemplatePublic'、'h.repo.ListRuleTemplateOwners'、'http.Error'、'len'、'make'、'os.ReadDir'、'r.Context'、'strings.HasSuffix'、'strings.ToLower'、'w.Header' |
| 136–164 | function | `(*RuleTemplatesHandler).handleGetTemplate` | *RuleTemplatesHandler 的方法，处理与 'handle get template' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'Encode'、'Set'、'filepath.Join'、'http.Error'、'json.NewEncoder'、'os.IsNotExist'、'os.ReadFile'、'os.Stat'、'string'、'strings.Contains'、'w.Header' |
| 166–212 | function | `(*RuleTemplatesHandler).handleUpdateTemplate` | *RuleTemplatesHandler 的方法，处理与 'handle update template' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 4；goroutine 1；调用 'Decode'、'Encode'、'RefreshSubscriptionsByTemplate'、'Set'、'auth.UsernameFromContext'、'filepath.Join'、'http.Error'、'json.NewDecoder'、'json.NewEncoder'、'os.IsNotExist'、'os.Stat'、'os.WriteFile'、'r.Context'、'strings.Contains'、'w.Header'、'w.WriteHeader' |
| 214–246 | function | `(*RuleTemplatesHandler).handleDeleteTemplate` | *RuleTemplatesHandler 的方法，处理与 'handle delete template' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'Encode'、'Set'、'filepath.Join'、'h.repo.DeleteRuleTemplateOwner'、'http.Error'、'json.NewEncoder'、'os.IsNotExist'、'os.Remove'、'os.Stat'、'r.Context'、'strings.Contains'、'w.Header'、'w.WriteHeader' |
| 248–330 | function | `(*RuleTemplatesHandler).handleRenameTemplate` | *RuleTemplatesHandler 的方法，处理与 'handle rename template' 对应的业务或基础设施操作。 | 分支 9；循环 0；返回 7；goroutine 0；调用 'Decode'、'Encode'、'Set'、'filepath.Join'、'h.canModify'、'http.Error'、'json.NewDecoder'、'json.NewEncoder'、'os.IsNotExist'、'os.Stat'、'strings.Contains'、'strings.HasSuffix'、'strings.ToLower'、'strings.TrimSpace'、'w.Header'、'w.WriteHeader' |
| 332–408 | function | `(*RuleTemplatesHandler).handleUploadTemplate` | *RuleTemplatesHandler 的方法，处理与 'handle upload template' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 8；goroutine 0；调用 'Encode'、'Set'、'file.Close'、'filepath.Base'、'filepath.Join'、'http.Error'、'json.NewEncoder'、'os.MkdirAll'、'os.Stat'、'r.FormFile'、'r.ParseMultipartForm'、'strings.Contains'、'strings.HasSuffix'、'strings.ToLower'、'w.Header'、'w.WriteHeader' |
| 410–432 | function | `(*RuleTemplatesHandler).handleVisibility` | *RuleTemplatesHandler 的方法，处理与 'handle visibility' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 4；goroutine 0；调用 'Decode'、'err.Error'、'filepath.Base'、'h.isAdmin'、'h.repo.SetRuleTemplatePublic'、'http.Error'、'json.NewDecoder'、'r.Context'、'respondJSON' |

## `internal/handler/rules.go`

依赖：`encoding/json`、`errors`、`io`、`net/http`、`os`、`path/filepath`、`strings`、`gopkg.in/yaml.v3`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 18–22 | type | `RuleEditorHandler` | 定义 'RuleEditorHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 24–31 | function | `NewRuleEditorHandler` | 创建并初始化与 'new rule editor handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 33–79 | function | `(*RuleEditorHandler).ServeHTTP` | *RuleEditorHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 7；goroutine 0；调用 'h.handleGet'、'h.handleHistory'、'h.handleList'、'h.handleUpdate'、'http.Error'、'http.NotFound'、'len'、'methodNotAllowed'、'strings.Split'、'strings.Trim' |
| 81–127 | function | `(*RuleEditorHandler).handleList` | *RuleEditorHandler 的方法，处理与 'handle list' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 1；goroutine 0；调用 'Unix'、'append'、'entry.Info'、'entry.IsDir'、'entry.Name'、'h.repo.ListRuleVersions'、'http.Error'、'info.ModTime'、'info.Size'、'isYAMLFile'、'len'、'make'、'os.ReadDir'、'r.Context'、'respondJSON' |
| 129–158 | function | `(*RuleEditorHandler).handleGet` | *RuleEditorHandler 的方法，处理与 'handle get' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 3；goroutine 0；调用 'err.Error'、'errors.Is'、'h.repo.ListRuleVersions'、'h.resolveFilename'、'http.Error'、'http.NotFound'、'len'、'os.ReadFile'、'r.Context'、'respondJSON'、'string'、'writeBadRequest' |
| 160–188 | function | `(*RuleEditorHandler).handleHistory` | *RuleEditorHandler 的方法，处理与 'handle history' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'err.Error'、'errors.Is'、'h.repo.ListRuleVersions'、'h.resolveFilename'、'http.Error'、'http.NotFound'、'os.Stat'、'r.Context'、'respondJSON'、'writeBadRequest' |
| 190–250 | function | `(*RuleEditorHandler).handleUpdate` | *RuleEditorHandler 的方法，处理与 'handle update' 对应的业务或基础设施操作。 | 分支 10；循环 0；返回 9；goroutine 0；调用 'auth.UsernameOrDefault'、'err.Error'、'errors.Is'、'h.repo.SaveRuleVersion'、'h.resolveFilename'、'http.Error'、'http.MaxBytesReader'、'http.NotFound'、'io.ReadAll'、'json.Unmarshal'、'os.Stat'、'os.WriteFile'、'r.Context'、'respondJSON'、'writeBadRequest'、'yaml.Unmarshal' |
| 252–272 | function | `(*RuleEditorHandler).resolveFilename` | *RuleEditorHandler 的方法，解析或求解与 'resolve filename' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'errors.New'、'filepath.Base'、'filepath.Clean'、'filepath.Join'、'isYAMLFile'、'strings.Contains'、'strings.TrimSpace' |
| 274–277 | function | `isYAMLFile` | 判断与 'is yaml file' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'strings.HasSuffix'、'strings.ToLower' |
| 279–282 | function | `methodNotAllowed` | 执行与 'method not allowed' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'Set'、'http.Error'、'strings.Join'、'w.Header' |
| 284–286 | function | `writeBadRequest` | 执行与 'write bad request' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'respondJSON' |
| 288–292 | function | `respondJSON` | 执行与 'respond json' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'Encode'、'Set'、'json.NewEncoder'、'w.Header'、'w.WriteHeader' |

## `internal/handler/rules_metadata.go`

依赖：`encoding/json`、`net/http`、`os`、`path/filepath`、`strings`、`time`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–17 | type | `ruleMetadataHandler` | 定义 'ruleMetadataHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 20–25 | function | `NewRuleMetadataHandler` | 创建并初始化与 'new rule metadata handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'panic' |
| 27–77 | function | `(*ruleMetadataHandler).ServeHTTP` | *ruleMetadataHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 1；goroutine 0；调用 'Format'、'Set'、'Unix'、'append'、'filepath.Join'、'h.repo.LatestRuleVersion'、'http.Error'、'info.ModTime'、'latest.CreatedAt.UTC'、'len'、'make'、'os.Stat'、'r.Context'、'r.URL.Query'、'sanitizeRuleFilename'、'w.Header' |
| 79–104 | function | `sanitizeRuleFilename` | 执行与 'sanitize rule filename' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'filepath.Base'、'filepath.Clean'、'strings.Contains'、'strings.HasSuffix'、'strings.ToLower'、'strings.TrimSpace' |

## `internal/handler/security_logs.go`

依赖：`encoding/json`、`net`、`net/http`、`strconv`、`strings`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 22–24 | type | `SecurityLogHandler` | 定义 'SecurityLogHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 26–28 | function | `NewSecurityLogHandler` | 创建并初始化与 'new security log handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 30–45 | function | `(*SecurityLogHandler).ServeHTTP` | *SecurityLogHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'h.handleCreateBan'、'h.handleEvents'、'h.handleListBans'、'h.handleUnban'、'http.Error'、'strings.HasPrefix'、'strings.TrimPrefix' |
| 47–62 | function | `(*SecurityLogHandler).handleEvents` | *SecurityLogHandler 的方法，处理与 'handle events' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'Get'、'atoiDefault'、'h.repo.ListSecurityEvents'、'r.Context'、'r.URL.Query'、'respondJSON'、'strings.TrimSpace'、'writeError' |
| 64–74 | function | `(*SecurityLogHandler).handleListBans` | *SecurityLogHandler 的方法，处理与 'handle list bans' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'h.repo.ListActiveIPBans'、'r.Context'、'respondJSON'、'writeError' |
| 76–97 | function | `(*SecurityLogHandler).handleCreateBan` | *SecurityLogHandler 的方法，处理与 'handle create ban' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'Decode'、'GetBruteForceProtector'、'auth.UsernameFromContext'、'json.NewDecoder'、'net.ParseIP'、'p.BanIP'、'r.Context'、'respondJSON'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |
| 99–112 | function | `(*SecurityLogHandler).handleUnban` | *SecurityLogHandler 的方法，处理与 'handle unban' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'GetBruteForceProtector'、'auth.UsernameFromContext'、'net.ParseIP'、'p.UnbanIP'、'r.Context'、'respondJSON'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |
| 114–122 | function | `atoiDefault` | 执行与 'atoi default' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'strconv.Atoi' |

## `internal/handler/setup.go`

依赖：`encoding/json`、`errors`、`net/http`、`strings`、`golang.org/x/crypto/bcrypt`、`miaomiaowu/internal/logger`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 15–17 | type | `setupStatusResponse` | 定义 'setupStatusResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 19–25 | type | `setupRequest` | 定义 'setupRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 27–31 | type | `setupResponse` | 定义 'setupResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 34–81 | function | `NewSetupStatusHandler` | 创建并初始化与 'new setup status handler' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 3；goroutine 0；调用 'Encode'、'Set'、'errors.New'、'http.HandlerFunc'、'json.NewEncoder'、'len'、'logger.Error'、'logger.Info'、'make'、'panic'、'r.Context'、'repo.ListUsers'、'w.Header'、'w.WriteHeader'、'writeError' |
| 39–80 | closure | `NewSetupStatusHandler.closure#1` | 供 NewSetupStatusHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 3；循环 1；返回 2；goroutine 0；调用 'Encode'、'Set'、'errors.New'、'json.NewEncoder'、'len'、'logger.Error'、'logger.Info'、'make'、'r.Context'、'repo.ListUsers'、'w.Header'、'w.WriteHeader'、'writeError' |
| 84–196 | function | `NewInitialSetupHandler` | 创建并初始化与 'new initial setup handler' 对应的业务或基础设施操作。 | 分支 12；循环 0；返回 10；goroutine 0；调用 'Decode'、'bcrypt.GenerateFromPassword'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'len'、'logger.Error'、'logger.Info'、'logger.Warn'、'panic'、'r.Context'、'repo.CreateUser'、'repo.ListUsers'、'string'、'strings.TrimSpace'、'writeError' |
| 89–195 | closure | `NewInitialSetupHandler.closure#1` | 供 NewInitialSetupHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 11；循环 0；返回 9；goroutine 0；调用 'Decode'、'bcrypt.GenerateFromPassword'、'errors.Is'、'errors.New'、'json.NewDecoder'、'len'、'logger.Error'、'logger.Info'、'logger.Warn'、'r.Context'、'repo.CreateUser'、'repo.ListUsers'、'repo.UpdateUserRole'、'string'、'strings.TrimSpace'、'writeError' |

## `internal/handler/short_link.go`

依赖：`encoding/json`、`errors`、`fmt`、`net/http`、`strings`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–17 | type | `shortLinkHandler` | 定义 'shortLinkHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 20–32 | function | `NewShortLinkHandler` | 创建并初始化与 'new short link handler' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'panic' |
| 36–92 | function | `(*shortLinkHandler).TryServe` | *shortLinkHandler 的方法，执行与 'try serve' 对应的业务或基础设施操作。 | 分支 7；循环 1；返回 6；goroutine 0；调用 'Get'、'auth.ContextWithUsername'、'h.repo.GetAllFileShortCodes'、'h.repo.GetAllUserShortCodes'、'h.subscriptionHandler.ServeHTTP'、'len'、'newURL.Query'、'q.Encode'、'q.Set'、'r.Clone'、'r.Context'、'r.URL.Query'、'strings.Trim' |
| 95–99 | function | `(*shortLinkHandler).ServeHTTP` | *shortLinkHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'h.TryServe'、'http.NotFound' |
| 102–104 | type | `shortLinkResetHandler` | 定义 'shortLinkResetHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 107–113 | function | `NewShortLinkResetHandler` | 创建并初始化与 'new short link reset handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'panic' |
| 115–129 | function | `(*shortLinkResetHandler).ServeHTTP` | *shortLinkResetHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'h.handleReset'、'r.Context'、'writeError' |
| 131–146 | function | `(*shortLinkResetHandler).handleReset` | *shortLinkResetHandler 的方法，处理与 'handle reset' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'GetSilentModeManager'、'Set'、'fmt.Fprintf'、'h.repo.ResetAllSubscriptionShortURLs'、'm.InvalidateShortLinkCache'、'r.Context'、'w.Header'、'w.WriteHeader'、'writeError' |
| 149–221 | function | `NewUserCustomShortCodeSelfHandler` | 创建并初始化与 'new user custom short code self handler' 对应的业务或基础设施操作。 | 分支 12；循环 1；返回 8；goroutine 0；调用 'Decode'、'Encode'、'Set'、'auth.UsernameFromContext'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.GetAllUserShortCodes'、'repo.GetEffectiveUserShortCode'、'repo.GetUser'、'repo.GetUserCustomShortCode'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 150–220 | closure | `NewUserCustomShortCodeSelfHandler.closure#1` | 供 NewUserCustomShortCodeSelfHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 12；循环 1；返回 7；goroutine 0；调用 'Decode'、'Encode'、'Set'、'auth.UsernameFromContext'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.GetAllUserShortCodes'、'repo.GetEffectiveUserShortCode'、'repo.GetUser'、'repo.GetUserCustomShortCode'、'repo.UpdateUserCustomShortCode'、'strings.TrimSpace'、'w.Header'、'writeError' |

## `internal/handler/silent_mode.go`

依赖：`context`、`net/http`、`strings`、`sync`、`time`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/logger`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 15–15 | var | `globalSilentModeManager` | 保存 'globalSilentModeManager' 的包级共享状态、配置或预计算值。 |  |
| 17–27 | type | `SilentModeManager` | 定义 'SilentModeManager' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 29–40 | function | `NewSilentModeManager` | 创建并初始化与 'new silent mode manager' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'logger.Info'、'm.startTime.Format'、'time.Now' |
| 42–44 | function | `GetSilentModeManager` | 查询或读取与 'get silent mode manager' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 47–51 | function | `(*SilentModeManager).InvalidateShortLinkCache` | *SilentModeManager 的方法，执行与 'invalidate short link cache' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'm.shortLinkSetMu.Lock'、'm.shortLinkSetMu.Unlock' |
| 53–75 | function | `(*SilentModeManager).refreshShortLinkSet` | *SilentModeManager 的方法，执行与 'refresh short link set' 对应的业务或基础设施操作。 | 分支 2；循环 2；返回 2；goroutine 0；调用 'context.Background'、'len'、'm.repo.GetAllFileShortCodes'、'm.repo.GetAllUserShortCodes'、'm.shortLinkSetMu.Lock'、'm.shortLinkSetMu.Unlock'、'make'、'time.Now' |
| 77–94 | function | `(*SilentModeManager).isKnownShortLink` | *SilentModeManager 的方法，判断与 'is known short link' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'isAlphanumericPath'、'len'、'm.refreshShortLinkSet'、'm.shortLinkSetMu.RLock'、'm.shortLinkSetMu.RUnlock'、'time.Since' |
| 96–105 | function | `(*SilentModeManager).RecordSubscriptionAccess` | *SilentModeManager 的方法，执行与 'record subscription access' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'Format'、'logger.Info'、'm.lastActiveTime.Store'、'time.Now' |
| 108–125 | function | `(*SilentModeManager).RecordSubscriptionAccessWithIP` | *SilentModeManager 的方法，执行与 'record subscription access with ip' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'logger.Info'、'm.globalActiveMu.Lock'、'm.globalActiveMu.Unlock'、'm.lastActiveTime.Store'、'now.Format'、'time.Now' |
| 127–140 | function | `(*SilentModeManager).isUserActive` | *SilentModeManager 的方法，判断与 'is user active' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'Before'、'lastActive.Add'、'm.lastActiveTime.Load'、'time.Duration'、'time.Now' |
| 143–154 | function | `(*SilentModeManager).isGlobalActive` | *SilentModeManager 的方法，判断与 'is global active' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'Before'、'lastActive.Add'、'lastActive.IsZero'、'm.globalActiveMu.Lock'、'm.globalActiveMu.Unlock'、'time.Duration'、'time.Now' |
| 157–175 | function | `(*SilentModeManager).extractUsername` | *SilentModeManager 的方法，执行与 'extract username' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 4；goroutine 0；调用 'Get'、'm.tokens.Lookup'、'r.Header.Get'、'r.URL.Query'、'strings.TrimSpace' |
| 177–198 | function | `(*SilentModeManager).isAllowedPath` | *SilentModeManager 的方法，判断与 'is allowed path' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 3；goroutine 0；调用 'm.isKnownShortLink'、'strings.HasPrefix'、'strings.Trim' |
| 200–207 | function | `isAlphanumericPath` | 判断与 'is alphanumeric path' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0 |
| 209–257 | function | `(*SilentModeManager).Middleware` | *SilentModeManager 的方法，执行与 'middleware' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 7；goroutine 0；调用 'Before'、'GetClientIP'、'Set'、'context.Background'、'http.HandlerFunc'、'logger.Info'、'm.extractUsername'、'm.isAllowedPath'、'm.isGlobalActive'、'm.isUserActive'、'm.repo.GetSystemConfig'、'm.startTime.Add'、'next.ServeHTTP'、'time.Duration'、'time.Now'、'w.Header' |
| 210–256 | closure | `Middleware.closure#1` | 供 Middleware 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'Before'、'GetClientIP'、'Set'、'context.Background'、'http.NotFound'、'logger.Info'、'm.extractUsername'、'm.isAllowedPath'、'm.isGlobalActive'、'm.isUserActive'、'm.repo.GetSystemConfig'、'm.startTime.Add'、'next.ServeHTTP'、'time.Duration'、'time.Now'、'w.Header' |

## `internal/handler/speedtest.go`

依赖：`context`、`crypto/rand`、`crypto/sha256`、`encoding/hex`、`encoding/json`、`errors`、`net/http`、`strconv`、`time`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/speedtest`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 19–22 | type | `SpeedTestHandler` | 定义 'SpeedTestHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 24–26 | function | `NewSpeedTestHandler` | 创建并初始化与 'new speed test handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 28–28 | function | `(*SpeedTestHandler).SetTesterWS` | *SpeedTestHandler 的方法，设置与 'set tester ws' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0 |
| 30–50 | function | `(*SpeedTestHandler).ServeHTTP` | *SpeedTestHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'errors.New'、'h.handleResults'、'h.handleRun'、'h.handleTesterCreate'、'h.handleTesterRevoke'、'h.handleTesterRotateToken'、'h.handleTestersList'、'respondJSON'、'speedtest.MihomoStatus'、'writeError' |
| 52–104 | function | `(*SpeedTestHandler).handleRun` | *SpeedTestHandler 的方法，处理与 'handle run' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 5；goroutine 1；调用 'Decode'、'auth.UsernameFromContext'、'errors.New'、'h.repo.GetNodeByID'、'h.repo.InsertSpeedTestResult'、'h.runSpeedTestAsync'、'json.NewDecoder'、'r.Context'、'respondJSON'、'time.Now'、'writeBadRequest'、'writeError' |
| 106–130 | function | `(*SpeedTestHandler).runSpeedTestAsync` | *SpeedTestHandler 的方法，运行与 'run speed test async' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 0；goroutine 0；调用 'cancel'、'context.Background'、'context.WithTimeout'、'h.repo.UpdateSpeedTestResult'、'h.testerWS.Dispatch'、'speedtest.EnsureMihomo'、'speedtest.RunNodeTest'、'terr.Error' |
| 132–149 | function | `(*SpeedTestHandler).handleTesterCreate` | *SpeedTestHandler 的方法，处理与 'handle tester create' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'Decode'、'auth.UsernameFromContext'、'h.repo.CreateSpeedTester'、'hashSpeedTesterToken'、'hex.EncodeToString'、'json.NewDecoder'、'make'、'r.Context'、'rand.Read'、'respondJSON'、'writeError' |
| 151–166 | function | `(*SpeedTestHandler).handleTestersList` | *SpeedTestHandler 的方法，处理与 'handle testers list' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 1；goroutine 0；调用 'append'、'h.repo.ListSpeedTesters'、'h.testerWS.Online'、'len'、'make'、'r.Context'、'respondJSON'、'writeError' |
| 168–181 | function | `(*SpeedTestHandler).handleTesterRevoke` | *SpeedTestHandler 的方法，处理与 'handle tester revoke' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'Decode'、'h.repo.DeleteSpeedTester'、'json.NewDecoder'、'r.Context'、'respondJSON'、'writeBadRequest'、'writeError' |
| 185–204 | function | `(*SpeedTestHandler).handleTesterRotateToken` | *SpeedTestHandler 的方法，处理与 'handle tester rotate token' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'Decode'、'h.repo.UpdateSpeedTesterToken'、'hashSpeedTesterToken'、'hex.EncodeToString'、'json.NewDecoder'、'make'、'r.Context'、'rand.Read'、'respondJSON'、'writeBadRequest'、'writeError' |
| 206–224 | function | `(*SpeedTestHandler).handleResults` | *SpeedTestHandler 的方法，处理与 'handle results' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'Get'、'h.repo.ListLatestSpeedTestResults'、'h.repo.ListSpeedTestResults'、'r.Context'、'r.URL.Query'、'respondJSON'、'strconv.Atoi'、'strconv.ParseInt'、'writeError' |
| 226–229 | function | `hashSpeedTesterToken` | 判断是否具有与 'hash speed tester token' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'hex.EncodeToString'、'sha256.Sum256' |

## `internal/handler/speedtester_ws.go`

依赖：`context`、`encoding/json`、`errors`、`log`、`net/http`、`sync`、`time`、`github.com/google/uuid`、`github.com/gorilla/websocket`、`miaomiaowu/internal/speedtest`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 19–32 | type | `stWSMsg` | 定义 'stWSMsg' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 34–39 | type | `testerConn` | 定义 'testerConn' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 41–46 | function | `(*testerConn).send` | *testerConn 的方法，执行与 'send' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'json.Marshal'、'tc.conn.WriteMessage'、'tc.writeMu.Lock'、'tc.writeMu.Unlock' |
| 49–53 | type | `SpeedTesterWSHandler` | 定义 'SpeedTesterWSHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 55–60 | function | `NewSpeedTesterWSHandler` | 创建并初始化与 'new speed tester ws handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 2；goroutine 0 |
| 58–58 | closure | `NewSpeedTesterWSHandler.closure#1` | 供 NewSpeedTesterWSHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 62–65 | function | `(*SpeedTesterWSHandler).Online` | *SpeedTesterWSHandler 的方法，执行与 'online' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'h.conns.Load' |
| 67–121 | function | `(*SpeedTesterWSHandler).ServeHTTP` | *SpeedTesterWSHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 10；循环 1；返回 4；goroutine 0；调用 'Get'、'conn.Close'、'conn.ReadMessage'、'conn.SetReadLimit'、'context.Background'、'h.conns.Delete'、'h.conns.Load'、'h.conns.Store'、'h.repo.GetSpeedTesterByTokenHash'、'h.repo.TouchSpeedTester'、'h.upgrader.Upgrade'、'hashSpeedTesterToken'、'http.Error'、'log.Printf'、'r.Context'、'r.URL.Query' |
| 90–96 | closure | `ServeHTTP.closure#1` | 供 ServeHTTP 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'conn.Close'、'h.conns.Delete'、'h.conns.Load'、'log.Printf' |
| 125–154 | function | `(*SpeedTesterWSHandler).Dispatch` | *SpeedTesterWSHandler 的方法，执行与 'dispatch' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 6；goroutine 0；调用 'String'、'ctx.Done'、'ctx.Err'、'err.Error'、'errors.New'、'h.conns.Load'、'make'、'tc.pending.Delete'、'tc.pending.Store'、'tc.send'、'time.After'、'uuid.New' |

## `internal/handler/ssrf_safe_fetch.go`

依赖：`context`、`errors`、`fmt`、`net`、`net/http`、`net/url`、`strings`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–14 | var | `errSSRFBlocked` | 保存 'errSSRFBlocked' 的包级共享状态、配置或预计算值。 |  |
| 16–16 | const | `maxFetchBodyBytes` | 定义 'maxFetchBodyBytes' 的不可变协议值、默认值或枚举成员。 |  |
| 18–32 | var | `ssrfBlockedNetworks` | 保存 'ssrfBlockedNetworks' 的包级共享状态、配置或预计算值。 |  |
| 34–45 | function | `isBlockedFetchIP` | 判断与 'is blocked fetch ip' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 3；goroutine 0；调用 'ip.IsInterfaceLocalMulticast'、'ip.IsLinkLocalMulticast'、'ip.IsLinkLocalUnicast'、'ip.IsLoopback'、'ip.IsUnspecified'、'network.Contains' |
| 47–59 | function | `validateFetchURL` | 校验与 'validate fetch url' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'errors.New'、'strings.TrimSpace'、'u.Hostname'、'url.Parse' |
| 61–87 | function | `ssrfSafeDialContext` | 执行与 'ssrf safe dial context' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 8；goroutine 0；调用 'dialer.DialContext'、'fmt.Errorf'、'ips.IP.String'、'isBlockedFetchIP'、'len'、'net.DefaultResolver.LookupIPAddr'、'net.JoinHostPort'、'net.ParseIP'、'net.SplitHostPort' |
| 62–86 | closure | `ssrfSafeDialContext.closure#1` | 供 ssrfSafeDialContext 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 6；循环 1；返回 7；goroutine 0；调用 'dialer.DialContext'、'fmt.Errorf'、'ips.IP.String'、'isBlockedFetchIP'、'len'、'net.DefaultResolver.LookupIPAddr'、'net.JoinHostPort'、'net.ParseIP'、'net.SplitHostPort' |
| 89–108 | function | `newSSRFSafeHTTPClient` | 创建并初始化与 'new ssrf safe http client' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 4；goroutine 0；调用 'errors.New'、'len'、'ssrfSafeDialContext' |
| 93–101 | closure | `newSSRFSafeHTTPClient.closure#1` | 供 newSSRFSafeHTTPClient 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'errors.New'、'len' |

## `internal/handler/ssrf_safe_fetch_test.go`

依赖：`context`、`errors`、`net`、`testing`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 11–20 | function | `TestValidateFetchURL` | 执行与 'test validate fetch url' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 0；goroutine 0；调用 't.Fatalf'、'validateFetchURL' |
| 22–28 | function | `TestSSRFSafeDialBlocksPrivateLiteral` | 执行与 'test ssrf safe dial blocks private literal' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'context.Background'、'dial'、'errors.Is'、'ssrfSafeDialContext'、't.Fatalf' |

## `internal/handler/subscribe_filename_test.go`

依赖：`testing`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 5–14 | function | `TestSanitizeSubscribeFilename` | 执行与 'test sanitize subscribe filename' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 0；goroutine 0；调用 'sanitizeSubscribeFilename'、't.Errorf'、't.Fatalf' |

## `internal/handler/subscribe_files.go`

依赖：`bytes`、`context`、`encoding/json`、`errors`、`fmt`、`io`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/logger`、`net/http`、`net/url`、`os`、`path/filepath`、`strconv`、`strings`、`time`、`github.com/MMWOrg/mmwX-plugins/proxyparser/substore`、`miaomiaowu/internal/storage`、`miaomiaowu/internal/validator`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 27–29 | type | `subscribeFilesHandler` | 定义 'subscribeFilesHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 31–49 | function | `sanitizeSubscribeFilename` | 执行与 'sanitize subscribe filename' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 4；goroutine 0；调用 'errors.New'、'filepath.Base'、'filepath.Ext'、'strings.Contains'、'strings.ContainsAny'、'strings.ToLower'、'strings.TrimSpace' |
| 51–60 | function | `(*subscribeFilesHandler).ensureFilenameAvailable` | *subscribeFilesHandler 的方法，执行与 'ensure filename available' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'errors.Is'、'fmt.Errorf'、'h.repo.GetSubscribeFileByFilename' |
| 63–71 | function | `NewSubscribeFilesHandler` | 创建并初始化与 'new subscribe files handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'panic' |
| 73–112 | function | `(*subscribeFilesHandler).ServeHTTP` | *subscribeFilesHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'h.handleCreate'、'h.handleCreateAggregate'、'h.handleCreateFromConfig'、'h.handleDelete'、'h.handleGetContent'、'h.handleGetSubscriptionUsers'、'h.handleImport'、'h.handleList'、'h.handleReorder'、'h.handleUpdate'、'h.handleUpdateContent'、'h.handleUpload'、'strings.HasSuffix'、'strings.Trim'、'strings.TrimPrefix'、'strings.TrimSuffix' |
| 114–124 | function | `(*subscribeFilesHandler).handleList` | *subscribeFilesHandler 的方法，处理与 'handle list' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'h.convertSubscribeFilesWithVersions'、'h.repo.ListSubscribeFiles'、'r.Context'、'respondJSON'、'writeError' |
| 126–143 | function | `(*subscribeFilesHandler).handleReorder` | *subscribeFilesHandler 的方法，处理与 'handle reorder' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'Decode'、'h.repo.ReorderSubscribeFiles'、'json.NewDecoder'、'len'、'r.Context'、'respondJSON'、'writeBadRequest'、'writeError' |
| 145–210 | function | `(*subscribeFilesHandler).handleCreate` | *subscribeFilesHandler 的方法，处理与 'handle create' 对应的业务或基础设施操作。 | 分支 10；循环 0；返回 10；goroutine 0；调用 'Decode'、'convertSubscribeFile'、'err.Error'、'errors.Is'、'errors.New'、'h.ensureFilenameAvailable'、'h.repo.CreateSubscribeFile'、'json.NewDecoder'、'parseExpireAt'、'r.Context'、'respondJSON'、'sanitizeSubscribeFilename'、'writeBadRequest'、'writeError' |
| 212–342 | function | `(*subscribeFilesHandler).handleImport` | *subscribeFilesHandler 的方法，处理与 'handle import' 对应的业务或基础设施操作。 | 分支 19；循环 0；返回 16；goroutine 0；调用 'Decode'、'client.Do'、'err.Error'、'errors.New'、'http.NewRequest'、'httpReq.Header.Set'、'io.LimitReader'、'io.ReadAll'、'json.NewDecoder'、'len'、'newSSRFSafeHTTPClient'、'resp.Body.Close'、'validateFetchURL'、'writeBadRequest'、'writeError'、'yaml.Unmarshal' |
| 344–481 | function | `(*subscribeFilesHandler).handleUpload` | *subscribeFilesHandler 的方法，处理与 'handle upload' 对应的业务或基础设施操作。 | 分支 22；循环 0；返回 16；goroutine 0；调用 'errors.Is'、'errors.New'、'file.Close'、'filepath.Join'、'h.repo.GetSubscribeFileByID'、'io.ReadAll'、'os.MkdirAll'、'os.WriteFile'、'r.Context'、'r.FormFile'、'r.FormValue'、'r.ParseMultipartForm'、'strconv.ParseInt'、'writeBadRequest'、'writeError'、'yaml.Unmarshal' |
| 483–666 | function | `(*subscribeFilesHandler).handleUpdate` | *subscribeFilesHandler 的方法，处理与 'handle update' 对应的业务或基础设施操作。 | 分支 39；循环 1；返回 14；goroutine 1；调用 'Decode'、'GetSilentModeManager'、'errors.Is'、'errors.New'、'filepath.Ext'、'h.repo.GetAllFileShortCodes'、'h.repo.GetSubscribeFileByFilename'、'h.repo.GetSubscribeFileByID'、'json.NewDecoder'、'm.InvalidateShortLinkCache'、'parseExpireAt'、'r.Context'、'strconv.ParseInt'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |
| 648–660 | closure | `handleUpdate.closure#1` | 供 handleUpdate 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'auth.UsernameFromContext'、'context.Background'、'h.regenerateFromTemplate'、'logger.Info'、'r.Context' |
| 668–701 | function | `(*subscribeFilesHandler).handleDelete` | *subscribeFilesHandler 的方法，处理与 'handle delete' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'errors.Is'、'filepath.Join'、'h.repo.DeleteSubscribeFile'、'h.repo.GetSubscribeFileByID'、'os.Remove'、'r.Context'、'respondJSON'、'strconv.ParseInt'、'writeBadRequest'、'writeError' |
| 705–731 | function | `parseFilenameFromContentDisposition` | 解析与 'parse filename from content disposition' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 3；goroutine 0；调用 'strings.Index'、'strings.IndexAny'、'strings.LastIndex'、'strings.Trim'、'strings.TrimSpace'、'url.QueryUnescape' |
| 733–750 | type | `subscribeFileRequest` | 定义 'subscribeFileRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 752–772 | type | `subscribeFileDTO` | 定义 'subscribeFileDTO' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 774–811 | function | `convertSubscribeFile` | 转换与 'convert subscribe file' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 1；goroutine 0 |
| 813–819 | function | `convertSubscribeFiles` | 转换与 'convert subscribe files' 对应的业务或基础设施操作。 | 分支 0；循环 1；返回 1；goroutine 0；调用 'append'、'convertSubscribeFile'、'len'、'make' |
| 821–834 | function | `(*subscribeFilesHandler).convertSubscribeFilesWithVersions` | *subscribeFilesHandler 的方法，转换与 'convert subscribe files with versions' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 1；goroutine 0；调用 'append'、'convertSubscribeFile'、'h.repo.ListRuleVersions'、'len'、'make' |
| 836–854 | function | `parseExpireAt` | 解析与 'parse expire at' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 4；goroutine 0；调用 'strings.TrimSpace'、'time.Parse' |
| 857–1038 | function | `(*subscribeFilesHandler).handleCreateFromConfig` | *subscribeFilesHandler 的方法，处理与 'handle create from config' 对应的业务或基础设施操作。 | 分支 25；循环 2；返回 16；goroutine 1；调用 'Decode'、'auth.UsernameFromContext'、'err.Error'、'errors.Is'、'errors.New'、'h.ensureFilenameAvailable'、'h.repo.GetUserSettings'、'json.NewDecoder'、'logger.Info'、'r.Context'、'sanitizeSubscribeFilename'、'tempEncoder.SetIndent'、'writeBadRequest'、'writeError'、'yaml.NewEncoder'、'yaml.Unmarshal' |
| 1041–1080 | function | `(*subscribeFilesHandler).handleGetContent` | *subscribeFilesHandler 的方法，处理与 'handle get content' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'errors.Is'、'errors.New'、'filepath.Join'、'h.repo.GetSubscribeFileByFilename'、'os.ReadFile'、'r.Context'、'respondJSON'、'string'、'url.QueryUnescape'、'writeBadRequest'、'writeError' |
| 1083–1192 | function | `(*subscribeFilesHandler).handleUpdateContent` | *subscribeFilesHandler 的方法，处理与 'handle update content' 对应的业务或基础设施操作。 | 分支 15；循环 2；返回 12；goroutine 0；调用 'Decode'、'append'、'err.Error'、'errors.Is'、'errors.New'、'fmt.Sprintf'、'h.repo.GetSubscribeFileByFilename'、'json.NewDecoder'、'logger.Info'、'r.Context'、'strings.Join'、'url.QueryUnescape'、'validator.ValidateClashConfig'、'writeBadRequest'、'writeError'、'yaml.Unmarshal' |
| 1196–1198 | function | `(*subscribeFilesHandler).syncMMWProxyProvidersToFile` | *subscribeFilesHandler 的方法，同步与 'sync mmw proxy providers to file' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'SyncMMWProxyProvidersToFile' |
| 1202–1303 | function | `SyncMMWProxyProvidersToFile` | 同步与 'sync mmw proxy providers to file' 对应的业务或基础设施操作。 | 分支 14；循环 2；返回 3；goroutine 0；调用 'GetProxyProviderCache'、'RefreshProxyProviderCache'、'cache.Get'、'cache.IsExpired'、'collectExistingProxyNodes'、'collectUsedProviderNames'、'context.Background'、'filepath.Join'、'fmt.Sprintf'、'len'、'logger.Info'、'os.ReadFile'、'repo.GetExternalSubscription'、'repo.GetProxyProviderConfigByName'、'strings.Index'、'yaml.Unmarshal' |
| 1306–1349 | function | `collectExistingProxyNodes` | 执行与 'collect existing proxy nodes' 对应的业务或基础设施操作。 | 分支 6；循环 3；返回 4；goroutine 0；调用 'append'、'len'、'make' |
| 1355–1427 | function | `collectUsedProviderNames` | 执行与 'collect used provider names' 对应的业务或基础设施操作。 | 分支 12；循环 4；返回 4；goroutine 0；调用 'append'、'len'、'make' |
| 1430–1445 | function | `copyMap` | 执行与 'copy map' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 1；goroutine 0；调用 'copy'、'copyMap'、'len'、'make' |
| 1450–1570 | function | `(*subscribeFilesHandler).handleCreateAggregate` | *subscribeFilesHandler 的方法，处理与 'handle create aggregate' 对应的业务或基础设施操作。 | 分支 17；循环 1；返回 11；goroutine 1；调用 'Decode'、'append'、'auth.UsernameFromContext'、'buildAggregateConfigContent'、'err.Error'、'errors.New'、'h.ensureFilenameAvailable'、'json.NewDecoder'、'len'、'make'、'r.Context'、'sanitizeSubscribeFilename'、'strings.Join'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |
| 1559–1564 | closure | `handleCreateAggregate.closure#1` | 供 handleCreateAggregate 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'context.Background'、'h.regenerateFromTemplate'、'logger.Info' |
| 1574–1648 | function | `buildAggregateConfigContent` | 构建与 'build aggregate config content' 对应的业务或基础设施操作。 | 分支 9；循环 2；返回 7；goroutine 0；调用 'append'、'filepath.Join'、'fmt.Errorf'、'injectProxiesIntoTemplate'、'json.Unmarshal'、'len'、'make'、'node.HasAnyTag'、'os.ReadFile'、'processor.ProcessTemplate'、'repo.ListNodes'、'string'、'substore.NewTemplateV3Processor'、'yaml.Marshal' |
| 1651–1787 | function | `(*subscribeFilesHandler).regenerateFromTemplate` | *subscribeFilesHandler 的方法，执行与 'regenerate from template' 对应的业务或基础设施操作。 | 分支 19；循环 7；返回 7；goroutine 0；调用 'append'、'errors.New'、'filepath.Join'、'fmt.Errorf'、'h.repo.ListNodes'、'h.repo.ListProxyProviderConfigs'、'injectProxiesIntoTemplate'、'json.Unmarshal'、'len'、'logger.Info'、'make'、'node.HasAnyTag'、'os.ReadFile'、'processor.ProcessTemplate'、'string'、'substore.NewTemplateV3Processor' |
| 1791–1822 | function | `RefreshAllTemplateSubscriptions` | 执行与 'refresh all template subscriptions' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 2；goroutine 0；调用 'context.Background'、'h.regenerateFromTemplate'、'len'、'logger.Info'、'repo.GetSubscribeFilesWithTemplate' |
| 1825–1850 | function | `RefreshSubscriptionsByTemplate` | 执行与 'refresh subscriptions by template' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 2；goroutine 0；调用 'context.Background'、'h.regenerateFromTemplate'、'len'、'logger.Info'、'repo.GetSubscribeFilesByTemplate' |
| 1852–1869 | function | `(*subscribeFilesHandler).handleGetSubscriptionUsers` | *subscribeFilesHandler 的方法，处理与 'handle get subscription users' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 2；goroutine 0；调用 'h.repo.GetUsersBySubscriptionID'、'r.Context'、'respondJSON'、'strconv.ParseInt'、'writeBadRequest'、'writeError' |
| 1874–1931 | function | `pruneUnreferencedProxiesYAML` | 执行与 'prune unreferenced proxies yaml' 对应的业务或基础设施操作。 | 分支 11；循环 3；返回 8；goroutine 0；调用 'MarshalYAMLWithIndent'、'RemoveUnicodeEscapeQuotes'、'append'、'len'、'make'、'string'、'substore.CollectUsedProxyNamesFromGroups'、'yaml.Unmarshal' |

## `internal/handler/subscribe_files_list.go`

依赖：`net/http`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 9–11 | type | `subscribeFilesListHandler` | 定义 'subscribeFilesListHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 14–22 | function | `NewSubscribeFilesListHandler` | 创建并初始化与 'new subscribe files list handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'panic' |
| 24–54 | function | `(*subscribeFilesListHandler).ServeHTTP` | *subscribeFilesListHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 2；goroutine 0；调用 'append'、'h.repo.ListSubscribeFiles'、'len'、'make'、'methodNotAllowed'、'r.Context'、'respondJSON'、'writeError' |

## `internal/handler/subscription.go`

依赖：`bytes`、`context`、`encoding/json`、`errors`、`fmt`、`miaomiaowu/internal/logger`、`net/http`、`net/url`、`os`、`path/filepath`、`sort`、`strconv`、`strings`、`time`、`github.com/MMWOrg/mmwX-plugins/proxyparser/substore`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/notify`、`miaomiaowu/internal/scriptengine`、`miaomiaowu/internal/storage`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 28–28 | const | `subscriptionDefaultType` | 定义 'subscriptionDefaultType' 的不可变协议值、默认值或枚举成员。 |  |
| 31–84 | const | `tokenInvalidYAML` | 定义 'tokenInvalidYAML' 的不可变协议值、默认值或枚举成员。 |  |
| 86–86 | const | `tokenInvalidFilename` | 定义 'tokenInvalidFilename' 的不可变协议值、默认值或枚举成员。 |  |
| 89–89 | type | `ContextKey` | 定义 'ContextKey' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 91–91 | const | `TokenInvalidKey` | 定义 'TokenInvalidKey' 的不可变协议值、默认值或枚举成员。 |  |
| 93–98 | type | `SubscriptionHandler` | 定义 'SubscriptionHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 100–104 | type | `subscriptionEndpoint` | 定义 'subscriptionEndpoint' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 106–113 | function | `NewSubscriptionHandler` | 创建并初始化与 'new subscription handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'NewTrafficSummaryHandler'、'newSubscriptionHandler'、'panic' |
| 117–124 | function | `NewSubscriptionHandlerConcrete` | 创建并初始化与 'new subscription handler concrete' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'NewTrafficSummaryHandler'、'newSubscriptionHandler'、'panic' |
| 127–137 | function | `NewSubscriptionEndpoint` | 创建并初始化与 'new subscription endpoint' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'newSubscriptionHandler'、'panic' |
| 139–161 | function | `newSubscriptionHandler` | 创建并初始化与 'new subscription handler' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 1；goroutine 0；调用 'NewTrafficSummaryHandler'、'filepath.Clean'、'filepath.FromSlash'、'panic' |
| 163–175 | function | `(*subscriptionEndpoint).ServeHTTP` | *subscriptionEndpoint 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'GetBruteForceProtector'、'GetClientIP'、'bfp.IsBlocked'、'http.NotFound'、's.authorizeRequest'、's.inner.ServeHTTP' |
| 177–214 | function | `(*subscriptionEndpoint).authorizeRequest` | *subscriptionEndpoint 的方法，执行与 'authorize request' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 5；goroutine 0；调用 'Get'、'GetBruteForceProtector'、'GetClientIP'、'auth.ContextWithUsername'、'bfp.RecordFailure'、'context.WithValue'、'errors.Is'、'r.Context'、'r.Header.Get'、'r.URL.Query'、'r.WithContext'、's.repo.ValidateUserToken'、's.tokens.Lookup'、'strings.TrimSpace'、'writeError' |
| 216–1045 | function | `(*SubscriptionHandler).ServeHTTP` | *SubscriptionHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 136；循环 12；返回 18；goroutine 1；调用 'Get'、'GetBruteForceProtector'、'GetClientIP'、'Value'、'auth.UsernameFromContext'、'bfp.RecordFailure'、'errors.Is'、'errors.New'、'h.repo.GetSubscribeFileByFilename'、'h.serveTokenInvalidResponse'、'r.Context'、'r.URL.Query'、'rejectBlockedSubscriptionUA'、'strings.TrimSpace'、'time.Now'、'writeError' |
| 1047–1072 | function | `(*SubscriptionHandler).resolveSubscription` | *SubscriptionHandler 的方法，解析或求解与 'resolve subscription' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'errors.Is'、'errors.New'、'h.repo.GetFirstSubscriptionLink'、'h.repo.GetSubscriptionByName'、'strings.TrimSpace' |
| 1074–1082 | function | `buildSubscriptionHeader` | 构建与 'build subscription header' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'expireAt.Unix'、'strconv.FormatInt' |
| 1085–1091 | function | `getKeys` | 查询或读取与 'get keys' 对应的业务或基础设施操作。 | 分支 0；循环 1；返回 1；goroutine 0；调用 'append'、'len'、'make' |
| 1096–1263 | function | `GetExternalSubscriptionsFromFile` | 查询或读取与 'get external subscriptions from file' 对应的业务或基础设施操作。 | 分支 27；循环 11；返回 3；goroutine 0；调用 'fmt.Errorf'、'len'、'logger.Info'、'make'、'repo.ListExternalSubscriptions'、'repo.ListNodes'、'repo.ListProxyProviderConfigs'、'strconv.ParseInt'、'strings.CutPrefix'、'yaml.Unmarshal' |
| 1266–1351 | function | `syncReferencedExternalSubscriptions` | 同步与 'sync referenced external subscriptions' 对应的业务或基础设施操作。 | 分支 7；循环 4；返回 2；goroutine 0；调用 'GetProxyProviderCache'、'InvalidateSubscriptionContentCache'、'Milliseconds'、'cache.Delete'、'fmt.Errorf'、'len'、'logger.Info'、'make'、'repo.GetUserSettings'、'repo.ListProxyProviderConfigs'、'repo.UpdateExternalSubscription'、'syncSingleExternalSubscription'、'time.Now'、'time.Since' |
| 1353–1366 | function | `(*SubscriptionHandler).loadTokenInvalidContent` | *SubscriptionHandler 的方法，加载与 'load token invalid content' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'filepath.Join'、'len'、'logger.Info'、'os.ReadFile' |
| 1369–1416 | function | `(*SubscriptionHandler).serveTokenInvalidResponse` | *SubscriptionHandler 的方法，提供 HTTP 服务与 'serve token invalid response' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 0；goroutine 0；调用 'Set'、'h.convertSubscription'、'h.loadTokenInvalidContent'、'logger.Info'、'r.Context'、'resolveClientType'、'url.PathEscape'、'w.Header'、'w.Write'、'w.WriteHeader' |
| 1419–1440 | function | `(*SubscriptionHandler).runPostFetchScript` | *SubscriptionHandler 的方法，运行与 'run post fetch script' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'fmt.Errorf'、'scriptengine.RunPostFetch'、'yaml.Marshal'、'yaml.Unmarshal'、'yamlNodeToMap' |
| 1443–1526 | function | `(*SubscriptionHandler).convertSubscription` | *SubscriptionHandler 的方法，转换与 'convert subscription' 对应的业务或基础设施操作。 | 分支 13；循环 1；返回 14；goroutine 0；调用 'append'、'errors.New'、'factory.GetProducer'、'fmt.Errorf'、'h.convertClashToLoon'、'h.convertClashToSurge'、'h.repo.GetSystemConfig'、'len'、'producer.Produce'、'substore.BuildLoonKeleeConfig'、'substore.GetDefaultFactory'、'substore.Proxy'、'yaml.Unmarshal'、'yamlNodeToMap' |
| 1529–1657 | function | `(*SubscriptionHandler).convertClashToSurge` | *SubscriptionHandler 的方法，转换与 'convert clash to surge' 对应的业务或基础设施操作。 | 分支 34；循环 6；返回 2；goroutine 0；调用 'append'、'fmt.Errorf'、'make'、'substore.BuildCompleteSurgeConfig' |
| 1660–1758 | function | `(*SubscriptionHandler).convertClashToLoon` | *SubscriptionHandler 的方法，转换与 'convert clash to loon' 对应的业务或基础设施操作。 | 分支 26；循环 4；返回 2；goroutine 0；调用 'append'、'fmt.Errorf'、'make'、'substore.BuildCompleteLoonConfig' |
| 1761–1817 | function | `fixWireGuardAllowedIPs` | 执行与 'fix wire guard allowed i ps' 对应的业务或基础设施操作。 | 分支 11；循环 4；返回 1；goroutine 0；调用 'len' |
| 1820–1831 | function | `reorderProxies` | 执行与 'reorder proxies' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 1；goroutine 0；调用 'reorderProxyNode' |
| 1835–1907 | function | `reorderProxyNode` | 执行与 'reorder proxy node' 对应的业务或基础设施操作。 | 分支 8；循环 3；返回 1；goroutine 0；调用 'append'、'len'、'make' |
| 1910–1921 | function | `reorderProxyGroups` | 执行与 'reorder proxy groups' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 1；goroutine 0；调用 'reorderProxyGroupFields' |
| 1925–1980 | function | `reorderProxyGroupFields` | 执行与 'reorder proxy group fields' 对应的业务或基础设施操作。 | 分支 5；循环 3；返回 1；goroutine 0；调用 'append'、'len'、'make' |
| 1985–2054 | function | `injectLegacyDialerProxy` | 执行与 'inject legacy dialer proxy' 对应的业务或基础设施操作。 | 分支 12；循环 6；返回 2；goroutine 0；调用 'append'、'len'、'make'、'yamlMapGet' |
| 2056–2185 | function | `injectRelayGroups` | 执行与 'inject relay groups' 对应的业务或基础设施操作。 | 分支 18；循环 9；返回 3；goroutine 0；调用 'append'、'json.Unmarshal'、'len'、'make'、'mapToYAMLNode'、'repo.ListNodes'、'yamlMapGet' |
| 2188–2195 | function | `yamlMapGet` | 执行与 'yaml map get' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0；调用 'len' |
| 2198–2215 | function | `stripDialerProxyGroup` | 执行与 'strip dialer proxy group' 对应的业务或基础设施操作。 | 分支 3；循环 2；返回 0；goroutine 0；调用 'append'、'len'、'make' |
| 2218–2240 | function | `sortNodesByNodeOrder` | 执行与 'sort nodes by node order' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 4；goroutine 0；调用 'len'、'make'、'sort.SliceStable' |
| 2228–2239 | closure | `sortNodesByNodeOrder.closure#1` | 供 sortNodesByNodeOrder 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 3；goroutine 0 |
| 2244–2351 | function | `sortProxiesByNodeOrder` | 执行与 'sort proxies by node order' 对应的业务或基础设施操作。 | 分支 11；循环 5；返回 7；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'len'、'logger.Info'、'make'、'repo.ListNodes'、'sort.SliceStable' |
| 2326–2340 | closure | `sortProxiesByNodeOrder.closure#1` | 供 sortProxiesByNodeOrder 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 3；goroutine 0 |
| 2355–2559 | function | `(*SubscriptionHandler).generateFromTemplate` | *SubscriptionHandler 的方法，生成与 'generate from template' 对应的业务或基础设施操作。 | 分支 30；循环 9；返回 10；goroutine 0；调用 'append'、'buildProxyConfig'、'errors.New'、'filepath.Join'、'fmt.Errorf'、'h.repo.GetAdminUsername'、'h.repo.GetUser'、'h.repo.GetUserSettings'、'h.repo.ListNodes'、'json.Unmarshal'、'len'、'logger.Info'、'make'、'node.HasAnyTag'、'os.ReadFile'、'sortNodesByNodeOrder' |
| 2402–2422 | closure | `generateFromTemplate.closure#1` | 供 generateFromTemplate 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 4；循环 0；返回 2；goroutine 0；调用 'json.Unmarshal'、'len'、'logger.Info' |
| 2563–2647 | function | `(*SubscriptionHandler).generateFromSelectedTags` | *SubscriptionHandler 的方法，生成与 'generate from selected tags' 对应的业务或基础设施操作。 | 分支 12；循环 3；返回 4；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'h.repo.GetAdminUsername'、'h.repo.GetUser'、'h.repo.GetUserSettings'、'h.repo.ListNodes'、'json.Unmarshal'、'len'、'logger.Info'、'make'、'node.HasAnyTag'、'sortNodesByNodeOrder'、'yaml.Marshal' |
| 2650–2687 | function | `createSubInfoNodes` | 创建与 'create sub info nodes' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'append'、'createDummyNode'、'expireAt.Format'、'formatTrafficSize' |
| 2665–2683 | closure | `createSubInfoNodes.closure#1` | 供 createSubInfoNodes 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 2690–2704 | function | `formatTrafficSize` | 执行与 'format traffic size' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'float64'、'fmt.Sprintf' |
| 2709–2746 | function | `marshalSubscriptionJSON` | 执行与 'marshal subscription json' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 3；goroutine 0；调用 'buf.Bytes'、'buf.WriteByte'、'buf.WriteString'、'fmt.Errorf'、'jsonEncodeString'、'jsonWriteCompact'、'jsonWriteSeqExpanded'、'len'、'yaml.Unmarshal' |
| 2748–2757 | function | `makeIDSet` | 执行与 'make id set' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0；调用 'len'、'make' |
| 2759–2759 | var | `jsonProxyKeyPriority` | 保存 'jsonProxyKeyPriority' 的包级共享状态、配置或预计算值。 |  |
| 2761–2780 | function | `jsonWriteSeqExpanded` | 执行与 'json write seq expanded' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 1；goroutine 0；调用 'buf.WriteByte'、'buf.WriteString'、'jsonWriteCompact'、'jsonWriteMappingReordered'、'len' |
| 2782–2807 | function | `jsonWriteCompact` | 执行与 'json write compact' 对应的业务或基础设施操作。 | 分支 3；循环 2；返回 0；goroutine 0；调用 'buf.WriteByte'、'buf.WriteString'、'jsonEncodeString'、'jsonWriteCompact'、'jsonWriteScalar'、'len' |
| 2809–2849 | function | `jsonWriteMappingReordered` | 执行与 'json write mapping reordered' 对应的业务或基础设施操作。 | 分支 4；循环 3；返回 0；goroutine 0；调用 'buf.WriteByte'、'buf.WriteString'、'jsonEncodeString'、'jsonWriteCompact'、'len'、'make' |
| 2851–2877 | function | `jsonWriteScalar` | 执行与 'json write scalar' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 0；goroutine 0；调用 'buf.WriteString'、'jsonEncodeString'、'strconv.FormatInt'、'strconv.ParseInt'、'strings.ToLower' |
| 2879–2882 | function | `jsonEncodeString` | 执行与 'json encode string' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'buf.Write'、'json.Marshal' |
| 2887–2979 | function | `deduplicateProxies` | 执行与 'deduplicate proxies' 对应的业务或基础设施操作。 | 分支 16；循环 3；返回 4；goroutine 0；调用 'append'、'len'、'logger.Warn'、'make'、'yaml.Marshal'、'yaml.Unmarshal' |

## `internal/handler/subscription_admin.go`

依赖：`context`、`errors`、`fmt`、`io`、`mime/multipart`、`net/http`、`os`、`path/filepath`、`strconv`、`strings`、`time`、`unicode`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 21–24 | type | `subscriptionAdminHandler` | 定义 'subscriptionAdminHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 27–40 | function | `NewSubscriptionAdminHandler` | 创建并初始化与 'new subscription admin handler' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'filepath.Clean'、'filepath.FromSlash'、'panic' |
| 42–59 | function | `(*subscriptionAdminHandler).ServeHTTP` | *subscriptionAdminHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'h.handleCreate'、'h.handleDelete'、'h.handleList'、'h.handleUpdate'、'methodNotAllowed'、'strings.Trim'、'strings.TrimPrefix' |
| 61–71 | function | `(*subscriptionAdminHandler).handleList` | *subscriptionAdminHandler 的方法，处理与 'handle list' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'convertSubscriptions'、'h.repo.ListSubscriptionLinks'、'r.Context'、'respondJSON'、'writeError' |
| 73–119 | function | `(*subscriptionAdminHandler).handleCreate` | *subscriptionAdminHandler 的方法，处理与 'handle create' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 4；goroutine 0；调用 'convertSubscription'、'errors.Is'、'file.Close'、'h.persistRuleFile'、'h.repo.CreateSubscriptionLink'、'r.Context'、'r.FormFile'、'r.FormValue'、'r.ParseMultipartForm'、'respondJSON'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |
| 121–196 | function | `(*subscriptionAdminHandler).handleUpdate` | *subscriptionAdminHandler 的方法，处理与 'handle update' 对应的业务或基础设施操作。 | 分支 12；循环 0；返回 6；goroutine 0；调用 'errors.Is'、'file.Close'、'fileHeader'、'firstValue'、'h.cleanupRuleFile'、'h.persistRuleFile'、'h.repo.GetSubscriptionByID'、'h.repo.UpdateSubscriptionLink'、'header.Open'、'len'、'r.Context'、'r.ParseMultipartForm'、'strconv.ParseInt'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |
| 198–227 | function | `(*subscriptionAdminHandler).handleDelete` | *subscriptionAdminHandler 的方法，处理与 'handle delete' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 3；goroutine 0；调用 'errors.Is'、'h.cleanupRuleFile'、'h.repo.DeleteSubscriptionLink'、'h.repo.GetSubscriptionByID'、'r.Context'、'respondJSON'、'strconv.ParseInt'、'writeBadRequest'、'writeError' |
| 229–257 | function | `(*subscriptionAdminHandler).persistRuleFile` | *subscriptionAdminHandler 的方法，执行与 'persist rule file' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'buildRuleFilename'、'errors.New'、'filepath.Ext'、'filepath.Join'、'fmt.Errorf'、'os.MkdirAll'、'strings.ToLower'、'writeToFile' |
| 259–274 | function | `(*subscriptionAdminHandler).cleanupRuleFile` | *subscriptionAdminHandler 的方法，清理与 'cleanup rule file' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'errors.Is'、'filepath.Join'、'h.repo.CountSubscriptionsByFilename'、'os.Remove'、'strings.TrimSpace' |
| 276–288 | function | `writeToFile` | 执行与 'write to file' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'fmt.Errorf'、'io.Copy'、'os.Create'、'out.Close' |
| 290–316 | function | `buildRuleFilename` | 构建与 'build rule filename' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 1；goroutine 0；调用 'UnixNano'、'append'、'fmt.Sprintf'、'len'、'make'、'string'、'strings.ToLower'、'strings.Trim'、'strings.TrimSpace'、'time.Now'、'unicode.IsDigit'、'unicode.IsLetter'、'unicode.IsSpace' |
| 318–323 | function | `fileHeader` | 执行与 'file header' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'errors.New'、'len' |
| 325–330 | function | `firstValue` | 执行与 'first value' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'len' |
| 332–341 | type | `subscriptionDTO` | 定义 'subscriptionDTO' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 343–354 | function | `convertSubscription` | 转换与 'convert subscription' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'append' |
| 356–362 | function | `convertSubscriptions` | 转换与 'convert subscriptions' 对应的业务或基础设施操作。 | 分支 0；循环 1；返回 1；goroutine 0；调用 'append'、'convertSubscription'、'len'、'make' |
| 366–472 | function | `NewSubscriptionListHandler` | 创建并初始化与 'new subscription list handler' 对应的业务或基础设施操作。 | 分支 11；循环 1；返回 6；goroutine 0；调用 'append'、'auth.UsernameFromContext'、'errors.New'、'http.HandlerFunc'、'len'、'make'、'methodNotAllowed'、'panic'、'r.Context'、'repo.GetEffectiveUserShortCode'、'repo.GetSystemConfig'、'repo.GetUser'、'repo.GetUserSubscriptions'、'repo.ListRuleVersions'、'repo.ListSubscribeFiles'、'writeError' |
| 371–471 | closure | `NewSubscriptionListHandler.closure#1` | 供 NewSubscriptionListHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 10；循环 1；返回 5；goroutine 0；调用 'append'、'auth.UsernameFromContext'、'errors.New'、'len'、'make'、'methodNotAllowed'、'r.Context'、'repo.GetEffectiveUserShortCode'、'repo.GetSystemConfig'、'repo.GetUser'、'repo.GetUserSubscriptions'、'repo.ListRuleVersions'、'repo.ListSubscribeFiles'、'respondJSON'、'writeError' |

## `internal/handler/subscription_rate.go`

依赖：`context`、`sync`、`time`、`miaomiaowu/internal/logger`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 11–11 | var | `globalSubscriptionRateLimiter` | 保存 'globalSubscriptionRateLimiter' 的包级共享状态、配置或预计算值。 |  |
| 13–15 | function | `GetSubscriptionRateLimiter` | 查询或读取与 'get subscription rate limiter' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 17–20 | type | `subRateRecord` | 定义 'subRateRecord' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 22–29 | type | `SubscriptionRateLimiter` | 定义 'SubscriptionRateLimiter' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 31–47 | function | `NewSubscriptionRateLimiter` | 创建并初始化与 'new subscription rate limiter' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'make' |
| 49–53 | function | `(*SubscriptionRateLimiter).SetSkipLocalIP` | *SubscriptionRateLimiter 的方法，设置与 'set skip local ip' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'l.mu.Lock'、'l.mu.Unlock' |
| 55–65 | function | `(*SubscriptionRateLimiter).UpdateConfig` | *SubscriptionRateLimiter 的方法，更新与 'update config' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'l.mu.Lock'、'l.mu.Unlock'、'time.Duration' |
| 68–96 | function | `(*SubscriptionRateLimiter).Allow` | *SubscriptionRateLimiter 的方法，执行与 'allow' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'IsLocalOrPrivateIP'、'l.mu.Lock'、'l.mu.Unlock'、'l.window.String'、'logger.Warn'、'now.Sub'、'time.Now' |
| 99–116 | function | `(*SubscriptionRateLimiter).StartCleanup` | *SubscriptionRateLimiter 的方法，启动与 'start cleanup' 对应的业务或基础设施操作。 | 分支 2；循环 2；返回 1；goroutine 0；调用 'ctx.Done'、'delete'、'l.mu.Lock'、'l.mu.Unlock'、'now.Sub'、'ticker.Stop'、'time.NewTicker' |

## `internal/handler/subscription_ua_guard.go`

依赖：`errors`、`net/http`、`sync/atomic`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 9–9 | var | `blockUnknownSubscriptionUA` | 保存 'blockUnknownSubscriptionUA' 的包级共享状态、配置或预计算值。 |  |
| 11–13 | function | `SetBlockUnknownSubscriptionUA` | 设置与 'set block unknown subscription ua' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'blockUnknownSubscriptionUA.Store' |
| 15–22 | function | `rejectBlockedSubscriptionUA` | 执行与 'reject blocked subscription ua' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'Set'、'blockUnknownSubscriptionUA.Load'、'detectClientTypeFromUA'、'errors.New'、'r.Header.Get'、'w.Header'、'writeError' |

## `internal/handler/subscription_ua_guard_test.go`

依赖：`net/http/httptest`、`testing`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–19 | function | `TestDetectClientTypeFromUA` | 执行与 'test detect client type from ua' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 0；goroutine 0；调用 'detectClientTypeFromUA'、't.Errorf' |
| 21–37 | function | `TestResolveClientTypeAuto` | 执行与 'test resolve client type auto' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 0；goroutine 0；调用 'httptest.NewRequest'、'req.Header.Set'、'resolveClientType'、't.Errorf'、't.Fatalf' |
| 39–48 | function | `TestSubscriptionUAGuard` | 执行与 'test subscription ua guard' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'SetBlockUnknownSubscriptionUA'、'httptest.NewRecorder'、'httptest.NewRequest'、'rejectBlockedSubscriptionUA'、'req.Header.Set'、't.Cleanup'、't.Fatalf' |
| 41–41 | closure | `TestSubscriptionUAGuard.closure#1` | 供 TestSubscriptionUAGuard 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'SetBlockUnknownSubscriptionUA' |

## `internal/handler/surge_template_test.go`

依赖：`strings`、`testing`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–25 | function | `TestInjectProxiesIntoSurgeTemplate` | 执行与 'test inject proxies into surge template' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 0；goroutine 0；调用 'injectProxiesIntoSurgeTemplate'、'strings.Contains'、't.Errorf'、't.Fatal'、't.Fatalf' |
| 27–34 | function | `TestLooksLikeSurgeTemplate` | 执行与 'test looks like surge template' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'looksLikeSurgeTemplate'、't.Fatal' |

## `internal/handler/task_logs.go`

依赖：`net/http`、`strings`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–16 | type | `TaskLogHandler` | 定义 'TaskLogHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 18–20 | function | `NewTaskLogHandler` | 创建并初始化与 'new task log handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 23–26 | type | `taskType` | 定义 'taskType' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 29–41 | var | `taskTypes` | 保存 'taskTypes' 的包级共享状态、配置或预计算值。 |  |
| 43–69 | function | `(*TaskLogHandler).ServeHTTP` | *TaskLogHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 2；goroutine 0；调用 'Get'、'atoiDefault'、'h.repo.ListTaskRuns'、'http.Error'、'r.Context'、'r.URL.Query'、'respondJSON'、'strings.TrimPrefix'、'strings.TrimSpace'、'writeError' |

## `internal/handler/tcping.go`

依赖：`encoding/json`、`fmt`、`net`、`net/http`、`time`、`miaomiaowu/internal/logger`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–18 | type | `TCPingRequest` | 定义 'TCPingRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 21–25 | type | `TCPingResponse` | 定义 'TCPingResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 28–85 | function | `NewTCPingHandler` | 创建并初始化与 'new tc ping handler' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 5；goroutine 0；调用 'Decode'、'Microseconds'、'Set'、'conn.Close'、'err.Error'、'float64'、'fmt.Sprintf'、'http.HandlerFunc'、'json.NewDecoder'、'logger.Debug'、'net.DialTimeout'、'net.JoinHostPort'、'time.Duration'、'time.Now'、'time.Since'、'writeJSONError' |
| 29–84 | closure | `NewTCPingHandler.closure#1` | 供 NewTCPingHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 7；循环 0；返回 4；goroutine 0；调用 'Decode'、'Microseconds'、'Set'、'conn.Close'、'err.Error'、'float64'、'fmt.Sprintf'、'json.NewDecoder'、'logger.Debug'、'net.DialTimeout'、'net.JoinHostPort'、'time.Duration'、'time.Now'、'time.Since'、'w.Header'、'writeJSONError' |
| 88–155 | function | `NewTCPingBatchHandler` | 创建并初始化与 'new tc ping batch handler' 对应的业务或基础设施操作。 | 分支 8；循环 2；返回 6；goroutine 1；调用 'Decode'、'Microseconds'、'conn.Close'、'err.Error'、'float64'、'fmt.Sprintf'、'http.HandlerFunc'、'json.NewDecoder'、'len'、'make'、'net.DialTimeout'、'net.JoinHostPort'、'time.Duration'、'time.Now'、'time.Since'、'writeJSONError' |
| 89–154 | closure | `NewTCPingBatchHandler.closure#1` | 供 NewTCPingBatchHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 8；循环 2；返回 5；goroutine 1；调用 'Decode'、'Microseconds'、'Set'、'conn.Close'、'err.Error'、'float64'、'fmt.Sprintf'、'json.NewDecoder'、'len'、'make'、'net.DialTimeout'、'net.JoinHostPort'、'time.Duration'、'time.Now'、'time.Since'、'writeJSONError' |
| 115–144 | closure | `NewTCPingBatchHandler.closure#2` | 供 NewTCPingBatchHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 4；循环 0；返回 1；goroutine 0；调用 'Microseconds'、'conn.Close'、'err.Error'、'float64'、'fmt.Sprintf'、'net.DialTimeout'、'net.JoinHostPort'、'time.Duration'、'time.Now'、'time.Since' |
| 116–116 | closure | `NewTCPingBatchHandler.closure#3` | 供 NewTCPingBatchHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0 |
| 157–161 | function | `writeJSONError` | 执行与 'write json error' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'Encode'、'Set'、'json.NewEncoder'、'w.Header'、'w.WriteHeader' |

## `internal/handler/temp_subscription.go`

依赖：`crypto/rand`、`encoding/hex`、`encoding/json`、`errors`、`net/http`、`strings`、`sync`、`time`、`miaomiaowu/internal/util`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 19–26 | type | `TempSubscription` | 定义 'TempSubscription' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 29–32 | type | `TempSubscriptionStore` | 定义 'TempSubscriptionStore' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 35–37 | var | `tempSubStore` | 保存 'tempSubStore' 的包级共享状态、配置或预计算值。 |  |
| 40–44 | function | `generateTempSubCode` | 生成与 'generate temp sub code' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'hex.EncodeToString'、'make'、'rand.Read' |
| 47–71 | function | `(*TempSubscriptionStore).Create` | *TempSubscriptionStore 的方法，创建与 'create' 对应的业务或基础设施操作。 | 分支 0；循环 1；返回 1；goroutine 0；调用 'Add'、'generateTempSubCode'、's.cleanupLocked'、's.mu.Lock'、's.mu.Unlock'、'time.Duration'、'time.Now' |
| 74–104 | function | `(*TempSubscriptionStore).Get` | *TempSubscriptionStore 的方法，查询或读取与 'get' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 4；goroutine 0；调用 'After'、'delete'、'errors.New'、's.mu.Lock'、's.mu.Unlock'、'time.Now' |
| 107–114 | function | `(*TempSubscriptionStore).cleanupLocked` | *TempSubscriptionStore 的方法，清理与 'cleanup locked' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 0；goroutine 0；调用 'delete'、'now.After'、'time.Now' |
| 117–117 | type | `TempSubscriptionHandler` | 定义 'TempSubscriptionHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 120–122 | function | `NewTempSubscriptionHandler` | 创建并初始化与 'new temp subscription handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 124–131 | function | `(*TempSubscriptionHandler).ServeHTTP` | *TempSubscriptionHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'errors.New'、'h.handleCreate'、'writeError' |
| 134–138 | type | `CreateTempSubRequest` | 定义 'CreateTempSubRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 141–146 | type | `CreateTempSubResponse` | 定义 'CreateTempSubResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 148–187 | function | `(*TempSubscriptionHandler).handleCreate` | *TempSubscriptionHandler 的方法，处理与 'handle create' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 2；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'len'、'tempSubStore.Create'、'w.Header'、'writeError' |
| 190–190 | type | `TempSubscriptionAccessHandler` | 定义 'TempSubscriptionAccessHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 193–195 | function | `NewTempSubscriptionAccessHandler` | 创建并初始化与 'new temp subscription access handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 197–254 | function | `(*TempSubscriptionAccessHandler).ServeHTTP` | *TempSubscriptionAccessHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 5；goroutine 0；调用 'MarshalYAMLWithIndent'、'RemoveUnicodeEscapeQuotes'、'append'、'errors.New'、'http.Error'、'http.NotFound'、'len'、'r.Header.Get'、'string'、'strings.Contains'、'strings.ToLower'、'strings.TrimPrefix'、'strings.TrimSuffix'、'tempSubStore.Get'、'util.ReorderProxyFieldsToNode'、'writeError' |

## `internal/handler/template_v3.go`

依赖：`encoding/json`、`fmt`、`net/http`、`os`、`path/filepath`、`strings`、`github.com/MMWOrg/mmwX-plugins/proxyparser/substore`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 19–21 | type | `TemplateV3Handler` | 定义 'TemplateV3Handler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 24–26 | function | `NewTemplateV3Handler` | 创建并初始化与 'new template v3 handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 29–78 | function | `(*TemplateV3Handler).ServeHTTP` | *TemplateV3Handler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 8；循环 0；返回 7；goroutine 0；调用 'h.handleAnalyzeSubscription'、'h.handleConvertV2Template'、'h.handleGetRegionFilters'、'h.handleListTemplates'、'h.handlePreviewTemplate'、'h.handlePreviewWithTags'、'h.handleProcessTemplate'、'http.Error'、'strings.TrimPrefix' |
| 81–84 | type | `processTemplateRequest` | 定义 'processTemplateRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 87–90 | type | `previewTemplateRequest` | 定义 'previewTemplateRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 93–137 | function | `(*TemplateV3Handler).handleProcessTemplate` | *TemplateV3Handler 的方法，处理与 'handle process template' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 5；goroutine 0；调用 'Decode'、'Encode'、'Set'、'err.Error'、'filepath.Join'、'h.processV3Template'、'json.NewDecoder'、'json.NewEncoder'、'os.IsNotExist'、'os.ReadFile'、'string'、'strings.Contains'、'strings.TrimSpace'、'w.Header'、'writeJSONError' |
| 140–163 | function | `(*TemplateV3Handler).handlePreviewTemplate` | *TemplateV3Handler 的方法，处理与 'handle preview template' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'Decode'、'Encode'、'Set'、'err.Error'、'h.processV3Template'、'json.NewDecoder'、'json.NewEncoder'、'strings.TrimSpace'、'w.Header'、'writeJSONError' |
| 166–266 | function | `(*TemplateV3Handler).handlePreviewWithTags` | *TemplateV3Handler 的方法，处理与 'handle preview with tags' 对应的业务或基础设施操作。 | 分支 14；循环 3；返回 7；goroutine 0；调用 'Decode'、'auth.UsernameFromContext'、'filepath.Join'、'h.repo.GetUserSettings'、'h.repo.ListNodes'、'json.NewDecoder'、'json.Unmarshal'、'len'、'make'、'node.HasAnyTag'、'os.IsNotExist'、'os.ReadFile'、'r.Context'、'sortNodesByNodeOrder'、'strings.Contains'、'writeJSONError' |
| 269–289 | function | `(*TemplateV3Handler).processV3Template` | *TemplateV3Handler 的方法，执行与 'process v3 template' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'injectProxiesIntoSurgeTemplate'、'injectProxiesIntoTemplate'、'looksLikeSurgeTemplate'、'processor.ProcessTemplate'、'substore.NewTemplateV3Processor' |
| 291–299 | function | `looksLikeSurgeTemplate` | 执行与 'looks like surge template' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0；调用 'strings.Split'、'strings.ToLower'、'strings.TrimSpace' |
| 301–303 | function | `isSurgeTemplateFile` | 判断与 'is surge template file' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'strings.HasSuffix'、'strings.ToLower'、'strings.TrimSpace' |
| 305–312 | function | `isSurgeClientType` | 判断与 'is surge client type' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'strings.ToLower'、'strings.TrimSpace' |
| 314–359 | function | `injectProxiesIntoSurgeTemplate` | 执行与 'inject proxies into surge template' 对应的业务或基础设施操作。 | 分支 9；循环 2；返回 3；goroutine 0；调用 'Produce'、'append'、'fmt.Errorf'、'len'、'make'、'strings.EqualFold'、'strings.HasPrefix'、'strings.HasSuffix'、'strings.Join'、'strings.Split'、'strings.TrimRight'、'strings.TrimSpace'、'substore.NewSurgeProducer'、'substore.Proxy' |
| 362–410 | function | `injectProxiesIntoTemplate` | 执行与 'inject proxies into template' 对应的业务或基础设施操作。 | 分支 5；循环 2；返回 5；goroutine 0；调用 'RemoveUnicodeEscapeQuotes'、'append'、'buf.String'、'encoder.Close'、'encoder.Encode'、'encoder.SetIndent'、'len'、'mapToYAMLNode'、'yaml.NewEncoder'、'yaml.Unmarshal' |
| 412–445 | function | `injectRelayGroupsIntoTemplate` | 执行与 'inject relay groups into template' 对应的业务或基础设施操作。 | 分支 6；循环 2；返回 5；goroutine 0；调用 'RemoveUnicodeEscapeQuotes'、'append'、'buf.String'、'encoder.Close'、'encoder.Encode'、'encoder.SetIndent'、'len'、'mapToYAMLNode'、'yaml.NewEncoder'、'yaml.Unmarshal' |
| 448–474 | function | `mapToYAMLNode` | 执行与 'map to yaml node' 对应的业务或基础设施操作。 | 分支 2；循环 2；返回 1；goroutine 0；调用 'addKeyValueToNode'、'make' |
| 477–486 | function | `addKeyValueToNode` | 添加与 'add key value to node' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'anyToYAMLNode'、'append' |
| 489–559 | function | `anyToYAMLNode` | 执行与 'any to yaml node' 对应的业务或基础设施操作。 | 分支 2；循环 2；返回 10；goroutine 0；调用 'anyToYAMLNode'、'append'、'boolToString'、'float64'、'floatToString'、'int'、'int64ToString'、'intToString'、'mapToYAMLNode' |
| 561–578 | function | `intToString` | 执行与 'int to string' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 2；goroutine 0；调用 'append'、'byte'、'string' |
| 580–582 | function | `int64ToString` | 执行与 'int64 to string' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'int'、'intToString' |
| 584–589 | function | `floatToString` | 执行与 'float to string' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'int'、'rune'、'string'、'strings.Replace'、'strings.TrimRight' |
| 591–596 | function | `boolToString` | 执行与 'bool to string' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0 |
| 599–601 | type | `convertV2Request` | 定义 'convertV2Request' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 604–629 | function | `(*TemplateV3Handler).handleConvertV2Template` | *TemplateV3Handler 的方法，处理与 'handle convert v2 template' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'Decode'、'Encode'、'Set'、'err.Error'、'json.NewDecoder'、'json.NewEncoder'、'strings.TrimSpace'、'substore.ConvertACLToV3'、'w.Header'、'writeJSONError' |
| 632–635 | type | `analyzeSubscriptionRequest` | 定义 'analyzeSubscriptionRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 638–705 | function | `(*TemplateV3Handler).handleAnalyzeSubscription` | *TemplateV3Handler 的方法，处理与 'handle analyze subscription' 对应的业务或基础设施操作。 | 分支 10；循环 1；返回 6；goroutine 0；调用 'Decode'、'append'、'appendRuleProvidersToTemplate'、'auth.UsernameFromContext'、'err.Error'、'filepath.Join'、'h.repo.ListNodes'、'json.NewDecoder'、'os.IsNotExist'、'os.ReadFile'、'r.Context'、'string'、'strings.Contains'、'substore.AnalyzeSubscription'、'substore.GenerateV3TemplateFromAnalysis'、'writeJSONError' |
| 709–727 | function | `appendRuleProvidersToTemplate` | 执行与 'append rule providers to template' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'len'、'string'、'strings.TrimRight'、'yaml.Marshal'、'yaml.Unmarshal' |
| 730–736 | function | `(*TemplateV3Handler).handleGetRegionFilters` | *TemplateV3Handler 的方法，处理与 'handle get region filters' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'Encode'、'Set'、'json.NewEncoder'、'w.Header' |
| 740–797 | function | `(*TemplateV3Handler).handleListTemplates` | *TemplateV3Handler 的方法，处理与 'handle list templates' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 1；goroutine 0；调用 'Set'、'append'、'entry.IsDir'、'entry.Name'、'err.Error'、'filepath.Join'、'os.ReadDir'、'os.ReadFile'、'string'、'strings.HasSuffix'、'strings.ReplaceAll'、'strings.ToLower'、'strings.TrimSuffix'、'substore.ExtractTemplateVariables'、'w.Header'、'writeJSONError' |

## `internal/handler/template_v3_rule_providers_test.go`

依赖：`strings`、`testing`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 8–26 | function | `TestAppendRuleProvidersToTemplate` | 执行与 'test append rule providers to template' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 0；goroutine 0；调用 'appendRuleProvidersToTemplate'、'strings.Contains'、't.Fatal'、't.Fatalf' |
| 28–39 | function | `TestAppendRuleProvidersDoesNotDuplicateExistingSection` | 执行与 'test append rule providers does not duplicate existing section' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'appendRuleProvidersToTemplate'、't.Fatal'、't.Fatalf' |

## `internal/handler/templates.go`

依赖：`encoding/json`、`errors`、`fmt`、`io`、`net/http`、`strconv`、`strings`、`time`、`github.com/MMWOrg/mmwX-plugins/proxyparser/substore`、`gopkg.in/yaml.v3`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 18–25 | type | `templateRequest` | 定义 'templateRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 27–37 | type | `templateResponse` | 定义 'templateResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 39–46 | type | `convertRulesRequest` | 定义 'convertRulesRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 48–50 | type | `convertRulesResponse` | 定义 'convertRulesResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 53–68 | function | `NewTemplatesHandler` | 创建并初始化与 'new templates handler' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'errors.New'、'handleCreateTemplate'、'handleListTemplates'、'http.HandlerFunc'、'panic'、'writeError' |
| 58–67 | closure | `NewTemplatesHandler.closure#1` | 供 NewTemplatesHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'errors.New'、'handleCreateTemplate'、'handleListTemplates'、'writeError' |
| 71–102 | function | `NewTemplateHandler` | 创建并初始化与 'new template handler' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 3；goroutine 0；调用 'errors.New'、'handleDeleteTemplate'、'handleGetTemplate'、'handleUpdateTemplate'、'http.HandlerFunc'、'panic'、'strconv.ParseInt'、'strings.TrimPrefix'、'strings.TrimSpace'、'writeError' |
| 76–101 | closure | `NewTemplateHandler.closure#1` | 供 NewTemplateHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 3；循环 0；返回 2；goroutine 0；调用 'errors.New'、'handleDeleteTemplate'、'handleGetTemplate'、'handleUpdateTemplate'、'strconv.ParseInt'、'strings.TrimPrefix'、'strings.TrimSpace'、'writeError' |
| 105–201 | function | `NewTemplateConvertHandler` | 创建并初始化与 'new template convert handler' 对应的业务或基础设施操作。 | 分支 15；循环 1；返回 10；goroutine 0；调用 'Decode'、'err.Error'、'errors.New'、'fetchRemoteContent'、'http.HandlerFunc'、'json.NewDecoder'、'strings.ContainsAny'、'strings.HasPrefix'、'strings.TrimSpace'、'substore.DetectTemplateType'、'substore.GenerateSurgeProxyGroups'、'substore.GenerateSurgeRules'、'substore.GetDefaultClashTemplate'、'substore.GetDefaultSurgeTemplate'、'substo… |
| 106–200 | closure | `NewTemplateConvertHandler.closure#1` | 供 NewTemplateConvertHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 15；循环 1；返回 9；goroutine 0；调用 'Decode'、'err.Error'、'errors.New'、'fetchRemoteContent'、'json.NewDecoder'、'strings.ContainsAny'、'strings.HasPrefix'、'strings.TrimSpace'、'substore.DetectTemplateType'、'substore.GenerateSurgeProxyGroups'、'substore.GenerateSurgeRules'、'substore.GetDefaultClashTemplate'、'substore.GetDefaultSurgeTemplate'、'substore.MergeToSurgeTempl… |
| 205–248 | function | `ensureV2ProxyGroupMembers` | 执行与 'ensure v2 proxy group members' 对应的业务或基础设施操作。 | 分支 8；循环 2；返回 7；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'len'、'make'、'nonEmptyList'、'string'、'strings.HasPrefix'、'strings.ToLower'、'strings.Trim'、'strings.TrimSpace'、'yaml.Marshal'、'yaml.Unmarshal' |
| 250–253 | function | `nonEmptyList` | 执行与 'non empty list' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'len' |
| 255–269 | function | `handleListTemplates` | 处理与 'handle list templates' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 1；goroutine 0；调用 'Encode'、'Set'、'append'、'json.NewEncoder'、'len'、'make'、'r.Context'、'repo.ListTemplates'、'templateToResponse'、'w.Header'、'writeError' |
| 271–284 | function | `handleGetTemplate` | 处理与 'handle get template' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'Encode'、'Set'、'errors.Is'、'json.NewEncoder'、'r.Context'、'repo.GetTemplateByID'、'templateToResponse'、'w.Header'、'writeError' |
| 286–322 | function | `handleCreateTemplate` | 处理与 'handle create template' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 4；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.Is'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.CreateTemplate'、'repo.GetTemplateByID'、'strings.TrimSpace'、'templateToResponse'、'w.Header'、'w.WriteHeader'、'writeError' |
| 324–363 | function | `handleUpdateTemplate` | 处理与 'handle update template' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.Is'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.GetTemplateByID'、'repo.UpdateTemplate'、'strings.TrimSpace'、'templateToResponse'、'w.Header'、'writeError' |
| 365–377 | function | `handleDeleteTemplate` | 处理与 'handle delete template' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'Encode'、'Set'、'errors.Is'、'json.NewEncoder'、'r.Context'、'repo.DeleteTemplate'、'w.Header'、'writeError' |
| 379–391 | function | `templateToResponse` | 执行与 'template to response' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 't.CreatedAt.Format'、't.UpdatedAt.Format' |
| 393–411 | function | `fetchRemoteContent` | 从外部获取与 'fetch remote content' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'client.Get'、'errors.New'、'io.ReadAll'、'resp.Body.Close'、'string' |
| 413–416 | type | `fetchSourceRequest` | 定义 'fetchSourceRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 418–420 | type | `fetchSourceResponse` | 定义 'fetchSourceResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 423–456 | function | `NewTemplateFetchSourceHandler` | 创建并初始化与 'new template fetch source handler' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.New'、'fetchRemoteContent'、'http.HandlerFunc'、'json.NewDecoder'、'json.NewEncoder'、'strings.HasPrefix'、'w.Header'、'writeError' |
| 424–455 | closure | `NewTemplateFetchSourceHandler.closure#1` | 供 NewTemplateFetchSourceHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 5；循环 0；返回 4；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.New'、'fetchRemoteContent'、'json.NewDecoder'、'json.NewEncoder'、'strings.HasPrefix'、'w.Header'、'writeError' |

## `internal/handler/templates_v2_groups_test.go`

依赖：`bytes`、`encoding/json`、`net/http`、`net/http/httptest`、`strings`、`testing`、`github.com/MMWOrg/mmwX-plugins/proxyparser/substore`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 15–24 | function | `TestV2ProxyGroupsContainSelectedNodes` | 执行与 'test v2 proxy groups contain selected nodes' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 0；goroutine 0；调用 'strings.Contains'、'substore.GenerateClashProxyGroups'、'substore.ParseACLConfig'、't.Fatalf' |
| 26–65 | function | `TestV2ConvertEndpointKeepsProxyGroupMembers` | 执行与 'test v2 convert endpoint keeps proxy group members' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 0；goroutine 0；调用 'NewTemplateConvertHandler'、'ServeHTTP'、'aclServer.Close'、'bytes.NewReader'、'http.HandlerFunc'、'httptest.NewRecorder'、'httptest.NewRequest'、'httptest.NewServer'、'json.Marshal'、'json.Unmarshal'、'response.Body.Bytes'、'response.Body.String'、't.Fatal'、't.Fatalf'、'w.Write'、'yaml.Unmarshal' |
| 27–29 | closure | `TestV2ConvertEndpointKeepsProxyGroupMembers.closure#1` | 供 TestV2ConvertEndpointKeepsProxyGroupMembers 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'w.Write' |
| 67–77 | function | `TestEnsureV2ProxyGroupMembersFillsEmptyStaticGroup` | 执行与 'test ensure v2 proxy group members fills empty static group' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 0；goroutine 0；调用 'ensureV2ProxyGroupMembers'、'strings.Contains'、't.Fatal'、't.Fatalf' |
| 79–84 | function | `TestEnsureV2ProxyGroupMembersRejectsInvalidRuleSource` | 执行与 'test ensure v2 proxy group members rejects invalid rule source' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'ensureV2ProxyGroupMembers'、'err.Error'、'strings.Contains'、't.Fatalf' |

## `internal/handler/tls_fingerprint.go`

依赖：`context`、`crypto/sha256`、`crypto/tls`、`crypto/x509`、`encoding/hex`、`encoding/pem`、`errors`、`fmt`、`net`、`strconv`、`strings`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 18–28 | function | `certPEMSha256` | 执行与 'cert pem sha256' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'hex.EncodeToString'、'pem.Decode'、'sha256.Sum256'、'x509.ParseCertificate' |
| 30–61 | function | `fetchPeerCertSha256` | 从外部获取与 'fetch peer cert sha256' 对应的业务或基础设施操作。 | 分支 7；循环 1；返回 4；goroutine 0；调用 'append'、'conn.Close'、'conn.ConnectionState'、'ctx.Deadline'、'errors.New'、'hex.EncodeToString'、'len'、'net.JoinHostPort'、'sha256.Sum256'、'strconv.Itoa'、'strings.Split'、'strings.TrimSpace'、'time.Until'、'tls.DialWithDialer' |

## `internal/handler/tls_fingerprint_test.go`

依赖：`crypto/sha256`、`encoding/hex`、`fmt`、`net/http`、`net/http/httptest`、`strings`、`testing`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 13–29 | function | `TestFetchPeerCertSha256` | 执行与 'test fetch peer cert sha256' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'fetchPeerCertSha256'、'fmt.Sscanf'、'hex.EncodeToString'、'http.HandlerFunc'、'httptest.NewTLSServer'、'server.Close'、'sha256.Sum256'、'strings.Cut'、'strings.TrimPrefix'、't.Context'、't.Fatal'、't.Fatalf' |
| 14–14 | closure | `TestFetchPeerCertSha256.closure#1` | 供 TestFetchPeerCertSha256 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0 |

## `internal/handler/traffic_summary.go`

依赖：`bytes`、`context`、`encoding/json`、`errors`、`fmt`、`miaomiaowu/internal/logger`、`math`、`net/http`、`net/url`、`os`、`path/filepath`、`sort`、`strconv`、`strings`、`time`、`github.com/gorilla/websocket`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 26–26 | const | `bytesPerGigabyte` | 定义 'bytesPerGigabyte' 的不可变协议值、默认值或枚举成员。 |  |
| 28–31 | type | `TrafficSummaryHandler` | 定义 'TrafficSummaryHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 33–36 | type | `trafficSummaryResponse` | 定义 'trafficSummaryResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 38–43 | type | `trafficSummaryMetrics` | 定义 'trafficSummaryMetrics' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 45–48 | type | `trafficDailyUsage` | 定义 'trafficDailyUsage' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 50–60 | type | `batchTrafficResponse` | 定义 'batchTrafficResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 62–69 | function | `NewTrafficSummaryHandler` | 创建并初始化与 'new traffic summary handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'newTrafficSummaryHandler'、'panic' |
| 71–77 | function | `newTrafficSummaryHandler` | 创建并初始化与 'new traffic summary handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0 |
| 79–143 | function | `(*TrafficSummaryHandler).ServeHTTP` | *TrafficSummaryHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 2；goroutine 0；调用 'Set'、'auth.UsernameFromContext'、'bytesToGigabytes'、'errors.Is'、'errors.New'、'h.fetchExternalSubscriptionTraffic'、'h.fetchTotals'、'h.loadHistory'、'h.recordSnapshot'、'logger.Info'、'r.Context'、'roundUpTwoDecimals'、'usagePercentage'、'w.Header'、'w.WriteHeader'、'writeError' |
| 146–211 | function | `(*TrafficSummaryHandler).RecordDailyUsage` | *TrafficSummaryHandler 的方法，执行与 'record daily usage' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 3；goroutine 0；调用 'bytesToGigabytes'、'errors.Is'、'h.fetchTotals'、'h.recordSnapshot'、'h.syncAndFetchExternalSubscriptionTraffic'、'logger.Error'、'logger.Info'、'logger.Warn'、'roundUpTwoDecimals'、'usagePercentage' |
| 215–307 | function | `(*TrafficSummaryHandler).syncAndFetchExternalSubscriptionTraffic` | *TrafficSummaryHandler 的方法，同步与 'sync and fetch external subscription traffic' 对应的业务或基础设施操作。 | 分支 11；循环 1；返回 6；goroutine 0；调用 'bytesToGigabytes'、'h.fetchExternalSubscriptionTrafficInfo'、'h.repo.IsSyncTrafficEnabled'、'h.repo.ListAllExternalSubscriptions'、'h.repo.UpdateExternalSubscription'、'len'、'logger.Info'、'logger.Warn'、'strings.ToLower'、'strings.TrimSpace'、'time.Now'、'updatedSub.Expire.Before'、'updatedSub.Expire.Format' |
| 310–353 | function | `(*TrafficSummaryHandler).fetchExternalSubscriptionTrafficInfo` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch external subscription traffic info' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'ParseTrafficInfoHeader'、'float64'、'fmt.Errorf'、'h.client.Do'、'http.NewRequestWithContext'、'logger.Info'、'req.Header.Set'、'resp.Body.Close'、'resp.Header.Get' |
| 355–466 | function | `(*TrafficSummaryHandler).fetchTotals` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch totals' 对应的业务或基础设施操作。 | 分支 18；循环 4；返回 12；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'h.fetchBatchSummary'、'h.fetchKomariTotals'、'h.fetchNezhaTotals'、'h.fetchNezhaV0Totals'、'h.repo.GetProbeConfig'、'h.repo.GetUserSettings'、'h.repo.ListNodes'、'len'、'logger.Info'、'make'、'strings.TrimSpace' |
| 470–524 | function | `(*TrafficSummaryHandler).fetchTotalsByServerIDs` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch totals by server i ds' 对应的业务或基础设施操作。 | 分支 8；循环 3；返回 9；goroutine 0；调用 'append'、'errors.New'、'fmt.Errorf'、'h.fetchBatchSummary'、'h.fetchKomariTotals'、'h.fetchNezhaTotals'、'h.fetchNezhaV0Totals'、'h.repo.GetProbeConfig'、'len'、'make'、'strings.TrimSpace' |
| 526–704 | function | `(*TrafficSummaryHandler).fetchNezhaTotals` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch nezha totals' 对应的业务或基础设施操作。 | 分支 24；循环 2；返回 10；goroutine 0；调用 'Add'、'base.ResolveReference'、'cancel'、'conn.Close'、'conn.ReadMessage'、'conn.SetReadDeadline'、'context.WithTimeout'、'errors.New'、'fmt.Errorf'、'resp.Body.Close'、'strings.ToLower'、'strings.TrimSpace'、'target.String'、'time.Now'、'url.Parse'、'websocket.DefaultDialer.DialContext' |
| 706–863 | function | `(*TrafficSummaryHandler).fetchNezhaV0Totals` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch nezha v0 totals' 对应的业务或基础设施操作。 | 分支 21；循环 2；返回 6；goroutine 0；调用 'base.ResolveReference'、'decoder.Decode'、'decoder.UseNumber'、'entry.ID.Int64'、'errors.New'、'fmt.Errorf'、'h.client.Do'、'http.NewRequestWithContext'、'json.NewDecoder'、'len'、'make'、'resp.Body.Close'、'strconv.FormatInt'、'strings.TrimSpace'、'target.String'、'url.Parse' |
| 865–872 | function | `(*TrafficSummaryHandler).fetchBatchSummary` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch batch summary' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'fmt.Errorf'、'h.fetchBatchTraffic'、'strings.TrimSpace'、'url.Parse' |
| 874–1015 | function | `(*TrafficSummaryHandler).fetchKomariTotals` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch komari totals' 对应的业务或基础设施操作。 | 分支 16；循环 2；返回 8；goroutine 0；调用 'base.ResolveReference'、'bytes.NewReader'、'decoder.Decode'、'decoder.UseNumber'、'errors.New'、'fmt.Errorf'、'h.client.Do'、'http.NewRequestWithContext'、'json.Marshal'、'json.NewDecoder'、'make'、'req.Header.Set'、'resp.Body.Close'、'strings.TrimSpace'、'target.String'、'url.Parse' |
| 1017–1087 | function | `(*TrafficSummaryHandler).fetchBatchTraffic` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch batch traffic' 对应的业务或基础设施操作。 | 分支 7；循环 1；返回 8；goroutine 0；调用 'base.ResolveReference'、'bytes.NewReader'、'bytesToGigabytes'、'decoder.Decode'、'decoder.UseNumber'、'errors.New'、'h.client.Do'、'http.NewRequestWithContext'、'json.Marshal'、'json.NewDecoder'、'jsonNumberToInt64'、'len'、'logger.Info'、'req.Header.Set'、'resp.Body.Close'、'target.String' |
| 1089–1103 | function | `jsonNumberToInt64` | 执行与 'json number to int64' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'int64'、'n.Float64'、'n.Int64' |
| 1105–1107 | function | `roundUpTwoDecimals` | 执行与 'round up two decimals' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'math.Ceil' |
| 1109–1115 | function | `bytesToGigabytes` | 执行与 'bytes to gigabytes' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'float64' |
| 1117–1123 | function | `usagePercentage` | 执行与 'usage percentage' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'float64' |
| 1125–1131 | function | `(*TrafficSummaryHandler).recordSnapshot` | *TrafficSummaryHandler 的方法，执行与 'record snapshot' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'h.repo.RecordDaily'、'time.Now' |
| 1133–1174 | function | `(*TrafficSummaryHandler).loadHistory` | *TrafficSummaryHandler 的方法，加载与 'load history' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 5；goroutine 0；调用 'append'、'bytesToGigabytes'、'h.repo.ListRecent'、'len'、'make'、'record.Date.Format'、'records.Date.Before'、'roundUpTwoDecimals'、'sort.SliceStable' |
| 1147–1149 | closure | `loadHistory.closure#1` | 供 loadHistory 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'records.Date.Before' |
| 1178–1279 | function | `(*TrafficSummaryHandler).fetchExternalSubscriptionTraffic` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch external subscription traffic' 对应的业务或基础设施操作。 | 分支 11；循环 3；返回 5；goroutine 0；调用 'GetExternalSubscriptionsFromFile'、'filepath.Join'、'h.repo.GetUserSettings'、'h.repo.ListExternalSubscriptions'、'h.repo.ListSubscribeFiles'、'len'、'logger.Info'、'make'、'os.ReadFile'、'strings.ToLower'、'strings.TrimSpace'、'sub.Expire.Before'、'sub.Expire.Format'、'time.Now' |
| 1281–1403 | function | `(*TrafficSummaryHandler).fetchNezhaV0TotalsViaWebSocket` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch nezha v0 totals via web socket' 对应的业务或基础设施操作。 | 分支 17；循环 1；返回 9；goroutine 0；调用 'Add'、'bytes.TrimSpace'、'cancel'、'conn.Close'、'conn.ReadMessage'、'conn.SetReadDeadline'、'context.WithTimeout'、'errors.New'、'fmt.Errorf'、'len'、'resp.Body.Close'、'strings.ToLower'、'target.String'、'time.Now'、'websocket.DefaultDialer.DialContext'、'wsBase.ResolveReference' |
| 1405–1411 | function | `writeError` | 执行与 'write error' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'Encode'、'Set'、'err.Error'、'json.NewEncoder'、'w.Header'、'w.WriteHeader' |
| 1415–1491 | function | `(*TrafficSummaryHandler).HandleSubscribeTraffic` | *TrafficSummaryHandler 的方法，处理与 'handle subscribe traffic' 对应的业务或基础设施操作。 | 分支 9；循环 1；返回 2；goroutine 0；调用 'append'、'bytesToGigabytes'、'errors.New'、'h.fetchTotals'、'h.fetchTotalsByServerIDs'、'h.repo.ListSubscribeFiles'、'int64'、'r.Context'、'respondJSON'、'roundUpTwoDecimals'、'strings.Split'、'writeError' |
| 1494–1498 | type | `ServerTraffic` | 定义 'ServerTraffic' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 1501–1526 | function | `(*TrafficSummaryHandler).FetchTrafficSummaryForNotify` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch traffic summary for notify' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 1；goroutine 0；调用 'bytesToGigabytes'、'h.fetchExternalSubsForNotify'、'h.fetchPerServerTraffic'、'h.fetchTotals'、'h.repo.GetProbeConfig'、'int64'、'len'、'roundUpTwoDecimals' |
| 1528–1562 | function | `(*TrafficSummaryHandler).fetchExternalSubsForNotify` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch external subs for notify' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 3；goroutine 0；调用 'append'、'bytesToGigabytes'、'h.repo.IsSyncTrafficEnabled'、'h.repo.ListAllExternalSubscriptions'、'len'、'roundUpTwoDecimals'、'strings.ToLower'、'strings.TrimSpace'、'sub.Expire.Before'、'time.Now' |
| 1564–1577 | function | `(*TrafficSummaryHandler).fetchPerServerTraffic` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch per server traffic' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 5；goroutine 0；调用 'h.fetchDstatusPerServer'、'h.fetchKomariPerServer'、'h.fetchNezhaPerServer'、'h.fetchNezhaV0PerServer' |
| 1579–1671 | function | `(*TrafficSummaryHandler).fetchNezhaPerServer` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch nezha per server' 对应的业务或基础设施操作。 | 分支 13；循环 1；返回 10；goroutine 0；调用 'Add'、'base.ResolveReference'、'bytes.TrimSpace'、'cancel'、'conn.Close'、'conn.ReadMessage'、'conn.SetReadDeadline'、'context.WithTimeout'、'len'、'resp.Body.Close'、'strings.ToLower'、'strings.TrimSpace'、'target.String'、'time.Now'、'url.Parse'、'websocket.DefaultDialer.DialContext' |
| 1673–1757 | function | `(*TrafficSummaryHandler).fetchNezhaV0PerServer` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch nezha v0 per server' 对应的业务或基础设施操作。 | 分支 11；循环 1；返回 8；goroutine 0；调用 'Add'、'base.ResolveReference'、'bytes.TrimSpace'、'cancel'、'conn.Close'、'conn.ReadMessage'、'conn.SetReadDeadline'、'context.WithTimeout'、'len'、'resp.Body.Close'、'strings.ToLower'、'strings.TrimSpace'、'target.String'、'time.Now'、'url.Parse'、'websocket.DefaultDialer.DialContext' |
| 1759–1807 | function | `(*TrafficSummaryHandler).fetchDstatusPerServer` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch dstatus per server' 对应的业务或基础设施操作。 | 分支 6；循环 3；返回 4；goroutine 0；调用 'Decode'、'append'、'bytesToGigabytes'、'data.Monthly.Used.Int64'、'fmt.Sprintf'、'h.client.Get'、'json.NewDecoder'、'len'、'make'、'resp.Body.Close'、'roundUpTwoDecimals'、'strings.Join'、'strings.TrimRight'、'strings.TrimSpace' |
| 1809–1852 | function | `(*TrafficSummaryHandler).fetchKomariPerServer` | *TrafficSummaryHandler 的方法，从外部获取与 'fetch komari per server' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 4；goroutine 0；调用 'buildPerServerResult'、'decoder.Decode'、'decoder.UseNumber'、'fmt.Sprintf'、'h.client.Get'、'json.NewDecoder'、'jsonNumberToInt64'、'make'、'normalizeServerID'、'resp.Body.Close'、'strings.TrimRight'、'strings.TrimSpace' |
| 1854–1865 | function | `normalizeServerID` | 规范化与 'normalize server id' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'id.Float64'、'id.Int64'、'id.String'、'int64'、'math.Round'、'strconv.FormatInt'、'strings.ContainsAny'、'strings.TrimSpace' |
| 1867–1902 | function | `buildPerServerResult` | 构建与 'build per server result' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 1；goroutine 0；调用 'append'、'bytesToGigabytes'、'roundUpTwoDecimals'、'strings.ToLower'、'strings.TrimSpace' |

## `internal/handler/turnstile_settings.go`

依赖：`encoding/json`、`net/http`、`strings`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 11–13 | type | `TurnstileSettingsHandler` | 定义 'TurnstileSettingsHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 15–17 | function | `NewTurnstileSettingsHandler` | 创建并初始化与 'new turnstile settings handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 19–66 | function | `(*TurnstileSettingsHandler).ServeHTTP` | *TurnstileSettingsHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 9；循环 0；返回 6；goroutine 0；调用 'Decode'、'h.repo.GetSystemSetting'、'h.repo.SetSystemSetting'、'json.NewDecoder'、'len'、'methodNotAllowed'、'r.Context'、'respondJSON'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |

## `internal/handler/two_factor.go`

依赖：`encoding/json`、`errors`、`fmt`、`net/http`、`strings`、`time`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/logger`、`miaomiaowu/internal/notify`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 17–53 | function | `NewTwoFactorLoginHandler` | 创建并初始化与 'new two factor login handler' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'Decode'、'auth.ValidateTOTPCode'、'errors.New'、'http.HandlerFunc'、'issueLoginSession'、'json.NewDecoder'、'r.Context'、'repo.GetUser'、'strings.TrimSpace'、'tfStore.Consume'、'tfStore.Validate'、'writeError' |
| 18–52 | closure | `NewTwoFactorLoginHandler.closure#1` | 供 NewTwoFactorLoginHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'Decode'、'auth.ValidateTOTPCode'、'errors.New'、'issueLoginSession'、'json.NewDecoder'、'r.Context'、'repo.GetUser'、'strings.TrimSpace'、'tfStore.Consume'、'tfStore.Validate'、'writeError' |
| 55–103 | function | `NewRecoveryLoginHandler` | 创建并初始化与 'new recovery login handler' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 7；goroutine 0；调用 'Decode'、'auth.ValidateRecoveryCode'、'errors.New'、'http.HandlerFunc'、'issueLoginSession'、'json.NewDecoder'、'logger.Warn'、'parseRecoveryCodes'、'r.Context'、'repo.DisableUserTOTP'、'repo.GetUser'、'tfStore.Consume'、'tfStore.Validate'、'writeError' |
| 56–102 | closure | `NewRecoveryLoginHandler.closure#1` | 供 NewRecoveryLoginHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 7；循环 0；返回 6；goroutine 0；调用 'Decode'、'auth.ValidateRecoveryCode'、'errors.New'、'issueLoginSession'、'json.NewDecoder'、'logger.Warn'、'parseRecoveryCodes'、'r.Context'、'repo.DisableUserTOTP'、'repo.GetUser'、'tfStore.Consume'、'tfStore.Validate'、'writeError' |
| 105–124 | function | `NewTwoFactorStatusHandler` | 创建并初始化与 'new two factor status handler' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'Encode'、'Set'、'auth.UsernameFromContext'、'errors.New'、'http.HandlerFunc'、'json.NewEncoder'、'r.Context'、'repo.GetUser'、'w.Header'、'writeError' |
| 106–123 | closure | `NewTwoFactorStatusHandler.closure#1` | 供 NewTwoFactorStatusHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'Encode'、'Set'、'auth.UsernameFromContext'、'errors.New'、'json.NewEncoder'、'r.Context'、'repo.GetUser'、'w.Header'、'writeError' |
| 126–165 | function | `NewTwoFactorSetupHandler` | 创建并初始化与 'new two factor setup handler' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'Decode'、'Encode'、'Set'、'auth.GenerateTOTPKey'、'auth.UsernameFromContext'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'json.NewEncoder'、'key.Secret'、'key.URL'、'manager.ValidatePassword'、'r.Context'、'repo.SetUserTOTPSecret'、'w.Header'、'writeError' |
| 127–164 | closure | `NewTwoFactorSetupHandler.closure#1` | 供 NewTwoFactorSetupHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'Decode'、'Encode'、'Set'、'auth.GenerateTOTPKey'、'auth.UsernameFromContext'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'key.Secret'、'key.URL'、'manager.ValidatePassword'、'r.Context'、'repo.SetUserTOTPSecret'、'w.Header'、'writeError' |
| 167–216 | function | `NewTwoFactorVerifySetupHandler` | 创建并初始化与 'new two factor verify setup handler' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 8；goroutine 0；调用 'Decode'、'Set'、'auth.GenerateRecoveryCodes'、'auth.UsernameFromContext'、'auth.ValidateTOTPCode'、'errors.New'、'http.HandlerFunc'、'json.Marshal'、'json.NewDecoder'、'r.Context'、'repo.EnableUserTOTP'、'repo.GetUser'、'string'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 168–215 | closure | `NewTwoFactorVerifySetupHandler.closure#1` | 供 NewTwoFactorVerifySetupHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 7；循环 0；返回 7；goroutine 0；调用 'Decode'、'Encode'、'Set'、'auth.GenerateRecoveryCodes'、'auth.UsernameFromContext'、'auth.ValidateTOTPCode'、'errors.New'、'json.Marshal'、'json.NewDecoder'、'r.Context'、'repo.EnableUserTOTP'、'repo.GetUser'、'string'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 218–258 | function | `NewTwoFactorDisableHandler` | 创建并初始化与 'new two factor disable handler' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 7；goroutine 0；调用 'Decode'、'Encode'、'Set'、'auth.UsernameFromContext'、'auth.ValidateTOTPCode'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.DisableUserTOTP'、'repo.GetUser'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 219–257 | closure | `NewTwoFactorDisableHandler.closure#1` | 供 NewTwoFactorDisableHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 6；循环 0；返回 6；goroutine 0；调用 'Decode'、'Encode'、'Set'、'auth.UsernameFromContext'、'auth.ValidateTOTPCode'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.DisableUserTOTP'、'repo.GetUser'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 260–307 | function | `issueLoginSession` | 判断与 'issue login session' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 1；goroutine 1；调用 'Encode'、'GetClientIP'、'GetNotifier'、'Set'、'expiry.Format'、'fmt.Sprintf'、'json.NewEncoder'、'logger.Info'、'logger.Warn'、'n.Send'、'r.Context'、'repo.CreateSession'、'tokens.IssueWithTTL'、'w.Header'、'w.WriteHeader'、'writeError' |
| 309–315 | function | `parseRecoveryCodes` | 解析与 'parse recovery codes' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'fmt.Errorf'、'json.Unmarshal' |

## `internal/handler/update.go`

依赖：`encoding/json`、`errors`、`fmt`、`io`、`net/http`、`os`、`os/exec`、`path/filepath`、`runtime`、`sort`、`strings`、`syscall`、`time`、`miaomiaowu/internal/logger`、`miaomiaowu/internal/version`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 23–23 | const | `githubRepo` | 定义 'githubRepo' 的不可变协议值、默认值或枚举成员。 |  |
| 24–24 | const | `githubAPIURL` | 定义 'githubAPIURL' 的不可变协议值、默认值或枚举成员。 |  |
| 28–35 | type | `UpdateInfo` | 定义 'UpdateInfo' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 38–42 | type | `UpdateProgress` | 定义 'UpdateProgress' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 45–53 | type | `GitHubRelease` | 定义 'GitHubRelease' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 56–73 | function | `NewUpdateCheckHandler` | 创建并初始化与 'new update check handler' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'Encode'、'Set'、'checkLatestVersion'、'errors.New'、'fmt.Errorf'、'http.HandlerFunc'、'json.NewEncoder'、'w.Header'、'w.WriteHeader'、'writeUpdateError' |
| 57–72 | closure | `NewUpdateCheckHandler.closure#1` | 供 NewUpdateCheckHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'Encode'、'Set'、'checkLatestVersion'、'errors.New'、'fmt.Errorf'、'json.NewEncoder'、'w.Header'、'w.WriteHeader'、'writeUpdateError' |
| 76–150 | function | `NewUpdateApplyHandler` | 创建并初始化与 'new update apply handler' 对应的业务或基础设施操作。 | 分支 9；循环 0；返回 9；goroutine 1；调用 'Set'、'backupBinary'、'checkLatestVersion'、'downloadBinary'、'errors.New'、'fmt.Errorf'、'getUpdateTargetPath'、'http.HandlerFunc'、'logger.Info'、'logger.Warn'、'os.Chmod'、'os.Remove'、'replaceBinary'、'w.Header'、'w.WriteHeader'、'writeUpdateError' |
| 77–149 | closure | `NewUpdateApplyHandler.closure#1` | 供 NewUpdateApplyHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 9；循环 0；返回 8；goroutine 1；调用 'Encode'、'Set'、'backupBinary'、'checkLatestVersion'、'downloadBinary'、'errors.New'、'fmt.Errorf'、'getUpdateTargetPath'、'logger.Info'、'logger.Warn'、'os.Chmod'、'os.Remove'、'replaceBinary'、'w.Header'、'w.WriteHeader'、'writeUpdateError' |
| 145–148 | closure | `NewUpdateApplyHandler.closure#2` | 供 NewUpdateApplyHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'restartSelf'、'time.Sleep' |
| 153–257 | function | `NewUpdateApplySSEHandler` | 创建并初始化与 'new update apply sse handler' 对应的业务或基础设施操作。 | 分支 10；循环 0；返回 9；goroutine 1；调用 'Set'、'backupBinary'、'checkLatestVersion'、'downloadBinaryWithProgressAndRetry'、'flusher.Flush'、'fmt.Fprintf'、'fmt.Sprintf'、'getUpdateTargetPath'、'http.Error'、'http.HandlerFunc'、'int'、'json.Marshal'、'logger.Info'、'os.Remove'、'sendProgress'、'w.Header' |
| 154–256 | closure | `NewUpdateApplySSEHandler.closure#1` | 供 NewUpdateApplySSEHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 10；循环 0；返回 8；goroutine 1；调用 'Set'、'backupBinary'、'checkLatestVersion'、'downloadBinaryWithProgressAndRetry'、'flusher.Flush'、'fmt.Fprintf'、'fmt.Sprintf'、'getUpdateTargetPath'、'http.Error'、'int'、'json.Marshal'、'logger.Info'、'logger.Warn'、'os.Remove'、'sendProgress'、'w.Header' |
| 168–173 | closure | `NewUpdateApplySSEHandler.closure#2` | 供 NewUpdateApplySSEHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'flusher.Flush'、'fmt.Fprintf'、'json.Marshal' |
| 199–206 | closure | `NewUpdateApplySSEHandler.closure#3` | 供 NewUpdateApplySSEHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'fmt.Sprintf'、'int'、'sendProgress' |
| 206–210 | closure | `NewUpdateApplySSEHandler.closure#4` | 供 NewUpdateApplySSEHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'sendProgress' |
| 252–255 | closure | `NewUpdateApplySSEHandler.closure#5` | 供 NewUpdateApplySSEHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'restartSelf'、'time.Sleep' |
| 260–316 | function | `checkLatestVersion` | 检查与 'check latest version' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 5；goroutine 0；调用 'Decode'、'client.Do'、'compareVersions'、'fmt.Errorf'、'fmt.Sprintf'、'http.NewRequest'、'json.NewDecoder'、'logger.Debug'、'logger.Error'、'req.Header.Set'、'resp.Body.Close'、'strings.TrimPrefix' |
| 319–340 | function | `compareVersions` | 执行与 'compare versions' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 3；goroutine 0；调用 'len'、'parseVersion' |
| 343–353 | function | `parseVersion` | 解析与 'parse version' 对应的业务或基础设施操作。 | 分支 0；循环 1；返回 1；goroutine 0；调用 'fmt.Sscanf'、'len'、'make'、'strings.Split'、'strings.TrimPrefix' |
| 357–357 | const | `githubProxyURL` | 定义 'githubProxyURL' 的不可变协议值、默认值或枚举成员。 |  |
| 359–361 | function | `downloadBinary` | 执行与 'download binary' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'downloadBinaryWithProgress' |
| 365–367 | function | `downloadBinaryWithProgress` | 执行与 'download binary with progress' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'downloadBinaryWithProgressAndRetry' |
| 370–394 | function | `downloadBinaryWithProgressAndRetry` | 执行与 'download binary with progress and retry' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'downloadBinaryDirect'、'fmt.Errorf'、'logger.Info'、'logger.Warn'、'onRetry' |
| 397–451 | function | `downloadBinaryDirect` | 执行与 'download binary direct' 对应的业务或基础设施操作。 | 分支 9；循环 1；返回 7；goroutine 0；调用 'client.Get'、'fmt.Errorf'、'int64'、'io.Copy'、'make'、'onProgress'、'os.CreateTemp'、'os.Remove'、'resp.Body.Close'、'resp.Body.Read'、'tempFile.Close'、'tempFile.Name'、'tempFile.Write' |
| 454–475 | function | `getUpdateTargetPath` | 查询或读取与 'get update target path' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'filepath.EvalSymlinks'、'isDocker'、'os.Executable'、'os.MkdirAll' |
| 478–496 | function | `isDocker` | 判断与 'is docker' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'os.Getenv'、'os.ReadFile'、'os.Stat'、'string'、'strings.Contains' |
| 499–520 | function | `replaceBinary` | 执行与 'replace binary' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'copyFile'、'os.IsNotExist'、'os.Remove'、'os.Rename' |
| 523–542 | function | `copyFile` | 执行与 'copy file' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'dstFile.Close'、'dstFile.Sync'、'io.Copy'、'os.Create'、'os.Open'、'srcFile.Close' |
| 544–544 | const | `maxBackups` | 定义 'maxBackups' 的不可变协议值、默认值或枚举成员。 |  |
| 546–572 | function | `backupBinary` | 执行与 'backup binary' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 4；goroutine 0；调用 'Format'、'cleanOldBackups'、'copyFile'、'filepath.Join'、'fmt.Errorf'、'fmt.Sprintf'、'isDocker'、'logger.Info'、'os.MkdirAll'、'os.Stat'、'time.Now' |
| 574–600 | function | `cleanOldBackups` | 执行与 'clean old backups' 对应的业务或基础设施操作。 | 分支 4；循环 2；返回 2；goroutine 0；调用 'append'、'e.IsDir'、'e.Name'、'filepath.Join'、'len'、'logger.Info'、'logger.Warn'、'os.ReadDir'、'os.Remove'、'sort.Strings'、'strings.HasPrefix' |
| 603–626 | function | `restartSelf` | 执行与 'restart self' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'cmd.Start'、'exec.Command'、'logger.Error'、'logger.Info'、'logger.Warn'、'os.Environ'、'os.Exit'、'syscall.Exec' |
| 628–634 | function | `writeUpdateError` | 执行与 'write update error' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'Encode'、'Set'、'err.Error'、'json.NewEncoder'、'w.Header'、'w.WriteHeader' |

## `internal/handler/user_config.go`

依赖：`context`、`encoding/json`、`errors`、`fmt`、`net/http`、`net/url`、`strings`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/notify`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 17–55 | type | `userConfigRequest` | 定义 'userConfigRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 57–95 | type | `userConfigResponse` | 定义 'userConfigResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 97–118 | function | `NewUserConfigHandler` | 创建并初始化与 'new user config handler' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 2；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'handleGetUserConfig'、'handleUpdateUserConfig'、'http.HandlerFunc'、'panic'、'r.Context'、'strings.TrimSpace'、'writeError' |
| 102–117 | closure | `NewUserConfigHandler.closure#1` | 供 NewUserConfigHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'handleGetUserConfig'、'handleUpdateUserConfig'、'r.Context'、'strings.TrimSpace'、'writeError' |
| 120–221 | function | `handleGetUserConfig` | 处理与 'handle get user config' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'Encode'、'Set'、'errors.Is'、'fmt.Errorf'、'json.NewEncoder'、'r.Context'、'repo.GetSystemConfig'、'repo.GetUserSettings'、'w.Header'、'w.WriteHeader'、'writeError' |
| 223–426 | function | `handleUpdateUserConfig` | 处理与 'handle update user config' 对应的业务或基础设施操作。 | 分支 25；循环 0；返回 8；goroutine 1；调用 'Decode'、'GetBruteForceProtector'、'GetLoginRateLimiter'、'GetSubscriptionRateLimiter'、'bfp.UpdateConfig'、'errors.New'、'fmt.Errorf'、'json.NewDecoder'、'r.Context'、'repo.GetSystemConfig'、'repo.UpdateSystemConfig'、'repo.UpsertUserSettings'、'rl.UpdateConfig'、'strings.TrimSpace'、'validateProxyGroupsSourceURL'、'writeError' |
| 430–445 | function | `validateProxyGroupsSourceURL` | 校验与 'validate proxy groups source url' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'errors.New'、'fmt.Errorf'、'url.ParseRequestURI' |

## `internal/handler/user_default_template.go`

依赖：`encoding/json`、`errors`、`net/http`、`os`、`path/filepath`、`strings`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 15–15 | type | `userDefaultTemplateHandler` | 定义 'userDefaultTemplateHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 17–19 | function | `NewUserDefaultTemplateHandler` | 创建并初始化与 'new user default template handler' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 21–79 | function | `(*userDefaultTemplateHandler).ServeHTTP` | *userDefaultTemplateHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 11；循环 1；返回 9；goroutine 0；调用 'Decode'、'auth.UsernameFromContext'、'errors.Is'、'errors.New'、'filepath.Base'、'h.repo.GetUserSettings'、'http.Error'、'json.NewDecoder'、'os.Stat'、'r.Context'、'respondJSON'、'strings.HasSuffix'、'strings.ToLower'、'strings.TrimSpace'、'writeBadRequest'、'writeError' |

## `internal/handler/user_settings.go`

依赖：`encoding/json`、`errors`、`net/http`、`strings`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 13–18 | type | `userSettingsRequest` | 定义 'userSettingsRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 20–22 | type | `userSettingsResponse` | 定义 'userSettingsResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 24–116 | function | `NewUserSettingsHandler` | 创建并初始化与 'new user settings handler' 对应的业务或基础设施操作。 | 分支 14；循环 0；返回 12；goroutine 0；调用 'Decode'、'auth.UsernameFromContext'、'err.Error'、'errors.Is'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'panic'、'r.Context'、'repo.GetUser'、'repo.RenameUser'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace'、'tokens.UpdateUsername'、'writeError' |
| 29–115 | closure | `NewUserSettingsHandler.closure#1` | 供 NewUserSettingsHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 13；循环 0；返回 11；goroutine 0；调用 'Decode'、'Set'、'auth.UsernameFromContext'、'err.Error'、'errors.Is'、'errors.New'、'json.NewDecoder'、'r.Context'、'repo.GetUser'、'repo.RenameUser'、'repo.UpdateUserProfile'、'strings.Contains'、'strings.ToLower'、'strings.TrimSpace'、'tokens.UpdateUsername'、'writeError' |

## `internal/handler/user_subscriptions.go`

依赖：`encoding/json`、`errors`、`net/http`、`strings`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–14 | type | `userSubscriptionsHandler` | 定义 'userSubscriptionsHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 20–26 | function | `NewUserSubscriptionsHandler` | 创建并初始化与 'new user subscriptions handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'panic' |
| 28–50 | function | `(*userSubscriptionsHandler).ServeHTTP` | *userSubscriptionsHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 2；goroutine 0；调用 'errors.New'、'h.handleGet'、'h.handleUpdate'、'len'、'methodNotAllowed'、'strings.Split'、'strings.TrimPrefix'、'writeError' |
| 52–62 | function | `(*userSubscriptionsHandler).handleGet` | *userSubscriptionsHandler 的方法，处理与 'handle get' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'h.repo.GetUserSubscriptionIDs'、'r.Context'、'respondJSON'、'writeError' |
| 64–102 | function | `(*userSubscriptionsHandler).handleUpdate` | *userSubscriptionsHandler 的方法，处理与 'handle update' 对应的业务或基础设施操作。 | 分支 4；循环 2；返回 3；goroutine 0；调用 'Decode'、'append'、'h.repo.ListSubscribeFiles'、'h.repo.SetUserSubscriptions'、'json.NewDecoder'、'len'、'make'、'r.Context'、'respondJSON'、'writeBadRequest'、'writeError' |

## `internal/handler/user_token.go`

依赖：`encoding/json`、`errors`、`net/http`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–14 | type | `userTokenHandler` | 定义 'userTokenHandler' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 16–18 | type | `userTokenResponse` | 定义 'userTokenResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 21–27 | function | `NewUserTokenHandler` | 创建并初始化与 'new user token handler' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'panic' |
| 29–44 | function | `(*userTokenHandler).ServeHTTP` | *userTokenHandler 的方法，提供 HTTP 服务与 'serve http' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'auth.UsernameFromContext'、'errors.New'、'h.handleGet'、'h.handleReset'、'r.Context'、'writeError' |
| 46–54 | function | `(*userTokenHandler).handleGet` | *userTokenHandler 的方法，处理与 'handle get' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'h.repo.GetOrCreateUserToken'、'r.Context'、'respondWithToken'、'writeError' |
| 56–64 | function | `(*userTokenHandler).handleReset` | *userTokenHandler 的方法，处理与 'handle reset' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'h.repo.ResetUserToken'、'r.Context'、'respondWithToken'、'writeError' |
| 66–70 | function | `respondWithToken` | 执行与 'respond with token' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'Encode'、'Set'、'json.NewEncoder'、'w.Header'、'w.WriteHeader' |

## `internal/handler/users.go`

依赖：`crypto/rand`、`encoding/json`、`errors`、`net/http`、`strings`、`golang.org/x/crypto/bcrypt`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 15–24 | type | `userEntry` | 定义 'userEntry' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 26–29 | type | `userStatusRequest` | 定义 'userStatusRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 31–34 | type | `userResetRequest` | 定义 'userResetRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 36–39 | type | `userResetResponse` | 定义 'userResetResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 41–47 | type | `userCreateRequest` | 定义 'userCreateRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 49–55 | type | `userCreateResponse` | 定义 'userCreateResponse' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 57–87 | function | `NewUserListHandler` | 创建并初始化与 'new user list handler' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 2；goroutine 0；调用 'Encode'、'Set'、'append'、'http.HandlerFunc'、'json.NewEncoder'、'len'、'make'、'panic'、'r.Context'、'repo.GetUserCustomShortCode'、'repo.ListUsers'、'w.Header'、'writeError' |
| 62–86 | closure | `NewUserListHandler.closure#1` | 供 NewUserListHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 1；返回 1；goroutine 0；调用 'Encode'、'Set'、'append'、'json.NewEncoder'、'len'、'make'、'r.Context'、'repo.GetUserCustomShortCode'、'repo.ListUsers'、'w.Header'、'writeError' |
| 89–140 | function | `NewUserStatusHandler` | 创建并初始化与 'new user status handler' 对应的业务或基础设施操作。 | 分支 9；循环 0；返回 9；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.Is'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'json.NewEncoder'、'panic'、'r.Context'、'repo.GetUser'、'repo.UpdateUserStatus'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 94–139 | closure | `NewUserStatusHandler.closure#1` | 供 NewUserStatusHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 8；循环 0；返回 8；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.Is'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.GetUser'、'repo.UpdateUserStatus'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 142–209 | function | `NewUserResetPasswordHandler` | 创建并初始化与 'new user reset password handler' 对应的业务或基础设施操作。 | 分支 12；循环 0；返回 11；goroutine 0；调用 'Decode'、'Set'、'bcrypt.GenerateFromPassword'、'errors.Is'、'errors.New'、'generateRandomPassword'、'http.HandlerFunc'、'json.NewDecoder'、'panic'、'r.Context'、'repo.GetUser'、'repo.UpdateUserPassword'、'string'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 147–208 | closure | `NewUserResetPasswordHandler.closure#1` | 供 NewUserResetPasswordHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 11；循环 0；返回 10；goroutine 0；调用 'Decode'、'Encode'、'Set'、'bcrypt.GenerateFromPassword'、'errors.Is'、'errors.New'、'generateRandomPassword'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.GetUser'、'repo.UpdateUserPassword'、'string'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 211–278 | function | `NewUserCreateHandler` | 创建并初始化与 'new user create handler' 对应的业务或基础设施操作。 | 分支 10；循环 0；返回 8；goroutine 0；调用 'Decode'、'Encode'、'Set'、'bcrypt.GenerateFromPassword'、'errors.Is'、'errors.New'、'generateRandomPassword'、'http.HandlerFunc'、'json.NewDecoder'、'panic'、'r.Context'、'repo.CreateUser'、'string'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 216–277 | closure | `NewUserCreateHandler.closure#1` | 供 NewUserCreateHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 9；循环 0；返回 7；goroutine 0；调用 'Decode'、'Encode'、'Set'、'bcrypt.GenerateFromPassword'、'errors.Is'、'errors.New'、'generateRandomPassword'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.CreateUser'、'string'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 280–282 | type | `userDeleteRequest` | 定义 'userDeleteRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 284–335 | function | `NewUserDeleteHandler` | 创建并初始化与 'new user delete handler' 对应的业务或基础设施操作。 | 分支 9；循环 0；返回 9；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.Is'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'json.NewEncoder'、'panic'、'r.Context'、'repo.DeleteUser'、'repo.GetUser'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 289–334 | closure | `NewUserDeleteHandler.closure#1` | 供 NewUserDeleteHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 8；循环 0；返回 8；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.Is'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.DeleteUser'、'repo.GetUser'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 337–350 | function | `generateRandomPassword` | 生成与 'generate random password' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 2；goroutine 0；调用 'int'、'len'、'make'、'rand.Read'、'string' |
| 352–355 | type | `userRemarkRequest` | 定义 'userRemarkRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 357–388 | function | `NewUserRemarkHandler` | 创建并初始化与 'new user remark handler' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'json.NewEncoder'、'panic'、'r.Context'、'repo.UpdateUserRemark'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 362–387 | closure | `NewUserRemarkHandler.closure#1` | 供 NewUserRemarkHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 4；循环 0；返回 4；goroutine 0；调用 'Decode'、'Encode'、'Set'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'r.Context'、'repo.UpdateUserRemark'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 390–393 | type | `userCustomShortCodeRequest` | 定义 'userCustomShortCodeRequest' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 395–455 | function | `NewUserCustomShortCodeHandler` | 创建并初始化与 'new user custom short code handler' 对应的业务或基础设施操作。 | 分支 11；循环 1；返回 8；goroutine 0；调用 'Decode'、'GetSilentModeManager'、'Set'、'err.Error'、'errors.Is'、'errors.New'、'http.HandlerFunc'、'json.NewDecoder'、'm.InvalidateShortLinkCache'、'panic'、'r.Context'、'repo.GetAllUserShortCodes'、'repo.UpdateUserCustomShortCode'、'strings.TrimSpace'、'w.Header'、'writeError' |
| 400–454 | closure | `NewUserCustomShortCodeHandler.closure#1` | 供 NewUserCustomShortCodeHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 10；循环 1；返回 7；goroutine 0；调用 'Decode'、'Encode'、'GetSilentModeManager'、'Set'、'err.Error'、'errors.Is'、'errors.New'、'json.NewDecoder'、'json.NewEncoder'、'm.InvalidateShortLinkCache'、'r.Context'、'repo.GetAllUserShortCodes'、'repo.UpdateUserCustomShortCode'、'strings.TrimSpace'、'w.Header'、'writeError' |

## `internal/handler/v2ray_parser.go`

依赖：`encoding/base64`、`strings`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 9–24 | function | `base64DecodeV2ray` | 执行与 'base64 decode v2ray' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'base64.StdEncoding.DecodeString'、'len'、'string'、'strings.Repeat'、'strings.ReplaceAll'、'strings.TrimSpace' |

## `internal/handler/yaml_sync.go`

依赖：`encoding/json`、`fmt`、`miaomiaowu/internal/logger`、`os`、`path/filepath`、`miaomiaowu/internal/util`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 16–35 | function | `proxyKeysChanged` | 执行与 'proxy keys changed' 对应的业务或基础设施操作。 | 分支 4；循环 2；返回 4；goroutine 0；调用 'len'、'make' |
| 38–61 | function | `updateProxyNodeFields` | 更新与 'update proxy node fields' 对应的业务或基础设施操作。 | 分支 3；循环 2；返回 1；goroutine 0；调用 'len'、'make'、'updateValueNode' |
| 64–70 | function | `reorderProxyNodeFieldsInPlace` | 执行与 'reorder proxy node fields in place' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'util.ReorderProxyNode' |
| 73–146 | function | `updateValueNode` | 更新与 'update value node' 对应的业务或基础设施操作。 | 分支 12；循环 1；返回 1；goroutine 0；调用 'append'、'encodeValue'、'fmt.Sprintf'、'node.SetString'、'updateProxyNodeFields' |
| 149–243 | function | `encodeValue` | 执行与 'encode value' 对应的业务或基础设施操作。 | 分支 7；循环 2；返回 2；goroutine 0；调用 'append'、'encodeValue'、'float64'、'fmt.Sprintf'、'int64'、'node.SetString' |
| 246–262 | function | `convertNilToEmptyString` | 转换与 'convert nil to empty string' 对应的业务或基础设施操作。 | 分支 5；循环 2；返回 0；goroutine 0；调用 'convertNilToEmptyString' |
| 265–280 | function | `MarshalYAMLWithQuotedEmptyStrings` | 执行与 'marshal yaml with quoted empty strings' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'convertNilToEmptyString'、'encodeValue'、'yaml.Marshal' |
| 283–326 | function | `fixShortIdStyleInNode` | 执行与 'fix short id style in node' 对应的业务或基础设施操作。 | 分支 8；循环 3；返回 1；goroutine 0；调用 'fixShortIdStyleInNode'、'len' |
| 329–607 | function | `syncNodeToYAMLFiles` | 同步与 'sync node to yaml files' 对应的业务或基础设施操作。 | 分支 44；循环 14；返回 4；goroutine 0；调用 'append'、'containsNodeName'、'convertNilToEmptyString'、'entry.IsDir'、'entry.Name'、'filepath.Ext'、'filepath.Join'、'fmt.Errorf'、'json.Unmarshal'、'len'、'make'、'os.ReadDir'、'os.ReadFile'、'proxyKeysChanged'、'replaceNodeNameInRule'、'yaml.Unmarshal' |
| 610–807 | function | `batchSyncNodesToYAMLFiles` | 执行与 'batch sync nodes to yaml files' 对应的业务或基础设施操作。 | 分支 31；循环 11；返回 4；goroutine 0；调用 'append'、'convertNilToEmptyString'、'entry.IsDir'、'entry.Name'、'filepath.Ext'、'filepath.Join'、'fmt.Errorf'、'json.Unmarshal'、'len'、'make'、'os.ReadDir'、'os.ReadFile'、'proxyKeysChanged'、'updateProxyNodeFields'、'util.ReorderProxyFieldsToNode'、'yaml.Unmarshal' |
| 810–840 | function | `updateProxyGroupsNode` | 更新与 'update proxy groups node' 对应的业务或基础设施操作。 | 分支 6；循环 3；返回 1；goroutine 0；调用 'len' |
| 843–855 | function | `updateRulesNode` | 更新与 'update rules node' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 1；goroutine 0；调用 'containsNodeName'、'replaceNodeNameInRule' |
| 858–866 | function | `containsNodeName` | 执行与 'contains node name' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'len'、'splitRule' |
| 869–883 | function | `replaceNodeNameInRule` | 执行与 'replace node name in rule' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 2；goroutine 0；调用 'len'、'splitRule' |
| 886–917 | function | `splitRule` | 执行与 'split rule' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 1；goroutine 0；调用 'append'、'string' |
| 920–996 | function | `reorderTopLevelFields` | 执行与 'reorder top level fields' 对应的业务或基础设施操作。 | 分支 5；循环 4；返回 1；goroutine 0；调用 'append'、'len'、'make' |
| 999–1226 | function | `deleteNodeFromYAMLFilesWithLog` | 删除与 'delete node from yaml files with log' 对应的业务或基础设施操作。 | 分支 36；循环 10；返回 3；goroutine 0；调用 'append'、'containsNodeName'、'entry.IsDir'、'entry.Name'、'filepath.Ext'、'filepath.Join'、'fixShortIdStyleInNode'、'fmt.Errorf'、'len'、'make'、'os.ReadDir'、'os.ReadFile'、'removeNodeFromProxyGroupsNode'、'removeNodeFromRulesNode'、'reorderTopLevelFields'、'yaml.Unmarshal' |
| 1229–1232 | function | `deleteNodeFromYAMLFiles` | 删除与 'delete node from yaml files' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'deleteNodeFromYAMLFilesWithLog' |
| 1235–1267 | function | `removeNodeFromProxyGroupsNode` | 移除与 'remove node from proxy groups node' 对应的业务或基础设施操作。 | 分支 6；循环 3；返回 1；goroutine 0；调用 'append'、'len'、'make' |
| 1270–1287 | function | `removeNodeFromRulesNode` | 移除与 'remove node from rules node' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 1；goroutine 0；调用 'append'、'containsNodeName'、'len'、'make' |

## `internal/handler/yaml_sync_manager.go`

依赖：`miaomiaowu/internal/logger`、`sync`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 9–12 | type | `YAMLSyncManager` | 定义 'YAMLSyncManager' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 15–19 | function | `NewYAMLSyncManager` | 创建并初始化与 'new yaml sync manager' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 22–38 | function | `(*YAMLSyncManager).SyncNode` | *YAMLSyncManager 的方法，同步与 'sync node' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'logger.Info'、'm.mu.Lock'、'm.mu.Unlock'、'syncNodeToYAMLFiles' |
| 41–59 | function | `(*YAMLSyncManager).DeleteNode` | *YAMLSyncManager 的方法，删除与 'delete node' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 2；goroutine 0；调用 'deleteNodeFromYAMLFilesWithLog'、'len'、'logger.Info'、'm.mu.Lock'、'm.mu.Unlock' |
| 62–104 | function | `(*YAMLSyncManager).BatchDeleteNodes` | *YAMLSyncManager 的方法，执行与 'batch delete nodes' 对应的业务或基础设施操作。 | 分支 4；循环 3；返回 2；goroutine 0；调用 'deleteNodeFromYAMLFilesWithLog'、'len'、'logger.Info'、'm.mu.Lock'、'm.mu.Unlock'、'make' |
| 107–111 | type | `NodeUpdate` | 定义 'NodeUpdate' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 115–133 | function | `(*YAMLSyncManager).BatchSyncNodes` | *YAMLSyncManager 的方法，执行与 'batch sync nodes' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'batchSyncNodesToYAMLFiles'、'len'、'logger.Info'、'm.mu.Lock'、'm.mu.Unlock' |

## `internal/handler/yaml_utils.go`

依赖：`bytes`、`fmt`、`regexp`、`strings`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 13–27 | function | `MarshalYAMLWithIndent` | 执行与 'marshal yaml with indent' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'buf.Bytes'、'encoder.Close'、'encoder.Encode'、'encoder.SetIndent'、'sanitizeExplicitStringTags'、'yaml.NewEncoder' |
| 30–41 | function | `MarshalWithIndent` | 执行与 'marshal with indent' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'buf.Bytes'、'encoder.Close'、'encoder.Encode'、'encoder.SetIndent'、'yaml.NewEncoder' |
| 47–108 | function | `RemoveUnicodeEscapeQuotes` | 移除与 'remove unicode escape quotes' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'convertUnicodeEscapes'、'fmt.Sprintf'、'len'、'nameserverPolicyRe.ReplaceAllStringFunc'、'numericQuotesRe.ReplaceAllString'、'quotedUnicodeRe.ReplaceAllStringFunc'、'regexp.MustCompile'、'strings.Join'、'strings.Replace'、'strings.Trim' |
| 51–54 | closure | `RemoveUnicodeEscapeQuotes.closure#1` | 供 RemoveUnicodeEscapeQuotes 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 60–84 | closure | `RemoveUnicodeEscapeQuotes.closure#2` | 供 RemoveUnicodeEscapeQuotes 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'convertUnicodeEscapes'、'len'、'strings.Trim' |
| 111–122 | function | `convertUnicodeEscapes` | 转换与 'convert unicode escapes' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'escapeRe.ReplaceAllStringFunc'、'fmt.Sscanf'、'regexp.MustCompile'、'rune'、'string'、'strings.HasPrefix' |
| 113–121 | closure | `convertUnicodeEscapes.closure#1` | 供 convertUnicodeEscapes 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'fmt.Sscanf'、'rune'、'string'、'strings.HasPrefix' |
| 125–153 | function | `yamlNodeToMap` | 执行与 'yaml node to map' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 5；goroutine 0；调用 'fmt.Errorf'、'len'、'make'、'yamlNodeToMap'、'yamlNodeToValue' |
| 156–193 | function | `yamlNodeToValue` | 执行与 'yaml node to value' 对应的业务或基础设施操作。 | 分支 5；循环 1；返回 9；goroutine 0；调用 'append'、'looksLikeNumericStringWithLeadingZero'、'node.Decode'、'yamlNodeToMap'、'yamlNodeToValue' |
| 196–210 | function | `looksLikeNumericStringWithLeadingZero` | 执行与 'looks like numeric string with leading zero' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 4；goroutine 0；调用 'len' |
| 220–234 | function | `sanitizeExplicitStringTags` | 执行与 'sanitize explicit string tags' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 1；goroutine 0；调用 'isExplicitStringTag'、'sanitizeExplicitStringTags' |
| 237–239 | function | `isExplicitStringTag` | 判断与 'is explicit string tag' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |

