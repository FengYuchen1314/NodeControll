import { cleanup, fireEvent, render, screen } from '@testing-library/vue'
import { afterEach, describe, expect, it } from 'vitest'

import { vuetify } from '../../plugins/vuetify'
import StatusChip from './StatusChip.vue'

afterEach(() => cleanup())

describe('StatusChip', () => {
  it('always exposes icon, text and evidence to keyboard and assistive technology', async () => {
    render(StatusChip, {
      global: { plugins: [vuetify] },
      props: {
        description: '心跳仍在允许窗口内',
        label: '在线',
        observedAt: '刚刚',
        source: 'Agent heartbeat',
        tone: 'success',
      },
    })

    const chip = screen.getByLabelText(
      '在线；心跳仍在允许窗口内 · 来源：Agent heartbeat · 时间：刚刚',
    )
    expect(chip.textContent).toContain('在线')
    expect(chip.querySelector('.mdi-check-circle-outline')).not.toBeNull()

    await fireEvent.focus(chip)
    expect(
      await screen.findByText('心跳仍在允许窗口内 · 来源：Agent heartbeat · 时间：刚刚'),
    ).not.toBeNull()
  })

  it('registers both light and dark SaaS themes', () => {
    expect(vuetify.theme.themes.value.nodecontrollLight?.dark).toBe(false)
    expect(vuetify.theme.themes.value.nodecontrollDark?.dark).toBe(true)
  })
})
