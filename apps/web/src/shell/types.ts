import type { ShellRouteName } from '../router/route-names'

export type ShellNavigationItem = Readonly<{
  icon: string
  id: string
  label: string
  routeName: ShellRouteName
}>
