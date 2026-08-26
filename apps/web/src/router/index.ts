import type { Pinia } from 'pinia'
import {
  createRouter,
  createWebHistory,
  type RouteLocationNormalized,
  type RouterHistory,
} from 'vue-router'

import { pinia } from '../stores'
import { useSessionStore, type SessionStatus } from '../stores/session'

const redirectBase = 'https://nodecontroll.invalid'
const controlCharacterPattern = /\p{Cc}/u
const redirectLoopPaths = new Set(['/login', '/reauth', '/setup'])

const redirectPathIsAmbiguous = (path: string) =>
  path.includes('//') || path.includes('\\') || controlCharacterPattern.test(path)

const fullyDecodedPath = (path: string) => {
  let decoded = path
  for (let pass = 0; pass < 4; pass += 1) {
    const next = decodeURIComponent(decoded)
    if (redirectPathIsAmbiguous(next)) return undefined
    if (next === decoded) return next
    decoded = next
  }
  return decoded
}

const encodedSuffixIsSafe = (value: string) => {
  let decoded = value
  for (let pass = 0; pass < 4; pass += 1) {
    const next = decodeURIComponent(decoded)
    if (next.includes('\\') || controlCharacterPattern.test(next)) return false
    if (next === decoded) return true
    decoded = next
  }
  return true
}

export function safeRedirectPath(value: unknown): string {
  if (
    typeof value !== 'string' ||
    !value.startsWith('/') ||
    value.startsWith('//') ||
    value.startsWith('/\\') ||
    value.includes('\\') ||
    controlCharacterPattern.test(value)
  ) {
    return '/'
  }

  try {
    const target = new URL(value, redirectBase)
    if (target.origin !== redirectBase) return '/'
    const decodedPath = fullyDecodedPath(target.pathname)
    if (
      !decodedPath ||
      !encodedSuffixIsSafe(`${target.search}${target.hash}`) ||
      redirectLoopPaths.has(decodedPath.replace(/\/+$/, '').toLowerCase())
    ) {
      return '/'
    }
    return `${target.pathname}${target.search}${target.hash}`
  } catch {
    return '/'
  }
}

export function accessRedirect(
  to: Pick<RouteLocationNormalized, 'fullPath' | 'meta' | 'name' | 'query'>,
  status: SessionStatus,
  security: { passwordChangeRequired: boolean; recentAuthValid: boolean } = {
    passwordChangeRequired: false,
    recentAuthValid: false,
  },
) {
  if (status === 'setup-required') {
    return to.name === 'setup' ? undefined : { name: 'setup' }
  }
  if (status === 'anonymous' || status === 'relogin-required') {
    if (to.name === 'login') return undefined
    const redirect = to.meta.requiresAuth ? safeRedirectPath(to.fullPath) : undefined
    return {
      name: 'login',
      ...(redirect && redirect !== '/' ? { query: { redirect } } : {}),
    }
  }
  if (status === 'authenticated') {
    if (security.passwordChangeRequired && to.meta.allowDuringPasswordChange !== true) {
      const redirect = to.meta.requiresAuth ? safeRedirectPath(to.fullPath) : undefined
      return {
        name: 'password-change',
        ...(redirect && redirect !== '/' ? { query: { redirect } } : {}),
      }
    }
    if (to.meta.requiresRecentAuth && !security.recentAuthValid) {
      const redirect = safeRedirectPath(to.fullPath)
      return {
        name: 'reauth',
        ...(redirect !== '/' ? { query: { redirect } } : {}),
      }
    }
    if (to.meta.guestOnly) {
      return { path: to.name === 'login' ? safeRedirectPath(to.query.redirect) : '/' }
    }
  }
  return undefined
}

export function createAppRouter(
  history: RouterHistory = createWebHistory(),
  storePinia: Pinia = pinia,
) {
  const appRouter = createRouter({
    history,
    routes: [
      {
        path: '/',
        name: 'dashboard',
        component: () => import('../views/DashboardPage.vue'),
        meta: { requiresAuth: true, title: '总览' },
      },
      {
        path: '/system',
        name: 'system',
        component: () => import('../views/SystemPage.vue'),
        meta: { requiresAuth: true, title: '系统' },
      },
      {
        path: '/login',
        name: 'login',
        component: () => import('../views/LoginPage.vue'),
        meta: { guestOnly: true, title: '登录' },
      },
      {
        path: '/setup',
        name: 'setup',
        component: () => import('../views/SetupPage.vue'),
        meta: { guestOnly: true, title: '初始化' },
      },
      {
        path: '/reauth',
        name: 'reauth',
        component: () => import('../views/ReauthenticatePage.vue'),
        meta: {
          allowDuringPasswordChange: true,
          requiresAuth: true,
          title: '确认身份',
        },
      },
      {
        path: '/profile/security',
        name: 'profile-security',
        component: () => import('../views/ProfileSecurityPage.vue'),
        meta: {
          allowDuringPasswordChange: true,
          requiresAuth: true,
          title: '账户安全',
        },
      },
      {
        path: '/profile/security/password',
        name: 'password-change',
        component: () => import('../views/ChangePasswordPage.vue'),
        meta: {
          allowDuringPasswordChange: true,
          requiresAuth: true,
          requiresRecentAuth: true,
          title: '修改密码',
        },
      },
      {
        path: '/:pathMatch(.*)*',
        redirect: '/',
      },
    ],
  })

  appRouter.beforeEach(async (to) => {
    const session = useSessionStore(storePinia)
    await session.ensureLoaded()
    session.syncRecentAuthClock()
    return (
      accessRedirect(to, session.status, {
        passwordChangeRequired: session.passwordChangeRequired,
        recentAuthValid: session.recentAuthValid,
      }) ?? true
    )
  })

  return appRouter
}

export const router = createAppRouter()
