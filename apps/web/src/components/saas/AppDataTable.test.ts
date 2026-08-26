import { cleanup, fireEvent, render, screen } from '@testing-library/vue'
import { afterEach, describe, expect, it } from 'vitest'

import { vuetify } from '../../plugins/vuetify'
import AppDataTable from './AppDataTable.vue'
import type { AppDataTableLabels } from './types'

const labels: AppDataTableLabels = {
  actions: 'Actions',
  empty: 'No resources',
  emptyValue: 'Not set',
  falseValue: 'No',
  invalidConfiguration: 'Table configuration is invalid',
  loading: 'Loading resources',
  mobile: 'Resource cards',
  retry: 'Retry',
  selectAll: 'Select every row',
  selectRow: (key) => `Select ${key}`,
  stale: 'Showing last known data',
  trueValue: 'Yes',
}

afterEach(() => cleanup())

describe('AppDataTable', () => {
  it('renders loading, error, empty and stale states with caller-localized copy', async () => {
    const result = render(AppDataTable, {
      global: { plugins: [vuetify] },
      props: { columns: [{ key: 'name', label: 'Name' }], labels, loading: true, rowKey: (row) => String(row.id), rows: [], tableLabel: 'Servers' },
    })
    expect(screen.getByText('Loading resources')).not.toBeNull()

    await result.rerender({ loading: false, errorMessage: 'Network unavailable' })
    expect(screen.getByRole('alert').textContent).toContain('Network unavailable')
    await fireEvent.click(screen.getByRole('button', { name: 'Retry' }))
    expect(result.emitted().retry).toHaveLength(1)

    await result.rerender({ errorMessage: undefined })
    expect(screen.getByText('No resources')).not.toBeNull()

    await result.rerender({ rows: [{ id: 'a', name: 'Alpha' }], stale: true, staleAt: '10:00' })
    expect(screen.getByText(/Showing last known data/).textContent).toContain('10:00')
  })

  it('fails closed for duplicate column or row keys', async () => {
    const result = render(AppDataTable, {
      global: { plugins: [vuetify] },
      props: {
        columns: [{ key: 'name', label: 'Name' }, { key: 'name', label: 'Duplicate' }],
        labels,
        rowKey: (row) => String(row.id),
        rows: [{ id: 'same', name: 'Alpha' }, { id: 'same', name: 'Beta' }],
        tableLabel: 'Servers',
      },
    })

    expect(screen.getByRole('alert').textContent).toContain('Table configuration is invalid')
    expect(screen.queryByText('Alpha')).toBeNull()
    await result.rerender({ columns: [{ key: 'name', label: 'Name' }] })
    expect(screen.getByRole('alert').textContent).toContain('Table configuration is invalid')
  })

  it('drops stale selected keys and keeps both 360px representations in the semantic DOM', async () => {
    const originalWidth = globalThis.innerWidth
    Object.defineProperty(globalThis, 'innerWidth', { configurable: true, value: 360 })
    try {
      const result = render(AppDataTable, {
        global: { plugins: [vuetify] },
        props: {
          columns: [{ key: 'name', label: 'Name' }],
          labels,
          rowKey: (row) => String(row.id),
          rows: [{ id: 'a', name: 'Alpha' }],
          selectable: true,
          selectedKeys: ['deleted-row'],
          tableLabel: 'Servers',
        },
      })

      expect(screen.getByTestId('app-data-table-desktop')).not.toBeNull()
      expect(screen.getByTestId('app-data-table-mobile')).not.toBeNull()
      const selectors = screen.getAllByLabelText('Select a')
      await fireEvent.click(selectors[0]!)
      expect(result.emitted()['update:selectedKeys']).toEqual([[['a']]])
    } finally {
      Object.defineProperty(globalThis, 'innerWidth', { configurable: true, value: originalWidth })
    }
  })
})
