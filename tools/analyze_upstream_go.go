// Command analyze_upstream_go generates an auditable Go declaration and
// function inventory for the cloned 妙妙屋 source tree.
package main

import (
	"bytes"
	"flag"
	"fmt"
	"go/ast"
	"go/format"
	"go/parser"
	"go/token"
	"os"
	"path/filepath"
	"sort"
	"strings"
	"unicode"
)

type symbol struct {
	Kind       string
	Name       string
	Receiver   string
	Signature  string
	Purpose    string
	Doc        string
	File       string
	StartLine  int
	EndLine    int
	Calls      []string
	Complexity string
}

type fileInfo struct {
	Path    string
	Package string
	Imports []string
	Symbols []symbol
}

var packagePurposes = map[string]string{
	"main":          "组合全部基础设施与 HTTP 端点，启动和优雅停止单体服务。",
	"auth":          "用户认证、密码、会话令牌、角色授权和两步验证上下文。",
	"captcha":       "Cloudflare Turnstile 配置读取与服务端验证码校验。",
	"handler":       "HTTP/WebSocket/SSE 适配层以及多数业务编排逻辑。",
	"logger":        "结构化日志、日志文件轮转和历史日志清理。",
	"notify":        "Telegram 等外部通知的格式化、发送和开关控制。",
	"patches":       "对历史配置文件做幂等、精确匹配的数据修补。",
	"proxygroups":   "代理组远程配置的拉取、验证、内存缓存和查询。",
	"scriptengine":  "基于 goja 的 JavaScript 覆写脚本执行沙箱与对象转换。",
	"speedtest":     "Mihomo/远程测试器驱动的节点测速模型和执行能力。",
	"storage":       "SQLite 建表、迁移、Repository 方法和持久化数据模型。",
	"taskrun":       "后台任务运行记录、状态和可观测性封装。",
	"util":          "跨模块复用的网络、时间、字符串和文件工具。",
	"validator":     "配置、节点和请求数据的语义校验。",
	"version":       "构建版本、更新通道和版本比较。",
	"web":           "嵌入式前端静态资源和 SPA fallback HTTP Handler。",
	"ruletemplates": "随二进制嵌入并落盘默认规则模板。",
	"subscribes":    "随二进制嵌入并准备默认订阅配置文件。",
}

func main() {
	root := flag.String("root", "upstream/miaomiaowu", "upstream source root")
	out := flag.String("out", "docs/01-upstream-source/generated/go", "output directory")
	flag.Parse()

	absRoot, err := filepath.Abs(*root)
	must(err)
	files, err := parseTree(absRoot)
	must(err)
	must(os.MkdirAll(*out, 0o755))
	must(writeOverview(*out, files))
	must(writePackages(*out, files))
	fmt.Printf("documented %d Go files and %d symbols\n", len(files), symbolCount(files))
}

func parseTree(root string) ([]fileInfo, error) {
	fset := token.NewFileSet()
	var paths []string
	err := filepath.WalkDir(root, func(path string, entry os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return walkErr
		}
		if entry.IsDir() && entry.Name() == ".git" {
			return filepath.SkipDir
		}
		if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".go") {
			paths = append(paths, path)
		}
		return nil
	})
	if err != nil {
		return nil, err
	}
	sort.Strings(paths)
	result := make([]fileInfo, 0, len(paths))
	for _, path := range paths {
		parsed, parseErr := parser.ParseFile(fset, path, nil, parser.ParseComments|parser.SkipObjectResolution)
		if parseErr != nil {
			return nil, parseErr
		}
		rel, _ := filepath.Rel(root, path)
		info := fileInfo{Path: filepath.ToSlash(rel), Package: parsed.Name.Name}
		for _, spec := range parsed.Imports {
			info.Imports = append(info.Imports, strings.Trim(spec.Path.Value, "\""))
		}
		for _, decl := range parsed.Decls {
			switch node := decl.(type) {
			case *ast.FuncDecl:
				info.Symbols = append(info.Symbols, describeFunc(fset, info.Path, node))
				info.Symbols = append(info.Symbols, describeClosures(fset, info.Path, node)...)
			case *ast.GenDecl:
				info.Symbols = append(info.Symbols, describeGeneral(fset, info.Path, node)...)
			}
		}
		result = append(result, info)
	}
	return result, nil
}

func describeFunc(fset *token.FileSet, file string, fn *ast.FuncDecl) symbol {
	receiver := ""
	if fn.Recv != nil && len(fn.Recv.List) > 0 {
		receiver = nodeText(fset, fn.Recv.List[0].Type)
	}
	name := fn.Name.Name
	doc := ""
	start, end := fset.Position(fn.Pos()).Line, fset.Position(fn.End()).Line
	calls, branches, loops, returns, goroutines := bodyEvidence(fset, fn.Body)
	return symbol{
		Kind:       "function",
		Name:       name,
		Receiver:   receiver,
		Signature:  signatureText(fset, fn),
		Purpose:    inferPurpose(name, receiver, doc, false),
		Doc:        doc,
		File:       file,
		StartLine:  start,
		EndLine:    end,
		Calls:      calls,
		Complexity: fmt.Sprintf("分支 %d；循环 %d；返回 %d；goroutine %d", branches, loops, returns, goroutines),
	}
}

func describeClosures(fset *token.FileSet, file string, fn *ast.FuncDecl) []symbol {
	if fn.Body == nil {
		return nil
	}
	var result []symbol
	index := 0
	ast.Inspect(fn.Body, func(node ast.Node) bool {
		literal, ok := node.(*ast.FuncLit)
		if !ok {
			return true
		}
		index++
		calls, branches, loops, returns, goroutines := bodyEvidence(fset, literal.Body)
		result = append(result, symbol{
			Kind:       "closure",
			Name:       fmt.Sprintf("%s.closure#%d", fn.Name.Name, index),
			Signature:  "func" + nodeText(fset, literal.Type)[4:],
			Purpose:    inferPurpose(fn.Name.Name, "", "", true),
			File:       file,
			StartLine:  fset.Position(literal.Pos()).Line,
			EndLine:    fset.Position(literal.End()).Line,
			Calls:      calls,
			Complexity: fmt.Sprintf("分支 %d；循环 %d；返回 %d；goroutine %d", branches, loops, returns, goroutines),
		})
		return true
	})
	return result
}

func describeGeneral(fset *token.FileSet, file string, decl *ast.GenDecl) []symbol {
	var result []symbol
	for _, raw := range decl.Specs {
		switch spec := raw.(type) {
		case *ast.TypeSpec:
			doc := ""
			result = append(result, symbol{
				Kind: "type", Name: spec.Name.Name, Signature: "type " + spec.Name.Name + " " + nodeText(fset, spec.Type),
				Purpose: inferTypePurpose(spec.Name.Name, doc), Doc: doc, File: file,
				StartLine: fset.Position(spec.Pos()).Line, EndLine: fset.Position(spec.End()).Line,
			})
		case *ast.ValueSpec:
			kind := strings.ToLower(decl.Tok.String())
			doc := ""
			for _, name := range spec.Names {
				result = append(result, symbol{
					Kind: kind, Name: name.Name, Signature: valueSignature(fset, kind, name.Name, spec),
					Purpose: inferValuePurpose(name.Name, kind, doc), Doc: doc, File: file,
					StartLine: fset.Position(spec.Pos()).Line, EndLine: fset.Position(spec.End()).Line,
				})
			}
		}
	}
	return result
}

func bodyEvidence(fset *token.FileSet, body *ast.BlockStmt) ([]string, int, int, int, int) {
	if body == nil {
		return nil, 0, 0, 0, 0
	}
	seen := map[string]bool{}
	var calls []string
	branches, loops, returns, goroutines := 0, 0, 0, 0
	ast.Inspect(body, func(node ast.Node) bool {
		switch value := node.(type) {
		case *ast.IfStmt, *ast.SwitchStmt, *ast.TypeSwitchStmt, *ast.SelectStmt:
			branches++
		case *ast.ForStmt, *ast.RangeStmt:
			loops++
		case *ast.ReturnStmt:
			returns++
		case *ast.GoStmt:
			goroutines++
		case *ast.CallExpr:
			name := expressionName(value.Fun)
			if name != "" && !seen[name] && len(calls) < 16 {
				seen[name] = true
				calls = append(calls, name)
			}
		}
		return true
	})
	sort.Strings(calls)
	_ = fset
	return calls, branches, loops, returns, goroutines
}

func expressionName(expr ast.Expr) string {
	switch node := expr.(type) {
	case *ast.Ident:
		return node.Name
	case *ast.SelectorExpr:
		prefix := expressionName(node.X)
		if prefix == "" {
			return node.Sel.Name
		}
		return prefix + "." + node.Sel.Name
	case *ast.IndexExpr:
		return expressionName(node.X)
	case *ast.IndexListExpr:
		return expressionName(node.X)
	}
	return ""
}

func signatureText(fset *token.FileSet, fn *ast.FuncDecl) string {
	clone := *fn
	clone.Body = nil
	clone.Doc = nil
	return strings.TrimSpace(nodeText(fset, &clone))
}

func valueSignature(fset *token.FileSet, kind, name string, spec *ast.ValueSpec) string {
	parts := []string{kind, name}
	if spec.Type != nil {
		parts = append(parts, nodeText(fset, spec.Type))
	}
	if len(spec.Values) == 1 {
		value := compact(nodeText(fset, spec.Values[0]), 160)
		if literal, ok := spec.Values[0].(*ast.BasicLit); ok && literal.Kind == token.STRING && sensitiveIdentifier(name) {
			value = `"<redacted-sensitive-source-literal>"`
		}
		parts = append(parts, "=", value)
	}
	return strings.Join(parts, " ")
}

func sensitiveIdentifier(name string) bool {
	normalized := strings.ToLower(strings.NewReplacer("_", "", "-", "").Replace(name))
	for _, marker := range []string{"token", "secret", "password", "credential", "apikey", "privatekey"} {
		if strings.Contains(normalized, marker) {
			return true
		}
	}
	return false
}

func nodeText(fset *token.FileSet, node any) string {
	var buffer bytes.Buffer
	if err := format.Node(&buffer, fset, node); err != nil {
		return "<format-error>"
	}
	return buffer.String()
}

func cleanDoc(group *ast.CommentGroup) string {
	if group == nil {
		return ""
	}
	return strings.TrimSpace(strings.Join(strings.Fields(group.Text()), " "))
}

func inferPurpose(name, receiver, doc string, closure bool) string {
	if doc != "" {
		return firstSentence(doc)
	}
	if closure {
		return "供 " + name + " 内部使用的匿名回调/并发任务；调用与控制流证据见本行后续列。"
	}
	words := splitIdentifier(name)
	object := strings.Join(words, " ")
	verb := "执行"
	lower := strings.ToLower(name)
	patterns := []struct{ Prefix, Verb string }{
		{"new", "创建并初始化"}, {"get", "查询或读取"}, {"list", "列举"}, {"find", "查找"},
		{"load", "加载"}, {"fetch", "从外部获取"}, {"create", "创建"}, {"add", "添加"},
		{"insert", "写入"}, {"update", "更新"}, {"set", "设置"}, {"save", "持久化"},
		{"delete", "删除"}, {"remove", "移除"}, {"cleanup", "清理"}, {"parse", "解析"},
		{"validate", "校验"}, {"is", "判断"}, {"has", "判断是否具有"}, {"can", "判断是否允许"},
		{"handle", "处理"}, {"serve", "提供 HTTP 服务"}, {"build", "构建"}, {"generate", "生成"},
		{"convert", "转换"}, {"import", "导入"}, {"export", "导出"}, {"sync", "同步"},
		{"run", "运行"}, {"start", "启动"}, {"stop", "停止"}, {"reset", "重置"},
		{"apply", "应用"}, {"check", "检查"}, {"resolve", "解析或求解"}, {"normalize", "规范化"},
	}
	for _, pattern := range patterns {
		if strings.HasPrefix(lower, pattern.Prefix) {
			verb = pattern.Verb
			break
		}
	}
	if name == "main" {
		return "程序入口：组装依赖、注册路由、启动后台任务和 HTTP 服务，并处理优雅退出。"
	}
	if name == "init" {
		return "包初始化钩子：在首次使用包前准备全局状态或注册信息。"
	}
	scope := ""
	if receiver != "" {
		scope = receiver + " 的方法，"
	}
	return fmt.Sprintf("%s%s与 `%s` 对应的业务或基础设施操作。", scope, verb, object)
}

func inferTypePurpose(name, doc string) string {
	if doc != "" {
		return firstSentence(doc)
	}
	return fmt.Sprintf("定义 `%s` 的数据结构、接口或别名；字段语义由符号名称、行号和使用方交叉确认。", name)
}

func inferValuePurpose(name, kind, doc string) string {
	if doc != "" {
		return firstSentence(doc)
	}
	if kind == "const" {
		return fmt.Sprintf("定义 `%s` 的不可变协议值、默认值或枚举成员。", name)
	}
	return fmt.Sprintf("保存 `%s` 的包级共享状态、配置或预计算值。", name)
}

func firstSentence(value string) string {
	for _, separator := range []string{"。", ". ", "\n"} {
		if index := strings.Index(value, separator); index >= 0 {
			return strings.TrimSpace(value[:index+len(separator)])
		}
	}
	return compact(value, 240)
}

func splitIdentifier(value string) []string {
	var words []string
	start := 0
	runes := []rune(value)
	for index := 1; index < len(runes); index++ {
		if unicode.IsUpper(runes[index]) && (unicode.IsLower(runes[index-1]) || (index+1 < len(runes) && unicode.IsLower(runes[index+1]))) {
			words = append(words, strings.ToLower(string(runes[start:index])))
			start = index
		}
	}
	words = append(words, strings.ToLower(string(runes[start:])))
	return words
}

func writeOverview(out string, files []fileInfo) error {
	packages := map[string]int{}
	functions, closures, types, values := 0, 0, 0, 0
	for _, file := range files {
		packages[file.Package]++
		for _, item := range file.Symbols {
			switch item.Kind {
			case "function":
				functions++
			case "closure":
				closures++
			case "type":
				types++
			default:
				values++
			}
		}
	}
	names := make([]string, 0, len(packages))
	for name := range packages {
		names = append(names, name)
	}
	sort.Strings(names)
	var b strings.Builder
	b.WriteString("# Go 源码符号总览\n\n")
	b.WriteString("> 自动生成索引；公开版只保留符号、行号、原创作用说明、调用和控制流证据，不公开源码签名或常量字面量。\n\n")
	fmt.Fprintf(&b, "- 文件：%d\n- 包：%d\n- 具名函数/方法：%d\n- 匿名闭包：%d\n- 类型：%d\n- 常量/变量：%d\n\n", len(files), len(packages), functions, closures, types, values)
	b.WriteString("| 包 | 文件数 | 模块作用 | 详细索引 |\n|---|---:|---|---|\n")
	for _, name := range names {
		purpose := packagePurposes[name]
		if purpose == "" {
			purpose = "上游源码包；详细职责见文件与符号索引。"
		}
		fmt.Fprintf(&b, "| `%s` | %d | %s | [%s](packages/%s.md) |\n", name, packages[name], escape(purpose), name, name)
	}
	return os.WriteFile(filepath.Join(out, "README.md"), []byte(b.String()), 0o644)
}

func writePackages(out string, files []fileInfo) error {
	byPackage := map[string][]fileInfo{}
	for _, file := range files {
		byPackage[file.Package] = append(byPackage[file.Package], file)
	}
	packageDir := filepath.Join(out, "packages")
	if err := os.MkdirAll(packageDir, 0o755); err != nil {
		return err
	}
	names := make([]string, 0, len(byPackage))
	for name := range byPackage {
		names = append(names, name)
	}
	sort.Strings(names)
	for _, name := range names {
		var b strings.Builder
		fmt.Fprintf(&b, "# Go 包 `%s`\n\n", name)
		purpose := packagePurposes[name]
		if purpose == "" {
			purpose = "上游源码包；职责依据下列文件和调用关系确定。"
		}
		fmt.Fprintf(&b, "%s\n\n", purpose)
		for _, file := range byPackage[name] {
			fmt.Fprintf(&b, "## `%s`\n\n", file.Path)
			if len(file.Imports) > 0 {
				fmt.Fprintf(&b, "依赖：`%s`。\n\n", strings.Join(file.Imports, "`、`"))
			}
			b.WriteString("| 行 | 类别 | 符号 | 作用 | 调用与复杂度证据 |\n|---:|---|---|---|---|\n")
			for _, item := range file.Symbols {
				nameText := item.Name
				if item.Receiver != "" {
					nameText = "(" + item.Receiver + ")." + item.Name
				}
				evidence := item.Complexity
				if len(item.Calls) > 0 {
					if evidence != "" {
						evidence += "；"
					}
					evidence += "调用 `" + strings.Join(item.Calls, "`、`") + "`"
				}
				fmt.Fprintf(&b, "| %d–%d | %s | `%s` | %s | %s |\n",
					item.StartLine, item.EndLine, item.Kind, escape(nameText), escape(item.Purpose),
					escape(compact(evidence, 360)))
			}
			b.WriteString("\n")
		}
		if err := os.WriteFile(filepath.Join(packageDir, name+".md"), []byte(b.String()), 0o644); err != nil {
			return err
		}
	}
	return nil
}

func symbolCount(files []fileInfo) int {
	total := 0
	for _, file := range files {
		total += len(file.Symbols)
	}
	return total
}

func compact(value string, max int) string {
	value = strings.Join(strings.Fields(value), " ")
	if len([]rune(value)) <= max {
		return value
	}
	runes := []rune(value)
	return string(runes[:max-1]) + "…"
}

func escape(value string) string {
	value = strings.ReplaceAll(value, "|", "\\|")
	value = strings.ReplaceAll(value, "`", "'")
	return strings.ReplaceAll(value, "\n", " ")
}

func must(err error) {
	if err != nil {
		panic(err)
	}
}
