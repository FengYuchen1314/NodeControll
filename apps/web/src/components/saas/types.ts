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

export type AppDataTableColumn = Readonly<{
  align?: 'center' | 'end' | 'start'
  key: string
  label: string
  mobileLabel?: string
}>

export type AppDataTableRow = Readonly<Record<string, unknown>>

export type AppDataTableLabels = Readonly<{
  actions: string
  empty: string
  emptyValue: string
  falseValue: string
  invalidConfiguration: string
  loading: string
  mobile: string
  retry: string
  selectAll: string
  selectRow: (rowKey: string) => string
  stale: string
  trueValue: string
}>

export type JobPresentationState =
  | 'cancelled'
  | 'expired'
  | 'failed'
  | 'queued'
  | 'running'
  | 'succeeded'
  | 'waiting'
export type JobStepPresentationState = 'failed' | 'pending' | 'running' | 'skipped' | 'succeeded'

export type JobStepPresentation = Readonly<{
  id: string
  label: string
  message?: SafeDisplayValue
  state: JobStepPresentationState
}>

export type JobPresentation = Readonly<{
  createdAt?: string
  id: string
  label: string
  message?: SafeDisplayValue
  progressPercent?: number
  source: string
  state: JobPresentationState
  steps?: readonly JobStepPresentation[]
  updatedAt?: string
}>

export type JobStateLabels = Readonly<Record<JobPresentationState, string>>
export type JobStepStateLabels = Readonly<Record<JobStepPresentationState, string>>

export type JobChipLabels = Readonly<{
  source: (source: string) => string
  states: JobStateLabels
  updatedAt: (updatedAt: string) => string
}>

export type JobDrawerLabels = Readonly<{
  close: string
  createdAt: string
  empty: string
  emptyValue: string
  jobId: string
  overline: string
  progress: (percent: number) => string
  redactedValue: string
  source: string
  stepStates: JobStepStateLabels
  steps: string
  title: string
  updatedAt: string
}>
