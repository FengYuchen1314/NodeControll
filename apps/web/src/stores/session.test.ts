import { createPinia, disposePinia, setActivePinia } from 'pinia'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

const sdk = vi.hoisted(() => ({
  changeCurrentPassword: vi.fn(),
  getBootstrapState: vi.fn(),
  getCurrentActor: vi.fn(),
  listCurrentSessions: vi.fn(),
  login: vi.fn(),
  logout: vi.fn(),
  logoutAll: vi.fn(),
  reauthenticate: vi.fn(),
  revokeCurrentUserSession: vi.fn(),
}))

const recoveryApi = vi.hoisted(() => ({
  getRecoveryCodeStatus: vi.fn(),
  regenerateRecoveryCodes: vi.fn(),
}))

vi.mock('../api/generated/sdk.gen', () => sdk)
vi.mock('../api/recovery-codes', () => recoveryApi)

import { CSRF_COOKIE_NAME, LoginFailure, readCsrfCookie, useSessionStore } from './session'
import { CREDENTIAL_COORDINATION_KEY } from '../lib/credential-coordinator'

const actor = {
  capabilities: ['system:read'],
  force_password_change: false,
  id: '01900000-0000-7000-8000-000000000001',
  role: 'owner',
  username: 'owner',
}
const sessionProjection = {
  absolute_expires_at_ms: 2_000,
  auth_level: 'password',
  created_at_ms: 1_000,
  id: '01900000-0000-7000-8000-000000000002',
  idle_expires_at_ms: 1_500,
  last_seen_at_ms: 1_000,
  recent_auth_expires_at_ms: 1_800,
}

const seedCredentialRecord = (disposition: 'quarantine' | 'reconcile' = 'reconcile') => {
  globalThis.localStorage.setItem(
    CREDENTIAL_COORDINATION_KEY,
    JSON.stringify({
      baseSeq: '1',
      disposition,
      epoch: '10000000-0000-4000-8000-000000000001',
      opId: '10000000-0000-4000-8000-000000000002',
      operation: 'login',
      phase: 'settled',
      senderId: '10000000-0000-4000-8000-000000000003',
      seq: '2',
      v: 1,
    }),
  )
}

const response = (status: number, headers: Record<string, string> = {}) => ({
  headers: new Headers(headers),
  status,
})

const bootstrapResult = (initialized: boolean) => ({
  data: {
    data: {
      initialized,
      login_methods: initialized ? ['password'] : [],
      product: 'NodeControll',
      setup_capability_required: !initialized,
    },
    meta: { api_version: 'v1', request_id: 'request-bootstrap' },
  },
  error: undefined,
  response: response(200),
})

const authenticatedResult = () => ({
  data: {
    data: { actor, session: sessionProjection },
    meta: { api_version: 'v1', request_id: 'request-auth' },
  },
  error: undefined,
  response: response(200),
})

const malformedAuthenticatedResult = () => ({
  data: {
    data: {},
    meta: { api_version: 'v1', request_id: 'request-malformed-auth' },
  },
  error: undefined,
  response: response(200),
})

const problemResult = (status: number, code = 'UNTRUSTED_CODE') => ({
  data: undefined,
  error: {
    code,
    detail: 'untrusted server detail',
    request_id: 'request-error',
    status,
    title: 'untrusted server title',
    type: 'urn:untrusted',
  },
  response: response(status),
})

const recoveryCodes = Array.from(
  { length: 8 },
  (_, index) => `${index.toString(16).padStart(4, '0')}-1111-2222-3333-4444-5555-6666-7777`,
)

const recoveryRegenerationResult = () => ({
  data: {
    data: {
      created_at_ms: 1_780_000_000_000,
      one_time_recovery_codes: [...recoveryCodes],
      set_version: 2,
    },
    meta: { api_version: 'v1', request_id: 'recovery-regeneration' },
  },
  error: undefined,
  response: response(200, { 'cache-control': 'no-store' }),
})

beforeEach(() => {
  globalThis.localStorage.clear()
  globalThis.sessionStorage.clear()
  seedCredentialRecord()
  setActivePinia(createPinia())
  for (const mock of Object.values(sdk)) mock.mockReset()
  recoveryApi.getRecoveryCodeStatus.mockReset()
  recoveryApi.regenerateRecoveryCodes.mockReset()
})

afterEach(() => {
  vi.restoreAllMocks()
})

describe('session store', () => {
  it('short-circuits at setup-required and does not call /me', async () => {
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(false))
    const store = useSessionStore()

    await store.refresh()

    expect(store.status).toBe('setup-required')
    expect(sdk.getBootstrapState).toHaveBeenCalledWith({
      credentials: 'same-origin',
      signal: expect.anything(),
    })
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
  })

  it('restores the actor from /me and coalesces concurrent refreshes', async () => {
    let resolveBootstrap!: (value: ReturnType<typeof bootstrapResult>) => void
    sdk.getBootstrapState.mockReturnValue(
      new Promise<ReturnType<typeof bootstrapResult>>((resolve) => {
        resolveBootstrap = resolve
      }),
    )
    sdk.getCurrentActor.mockResolvedValue(authenticatedResult())
    const store = useSessionStore()

    const first = store.refresh()
    const second = store.refresh()
    resolveBootstrap(bootstrapResult(true))
    await Promise.all([first, second])

    expect(sdk.getBootstrapState).toHaveBeenCalledTimes(1)
    expect(sdk.getCurrentActor).toHaveBeenCalledWith({
      credentials: 'same-origin',
      signal: expect.anything(),
    })
    expect(store.status).toBe('authenticated')
    expect(store.actor?.username).toBe('owner')
  })

  it('rejects a truthy but malformed /me projection without exposing identity', async () => {
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(true))
    sdk.getCurrentActor.mockResolvedValue(malformedAuthenticatedResult())
    const store = useSessionStore()

    await store.refresh()

    expect(store.status).toBe('unavailable')
    expect(store.actor).toBeUndefined()
    expect(store.session).toBeUndefined()
  })

  it('treats only an explicit /me 401 as anonymous', async () => {
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(true))
    sdk.getCurrentActor.mockResolvedValue(problemResult(503))
    const store = useSessionStore()

    await store.refresh()
    expect(store.status).toBe('unavailable')

    sdk.getCurrentActor.mockResolvedValue(problemResult(401))
    await store.refresh()
    expect(store.status).toBe('anonymous')
  })

  it('maps login failures locally without retaining credentials in Pinia or browser storage', async () => {
    const browserStorageWrite = vi.spyOn(Storage.prototype, 'setItem')
    sdk.login.mockResolvedValue(problemResult(401))
    const store = useSessionStore()
    store.markInitialized()

    const attempt = store.login('owner', 'never-render-this-password')

    await expect(attempt).rejects.toMatchObject({
      reason: 'invalid-credentials',
    } satisfies Partial<LoginFailure>)
    expect(sdk.login).toHaveBeenCalledWith({
      credentials: 'same-origin',
      body: { username: 'owner', password: 'never-render-this-password' },
      signal: expect.anything(),
    })
    expect(JSON.stringify(store.$state)).not.toContain('never-render-this-password')
    expect(browserStorageWrite).toHaveBeenCalledTimes(2)
    expect(browserStorageWrite).toHaveBeenCalledWith(
      CREDENTIAL_COORDINATION_KEY,
      expect.any(String),
    )
    expect(globalThis.localStorage.length).toBe(1)
    const coordinationRecord = globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY)
    expect(coordinationRecord).not.toContain('never-render-this-password')
    expect(JSON.parse(coordinationRecord ?? '{}')).toMatchObject({
      baseSeq: '3',
      disposition: 'reconcile',
      operation: 'login',
      phase: 'settled',
      seq: '4',
      v: 1,
    })
    expect(globalThis.sessionStorage.length).toBe(0)
  })

  it('treats a truthy malformed login 200 as unknown and performs fail-safe logout', async () => {
    const csrf = `ncc1_${'5'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.login.mockResolvedValue(malformedAuthenticatedResult())
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.markInitialized()

    await expect(store.login('owner', 'never-trust-this-body')).rejects.toMatchObject({
      reason: 'unavailable',
    })

    expect(sdk.login).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      signal: expect.anything(),
    })
    expect(store.status).toBe('anonymous')
    expect(store.actor).toBeUndefined()
    expect(store.session).toBeUndefined()
  })

  it('accepts each authenticated projection as one snapshot and derives password and recent-auth state', () => {
    const store = useSessionStore()
    store.acceptAuthenticated({
      actor: { ...actor, force_password_change: true },
      session: sessionProjection,
    })

    store.syncRecentAuthClock(1_200)
    expect(store.status).toBe('authenticated')
    expect(store.passwordChangeRequired).toBe(true)
    expect(store.recentAuthValid).toBe(true)
    expect(store.recentAuthExpired).toBe(false)

    store.syncRecentAuthClock(1_800)
    expect(store.recentAuthValid).toBe(false)
    expect(store.recentAuthExpired).toBe(true)
  })

  it('re-reads the CSRF cookie at the instant of reauthentication and accepts the rotated projection', async () => {
    const firstCsrf = `ncc1_${'a'.repeat(64)}`
    const rotatedCsrf = `ncc1_${'b'.repeat(64)}`
    const cookie = vi
      .spyOn(Document.prototype, 'cookie', 'get')
      .mockReturnValueOnce(`${CSRF_COOKIE_NAME}=${firstCsrf}`)
      .mockReturnValueOnce(`${CSRF_COOKIE_NAME}=${rotatedCsrf}`)
    sdk.reauthenticate
      .mockResolvedValueOnce(authenticatedResult())
      .mockResolvedValueOnce(authenticatedResult())
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await store.reauthenticate('first-current-password')
    await store.reauthenticate('second-current-password')

    expect(cookie).toHaveBeenCalledTimes(2)
    expect(sdk.reauthenticate).toHaveBeenNthCalledWith(1, {
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': firstCsrf },
      body: { method: 'password', password: 'first-current-password' },
      signal: expect.anything(),
    })
    expect(sdk.reauthenticate).toHaveBeenNthCalledWith(2, {
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': rotatedCsrf },
      body: { method: 'password', password: 'second-current-password' },
      signal: expect.anything(),
    })
    expect(store.session?.id).toBe(sessionProjection.id)
    expect(JSON.stringify(store.$state)).not.toContain('current-password')
  })

  it('requires relogin when a reauthentication rotation has an unknown transport outcome', async () => {
    const csrf = `ncc1_${'9'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.reauthenticate.mockRejectedValue(new TypeError('connection closed after request'))
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.reauthenticate('current-password')).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })

    expect(sdk.reauthenticate).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      signal: expect.anything(),
    })
    expect(store.status).toBe('anonymous')
  })

  it('blocks refresh while fail-safe logout is pending after an unknown mutation', async () => {
    const csrf = `ncc1_${'0'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    let resolveLogout!: (value: {
      data: Record<string, never>
      error: undefined
      response: ReturnType<typeof response>
    }) => void
    sdk.reauthenticate.mockRejectedValue(new TypeError('connection closed after request'))
    sdk.logout.mockReturnValue(
      new Promise<{
        data: Record<string, never>
        error: undefined
        response: ReturnType<typeof response>
      }>((resolve) => {
        resolveLogout = resolve
      }),
    )
    sdk.login.mockResolvedValue(authenticatedResult())
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(true))
    sdk.getCurrentActor.mockResolvedValue(authenticatedResult())
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    const mutation = expect(store.reauthenticate('current-password')).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })
    await vi.waitFor(() => expect(sdk.logout).toHaveBeenCalledTimes(1))

    expect(store.status).toBe('relogin-required')
    await store.refresh()
    expect(sdk.getBootstrapState).not.toHaveBeenCalled()
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
    expect(store.status).toBe('relogin-required')

    const login = store.login('owner', 'replacement-password')
    expect(sdk.login).not.toHaveBeenCalled()

    resolveLogout({ data: {}, error: undefined, response: response(204) })
    await mutation
    await login
    expect(sdk.login).toHaveBeenCalledTimes(1)
    expect(store.status).toBe('authenticated')
  })

  it('keeps the relogin quarantine sticky when fail-safe logout returns 503', async () => {
    const csrf = `ncc1_${'1'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    let resolveLogout!: (value: ReturnType<typeof problemResult>) => void
    let resolveLogin!: (value: ReturnType<typeof authenticatedResult>) => void
    sdk.reauthenticate.mockRejectedValue(new TypeError('connection closed after request'))
    sdk.logout.mockReturnValue(
      new Promise<ReturnType<typeof problemResult>>((resolve) => {
        resolveLogout = resolve
      }),
    )
    sdk.login.mockReturnValue(
      new Promise<ReturnType<typeof authenticatedResult>>((resolve) => {
        resolveLogin = resolve
      }),
    )
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(true))
    sdk.getCurrentActor.mockResolvedValue(authenticatedResult())
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    const mutation = expect(store.reauthenticate('current-password')).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })
    await vi.waitFor(() => expect(sdk.logout).toHaveBeenCalledTimes(1))
    expect(store.status).toBe('relogin-required')

    await store.refresh()
    expect(sdk.getBootstrapState).not.toHaveBeenCalled()
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
    expect(store.status).toBe('relogin-required')

    const login = store.login('owner', 'replacement-password')
    expect(sdk.login).not.toHaveBeenCalled()

    resolveLogout(problemResult(503, 'AUTHENTICATION_UNAVAILABLE'))
    await mutation
    await vi.waitFor(() => expect(sdk.login).toHaveBeenCalledTimes(1))
    expect(store.status).toBe('relogin-required')

    resolveLogin(authenticatedResult())
    await login
    expect(store.status).toBe('authenticated')
  })

  it('requires relogin when reauthentication returns an explicit authentication 503', async () => {
    const csrf = `ncc1_${'8'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.reauthenticate.mockResolvedValue(problemResult(503, 'AUTHENTICATION_UNAVAILABLE'))
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.reauthenticate('current-password')).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })

    expect(sdk.reauthenticate).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      signal: expect.anything(),
    })
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
    expect(store.status).toBe('anonymous')
  })

  it('requires relogin when password change returns an explicit authentication 503', async () => {
    const csrf = `ncc1_${'7'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.changeCurrentPassword.mockResolvedValue(problemResult(503, 'AUTHENTICATION_UNAVAILABLE'))
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.changePassword('new-password-never-store')).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })

    expect(sdk.changeCurrentPassword).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      signal: expect.anything(),
    })
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
    expect(store.status).toBe('anonymous')
  })

  it('requires relogin when logout-all returns an explicit authentication 503', async () => {
    const csrf = `ncc1_${'6'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.logoutAll.mockResolvedValue(problemResult(503, 'AUTHENTICATION_UNAVAILABLE'))
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.logoutAll(true)).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })

    expect(sdk.logoutAll).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      signal: expect.anything(),
    })
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
    expect(store.status).toBe('anonymous')
  })

  it('accepts password-change and keep-current logout-all projections without retaining passwords', async () => {
    const csrf = `ncc1_${'c'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.changeCurrentPassword.mockResolvedValue({
      ...authenticatedResult(),
      data: {
        ...authenticatedResult().data,
        data: { actor, revoked_sessions: 3, session: sessionProjection },
      },
    })
    sdk.logoutAll.mockResolvedValue({
      ...authenticatedResult(),
      data: {
        ...authenticatedResult().data,
        data: { actor, revoked_sessions: 2, session: sessionProjection },
      },
    })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.changePassword('new-password-never-store')).resolves.toBe(3)
    await expect(store.logoutAll(true)).resolves.toBe(2)

    expect(sdk.changeCurrentPassword).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      body: { new_password: 'new-password-never-store' },
      signal: expect.anything(),
    })
    expect(sdk.logoutAll).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      body: { keep_current: true },
      signal: expect.anything(),
    })
    expect(JSON.stringify(store.$state)).not.toContain('new-password-never-store')
  })

  it('maps JSON media, size, and shape rejections locally without trusting Problem text', async () => {
    const csrf = `ncc1_${'d'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.reauthenticate.mockResolvedValue(problemResult(415))
    sdk.changeCurrentPassword.mockResolvedValue(problemResult(413))
    sdk.logoutAll.mockResolvedValue(problemResult(422))
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.reauthenticate('password')).rejects.toMatchObject({
      reason: 'request-rejected',
    })
    await expect(store.changePassword('a-new-password')).rejects.toMatchObject({
      reason: 'request-rejected',
    })
    await expect(store.logoutAll(true)).rejects.toMatchObject({
      reason: 'request-rejected',
    })
  })

  it('treats malformed HTTP 200 rotation bodies as unknown and closes each local snapshot', async () => {
    const csrf = `ncc1_${'2'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    const malformed = {
      data: undefined,
      error: { message: 'invalid JSON response' },
      response: response(200),
    }
    sdk.reauthenticate.mockResolvedValue(malformed)
    sdk.changeCurrentPassword.mockResolvedValue(malformed)
    sdk.logoutAll.mockResolvedValue(malformed)
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()

    for (const mutation of [
      () => store.reauthenticate('current-password'),
      () => store.changePassword('replacement-password'),
      () => store.logoutAll(true),
    ]) {
      store.acceptAuthenticated({ actor, session: sessionProjection })
      await expect(mutation()).rejects.toMatchObject({ reason: 'outcome-unknown' })
      expect(store.status).toBe('anonymous')
    }
    expect(sdk.reauthenticate).toHaveBeenCalledTimes(1)
    expect(sdk.changeCurrentPassword).toHaveBeenCalledTimes(1)
    expect(sdk.logoutAll).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledTimes(3)
  })

  it('rejects truthy malformed rotation projections and never restores protected state', async () => {
    const csrf = `ncc1_${'3'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.reauthenticate.mockResolvedValue(malformedAuthenticatedResult())
    sdk.changeCurrentPassword.mockResolvedValue(malformedAuthenticatedResult())
    sdk.logoutAll.mockResolvedValue(malformedAuthenticatedResult())
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()

    for (const mutation of [
      () => store.reauthenticate('current-password'),
      () => store.changePassword('replacement-password'),
      () => store.logoutAll(true),
    ]) {
      store.acceptAuthenticated({ actor, session: sessionProjection })
      await expect(mutation()).rejects.toMatchObject({ reason: 'outcome-unknown' })
      expect(store.status).toBe('anonymous')
      expect(store.actor).toBeUndefined()
      expect(store.session).toBeUndefined()
    }

    expect(sdk.reauthenticate).toHaveBeenCalledTimes(1)
    expect(sdk.changeCurrentPassword).toHaveBeenCalledTimes(1)
    expect(sdk.logoutAll).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledTimes(3)
  })

  it('does not let a late reauthentication success resurrect identity after logout', async () => {
    const csrf = `ncc1_${'4'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    let resolveReauthentication!: (value: ReturnType<typeof authenticatedResult>) => void
    sdk.reauthenticate.mockReturnValue(
      new Promise<ReturnType<typeof authenticatedResult>>((resolve) => {
        resolveReauthentication = resolve
      }),
    )
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    const lateReauthentication = expect(
      store.reauthenticate('current-password'),
    ).rejects.toMatchObject({ reason: 'outcome-unknown' })
    await vi.waitFor(() => expect(sdk.reauthenticate).toHaveBeenCalledTimes(1))
    const logout = store.logout()
    expect(store.status).toBe('relogin-required')
    expect(sdk.logout).not.toHaveBeenCalled()

    resolveReauthentication(authenticatedResult())
    await lateReauthentication
    await logout

    expect(sdk.reauthenticate).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledTimes(2)
    expect(store.status).toBe('anonymous')
  })

  it('rejects a second protected mutation while the first cookie rotation is in flight', async () => {
    const csrf = `ncc1_${'3'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    let resolveReauthentication!: (value: ReturnType<typeof authenticatedResult>) => void
    sdk.reauthenticate.mockReturnValue(
      new Promise<ReturnType<typeof authenticatedResult>>((resolve) => {
        resolveReauthentication = resolve
      }),
    )
    sdk.changeCurrentPassword.mockRejectedValue(new TypeError('connection closed'))
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    const reauthentication = store.reauthenticate('current-password')
    await vi.waitFor(() => expect(sdk.reauthenticate).toHaveBeenCalledTimes(1))
    const passwordChange = expect(
      store.changePassword('replacement-password'),
    ).rejects.toMatchObject({ reason: 'session-invalid' })
    expect(sdk.changeCurrentPassword).not.toHaveBeenCalled()

    resolveReauthentication(authenticatedResult())
    await reauthentication
    await passwordChange

    expect(sdk.reauthenticate).toHaveBeenCalledTimes(1)
    expect(sdk.changeCurrentPassword).not.toHaveBeenCalled()
    expect(sdk.logout).not.toHaveBeenCalled()
    expect(store.status).toBe('authenticated')
  })

  it('preserves a server recent-auth challenge from session revocation without replaying', async () => {
    const csrf = `ncc1_${'9'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.revokeCurrentUserSession.mockResolvedValue(problemResult(403, 'RECENT_AUTH_REQUIRED'))
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.revokeSession('01900000-0000-7000-8000-000000000004')).rejects.toMatchObject(
      { reason: 'recent-auth-required' },
    )
    expect(sdk.revokeCurrentUserSession).toHaveBeenCalledTimes(1)
    expect(store.session?.id).toBe(sessionProjection.id)
  })

  it('fails closed when revoking the current session has an unknown transport outcome', async () => {
    const csrf = `ncc1_${'8'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.revokeCurrentUserSession.mockRejectedValue(new TypeError('connection closed'))
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.revokeSession(sessionProjection.id)).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })

    expect(sdk.revokeCurrentUserSession).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      signal: expect.anything(),
    })
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
    expect(store.status).toBe('anonymous')
  })

  it('uses the request-start snapshot when current-session revocation races a local rotation', async () => {
    const csrf = `ncc1_${'5'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    let rejectRevocation!: (reason?: unknown) => void
    sdk.revokeCurrentUserSession.mockReturnValue(
      new Promise((_resolve, reject) => {
        rejectRevocation = reject
      }),
    )
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    const pending = expect(store.revokeSession(sessionProjection.id)).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })
    store.acceptAuthenticated({
      actor,
      session: { ...sessionProjection, id: '01900000-0000-7000-8000-000000000005' },
    })
    rejectRevocation(new TypeError('connection closed'))
    await pending

    expect(sdk.revokeCurrentUserSession).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledTimes(1)
    expect(store.status).toBe('anonymous')
  })

  it('keeps protected state closed when current-session revocation and fail-safe logout are unknown', async () => {
    const csrf = `ncc1_${'7'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.revokeCurrentUserSession.mockRejectedValue(new TypeError('connection closed'))
    sdk.logout.mockRejectedValue(new TypeError('proxy unavailable'))
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.revokeSession(sessionProjection.id)).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })

    expect(sdk.revokeCurrentUserSession).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledTimes(1)
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
    expect(store.status).toBe('relogin-required')
  })

  it('does not sign out a valid current session when another-session revocation is unknown', async () => {
    const csrf = `ncc1_${'6'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.revokeCurrentUserSession.mockRejectedValue(new TypeError('connection closed'))
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.revokeSession('01900000-0000-7000-8000-000000000004')).rejects.toMatchObject(
      { reason: 'outcome-unknown' },
    )

    expect(sdk.revokeCurrentUserSession).toHaveBeenCalledTimes(1)
    expect(sdk.logout).not.toHaveBeenCalled()
    expect(store.status).toBe('authenticated')
    expect(store.session?.id).toBe(sessionProjection.id)
  })

  it('fails closed when logout-everywhere has an unknown transport outcome', async () => {
    const csrf = `ncc1_${'e'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.logoutAll.mockRejectedValue(new TypeError('connection closed'))
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.logoutAll(false)).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })
    expect(store.status).toBe('anonymous')
    expect(sdk.logout).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      signal: expect.anything(),
    })
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
  })

  it('requires relogin after a keep-current transport failure without replaying the mutation', async () => {
    const csrf = `ncc1_${'f'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.logoutAll.mockRejectedValue(new TypeError('connection closed'))
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.logoutAll(true)).rejects.toMatchObject({ reason: 'outcome-unknown' })

    expect(sdk.logoutAll).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledTimes(1)
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
    expect(store.status).toBe('anonymous')
  })

  it('does not infer keep-current success from replacement cookies when the body was lost', async () => {
    const csrf = `ncc1_${'1'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.logoutAll.mockResolvedValue({
      data: undefined,
      error: undefined,
      response: response(200),
    })
    sdk.logout.mockRejectedValue(new TypeError('proxy unavailable'))
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.logoutAll(true)).rejects.toMatchObject({ reason: 'outcome-unknown' })

    expect(sdk.logoutAll).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledTimes(1)
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
    expect(store.status).toBe('relogin-required')
  })

  it('reads only one exact, strictly formatted CSRF cookie for logout', async () => {
    const csrf = `ncc1_${'a'.repeat(64)}`
    expect(readCsrfCookie(`prefix${CSRF_COOKIE_NAME}=${csrf}`)).toBeUndefined()
    expect(
      readCsrfCookie(`${CSRF_COOKIE_NAME}=${csrf}; ${CSRF_COOKIE_NAME}=${csrf}`),
    ).toBeUndefined()
    expect(readCsrfCookie(`${CSRF_COOKIE_NAME}=encoded%20value`)).toBeUndefined()
    expect(readCsrfCookie(`theme=dark; ${CSRF_COOKIE_NAME}=${csrf}`)).toBe(csrf)

    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(
      `theme=dark; ${CSRF_COOKIE_NAME}=${csrf}`,
    )
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()

    await store.logout()

    expect(sdk.logout).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      signal: expect.anything(),
    })
    expect(store.status).toBe('anonymous')
  })

  it('serializes explicit relogin behind an ordinary logout that settles as 503', async () => {
    const csrf = `ncc1_${'b'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    let resolveLogout!: (value: ReturnType<typeof problemResult>) => void
    let resolveLogin!: (value: ReturnType<typeof authenticatedResult>) => void
    sdk.logout.mockReturnValue(
      new Promise<ReturnType<typeof problemResult>>((resolve) => {
        resolveLogout = resolve
      }),
    )
    sdk.login.mockReturnValue(
      new Promise<ReturnType<typeof authenticatedResult>>((resolve) => {
        resolveLogin = resolve
      }),
    )
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(true))
    sdk.getCurrentActor.mockResolvedValue(authenticatedResult())
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    const logout = expect(store.logout()).rejects.toMatchObject({ reason: 'unavailable' })
    await vi.waitFor(() => expect(sdk.logout).toHaveBeenCalledTimes(1))

    expect(store.status).toBe('relogin-required')
    await store.refresh()
    expect(sdk.getBootstrapState).not.toHaveBeenCalled()
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
    expect(store.status).toBe('relogin-required')

    const login = store.login('owner', 'replacement-password')
    expect(sdk.login).not.toHaveBeenCalled()

    resolveLogout(problemResult(503, 'AUTHENTICATION_UNAVAILABLE'))
    await logout
    expect(sdk.logout).toHaveBeenCalledTimes(1)
    await vi.waitFor(() => expect(sdk.login).toHaveBeenCalledTimes(1))
    expect(store.status).toBe('relogin-required')
    expect(store.actor).toBeUndefined()
    expect(store.session).toBeUndefined()

    resolveLogin(authenticatedResult())
    await login
    expect(store.status).toBe('authenticated')
  })

  it('persists the exact inflight and terminal revisions around a login request', async () => {
    let resolveLogin!: (value: ReturnType<typeof problemResult>) => void
    sdk.login.mockReturnValue(
      new Promise<ReturnType<typeof problemResult>>((resolve) => {
        resolveLogin = resolve
      }),
    )
    const store = useSessionStore()
    store.markInitialized()

    const login = expect(store.login('owner', 'never-persist-this-password')).rejects.toMatchObject(
      { reason: 'invalid-credentials' },
    )
    await vi.waitFor(() => expect(sdk.login).toHaveBeenCalledTimes(1))

    const inflight = JSON.parse(
      globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) ?? '{}',
    )
    expect(inflight).toMatchObject({
      baseSeq: '2',
      disposition: 'quarantine',
      operation: 'login',
      phase: 'inflight',
      seq: '3',
      v: 1,
    })
    expect(JSON.stringify(inflight)).not.toContain('never-persist-this-password')

    resolveLogin(problemResult(401))
    await login
    const terminal = JSON.parse(
      globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) ?? '{}',
    )
    expect(terminal).toMatchObject({
      baseSeq: '3',
      disposition: 'reconcile',
      epoch: inflight.epoch,
      opId: inflight.opId,
      operation: 'login',
      phase: 'settled',
      seq: '4',
      v: 1,
    })
    expect(store.status).toBe('anonymous')
  })

  it('does not clear quarantine observed while a login waits for the credential lock', async () => {
    for (const [httpStatus, expectedReason] of [
      [401, 'invalid-credentials'],
      [429, 'rate-limited'],
    ] as const) {
      seedCredentialRecord()
      sdk.login.mockReset()
      sdk.login.mockResolvedValueOnce(problemResult(httpStatus))
      const pinia = createPinia()
      const store = useSessionStore(pinia)
      store.markInitialized()
      let signalExternalLockStarted!: () => void
      let releaseExternalMutation!: () => void
      const externalLockStarted = new Promise<void>((resolve) => {
        signalExternalLockStarted = resolve
      })
      const externalMutationMaySettle = new Promise<void>((resolve) => {
        releaseExternalMutation = resolve
      })
      const externalMutation = globalThis.navigator.locks.request(
        'nodecontroll:credential-cookie',
        { mode: 'exclusive' },
        async () => {
          signalExternalLockStarted()
          await externalMutationMaySettle
          const quarantine = {
            baseSeq: '2',
            disposition: 'quarantine',
            epoch: '10000000-0000-4000-8000-000000000001',
            opId: '10000000-0000-4000-8000-000000000004',
            operation: 'logout',
            phase: 'settled',
            senderId: '10000000-0000-4000-8000-000000000005',
            seq: '3',
            v: 1,
          }
          globalThis.localStorage.setItem(CREDENTIAL_COORDINATION_KEY, JSON.stringify(quarantine))
          globalThis.dispatchEvent(
            new StorageEvent('storage', {
              key: CREDENTIAL_COORDINATION_KEY,
              newValue: JSON.stringify(quarantine),
            }),
          )
        },
      )
      await externalLockStarted

      const login = expect(store.login('owner', 'queued-login-password')).rejects.toMatchObject({
        reason: expectedReason,
      })
      expect(sdk.login).not.toHaveBeenCalled()
      releaseExternalMutation()
      await externalMutation
      await login

      expect(sdk.login).toHaveBeenCalledTimes(1)
      expect(store.status).toBe('relogin-required')
      expect(store.actor).toBeUndefined()
      expect(store.session).toBeUndefined()
      expect(
        JSON.parse(globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) ?? '{}'),
      ).toMatchObject({
        disposition: 'quarantine',
        operation: 'login',
        phase: 'settled',
      })
      disposePinia(pinia)
    }
  })

  it('keeps a late-joining store quarantined until an explicit login succeeds', async () => {
    seedCredentialRecord('quarantine')
    sdk.login.mockResolvedValue(authenticatedResult())
    const store = useSessionStore()

    expect(store.status).toBe('relogin-required')
    await store.refresh()
    expect(sdk.getBootstrapState).not.toHaveBeenCalled()
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()

    await store.login('owner', 'explicit-recovery-password')
    expect(store.status).toBe('authenticated')
    expect(
      JSON.parse(globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) ?? '{}'),
    ).toMatchObject({
      baseSeq: '3',
      disposition: 'reconcile',
      operation: 'login',
      phase: 'settled',
      seq: '4',
    })
  })

  it('propagates an observed session 401 to another store and persists invalidation', async () => {
    let resolveList!: (value: ReturnType<typeof problemResult>) => void
    sdk.listCurrentSessions.mockReturnValue(
      new Promise<ReturnType<typeof problemResult>>((resolve) => {
        resolveList = resolve
      }),
    )
    const firstPinia = createPinia()
    const secondPinia = createPinia()
    const first = useSessionStore(firstPinia)
    const second = useSessionStore(secondPinia)
    first.acceptAuthenticated({ actor, session: sessionProjection })
    second.acceptAuthenticated({ actor, session: sessionProjection })

    const list = expect(first.listSessions()).rejects.toMatchObject({
      reason: 'session-invalid',
    })
    await vi.waitFor(() => expect(sdk.listCurrentSessions).toHaveBeenCalledTimes(1))
    expect(second.status).toBe('authenticated')

    resolveList(problemResult(401, 'SESSION_INVALID'))
    await list
    expect(first.status).toBe('anonymous')
    expect(second.status).toBe('anonymous')
    expect(
      JSON.parse(globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) ?? '{}'),
    ).toMatchObject({
      baseSeq: '2',
      observedSessionId: sessionProjection.id,
      operation: 'read-401',
      phase: 'invalidated',
      seq: '3',
    })

    disposePinia(firstPinia)
    disposePinia(secondPinia)
  })

  it('orders two stores so explicit login cannot overtake an ordinary logout', async () => {
    const csrf = `ncc1_${'c'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    let resolveLogout!: (value: {
      data: Record<string, never>
      error: undefined
      response: ReturnType<typeof response>
    }) => void
    sdk.logout.mockReturnValue(
      new Promise<{
        data: Record<string, never>
        error: undefined
        response: ReturnType<typeof response>
      }>((resolve) => {
        resolveLogout = resolve
      }),
    )
    sdk.login.mockResolvedValue(authenticatedResult())
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(true))
    sdk.getCurrentActor.mockResolvedValue(authenticatedResult())
    const firstPinia = createPinia()
    const secondPinia = createPinia()
    const first = useSessionStore(firstPinia)
    const second = useSessionStore(secondPinia)
    first.acceptAuthenticated({ actor, session: sessionProjection })
    second.acceptAuthenticated({ actor, session: sessionProjection })

    const logout = first.logout()
    await vi.waitFor(() => expect(sdk.logout).toHaveBeenCalledTimes(1))
    expect(first.status).toBe('relogin-required')
    expect(second.status).toBe('relogin-required')

    const login = second.login('owner', 'replacement-password')
    expect(sdk.login).not.toHaveBeenCalled()
    resolveLogout({ data: {}, error: undefined, response: response(204) })
    await logout
    await vi.waitFor(() => expect(sdk.login).toHaveBeenCalledTimes(1))
    await login
    await vi.waitFor(() => expect(first.status).toBe('authenticated'))
    expect(second.status).toBe('authenticated')

    disposePinia(firstPinia)
    disposePinia(secondPinia)
  })

  it('holds the credential read lock through response validation before logout can rotate cookies', async () => {
    const csrf = `ncc1_${'d'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    let resolveList!: (value: {
      data: {
        data: { sessions: Array<typeof sessionProjection & { is_current: boolean }> }
        meta: { api_version: string; request_id: string }
      }
      error: undefined
      response: ReturnType<typeof response>
    }) => void
    sdk.listCurrentSessions.mockReturnValue(
      new Promise((resolve) => {
        resolveList = resolve
      }),
    )
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    const list = expect(store.listSessions()).rejects.toMatchObject({
      reason: 'session-invalid',
    })
    await vi.waitFor(() => expect(sdk.listCurrentSessions).toHaveBeenCalledTimes(1))
    const logout = store.logout()
    expect(sdk.logout).not.toHaveBeenCalled()

    resolveList({
      data: {
        data: { sessions: [{ ...sessionProjection, is_current: true }] },
        meta: { api_version: 'v1', request_id: 'request-list' },
      },
      error: undefined,
      response: response(200),
    })
    await list
    await vi.waitFor(() => expect(sdk.logout).toHaveBeenCalledTimes(1))
    await logout

    expect(store.status).toBe('anonymous')
  })

  it('clears a stale managed-session list when a truthy 200 body fails validation', async () => {
    sdk.listCurrentSessions
      .mockResolvedValueOnce({
        data: {
          data: { sessions: [{ ...sessionProjection, is_current: true }] },
          meta: { api_version: 'v1', request_id: 'request-list-valid' },
        },
        error: undefined,
        response: response(200),
      })
      .mockResolvedValueOnce({
        data: {
          data: { sessions: [{}] },
          meta: { api_version: 'v1', request_id: 'request-list-malformed' },
        },
        error: undefined,
        response: response(200),
      })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.listSessions()).resolves.toHaveLength(1)
    expect(store.managedSessions).toHaveLength(1)
    await expect(store.listSessions()).rejects.toMatchObject({ reason: 'unavailable' })

    expect(store.status).toBe('authenticated')
    expect(store.managedSessions).toEqual([])
  })

  it('closes an authenticated projection when Web Lock acquisition fails', async () => {
    vi.spyOn(globalThis.navigator.locks, 'request').mockRejectedValueOnce(
      new Error('lock service unavailable'),
    )
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.reauthenticate('current-password')).rejects.toMatchObject({
      reason: 'unavailable',
    })

    expect(sdk.reauthenticate).not.toHaveBeenCalled()
    expect(store.status).toBe('relogin-required')
    expect(store.actor).toBeUndefined()
    expect(store.session).toBeUndefined()
  })

  it('allows an empty setup origin on focus but quarantines a lost authenticated journal', async () => {
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(false))
    const store = useSessionStore()
    await store.refresh()
    expect(store.status).toBe('setup-required')

    globalThis.localStorage.clear()
    globalThis.dispatchEvent(new Event('focus'))
    expect(store.status).toBe('setup-required')

    store.acceptAuthenticated({ actor, session: sessionProjection })
    globalThis.dispatchEvent(new Event('focus'))
    expect(store.status).toBe('relogin-required')
    expect(store.actor).toBeUndefined()
    expect(store.session).toBeUndefined()
  })

  it('recovers a cleared journal only through an explicit login with a new epoch', async () => {
    sdk.login.mockResolvedValue(authenticatedResult())
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })
    const previousEpoch = JSON.parse(
      globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) ?? '{}',
    ).epoch

    globalThis.localStorage.clear()
    globalThis.dispatchEvent(new Event('focus'))
    expect(store.status).toBe('relogin-required')

    await store.login('owner', 'explicit-recovery-password')

    const recovered = JSON.parse(
      globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) ?? '{}',
    )
    expect(recovered).toMatchObject({
      baseSeq: '1',
      disposition: 'reconcile',
      operation: 'login',
      phase: 'settled',
      seq: '2',
    })
    expect(recovered.epoch).not.toBe(previousEpoch)
    expect(store.status).toBe('authenticated')
  })

  it('quarantines same-epoch journal rollback, same-revision mutation, and skipped revisions', () => {
    const cases = [
      {
        baseSeq: '0',
        disposition: 'reconcile',
        epoch: '10000000-0000-4000-8000-000000000001',
        opId: '10000000-0000-4000-8000-000000000002',
        operation: 'login',
        phase: 'settled',
        senderId: '10000000-0000-4000-8000-000000000003',
        seq: '1',
        v: 1,
      },
      {
        baseSeq: '1',
        disposition: 'reconcile',
        epoch: '10000000-0000-4000-8000-000000000001',
        opId: '10000000-0000-4000-8000-000000000002',
        operation: 'reauth',
        phase: 'settled',
        senderId: '10000000-0000-4000-8000-000000000003',
        seq: '2',
        v: 1,
      },
      {
        baseSeq: '8',
        disposition: 'quarantine',
        epoch: '10000000-0000-4000-8000-000000000001',
        opId: '10000000-0000-4000-8000-000000000004',
        operation: 'reauth',
        phase: 'inflight',
        senderId: '10000000-0000-4000-8000-000000000005',
        seq: '9',
        v: 1,
      },
    ]

    for (const candidate of cases) {
      seedCredentialRecord()
      const pinia = createPinia()
      const store = useSessionStore(pinia)
      store.acceptAuthenticated({ actor, session: sessionProjection })
      globalThis.localStorage.setItem(CREDENTIAL_COORDINATION_KEY, JSON.stringify(candidate))
      globalThis.dispatchEvent(
        new StorageEvent('storage', {
          key: CREDENTIAL_COORDINATION_KEY,
          newValue: JSON.stringify(candidate),
        }),
      )

      expect(store.status).toBe('relogin-required')
      expect(store.actor).toBeUndefined()
      expect(store.session).toBeUndefined()
      if (candidate.phase === 'inflight') {
        const terminal = {
          ...candidate,
          baseSeq: candidate.seq,
          disposition: 'reconcile',
          phase: 'settled',
          seq: '10',
        }
        globalThis.localStorage.setItem(CREDENTIAL_COORDINATION_KEY, JSON.stringify(terminal))
        globalThis.dispatchEvent(
          new StorageEvent('storage', {
            key: CREDENTIAL_COORDINATION_KEY,
            newValue: JSON.stringify(terminal),
          }),
        )
        expect(store.status).toBe('relogin-required')
        expect(sdk.getBootstrapState).not.toHaveBeenCalled()
      }
      disposePinia(pinia)
    }
  })

  it('fails closed when the coordination epoch is replaced without an observed inflight record', () => {
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })
    globalThis.localStorage.setItem(
      CREDENTIAL_COORDINATION_KEY,
      JSON.stringify({
        baseSeq: '8',
        disposition: 'reconcile',
        epoch: '30000000-0000-4000-8000-000000000001',
        opId: '30000000-0000-4000-8000-000000000002',
        operation: 'login',
        phase: 'settled',
        senderId: '30000000-0000-4000-8000-000000000003',
        seq: '9',
        v: 1,
      }),
    )

    globalThis.dispatchEvent(
      new StorageEvent('storage', {
        key: CREDENTIAL_COORDINATION_KEY,
        newValue: globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY),
      }),
    )

    expect(store.status).toBe('relogin-required')
    expect(store.actor).toBeUndefined()
    expect(store.session).toBeUndefined()
  })

  it('returns regenerated recovery codes only to the caller and keeps them out of browser state', async () => {
    const csrf = `ncc1_${'a'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    recoveryApi.regenerateRecoveryCodes.mockResolvedValue(recoveryRegenerationResult())
    const storageWrite = vi.spyOn(Storage.prototype, 'setItem')
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.regenerateRecoveryCodes()).resolves.toEqual(recoveryCodes)

    expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledWith({
      csrfToken: csrf,
      signal: expect.anything(),
    })
    expect(JSON.stringify(store.$state)).not.toContain(recoveryCodes[0])
    expect(globalThis.sessionStorage.length).toBe(0)
    const writtenValues = storageWrite.mock.calls.map((call) => String(call[1])).join('\n')
    expect(writtenValues).not.toContain(recoveryCodes[0])
    expect(JSON.parse(globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) ?? '{}')).toMatchObject(
      {
        disposition: 'reconcile',
        operation: 'regenerate-recovery-codes',
        phase: 'settled',
      },
    )
  })

  it('withholds and wipes regenerated plaintext when terminal journal settlement fails', async () => {
    const csrf = `ncc1_${'d'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    const apiResponse = recoveryRegenerationResult()
    let resolveRegeneration!: (value: ReturnType<typeof recoveryRegenerationResult>) => void
    recoveryApi.regenerateRecoveryCodes.mockReturnValue(
      new Promise<ReturnType<typeof recoveryRegenerationResult>>((resolve) => {
        resolveRegeneration = resolve
      }),
    )
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    const regeneration = expect(store.regenerateRecoveryCodes()).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })
    await vi.waitFor(() => expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledTimes(1))
    const inflight = JSON.parse(
      globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) ?? '{}',
    )
    globalThis.localStorage.setItem(
      CREDENTIAL_COORDINATION_KEY,
      JSON.stringify({
        ...inflight,
        opId: '40000000-0000-4000-8000-000000000004',
      }),
    )

    resolveRegeneration(apiResponse)
    await regeneration

    expect(store.status).toBe('relogin-required')
    expect(apiResponse.data.data.one_time_recovery_codes.every((code) => code === '')).toBe(true)
    expect(JSON.stringify(store.$state)).not.toContain(recoveryCodes[0])
    expect(globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY)).not.toContain(
      recoveryCodes[0],
    )
  })

  it('rejects and wipes a regeneration projection carried by an unexpected 201 response', async () => {
    const csrf = `ncc1_${'2'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    const apiResponse = recoveryRegenerationResult()
    apiResponse.response = response(201, { 'cache-control': 'no-store' })
    recoveryApi.regenerateRecoveryCodes.mockResolvedValue(apiResponse)
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.regenerateRecoveryCodes()).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })

    expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledTimes(1)
    expect(apiResponse.data.data.one_time_recovery_codes.every((code) => code === '')).toBe(true)
  })

  it('maps stale regeneration success, 401, and 403 to outcome-unknown without returning secrets', async () => {
    for (const responseKind of ['success', '401', '403'] as const) {
      seedCredentialRecord()
      recoveryApi.regenerateRecoveryCodes.mockReset()
      sdk.logout.mockReset()
      sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
      let resolveRegeneration!: (
        value:
          | ReturnType<typeof recoveryRegenerationResult>
          | ReturnType<typeof problemResult>
      ) => void
      recoveryApi.regenerateRecoveryCodes.mockReturnValue(
        new Promise<
          ReturnType<typeof recoveryRegenerationResult> | ReturnType<typeof problemResult>
        >((resolve) => {
          resolveRegeneration = resolve
        }),
      )
      const pinia = createPinia()
      const store = useSessionStore(pinia)
      store.acceptAuthenticated({ actor, session: sessionProjection })

      const regeneration = expect(store.regenerateRecoveryCodes()).rejects.toMatchObject({
        reason: 'outcome-unknown',
      })
      await vi.waitFor(() => expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledTimes(1))
      store.acceptAuthenticated({ actor, session: sessionProjection })
      const apiResponse =
        responseKind === 'success'
          ? recoveryRegenerationResult()
          : problemResult(
              Number(responseKind),
              responseKind === '403' ? 'RECENT_AUTH_REQUIRED' : 'SESSION_INVALID',
            )
      resolveRegeneration(apiResponse)
      await regeneration

      expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledTimes(1)
      expect(sdk.logout).toHaveBeenCalledTimes(1)
      if (responseKind === 'success' && apiResponse.data !== undefined) {
        expect(apiResponse.data.data.one_time_recovery_codes.every((code) => code === '')).toBe(true)
      }
      disposePinia(pinia)
    }
  })

  it('does not replay regeneration after a server-side recent-auth rejection', async () => {
    const csrf = `ncc1_${'b'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    recoveryApi.regenerateRecoveryCodes.mockResolvedValue(
      problemResult(403, 'RECENT_AUTH_REQUIRED'),
    )
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.regenerateRecoveryCodes()).rejects.toMatchObject({
      reason: 'recent-auth-required',
    })

    expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledTimes(1)
    expect(store.status).toBe('authenticated')
    expect(JSON.parse(globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) ?? '{}')).toMatchObject(
      {
        disposition: 'reconcile',
        operation: 'regenerate-recovery-codes',
        phase: 'settled',
      },
    )
  })

  it('never replays an unknown regeneration outcome and requires a confirmed relogin', async () => {
    const csrf = `ncc1_${'c'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    recoveryApi.regenerateRecoveryCodes.mockRejectedValue(new TypeError('connection closed'))
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const store = useSessionStore()
    store.acceptAuthenticated({ actor, session: sessionProjection })

    await expect(store.regenerateRecoveryCodes()).rejects.toMatchObject({
      reason: 'outcome-unknown',
    })

    expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledTimes(1)
    expect(sdk.logout).toHaveBeenCalledTimes(1)
    expect(store.status).toBe('anonymous')
    expect(JSON.parse(globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) ?? '{}')).toMatchObject(
      {
        disposition: 'reconcile',
        operation: 'regenerate-recovery-codes',
        phase: 'settled',
      },
    )
  })
})
