import { cleanup, fireEvent, render, screen } from '@testing-library/vue'
import { afterEach, describe, expect, it } from 'vitest'

import { vuetify } from '../../plugins/vuetify'
import JobChip from './JobChip.vue'
import JobDrawer from './JobDrawer.vue'
import type { JobChipLabels, JobDrawerLabels, JobPresentation, SafeDisplayValue } from './types'

const chipLabels: JobChipLabels = {
  source: (source) => `Source: ${source}`,
  states: {
    cancelled: 'Cancelled',
    expired: 'Expired',
    failed: 'Failed',
    queued: 'Queued',
    running: 'Running',
    succeeded: 'Succeeded',
    waiting: 'Waiting',
  },
  updatedAt: (updatedAt) => `Updated: ${updatedAt}`,
}

const drawerLabels: JobDrawerLabels = {
  close: 'Close job details',
  createdAt: 'Created',
  empty: 'No job selected',
  emptyValue: 'Not set',
  jobId: 'Job ID',
  overline: 'JOB',
  progress: (percent) => `Job progress: ${percent}%`,
  redactedValue: 'Sensitive value hidden',
  source: 'Source',
  stepStates: {
    failed: 'Failed',
    pending: 'Pending',
    running: 'Running',
    skipped: 'Skipped',
    succeeded: 'Succeeded',
  },
  steps: 'Execution steps',
  title: 'Job details',
  updatedAt: 'Updated',
}

const job = (state: JobPresentation['state']): JobPresentation => ({
  id: `job-${state}`,
  label: 'Reconcile server',
  progressPercent: 140,
  source: 'Master scheduler',
  state,
  updatedAt: '10:00',
})

afterEach(() => cleanup())

describe('job presentation primitives', () => {
  it('covers canonical waiting and expired states and clamps progress', async () => {
    const result = render(JobChip, {
      global: { plugins: [vuetify] },
      props: { job: job('waiting'), labels: chipLabels },
    })
    const waiting = screen.getByText(/Waiting/)
    expect(waiting.textContent).toContain('100%')
    expect(waiting.closest('[aria-label]')?.getAttribute('aria-label')).toContain(
      'Source: Master scheduler',
    )
    await fireEvent.click(waiting)
    expect(result.emitted().open).toEqual([['job-waiting']])

    await result.rerender({ job: job('expired') })
    expect(screen.getByText(/Expired/)).not.toBeNull()
  })

  it('renders only the safe job-message projection and never reads hidden extra text', async () => {
    const redacted: SafeDisplayValue & { text: string } = {
      kind: 'redacted',
      text: 'RAW-SECRET-MUST-NOT-RENDER',
    }
    render(JobDrawer, {
      global: { plugins: [vuetify] },
      props: {
        chipLabels,
        job: {
          ...job('running'),
          message: redacted,
          steps: [{ id: 'step-1', label: 'Apply', message: redacted, state: 'running' }],
        },
        labels: drawerLabels,
        modelValue: true,
      },
    })

    expect(await screen.findAllByText('Sensitive value hidden')).toHaveLength(2)
    expect(screen.queryByText('RAW-SECRET-MUST-NOT-RENDER')).toBeNull()
    expect(screen.getByLabelText('Job progress: 100%')).not.toBeNull()
  })

  it('is read-only and uses caller-provided empty and close labels', async () => {
    const result = render(JobDrawer, {
      global: { plugins: [vuetify] },
      props: { chipLabels, labels: drawerLabels, modelValue: true },
    })

    expect(await screen.findByText('No job selected')).not.toBeNull()
    expect(screen.queryByRole('button', { name: /retry|cancel/i })).toBeNull()
    await fireEvent.click(screen.getByRole('button', { name: 'Close job details' }))
    expect(result.emitted()['update:modelValue']).toEqual([[false]])
  })
})
