import { createPinia, disposePinia, setActivePinia } from 'pinia'
import { afterEach, beforeEach, describe, expect, it } from 'vitest'

import { CREDENTIAL_COORDINATION_KEY } from '../lib/credential-coordinator'
import { useOneTimeRecoveryCodeStore } from './one-time-recovery'

const codes = Array.from(
  { length: 8 },
  (_, index) => `${index.toString(16).padStart(4, '0')}-1111-2222-3333-4444-5555-6666-7777`,
)

let pinia: ReturnType<typeof createPinia>

beforeEach(() => {
  pinia = createPinia()
  setActivePinia(pinia)
})

afterEach(() => {
  disposePinia(pinia)
})

describe('one-time recovery-code store', () => {
  it('binds plaintext handoff to an in-memory operation owner and can abandon it', () => {
    const store = useOneTimeRecoveryCodeStore()
    const owner = store.beginOperation()
    const unrelatedOwner = Symbol('unrelated')

    expect(store.acceptForOperation(unrelatedOwner, codes)).toBe(false)
    expect(store.hasCodes).toBe(false)
    expect(store.ownsOperation(owner)).toBe(true)
    store.clear()
    expect(store.acceptForOperation(owner, codes)).toBe(true)
    expect(store.hasCodes).toBe(true)

    const abandonedOwner = store.beginOperation()
    store.abandonOperation()
    expect(store.acceptForOperation(abandonedOwner, codes)).toBe(false)
    expect(store.hasCodes).toBe(false)
  })

  it('keeps codes in memory and clears them when a credential mutation starts', () => {
    const store = useOneTimeRecoveryCodeStore()
    expect(store.accept(codes)).toBe(true)
    expect(store.codes).toEqual(codes)
    expect(JSON.stringify(globalThis.localStorage)).not.toContain(codes[0])

    const inflight = JSON.stringify({
      baseSeq: '2',
      disposition: 'quarantine',
      epoch: '20000000-0000-4000-8000-000000000001',
      opId: '20000000-0000-4000-8000-000000000004',
      operation: 'change-password',
      phase: 'inflight',
      senderId: '20000000-0000-4000-8000-000000000005',
      seq: '3',
      v: 1,
    })
    globalThis.localStorage.setItem(CREDENTIAL_COORDINATION_KEY, inflight)
    globalThis.dispatchEvent(
      new StorageEvent('storage', { key: CREDENTIAL_COORDINATION_KEY, newValue: inflight }),
    )

    expect(store.hasCodes).toBe(false)
    expect(store.codes).toEqual([])
  })
})
