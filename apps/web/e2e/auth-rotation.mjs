import assert from 'node:assert/strict'
import { Buffer } from 'node:buffer'
import { createHash, createPrivateKey, createPublicKey, X509Certificate } from 'node:crypto'
import { constants as fsConstants, createReadStream } from 'node:fs'
import { link, lstat, open, readFile, readdir, realpath, unlink } from 'node:fs/promises'
import { request as httpRequest } from 'node:http'
import { createServer as createHttpsServer } from 'node:https'
import {
  basename,
  dirname,
  extname,
  isAbsolute,
  join,
  normalize,
  relative,
  resolve,
  sep,
} from 'node:path'
import process from 'node:process'
import { setTimeout as delay } from 'node:timers/promises'
import { URL } from 'node:url'

import { chromium } from '@playwright/test'

const requiredEnvironment = (name) => {
  const value = process.env[name]?.trim()
  if (!value) throw new Error(`${name} is required`)
  return value
}

const CLEANUP_TIMEOUT_MS = 5_000
const PROXY_TIMEOUT_MS = 10_000
const SCAN_READY_TIMEOUT_MS = 120_000
const knownSecrets = []
const knownSecretBinaryValues = []
const serverSockets = new Set()
let browser
let browserContext
let runRoot
let server
let serverOpen = false
let testFailure

const textualSecretRepresentations = (secret) => {
  const bytes = Buffer.from(secret, 'utf8')
  const json = JSON.stringify(secret)
  const percentEncoded = encodeURIComponent(secret)
  const htmlText = secret.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;')
  const htmlAttribute = htmlText.replaceAll('"', '&quot;').replaceAll("'", '&#39;')
  return [
    secret,
    json,
    json.slice(1, -1),
    percentEncoded,
    percentEncoded.replaceAll('%20', '+'),
    htmlText,
    htmlAttribute,
    bytes.toString('base64'),
    bytes.toString('base64url'),
    bytes.toString('hex'),
  ]
}

const decodedSecretBinaryValues = (secrets) => {
  const values = [...knownSecretBinaryValues]
  for (const secret of secrets) {
    if (typeof secret !== 'string') continue
    const match = /(?:^|_)([0-9a-f]{64})$/u.exec(secret)
    if (match) values.push(Buffer.from(match[1], 'hex'))
  }
  return values
}

const binaryEncodingRepresentations = (bytes) => [
  bytes.toString('base64'),
  bytes.toString('base64url'),
  bytes.toString('hex'),
]

const distinctTextualSecretRepresentations = (secrets) => [
  ...new Set(
    [
      ...secrets
        .filter((secret) => typeof secret === 'string' && secret.length > 0)
        .flatMap(textualSecretRepresentations),
      ...decodedSecretBinaryValues(secrets).flatMap(binaryEncodingRepresentations),
    ].filter((secret) => secret.length > 0),
  ),
]

const distinctSecretBuffers = (secrets) => {
  const buffers = [
    ...distinctTextualSecretRepresentations(secrets).map((secret) => Buffer.from(secret, 'utf8')),
    ...decodedSecretBinaryValues(secrets),
  ]
  return [
    ...new Map(
      buffers
        .filter((buffer) => Buffer.isBuffer(buffer) && buffer.byteLength > 0)
        .map((buffer) => [buffer.toString('base64'), buffer]),
    ).values(),
  ]
}

const redactKnownSecrets = (value) =>
  distinctTextualSecretRepresentations(knownSecrets)
    .sort((left, right) => right.length - left.length)
    .reduce((redacted, secret) => redacted.replaceAll(secret, '[REDACTED]'), value)

const bounded = async (operation, label, timeoutMs = CLEANUP_TIMEOUT_MS) => {
  const controller = new globalThis.AbortController()
  try {
    return await Promise.race([
      operation,
      delay(timeoutMs, undefined, { signal: controller.signal }).then(() => {
        throw new Error(`${label} exceeded ${timeoutMs}ms`)
      }),
    ])
  } finally {
    controller.abort()
  }
}

const closeBrowserContext = async () => {
  if (!browserContext) return
  const currentContext = browserContext
  browserContext = undefined
  await bounded(currentContext.close(), 'browser context close')
}

const closeBrowser = async () => {
  if (!browser) return
  const currentBrowser = browser
  browser = undefined
  await bounded(currentBrowser.close(), 'browser close')
}

const forceCloseServerConnections = (currentServer) => {
  currentServer.closeIdleConnections?.()
  currentServer.closeAllConnections?.()
  for (const socket of serverSockets) socket.destroy()
}

const closeServer = async () => {
  if (!server || !serverOpen) return
  const currentServer = server
  const closeOperation = new Promise((resolveClose, rejectClose) => {
    currentServer.close((error) => (error ? rejectClose(error) : resolveClose()))
  })
  try {
    try {
      await bounded(closeOperation, 'HTTPS proxy graceful close')
    } catch (gracefulError) {
      forceCloseServerConnections(currentServer)
      try {
        await bounded(closeOperation, 'HTTPS proxy forced close', 1_000)
      } catch {
        throw gracefulError
      }
    }
  } finally {
    serverOpen = false
    server = undefined
    serverSockets.clear()
  }
}

const readSecretFile = async (environmentName) => {
  const path = resolve(requiredEnvironment(environmentName))
  const metadata = await lstat(path)
  if (!metadata.isFile() || metadata.isSymbolicLink() || (metadata.mode & 0o077) !== 0) {
    throw new Error(`${environmentName} must reference a private regular non-symlink file`)
  }
  assert.equal(
    await realpath(path),
    path,
    `${environmentName} must not traverse a symlinked ancestor`,
  )
  const value = await readFile(path, 'utf8')
  const secret = value.trim()
  if (!secret || secret.includes('\n') || secret.includes('\r')) {
    throw new Error(`${environmentName} must reference one non-empty line`)
  }
  return secret
}

const isWithin = (root, candidate) => {
  const pathFromRoot = relative(root, candidate)
  return (
    pathFromRoot === '' ||
    (pathFromRoot !== '..' && !pathFromRoot.startsWith(`..${sep}`) && !isAbsolute(pathFromRoot))
  )
}

const requireAbsent = async (path, label) => {
  try {
    await lstat(path)
  } catch (error) {
    if (error && typeof error === 'object' && error.code === 'ENOENT') return
    throw error
  }
  throw new Error(`${label} must not already exist`)
}

const publishNewFile = async (path, contents) => {
  const temporary = join(dirname(path), `.${basename(path)}.${process.pid}.temporary`)
  await requireAbsent(path, 'published file')
  await requireAbsent(temporary, 'published temporary file')
  let handle = await open(temporary, 'wx', 0o600)
  try {
    await handle.writeFile(contents, 'utf8')
    await handle.sync()
    await handle.close()
    handle = undefined
    await link(temporary, path)
  } finally {
    if (handle) await handle.close().catch(() => undefined)
    await unlink(temporary).catch((error) => {
      if (!(error && typeof error === 'object' && error.code === 'ENOENT')) throw error
    })
  }
}

const containedRunFile = async (environmentName, label) => {
  const requested = resolve(requiredEnvironment(environmentName))
  assert.ok(
    requested !== runRoot && isWithin(runRoot, requested),
    `${label} must be inside run root`,
  )
  const requestedParent = dirname(requested)
  const parentMetadata = await lstat(requestedParent)
  assert.ok(
    parentMetadata.isDirectory() && !parentMetadata.isSymbolicLink(),
    `${label} parent must be a pre-created non-symlink directory`,
  )
  const parent = await realpath(requestedParent)
  assert.ok(isWithin(runRoot, parent), `${label} parent escaped run root`)
  assert.equal(parent, requestedParent, `${label} parent must not traverse a symlinked ancestor`)
  return join(parent, basename(requested))
}

const sha256File = async (path) => {
  const digest = createHash('sha256')
  for await (const chunk of createReadStream(path)) digest.update(chunk)
  return digest.digest('hex')
}

const waitForReadyMarker = async (path, expectedContents) => {
  const deadline = Date.now() + SCAN_READY_TIMEOUT_MS
  while (Date.now() < deadline) {
    let handle
    try {
      handle = await open(path, fsConstants.O_RDONLY | fsConstants.O_NOFOLLOW)
      const metadata = await handle.stat()
      if (!metadata.isFile()) {
        throw new Error('scan-ready marker must be a regular non-symlink file')
      }
      assert.equal(
        metadata.mode & 0o022,
        0,
        'scan-ready marker must not grant group/other write access',
      )
      const contents = await handle.readFile('utf8')
      assert.equal(contents, expectedContents, 'scan-ready marker is bound to another run')
      return metadata
    } catch (error) {
      if (!(error && typeof error === 'object' && error.code === 'ENOENT')) throw error
    } finally {
      if (handle) await handle.close()
    }
    await delay(100)
  }
  throw new Error('timed out waiting for the verifier to freeze browser scan artifacts')
}

try {
  const baseUrl = new URL(requiredEnvironment('NODECONTROLL_E2E_BASE_URL'))
  const upstreamUrl = new URL(requiredEnvironment('NODECONTROLL_E2E_UPSTREAM_URL'))
  assert.equal(baseUrl.protocol, 'https:', 'browser base URL must use HTTPS')
  assert.equal(upstreamUrl.protocol, 'http:', 'test proxy upstream must use HTTP')
  assert.equal(baseUrl.hostname, '127.0.0.1', 'HTTPS must bind literal IPv4 loopback')
  assert.equal(upstreamUrl.hostname, '127.0.0.1', 'Master upstream must bind literal IPv4 loopback')
  assert.ok(baseUrl.port, 'browser base URL must use an explicit port')
  assert.ok(upstreamUrl.port, 'Master upstream URL must use an explicit port')
  for (const port of [baseUrl.port, upstreamUrl.port]) {
    assert.ok(/^[1-9][0-9]{0,4}$/u.test(port), 'test URL port must be a positive decimal integer')
    assert.ok(Number(port) <= 65_535, 'test URL port must not exceed 65535')
  }
  for (const [label, url] of [
    ['browser base URL', baseUrl],
    ['Master upstream URL', upstreamUrl],
  ]) {
    assert.equal(url.pathname, '/', `${label} must not contain a path prefix`)
    assert.equal(url.search, '', `${label} must not contain a query`)
    assert.equal(url.hash, '', `${label} must not contain a fragment`)
    assert.equal(url.username, '', `${label} must not contain credentials`)
    assert.equal(url.password, '', `${label} must not contain credentials`)
  }
  assert.notEqual(baseUrl.origin, upstreamUrl.origin, 'browser and upstream origins must differ')

  const distInput = requiredEnvironment('NODECONTROLL_E2E_DIST_DIR')
  const runRootInput = requiredEnvironment('NODECONTROLL_E2E_RUN_ROOT')
  const tlsKeyInput = requiredEnvironment('NODECONTROLL_E2E_TLS_KEY_FILE')
  const tlsCertificateInput = requiredEnvironment('NODECONTROLL_E2E_TLS_CERT_FILE')
  for (const [label, path, expectedType] of [
    ['web dist', distInput, 'directory'],
    ['run root', runRootInput, 'directory'],
    ['TLS key', tlsKeyInput, 'file'],
    ['TLS certificate', tlsCertificateInput, 'file'],
  ]) {
    const metadata = await lstat(path)
    assert.ok(!metadata.isSymbolicLink(), `${label} input must not be a symbolic link`)
    assert.ok(
      expectedType === 'file' ? metadata.isFile() : metadata.isDirectory(),
      `${label} input has the wrong file type`,
    )
  }
  const tlsKeyMetadata = await lstat(tlsKeyInput)
  assert.equal(tlsKeyMetadata.mode & 0o077, 0, 'TLS private key must not grant group/other access')
  assert.equal(
    await realpath(tlsKeyInput),
    resolve(tlsKeyInput),
    'TLS private key must not traverse a symlinked ancestor',
  )
  const distRoot = await realpath(distInput)
  const tlsKey = await readFile(tlsKeyInput)
  const tlsCertificate = await readFile(tlsCertificateInput)
  const tlsKeyText = tlsKey.toString('utf8')
  const tlsKeyLf = tlsKeyText.replaceAll('\r\n', '\n')
  const tlsKeyCrlf = tlsKeyLf.replaceAll('\n', '\r\n')
  const tlsKeyBodyLines = tlsKeyLf
    .split('\n')
    .filter((line) => line.length > 0 && !line.startsWith('-----'))
  const tlsKeyPayload = tlsKeyBodyLines.join('')
  knownSecrets.push(
    ...new Set([
      tlsKeyText,
      tlsKeyText.trimEnd(),
      tlsKeyLf,
      tlsKeyLf.trimEnd(),
      tlsKeyCrlf,
      tlsKeyCrlf.trimEnd(),
      tlsKeyPayload,
      ...tlsKeyBodyLines,
    ]),
  )
  runRoot = await realpath(runRootInput)
  assert.equal(runRoot, '/evidence', 'browser gate run root must be the verifier /evidence mount')
  const runId = requiredEnvironment('NODECONTROLL_E2E_RUN_ID')
  assert.match(runId, /^[0-9]{8}T[0-9]{15}Z-[a-z0-9-]+$/u, 'browser gate run ID is invalid')
  const browserImageDigest = requiredEnvironment('NODECONTROLL_E2E_BROWSER_IMAGE_DIGEST')
  assert.match(browserImageDigest, /^sha256:[0-9a-f]{64}$/u, 'browser image digest is invalid')
  const sourceRevision = requiredEnvironment('NODECONTROLL_E2E_SOURCE_REVISION')
  assert.match(sourceRevision, /^[0-9a-f]{40}$/u, 'browser gate source revision is invalid')
  const tlsCertificateObject = new X509Certificate(tlsCertificate)
  const certificateValidFromMs = Date.parse(tlsCertificateObject.validFrom)
  const certificateValidToMs = Date.parse(tlsCertificateObject.validTo)
  const certificateValidationNowMs = Date.now()
  assert.ok(
    Number.isFinite(certificateValidFromMs) && Number.isFinite(certificateValidToMs),
    'TLS certificate validity bounds must be parseable',
  )
  assert.ok(
    certificateValidFromMs <= certificateValidationNowMs &&
      certificateValidationNowMs < certificateValidToMs,
    'TLS certificate must be currently valid',
  )
  assert.equal(
    tlsCertificateObject.checkIP(baseUrl.hostname),
    baseUrl.hostname,
    'TLS certificate SAN must contain the literal browser IP',
  )
  const tlsPrivateKey = createPrivateKey(tlsKey)
  knownSecretBinaryValues.push(
    tlsPrivateKey.export({ format: 'der', type: 'pkcs8' }),
    tlsPrivateKey.export({ format: 'der', type: 'pkcs1' }),
  )
  const tlsPrivatePublicKey = createPublicKey(tlsPrivateKey).export({ format: 'der', type: 'spki' })
  const tlsCertificatePublicKey = tlsCertificateObject.publicKey.export({
    format: 'der',
    type: 'spki',
  })
  assert.deepEqual(
    tlsPrivatePublicKey,
    tlsCertificatePublicKey,
    'TLS private key must match the browser certificate public key',
  )
  const tlsCertificateDerSha256 = createHash('sha256')
    .update(tlsCertificateObject.raw)
    .digest('hex')
  const tlsCertificatePemSha256 = createHash('sha256').update(tlsCertificate).digest('hex')
  const scanTargetContract = [
    { kind: 'directory', label: 'build_artifacts', relativePath: 'compiled/bin' },
    { kind: 'file', label: 'database', relativePath: 'browser/database' },
    {
      kind: 'file',
      label: 'database_dump',
      relativePath: 'browser/database-dump/control.sql',
    },
    {
      kind: 'file',
      label: 'openapi',
      relativePath: 'compiled/openapi/nodecontroll-v1.json',
    },
    { kind: 'directory', label: 'runtime_logs', relativePath: 'browser/runtime-logs' },
    { kind: 'directory', label: 'test_artifacts', relativePath: 'browser/test-artifacts' },
    { kind: 'directory', label: 'web_dist', relativePath: 'compiled/web' },
  ].map((target) => ({ ...target, requestedPath: resolve(runRoot, target.relativePath) }))
  assert.equal(
    new Set(scanTargetContract.map((target) => target.requestedPath)).size,
    scanTargetContract.length,
    'canonical scan target contract paths must be unique',
  )
  assert.equal(
    distRoot,
    scanTargetContract.find((target) => target.label === 'web_dist').requestedPath,
    'web dist must use the verifier canonical /evidence/compiled/web path',
  )
  const evidenceFile = await containedRunFile(
    'NODECONTROLL_E2E_EVIDENCE_FILE',
    'browser evidence file',
  )
  const behaviorReadyFile = await containedRunFile(
    'NODECONTROLL_E2E_BEHAVIOR_READY_FILE',
    'browser behavior-ready marker',
  )
  const scanReadyFile = await containedRunFile(
    'NODECONTROLL_E2E_SCAN_READY_FILE',
    'browser scan-ready marker',
  )
  const gateLogFile = resolve(requiredEnvironment('NODECONTROLL_E2E_GATE_LOG_FILE'))
  const gateLogMetadata = await lstat(gateLogFile)
  assert.ok(
    gateLogMetadata.isFile() && !gateLogMetadata.isSymbolicLink(),
    'gate log must be a pre-created regular non-symlink file',
  )
  assert.equal(gateLogMetadata.mode & 0o077, 0, 'gate log must not grant group/other access')
  assert.equal(
    await realpath(gateLogFile),
    gateLogFile,
    'gate log must not traverse a symlinked ancestor',
  )
  await requireAbsent(evidenceFile, 'browser evidence file')
  await requireAbsent(behaviorReadyFile, 'browser behavior-ready marker')
  await requireAbsent(scanReadyFile, 'browser scan-ready marker')
  const gateOutputFiles = [evidenceFile, behaviorReadyFile, scanReadyFile, gateLogFile]
  assert.equal(
    new Set(gateOutputFiles).size,
    gateOutputFiles.length,
    'gate output paths must be unique',
  )
  for (const outputFile of gateOutputFiles) {
    for (const target of scanTargetContract) {
      assert.ok(
        !isWithin(target.requestedPath, outputFile) && !isWithin(outputFile, target.requestedPath),
        'evidence and handshake files must remain outside every secret scan target',
      )
    }
  }
  const username = process.env.NODECONTROLL_E2E_USERNAME?.trim() || 'owner'
  const instanceName =
    process.env.NODECONTROLL_E2E_INSTANCE_NAME?.trim() || 'NodeControll Browser Gate'
  const setupToken = await readSecretFile('NODECONTROLL_E2E_SETUP_TOKEN_FILE')
  knownSecrets.push(setupToken)
  const password = await readSecretFile('NODECONTROLL_E2E_PASSWORD_FILE')
  knownSecrets.push(password)
  const rootKey = await readSecretFile('NODECONTROLL_E2E_ROOT_KEY_FILE')
  knownSecrets.push(rootKey)
  const wrongProofFixture = 'intentionally-wrong-browser-gate-proof'
  const intentionallyWrongProof =
    password === wrongProofFixture ? `${wrongProofFixture}-alternate` : wrongProofFixture
  assert.notEqual(
    password,
    intentionallyWrongProof,
    'E2E wrong proof must differ from the password',
  )
  const playwrightPackage = JSON.parse(
    await readFile(
      new URL('../node_modules/@playwright/test/package.json', import.meta.url),
      'utf8',
    ),
  )
  assert.equal(
    playwrightPackage.version,
    '1.62.0',
    'Playwright package version must match the gate',
  )
  const gateScriptSha256 = createHash('sha256')
    .update(await readFile(new URL(import.meta.url)))
    .digest('hex')
  const browserExecutable = chromium.executablePath()
  const browserExecutableMetadata = await lstat(browserExecutable)
  assert.ok(
    browserExecutableMetadata.isFile() && !browserExecutableMetadata.isSymbolicLink(),
    'Chromium executable must be a regular non-symlink file',
  )
  const browserExecutableCanonical = await realpath(browserExecutable)
  const browserExecutableSha256 = await sha256File(browserExecutableCanonical)

  const contentTypes = new Map([
    ['.css', 'text/css; charset=utf-8'],
    ['.html', 'text/html; charset=utf-8'],
    ['.ico', 'image/x-icon'],
    ['.js', 'text/javascript; charset=utf-8'],
    ['.json', 'application/json; charset=utf-8'],
    ['.map', 'application/json; charset=utf-8'],
    ['.svg', 'image/svg+xml'],
    ['.woff', 'font/woff'],
    ['.woff2', 'font/woff2'],
  ])

  const existingStaticFile = async (candidate) => {
    const lexicalCandidate = resolve(candidate)
    if (!isWithin(distRoot, lexicalCandidate)) return undefined
    const metadata = await lstat(lexicalCandidate)
    if (metadata.isSymbolicLink()) throw new Error('web dist must not contain symbolic links')
    const file = metadata.isDirectory() ? join(lexicalCandidate, 'index.html') : lexicalCandidate
    const fileMetadata = metadata.isDirectory() ? await lstat(file) : metadata
    if (fileMetadata.isSymbolicLink()) throw new Error('web dist must not contain symbolic links')
    if (!fileMetadata.isFile()) return undefined
    const canonicalFile = await realpath(file)
    if (!isWithin(distRoot, canonicalFile))
      throw new Error('web dist file escaped its canonical root')
    return canonicalFile
  }

  const safeStaticPath = async (pathname) => {
    let decoded
    try {
      decoded = decodeURIComponent(pathname)
    } catch {
      return undefined
    }
    const normalized = normalize(decoded).replace(/^[/\\]+/u, '')
    if (!normalized || normalized === '.') return existingStaticFile(join(distRoot, 'index.html'))
    const candidate = resolve(distRoot, normalized)
    if (!isWithin(distRoot, candidate)) return undefined
    try {
      const file = await existingStaticFile(candidate)
      if (file) return file
    } catch {
      // The production SPA falls through to its root index for client-side routes.
    }
    return existingStaticFile(join(distRoot, 'index.html'))
  }

  const fixedHopByHopHeaders = new Set([
    'connection',
    'keep-alive',
    'proxy-authenticate',
    'proxy-authorization',
    'proxy-connection',
    'te',
    'trailer',
    'transfer-encoding',
    'upgrade',
  ])

  const connectionHeaderTokens = (value) =>
    (Array.isArray(value) ? value.join(',') : value || '')
      .split(',')
      .map((token) => token.trim().toLowerCase())
      .filter((token) => token.length > 0)

  const withoutHopByHopHeaders = (sourceHeaders) => {
    const headers = { ...sourceHeaders }
    const rejectedNames = new Set([
      ...fixedHopByHopHeaders,
      ...connectionHeaderTokens(headers.connection),
    ])
    for (const name of rejectedNames) delete headers[name]
    return headers
  }

  const proxyRequest = (request, response) => {
    if (!request.url?.startsWith('/') || request.url.startsWith('//')) {
      response.writeHead(400, { 'cache-control': 'no-store' })
      response.end()
      return
    }
    let target
    try {
      target = new URL(request.url || '/', upstreamUrl)
    } catch {
      response.writeHead(400, { 'cache-control': 'no-store' })
      response.end()
      return
    }
    if (target.origin !== upstreamUrl.origin) {
      response.writeHead(400, { 'cache-control': 'no-store' })
      response.end()
      return
    }
    const headers = withoutHopByHopHeaders(request.headers)
    headers.host = baseUrl.host
    let proxyFailed = false
    const failProxy = () => {
      if (proxyFailed) return
      proxyFailed = true
      if (response.headersSent || response.destroyed) {
        response.destroy()
        return
      }
      try {
        response.writeHead(502, { 'cache-control': 'no-store', 'content-type': 'text/plain' })
        response.end('upstream unavailable')
      } catch {
        response.destroy()
      }
    }
    const upstream = httpRequest(
      target,
      {
        headers,
        method: request.method,
      },
      (upstreamResponse) => {
        try {
          upstreamResponse.once('aborted', failProxy)
          upstreamResponse.once('error', failProxy)
          response.writeHead(
            upstreamResponse.statusCode || 502,
            withoutHopByHopHeaders(upstreamResponse.headers),
          )
          upstreamResponse.pipe(response)
        } catch {
          upstreamResponse.destroy()
          failProxy()
        }
      },
    )
    upstream.setTimeout(PROXY_TIMEOUT_MS, () => {
      upstream.destroy(new Error('Master proxy request timed out'))
    })
    upstream.once('error', failProxy)
    request.once('aborted', () => upstream.destroy())
    request.once('error', () => upstream.destroy())
    response.once('close', () => {
      if (!response.writableEnded) upstream.destroy()
    })
    request.pipe(upstream)
  }

  const serveStatic = async (request, response) => {
    if (request.method !== 'GET' && request.method !== 'HEAD') {
      response.writeHead(405, { allow: 'GET, HEAD', 'cache-control': 'no-store' })
      response.end()
      return
    }
    const pathname = new URL(request.url || '/', baseUrl).pathname
    const file = await safeStaticPath(pathname)
    if (!file) {
      response.writeHead(400, { 'cache-control': 'no-store' })
      response.end()
      return
    }
    const body = await readFile(file)
    response.writeHead(200, {
      'cache-control': 'no-store',
      'content-length': body.byteLength,
      'content-type': contentTypes.get(extname(file).toLowerCase()) || 'application/octet-stream',
    })
    response.end(request.method === 'HEAD' ? undefined : body)
  }

  let serverRuntimeError
  const startHttpsServer = async () => {
    assert.equal(server, undefined, 'HTTPS proxy must start only once')
    const currentServer = createHttpsServer(
      { cert: tlsCertificate, key: tlsKey, minVersion: 'TLSv1.2' },
      (request, response) => {
        let pathname
        try {
          pathname = new URL(request.url || '/', baseUrl).pathname
        } catch {
          response.writeHead(400, { 'cache-control': 'no-store' })
          response.end()
          return
        }
        if (
          pathname === '/healthz' ||
          pathname === '/readyz' ||
          pathname.startsWith('/api/') ||
          pathname.startsWith('/api-docs/')
        ) {
          try {
            proxyRequest(request, response)
          } catch {
            if (!response.headersSent) response.writeHead(502, { 'cache-control': 'no-store' })
            response.end()
          }
          return
        }
        void serveStatic(request, response).catch(() => {
          if (!response.headersSent) response.writeHead(500, { 'cache-control': 'no-store' })
          response.end()
        })
      },
    )
    currentServer.on('connection', (socket) => {
      serverSockets.add(socket)
      socket.once('close', () => serverSockets.delete(socket))
    })
    server = currentServer
    await new Promise((resolveListen, rejectListen) => {
      currentServer.once('error', rejectListen)
      currentServer.listen(Number(baseUrl.port), baseUrl.hostname, () => {
        currentServer.off('error', rejectListen)
        resolveListen()
      })
    })
    serverOpen = true
    currentServer.on('error', (error) => {
      serverRuntimeError ??= error
    })
  }

  const upstreamReadinessStatus = () =>
    new Promise((resolveStatus) => {
      let responseReceived = false
      let settled = false
      const settle = (status) => {
        if (settled) return
        settled = true
        resolveStatus(status)
      }
      const request = httpRequest(
        new URL('/readyz', upstreamUrl),
        { headers: { host: baseUrl.host }, method: 'GET' },
        (response) => {
          responseReceived = true
          response.resume()
          response.once('end', () => settle(response.statusCode || 0))
          response.once('aborted', () => settle(0))
          response.once('error', () => settle(0))
          response.once('close', () => {
            if (!response.complete) settle(0)
          })
        },
      )
      request.setTimeout(1_000, () => {
        request.destroy()
        settle(0)
      })
      request.once('error', () => settle(0))
      request.once('close', () => {
        if (!responseReceived) settle(0)
      })
      request.end()
    })

  const waitForUpstreamReadiness = async () => {
    const deadline = Date.now() + 10_000
    while (Date.now() < deadline) {
      if ((await upstreamReadinessStatus()) === 200) return
      await delay(Math.min(100, Math.max(1, deadline - Date.now())))
    }
    throw new Error('fresh E2E Master did not become ready within the bounded wait')
  }

  const sessionCookieName = '__Host-nodecontroll_session'
  const csrfCookieName = '__Host-nodecontroll_csrf'
  const credentialCoordinationKey = 'nodecontroll:credential-coordination:v1'
  const credentialRecordCommonFields = [
    'baseSeq',
    'disposition',
    'epoch',
    'opId',
    'operation',
    'phase',
    'senderId',
    'seq',
    'v',
  ]
  const credentialMutationOperations = new Set([
    'change-password',
    'login',
    'logout',
    'logout-all',
    'reauth',
    'revoke',
  ])
  const credentialIdentifierPattern =
    /^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/iu
  const credentialRandomIdentifierPattern =
    /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/iu
  const credentialSequencePattern = /^(0|[1-9][0-9]{0,19})$/u
  const maximumCredentialSequence = 18_446_744_073_709_551_615n

  const storageEntries = async (page) =>
    page.evaluate(() => {
      const entries = (storage) =>
        Array.from({ length: storage.length }, (_, index) => {
          const key = storage.key(index)
          return [key, key === null ? null : storage.getItem(key)]
        }).sort(([left], [right]) => String(left).localeCompare(String(right), 'en'))
      return {
        localStorage: entries(globalThis.localStorage),
        sessionStorage: entries(globalThis.sessionStorage),
      }
    })

  const credentialSequence = (value, label) => {
    assert.equal(typeof value, 'string', `${label}: credential sequence must be a string`)
    assert.match(value, credentialSequencePattern, `${label}: credential sequence is malformed`)
    const parsed = BigInt(value)
    assert.ok(parsed <= maximumCredentialSequence, `${label}: credential sequence exceeds uint64`)
    return parsed
  }

  const assertCredentialCoordinationJournal = (entries, label) => {
    assert.deepEqual(
      entries.map(([key]) => key),
      [credentialCoordinationKey],
      `${label}: localStorage must contain only the credential coordination journal`,
    )
    const raw = entries[0]?.[1]
    assert.equal(typeof raw, 'string', `${label}: credential coordination journal must be text`)
    assert.ok(
      Buffer.byteLength(raw, 'utf8') <= 2_048,
      `${label}: credential coordination journal must remain bounded`,
    )
    let record
    try {
      record = JSON.parse(raw)
    } catch {
      assert.fail(`${label}: credential coordination journal must contain JSON`)
    }
    assert.ok(
      record && typeof record === 'object' && !Array.isArray(record),
      `${label}: credential coordination journal must contain one object`,
    )
    const expectedFields =
      record.phase === 'invalidated' && record.observedSessionId !== undefined
        ? [...credentialRecordCommonFields, 'observedSessionId']
        : credentialRecordCommonFields
    assert.deepEqual(
      Object.keys(record).sort(),
      [...expectedFields].sort(),
      `${label}: credential coordination journal fields differ from the strict schema`,
    )
    assert.equal(record.v, 1, `${label}: credential coordination journal version must be 1`)
    for (const field of ['epoch', 'opId', 'senderId']) {
      assert.equal(typeof record[field], 'string', `${label}: ${field} must be a string`)
      assert.match(
        record[field],
        credentialRandomIdentifierPattern,
        `${label}: ${field} must be a random UUID`,
      )
    }
    const baseSequence = credentialSequence(record.baseSeq, `${label} baseSeq`)
    const sequence = credentialSequence(record.seq, `${label} seq`)
    assert.ok(sequence >= 1n, `${label}: credential record sequence must be positive`)
    assert.equal(
      baseSequence + 1n,
      sequence,
      `${label}: credential record sequence must advance exactly once`,
    )

    if (record.phase === 'inflight') {
      assert.equal(record.disposition, 'quarantine', `${label}: inflight records must quarantine`)
      assert.ok(
        credentialMutationOperations.has(record.operation),
        `${label}: inflight operation is not allowed`,
      )
    } else if (record.phase === 'settled') {
      assert.ok(
        record.disposition === 'quarantine' || record.disposition === 'reconcile',
        `${label}: settled disposition is not allowed`,
      )
      assert.ok(
        credentialMutationOperations.has(record.operation),
        `${label}: settled operation is not allowed`,
      )
    } else {
      assert.equal(record.phase, 'invalidated', `${label}: credential record phase is not allowed`)
      assert.equal(record.disposition, 'reconcile', `${label}: invalidation must reconcile`)
      assert.equal(
        record.operation,
        'read-401',
        `${label}: invalidation operation must be read-401`,
      )
      if (record.observedSessionId !== undefined) {
        assert.equal(
          typeof record.observedSessionId,
          'string',
          `${label}: observed session ID must be a string`,
        )
        assert.match(
          record.observedSessionId,
          credentialIdentifierPattern,
          `${label}: observed session ID must be a UUID`,
        )
      }
    }

    for (const secret of distinctTextualSecretRepresentations(knownSecrets)) {
      assert.ok(!raw.includes(secret), `${label}: credential coordination journal exposed a secret`)
    }
    return { raw, record }
  }

  const cookiePair = (cookies, label) => {
    const session = cookies.filter((cookie) => cookie.name === sessionCookieName)
    const csrf = cookies.filter((cookie) => cookie.name === csrfCookieName)
    assert.equal(session.length, 1, `${label}: exactly one session cookie is required`)
    assert.equal(csrf.length, 1, `${label}: exactly one CSRF cookie is required`)
    assert.equal(session[0].httpOnly, true, `${label}: session cookie must be HttpOnly`)
    assert.equal(csrf[0].httpOnly, false, `${label}: CSRF cookie must be script-readable`)
    for (const cookie of [session[0], csrf[0]]) {
      assert.equal(cookie.secure, true, `${label}: cookie must be Secure`)
      assert.equal(cookie.sameSite, 'Lax', `${label}: cookie must be SameSite=Lax`)
      assert.equal(cookie.path, '/', `${label}: cookie must be host-wide`)
      assert.equal(cookie.domain, baseUrl.hostname, `${label}: cookie must be host-only`)
    }
    return { csrf: csrf[0], session: session[0] }
  }

  const responseCookiePair = (headerFields, expectedValues, label) => {
    const setCookieFields = headerFields.filter(
      (field) => field.name.toLowerCase() === 'set-cookie',
    )
    assert.equal(setCookieFields.length, 2, `${label}: exactly two Set-Cookie fields are required`)
    const parsed = new Map()
    for (const field of setCookieFields) {
      const segments = field.value.split(';').map((segment) => segment.trim())
      const pairSeparator = segments[0].indexOf('=')
      assert.ok(pairSeparator > 0, `${label}: Set-Cookie must start with a name/value pair`)
      const name = segments[0].slice(0, pairSeparator)
      const value = segments[0].slice(pairSeparator + 1)
      assert.ok(!parsed.has(name), `${label}: cookie names must be unique`)
      const attributes = new Map()
      for (const segment of segments.slice(1)) {
        const separator = segment.indexOf('=')
        const attributeName = (separator < 0 ? segment : segment.slice(0, separator)).toLowerCase()
        const attributeValue = separator < 0 ? true : segment.slice(separator + 1)
        assert.ok(attributeName.length > 0, `${label}: cookie attribute names must not be empty`)
        assert.ok(!attributes.has(attributeName), `${label}: cookie attributes must not repeat`)
        attributes.set(attributeName, attributeValue)
      }
      parsed.set(name, { attributes, value })
    }
    assert.equal(parsed.size, 2, `${label}: only the session and CSRF cookies are allowed`)
    const session = parsed.get(sessionCookieName)
    const csrf = parsed.get(csrfCookieName)
    assert.ok(session, `${label}: session Set-Cookie is required`)
    assert.ok(csrf, `${label}: CSRF Set-Cookie is required`)
    assert.ok(session.value === expectedValues.session.value, `${label}: session value mismatch`)
    assert.ok(csrf.value === expectedValues.csrf.value, `${label}: CSRF value mismatch`)
    for (const [name, cookie] of [
      [sessionCookieName, session],
      [csrfCookieName, csrf],
    ]) {
      assert.equal(cookie.attributes.get('secure'), true, `${label}: ${name} must be Secure`)
      assert.equal(cookie.attributes.get('path'), '/', `${label}: ${name} must use Path=/`)
      assert.equal(
        cookie.attributes.get('samesite'),
        'Lax',
        `${label}: ${name} must be SameSite=Lax`,
      )
      assert.ok(!cookie.attributes.has('domain'), `${label}: ${name} must be host-only`)
      const maxAgeText = cookie.attributes.get('max-age')
      assert.equal(typeof maxAgeText, 'string', `${label}: ${name} must have one Max-Age`)
      assert.match(maxAgeText, /^[1-9][0-9]*$/u, `${label}: ${name} Max-Age must be positive`)
    }
    assert.equal(session.attributes.get('httponly'), true, `${label}: session must be HttpOnly`)
    assert.ok(!csrf.attributes.has('httponly'), `${label}: CSRF cookie must be script-readable`)
    const sessionMaxAge = Number(session.attributes.get('max-age'))
    const csrfMaxAge = Number(csrf.attributes.get('max-age'))
    assert.equal(sessionMaxAge, csrfMaxAge, `${label}: both cookies must use the same Max-Age`)
    assert.ok(Number.isSafeInteger(sessionMaxAge), `${label}: Max-Age must be a safe integer`)
    return { maxAgeSeconds: sessionMaxAge }
  }

  const authenticationSetCookieFields = (headerFields) =>
    headerFields.filter(
      (field) =>
        field.name.toLowerCase() === 'set-cookie' &&
        (field.value.startsWith(`${sessionCookieName}=`) ||
          field.value.startsWith(`${csrfCookieName}=`)),
    )

  const setCookieFields = (headerFields) =>
    headerFields.filter((field) => field.name.toLowerCase() === 'set-cookie')

  const assertNoSetCookie = (headerFields, label) => {
    const fields = setCookieFields(headerFields)
    assert.equal(fields.length, 0, `${label}: response must not emit Set-Cookie`)
    return fields.length
  }

  const assertNoAuthenticationSetCookie = (headerFields, label) => {
    assert.equal(
      authenticationSetCookieFields(headerFields).length,
      0,
      `${label}: failed authentication must not mutate browser credentials`,
    )
  }

  const assertClearingResponseCookiePair = (headerFields, label) => {
    const fields = headerFields.filter((field) => field.name.toLowerCase() === 'set-cookie')
    assert.equal(fields.length, 2, `${label}: exactly two clearing Set-Cookie fields are required`)
    const parsed = new Map()
    for (const field of fields) {
      const segments = field.value.split(';').map((segment) => segment.trim())
      const pairSeparator = segments[0].indexOf('=')
      assert.ok(pairSeparator > 0, `${label}: clearing cookie must start with a name/value pair`)
      const name = segments[0].slice(0, pairSeparator)
      const value = segments[0].slice(pairSeparator + 1)
      assert.equal(value, '', `${label}: clearing cookie value must be empty`)
      assert.ok(!parsed.has(name), `${label}: clearing cookie names must be unique`)
      const attributes = new Map()
      for (const segment of segments.slice(1)) {
        const separator = segment.indexOf('=')
        const attributeName = (separator < 0 ? segment : segment.slice(0, separator)).toLowerCase()
        const attributeValue = separator < 0 ? true : segment.slice(separator + 1)
        assert.ok(attributeName.length > 0, `${label}: clearing attribute name must not be empty`)
        assert.ok(!attributes.has(attributeName), `${label}: clearing attributes must not repeat`)
        attributes.set(attributeName, attributeValue)
      }
      parsed.set(name, attributes)
    }
    const session = parsed.get(sessionCookieName)
    const csrf = parsed.get(csrfCookieName)
    assert.ok(session, `${label}: session clearing field is required`)
    assert.ok(csrf, `${label}: CSRF clearing field is required`)
    for (const [name, attributes] of [
      [sessionCookieName, session],
      [csrfCookieName, csrf],
    ]) {
      assert.equal(attributes.get('max-age'), '0', `${label}: ${name} must use Max-Age=0`)
      assert.equal(attributes.get('secure'), true, `${label}: ${name} must remain Secure`)
      assert.equal(attributes.get('path'), '/', `${label}: ${name} must use Path=/`)
      assert.equal(attributes.get('samesite'), 'Lax', `${label}: ${name} must be SameSite=Lax`)
      assert.ok(!attributes.has('domain'), `${label}: ${name} must remain host-only`)
    }
    assert.equal(session.get('httponly'), true, `${label}: session clearing field must be HttpOnly`)
    assert.ok(!csrf.has('httponly'), `${label}: CSRF clearing field must remain script-readable`)
  }

  const comparableCookie = (cookie) => ({
    domain: cookie.domain,
    expires: cookie.expires,
    httpOnly: cookie.httpOnly,
    name: cookie.name,
    path: cookie.path,
    sameSite: cookie.sameSite,
    secure: cookie.secure,
    value: cookie.value,
  })

  const assertCookiePairUnchanged = (actual, expected, label) => {
    assert.deepEqual(
      comparableCookie(actual.session),
      comparableCookie(expected.session),
      `${label}: session cookie changed`,
    )
    assert.deepEqual(
      comparableCookie(actual.csrf),
      comparableCookie(expected.csrf),
      `${label}: CSRF cookie changed`,
    )
  }

  const assertSessionProjectionUnchanged = (actualResult, expectedSession, label) => {
    assert.equal(actualResult.status, 200, `${label}: rotated session must remain active`)
    assert.deepEqual(
      actualResult.body?.data?.session,
      expectedSession,
      `${label}: server session projection changed`,
    )
  }

  const pageFetch = async (page, path) =>
    page.evaluate(async (requestPath) => {
      const response = await globalThis.fetch(requestPath, {
        cache: 'no-store',
        credentials: 'same-origin',
        headers: { accept: 'application/json' },
      })
      let body
      try {
        body = await response.json()
      } catch {
        body = undefined
      }
      return { body, status: response.status }
    }, path)

  const pageReauthenticationAttempt = async (page, csrfToken, proof) =>
    page.evaluate(
      async ({ csrf, password: attemptedPassword }) => {
        const response = await globalThis.fetch('/api/v1/auth/reauth', {
          body: JSON.stringify({ method: 'password', password: attemptedPassword }),
          cache: 'no-store',
          credentials: 'same-origin',
          headers: {
            accept: 'application/json',
            'content-type': 'application/json',
            'x-nodecontroll-csrf': csrf,
          },
          method: 'POST',
        })
        let body
        try {
          body = await response.json()
        } catch {
          body = undefined
        }
        return { body, status: response.status }
      },
      { csrf: csrfToken, password: proof },
    )

  const assertFrozenMetadata = (metadata, label) => {
    assert.equal(metadata.mode & 0o222, 0, `${label} must not have owner/group/other write bits`)
  }

  const readFrozenFile = async (path) => {
    const handle = await open(path, fsConstants.O_RDONLY | fsConstants.O_NOFOLLOW)
    try {
      const metadata = await handle.stat()
      assert.ok(metadata.isFile(), `frozen artifact is not a regular file: ${path}`)
      assertFrozenMetadata(metadata, `frozen artifact ${path}`)
      return await handle.readFile()
    } finally {
      await handle.close()
    }
  }

  const regularFiles = async (root) => {
    const metadata = await lstat(root)
    if (metadata.isSymbolicLink()) throw new Error(`secret scan root contains symlink: ${root}`)
    assertFrozenMetadata(metadata, `secret scan path ${root}`)
    if (metadata.isFile()) return [root]
    if (!metadata.isDirectory())
      throw new Error(`secret scan root is not a regular file/directory: ${root}`)
    const files = []
    const entries = await readdir(root, { withFileTypes: true })
    entries.sort((left, right) => left.name.localeCompare(right.name, 'en'))
    for (const entry of entries) {
      const child = join(root, entry.name)
      if (entry.isSymbolicLink()) throw new Error(`secret scan root contains symlink: ${child}`)
      if (entry.isDirectory()) files.push(...(await regularFiles(child)))
      else if (entry.isFile()) files.push(child)
      else throw new Error(`secret scan root contains special file: ${child}`)
    }
    return files
  }

  const scanFilesForSecrets = async (targets, secrets) => {
    const secretBuffers = secrets
    let scannedFiles = 0
    let scannedBytes = 0
    const targetEvidence = []
    for (const target of targets) {
      const files = await regularFiles(target.path)
      assert.ok(files.length > 0, `secret scan target ${target.label} must not be empty`)
      const treeHash = createHash('sha256')
      let targetBytes = 0
      for (const file of files) {
        const body = await readFrozenFile(file)
        scannedFiles += 1
        scannedBytes += body.byteLength
        targetBytes += body.byteLength
        const relativeFile = relative(target.path, file) || basename(file)
        treeHash.update(Buffer.from(relativeFile.replaceAll(sep, '/'), 'utf8'))
        treeHash.update(Buffer.from([0]))
        treeHash.update(Buffer.from(String(body.byteLength), 'utf8'))
        treeHash.update(Buffer.from([0]))
        treeHash.update(body)
        for (const secret of secretBuffers) {
          if (body.includes(secret)) {
            throw new Error(`secret material found in scanned artifact: ${file}`)
          }
        }
      }
      assert.ok(targetBytes > 0, `secret scan target ${target.label} must contain non-empty bytes`)
      targetEvidence.push({
        bytes: targetBytes,
        files: files.length,
        label: target.label,
        path: relative(runRoot, target.path).replaceAll(sep, '/'),
        treeSha256: treeHash.digest('hex'),
      })
    }
    return { scannedBytes, scannedFiles, targets: targetEvidence }
  }

  const prepareCanonicalScanTargets = async () => {
    const targets = []
    for (const contract of scanTargetContract) {
      assert.ok(
        contract.requestedPath !== runRoot && isWithin(runRoot, contract.requestedPath),
        `scan target ${contract.label} escaped the run root`,
      )
      const metadata = await lstat(contract.requestedPath)
      assert.ok(!metadata.isSymbolicLink(), `scan target ${contract.label} must not be a symlink`)
      assert.equal(
        contract.kind === 'file' ? metadata.isFile() : metadata.isDirectory(),
        true,
        `scan target ${contract.label} has the wrong root type`,
      )
      assertFrozenMetadata(metadata, `scan target ${contract.label}`)
      const canonicalPath = await realpath(contract.requestedPath)
      assert.equal(
        canonicalPath,
        contract.requestedPath,
        `scan target ${contract.label} must not traverse a symlinked ancestor`,
      )
      targets.push({
        kind: contract.kind,
        label: contract.label,
        path: canonicalPath,
        relativePath: contract.relativePath,
      })
    }
    for (const [index, target] of targets.entries()) {
      assert.ok(
        targets.every(
          (candidate, candidateIndex) =>
            candidateIndex === index ||
            (!isWithin(target.path, candidate.path) && !isWithin(candidate.path, target.path)),
        ),
        'canonical scan target paths must not overlap',
      )
    }
    return targets
  }

  const assertElfExecutable = async (path, label) => {
    const metadata = await lstat(path)
    assert.ok(metadata.isFile() && !metadata.isSymbolicLink(), `${label} must be a regular file`)
    assertFrozenMetadata(metadata, label)
    assert.notEqual(metadata.mode & 0o111, 0, `${label} must retain an executable bit`)
    const body = await readFrozenFile(path)
    assert.ok(
      body.subarray(0, 4).equals(Buffer.from([0x7f, 0x45, 0x4c, 0x46])),
      `${label} must be ELF`,
    )
  }

  const validateScanTargetSemantics = async (targets) => {
    const byLabel = new Map(targets.map((target) => [target.label, target]))

    const buildArtifacts = byLabel.get('build_artifacts').path
    await assertElfExecutable(join(buildArtifacts, 'nodecontroll-master'), 'Master build artifact')
    await assertElfExecutable(join(buildArtifacts, 'nodecontroll-agent'), 'Agent build artifact')

    const database = byLabel.get('database').path
    const databaseBody = await readFrozenFile(database)
    assert.ok(
      databaseBody.subarray(0, 16).equals(Buffer.from('SQLite format 3\0', 'utf8')),
      'browser database must be a SQLite database',
    )
    await requireAbsent(`${database}-wal`, 'frozen SQLite WAL')
    await requireAbsent(`${database}-shm`, 'frozen SQLite shared-memory file')
    await requireAbsent(`${database}-journal`, 'frozen SQLite rollback journal')

    const databaseDump = (await readFrozenFile(byLabel.get('database_dump').path)).toString('utf8')
    assert.match(databaseDump, /BEGIN TRANSACTION;/u, 'database dump must begin a transaction')
    assert.match(
      databaseDump,
      /CREATE TABLE "?auth_sessions"?/u,
      'database dump must contain sessions',
    )
    assert.match(
      databaseDump,
      /CREATE TABLE "?login_security_events"?/u,
      'database dump must contain security events',
    )
    assert.match(
      databaseDump,
      /INSERT INTO "?auth_sessions"?/u,
      'database dump must contain the browser session rows',
    )
    assert.match(
      databaseDump,
      /INSERT INTO "?login_security_events"?/u,
      'database dump must contain authentication audit rows',
    )
    assert.match(databaseDump, /COMMIT;/u, 'database dump must commit its transaction')

    const openapiBody = await readFrozenFile(byLabel.get('openapi').path)
    const openapi = JSON.parse(openapiBody.toString('utf8'))
    assert.match(
      openapi.openapi,
      /^3\.[0-9]+\.[0-9]+$/u,
      'OpenAPI artifact must declare version 3.x',
    )
    for (const requiredPath of [
      '/api/v1/auth/login',
      '/api/v1/auth/reauth',
      '/api/v1/me',
      '/api/v1/me/sessions/{session_id}',
    ]) {
      assert.ok(openapi.paths?.[requiredPath], `OpenAPI artifact is missing ${requiredPath}`)
    }

    const runtimeLogs = byLabel.get('runtime_logs').path
    const masterRuntimeLog = await readFrozenFile(join(runtimeLogs, 'master-runtime.log'))
    assert.ok(masterRuntimeLog.byteLength > 0, 'frozen Master runtime log must not be empty')
    const runtimeLogFiles = await regularFiles(runtimeLogs)
    assert.ok(
      runtimeLogFiles.every(
        (path) =>
          !path.endsWith('.capturing') && !path.endsWith('.temporary') && !path.endsWith('.tmp'),
      ),
      'runtime log target must not contain unfinished files',
    )

    const testArtifacts = byLabel.get('test_artifacts').path
    const frozenCertificate = await readFrozenFile(join(testArtifacts, 'tls-certificate.pem'))
    assert.deepEqual(
      frozenCertificate,
      tlsCertificate,
      'frozen browser certificate must match the certificate used by the HTTPS proxy',
    )
    const attestation = JSON.parse(
      (await readFrozenFile(join(testArtifacts, 'gate-attestation.json'))).toString('utf8'),
    )
    assert.deepEqual(
      Object.keys(attestation).sort(),
      ['browser_image_digest', 'run_id', 'source_revision'],
      'gate attestation must contain only the host-bound provenance fields',
    )
    assert.deepEqual(
      attestation,
      {
        browser_image_digest: browserImageDigest,
        run_id: runId,
        source_revision: sourceRevision,
      },
      'gate attestation does not match the executing run',
    )

    const webDist = byLabel.get('web_dist').path
    assert.equal(webDist, distRoot, 'web_dist must be the bundle served by the HTTPS proxy')
    const webFiles = await regularFiles(webDist)
    assert.ok(webFiles.includes(join(webDist, 'index.html')), 'web dist must contain index.html')
    assert.ok(
      webFiles.some((path) => extname(path) === '.js'),
      'web dist must contain JavaScript',
    )
    assert.ok(
      (await readFrozenFile(join(webDist, 'index.html'))).byteLength > 0,
      'index.html is empty',
    )
  }

  await startHttpsServer()
  await waitForUpstreamReadiness()
  browser = await chromium.launch({ executablePath: browserExecutableCanonical, headless: true })
  browserContext = await browser.newContext({
    baseURL: baseUrl.href,
    ignoreHTTPSErrors: true,
  })
  const context = browserContext

  const bootstrap = await context.request.post('/api/v1/bootstrap', {
    data: { instance_name: instanceName, password, username },
    headers: { origin: baseUrl.origin, 'x-nodecontroll-setup-token': setupToken },
  })
  assert.equal(bootstrap.status(), 201, `bootstrap failed with ${bootstrap.status()}`)

  const page = await context.newPage()
  const browserConsoleMessages = []
  const browserPageErrors = []
  const browserRequestUrls = []
  page.on('console', (message) => browserConsoleMessages.push(message.text()))
  page.on('pageerror', (error) => browserPageErrors.push(error.stack || error.message))
  page.on('request', (request) => browserRequestUrls.push(request.url()))
  await page.goto('/login', { waitUntil: 'domcontentloaded' })
  await page.getByLabel('用户名').fill(username)
  await page.getByLabel('密码').fill(password)
  const loginStartedAtMs = Date.now()
  const loginResponsePromise = page.waitForResponse(
    (response) =>
      response.request().method() === 'POST' &&
      new URL(response.url()).pathname === '/api/v1/auth/login',
  )
  await page.getByRole('button', { exact: true, name: '登录' }).click()
  const loginResponse = await loginResponsePromise
  const loginReceivedAtMs = Date.now()
  assert.equal(loginResponse.status(), 200, 'password login API must return 200')
  await page.waitForURL((url) => url.pathname === '/', { timeout: 15_000 })

  const loginCookies = cookiePair(await context.cookies(baseUrl.href), 'password login')
  knownSecrets.push(loginCookies.session.value, loginCookies.csrf.value)
  const loginResponseCookies = responseCookiePair(
    await loginResponse.headersArray(),
    loginCookies,
    'password login response',
  )
  const loginProjection = await pageFetch(page, '/api/v1/me')
  assert.equal(loginProjection.status, 200, 'new password session must be active')
  const loginSessionId = loginProjection.body?.data?.session?.id
  assert.equal(typeof loginSessionId, 'string', 'login session projection must contain an ID')
  const loginAbsoluteExpiresAtMs = loginProjection.body?.data?.session?.absolute_expires_at_ms
  assert.ok(
    Number.isSafeInteger(loginAbsoluteExpiresAtMs),
    'login session projection must contain a safe absolute deadline',
  )
  assert.ok(
    loginResponseCookies.maxAgeSeconds <=
      Math.floor((loginAbsoluteExpiresAtMs - loginStartedAtMs) / 1_000),
    'login Max-Age must not outlive the projected server absolute deadline',
  )
  const nowEpochSeconds = Date.now() / 1_000
  for (const cookie of [loginCookies.session, loginCookies.csrf]) {
    assert.ok(cookie.expires > nowEpochSeconds, 'login cookie must have a future persistent expiry')
    assert.ok(
      cookie.expires >= loginStartedAtMs / 1_000 + loginResponseCookies.maxAgeSeconds - 1,
      'login browser expiry must represent response Max-Age',
    )
    assert.ok(
      cookie.expires <= loginReceivedAtMs / 1_000 + loginResponseCookies.maxAgeSeconds + 1,
      'login browser expiry must stay within response Max-Age tolerance',
    )
    assert.ok(
      cookie.expires <= loginAbsoluteExpiresAtMs / 1_000 + 1,
      'login cookie must not outlive the projected server absolute deadline',
    )
  }

  await page.goto('/reauth?redirect=/profile/security', { waitUntil: 'domcontentloaded' })
  await page.getByLabel('当前密码').fill(password)
  const reauthenticationStartedAtMs = Date.now()
  const reauthenticationResponsePromise = page.waitForResponse(
    (response) =>
      response.request().method() === 'POST' &&
      new URL(response.url()).pathname === '/api/v1/auth/reauth',
  )
  await page.getByRole('button', { exact: true, name: '确认身份' }).click()
  const reauthenticationResponse = await reauthenticationResponsePromise
  const reauthenticationReceivedAtMs = Date.now()
  assert.equal(reauthenticationResponse.status(), 200, 'reauthentication API must return 200')
  await page.waitForURL((url) => url.pathname === '/profile/security', { timeout: 15_000 })

  const rotatedCookies = cookiePair(await context.cookies(baseUrl.href), 'reauthentication')
  knownSecrets.push(rotatedCookies.session.value, rotatedCookies.csrf.value)
  const rotatedResponseCookies = responseCookiePair(
    await reauthenticationResponse.headersArray(),
    rotatedCookies,
    'reauthentication response',
  )
  assert.ok(
    rotatedCookies.session.value !== loginCookies.session.value,
    'reauthentication must rotate the session token',
  )
  assert.ok(
    rotatedCookies.csrf.value !== loginCookies.csrf.value,
    'reauthentication must rotate the CSRF token',
  )
  for (const cookie of [rotatedCookies.session, rotatedCookies.csrf]) {
    assert.ok(
      cookie.expires > nowEpochSeconds,
      'rotated cookie must have a future persistent expiry',
    )
    assert.ok(
      cookie.expires >=
        reauthenticationStartedAtMs / 1_000 + rotatedResponseCookies.maxAgeSeconds - 1,
      'rotated browser expiry must represent response Max-Age',
    )
    assert.ok(
      cookie.expires <=
        reauthenticationReceivedAtMs / 1_000 + rotatedResponseCookies.maxAgeSeconds + 1,
      'rotated browser expiry must stay within response Max-Age tolerance',
    )
    assert.ok(
      cookie.expires <= loginAbsoluteExpiresAtMs / 1_000 + 1,
      'rotated cookie must not outlive the projected server absolute deadline',
    )
  }
  assert.ok(
    rotatedCookies.session.expires <= loginCookies.session.expires + 1,
    'rotation must not extend the browser session beyond the original absolute deadline',
  )
  assert.ok(
    rotatedCookies.csrf.expires <= loginCookies.csrf.expires + 1,
    'rotation must not extend the CSRF cookie beyond the original absolute deadline',
  )

  const rotatedProjection = await pageFetch(page, '/api/v1/me')
  assert.equal(rotatedProjection.status, 200, 'rotated browser credentials must be active')
  const rotatedSessionId = rotatedProjection.body?.data?.session?.id
  assert.equal(typeof rotatedSessionId, 'string', 'rotated projection must contain a session ID')
  const stableRotatedSessionProjection = globalThis.structuredClone(
    rotatedProjection.body?.data?.session,
  )
  assert.notEqual(rotatedSessionId, loginSessionId, 'rotation must create a new session row')
  assert.equal(
    rotatedProjection.body?.data?.session?.absolute_expires_at_ms,
    loginAbsoluteExpiresAtMs,
    'rotation must preserve the server absolute deadline exactly',
  )
  assert.ok(
    rotatedResponseCookies.maxAgeSeconds <=
      Math.floor((loginAbsoluteExpiresAtMs - reauthenticationStartedAtMs) / 1_000),
    'rotated Max-Age must not outlive the inherited server absolute deadline',
  )
  assert.ok(
    rotatedResponseCookies.maxAgeSeconds <= loginResponseCookies.maxAgeSeconds,
    'rotation must not increase browser Max-Age',
  )

  const browserState = await page.evaluate(async () => {
    const entries = (storage) =>
      Array.from({ length: storage.length }, (_, index) => {
        const key = storage.key(index)
        return [key, key === null ? null : storage.getItem(key)]
      })
    return {
      bodyText: globalThis.document.body.textContent,
      cacheNames: await globalThis.caches.keys(),
      cookie: globalThis.document.cookie,
      documentHtml: globalThis.document.documentElement.outerHTML,
      formValues: Array.from(globalThis.document.querySelectorAll('input'), (input) => input.value),
      indexedDatabases:
        typeof globalThis.indexedDB.databases === 'function'
          ? await globalThis.indexedDB.databases()
          : [],
      localStorage: entries(globalThis.localStorage).sort(([left], [right]) =>
        String(left).localeCompare(String(right), 'en'),
      ),
      performanceEntryUrls: globalThis.performance.getEntries().map((entry) => entry.name),
      sessionStorage: entries(globalThis.sessionStorage).sort(([left], [right]) =>
        String(left).localeCompare(String(right), 'en'),
      ),
      url: globalThis.location.href,
    }
  })
  assert.ok(
    browserState.cookie === `${csrfCookieName}=${rotatedCookies.csrf.value}`,
    'document.cookie may expose only the intended CSRF credential',
  )
  const rotatedCoordinationJournal = assertCredentialCoordinationJournal(
    browserState.localStorage,
    'reauthenticated browser state',
  )
  assert.equal(rotatedCoordinationJournal.record.phase, 'settled')
  assert.equal(rotatedCoordinationJournal.record.disposition, 'reconcile')
  assert.equal(rotatedCoordinationJournal.record.operation, 'reauth')
  assert.equal(
    browserState.sessionStorage.length,
    0,
    'authentication state must not use sessionStorage',
  )
  assert.equal(browserState.cacheNames.length, 0, 'authentication state must not use CacheStorage')
  assert.equal(
    browserState.indexedDatabases.length,
    0,
    'authentication state must not use IndexedDB',
  )

  await context.clearCookies()
  await context.addCookies([loginCookies.session, loginCookies.csrf])
  const oldCredentialResponsePromise = page.waitForResponse(
    (response) =>
      response.request().method() === 'GET' && new URL(response.url()).pathname === '/api/v1/me',
  )
  const [oldCredentialResponse, oldCredentialResult] = await Promise.all([
    oldCredentialResponsePromise,
    pageFetch(page, '/api/v1/me'),
  ])
  assert.equal(oldCredentialResult.status, 401, 'pre-rotation browser credentials must be invalid')
  assert.equal(oldCredentialResponse.status(), oldCredentialResult.status)
  const oldCredentialSetCookieFields = assertNoSetCookie(
    await oldCredentialResponse.headersArray(),
    'pre-rotation invalid-session response',
  )
  assert.equal(oldCredentialResult.body?.code, 'SESSION_INVALID')
  assert.equal(oldCredentialResult.body?.status, 401)
  assert.equal(typeof oldCredentialResult.body?.type, 'string')
  assert.ok(oldCredentialResult.body.type.length > 0)
  assertCookiePairUnchanged(
    cookiePair(await context.cookies(baseUrl.href), 'pre-rotation invalid-session response'),
    loginCookies,
    'pre-rotation invalid-session response',
  )

  await context.clearCookies()
  assert.equal(
    (await context.cookies(baseUrl.href)).filter(
      (cookie) => cookie.name === sessionCookieName || cookie.name === csrfCookieName,
    ).length,
    0,
    'old-credential probe context must be cleaned explicitly for test isolation',
  )

  await context.addCookies([rotatedCookies.session, loginCookies.csrf])
  const oldCsrfResponsePromise = page.waitForResponse(
    (response) =>
      response.request().method() === 'POST' &&
      new URL(response.url()).pathname === '/api/v1/auth/reauth',
  )
  const [oldCsrfResponse, oldCsrfResult] = await Promise.all([
    oldCsrfResponsePromise,
    pageReauthenticationAttempt(page, loginCookies.csrf.value, intentionallyWrongProof),
  ])
  assert.equal(oldCsrfResult.status, 403, 'pre-rotation CSRF credential must be rejected')
  assert.equal(oldCsrfResult.body?.code, 'CSRF_INVALID')
  assert.equal(oldCsrfResponse.status(), oldCsrfResult.status)
  assertNoAuthenticationSetCookie(await oldCsrfResponse.headersArray(), 'old CSRF response')
  assertCookiePairUnchanged(
    cookiePair(await context.cookies(baseUrl.href), 'old CSRF rejection'),
    { csrf: loginCookies.csrf, session: rotatedCookies.session },
    'old CSRF rejection',
  )
  assertSessionProjectionUnchanged(
    await pageFetch(page, '/api/v1/me'),
    stableRotatedSessionProjection,
    'old CSRF rejection',
  )

  await context.clearCookies()
  await context.addCookies([rotatedCookies.session, rotatedCookies.csrf])
  const newCsrfResponsePromise = page.waitForResponse(
    (response) =>
      response.request().method() === 'POST' &&
      new URL(response.url()).pathname === '/api/v1/auth/reauth',
  )
  const [newCsrfResponse, newCsrfResult] = await Promise.all([
    newCsrfResponsePromise,
    pageReauthenticationAttempt(page, rotatedCookies.csrf.value, intentionallyWrongProof),
  ])
  assert.equal(newCsrfResult.status, 403, 'new CSRF credential must pass before proof rejection')
  assert.equal(newCsrfResult.body?.code, 'REAUTHENTICATION_FAILED')
  assert.equal(newCsrfResponse.status(), newCsrfResult.status)
  assertNoAuthenticationSetCookie(await newCsrfResponse.headersArray(), 'wrong-proof response')
  assertCookiePairUnchanged(
    cookiePair(await context.cookies(baseUrl.href), 'wrong-proof rejection'),
    rotatedCookies,
    'wrong-proof rejection',
  )
  const newCredentialResult = await pageFetch(page, '/api/v1/me')
  assertSessionProjectionUnchanged(
    newCredentialResult,
    stableRotatedSessionProjection,
    'wrong-proof rejection',
  )

  const peerPage = await context.newPage()
  const peerBrowserConsoleMessages = []
  const peerBrowserPageErrors = []
  const peerBrowserRequests = []
  peerPage.on('console', (message) => peerBrowserConsoleMessages.push(message.text()))
  peerPage.on('pageerror', (error) => peerBrowserPageErrors.push(error.stack || error.message))
  peerPage.on('request', (request) =>
    peerBrowserRequests.push({ method: request.method(), url: request.url() }),
  )
  await peerPage.goto('/profile/security', { waitUntil: 'domcontentloaded' })
  await peerPage.getByText('登录会话', { exact: true }).waitFor({ timeout: 15_000 })
  await page.getByText('登录会话', { exact: true }).waitFor({ timeout: 15_000 })
  const staleInvalidationCursor = assertCredentialCoordinationJournal(
    (await storageEntries(peerPage)).localStorage,
    'cross-tab stale invalidation cursor',
  )
  assert.equal(staleInvalidationCursor.raw, rotatedCoordinationJournal.raw)
  assert.equal(staleInvalidationCursor.record.phase, 'settled')
  assert.equal(staleInvalidationCursor.record.disposition, 'reconcile')

  const logoutFailureRoute = async (route) => {
    if (
      route.request().method() === 'POST' &&
      new URL(route.request().url()).pathname === '/api/v1/auth/logout'
    ) {
      await route.fulfill({
        body: JSON.stringify({
          code: 'AUTHENTICATION_UNAVAILABLE',
          detail: 'Injected browser-gate logout failure',
          request_id: 'browser-gate-logout-failure',
          status: 503,
          title: 'Authentication unavailable',
          type: 'urn:nodecontroll:problem:authentication-unavailable',
        }),
        contentType: 'application/problem+json',
        headers: { 'cache-control': 'no-store' },
        status: 503,
      })
      return
    }
    await route.continue()
  }
  await peerPage.route('**/api/v1/auth/logout', logoutFailureRoute)
  await peerPage.getByRole('button', { exact: true, name: '账户菜单' }).click()
  const failedLogoutResponsePromise = peerPage.waitForResponse(
    (response) =>
      response.request().method() === 'POST' &&
      new URL(response.url()).pathname === '/api/v1/auth/logout',
  )
  await peerPage.getByText('退出登录', { exact: true }).click()
  const failedLogoutResponse = await failedLogoutResponsePromise
  assert.equal(failedLogoutResponse.status(), 503, 'injected ordinary logout must return 503')
  assertNoAuthenticationSetCookie(
    await failedLogoutResponse.headersArray(),
    'injected ordinary logout failure',
  )
  await Promise.all([
    page.getByTestId('protected-route-session-gate').waitFor({ timeout: 15_000 }),
    peerPage.getByTestId('protected-route-session-gate').waitFor({ timeout: 15_000 }),
  ])
  for (const [label, currentPage] of [
    ['remote page', page],
    ['initiating page', peerPage],
  ]) {
    assert.equal(
      await currentPage.getByText('登录会话', { exact: true }).count(),
      0,
      `${label}: failed ordinary logout must remove protected route content from the DOM`,
    )
  }
  assertCookiePairUnchanged(
    cookiePair(await context.cookies(baseUrl.href), 'failed ordinary logout'),
    rotatedCookies,
    'failed ordinary logout',
  )
  assertSessionProjectionUnchanged(
    await pageFetch(page, '/api/v1/me'),
    stableRotatedSessionProjection,
    'failed ordinary logout',
  )
  await peerPage.waitForFunction(
    (key) => {
      try {
        const record = JSON.parse(globalThis.localStorage.getItem(key) ?? '')
        return (
          record.phase === 'settled' &&
          record.disposition === 'quarantine' &&
          record.operation === 'logout'
        )
      } catch {
        return false
      }
    },
    credentialCoordinationKey,
    { polling: 25, timeout: 15_000 },
  )
  const failedLogoutJournal = assertCredentialCoordinationJournal(
    (await storageEntries(peerPage)).localStorage,
    'failed logout quarantine',
  )
  assert.equal(failedLogoutJournal.record.phase, 'settled')
  assert.equal(failedLogoutJournal.record.disposition, 'quarantine')
  assert.equal(failedLogoutJournal.record.operation, 'logout')
  const failedLogoutHtml = await page.content()
  const failedLogoutInitiatorHtml = await peerPage.content()
  assert.ok(
    !failedLogoutHtml.includes('登录会话') && !failedLogoutInitiatorHtml.includes('登录会话'),
    'failed ordinary logout HTML must not retain protected content in either page',
  )
  await peerPage.unroute('**/api/v1/auth/logout', logoutFailureRoute)

  const peerRequestCount = (method, pathname) =>
    peerBrowserRequests.filter(
      (request) => request.method === method && new URL(request.url).pathname === pathname,
    ).length
  const meReadsBeforeQuarantineReload = peerRequestCount('GET', '/api/v1/me')
  const loginPostsBeforeQuarantineReload = peerRequestCount('POST', '/api/v1/auth/login')
  await peerPage.reload({ waitUntil: 'domcontentloaded' })
  await peerPage.waitForURL((url) => url.pathname === '/login', { timeout: 15_000 })
  await peerPage.getByLabel('用户名').waitFor({ timeout: 15_000 })
  const quarantineReloadAuthenticationReads =
    peerRequestCount('GET', '/api/v1/me') - meReadsBeforeQuarantineReload
  const quarantineReloadLoginRequests =
    peerRequestCount('POST', '/api/v1/auth/login') - loginPostsBeforeQuarantineReload
  assert.equal(
    quarantineReloadAuthenticationReads,
    0,
    'quarantine reload must not attempt automatic credential recovery',
  )
  assert.equal(
    quarantineReloadLoginRequests,
    0,
    'quarantine reload must not attempt an automatic login',
  )
  assert.equal(
    await peerPage.getByRole('button', { exact: true, name: '账户菜单' }).count(),
    0,
    'quarantine reload must not restore the authenticated shell',
  )
  const reloadedQuarantineJournal = assertCredentialCoordinationJournal(
    (await storageEntries(peerPage)).localStorage,
    'reloaded failed logout quarantine',
  )
  assert.equal(
    reloadedQuarantineJournal.raw,
    failedLogoutJournal.raw,
    'failed logout quarantine journal must persist unchanged across reload',
  )
  assertCookiePairUnchanged(
    cookiePair(await context.cookies(baseUrl.href), 'reloaded failed logout quarantine'),
    rotatedCookies,
    'reloaded failed logout quarantine',
  )
  const quarantineReloadHtml = await peerPage.content()
  assert.ok(
    !quarantineReloadHtml.includes('登录会话'),
    'quarantine reload must keep protected content out of the DOM',
  )

  await peerPage.getByLabel('用户名').fill(username)
  await peerPage.getByLabel('密码').fill(password)
  const recoveryLoginResponsePromise = peerPage.waitForResponse(
    (response) =>
      response.request().method() === 'POST' &&
      new URL(response.url()).pathname === '/api/v1/auth/login',
  )
  await peerPage.getByRole('button', { exact: true, name: '登录' }).click()
  const recoveryLoginResponse = await recoveryLoginResponsePromise
  assert.equal(
    recoveryLoginResponse.status(),
    200,
    'explicit quarantine recovery login must succeed',
  )
  await peerPage.waitForURL((url) => url.pathname === '/profile/security', { timeout: 15_000 })
  const recoveryCookies = cookiePair(await context.cookies(baseUrl.href), 'explicit recovery login')
  knownSecrets.push(recoveryCookies.session.value, recoveryCookies.csrf.value)
  responseCookiePair(
    await recoveryLoginResponse.headersArray(),
    recoveryCookies,
    'explicit recovery login response',
  )
  const recoveredCredentialResult = await pageFetch(peerPage, '/api/v1/me')
  assert.equal(
    recoveredCredentialResult.status,
    200,
    'explicit recovery credentials must be active',
  )
  const recoverySessionId = recoveredCredentialResult.body?.data?.session?.id
  assert.equal(typeof recoverySessionId, 'string', 'explicit recovery must project a session ID')
  assert.notEqual(
    recoverySessionId,
    rotatedSessionId,
    'explicit recovery must establish a newer session',
  )
  await Promise.all([
    page.getByText('登录会话', { exact: true }).waitFor({ timeout: 15_000 }),
    peerPage.getByText('登录会话', { exact: true }).waitFor({ timeout: 15_000 }),
  ])
  const recoveredJournal = assertCredentialCoordinationJournal(
    (await storageEntries(page)).localStorage,
    'explicit cross-tab recovery',
  )
  assert.equal(recoveredJournal.record.phase, 'settled')
  assert.equal(recoveredJournal.record.disposition, 'reconcile')
  assert.equal(recoveredJournal.record.operation, 'login')

  const staleMessageIdentifiers = await peerPage.evaluate(() => ({
    eventId: globalThis.crypto.randomUUID(),
    senderId: globalThis.crypto.randomUUID(),
  }))
  const staleInvalidationMessage = {
    baseEpoch: staleInvalidationCursor.record.epoch,
    baseSeq: staleInvalidationCursor.record.seq,
    eventId: staleMessageIdentifiers.eventId,
    kind: 'observed-invalid',
    observedSessionId: rotatedSessionId,
    senderId: staleMessageIdentifiers.senderId,
    v: 1,
  }
  assert.deepEqual(Object.keys(staleInvalidationMessage).sort(), [
    'baseEpoch',
    'baseSeq',
    'eventId',
    'kind',
    'observedSessionId',
    'senderId',
    'v',
  ])
  assert.match(staleInvalidationMessage.baseEpoch, credentialRandomIdentifierPattern)
  credentialSequence(staleInvalidationMessage.baseSeq, 'late stale invalidation baseSeq')
  assert.match(staleInvalidationMessage.eventId, credentialRandomIdentifierPattern)
  assert.equal(staleInvalidationMessage.kind, 'observed-invalid')
  assert.match(staleInvalidationMessage.observedSessionId, credentialIdentifierPattern)
  assert.match(staleInvalidationMessage.senderId, credentialRandomIdentifierPattern)
  assert.equal(staleInvalidationMessage.v, 1)
  assert.equal(
    staleInvalidationMessage.baseEpoch,
    recoveredJournal.record.epoch,
    'late invalidation must be stale by sequence within the current epoch',
  )
  assert.ok(
    BigInt(staleInvalidationMessage.baseSeq) < BigInt(recoveredJournal.record.seq),
    'late invalidation baseSeq must predate the explicit recovery journal',
  )
  await page.evaluate(
    ({ channelName }) => {
      globalThis.__nodecontrollE2eStaleObserver?.close()
      globalThis.__nodecontrollE2eStaleDeliveries = []
      const observer = new globalThis.BroadcastChannel(channelName)
      observer.addEventListener('message', (event) => {
        if (event.data?.kind === 'observed-invalid' && typeof event.data?.eventId === 'string') {
          globalThis.__nodecontrollE2eStaleDeliveries.push(event.data.eventId)
        }
      })
      globalThis.__nodecontrollE2eStaleObserver = observer
    },
    { channelName: credentialCoordinationKey },
  )
  await peerPage.evaluate(
    ({ channelName, message }) => {
      const sender = new globalThis.BroadcastChannel(channelName)
      sender.postMessage(message)
      sender.close()
    },
    { channelName: credentialCoordinationKey, message: staleInvalidationMessage },
  )
  await page.waitForFunction(
    (eventId) => globalThis.__nodecontrollE2eStaleDeliveries?.includes(eventId) === true,
    staleInvalidationMessage.eventId,
    { polling: 25, timeout: 5_000 },
  )
  await page.evaluate(() => {
    globalThis.__nodecontrollE2eStaleObserver?.close()
    delete globalThis.__nodecontrollE2eStaleObserver
    delete globalThis.__nodecontrollE2eStaleDeliveries
  })
  await Promise.all(
    [page, peerPage].map((currentPage) =>
      currentPage.evaluate(
        () =>
          new Promise((resolveTimer) => {
            globalThis.setTimeout(resolveTimer, 50)
          }),
      ),
    ),
  )
  const staleInvalidationDelivered = true
  assert.equal(
    staleInvalidationDelivered,
    true,
    'late stale invalidation must be delivered to the other real page',
  )
  const credentialAfterStaleInvalidation = await pageFetch(page, '/api/v1/me')
  assert.equal(
    credentialAfterStaleInvalidation.status,
    200,
    'late stale invalidation must not clear the newer credentials',
  )
  assert.equal(
    credentialAfterStaleInvalidation.body?.data?.session?.id,
    recoverySessionId,
    'late stale invalidation must preserve the newer session identity',
  )
  assert.equal(
    (await storageEntries(page)).localStorage[0]?.[1],
    recoveredJournal.raw,
    'late stale invalidation must not replace the newer coordination journal',
  )
  for (const [label, currentPage] of [
    ['primary page', page],
    ['peer page', peerPage],
  ]) {
    assert.equal(
      await currentPage.getByTestId('protected-route-session-gate').count(),
      0,
      `${label}: late stale invalidation must not close the recovered protected DOM`,
    )
    await currentPage.getByText('登录会话', { exact: true }).waitFor({ timeout: 15_000 })
  }

  const primaryRecoveredStorage = await storageEntries(page)
  const peerRecoveredStorage = await storageEntries(peerPage)
  const primaryRecoveredJournal = assertCredentialCoordinationJournal(
    primaryRecoveredStorage.localStorage,
    'primary recovered page storage',
  )
  const peerRecoveredJournal = assertCredentialCoordinationJournal(
    peerRecoveredStorage.localStorage,
    'peer recovered page storage',
  )
  assert.equal(primaryRecoveredJournal.raw, peerRecoveredJournal.raw)
  assert.equal(primaryRecoveredStorage.sessionStorage.length, 0)
  assert.equal(peerRecoveredStorage.sessionStorage.length, 0)
  const peerRecoveredHtml = await peerPage.content()
  await peerPage.close()

  await page.getByRole('button', { exact: true, name: '账户菜单' }).waitFor({ timeout: 15_000 })
  await page.getByRole('button', { exact: true, name: '账户菜单' }).click()
  const logoutResponsePromise = page.waitForResponse(
    (response) =>
      response.request().method() === 'POST' &&
      new URL(response.url()).pathname === '/api/v1/auth/logout',
  )
  await page.getByText('退出登录', { exact: true }).click()
  const logoutResponse = await logoutResponsePromise
  assert.equal(logoutResponse.status(), 204, 'ordinary logout API must return 204')
  assertClearingResponseCookiePair(await logoutResponse.headersArray(), 'ordinary logout response')
  await page.waitForURL((url) => url.pathname === '/login', { timeout: 15_000 })
  assert.equal(
    (await context.cookies(baseUrl.href)).filter(
      (cookie) => cookie.name === sessionCookieName || cookie.name === csrfCookieName,
    ).length,
    0,
    'ordinary logout must clear both browser credentials',
  )
  const postLogoutCredentialResult = await pageFetch(page, '/api/v1/me')
  assert.equal(postLogoutCredentialResult.status, 401, 'logged-out credentials must remain invalid')
  const postLogoutHtml = await page.content()
  assert.ok(
    !postLogoutHtml.includes('登录会话'),
    'successful ordinary logout HTML must not retain protected content',
  )
  const finalBrowserStorage = await storageEntries(page)
  const finalCoordinationJournal = assertCredentialCoordinationJournal(
    finalBrowserStorage.localStorage,
    'successful explicit logout storage',
  )
  assert.equal(finalCoordinationJournal.record.phase, 'settled')
  assert.equal(finalCoordinationJournal.record.disposition, 'reconcile')
  assert.equal(finalCoordinationJournal.record.operation, 'logout')
  assert.equal(
    finalBrowserStorage.sessionStorage.length,
    0,
    'successful explicit logout must leave sessionStorage empty',
  )

  const persistentBrowserStateText = JSON.stringify({
    bodyText: browserState.bodyText,
    browserConsoleMessages,
    browserPageErrors,
    browserRequestUrls,
    cacheNames: browserState.cacheNames,
    documentHtml: browserState.documentHtml,
    failedLogoutInitiatorHtml,
    failedLogoutHtml,
    finalBrowserStorage,
    formValues: browserState.formValues,
    indexedDatabases: browserState.indexedDatabases,
    localStorage: browserState.localStorage,
    performanceEntryUrls: browserState.performanceEntryUrls,
    peerBrowserConsoleMessages,
    peerBrowserPageErrors,
    peerBrowserRequests,
    peerRecoveredHtml,
    postLogoutHtml,
    primaryRecoveredStorage,
    quarantineReloadHtml,
    sessionStorage: browserState.sessionStorage,
    url: browserState.url,
  })
  for (const forbidden of distinctTextualSecretRepresentations(knownSecrets)) {
    assert.ok(
      !persistentBrowserStateText.includes(forbidden),
      'browser DOM, attributes, console, URLs, or persistent storage exposed forbidden secret',
    )
  }

  const browserVersion = browser.version()
  await closeBrowserContext()
  await closeBrowser()
  if (serverRuntimeError) throw serverRuntimeError
  await closeServer()
  if (serverRuntimeError) throw serverRuntimeError
  await publishNewFile(behaviorReadyFile, `${runId}\n`)
  const behaviorReadyMetadata = await lstat(behaviorReadyFile)
  const scanReadyMetadata = await waitForReadyMarker(scanReadyFile, `${runId}\n`)
  assert.ok(
    scanReadyMetadata.mtimeMs >= behaviorReadyMetadata.mtimeMs,
    'scan-ready marker must be published after behavior-ready',
  )
  assert.equal(
    await upstreamReadinessStatus(),
    0,
    'scan-ready requires the E2E Master listener to be fully stopped',
  )

  const scanTargets = await prepareCanonicalScanTargets()
  const secretCanaries = distinctSecretBuffers(knownSecrets)
  const secretScan = await scanFilesForSecrets(scanTargets, secretCanaries)
  await validateScanTargetSemantics(scanTargets)
  const verifiedSecretScan = await scanFilesForSecrets(scanTargets, secretCanaries)
  assert.deepEqual(
    verifiedSecretScan,
    secretScan,
    'frozen scan targets changed during semantic validation and secret scanning',
  )

  const evidence = {
    browserExecutable: {
      bytes: browserExecutableMetadata.size,
      name: basename(browserExecutableCanonical),
      sha256: browserExecutableSha256,
    },
    browserImageDigest,
    browserVersion,
    certificate: {
      derSha256: tlsCertificateDerSha256,
      ipSan: baseUrl.hostname,
      keyPairMatched: true,
      pemSha256: tlsCertificatePemSha256,
      currentlyValid: true,
      validFrom: tlsCertificateObject.validFrom,
      validTo: tlsCertificateObject.validTo,
    },
    cookieContract: {
      csrfHttpOnly: rotatedCookies.csrf.httpOnly,
      host: baseUrl.hostname,
      loginMaxAgeSeconds: loginResponseCookies.maxAgeSeconds,
      rotatedMaxAgeSeconds: rotatedResponseCookies.maxAgeSeconds,
      sameSite: rotatedCookies.session.sameSite,
      secure: rotatedCookies.session.secure,
      sessionHttpOnly: rotatedCookies.session.httpOnly,
    },
    crossTabContract: {
      explicitLoginRecoveredBothPages: true,
      lateStaleInvalidationBoundToOldCursor: true,
      lateStaleInvalidationDelivered: staleInvalidationDelivered,
      lateStaleInvalidationIgnored: true,
      lateStaleInvalidationStructurallyValid: true,
      newerCredentialStatus: credentialAfterStaleInvalidation.status,
      newerProtectedDomRetainedAcrossPages: true,
      newerSessionPreserved: true,
      pages: 2,
      remoteMutationProtectedDomClosed: true,
    },
    loginSessionId,
    gateScriptSha256,
    failedMutationContract: {
      authenticationSetCookieFields: 0,
      cookiesUnchanged: true,
      sessionProjectionUnchanged: true,
    },
    logoutFailureContract: {
      authenticationSetCookieFields: 0,
      automaticAuthenticationReadsAfterReload: quarantineReloadAuthenticationReads,
      automaticLoginRequestsAfterReload: quarantineReloadLoginRequests,
      coordinationDisposition: failedLogoutJournal.record.disposition,
      coordinationPhase: failedLogoutJournal.record.phase,
      cookiesUnchanged: true,
      protectedDomClosed: true,
      quarantinePersistedAcrossReload: true,
      recoveredByExplicitLogin: true,
      recoveryLoginStatus: recoveryLoginResponse.status(),
      sessionProjectionUnchanged: true,
      status: failedLogoutResponse.status(),
    },
    logoutContract: {
      coordinationJournalSettled: true,
      cookiesCleared: true,
      protectedDomClosed: true,
      status: logoutResponse.status(),
    },
    newCsrfStatus: newCsrfResult.status,
    oldCsrfStatus: oldCsrfResult.status,
    oldCredentialContract: {
      browserCookiesUnchanged: true,
      contextCleanedAfterProbe: true,
      setCookieFields: oldCredentialSetCookieFields,
      status: oldCredentialResult.status,
    },
    oldCredentialStatus: oldCredentialResult.status,
    postLogoutCredentialStatus: postLogoutCredentialResult.status,
    recoveredCredentialStatus: recoveredCredentialResult.status,
    recoverySessionId,
    rotatedSessionId,
    rotatedCredentialStatus: newCredentialResult.status,
    scannedArtifactBytes: secretScan.scannedBytes,
    scannedArtifactFiles: secretScan.scannedFiles,
    scanTargets: secretScan.targets,
    storage: {
      cacheNames: browserState.cacheNames.length,
      indexedDatabases: browserState.indexedDatabases.length,
      journal: {
        disposition: finalCoordinationJournal.record.disposition,
        fieldNames: Object.keys(finalCoordinationJournal.record).sort(),
        key: credentialCoordinationKey,
        operation: finalCoordinationJournal.record.operation,
        phase: finalCoordinationJournal.record.phase,
        strictNonSecretSchema: true,
        version: finalCoordinationJournal.record.v,
      },
      localStorageEntriesByPage: [
        primaryRecoveredStorage.localStorage.length,
        peerRecoveredStorage.localStorage.length,
      ],
      localStorageKeys: finalBrowserStorage.localStorage.map(([key]) => key),
      pagesChecked: 2,
      sessionStorageEntriesByPage: [
        primaryRecoveredStorage.sessionStorage.length,
        peerRecoveredStorage.sessionStorage.length,
      ],
    },
    nodeVersion: process.version,
    playwrightVersion: playwrightPackage.version,
    runId,
    scanContract: scanTargetContract.map(({ kind, label, relativePath }) => ({
      kind,
      label,
      path: relativePath,
    })),
    sourceRevision,
    test: 'nodecontroll-auth-c1-https-v3',
  }
  const serializedEvidence = `${JSON.stringify(evidence, null, 2)}\n`
  const serializedEvidenceBytes = Buffer.from(serializedEvidence, 'utf8')
  for (const secret of secretCanaries) {
    assert.ok(
      !serializedEvidenceBytes.includes(secret),
      'browser evidence must not contain a secret',
    )
  }
  await publishNewFile(evidenceFile, serializedEvidence)
  process.stdout.write(
    `HTTPS browser rotation passed: ${basename(evidenceFile)} (${secretScan.scannedFiles} scanned files)\n`,
  )
} catch (error) {
  testFailure = error
} finally {
  for (const cleanup of [closeBrowserContext, closeBrowser, closeServer]) {
    try {
      await cleanup()
    } catch (error) {
      testFailure ??= error
    }
  }
}

if (testFailure !== undefined) {
  const diagnostic =
    testFailure instanceof Error ? testFailure.stack || testFailure.message : String(testFailure)
  await new Promise((resolveWrite) => {
    process.stderr.write(
      `HTTPS browser rotation failed\n${redactKnownSecrets(diagnostic)}\n`,
      resolveWrite,
    )
  })
  process.exit(1)
}
