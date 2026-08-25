#!/usr/bin/env node

import fs from 'node:fs'
import path from 'node:path'
import process from 'node:process'

const rootArgument = process.argv[2]
if (!rootArgument) throw new Error('usage: node tools/verify_web_artifact.mjs <web-dist-directory>')

const root = fs.realpathSync(rootArgument)
const indexPath = path.join(root, 'index.html')
const index = fs.readFileSync(indexPath, 'utf8')
const references = [...index.matchAll(/\b(?:src|href)=["']([^"']+)["']/g)].map((match) => match[1])
const localAssets = references.filter((reference) =>
  !reference.startsWith('http://') &&
  !reference.startsWith('https://') &&
  !reference.startsWith('//') &&
  !reference.startsWith('data:') &&
  !reference.startsWith('#'),
)

if (!localAssets.some((reference) => reference.replace(/[?#].*$/, '').endsWith('.js'))) {
  throw new Error('index.html does not reference a JavaScript application bundle')
}

for (const reference of localAssets) {
  const pathname = decodeURIComponent(reference.replace(/[?#].*$/, '')).replace(/^\/+/, '')
  const target = path.resolve(root, pathname)
  if (target !== root && !target.startsWith(`${root}${path.sep}`)) {
    throw new Error(`asset escapes Web artifact root: ${reference}`)
  }
  const real = fs.realpathSync(target)
  if (real !== root && !real.startsWith(`${root}${path.sep}`)) {
    throw new Error(`asset symlink escapes Web artifact root: ${reference}`)
  }
  if (!fs.statSync(real).isFile() || fs.statSync(real).size === 0) {
    throw new Error(`referenced Web asset is missing or empty: ${reference}`)
  }
}

console.log(`verified Web artifact index and ${localAssets.length} local asset reference(s)`)
