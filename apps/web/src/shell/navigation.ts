import type { Router, RouteMeta } from 'vue-router'

import { routeCapabilityAllowed } from '../router/access'
import { appRouteNames, type ShellRouteName } from '../router/route-names'

export type NavigationLocation = 'account' | 'command' | 'drawer'

export type NavigationDefinition = Readonly<{
  icon: string
  id: string
  labelKey: string
  locations: readonly NavigationLocation[]
  routeName: ShellRouteName
}>

export const navigationRegistry = [
  {
    icon: 'mdi-view-dashboard-outline',
    id: 'dashboard',
    labelKey: 'nav.dashboard',
    locations: ['drawer', 'command'],
    routeName: appRouteNames.dashboard,
  },
  {
    icon: 'mdi-cog-outline',
    id: 'system',
    labelKey: 'nav.system',
    locations: ['drawer', 'command'],
    routeName: appRouteNames.system,
  },
  {
    icon: 'mdi-shield-account-outline',
    id: 'profile-security',
    labelKey: 'nav.security',
    locations: ['drawer', 'command', 'account'],
    routeName: appRouteNames.profileSecurity,
  },
  {
    icon: 'mdi-lock-reset',
    id: 'password-change',
    labelKey: 'nav.password',
    locations: ['drawer', 'command', 'account'],
    routeName: appRouteNames.passwordChange,
  },
] as const satisfies readonly NavigationDefinition[]

export type ProjectedNavigationItem = NavigationDefinition &
  Readonly<{
    meta: RouteMeta
  }>

export type NavigationProjectionContext = Readonly<{
  capabilities: readonly string[]
  passwordChangeRequired: boolean
}>

export function routeVisibleInShell(meta: RouteMeta, context: NavigationProjectionContext) {
  if (meta.requiresAuth !== true || meta.guestOnly === true) return false
  if (context.passwordChangeRequired && meta.allowDuringPasswordChange !== true) return false
  return routeCapabilityAllowed(meta, context.capabilities)
}

export function projectNavigation(
  router: Router,
  context: NavigationProjectionContext,
): readonly ProjectedNavigationItem[] {
  return navigationRegistry.flatMap((definition) => {
    try {
      const resolved = router.resolve({ name: definition.routeName })
      const matched = resolved.matched.at(-1)
      if (!matched || matched.name !== definition.routeName) return []
      if (!routeVisibleInShell(matched.meta, context)) return []
      return [{ ...definition, meta: matched.meta }]
    } catch {
      return []
    }
  })
}

export function navigationAt(
  items: readonly ProjectedNavigationItem[],
  location: NavigationLocation,
) {
  return items.filter((item) => item.locations.includes(location))
}
