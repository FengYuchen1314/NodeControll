# Go 包 `captcha`

Cloudflare Turnstile 配置读取与服务端验证码校验。

## `internal/captcha/turnstile.go`

依赖：`context`、`encoding/json`、`net/http`、`net/url`、`strings`、`time`、`miaomiaowu/internal/storage`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 20–20 | const | `settingKeySiteKey` | 定义 'settingKeySiteKey' 的不可变协议值、默认值或枚举成员。 |  |
| 21–21 | const | `settingKeySecretKey` | 定义 'settingKeySecretKey' 的不可变协议值、默认值或枚举成员。 |  |
| 22–22 | const | `siteVerifyURL` | 定义 'siteVerifyURL' 的不可变协议值、默认值或枚举成员。 |  |
| 25–28 | type | `Turnstile` | 定义 'Turnstile' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 30–35 | function | `New` | 创建并初始化与 'new' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0 |
| 38–45 | function | `(*Turnstile).Enabled` | *Turnstile 的方法，执行与 'enabled' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'strings.TrimSpace'、't.repo.GetSystemSetting' |
| 49–55 | function | `(*Turnstile).SiteKey` | *Turnstile 的方法，执行与 'site key' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 2；goroutine 0；调用 'strings.TrimSpace'、't.repo.GetSystemSetting' |
| 59–62 | function | `(*Turnstile).Verify` | *Turnstile 的方法，执行与 'verify' 对应的业务或基础设施操作。 | 分支 0；循环 0；返回 1；goroutine 0；调用 't.VerifyDetailed' |
| 70–78 | type | `VerifyResult` | 定义 'VerifyResult' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 82–122 | function | `(*Turnstile).VerifyDetailed` | *Turnstile 的方法，执行与 'verify detailed' 对应的业务或基础设施操作。 | 分支 5；循环 0；返回 6；goroutine 0；调用 'Decode'、'Encode'、'http.NewRequestWithContext'、'json.NewDecoder'、'req.Header.Set'、'resp.Body.Close'、'strings.NewReader'、'strings.TrimSpace'、't.Enabled'、't.client.Do'、't.repo.GetSystemSetting' |

