import 'vue-router'

import type { KnownCapability } from './capabilities'

declare module 'vue-router' {
  interface RouteMeta {
    allowDuringPasswordChange?: boolean
    guestOnly?: boolean
    requiredCapabilities?: readonly KnownCapability[]
    requiresAuth?: boolean
    requiresRecentAuth?: boolean
    title?: string
    titleKey?: string
  }
}

export {}
