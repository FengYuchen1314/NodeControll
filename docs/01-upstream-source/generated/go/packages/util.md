# Go 包 `util`

跨模块复用的网络、时间、字符串和文件工具。

## `internal/util/yaml.go`

依赖：`fmt`、`strconv`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 11–11 | var | `ProxyPriorityFields` | 保存 'ProxyPriorityFields' 的包级共享状态、配置或预计算值。 |  |
| 15–41 | function | `ReorderProxyFieldsToNode` | 执行与 'reorder proxy fields to node' 对应的业务或基础设施操作。 | 分支 2；循环 2；返回 1；goroutine 0；调用 'append'、'isPriorityField'、'valNode.Encode' |
| 45–83 | function | `ReorderProxyNode` | 执行与 'reorder proxy node' 对应的业务或基础设施操作。 | 分支 4；循环 3；返回 2；goroutine 0；调用 'append'、'isPriorityField'、'len'、'make' |
| 86–93 | function | `isPriorityField` | 判断与 'is priority field' 对应的业务或基础设施操作。 | 分支 1；循环 1；返回 2；goroutine 0 |
| 96–128 | function | `ValueToYAMLNode` | 执行与 'value to yaml node' 对应的业务或基础设施操作。 | 分支 4；循环 0；返回 8；goroutine 0；调用 'float64'、'fmt.Sprintf'、'int64'、'len'、'strconv.FormatFloat'、'strconv.FormatInt'、'strconv.Itoa'、'yaml.Marshal'、'yaml.Unmarshal' |
| 131–147 | function | `GetNodeFieldValue` | 查询或读取与 'get node field value' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 3；goroutine 0；调用 'len' |
| 150–170 | function | `SetNodeField` | 设置与 'set node field' 对应的业务或基础设施操作。 | 分支 2；循环 1；返回 2；goroutine 0；调用 'ValueToYAMLNode'、'append'、'len' |

