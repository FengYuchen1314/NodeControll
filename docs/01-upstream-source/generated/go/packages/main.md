# Go 包 `main`

组合全部基础设施与 HTTP 端点，启动和优雅停止单体服务。

## `cmd/server/cors.go`

依赖：`net/http`、`os`、`strings`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 9–15 | function | `getAllowedOrigins` | 查询或读取与 'get allowed origins' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'os.Getenv'、'parseAllowedOrigins' |
| 17–32 | function | `parseAllowedOrigins` | 解析与 'parse allowed origins' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 3；goroutine 0；调用 'append'、'len'、'strings.Split'、'strings.TrimSpace' |
| 34–55 | function | `withCORS` | 执行与 'with cors' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 3；goroutine 0；调用 'http.HandlerFunc'、'len'、'next.ServeHTTP'、'originAllowed'、'r.Header.Get'、'setCORSHeaders'、'w.WriteHeader' |
| 36–36 | closure | `withCORS.closure#1` | 供 withCORS 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0 |
| 40–54 | closure | `withCORS.closure#2` | 供 withCORS 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 3；循环 0；返回 1；goroutine 0；调用 'next.ServeHTTP'、'originAllowed'、'r.Header.Get'、'setCORSHeaders'、'w.WriteHeader' |
| 57–64 | function | `originAllowed` | 执行与 'origin allowed' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0 |
| 66–80 | function | `setCORSHeaders` | 设置与 'set cors headers' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 0；goroutine 0；调用 'Get'、'Set'、'strings.Contains'、'w.Header' |

## `cmd/server/main.go`

依赖：`context`、`encoding/json`、`errors`、`fmt`、`net/http`、`os`、`os/signal`、`path/filepath`、`strings`、`syscall`、`time`、`miaomiaowu/internal/auth`、`miaomiaowu/internal/captcha`、`miaomiaowu/internal/handler`、`miaomiaowu/internal/logger`、`miaomiaowu/internal/notify`、`miaomiaowu/internal/patches`、`miaomiaowu/internal/proxygroups`、`miaomiaowu/internal/storage`、`miaomiaowu/internal/taskrun`、`miaomiaowu/internal/version`、`miaomiaowu/internal/web`、`miaomiaowu/rule_templates`、`miaomiaowu/subscribes`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 31–366 | function | `main` | 程序入口：组装依赖、注册路由、启动后台任务和 HTTP 服务，并处理优雅退出。 | 分支 20；循环 1；返回 5；goroutine 11；调用 'auth.NewManager'、'auth.NewTokenStore'、'auth.NewTwoFactorPendingStore'、'context.Background'、'filepath.Join'、'getAddr'、'logger.Error'、'logger.Info'、'logger.Init'、'logger.Warn'、'os.Exit'、'repo.Close'、'repo.LoadSessions'、'startLogCleanup'、'storage.NewTrafficRepository'、'tokenStore.LoadSession' |
| 160–167 | closure | `main.closure#1` | 供 main 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'Encode'、'Set'、'http.Error'、'json.NewEncoder'、'r.Context'、'turnstileVerifier.Enabled'、'turnstileVerifier.SiteKey'、'w.Header' |
| 286–328 | closure | `main.closure#2` | 供 main 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 6；循环 0；返回 4；goroutine 0；调用 'ServeHTTP'、'bruteForceProtector.IsBlocked'、'bruteForceProtector.RecordFailure'、'handler.GetClientIP'、'http.Error'、'http.NotFound'、'isAlphanumeric'、'len'、'shortLinkHandler.TryServe'、'strings.HasPrefix'、'strings.Trim'、'subRateLimiter.Allow'、'tempSubAccessHandler.ServeHTTP'、'web.Handler' |
| 357–363 | closure | `main.closure#3` | 供 main 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'logger.Error'、'logger.Info'、'os.Exit'、'srv.ListenAndServe' |
| 368–390 | function | `startWALCheckpointTask` | 启动与 'start wal checkpoint task' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 4；goroutine 0；调用 'ctx.Done'、'fmt.Sprintf'、'logger.Warn'、'repo.CheckpointBestEffort'、'taskrun.Record'、'ticker.Stop'、'time.NewTicker' |
| 376–387 | closure | `startWALCheckpointTask.closure#1` | 供 startWALCheckpointTask 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'fmt.Sprintf'、'logger.Warn'、'repo.CheckpointBestEffort' |
| 392–417 | function | `startDatabaseLogCleanup` | 启动与 'start database log cleanup' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 2；goroutine 0；调用 'cleanup'、'ctx.Done'、'logger.Info'、'logger.Warn'、'now.AddDate'、'repo.DeleteOldOperationLogs'、'repo.DeleteOldSecurityEvents'、'repo.DeleteOldTaskRuns'、'ticker.Stop'、'time.NewTicker'、'time.Now' |
| 393–405 | closure | `startDatabaseLogCleanup.closure#1` | 供 startDatabaseLogCleanup 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 1；goroutine 0；调用 'logger.Info'、'logger.Warn'、'now.AddDate'、'repo.DeleteOldOperationLogs'、'repo.DeleteOldSecurityEvents'、'repo.DeleteOldTaskRuns'、'time.Now' |
| 419–425 | function | `getAddr` | 查询或读取与 'get addr' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'os.Getenv' |
| 432–443 | var | `reservedFrontendRoutes` | 保存 'reservedFrontendRoutes' 的包级共享状态、配置或预计算值。 |  |
| 446–453 | function | `isAlphanumeric` | 判断与 'is alphanumeric' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0 |
| 455–477 | function | `waitForShutdown` | 执行与 'wait for shutdown' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 0；goroutine 0；调用 'cancel'、'cancelFunc'、'context.Background'、'context.WithTimeout'、'logger.Error'、'logger.Info'、'make'、'signal.Notify'、'srv.Shutdown' |
| 479–540 | function | `startTrafficCollector` | 启动与 'start traffic collector' 对应的业务或基础设施操作。 | 分支 6；循环 2；返回 5；goroutine 0；调用 'Format'、'cancel'、'context.WithTimeout'、'ctx.Done'、'errors.Is'、'logger.Error'、'logger.Info'、'logger.Warn'、'runWithRetry'、'ticker.Stop'、'time.After'、'time.NewTicker'、'time.Now'、'trafficHandler.RecordDailyUsage' |
| 485–522 | closure | `startTrafficCollector.closure#1` | 供 startTrafficCollector 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 4；循环 1；返回 3；goroutine 0；调用 'Format'、'cancel'、'context.WithTimeout'、'ctx.Done'、'errors.Is'、'logger.Error'、'logger.Info'、'logger.Warn'、'time.After'、'time.Now'、'trafficHandler.RecordDailyUsage' |
| 545–608 | function | `syncSubscribeFilesToDatabase` | 同步与 'sync subscribe files to database' 对应的业务或基础设施操作。 | 分支 9；循环 1；返回 2；goroutine 0；调用 'cancel'、'context.Background'、'context.WithTimeout'、'entry.IsDir'、'entry.Name'、'errors.Is'、'filepath.Ext'、'len'、'logger.Info'、'logger.Warn'、'os.ReadDir'、'repo.CreateSubscribeFile'、'repo.GetSubscribeFileByFilename' |
| 611–630 | function | `startLogCleanup` | 启动与 'start log cleanup' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 0；goroutine 0；调用 'logManager.CleanupOldLogs'、'logger.Error'、'logger.Info'、'logger.NewLogManager'、'ticker.Stop'、'time.NewTicker' |

