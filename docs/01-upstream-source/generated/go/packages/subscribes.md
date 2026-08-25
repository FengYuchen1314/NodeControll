# Go 包 `subscribes`

随二进制嵌入并准备默认订阅配置文件。

## `subscribes/embed.go`

依赖：`embed`、`errors`、`io/fs`、`os`、`path/filepath`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 12–12 | var | `files` | 保存 'files' 的包级共享状态、配置或预计算值。 |  |
| 17–62 | function | `Ensure` | 执行与 'ensure' 对应的业务或基础设施操作。 | 分支 9；循环 1；返回 6；goroutine 0；调用 'entry.IsDir'、'entry.Name'、'errors.Is'、'filepath.Join'、'fs.ReadDir'、'fs.ReadFile'、'os.MkdirAll'、'os.Stat'、'os.WriteFile' |

