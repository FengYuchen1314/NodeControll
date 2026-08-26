import { createPinia } from 'pinia'
import { createMemoryHistory } from 'vue-router'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const sdk = vi.hoisted(() => ({
  getBootstrapState: vi.fn(),
  getCurrentActor: vi.fn(),
  getReadiness: vi.fn(),
  getSystemVersion: vi.fn(),
  initializeControlPlane: vi.fn(),
  login: vi.fn(),
  logout: vi.fn(),
}))

vi.mock('../api/generated/sdk.gen', () => sdk)

import { createAppRouter, safeRedirectPath } from './index'

const response = (status: number) => ({ headers: new Headers(), status })
const bootstrapResult = (initialized: boolean) => ({
  data: {
    data: {
      initialized,
      login_methods: initialized ? ['password'] : [],
      product: 'NodeControll',
      setup_capability_required: !initialized,
    },
    meta: { api_version: 'v1', request_id: 'bootstrap-request' },
  },
  error: undefined,
  response: response(200),
})
const anonymousResult = () => ({
  data: undefined,
  error: {
    code: 'SESSION_INVALID',
    detail: 'untrusted detail',
    request_id: 'me-request',
    status: 401,
    title: 'untrusted title',
    type: 'urn:nodecontroll:problem:session-invalid',
  },
  response: response(401),
})
const authenticatedResult = () => ({
  data: {
    data: {
      actor: {
        capabilities: ['system:read'],
        force_password_change: false,
        id: '01900000-0000-7000-8000-000000000001',
        role: 'owner',
        username: 'owner',
      },
      session: {
        absolute_expires_at_ms: 2_000,
        created_at_ms: 1_000,
        id: '01900000-0000-7000-8000-000000000002',
        idle_expires_at_ms: 1_500,
        last_seen_at_ms: 1_000,
      },
    },
    meta: { api_version: 'v1', request_id: 'me-request' },
  },
  error: undefined,
  response: response(200),
})

beforeEach(() => {
  for (const mock of Object.values(sdk)) mock.mockReset()
})

describe('authentication router guard', () => {
  it('forces an uninitialized instance to setup without probing /me', async () => {
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(false))
    const router = createAppRouter(createMemoryHistory(), createPinia())

    await router.push('/system')

    expect(router.currentRoute.value.name).toBe('setup')
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
  })

  it('sends an initialized anonymous actor to login with the protected local path', async () => {
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(true))
    sdk.getCurrentActor.mockResolvedValue(anonymousResult())
    const router = createAppRouter(createMemoryHistory(), createPinia())

    await router.push('/system?panel=readiness')

    expect(router.currentRoute.value.name).toBe('login')
    expect(router.currentRoute.value.query.redirect).toBe('/system?panel=readiness')
  })

  it('keeps authenticated actors away from login and setup', async () => {
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(true))
    sdk.getCurrentActor.mockResolvedValue(authenticatedResult())
    const router = createAppRouter(createMemoryHistory(), createPinia())

    await router.push('/login?redirect=/system')
    expect(router.currentRoute.value.fullPath).toBe('/system')

    await router.push('/setup')
    expect(router.currentRoute.value.fullPath).toBe('/')
  })
})

describe('safeRedirectPath', () => {
  it('accepts a single-slash in-app path and rejects external or ambiguous targets', () => {
    expect(safeRedirectPath('/system?panel=readiness#database')).toBe(
      '/system?panel=readiness#database',
    )
    expect(safeRedirectPath('https://attacker.example/system')).toBe('/')
    expect(safeRedirectPath('//attacker.example/system')).toBe('/')
    expect(safeRedirectPath('/%2f%2fattacker.example')).toBe('/')
    expect(safeRedirectPath('/%5cattacker.example')).toBe('/')
    expect(safeRedirectPath('/system%0a/hidden')).toBe('/')
    expect(safeRedirectPath(['/system'])).toBe('/')
    expect(safeRedirectPath('/login')).toBe('/')
  })
})
