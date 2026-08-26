import { cleanup, fireEvent, render, screen } from '@testing-library/vue'
import { afterEach, describe, expect, it, vi } from 'vitest'

import { vuetify } from '../../plugins/vuetify'
import SecretField from './SecretField.vue'

afterEach(() => {
  cleanup()
  vi.restoreAllMocks()
})

describe('SecretField', () => {
  it('keeps the value out of browser persistence and conceals it on pagehide', async () => {
    const secret = 'candidate-secret-material'
    const storageWrite = vi.spyOn(Storage.prototype, 'setItem')
    const consoleLog = vi.spyOn(console, 'log').mockImplementation(() => undefined)
    const result = render(SecretField, {
      global: { plugins: [vuetify] },
      props: {
        allowClear: true,
        configured: true,
        label: 'API 密钥',
        mode: 'replace',
        modelValue: secret,
        oneTimeReveal: true,
      },
    })

    const input = screen.getByLabelText('API 密钥') as HTMLInputElement
    expect(input.type).toBe('password')
    await fireEvent.click(screen.getByRole('button', { name: '显示 API 密钥' }))
    expect(input.type).toBe('text')
    globalThis.dispatchEvent(new Event('pagehide'))
    expect(input.type).toBe('password')

    await fireEvent.update(input, 'replacement-secret')
    expect(result.emitted()['update:modelValue']).toContainEqual(['replacement-secret'])
    await fireEvent.click(screen.getByLabelText('移除已配置的秘密值'))
    expect(result.emitted()['update:clearRequested']).toContainEqual([true])
    expect(result.emitted()['update:modelValue']).toContainEqual([''])

    const persisted = storageWrite.mock.calls.map((call) => String(call[1])).join('\n')
    expect(persisted).not.toContain(secret)
    expect(persisted).not.toContain('replacement-secret')
    expect(consoleLog.mock.calls.flat().join('\n')).not.toContain(secret)
  })
})
