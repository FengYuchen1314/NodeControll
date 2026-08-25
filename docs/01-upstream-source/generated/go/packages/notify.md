# Go 包 `notify`

Telegram 等外部通知的格式化、发送和开关控制。

## `internal/notify/notifier.go`

依赖：`context`、`fmt`、`sync`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 10–13 | type | `Notifier` | 定义 'Notifier' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 16–18 | function | `New` | 创建并初始化与 'new' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 21–25 | function | `(*Notifier).UpdateConfig` | *Notifier 的方法，更新与 'update config' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 0；goroutine 0；调用 'n.mu.Lock'、'n.mu.Unlock' |
| 28–32 | function | `(*Notifier).GetConfig` | *Notifier 的方法，查询或读取与 'get config' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 'n.mu.RLock'、'n.mu.RUnlock' |
| 35–59 | function | `(*Notifier).IsEnabled` | *Notifier 的方法，判断与 'is enabled' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 8；goroutine 0；调用 'n.mu.RLock'、'n.mu.RUnlock' |
| 62–70 | function | `(*Notifier).Send` | *Notifier 的方法，执行与 'send' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'fmt.Sprintf'、'n.GetConfig'、'n.IsEnabled'、'sendTelegram' |
| 73–79 | function | `(*Notifier).SendTest` | *Notifier 的方法，执行与 'send test' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'fmt.Errorf'、'n.GetConfig'、'sendTelegram' |

## `internal/notify/telegram.go`

依赖：`context`、`encoding/json`、`fmt`、`net/http`、`net/url`、`time`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–12 | const | `telegramAPIBase` | 定义 'telegramAPIBase' 的不可变协议值、默认值或枚举成员。 |  |
| 14–14 | var | `httpClient` | 保存 'httpClient' 的包级共享状态、配置或预计算值。 |  |
| 16–50 | function | `sendTelegram` | 执行与 'send telegram' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 5；goroutine 0；调用 'Decode'、'fmt.Errorf'、'http.NewRequestWithContext'、'httpClient.Do'、'json.NewDecoder'、'params.Encode'、'resp.Body.Close' |

## `internal/notify/types.go`

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 4–4 | type | `EventType` | 定义 'EventType' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 7–7 | const | `EventSubscribeFetch` | 定义 'EventSubscribeFetch' 的不可变协议值、默认值或枚举成员。 |  |
| 8–8 | const | `EventLogin` | 定义 'EventLogin' 的不可变协议值、默认值或枚举成员。 |  |
| 9–9 | const | `EventIPBan` | 定义 'EventIPBan' 的不可变协议值、默认值或枚举成员。 |  |
| 10–10 | const | `EventSilentMode` | 定义 'EventSilentMode' 的不可变协议值、默认值或枚举成员。 |  |
| 11–11 | const | `EventDailyTraffic` | 定义 'EventDailyTraffic' 的不可变协议值、默认值或枚举成员。 |  |
| 12–12 | const | `EventExpiry` | 定义 'EventExpiry' 的不可变协议值、默认值或枚举成员。 |  |
| 17–28 | type | `Config` | 定义 'Config' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 31–35 | type | `Event` | 定义 'Event' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |

