# Go 包 `validator`

配置、节点和请求数据的语义校验。

## `internal/validator/clash_validator.go`

依赖：`encoding/json`、`fmt`、`strings`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 10–10 | type | `ValidationLevel` | 定义 'ValidationLevel' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 13–13 | const | `ErrorLevel` | 定义 'ErrorLevel' 的不可变协议值、默认值或枚举成员。 |  |
| 14–14 | const | `WarningLevel` | 定义 'WarningLevel' 的不可变协议值、默认值或枚举成员。 |  |
| 15–15 | const | `InfoLevel` | 定义 'InfoLevel' 的不可变协议值、默认值或枚举成员。 |  |
| 19–25 | type | `ValidationIssue` | 定义 'ValidationIssue' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 28–32 | type | `ValidationResult` | 定义 'ValidationResult' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 35–90 | function | `ValidateClashConfig` | 校验与 'validate clash config' 对应的业务或基础设施操作。 | 分支 8；循环 2；返回 1；goroutine 0；调用 'append'、'deepCopyMap'、'detectCircularReferences'、'validateProxies'、'validateProxyGroups' |
| 93–96 | type | `ProxyValidationResult` | 定义 'ProxyValidationResult' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 99–163 | function | `validateProxies` | 校验与 'validate proxies' 对应的业务或基础设施操作。 | 分支 4；循环 1；返回 1；goroutine 0；调用 'append'、'fmt.Sprintf'、'getMapKeys'、'len'、'make'、'reorderProxyFields'、'strings.TrimSpace' |
| 166–169 | type | `GroupValidationResult` | 定义 'GroupValidationResult' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 172–353 | function | `validateProxyGroups` | 校验与 'validate proxy groups' 对应的业务或基础设施操作。 | 分支 15；循环 4；返回 1；goroutine 0；调用 'append'、'fmt.Sprintf'、'getMapKeys'、'len'、'make'、'reorderGroupFields'、'strings.TrimSpace' |
| 356–434 | function | `detectCircularReferences` | 执行与 'detect circular references' 对应的业务或基础设施操作。 | 分支 11；循环 6；返回 4；goroutine 0；调用 'append'、'dfs'、'fmt.Sprintf'、'make'、'strings.Join' |
| 392–425 | closure | `detectCircularReferences.closure#1` | 供 detectCircularReferences 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 5；循环 2；返回 3；goroutine 0；调用 'append'、'dfs'、'fmt.Sprintf'、'strings.Join' |
| 437–488 | function | `FormatValidationIssues` | 执行与 'format validation issues' 对应的业务或基础设施操作。 | 分支 9；循环 3；返回 2；goroutine 0；调用 'append'、'fmt.Sprintf'、'len'、'message.Len'、'message.String'、'message.WriteString' |
| 492–503 | function | `deepCopyMap` | 执行与 'deep copy map' 对应的业务或基础设施操作。 | 分支 2；循环 0；返回 3；goroutine 0；调用 'json.Marshal'、'json.Unmarshal' |
| 505–513 | function | `getMapKeys` | 查询或读取与 'get map keys' 对应的业务或基础设施操作。 | 分支 0；循环 1；返回 1；goroutine 0；调用 'append'、'len'、'make' |
| 515–534 | function | `reorderProxyFields` | 执行与 'reorder proxy fields' 对应的业务或基础设施操作。 | 分支 2；循环 2；返回 1；goroutine 0；调用 'contains'、'make' |
| 536–555 | function | `reorderGroupFields` | 执行与 'reorder group fields' 对应的业务或基础设施操作。 | 分支 2；循环 2；返回 1；goroutine 0；调用 'contains'、'make' |
| 557–564 | function | `contains` | 执行与 'contains' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0 |

