import { cleanup, render, screen } from '@testing-library/vue'
import { afterEach, describe, expect, it } from 'vitest'

import { vuetify } from '../../plugins/vuetify'
import DesiredReportedDiff from './DesiredReportedDiff.vue'
import type { DesiredReportedField, SafeDisplayValue } from './types'

afterEach(() => cleanup())

describe('DesiredReportedDiff', () => {
  it('renders semantic states and never reads extra payload from a redacted value', () => {
    const redactedWithUntrustedExtra: SafeDisplayValue & { readonly text: string } = {
      kind: 'redacted',
      label: '访问凭据已隐藏',
      text: 'DO-NOT-RENDER-THIS-SECRET',
    }
    const fields: DesiredReportedField[] = [
      {
        desired: { kind: 'text', text: 'enabled' },
        evidenceSource: 'Agent heartbeat',
        evidenceTime: '2026-08-26T09:00:00Z',
        id: 'service-state',
        label: '服务状态',
        lastGood: { kind: 'text', text: 'enabled' },
        reported: { kind: 'text', text: 'disabled' },
        state: 'drift',
      },
      {
        desired: redactedWithUntrustedExtra,
        evidenceSource: '配置编译器',
        id: 'credential',
        label: '访问凭据',
        reported: { kind: 'redacted' },
        state: 'unknown',
      },
    ]

    render(DesiredReportedDiff, {
      global: { plugins: [vuetify] },
      props: { fields },
      slots: { redactedRaw: '<pre>token: [REDACTED]</pre>' },
    })

    expect(screen.getByRole('heading', { level: 2, name: '期望状态与实际状态' })).not.toBeNull()
    expect(screen.getByRole('table', { name: '期望状态与实际状态' })).not.toBeNull()
    expect(screen.getByLabelText(/存在偏差；.*Agent heartbeat/)).not.toBeNull()
    expect(screen.getAllByText('访问凭据已隐藏')).toHaveLength(1)
    expect(document.body.textContent).not.toContain('DO-NOT-RENDER-THIS-SECRET')
    expect(screen.getByText('查看已脱敏原始差异')).not.toBeNull()
  })

  it('provides an explicit empty state', () => {
    render(DesiredReportedDiff, {
      global: { plugins: [vuetify] },
      props: { fields: [] },
    })

    expect(screen.getByText('暂无可比较的状态。')).not.toBeNull()
    expect(screen.queryByRole('table')).toBeNull()
  })
})
