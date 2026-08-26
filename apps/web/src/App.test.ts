import { cleanup, render, screen, waitFor } from '@testing-library/vue'
import { createPinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import { h } from 'vue'
import { afterEach, describe, expect, it } from 'vitest'

import App from './App.vue'
import { i18n } from './plugins/i18n'
import { vuetify } from './plugins/vuetify'
import { useSessionStore } from './stores/session'

const authenticatedProjection = (forcePasswordChange = false) => ({
  actor: {
    capabilities: ['system:read'],
    force_password_change: forcePasswordChange,
    id: '01900000-0000-7000-8000-000000000001',
    role: 'owner',
    username: 'owner',
  },
  session: {
    absolute_expires_at_ms: Date.now() + 600_000,
    auth_level: 'password',
    created_at_ms: Date.now() - 1_000,
    id: '01900000-0000-7000-8000-000000000002',
    idle_expires_at_ms: Date.now() + 300_000,
    last_seen_at_ms: Date.now(),
    recent_auth_expires_at_ms: Date.now() + 60_000,
  },
})

const renderProtectedApp = async () => {
  const pinia = createPinia()
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      {
        path: '/',
        name: 'dashboard',
        component: { render: () => h('div', 'SENSITIVE-PROTECTED-CONTENT') },
        meta: { requiresAuth: true, title: '总览' },
      },
      {
        path: '/login',
        name: 'login',
        component: { render: () => h('div', 'LOGIN-PAGE') },
        meta: { guestOnly: true, title: '登录' },
      },
      {
        path: '/setup',
        name: 'setup',
        component: { render: () => h('div', 'SETUP-PAGE') },
        meta: { guestOnly: true, title: '初始化' },
      },
      {
        path: '/system',
        name: 'system',
        component: { render: () => h('div', 'SYSTEM-PAGE') },
        meta: { requiresAuth: true, title: '系统' },
      },
      {
        path: '/profile/security',
        name: 'profile-security',
        component: { render: () => h('div', 'SECURITY-PAGE') },
        meta: { allowDuringPasswordChange: true, requiresAuth: true, title: '账户安全' },
      },
      {
        path: '/profile/security/password',
        name: 'password-change',
        component: { render: () => h('div', 'PASSWORD-PAGE') },
        meta: { allowDuringPasswordChange: true, requiresAuth: true, title: '修改密码' },
      },
    ],
  })
  await router.push('/')
  await router.isReady()

  const session = useSessionStore(pinia)
  session.acceptAuthenticated(authenticatedProjection())
  render(App, { global: { plugins: [pinia, router, vuetify, i18n] } })
  expect(await screen.findByText('SENSITIVE-PROTECTED-CONTENT')).not.toBeNull()
  return { router, session }
}

afterEach(() => {
  cleanup()
})

describe('App session boundary', () => {
  it('removes a protected route from the DOM as soon as the session becomes anonymous', async () => {
    const { session } = await renderProtectedApp()

    session.markInitialized()

    await waitFor(() => {
      expect(screen.queryByText('SENSITIVE-PROTECTED-CONTENT')).toBeNull()
    })
    expect(screen.getByTestId('protected-route-session-gate').textContent).toContain(
      '受保护页面已关闭',
    )
  })

  it('removes protected DOM even when logout cannot start without a CSRF cookie', async () => {
    const { session } = await renderProtectedApp()

    await expect(session.logout()).rejects.toMatchObject({ reason: 'csrf-unavailable' })

    await waitFor(() => {
      expect(screen.queryByText('SENSITIVE-PROTECTED-CONTENT')).toBeNull()
    })
    expect(session.status).toBe('relogin-required')
    expect(screen.getByTestId('protected-route-session-gate').textContent).toContain(
      '受保护页面已关闭',
    )
  })

  it('immediately removes ordinary protected content when forced password change becomes true', async () => {
    const { session } = await renderProtectedApp()

    session.acceptAuthenticated(authenticatedProjection(true))

    await waitFor(() => {
      expect(screen.queryByText('SENSITIVE-PROTECTED-CONTENT')).toBeNull()
    })
    expect(screen.getByTestId('password-restricted-route-gate').textContent).toContain(
      '请先修改密码',
    )
    expect(screen.getByText('修改密码')).not.toBeNull()
    expect(screen.getByText('账户安全')).not.toBeNull()
  })
})
