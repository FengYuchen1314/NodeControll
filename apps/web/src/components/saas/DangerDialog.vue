<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from 'vue'

const props = withDefaults(
  defineProps<{
    cancelLabel?: string
    confirmLabel?: string
    dependencies?: readonly string[]
    errorMessage?: string
    impactSummary: string
    modelValue: boolean
    objectName: string
    pending?: boolean
    reasonLabel?: string
    reasonRequired?: boolean
    title: string
  }>(),
  {
    cancelLabel: '取消',
    confirmLabel: '确认执行',
    dependencies: () => [],
    errorMessage: undefined,
    pending: false,
    reasonLabel: '操作原因',
    reasonRequired: false,
  },
)

const emit = defineEmits<{
  confirm: [payload: { reason?: string }]
  'update:modelValue': [value: boolean]
}>()

defineSlots<{
  dependencies?: () => unknown
  details?: () => unknown
  impact?: () => unknown
}>()

const typedObjectName = ref('')
const reason = ref('')
const submissionLocked = ref(false)

const confirmationMatches = computed(
  () => props.objectName.length > 0 && typedObjectName.value === props.objectName,
)
const reasonAccepted = computed(
  () => !props.reasonRequired || reason.value.trim().length > 0,
)
const canConfirm = computed(
  () => confirmationMatches.value && reasonAccepted.value && !props.pending && !submissionLocked.value,
)

const wipeLocalInputs = () => {
  typedObjectName.value = ''
  reason.value = ''
  submissionLocked.value = false
}

watch(
  () => props.modelValue,
  () => wipeLocalInputs(),
)

watch(
  () => props.pending,
  (pending, wasPending) => {
    if (wasPending && !pending) submissionLocked.value = false
  },
)

const requestClose = () => {
  if (props.pending || submissionLocked.value) return
  wipeLocalInputs()
  emit('update:modelValue', false)
}

const updateDialogModel = (nextValue: boolean) => {
  if (!nextValue) requestClose()
}

const confirm = () => {
  if (!canConfirm.value) return
  submissionLocked.value = true
  const trimmedReason = reason.value.trim()
  emit('confirm', trimmedReason ? { reason: trimmedReason } : {})
}

onBeforeUnmount(wipeLocalInputs)
</script>

<template>
  <v-dialog
    :model-value="modelValue"
    :persistent="pending || submissionLocked"
    max-width="620"
    role="alertdialog"
    @update:model-value="updateDialogModel"
  >
    <v-card border class="danger-dialog" data-testid="danger-dialog">
      <v-form aria-label="危险操作确认" @submit.prevent="confirm">
        <v-card-item prepend-icon="mdi-alert-octagon-outline">
          <v-card-title>{{ title }}</v-card-title>
          <v-card-subtitle>此操作需要逐字确认资源名称</v-card-subtitle>
        </v-card-item>

        <v-card-text>
          <v-alert class="mb-5" type="error" variant="tonal">
            <slot name="impact">
              {{ impactSummary }}
            </slot>
          </v-alert>

          <section v-if="dependencies.length > 0 || $slots.dependencies" class="mb-5">
            <h2 class="text-subtitle-2 mb-2">受影响依赖</h2>
            <slot name="dependencies">
              <ul class="danger-dialog__dependencies text-body-2">
                <li v-for="dependency in dependencies" :key="dependency">{{ dependency }}</li>
              </ul>
            </slot>
          </section>

          <slot name="details" />

          <p class="text-body-2 mb-2">
            输入 <strong class="danger-dialog__object-name">{{ objectName }}</strong> 以继续。
          </p>
          <v-text-field
            v-model="typedObjectName"
            :disabled="pending || submissionLocked"
            :error="typedObjectName.length > 0 && !confirmationMatches"
            :error-messages="
              typedObjectName.length > 0 && !confirmationMatches ? ['名称必须逐字匹配。'] : []
            "
            :label="`资源名称：${objectName}`"
            autocomplete="off"
            autofocus
            data-testid="danger-object-confirmation"
            spellcheck="false"
          />
          <v-textarea
            v-model="reason"
            :disabled="pending || submissionLocked"
            :label="reasonLabel"
            :required="reasonRequired"
            :rules="reasonRequired ? [(value: string) => value.trim().length > 0 || '必须填写原因。'] : []"
            auto-grow
            maxlength="500"
            rows="2"
          />

          <v-alert
            v-if="errorMessage"
            class="mt-2"
            data-testid="danger-dialog-error"
            role="alert"
            type="error"
            variant="tonal"
          >
            {{ errorMessage }}
          </v-alert>
          <p v-if="pending || submissionLocked" class="text-body-2 mt-3 mb-0" role="status">
            正在提交。确认完成前窗口保持锁定，操作不会重复发送。
          </p>
        </v-card-text>

        <v-divider />
        <v-card-actions class="danger-dialog__actions pa-4">
          <v-spacer />
          <v-btn :disabled="pending || submissionLocked" variant="text" @click="requestClose">
            {{ cancelLabel }}
          </v-btn>
          <v-btn
            :disabled="!canConfirm"
            :loading="pending || submissionLocked"
            color="error"
            type="submit"
            variant="flat"
          >
            {{ confirmLabel }}
          </v-btn>
        </v-card-actions>
      </v-form>
    </v-card>
  </v-dialog>
</template>

<style scoped>
.danger-dialog {
  border-radius: 12px;
}

.danger-dialog__dependencies {
  display: grid;
  gap: 6px;
  padding-inline-start: 22px;
  margin: 0;
}

.danger-dialog__object-name {
  font-family: ui-monospace, SFMono-Regular, Consolas, monospace;
  overflow-wrap: anywhere;
}

@media (max-width: 599px) {
  .danger-dialog__actions {
    display: grid;
    grid-template-columns: 1fr;
  }

  .danger-dialog__actions .v-spacer {
    display: none;
  }

  .danger-dialog__actions :deep(.v-btn) {
    width: 100%;
  }
}
</style>
