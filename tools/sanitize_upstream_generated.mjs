#!/usr/bin/env node

import fs from 'node:fs'
import path from 'node:path'
import process from 'node:process'
import { fileURLToPath } from 'node:url'

const workspace = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const generatedRoot = path.join(workspace, 'docs', '01-upstream-source', 'generated')
const rewrite = process.argv.includes('--rewrite')
const failures = []
let changed = 0

sanitizeJson(path.join(generatedRoot, 'database-schema.json'), new Set(['sql']))
sanitizeJson(path.join(generatedRoot, 'http-routes.json'), new Set(['statement']))

for (const directory of [
  path.join(generatedRoot, 'go', 'packages'),
  path.join(generatedRoot, 'typescript', 'areas'),
]) {
  for (const file of walkMarkdown(directory)) sanitizeMarkdown(file)
}

if (failures.length !== 0) {
  throw new Error(`unsafe generated upstream analysis remains:\n- ${failures.join('\n- ')}`)
}

console.log(`${rewrite ? 'rewrote' : 'checked'} generated upstream analysis: ${changed} file(s) changed, no source signatures/full SQL/route statements`)

function sanitizeJson(file, forbiddenKeys) {
  const value = JSON.parse(fs.readFileSync(file, 'utf8'))
  const found = []
  visit(value, '$', (object, key, objectPath) => {
    if (!forbiddenKeys.has(key)) return
    found.push(`${objectPath}.${key}`)
    if (rewrite) delete object[key]
  })
  if (found.length !== 0 && !rewrite) {
    failures.push(`${relative(file)} contains forbidden key(s): ${found.slice(0, 4).join(', ')}`)
    return
  }
  if (found.length !== 0) {
    fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`)
    changed += 1
  }
}

function sanitizeMarkdown(file) {
  const original = fs.readFileSync(file, 'utf8')
  let removeColumn = null
  let foundSignatureColumn = false
  let output = original.split(/\r?\n/).map((line) => {
    if (!line.startsWith('|')) {
      removeColumn = null
      return line
    }
    const cells = splitMarkdownRow(line)
    if (cells.length < 3) return line
    const headerIndex = cells.indexOf('签名/定义')
    if (headerIndex >= 0) {
      removeColumn = headerIndex
      foundSignatureColumn = true
    }
    if (removeColumn === null || removeColumn >= cells.length) return line
    cells.splice(removeColumn, 1)
    return `| ${cells.join(' | ')} |`
  }).join('\n')
  output = output
    .replaceAll('字段语义以签名和使用方为准。', '字段语义由符号名称、行号和使用方交叉确认。')
    .replaceAll(
      '> 自动生成索引；作用说明由源码注释、命名、签名、调用和控制流证据共同生成，人工模块解读文档会进一步校正业务语义。',
      '> 自动生成索引；公开版只保留符号、行号、原创作用说明、调用和控制流证据，不公开源码签名或常量字面量。',
    )

  if (foundSignatureColumn && !rewrite) {
    failures.push(`${relative(file)} contains a source signature/definition column`)
  } else if (rewrite && output !== original) {
    fs.writeFileSync(file, output)
    changed += 1
  }
}

function splitMarkdownRow(line) {
  const cells = []
  let current = ''
  for (let index = 1; index < line.length; index += 1) {
    const character = line[index]
    if (character === '|' && line[index - 1] !== '\\') {
      cells.push(current.trim())
      current = ''
    } else {
      current += character
    }
  }
  if (current.trim()) cells.push(current.trim())
  return cells
}

function visit(value, objectPath, callback) {
  if (Array.isArray(value)) {
    value.forEach((item, index) => visit(item, `${objectPath}[${index}]`, callback))
    return
  }
  if (!value || typeof value !== 'object') return
  for (const key of Object.keys(value)) {
    callback(value, key, objectPath)
    if (Object.hasOwn(value, key)) visit(value[key], `${objectPath}.${key}`, callback)
  }
}

function walkMarkdown(directory) {
  if (!fs.existsSync(directory)) return []
  const result = []
  for (const entry of fs.readdirSync(directory, { withFileTypes: true })) {
    const target = path.join(directory, entry.name)
    if (entry.isDirectory()) result.push(...walkMarkdown(target))
    else if (entry.isFile() && entry.name.endsWith('.md')) result.push(target)
  }
  return result.sort()
}

function relative(file) {
  return path.relative(workspace, file).split(path.sep).join('/')
}
