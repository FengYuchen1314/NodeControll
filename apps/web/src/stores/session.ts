import { computed, onScopeDispose, ref } from 'vue'
import { defineStore } from 'pinia'

import {
  changeCurrentPassword as requestPasswordChange,
  getBootstrapState,
  getCurrentActor,
  listCurrentSessions as requestSessionList,
  login as requestLogin,
  logout as requestLogout,
  logoutAll as requestLogoutAll,
  reauthenticate as requestReauthentication,
  revokeCurrentUserSession as requestSessionRevocation,
} from '../api/generated/sdk.gen'
import type {
  ActorResponse,
  SessionResponse,
  UserSessionResponse,
} from '../api/generated/types.gen'
import {
  acquireCredentialMutation,
  CredentialCoordinationFailure,
  getCredentialCoordinationCursor,
  getCredentialCoordinationSnapshot,
  newCredentialParticipantId,
  persistCredentialInvalidation,
  publishCredentialInvalidation,
  subscribeCredentialCoordination,
  withCredentialReadLock,
  type CredentialCoordinationCursor,
  type CredentialCoordinationEvent,
  type CredentialCoordinationRecord,
  type CredentialDisposition,
  type CredentialMutationLease,
  type CredentialReadObservation,
} from '../lib/credential-coordinator'

export const CSRF_COOKIE_NAME = '__Host-nodecontroll_csrf'

const csrfTokenPattern = /^ncc1_[0-9a-f]{64}$/
const sameOriginCredentials = 'same-origin' as const
const credentialMutationTimeoutMs = 15_000

export type SessionStatus =
  | 'unknown'
  | 'loading'
  | 'setup-required'
  | 'anonymous'
  | 'authenticated'
  | 'unavailable'
  | 'relogin-required'

export type LoginFailureReason =
  'invalid-credentials' | 'rate-limited' | 'request-rejected' | 'setup-required' | 'unavailable'

export type LogoutFailureReason = 'csrf-unavailable' | 'request-rejected' | 'unavailable'

export type ReauthenticationFailureReason =
  | 'csrf-unavailable'
  | 'invalid-proof'
  | 'outcome-unknown'
  | 'rate-limited'
  | 'request-rejected'
  | 'session-invalid'
  | 'unavailable'

export type PasswordChangeFailureReason =
  | 'csrf-unavailable'
  | 'outcome-unknown'
  | 'password-policy'
  | 'password-unchanged'
  | 'rate-limited'
  | 'recent-auth-required'
  | 'request-rejected'
  | 'session-invalid'
  | 'unavailable'

export type SessionManagementFailureReason =
  | 'csrf-unavailable'
  | 'outcome-unknown'
  | 'recent-auth-required'
  | 'request-rejected'
  | 'session-invalid'
  | 'unavailable'

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

export class ReauthenticationFailure extends Error {
  constructor(
    readonly reason: ReauthenticationFailureReason,
    readonly retryAfterSeconds?: number,
  ) {
    super('Reauthentication failed')
    this.name = 'ReauthenticationFailure'
  }
}

export class PasswordChangeFailure extends Error {
  constructor(readonly reason: PasswordChangeFailureReason) {
    super('Password change failed')
    this.name = 'PasswordChangeFailure'
  }
}

export class SessionManagementFailure extends Error {
  constructor(readonly reason: SessionManagementFailureReason) {
    super('Session operation failed')
    this.name = 'SessionManagementFailure'
  }
}

type AuthenticatedSnapshot = {
  actor: ActorResponse
  session: SessionResponse
  status: 'authenticated'
}

type UnauthenticatedSnapshot = {
  status: Exclude<SessionStatus, 'authenticated'>
}

type SessionSnapshot = AuthenticatedSnapshot | UnauthenticatedSnapshot

const uuidPattern = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i
const boundedText = (value: unknown, maximumLength: number): value is string =>
  typeof value === 'string' && value.length >= 1 && value.length <= maximumLength
const nonnegativeSafeInteger = (value: unknown): value is number =>
  typeof value === 'number' && Number.isSafeInteger(value) && value >= 0
const objectRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === 'object' && value !== null

const validActorProjection = (value: unknown): value is ActorResponse => {
  if (!objectRecord(value)) return false
  return (
    typeof value.id === 'string' &&
    uuidPattern.test(value.id) &&
    boundedText(value.username, 256) &&
    boundedText(value.role, 64) &&
    typeof value.force_password_change === 'boolean' &&
    Array.isArray(value.capabilities) &&
    value.capabilities.length <= 256 &&
    value.capabilities.every((capability) => boundedText(capability, 128)) &&
    new Set(value.capabilities).size === value.capabilities.length
  )
}

const validSessionProjection = (value: unknown): value is SessionResponse => {
  if (!objectRecord(value)) return false
  if (
    typeof value.id !== 'string' ||
    !uuidPattern.test(value.id) ||
    !boundedText(value.auth_level, 64) ||
    !nonnegativeSafeInteger(value.created_at_ms) ||
    !nonnegativeSafeInteger(value.last_seen_at_ms) ||
    !nonnegativeSafeInteger(value.idle_expires_at_ms) ||
    !nonnegativeSafeInteger(value.absolute_expires_at_ms) ||
    !nonnegativeSafeInteger(value.recent_auth_expires_at_ms)
  ) {
    return false
  }
  return (
    value.created_at_ms <= value.last_seen_at_ms &&
    value.last_seen_at_ms <= value.idle_expires_at_ms &&
    value.idle_expires_at_ms <= value.absolute_expires_at_ms &&
    value.recent_auth_expires_at_ms <= value.absolute_expires_at_ms
  )
}

const validAuthenticatedProjection = (
  value: unknown,
): value is { actor: ActorResponse; session: SessionResponse } =>
  objectRecord(value) && validActorProjection(value.actor) && validSessionProjection(value.session)

const validRevokedSessionCount = (value: unknown): value is number => nonnegativeSafeInteger(value)

const validUserSessionProjection = (value: unknown): value is UserSessionResponse => {
  if (!validSessionProjection(value)) return false
  return typeof (value as SessionResponse & { is_current?: unknown }).is_current === 'boolean'
}

const validSessionList = (
  value: unknown,
  currentSessionId: string,
): value is UserSessionResponse[] => {
  if (!Array.isArray(value) || !value.every(validUserSessionProjection)) return false
  const identifiers = new Set(value.map((candidate) => candidate.id))
  return (
    identifiers.size === value.length &&
    value.filter((candidate) => candidate.is_current).length === 1 &&
    value.some((candidate) => candidate.is_current && candidate.id === currentSessionId)
  )
}

const sameCredentialRecord = (
  left: CredentialCoordinationRecord,
  right: CredentialCoordinationRecord,
) =>
  left.baseSeq === right.baseSeq &&
  left.disposition === right.disposition &&
  left.epoch === right.epoch &&
  left.opId === right.opId &&
  left.operation === right.operation &&
  left.phase === right.phase &&
  left.senderId === right.senderId &&
  left.seq === right.seq &&
  left.v === right.v &&
  ('observedSessionId' in left ? left.observedSessionId : undefined) ===
    ('observedSessionId' in right ? right.observedSessionId : undefined)

const retryAfterSeconds = (response?: Response) => {
  const value = response?.headers.get('retry-after')?.trim()
  if (!value || !/^\d{1,5}$/.test(value)) return undefined
  const seconds = Number(value)
  return seconds >= 1 && seconds <= 3_600 ? seconds : undefined
}

const problemCode = (error: unknown): string | undefined => {
  if (typeof error !== 'object' || error === null || !('code' in error)) return undefined
  return typeof error.code === 'string' ? error.code : undefined
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

const currentCsrfHeaders = () => {
  const csrfToken = readCsrfCookie(document.cookie)
  return csrfToken ? { 'x-nodecontroll-csrf': csrfToken } : undefined
}

const credentialMutationSignal = () => AbortSignal.timeout(credentialMutationTimeoutMs)
const credentialReadSignal = () => AbortSignal.timeout(credentialMutationTimeoutMs)

export const useSessionStore = defineStore('session', () => {
  const snapshot = ref<SessionSnapshot>({ status: 'unknown' })
  const recentAuthNowMs = ref(Date.now())
  const loginPending = ref(false)
  const logoutPending = ref(false)
  const reauthenticationPending = ref(false)
  const passwordChangePending = ref(false)
  const sessionListPending = ref(false)
  const logoutAllPending = ref(false)
  const revokingSessionIds = ref<string[]>([])
  const managedSessions = ref<UserSessionResponse[]>([])
  const participantId = newCredentialParticipantId()
  let refreshPromise: Promise<void> | undefined
  let credentialReconciliationPromise: Promise<void> | undefined
  let snapshotGeneration = 0
  let latestCredentialRecord: CredentialCoordinationRecord | undefined

  const status = computed(() => snapshot.value.status)
  const actor = computed(() =>
    snapshot.value.status === 'authenticated' ? snapshot.value.actor : undefined,
  )
  const session = computed(() =>
    snapshot.value.status === 'authenticated' ? snapshot.value.session : undefined,
  )
  const isAuthenticated = computed(() => status.value === 'authenticated')
  const isResolving = computed(() => status.value === 'unknown' || status.value === 'loading')
  const hasResolutionError = computed(() => status.value === 'unavailable')
  const passwordChangeRequired = computed(() => actor.value?.force_password_change === true)
  const recentAuthValid = computed(
    () =>
      session.value !== undefined &&
      recentAuthNowMs.value >= 0 &&
      recentAuthNowMs.value < session.value.recent_auth_expires_at_ms,
  )
  const recentAuthExpired = computed(() => session.value !== undefined && !recentAuthValid.value)

  const replaceSnapshot = (nextSnapshot: SessionSnapshot) => {
    snapshotGeneration += 1
    if (nextSnapshot.status !== 'authenticated') managedSessions.value = []
    snapshot.value = nextSnapshot
    return snapshotGeneration
  }

  const setStatus = (nextStatus: UnauthenticatedSnapshot['status']) => {
    return replaceSnapshot({ status: nextStatus })
  }

  const setStatusIfCurrent = (
    expectedGeneration: number,
    nextStatus: UnauthenticatedSnapshot['status'],
  ) => {
    if (snapshotGeneration !== expectedGeneration) return false
    setStatus(nextStatus)
    return true
  }

  const syncRecentAuthClock = (nowMs = Date.now()) => {
    recentAuthNowMs.value = nowMs
  }

  const acceptAuthenticated = (projection: { actor: ActorResponse; session: SessionResponse }) => {
    syncRecentAuthClock()
    replaceSnapshot({
      actor: projection.actor,
      session: projection.session,
      status: 'authenticated',
    })
  }

  const acceptAuthenticatedIfCurrent = (
    expectedGeneration: number,
    projection: { actor: ActorResponse; session: SessionResponse },
  ) => {
    if (snapshotGeneration !== expectedGeneration) return false
    acceptAuthenticated(projection)
    return true
  }

  const markSetupRequired = () => {
    setStatus('setup-required')
  }

  const markInitialized = () => {
    setStatus('anonymous')
  }

  const enterCredentialQuarantine = () => {
    setStatus('relogin-required')
  }

  const finishCredentialMutation = (
    lease: CredentialMutationLease | undefined,
    disposition: CredentialDisposition,
  ) => {
    if (!lease) return
    if (!lease.settle(disposition)) enterCredentialQuarantine()
  }

  const authenticatedCredentialCursor = (): CredentialCoordinationCursor | undefined => {
    const cursor = getCredentialCoordinationCursor()
    if (!cursor) enterCredentialQuarantine()
    return cursor
  }

  const runRefresh = async () => {
    const observedSessionId = session.value?.id
    const refreshGeneration = setStatus('loading')
    let invalidation: { observation: CredentialReadObservation; sessionId?: string } | undefined
    try {
      const bootstrap = await getBootstrapState({
        credentials: sameOriginCredentials,
        signal: credentialReadSignal(),
      })
      if (snapshotGeneration !== refreshGeneration) return
      if (bootstrap.error !== undefined || bootstrap.data === undefined) {
        setStatusIfCurrent(refreshGeneration, 'unavailable')
        return
      }
      if (!bootstrap.data.data.initialized) {
        setStatusIfCurrent(refreshGeneration, 'setup-required')
        return
      }

      await withCredentialReadLock(async (observation) => {
        const current = await getCurrentActor({
          credentials: sameOriginCredentials,
          signal: credentialReadSignal(),
        })
        if (snapshotGeneration !== refreshGeneration) return
        if (current.error !== undefined || current.data === undefined) {
          if (current.response?.status === 401) {
            if (setStatusIfCurrent(refreshGeneration, 'anonymous')) {
              publishCredentialInvalidation(observation, participantId, observedSessionId)
              invalidation = { observation, sessionId: observedSessionId }
            }
          } else {
            setStatusIfCurrent(refreshGeneration, 'unavailable')
          }
          return
        }
        if (!validAuthenticatedProjection(current.data.data)) {
          setStatusIfCurrent(refreshGeneration, 'unavailable')
          return
        }
        acceptAuthenticatedIfCurrent(refreshGeneration, current.data.data)
      })
    } catch (error) {
      if (error instanceof CredentialCoordinationFailure) {
        if (error.reason === 'quarantine') {
          enterCredentialQuarantine()
        } else if (error.reason === 'invalidated') {
          if (status.value !== 'relogin-required') setStatus('anonymous')
        } else if (status.value !== 'relogin-required') {
          setStatusIfCurrent(refreshGeneration, 'unavailable')
        }
      } else {
        setStatusIfCurrent(refreshGeneration, 'unavailable')
      }
    } finally {
      if (invalidation) {
        await persistCredentialInvalidation(
          invalidation.observation,
          participantId,
          invalidation.sessionId,
        )
      }
    }
  }

  const refresh = async () => {
    if (status.value === 'relogin-required') return
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

  const runCredentialReconciliation = async () => {
    const reconciliationGeneration = setStatus('unknown')
    if (refreshPromise) await refreshPromise
    if (snapshotGeneration !== reconciliationGeneration) return
    await refresh()
  }

  const reconcileAfterCredentialMutation = async () => {
    if (credentialReconciliationPromise) return credentialReconciliationPromise
    const pending = runCredentialReconciliation()
    credentialReconciliationPromise = pending
    try {
      await pending
    } finally {
      if (credentialReconciliationPromise === pending) {
        credentialReconciliationPromise = undefined
      }
    }
  }

  const applyCredentialRecord = (record: CredentialCoordinationRecord, initial = false) => {
    const previous = latestCredentialRecord
    if (initial) {
      latestCredentialRecord = record
      if (record.phase === 'inflight' || record.disposition === 'quarantine') {
        enterCredentialQuarantine()
      } else if (record.phase === 'invalidated') {
        setStatus('anonymous')
      }
      return
    }

    if (previous) {
      if (record.epoch !== previous.epoch) {
        latestCredentialRecord = record
        enterCredentialQuarantine()
        return
      }
      const previousSequence = BigInt(previous.seq)
      const sequence = BigInt(record.seq)
      if (sequence < previousSequence) {
        enterCredentialQuarantine()
        return
      }
      if (sequence === previousSequence) {
        if (!sameCredentialRecord(record, previous)) enterCredentialQuarantine()
        return
      }
      if (record.baseSeq !== previous.seq) {
        enterCredentialQuarantine()
        return
      }
    }

    if (record.phase === 'inflight') {
      latestCredentialRecord = record
      enterCredentialQuarantine()
      return
    }

    if (record.phase === 'invalidated') {
      latestCredentialRecord = record
      if (status.value !== 'relogin-required') setStatus('anonymous')
      return
    }

    const followsObservedInflight =
      previous?.phase === 'inflight' &&
      previous.epoch === record.epoch &&
      previous.opId === record.opId &&
      previous.seq === record.baseSeq
    latestCredentialRecord = record
    if (!followsObservedInflight || record.disposition === 'quarantine') {
      enterCredentialQuarantine()
      return
    }
    if (record.senderId !== participantId || status.value === 'relogin-required') {
      void reconcileAfterCredentialMutation()
    }
  }

  const handleCredentialCoordination = (event: CredentialCoordinationEvent) => {
    if (event.kind === 'corrupt' || event.kind === 'reset') {
      enterCredentialQuarantine()
      return
    }
    if (event.kind === 'observed-invalid') {
      if (status.value === 'relogin-required') return
      const observedSessionId = event.message.observedSessionId
      if (observedSessionId !== undefined && session.value?.id !== observedSessionId) return
      setStatus('anonymous')
      return
    }
    applyCredentialRecord(event.record)
  }

  const unsubscribeCredentialCoordination = subscribeCredentialCoordination(
    handleCredentialCoordination,
  )
  const initialCredentialSnapshot = getCredentialCoordinationSnapshot()
  if (initialCredentialSnapshot.kind === 'invalid') {
    enterCredentialQuarantine()
  } else if (initialCredentialSnapshot.kind === 'record') {
    applyCredentialRecord(initialCredentialSnapshot.record, true)
  } else if (currentCsrfHeaders()) {
    enterCredentialQuarantine()
  }
  const reconcileVisibleCredentialState = () => {
    if (document.visibilityState !== 'visible') return
    const currentCredentialSnapshot = getCredentialCoordinationSnapshot()
    if (currentCredentialSnapshot.kind === 'invalid') {
      enterCredentialQuarantine()
      return
    }
    if (currentCredentialSnapshot.kind === 'absent') {
      if (
        currentCsrfHeaders() ||
        status.value === 'authenticated' ||
        status.value === 'unavailable' ||
        status.value === 'relogin-required'
      ) {
        enterCredentialQuarantine()
      }
      return
    }

    applyCredentialRecord(currentCredentialSnapshot.record)
    if (
      currentCredentialSnapshot.record.phase === 'settled' &&
      currentCredentialSnapshot.record.disposition === 'reconcile' &&
      (status.value === 'authenticated' || status.value === 'unavailable')
    ) {
      void reconcileAfterCredentialMutation()
    }
  }
  document.addEventListener('visibilitychange', reconcileVisibleCredentialState)
  window.addEventListener('focus', reconcileVisibleCredentialState)
  onScopeDispose(() => {
    unsubscribeCredentialCoordination()
    document.removeEventListener('visibilitychange', reconcileVisibleCredentialState)
    window.removeEventListener('focus', reconcileVisibleCredentialState)
  })

  const requireReloginAfterUnknownMutation = async () => {
    enterCredentialQuarantine()
    const reloginGeneration = snapshotGeneration
    const headers = currentCsrfHeaders()
    if (!headers) return false
    try {
      const response = await requestLogout({
        credentials: sameOriginCredentials,
        headers,
        signal: credentialMutationSignal(),
      })
      if (response.error === undefined && response.response?.status === 204) {
        setStatusIfCurrent(reloginGeneration, 'anonymous')
        return true
      }
    } catch {
      // The original mutation must not be replayed. The relogin-required quarantine remains
      // sticky until logout is confirmed or the operator explicitly signs in again.
    }
    return false
  }

  const login = async (username: string, password: string) => {
    const startingStatus = status.value
    const startingGeneration = snapshotGeneration
    let recoveryRequired = startingStatus === 'relogin-required'
    let loginGeneration: number | undefined
    let credentialLease: CredentialMutationLease | undefined
    let disposition: CredentialDisposition = 'quarantine'
    let requestDispatched = false
    loginPending.value = true
    const restoreKnownFailure = () => {
      if (
        recoveryRequired ||
        (loginGeneration !== undefined && snapshotGeneration !== loginGeneration)
      ) {
        enterCredentialQuarantine()
      } else if (startingStatus === 'setup-required') {
        markSetupRequired()
        disposition = 'reconcile'
      } else {
        markInitialized()
        disposition = 'reconcile'
      }
    }
    try {
      credentialLease = await acquireCredentialMutation(participantId, 'login')
      if (!credentialLease) {
        if (startingStatus === 'authenticated') enterCredentialQuarantine()
        throw new LoginFailure('unavailable')
      }
      loginGeneration = snapshotGeneration
      if (loginGeneration !== startingGeneration + 1) recoveryRequired = true
      requestDispatched = true
      const response = await requestLogin({
        credentials: sameOriginCredentials,
        body: { username, password },
        signal: credentialMutationSignal(),
      })
      if (
        response.error === undefined &&
        response.data !== undefined &&
        response.response?.status === 200 &&
        validAuthenticatedProjection(response.data.data)
      ) {
        if (!acceptAuthenticatedIfCurrent(loginGeneration, response.data.data)) {
          disposition = (await requireReloginAfterUnknownMutation()) ? 'reconcile' : 'quarantine'
          throw new LoginFailure('unavailable')
        }
        disposition = 'reconcile'
        return
      }

      const httpStatus = response.response?.status
      if (httpStatus === 409) {
        restoreKnownFailure()
        throw new LoginFailure('setup-required')
      }
      if (httpStatus === 401) {
        restoreKnownFailure()
        throw new LoginFailure('invalid-credentials')
      }
      if (httpStatus === 429) {
        restoreKnownFailure()
        throw new LoginFailure('rate-limited', retryAfterSeconds(response.response))
      }
      if (
        httpStatus === 400 ||
        httpStatus === 403 ||
        httpStatus === 413 ||
        httpStatus === 415 ||
        httpStatus === 422
      ) {
        restoreKnownFailure()
        throw new LoginFailure('request-rejected')
      }
      if (httpStatus !== undefined && httpStatus !== 200) {
        restoreKnownFailure()
        throw new LoginFailure('unavailable')
      }
      disposition = (await requireReloginAfterUnknownMutation()) ? 'reconcile' : 'quarantine'
      throw new LoginFailure('unavailable')
    } catch (error) {
      if (error instanceof LoginFailure) throw error
      if (credentialLease && requestDispatched) {
        disposition = (await requireReloginAfterUnknownMutation()) ? 'reconcile' : 'quarantine'
      } else if (credentialLease) {
        restoreKnownFailure()
      }
      throw new LoginFailure('unavailable')
    } finally {
      loginPending.value = false
      finishCredentialMutation(credentialLease, disposition)
    }
  }

  const reauthenticate = async (password: string) => {
    if (status.value !== 'authenticated') {
      throw new ReauthenticationFailure('session-invalid')
    }
    const startingProjection = { actor: actor.value!, session: session.value! }
    const expectedCursor = authenticatedCredentialCursor()
    if (!expectedCursor) throw new ReauthenticationFailure('session-invalid')
    let credentialLease: CredentialMutationLease | undefined
    let disposition: CredentialDisposition = 'quarantine'
    reauthenticationPending.value = true
    try {
      credentialLease = await acquireCredentialMutation(participantId, 'reauth', expectedCursor)
      if (!credentialLease) {
        enterCredentialQuarantine()
        throw new ReauthenticationFailure('unavailable')
      }
      const reauthenticationGeneration = snapshotGeneration
      const restoreKnownFailure = () => {
        disposition = acceptAuthenticatedIfCurrent(reauthenticationGeneration, startingProjection)
          ? 'reconcile'
          : 'quarantine'
      }
      const headers = currentCsrfHeaders()
      if (!headers) {
        restoreKnownFailure()
        throw new ReauthenticationFailure('csrf-unavailable')
      }
      const response = await requestReauthentication({
        credentials: sameOriginCredentials,
        headers,
        body: { method: 'password', password },
        signal: credentialMutationSignal(),
      })
      if (
        response.error === undefined &&
        response.data !== undefined &&
        response.response?.status === 200 &&
        validAuthenticatedProjection(response.data.data)
      ) {
        if (!acceptAuthenticatedIfCurrent(reauthenticationGeneration, response.data.data)) {
          throw new ReauthenticationFailure('outcome-unknown')
        }
        disposition = 'reconcile'
        return
      }

      const httpStatus = response.response?.status
      if (httpStatus === 401) {
        markInitialized()
        disposition = 'reconcile'
        throw new ReauthenticationFailure('session-invalid')
      }
      if (httpStatus === 429) {
        restoreKnownFailure()
        throw new ReauthenticationFailure('rate-limited', retryAfterSeconds(response.response))
      }
      if (httpStatus === 403 && problemCode(response.error) === 'REAUTHENTICATION_FAILED') {
        restoreKnownFailure()
        throw new ReauthenticationFailure('invalid-proof')
      }
      if (
        httpStatus === 400 ||
        httpStatus === 403 ||
        httpStatus === 413 ||
        httpStatus === 415 ||
        httpStatus === 422
      ) {
        restoreKnownFailure()
        throw new ReauthenticationFailure('request-rejected')
      }
      if (httpStatus !== undefined && httpStatus >= 400 && httpStatus < 500) {
        restoreKnownFailure()
        throw new ReauthenticationFailure('unavailable')
      }
      throw new ReauthenticationFailure('outcome-unknown')
    } catch (error) {
      const failure =
        error instanceof ReauthenticationFailure
          ? error
          : new ReauthenticationFailure('outcome-unknown')
      if (failure.reason === 'outcome-unknown') {
        disposition = (await requireReloginAfterUnknownMutation()) ? 'reconcile' : 'quarantine'
      }
      throw failure
    } finally {
      reauthenticationPending.value = false
      finishCredentialMutation(credentialLease, disposition)
    }
  }

  const changePassword = async (newPassword: string) => {
    if (status.value !== 'authenticated') {
      throw new PasswordChangeFailure('session-invalid')
    }
    const startingProjection = { actor: actor.value!, session: session.value! }
    const expectedCursor = authenticatedCredentialCursor()
    if (!expectedCursor) throw new PasswordChangeFailure('session-invalid')
    let credentialLease: CredentialMutationLease | undefined
    let disposition: CredentialDisposition = 'quarantine'
    passwordChangePending.value = true
    try {
      credentialLease = await acquireCredentialMutation(
        participantId,
        'change-password',
        expectedCursor,
      )
      if (!credentialLease) {
        enterCredentialQuarantine()
        throw new PasswordChangeFailure('unavailable')
      }
      const passwordChangeGeneration = snapshotGeneration
      const restoreKnownFailure = () => {
        disposition = acceptAuthenticatedIfCurrent(passwordChangeGeneration, startingProjection)
          ? 'reconcile'
          : 'quarantine'
      }
      const headers = currentCsrfHeaders()
      if (!headers) {
        restoreKnownFailure()
        throw new PasswordChangeFailure('csrf-unavailable')
      }
      const response = await requestPasswordChange({
        credentials: sameOriginCredentials,
        headers,
        body: { new_password: newPassword },
        signal: credentialMutationSignal(),
      })
      if (
        response.error === undefined &&
        response.data !== undefined &&
        response.response?.status === 200 &&
        validAuthenticatedProjection(response.data.data) &&
        validRevokedSessionCount(response.data.data.revoked_sessions)
      ) {
        if (!acceptAuthenticatedIfCurrent(passwordChangeGeneration, response.data.data)) {
          throw new PasswordChangeFailure('outcome-unknown')
        }
        disposition = 'reconcile'
        return response.data.data.revoked_sessions
      }

      const httpStatus = response.response?.status
      const code = problemCode(response.error)
      if (httpStatus === 401) {
        markInitialized()
        disposition = 'reconcile'
        throw new PasswordChangeFailure('session-invalid')
      }
      if (httpStatus === 403 && code === 'RECENT_AUTH_REQUIRED') {
        restoreKnownFailure()
        throw new PasswordChangeFailure('recent-auth-required')
      }
      if (httpStatus === 422 && code === 'PASSWORD_POLICY_REJECTED') {
        restoreKnownFailure()
        throw new PasswordChangeFailure('password-policy')
      }
      if (httpStatus === 422 && code === 'PASSWORD_UNCHANGED') {
        restoreKnownFailure()
        throw new PasswordChangeFailure('password-unchanged')
      }
      if (httpStatus === 429) {
        restoreKnownFailure()
        throw new PasswordChangeFailure('rate-limited')
      }
      if (
        httpStatus === 400 ||
        httpStatus === 403 ||
        httpStatus === 413 ||
        httpStatus === 415 ||
        httpStatus === 422
      ) {
        restoreKnownFailure()
        throw new PasswordChangeFailure('request-rejected')
      }
      if (httpStatus !== undefined && httpStatus >= 400 && httpStatus < 500) {
        restoreKnownFailure()
        throw new PasswordChangeFailure('unavailable')
      }
      throw new PasswordChangeFailure('outcome-unknown')
    } catch (error) {
      const failure =
        error instanceof PasswordChangeFailure
          ? error
          : new PasswordChangeFailure('outcome-unknown')
      if (failure.reason === 'outcome-unknown') {
        disposition = (await requireReloginAfterUnknownMutation()) ? 'reconcile' : 'quarantine'
      }
      throw failure
    } finally {
      passwordChangePending.value = false
      finishCredentialMutation(credentialLease, disposition)
    }
  }

  const listSessions = async (): Promise<UserSessionResponse[]> => {
    sessionListPending.value = true
    managedSessions.value = []
    let invalidation: { observation: CredentialReadObservation; sessionId: string } | undefined
    try {
      const result = await withCredentialReadLock(async (observation) => {
        if (status.value !== 'authenticated') {
          throw new SessionManagementFailure('session-invalid')
        }
        const listGeneration = snapshotGeneration
        const observedSessionId = session.value?.id
        if (!observedSessionId) throw new SessionManagementFailure('session-invalid')
        const response = await requestSessionList({
          credentials: sameOriginCredentials,
          signal: credentialReadSignal(),
        })
        if (snapshotGeneration !== listGeneration) {
          throw new SessionManagementFailure('session-invalid')
        }
        if (
          response.error === undefined &&
          response.data !== undefined &&
          response.response?.status === 200 &&
          validSessionList(response.data.data.sessions, observedSessionId)
        ) {
          managedSessions.value = response.data.data.sessions
          return response.data.data.sessions
        }
        if (response.response?.status === 401) {
          if (setStatusIfCurrent(listGeneration, 'anonymous')) {
            publishCredentialInvalidation(observation, participantId, observedSessionId)
            invalidation = { observation, sessionId: observedSessionId }
          }
          throw new SessionManagementFailure('session-invalid')
        }
        if (response.response?.status === 400 || response.response?.status === 403) {
          throw new SessionManagementFailure('request-rejected')
        }
        throw new SessionManagementFailure('unavailable')
      })
      return result
    } catch (error) {
      if (error instanceof CredentialCoordinationFailure) {
        if (error.reason === 'quarantine') {
          enterCredentialQuarantine()
          throw new SessionManagementFailure('session-invalid')
        }
        if (error.reason === 'invalidated') {
          if (status.value !== 'relogin-required') setStatus('anonymous')
          throw new SessionManagementFailure('session-invalid')
        }
        enterCredentialQuarantine()
        throw new SessionManagementFailure('unavailable')
      }
      if (error instanceof SessionManagementFailure) throw error
      throw new SessionManagementFailure('unavailable')
    } finally {
      if (invalidation) {
        await persistCredentialInvalidation(
          invalidation.observation,
          participantId,
          invalidation.sessionId,
        )
      }
      sessionListPending.value = false
    }
  }

  const revokeSession = async (sessionId: string) => {
    if (status.value !== 'authenticated') {
      throw new SessionManagementFailure('session-invalid')
    }
    const startingProjection = { actor: actor.value!, session: session.value! }
    const revokesCurrentSession = session.value?.id === sessionId
    const expectedCursor = authenticatedCredentialCursor()
    if (!expectedCursor) throw new SessionManagementFailure('session-invalid')
    let credentialLease: CredentialMutationLease | undefined
    let disposition: CredentialDisposition = 'quarantine'
    let restoreKnownFailure = enterCredentialQuarantine
    revokingSessionIds.value = [...revokingSessionIds.value, sessionId]
    try {
      credentialLease = await acquireCredentialMutation(participantId, 'revoke', expectedCursor)
      if (!credentialLease) {
        enterCredentialQuarantine()
        throw new SessionManagementFailure('unavailable')
      }
      const revocationGeneration = snapshotGeneration
      restoreKnownFailure = () => {
        disposition = acceptAuthenticatedIfCurrent(revocationGeneration, startingProjection)
          ? 'reconcile'
          : 'quarantine'
      }
      const headers = currentCsrfHeaders()
      if (!headers) {
        restoreKnownFailure()
        throw new SessionManagementFailure('csrf-unavailable')
      }
      const response = await requestSessionRevocation({
        credentials: sameOriginCredentials,
        headers,
        path: { session_id: sessionId },
        signal: credentialMutationSignal(),
      })
      if (response.error === undefined && response.response?.status === 204) {
        if (revokesCurrentSession) markInitialized()
        else managedSessions.value = managedSessions.value.filter((item) => item.id !== sessionId)
        disposition = 'reconcile'
        return
      }
      if (response.response?.status === 401) {
        markInitialized()
        disposition = 'reconcile'
        throw new SessionManagementFailure('session-invalid')
      }
      if (
        response.response?.status === 403 &&
        problemCode(response.error) === 'RECENT_AUTH_REQUIRED'
      ) {
        restoreKnownFailure()
        throw new SessionManagementFailure('recent-auth-required')
      }
      if (response.response?.status === 400 || response.response?.status === 403) {
        restoreKnownFailure()
        throw new SessionManagementFailure('request-rejected')
      }
      if (
        response.response?.status !== undefined &&
        response.response.status >= 400 &&
        response.response.status < 500
      ) {
        restoreKnownFailure()
        throw new SessionManagementFailure('unavailable')
      }
      throw new SessionManagementFailure('outcome-unknown')
    } catch (error) {
      const failure =
        error instanceof SessionManagementFailure
          ? error
          : new SessionManagementFailure('outcome-unknown')
      if (failure.reason === 'outcome-unknown' && revokesCurrentSession) {
        disposition = (await requireReloginAfterUnknownMutation()) ? 'reconcile' : 'quarantine'
      } else if (failure.reason === 'outcome-unknown') {
        restoreKnownFailure()
      }
      throw failure
    } finally {
      revokingSessionIds.value = revokingSessionIds.value.filter(
        (candidate) => candidate !== sessionId,
      )
      finishCredentialMutation(credentialLease, disposition)
    }
  }

  const logoutAll = async (keepCurrent: boolean) => {
    if (status.value !== 'authenticated') {
      throw new SessionManagementFailure('session-invalid')
    }
    const startingProjection = { actor: actor.value!, session: session.value! }
    const expectedCursor = authenticatedCredentialCursor()
    if (!expectedCursor) throw new SessionManagementFailure('session-invalid')
    let credentialLease: CredentialMutationLease | undefined
    let disposition: CredentialDisposition = 'quarantine'
    logoutAllPending.value = true
    try {
      credentialLease = await acquireCredentialMutation(participantId, 'logout-all', expectedCursor)
      if (!credentialLease) {
        enterCredentialQuarantine()
        throw new SessionManagementFailure('unavailable')
      }
      const logoutAllGeneration = snapshotGeneration
      const restoreKnownFailure = () => {
        disposition = acceptAuthenticatedIfCurrent(logoutAllGeneration, startingProjection)
          ? 'reconcile'
          : 'quarantine'
      }
      const headers = currentCsrfHeaders()
      if (!headers) {
        restoreKnownFailure()
        throw new SessionManagementFailure('csrf-unavailable')
      }
      const response = await requestLogoutAll({
        credentials: sameOriginCredentials,
        headers,
        body: { keep_current: keepCurrent },
        signal: credentialMutationSignal(),
      })
      if (response.error === undefined) {
        if (!keepCurrent && response.response?.status === 204) {
          markInitialized()
          disposition = 'reconcile'
          return undefined
        }
        if (
          keepCurrent &&
          response.response?.status === 200 &&
          response.data !== undefined &&
          validAuthenticatedProjection(response.data.data) &&
          validRevokedSessionCount(response.data.data.revoked_sessions)
        ) {
          if (!acceptAuthenticatedIfCurrent(logoutAllGeneration, response.data.data)) {
            throw new SessionManagementFailure('outcome-unknown')
          }
          managedSessions.value = [{ ...response.data.data.session, is_current: true }]
          disposition = 'reconcile'
          return response.data.data.revoked_sessions
        }
      }

      const httpStatus = response.response?.status
      if (httpStatus === 401) {
        markInitialized()
        disposition = 'reconcile'
        throw new SessionManagementFailure('session-invalid')
      }
      if (httpStatus === 403 && problemCode(response.error) === 'RECENT_AUTH_REQUIRED') {
        restoreKnownFailure()
        throw new SessionManagementFailure('recent-auth-required')
      }
      if (
        httpStatus === 400 ||
        httpStatus === 403 ||
        httpStatus === 413 ||
        httpStatus === 415 ||
        httpStatus === 422
      ) {
        restoreKnownFailure()
        throw new SessionManagementFailure('request-rejected')
      }
      if (httpStatus !== undefined && httpStatus >= 400 && httpStatus < 500) {
        restoreKnownFailure()
        throw new SessionManagementFailure('unavailable')
      }
      throw new SessionManagementFailure('outcome-unknown')
    } catch (error) {
      const failure =
        error instanceof SessionManagementFailure
          ? error
          : new SessionManagementFailure('outcome-unknown')
      if (failure.reason === 'outcome-unknown') {
        disposition = (await requireReloginAfterUnknownMutation()) ? 'reconcile' : 'quarantine'
      }
      throw failure
    } finally {
      logoutAllPending.value = false
      finishCredentialMutation(credentialLease, disposition)
    }
  }

  const logout = async () => {
    enterCredentialQuarantine()
    let credentialLease: CredentialMutationLease | undefined
    let disposition: CredentialDisposition = 'quarantine'
    logoutPending.value = true
    try {
      credentialLease = await acquireCredentialMutation(participantId, 'logout')
      if (!credentialLease) throw new LogoutFailure('unavailable')
      const logoutGeneration = snapshotGeneration
      const csrfToken = readCsrfCookie(document.cookie)
      if (!csrfToken) throw new LogoutFailure('csrf-unavailable')
      const response = await requestLogout({
        credentials: sameOriginCredentials,
        headers: { 'x-nodecontroll-csrf': csrfToken },
        signal: credentialMutationSignal(),
      })
      if (response.error === undefined && response.response?.status === 204) {
        setStatusIfCurrent(logoutGeneration, 'anonymous')
        disposition = 'reconcile'
        return
      }
      if (response.response?.status === 403) throw new LogoutFailure('request-rejected')
      throw new LogoutFailure('unavailable')
    } catch (error) {
      if (error instanceof LogoutFailure) throw error
      throw new LogoutFailure('unavailable')
    } finally {
      logoutPending.value = false
      finishCredentialMutation(credentialLease, disposition)
    }
  }

  return {
    acceptAuthenticated,
    actor,
    changePassword,
    ensureLoaded,
    hasResolutionError,
    isAuthenticated,
    isResolving,
    listSessions,
    login,
    loginPending,
    logout,
    logoutAll,
    logoutAllPending,
    logoutPending,
    managedSessions,
    markInitialized,
    passwordChangePending,
    passwordChangeRequired,
    reauthenticate,
    reauthenticationPending,
    recentAuthExpired,
    recentAuthValid,
    refresh,
    revokeSession,
    revokingSessionIds,
    session,
    sessionListPending,
    status,
    syncRecentAuthClock,
  }
})
