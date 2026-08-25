<script setup lang="ts">
import { useMutation, useQuery } from '@tanstack/vue-query'
import { computed, reactive, ref } from 'vue'

import { getBootstrapState, initializeControlPlane } from '../api/generated/sdk.gen'

type BootstrapField = 'instanceName' | 'password' | 'username'

type SafeProblemCode =
  | 'ALREADY_INITIALIZED'
  | 'BOOTSTRAP_JSON_INVALID'
  | 'BOOTSTRAP_JSON_SHAPE_INVALID'
  | 'BOOTSTRAP_RATE_LIMITED'
  | 'BOOTSTRAP_STATE_INCONSISTENT'
  | 'BOOTSTRAP_UNAVAILABLE'
  | 'IDENTITY_CONFLICT'
  | 'PAYLOAD_TOO_LARGE'
  | 'SETUP_CAPABILITY_INVALID'
  | 'UNSUPPORTED_MEDIA_TYPE'
  | 'VALIDATION_FAILED'

interface SafeFieldErrors {
  instanceName: string[]
  password: string[]
  username: string[]
}

interface HttpResponseLike {
  headers: {
    get(name: string): string | null
  }
  status: number
}

const safeProblemCodes = new Set<SafeProblemCode>([
  'ALREADY_INITIALIZED',
  'BOOTSTRAP_JSON_INVALID',
  'BOOTSTRAP_JSON_SHAPE_INVALID',
  'BOOTSTRAP_RATE_LIMITED',
  'BOOTSTRAP_STATE_INCONSISTENT',
  'BOOTSTRAP_UNAVAILABLE',
  'IDENTITY_CONFLICT',
  'PAYLOAD_TOO_LARGE',
  'SETUP_CAPABILITY_INVALID',
  'UNSUPPORTED_MEDIA_TYPE',
  'VALIDATION_FAILED',
])

const fieldByPointer = new Map<string, BootstrapField>([
  ['/instance_name', 'instanceName'],
  ['/password', 'password'],
  ['/username', 'username'],
])

const safeFieldMessages = new Map<BootstrapField, ReadonlyMap<string, string>>([
  [
    'instanceName',
    new Map([
      ['invalid_instance_name', '实例名称须为 1–80 个 Unicode 标量值，且不能包含控制字符。'],
    ]),
  ],
  ['password', new Map([['invalid_password', '密码不符合服务端密码策略，请修改后重试。']])],
  [
    'username',
    new Map([['invalid_username', '用户名须为 3–32 位英文、数字、下划线、连字符或点。']]),
  ],
])

class InitializationFailure extends Error {
  constructor(
    readonly summary: string,
    readonly fieldErrors: SafeFieldErrors,
    readonly status?: number,
    readonly code?: SafeProblemCode,
  ) {
    super('Control-plane initialization failed')
    this.name = 'InitializationFailure'
  }
}

const emptyFieldErrors = (): SafeFieldErrors => ({
  instanceName: [],
  password: [],
  username: [],
})

const asRecord = (value: unknown): Record<string, unknown> | undefined =>
  typeof value === 'object' && value !== null ? (value as Record<string, unknown>) : undefined

const safeStatus = (problem: Record<string, unknown> | undefined, response?: HttpResponseLike) => {
  if (response && Number.isInteger(response.status)) return response.status
  const candidate = problem?.status
  return typeof candidate === 'number' && Number.isInteger(candidate) && candidate >= 400 && candidate < 600
    ? candidate
    : undefined
}

const safeProblemCode = (problem: Record<string, unknown> | undefined) => {
  const candidate = problem?.code
  return typeof candidate === 'string' && safeProblemCodes.has(candidate as SafeProblemCode)
    ? (candidate as SafeProblemCode)
    : undefined
}

const retryAfterSeconds = (response?: HttpResponseLike) => {
  const value = response?.headers.get('retry-after')?.trim()
  if (!value) return undefined
  if (/^\d{1,5}$/.test(value)) {
    const seconds = Number(value)
    return seconds <= 3_600 ? seconds : undefined
  }
  const timestamp = Date.parse(value)
  if (!Number.isFinite(timestamp)) return undefined
  const seconds = Math.max(0, Math.ceil((timestamp - Date.now()) / 1_000))
  return seconds <= 3_600 ? seconds : undefined
}

const extractFieldErrors = (problem: Record<string, unknown> | undefined) => {
  const fieldErrors = emptyFieldErrors()
  if (!Array.isArray(problem?.errors)) return fieldErrors

  for (const item of problem.errors) {
    const error = asRecord(item)
    const pointer = typeof error?.pointer === 'string' ? error.pointer : undefined
    const field = pointer ? fieldByPointer.get(pointer) : undefined
    if (!field) continue
    const code = typeof error?.code === 'string' ? error.code : ''
    fieldErrors[field].push(
      safeFieldMessages.get(field)?.get(code) ?? '服务端拒绝了此字段，请修改后重试。',
    )
  }
  return fieldErrors
}

const failureSummary = (
  status: number | undefined,
  code: SafeProblemCode | undefined,
  retryAfter: number | undefined,
  hasFieldErrors: boolean,
) => {
  if (status === 403 || code === 'SETUP_CAPABILITY_INVALID') {
    return 'Setup Token 无效、已过期或已使用。请从部署服务器重新读取当前 setup-token 文件。'
  }
  if (status === 409 || code === 'ALREADY_INITIALIZED' || code === 'IDENTITY_CONFLICT') {
    return '初始化状态已发生变化，页面已重新读取 Master 的最新状态。'
  }
  if (status === 429 || code === 'BOOTSTRAP_RATE_LIMITED') {
    return retryAfter === undefined
      ? '初始化尝试过于频繁，请稍后重试。'
      : `初始化尝试过于频繁，请在 ${retryAfter} 秒后重试。`
  }
  if (hasFieldErrors || code === 'VALIDATION_FAILED') {
    return '部分字段未通过服务端校验，请修改标记的字段。'
  }
  if (status === 413 || code === 'PAYLOAD_TOO_LARGE') {
    return '提交内容超过 Master 接受的大小限制。'
  }
  if (status === 415 || code === 'UNSUPPORTED_MEDIA_TYPE') {
    return 'Master 拒绝了请求格式，请刷新页面后重试。'
  }
  if (status === 422 || code === 'BOOTSTRAP_JSON_SHAPE_INVALID') {
    return '提交内容与 Master 当前接受的数据结构不一致，请刷新页面后重试。'
  }
  if (code === 'BOOTSTRAP_STATE_INCONSISTENT') {
    return 'Master 检测到初始化状态不一致，需要部署管理员检查数据库。'
  }
  return '初始化未完成。请检查 Master readiness 与网络连接后重试。'
}

const toInitializationFailure = (error: unknown, response?: HttpResponseLike) => {
  const problem = asRecord(error)
  const status = safeStatus(problem, response)
  const code = safeProblemCode(problem)
  const fieldErrors = extractFieldErrors(problem)
  const retryAfter =
    status === 429 || code === 'BOOTSTRAP_RATE_LIMITED'
      ? retryAfterSeconds(response)
      : undefined
  const hasFieldErrors = Object.values(fieldErrors).some((messages) => messages.length > 0)
  return new InitializationFailure(
    failureSummary(status, code, retryAfter, hasFieldErrors),
    fieldErrors,
    status,
    code,
  )
}

const form = reactive({
  instanceName: '',
  username: '',
  password: '',
  passwordConfirmation: '',
  setupToken: '',
})

const utf8Encoder = new globalThis.TextEncoder()
const controlCharacterPattern = /\p{Cc}/u
const usernamePattern = /^[A-Za-z0-9_.-]{3,32}$/
const setupTokenPattern = /^[0-9a-f]{64}$/
const unicodeScalarCount = (value: string) => Array.from(value).length
const hasOnlyUnicodeScalars = (value: string) =>
  Array.from(value).every((character) => {
    const codePoint = character.codePointAt(0)
    return codePoint !== undefined && (codePoint < 0xd800 || codePoint > 0xdfff)
  })

const instanceNameScalarCount = computed(() => unicodeScalarCount(form.instanceName.trim()))
const passwordScalarCount = computed(() => unicodeScalarCount(form.password))
const passwordByteCount = computed(() => utf8Encoder.encode(form.password).length)
const instanceNameIsValid = computed(
  () =>
    instanceNameScalarCount.value >= 1 &&
    instanceNameScalarCount.value <= 80 &&
    hasOnlyUnicodeScalars(form.instanceName.trim()) &&
    !controlCharacterPattern.test(form.instanceName.trim()),
)
const usernameIsValid = computed(() => usernamePattern.test(form.username))
const passwordIsValid = computed(
  () =>
    passwordScalarCount.value >= 12 &&
    passwordByteCount.value <= 1024 &&
    hasOnlyUnicodeScalars(form.password) &&
    !controlCharacterPattern.test(form.password),
)
const passwordConfirmationIsValid = computed(
  () => form.passwordConfirmation.length > 0 && form.passwordConfirmation === form.password,
)
const setupTokenIsValid = computed(() => setupTokenPattern.test(form.setupToken))
const canSubmit = computed(
  () =>
    instanceNameIsValid.value &&
    usernameIsValid.value &&
    passwordIsValid.value &&
    passwordConfirmationIsValid.value &&
    setupTokenIsValid.value,
)
const passwordHint = computed(
  () =>
    `至少 12 个 Unicode 标量值、最多 1,024 UTF-8 字节；当前 ${passwordScalarCount.value} 个标量值 / ${passwordByteCount.value} 字节`,
)
const lastFailure = ref<InitializationFailure>()
const setupTokenErrors = computed(() =>
  lastFailure.value?.status === 403 || lastFailure.value?.code === 'SETUP_CAPABILITY_INVALID'
    ? ['请填写部署服务器当前 setup-token 文件中的一次性 Token。']
    : [],
)
const instanceNameErrors = computed(() => lastFailure.value?.fieldErrors.instanceName ?? [])
const usernameErrors = computed(() => lastFailure.value?.fieldErrors.username ?? [])
const passwordErrors = computed(() => lastFailure.value?.fieldErrors.password ?? [])

const bootstrap = useQuery({
  queryKey: ['bootstrap'],
  queryFn: async () => {
    const response = await getBootstrapState()
    if (response.error) throw new Error('Unable to load bootstrap state')
    return response.data
  },
})

const initialize = useMutation({
  mutationFn: async () => {
    const response = await initializeControlPlane({
      headers: {
        'x-nodecontroll-setup-token': form.setupToken,
      },
      body: {
        instance_name: form.instanceName,
        username: form.username,
        password: form.password,
      },
    })
    if (response.error) throw toInitializationFailure(response.error, response.response)
    return response.data
  },
  onMutate: () => {
    lastFailure.value = undefined
  },
  onSuccess: async () => {
    form.password = ''
    form.passwordConfirmation = ''
    form.setupToken = ''
    lastFailure.value = undefined
    await bootstrap.refetch()
  },
  onError: async (error) => {
    form.password = ''
    form.passwordConfirmation = ''
    form.setupToken = ''
    lastFailure.value =
      error instanceof InitializationFailure ? error : toInitializationFailure(undefined)
    if (
      lastFailure.value.status === 409 ||
      lastFailure.value.code === 'ALREADY_INITIALIZED' ||
      lastFailure.value.code === 'IDENTITY_CONFLICT'
    ) {
      await bootstrap.refetch()
    }
  },
})

const submit = () => {
  if (!canSubmit.value || initialize.isPending.value) return
  initialize.mutate()
}
</script>

<template>
  <header class="mb-8">
    <p class="text-overline text-primary mb-2">SELF-HOSTED SETUP</p>
    <h1 class="text-h4 font-weight-bold mb-2">实例初始化</h1>
    <p class="text-body-1 text-medium-emphasis">
      此投影来自 Master 的真实数据库状态，不依赖授权服务器或官方域名。
    </p>
  </header>

  <v-card border flat :loading="bootstrap.isPending.value">
    <v-card-text v-if="bootstrap.data.value" class="pa-6">
      <div class="d-flex align-center ga-4 mb-5">
        <v-icon
          :icon="bootstrap.data.value.data.initialized ? 'mdi-check-circle' : 'mdi-progress-wrench'"
          :color="bootstrap.data.value.data.initialized ? 'success' : 'warning'"
          size="36"
        />
        <div>
          <div class="text-h6">
            {{ bootstrap.data.value.data.initialized ? '实例已初始化' : '等待首次初始化' }}
          </div>
          <div class="text-body-2 text-medium-emphasis">
            {{
              bootstrap.data.value.data.initialized
                ? '首次初始化已关闭，重复请求会被 Master 拒绝。'
                : '提交后会在一个事务中完成当前数据库所需的初始化写入。'
            }}
          </div>
        </div>
      </div>
      <v-alert v-if="bootstrap.data.value.data.initialized" type="success" variant="tonal">
        控制面初始化锁已关闭。空库会创建实例、Owner 与默认设置；历史库会保留已有资源并补齐缺失的初始化记录。登录与会话端点尚未启用。
      </v-alert>

      <v-form v-else @submit.prevent="submit">
        <v-text-field
          v-model="form.setupToken"
          label="一次性 Setup Token"
          type="password"
          autocomplete="off"
          minlength="64"
          maxlength="64"
          hint="读取部署时创建的 setup-token 文件；64 位小写十六进制，启动后默认 30 分钟失效"
          persistent-hint
          :error-messages="setupTokenErrors"
          required
        />
        <v-text-field
          v-model="form.instanceName"
          label="实例名称"
          autocomplete="organization"
          :hint="`1–80 个 Unicode 标量值；当前 ${instanceNameScalarCount}`"
          persistent-hint
          :error-messages="instanceNameErrors"
          required
        />
        <v-text-field
          v-model="form.username"
          label="Owner 用户名"
          autocomplete="username"
          minlength="3"
          maxlength="32"
          hint="3–32 位英文、数字、下划线、连字符或点"
          persistent-hint
          :error-messages="usernameErrors"
          required
        />
        <v-text-field
          v-model="form.password"
          label="Owner 密码"
          type="password"
          autocomplete="new-password"
          :hint="passwordHint"
          persistent-hint
          :error-messages="passwordErrors"
          required
        />
        <v-text-field
          v-model="form.passwordConfirmation"
          label="确认 Owner 密码"
          type="password"
          autocomplete="new-password"
          :error-messages="
            form.passwordConfirmation.length > 0 && !passwordConfirmationIsValid
              ? ['两次输入的密码不一致']
              : []
          "
          required
        />
        <v-alert
          v-if="lastFailure"
          type="error"
          variant="tonal"
          class="mb-4"
          data-testid="initialization-error"
        >
          <div>{{ lastFailure.summary }}</div>
          <div v-if="lastFailure.code" class="text-caption mt-1">
            错误代码：{{ lastFailure.code }}
          </div>
        </v-alert>
        <v-btn
          type="submit"
          color="primary"
          size="large"
          :loading="initialize.isPending.value"
          :disabled="!canSubmit || initialize.isPending.value"
        >
          完成控制面初始化
        </v-btn>
      </v-form>
    </v-card-text>
    <v-alert v-else-if="bootstrap.isError.value" type="error" variant="tonal" class="ma-6">
      无法读取初始化状态；请检查 Master readiness 与数据库迁移。
    </v-alert>
  </v-card>
</template>
