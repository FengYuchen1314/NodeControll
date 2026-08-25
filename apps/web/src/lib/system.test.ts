import { describe, expect, it } from 'vitest'

import { formatStartedAt } from './system'

describe('formatStartedAt', () => {
  it('does not expose invalid date implementation text', () => {
    expect(formatStartedAt('not-a-date')).toBe('—')
  })

  it('formats a valid UTC instant', () => {
    expect(formatStartedAt('2026-08-25T14:00:00Z', 'en-US')).toContain('2026')
  })
})

