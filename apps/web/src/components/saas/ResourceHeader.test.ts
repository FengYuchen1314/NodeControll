import { cleanup, fireEvent, render, screen } from '@testing-library/vue'
import { createMemoryHistory, createRouter } from 'vue-router'
import { afterEach, describe, expect, it } from 'vitest'

import { vuetify } from '../../plugins/vuetify'
import ResourceHeader from './ResourceHeader.vue'

afterEach(() => cleanup())

describe('ResourceHeader', () => {
  it('renders one page heading, typed breadcrumbs, status and keyboard-reachable actions', async () => {
    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: '/', component: { render: () => null } },
        { path: '/servers', component: { render: () => null } },
      ],
    })
    await router.push('/servers')
    const result = render(ResourceHeader, {
      global: { plugins: [router, vuetify] },
      props: {
        breadcrumbs: [{ label: '资源', to: '/' }, { label: '服务器' }],
        description: '展示资源事实和可执行动作。',
        overflowLabel: '服务器更多操作',
        primaryActionLabel: '添加服务器',
        revision: 7,
        title: '服务器',
      },
      slots: {
        actions: '<button type="button">刷新状态</button>',
        overflow: '<div role="menuitem">导出</div>',
        status: '<span data-testid="header-status">状态插槽</span>',
      },
    })

    expect(screen.getAllByRole('heading', { level: 1 })).toHaveLength(1)
    expect(screen.getByRole('navigation', { name: '面包屑' })).not.toBeNull()
    expect(screen.getByRole('link', { name: '资源' }).getAttribute('href')).toBe('/')
    expect(screen.getByText('Revision 7')).not.toBeNull()
    expect(screen.getByTestId('header-status')).not.toBeNull()
    expect(screen.getByRole('button', { name: '刷新状态' })).not.toBeNull()
    expect(screen.getByRole('button', { name: '服务器更多操作' })).not.toBeNull()

    await fireEvent.click(screen.getByRole('button', { name: '添加服务器' }))
    expect(result.emitted().primaryAction).toHaveLength(1)
  })
})
