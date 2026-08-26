import { cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/vue'
import { createPinia } from 'pinia'
import { RouterView, createMemoryHistory, createRouter } from 'vue-router'
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

const recoveryApi = vi.hoisted(() => ({
  getRecoveryCodeStatus: vi.fn(),
  initializeControlPlaneWithRecoveryCodes: vi.fn(),
  regenerateRecoveryCodes: vi.fn(),
  validOneTimeRecoveryCodes: (value: unknown) => Array.isArray(value) && value.length === 8,
}))

vi.mock('../../api/generated/sdk.gen', () => sdk)
vi.mock('../../api/recovery-codes', () => recoveryApi)

import { CSRF_COOKIE_NAME, useSessionStore } from '../../stores/session'
import { CREDENTIAL_COORDINATION_KEY } from '../../lib/credential-coordinator'
import { useOneTimeRecoveryCodeStore } from '../../stores/one-time-recovery'
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
const recoveryCodes = Array.from(
  { length: 8 },
  (_, index) => `${index.toString(16).padStart(4, '0')}-1111-2222-3333-4444-5555-6666-7777`,
)

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

const recoveryStatusResult = (remainingCount = 8) => ({
  data: {
    data: {
      created_at_ms: now - 30_000,
      remaining_count: remainingCount,
      set_version: 1,
      total_count: 8,
    },
    meta: { api_version: 'v1', request_id: 'recovery-status-request' },
  },
  error: undefined,
  response: response(200),
})

const recoveryRegenerationResult = () => ({
  data: {
    data: {
      created_at_ms: now,
      one_time_recovery_codes: [...recoveryCodes],
      set_version: 2,
    },
    meta: { api_version: 'v1', request_id: 'recovery-regeneration-request' },
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
        component: ProfileSecurityPage,
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
    ...render(RouterView, { global: { plugins: [pinia, router, vuetify] } }),
    pinia,
    router,
    session,
  }
}

beforeEach(() => {
  for (const mock of Object.values(sdk)) mock.mockReset()
  recoveryApi.getRecoveryCodeStatus.mockReset()
  recoveryApi.regenerateRecoveryCodes.mockReset()
  sdk.listCurrentSessions.mockResolvedValue(sessionListResult())
  recoveryApi.getRecoveryCodeStatus.mockResolvedValue(recoveryStatusResult())
})

afterEach(() => {
  cleanup()
  vi.restoreAllMocks()
})

describe('ProfileSecurityPage', () => {
  it('shows only recovery-code metadata and never fetches the code plaintext on status read', async () => {
    await renderPage()

    const status = await screen.findByTestId('recovery-code-status')
    expect(status.textContent).toContain('剩余 8 / 8 枚')
    expect(status.textContent).toContain('第 1 组')
    expect(recoveryApi.getRecoveryCodeStatus).toHaveBeenCalledWith({ signal: expect.anything() })
    expect(document.body.textContent).not.toContain(recoveryCodes[0])
  })

  it('regenerates once after recent auth and clears the only plaintext copy on confirmation', async () => {
    const csrf = `ncc1_${'6'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    recoveryApi.regenerateRecoveryCodes.mockResolvedValue(recoveryRegenerationResult())
    recoveryApi.getRecoveryCodeStatus
      .mockResolvedValueOnce(recoveryStatusResult())
      .mockResolvedValueOnce(recoveryStatusResult(8))
    await renderPage()

    await screen.findByTestId('recovery-code-status')
    await fireEvent.click(screen.getByTestId('regenerate-recovery-codes'))
    const dialog = await screen.findByRole('dialog')
    await fireEvent.click(within(dialog).getByRole('button', { name: '确认重新生成' }))

    expect(await screen.findByTestId('one-time-recovery-codes')).not.toBeNull()
    expect(screen.getAllByTestId('recovery-code')).toHaveLength(8)
    expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledTimes(1)
    expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledWith({
      csrfToken: csrf,
      signal: expect.anything(),
    })
    expect(globalThis.localStorage.getItem('nodecontroll:recovery-codes')).toBeNull()
    expect(globalThis.sessionStorage.length).toBe(0)
    const persistedValues = Array.from({ length: globalThis.localStorage.length }, (_, index) => {
      const key = globalThis.localStorage.key(index)
      return key ? globalThis.localStorage.getItem(key) : ''
    }).join('\n')
    expect(persistedValues).not.toContain(recoveryCodes[0])

    await fireEvent.click(screen.getByLabelText('我已把这组恢复码保存到安全位置'))
    await fireEvent.click(screen.getByTestId('confirm-recovery-codes'))
    expect(screen.queryByTestId('one-time-recovery-codes')).toBeNull()
    expect(await screen.findByTestId('recovery-codes-saved')).not.toBeNull()
    expect(document.body.textContent).not.toContain(recoveryCodes[0])
  })

  it('clears the page-scoped one-time store explicitly when the profile page unmounts', async () => {
    const rendered = await renderPage()
    const oneTimeCodes = useOneTimeRecoveryCodeStore(rendered.pinia)
    expect(oneTimeCodes.accept(recoveryCodes)).toBe(true)
    expect(oneTimeCodes.hasCodes).toBe(true)

    rendered.unmount()

    expect(oneTimeCodes.hasCodes).toBe(false)
    expect(oneTimeCodes.codes).toEqual([])
  })

  it('does not refill plaintext after navigation leaves and returns to the same full path', async () => {
    const csrf = `ncc1_${'8'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    const apiResponse = recoveryRegenerationResult()
    let resolveRegeneration!: (value: ReturnType<typeof recoveryRegenerationResult>) => void
    recoveryApi.regenerateRecoveryCodes.mockReturnValue(
      new Promise<ReturnType<typeof recoveryRegenerationResult>>((resolve) => {
        resolveRegeneration = resolve
      }),
    )
    const rendered = await renderPage()
    const oneTimeCodes = useOneTimeRecoveryCodeStore(rendered.pinia)

    await screen.findByTestId('recovery-code-status')
    await fireEvent.click(screen.getByTestId('regenerate-recovery-codes'))
    await fireEvent.click(
      within(await screen.findByRole('dialog')).getByRole('button', {
        name: '确认重新生成',
      }),
    )
    await waitFor(() => expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledTimes(1))
    expect(oneTimeCodes.accept(recoveryCodes)).toBe(true)
    await rendered.router.push('/')
    await waitFor(() => expect(oneTimeCodes.hasCodes).toBe(false))
    await rendered.router.push('/profile/security')
    resolveRegeneration(apiResponse)
    await waitFor(() => expect(rendered.session.recoveryCodeRegenerationPending).toBe(false))

    expect(oneTimeCodes.hasCodes).toBe(false)
    expect(screen.queryByTestId('one-time-recovery-codes')).toBeNull()
    expect(apiResponse.data.data.one_time_recovery_codes.every((code) => code === '')).toBe(true)
  })

  it('clears and abandons a pending plaintext handoff on pagehide for BFCache', async () => {
    const csrf = `ncc1_${'9'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    const apiResponse = recoveryRegenerationResult()
    let resolveRegeneration!: (value: ReturnType<typeof recoveryRegenerationResult>) => void
    recoveryApi.regenerateRecoveryCodes.mockReturnValue(
      new Promise<ReturnType<typeof recoveryRegenerationResult>>((resolve) => {
        resolveRegeneration = resolve
      }),
    )
    const rendered = await renderPage()
    const oneTimeCodes = useOneTimeRecoveryCodeStore(rendered.pinia)

    await screen.findByTestId('recovery-code-status')
    await fireEvent.click(screen.getByTestId('regenerate-recovery-codes'))
    await fireEvent.click(
      within(await screen.findByRole('dialog')).getByRole('button', {
        name: '确认重新生成',
      }),
    )
    await waitFor(() => expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledTimes(1))
    expect(oneTimeCodes.accept(recoveryCodes)).toBe(true)
    globalThis.dispatchEvent(new Event('pagehide'))
    expect(oneTimeCodes.hasCodes).toBe(false)
    resolveRegeneration(apiResponse)
    await waitFor(() => expect(rendered.session.recoveryCodeRegenerationPending).toBe(false))

    expect(oneTimeCodes.hasCodes).toBe(false)
    expect(screen.queryByTestId('one-time-recovery-codes')).toBeNull()
    expect(apiResponse.data.data.one_time_recovery_codes.every((code) => code === '')).toBe(true)
  })

  it('does not accept plaintext when the credential terminal settlement is lost', async () => {
    const csrf = `ncc1_${'e'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    const apiResponse = recoveryRegenerationResult()
    let resolveRegeneration!: (value: ReturnType<typeof recoveryRegenerationResult>) => void
    recoveryApi.regenerateRecoveryCodes.mockReturnValue(
      new Promise<ReturnType<typeof recoveryRegenerationResult>>((resolve) => {
        resolveRegeneration = resolve
      }),
    )
    const rendered = await renderPage()
    const oneTimeCodes = useOneTimeRecoveryCodeStore(rendered.pinia)

    await screen.findByTestId('recovery-code-status')
    await fireEvent.click(screen.getByTestId('regenerate-recovery-codes'))
    await fireEvent.click(
      within(await screen.findByRole('dialog')).getByRole('button', {
        name: '确认重新生成',
      }),
    )
    await waitFor(() => expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledTimes(1))
    const inflight = JSON.parse(
      globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) ?? '{}',
    )
    globalThis.localStorage.setItem(
      CREDENTIAL_COORDINATION_KEY,
      JSON.stringify({
        ...inflight,
        opId: '50000000-0000-4000-8000-000000000005',
      }),
    )
    resolveRegeneration(apiResponse)
    await waitFor(() => expect(rendered.session.status).toBe('relogin-required'))

    expect(oneTimeCodes.hasCodes).toBe(false)
    expect(screen.queryByTestId('one-time-recovery-codes')).toBeNull()
    expect(apiResponse.data.data.one_time_recovery_codes.every((code) => code === '')).toBe(true)
  })

  it('wipes the transfer array even if the page-scoped store rejects by throwing', async () => {
    const csrf = `ncc1_${'f'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    recoveryApi.regenerateRecoveryCodes.mockResolvedValue(recoveryRegenerationResult())
    const rendered = await renderPage()
    const oneTimeCodes = useOneTimeRecoveryCodeStore(rendered.pinia)
    let transferredCodes: string[] | undefined
    vi.spyOn(oneTimeCodes, 'acceptForOperation').mockImplementation((_owner, codes: unknown) => {
      transferredCodes = codes as string[]
      throw new Error('store rejected transfer')
    })

    await screen.findByTestId('recovery-code-status')
    await fireEvent.click(screen.getByTestId('regenerate-recovery-codes'))
    await fireEvent.click(
      within(await screen.findByRole('dialog')).getByRole('button', {
        name: '确认重新生成',
      }),
    )
    await screen.findByTestId('security-error')

    expect(transferredCodes).toBeDefined()
    expect(transferredCodes?.every((code) => code === '')).toBe(true)
    expect(oneTimeCodes.hasCodes).toBe(false)
    expect(screen.queryByTestId('one-time-recovery-codes')).toBeNull()
  })

  it('requires step-up before regeneration and never auto-replays it', async () => {
    const { router } = await renderPage(false)

    await screen.findByTestId('recovery-code-status')
    await fireEvent.click(screen.getByTestId('regenerate-recovery-codes'))

    await waitFor(() => expect(router.currentRoute.value.name).toBe('reauth'))
    expect(router.currentRoute.value.query.redirect).toBe('/profile/security')
    expect(recoveryApi.regenerateRecoveryCodes).not.toHaveBeenCalled()
  })

  it('steps up once when regeneration reaches the server-side recent-auth boundary', async () => {
    const csrf = `ncc1_${'7'.repeat(64)}`
    vi.spyOn(Document.prototype, 'cookie', 'get').mockReturnValue(`${CSRF_COOKIE_NAME}=${csrf}`)
    recoveryApi.regenerateRecoveryCodes.mockResolvedValue(
      problemResult(403, 'RECENT_AUTH_REQUIRED'),
    )
    const { router } = await renderPage()

    await screen.findByTestId('recovery-code-status')
    await fireEvent.click(screen.getByTestId('regenerate-recovery-codes'))
    await fireEvent.click(
      within(await screen.findByRole('dialog')).getByRole('button', {
        name: '确认重新生成',
      }),
    )

    await waitFor(() => expect(router.currentRoute.value.name).toBe('reauth'))
    expect(recoveryApi.regenerateRecoveryCodes).toHaveBeenCalledTimes(1)
    expect(document.body.textContent).not.toContain('untrusted server detail')
  })

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
