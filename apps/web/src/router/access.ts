import type { RouteMeta } from 'vue-router'

export function routeCapabilityAllowed(meta: RouteMeta, capabilities: readonly string[]) {
  const required = meta.requiredCapabilities ?? []
  if (required.length === 0) return true
  const granted = new Set(capabilities)
  return required.every((capability) => granted.has(capability))
}
