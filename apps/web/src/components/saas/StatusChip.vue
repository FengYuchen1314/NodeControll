<script setup lang="ts">
import { computed } from 'vue'

import type { StatusTone } from './types'

const props = defineProps<{
  description?: string
  icon?: string
  label: string
  observedAt?: string
  source: string
  tone: StatusTone
}>()

const toneContract: Record<StatusTone, { color: string; icon: string }> = {
  error: { color: 'error', icon: 'mdi-alert-circle-outline' },
  info: { color: 'info', icon: 'mdi-information-outline' },
  neutral: { color: 'secondary', icon: 'mdi-circle-medium' },
  success: { color: 'success', icon: 'mdi-check-circle-outline' },
  warning: { color: 'warning', icon: 'mdi-alert-outline' },
}

const color = computed(() => toneContract[props.tone].color)
const resolvedIcon = computed(() => props.icon ?? toneContract[props.tone].icon)
const evidence = computed(() =>
  [
    props.description,
    `来源：${props.source}`,
    props.observedAt ? `时间：${props.observedAt}` : undefined,
  ]
    .filter((part): part is string => Boolean(part))
    .join(' · '),
)
const accessibleLabel = computed(() => `${props.label}；${evidence.value}`)
</script>

<template>
  <v-tooltip location="bottom" max-width="360">
    <template #activator="{ props: activatorProps }">
      <v-chip
        v-bind="activatorProps"
        :aria-label="accessibleLabel"
        :color="color"
        :prepend-icon="resolvedIcon"
        class="status-chip"
        size="small"
        tabindex="0"
        variant="tonal"
      >
        {{ label }}
      </v-chip>
    </template>
    <span>{{ evidence }}</span>
  </v-tooltip>
</template>

<style scoped>
.status-chip {
  max-width: 100%;
  font-weight: 600;
}
</style>
