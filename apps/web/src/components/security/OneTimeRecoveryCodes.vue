<script setup lang="ts">
import { computed, ref } from 'vue'

const props = defineProps<{
  codes: readonly string[]
  confirmationReady?: boolean
  context: 'bootstrap' | 'regenerated'
}>()

const emit = defineEmits<{
  confirmed: []
  downloadFailed: []
}>()

const savedConfirmation = ref(false)
const downloadStarted = ref(false)

const heading = computed(() =>
  props.context === 'bootstrap' ? '保存 Owner 恢复码' : '保存新的恢复码',
)

const explanation = computed(() =>
  props.context === 'bootstrap'
    ? '这些恢复码随首次初始化生成，每枚只能使用一次。离开此页后无法再次读取。'
    : '新恢复码已经生效，旧恢复码同时失效。离开此窗口后无法再次读取这一组明文。',
)

const download = () => {
  downloadStarted.value = false
  let objectUrl: string | undefined
  let anchor: HTMLAnchorElement | undefined
  try {
    const contents = ['NodeControll recovery codes', '', ...props.codes, ''].join('\n')
    const blob = new Blob([contents], { type: 'text/plain;charset=utf-8' })
    objectUrl = globalThis.URL.createObjectURL(blob)
    anchor = globalThis.document.createElement('a')
    anchor.href = objectUrl
    anchor.download = 'nodecontroll-recovery-codes.txt'
    anchor.rel = 'noopener'
    anchor.hidden = true
    globalThis.document.body.append(anchor)
    anchor.click()
    downloadStarted.value = true
  } catch {
    emit('downloadFailed')
  } finally {
    anchor?.remove()
    if (objectUrl) {
      const urlToRevoke = objectUrl
      globalThis.queueMicrotask(() => globalThis.URL.revokeObjectURL(urlToRevoke))
    }
  }
}

const confirm = () => {
  if (!savedConfirmation.value) return
  emit('confirmed')
}
</script>

<template>
  <v-card border flat data-testid="one-time-recovery-codes">
    <v-card-item prepend-icon="mdi-key-chain-variant">
      <v-card-title>{{ heading }}</v-card-title>
      <v-card-subtitle>只显示这一次</v-card-subtitle>
    </v-card-item>
    <v-card-text>
      <v-alert type="warning" variant="tonal" class="mb-5">
        {{ explanation }} 建议下载后存入离线密码管理器或其他受保护位置。
      </v-alert>

      <ol class="recovery-code-grid mb-5" aria-label="一次性恢复码">
        <li v-for="code in codes" :key="code">
          <code data-testid="recovery-code">{{ code }}</code>
        </li>
      </ol>

      <v-btn
        prepend-icon="mdi-download-lock-outline"
        color="primary"
        variant="tonal"
        data-testid="download-recovery-codes"
        @click="download"
      >
        下载恢复码
      </v-btn>
      <p v-if="downloadStarted" class="text-caption text-medium-emphasis mt-2" role="status">
        下载已开始。请确认文件已保存到受保护位置。
      </p>

      <v-checkbox
        v-model="savedConfirmation"
        class="mt-4"
        color="primary"
        hide-details
        label="我已把这组恢复码保存到安全位置"
      />
      <p
        v-if="savedConfirmation && confirmationReady === false"
        class="text-caption text-medium-emphasis mt-2 mb-0"
        role="status"
      >
        正在确认控制面的最终状态，确认完成后才能离开此页。
      </p>
    </v-card-text>
    <v-divider />
    <v-card-actions class="pa-4 justify-end">
      <v-btn
        color="primary"
        variant="flat"
        :disabled="!savedConfirmation || confirmationReady === false"
        data-testid="confirm-recovery-codes"
        @click="confirm"
      >
        已保存，继续
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<style scoped>
.recovery-code-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 10px 28px;
  padding: 18px 18px 18px 46px;
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  border-radius: 12px;
  background: rgb(var(--v-theme-surface));
}

.recovery-code-grid code {
  font-size: 0.9rem;
  overflow-wrap: anywhere;
  user-select: all;
}

@media (max-width: 599px) {
  .recovery-code-grid {
    grid-template-columns: 1fr;
  }
}
</style>
