import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/vue'
import { afterEach, describe, expect, it } from 'vitest'

import { vuetify } from '../../plugins/vuetify'
import DangerDialog from './DangerDialog.vue'

afterEach(() => cleanup())

describe('DangerDialog', () => {
  it('requires an exact object name and reason, then locks before the parent reports pending', async () => {
    const result = render(DangerDialog, {
      global: { plugins: [vuetify] },
      props: {
        dependencies: ['活动连接', '待部署配置'],
        impactSummary: '删除后无法恢复。',
        modelValue: true,
        objectName: 'edge-01',
        reasonRequired: true,
        title: '删除服务器',
      },
    })

    expect(await screen.findByRole('alertdialog')).not.toBeNull()
    const confirm = await screen.findByRole('button', { name: '确认执行' })
    expect((confirm as HTMLButtonElement).disabled).toBe(true)
    const objectConfirmation = screen.getByLabelText('资源名称：edge-01')
    await fireEvent.update(objectConfirmation, 'EDGE-01')
    await fireEvent.update(screen.getByLabelText('操作原因'), '维护下线')
    expect((confirm as HTMLButtonElement).disabled).toBe(true)

    await fireEvent.update(objectConfirmation, 'edge-01')
    expect((confirm as HTMLButtonElement).disabled).toBe(false)
    await fireEvent.submit(screen.getByRole('form', { name: '危险操作确认' }))
    await fireEvent.click(confirm)

    expect(result.emitted().confirm).toEqual([[{ reason: '维护下线' }]])
    expect(screen.getByRole('status').textContent).toContain('不会重复发送')
    expect((screen.getByRole('button', { name: '取消' }) as HTMLButtonElement).disabled).toBe(true)
  })

  it('fails closed for an empty object name and cannot be dismissed while pending', async () => {
    const result = render(DangerDialog, {
      global: { plugins: [vuetify] },
      props: {
        impactSummary: '操作影响未知。',
        modelValue: true,
        objectName: '',
        pending: true,
        title: '危险操作',
      },
    })

    const confirm = await screen.findByRole('button', { name: '确认执行' })
    expect((confirm as HTMLButtonElement).disabled).toBe(true)
    await fireEvent.click(screen.getByRole('button', { name: '取消' }))
    await waitFor(() => expect(result.emitted()['update:modelValue']).toBeUndefined())
  })

  it('unlocks a retry only after a new explicit terminal failure revision', async () => {
    const result = render(DangerDialog, {
      global: { plugins: [vuetify] },
      props: {
        errorMessage: undefined,
        impactSummary: '删除后无法恢复。',
        modelValue: true,
        objectName: 'edge-01',
        retryRevision: 7,
        title: '删除服务器',
      },
    })

    await fireEvent.update(screen.getByLabelText('资源名称：edge-01'), 'edge-01')
    const confirm = await screen.findByRole('button', { name: '确认执行' })
    await fireEvent.click(confirm)
    expect(result.emitted().confirm).toHaveLength(1)
    expect((confirm as HTMLButtonElement).disabled).toBe(true)

    await result.rerender({ errorMessage: '请求未发送，请检查连接。' })
    expect(screen.getByTestId('danger-dialog-error').textContent).toContain('请求未发送')
    expect((confirm as HTMLButtonElement).disabled).toBe(true)

    await result.rerender({ pending: true })
    await result.rerender({ pending: false })
    expect((confirm as HTMLButtonElement).disabled).toBe(true)

    await result.rerender({ retryRevision: 8 })
    expect((confirm as HTMLButtonElement).disabled).toBe(false)
    await fireEvent.click(confirm)
    expect(result.emitted().confirm).toHaveLength(2)
  })
})
