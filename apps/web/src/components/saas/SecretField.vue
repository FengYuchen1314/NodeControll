<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'

const props = withDefaults(
  defineProps<{
    allowClear?: boolean
    clearLabel?: string
    clearRequested?: boolean
    configured?: boolean
    configuredLabel?: string
    disabled?: boolean
    hint?: string
    label: string
    mode: 'create' | 'replace'
    modelValue: string
    name?: string
    oneTimeMessage?: string
    oneTimeReveal?: boolean
    pending?: boolean
  }>(),
  {
    allowClear: false,
    clearLabel: '移除已配置的秘密值',
    clearRequested: false,
    configured: false,
    configuredLabel: '已配置',
    disabled: false,
    hint: undefined,
    name: undefined,
    oneTimeMessage: '该值只会显示一次。离开当前步骤前请保存到受保护位置。',
    oneTimeReveal: false,
    pending: false,
  },
)

const emit = defineEmits<{
  'update:clearRequested': [value: boolean]
  'update:modelValue': [value: string]
}>()

const revealed = ref(false)
const interactionDisabled = computed(() => props.disabled || props.pending || props.clearRequested)
const visibleValue = computed(() => (props.clearRequested ? '' : props.modelValue))
const revealLabel = computed(() => `${revealed.value ? '隐藏' : '显示'} ${props.label}`)

const conceal = () => {
  revealed.value = false
}

const updateValue = (value: string | null) => {
  if (interactionDisabled.value) return
  emit('update:modelValue', value ?? '')
}

const updateClearRequest = (value: boolean | null) => {
  if (!props.allowClear || props.disabled || props.pending) return
  const nextValue = value === true
  conceal()
  emit('update:clearRequested', nextValue)
  if (nextValue) emit('update:modelValue', '')
}

const concealWhenHidden = () => {
  if (globalThis.document.visibilityState !== 'visible') conceal()
}

watch(
  () => props.clearRequested,
  (clearRequested) => {
    if (clearRequested) conceal()
  },
)

onMounted(() => {
  globalThis.document.addEventListener('visibilitychange', concealWhenHidden)
  globalThis.addEventListener('pagehide', conceal)
})

onBeforeUnmount(() => {
  conceal()
  globalThis.document.removeEventListener('visibilitychange', concealWhenHidden)
  globalThis.removeEventListener('pagehide', conceal)
})
</script>

<template>
  <div class="secret-field" :data-mode="mode">
    <div v-if="configured || oneTimeReveal" class="secret-field__state mb-2">
      <v-chip
        v-if="configured"
        color="success"
        prepend-icon="mdi-check-circle-outline"
        size="small"
        variant="tonal"
      >
        {{ configuredLabel }}
      </v-chip>
      <span v-if="configured" class="text-caption text-medium-emphasis">
        当前值不会从服务端回填。
      </span>
    </div>

    <v-alert v-if="oneTimeReveal" class="mb-3" type="warning" variant="tonal">
      {{ oneTimeMessage }}
    </v-alert>

    <v-text-field
      :autocomplete="mode === 'create' ? 'new-password' : 'off'"
      :disabled="interactionDisabled"
      :hint="hint"
      :label="label"
      :model-value="visibleValue"
      :name="name"
      :persistent-hint="Boolean(hint)"
      :spellcheck="false"
      :type="revealed ? 'text' : 'password'"
      @update:model-value="updateValue"
    >
      <template #append-inner>
        <v-btn
          :aria-label="revealLabel"
          :aria-pressed="revealed"
          :disabled="interactionDisabled || modelValue.length === 0"
          :icon="revealed ? 'mdi-eye-off-outline' : 'mdi-eye-outline'"
          size="small"
          variant="text"
          @click="revealed = !revealed"
        />
      </template>
    </v-text-field>

    <v-checkbox
      v-if="allowClear"
      :disabled="disabled || pending"
      :label="clearLabel"
      :model-value="clearRequested"
      color="error"
      density="compact"
      hide-details
      @update:model-value="updateClearRequest"
    />
  </div>
</template>

<style scoped>
.secret-field {
  min-width: 0;
}

.secret-field__state {
  display: flex;
  align-items: center;
  gap: 8px;
}

@media (max-width: 599px) {
  .secret-field__state {
    align-items: flex-start;
    flex-direction: column;
  }
}
</style>
