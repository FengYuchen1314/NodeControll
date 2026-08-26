import { afterEach, beforeEach } from 'vitest'

import { CREDENTIAL_COORDINATION_KEY } from '../lib/credential-coordinator'

class ResizeObserverStub {
  disconnect() {}

  observe() {}

  unobserve() {}
}

class VisualViewportStub extends globalThis.EventTarget {
  readonly height = 768
  readonly offsetLeft = 0
  readonly offsetTop = 0
  readonly pageLeft = 0
  readonly pageTop = 0
  readonly scale = 1
  readonly width = 1_024
  onresize = null
  onscroll = null
  onscrollend = null
}

class BroadcastChannelStub extends globalThis.EventTarget {
  onmessage = null
  onmessageerror = null

  constructor(readonly name: string) {
    super()
  }

  close() {}

  postMessage() {}
}

class LockManagerStub {
  private readonly tails = new Map<string, Promise<void>>()

  async request<T>(
    name: string,
    optionsOrCallback: LockOptions | ((lock: Lock) => PromiseLike<T> | T),
    callbackArgument?: (lock: Lock) => PromiseLike<T> | T,
  ): Promise<T> {
    const callback =
      typeof optionsOrCallback === 'function' ? optionsOrCallback : callbackArgument
    if (!callback) throw new TypeError('A Web Lock callback is required')
    const mode =
      typeof optionsOrCallback === 'function' ? 'exclusive' : (optionsOrCallback.mode ?? 'exclusive')
    const previous = this.tails.get(name) ?? Promise.resolve()
    let releaseTurn!: () => void
    const turn = new Promise<void>((resolve) => {
      releaseTurn = resolve
    })
    const tail = previous.then(() => turn)
    this.tails.set(name, tail)
    await previous
    try {
      return await callback({ mode, name } as Lock)
    } finally {
      releaseTurn()
      if (this.tails.get(name) === tail) {
        void tail.then(() => {
          if (this.tails.get(name) === tail) this.tails.delete(name)
        })
      }
    }
  }
}

Object.defineProperty(globalThis, 'ResizeObserver', {
  configurable: true,
  value: ResizeObserverStub,
  writable: true,
})

Object.defineProperty(globalThis, 'visualViewport', {
  configurable: true,
  value: new VisualViewportStub(),
  writable: true,
})

Object.defineProperty(globalThis, 'BroadcastChannel', {
  configurable: true,
  value: BroadcastChannelStub,
  writable: true,
})

Object.defineProperty(globalThis.navigator, 'locks', {
  configurable: true,
  value: new LockManagerStub(),
})

beforeEach(() => {
  globalThis.localStorage.clear()
  globalThis.sessionStorage.clear()
  globalThis.localStorage.setItem(
    CREDENTIAL_COORDINATION_KEY,
    JSON.stringify({
      baseSeq: '1',
      disposition: 'reconcile',
      epoch: '20000000-0000-4000-8000-000000000001',
      opId: '20000000-0000-4000-8000-000000000002',
      operation: 'login',
      phase: 'settled',
      senderId: '20000000-0000-4000-8000-000000000003',
      seq: '2',
      v: 1,
    }),
  )
})

afterEach(async () => {
  const { resetCredentialCoordinatorForTests } = await import('../lib/credential-coordinator')
  resetCredentialCoordinatorForTests()
})
