import type { RouteLocationRaw } from 'vue-router'

export type StatusTone = 'error' | 'info' | 'neutral' | 'success' | 'warning'

export type ResourceBreadcrumb = Readonly<{
  label: string
  to?: RouteLocationRaw
}>

export type SafeDisplayValue =
  | Readonly<{ code?: boolean; kind: 'text'; text: string }>
  | Readonly<{ kind: 'redacted'; label?: string }>
  | Readonly<{ kind: 'empty'; label?: string }>

export type DesiredReportedState = 'drift' | 'match' | 'pending' | 'unknown'

export type DesiredReportedField = Readonly<{
  desired: SafeDisplayValue
  evidenceSource: string
  evidenceTime?: string
  explanation?: string
  id: string
  label: string
  lastGood?: SafeDisplayValue
  reported: SafeDisplayValue
  state: DesiredReportedState
}>

export type PolicyContributorState = 'applied' | 'excluded' | 'overridden'

export type PolicyContributor = Readonly<{
  id: string
  label: string
  priority: number
  scope?: string
  state: PolicyContributorState
  timeRange?: string
  value: SafeDisplayValue
}>
