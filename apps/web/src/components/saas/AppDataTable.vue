<script setup lang="ts">
import { computed, watch } from 'vue'

import type { AppDataTableColumn, AppDataTableLabels, AppDataTableRow } from './types'

const props = withDefaults(
  defineProps<{
    columns: readonly AppDataTableColumn[]
    errorMessage?: string
    labels: AppDataTableLabels
    loading?: boolean
    rowKey: (row: AppDataTableRow, index: number) => string
    rows: readonly AppDataTableRow[]
    selectable?: boolean
    selectedKeys?: readonly string[]
    stale?: boolean
    staleAt?: string
    tableLabel: string
  }>(),
  {
    errorMessage: undefined,
    loading: false,
    selectable: false,
    selectedKeys: () => [],
    stale: false,
    staleAt: undefined,
  },
)

const emit = defineEmits<{
  retry: []
  'update:selectedKeys': [keys: readonly string[]]
}>()

defineSlots<{
  actions?: (props: { row: AppDataTableRow; rowKey: string }) => unknown
  cell?: (props: {
    column: AppDataTableColumn
    row: AppDataTableRow
    rowKey: string
    value: unknown
  }) => unknown
  empty?: () => unknown
  error?: (props: { message: string }) => unknown
  loading?: () => unknown
  mobile?: (props: { row: AppDataTableRow; rowKey: string }) => unknown
  toolbar?: () => unknown
}>()

const keyedRows = computed(() =>
  props.rows.map((row, index) => ({ key: props.rowKey(row, index), row })),
)
const validRowKeys = computed(() => new Set(keyedRows.value.map((candidate) => candidate.key)))
const configurationValid = computed(() => {
  const columnKeys = props.columns.map((column) => column.key)
  const rowKeys = keyedRows.value.map((candidate) => candidate.key)
  return (
    columnKeys.every((key) => key.length > 0) &&
    new Set(columnKeys).size === columnKeys.length &&
    rowKeys.every((key) => key.length > 0) &&
    new Set(rowKeys).size === rowKeys.length
  )
})
const sanitizedSelectedKeys = computed(() =>
  configurationValid.value
    ? [...new Set(props.selectedKeys.filter((key) => validRowKeys.value.has(key)))]
    : [],
)
const selected = computed(() => new Set(sanitizedSelectedKeys.value))

watch(
  [() => props.selectedKeys, sanitizedSelectedKeys],
  ([input, sanitized]) => {
    if (
      input.length !== sanitized.length ||
      input.some((key, index) => key !== sanitized[index])
    ) {
      emit('update:selectedKeys', sanitized)
    }
  },
  { immediate: true },
)
const allSelected = computed(
  () =>
    keyedRows.value.length > 0 && keyedRows.value.every((candidate) => selected.value.has(candidate.key)),
)
const someSelected = computed(
  () => !allSelected.value && keyedRows.value.some((candidate) => selected.value.has(candidate.key)),
)

const toggleRow = (key: string, nextSelected: boolean | null) => {
  const next = new Set(selected.value)
  if (nextSelected === true) next.add(key)
  else next.delete(key)
  emit('update:selectedKeys', [...next])
}

const toggleAll = (nextSelected: boolean | null) => {
  if (nextSelected === true) emit('update:selectedKeys', keyedRows.value.map((candidate) => candidate.key))
  else emit('update:selectedKeys', [])
}

const displayValue = (value: unknown) => {
  if (value === null || value === undefined || value === '') return props.labels.emptyValue
  if (typeof value === 'boolean') return value ? props.labels.trueValue : props.labels.falseValue
  if (typeof value === 'string' || typeof value === 'number') return String(value)
  return props.labels.emptyValue
}
</script>

<template>
  <section class="app-data-table" :aria-label="tableLabel">
    <div v-if="$slots.toolbar" class="app-data-table__toolbar mb-3">
      <slot name="toolbar" />
    </div>

    <v-alert v-if="stale && !loading && !errorMessage" class="mb-3" type="warning" variant="tonal">
      {{ labels.stale }}<span v-if="staleAt"> {{ staleAt }}</span>
    </v-alert>

    <div v-if="loading" class="app-data-table__state" role="status" aria-live="polite">
      <slot name="loading">
        <v-progress-circular color="primary" indeterminate size="32" />
        <span>{{ labels.loading }}</span>
      </slot>
    </div>

    <div v-else-if="errorMessage" class="app-data-table__state">
      <slot name="error" :message="errorMessage">
        <v-alert class="w-100" role="alert" type="error" variant="tonal">
          <div class="d-flex flex-wrap align-center ga-3">
            <span>{{ errorMessage }}</span>
            <v-spacer />
            <v-btn size="small" variant="text" @click="emit('retry')">{{ labels.retry }}</v-btn>
          </div>
        </v-alert>
      </slot>
    </div>

    <div v-else-if="!configurationValid" class="app-data-table__state">
      <v-alert class="w-100" role="alert" type="error" variant="tonal">
        {{ labels.invalidConfiguration }}
      </v-alert>
    </div>

    <div v-else-if="keyedRows.length === 0" class="app-data-table__state">
      <slot name="empty">
        <v-empty-state :text="labels.empty" icon="mdi-database-off-outline" />
      </slot>
    </div>

    <template v-else>
      <div class="app-data-table__desktop" data-testid="app-data-table-desktop">
        <table>
          <caption class="sr-only">{{ tableLabel }}</caption>
          <thead>
            <tr>
              <th v-if="selectable" class="app-data-table__selection" scope="col">
                <v-checkbox-btn
                  :aria-label="labels.selectAll"
                  :indeterminate="someSelected"
                  :model-value="allSelected"
                  @update:model-value="toggleAll"
                />
              </th>
              <th
                v-for="column in columns"
                :key="column.key"
                :class="`text-${column.align ?? 'start'}`"
                scope="col"
              >
                {{ column.label }}
              </th>
              <th v-if="$slots.actions" class="text-end" scope="col">{{ labels.actions }}</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="candidate in keyedRows" :key="candidate.key">
              <td v-if="selectable" class="app-data-table__selection">
                <v-checkbox-btn
                  :aria-label="labels.selectRow(candidate.key)"
                  :model-value="selected.has(candidate.key)"
                  @update:model-value="toggleRow(candidate.key, $event)"
                />
              </td>
              <td
                v-for="column in columns"
                :key="column.key"
                :class="`text-${column.align ?? 'start'}`"
              >
                <slot
                  name="cell"
                  :column="column"
                  :row="candidate.row"
                  :row-key="candidate.key"
                  :value="candidate.row[column.key]"
                >
                  {{ displayValue(candidate.row[column.key]) }}
                </slot>
              </td>
              <td v-if="$slots.actions" class="text-end">
                <slot name="actions" :row="candidate.row" :row-key="candidate.key" />
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <ul
        class="app-data-table__mobile"
        data-testid="app-data-table-mobile"
        :aria-label="labels.mobile"
      >
        <li v-for="candidate in keyedRows" :key="candidate.key" class="app-data-table__card">
          <slot name="mobile" :row="candidate.row" :row-key="candidate.key">
            <div v-if="selectable" class="app-data-table__mobile-selection">
              <v-checkbox-btn
                :aria-label="labels.selectRow(candidate.key)"
                :model-value="selected.has(candidate.key)"
                @update:model-value="toggleRow(candidate.key, $event)"
              />
            </div>
            <dl>
              <template v-for="column in columns" :key="column.key">
                <dt>{{ column.mobileLabel ?? column.label }}</dt>
                <dd>
                  <slot
                    name="cell"
                    :column="column"
                    :row="candidate.row"
                    :row-key="candidate.key"
                    :value="candidate.row[column.key]"
                  >
                    {{ displayValue(candidate.row[column.key]) }}
                  </slot>
                </dd>
              </template>
            </dl>
            <div v-if="$slots.actions" class="app-data-table__card-actions">
              <slot name="actions" :row="candidate.row" :row-key="candidate.key" />
            </div>
          </slot>
        </li>
      </ul>
    </template>
  </section>
</template>

<style scoped>
.app-data-table {
  min-width: 0;
}

.app-data-table__toolbar {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.app-data-table__state {
  display: flex;
  min-height: 180px;
  align-items: center;
  justify-content: center;
  gap: 12px;
}

.app-data-table__desktop {
  overflow-x: auto;
  border: 1px solid rgb(var(--v-theme-outline));
  border-radius: 10px;
}

.app-data-table table {
  width: 100%;
  min-width: 640px;
  border-collapse: collapse;
  background: rgb(var(--v-theme-surface));
}

.app-data-table th,
.app-data-table td {
  padding: 12px 14px;
  border-bottom: 1px solid rgb(var(--v-theme-outline));
  vertical-align: middle;
}

.app-data-table th {
  background: rgb(var(--v-theme-surface-variant));
  font-size: 0.75rem;
  font-weight: 700;
}

.app-data-table tbody tr:last-child td {
  border-bottom: 0;
}

.app-data-table__selection {
  width: 52px;
}

.app-data-table__mobile {
  display: none;
  padding: 0;
  margin: 0;
  list-style: none;
}

.app-data-table__card {
  position: relative;
  padding: 14px;
  border: 1px solid rgb(var(--v-theme-outline));
  border-radius: 10px;
  background: rgb(var(--v-theme-surface));
}

.app-data-table__card dl {
  display: grid;
  grid-template-columns: minmax(92px, 0.75fr) minmax(0, 1.25fr);
  gap: 10px 14px;
  margin: 0;
}

.app-data-table__card dt {
  font-size: 0.75rem;
  font-weight: 700;
}

.app-data-table__card dd {
  min-width: 0;
  margin: 0;
  overflow-wrap: anywhere;
}

.app-data-table__mobile-selection {
  display: flex;
  justify-content: flex-end;
  margin-block-end: 6px;
}

.app-data-table__card-actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: flex-end;
  gap: 8px;
  margin-block-start: 14px;
}

@media (max-width: 599px) {
  .app-data-table__desktop {
    display: none;
  }

  .app-data-table__mobile {
    display: grid;
    gap: 12px;
  }
}
</style>
