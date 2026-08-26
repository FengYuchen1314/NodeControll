export const appRouteNames = {
  dashboard: 'dashboard',
  login: 'login',
  passwordChange: 'password-change',
  profileSecurity: 'profile-security',
  reauthenticate: 'reauth',
  setup: 'setup',
  system: 'system',
} as const

export type AppRouteName = (typeof appRouteNames)[keyof typeof appRouteNames]
export type ShellRouteName =
  | typeof appRouteNames.dashboard
  | typeof appRouteNames.passwordChange
  | typeof appRouteNames.profileSecurity
  | typeof appRouteNames.system
