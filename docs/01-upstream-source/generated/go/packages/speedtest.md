# Go 包 `speedtest`

Mihomo/远程测试器驱动的节点测速模型和执行能力。

## `internal/speedtest/mihomo.go`

依赖：`archive/zip`、`bytes`、`compress/gzip`、`context`、`encoding/json`、`fmt`、`io`、`net/http`、`os`、`os/exec`、`path/filepath`、`regexp`、`runtime`、`strconv`、`strings`、`sync`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 23–23 | const | `mihomoCacheDir` | 定义 'mihomoCacheDir' 的不可变协议值、默认值或枚举成员。 |  |
| 27–27 | const | `minMihomoVersion` | 定义 'minMihomoVersion' 的不可变协议值、默认值或枚举成员。 |  |
| 29–29 | var | `mihomoVerRe` | 保存 'mihomoVerRe' 的包级共享状态、配置或预计算值。 |  |
| 32–41 | function | `mihomoVersion` | 执行与 'mihomo version' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'CombinedOutput'、'cancel'、'context.Background'、'context.WithTimeout'、'exec.CommandContext'、'mihomoVerRe.FindStringSubmatch'、'string' |
| 44–59 | function | `versionGTE` | 执行与 'version gte' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 2；goroutine 0；调用 'len'、'strconv.Atoi'、'strings.Split' |
| 63–69 | function | `mihomoSupportsSnell` | 执行与 'mihomo supports snell' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'mihomoVersion'、'versionGTE' |
| 71–76 | function | `mihomoBinName` | 执行与 'mihomo bin name' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0 |
| 79–79 | var | `mihomoMu` | 保存 'mihomoMu' 的包级共享状态、配置或预计算值。 |  |
| 80–80 | var | `cachedPath` | 保存 'cachedPath' 的包级共享状态、配置或预计算值。 |  |
| 84–110 | function | `EnsureMihomo` | 执行与 'ensure mihomo' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'downloadMihomo'、'exec.LookPath'、'fileExists'、'filepath.Join'、'fmt.Errorf'、'mihomoBinName'、'mihomoMu.Lock'、'mihomoMu.Unlock'、'mihomoSupportsSnell'、'os.Getenv' |
| 113–129 | function | `MihomoStatus` | 执行与 'mihomo status' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'exec.LookPath'、'fileExists'、'filepath.Join'、'mihomoBinName'、'mihomoSupportsSnell'、'os.Getenv' |
| 131–134 | function | `fileExists` | 执行与 'file exists' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'os.Stat'、'st.IsDir' |
| 136–243 | function | `downloadMihomo` | 执行与 'download mihomo' 对应的业务或基础设施操作。 | 分支 20；循环 2；返回 16；goroutine 0；调用 'Do'、'f.Close'、'fetchLatestRelease'、'filepath.Dir'、'fmt.Errorf'、'fmt.Sprintf'、'http.NewRequestWithContext'、'io.ReadAll'、'os.MkdirAll'、'os.OpenFile'、'os.Remove'、'pick'、'resp.Body.Close'、'strings.HasPrefix'、'strings.HasSuffix'、'zip.NewReader' |
| 151–159 | closure | `downloadMihomo.closure#1` | 供 downloadMihomo 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 1；循环 1；返回 2；goroutine 0；调用 'fmt.Sprintf'、'strings.HasPrefix'、'strings.HasSuffix' |
| 245–251 | type | `ghRelease` | 定义 'ghRelease' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 253–270 | function | `fetchLatestRelease` | 从外部获取与 'fetch latest release' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'Decode'、'Do'、'fmt.Errorf'、'http.NewRequestWithContext'、'json.NewDecoder'、'req.Header.Set'、'resp.Body.Close' |

## `internal/speedtest/runner.go`

依赖：`context`、`crypto/tls`、`encoding/json`、`fmt`、`io`、`net`、`net/http`、`net/url`、`os`、`os/exec`、`path/filepath`、`runtime`、`strings`、`sync`、`syscall`、`time`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 25–25 | const | `defaultTestURL` | 定义 'defaultTestURL' 的不可变协议值、默认值或枚举成员。 |  |
| 26–26 | const | `defaultTestDuration` | 定义 'defaultTestDuration' 的不可变协议值、默认值或枚举成员。 |  |
| 27–27 | const | `latencyProbeURL` | 定义 'latencyProbeURL' 的不可变协议值、默认值或枚举成员。 |  |
| 28–28 | const | `cfLatencyProbeURL` | 定义 'cfLatencyProbeURL' 的不可变协议值、默认值或枚举成员。 |  |
| 29–29 | const | `egressIPProbeURL` | 定义 'egressIPProbeURL' 的不可变协议值、默认值或枚举成员。 |  |
| 30–30 | const | `mixedPort` | 定义 'mixedPort' 的不可变协议值、默认值或枚举成员。 |  |
| 31–31 | const | `cfLatencySamples` | 定义 'cfLatencySamples' 的不可变协议值、默认值或枚举成员。 |  |
| 34–34 | var | `runMu` | 保存 'runMu' 的包级共享状态、配置或预计算值。 |  |
| 36–42 | type | `Result` | 定义 'Result' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 44–51 | type | `Options` | 定义 'Options' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 54–128 | function | `RunNodeTest` | 运行与 'run node test' 对应的业务或基础设施操作。 | 分支 11；循环 0；返回 6；goroutine 0；调用 'UnixNano'、'cancel'、'context.WithTimeout'、'filepath.Join'、'fmt.Errorf'、'fmt.Sprintf'、'json.Unmarshal'、'measureEgressIP'、'measureLatencyCloudflare'、'os.RemoveAll'、'runMu.Lock'、'runMu.Unlock'、'startMihomo'、'stop'、'time.Now'、'yaml.Marshal' |
| 103–103 | closure | `RunNodeTest.closure#1` | 供 RunNodeTest 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'os.RemoveAll'、'stop' |
| 130–171 | function | `startMihomo` | 启动与 'start mihomo' 对应的业务或基础设施操作。 | 分支 6；循环 1；返回 5；goroutine 1；调用 'Add'、'Before'、'Dial'、'c.Close'、'cmd.Process.Kill'、'cmd.Process.Signal'、'cmd.Start'、'cmd.Wait'、'exec.Command'、'filepath.Join'、'fmt.Sprintf'、'make'、'once.Do'、'os.MkdirAll'、'os.WriteFile'、'time.Now' |
| 148–164 | closure | `startMihomo.closure#1` | 供 startMihomo 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 0；goroutine 1；调用 'cmd.Process.Kill'、'cmd.Process.Signal'、'cmd.Wait'、'make'、'once.Do'、'time.After' |
| 149–163 | closure | `startMihomo.closure#2` | 供 startMihomo 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 0；goroutine 1；调用 'cmd.Process.Kill'、'cmd.Process.Signal'、'cmd.Wait'、'make'、'time.After' |
| 151–151 | closure | `startMihomo.closure#3` | 供 startMihomo 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'cmd.Wait' |
| 175–177 | function | `proxyClient` | 执行与 'proxy client' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'sharedProxyTransport' |
| 180–180 | var | `sharedTransportOnce` | 保存 'sharedTransportOnce' 的包级共享状态、配置或预计算值。 |  |
| 181–181 | var | `sharedTransport` | 保存 'sharedTransport' 的包级共享状态、配置或预计算值。 |  |
| 184–199 | function | `sharedProxyTransport` | 执行与 'shared proxy transport' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'fmt.Sprintf'、'http.ProxyURL'、'sharedTransportOnce.Do'、'url.Parse' |
| 185–197 | closure | `sharedProxyTransport.closure#1` | 供 sharedProxyTransport 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'fmt.Sprintf'、'http.ProxyURL'、'url.Parse' |
| 202–204 | var | `bigCopyBufPool` | 保存 'bigCopyBufPool' 的包级共享状态、配置或预计算值。 |  |
| 206–221 | function | `measureLatency` | 执行与 'measure latency' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'Milliseconds'、'client.Do'、'http.NewRequestWithContext'、'io.Copy'、'proxyClient'、'resp.Body.Close'、'time.Now'、'time.Since' |
| 225–263 | function | `measureLatencyCloudflare` | 执行与 'measure latency cloudflare' 对应的业务或基础设施操作。 | 分支 6；循环 2；返回 2；goroutine 0；调用 'Milliseconds'、'append'、'client.Do'、'ctx.Err'、'http.NewRequestWithContext'、'int64'、'io.Copy'、'len'、'make'、'proxyClient'、'resp.Body.Close'、'sortInt64Asc'、'time.Now'、'time.Since' |
| 265–271 | function | `sortInt64Asc` | 执行与 'sort int64 asc' 对应的业务或基础设施操作。 | 分支 0；循环 2；返回 0；goroutine 0；调用 'len' |
| 273–297 | function | `measureEgressIP` | 执行与 'measure egress ip' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'client.Do'、'http.NewRequestWithContext'、'io.LimitReader'、'io.ReadAll'、'len'、'proxyClient'、'resp.Body.Close'、'string'、'strings.Contains'、'strings.TrimSpace' |
| 299–335 | function | `downloadTimed` | 执行与 'download timed' 对应的业务或基础设施操作。 | 分支 3；循环 2；返回 3；goroutine 1；调用 'cancel'、'context.WithTimeout'、'downloadSingle'、'make'、'time.Now'、'time.Since'、'wg.Add'、'wg.Done'、'wg.Wait' |
| 313–318 | closure | `downloadTimed.closure#1` | 供 downloadTimed 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'downloadSingle'、'wg.Done' |
| 337–366 | function | `downloadSingle` | 执行与 'download single' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'bigCopyBufPool.Get'、'bigCopyBufPool.Put'、'client.Do'、'ctx.Err'、'fmt.Errorf'、'http.NewRequestWithContext'、'io.CopyBuffer'、'io.LimitReader'、'proxyClient'、'req.Header.Set'、'resp.Body.Close'、'time.Now'、'time.Since' |

