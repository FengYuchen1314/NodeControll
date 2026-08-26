import { createPinia } from 'pinia'
import { createMemoryHistory } from 'vue-router'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const sdk = vi.hoisted(() => ({
  changeCurrentPassword: vi.fn(),
  getBootstrapState: vi.fn(),
  getCurrentActor: vi.fn(),
  getReadiness: vi.fn(),
  getSystemVersion: vi.fn(),
  initializeControlPlane: vi.fn(),
  listCurrentSessions: vi.fn(),
  login: vi.fn(),
  logout: vi.fn(),
  logoutAll: vi.fn(),
  reauthenticate: vi.fn(),
  revokeCurrentUserSession: vi.fn(),
}))

vi.mock('../api/generated/sdk.gen', () => sdk)

import { accessRedirect, createAppRouter, safeRedirectPath } from './index'
import { appRouteNames } from './route-names'

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
const authenticatedResult = (
  capabilities = ['credentials:manage', 'sessions:read', 'system:read'],
) => {
  const now = Date.now()
  return {
    data: {
    data: {
      actor: {
        capabilities,
        force_password_change: false,
        id: '01900000-0000-7000-8000-000000000001',
        role: 'owner',
        username: 'owner',
      },
      session: {
        absolute_expires_at_ms: now + 600_000,
        auth_level: 'password',
        created_at_ms: now - 2_000,
        id: '01900000-0000-7000-8000-000000000002',
        idle_expires_at_ms: now + 300_000,
        last_seen_at_ms: now,
        recent_auth_expires_at_ms: now + 60_000,
      },
    },
    meta: { api_version: 'v1', request_id: 'me-request' },
  },
  error: undefined,
  response: response(200),
  }
}

beforeEach(() => {
  for (const mock of Object.values(sdk)) mock.mockReset()
})

describe('authentication router guard', () => {
  it('treats a relogin-required quarantine as anonymous without a login redirect loop', () => {
    expect(
      accessRedirect(
        {
          fullPath: '/system?panel=readiness',
          meta: { requiresAuth: true },
          name: 'system',
          query: {},
        },
        'relogin-required',
      ),
    ).toEqual({
      name: 'login',
      query: { redirect: '/system?panel=readiness' },
    })
    expect(
      accessRedirect(
        {
          fullPath: '/login',
          meta: { guestOnly: true },
          name: 'login',
          query: {},
        },
        'relogin-required',
      ),
    ).toBeUndefined()
  })

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

  it('applies setup, anonymous, forced-password, recent-auth, then guest rules in order', async () => {
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(true))
    const forced = authenticatedResult()
    forced.data.data.actor.force_password_change = true
    sdk.getCurrentActor.mockResolvedValue(forced)
    const router = createAppRouter(createMemoryHistory(), createPinia())

    await router.push('/system')
    expect(router.currentRoute.value.name).toBe('password-change')
    expect(router.currentRoute.value.query.redirect).toBe('/system')

    await router.push('/profile/security')
    expect(router.currentRoute.value.name).toBe('profile-security')

    await router.push('/login?redirect=/system')
    expect(router.currentRoute.value.name).toBe('password-change')
  })

  it('sends an expired recent-auth projection to step-up without creating a redirect loop', async () => {
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(true))
    const expired = authenticatedResult()
    expired.data.data.session.recent_auth_expires_at_ms = Date.now() - 1
    sdk.getCurrentActor.mockResolvedValue(expired)
    const router = createAppRouter(createMemoryHistory(), createPinia())

    await router.push('/profile/security/password')

    expect(router.currentRoute.value.name).toBe('reauth')
    expect(router.currentRoute.value.query.redirect).toBe('/profile/security/password')
  })

  it('rejects a direct route when the actor lacks every required capability', async () => {
    sdk.getBootstrapState.mockResolvedValue(bootstrapResult(true))
    sdk.getCurrentActor.mockResolvedValue(authenticatedResult([]))
    const router = createAppRouter(createMemoryHistory(), createPinia())

    await router.push('/system')

    expect(router.currentRoute.value.name).toBe('dashboard')
  })

  it('keeps the dashboard capability-free and validates protected metadata invariants', () => {
    const router = createAppRouter(createMemoryHistory(), createPinia())
    const dashboard = router.resolve({ name: 'dashboard' }).matched.at(-1)
    const reauthenticate = router.resolve({ name: appRouteNames.reauthenticate }).matched.at(-1)
    const profileSecurity = router.resolve({ name: appRouteNames.profileSecurity }).matched.at(-1)
    const passwordChange = router.resolve({ name: appRouteNames.passwordChange }).matched.at(-1)

    expect(dashboard?.meta.requiresAuth).toBe(true)
    expect(dashboard?.meta.requiredCapabilities).toBeUndefined()
    expect(reauthenticate?.meta.requiredCapabilities).toBeUndefined()
    expect(profileSecurity?.meta.requiredCapabilities).toEqual([
      'sessions:read',
      'credentials:manage',
    ])
    expect(passwordChange?.meta.requiredCapabilities).toEqual(['credentials:manage'])
    for (const record of router.getRoutes()) {
      if (record.meta.requiredCapabilities || record.meta.requiresRecentAuth) {
        expect(record.meta.requiresAuth).toBe(true)
      }
    }
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
    expect(safeRedirectPath('/system?panel=%250a')).toBe('/')
    expect(safeRedirectPath('/system#%255cadmin')).toBe('/')
    expect(safeRedirectPath('/%252f%252fattacker.example')).toBe('/')
    expect(safeRedirectPath(['/system'])).toBe('/')
    expect(safeRedirectPath('/login')).toBe('/')
    expect(safeRedirectPath('/setup')).toBe('/')
    expect(safeRedirectPath('/rea%75th')).toBe('/')
    expect(safeRedirectPath('/LOGIN')).toBe('/')
  })
})
