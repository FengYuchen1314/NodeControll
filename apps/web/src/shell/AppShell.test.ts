import { cleanup, fireEvent, render, screen, waitFor, within } from '@testing-library/vue'
import { createPinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import { defineComponent, h } from 'vue'
import { VApp } from 'vuetify/components'
import { afterEach, describe, expect, it } from 'vitest'

import { i18n } from '../plugins/i18n'
import { vuetify } from '../plugins/vuetify'
import { appRouteNames } from '../router/route-names'
import { useSessionStore } from '../stores/session'
import AppShell from './AppShell.vue'

const page = { template: '<div />' }
const makeRouter = () =>
  createRouter({
    history: createMemoryHistory(),
    routes: [
      {
        path: '/',
        name: appRouteNames.dashboard,
        component: page,
        meta: { requiresAuth: true, titleKey: 'routes.dashboard' },
      },
      {
        path: '/system',
        name: appRouteNames.system,
        component: page,
        meta: {
          requiredCapabilities: ['system:read'],
          requiresAuth: true,
          titleKey: 'routes.system',
        },
      },
      {
        path: '/profile/security',
        name: appRouteNames.profileSecurity,
        component: page,
        meta: {
          allowDuringPasswordChange: true,
          requiredCapabilities: ['sessions:read', 'credentials:manage'],
          requiresAuth: true,
          titleKey: 'routes.profileSecurity',
        },
      },
      {
        path: '/profile/security/password',
        name: appRouteNames.passwordChange,
        component: page,
        meta: {
          allowDuringPasswordChange: true,
          requiredCapabilities: ['credentials:manage'],
          requiresAuth: true,
          titleKey: 'routes.passwordChange',
        },
      },
    ],
  })

const renderShell = async (capabilities: string[]) => {
  const originalWidth = globalThis.innerWidth
  Object.defineProperty(globalThis, 'innerWidth', { configurable: true, value: 360 })
  const pinia = createPinia()
  const router = makeRouter()
  await router.push('/')
  await router.isReady()
  useSessionStore(pinia).acceptAuthenticated({
    actor: {
      capabilities,
      force_password_change: false,
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
  const Harness = defineComponent({
    setup: () => () =>
      h(VApp, null, {
        default: () => h(AppShell, null, { default: () => h('div', 'CURRENT-PAGE') }),
      }),
  })
  const result = render(Harness, { global: { plugins: [pinia, router, i18n, vuetify] } })
  window.dispatchEvent(new globalThis.Event('resize'))
  await waitFor(() => expect(screen.getByLabelText('打开主导航')).not.toBeNull())
  return {
    ...result,
    restoreWidth: () => {
      Object.defineProperty(globalThis, 'innerWidth', { configurable: true, value: originalWidth })
      window.dispatchEvent(new globalThis.Event('resize'))
    },
    router,
  }
}

afterEach(() => cleanup())

describe('AppShell', () => {
  it('keeps unauthorized pages out of mobile navigation and the command palette at 360px', async () => {
    const shell = await renderShell(['credentials:manage', 'sessions:read'])
    try {
      expect(screen.getByLabelText('打开主导航')).not.toBeNull()
      expect(screen.getByRole('link', { name: '跳到主要内容' }).getAttribute('href')).toBe(
        '#app-main-content',
      )
      expect(screen.queryByText('系统')).toBeNull()

      await fireEvent.keyDown(window, { ctrlKey: true, key: 'k' })
      const dialog = await screen.findByRole('dialog')
      expect(within(dialog).queryByText('系统')).toBeNull()
      expect(within(dialog).getByText('账户安全')).not.toBeNull()
      expect(within(dialog).getByRole('combobox')).toBe(document.activeElement)

      await fireEvent.keyDown(within(dialog).getByRole('combobox'), { key: 'Escape' })
      await waitFor(() => expect(screen.queryByRole('dialog')).toBeNull())
    } finally {
      shell.restoreWidth()
    }
  })

  it('keeps the palette open with a generic error when router navigation is aborted', async () => {
    const shell = await renderShell(['credentials:manage', 'sessions:read', 'system:read'])
    shell.router.beforeEach((to) => (to.name === appRouteNames.system ? false : true))
    try {
      await fireEvent.keyDown(window, { metaKey: true, key: 'K' })
      const dialog = await screen.findByRole('dialog')
      await fireEvent.click(within(dialog).getByText('系统'))
      expect((await within(dialog).findByRole('alert')).textContent).toContain('无法完成页面跳转')
      expect(screen.getByRole('dialog')).not.toBeNull()
      expect(shell.router.currentRoute.value.name).toBe(appRouteNames.dashboard)
    } finally {
      shell.restoreWidth()
    }
  })

  it('announces a route change and moves focus to main content', async () => {
    const shell = await renderShell(['credentials:manage', 'sessions:read'])
    try {
      await shell.router.push({ name: appRouteNames.profileSecurity })
      await waitFor(() => expect(document.activeElement?.id).toBe('app-main-content'))
      expect(screen.getByText('已进入：账户安全')).not.toBeNull()
    } finally {
      shell.restoreWidth()
    }
  })
})
