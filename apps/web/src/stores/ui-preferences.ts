import { ref } from 'vue'
import { defineStore } from 'pinia'

export const THEME_PREFERENCE_KEY = 'nodecontroll.ui.theme'
export const LOCALE_PREFERENCE_KEY = 'nodecontroll.ui.locale'

export type ThemePreference = 'dark' | 'light' | 'system'
export type ShellLocale = 'en' | 'zh-CN'

const themePreferences = new Set<ThemePreference>(['dark', 'light', 'system'])
const shellLocales = new Set<ShellLocale>(['en', 'zh-CN'])

const readPreference = <T extends string>(key: string, allowed: ReadonlySet<T>, fallback: T) => {
  try {
    const stored = globalThis.localStorage?.getItem(key)
    return stored && allowed.has(stored as T) ? (stored as T) : fallback
  } catch {
    return fallback
  }
}

const persistPreference = (key: string, value: string) => {
  try {
    globalThis.localStorage?.setItem(key, value)
  } catch {
    // A blocked preference store must not prevent the control plane from rendering.
  }
}

export const useUiPreferencesStore = defineStore('ui-preferences', () => {
  const themePreference = ref<ThemePreference>(
    readPreference(THEME_PREFERENCE_KEY, themePreferences, 'system'),
  )
  const locale = ref<ShellLocale>(readPreference(LOCALE_PREFERENCE_KEY, shellLocales, 'zh-CN'))

  const setThemePreference = (nextPreference: ThemePreference) => {
    if (!themePreferences.has(nextPreference)) return
    themePreference.value = nextPreference
    persistPreference(THEME_PREFERENCE_KEY, nextPreference)
  }

  const setLocale = (nextLocale: ShellLocale) => {
    if (!shellLocales.has(nextLocale)) return
    locale.value = nextLocale
    persistPreference(LOCALE_PREFERENCE_KEY, nextLocale)
  }

  return {
    locale,
    setLocale,
    setThemePreference,
    themePreference,
  }
})
