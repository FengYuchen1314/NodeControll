<script setup lang="ts">
import { computed, useId } from 'vue'

import type { PolicyContributor, PolicyContributorState, SafeDisplayValue, StatusTone } from './types'
import SafeDisplayValueView from './SafeDisplayValueView.vue'
import StatusChip from './StatusChip.vue'

const props = withDefaults(
  defineProps<{
    contributors: readonly PolicyContributor[]
    effective: SafeDisplayValue
    effectiveLabel?: string
    emptyLabel?: string
    title?: string
  }>(),
  {
    effectiveLabel: '当前生效值',
    emptyLabel: '暂无策略来源。',
    title: '策略计算说明',
  },
)

defineSlots<{
  contributor?: (props: { contributor: PolicyContributor; index: number }) => unknown
  effective?: (props: { value: SafeDisplayValue }) => unknown
}>()

const stateContract: Record<PolicyContributorState, { label: string; tone: StatusTone }> = {
  applied: { label: '已采用', tone: 'success' },
  excluded: { label: '未命中', tone: 'neutral' },
  overridden: { label: '被覆盖', tone: 'warning' },
}

const orderedContributors = computed(() =>
  [...props.contributors].sort((left, right) => right.priority - left.priority),
)
const titleId = useId()
</script>

<template>
  <section class="policy-explainer" :aria-labelledby="titleId">
    <h2 :id="titleId" class="text-h6 mb-3">{{ title }}</h2>

    <v-card border class="policy-explainer__effective mb-4" variant="flat">
      <v-card-text>
        <span class="text-caption text-medium-emphasis">{{ effectiveLabel }}</span>
        <div class="text-h6 mt-1">
          <slot name="effective" :value="effective">
            <safe-display-value-view :value="effective" />
          </slot>
        </div>
      </v-card-text>
    </v-card>

    <v-empty-state
      v-if="orderedContributors.length === 0"
      :text="emptyLabel"
      icon="mdi-shield-search-outline"
      title="没有策略来源"
    />

    <ol v-else class="policy-explainer__list" aria-label="策略来源（按优先级从高到低）">
      <li
        v-for="(contributor, index) in orderedContributors"
        :key="contributor.id"
        class="policy-explainer__item"
      >
        <slot name="contributor" :contributor="contributor" :index="index">
          <div class="policy-explainer__identity">
            <strong>{{ contributor.label }}</strong>
            <span v-if="contributor.scope" class="text-caption text-medium-emphasis">
              范围：{{ contributor.scope }}
            </span>
          </div>

          <div class="policy-explainer__priority">
            <span class="text-caption text-medium-emphasis">优先级</span>
            <strong>{{ contributor.priority }}</strong>
          </div>

          <div class="policy-explainer__value">
            <span class="text-caption text-medium-emphasis">候选值</span>
            <safe-display-value-view :value="contributor.value" />
          </div>

          <status-chip
            :description="contributor.timeRange ? `时间范围：${contributor.timeRange}` : undefined"
            :label="stateContract[contributor.state].label"
            :observed-at="contributor.timeRange"
            :source="contributor.scope ?? contributor.label"
            :tone="stateContract[contributor.state].tone"
          />
        </slot>
      </li>
    </ol>
  </section>
</template>

<style scoped>
.policy-explainer {
  min-width: 0;
}

.policy-explainer__effective {
  border-inline-start: 4px solid rgb(var(--v-theme-primary));
}

.policy-explainer__list {
  display: grid;
  gap: 10px;
  padding: 0;
  margin: 0;
  list-style: none;
}

.policy-explainer__item {
  display: grid;
  grid-template-columns: minmax(150px, 1.4fr) minmax(72px, 0.5fr) minmax(130px, 1fr) auto;
  align-items: center;
  gap: 16px;
  padding: 14px;
  border: 1px solid rgb(var(--v-theme-outline));
  border-radius: 10px;
  background: rgb(var(--v-theme-surface));
}

.policy-explainer__identity,
.policy-explainer__priority,
.policy-explainer__value {
  display: flex;
  min-width: 0;
  flex-direction: column;
  gap: 3px;
}

@media (max-width: 599px) {
  .policy-explainer__item {
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: start;
    gap: 12px;
  }

  .policy-explainer__identity,
  .policy-explainer__value {
    grid-column: 1 / -1;
  }

  .policy-explainer__priority {
    grid-column: 1;
  }
}
</style>
