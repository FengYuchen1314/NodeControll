# Go 包 `proxygroups`

代理组远程配置的拉取、验证、内存缓存和查询。

## `internal/proxygroups/normalize.go`

依赖：`encoding/json`、`fmt`、`net/url`、`path`、`strings`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–12 | const | `defaultRuleProviderType` | 定义 'defaultRuleProviderType' 的不可变协议值、默认值或枚举成员。 |  |
| 13–13 | const | `defaultRuleProviderFormat` | 定义 'defaultRuleProviderFormat' 的不可变协议值、默认值或枚举成员。 |  |
| 14–14 | const | `defaultRuleProviderInterval` | 定义 'defaultRuleProviderInterval' 的不可变协议值、默认值或枚举成员。 |  |
| 15–15 | const | `defaultPreset` | 定义 'defaultPreset' 的不可变协议值、默认值或枚举成员。 |  |
| 16–16 | const | `metaRulesBaseURL` | 定义 'metaRulesBaseURL' 的不可变协议值、默认值或枚举成员。 |  |
| 20–28 | type | `RuleProviderConfig` | 定义 'RuleProviderConfig' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 31–41 | type | `ProxyGroupCategory` | 定义 'ProxyGroupCategory' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 44–63 | function | `NormalizeConfig` | 规范化与 'normalize config' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 3；goroutine 0；调用 'fmt.Errorf'、'json.Marshal'、'json.Unmarshal'、'len'、'normalizeCategory'、'string'、'strings.TrimSpace' |
| 65–116 | function | `normalizeCategory` | 规范化与 'normalize category' 对应的业务或基础设施操作。 | 分支 9；循环 2；返回 0；goroutine 0；调用 'normalizeRuleProvider'、'strings.TrimSpace' |
| 118–147 | function | `normalizeRuleProvider` | 规范化与 'normalize rule provider' 对应的业务或基础设施操作。 | 分支 7；循环 0；返回 0；goroutine 0；调用 'fmt.Sprintf'、'inferRuleFormat'、'inferRuleKey'、'strings.TrimSpace' |
| 149–166 | function | `inferRuleFormat` | 执行与 'infer rule format' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'path.Ext'、'strings.ToLower'、'strings.TrimPrefix'、'url.Parse' |
| 168–190 | function | `inferRuleKey` | 执行与 'infer rule key' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 5；goroutine 0；调用 'extractBaseName'、'path.Base'、'path.Ext'、'strings.TrimSpace'、'strings.TrimSuffix'、'url.Parse' |
| 169–179 | closure | `inferRuleKey.closure#1` | 供 inferRuleKey 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'path.Base'、'path.Ext'、'strings.TrimSpace'、'strings.TrimSuffix' |

## `internal/proxygroups/normalize_test.go`

依赖：`encoding/json`、`testing`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 9–73 | function | `TestNormalizeConfigAppliesDefaults` | 执行与 'test normalize config applies defaults' 对应的业务或基础设施操作。 | 分支 15；循环 0；返回 0；goroutine 0；调用 'NormalizeConfig'、'json.Unmarshal'、'len'、't.Fatalf' |
| 75–111 | function | `TestNormalizeConfigInfersKeyAndFormatFromURL` | 执行与 'test normalize config infers key and format from url' 对应的业务或基础设施操作。 | 分支 6；循环 0；返回 0；goroutine 0；调用 'NormalizeConfig'、'json.Unmarshal'、't.Fatalf' |
| 113–145 | function | `TestStoreUpdateNormalizesData` | 执行与 'test store update normalizes data' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 0；goroutine 0；调用 'NewStore'、'json.Unmarshal'、'len'、'store.Snapshot'、'store.Update'、't.Fatalf'、'time.Date' |
| 147–175 | function | `TestNormalizeConfigPreservesExplicitEmptyPresets` | 执行与 'test normalize config preserves explicit empty presets' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 0；goroutine 0；调用 'NormalizeConfig'、'json.Unmarshal'、'len'、't.Fatalf' |
| 177–210 | function | `TestNormalizeConfigEmojiIconCanUseEitherOne` | 执行与 'test normalize config emoji icon can use either one' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 0；goroutine 0；调用 'NormalizeConfig'、'json.Unmarshal'、'len'、't.Fatalf' |

## `internal/proxygroups/proxygroups.go`

依赖：`errors`、`fmt`、`io`、`net/http`、`os`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 14–14 | const | `DefaultSourceURL` | 定义 'DefaultSourceURL' 的不可变协议值、默认值或枚举成员。 |  |
| 18–18 | var | `ErrInvalidConfig` | 保存 'ErrInvalidConfig' 的包级共享状态、配置或预计算值。 |  |
| 19–19 | var | `ErrDownloadFailed` | 保存 'ErrDownloadFailed' 的包级共享状态、配置或预计算值。 |  |
| 22–24 | var | `httpClient` | 保存 'httpClient' 的包级共享状态、配置或预计算值。 |  |
| 28–38 | function | `ResolveSourceURL` | 解析或求解与 'resolve source url' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'os.Getenv' |
| 46–61 | function | `FetchConfig` | 从外部获取与 'fetch config' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'NormalizeConfig'、'ResolveSourceURL'、'downloadConfig' |
| 64–81 | function | `downloadConfig` | 执行与 'download config' 对应的业务或基础设施操作。 | 分支 3；循环 0；返回 4；goroutine 0；调用 'fmt.Errorf'、'httpClient.Get'、'io.ReadAll'、'resp.Body.Close' |

## `internal/proxygroups/store.go`

依赖：`encoding/json`、`fmt`、`sync`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 11–16 | type | `Store` | 定义 'Store' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 19–40 | function | `NewStore` | 创建并初始化与 'new store' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'NormalizeConfig'、'copy'、'fmt.Errorf'、'len'、'make'、'time.Now' |
| 43–52 | function | `(*Store).Snapshot` | *Store 的方法，执行与 'snapshot' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'copy'、'len'、'make'、's.mu.RLock'、's.mu.RUnlock' |
| 55–78 | function | `(*Store).Update` | *Store 的方法，更新与 'update' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 2；goroutine 0；调用 'NormalizeConfig'、'copy'、'fmt.Errorf'、'len'、'make'、's.mu.Lock'、's.mu.Unlock'、'syncedAt.IsZero'、'time.Now' |
| 81–90 | function | `(*Store).Unmarshal` | *Store 的方法，执行与 'unmarshal' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'json.Unmarshal'、's.mu.RLock'、's.mu.RUnlock' |

