# Go 包 `web`

嵌入式前端静态资源和 SPA fallback HTTP Handler。

## `internal/web/handler.go`

依赖：`bytes`、`embed`、`io/fs`、`net/http`、`path`、`strings`、`sync`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 15–15 | var | `embeddedFiles` | 保存 'embeddedFiles' 的包级共享状态、配置或预计算值。 |  |
| 18–18 | var | `initOnce` | 保存 'initOnce' 的包级共享状态、配置或预计算值。 |  |
| 19–19 | var | `staticFS` | 保存 'staticFS' 的包级共享状态、配置或预计算值。 |  |
| 20–20 | var | `staticFiles` | 保存 'staticFiles' 的包级共享状态、配置或预计算值。 |  |
| 21–21 | var | `indexBytes` | 保存 'indexBytes' 的包级共享状态、配置或预计算值。 |  |
| 22–22 | var | `indexMod` | 保存 'indexMod' 的包级共享状态、配置或预计算值。 |  |
| 25–44 | function | `initialize` | 执行与 'initialize' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 0；goroutine 0；调用 'fs.ReadFile'、'fs.Stat'、'fs.Sub'、'http.FS'、'http.FileServer'、'info.ModTime'、'panic'、'time.Now' |
| 47–79 | function | `Handler` | 处理与 'handler' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'fileExists'、'http.HandlerFunc'、'http.NotFound'、'initOnce.Do'、'path.Clean'、'serveIndex'、'staticFiles.ServeHTTP'、'strings.HasPrefix'、'strings.TrimPrefix' |
| 50–78 | closure | `Handler.closure#1` | 供 Handler 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 5；循环 0；返回 4；goroutine 0；调用 'fileExists'、'http.NotFound'、'path.Clean'、'serveIndex'、'staticFiles.ServeHTTP'、'strings.HasPrefix'、'strings.TrimPrefix' |
| 81–92 | function | `serveIndex` | 提供 HTTP 服务与 'serve index' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'Set'、'bytes.NewReader'、'http.ServeContent'、'initOnce.Do'、'w.Header'、'w.WriteHeader' |
| 94–102 | function | `fileExists` | 执行与 'file exists' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'fs.Stat'、'info.IsDir'、'initOnce.Do' |

