# Go 包 `scriptengine`

基于 goja 的 JavaScript 覆写脚本执行沙箱与对象转换。

## `internal/scriptengine/engine.go`

依赖：`context`、`encoding/json`、`fmt`、`strings`、`time`、`miaomiaowu/internal/logger`、`github.com/MMWOrg/mmwX-plugins/proxyparser/substore`、`github.com/dop251/goja`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 16–16 | const | `defaultTimeout` | 定义 'defaultTimeout' 的不可变协议值、默认值或枚举成员。 |  |
| 18–70 | function | `setupVM` | 设置与 'setup vm' 对应的业务或基础设施操作。 | 分支 5；循环 2；返回 3；goroutine 0；调用 'arg.Export'、'call.Arguments.Export'、'console.Set'、'fmt.Sprintf'、'goja.Undefined'、'len'、'logger.Error'、'logger.Info'、'logger.Warn'、'make'、'makeLogFn'、'panic'、'strings.Join'、'vm.NewObject'、'vm.Set'、'vm.ToValue' |
| 20–37 | closure | `setupVM.closure#1` | 供 setupVM 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 1；返回 2；goroutine 0；调用 'arg.Export'、'fmt.Sprintf'、'goja.Undefined'、'len'、'logger.Error'、'logger.Info'、'logger.Warn'、'make'、'strings.Join' |
| 21–36 | closure | `setupVM.closure#2` | 供 setupVM 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 1；返回 1；goroutine 0；调用 'arg.Export'、'fmt.Sprintf'、'goja.Undefined'、'len'、'logger.Error'、'logger.Info'、'logger.Warn'、'make'、'strings.Join' |
| 43–69 | closure | `setupVM.closure#3` | 供 setupVM 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 4；循环 1；返回 1；goroutine 0；调用 'ConvertProxies'、'append'、'call.Arguments.Export'、'call.Arguments.String'、'err.Error'、'len'、'make'、'panic'、'substore.GetDefaultFactory'、'substore.Proxy'、'vm.ToValue' |
| 74–105 | function | `RunPostFetch` | 运行与 'run post fetch' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'fmt.Errorf'、'goja.New'、'json.Marshal'、'result.Export'、'runWithTimeout'、'setupVM'、'string'、'vm.Set' |
| 109–145 | function | `RunPreSaveNodes` | 运行与 'run pre save nodes' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 6；goroutine 0；调用 'append'、'fmt.Errorf'、'goja.New'、'json.Marshal'、'len'、'make'、'result.Export'、'runWithTimeout'、'setupVM'、'string'、'vm.Set' |
| 147–169 | function | `runWithTimeout` | 运行与 'run with timeout' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 3；goroutine 1；调用 'cancel'、'context.WithTimeout'、'fmt.Errorf'、'interrupted.Value'、'timeoutCtx.Done'、'timeoutCtx.Err'、'vm.ClearInterrupt'、'vm.Interrupt'、'vm.RunString' |
| 153–158 | closure | `runWithTimeout.closure#1` | 供 runWithTimeout 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'timeoutCtx.Done'、'timeoutCtx.Err'、'vm.Interrupt' |

## `internal/scriptengine/engine_test.go`

依赖：`context`、`strings`、`testing`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 10–44 | function | `TestRunPostFetch` | 执行与 'test run post fetch' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、'len'、't.Errorf'、't.Fatal'、't.Fatalf' |
| 46–57 | function | `TestRunPostFetch_ReturnNil` | 执行与 'test run post fetch_ return nil' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、't.Errorf'、't.Fatalf' |
| 59–70 | function | `TestRunPostFetch_ReturnNonObject` | 执行与 'test run post fetch_ return non object' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、'err.Error'、'strings.Contains'、't.Errorf'、't.Fatal' |
| 72–103 | function | `TestRunPreSaveNodes` | 执行与 'test run pre save nodes' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 0；goroutine 0；调用 'RunPreSaveNodes'、'context.Background'、'len'、't.Errorf'、't.Fatalf' |
| 105–129 | function | `TestRunPreSaveNodes_Filter` | 执行与 'test run pre save nodes_ filter' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 0；goroutine 0；调用 'RunPreSaveNodes'、'context.Background'、'len'、't.Errorf'、't.Fatalf' |
| 131–144 | function | `TestRunPreSaveNodes_ReturnNil` | 执行与 'test run pre save nodes_ return nil' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPreSaveNodes'、'context.Background'、'len'、't.Errorf'、't.Fatalf' |
| 146–159 | function | `TestRunPreSaveNodes_ReturnNonArray` | 执行与 'test run pre save nodes_ return non array' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPreSaveNodes'、'context.Background'、'err.Error'、'strings.Contains'、't.Errorf'、't.Fatal' |
| 161–175 | function | `TestRunPostFetch_Timeout` | 执行与 'test run post fetch_ timeout' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、'err.Error'、'strings.Contains'、't.Errorf'、't.Fatal' |
| 177–191 | function | `TestRunPostFetch_ContextCancel` | 执行与 'test run post fetch_ context cancel' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'cancel'、'context.Background'、'context.WithTimeout'、't.Fatal' |
| 193–204 | function | `TestRunPostFetch_SyntaxError` | 执行与 'test run post fetch_ syntax error' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、'err.Error'、'strings.Contains'、't.Errorf'、't.Fatal' |
| 206–217 | function | `TestRunPostFetch_RuntimeError` | 执行与 'test run post fetch_ runtime error' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、't.Fatal' |
| 219–229 | function | `TestRunPreSaveNodes_EmptyInput` | 执行与 'test run pre save nodes_ empty input' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPreSaveNodes'、'context.Background'、'len'、't.Errorf'、't.Fatalf' |
| 231–247 | function | `TestConsoleLog` | 执行与 'test console log' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、't.Errorf'、't.Fatalf' |
| 249–270 | function | `TestConsoleLog_DifferentTypes` | 执行与 'test console log_ different types' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPreSaveNodes'、'context.Background'、'len'、't.Errorf'、't.Fatalf' |
| 272–303 | function | `TestProduce_URI` | 执行与 'test produce_uri' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、'int64'、't.Error'、't.Fatalf' |
| 305–329 | function | `TestProduce_InvalidFormat` | 执行与 'test produce_ invalid format' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、'err.Error'、'int64'、'strings.Contains'、't.Errorf'、't.Fatal' |
| 331–348 | function | `TestProduce_TooFewArguments` | 执行与 'test produce_ too few arguments' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、'err.Error'、'strings.Contains'、't.Errorf'、't.Fatal' |
| 350–365 | function | `TestProduce_InvalidFirstArgument` | 执行与 'test produce_ invalid first argument' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、'err.Error'、'strings.Contains'、't.Errorf'、't.Fatal' |
| 367–383 | function | `TestProduce_EmptyProxies` | 执行与 'test produce_ empty proxies' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、't.Error'、't.Fatalf' |
| 385–411 | function | `TestProduce_InPreSaveNodes` | 执行与 'test produce_ in pre save nodes' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPreSaveNodes'、'context.Background'、'int64'、'len'、't.Fatalf' |
| 413–448 | function | `TestProduce_MultipleFormats` | 执行与 'test produce_ multiple formats' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、'int64'、't.Error'、't.Fatalf' |
| 450–481 | function | `TestScriptModifiesAndProduces` | 执行与 'test script modifies and produces' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 0；goroutine 0；调用 'RunPreSaveNodes'、'context.Background'、'int64'、't.Errorf'、't.Fatalf' |
| 483–514 | function | `TestRunPostFetch_AddRules` | 执行与 'test run post fetch_ add rules' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、'len'、't.Errorf'、't.Fatalf' |
| 516–554 | function | `TestRunPostFetch_ModifyProxyGroups` | 执行与 'test run post fetch_ modify proxy groups' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 0；goroutine 0；调用 'RunPostFetch'、'context.Background'、'int64'、't.Errorf'、't.Fatalf' |

