import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

import {
  getBootstrapState,
  getCurrentActor,
  login as requestLogin,
  logout as requestLogout,
} from '../api/generated/sdk.gen'
import type { ActorResponse, SessionResponse } from '../api/generated/types.gen'

export const CSRF_COOKIE_NAME = '__Host-nodecontroll_csrf'

const csrfTokenPattern = /^ncc1_[0-9a-f]{64}$/
const sameOriginCredentials = 'same-origin' as const

export type SessionStatus =
  | 'unknown'
  | 'loading'
  | 'setup-required'
  | 'anonymous'
  | 'authenticated'
  | 'unavailable'

export type LoginFailureReason =
  | 'invalid-credentials'
  | 'rate-limited'
  | 'request-rejected'
  | 'setup-required'
  | 'unavailable'

export type LogoutFailureReason = 'csrf-unavailable' | 'request-rejected' | 'unavailable'

export class LoginFailure extends Error {
  constructor(
    readonly reason: LoginFailureReason,
    readonly retryAfterSeconds?: number,
  ) {
    super('Login failed')
    this.name = 'LoginFailure'
  }
}

export class LogoutFailure extends Error {
  constructor(readonly reason: LogoutFailureReason) {
    super('Logout failed')
    this.name = 'LogoutFailure'
  }
}

const retryAfterSeconds = (response?: Response) => {
  const value = response?.headers.get('retry-after')?.trim()
  if (!value || !/^\d{1,5}$/.test(value)) return undefined
  const seconds = Number(value)
  return seconds >= 1 && seconds <= 3_600 ? seconds : undefined
}

export function readCsrfCookie(cookieHeader: string): string | undefined {
  let csrfToken: string | undefined
  for (const rawPair of cookieHeader.split(';')) {
    const pair = rawPair.trim()
    const separator = pair.indexOf('=')
    if (separator < 1 || pair.slice(0, separator) !== CSRF_COOKIE_NAME) continue
    if (csrfToken !== undefined) return undefined

    const candidate = pair.slice(separator + 1)
    if (!csrfTokenPattern.test(candidate)) return undefined
    csrfToken = candidate
  }
  return csrfToken
}

export const useSessionStore = defineStore('session', () => {
  const status = ref<SessionStatus>('unknown')
  const actor = ref<ActorResponse>()
  const session = ref<SessionResponse>()
  const loginPending = ref(false)
  const logoutPending = ref(false)
  let refreshPromise: Promise<void> | undefined

  const isAuthenticated = computed(() => status.value === 'authenticated')
  const isResolving = computed(() => status.value === 'unknown' || status.value === 'loading')
  const hasResolutionError = computed(() => status.value === 'unavailable')

  const clearProjection = () => {
    actor.value = undefined
    session.value = undefined
  }

  const markSetupRequired = () => {
    clearProjection()
    status.value = 'setup-required'
  }

  const markInitialized = () => {
    clearProjection()
    status.value = 'anonymous'
  }

  const markAuthenticated = (nextActor: ActorResponse, nextSession: SessionResponse) => {
    actor.value = nextActor
    session.value = nextSession
    status.value = 'authenticated'
  }

  const runRefresh = async () => {
    status.value = 'loading'
    clearProjection()

    try {
      const bootstrap = await getBootstrapState({ credentials: sameOriginCredentials })
      if (bootstrap.error !== undefined || bootstrap.data === undefined) {
        status.value = 'unavailable'
        return
      }
      if (!bootstrap.data.data.initialized) {
        markSetupRequired()
        return
      }

      const current = await getCurrentActor({ credentials: sameOriginCredentials })
      if (current.error !== undefined || current.data === undefined) {
        if (current.response?.status === 401) {
          markInitialized()
        } else {
          status.value = 'unavailable'
        }
        return
      }
      markAuthenticated(current.data.data.actor, current.data.data.session)
    } catch {
      clearProjection()
      status.value = 'unavailable'
    }
  }

  const refresh = async () => {
    if (refreshPromise) return refreshPromise
    const pending = runRefresh()
    refreshPromise = pending
    try {
      await pending
    } finally {
      if (refreshPromise === pending) refreshPromise = undefined
    }
  }

  const ensureLoaded = async () => {
    if (status.value === 'unknown') await refresh()
  }

  const login = async (username: string, password: string) => {
    loginPending.value = true
    try {
      const response = await requestLogin({
        credentials: sameOriginCredentials,
        body: { username, password },
      })
      if (response.error === undefined && response.data !== undefined) {
        markAuthenticated(response.data.data.actor, response.data.data.session)
        return
      }

      const httpStatus = response.response?.status
      if (httpStatus === 409) {
        markSetupRequired()
        throw new LoginFailure('setup-required')
      }
      if (httpStatus === 401) throw new LoginFailure('invalid-credentials')
      if (httpStatus === 429) {
        throw new LoginFailure('rate-limited', retryAfterSeconds(response.response))
      }
      if (httpStatus === 400 || httpStatus === 403) {
        throw new LoginFailure('request-rejected')
      }
      throw new LoginFailure('unavailable')
    } catch (error) {
      if (error instanceof LoginFailure) throw error
      throw new LoginFailure('unavailable')
    } finally {
      loginPending.value = false
    }
  }

  const logout = async () => {
    const csrfToken = readCsrfCookie(document.cookie)
    if (!csrfToken) throw new LogoutFailure('csrf-unavailable')

    logoutPending.value = true
    try {
      const response = await requestLogout({
        credentials: sameOriginCredentials,
        headers: { 'x-nodecontroll-csrf': csrfToken },
      })
      if (response.error === undefined || response.response?.status === 401) {
        markInitialized()
        return
      }
      if (response.response?.status === 403) throw new LogoutFailure('request-rejected')
      throw new LogoutFailure('unavailable')
    } catch (error) {
      if (error instanceof LogoutFailure) throw error
      throw new LogoutFailure('unavailable')
    } finally {
      logoutPending.value = false
    }
  }

  return {
    actor,
    ensureLoaded,
    hasResolutionError,
    isAuthenticated,
    isResolving,
    login,
    loginPending,
    logout,
    logoutPending,
    markInitialized,
    refresh,
    session,
    status,
  }
})
