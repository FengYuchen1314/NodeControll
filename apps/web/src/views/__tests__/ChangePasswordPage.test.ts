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
import ChangePasswordPage from '../ChangePasswordPage.vue'

const response = (status: number) => ({ headers: new Headers(), status })
const projection = (options: { force?: boolean; recent?: boolean; sessionId?: string } = {}) => ({
  actor: {
    capabilities: ['system:read'],
    force_password_change: options.force ?? false,
    id: '01900000-0000-7000-8000-000000000001',
    role: 'owner',
    username: 'owner',
  },
  session: {
    absolute_expires_at_ms: Date.now() + 600_000,
    auth_level: 'password',
    created_at_ms: Date.now() - 2_000,
    id: options.sessionId ?? '01900000-0000-7000-8000-000000000002',
    idle_expires_at_ms: Date.now() + 300_000,
    last_seen_at_ms: Date.now(),
    recent_auth_expires_at_ms: Date.now() + (options.recent === false ? -1_000 : 60_000),
  },
})

const passwordChangedResult = () => ({
  data: {
    data: {
      ...projection({ sessionId: '01900000-0000-7000-8000-000000000003' }),
      revoked_sessions: 2,
    },
    meta: { api_version: 'v1', request_id: 'password-request' },
  },
  error: undefined,
  response: response(200),
})

const renderPage = async (recent = true) => {
  const pinia = createPinia()
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/', name: 'dashboard', component: { render: () => null } },
      { path: '/login', name: 'login', component: { render: () => null } },
      { path: '/reauth', name: 'reauth', component: { render: () => null } },
      {
        path: '/profile/security',
        name: 'profile-security',
        component: { render: () => null },
      },
      {
        path: '/profile/security/password',
        name: 'password-change',
        component: { render: () => null },
      },
    ],
  })
  await router.push('/profile/security/password?redirect=/profile/security')
  const session = useSessionStore(pinia)
  session.acceptAuthenticated(projection({ force: true, recent }))
  return {
    ...render(ChangePasswordPage, { global: { plugins: [pinia, router, vuetify] } }),
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

describe('ChangePasswordPage', () => {
  it('never sends the confirmation and clears both password fields before navigating', async () => {
    const csrf = `ncc1_${'1'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.changeCurrentPassword.mockResolvedValue(passwordChangedResult())
    const { router, session } = await renderPage()
    const newPassword = 'a-secure-new-password'

    await fireEvent.update(screen.getByLabelText('新密码'), newPassword)
    await fireEvent.update(screen.getByLabelText('确认新密码'), newPassword)
    await fireEvent.click(screen.getByRole('button', { name: '修改密码并轮换会话' }))

    await waitFor(() => expect(router.currentRoute.value.name).toBe('profile-security'))
    expect(sdk.changeCurrentPassword).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      body: { new_password: newPassword },
      signal: expect.anything(),
    })
    expect(JSON.stringify(sdk.changeCurrentPassword.mock.calls[0])).not.toContain('confirmation')
    expect((screen.getByLabelText('新密码') as HTMLInputElement).value).toBe('')
    expect((screen.getByLabelText('确认新密码') as HTMLInputElement).value).toBe('')
    expect(session.passwordChangeRequired).toBe(false)
  })

  it('sends an expired recent-auth session to step-up without replaying the password', async () => {
    const { router } = await renderPage(false)
    const newPassword = 'another-secure-password'

    await fireEvent.update(screen.getByLabelText('新密码'), newPassword)
    await fireEvent.update(screen.getByLabelText('确认新密码'), newPassword)
    await fireEvent.click(screen.getByRole('button', { name: '修改密码并轮换会话' }))

    await waitFor(() => expect(router.currentRoute.value.name).toBe('reauth'))
    expect(router.currentRoute.value.query.redirect).toContain('/profile/security/password')
    expect(sdk.changeCurrentPassword).not.toHaveBeenCalled()
    expect(document.body.textContent).not.toContain(newPassword)
  })

  it('locks the form and requires relogin when the transport outcome is unknown', async () => {
    const csrf = `ncc1_${'2'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.changeCurrentPassword.mockRejectedValue(new TypeError('network interrupted'))
    sdk.logout.mockResolvedValue({ data: {}, error: undefined, response: response(204) })
    const { session } = await renderPage()
    const newPassword = 'unknown-outcome-password'

    await fireEvent.update(screen.getByLabelText('新密码'), newPassword)
    await fireEvent.update(screen.getByLabelText('确认新密码'), newPassword)
    await fireEvent.click(screen.getByRole('button', { name: '修改密码并轮换会话' }))

    const alert = await screen.findByTestId('password-change-error')
    expect(alert.textContent).toContain('不会自动重试')
    expect(alert.textContent).toContain('重新登录')
    expect(sdk.getCurrentActor).not.toHaveBeenCalled()
    expect(sdk.logout).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      signal: expect.anything(),
    })
    expect(session.status).toBe('anonymous')
    expect((screen.getByLabelText('新密码') as HTMLInputElement).disabled).toBe(true)
    expect(document.body.textContent).not.toContain(newPassword)

    await fireEvent.update(screen.getByLabelText('新密码'), 'must-not-replay-password')
    await fireEvent.click(screen.getByRole('button', { name: '修改密码并轮换会话' }))
    expect(sdk.changeCurrentPassword).toHaveBeenCalledTimes(1)
  })
})
