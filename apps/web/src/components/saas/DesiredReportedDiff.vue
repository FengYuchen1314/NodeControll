<script setup lang="ts">
import { useId } from 'vue'

import type { DesiredReportedField, DesiredReportedState, StatusTone } from './types'
import SafeDisplayValueView from './SafeDisplayValueView.vue'
import StatusChip from './StatusChip.vue'

withDefaults(
  defineProps<{
    emptyLabel?: string
    fields: readonly DesiredReportedField[]
    rawDiffLabel?: string
    title?: string
  }>(),
  {
    emptyLabel: '暂无可比较的状态。',
    rawDiffLabel: '查看已脱敏原始差异',
    title: '期望状态与实际状态',
  },
)

defineSlots<{
  redactedRaw?: () => unknown
}>()

const stateContract: Record<DesiredReportedState, { label: string; tone: StatusTone }> = {
  drift: { label: '存在偏差', tone: 'warning' },
  match: { label: '一致', tone: 'success' },
  pending: { label: '等待生效', tone: 'info' },
  unknown: { label: '状态未知', tone: 'neutral' },
}

const titleId = useId()
</script>

<template>
  <section class="desired-reported-diff" :aria-labelledby="titleId">
    <div class="desired-reported-diff__heading">
      <div>
        <h2 :id="titleId" class="text-h6">{{ title }}</h2>
        <p class="text-body-2 text-medium-emphasis mb-0">
          分别展示控制面期望值、目标报告值与最近一次已知正常值。
        </p>
      </div>
    </div>

    <v-empty-state
      v-if="fields.length === 0"
      :text="emptyLabel"
      icon="mdi-compare-horizontal"
      title="没有状态差异"
    />

    <div v-else class="desired-reported-diff__table" role="table" :aria-label="title">
      <div class="desired-reported-diff__row desired-reported-diff__row--header" role="row">
        <span role="columnheader">字段</span>
        <span role="columnheader">状态</span>
        <span role="columnheader">期望值</span>
        <span role="columnheader">实际值</span>
        <span role="columnheader">最近正常值</span>
      </div>

      <div v-for="field in fields" :key="field.id" class="desired-reported-diff__row" role="row">
        <div class="desired-reported-diff__field" role="rowheader">
          <strong>{{ field.label }}</strong>
          <span v-if="field.explanation" class="text-caption text-medium-emphasis">
            {{ field.explanation }}
          </span>
        </div>
        <div class="desired-reported-diff__cell" role="cell">
          <span class="desired-reported-diff__mobile-label">状态</span>
          <status-chip
            :description="field.explanation"
            :label="stateContract[field.state].label"
            :observed-at="field.evidenceTime"
            :source="field.evidenceSource"
            :tone="stateContract[field.state].tone"
          />
        </div>
        <div class="desired-reported-diff__cell" role="cell">
          <span class="desired-reported-diff__mobile-label">期望值</span>
          <safe-display-value-view :value="field.desired" />
        </div>
        <div class="desired-reported-diff__cell" role="cell">
          <span class="desired-reported-diff__mobile-label">实际值</span>
          <safe-display-value-view :value="field.reported" />
        </div>
        <div class="desired-reported-diff__cell" role="cell">
          <span class="desired-reported-diff__mobile-label">最近正常值</span>
          <safe-display-value-view :value="field.lastGood ?? { kind: 'empty' }" />
        </div>
      </div>
    </div>

    <v-expansion-panels v-if="$slots.redactedRaw" class="mt-4" variant="accordion">
      <v-expansion-panel>
        <v-expansion-panel-title>{{ rawDiffLabel }}</v-expansion-panel-title>
        <v-expansion-panel-text>
          <v-alert class="mb-3" type="info" variant="tonal">
            原始差异插槽只允许传入已脱敏内容；本组件不接收原始配置对象。
          </v-alert>
          <slot name="redactedRaw" />
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>
  </section>
</template>

<style scoped>
.desired-reported-diff {
  min-width: 0;
}

.desired-reported-diff__heading {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 16px;
}

.desired-reported-diff__table {
  overflow: hidden;
  border: 1px solid rgb(var(--v-theme-outline));
  border-radius: 10px;
  background: rgb(var(--v-theme-surface));
}

.desired-reported-diff__row {
  display: grid;
  grid-template-columns: minmax(150px, 1.25fr) minmax(120px, 0.9fr) repeat(3, minmax(130px, 1fr));
  min-width: 760px;
  border-top: 1px solid rgb(var(--v-theme-outline));
}

.desired-reported-diff__row:first-child {
  border-top: 0;
}

.desired-reported-diff__row > * {
  min-width: 0;
  padding: 14px;
}

.desired-reported-diff__row--header {
  background: rgb(var(--v-theme-surface-variant));
  color: rgb(var(--v-theme-on-surface));
  font-size: 0.75rem;
  font-weight: 700;
}

.desired-reported-diff__field {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.desired-reported-diff__cell {
  display: flex;
  align-items: flex-start;
}

.desired-reported-diff__mobile-label {
  display: none;
}

@media (max-width: 599px) {
  .desired-reported-diff__table {
    display: grid;
    gap: 12px;
    overflow: visible;
    border: 0;
    background: transparent;
  }

  .desired-reported-diff__row {
    display: grid;
    grid-template-columns: 1fr;
    min-width: 0;
    overflow: hidden;
    border: 1px solid rgb(var(--v-theme-outline));
    border-radius: 10px;
    background: rgb(var(--v-theme-surface));
  }

  .desired-reported-diff__row--header {
    display: none;
  }

  .desired-reported-diff__row > * {
    padding: 12px;
  }

  .desired-reported-diff__cell {
    display: grid;
    grid-template-columns: minmax(92px, 0.75fr) minmax(0, 1.25fr);
    gap: 12px;
    border-top: 1px solid rgb(var(--v-theme-outline));
  }

  .desired-reported-diff__mobile-label {
    display: inline;
    color: rgb(var(--v-theme-on-surface));
    font-size: 0.75rem;
    font-weight: 700;
  }
}
</style>
