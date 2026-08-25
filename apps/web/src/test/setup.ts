class ResizeObserverStub {
  disconnect() {}

  observe() {}

  unobserve() {}
}

Object.defineProperty(globalThis, 'ResizeObserver', {
  configurable: true,
  value: ResizeObserverStub,
  writable: true,
})
