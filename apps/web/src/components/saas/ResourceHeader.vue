<script setup lang="ts">
import { RouterLink } from 'vue-router'

import type { ResourceBreadcrumb } from './types'

withDefaults(
  defineProps<{
    breadcrumbs?: readonly ResourceBreadcrumb[]
    description?: string
    overflowLabel?: string
    primaryActionDisabled?: boolean
    primaryActionIcon?: string
    primaryActionLabel?: string
    primaryActionPending?: boolean
    revision?: number | string
    title: string
  }>(),
  {
    breadcrumbs: () => [],
    description: undefined,
    overflowLabel: '更多操作',
    primaryActionDisabled: false,
    primaryActionIcon: undefined,
    primaryActionLabel: undefined,
    primaryActionPending: false,
    revision: undefined,
  },
)

const emit = defineEmits<{
  primaryAction: []
}>()

defineSlots<{
  actions?: () => unknown
  eyebrow?: () => unknown
  overflow?: () => unknown
  status?: () => unknown
}>()
</script>

<template>
  <header class="resource-header">
    <nav v-if="breadcrumbs.length > 0" class="resource-header__breadcrumbs" aria-label="面包屑">
      <ol>
        <li v-for="(breadcrumb, index) in breadcrumbs" :key="`${index}:${breadcrumb.label}`">
          <RouterLink v-if="breadcrumb.to" :to="breadcrumb.to">{{ breadcrumb.label }}</RouterLink>
          <span v-else :aria-current="index === breadcrumbs.length - 1 ? 'page' : undefined">
            {{ breadcrumb.label }}
          </span>
          <v-icon
            v-if="index < breadcrumbs.length - 1"
            aria-hidden="true"
            icon="mdi-chevron-right"
            size="16"
          />
        </li>
      </ol>
    </nav>

    <div class="resource-header__layout">
      <div class="resource-header__identity min-width-0">
        <div v-if="$slots.eyebrow" class="resource-header__eyebrow text-overline text-primary">
          <slot name="eyebrow" />
        </div>
        <h1 class="resource-header__title text-h4 font-weight-bold">{{ title }}</h1>
        <p v-if="description" class="resource-header__description text-body-2 text-medium-emphasis">
          {{ description }}
        </p>
        <div v-if="$slots.status || revision !== undefined" class="resource-header__meta">
          <slot name="status" />
          <span v-if="revision !== undefined" class="resource-header__revision text-caption">
            Revision {{ revision }}
          </span>
        </div>
      </div>

      <div
        v-if="$slots.actions || primaryActionLabel || $slots.overflow"
        class="resource-header__actions"
        aria-label="资源操作"
      >
        <div v-if="$slots.actions" class="resource-header__secondary-actions">
          <slot name="actions" />
        </div>
        <v-btn
          v-if="primaryActionLabel"
          :disabled="primaryActionDisabled || primaryActionPending"
          :loading="primaryActionPending"
          :prepend-icon="primaryActionIcon"
          class="resource-header__primary-action"
          color="primary"
          variant="flat"
          @click="emit('primaryAction')"
        >
          {{ primaryActionLabel }}
        </v-btn>
        <v-menu v-if="$slots.overflow" location="bottom end">
          <template #activator="{ props: activatorProps }">
            <v-btn
              v-bind="activatorProps"
              :aria-label="overflowLabel"
              class="resource-header__overflow-action"
              icon="mdi-dots-horizontal"
              variant="outlined"
            />
          </template>
          <v-list min-width="220">
            <slot name="overflow" />
          </v-list>
        </v-menu>
      </div>
    </div>
  </header>
</template>

<style scoped>
.resource-header {
  display: grid;
  gap: 16px;
  margin-block-end: 28px;
}

.resource-header__breadcrumbs ol {
  display: flex;
  min-width: 0;
  align-items: center;
  gap: 4px;
  padding: 0;
  margin: 0;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  font-size: 0.8125rem;
  list-style: none;
}

.resource-header__breadcrumbs li {
  display: inline-flex;
  min-width: 0;
  align-items: center;
  gap: 4px;
}

.resource-header__breadcrumbs a {
  overflow: hidden;
  border-radius: 4px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.resource-header__breadcrumbs a:focus-visible {
  outline: 3px solid rgba(var(--v-theme-primary), 0.35);
  outline-offset: 2px;
}

.resource-header__layout {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 24px;
}

.resource-header__identity {
  max-width: 760px;
}

.resource-header__eyebrow {
  margin-block-end: 4px;
}

.resource-header__title,
.resource-header__description {
  margin: 0;
}

.resource-header__title {
  overflow-wrap: anywhere;
}

.resource-header__description {
  margin-block-start: 8px;
}

.resource-header__meta,
.resource-header__actions,
.resource-header__secondary-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.resource-header__meta {
  flex-wrap: wrap;
  margin-block-start: 12px;
}

.resource-header__revision {
  padding: 3px 8px;
  border: 1px solid rgba(var(--v-theme-on-surface), 0.14);
  border-radius: 999px;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  font-family: ui-monospace, SFMono-Regular, Consolas, monospace;
}

.resource-header__actions {
  flex: 0 0 auto;
  flex-wrap: wrap;
  justify-content: flex-end;
}

@media (max-width: 599px) {
  .resource-header {
    gap: 12px;
    margin-block-end: 24px;
  }

  .resource-header__breadcrumbs ol {
    overflow-x: auto;
    padding-block-end: 4px;
  }

  .resource-header__layout {
    flex-direction: column;
    gap: 20px;
  }

  .resource-header__title {
    font-size: 1.75rem !important;
    line-height: 1.2;
  }

  .resource-header__actions {
    display: grid;
    width: 100%;
    grid-template-columns: minmax(0, 1fr) auto;
  }

  .resource-header__secondary-actions {
    grid-column: 1 / -1;
    flex-wrap: wrap;
  }

  .resource-header__secondary-actions :deep(.v-btn),
  .resource-header__primary-action {
    min-width: 0;
  }

  .resource-header__primary-action {
    width: 100%;
  }
}
</style>
