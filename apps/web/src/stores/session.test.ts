import { createPinia, setActivePinia } from 'pinia'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

const sdk = vi.hoisted(() => ({
  getBootstrapState: vi.fn(),
  getCurrentActor: vi.fn(),
  login: vi.fn(),
  logout: vi.fn(),
}))

vi.mock('../api/generated/sdk.gen', () => sdk)

import {
  CSRF_COOKIE_NAME,
  LoginFailure,
  readCsrfCookie,
  useSessionStore,
} from './session'

const actor = {
  capabilities: ['system:read'],
  force_password_change: false,
  id: '01900000-0000-7000-8000-000000000001',
  role: 'owner',
  username: 'owner',
}
const sessionProjection = {
  absolute_expires_at_ms: 2_000,
  created_at_ms: 1_000,
  id: '01900000-0000-7000-8000-000000000002',
  idle_expires_at_ms: 1_500,
  last_seen_at_ms: 1_000,
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

const problemResult = (status: number) => ({
  data: undefined,
  error: {
    code: 'UNTRUSTED_CODE',
    detail: 'untrusted server detail',
    request_id: 'request-error',
    status,
    title: 'untrusted server title',
    type: 'urn:untrusted',
  },
  response: response(status),
})

beforeEach(() => {
  setActivePinia(createPinia())
  for (const mock of Object.values(sdk)) mock.mockReset()
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
    expect(sdk.getBootstrapState).toHaveBeenCalledWith({ credentials: 'same-origin' })
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
    expect(sdk.getCurrentActor).toHaveBeenCalledWith({ credentials: 'same-origin' })
    expect(store.status).toBe('authenticated')
    expect(store.actor?.username).toBe('owner')
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
    })
    expect(JSON.stringify(store.$state)).not.toContain('never-render-this-password')
    expect(browserStorageWrite).not.toHaveBeenCalled()
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
    })
    expect(store.status).toBe('anonymous')
  })
})
