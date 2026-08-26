import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/vue'
import { createPinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

import { vuetify } from '../../plugins/vuetify'

const sdk = vi.hoisted(() => ({
  getBootstrapState: vi.fn(),
  getCurrentActor: vi.fn(),
  login: vi.fn(),
  logout: vi.fn(),
}))

vi.mock('../../api/generated/sdk.gen', () => sdk)

import LoginPage from '../LoginPage.vue'

const response = (status: number) => ({ headers: new Headers(), status })
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
    meta: { api_version: 'v1', request_id: 'login-request' },
  },
  error: undefined,
  response: response(200),
})

const renderLogin = async (target = '/login') => {
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/', name: 'dashboard', component: { render: () => null } },
      { path: '/login', name: 'login', component: { render: () => null } },
      { path: '/setup', name: 'setup', component: { render: () => null } },
      { path: '/system', name: 'system', component: { render: () => null } },
    ],
  })
  await router.push(target)
  return {
    ...render(LoginPage, {
      global: { plugins: [createPinia(), router, vuetify] },
    }),
    router,
  }
}

beforeEach(() => {
  for (const mock of Object.values(sdk)) mock.mockReset()
})

afterEach(() => {
  cleanup()
})

describe('LoginPage', () => {
  it('uses same-origin credentials and never renders the server Problem text or submitted password', async () => {
    const password = 'do-not-render-owner-password'
    sdk.login.mockResolvedValue({
      data: undefined,
      error: {
        code: 'INVALID_CREDENTIALS',
        detail: `untrusted detail ${password}`,
        request_id: 'login-error',
        status: 401,
        title: 'untrusted server title',
        type: 'urn:nodecontroll:problem:invalid-credentials',
      },
      response: response(401),
    })
    await renderLogin()

    await fireEvent.update(screen.getByLabelText('用户名'), 'owner')
    await fireEvent.update(screen.getByLabelText('密码'), password)
    await fireEvent.click(screen.getByRole('button', { name: '登录' }))

    const alert = await screen.findByTestId('login-error')
    expect(alert.textContent).toContain('用户名或密码错误')
    expect(document.body.textContent).not.toContain('untrusted server title')
    expect(document.body.textContent).not.toContain('untrusted detail')
    expect(document.body.textContent).not.toContain(password)
    expect((screen.getByLabelText('密码') as HTMLInputElement).value).toBe('')
    expect(sdk.login).toHaveBeenCalledWith({
      credentials: 'same-origin',
      body: { username: 'owner', password },
    })
  })

  it('replaces the route with an allowed in-app redirect after login', async () => {
    sdk.login.mockResolvedValue(authenticatedResult())
    const { router } = await renderLogin('/login?redirect=/system')

    await fireEvent.update(screen.getByLabelText('用户名'), 'owner')
    await fireEvent.update(screen.getByLabelText('密码'), 'owner-password-2026')
    await fireEvent.click(screen.getByRole('button', { name: '登录' }))

    await waitFor(() => expect(router.currentRoute.value.fullPath).toBe('/system'))
    expect((screen.getByLabelText('密码') as HTMLInputElement).value).toBe('')
  })

  it('clears the password and prevents another login when authenticated navigation fails', async () => {
    sdk.login.mockResolvedValue(authenticatedResult())
    const { router } = await renderLogin('/login?redirect=/system')
    const replace = vi.spyOn(router, 'replace').mockRejectedValueOnce(new Error('navigation failed'))

    await fireEvent.update(screen.getByLabelText('用户名'), 'owner')
    await fireEvent.update(screen.getByLabelText('密码'), 'owner-password-2026')
    await fireEvent.click(screen.getByRole('button', { name: '登录' }))

    expect((await screen.findByTestId('login-navigation-error')).textContent).toContain(
      '登录状态已经更新',
    )
    expect((screen.getByLabelText('密码') as HTMLInputElement).value).toBe('')
    await fireEvent.update(screen.getByLabelText('密码'), 'must-not-create-another-session')
    expect((screen.getByRole('button', { name: '登录' }) as HTMLButtonElement).disabled).toBe(true)
    expect(sdk.login).toHaveBeenCalledTimes(1)

    await fireEvent.click(screen.getByRole('button', { name: '重试进入控制台' }))
    await waitFor(() => expect(router.currentRoute.value.fullPath).toBe('/system'))
    expect(replace).toHaveBeenCalledTimes(2)
    expect(sdk.login).toHaveBeenCalledTimes(1)
  })
})
