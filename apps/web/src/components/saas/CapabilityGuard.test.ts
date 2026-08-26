import { cleanup, fireEvent, render, screen } from '@testing-library/vue'
import { h } from 'vue'
import { afterEach, describe, expect, it, vi } from 'vitest'

import { vuetify } from '../../plugins/vuetify'
import CapabilityGuard from './CapabilityGuard.vue'

afterEach(() => cleanup())

describe('CapabilityGuard', () => {
  it('removes denied content in hide mode', () => {
    render(CapabilityGuard, {
      global: { plugins: [vuetify] },
      props: { allowed: false, label: 'Server actions', reason: 'Missing system:execute' },
      slots: { default: '<button type="button">Restart</button>' },
    })

    expect(screen.queryByRole('button', { name: 'Restart' })).toBeNull()
  })

  it('makes links and custom actions inert in disable mode while preserving the reason', async () => {
    const action = vi.fn()
    const { container } = render(CapabilityGuard, {
      global: { plugins: [vuetify] },
      props: {
        allowed: false,
        label: 'Server actions',
        mode: 'disable',
        reason: 'Missing system:execute',
      },
      slots: {
        default: () => h('a', { href: '/restart', onClick: action }, 'Restart'),
      },
    })

    const link = screen.getByText('Restart')
    expect(link.closest('[inert]')).not.toBeNull()
    await fireEvent.click(link)
    expect(action).not.toHaveBeenCalled()
    expect(container.textContent).toContain('Missing system:execute')
  })

  it('exposes an explicit explanation without rendering the denied action', () => {
    render(CapabilityGuard, {
      global: { plugins: [vuetify] },
      props: {
        allowed: false,
        label: 'Audit export',
        mode: 'explain',
        reason: 'Missing audit:read',
      },
      slots: { default: '<button type="button">Export</button>' },
    })

    expect(screen.queryByRole('button', { name: 'Export' })).toBeNull()
    expect(screen.getByText('Missing audit:read')).not.toBeNull()
  })
})
