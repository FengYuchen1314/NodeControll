<script setup lang="ts">
import { computed } from 'vue'

import type { JobChipLabels, JobPresentation, JobPresentationState, StatusTone } from './types'

const props = withDefaults(
  defineProps<{
    interactive?: boolean
    job: JobPresentation
    labels: JobChipLabels
  }>(),
  {
    interactive: true,
  },
)

const emit = defineEmits<{
  open: [jobId: string]
}>()

const stateContract: Record<
  JobPresentationState,
  { icon: string; tone: StatusTone }
> = {
  cancelled: { icon: 'mdi-cancel', tone: 'neutral' },
  expired: { icon: 'mdi-timer-off-outline', tone: 'warning' },
  failed: { icon: 'mdi-alert-circle-outline', tone: 'error' },
  queued: { icon: 'mdi-clock-outline', tone: 'neutral' },
  running: { icon: 'mdi-progress-clock', tone: 'info' },
  succeeded: { icon: 'mdi-check-circle-outline', tone: 'success' },
  waiting: { icon: 'mdi-pause-circle-outline', tone: 'warning' },
}

const state = computed(() => stateContract[props.job.state])
const progress = computed(() =>
  props.job.progressPercent === undefined || !Number.isFinite(props.job.progressPercent)
    ? undefined
    : Math.min(100, Math.max(0, Math.round(props.job.progressPercent))),
)
const accessibleLabel = computed(() =>
  [
    props.job.label,
    props.labels.states[props.job.state],
    progress.value === undefined ? undefined : `${progress.value}%`,
    props.labels.source(props.job.source),
    props.job.updatedAt ? props.labels.updatedAt(props.job.updatedAt) : undefined,
  ]
    .filter((part): part is string => Boolean(part))
    .join(', '),
)
</script>

<template>
  <v-chip
    :aria-label="accessibleLabel"
    :color="state.tone === 'neutral' ? 'secondary' : state.tone"
    :link="interactive"
    :prepend-icon="state.icon"
    :tabindex="interactive ? 0 : undefined"
    variant="tonal"
    @click="interactive && emit('open', job.id)"
    @keydown.enter.prevent="interactive && emit('open', job.id)"
    @keydown.space.prevent="interactive && emit('open', job.id)"
  >
    {{ job.label }} · {{ labels.states[job.state] }}<span v-if="progress !== undefined"> · {{ progress }}%</span>
  </v-chip>
</template>
