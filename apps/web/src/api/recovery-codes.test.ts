import { afterEach, describe, expect, it, vi } from 'vitest'

import {
  getRecoveryCodeStatus,
  initializeControlPlaneWithRecoveryCodes,
  regenerateRecoveryCodes,
  validOneTimeRecoveryCodes,
} from './recovery-codes'

const codes = Array.from(
  { length: 8 },
  (_, index) => `${index.toString(16).padStart(4, '0')}-1111-2222-3333-4444-5555-6666-7777`,
)

const envelope = (data: unknown) => ({
  data,
  meta: { api_version: 'v1', request_id: 'recovery-request' },
})

const jsonResponse = (status: number, data: unknown, cacheControl?: string) =>
  new Response(JSON.stringify(data), {
    headers: {
      'content-type': 'application/json',
      ...(cacheControl ? { 'cache-control': cacheControl } : {}),
    },
    status,
  })

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('temporary recovery-code API boundary', () => {
  it('accepts only eight canonical 39-character hex codes and normalizes case for uniqueness', () => {
    expect(validOneTimeRecoveryCodes(codes)).toBe(true)
    expect(validOneTimeRecoveryCodes(codes.map((code) => code.toUpperCase()))).toBe(true)
    expect(validOneTimeRecoveryCodes(codes.slice(0, 7))).toBe(false)
    expect(validOneTimeRecoveryCodes([...codes, codes[0]])).toBe(false)
    expect(
      validOneTimeRecoveryCodes([codes[0]!, codes[0]!.toUpperCase(), ...codes.slice(2)]),
    ).toBe(false)
    expect(validOneTimeRecoveryCodes(['0000-1111', ...codes.slice(1)])).toBe(false)
    expect(
      validOneTimeRecoveryCodes([
        'zzzz-1111-2222-3333-4444-5555-6666-7777',
        ...codes.slice(1),
      ]),
    ).toBe(false)
  })

  it('accepts the bootstrap secret only with the exact typed shape and no-store response', async () => {
    const fetchMock = vi.fn().mockResolvedValue(
      jsonResponse(
        201,
        envelope({
          instance_id: '00000000-0000-4000-8000-000000000002',
          one_time_recovery_codes: codes,
          owner_id: '00000000-0000-4000-8000-000000000003',
        }),
        'private, no-store',
      ),
    )
    vi.stubGlobal('fetch', fetchMock)

    const result = await initializeControlPlaneWithRecoveryCodes({
      body: { instance_name: 'test', password: 'owner-password-2026', username: 'owner' },
      setupToken: 'a'.repeat(64),
    })

    expect(result.data?.data.one_time_recovery_codes).toEqual(codes)
    expect(fetchMock).toHaveBeenCalledWith(
      '/api/v1/bootstrap',
      expect.objectContaining({
        cache: 'no-store',
        credentials: 'same-origin',
        headers: expect.objectContaining({ 'x-nodecontroll-setup-token': 'a'.repeat(64) }),
        method: 'POST',
        redirect: 'error',
      }),
    )
  })

  it('discards a secret-bearing success when no-store is absent or codes are duplicated', async () => {
    const bootstrapData = {
      instance_id: '00000000-0000-4000-8000-000000000002',
      one_time_recovery_codes: codes,
      owner_id: '00000000-0000-4000-8000-000000000003',
    }
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(jsonResponse(201, envelope(bootstrapData)))
      .mockResolvedValueOnce(
        jsonResponse(
          201,
          envelope({
            created_at_ms: 1_780_000_000_000,
            one_time_recovery_codes: [codes[0], ...codes.slice(0, 7)],
            set_version: 2,
          }),
          'no-store',
        ),
      )
    vi.stubGlobal('fetch', fetchMock)

    const bootstrap = await initializeControlPlaneWithRecoveryCodes({
      body: { instance_name: 'test', password: 'owner-password-2026', username: 'owner' },
      setupToken: 'b'.repeat(64),
    })
    const regeneration = await regenerateRecoveryCodes({ csrfToken: `ncc1_${'c'.repeat(64)}` })

    expect(bootstrap.data).toBeUndefined()
    expect(regeneration.data).toBeUndefined()
  })

  it('rejects an otherwise valid regeneration envelope when the status is not exactly 200', async () => {
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        jsonResponse(
          201,
          envelope({
            created_at_ms: 1_780_000_001_000,
            one_time_recovery_codes: codes,
            set_version: 2,
          }),
          'no-store',
        ),
      ),
    )

    const result = await regenerateRecoveryCodes({ csrfToken: `ncc1_${'1'.repeat(64)}` })

    expect(result.data).toBeUndefined()
    expect(result.payloadState).toBe('valid-json')
    expect(result.response.status).toBe(201)
  })

  it('distinguishes empty and malformed error bodies and accepts Problems only by media type', async () => {
    const problem = {
      code: 'SETUP_CAPABILITY_INVALID',
      detail: 'untrusted',
      request_id: 'request-error',
      status: 403,
      title: 'untrusted',
      type: 'urn:nodecontroll:problem:setup-capability-invalid',
    }
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(
        new Response(null, {
          headers: { 'content-type': 'application/problem+json' },
          status: 500,
        }),
      )
      .mockResolvedValueOnce(
        new Response('{', {
          headers: { 'content-type': 'application/problem+json' },
          status: 500,
        }),
      )
      .mockResolvedValueOnce(jsonResponse(403, problem))
      .mockResolvedValueOnce(
        new Response(JSON.stringify(problem), {
          headers: { 'content-type': 'application/problem+json' },
          status: 403,
        }),
      )
    vi.stubGlobal('fetch', fetchMock)

    const results = []
    for (let index = 0; index < 4; index += 1) {
      results.push(
        await initializeControlPlaneWithRecoveryCodes({
          body: { instance_name: 'test', password: 'owner-password-2026', username: 'owner' },
          setupToken: 'e'.repeat(64),
        }),
      )
    }

    expect(results[0]).toMatchObject({ payloadState: 'invalid', response: { status: 500 } })
    expect(results[1]).toMatchObject({ payloadState: 'invalid', response: { status: 500 } })
    expect(results[2]).toMatchObject({ payloadState: 'valid-json', response: { status: 403 } })
    expect(results[2]?.error).toBeUndefined()
    expect(results[3]?.error).toEqual(problem)
  })

  it('cancels the response stream as soon as the cumulative 64 KiB limit is crossed', async () => {
    const cancel = vi.fn()
    let emitted = 0
    const stream = new ReadableStream<Uint8Array>({
      cancel,
      pull(controller) {
        emitted += 1
        controller.enqueue(new Uint8Array(32 * 1024))
        if (emitted === 4) controller.close()
      },
    })
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue(
        new Response(stream, {
          headers: { 'content-type': 'application/problem+json' },
          status: 500,
        }),
      ),
    )

    const result = await initializeControlPlaneWithRecoveryCodes({
      body: { instance_name: 'test', password: 'owner-password-2026', username: 'owner' },
      setupToken: 'f'.repeat(64),
    })

    expect(result.payloadState).toBe('invalid')
    expect(result.error).toBeUndefined()
    expect(cancel).toHaveBeenCalledTimes(1)
  })

  it('uses status GET without CSRF and regeneration POST with CSRF', async () => {
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(
        jsonResponse(
          200,
          envelope({
            created_at_ms: 1_780_000_000_000,
            remaining_count: 7,
            set_version: 1,
            total_count: 8,
          }),
        ),
      )
      .mockResolvedValueOnce(
        jsonResponse(
          200,
          envelope({
            created_at_ms: 1_780_000_001_000,
            one_time_recovery_codes: codes,
            set_version: 2,
          }),
          'no-store',
        ),
      )
    vi.stubGlobal('fetch', fetchMock)

    expect((await getRecoveryCodeStatus()).data?.data.remaining_count).toBe(7)
    expect(
      (await regenerateRecoveryCodes({ csrfToken: `ncc1_${'d'.repeat(64)}` })).data?.data
        .set_version,
    ).toBe(2)
    expect(fetchMock).toHaveBeenNthCalledWith(
      1,
      '/api/v1/me/recovery-codes',
      expect.objectContaining({ method: 'GET' }),
    )
    expect(fetchMock).toHaveBeenNthCalledWith(
      2,
      '/api/v1/me/recovery-codes',
      expect.objectContaining({
        headers: expect.objectContaining({ 'x-nodecontroll-csrf': `ncc1_${'d'.repeat(64)}` }),
        method: 'POST',
        redirect: 'error',
      }),
    )
  })
})
