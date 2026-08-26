export const knownCapabilities = [
  'profile:read',
  'profile:write',
  'sessions:read',
  'sessions:revoke',
  'credentials:manage',
  'users:read',
  'users:write',
  'system:read',
  'system:execute',
  'audit:read',
  'instance:manage',
] as const

export type KnownCapability = (typeof knownCapabilities)[number]
