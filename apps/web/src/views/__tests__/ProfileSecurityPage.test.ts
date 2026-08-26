import { cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/vue'
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
import ProfileSecurityPage from '../ProfileSecurityPage.vue'

const response = (status: number) => ({ headers: new Headers(), status })
const now = Date.now()
const projection = (recent = true) => ({
  actor: {
    capabilities: ['system:read'],
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
    recent_auth_expires_at_ms: now + (recent ? 60_000 : -1_000),
  },
})
const currentSession = {
  ...projection().session,
  is_current: true,
}
const otherSession = {
  ...projection().session,
  auth_level: 'phishing_resistant',
  created_at_ms: now - 20_000,
  id: '01900000-0000-7000-8000-000000000004',
  is_current: false,
  last_seen_at_ms: now - 10_000,
}

const sessionListResult = () => ({
  data: {
    data: { sessions: [currentSession, otherSession] },
    meta: { api_version: 'v1', request_id: 'sessions-request' },
  },
  error: undefined,
  response: response(200),
})

const problemResult = (status: number, code: string) => ({
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

const logoutOthersResult = () => ({
  data: {
    data: {
      ...projection(),
      session: {
        ...projection().session,
        id: '01900000-0000-7000-8000-000000000005',
        last_seen_at_ms: now + 1_000,
      },
      revoked_sessions: 1,
    },
    meta: { api_version: 'v1', request_id: 'logout-all-request' },
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
  await router.push('/profile/security')
  const session = useSessionStore(pinia)
  session.acceptAuthenticated(projection(recent))
  return {
    ...render(ProfileSecurityPage, { global: { plugins: [pinia, router, vuetify] } }),
    router,
    session,
  }
}

beforeEach(() => {
  for (const mock of Object.values(sdk)) mock.mockReset()
  sdk.listCurrentSessions.mockResolvedValue(sessionListResult())
})

afterEach(() => {
  cleanup()
  vi.restoreAllMocks()
})

describe('ProfileSecurityPage', () => {
  it('shows only coarse session data and revokes one non-current session after confirmation', async () => {
    const csrf = `ncc1_${'3'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.revokeCurrentUserSession.mockResolvedValue({
      data: undefined,
      error: undefined,
      response: response(204),
    })
    await renderPage()

    expect(await screen.findByText('其他会话')).not.toBeNull()
    expect(screen.getByText('抗钓鱼认证')).not.toBeNull()
    expect(document.body.textContent).not.toContain('User-Agent')
    expect(document.body.textContent).not.toContain('192.0.2.')
    await fireEvent.click(screen.getByRole('button', { name: '撤销' }))
    const cancel = await screen.findByRole('button', { name: '取消' })
    expect(cancel.hasAttribute('autofocus')).toBe(true)
    await fireEvent.click(screen.getByRole('button', { name: '确认撤销' }))

    await waitFor(() => expect(screen.queryByText('其他会话')).toBeNull())
    expect(sdk.revokeCurrentUserSession).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      path: { session_id: otherSession.id },
      signal: expect.anything(),
    })
  })

  it('steps up once when the server rejects revocation at the recent-auth boundary', async () => {
    const csrf = `ncc1_${'5'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.revokeCurrentUserSession.mockResolvedValue(problemResult(403, 'RECENT_AUTH_REQUIRED'))
    const { router, session } = await renderPage()

    expect(await screen.findByText('其他会话')).not.toBeNull()
    await fireEvent.click(screen.getByRole('button', { name: '撤销' }))
    await fireEvent.click(await screen.findByRole('button', { name: '确认撤销' }))

    await waitFor(() => expect(router.currentRoute.value.name).toBe('reauth'))
    expect(router.currentRoute.value.query.redirect).toBe('/profile/security')
    expect(sdk.revokeCurrentUserSession).toHaveBeenCalledTimes(1)
    expect(session.session?.id).toBe(currentSession.id)
    expect(document.body.textContent).not.toContain('untrusted server detail')
  })

  it('requires step-up before a dangerous action and never auto-replays the action', async () => {
    const { router } = await renderPage(false)

    await screen.findByText('其他会话')
    await fireEvent.click(screen.getByRole('button', { name: '退出其他会话' }))
    const confirmationDialog = await screen.findByRole('dialog')
    await fireEvent.click(within(confirmationDialog).getByRole('button', { name: '退出其他会话' }))

    await waitFor(() => expect(router.currentRoute.value.name).toBe('reauth'))
    expect(router.currentRoute.value.query.redirect).toBe('/profile/security')
    expect(sdk.logoutAll).not.toHaveBeenCalled()
  })

  it('keeps logout-other and logout-everywhere requests and outcomes distinct', async () => {
    const csrf = `ncc1_${'4'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    sdk.logoutAll
      .mockResolvedValueOnce(logoutOthersResult())
      .mockResolvedValueOnce({ data: undefined, error: undefined, response: response(204) })
    const { router, session } = await renderPage()

    await screen.findByText('其他会话')
    await fireEvent.click(screen.getByRole('button', { name: '退出其他会话' }))
    const otherDialog = await screen.findByRole('dialog')
    await fireEvent.click(within(otherDialog).getByRole('button', { name: '退出其他会话' }))
    expect((await screen.findByTestId('logout-others-result')).textContent).toContain(
      '共撤销 1 个会话',
    )
    expect(screen.getByText('当前浏览器').closest('.v-list-item')?.textContent).toContain(
      '登录时间',
    )
    expect(screen.queryByText('其他会话')).toBeNull()
    expect(session.session?.id).toBe('01900000-0000-7000-8000-000000000005')

    await fireEvent.click(screen.getByRole('button', { name: '退出所有会话' }))
    await fireEvent.click(await screen.findByRole('button', { name: '确认全部退出' }))
    await waitFor(() => expect(router.currentRoute.value.name).toBe('login'))
    expect(sdk.logoutAll).toHaveBeenNthCalledWith(1, {
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      body: { keep_current: true },
      signal: expect.anything(),
    })
    expect(sdk.logoutAll).toHaveBeenNthCalledWith(2, {
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-csrf': csrf },
      body: { keep_current: false },
      signal: expect.anything(),
    })
  })
})
