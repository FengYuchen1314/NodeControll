import { cleanup, fireEvent, render, screen } from '@testing-library/vue'
import { afterEach, describe, expect, it, vi } from 'vitest'

import { vuetify } from '../../plugins/vuetify'
import OneTimeRecoveryCodes from './OneTimeRecoveryCodes.vue'

const codes = Array.from(
  { length: 8 },
  (_, index) => `${index.toString(16).padStart(4, '0')}-1111-2222-3333-4444-5555-6666-7777`,
)

afterEach(() => {
  cleanup()
  vi.restoreAllMocks()
  vi.unstubAllGlobals()
})

describe('OneTimeRecoveryCodes', () => {
  it('requires explicit confirmation and downloads through a promptly revoked object URL', async () => {
    const createObjectURL = vi.fn().mockReturnValue('blob:recovery-codes')
    const revokeObjectURL = vi.fn()
    vi.stubGlobal('URL', { createObjectURL, revokeObjectURL })
    const anchorClick = vi.spyOn(HTMLAnchorElement.prototype, 'click').mockImplementation(() => {})
    const result = render(OneTimeRecoveryCodes, {
      global: { plugins: [vuetify] },
      props: { codes, context: 'regenerated' },
    })

    const confirm = screen.getByTestId('confirm-recovery-codes') as HTMLButtonElement
    expect(confirm.disabled).toBe(true)
    await fireEvent.click(screen.getByTestId('download-recovery-codes'))
    await Promise.resolve()

    expect(createObjectURL).toHaveBeenCalledWith(expect.any(Blob))
    expect(anchorClick).toHaveBeenCalledTimes(1)
    expect(revokeObjectURL).toHaveBeenCalledWith('blob:recovery-codes')

    await fireEvent.click(screen.getByLabelText('我已把这组恢复码保存到安全位置'))
    await fireEvent.click(confirm)
    expect(result.emitted().confirmed).toHaveLength(1)
  })

  it('keeps confirmation disabled until bootstrap reconciliation finishes', async () => {
    const result = render(OneTimeRecoveryCodes, {
      global: { plugins: [vuetify] },
      props: { codes, confirmationReady: false, context: 'bootstrap' },
    })

    await fireEvent.click(screen.getByLabelText('我已把这组恢复码保存到安全位置'))
    const confirm = screen.getByTestId('confirm-recovery-codes') as HTMLButtonElement
    expect(confirm.disabled).toBe(true)
    await result.rerender({ codes, confirmationReady: true, context: 'bootstrap' })
    expect(confirm.disabled).toBe(false)
  })
})
