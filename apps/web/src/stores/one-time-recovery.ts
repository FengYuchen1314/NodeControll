import { computed, onScopeDispose, shallowRef } from 'vue'
import { defineStore } from 'pinia'

import { subscribeCredentialCoordination } from '../lib/credential-coordinator'
import { validOneTimeRecoveryCodes } from '../api/recovery-codes'

/**
 * Memory-only handoff for a regeneration response that can outlive the protected page while the
 * credential coordinator closes and reopens the authenticated DOM. No persistence plugin is used.
 */
export const useOneTimeRecoveryCodeStore = defineStore('one-time-recovery-codes', () => {
  const values = shallowRef<string[]>([])
  const hasCodes = computed(() => values.value.length > 0)
  let operationOwner: symbol | undefined

  const clear = () => {
    for (let index = 0; index < values.value.length; index += 1) values.value[index] = ''
    values.value = []
  }

  const accept = (codes: unknown) => {
    clear()
    if (!validOneTimeRecoveryCodes(codes)) return false
    values.value = [...codes]
    return true
  }

  const beginOperation = () => {
    clear()
    const owner = Symbol('one-time-recovery-code-operation')
    operationOwner = owner
    return owner
  }

  const ownsOperation = (owner: symbol) => operationOwner === owner

  const releaseOperation = (owner: symbol) => {
    if (operationOwner === owner) operationOwner = undefined
  }

  const abandonOperation = () => {
    operationOwner = undefined
    clear()
  }

  const acceptForOperation = (owner: symbol, codes: unknown) => {
    if (!ownsOperation(owner)) return false
    const accepted = accept(codes)
    if (accepted) operationOwner = undefined
    return accepted
  }

  const abandonForPageHide = () => {
    abandonOperation()
  }

  globalThis.addEventListener('pagehide', abandonForPageHide)

  const unsubscribe = subscribeCredentialCoordination((event) => {
    if (
      event.kind === 'corrupt' ||
      event.kind === 'observed-invalid' ||
      event.kind === 'reset'
    ) {
      clear()
      return
    }
    if (
      event.kind === 'record' &&
      (event.record.phase !== 'settled' || event.record.disposition === 'quarantine')
    ) {
      clear()
    }
  })

  onScopeDispose(() => {
    abandonOperation()
    globalThis.removeEventListener('pagehide', abandonForPageHide)
    unsubscribe()
  })

  return {
    abandonOperation,
    accept,
    acceptForOperation,
    beginOperation,
    clear,
    codes: values,
    hasCodes,
    ownsOperation,
    releaseOperation,
  }
})
