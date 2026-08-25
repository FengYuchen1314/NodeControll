#!/usr/bin/env node
/** Generate an auditable TypeScript/TSX declaration and function inventory. */

import fs from 'node:fs'
import { createRequire } from 'node:module'
import path from 'node:path'
import process from 'node:process'

const requireFromWebWorkspace = createRequire(path.resolve('apps/web/package.json'))
const ts = requireFromWebWorkspace('typescript')

const args = parseArgs(process.argv.slice(2))
const root = path.resolve(args.root ?? 'upstream/miaomiaowu/miaomiaowu/src')
const out = path.resolve(args.out ?? 'docs/01-upstream-source/generated/typescript')
const sourcePaths = walk(root).filter((file) => /\.(?:ts|tsx)$/.test(file)).sort()
const records = sourcePaths.map(analyzeFile)
fs.mkdirSync(path.join(out, 'areas'), { recursive: true })
writeOverview(records)
writeAreas(records)
writeRoutes(records)
writeApiCalls(records)

const functionCount = records.reduce((sum, file) => sum + file.symbols.filter((symbol) => symbol.kind === 'function').length, 0)
const declarationCount = records.reduce((sum, file) => sum + file.symbols.length, 0)
console.log(`documented ${records.length} TypeScript files, ${functionCount} functions, and ${declarationCount} declarations`)

function parseArgs(values) {
  const result = {}
  for (let index = 0; index < values.length; index += 1) {
    if (values[index] === '--root') result.root = values[++index]
    else if (values[index] === '--out') result.out = values[++index]
  }
  return result
}

function walk(directory) {
  const result = []
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const target = path.join(directory, entry.name)
    if (entry.isDirectory()) result.push(...walk(target))
    else result.push(target)
  }
  return result
}

function analyzeFile(filePath) {
  const sourceText = fs.readFileSync(filePath, 'utf8')
  const scriptKind = filePath.endsWith('.tsx') ? ts.ScriptKind.TSX : ts.ScriptKind.TS
  const source = ts.createSourceFile(filePath, sourceText, ts.ScriptTarget.Latest, true, scriptKind)
  const relativePath = slash(path.relative(root, filePath))
  const record = {
    path: relativePath,
    area: areaFor(relativePath),
    generated: relativePath.endsWith('.gen.ts') || sourceText.includes('/* eslint-disable */') && sourceText.includes('generated'),
    imports: [],
    routes: [],
    apiCalls: [],
    symbols: [],
  }
  let anonymousIndex = 0

  for (const statement of source.statements) {
    if (ts.isImportDeclaration(statement) && ts.isStringLiteral(statement.moduleSpecifier)) {
      record.imports.push(statement.moduleSpecifier.text)
    }
    for (const symbol of describeTopLevelDeclaration(statement, source, relativePath)) {
      record.symbols.push(symbol)
    }
  }

  const visit = (node, scope = []) => {
    if (isFunctionLike(node) && !ts.isSourceFile(node)) {
      const name = functionName(node, ++anonymousIndex)
      const nextScope = [...scope, name]
      record.symbols.push(describeFunction(node, name, nextScope, source, relativePath))
      inspectFunctionEvidence(node, source, relativePath, record, nextScope)
      ts.forEachChild(node, (child) => visit(child, nextScope))
      return
    }
    if (ts.isCallExpression(node)) {
      extractRoute(node, source, relativePath, record)
      extractApiCall(node, source, relativePath, record, scope)
    }
    ts.forEachChild(node, (child) => visit(child, scope))
  }
  visit(source)
  record.symbols.sort((left, right) => left.startLine - right.startLine || left.name.localeCompare(right.name))
  record.apiCalls = uniqueBy(record.apiCalls, (item) => `${item.line}:${item.method}:${item.endpoint}`)
  record.routes = uniqueBy(record.routes, (item) => `${item.line}:${item.route}`)
  return record
}

function describeTopLevelDeclaration(node, source, file) {
  const result = []
  const location = linesFor(node, source)
  const add = (kind, name, signature, doc = '') => result.push({
    kind,
    name,
    scope: name,
    signature: compact(signature, 360),
    purpose: declarationPurpose(kind, name, doc),
    doc,
    file,
    ...location,
    calls: [],
    complexity: '',
    component: false,
    hook: false,
  })
  if (ts.isInterfaceDeclaration(node)) add('interface', node.name.text, node.getText(source))
  else if (ts.isTypeAliasDeclaration(node)) add('type', node.name.text, node.getText(source))
  else if (ts.isEnumDeclaration(node)) add('enum', node.name.text, node.getText(source))
  else if (ts.isClassDeclaration(node)) add('class', node.name?.text ?? '<anonymous-class>', headerText(node, source))
  else if (ts.isVariableStatement(node)) {
    for (const declaration of node.declarationList.declarations) {
      if (ts.isIdentifier(declaration.name) && !isFunctionLike(declaration.initializer)) {
        const kind = node.declarationList.flags & ts.NodeFlags.Const ? 'const' : node.declarationList.flags & ts.NodeFlags.Let ? 'let' : 'var'
        add(kind, declaration.name.text, variableSignature(declaration, source))
      }
    }
  }
  return result
}

function variableSignature(declaration, source) {
  const text = declaration.getText(source)
  const initializer = declaration.initializer
  if (!initializer || !sensitiveIdentifier(declaration.name.text)) return text
  if (!ts.isStringLiteral(initializer) && !ts.isNoSubstitutionTemplateLiteral(initializer)) return text
  const prefixLength = initializer.getStart(source) - declaration.getStart(source)
  return `${text.slice(0, prefixLength)}'<redacted-sensitive-source-literal>'`
}

function sensitiveIdentifier(name) {
  const normalized = String(name).toLowerCase().replaceAll('_', '').replaceAll('-', '')
  return ['token', 'secret', 'password', 'credential', 'apikey', 'privatekey'].some((marker) => normalized.includes(marker))
}

function describeFunction(node, name, scope, source, file) {
  const evidence = functionEvidence(node)
  const doc = ''
  return {
    kind: 'function',
    name,
    scope: scope.join(' > '),
    signature: compact(headerText(node, source), 420),
    purpose: functionPurpose(name, doc, evidence.component),
    doc,
    file,
    ...linesFor(node, source),
    calls: evidence.calls,
    complexity: `分支 ${evidence.branches}；循环 ${evidence.loops}；返回 ${evidence.returns}；await ${evidence.awaits}`,
    component: evidence.component,
    hook: /^use[A-Z0-9]/.test(name),
  }
}

function inspectFunctionEvidence(node, source, file, record, scope) {
  const visit = (child) => {
    if (child !== node && isFunctionLike(child)) return
    if (ts.isCallExpression(child)) {
      extractRoute(child, source, file, record)
      extractApiCall(child, source, file, record, scope)
    }
    ts.forEachChild(child, visit)
  }
  visit(node)
}

function functionEvidence(node) {
  const calls = new Set()
  let branches = 0
  let loops = 0
  let returns = 0
  let awaits = 0
  let component = false
  const visit = (child) => {
    if (child !== node && isFunctionLike(child)) return
    if (ts.isCallExpression(child) && calls.size < 18) calls.add(expressionName(child.expression))
    if (ts.isIfStatement(child) || ts.isSwitchStatement(child) || ts.isConditionalExpression(child)) branches += 1
    if (ts.isForStatement(child) || ts.isForInStatement(child) || ts.isForOfStatement(child) || ts.isWhileStatement(child) || ts.isDoStatement(child)) loops += 1
    if (ts.isReturnStatement(child)) returns += 1
    if (ts.isAwaitExpression(child)) awaits += 1
    if (ts.isJsxElement(child) || ts.isJsxSelfClosingElement(child) || ts.isJsxFragment(child)) component = true
    ts.forEachChild(child, visit)
  }
  visit(node)
  return { calls: [...calls].filter(Boolean).sort(), branches, loops, returns, awaits, component }
}

function extractRoute(node, source, file, record) {
  const callee = expressionName(node.expression)
  if (!/(?:createFileRoute|createRootRouteWithContext|createRootRoute)$/.test(callee)) return
  const first = node.arguments[0]
  const isFileRoute = /createFileRoute$/.test(callee)
  const hasLiteralPath = first && (ts.isStringLiteral(first) || ts.isNoSubstitutionTemplateLiteral(first))
  if (isFileRoute && !hasLiteralPath) return
  const route = hasLiteralPath ? first.text : '<root/context route>'
  record.routes.push({ file, line: lineOf(node, source), route, factory: callee })
}

function extractApiCall(node, source, file, record, scope) {
  const callee = expressionName(node.expression)
  const methodMatch = callee.match(/(?:^|\.)(get|post|put|delete|patch|head|options)$/i)
  if (!methodMatch || node.arguments.length === 0) return
  const endpoint = literalText(node.arguments[0])
  if (!endpoint || !endpoint.includes('/api/')) return
  record.apiCalls.push({
    file,
    line: lineOf(node, source),
    method: methodMatch[1].toUpperCase(),
    endpoint: compact(endpoint, 220),
    caller: scope.join(' > ') || '<module>',
    callee,
  })
}

function isFunctionLike(node) {
  return Boolean(node) && (
    ts.isFunctionDeclaration(node) || ts.isFunctionExpression(node) || ts.isArrowFunction(node) ||
    ts.isMethodDeclaration(node) || ts.isConstructorDeclaration(node) || ts.isGetAccessorDeclaration(node) ||
    ts.isSetAccessorDeclaration(node)
  )
}

function functionName(node, anonymousIndex) {
  if (node.name && ts.isIdentifier(node.name)) return node.name.text
  if (node.name && (ts.isStringLiteral(node.name) || ts.isNumericLiteral(node.name))) return node.name.text
  const parent = node.parent
  if (ts.isVariableDeclaration(parent) && ts.isIdentifier(parent.name)) return parent.name.text
  if (ts.isPropertyAssignment(parent)) return propertyName(parent.name)
  if (ts.isPropertyDeclaration(parent)) return propertyName(parent.name)
  if (ts.isJsxExpression(parent) && ts.isJsxAttribute(parent.parent)) {
    const attribute = parent.parent.name
    const attributeName = ts.isIdentifier(attribute)
      ? attribute.text
      : `${attribute.namespace.text}:${attribute.name.text}`
    return `${attributeName}.callback#${anonymousIndex}`
  }
  if (ts.isCallExpression(parent)) return `${expressionName(parent.expression)}.callback#${anonymousIndex}`
  if (ts.isArrayLiteralExpression(parent)) return `array.callback#${anonymousIndex}`
  return `<anonymous#${anonymousIndex}>`
}

function propertyName(node) {
  if (!node) return '<property>'
  if ('text' in node) return String(node.text)
  if (ts.isComputedPropertyName(node)) return '<computed-property>'
  return `<${ts.SyntaxKind[node.kind] ?? 'property'}>`
}

function expressionName(node) {
  if (!node) return ''
  if (ts.isIdentifier(node) || ts.isPrivateIdentifier(node)) return node.text
  if (ts.isPropertyAccessExpression(node)) return `${expressionName(node.expression)}.${node.name.text}`
  if (ts.isElementAccessExpression(node)) return `${expressionName(node.expression)}[<key>]`
  if (ts.isCallExpression(node)) return expressionName(node.expression)
  if (ts.isParenthesizedExpression(node)) return expressionName(node.expression)
  return `<${ts.SyntaxKind[node.kind] ?? 'expression'}>`
}

function literalText(node) {
  if (ts.isStringLiteral(node) || ts.isNoSubstitutionTemplateLiteral(node)) return node.text
  if (ts.isTemplateExpression(node)) {
    return `${node.head.text}${node.templateSpans.map((span) => `<expression>${span.literal.text}`).join('')}`
  }
  return ''
}

function headerText(node, source) {
  const text = node.getText(source)
  if (ts.isArrowFunction(node)) {
    const arrow = text.indexOf('=>')
    return arrow >= 0 ? `${text.slice(0, arrow + 2)} …` : compact(text, 420)
  }
  if (node.body) {
    const bodyStart = node.body.getStart(source) - node.getStart(source)
    return `${text.slice(0, bodyStart).trim()} { … }`
  }
  return text
}

function linesFor(node, source) {
  return { startLine: lineOf(node, source), endLine: source.getLineAndCharacterOfPosition(node.end).line + 1 }
}

function lineOf(node, source) {
  return source.getLineAndCharacterOfPosition(node.getStart(source)).line + 1
}

function areaFor(relativePath) {
  const parts = relativePath.split('/')
  if (parts[0] === 'components' && parts.length > 2) return `components-${parts[1]}`
  if (parts.length > 1) return parts[0]
  return 'root'
}

function areaPurpose(area) {
  const purposes = {
    routes: 'TanStack Router 页面、加载/重定向守卫和页面内业务交互。',
    components: '跨页面复用的业务组件和交互对话框。',
    'components-ui': '基于 Radix UI/Tailwind 的无业务基础 UI 封装。',
    'components-layout': '导航栏、顶栏、用户菜单和应用外壳。',
    'components-template-v3': 'V3 模板编辑、预览、筛选与代理组控件。',
    hooks: '可复用 React Hook、响应式状态和拖拽/媒体查询行为。',
    lib: 'API 客户端、Clash/订阅构建、校验、格式化和通用工具。',
    stores: 'Zustand 全局状态，主要承载认证会话。',
    context: '主题、字体、方向等 React Context。',
    config: '前端预设、字体和覆写脚本模板。',
    root: '应用入口、生成路由树和顶层类型。',
  }
  return purposes[area] ?? '前端源代码分区；职责由文件和符号索引给出。'
}

function functionPurpose(name, doc, component) {
  if (doc) return firstSentence(doc)
  if (component && /^[A-Z]/.test(name)) return `渲染并协调 \`${name}\` React 组件的状态、数据请求和用户交互。`
  if (/^use[A-Z0-9]/.test(name)) return `封装 \`${name}\` Hook 的响应式状态、副作用和复用逻辑。`
  if (/callback#|<anonymous|array\.callback/.test(name)) return '供父级函数、组件或 JSX 属性使用的内联回调；调用与控制流证据见本行。'
  const lower = name.toLowerCase()
  const patterns = [
    ['handle', '处理'], ['get', '读取或计算'], ['set', '设置'], ['fetch', '从后端获取'], ['load', '加载'],
    ['create', '创建'], ['add', '添加'], ['update', '更新'], ['save', '保存'], ['delete', '删除'],
    ['remove', '移除'], ['parse', '解析'], ['format', '格式化'], ['validate', '校验'], ['build', '构建'],
    ['generate', '生成'], ['convert', '转换'], ['normalize', '规范化'], ['toggle', '切换'], ['reset', '重置'],
    ['render', '渲染'], ['filter', '筛选'], ['sort', '排序'], ['map', '映射'], ['is', '判断'], ['has', '判断是否具有'],
  ]
  const verb = patterns.find(([prefix]) => lower.startsWith(prefix))?.[1] ?? '执行'
  return `${verb}与 \`${name}\` 对应的前端业务、状态或数据转换逻辑。`
}

function declarationPurpose(kind, name, doc) {
  if (doc) return firstSentence(doc)
  if (kind === 'interface' || kind === 'type') return `定义 \`${name}\` 的数据契约、联合类型或组件属性。`
  if (kind === 'class') return `封装 \`${name}\` 的实例状态与行为。`
  if (kind === 'enum') return `枚举 \`${name}\` 的受限取值。`
  return `保存 \`${name}\` 的模块级常量、配置、路由或预计算值。`
}

function firstSentence(value) {
  const normalized = value.replace(/\s+/g, ' ').trim()
  const match = normalized.match(/^.*?(?:。|\.\s|$)/)
  return compact(match?.[0] || normalized, 240)
}

function writeOverview(files) {
  const areas = groupBy(files, (file) => file.area)
  const functionCount = files.reduce((sum, file) => sum + file.symbols.filter((item) => item.kind === 'function').length, 0)
  const declarationCount = files.reduce((sum, file) => sum + file.symbols.filter((item) => item.kind !== 'function').length, 0)
  const componentCount = files.reduce((sum, file) => sum + file.symbols.filter((item) => item.component).length, 0)
  const hookCount = files.reduce((sum, file) => sum + file.symbols.filter((item) => item.hook).length, 0)
  const lines = [
    '# TypeScript/TSX 源码符号总览', '',
    '> 使用 TypeScript 5.9 AST 自动生成。公开版只保留符号、行号、原创作用说明、调用和控制流证据，不公开源码签名、常量字面量或表达式正文。', '',
    `- 文件：${files.length}`,
    `- 函数/方法/闭包：${functionCount}`,
    `- 其他顶层声明：${declarationCount}`,
    `- 检测为 React 组件的函数：${componentCount}`,
    `- 自定义 Hook：${hookCount}`,
    `- TanStack 路由：${files.reduce((sum, file) => sum + file.routes.length, 0)}`,
    `- 静态可识别的 /api 调用：${files.reduce((sum, file) => sum + file.apiCalls.length, 0)}`, '',
    '| 分区 | 文件数 | 函数数 | 作用 | 详细索引 |', '|---|---:|---:|---|---|',
  ]
  for (const area of [...areas.keys()].sort()) {
    const areaFiles = areas.get(area)
    const functions = areaFiles.reduce((sum, file) => sum + file.symbols.filter((item) => item.kind === 'function').length, 0)
    lines.push(`| \`${area}\` | ${areaFiles.length} | ${functions} | ${escape(areaPurpose(area))} | [${area}](areas/${area}.md) |`)
  }
  fs.writeFileSync(path.join(out, 'README.md'), `${lines.join('\n')}\n`)
}

function writeAreas(files) {
  const areas = groupBy(files, (file) => file.area)
  for (const area of [...areas.keys()].sort()) {
    const lines = [`# TypeScript 分区 \`${area}\``, '', areaPurpose(area), '']
    for (const file of areas.get(area)) {
      lines.push(`## \`${file.path}\``, '')
      if (file.generated) lines.push('> 此文件由工具生成；仍纳入符号清单，但重构时不手工移植。', '')
      if (file.imports.length) lines.push(`依赖：\`${file.imports.join('`、`')}\`。`, '')
      lines.push('| 行 | 类别 | 符号/作用域 | 作用 | 调用与复杂度证据 |', '|---:|---|---|---|---|')
      for (const item of file.symbols) {
        let evidence = item.complexity
        if (item.calls.length) evidence += `${evidence ? '；' : ''}调用 \`${item.calls.join('`、`')}\``
        lines.push(`| ${item.startLine}–${item.endLine} | ${item.kind} | \`${escape(item.scope)}\` | ${escape(item.purpose)} | ${escape(compact(evidence, 360))} |`)
      }
      lines.push('')
    }
    fs.writeFileSync(path.join(out, 'areas', `${area}.md`), `${lines.join('\n')}\n`)
  }
}

function writeRoutes(files) {
  const routes = files.flatMap((file) => file.routes).sort((left, right) => left.route.localeCompare(right.route))
  const lines = ['# 前端路由索引', '', '| 路由 | 工厂 | 文件:行 |', '|---|---|---|']
  for (const route of routes) lines.push(`| \`${escape(route.route)}\` | \`${escape(route.factory)}\` | \`${route.file}:${route.line}\` |`)
  fs.writeFileSync(path.join(out, 'routes.md'), `${lines.join('\n')}\n`)
}

function writeApiCalls(files) {
  const calls = files.flatMap((file) => file.apiCalls).sort((left, right) => left.endpoint.localeCompare(right.endpoint) || left.file.localeCompare(right.file))
  const lines = ['# 前端 API 调用索引', '', '> 仅列出 TypeScript AST 能静态识别且参数文本包含 `/api/` 的请求；动态拼接会在人工 API 对照中补齐。', '', '| 方法 | 端点表达式 | 调用者 | 文件:行 | 客户端 |', '|---|---|---|---|---|']
  for (const call of calls) lines.push(`| ${call.method} | \`${escape(call.endpoint)}\` | \`${escape(call.caller)}\` | \`${call.file}:${call.line}\` | \`${escape(call.callee)}\` |`)
  fs.writeFileSync(path.join(out, 'api-calls.md'), `${lines.join('\n')}\n`)
}

function groupBy(values, keyOf) {
  const result = new Map()
  for (const value of values) {
    const key = keyOf(value)
    if (!result.has(key)) result.set(key, [])
    result.get(key).push(value)
  }
  return result
}

function uniqueBy(values, keyOf) {
  const seen = new Set()
  return values.filter((value) => {
    const key = keyOf(value)
    if (seen.has(key)) return false
    seen.add(key)
    return true
  })
}

function compact(value, max) {
  const normalized = String(value ?? '').replace(/\s+/g, ' ').trim()
  return [...normalized].length <= max ? normalized : `${[...normalized].slice(0, max - 1).join('')}…`
}

function escape(value) {
  return String(value ?? '').replaceAll('|', '\\|').replaceAll('`', "'").replaceAll('\n', ' ')
}

function slash(value) {
  return value.split(path.sep).join('/')
}
