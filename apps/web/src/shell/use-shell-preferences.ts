import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useTheme } from 'vuetify'

import { useUiPreferencesStore } from '../stores/ui-preferences'

const systemDarkQuery = '(prefers-color-scheme: dark)'

export function useShellPreferences() {
  const preferences = useUiPreferencesStore()
  const { locale } = useI18n()
  const theme = useTheme()
  let mediaQuery =
    typeof globalThis.matchMedia === 'function'
      ? globalThis.matchMedia(systemDarkQuery)
      : undefined
  const systemDark = ref(mediaQuery?.matches ?? false)
  const initialDocumentLanguage = globalThis.document?.documentElement.lang ?? ''
  const initialColorScheme = globalThis.document?.documentElement.style.colorScheme ?? ''

  const resolvedTheme = computed(() => {
    if (preferences.themePreference === 'system') {
      return systemDark.value ? 'nodecontrollDark' : 'nodecontrollLight'
    }
    return preferences.themePreference === 'dark' ? 'nodecontrollDark' : 'nodecontrollLight'
  })

  const updateSystemTheme = (event: MediaQueryListEvent | MediaQueryList) => {
    systemDark.value = event.matches
  }

  watch(
    resolvedTheme,
    (nextTheme) => {
      theme.global.name.value = nextTheme
      if (globalThis.document) {
        globalThis.document.documentElement.style.colorScheme =
          nextTheme === 'nodecontrollDark' ? 'dark' : 'light'
      }
    },
    { immediate: true },
  )

  watch(
    () => preferences.locale,
    (nextLocale) => {
      locale.value = nextLocale
      if (globalThis.document) globalThis.document.documentElement.lang = nextLocale
    },
    { immediate: true },
  )

  onMounted(() => {
    mediaQuery?.addEventListener('change', updateSystemTheme)
  })

  onBeforeUnmount(() => {
    mediaQuery?.removeEventListener('change', updateSystemTheme)
    mediaQuery = undefined
    if (globalThis.document) {
      globalThis.document.documentElement.lang = initialDocumentLanguage
      globalThis.document.documentElement.style.colorScheme = initialColorScheme
    }
  })

  return {
    locale: computed(() => preferences.locale),
    resolvedTheme,
    setLocale: preferences.setLocale,
    setThemePreference: preferences.setThemePreference,
    themePreference: computed(() => preferences.themePreference),
  }
}
