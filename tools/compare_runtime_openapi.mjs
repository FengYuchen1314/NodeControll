#!/usr/bin/env node

import fs from 'node:fs'
import process from 'node:process'

const [runtimeUrl, artifactFile] = process.argv.slice(2)
if (!runtimeUrl || !artifactFile) {
  throw new Error('usage: node tools/compare_runtime_openapi.mjs <runtime-openapi-url> <artifact-openapi-file>')
}

let response
let lastError
for (let attempt = 1; attempt <= 50; attempt += 1) {
  try {
    response = await fetch(runtimeUrl, { signal: AbortSignal.timeout(1_000) })
    if (response.ok) break
    lastError = new Error(`runtime OpenAPI returned HTTP ${response.status}`)
  } catch (error) {
    lastError = error
  }
  await new Promise((resolve) => setTimeout(resolve, 200))
}

if (!response?.ok) throw lastError ?? new Error('runtime OpenAPI did not become ready')
const runtime = await response.json()
const artifact = JSON.parse(fs.readFileSync(artifactFile, 'utf8'))

const runtimeCanonical = JSON.stringify(canonicalize(runtime))
const artifactCanonical = JSON.stringify(canonicalize(artifact))
if (runtimeCanonical !== artifactCanonical) {
  throw new Error('runtime OpenAPI differs from the packaged OpenAPI document')
}

console.log(`runtime OpenAPI exactly matches packaged contract (${Object.keys(runtime.paths ?? {}).length} paths)`)

function canonicalize(value) {
  if (Array.isArray(value)) return value.map(canonicalize)
  if (!value || typeof value !== 'object') return value
  return Object.fromEntries(
    Object.keys(value).sort().map((key) => [key, canonicalize(value[key])]),
  )
}
