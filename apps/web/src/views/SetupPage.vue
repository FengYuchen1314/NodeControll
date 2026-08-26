<script setup lang="ts">
import { useMutation, useQuery } from '@tanstack/vue-query'
import { computed, nextTick, onBeforeUnmount, reactive, ref } from 'vue'
import { isNavigationFailure, useRouter } from 'vue-router'

import { getBootstrapState } from '../api/generated/sdk.gen'
import { initializeControlPlaneWithRecoveryCodes } from '../api/recovery-codes'
import OneTimeRecoveryCodes from '../components/security/OneTimeRecoveryCodes.vue'
import { useSessionStore } from '../stores/session'

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

const nonCommittingBootstrapFailures = new Map<SafeProblemCode, number>([
  ['BOOTSTRAP_JSON_INVALID', 400],
  ['VALIDATION_FAILED', 400],
  ['SETUP_CAPABILITY_INVALID', 403],
  ['IDENTITY_CONFLICT', 409],
  ['PAYLOAD_TOO_LARGE', 413],
  ['UNSUPPORTED_MEDIA_TYPE', 415],
  ['BOOTSTRAP_JSON_SHAPE_INVALID', 422],
  ['BOOTSTRAP_RATE_LIMITED', 429],
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
    readonly outcomeUnknown = false,
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

const exactKeys = (candidate: Record<string, unknown>, expected: readonly string[]) => {
  const actual = Object.keys(candidate).sort()
  const sortedExpected = [...expected].sort()
  return (
    actual.length === sortedExpected.length &&
    actual.every((key, index) => key === sortedExpected[index])
  )
}

const boundedProblemText = (value: unknown, maximumLength: number): value is string =>
  typeof value === 'string' && value.length >= 1 && value.length <= maximumLength

const validProblemFieldError = (value: unknown) => {
  const fieldError = asRecord(value)
  return (
    fieldError !== undefined &&
    exactKeys(fieldError, ['code', 'message', 'pointer']) &&
    boundedProblemText(fieldError.code, 128) &&
    boundedProblemText(fieldError.message, 1_024) &&
    boundedProblemText(fieldError.pointer, 256)
  )
}

const responseHasProblemContentType = (response: HttpResponseLike) =>
  response.headers.get('content-type')?.split(';', 1)[0]?.trim().toLowerCase() ===
  'application/problem+json'

const validatedProblem = (value: unknown, response: HttpResponseLike) => {
  const problem = asRecord(value)
  const expectedKeys = problem?.errors === undefined
    ? ['code', 'detail', 'request_id', 'status', 'title', 'type']
    : ['code', 'detail', 'errors', 'request_id', 'status', 'title', 'type']
  if (
    problem === undefined ||
    !responseHasProblemContentType(response) ||
    !exactKeys(problem, expectedKeys) ||
    problem.status !== response.status ||
    !Number.isInteger(problem.status) ||
    response.status < 400 ||
    response.status >= 600 ||
    !boundedProblemText(problem.type, 512) ||
    !boundedProblemText(problem.title, 512) ||
    !boundedProblemText(problem.detail, 4_096) ||
    !boundedProblemText(problem.request_id, 128) ||
    !boundedProblemText(problem.code, 128) ||
    (problem.errors !== undefined &&
      (!Array.isArray(problem.errors) ||
        problem.errors.length > 64 ||
        !problem.errors.every(validProblemFieldError)))
  ) {
    return undefined
  }
  return problem
}

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
  if (code === 'IDENTITY_CONFLICT') {
    return 'Owner 用户名与数据库中的现有身份冲突。请更换用户名，或由部署管理员检查待初始化数据。'
  }
  if (status === 409 || code === 'ALREADY_INITIALIZED') {
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

const toInitializationFailure = (
  error: unknown,
  response?: HttpResponseLike,
  outcomeUnknown = false,
) => {
  const problem = asRecord(error)
  const status = safeStatus(problem, response)
  const code = safeProblemCode(problem)
  const fieldErrors = extractFieldErrors(problem)
  if (code === 'IDENTITY_CONFLICT' && fieldErrors.username.length === 0) {
    fieldErrors.username.push('此 Owner 用户名与数据库中的现有身份冲突。')
  }
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
    outcomeUnknown,
  )
}

const isProvablyNonCommittingFailure = (failure: InitializationFailure) =>
  failure.code !== undefined &&
  nonCommittingBootstrapFailures.get(failure.code) === failure.status

const isKnownInitializedFailure = (failure: InitializationFailure) =>
  failure.code === 'ALREADY_INITIALIZED' && failure.status === 409

const unknownInitializationFailure = (status?: number) =>
  new InitializationFailure(
    '初始化请求的结果无法确认。页面不会自动重放请求，请重新读取 Master 状态。',
    emptyFieldErrors(),
    status,
    undefined,
    true,
  )

const form = reactive({
  instanceName: '',
  username: '',
  password: '',
  passwordConfirmation: '',
  setupToken: '',
})
const router = useRouter()
const session = useSessionStore()

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
const createdAwaitingReconcile = ref(false)
const oneTimeRecoveryCodes = ref<string[]>([])
const recoveryCodesReconciled = ref(false)
const recoveryCodesUnavailable = ref(false)
const recoveryCodeDownloadFailed = ref(false)
const recoveryCodeNavigationFailed = ref(false)
let setupPageActive = true
const setupTokenErrors = computed(() =>
  lastFailure.value?.code === 'SETUP_CAPABILITY_INVALID'
    ? ['请填写部署服务器当前 setup-token 文件中的一次性 Token。']
    : [],
)
const instanceNameErrors = computed(() => lastFailure.value?.fieldErrors.instanceName ?? [])
const usernameErrors = computed(() => lastFailure.value?.fieldErrors.username ?? [])
const passwordErrors = computed(() => lastFailure.value?.fieldErrors.password ?? [])

const bootstrap = useQuery({
  queryKey: ['bootstrap'],
  queryFn: async () => {
    const response = await getBootstrapState({ credentials: 'same-origin' })
    if (response.error) throw new Error('Unable to load bootstrap state')
    return response.data
  },
})

const reconcileBootstrap = async () => {
  const result = await bootstrap.refetch()
  if (result.data?.data.initialized !== true) return
  recoveryCodesReconciled.value = true
  session.markInitialized()
  if (oneTimeRecoveryCodes.value.length > 0 || recoveryCodesUnavailable.value) return
  await router.replace({ name: 'login' })
}

const initialize = useMutation({
  mutationFn: async () => {
    let response
    try {
      response = await initializeControlPlaneWithRecoveryCodes({
        setupToken: form.setupToken,
        body: {
          instance_name: form.instanceName,
          username: form.username,
          password: form.password,
        },
        signal: AbortSignal.timeout(15_000),
      })
    } catch {
      throw unknownInitializationFailure()
    }
    if (response.data === undefined) {
      if (response.response.status === 201) {
        throw new InitializationFailure(
          'Master 已接受初始化请求，但恢复码响应未通过安全校验。请登录后在账户安全页重新生成。',
          emptyFieldErrors(),
          response.response.status,
          undefined,
          true,
        )
      }
      if (response.payloadState === 'valid-json') {
        const problem = validatedProblem(response.error, response.response)
        if (problem) {
          const failure = toInitializationFailure(problem, response.response)
          if (isProvablyNonCommittingFailure(failure) || isKnownInitializedFailure(failure)) {
            throw failure
          }
        }
      }
      throw unknownInitializationFailure(response.response.status)
    }
    form.password = ''
    form.passwordConfirmation = ''
    form.setupToken = ''
    const receivedCodes = response.data.data.one_time_recovery_codes
    const displayCodes = [...receivedCodes]
    for (let index = 0; index < receivedCodes.length; index += 1) receivedCodes[index] = ''
    await nextTick()
    if (!setupPageActive) {
      for (let index = 0; index < displayCodes.length; index += 1) displayCodes[index] = ''
      return
    }
    oneTimeRecoveryCodes.value = displayCodes
  },
  onMutate: () => {
    lastFailure.value = undefined
  },
  onSuccess: async () => {
    if (!setupPageActive) return
    lastFailure.value = undefined
    recoveryCodesReconciled.value = false
    recoveryCodesUnavailable.value = false
    recoveryCodeDownloadFailed.value = false
    recoveryCodeNavigationFailed.value = false
    await nextTick()
    createdAwaitingReconcile.value = true
    await reconcileBootstrap()
  },
  onError: async (error) => {
    form.password = ''
    form.passwordConfirmation = ''
    form.setupToken = ''
    if (!setupPageActive) return
    lastFailure.value =
      error instanceof InitializationFailure ? error : unknownInitializationFailure()
    if (lastFailure.value.outcomeUnknown) {
      createdAwaitingReconcile.value = true
      recoveryCodesUnavailable.value = true
      await reconcileBootstrap()
      return
    }
    if (
      lastFailure.value.code === 'ALREADY_INITIALIZED' ||
      (lastFailure.value.status === 409 && lastFailure.value.code !== 'IDENTITY_CONFLICT')
    ) {
      createdAwaitingReconcile.value = true
      await reconcileBootstrap()
    }
  },
})

const bootstrapInitialized = computed(() => bootstrap.data.value?.data.initialized === true)
const setupLocked = computed(() => bootstrapInitialized.value || createdAwaitingReconcile.value)

const retryBootstrap = async () => {
  await reconcileBootstrap()
}

const clearOneTimeRecoveryCodes = () => {
  for (let index = 0; index < oneTimeRecoveryCodes.value.length; index += 1) {
    oneTimeRecoveryCodes.value[index] = ''
  }
  oneTimeRecoveryCodes.value = []
}

const continueToLogin = async () => {
  recoveryCodeNavigationFailed.value = false
  clearOneTimeRecoveryCodes()
  recoveryCodesUnavailable.value = false
  await nextTick()
  try {
    const result = await router.replace({ name: 'login' })
    if (isNavigationFailure(result)) recoveryCodeNavigationFailed.value = true
  } catch {
    recoveryCodeNavigationFailed.value = true
  }
}

onBeforeUnmount(() => {
  setupPageActive = false
  clearOneTimeRecoveryCodes()
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

  <v-alert
    v-if="recoveryCodeNavigationFailed"
    type="warning"
    variant="tonal"
    class="mb-5"
    role="alert"
  >
    恢复码已经从页面清除，但登录页跳转失败。请手动打开登录页。
  </v-alert>

  <div v-if="oneTimeRecoveryCodes.length > 0">
    <v-alert
      v-if="recoveryCodeDownloadFailed"
      type="error"
      variant="tonal"
      class="mb-5"
      role="alert"
    >
      浏览器未能创建下载文件。请直接从页面抄录恢复码，并保存到受保护位置。
    </v-alert>
    <v-alert
      v-if="!recoveryCodesReconciled"
      type="warning"
      variant="tonal"
      class="mb-5"
      data-testid="setup-reconcile-lock"
    >
      <div>恢复码已在此页面显示，但 Master 的最终初始化状态尚未确认。请先保存恢复码，再重新读取状态。</div>
      <v-btn
        class="mt-4"
        variant="outlined"
        :loading="bootstrap.isFetching.value"
        @click="retryBootstrap"
      >
        重新读取状态
      </v-btn>
    </v-alert>
    <OneTimeRecoveryCodes
      :codes="oneTimeRecoveryCodes"
      context="bootstrap"
      :confirmation-ready="recoveryCodesReconciled"
      @download-failed="recoveryCodeDownloadFailed = true"
      @confirmed="continueToLogin"
    />
  </div>

  <v-card v-else-if="recoveryCodesUnavailable && recoveryCodesReconciled" border flat>
    <v-card-item prepend-icon="mdi-alert-octagon-outline">
      <v-card-title>初始化完成，但未接收恢复码</v-card-title>
      <v-card-subtitle>安全响应校验未通过</v-card-subtitle>
    </v-card-item>
    <v-card-text>
      <v-alert type="warning" variant="tonal" class="mb-5">
        页面没有展示、记录或保存服务端返回的无效恢复码响应。请使用 Owner 密码登录，并立即到账户安全页重新生成恢复码。
      </v-alert>
      <v-btn color="primary" variant="flat" @click="continueToLogin"> 前往登录 </v-btn>
    </v-card-text>
  </v-card>

  <v-card v-else border flat :loading="bootstrap.isPending.value">
    <v-card-text v-if="bootstrap.data.value" class="pa-6">
      <div class="d-flex align-center ga-4 mb-5">
        <v-icon
          :icon="bootstrapInitialized ? 'mdi-check-circle' : 'mdi-progress-wrench'"
          :color="bootstrapInitialized ? 'success' : 'warning'"
          size="36"
        />
        <div>
          <div class="text-h6">
            {{
              bootstrapInitialized
                ? '实例已初始化'
                : createdAwaitingReconcile
                  ? '初始化写入已被接受'
                  : '等待首次初始化'
            }}
          </div>
          <div class="text-body-2 text-medium-emphasis">
            {{
              bootstrapInitialized
                ? '首次初始化已关闭，重复请求会被 Master 拒绝。'
                : createdAwaitingReconcile
                  ? '为避免重复提交，页面会保持锁定，直到重新读取到 Master 的最终状态。'
                : '提交后会在一个事务中完成当前数据库所需的初始化写入。'
            }}
          </div>
        </div>
      </div>
      <v-alert v-if="bootstrapInitialized" type="success" variant="tonal">
        控制面初始化锁已关闭，正在转到本地账户登录。
      </v-alert>

      <v-alert
        v-else-if="setupLocked"
        type="warning"
        variant="tonal"
        data-testid="setup-reconcile-lock"
      >
        <div>Master 已接受初始化写入，或返回了无法安全重试的冲突。确认最新状态前，表单不会重新开放。</div>
        <div v-if="lastFailure" class="mt-2" data-testid="initialization-error">
          {{ lastFailure.summary }}
        </div>
        <v-btn
          class="mt-4"
          variant="outlined"
          :loading="bootstrap.isFetching.value"
          @click="retryBootstrap"
        >
          重新读取状态
        </v-btn>
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
