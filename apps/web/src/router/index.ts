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
const guestPaths = new Set(['/login', '/setup'])

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
    const decodedPath = decodeURIComponent(target.pathname)
    if (
      decodedPath.startsWith('//') ||
      decodedPath.startsWith('/\\') ||
      decodedPath.includes('\\') ||
      controlCharacterPattern.test(decodedPath) ||
      guestPaths.has(target.pathname)
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
) {
  if (status === 'setup-required') {
    return to.name === 'setup' ? undefined : { name: 'setup' }
  }
  if (status === 'anonymous') {
    if (to.name === 'login') return undefined
    const redirect = to.meta.requiresAuth ? safeRedirectPath(to.fullPath) : undefined
    return {
      name: 'login',
      ...(redirect && redirect !== '/' ? { query: { redirect } } : {}),
    }
  }
  if (status === 'authenticated' && (to.name === 'login' || to.name === 'setup')) {
    return { path: to.name === 'login' ? safeRedirectPath(to.query.redirect) : '/' }
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
        path: '/:pathMatch(.*)*',
        redirect: '/',
      },
    ],
  })

  appRouter.beforeEach(async (to) => {
    const session = useSessionStore(storePinia)
    await session.ensureLoaded()
    return accessRedirect(to, session.status) ?? true
  })

  return appRouter
}

export const router = createAppRouter()
