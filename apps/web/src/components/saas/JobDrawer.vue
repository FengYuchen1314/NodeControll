<script setup lang="ts">
import { computed, useId } from 'vue'

import JobChip from './JobChip.vue'
import SafeDisplayValueView from './SafeDisplayValueView.vue'
import type { JobChipLabels, JobDrawerLabels, JobPresentation, JobStepPresentationState } from './types'

const props = defineProps<{
  chipLabels: JobChipLabels
  job?: JobPresentation
  labels: JobDrawerLabels
  modelValue: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>()

const titleId = useId()
const stepsTitleId = useId()
const clampedProgress = computed(() =>
  props.job?.progressPercent === undefined || !Number.isFinite(props.job.progressPercent)
    ? undefined
    : Math.min(100, Math.max(0, Math.round(props.job.progressPercent))),
)
const stepContract: Record<JobStepPresentationState, { icon: string }> = {
  failed: { icon: 'mdi-alert-circle-outline' },
  pending: { icon: 'mdi-clock-outline' },
  running: { icon: 'mdi-progress-clock' },
  skipped: { icon: 'mdi-debug-step-over' },
  succeeded: { icon: 'mdi-check-circle-outline' },
}
</script>

<template>
  <v-navigation-drawer
    :model-value="modelValue"
    location="right"
    temporary
    width="420"
    :aria-labelledby="titleId"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <div class="job-drawer__header pa-4">
      <div class="min-width-0">
        <p class="text-overline text-primary mb-1">{{ labels.overline }}</p>
        <h2 :id="titleId" class="text-h6">{{ labels.title }}</h2>
      </div>
      <v-btn
        :aria-label="labels.close"
        icon="mdi-close"
        variant="text"
        @click="emit('update:modelValue', false)"
      />
    </div>
    <v-divider />

    <div v-if="job" class="pa-4">
      <job-chip :interactive="false" :job="job" :labels="chipLabels" />
      <p v-if="job.message" class="text-body-2 mt-4 mb-0">
        <safe-display-value-view
          :empty-label="labels.emptyValue"
          :redacted-label="labels.redactedValue"
          :value="job.message"
        />
      </p>

      <v-progress-linear
        v-if="clampedProgress !== undefined"
        class="mt-4"
        color="primary"
        height="8"
        :model-value="clampedProgress"
        rounded
        :aria-label="labels.progress(clampedProgress)"
      />

      <dl class="job-drawer__facts mt-5">
        <dt>{{ labels.jobId }}</dt>
        <dd><code>{{ job.id }}</code></dd>
        <dt>{{ labels.source }}</dt>
        <dd>{{ job.source }}</dd>
        <template v-if="job.createdAt">
          <dt>{{ labels.createdAt }}</dt>
          <dd>{{ job.createdAt }}</dd>
        </template>
        <template v-if="job.updatedAt">
          <dt>{{ labels.updatedAt }}</dt>
          <dd>{{ job.updatedAt }}</dd>
        </template>
      </dl>

      <section v-if="job.steps?.length" class="mt-6" :aria-labelledby="stepsTitleId">
        <h3 :id="stepsTitleId" class="text-subtitle-1 mb-3">{{ labels.steps }}</h3>
        <ol class="job-drawer__steps">
          <li v-for="step in job.steps" :key="step.id">
            <v-icon :icon="stepContract[step.state].icon" aria-hidden="true" />
            <div>
              <strong>{{ step.label }}</strong>
              <span class="d-block text-caption text-medium-emphasis">
                {{ labels.stepStates[step.state] }}<template v-if="step.message">
                  · <safe-display-value-view
                    :empty-label="labels.emptyValue"
                    :redacted-label="labels.redactedValue"
                    :value="step.message"
                  />
                </template>
              </span>
            </div>
          </li>
        </ol>
      </section>
    </div>

    <v-empty-state v-else icon="mdi-briefcase-clock-outline" :text="labels.empty" />
  </v-navigation-drawer>
</template>

<style scoped>
.job-drawer__header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
}

.job-drawer__facts {
  display: grid;
  grid-template-columns: minmax(92px, 0.6fr) minmax(0, 1.4fr);
  gap: 10px 14px;
  margin-bottom: 0;
}

.job-drawer__facts dt {
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  font-size: 0.75rem;
  font-weight: 700;
}

.job-drawer__facts dd {
  min-width: 0;
  margin: 0;
  overflow-wrap: anywhere;
}

.job-drawer__steps {
  display: grid;
  gap: 12px;
  padding: 0;
  margin: 0;
  list-style: none;
}

.job-drawer__steps li {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr);
  align-items: start;
  gap: 10px;
}

@media (max-width: 599px) {
  .job-drawer__facts {
    grid-template-columns: 1fr;
    gap: 4px;
  }

  .job-drawer__facts dd {
    margin-block-end: 8px;
  }
}
</style>
