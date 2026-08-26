/**
 * Temporary WP02-C2 boundary.
 *
 * The public d200c03 OpenAPI document predates recovery-code endpoints. Keep the temporary types,
 * request path bindings, and runtime validators here so generated code remains untouched. Once the
 * backend OpenAPI lands, reuse its generated response types and path contract, but retain this
 * bounded, redirect-rejecting transport until the generator can provide equivalent guarantees.
 */

const bootstrapPath = '/api/v1/bootstrap'
const recoveryCodesPath = '/api/v1/me/recovery-codes'
const maximumResponseBytes = 64 * 1024
const recoveryCodeCount = 8
const recoveryCodePattern = /^[0-9a-f]{4}(?:-[0-9a-f]{4}){7}$/i
const uuidPattern =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i

type HttpMethod = 'GET' | 'POST'

export type BootstrapRecoveryRequest = {
  instance_name: string
  password: string
  username: string
}

export type BootstrapRecoveryData = {
  instance_id: string
  one_time_recovery_codes: string[]
  owner_id: string
}

export type RecoveryCodeStatus = {
  created_at_ms: number
  remaining_count: number
  set_version: number
  total_count: number
}

export type RegeneratedRecoveryCodes = {
  created_at_ms: number
  one_time_recovery_codes: string[]
  set_version: number
}

export type ApiMeta = {
  api_version: string
  request_id: string
}

export type ApiEnvelope<T> = {
  data: T
  meta: ApiMeta
}

export type TemporaryApiResult<T> = {
  data?: ApiEnvelope<T>
  error?: unknown
  payloadState: 'invalid' | 'valid-json'
  response: Response
}

type ParsedResponseBody =
  | { payloadState: 'invalid' }
  | { payload: unknown; payloadState: 'valid-json' }

type RequestOptions = {
  body?: unknown
  headers?: Record<string, string>
  method: HttpMethod
  path: string
  signal?: AbortSignal
}

const objectRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === 'object' && value !== null

const exactKeys = (candidate: Record<string, unknown>, expected: readonly string[]) => {
  const actual = Object.keys(candidate).sort()
  const sortedExpected = [...expected].sort()
  return (
    actual.length === sortedExpected.length &&
    actual.every((key, index) => key === sortedExpected[index])
  )
}

const nonnegativeSafeInteger = (value: unknown): value is number =>
  typeof value === 'number' && Number.isSafeInteger(value) && value >= 0

const positiveSafeInteger = (value: unknown): value is number =>
  nonnegativeSafeInteger(value) && value > 0

const validMeta = (value: unknown): value is ApiMeta =>
  objectRecord(value) &&
  exactKeys(value, ['api_version', 'request_id']) &&
  typeof value.api_version === 'string' &&
  value.api_version.length >= 1 &&
  value.api_version.length <= 32 &&
  typeof value.request_id === 'string' &&
  value.request_id.length >= 1 &&
  value.request_id.length <= 128

const normalizedRecoveryCode = (value: string) => value.replaceAll('-', '').toUpperCase()

export const validOneTimeRecoveryCodes = (value: unknown): value is string[] => {
  if (
    !Array.isArray(value) ||
    value.length !== recoveryCodeCount ||
    !value.every((candidate) => typeof candidate === 'string' && recoveryCodePattern.test(candidate))
  ) {
    return false
  }
  return new Set(value.map(normalizedRecoveryCode)).size === recoveryCodeCount
}

const validBootstrapData = (value: unknown): value is BootstrapRecoveryData =>
  objectRecord(value) &&
  exactKeys(value, ['instance_id', 'one_time_recovery_codes', 'owner_id']) &&
  typeof value.instance_id === 'string' &&
  uuidPattern.test(value.instance_id) &&
  typeof value.owner_id === 'string' &&
  uuidPattern.test(value.owner_id) &&
  validOneTimeRecoveryCodes(value.one_time_recovery_codes)

const validRecoveryCodeStatus = (value: unknown): value is RecoveryCodeStatus =>
  objectRecord(value) &&
  exactKeys(value, ['created_at_ms', 'remaining_count', 'set_version', 'total_count']) &&
  positiveSafeInteger(value.set_version) &&
  nonnegativeSafeInteger(value.created_at_ms) &&
  value.total_count === recoveryCodeCount &&
  nonnegativeSafeInteger(value.remaining_count) &&
  value.remaining_count <= value.total_count

const validRegeneratedRecoveryCodes = (value: unknown): value is RegeneratedRecoveryCodes =>
  objectRecord(value) &&
  exactKeys(value, ['created_at_ms', 'one_time_recovery_codes', 'set_version']) &&
  positiveSafeInteger(value.set_version) &&
  nonnegativeSafeInteger(value.created_at_ms) &&
  validOneTimeRecoveryCodes(value.one_time_recovery_codes)

const validEnvelope = <T>(
  value: unknown,
  validData: (candidate: unknown) => candidate is T,
): value is ApiEnvelope<T> =>
  objectRecord(value) &&
  exactKeys(value, ['data', 'meta']) &&
  validData(value.data) &&
  validMeta(value.meta)

const responseHasNoStore = (response: Response) =>
  response.headers
    .get('cache-control')
    ?.split(',')
    .some((directive) => directive.trim().toLowerCase() === 'no-store') === true

const responseHasJsonContentType = (response: Response) => {
  const mediaType = response.headers.get('content-type')?.split(';', 1)[0]?.trim().toLowerCase()
  return mediaType === 'application/json'
}

const responseHasProblemContentType = (response: Response) => {
  const mediaType = response.headers.get('content-type')?.split(';', 1)[0]?.trim().toLowerCase()
  return mediaType === 'application/problem+json'
}

const cancelResponseBody = async (response: Response) => {
  try {
    await response.body?.cancel()
  } catch {
    // The response is already unusable. Cancellation is best-effort and never changes that result.
  }
}

const readBoundedJson = async (response: Response): Promise<ParsedResponseBody> => {
  const declaredLength = response.headers.get('content-length')?.trim()
  if (declaredLength && /^\d+$/.test(declaredLength) && Number(declaredLength) > maximumResponseBytes) {
    await cancelResponseBody(response)
    return { payloadState: 'invalid' }
  }

  const reader = response.body?.getReader()
  if (!reader) return { payloadState: 'invalid' }
  const chunks: Uint8Array[] = []
  let byteLength = 0
  try {
    while (true) {
      const chunk = await reader.read()
      if (chunk.done) break
      if (!chunk.value) continue
      byteLength += chunk.value.byteLength
      if (byteLength > maximumResponseBytes) {
        try {
          await reader.cancel()
        } catch {
          // The size violation is already terminal; cancellation remains best-effort.
        }
        return { payloadState: 'invalid' }
      }
      chunks.push(chunk.value)
    }
  } catch {
    try {
      await reader.cancel()
    } catch {
      // A read failure is already terminal; cancellation remains best-effort.
    }
    return { payloadState: 'invalid' }
  } finally {
    reader.releaseLock()
  }

  if (byteLength === 0) {
    return { payloadState: 'invalid' }
  }
  const bytes = new Uint8Array(byteLength)
  let offset = 0
  for (const chunk of chunks) {
    bytes.set(chunk, offset)
    offset += chunk.byteLength
  }
  try {
    const text = new TextDecoder('utf-8', { fatal: true }).decode(bytes)
    return { payload: JSON.parse(text) as unknown, payloadState: 'valid-json' }
  } catch {
    return { payloadState: 'invalid' }
  }
}

const request = async ({ body, headers = {}, method, path, signal }: RequestOptions) => {
  const response = await globalThis.fetch(path, {
    cache: 'no-store',
    credentials: 'same-origin',
    headers: {
      accept: 'application/json, application/problem+json',
      ...(body === undefined ? {} : { 'content-type': 'application/json' }),
      ...headers,
    },
    method,
    redirect: 'error',
    ...(body === undefined ? {} : { body: JSON.stringify(body) }),
    ...(signal === undefined ? {} : { signal }),
  })
  return { body: await readBoundedJson(response), response }
}

const result = <T>(
  response: Response,
  body: ParsedResponseBody,
  successStatus: (status: number) => boolean,
  validData: (candidate: unknown) => candidate is T,
  containsOneTimeSecret = false,
): TemporaryApiResult<T> => {
  if (
    body.payloadState === 'valid-json' &&
    successStatus(response.status) &&
    responseHasJsonContentType(response) &&
    (!containsOneTimeSecret || responseHasNoStore(response)) &&
    validEnvelope(body.payload, validData)
  ) {
    return { data: body.payload, payloadState: body.payloadState, response }
  }
  return {
    ...(!response.ok &&
    body.payloadState === 'valid-json' &&
    responseHasProblemContentType(response)
      ? { error: body.payload }
      : {}),
    payloadState: body.payloadState,
    response,
  }
}

export const initializeControlPlaneWithRecoveryCodes = async (options: {
  body: BootstrapRecoveryRequest
  setupToken: string
  signal?: AbortSignal
}): Promise<TemporaryApiResult<BootstrapRecoveryData>> => {
  const { body, response } = await request({
    body: options.body,
    headers: { 'x-nodecontroll-setup-token': options.setupToken },
    method: 'POST',
    path: bootstrapPath,
    signal: options.signal,
  })
  return result(response, body, (status) => status === 201, validBootstrapData, true)
}

export const getRecoveryCodeStatus = async (options: {
  signal?: AbortSignal
} = {}): Promise<TemporaryApiResult<RecoveryCodeStatus>> => {
  const { body, response } = await request({
    method: 'GET',
    path: recoveryCodesPath,
    signal: options.signal,
  })
  return result(response, body, (status) => status === 200, validRecoveryCodeStatus)
}

export const regenerateRecoveryCodes = async (options: {
  csrfToken: string
  signal?: AbortSignal
}): Promise<TemporaryApiResult<RegeneratedRecoveryCodes>> => {
  const { body, response } = await request({
    headers: { 'x-nodecontroll-csrf': options.csrfToken },
    method: 'POST',
    path: recoveryCodesPath,
    signal: options.signal,
  })
  return result(
    response,
    body,
    (status) => status === 200,
    validRegeneratedRecoveryCodes,
    true,
  )
}
