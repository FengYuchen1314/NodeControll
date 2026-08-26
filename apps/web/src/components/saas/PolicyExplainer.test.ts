import { cleanup, render, screen, within } from '@testing-library/vue'
import { afterEach, describe, expect, it } from 'vitest'

import { vuetify } from '../../plugins/vuetify'
import PolicyExplainer from './PolicyExplainer.vue'
import type { PolicyContributor, SafeDisplayValue } from './types'

afterEach(() => cleanup())

describe('PolicyExplainer', () => {
  it('orders sources by priority and exposes effective policy evidence', () => {
    const redactedWithUntrustedExtra: SafeDisplayValue & { readonly text: string } = {
      kind: 'redacted',
      text: 'DO-NOT-RENDER-POLICY-SECRET',
    }
    const contributors: PolicyContributor[] = [
      {
        id: 'user',
        label: '用户策略',
        priority: 20,
        scope: 'user:alice',
        state: 'overridden',
        value: { kind: 'text', text: '20 Mbps' },
      },
      {
        id: 'global',
        label: '全局策略',
        priority: 100,
        scope: 'global',
        state: 'applied',
        timeRange: '全天',
        value: redactedWithUntrustedExtra,
      },
    ]

    render(PolicyExplainer, {
      global: { plugins: [vuetify] },
      props: {
        contributors,
        effective: { kind: 'text', text: '10 Mbps' },
      },
    })

    expect(screen.getByRole('heading', { level: 2, name: '策略计算说明' })).not.toBeNull()
    expect(screen.getByText('10 Mbps')).not.toBeNull()
    const list = screen.getByRole('list', { name: '策略来源（按优先级从高到低）' })
    const items = within(list).getAllByRole('listitem')
    expect(items[0]?.textContent).toContain('全局策略')
    expect(items[1]?.textContent).toContain('用户策略')
    expect(screen.getByText('适用时间：全天')).not.toBeNull()
    const appliedStatus = screen.getByLabelText(/已采用；.*适用时间：全天.*来源：global/)
    expect(appliedStatus.getAttribute('aria-label')).not.toContain('时间：全天')
    expect(document.body.textContent).not.toContain('DO-NOT-RENDER-POLICY-SECRET')
  })
})
