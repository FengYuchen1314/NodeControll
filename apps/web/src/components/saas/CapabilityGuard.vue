<script setup lang="ts">
import { useId } from 'vue'

withDefaults(
  defineProps<{
    allowed: boolean
    label: string
    mode?: 'disable' | 'explain' | 'hide'
    reason: string
  }>(),
  { mode: 'hide' },
)

defineSlots<{
  default?: (props: { disabled: boolean; reason?: string }) => unknown
  fallback?: (props: { reason: string }) => unknown
}>()

const reasonId = useId()
</script>

<template>
  <slot v-if="allowed" :disabled="false" />

  <template v-else-if="mode === 'hide'">
    <slot name="fallback" :reason="reason" />
  </template>

  <fieldset
    v-else-if="mode === 'disable'"
    class="capability-guard capability-guard--disabled"
    disabled
    :aria-describedby="reasonId"
    :aria-label="label"
  >
    <div
      aria-hidden="true"
      class="capability-guard__inert-content"
      inert
      @click.capture.stop.prevent
      @keydown.capture.stop.prevent
    >
      <slot :disabled="true" :reason="reason" />
    </div>
    <p :id="reasonId" class="capability-guard__reason text-caption text-medium-emphasis">
      <v-icon aria-hidden="true" icon="mdi-shield-lock-outline" size="small" />
      <span>{{ reason }}</span>
    </p>
  </fieldset>

  <section v-else class="capability-guard" :aria-label="label">
    <slot name="fallback" :reason="reason">
      <v-alert type="info" variant="tonal">{{ reason }}</v-alert>
    </slot>
  </section>
</template>

<style scoped>
.capability-guard {
  min-width: 0;
}

.capability-guard--disabled {
  padding: 0;
  border: 0;
  margin: 0;
}

.capability-guard__inert-content {
  pointer-events: none;
}

.capability-guard__reason {
  display: flex;
  align-items: flex-start;
  gap: 6px;
  margin-block: 6px 0;
}
</style>
