import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/vue'
import { afterEach, describe, expect, it } from 'vitest'

import { vuetify } from '../plugins/vuetify'
import CommandPalette from './CommandPalette.vue'

const items = [
  { icon: 'mdi-view-dashboard-outline', id: 'dashboard', label: 'Dashboard', routeName: 'dashboard' },
  { icon: 'mdi-cog-outline', id: 'system', label: 'System', routeName: 'system' },
] as const

afterEach(() => cleanup())

describe('CommandPalette', () => {
  it('focuses search, supports arrows and emits only an authorized route', async () => {
    const result = render(CommandPalette, {
      global: { plugins: [vuetify] },
      props: {
        closeLabel: 'Close',
        emptyLabel: 'No result',
        items,
        label: 'Commands',
        modelValue: true,
        placeholder: 'Search pages',
        resultsLabel: 'Results',
      },
    })

    const search = await screen.findByRole('combobox', { name: 'Search pages' })
    await waitFor(() => expect(document.activeElement).toBe(search))
    expect(screen.getAllByRole('dialog')).toHaveLength(1)
    await fireEvent.keyDown(search, { key: 'ArrowDown' })
    await fireEvent.keyDown(search, { key: 'Enter' })
    expect(result.emitted().navigate).toEqual([['system']])
  })

  it('keeps a failed navigation recoverable and blocks duplicate submits while pending', async () => {
    const result = render(CommandPalette, {
      global: { plugins: [vuetify] },
      props: {
        closeLabel: 'Close',
        emptyLabel: 'No result',
        items,
        label: 'Commands',
        modelValue: true,
        navigationError: 'Navigation failed',
        navigationPending: true,
        placeholder: 'Search pages',
        resultsLabel: 'Results',
      },
    })

    expect((await screen.findByRole('alert')).textContent).toContain('Navigation failed')
    await fireEvent.click(screen.getByText('System'))
    expect(result.emitted().navigate).toBeUndefined()
    await result.rerender({ navigationPending: false })
    await fireEvent.click(screen.getByText('System'))
    expect(result.emitted().navigate).toEqual([['system']])
  })

  it('closes on Escape and reports an empty filtered result', async () => {
    const result = render(CommandPalette, {
      global: { plugins: [vuetify] },
      props: {
        closeLabel: 'Close',
        emptyLabel: 'No result',
        items,
        label: 'Commands',
        modelValue: true,
        placeholder: 'Search pages',
        resultsLabel: 'Results',
      },
    })
    const search = await screen.findByRole('combobox', { name: 'Search pages' })
    await fireEvent.update(search, 'missing')
    expect(await screen.findByText('No result')).not.toBeNull()
    await fireEvent.keyDown(search, { key: 'Escape' })
    expect(result.emitted()['update:modelValue']).toEqual([[false]])
  })
})
