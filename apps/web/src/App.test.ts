import { cleanup, render, screen, waitFor } from '@testing-library/vue'
import { createPinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import { h } from 'vue'
import { afterEach, describe, expect, it } from 'vitest'

import App from './App.vue'
import { i18n } from './plugins/i18n'
import { vuetify } from './plugins/vuetify'
import { useSessionStore } from './stores/session'

afterEach(() => {
  cleanup()
})

describe('App session boundary', () => {
  it('removes a protected route from the DOM as soon as the session becomes anonymous', async () => {
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
      ],
    })
    await router.push('/')
    await router.isReady()

    const session = useSessionStore(pinia)
    session.$patch({
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
      status: 'authenticated',
    })

    render(App, { global: { plugins: [pinia, router, vuetify, i18n] } })
    expect(await screen.findByText('SENSITIVE-PROTECTED-CONTENT')).not.toBeNull()

    session.markInitialized()

    await waitFor(() => {
      expect(screen.queryByText('SENSITIVE-PROTECTED-CONTENT')).toBeNull()
    })
    expect(screen.getByTestId('protected-route-session-gate').textContent).toContain(
      '受保护页面已关闭',
    )
  })
})
