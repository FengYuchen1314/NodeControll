#!/usr/bin/env node

import { execFileSync } from 'node:child_process'
import { createHash } from 'node:crypto'
import fs from 'node:fs'
import path from 'node:path'
import process from 'node:process'
import { fileURLToPath } from 'node:url'

const scriptDirectory = path.dirname(fileURLToPath(import.meta.url))
const workspace = path.resolve(scriptDirectory, '..')
const outputArgument = process.argv[2]
const overrideRoot = path.join(workspace, 'third_party', 'dependency-license-overrides')
const overrideCatalogPath = path.join(overrideRoot, 'overrides.json')
const overrideCatalogRepositoryPath = slash(path.relative(workspace, overrideCatalogPath))
const requiredRustRelease = '1.98.0'

if (!outputArgument) {
  throw new Error('usage: node tools/collect_third_party_licenses.mjs <empty-output-directory>')
}

const output = path.resolve(outputArgument)
if (output === workspace || output === path.parse(output).root) {
  throw new Error(`refusing unsafe output directory: ${output}`)
}
if (fs.existsSync(output) && fs.readdirSync(output).length !== 0) {
  throw new Error(`output directory must be empty: ${output}`)
}
fs.mkdirSync(output, { recursive: true })

const notices = []
const issues = []
const applicationVersion = readCargoWorkspaceVersion()
const overrideState = loadAndValidateOverrideCatalog()

collectCargoPackages()
collectPnpmPackages()
collectRustToolchainRuntime()
validateOverrideClosure()

notices.sort((left, right) =>
  compareText(left.ecosystem, right.ecosystem) ||
  compareText(left.name, right.name) ||
  compareText(left.version, right.version) ||
  compareText(left.source || '', right.source || '') ||
  compareText(left.locked_integrity || '', right.locked_integrity || ''),
)

const sourceRevision = process.env.SOURCE_REVISION?.trim() || 'unknown'
const sourceRepository = normalizedRepositoryName(process.env.SOURCE_REPOSITORY || 'FengYuchen1314/NodeControll')
const inventory = {
  schema_version: 2,
  source_revision: sourceRevision,
  source_repository: sourceRepository,
  generated_from: [
    'Cargo.lock',
    'pnpm-lock.yaml',
    overrideCatalogRepositoryPath,
    'rustc --print sysroot:share/doc/rust',
  ],
  override_catalog: {
    path: overrideCatalogRepositoryPath,
    sha256: overrideState.catalogSha256,
    schema_version: overrideState.document.schemaVersion ?? null,
    source_audit_date: overrideState.document.sourceAuditDate ?? null,
    declared_entries: overrideState.entries.length,
    used_entries: overrideState.usedEntryIndexes.size,
  },
  components: notices,
  issues: [...issues].sort(compareText),
  warnings: [],
}

writeJson(path.join(output, 'DEPENDENCIES.json'), inventory)
writeJson(path.join(output, 'bom.cdx.json'), cyclonedxDocument(notices, sourceRevision, sourceRepository, applicationVersion))
writeChecksums()
writeReadme()

if (issues.length !== 0) {
  throw new Error(`third-party license collection is incomplete:\n- ${[...issues].sort(compareText).join('\n- ')}`)
}

console.log(`collected ${notices.length} component records and ${licenseFileCount()} license-evidence files`)

function collectCargoPackages() {
  const lockPackages = readCargoLockPackages()
  const raw = execFileSync(
    'cargo',
    ['metadata', '--locked', '--format-version', '1'],
    { cwd: workspace, encoding: 'utf8', maxBuffer: 128 * 1024 * 1024 },
  )
  const metadata = JSON.parse(raw)
  const workspaceMembers = new Set(metadata.workspace_members)

  for (const pkg of metadata.packages) {
    if (workspaceMembers.has(pkg.id)) continue
    const packageRoot = path.dirname(pkg.manifest_path)
    const lockedIntegrity = lockPackages.get(cargoLockIdentity(pkg.name, pkg.version, pkg.source)) ?? null
    const record = makeRecord({
      ecosystem: 'cargo',
      name: pkg.name,
      version: pkg.version,
      declaredLicense: pkg.license,
      repository: normalizeRepository(pkg.repository, `cargo:${pkg.name}@${pkg.version}`),
      source: pkg.source,
      purl: cargoPurl(pkg.name, pkg.version, pkg.source),
      lockedIntegrity,
      lockedIntegrityKind: lockedIntegrity ? 'cargo-registry-sha256' : null,
    })
    record.license_files = copyLicenseFiles({
      ecosystem: record.ecosystem,
      packageRoot,
      packageSlug: cargoPackageSlug(pkg.name, pkg.version, pkg.source, lockedIntegrity),
      explicitLicenseFile: pkg.license_file,
    })
    attachRequiredOverride(record)
    validateRecord(record)
    notices.push(record)
  }
}

function collectPnpmPackages() {
  const store = path.join(workspace, 'node_modules', '.pnpm')
  if (!isDirectory(store)) {
    throw new Error(`pnpm virtual store is missing; run pnpm install first: ${store}`)
  }
  const lockIntegrities = readPnpmLockIntegrities()
  const activePackages = readActivePnpmPackages()

  for (const [key, active] of [...activePackages.entries()].sort(([left], [right]) => compareText(left, right))) {
    const lockedIntegrity = lockIntegrities.get(key) ?? null
    if (!lockedIntegrity) {
      issues.push(`npm:${key} is reachable in the installed pnpm graph but has no registry integrity in pnpm-lock.yaml`)
    }

    const manifests = active.paths
      .map((packageRoot) => readActivePnpmManifest(packageRoot, active.name, active.version))
      .filter(Boolean)
    if (manifests.length === 0) {
      issues.push(`npm:${key} has no readable package manifest at any active pnpm graph path`)
      continue
    }
    const canonical = manifests.sort((left, right) => compareWorkspacePaths(left.packageRoot, right.packageRoot))[0]
    const declaredLicense = normalizeLicense(canonical.pkg.license)
    const repository = normalizeRepository(canonical.pkg.repository, `npm:${key}`)
    for (const candidate of manifests.slice(1)) {
      const candidateRepository = normalizeRepository(
        candidate.pkg.repository,
        `npm:${key} at ${slash(path.relative(workspace, candidate.packageRoot))}`,
      )
      if (normalizeLicense(candidate.pkg.license) !== declaredLicense || candidateRepository !== repository) {
        issues.push(`npm:${key} has inconsistent license/repository metadata across active peer instances`)
      }
    }

    const record = makeRecord({
      ecosystem: 'npm',
      name: active.name,
      version: active.version,
      declaredLicense,
      repository,
      source: 'https://registry.npmjs.org/',
      purl: `pkg:npm/${active.name.split('/').map(purlEncode).join('/')}@${purlEncode(active.version)}`,
      lockedIntegrity,
      lockedIntegrityKind: lockedIntegrity ? 'pnpm-registry-integrity' : null,
    })
    record.license_files = copyLicenseFiles({
      ecosystem: record.ecosystem,
      packageRoot: canonical.packageRoot,
      packageSlug: `${active.name}-${active.version}`,
      explicitLicenseFile: licenseFileFromExpression(declaredLicense),
    })
    attachRequiredOverride(record)
    validateRecord(record)
    notices.push(record)
  }
}

function collectRustToolchainRuntime() {
  let sysroot = null
  let rustcDetails = {}
  try {
    sysroot = execFileSync('rustc', ['--print', 'sysroot'], {
      cwd: workspace,
      encoding: 'utf8',
    }).trim()
    const verboseVersion = execFileSync('rustc', ['-vV'], {
      cwd: workspace,
      encoding: 'utf8',
    })
    rustcDetails = parseRustcVerboseVersion(verboseVersion)
  } catch (error) {
    issues.push(`rust-toolchain:rust-std-runtime@${requiredRustRelease} could not query rustc sysroot/version: ${error.message}`)
  }

  const release = rustcDetails.release || requiredRustRelease
  const record = makeRecord({
    ecosystem: 'rust-toolchain',
    name: 'rust-std-runtime',
    version: release,
    declaredLicense: 'MIT OR Apache-2.0',
    repository: 'https://github.com/rust-lang/rust',
    source: 'rustc-sysroot:share/doc/rust',
    purl: `pkg:generic/rust-std-runtime@${purlEncode(release)}`,
    lockedIntegrity: rustcDetails['commit-hash'] || null,
    lockedIntegrityKind: rustcDetails['commit-hash'] ? 'rustc-commit' : null,
  })
  record.toolchain_provenance = {
    sysroot_query: 'rustc --print sysroot',
    evidence_root: 'share/doc/rust',
    rustc_release: release,
    rustc_commit_hash: rustcDetails['commit-hash'] || null,
    rustc_commit_date: rustcDetails['commit-date'] || null,
    rustc_host: rustcDetails.host || null,
    llvm_version: rustcDetails['LLVM version'] || null,
  }

  if (release !== requiredRustRelease) {
    issues.push(`rust-toolchain:rust-std-runtime must be collected with rustc ${requiredRustRelease}, found ${release}`)
  }

  if (sysroot) {
    const rustDocRoot = path.join(sysroot, 'share', 'doc', 'rust')
    const licenseDirectory = path.join(rustDocRoot, 'licenses')
    const sources = [
      { source: path.join(rustDocRoot, 'README.md'), kind: 'rust-toolchain-license-readme' },
      { source: path.join(rustDocRoot, 'COPYRIGHT-library.html'), kind: 'rust-standard-library-copyright' },
    ]

    const licenseFiles = strictRegularFiles(licenseDirectory, 'rust-toolchain:share/doc/rust/licenses')
    if (licenseFiles.length === 0) {
      issues.push('rust-toolchain:share/doc/rust/licenses contains no regular license files')
    }
    for (const source of licenseFiles) {
      sources.push({ source, kind: 'rust-toolchain-license-text' })
    }

    for (const evidence of sources.sort((left, right) => compareText(left.source, right.source))) {
      const sourceRelative = slash(path.relative(rustDocRoot, evidence.source))
      const stat = regularNonSymlinkStat(evidence.source, `rust-toolchain:share/doc/rust/${sourceRelative}`)
      if (!stat) continue
      const destination = path.join(
        output,
        'licenses',
        'rust-toolchain',
        `rust-std-runtime-${safePathSegment(release)}`,
        ...sourceRelative.split('/'),
      )
      fs.mkdirSync(path.dirname(destination), { recursive: true })
      fs.copyFileSync(evidence.source, destination)
      const sourceHash = sha256File(evidence.source)
      const destinationHash = sha256File(destination)
      if (sourceHash !== destinationHash) {
        issues.push(`rust-toolchain: copied evidence hash changed for share/doc/rust/${sourceRelative}`)
        continue
      }
      record.license_files.push({
        kind: evidence.kind,
        path: slash(path.relative(output, destination)),
        sha256: destinationHash,
        bytes: stat.size,
        source: {
          kind: 'installed-rustc-sysroot',
          sysroot_relative_path: `share/doc/rust/${sourceRelative}`,
          rustc_release: release,
          rustc_commit_hash: rustcDetails['commit-hash'] || null,
        },
      })
    }
  }

  record.license_files.sort((left, right) => compareText(left.path, right.path))
  validateRecord(record)
  notices.push(record)
}

function readActivePnpmPackages() {
  let roots
  try {
    const raw = execFileSync('pnpm', ['list', '--recursive', '--json', '--depth', 'Infinity'], {
      cwd: workspace,
      encoding: 'utf8',
      maxBuffer: 128 * 1024 * 1024,
    })
    roots = JSON.parse(raw)
  } catch (error) {
    throw new Error(`cannot read the active pnpm dependency graph: ${error.message}`)
  }
  if (!Array.isArray(roots) || roots.length === 0) {
    throw new Error('pnpm list --recursive --json --depth Infinity returned no workspace roots')
  }
  const workspaceGraph = readPnpmWorkspaceGraph(roots)

  const active = new Map()
  const reachability = new Map()
  const queue = []
  let queueIndex = 0
  roots.forEach((root, index) => {
    enqueuePnpmDependencies(root, queue, false, workspaceGraph.rootManifests.get(index) ?? null)
  })

  while (queueIndex < queue.length) {
    const { requestedName, node, optionalPath } = queue[queueIndex]
    queueIndex += 1
    if (!node || typeof node !== 'object' || Array.isArray(node)) {
      issues.push(`pnpm reachable dependency ${requestedName} is not an object`)
      continue
    }

    const name = typeof node.name === 'string' && node.name !== '' ? node.name : requestedName
    const version = typeof node.version === 'string' && node.version !== '' ? node.version : null
    const reportedPath = typeof node.path === 'string' && node.path !== '' ? node.path : null

    if (!name || !version) {
      issues.push(`pnpm reachable dependency ${requestedName} has no exact name/version identity`)
      enqueuePnpmDependencies(node, queue, optionalPath, null)
      continue
    }

    const state = trackPnpmReachability(reachability, { name, version, reportedPath, optionalPath })
    if (!reportedPath) {
      enqueuePnpmDependencies(node, queue, optionalPath, null)
      continue
    }

    let packageRoot
    try {
      packageRoot = fs.realpathSync(path.resolve(workspace, reportedPath))
    } catch (error) {
      // A successful pnpm list can include predicted paths for optional packages
      // excluded by the current OS/CPU/libc. Exclusion is safe only after every
      // path reaching this exact reported package instance is known to be optional.
      if (error?.code === 'ENOENT') state.missingPath = true
      else state.pathErrors.add(error.message)
      enqueuePnpmDependencies(node, queue, optionalPath, null)
      continue
    }
    if (!isPathInside(workspaceGraph.workspaceRealPath, packageRoot)) {
      state.outsideWorkspace = true
      enqueuePnpmDependencies(node, queue, optionalPath, null)
      continue
    }

    const reachableManifest = readReachablePnpmManifest(packageRoot, name, version)
    enqueuePnpmDependencies(node, queue, optionalPath, reachableManifest?.pkg ?? null)
    if (!reachableManifest) continue
    if (workspaceGraph.memberRealPaths.has(packageRoot)) continue
    const identity = `${reachableManifest.name}@${reachableManifest.version}`

    let component = active.get(identity)
    if (!component) {
      component = { name: reachableManifest.name, version: reachableManifest.version, paths: new Set() }
      active.set(identity, component)
    }
    component.paths.add(packageRoot)
  }

  for (const state of [...reachability.values()].sort((left, right) => compareText(left.key, right.key))) {
    const identity = `npm:${state.name}@${state.version}`
    if (!state.reportedPath && !state.allOptional) {
      issues.push(`${identity} is reachable on a required pnpm graph path but has no installed path`)
    }
    if (state.missingPath && !state.allOptional) {
      issues.push(`${identity} required active pnpm path does not exist: ${state.reportedPath}`)
    }
    for (const message of [...state.pathErrors].sort(compareText)) {
      issues.push(`${identity} active pnpm path is missing or unreadable: ${message}`)
    }
    if (state.outsideWorkspace) {
      issues.push(`${identity} active pnpm path resolves outside the workspace: ${state.reportedPath}`)
    }
  }

  for (const component of active.values()) {
    component.paths = [...component.paths].sort(compareWorkspacePaths)
  }
  return active
}

function readPnpmWorkspaceGraph(roots) {
  const workspaceRealPath = fs.realpathSync(workspace)
  const expectedMembers = new Map()
  for (const [name, expectedPath] of [
    ['nodecontroll-workspace', workspaceRealPath],
    ['@nodecontroll/web', path.join(workspaceRealPath, 'apps', 'web')],
  ]) {
    let memberRealPath
    try {
      memberRealPath = fs.realpathSync(expectedPath)
    } catch (error) {
      throw new Error(`required pnpm workspace member ${name} path is missing or unreadable: ${error.message}`)
    }
    if (!isPathInside(workspaceRealPath, memberRealPath)) {
      throw new Error(`required pnpm workspace member ${name} resolves outside the workspace`)
    }
    const pkg = readPnpmWorkspaceManifest(memberRealPath, `required pnpm workspace member ${name}`)
    if (!pkg) throw new Error(`cannot establish required pnpm workspace member ${name}`)
    if (pkg.name !== name) {
      issues.push(`required pnpm workspace member ${name} package.json name is ${pkg.name}`)
    }
    expectedMembers.set(name, { name, version: pkg.version, packageRoot: memberRealPath })
  }

  const memberRealPaths = new Set()
  const rootManifests = new Map()
  const rootCounts = new Map([...expectedMembers.keys()].map((name) => [name, 0]))
  const validatedRoots = new Set()
  if (roots.length !== expectedMembers.size) {
    issues.push(`pnpm reachable graph must contain exactly ${expectedMembers.size} workspace roots, found ${roots.length}`)
  }

  roots.forEach((root, index) => {
    const label = `pnpm workspace root[${index}]`
    if (!root || typeof root !== 'object' || Array.isArray(root)) {
      issues.push(`${label} is not an object`)
      return
    }

    const reportedName = typeof root.name === 'string' && root.name !== '' ? root.name : null
    const reportedVersion = typeof root.version === 'string' && root.version !== '' ? root.version : null
    const reportedPath = typeof root.path === 'string' && root.path !== '' ? root.path : null
    const expected = expectedMembers.get(reportedName)
    if (expected) {
      rootCounts.set(reportedName, rootCounts.get(reportedName) + 1)
    } else if (reportedName) {
      issues.push(`${label} has unexpected workspace identity ${reportedName}@${reportedVersion || '<missing>'}`)
    }
    if (!reportedName || !reportedVersion) {
      issues.push(`${label} has no exact name/version identity`)
    }
    if (!reportedPath) {
      issues.push(`${label} has no filesystem path`)
      return
    }

    let packageRoot
    try {
      packageRoot = fs.realpathSync(path.resolve(workspace, reportedPath))
    } catch (error) {
      issues.push(`${label} path is missing or unreadable: ${error.message}`)
      return
    }
    if (!isPathInside(workspaceRealPath, packageRoot)) {
      issues.push(`${label} path resolves outside the workspace: ${reportedPath}`)
      return
    }
    if (!expected) return
    if (packageRoot !== expected.packageRoot) {
      issues.push(
        `${label} ${reportedName} realpath must be ${slash(path.relative(workspaceRealPath, expected.packageRoot)) || '.'}, ` +
        `found ${slash(path.relative(workspaceRealPath, packageRoot)) || '.'}`,
      )
      return
    }

    const pkg = readPnpmWorkspaceManifest(packageRoot, label)
    if (!pkg) return
    rootManifests.set(index, pkg)
    if (
      reportedName !== expected.name ||
      reportedVersion !== expected.version ||
      pkg.name !== expected.name ||
      pkg.version !== expected.version
    ) {
      issues.push(
        `${label} reported identity ${reportedName || '<missing>'}@${reportedVersion || '<missing>'} ` +
        `and package.json ${pkg.name}@${pkg.version} must both match ${expected.name}@${expected.version}`,
      )
      return
    }
    if (validatedRoots.has(expected.name)) return
    validatedRoots.add(expected.name)
    memberRealPaths.add(packageRoot)
  })

  for (const [requiredRoot, count] of rootCounts) {
    if (count !== 1) {
      issues.push(`pnpm reachable graph must contain exactly one workspace root named ${requiredRoot}, found ${count}`)
    } else if (!validatedRoots.has(requiredRoot)) {
      issues.push(`pnpm workspace root ${requiredRoot} did not match its required realpath and manifest identity`)
    }
  }
  return { workspaceRealPath, memberRealPaths, rootManifests }
}

function readPnpmWorkspaceManifest(packageRoot, label) {
  const manifestPath = path.join(packageRoot, 'package.json')
  if (!regularNonSymlinkStat(manifestPath, `${label} package.json`)) return null
  let pkg
  try {
    pkg = JSON.parse(fs.readFileSync(manifestPath, 'utf8'))
  } catch (error) {
    issues.push(`${label} package.json is invalid JSON: ${error.message}`)
    return null
  }
  if (typeof pkg.name !== 'string' || pkg.name === '' || typeof pkg.version !== 'string' || pkg.version === '') {
    issues.push(`${label} package.json has no exact name/version identity`)
    return null
  }
  return pkg
}

function trackPnpmReachability(states, { name, version, reportedPath, optionalPath }) {
  const key = `${name}\0${version}\0${reportedPath ?? ''}`
  let state = states.get(key)
  if (!state) {
    state = {
      key,
      name,
      version,
      reportedPath,
      allOptional: true,
      missingPath: false,
      outsideWorkspace: false,
      pathErrors: new Set(),
    }
    states.set(key, state)
  }
  state.allOptional = state.allOptional && optionalPath
  return state
}

function enqueuePnpmDependencies(node, queue, parentOptionalPath, parentManifest) {
  for (const groupName of ['dependencies', 'devDependencies', 'optionalDependencies']) {
    const group = node?.[groupName]
    if (group === undefined) continue
    if (!group || typeof group !== 'object' || Array.isArray(group)) {
      issues.push(`pnpm list returned a non-object ${groupName} dependency group`)
      continue
    }
    for (const requestedName of Object.keys(group).sort(compareText)) {
      const declaredOptional = isManifestOptionalDependency(parentManifest, requestedName)
      queue.push({
        requestedName,
        node: group[requestedName],
        optionalPath: parentOptionalPath || groupName === 'optionalDependencies' || declaredOptional,
      })
    }
  }
}

function isManifestOptionalDependency(pkg, requestedName) {
  const optionalDependencies = pkg?.optionalDependencies
  return Boolean(
    optionalDependencies &&
    typeof optionalDependencies === 'object' &&
    !Array.isArray(optionalDependencies) &&
    Object.hasOwn(optionalDependencies, requestedName)
  )
}

function readReachablePnpmManifest(packageRoot, reportedName, reportedVersion) {
  const reportedIdentity = `${reportedName}@${reportedVersion}`
  const manifestPath = path.join(packageRoot, 'package.json')
  if (!regularNonSymlinkStat(manifestPath, `npm:${reportedIdentity} reachable package.json`)) return null
  let pkg
  try {
    pkg = JSON.parse(fs.readFileSync(manifestPath, 'utf8'))
  } catch (error) {
    issues.push(`npm:${reportedIdentity} reachable package.json is invalid JSON: ${error.message}`)
    return null
  }
  if (typeof pkg.name !== 'string' || pkg.name === '' || typeof pkg.version !== 'string' || pkg.version === '') {
    issues.push(`npm:${reportedIdentity} reachable package.json has no exact name/version identity`)
    return null
  }
  if (pkg.version !== reportedVersion) {
    issues.push(`npm:${reportedIdentity} reachable package.json version is ${pkg.version}`)
    return null
  }
  return { name: pkg.name, version: pkg.version, pkg }
}

function readActivePnpmManifest(packageRoot, expectedName, expectedVersion) {
  const identity = `${expectedName}@${expectedVersion}`
  const manifestPath = path.join(packageRoot, 'package.json')
  if (!regularNonSymlinkStat(manifestPath, `npm:${identity} active package.json`)) return null
  let pkg
  try {
    pkg = JSON.parse(fs.readFileSync(manifestPath, 'utf8'))
  } catch (error) {
    issues.push(`npm:${identity} active package.json is invalid JSON: ${error.message}`)
    return null
  }
  if (pkg.name !== expectedName || pkg.version !== expectedVersion) {
    issues.push(`npm:${identity} active package.json identity is ${pkg.name || '<missing>'}@${pkg.version || '<missing>'}`)
    return null
  }
  return { packageRoot, pkg }
}

function compareWorkspacePaths(left, right) {
  return compareText(slash(path.relative(workspace, left)), slash(path.relative(workspace, right)))
}

function makeRecord({ ecosystem, name, version, declaredLicense, repository, source, purl, lockedIntegrity, lockedIntegrityKind }) {
  return {
    ecosystem,
    name,
    version,
    declared_license: declaredLicense || null,
    repository: repository || null,
    source: source || null,
    purl,
    locked_integrity: lockedIntegrity || null,
    locked_integrity_kind: lockedIntegrityKind || null,
    license_files: [],
  }
}

function copyLicenseFiles({ ecosystem, packageRoot, packageSlug, explicitLicenseFile }) {
  const candidates = new Set()
  if (explicitLicenseFile) candidates.add(path.resolve(packageRoot, explicitLicenseFile))

  for (const entry of sortedDirectoryEntries(packageRoot)) {
    if (/^(?:licen[cs]e|copying|notice|unlicense|copyright)(?:[._-].*)?$/i.test(entry.name)) {
      candidates.add(path.join(packageRoot, entry.name))
    }
  }

  const root = fs.realpathSync(packageRoot)
  const destinationRoot = path.join(output, 'licenses', ecosystem, safePathSegment(packageSlug))
  const copied = []
  for (const candidate of [...candidates].sort(compareText)) {
    if (!fs.existsSync(candidate)) continue
    const real = fs.realpathSync(candidate)
    if (!isPathInside(root, real)) continue
    const stat = fs.statSync(real)
    if (!stat.isFile()) continue
    const payload = fs.readFileSync(real)
    if (payload.length === 0 || payload.toString('utf8').trim() === '') {
      issues.push(`${ecosystem}:${packageSlug} license candidate is empty or whitespace-only: ${path.basename(candidate)}`)
      continue
    }
    if (isRelativeLicensePointer(real, stat.size)) continue
    if (stat.size > 2 * 1024 * 1024) {
      issues.push(`${ecosystem}:${packageSlug} license file exceeds 2 MiB: ${path.basename(candidate)}`)
      continue
    }
    fs.mkdirSync(destinationRoot, { recursive: true })
    const destination = uniqueDestination(destinationRoot, path.basename(candidate))
    fs.copyFileSync(real, destination)
    copied.push({
      kind: 'package-license-or-notice',
      path: slash(path.relative(output, destination)),
      sha256: sha256File(destination),
      bytes: stat.size,
      source: {
        kind: 'package-archive',
        package_relative_path: slash(path.relative(root, real)),
      },
    })
  }
  return copied.sort((left, right) => compareText(left.path, right.path))
}

function attachRequiredOverride(record) {
  if (record.license_files.length !== 0) return

  const identity = recordIdentity(record)
  const wrapper = overrideState.byIdentity.get(identity)
  if (!wrapper) {
    issues.push(`${identity} package archive has no actual license/notice file and no exact vetted override`)
    return
  }

  overrideState.usedEntryIndexes.add(wrapper.index)
  const entry = wrapper.entry
  if (entry.ecosystem !== record.ecosystem || entry.name !== record.name || entry.version !== record.version) {
    issues.push(`${identity} override identity does not exactly match the installed component`)
  }
  if (entry.declaredLicense !== record.declared_license) {
    issues.push(`${identity} override declared license ${JSON.stringify(entry.declaredLicense)} does not exactly match package metadata ${JSON.stringify(record.declared_license)}`)
  }

  const expectedIntegrityField = record.ecosystem === 'cargo' ? 'registryChecksum' : 'registryIntegrity'
  const expectedIntegrity = entry[expectedIntegrityField]
  const expectedSource = record.ecosystem === 'cargo'
    ? 'registry+https://github.com/rust-lang/crates.io-index'
    : 'https://registry.npmjs.org/'
  if (record.source !== expectedSource) {
    issues.push(`${identity} override is restricted to canonical registry source ${expectedSource}, found ${record.source}`)
  }
  if (!record.locked_integrity) {
    issues.push(`${identity} has no locked registry integrity available for override verification`)
  } else if (expectedIntegrity !== record.locked_integrity) {
    issues.push(`${identity} override ${expectedIntegrityField} does not match the lockfile integrity`)
  }

  const packageSlug = record.ecosystem === 'cargo'
    ? cargoPackageSlug(record.name, record.version, record.source, record.locked_integrity)
    : `${record.name}-${record.version}`
  const destinationRoot = path.join(output, 'licenses', record.ecosystem, safePathSegment(packageSlug), 'vetted-source')
  for (const [index, evidence] of wrapper.evidence.entries()) {
    if (!evidence.absolutePath) continue
    const destination = path.join(destinationRoot, `${String(index + 1).padStart(2, '0')}-${safePathSegment(path.basename(evidence.spec.localPath))}`)
    fs.mkdirSync(path.dirname(destination), { recursive: true })
    fs.copyFileSync(evidence.absolutePath, destination)
    const copiedHash = sha256File(destination)
    if (copiedHash !== evidence.spec.sha256 || fs.statSync(destination).size !== evidence.spec.bytes) {
      issues.push(`${identity} copied override evidence changed: ${evidence.spec.localPath}`)
      continue
    }
    record.license_files.push({
      kind: 'vetted-source-license-or-notice',
      evidence_kind: evidence.spec.kind,
      path: slash(path.relative(output, destination)),
      sha256: copiedHash,
      bytes: evidence.spec.bytes,
      source: overrideEvidenceProvenance(evidence.spec),
    })
  }
  record.license_files.sort((left, right) => compareText(left.path, right.path))
  record.license_override = {
    catalog_path: overrideCatalogRepositoryPath,
    catalog_sha256: overrideState.catalogSha256,
    catalog_schema_version: overrideState.document.schemaVersion,
    catalog_entry_index: wrapper.index,
    source_audit_date: overrideState.document.sourceAuditDate || null,
    resolution: entry.resolution,
    repository: entry.repository,
    revision: entry.revision,
    version_tag: entry.versionTag || null,
    registry_git_head: entry.registryGitHead || null,
    locked_integrity_field: expectedIntegrityField,
    locked_integrity: expectedIntegrity || null,
    version_evidence: normalizeVersionEvidence(entry.versionEvidence),
    upstream_paths: wrapper.evidence.map(({ spec }) => `${spec.upstreamRepository}@${spec.upstreamRevision}:${spec.upstreamPath}`).sort(compareText),
  }
}

function validateRecord(record) {
  const identity = recordIdentity(record)
  if (!record.declared_license) issues.push(`${identity} has no declared license metadata`)
  if (record.license_files.length === 0) issues.push(`${identity} has no actual license/notice evidence`)
  if (record.repository !== null && !isCanonicalRepositoryUrl(record.repository)) {
    issues.push(`${identity} repository is not a canonical absolute http(s)/ssh URL: ${JSON.stringify(record.repository)}`)
  }
}

function loadAndValidateOverrideCatalog() {
  if (!regularDirectoryNonSymlinkStat(overrideRoot, slash(path.relative(workspace, overrideRoot)))) {
    throw new Error(`required dependency license override directory is unavailable: ${overrideRoot}`)
  }
  const catalogStat = regularNonSymlinkStat(overrideCatalogPath, overrideCatalogRepositoryPath)
  if (!catalogStat) throw new Error(`required dependency license override catalog is unavailable: ${overrideCatalogPath}`)

  let document
  try {
    document = JSON.parse(fs.readFileSync(overrideCatalogPath, 'utf8'))
  } catch (error) {
    throw new Error(`cannot parse ${overrideCatalogRepositoryPath}: ${error.message}`)
  }
  const state = {
    document,
    catalogSha256: sha256File(overrideCatalogPath),
    entries: [],
    byIdentity: new Map(),
    usedEntryIndexes: new Set(),
    referencedFiles: new Set(),
    fileOwners: new Map(),
  }

  if (document.schemaVersion !== 1) issues.push(`${overrideCatalogRepositoryPath} must have schemaVersion 1`)
  if (typeof document.sourceAuditDate !== 'string' || !/^\d{4}-\d{2}-\d{2}$/.test(document.sourceAuditDate)) {
    issues.push(`${overrideCatalogRepositoryPath} has an invalid sourceAuditDate`)
  }
  if (!Array.isArray(document.entries)) {
    issues.push(`${overrideCatalogRepositoryPath} entries must be an array`)
    return state
  }

  document.entries.forEach((entry, offset) => {
    const index = offset + 1
    const identity = overrideIdentity(entry, index)
    const wrapper = { index, identity, entry, evidence: [] }
    state.entries.push(wrapper)
    if (state.byIdentity.has(identity)) issues.push(`${overrideCatalogRepositoryPath} has duplicate override identity ${identity}`)
    else state.byIdentity.set(identity, wrapper)
    validateOverrideEntry(wrapper, state)
  })

  validateOverrideFilesystem(state)
  return state
}

function validateOverrideEntry(wrapper, state) {
  const { entry, identity, index } = wrapper
  if (!entry || typeof entry !== 'object' || Array.isArray(entry)) {
    issues.push(`${overrideCatalogRepositoryPath} entry ${index} must be an object`)
    return
  }
  if (entry.ecosystem !== 'cargo' && entry.ecosystem !== 'npm') issues.push(`${identity} override ecosystem must be cargo or npm`)
  for (const field of ['name', 'version', 'declaredLicense', 'repository', 'revision', 'resolution']) {
    if (typeof entry[field] !== 'string' || entry[field].trim() === '') issues.push(`${identity} override ${field} must be a non-empty string`)
  }
  if (typeof entry.revision === 'string' && !/^[0-9a-f]{40}$/.test(entry.revision)) {
    issues.push(`${identity} override revision must be a lowercase 40-hex commit`)
  }
  if (entry.ecosystem === 'cargo' && !/^[0-9a-f]{64}$/.test(entry.registryChecksum || '')) {
    issues.push(`${identity} override registryChecksum must be a lowercase SHA-256`)
  }
  if (entry.ecosystem === 'npm' && !/^sha(?:256|384|512)-[A-Za-z0-9+/]+={0,2}$/.test(entry.registryIntegrity || '')) {
    issues.push(`${identity} override registryIntegrity must be a valid SRI value`)
  }
  if (!entry.versionEvidence || typeof entry.versionEvidence !== 'object' || Array.isArray(entry.versionEvidence)) {
    issues.push(`${identity} override versionEvidence must be an object`)
  } else {
    for (const field of ['upstreamPath', 'sha256', 'expectedName', 'expectedVersion']) {
      if (typeof entry.versionEvidence[field] !== 'string' || entry.versionEvidence[field].trim() === '') {
        issues.push(`${identity} override versionEvidence.${field} must be a non-empty string`)
      }
    }
    if (!/^[0-9a-f]{64}$/.test(entry.versionEvidence.sha256 || '')) {
      issues.push(`${identity} override versionEvidence.sha256 must be a lowercase SHA-256`)
    }
    if (entry.versionEvidence.expectedVersion !== entry.version) {
      issues.push(`${identity} override versionEvidence version does not match its entry`)
    }
  }

  if (!Array.isArray(entry.files) || entry.files.length === 0) {
    issues.push(`${identity} override files must be a non-empty array`)
    return
  }

  const entryLocalPaths = new Set()
  for (const [fileOffset, spec] of entry.files.entries()) {
    const label = `${identity} override files[${fileOffset}]`
    if (!spec || typeof spec !== 'object' || Array.isArray(spec)) {
      issues.push(`${label} must be an object`)
      continue
    }
    for (const field of ['kind', 'upstreamRepository', 'upstreamRevision', 'upstreamPath', 'localPath', 'sha256']) {
      if (typeof spec[field] !== 'string' || spec[field].trim() === '') issues.push(`${label}.${field} must be a non-empty string`)
    }
    if (!/^[0-9a-f]{40}$/.test(spec.upstreamRevision || '')) issues.push(`${label}.upstreamRevision must be a lowercase 40-hex commit`)
    if (!/^[0-9a-f]{64}$/.test(spec.sha256 || '')) issues.push(`${label}.sha256 must be a lowercase SHA-256`)
    if (!Number.isSafeInteger(spec.bytes) || spec.bytes < 1) issues.push(`${label}.bytes must be a positive safe integer`)
    if (spec.upstreamTag !== undefined && (typeof spec.upstreamTag !== 'string' || spec.upstreamTag.trim() === '')) {
      issues.push(`${label}.upstreamTag must be a non-empty string when present`)
    }
    const hasLineStart = spec.upstreamLineStart !== undefined
    const hasLineEnd = spec.upstreamLineEnd !== undefined
    if (hasLineStart !== hasLineEnd) issues.push(`${label} must provide upstreamLineStart and upstreamLineEnd together`)
    if (hasLineStart && (!Number.isSafeInteger(spec.upstreamLineStart) || spec.upstreamLineStart < 1)) {
      issues.push(`${label}.upstreamLineStart must be a positive safe integer`)
    }
    if (hasLineEnd && (!Number.isSafeInteger(spec.upstreamLineEnd) || spec.upstreamLineEnd < spec.upstreamLineStart)) {
      issues.push(`${label}.upstreamLineEnd must be a safe integer no smaller than upstreamLineStart`)
    }
    if (spec.extraction !== undefined && (typeof spec.extraction !== 'string' || spec.extraction.trim() === '')) {
      issues.push(`${label}.extraction must be a non-empty string when present`)
    }
    if ((hasLineStart || hasLineEnd) && typeof spec.extraction !== 'string') {
      issues.push(`${label}.extraction is required for line-range evidence`)
    }
    if (entryLocalPaths.has(spec.localPath)) issues.push(`${identity} override references localPath more than once: ${spec.localPath}`)
    entryLocalPaths.add(spec.localPath)
    const absolutePath = validateOverrideLocalFile(spec, identity, index, state)
    wrapper.evidence.push({ spec, absolutePath })
  }

  const versionLocalPath = entry.versionEvidence?.localPath
  if (versionLocalPath !== undefined) {
    const matchingEvidence = wrapper.evidence.find(({ spec }) => spec.localPath === versionLocalPath)
    if (!matchingEvidence) issues.push(`${identity} override versionEvidence.localPath must also appear in files for bytes/SHA validation`)
    else if (matchingEvidence.spec.sha256 !== entry.versionEvidence.sha256) issues.push(`${identity} override versionEvidence SHA does not match its files entry`)
  }
}

function validateOverrideLocalFile(spec, identity, entryIndex, state) {
  const label = `${identity} override localPath`
  const localPath = spec.localPath
  if (typeof localPath !== 'string' || !isCanonicalRelativePosixPath(localPath)) {
    issues.push(`${label} must be a canonical relative POSIX path: ${JSON.stringify(localPath)}`)
    return null
  }
  const ownerSignature = JSON.stringify([
    spec.kind,
    spec.upstreamRepository,
    spec.upstreamRevision,
    spec.upstreamTag ?? null,
    spec.upstreamPath,
    spec.upstreamLineStart ?? null,
    spec.upstreamLineEnd ?? null,
    spec.extraction ?? null,
    spec.sha256,
    spec.bytes,
  ])
  const priorOwner = state.fileOwners.get(localPath)
  if (priorOwner !== undefined && priorOwner.signature !== ownerSignature) {
    issues.push(`${localPath} is shared by override entries ${priorOwner.entryIndex} and ${entryIndex} with different provenance`)
  } else if (priorOwner === undefined) {
    state.fileOwners.set(localPath, { entryIndex, signature: ownerSignature })
  }
  state.referencedFiles.add(localPath)

  const absolutePath = path.resolve(overrideRoot, ...localPath.split('/'))
  if (!isPathInside(overrideRoot, absolutePath)) {
    issues.push(`${label} escapes the override directory: ${localPath}`)
    return null
  }
  if (!pathHasNoSymlinkSegments(overrideRoot, localPath, label)) return null
  const stat = regularNonSymlinkStat(absolutePath, `${label} ${localPath}`)
  if (!stat) return null
  const realRoot = fs.realpathSync(overrideRoot)
  const realFile = fs.realpathSync(absolutePath)
  if (!isPathInside(realRoot, realFile)) {
    issues.push(`${label} realpath escapes the override directory: ${localPath}`)
    return null
  }
  if (stat.size !== spec.bytes) issues.push(`${label} byte length mismatch for ${localPath}: expected ${spec.bytes}, found ${stat.size}`)
  if (stat.size === 0 || fs.readFileSync(absolutePath).toString('utf8').trim() === '') {
    issues.push(`${label} evidence is empty or whitespace-only: ${localPath}`)
  }
  const actualHash = sha256File(absolutePath)
  if (actualHash !== spec.sha256) issues.push(`${label} SHA-256 mismatch for ${localPath}: expected ${spec.sha256}, found ${actualHash}`)
  return absolutePath
}

function validateOverrideFilesystem(state) {
  if (!isDirectory(overrideRoot)) {
    issues.push(`dependency override directory is missing: ${slash(path.relative(workspace, overrideRoot))}`)
    return
  }
  for (const localPath of strictRelativeRegularFiles(overrideRoot, 'dependency override directory')) {
    if (localPath === 'overrides.json' || localPath === 'README.md') continue
    if (!state.referencedFiles.has(localPath)) issues.push(`dependency override file is unused by overrides.json: ${localPath}`)
  }
}

function validateOverrideClosure() {
  for (const wrapper of overrideState.entries) {
    if (!overrideState.usedEntryIndexes.has(wrapper.index)) issues.push(`${wrapper.identity} override entry ${wrapper.index} is stale or unused`)
  }
}

function overrideEvidenceProvenance(spec) {
  return compactObject({
    kind: 'vetted-source-override',
    catalog_path: overrideCatalogRepositoryPath,
    local_path: spec.localPath,
    upstream_repository: spec.upstreamRepository,
    upstream_revision: spec.upstreamRevision,
    upstream_tag: spec.upstreamTag,
    upstream_path: spec.upstreamPath,
    upstream_line_start: spec.upstreamLineStart,
    upstream_line_end: spec.upstreamLineEnd,
    extraction: spec.extraction,
  })
}

function normalizeVersionEvidence(value) {
  if (!value || typeof value !== 'object') return null
  return compactObject({ upstream_path: value.upstreamPath, local_path: value.localPath, sha256: value.sha256, expected_name: value.expectedName, expected_version: value.expectedVersion })
}

function cyclonedxDocument(records, revision, repository, version) {
  return {
    bomFormat: 'CycloneDX',
    specVersion: '1.6',
    version: 1,
    metadata: {
      component: {
        type: 'application', name: 'NodeControll', version, 'bom-ref': githubPurl(repository, revision),
        externalReferences: revision === 'unknown' ? [] : [{ type: 'vcs', url: `https://github.com/${repository}/tree/${revision}` }],
      },
      tools: { components: [{ type: 'application', name: 'nodecontroll-license-collector', version: '2' }] },
      properties: [
        { name: 'nodecontroll:source-revision', value: revision },
        { name: 'nodecontroll:override-catalog-sha256', value: overrideState.catalogSha256 },
      ],
    },
    components: records.map(cyclonedxComponent),
  }
}

function cyclonedxComponent(record) {
  const component = {
    type: 'library', name: record.name, version: record.version, purl: record.purl, 'bom-ref': record.purl,
    licenses: record.declared_license ? [{ license: { name: record.declared_license } }] : [],
    externalReferences: record.repository ? [{ type: 'vcs', url: record.repository }] : [],
    properties: cyclonedxProperties(record),
  }
  if (record.ecosystem === 'cargo' && record.locked_integrity_kind === 'cargo-registry-sha256') {
    component.hashes = [{ alg: 'SHA-256', content: record.locked_integrity }]
  }
  return component
}

function cyclonedxProperties(record) {
  const properties = [
    { name: 'nodecontroll:ecosystem', value: record.ecosystem },
    { name: 'nodecontroll:source', value: record.source || 'unknown' },
  ]
  if (record.locked_integrity) properties.push({ name: `nodecontroll:locked-integrity:${record.locked_integrity_kind || 'unknown'}`, value: record.locked_integrity })
  if (record.license_override) {
    properties.push(
      { name: 'nodecontroll:license-override-catalog', value: record.license_override.catalog_path },
      { name: 'nodecontroll:license-override-catalog-sha256', value: record.license_override.catalog_sha256 },
      { name: 'nodecontroll:license-override-revision', value: record.license_override.revision },
    )
    record.license_override.upstream_paths.forEach((value, index) => properties.push({ name: `nodecontroll:license-override-upstream-path:${index + 1}`, value }))
  }
  if (record.toolchain_provenance) {
    for (const [name, value] of Object.entries(record.toolchain_provenance)) {
      if (value !== null) properties.push({ name: `nodecontroll:rust-toolchain:${name}`, value: String(value) })
    }
  }
  record.license_files.forEach((file, index) => {
    const sourcePath = file.source?.sysroot_relative_path || file.source?.upstream_path || file.source?.package_relative_path || 'unknown'
    properties.push({ name: `nodecontroll:license-evidence:${index + 1}`, value: `${file.kind}|${sourcePath}|sha256:${file.sha256}|bytes:${file.bytes}` })
  })
  return properties
}

function writeChecksums() {
  const files = walkFiles(path.join(output, 'licenses'))
  const lines = files.map((file) => `${sha256File(file)}  ${slash(path.relative(output, file))}`)
  fs.writeFileSync(path.join(output, 'LICENSES.sha256'), `${lines.join('\n')}\n`)
}

function writeReadme() {
  const cargoCount = notices.filter((record) => record.ecosystem === 'cargo').length
  const npmCount = notices.filter((record) => record.ecosystem === 'npm').length
  const rustRuntimeCount = notices.filter((record) => record.ecosystem === 'rust-toolchain').length
  const packageFileCount = evidenceFileCount('package-license-or-notice')
  const overrideFileCount = evidenceFileCount('vetted-source-license-or-notice')
  const rustFileCount = notices.filter((record) => record.ecosystem === 'rust-toolchain').reduce((sum, record) => sum + record.license_files.length, 0)
  const contents = [
    '# Third-party dependency notices', '',
    'This directory is generated from the exact locked dependency graph and Rust toolchain used for the build.',
    'It is an inventory and redistribution aid, not legal advice and not a replacement for each dependency license.', '',
    `- Source revision: \`${sourceRevision}\``, `- Source repository: \`${sourceRepository}\``,
    `- Cargo packages: ${cargoCount}`, `- npm packages installed from the pnpm lockfile: ${npmCount}`,
    `- Rust standard-library/runtime components: ${rustRuntimeCount}`,
    `- License/notice files shipped by dependency archives: ${packageFileCount}`,
    `- Vetted exact-source override evidence files: ${overrideFileCount}`,
    `- Installed Rust toolchain license/copyright files: ${rustFileCount}`,
    '- Collection warnings: 0',
    '- `DEPENDENCIES.json`: deterministic component, lock-integrity, override, toolchain, and evidence inventory.',
    '- `bom.cdx.json`: CycloneDX 1.6 component inventory with evidence provenance properties.',
    '- `LICENSES.sha256`: checksums for every copied license/notice file.',
    '- `licenses/`: package-archive evidence, vetted exact-source overrides, and installed Rust toolchain license material.', '',
  ].join('\n')
  fs.writeFileSync(path.join(output, 'README.md'), contents)
}

function readCargoLockPackages() {
  const raw = fs.readFileSync(path.join(workspace, 'Cargo.lock'), 'utf8')
  const result = new Map()
  for (const block of raw.split(/(?=^\[\[package\]\]\s*$)/m)) {
    if (!/^\[\[package\]\]/.test(block)) continue
    const name = tomlBasicString(block, 'name')
    const version = tomlBasicString(block, 'version')
    const source = tomlBasicString(block, 'source')
    const checksum = tomlBasicString(block, 'checksum')
    if (!name || !version || !source || !checksum) continue
    const key = cargoLockIdentity(name, version, source)
    if (result.has(key) && result.get(key) !== checksum) issues.push(`Cargo.lock has conflicting checksums for ${name}@${version} from ${source}`)
    else result.set(key, checksum)
  }
  return result
}

function readCargoWorkspaceVersion() {
  const manifestPath = path.join(workspace, 'Cargo.toml')
  const lines = fs.readFileSync(manifestPath, 'utf8').split(/\r?\n/)
  let section = null
  const versions = []
  for (const [lineIndex, line] of lines.entries()) {
    const sectionMatch = line.match(/^\s*\[([^\]]+)\]\s*(?:#.*)?$/)
    if (sectionMatch) {
      section = sectionMatch[1].trim()
      continue
    }
    if (section !== 'workspace.package' || !/^\s*version\s*=/.test(line)) continue
    const versionMatch = line.match(/^\s*version\s*=\s*("(?:\\.|[^"\\])*")\s*(?:#.*)?$/)
    if (!versionMatch) {
      throw new Error(`Cargo.toml workspace.package.version must be a TOML basic string at line ${lineIndex + 1}`)
    }
    versions.push(JSON.parse(versionMatch[1]))
  }
  if (versions.length !== 1) {
    throw new Error(`Cargo.toml must contain exactly one workspace.package.version, found ${versions.length}`)
  }
  const version = versions[0]
  if (!/^(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)(?:-[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?$/.test(version)) {
    throw new Error(`Cargo.toml workspace.package.version is not an exact SemVer: ${JSON.stringify(version)}`)
  }
  return version
}

function readPnpmLockIntegrities() {
  const lines = fs.readFileSync(path.join(workspace, 'pnpm-lock.yaml'), 'utf8').split(/\r?\n/)
  const topLevelEntries = scanPnpmLockTopLevelEntries(lines)
  const entriesByKey = new Map()
  for (const entry of topLevelEntries) {
    let matching = entriesByKey.get(entry.key)
    if (!matching) {
      matching = []
      entriesByKey.set(entry.key, matching)
    }
    matching.push(entry)
  }
  for (const [key, entries] of [...entriesByKey.entries()].sort(([left], [right]) => compareText(left, right))) {
    if (entries.length > 1) {
      throw new Error(`pnpm-lock.yaml has duplicate top-level key ${key} at lines ${entries.map(({ lineNumber }) => lineNumber).join(', ')}`)
    }
  }

  const expectedTopLevelKeys = ['lockfileVersion', 'settings', 'importers', 'packages', 'snapshots']
  const actualTopLevelKeys = topLevelEntries.map(({ key }) => key)
  if (
    actualTopLevelKeys.length !== expectedTopLevelKeys.length ||
    actualTopLevelKeys.some((key, index) => key !== expectedTopLevelKeys[index])
  ) {
    throw new Error(
      `pnpm-lock.yaml top-level keys must be exactly ${expectedTopLevelKeys.join(', ')} in that order; ` +
      `found ${actualTopLevelKeys.join(', ')}`,
    )
  }
  for (const section of ['settings', 'importers', 'packages', 'snapshots']) {
    const entry = entriesByKey.get(section)[0]
    if (entry.value.trim() !== '') {
      throw new Error(`pnpm-lock.yaml top-level ${section} must be a block mapping`)
    }
  }

  const versionEntries = entriesByKey.get('lockfileVersion') ?? []
  if (versionEntries.length === 0) {
    throw new Error('pnpm-lock.yaml is missing top-level lockfileVersion')
  } else if (versionEntries.length === 1) {
    let lockfileVersion = null
    try {
      lockfileVersion = parseYamlScalar(versionEntries[0].value.trim())
    } catch (error) {
      throw new Error(`pnpm-lock.yaml lockfileVersion is not a valid scalar: ${error.message}`)
    }
    if (lockfileVersion !== '9.0') {
      throw new Error(`pnpm-lock.yaml lockfileVersion must be exactly 9.0, found ${JSON.stringify(lockfileVersion)}`)
    }
  }

  const packageEntries = entriesByKey.get('packages') ?? []
  if (packageEntries.length === 0) {
    throw new Error('pnpm-lock.yaml is missing top-level packages')
  }
  const packagesEntry = packageEntries[0]
  const nextEntry = topLevelEntries.find(({ lineIndex }) => lineIndex > packagesEntry.lineIndex)
  const packagesEnd = nextEntry?.lineIndex ?? lines.length

  const result = new Map()
  const packageKeyLines = new Map()
  let currentKey = null
  for (let lineIndex = packagesEntry.lineIndex + 1; lineIndex < packagesEnd; lineIndex += 1) {
    const line = lines[lineIndex]
    const packageMatch = line.match(/^ {2}(\S.*):\s*$/)
    if (packageMatch) {
      try {
        currentKey = parseYamlScalar(packageMatch[1])
      } catch (error) {
        throw new Error(`pnpm-lock.yaml package key at line ${lineIndex + 1} is not a valid scalar: ${error.message}`)
      }
      if (typeof currentKey !== 'string' || currentKey === '') {
        throw new Error(`pnpm-lock.yaml package key at line ${lineIndex + 1} must be a non-empty string`)
      }
      const priorLine = packageKeyLines.get(currentKey)
      if (priorLine !== undefined) {
        throw new Error(`pnpm-lock.yaml has duplicate package key ${currentKey} at lines ${priorLine}, ${lineIndex + 1}`)
      }
      packageKeyLines.set(currentKey, lineIndex + 1)
      continue
    }
    if (/^ {2}\S/.test(line)) {
      throw new Error(`pnpm-lock.yaml has unsupported package-key YAML syntax at line ${lineIndex + 1}`)
    }
    if (!currentKey) continue
    const resolutionMatch = line.match(/^ {4}resolution:\s*\{.*?integrity:\s*([^,}]+).*\}\s*$/)
    if (!resolutionMatch) continue
    const integrity = parseYamlScalar(resolutionMatch[1].trim())
    if (result.has(currentKey) && result.get(currentKey) !== integrity) issues.push(`pnpm-lock.yaml has conflicting integrity values for ${currentKey}`)
    else result.set(currentKey, integrity)
  }
  return result
}

function scanPnpmLockTopLevelEntries(lines) {
  const allowedKeys = new Set(['lockfileVersion', 'settings', 'importers', 'packages', 'snapshots'])
  const entries = []
  for (const [lineIndex, line] of lines.entries()) {
    if (line.includes('\t')) {
      throw new Error(`pnpm-lock.yaml contains a tab at line ${lineIndex + 1}`)
    }
    if (line === '' || line.startsWith(' ') || line.startsWith('#')) continue
    const match = line.match(/^([A-Za-z][A-Za-z0-9_-]*):(?:\s*(.*))?$/)
    if (!match) {
      throw new Error(`pnpm-lock.yaml has unsupported top-level YAML syntax at line ${lineIndex + 1}`)
    }
    if (!allowedKeys.has(match[1])) {
      throw new Error(`pnpm-lock.yaml has unsupported top-level key ${match[1]} at line ${lineIndex + 1}`)
    }
    entries.push({ key: match[1], value: match[2] ?? '', lineIndex, lineNumber: lineIndex + 1 })
  }
  return entries
}

function tomlBasicString(block, field) {
  const escaped = field.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
  const match = block.match(new RegExp(`^${escaped}\\s*=\\s*("(?:\\\\.|[^"\\\\])*")\\s*$`, 'm'))
  if (!match) return null
  try { return JSON.parse(match[1]) } catch { return null }
}

function parseYamlScalar(value) {
  if (value.startsWith("'") && value.endsWith("'")) return value.slice(1, -1).replaceAll("''", "'")
  if (value.startsWith('"') && value.endsWith('"')) return JSON.parse(value)
  return value
}

function parseRustcVerboseVersion(raw) {
  const result = {}
  for (const line of raw.split(/\r?\n/)) {
    const separator = line.indexOf(': ')
    if (separator !== -1) result[line.slice(0, separator)] = line.slice(separator + 2).trim()
  }
  return result
}

function strictRegularFiles(directory, label) {
  if (!regularDirectoryNonSymlinkStat(directory, label)) return []
  return strictRelativeRegularFiles(directory, label).map((relative) => path.join(directory, ...relative.split('/')))
}

function strictRelativeRegularFiles(directory, label) {
  const result = []
  visit(directory, '')
  return result.sort(compareText)
  function visit(current, relativeDirectory) {
    let entries
    try { entries = fs.readdirSync(current, { withFileTypes: true }).sort((left, right) => compareText(left.name, right.name)) }
    catch (error) {
      issues.push(`${label} cannot be read: ${error.message}`)
      return
    }
    for (const entry of entries) {
      const relative = relativeDirectory ? `${relativeDirectory}/${entry.name}` : entry.name
      const target = path.join(current, entry.name)
      let stat
      try { stat = fs.lstatSync(target) } catch (error) {
        issues.push(`${label} cannot stat ${relative}: ${error.message}`)
        continue
      }
      if (stat.isSymbolicLink()) issues.push(`${label} must not contain symlinks: ${relative}`)
      else if (stat.isDirectory()) visit(target, relative)
      else if (stat.isFile()) result.push(relative)
      else issues.push(`${label} contains a non-regular filesystem entry: ${relative}`)
    }
  }
}

function regularNonSymlinkStat(target, label) {
  try {
    const stat = fs.lstatSync(target)
    if (stat.isSymbolicLink() || !stat.isFile()) {
      issues.push(`${label} must be a regular non-symlink file`)
      return null
    }
    return stat
  } catch (error) {
    issues.push(`${label} is missing or unreadable: ${error.message}`)
    return null
  }
}

function regularDirectoryNonSymlinkStat(target, label) {
  try {
    const stat = fs.lstatSync(target)
    if (stat.isSymbolicLink() || !stat.isDirectory()) {
      issues.push(`${label} must be a directory and not a symlink`)
      return null
    }
    return stat
  } catch (error) {
    issues.push(`${label} is missing or unreadable: ${error.message}`)
    return null
  }
}

function pathHasNoSymlinkSegments(root, relative, label) {
  let cursor = root
  for (const segment of relative.split('/')) {
    cursor = path.join(cursor, segment)
    try {
      const stat = fs.lstatSync(cursor)
      if (stat.isSymbolicLink()) {
        issues.push(`${label} traverses a symlink: ${relative}`)
        return false
      }
    } catch (error) {
      issues.push(`${label} is missing or unreadable: ${relative}: ${error.message}`)
      return false
    }
  }
  return true
}

function isCanonicalRelativePosixPath(value) {
  if (typeof value !== 'string' || value === '' || value.includes('\\') || value.includes('\0')) return false
  if (path.posix.isAbsolute(value) || path.posix.normalize(value) !== value) return false
  return !value.split('/').some((segment) => segment === '' || segment === '.' || segment === '..')
}

function isPathInside(root, target) {
  const relative = path.relative(path.resolve(root), path.resolve(target))
  return relative === '' || (!relative.startsWith(`..${path.sep}`) && relative !== '..' && !path.isAbsolute(relative))
}

function overrideIdentity(entry, index) {
  if (!entry || typeof entry !== 'object') return `invalid-override-entry-${index}`
  const integrity = entry.ecosystem === 'cargo' ? entry.registryChecksum : entry.registryIntegrity
  return `${entry.ecosystem ?? '<missing>'}:${entry.name ?? '<missing>'}@${entry.version ?? '<missing>'}#${integrity ?? '<missing>'}`
}

function recordIdentity(record) { return `${record.ecosystem}:${record.name}@${record.version}#${record.locked_integrity || '<missing>'}` }
function cargoLockIdentity(name, version, source) { return `${name}\0${version}\0${source || ''}` }
function compactObject(value) { return Object.fromEntries(Object.entries(value).filter(([, item]) => item !== undefined)) }

function normalizeLicense(value) {
  if (typeof value === 'string') return value.trim()
  if (value && typeof value.type === 'string') return value.type.trim()
  return null
}

function isRelativeLicensePointer(file, size) {
  if (!Number.isSafeInteger(size) || size <= 0 || size > 256) return false
  let text
  try {
    text = fs.readFileSync(file, 'utf8')
  } catch {
    return false
  }
  if (Buffer.byteLength(text, 'utf8') !== size) return false
  return /^(?:\.\.[\\/])+(?:licen[cs]e|copying|notice|copyright|unlicense)(?:[-._][A-Za-z0-9.-]+)?$/i.test(text.trim())
}

function normalizeRepository(value, label) {
  if (value === null || value === undefined) return null

  let raw
  if (typeof value === 'string') raw = value
  else if (value && typeof value === 'object' && !Array.isArray(value) && typeof value.url === 'string') raw = value.url
  else {
    issues.push(`${label} has unsupported repository metadata: ${JSON.stringify(value)}`)
    return null
  }

  const normalized = canonicalRepositoryUrl(raw)
  if (!normalized) {
    issues.push(`${label} repository cannot be safely normalized to an absolute http(s)/ssh URL: ${JSON.stringify(raw)}`)
    return null
  }
  return normalized
}

function canonicalRepositoryUrl(raw) {
  if (typeof raw !== 'string' || raw === '' || raw !== raw.trim() || /[\u0000-\u001f\u007f\\]/.test(raw)) return null

  let candidate = raw
  const githubShorthand = raw.match(/^github:([A-Za-z0-9_.-]+)\/([A-Za-z0-9_.-]+)$/i)
  const githubRelative = raw.match(/^([A-Za-z0-9_.-]+)\/([A-Za-z0-9_.-]+)$/)
  const scpLike = raw.match(/^([A-Za-z0-9._-]+)@([A-Za-z0-9.-]+):([A-Za-z0-9._~/-]+)$/)
  if (githubShorthand || githubRelative) {
    const [, owner, repository] = githubShorthand || githubRelative
    candidate = `https://github.com/${owner}/${repository}`
  } else if (scpLike) {
    const [, user, host, repositoryPath] = scpLike
    if (!isCanonicalRepositoryPath(repositoryPath)) return null
    candidate = `ssh://${user}@${host}/${repositoryPath}`
  } else if (/^git\+(?:https?|ssh):\/\//i.test(raw)) {
    candidate = raw.slice(4)
  } else if (/^git:\/\//i.test(raw)) {
    let gitUrl
    try {
      gitUrl = new URL(raw)
    } catch {
      return null
    }
    if (gitUrl.hostname.toLowerCase() !== 'github.com' || gitUrl.username || gitUrl.password || gitUrl.port) return null
    candidate = `https://github.com${gitUrl.pathname}${gitUrl.search}${gitUrl.hash}`
  }

  let parsed
  try {
    parsed = new URL(candidate)
  } catch {
    return null
  }
  if (!['http:', 'https:', 'ssh:'].includes(parsed.protocol) || !parsed.hostname || parsed.password) return null
  if ((parsed.protocol === 'http:' || parsed.protocol === 'https:') && parsed.username) return null

  if (parsed.hostname.toLowerCase() === 'github.com') {
    if (parsed.port) return null
    const segments = parsed.pathname.split('/').filter(Boolean)
    if (segments.length < 2 || !segments.every((segment) => /^[A-Za-z0-9_.-]+$/.test(segment))) return null
    parsed = new URL(`https://github.com${parsed.pathname}${parsed.search}${parsed.hash}`)
  }

  parsed.pathname = parsed.pathname.replace(/\.git\/?$/i, '').replace(/\/+$/, '')
  if (!parsed.pathname || parsed.pathname === '/') return null
  return parsed.href
}

function isCanonicalRepositoryPath(value) {
  if (!isCanonicalRelativePosixPath(value)) return false
  return value.split('/').every((segment) => /^[A-Za-z0-9._~-]+$/.test(segment))
}

function isCanonicalRepositoryUrl(value) {
  return typeof value === 'string' && canonicalRepositoryUrl(value) === value
}

function normalizedRepositoryName(value) {
  const normalized = String(value).trim()
  if (!/^[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/.test(normalized)) throw new Error(`invalid SOURCE_REPOSITORY owner/name: ${value}`)
  return normalized
}

function githubPurl(repository, revision) {
  const normalizedRepository = repository
    .split('/')
    .map((segment) => purlEncode(segment.toLowerCase()))
    .join('/')
  return `pkg:github/${normalizedRepository}@${purlEncode(revision)}`
}

function cargoPurl(name, version, source) {
  const base = `pkg:cargo/${purlEncode(name)}@${purlEncode(version)}`
  const sourceUrl = cargoSourceUrl(source)
  if (!sourceUrl) return base
  return `${base}?repository_url=${purlQualifierEncode(sourceUrl)}`
}

function cargoPackageSlug(name, version, source, checksum) {
  const identity = `${source || ''}\0${checksum || ''}`
  const suffix = createHash('sha256').update(identity).digest('hex').slice(0, 12)
  return `${name}-${version}-${suffix}`
}

function cargoSourceUrl(source) {
  if (typeof source !== 'string' || source === '') return null
  if (source.startsWith('registry+')) return source.slice('registry+'.length)
  if (source.startsWith('git+')) return source.slice('git+'.length)
  return null
}

function purlEncode(value) {
  return encodeURIComponent(value).replace(/[!'()*]/g, (character) =>
    `%${character.codePointAt(0).toString(16).toUpperCase()}`,
  )
}

function purlQualifierEncode(value) {
  return purlEncode(value).replaceAll('%3A', ':')
}

function licenseFileFromExpression(value) {
  const match = value?.match(/^SEE LICEN[CS]E IN (.+)$/i)
  return match?.[1]?.trim() || null
}

function sortedDirectoryEntries(directory) {
  try { return fs.readdirSync(directory, { withFileTypes: true }).sort((left, right) => compareText(left.name, right.name)) }
  catch (error) {
    if (error?.code === 'ENOENT' || error?.code === 'ENOTDIR') return []
    throw error
  }
}

function isDirectory(target) {
  try { return fs.statSync(target).isDirectory() }
  catch (error) {
    if (error?.code === 'ENOENT' || error?.code === 'ENOTDIR') return false
    throw error
  }
}

function safePathSegment(value) { return value.replaceAll('@', '').replaceAll('/', '__').replace(/[^A-Za-z0-9._-]/g, '_') }

function uniqueDestination(directory, basename) {
  let destination = path.join(directory, safePathSegment(basename))
  let index = 1
  while (fs.existsSync(destination)) {
    destination = path.join(directory, `${safePathSegment(basename)}.${index}`)
    index += 1
  }
  return destination
}

function walkFiles(directory) {
  if (!fs.existsSync(directory)) return []
  const result = []
  for (const entry of sortedDirectoryEntries(directory)) {
    const target = path.join(directory, entry.name)
    if (entry.isDirectory()) result.push(...walkFiles(target))
    else if (entry.isFile()) result.push(target)
  }
  return result.sort(compareText)
}

function sha256File(file) { return createHash('sha256').update(fs.readFileSync(file)).digest('hex') }
function licenseFileCount() { return notices.reduce((sum, record) => sum + record.license_files.length, 0) }
function evidenceFileCount(kind) { return notices.reduce((sum, record) => sum + record.license_files.filter((file) => file.kind === kind).length, 0) }
function writeJson(file, value) { fs.writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`) }
function slash(value) { return value.split(path.sep).join('/') }
function compareText(left, right) { return left < right ? -1 : left > right ? 1 : 0 }
