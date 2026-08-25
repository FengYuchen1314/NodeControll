#!/usr/bin/env node

import fs from 'node:fs'
import path from 'node:path'
import process from 'node:process'
import { fileURLToPath } from 'node:url'

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url))
const workspace = path.resolve(scriptDirectory, '..')
const nodeModulesRoot = path.join(workspace, 'node_modules')
const virtualStore = path.join(nodeModulesRoot, '.pnpm')
const inventoryArgument = process.argv[2]

if (!inventoryArgument) {
  throw new Error('usage: node tools/verify_installed_npm_license_inventory.mjs <DEPENDENCIES.json>')
}

const inventoryPath = path.resolve(inventoryArgument)
requireRegularFile(inventoryPath, 'dependency inventory')
requireDirectory(nodeModulesRoot, 'workspace node_modules')
requireDirectory(virtualStore, 'pnpm virtual store')

const realWorkspace = fs.realpathSync(workspace)
const realNodeModules = fs.realpathSync(nodeModulesRoot)
const realStore = fs.realpathSync(virtualStore)
if (!isStrictlyInside(realWorkspace, realNodeModules)) {
  throw new Error('workspace node_modules escapes the canonical workspace')
}
if (!isStrictlyInside(realWorkspace, realStore) || !isStrictlyInside(realNodeModules, realStore)) {
  throw new Error('pnpm virtual store escapes the canonical workspace node_modules directory')
}

const inventory = JSON.parse(fs.readFileSync(inventoryPath, 'utf8'))
if (!inventory || typeof inventory !== 'object' || Array.isArray(inventory) || !Array.isArray(inventory.components)) {
  throw new Error('dependency inventory has no component array')
}

const expected = new Set()
for (const component of inventory.components) {
  if (component?.ecosystem !== 'npm') continue
  const identity = exactIdentity(component?.name, component?.version, 'dependency inventory npm component')
  if (expected.has(identity)) throw new Error(`dependency inventory repeats npm identity ${printableIdentity(identity)}`)
  expected.add(identity)
}
if (expected.size === 0) throw new Error('dependency inventory contains no npm components')

const actual = new Set()
for (const virtualEntry of sortedDirectoryEntries(virtualStore)) {
  if (virtualEntry.name === 'node_modules' || !virtualEntry.isDirectory() || virtualEntry.isSymbolicLink()) continue
  const modulesRoot = path.join(virtualStore, virtualEntry.name, 'node_modules')
  if (!isRegularDirectory(modulesRoot)) continue
  for (const entry of sortedDirectoryEntries(modulesRoot)) {
    if (entry.isSymbolicLink() || !entry.isDirectory()) continue
    if (entry.name.startsWith('@')) {
      const scopeRoot = path.join(modulesRoot, entry.name)
      for (const scopedEntry of sortedDirectoryEntries(scopeRoot)) {
        if (scopedEntry.isSymbolicLink() || !scopedEntry.isDirectory()) continue
        collectPackage(path.join(scopeRoot, scopedEntry.name))
      }
    } else {
      collectPackage(path.join(modulesRoot, entry.name))
    }
  }
}

const missing = [...actual].filter((identity) => !expected.has(identity)).sort(compareText)
const extra = [...expected].filter((identity) => !actual.has(identity)).sort(compareText)
if (missing.length !== 0 || extra.length !== 0) {
  throw new Error(
    `installed npm/component inventory closure mismatch: ` +
    `unlisted-installed=${missing.map(printableIdentity).join(', ') || '<none>'}; ` +
    `listed-not-installed=${extra.map(printableIdentity).join(', ') || '<none>'}`,
  )
}

console.log(`verified ${actual.size} npm component identities from the fresh pnpm virtual store`)

function collectPackage(packageRoot) {
  const rootStat = fs.lstatSync(packageRoot)
  if (rootStat.isSymbolicLink() || !rootStat.isDirectory()) {
    throw new Error(`installed package root must be a non-symlink directory: ${relative(packageRoot)}`)
  }
  const realRoot = fs.realpathSync(packageRoot)
  if (!isPathInside(realStore, realRoot)) {
    throw new Error(`installed package root escapes the pnpm virtual store: ${relative(packageRoot)}`)
  }
  const manifestPath = path.join(packageRoot, 'package.json')
  requireRegularFile(manifestPath, `installed package manifest ${relative(manifestPath)}`)
  const manifest = JSON.parse(fs.readFileSync(manifestPath, 'utf8'))
  const identity = exactIdentity(manifest?.name, manifest?.version, `installed package ${relative(packageRoot)}`)
  actual.add(identity)
}

function exactIdentity(name, version, label) {
  if (typeof name !== 'string' || name === '' || name.includes('\0')) {
    throw new Error(`${label} has no exact package name`)
  }
  if (typeof version !== 'string' || version === '' || version.includes('\0')) {
    throw new Error(`${label} has no exact package version`)
  }
  return `${name}\0${version}`
}

function printableIdentity(identity) {
  return identity.replace('\0', '@')
}

function requireRegularFile(target, label) {
  const metadata = fs.lstatSync(target)
  if (metadata.isSymbolicLink() || !metadata.isFile()) throw new Error(`${label} must be a regular non-symlink file`)
}

function requireDirectory(target, label) {
  const metadata = fs.lstatSync(target)
  if (metadata.isSymbolicLink() || !metadata.isDirectory()) throw new Error(`${label} must be a non-symlink directory`)
}

function isRegularDirectory(target) {
  try {
    const metadata = fs.lstatSync(target)
    return metadata.isDirectory() && !metadata.isSymbolicLink()
  } catch (error) {
    if (error?.code === 'ENOENT' || error?.code === 'ENOTDIR') return false
    throw error
  }
}

function sortedDirectoryEntries(directory) {
  return fs.readdirSync(directory, { withFileTypes: true }).sort((left, right) => compareText(left.name, right.name))
}

function isPathInside(root, target) {
  const candidate = path.relative(path.resolve(root), path.resolve(target))
  return candidate === '' || (!candidate.startsWith(`..${path.sep}`) && candidate !== '..' && !path.isAbsolute(candidate))
}

function isStrictlyInside(root, target) {
  return path.resolve(root) !== path.resolve(target) && isPathInside(root, target)
}

function relative(target) {
  return path.relative(workspace, target).split(path.sep).join('/')
}

function compareText(left, right) {
  return left < right ? -1 : left > right ? 1 : 0
}
