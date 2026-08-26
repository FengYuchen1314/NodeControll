import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/vue'
import { createPinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

import { vuetify } from '../../plugins/vuetify'

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

vi.mock('../../api/generated/sdk.gen', () => sdk)

import { CSRF_COOKIE_NAME, useSessionStore } from '../../stores/session'
import ReauthenticatePage from '../ReauthenticatePage.vue'

const response = (status: number) => ({ headers: new Headers(), status })
const projection = () => ({
  actor: {
    capabilities: ['system:read'],
    force_password_change: false,
    id: '01900000-0000-7000-8000-000000000001',
    role: 'owner',
    username: 'owner',
  },
  session: {
    absolute_expires_at_ms: Date.now() + 600_000,
    auth_level: 'password',
    created_at_ms: Date.now() - 2_000,
    id: '01900000-0000-7000-8000-000000000003',
    idle_expires_at_ms: Date.now() + 300_000,
    last_seen_at_ms: Date.now(),
    recent_auth_expires_at_ms: Date.now() + 60_000,
  },
})

const authenticatedResult = () => ({
  data: {
    data: projection(),
    meta: { api_version: 'v1', request_id: 'reauth-request' },
  },
  error: undefined,
  response: response(200),
})

const renderPage = async () => {
  const pinia = createPinia()
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/', name: 'dashboard', component: { render: () => null } },
      { path: '/reauth', name: 'reauth', component: { render: () => null } },
      { path: '/system', name: 'system', component: { render: () => null } },
      {
        path: '/profile/security/password',
        name: 'password-change',
        component: { render: () => null },
      },
    ],
  })
  await router.push('/reauth?redirect=/system')
  const session = useSessionStore(pinia)
  session.acceptAuthenticated(projection())
  return {
    ...render(ReauthenticatePage, { global: { plugins: [pinia, router, vuetify] } }),
    router,
    session,
  }
}

beforeEach(() => {
  for (const mock of Object.values(sdk)) mock.mockReset()
})

afterEach(() => {
  cleanup()
  vi.restoreAllMocks()
})

describe('ReauthenticatePage', () => {
  it('clears the password before navigation and never submits it again after success', async () => {
    const csrf = `ncc1_${'d'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.reauthenticate.mockResolvedValue(authenticatedResult())
    const { router } = await renderPage()

    await fireEvent.update(screen.getByLabelText('当前密码'), 'sensitive-current-password')
    await fireEvent.click(screen.getByRole('button', { name: '确认身份' }))

    await waitFor(() => expect(router.currentRoute.value.fullPath).toBe('/system'))
    expect((screen.getByLabelText('当前密码') as HTMLInputElement).value).toBe('')
    expect(document.body.textContent).not.toContain('sensitive-current-password')
    expect(sdk.reauthenticate).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      body: { method: 'password', password: 'sensitive-current-password' },
      signal: expect.anything(),
    })
    await fireEvent.update(screen.getByLabelText('当前密码'), 'must-not-submit-again')
    expect((screen.getByRole('button', { name: '确认身份' }) as HTMLButtonElement).disabled).toBe(
      true,
    )
    expect(sdk.reauthenticate).toHaveBeenCalledTimes(1)
  })

  it('treats a resolved Vue Router navigation failure as a failure without reauthenticating again', async () => {
    const csrf = `ncc1_${'e'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.reauthenticate.mockResolvedValue(authenticatedResult())
    const { router } = await renderPage()
    const removeGuard = router.beforeEach((to) => (to.path === '/system' ? false : true))

    await fireEvent.update(screen.getByLabelText('当前密码'), 'sensitive-current-password')
    await fireEvent.click(screen.getByRole('button', { name: '确认身份' }))

    expect(await screen.findByTestId('reauth-navigation-error')).not.toBeNull()
    expect(router.currentRoute.value.name).toBe('reauth')
    removeGuard()
    await fireEvent.click(screen.getByRole('button', { name: '重试页面跳转' }))
    await waitFor(() => expect(router.currentRoute.value.name).toBe('system'))
    expect(sdk.reauthenticate).toHaveBeenCalledTimes(1)
  })

  it('maps only the whitelisted proof error and never renders server problem text', async () => {
    const csrf = `ncc1_${'f'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.reauthenticate.mockResolvedValue({
      data: undefined,
      error: {
        code: 'REAUTHENTICATION_FAILED',
        detail: 'untrusted proof detail',
        title: 'untrusted proof title',
      },
      response: response(403),
    })
    await renderPage()

    await fireEvent.update(screen.getByLabelText('当前密码'), 'wrong-password')
    await fireEvent.click(screen.getByRole('button', { name: '确认身份' }))

    const alert = await screen.findByTestId('reauth-error')
    expect(alert.textContent).toContain('密码不正确')
    expect(document.body.textContent).not.toContain('untrusted proof')
    expect(document.body.textContent).not.toContain('wrong-password')
  })
})
