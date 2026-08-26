import { QueryClient, VueQueryPlugin } from '@tanstack/vue-query'
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/vue'
import { createPinia } from 'pinia'
import { createMemoryHistory, createRouter } from 'vue-router'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

import { vuetify } from '../../plugins/vuetify'

const sdk = vi.hoisted(() => ({
  getBootstrapState: vi.fn(),
  initializeControlPlane: vi.fn(),
}))

vi.mock('../../api/generated/sdk.gen', () => sdk)

import SetupPage from '../SetupPage.vue'

const setupToken = 'a'.repeat(64)
const password = 'owner-password-2026'

const fakeResponse = (status: number, headers: Record<string, string> = {}) => {
  const normalizedHeaders = Object.fromEntries(
    Object.entries(headers).map(([name, value]) => [name.toLowerCase(), value]),
  )
  return {
    status,
    headers: {
      get: (name: string) => normalizedHeaders[name.toLowerCase()] ?? null,
    },
  }
}

const bootstrapResult = (initialized: boolean) => ({
  data: {
    data: {
      initialized,
      login_methods: [],
      product: 'NodeControll',
      setup_capability_required: !initialized,
    },
    meta: { api_version: 'v1', request_id: '00000000-0000-4000-8000-000000000001' },
  },
  error: undefined,
  response: fakeResponse(200),
})

const successResult = () => ({
  data: {
    data: {
      instance_id: '00000000-0000-4000-8000-000000000002',
      owner_id: '00000000-0000-4000-8000-000000000003',
    },
    meta: { api_version: 'v1', request_id: '00000000-0000-4000-8000-000000000004' },
  },
  error: undefined,
  response: fakeResponse(201),
})

const problemResult = (
  status: number,
  code: string,
  options: {
    detail?: string
    errors?: Array<{ code: string; message: string; pointer: string }>
    headers?: Record<string, string>
  } = {},
) => ({
  data: undefined,
  error: {
    type: `urn:nodecontroll:problem:${code.toLowerCase()}`,
    title: 'Untrusted server title',
    status,
    code,
    detail: options.detail ?? 'Untrusted server detail',
    request_id: '00000000-0000-4000-8000-000000000005',
    errors: options.errors ?? [],
  },
  response: fakeResponse(status, options.headers),
})

const renderSetupPage = () => {
  const queryClient = new QueryClient({
    defaultOptions: {
      mutations: { retry: false },
      queries: { gcTime: 0, retry: false },
    },
  })
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/', component: { render: () => null } },
      { path: '/login', name: 'login', component: { render: () => null } },
    ],
  })
  return {
    ...render(SetupPage, {
      global: {
        plugins: [createPinia(), router, vuetify, [VueQueryPlugin, { queryClient }]],
      },
    }),
    router,
  }
}

const fillValidForm = async (confirmation = password) => {
  await fireEvent.update(await screen.findByLabelText('一次性 Setup Token'), setupToken)
  await fireEvent.update(screen.getByLabelText('实例名称'), '测试实例')
  await fireEvent.update(screen.getByLabelText('Owner 用户名'), 'owner.test')
  await fireEvent.update(screen.getByLabelText('Owner 密码'), password)
  await fireEvent.update(screen.getByLabelText('确认 Owner 密码'), confirmation)
}

const submit = async () => {
  await fireEvent.click(screen.getByRole('button', { name: '完成控制面初始化' }))
}

beforeEach(() => {
  sdk.getBootstrapState.mockReset()
  sdk.initializeControlPlane.mockReset()
  sdk.getBootstrapState.mockResolvedValue(bootstrapResult(false))
})

afterEach(() => {
  cleanup()
})

describe('SetupPage', () => {
  it('sends the setup capability only in the header and excludes confirmation from the body', async () => {
    sdk.initializeControlPlane.mockResolvedValue(successResult())
    renderSetupPage()
    await fillValidForm()
    await submit()

    await waitFor(() => expect(sdk.initializeControlPlane).toHaveBeenCalledTimes(1))
    expect(sdk.initializeControlPlane).toHaveBeenCalledWith({
      credentials: 'same-origin',
      headers: { 'x-nodecontroll-setup-token': setupToken },
      body: {
        instance_name: '测试实例',
        username: 'owner.test',
        password,
      },
    })
  })

  it('keeps submission disabled while password confirmation does not match', async () => {
    renderSetupPage()
    await fillValidForm('different-password')

    const button = screen.getByRole('button', {
      name: '完成控制面初始化',
    }) as HTMLButtonElement
    expect(button.disabled).toBe(true)
    expect(screen.getByText('两次输入的密码不一致')).toBeTruthy()
    await fireEvent.click(button)
    expect(sdk.initializeControlPlane).not.toHaveBeenCalled()
  })

  it('clears all submitted secrets before refetching and transitions to initialized state', async () => {
    let resolveRefetch!: (value: ReturnType<typeof bootstrapResult>) => void
    const refetchResult = new Promise<ReturnType<typeof bootstrapResult>>((resolve) => {
      resolveRefetch = resolve
    })
    sdk.getBootstrapState
      .mockResolvedValueOnce(bootstrapResult(false))
      .mockReturnValueOnce(refetchResult)
    sdk.initializeControlPlane.mockResolvedValue(successResult())

    const { router } = renderSetupPage()
    await fillValidForm()
    const setupTokenInput = screen.getByLabelText('一次性 Setup Token') as HTMLInputElement
    const passwordInput = screen.getByLabelText('Owner 密码') as HTMLInputElement
    const passwordConfirmationInput = screen.getByLabelText(
      '确认 Owner 密码',
    ) as HTMLInputElement
    await submit()

    await waitFor(() => expect(sdk.getBootstrapState).toHaveBeenCalledTimes(2))
    expect(setupTokenInput.value).toBe('')
    expect(passwordInput.value).toBe('')
    expect(passwordConfirmationInput.value).toBe('')

    resolveRefetch(bootstrapResult(true))
    expect(await screen.findByText('实例已初始化')).toBeTruthy()
    await waitFor(() => expect(router.currentRoute.value.name).toBe('login'))
  })

  it('keeps the setup form locked when a successful write cannot be reconciled', async () => {
    sdk.getBootstrapState
      .mockResolvedValueOnce(bootstrapResult(false))
      .mockRejectedValueOnce(new Error('reconcile unavailable'))
    sdk.initializeControlPlane.mockResolvedValue(successResult())

    renderSetupPage()
    await fillValidForm()
    await submit()

    expect(await screen.findByTestId('setup-reconcile-lock')).toBeTruthy()
    expect(screen.getByText('初始化写入已被接受')).toBeTruthy()
    expect(screen.queryByRole('button', { name: '完成控制面初始化' })).toBeNull()
    expect(screen.getByRole('button', { name: '重新读取状态' })).toBeTruthy()
  })

  it('maps a typed 403 Problem to the setup capability control without rendering server detail', async () => {
    sdk.initializeControlPlane.mockResolvedValue(
      problemResult(403, 'SETUP_CAPABILITY_INVALID', {
        detail: `do not render ${setupToken} or ${password}`,
      }),
    )
    renderSetupPage()
    await fillValidForm()
    await submit()

    const alert = await screen.findByTestId('initialization-error')
    expect(alert.textContent).toContain('Setup Token 无效、已过期或已使用')
    expect(alert.textContent).toContain('SETUP_CAPABILITY_INVALID')
    expect(screen.getByText('请填写部署服务器当前 setup-token 文件中的一次性 Token。')).toBeTruthy()
    expect(alert.textContent).not.toContain(setupToken)
    expect(alert.textContent).not.toContain(password)
    expect((screen.getByLabelText('一次性 Setup Token') as HTMLInputElement).value).toBe('')
    expect((screen.getByLabelText('Owner 密码') as HTMLInputElement).value).toBe('')
    expect((screen.getByLabelText('确认 Owner 密码') as HTMLInputElement).value).toBe('')
  })

  it('uses a JSON pointer and code to place a trusted local field error on the matching control', async () => {
    const untrustedMessage = `server echoed ${password}`
    sdk.initializeControlPlane.mockResolvedValue(
      problemResult(400, 'VALIDATION_FAILED', {
        errors: [
          {
            pointer: '/username',
            code: 'invalid_username',
            message: untrustedMessage,
          },
        ],
      }),
    )
    renderSetupPage()
    await fillValidForm()
    await submit()

    const username = screen.getByLabelText('Owner 用户名')
    const localMessage = '用户名须为 3–32 位英文、数字、下划线、连字符或点。'
    await waitFor(() => expect(username.closest('.v-input')?.textContent).toContain(localMessage))
    expect(document.body.textContent).not.toContain(untrustedMessage)
  })

  it('refetches bootstrap state after a 409 and follows the server transition', async () => {
    sdk.getBootstrapState
      .mockResolvedValueOnce(bootstrapResult(false))
      .mockResolvedValueOnce(bootstrapResult(true))
    sdk.initializeControlPlane.mockResolvedValue(problemResult(409, 'ALREADY_INITIALIZED'))

    renderSetupPage()
    await fillValidForm()
    await submit()

    expect(await screen.findByText('实例已初始化')).toBeTruthy()
    expect(sdk.getBootstrapState).toHaveBeenCalledTimes(2)
    expect(screen.queryByRole('button', { name: '完成控制面初始化' })).toBeNull()
  })

  it('keeps an identity conflict editable and places a local error on the username', async () => {
    sdk.initializeControlPlane.mockResolvedValue(problemResult(409, 'IDENTITY_CONFLICT'))

    renderSetupPage()
    await fillValidForm()
    await submit()

    expect(
      await screen.findByText(
        'Owner 用户名与数据库中的现有身份冲突。请更换用户名，或由部署管理员检查待初始化数据。',
      ),
    ).toBeTruthy()
    const username = screen.getByLabelText('Owner 用户名')
    expect(username.closest('.v-input')?.textContent).toContain(
      '此 Owner 用户名与数据库中的现有身份冲突。',
    )
    expect(sdk.getBootstrapState).toHaveBeenCalledTimes(1)
    expect(screen.getByRole('button', { name: '完成控制面初始化' })).toBeTruthy()
  })

  it('locks the form for an unknown conflict until the server state is reconciled', async () => {
    sdk.getBootstrapState
      .mockResolvedValueOnce(bootstrapResult(false))
      .mockRejectedValueOnce(new Error('reconcile unavailable'))
    sdk.initializeControlPlane.mockResolvedValue(problemResult(409, 'UNKNOWN_CONFLICT'))

    renderSetupPage()
    await fillValidForm()
    await submit()

    expect(await screen.findByTestId('setup-reconcile-lock')).toBeTruthy()
    expect(screen.queryByRole('button', { name: '完成控制面初始化' })).toBeNull()
    expect(sdk.getBootstrapState).toHaveBeenCalledTimes(2)
  })

  it('renders a bounded Retry-After delay for typed rate limiting', async () => {
    sdk.initializeControlPlane.mockResolvedValue(
      problemResult(429, 'BOOTSTRAP_RATE_LIMITED', {
        headers: { 'Retry-After': '7' },
      }),
    )
    renderSetupPage()
    await fillValidForm()
    await submit()

    expect(await screen.findByText('初始化尝试过于频繁，请在 7 秒后重试。')).toBeTruthy()
  })

  it('does not render a Retry-After value above the one-hour bound', async () => {
    sdk.initializeControlPlane.mockResolvedValue(
      problemResult(429, 'BOOTSTRAP_RATE_LIMITED', {
        headers: { 'Retry-After': '3601' },
      }),
    )
    renderSetupPage()
    await fillValidForm()
    await submit()

    expect(await screen.findByText('初始化尝试过于频繁，请稍后重试。')).toBeTruthy()
    expect(document.body.textContent).not.toContain('3601')
  })

  it('treats prototype-shaped pointer and field codes as untrusted unknown values', async () => {
    const untrustedMessage = `prototype payload ${setupToken} ${password}`
    sdk.initializeControlPlane.mockResolvedValue(
      problemResult(400, 'VALIDATION_FAILED', {
        errors: [
          { pointer: '__proto__', code: 'invalid_username', message: untrustedMessage },
          { pointer: '/username', code: 'constructor', message: untrustedMessage },
        ],
      }),
    )
    renderSetupPage()
    await fillValidForm()
    await submit()

    const username = screen.getByLabelText('Owner 用户名')
    await waitFor(() =>
      expect(username.closest('.v-input')?.textContent).toContain(
        '服务端拒绝了此字段，请修改后重试。',
      ),
    )
    expect(document.body.textContent).not.toContain(untrustedMessage)
  })

  it('clears every submitted secret after a rejected network request', async () => {
    sdk.initializeControlPlane.mockRejectedValue(
      new Error(`untrusted network failure ${setupToken} ${password}`),
    )
    renderSetupPage()
    await fillValidForm()
    await submit()

    const alert = await screen.findByTestId('initialization-error')
    expect(alert.textContent).toContain('请检查 Master readiness 与网络连接后重试')
    expect(document.body.textContent).not.toContain(setupToken)
    expect(document.body.textContent).not.toContain(password)
    expect((screen.getByLabelText('一次性 Setup Token') as HTMLInputElement).value).toBe('')
    expect((screen.getByLabelText('Owner 密码') as HTMLInputElement).value).toBe('')
    expect((screen.getByLabelText('确认 Owner 密码') as HTMLInputElement).value).toBe('')
  })

  it('uses a generic fallback and never renders untrusted server detail or submitted secrets', async () => {
    const secretDetail = `unexpected ${setupToken} ${password}`
    sdk.initializeControlPlane.mockResolvedValue(
      problemResult(503, 'UNKNOWN_SERVER_CODE', { detail: secretDetail }),
    )
    renderSetupPage()
    await fillValidForm()
    await submit()

    const alert = await screen.findByTestId('initialization-error')
    expect(alert.textContent).toContain('请检查 Master readiness 与网络连接后重试')
    expect(alert.textContent).not.toContain(secretDetail)
    expect(alert.textContent).not.toContain(setupToken)
    expect(alert.textContent).not.toContain(password)
    expect(alert.textContent).not.toContain('UNKNOWN_SERVER_CODE')
  })
})
