# Go 包 `taskrun`

后台任务运行记录、状态和可观测性封装。

## `internal/taskrun/recorder.go`

依赖：`context`、`sync`、`time`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 21–26 | type | `Recorder` | 定义 'Recorder' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 30–39 | function | `New` | 创建并初始化与 'new' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'make' |
| 46–63 | function | `(*Recorder).Wrap` | *Recorder 的方法，执行与 'wrap' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 1；goroutine 0；调用 'dur.Milliseconds'、'err.Error'、'fn'、'r.repo.InsertTaskRun'、'r.throttled'、'time.Now'、'time.Since' |
| 66–79 | function | `(*Recorder).throttled` | *Recorder 的方法，执行与 'throttled' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'now.Sub'、'r.mu.Lock'、'r.mu.Unlock' |
| 83–83 | var | `defaultRecorder` | 保存 'defaultRecorder' 的包级共享状态、配置或预计算值。 |  |
| 86–86 | function | `Init` | 执行与 'init' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0 |
| 89–95 | function | `Record` | 执行与 'record' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 1；goroutine 0；调用 'defaultRecorder.Wrap'、'fn' |

