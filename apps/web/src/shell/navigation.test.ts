import { createMemoryHistory, createRouter } from 'vue-router'
import { describe, expect, it } from 'vitest'

import { appRouteNames } from '../router/route-names'
import { navigationAt, projectNavigation } from './navigation'

const component = { template: '<div />' }
const makeRouter = () =>
  createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/', name: appRouteNames.dashboard, component, meta: { requiresAuth: true } },
      {
        path: '/system',
        name: appRouteNames.system,
        component,
        meta: { requiredCapabilities: ['system:read'], requiresAuth: true },
      },
      {
        path: '/profile/security',
        name: appRouteNames.profileSecurity,
        component,
        meta: {
          allowDuringPasswordChange: true,
          requiredCapabilities: ['sessions:read', 'credentials:manage'],
          requiresAuth: true,
        },
      },
      {
        path: '/profile/security/password',
        name: appRouteNames.passwordChange,
        component,
        meta: {
          allowDuringPasswordChange: true,
          requiredCapabilities: ['credentials:manage'],
          requiresAuth: true,
        },
      },
    ],
  })

describe('shell navigation projection', () => {
  it('uses one capability projection for every navigation surface', () => {
    const items = projectNavigation(makeRouter(), {
      capabilities: ['credentials:manage', 'sessions:read'],
      passwordChangeRequired: false,
    })

    expect(items.map((item) => item.id)).toEqual([
      'dashboard',
      'profile-security',
      'password-change',
    ])
    expect(navigationAt(items, 'drawer').map((item) => item.id)).not.toContain('system')
    expect(navigationAt(items, 'command').map((item) => item.id)).not.toContain('system')
    expect(navigationAt(items, 'account').map((item) => item.id)).not.toContain('system')
  })

  it('shows only allow-during-password-change routes during a forced change', () => {
    const items = projectNavigation(makeRouter(), {
      capabilities: ['credentials:manage', 'sessions:read', 'system:read'],
      passwordChangeRequired: true,
    })

    expect(items.map((item) => item.id)).toEqual(['profile-security', 'password-change'])
  })

  it('fails closed when a registered route is absent', () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes: [{ path: '/', name: appRouteNames.dashboard, component, meta: { requiresAuth: true } }],
    })

    expect(
      projectNavigation(router, { capabilities: ['system:read'], passwordChangeRequired: false }).map(
        (item) => item.id,
      ),
    ).toEqual(['dashboard'])
  })
})
