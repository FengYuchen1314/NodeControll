import { createPinia, setActivePinia } from 'pinia'
import { beforeEach, describe, expect, it, vi } from 'vitest'

import {
  LOCALE_PREFERENCE_KEY,
  THEME_PREFERENCE_KEY,
  useUiPreferencesStore,
} from './ui-preferences'

beforeEach(() => setActivePinia(createPinia()))

describe('UI preferences', () => {
  it('restores only allowlisted non-sensitive enum values', () => {
    localStorage.setItem(THEME_PREFERENCE_KEY, 'token=secret')
    localStorage.setItem(LOCALE_PREFERENCE_KEY, 'fr')

    const preferences = useUiPreferencesStore()

    expect(preferences.themePreference).toBe('system')
    expect(preferences.locale).toBe('zh-CN')
    preferences.setThemePreference('dark')
    preferences.setLocale('en')
    expect(localStorage.getItem(THEME_PREFERENCE_KEY)).toBe('dark')
    expect(localStorage.getItem(LOCALE_PREFERENCE_KEY)).toBe('en')
  })

  it('renders with safe defaults when storage reads or writes fail', () => {
    const getter = vi.spyOn(Storage.prototype, 'getItem').mockImplementation(() => {
      throw new DOMException('blocked')
    })
    const preferences = useUiPreferencesStore()
    expect(preferences.themePreference).toBe('system')
    expect(preferences.locale).toBe('zh-CN')
    getter.mockRestore()

    const setter = vi.spyOn(Storage.prototype, 'setItem').mockImplementation(() => {
      throw new DOMException('blocked')
    })
    expect(() => preferences.setThemePreference('light')).not.toThrow()
    expect(preferences.themePreference).toBe('light')
    setter.mockRestore()
  })
})
