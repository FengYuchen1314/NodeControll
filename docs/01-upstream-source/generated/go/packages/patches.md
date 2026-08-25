# Go 包 `patches`

对历史配置文件做幂等、精确匹配的数据修补。

## `internal/patches/dns_template_patches.go`

依赖：`bytes`、`fmt`、`log`、`os`、`path/filepath`、`reflect`、`regexp`、`strings`、`gopkg.in/yaml.v3`。

| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |
|---:|---|---|---|---|
| 33–37 | type | `dnsPatch` | 定义 'dnsPatch' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 39–127 | var | `dnsPatches` | 保存 'dnsPatches' 的包级共享状态、配置或预计算值。 |  |
| 130–134 | type | `compiledPatch` | 定义 'compiledPatch' 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。 |  |
| 145–183 | function | `ApplyDNSPatches` | 应用与 'apply dns patches' 对应的业务或基础设施操作。 | 分支 8；循环 1；返回 4；goroutine 0；调用 'compilePatches'、'entry.IsDir'、'entry.Name'、'filepath.Join'、'fmt.Errorf'、'log.Printf'、'os.IsNotExist'、'os.ReadDir'、'strings.HasSuffix'、'strings.ToLower'、'tryApplyToFile' |
| 185–206 | function | `compilePatches` | 执行与 'compile patches' 对应的业务或基础设施操作。 | 分支 3；循环 1；返回 4；goroutine 0；调用 'append'、'fmt.Errorf'、'len'、'make'、'yaml.Unmarshal' |
| 210–275 | function | `tryApplyToFile` | 执行与 'try apply to file' 对应的业务或基础设施操作。 | 分支 11；循环 2；返回 11；goroutine 0；调用 'buf.Bytes'、'dnsValue.Decode'、'enc.Close'、'enc.Encode'、'enc.SetIndent'、'fmt.Errorf'、'len'、'os.ReadFile'、'os.Remove'、'os.Rename'、'os.WriteFile'、'reflect.DeepEqual'、'unescapeUnicodeEmoji'、'yaml.NewEncoder'、'yaml.Unmarshal' |
| 278–278 | var | `unicodeEscapeRe` | 保存 'unicodeEscapeRe' 的包级共享状态、配置或预计算值。 |  |
| 283–292 | function | `unescapeUnicodeEmoji` | 执行与 'unescape unicode emoji' 对应的业务或基础设施操作。 | 分支 0；循环 1；返回 2；goroutine 0；调用 'hexNibble'、'string'、'unicodeEscapeRe.ReplaceAllFunc' |
| 284–291 | closure | `unescapeUnicodeEmoji.closure#1` | 供 unescapeUnicodeEmoji 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。 | 分支 0；循环 1；返回 1；goroutine 0；调用 'hexNibble'、'string' |
| 294–304 | function | `hexNibble` | 执行与 'hex nibble' 对应的业务或基础设施操作。 | 分支 1；循环 0；返回 4；goroutine 0；调用 'rune' |

