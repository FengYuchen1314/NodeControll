# Go 包 `logger`

结构化日志、日志文件轮转和历史日志清理。

## `internal/logger/logger.go`

依赖：`fmt`、`io`、`log/slog`、`os`、`strings`、`sync`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 13–17 | type | `Logger` | 定义 'Logger' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 20–20 | var | `defaultLogger` | 保存 'defaultLogger' 的包级共享状态、配置或预计算值。 |  |
| 21–21 | var | `once` | 保存 'once' 的包级共享状态、配置或预计算值。 |  |
| 25–33 | function | `Init` | 执行与 'init' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'newTextHandler'、'once.Do'、'slog.New' |
| 26–31 | closure | `Init.closure#1` | 供 Init 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'newTextHandler'、'slog.New' |
| 36–41 | function | `GetLogger` | 查询或读取与 'get logger' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'Init' |
| 44–72 | function | `newTextHandler` | 创建并初始化与 'new text handler' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'a.Value.Any'、'a.Value.Kind'、'a.Value.Time'、'slog.NewTextHandler'、'slog.String'、't.Format' |
| 47–70 | closure | `newTextHandler.closure#1` | 供 newTextHandler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 3；循环 0；返回 3；goroutine 0；调用 'a.Value.Any'、'a.Value.Kind'、'a.Value.Time'、'slog.String'、't.Format' |
| 75–100 | function | `(*Logger).EnableDebugLog` | *Logger 的方法，执行与 'enable debug log' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'fmt.Errorf'、'io.MultiWriter'、'l.Info'、'l.debugFile.Close'、'l.mu.Lock'、'l.mu.Unlock'、'newTextHandler'、'os.Create'、'slog.New' |
| 103–123 | function | `(*Logger).DisableDebugLog` | *Logger 的方法，执行与 'disable debug log' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'l.Info'、'l.debugFile.Close'、'l.debugFile.Name'、'l.mu.Lock'、'l.mu.Unlock'、'newTextHandler'、'slog.New' |
| 126–130 | function | `(*Logger).IsDebugEnabled` | *Logger 的方法，判断与 'is debug enabled' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'l.mu.RLock'、'l.mu.RUnlock' |
| 133–140 | function | `(*Logger).GetDebugFilePath` | *Logger 的方法，查询或读取与 'get debug file path' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'l.debugFile.Name'、'l.mu.RLock'、'l.mu.RUnlock' |
| 143–164 | function | `sanitizeArgs` | 执行与 'sanitize args' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 2；goroutine 0；调用 'copy'、'len'、'make'、'strings.Contains'、'strings.ToLower' |
| 167–169 | function | `Info` | 执行与 'info' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'GetLogger'、'Info'、'sanitizeArgs' |
| 171–173 | function | `Warn` | 执行与 'warn' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'GetLogger'、'Warn'、'sanitizeArgs' |
| 175–177 | function | `Error` | 执行与 'error' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'Error'、'GetLogger'、'sanitizeArgs' |
| 179–181 | function | `Debug` | 执行与 'debug' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'Debug'、'GetLogger'、'sanitizeArgs' |
| 184–186 | function | `EnableDebug` | 执行与 'enable debug' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'EnableDebugLog'、'GetLogger' |
| 189–191 | function | `DisableDebug` | 执行与 'disable debug' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'DisableDebugLog'、'GetLogger' |
| 194–196 | function | `IsDebugEnabled` | 判断与 'is debug enabled' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'GetLogger'、'IsDebugEnabled' |

## `internal/logger/manager.go`

依赖：`fmt`、`os`、`path/filepath`、`strings`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–16 | type | `LogManager` | 定义 'LogManager' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 19–25 | function | `NewLogManager` | 创建并初始化与 'new log manager' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 28–40 | function | `(*LogManager).CreateLogFile` | *LogManager 的方法，创建与 'create log file' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'Format'、'filepath.Join'、'fmt.Errorf'、'fmt.Sprintf'、'os.MkdirAll'、'time.Now' |
| 43–86 | function | `(*LogManager).CleanupOldLogs` | *LogManager 的方法，清理与 'cleanup old logs' 对应的业务或基础设施操作。 | 分支 8；循环 1；返回 3；goroutine 0；调用 'AddDate'、'Before'、'Debug'、'Hours'、'entry.Info'、'entry.IsDir'、'entry.Name'、'filepath.Join'、'fmt.Errorf'、'info.ModTime'、'int'、'os.IsNotExist'、'os.ReadDir'、'os.Remove'、'strings.HasPrefix'、'time.Now' |
| 89–95 | function | `(*LogManager).GetLogFileSize` | *LogManager 的方法，查询或读取与 'get log file size' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'info.Size'、'os.Stat' |
| 98–113 | function | `(*LogManager).CheckRotation` | *LogManager 的方法，检查与 'check rotation' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'm.CreateLogFile'、'm.GetLogFileSize' |
| 116–122 | function | `(*LogManager).DeleteLogFile` | *LogManager 的方法，删除与 'delete log file' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'filepath.Join'、'fmt.Errorf'、'os.Remove' |
| 125–154 | function | `(*LogManager).ListLogFiles` | *LogManager 的方法，列举与 'list log files' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 3；goroutine 0；调用 'append'、'entry.Info'、'entry.IsDir'、'entry.Name'、'filepath.Join'、'fmt.Errorf'、'info.ModTime'、'info.Size'、'os.IsNotExist'、'os.ReadDir'、'strings.HasPrefix' |
| 157–162 | type | `LogFileInfo` | 定义 'LogFileInfo' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 165–176 | function | `(LogFileInfo).FormatSize` | LogFileInfo 的方法，执行与 'format size' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0；调用 'float64'、'fmt.Sprintf'、'int64' |
| 179–181 | function | `(LogFileInfo).Age` | LogFileInfo 的方法，执行与 'age' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'time.Since' |

