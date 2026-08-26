import 'vue-router'

declare module 'vue-router' {
  interface RouteMeta {
    allowDuringPasswordChange?: boolean
    guestOnly?: boolean
    requiresAuth?: boolean
    requiresRecentAuth?: boolean
    title?: string
  }
}

export {}
