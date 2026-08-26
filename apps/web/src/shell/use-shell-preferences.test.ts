import { cleanup, render, screen, waitFor } from '@testing-library/vue'
import { createPinia } from 'pinia'
import { defineComponent, h } from 'vue'
import { afterEach, describe, expect, it, vi } from 'vitest'

import { i18n } from '../plugins/i18n'
import { vuetify } from '../plugins/vuetify'
import { LOCALE_PREFERENCE_KEY, THEME_PREFERENCE_KEY } from '../stores/ui-preferences'
import { useShellPreferences } from './use-shell-preferences'

afterEach(() => {
  cleanup()
  i18n.global.locale.value = 'zh-CN'
  vuetify.theme.change('nodecontrollLight')
})

describe('useShellPreferences', () => {
  it('applies the initial system theme and locale, follows changes, and removes its listener', async () => {
    const listeners = new Set<(event: MediaQueryListEvent) => void>()
    const addEventListener = vi.fn((_type: string, listener: (event: MediaQueryListEvent) => void) => {
      listeners.add(listener)
    })
    const removeEventListener = vi.fn(
      (_type: string, listener: (event: MediaQueryListEvent) => void) => {
        listeners.delete(listener)
      },
    )
    const originalMatchMedia = globalThis.matchMedia
    const initialLanguage = document.documentElement.lang
    const initialColorScheme = document.documentElement.style.colorScheme
    Object.defineProperty(globalThis, 'matchMedia', {
      configurable: true,
      value: vi.fn(() => ({
        addEventListener,
        dispatchEvent: () => true,
        matches: true,
        media: '(prefers-color-scheme: dark)',
        onchange: null,
        removeEventListener,
      })),
    })
    localStorage.setItem(THEME_PREFERENCE_KEY, 'system')
    localStorage.setItem(LOCALE_PREFERENCE_KEY, 'en')

    const Harness = defineComponent({
      setup() {
        const shell = useShellPreferences()
        return () => h('span', shell.resolvedTheme.value)
      },
    })

    try {
      const result = render(Harness, {
        global: { plugins: [createPinia(), i18n, vuetify] },
      })
      expect(screen.getByText('nodecontrollDark')).not.toBeNull()
      expect(document.documentElement.lang).toBe('en')
      expect(document.documentElement.style.colorScheme).toBe('dark')
      expect(addEventListener).toHaveBeenCalledOnce()

      for (const listener of listeners) listener({ matches: false } as MediaQueryListEvent)
      await waitFor(() => expect(screen.getByText('nodecontrollLight')).not.toBeNull())
      expect(document.documentElement.style.colorScheme).toBe('light')

      result.unmount()
      expect(removeEventListener).toHaveBeenCalledOnce()
      expect(listeners.size).toBe(0)
      expect(document.documentElement.lang).toBe(initialLanguage)
      expect(document.documentElement.style.colorScheme).toBe(initialColorScheme)
    } finally {
      Object.defineProperty(globalThis, 'matchMedia', {
        configurable: true,
        value: originalMatchMedia,
      })
    }
  })
})
