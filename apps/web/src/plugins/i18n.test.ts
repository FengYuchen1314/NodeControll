import { describe, expect, it } from 'vitest'

import { i18n } from './i18n'

const flattenKeys = (value: unknown, prefix = ''): string[] => {
  if (!value || typeof value !== 'object' || Array.isArray(value)) return [prefix]
  return Object.entries(value as Record<string, unknown>).flatMap(([key, child]) =>
    flattenKeys(child, prefix ? `${prefix}.${key}` : key),
  )
}

describe('shell translations', () => {
  it('keeps the Chinese and English message structures identical', () => {
    const chinese = flattenKeys(i18n.global.getLocaleMessage('zh-CN')).sort()
    const english = flattenKeys(i18n.global.getLocaleMessage('en')).sort()

    expect(chinese).toEqual(english)
  })
})
