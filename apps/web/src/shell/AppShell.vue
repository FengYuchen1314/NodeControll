<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { isNavigationFailure, useRoute, useRouter } from 'vue-router'
import { useDisplay } from 'vuetify'

import type { ShellRouteName } from '../router/route-names'
import { useSessionStore } from '../stores/session'
import { useUiPreferencesStore } from '../stores/ui-preferences'
import { navigationAt, projectNavigation } from './navigation'
import CommandPalette from './CommandPalette.vue'
import type { ShellNavigationItem } from './types'

defineProps<{
  logoutPending?: boolean
}>()

const emit = defineEmits<{
  logout: []
}>()

const route = useRoute()
const router = useRouter()
const session = useSessionStore()
const { t } = useI18n()
const { mobile } = useDisplay()
const preferences = useUiPreferencesStore()

const drawer = ref(true)
const rail = ref(false)
const commandOpen = ref(false)
const commandNavigationError = ref('')
const commandNavigationPending = ref(false)
const mainElement = ref<globalThis.HTMLElement>()
const routeAnnouncement = ref('')
const themeOptions = ['light', 'dark', 'system'] as const

const actorInitial = computed(() => session.actor?.username.trim().charAt(0).toUpperCase() || 'U')
const actorRole = computed(() => {
  const role = session.actor?.role
  if (!role) return t('shell.roles.unknown')
  const knownRoles = new Set(['owner', 'admin', 'operator', 'support', 'auditor', 'member'])
  return knownRoles.has(role) ? t(`shell.roles.${role}`) : t('shell.roles.unknown')
})
const pageTitle = computed(() => {
  const titleKey = route.meta.titleKey
  return typeof titleKey === 'string' ? t(titleKey) : (route.meta.title ?? t('app.name'))
})
const projectedNavigation = computed(() =>
  projectNavigation(router, {
    capabilities: session.actor?.capabilities ?? [],
    passwordChangeRequired: session.passwordChangeRequired,
  }),
)
const localize = (items: ReturnType<typeof projectNavigation>): readonly ShellNavigationItem[] =>
  items.map((item) => ({
    icon: item.icon,
    id: item.id,
    label: t(item.labelKey),
    routeName: item.routeName,
  }))
const drawerItems = computed(() => localize(navigationAt(projectedNavigation.value, 'drawer')))
const commandItems = computed(() => localize(navigationAt(projectedNavigation.value, 'command')))
const accountItems = computed(() => localize(navigationAt(projectedNavigation.value, 'account')))

watch(
  mobile,
  (isMobile) => {
    drawer.value = !isMobile
    if (isMobile) rail.value = false
  },
  { immediate: true },
)

const focusMain = async () => {
  await nextTick()
  mainElement.value?.focus({ preventScroll: true })
}

watch(
  [() => route.fullPath, pageTitle],
  () => {
    commandOpen.value = false
    commandNavigationError.value = ''
    commandNavigationPending.value = false
    if (mobile.value) drawer.value = false
    routeAnnouncement.value = t('shell.routeAnnouncement', { title: pageTitle.value })
    void focusMain()
  },
  { immediate: true },
)

const handleGlobalKeydown = (event: globalThis.KeyboardEvent) => {
  if ((event.ctrlKey || event.metaKey) && event.key.toLocaleLowerCase() === 'k') {
    event.preventDefault()
    commandNavigationError.value = ''
    commandOpen.value = true
  }
}

onMounted(() => globalThis.addEventListener('keydown', handleGlobalKeydown))
onBeforeUnmount(() => globalThis.removeEventListener('keydown', handleGlobalKeydown))

const navigate = async (routeName: ShellRouteName) => {
  if (commandNavigationPending.value) return
  commandNavigationError.value = ''
  if (route.name === routeName) {
    commandOpen.value = false
    if (mobile.value) drawer.value = false
    return
  }
  commandNavigationPending.value = true
  try {
    const result = await router.push({ name: routeName })
    if (isNavigationFailure(result)) {
      commandNavigationError.value = t('shell.navigationFailure')
      return
    }
    commandOpen.value = false
    if (mobile.value) drawer.value = false
  } catch {
    commandNavigationError.value = t('shell.navigationFailure')
  } finally {
    commandNavigationPending.value = false
  }
}
</script>

<template>
  <a class="skip-link" href="#app-main-content">{{ $t('shell.skipToContent') }}</a>
  <p class="sr-only" aria-live="polite" aria-atomic="true">{{ routeAnnouncement }}</p>

  <v-navigation-drawer
    v-model="drawer"
    :aria-label="$t('shell.primaryNavigation')"
    :permanent="!mobile"
    :rail="!mobile && rail"
    :temporary="mobile"
    width="264"
  >
    <div class="brand-lockup px-3 py-4">
      <span class="brand-mark" aria-hidden="true">N</span>
      <div v-if="!rail || mobile" class="ml-3 min-width-0">
        <div class="text-subtitle-1 font-weight-bold text-truncate">{{ $t('app.name') }}</div>
        <div class="text-caption text-medium-emphasis text-truncate">{{ $t('app.stage') }}</div>
      </div>
      <v-spacer />
      <v-btn
        v-if="!mobile"
        :aria-label="rail ? $t('shell.expandNavigation') : $t('shell.collapseNavigation')"
        :icon="rail ? 'mdi-chevron-right' : 'mdi-chevron-left'"
        size="small"
        variant="text"
        @click="rail = !rail"
      />
    </div>

    <v-divider />
    <v-list nav density="comfortable" class="px-2 py-4" :aria-label="$t('shell.primaryNavigation')">
      <v-list-item
        v-for="item in drawerItems"
        :key="item.id"
        :prepend-icon="item.icon"
        :title="item.label"
        :to="{ name: item.routeName }"
        :value="item.id"
        rounded="lg"
        @click="mobile && (drawer = false)"
      />
    </v-list>

    <template #append>
      <div v-if="!rail || mobile" class="pa-3">
        <v-card color="surface-variant" flat>
          <v-card-text class="pa-3 d-flex align-center ga-3">
            <v-avatar color="primary" size="36" aria-hidden="true">
              <span class="text-body-2 font-weight-bold">{{ actorInitial }}</span>
            </v-avatar>
            <div class="min-width-0">
              <div class="text-body-2 font-weight-medium text-truncate">
                {{ session.actor?.username }}
              </div>
              <div class="text-caption text-medium-emphasis text-truncate">
                {{ actorRole }}
              </div>
            </div>
          </v-card-text>
        </v-card>
      </div>
    </template>
  </v-navigation-drawer>

  <v-app-bar flat border="b" height="64">
    <v-app-bar-nav-icon
      v-if="mobile"
      :aria-label="$t('shell.openNavigation')"
      @click="drawer = true"
    />
    <v-app-bar-title class="text-subtitle-1 font-weight-semibold">{{ pageTitle }}</v-app-bar-title>
    <template #append>
      <v-btn
        :aria-label="$t('shell.openCommandPalette')"
        class="mr-1"
        prepend-icon="mdi-magnify"
        variant="text"
        @click="commandOpen = true"
      >
        <span class="d-none d-md-inline">{{ $t('shell.search') }}</span>
        <kbd class="command-shortcut d-none d-lg-inline">Ctrl K</kbd>
      </v-btn>

      <v-menu location="bottom end" :close-on-content-click="false">
        <template #activator="{ props: activatorProps }">
          <v-btn
            v-bind="activatorProps"
            :aria-label="$t('shell.accountMenu')"
            class="account-button"
            variant="text"
          >
            <v-avatar color="primary" size="32" class="mr-sm-2" aria-hidden="true">
              <span class="text-caption font-weight-bold">{{ actorInitial }}</span>
            </v-avatar>
            <span class="account-label text-body-2">{{ session.actor?.username }}</span>
            <v-icon icon="mdi-chevron-down" size="18" class="ml-1" />
          </v-btn>
        </template>

        <v-list min-width="260" density="comfortable">
          <v-list-item
            v-for="item in accountItems"
            :key="item.id"
            :prepend-icon="item.icon"
            :title="item.label"
            :to="{ name: item.routeName }"
          />
          <v-divider />
          <v-list-subheader>{{ $t('shell.theme') }}</v-list-subheader>
          <v-list-item
            v-for="option in themeOptions"
            :key="option"
            :active="preferences.themePreference === option"
            :prepend-icon="
              option === 'light'
                ? 'mdi-white-balance-sunny'
                : option === 'dark'
                  ? 'mdi-weather-night'
                  : 'mdi-theme-light-dark'
            "
            :title="$t(`shell.themeOptions.${option}`)"
            @click="preferences.setThemePreference(option)"
          />
          <v-divider />
          <v-list-subheader>{{ $t('shell.language') }}</v-list-subheader>
          <v-list-item
            :active="preferences.locale === 'zh-CN'"
            title="简体中文"
            @click="preferences.setLocale('zh-CN')"
          />
          <v-list-item
            :active="preferences.locale === 'en'"
            title="English"
            @click="preferences.setLocale('en')"
          />
          <v-divider />
          <v-list-item
            :disabled="logoutPending"
            prepend-icon="mdi-logout"
            :title="$t('shell.logout')"
            @click="emit('logout')"
          />
        </v-list>
      </v-menu>
    </template>
  </v-app-bar>

  <v-main>
    <main
      id="app-main-content"
      ref="mainElement"
      class="app-main"
      tabindex="-1"
      :aria-label="pageTitle"
    >
      <v-container class="page-container py-6 py-md-8">
        <slot />
      </v-container>
    </main>
  </v-main>

  <command-palette
    :model-value="commandOpen"
    :close-label="$t('shell.closeCommandPalette')"
    :empty-label="$t('shell.noCommandResults')"
    :items="commandItems"
    :label="$t('shell.commandPalette')"
    :navigation-error="commandNavigationError"
    :navigation-pending="commandNavigationPending"
    :placeholder="$t('shell.commandPlaceholder')"
    :results-label="$t('shell.commandResults')"
    @navigate="navigate"
    @update:model-value="
      (value) => {
        commandOpen = value
        if (!value) commandNavigationError = ''
      }
    "
  />
</template>

<style scoped>
.skip-link {
  position: fixed;
  z-index: 3000;
  top: 8px;
  left: 8px;
  padding: 10px 14px;
  border-radius: 8px;
  color: rgb(var(--v-theme-on-primary));
  background: rgb(var(--v-theme-primary));
  opacity: 0;
  pointer-events: none;
  transform: translateY(-150%);
}

.skip-link:focus {
  opacity: 1;
  pointer-events: auto;
  transform: translateY(0);
}

.app-main {
  min-height: calc(100vh - 64px);
  outline: none;
}

.app-main:focus-visible {
  box-shadow: inset 0 0 0 3px rgba(var(--v-theme-primary), 0.35);
}

.command-shortcut {
  padding: 2px 6px;
  margin-inline-start: 8px;
  border: 1px solid rgba(var(--v-theme-on-surface), 0.2);
  border-radius: 5px;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  font: inherit;
  font-size: 0.7rem;
}

@media (max-width: 599px) {
  .app-main .page-container {
    padding-inline: 16px !important;
  }
}
</style>
