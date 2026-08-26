<script setup lang="ts">
import type { SafeDisplayValue } from './types'

defineProps<{
  emptyLabel?: string
  redactedLabel?: string
  value: SafeDisplayValue
}>()
</script>

<template>
  <span v-if="value.kind === 'redacted'" class="safe-display-value safe-display-value--redacted">
    <v-icon aria-hidden="true" icon="mdi-eye-off-outline" size="small" />
    <span>{{ value.label ?? redactedLabel ?? '敏感值已隐藏' }}</span>
  </span>
  <span v-else-if="value.kind === 'empty'" class="safe-display-value text-medium-emphasis">
    {{ value.label ?? emptyLabel ?? '未设置' }}
  </span>
  <code v-else-if="value.code" class="safe-display-value safe-display-value--code">
    {{ value.text }}
  </code>
  <span v-else class="safe-display-value">{{ value.text }}</span>
</template>

<style scoped>
.safe-display-value {
  min-width: 0;
  overflow-wrap: anywhere;
}

.safe-display-value--redacted {
  display: inline-flex;
  align-items: center;
  gap: 6px;
}

.safe-display-value--code {
  padding: 2px 6px;
  border-radius: 4px;
  background: rgb(var(--v-theme-surface-variant));
  color: rgb(var(--v-theme-on-surface));
  font-family: ui-monospace, SFMono-Regular, Consolas, monospace;
  white-space: normal;
}
</style>
