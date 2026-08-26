export const CREDENTIAL_COORDINATION_KEY = 'nodecontroll:credential-coordination:v1'

const credentialChannelName = 'nodecontroll:credential-coordination:v1'
const credentialLockName = 'nodecontroll:credential-cookie'
const protocolVersion = 1
const lockWaitTimeoutMs = 15_000
const maximumSequence = 18_446_744_073_709_551_615n

export type CredentialDisposition = 'quarantine' | 'reconcile'
export type CredentialMutationOperation =
  | 'change-password'
  | 'login'
  | 'logout'
  | 'logout-all'
  | 'reauth'
  | 'regenerate-recovery-codes'
  | 'revoke'

type CredentialRecordOperation = CredentialMutationOperation | 'read-401'

type CredentialRecordBase = {
  baseSeq: string
  disposition: CredentialDisposition
  epoch: string
  opId: string
  operation: CredentialRecordOperation
  senderId: string
  seq: string
  v: typeof protocolVersion
}

export type CredentialInflightRecord = CredentialRecordBase & {
  disposition: 'quarantine'
  phase: 'inflight'
}

export type CredentialSettledRecord = CredentialRecordBase & {
  phase: 'settled'
}

export type CredentialInvalidatedRecord = CredentialRecordBase & {
  disposition: 'reconcile'
  observedSessionId?: string
  operation: 'read-401'
  phase: 'invalidated'
}

export type CredentialCoordinationRecord =
  CredentialInflightRecord | CredentialInvalidatedRecord | CredentialSettledRecord

export type CredentialCoordinationCursor = {
  epoch?: string
  phase: 'absent' | CredentialCoordinationRecord['phase']
  seq: string
}

export type CredentialReadObservation = {
  cursor: CredentialCoordinationCursor
}

export type CredentialCoordinationSnapshot =
  | { kind: 'absent' }
  | { kind: 'invalid' }
  | { kind: 'record'; record: CredentialCoordinationRecord }

type CredentialObservedInvalidation = {
  baseEpoch?: string
  baseSeq: string
  eventId: string
  kind: 'observed-invalid'
  observedSessionId?: string
  senderId: string
  v: typeof protocolVersion
}

export type CredentialCoordinationEvent =
  | { kind: 'corrupt' }
  | { kind: 'reset' }
  | { kind: 'observed-invalid'; message: CredentialObservedInvalidation }
  | { kind: 'record'; record: CredentialCoordinationRecord }

export type CredentialMutationLease = {
  inflight: CredentialInflightRecord
  settle: (disposition: CredentialDisposition) => boolean
}

type CredentialEventListener = (event: CredentialCoordinationEvent) => void

const identifierPattern =
  /^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i
const randomIdentifierPattern =
  /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i
const sequencePattern = /^(0|[1-9][0-9]{0,19})$/
const mutationOperations = new Set<CredentialMutationOperation>([
  'change-password',
  'login',
  'logout',
  'logout-all',
  'reauth',
  'regenerate-recovery-codes',
  'revoke',
])
const listeners = new Set<CredentialEventListener>()
let channel: BroadcastChannel | undefined
let storageListenerInstalled = false

const validIdentifier = (value: unknown): value is string =>
  typeof value === 'string' && identifierPattern.test(value)

const validRandomIdentifier = (value: unknown): value is string =>
  typeof value === 'string' && randomIdentifierPattern.test(value)

const validSequence = (value: unknown): value is string => {
  if (typeof value !== 'string' || !sequencePattern.test(value)) return false
  try {
    return BigInt(value) <= maximumSequence
  } catch {
    return false
  }
}

const exactKeys = (candidate: Record<string, unknown>, expected: string[]) => {
  const actual = Object.keys(candidate).sort()
  const sortedExpected = [...expected].sort()
  return (
    actual.length === sortedExpected.length &&
    actual.every((key, index) => key === sortedExpected[index])
  )
}

const validRecord = (value: unknown): value is CredentialCoordinationRecord => {
  if (typeof value !== 'object' || value === null) return false
  const candidate = value as Record<string, unknown>
  const commonKeys = [
    'baseSeq',
    'disposition',
    'epoch',
    'opId',
    'operation',
    'phase',
    'senderId',
    'seq',
    'v',
  ]
  const expectedKeys =
    candidate.phase === 'invalidated' && candidate.observedSessionId !== undefined
      ? [...commonKeys, 'observedSessionId']
      : commonKeys
  if (
    !exactKeys(candidate, expectedKeys) ||
    candidate.v !== protocolVersion ||
    !validRandomIdentifier(candidate.epoch) ||
    !validRandomIdentifier(candidate.opId) ||
    !validRandomIdentifier(candidate.senderId) ||
    !validSequence(candidate.baseSeq) ||
    !validSequence(candidate.seq)
  ) {
    return false
  }
  const baseSequence = BigInt(candidate.baseSeq)
  const sequence = BigInt(candidate.seq)
  if (sequence < 1n || baseSequence + 1n !== sequence) return false

  if (candidate.phase === 'inflight') {
    return (
      candidate.disposition === 'quarantine' &&
      typeof candidate.operation === 'string' &&
      mutationOperations.has(candidate.operation as CredentialMutationOperation)
    )
  }
  if (candidate.phase === 'settled') {
    return (
      (candidate.disposition === 'quarantine' || candidate.disposition === 'reconcile') &&
      typeof candidate.operation === 'string' &&
      mutationOperations.has(candidate.operation as CredentialMutationOperation)
    )
  }
  return (
    candidate.phase === 'invalidated' &&
    candidate.disposition === 'reconcile' &&
    candidate.operation === 'read-401' &&
    (candidate.observedSessionId === undefined || validIdentifier(candidate.observedSessionId))
  )
}

const validObservedInvalidation = (value: unknown): value is CredentialObservedInvalidation => {
  if (typeof value !== 'object' || value === null) return false
  const candidate = value as Record<string, unknown>
  const commonKeys = ['baseSeq', 'eventId', 'kind', 'senderId', 'v']
  const expectedKeys = [
    ...commonKeys,
    ...(candidate.baseEpoch === undefined ? [] : ['baseEpoch']),
    ...(candidate.observedSessionId === undefined ? [] : ['observedSessionId']),
  ]
  return (
    exactKeys(candidate, expectedKeys) &&
    candidate.v === protocolVersion &&
    candidate.kind === 'observed-invalid' &&
    validRandomIdentifier(candidate.eventId) &&
    validRandomIdentifier(candidate.senderId) &&
    validSequence(candidate.baseSeq) &&
    (candidate.baseEpoch === undefined || validRandomIdentifier(candidate.baseEpoch)) &&
    (candidate.observedSessionId === undefined || validIdentifier(candidate.observedSessionId))
  )
}

const cursorForRecord = (record: CredentialCoordinationRecord): CredentialCoordinationCursor => ({
  epoch: record.epoch,
  phase: record.phase,
  seq: record.seq,
})

const sameCursor = (left: CredentialCoordinationCursor, right: CredentialCoordinationCursor) =>
  left.epoch === right.epoch && left.phase === right.phase && left.seq === right.seq

const incrementSequence = (sequence: string) => {
  const next = BigInt(sequence) + 1n
  if (next > maximumSequence) return undefined
  return next.toString()
}

const readSnapshot = (): CredentialCoordinationSnapshot => {
  try {
    const raw = globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY)
    if (raw === null) return { kind: 'absent' }
    const parsed: unknown = JSON.parse(raw)
    return validRecord(parsed) ? { kind: 'record', record: parsed } : { kind: 'invalid' }
  } catch {
    return { kind: 'invalid' }
  }
}

const cursorForSnapshot = (
  snapshot: CredentialCoordinationSnapshot,
): CredentialCoordinationCursor | undefined => {
  if (snapshot.kind === 'invalid') return undefined
  return snapshot.kind === 'absent'
    ? { phase: 'absent', seq: '0' }
    : cursorForRecord(snapshot.record)
}

const dispatch = (event: CredentialCoordinationEvent) => {
  for (const listener of listeners) listener(event)
}

const dispatchCurrentRecord = () => {
  const snapshot = readSnapshot()
  if (snapshot.kind === 'invalid') {
    dispatch({ kind: 'corrupt' })
  } else if (snapshot.kind === 'absent') {
    dispatch({ kind: 'reset' })
  } else {
    dispatch({ kind: 'record', record: snapshot.record })
  }
}

const observationMatchesCurrent = (message: CredentialObservedInvalidation) => {
  const cursor = cursorForSnapshot(readSnapshot())
  if (!cursor) return false
  return (
    cursor.epoch === message.baseEpoch &&
    cursor.seq === message.baseSeq &&
    (cursor.phase === 'absent' || cursor.phase === 'settled')
  )
}

const ensureTransport = () => {
  if (!storageListenerInstalled && typeof globalThis.addEventListener === 'function') {
    globalThis.addEventListener('storage', handleStorageEvent)
    storageListenerInstalled = true
  }
  if (channel) return channel
  if (typeof globalThis.BroadcastChannel !== 'function') return undefined
  try {
    channel = new globalThis.BroadcastChannel(credentialChannelName)
    channel.addEventListener('message', (event) => {
      if (validRecord(event.data)) {
        const snapshot = readSnapshot()
        if (
          snapshot.kind === 'record' &&
          snapshot.record.epoch === event.data.epoch &&
          snapshot.record.seq === event.data.seq &&
          snapshot.record.phase === event.data.phase
        ) {
          dispatch({ kind: 'record', record: snapshot.record })
        }
        return
      }
      if (validObservedInvalidation(event.data) && observationMatchesCurrent(event.data)) {
        dispatch({ kind: 'observed-invalid', message: event.data })
      }
    })
    return channel
  } catch {
    return undefined
  }
}

function handleStorageEvent(event: StorageEvent) {
  if (event.key === CREDENTIAL_COORDINATION_KEY) dispatchCurrentRecord()
}

const lockManager = () => {
  if (typeof navigator === 'undefined') return undefined
  const locks = navigator.locks
  return locks && typeof locks.request === 'function' ? locks : undefined
}

const storageReadable = () => {
  try {
    globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY)
    return true
  } catch {
    return false
  }
}

const persistRecord = (record: CredentialCoordinationRecord) => {
  const serialized = JSON.stringify(record)
  try {
    globalThis.localStorage.setItem(CREDENTIAL_COORDINATION_KEY, serialized)
    if (globalThis.localStorage.getItem(CREDENTIAL_COORDINATION_KEY) !== serialized) return false
  } catch {
    return false
  }
  dispatch({ kind: 'record', record })
  return true
}

const broadcastRecord = (record: CredentialCoordinationRecord) => {
  try {
    ensureTransport()?.postMessage(record)
    return channel !== undefined
  } catch {
    return false
  }
}

const recoveryOperation = (operation: CredentialMutationOperation) =>
  operation === 'login' || operation === 'logout'

export class CredentialCoordinationFailure extends Error {
  constructor(readonly reason: 'invalidated' | 'quarantine' | 'unavailable') {
    super('Credential coordination failed')
    this.name = 'CredentialCoordinationFailure'
  }
}

export const newCredentialParticipantId = () =>
  typeof globalThis.crypto?.randomUUID === 'function' ? globalThis.crypto.randomUUID() : undefined

export const credentialCoordinationSupported = () =>
  typeof globalThis.crypto?.randomUUID === 'function' &&
  lockManager() !== undefined &&
  storageReadable() &&
  ensureTransport() !== undefined

export const getCredentialCoordinationSnapshot = () => readSnapshot()

export const getCredentialCoordinationCursor = () => {
  const snapshot = readSnapshot()
  if (
    snapshot.kind !== 'record' ||
    snapshot.record.phase !== 'settled' ||
    snapshot.record.disposition !== 'reconcile'
  ) {
    return undefined
  }
  return cursorForRecord(snapshot.record)
}

export const subscribeCredentialCoordination = (listener: CredentialEventListener) => {
  ensureTransport()
  listeners.add(listener)
  return () => {
    listeners.delete(listener)
  }
}

export const acquireCredentialMutation = async (
  senderId: string | undefined,
  operation: CredentialMutationOperation,
  expectedCursor?: CredentialCoordinationCursor,
): Promise<CredentialMutationLease | undefined> => {
  const locks = lockManager()
  if (!locks || !validRandomIdentifier(senderId) || !credentialCoordinationSupported()) {
    return undefined
  }

  let acquisitionResolved = false
  let resolveAcquisition!: (lease: CredentialMutationLease | undefined) => void
  const acquisition = new Promise<CredentialMutationLease | undefined>((resolve) => {
    resolveAcquisition = resolve
  })
  let releaseHold: (() => void) | undefined

  let request: Promise<void>
  try {
    request = locks
      .request(
        credentialLockName,
        { mode: 'exclusive', signal: AbortSignal.timeout(lockWaitTimeoutMs) },
        async () => {
          const snapshot = readSnapshot()
          const canRecover = recoveryOperation(operation)
          const cursor =
            cursorForSnapshot(snapshot) ??
            (canRecover ? { phase: 'absent' as const, seq: '0' } : undefined)
          if (!cursor || (expectedCursor && !sameCursor(expectedCursor, cursor))) {
            acquisitionResolved = true
            resolveAcquisition(undefined)
            return
          }
          if (
            snapshot.kind === 'record' &&
            (snapshot.record.phase === 'inflight' ||
              snapshot.record.disposition === 'quarantine') &&
            !canRecover
          ) {
            acquisitionResolved = true
            resolveAcquisition(undefined)
            return
          }
          if (snapshot.kind === 'absent' && !canRecover) {
            acquisitionResolved = true
            resolveAcquisition(undefined)
            return
          }

          let epoch = snapshot.kind === 'record' ? snapshot.record.epoch : undefined
          let sequence = incrementSequence(cursor.seq)
          if (!sequence) {
            if (!canRecover) {
              acquisitionResolved = true
              resolveAcquisition(undefined)
              return
            }
            epoch = undefined
            sequence = '1'
          }
          epoch ??= globalThis.crypto.randomUUID()
          const inflight: CredentialInflightRecord = {
            baseSeq: cursor.seq,
            disposition: 'quarantine',
            epoch,
            opId: globalThis.crypto.randomUUID(),
            operation,
            phase: 'inflight',
            senderId,
            seq: sequence,
            v: protocolVersion,
          }
          if (!persistRecord(inflight) || !broadcastRecord(inflight)) {
            const terminalSequence = incrementSequence(inflight.seq)
            if (terminalSequence) {
              const terminal: CredentialSettledRecord = {
                ...inflight,
                baseSeq: inflight.seq,
                disposition: 'quarantine',
                phase: 'settled',
                seq: terminalSequence,
              }
              persistRecord(terminal)
              broadcastRecord(terminal)
            }
            acquisitionResolved = true
            resolveAcquisition(undefined)
            return
          }

          const hold = new Promise<void>((resolve) => {
            releaseHold = resolve
          })
          let finalized = false
          const lease: CredentialMutationLease = {
            inflight,
            settle: (disposition) => {
              if (finalized) return false
              finalized = true
              try {
                const current = readSnapshot()
                const terminalSequence = incrementSequence(inflight.seq)
                if (
                  current.kind !== 'record' ||
                  current.record.phase !== 'inflight' ||
                  current.record.epoch !== inflight.epoch ||
                  current.record.seq !== inflight.seq ||
                  current.record.opId !== inflight.opId ||
                  !terminalSequence
                ) {
                  return false
                }
                let terminal: CredentialSettledRecord = {
                  ...inflight,
                  baseSeq: inflight.seq,
                  disposition,
                  phase: 'settled',
                  seq: terminalSequence,
                }
                const committed = persistRecord(terminal) && broadcastRecord(terminal)
                if (!committed && disposition !== 'quarantine') {
                  const quarantineSequence = incrementSequence(terminal.seq)
                  if (quarantineSequence) {
                    terminal = {
                      ...terminal,
                      baseSeq: terminal.seq,
                      disposition: 'quarantine',
                      seq: quarantineSequence,
                    }
                    persistRecord(terminal)
                    broadcastRecord(terminal)
                  }
                }
                return committed
              } finally {
                releaseHold?.()
              }
            },
          }
          acquisitionResolved = true
          resolveAcquisition(lease)
          await hold
        },
      )
      .then(
        () => undefined,
        () => {
          if (!acquisitionResolved) {
            acquisitionResolved = true
            resolveAcquisition(undefined)
          }
        },
      )
  } catch {
    return undefined
  }

  const lease = await acquisition
  if (!lease) await request
  return lease
}

export const withCredentialReadLock = async <T>(
  operation: (observation: CredentialReadObservation) => Promise<T>,
): Promise<T> => {
  const locks = lockManager()
  if (!locks || !credentialCoordinationSupported()) {
    throw new CredentialCoordinationFailure('unavailable')
  }
  let operationFailed = false
  let operationError: unknown
  try {
    return await locks.request(
      credentialLockName,
      { mode: 'shared', signal: AbortSignal.timeout(lockWaitTimeoutMs) },
      async () => {
        const snapshot = readSnapshot()
        const cursor = cursorForSnapshot(snapshot)
        if (!cursor || snapshot.kind === 'absent') {
          throw new CredentialCoordinationFailure('quarantine')
        }
        if (
          snapshot.kind === 'record' &&
          (snapshot.record.phase === 'inflight' ||
            (snapshot.record.phase === 'settled' && snapshot.record.disposition === 'quarantine'))
        ) {
          throw new CredentialCoordinationFailure('quarantine')
        }
        if (snapshot.kind === 'record' && snapshot.record.phase === 'invalidated') {
          throw new CredentialCoordinationFailure('invalidated')
        }
        try {
          return await operation({ cursor })
        } catch (error) {
          operationFailed = true
          operationError = error
          throw error
        }
      },
    )
  } catch (error) {
    if (operationFailed) throw operationError
    if (error instanceof CredentialCoordinationFailure) throw error
    throw new CredentialCoordinationFailure('unavailable')
  }
}

export const publishCredentialInvalidation = (
  observation: CredentialReadObservation,
  senderId: string | undefined,
  observedSessionId?: string,
) => {
  const credentialChannel = ensureTransport()
  if (
    !credentialChannel ||
    !validRandomIdentifier(senderId) ||
    (observedSessionId !== undefined && !validIdentifier(observedSessionId))
  ) {
    return false
  }
  const message: CredentialObservedInvalidation = {
    ...(observation.cursor.epoch === undefined ? {} : { baseEpoch: observation.cursor.epoch }),
    baseSeq: observation.cursor.seq,
    eventId: globalThis.crypto.randomUUID(),
    kind: 'observed-invalid',
    ...(observedSessionId === undefined ? {} : { observedSessionId }),
    senderId,
    v: protocolVersion,
  }
  if (!observationMatchesCurrent(message)) return false
  dispatch({ kind: 'observed-invalid', message })
  try {
    credentialChannel.postMessage(message)
    return true
  } catch {
    return false
  }
}

export const persistCredentialInvalidation = async (
  observation: CredentialReadObservation,
  senderId: string | undefined,
  observedSessionId?: string,
) => {
  const locks = lockManager()
  if (
    !locks ||
    !validRandomIdentifier(senderId) ||
    !credentialCoordinationSupported() ||
    (observedSessionId !== undefined && !validIdentifier(observedSessionId))
  ) {
    return false
  }
  try {
    return await locks.request(
      credentialLockName,
      { mode: 'exclusive', signal: AbortSignal.timeout(lockWaitTimeoutMs) },
      async () => {
        const snapshot = readSnapshot()
        const cursor = cursorForSnapshot(snapshot)
        if (!cursor || !sameCursor(cursor, observation.cursor)) return false
        const sequence = incrementSequence(cursor.seq)
        if (!sequence) return false
        const record: CredentialInvalidatedRecord = {
          baseSeq: cursor.seq,
          disposition: 'reconcile',
          epoch: cursor.epoch ?? globalThis.crypto.randomUUID(),
          opId: globalThis.crypto.randomUUID(),
          operation: 'read-401',
          ...(observedSessionId === undefined ? {} : { observedSessionId }),
          phase: 'invalidated',
          senderId,
          seq: sequence,
          v: protocolVersion,
        }
        if (!persistRecord(record)) return false
        broadcastRecord(record)
        return true
      },
    )
  } catch {
    return false
  }
}

export const resetCredentialCoordinatorForTests = () => {
  listeners.clear()
  channel?.close()
  channel = undefined
  if (storageListenerInstalled) {
    globalThis.removeEventListener('storage', handleStorageEvent)
    storageListenerInstalled = false
  }
}
