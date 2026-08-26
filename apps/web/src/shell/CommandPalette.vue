<script setup lang="ts">
import { computed, nextTick, ref, useId, watch } from 'vue'

import type { ShellRouteName } from '../router/route-names'
import type { ShellNavigationItem } from './types'

const props = defineProps<{
  closeLabel: string
  emptyLabel: string
  items: readonly ShellNavigationItem[]
  label: string
  modelValue: boolean
  navigationError?: string
  navigationPending?: boolean
  placeholder: string
  resultsLabel: string
}>()

const emit = defineEmits<{
  navigate: [routeName: ShellRouteName]
  'update:modelValue': [value: boolean]
}>()

const query = ref('')
const activeIndex = ref(0)
const paletteId = `command-palette-${useId().replace(/[^a-zA-Z0-9_-]/g, '')}`
const listboxId = `${paletteId}-results`

const filteredItems = computed(() => {
  const needle = query.value.trim().toLocaleLowerCase()
  if (!needle) return props.items
  return props.items.filter((item) => item.label.toLocaleLowerCase().includes(needle))
})

const activeItem = computed(() => filteredItems.value[activeIndex.value])
const activeDescendant = computed(() =>
  activeItem.value ? `${paletteId}-option-${activeItem.value.id}` : undefined,
)

const focusSearch = async () => {
  await nextTick()
  const search = globalThis.document.querySelector<HTMLInputElement>(
    `[data-command-palette="${paletteId}"] input`,
  )
  search?.focus()
}

watch(
  () => props.modelValue,
  (isOpen) => {
    if (!isOpen) return
    query.value = ''
    activeIndex.value = 0
    void focusSearch()
  },
)

watch(query, () => {
  activeIndex.value = 0
})

const close = () => emit('update:modelValue', false)

const move = (offset: number) => {
  const length = filteredItems.value.length
  if (length === 0) return
  activeIndex.value = (activeIndex.value + offset + length) % length
}

const select = (item: ShellNavigationItem | undefined = activeItem.value) => {
  if (!item || props.navigationPending) return
  emit('navigate', item.routeName)
}

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'ArrowDown') {
    event.preventDefault()
    move(1)
  } else if (event.key === 'ArrowUp') {
    event.preventDefault()
    move(-1)
  } else if (event.key === 'Enter') {
    event.preventDefault()
    select()
  } else if (event.key === 'Escape') {
    event.preventDefault()
    close()
  }
}
</script>

<template>
  <v-dialog
    :model-value="modelValue"
    max-width="620"
    :retain-focus="true"
    @update:model-value="emit('update:modelValue', $event)"
  >
    <v-card :data-command-palette="paletteId" class="command-palette">
      <v-card-title class="d-flex align-center ga-2 pa-4 pb-2">
        <v-icon icon="mdi-magnify" aria-hidden="true" />
        <span>{{ label }}</span>
        <v-spacer />
        <v-btn :aria-label="closeLabel" icon="mdi-close" size="small" variant="text" @click="close" />
      </v-card-title>
      <v-card-text class="pa-4 pt-2">
        <v-text-field
          v-model="query"
          :aria-activedescendant="activeDescendant"
          :aria-controls="listboxId"
          :aria-expanded="modelValue"
          :label="placeholder"
          autocomplete="off"
          hide-details
          prepend-inner-icon="mdi-magnify"
          role="combobox"
          :disabled="navigationPending"
          @keydown="handleKeydown"
        />

        <v-alert
          v-if="navigationError"
          class="mt-3"
          color="error"
          density="compact"
          role="alert"
          variant="tonal"
        >
          {{ navigationError }}
        </v-alert>

        <p class="sr-only" aria-live="polite">{{ resultsLabel }}: {{ filteredItems.length }}</p>
        <v-list
          v-if="filteredItems.length > 0"
          :id="listboxId"
          class="command-palette__results mt-3"
          density="comfortable"
          role="listbox"
        >
          <v-list-item
            v-for="(item, index) in filteredItems"
            :id="`${paletteId}-option-${item.id}`"
            :key="item.id"
            :active="index === activeIndex"
            :aria-selected="index === activeIndex"
            :prepend-icon="item.icon"
            :title="item.label"
            :disabled="navigationPending"
            role="option"
            rounded="lg"
            @click="select(item)"
            @mousemove="activeIndex = index"
          />
        </v-list>
        <v-empty-state v-else :text="emptyLabel" icon="mdi-text-search" size="small" />
      </v-card-text>
    </v-card>
  </v-dialog>
</template>

<style scoped>
.command-palette {
  overflow: hidden;
  border-radius: 12px;
}

.command-palette__results {
  max-height: min(420px, 55vh);
  overflow-y: auto;
}

@media (max-width: 599px) {
  .command-palette {
    margin: 12px;
  }
}
</style>
