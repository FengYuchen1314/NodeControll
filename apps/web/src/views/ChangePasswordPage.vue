<script setup lang="ts">
import { computed, reactive, ref } from 'vue'
import { isNavigationFailure, useRoute, useRouter } from 'vue-router'

import { safeRedirectPath } from '../router'
import {
  PasswordChangeFailure,
  type PasswordChangeFailureReason,
  useSessionStore,
} from '../stores/session'

const route = useRoute()
const router = useRouter()
const session = useSessionStore()

const form = reactive({ confirmation: '', newPassword: '' })
const failure = ref<PasswordChangeFailureReason>()
const accepted = ref(false)
const outcomeUnknown = ref(false)
const navigationFailed = ref(false)

const utf8Encoder = new globalThis.TextEncoder()
const scalarLength = computed(() => Array.from(form.newPassword).length)
const byteLength = computed(() => utf8Encoder.encode(form.newPassword).byteLength)
const passwordShapeValid = computed(() => scalarLength.value >= 12 && byteLength.value <= 1_024)
const confirmationMatches = computed(
  () => form.confirmation.length > 0 && form.confirmation === form.newPassword,
)
const canSubmit = computed(
  () =>
    passwordShapeValid.value &&
    confirmationMatches.value &&
    session.isAuthenticated &&
    !session.passwordChangePending &&
    !accepted.value &&
    !outcomeUnknown.value,
)

const failureMessage = computed(() => {
  switch (failure.value) {
    case 'password-policy':
      return '新密码未通过服务端密码策略，请换一个更长且不易猜测的密码。'
    case 'password-unchanged':
      return '新密码必须与当前密码不同。'
    case 'rate-limited':
      return '密码处理容量暂时繁忙，请稍后再试。'
    case 'recent-auth-required':
      return '近期身份确认已经过期，需要先再次确认身份。'
    case 'csrf-unavailable':
      return '当前页面缺少有效的安全校验 Cookie，请刷新页面后重试。'
    case 'request-rejected':
      return '此修改密码请求已被安全策略拒绝，请确认当前访问地址。'
    case 'session-invalid':
      return '当前会话已经失效，请重新登录。'
    case 'unavailable':
      return '暂时无法修改密码，请检查网络与控制面状态后重试。'
    case 'outcome-unknown':
      return '请求传输结果无法确认。为避免重复修改，表单已经锁定且不会自动重试。'
    default:
      return ''
  }
})

const clearPasswords = () => {
  form.newPassword = ''
  form.confirmation = ''
}

const reauthenticationLocation = () => ({
  name: 'reauth',
  query: { redirect: safeRedirectPath(route.fullPath) },
})

const requireFreshAuthentication = async () => {
  navigationFailed.value = false
  clearPasswords()
  try {
    const result = await router.replace(reauthenticationLocation())
    if (isNavigationFailure(result)) navigationFailed.value = true
  } catch {
    navigationFailed.value = true
  }
}

const destination = () => {
  const redirect = safeRedirectPath(route.query.redirect)
  return redirect === '/profile/security/password' ? '/profile/security' : redirect
}

const navigateAfterAcceptance = async () => {
  navigationFailed.value = false
  try {
    const result = await router.replace(destination())
    if (isNavigationFailure(result)) navigationFailed.value = true
  } catch {
    navigationFailed.value = true
  }
}

const submit = async () => {
  if (!canSubmit.value) return
  session.syncRecentAuthClock()
  if (!session.recentAuthValid) {
    failure.value = 'recent-auth-required'
    await requireFreshAuthentication()
    return
  }

  failure.value = undefined
  navigationFailed.value = false
  const newPassword = form.newPassword
  let succeeded = false
  try {
    await session.changePassword(newPassword)
    accepted.value = true
    succeeded = true
  } catch (error) {
    const passwordFailure =
      error instanceof PasswordChangeFailure ? error : new PasswordChangeFailure('outcome-unknown')
    failure.value = passwordFailure.reason
    if (passwordFailure.reason === 'outcome-unknown') {
      outcomeUnknown.value = true
    }
  } finally {
    clearPasswords()
  }

  if (succeeded) {
    await navigateAfterAcceptance()
    return
  }
  if (failure.value === 'recent-auth-required') {
    await requireFreshAuthentication()
    return
  }
}
</script>

<template>
  <div class="security-form-page">
    <div class="page-lead mb-7">
      <p class="text-overline text-primary mb-2">PASSWORD SECURITY</p>
      <h1 class="text-h4 font-weight-bold mb-2">修改密码</h1>
      <p class="text-body-2 text-medium-emphasis mb-0">
        修改成功后，其他会话会被撤销，当前浏览器会收到一组新的会话凭据。
      </p>
    </div>

    <v-alert
      v-if="session.passwordChangeRequired"
      class="mb-5 security-form-card"
      type="warning"
      variant="tonal"
      role="alert"
    >
      管理员要求你先修改密码。完成前，控制台的普通功能保持关闭。
    </v-alert>

    <v-card class="security-form-card" border flat>
      <v-card-text class="pa-6 pa-sm-8">
        <v-form aria-label="修改密码" @submit.prevent="submit">
          <v-text-field
            v-model="form.newPassword"
            label="新密码"
            type="password"
            autocomplete="new-password"
            maxlength="1024"
            prepend-inner-icon="mdi-lock-reset"
            hint="至少 12 个字符；服务端会执行最终策略检查。"
            persistent-hint
            :disabled="accepted || outcomeUnknown"
            required
          />
          <v-text-field
            v-model="form.confirmation"
            label="确认新密码"
            type="password"
            autocomplete="new-password"
            maxlength="1024"
            prepend-inner-icon="mdi-lock-check-outline"
            :error-messages="
              form.confirmation.length > 0 && !confirmationMatches ? ['两次输入的密码不一致。'] : []
            "
            :disabled="accepted || outcomeUnknown"
            required
          />

          <v-alert
            v-if="failure"
            class="mb-5"
            :type="outcomeUnknown ? 'warning' : 'error'"
            variant="tonal"
            role="alert"
            data-testid="password-change-error"
          >
            {{ failureMessage }}
            <p v-if="outcomeUnknown" class="mb-0 mt-2">
              系统不会按 session ID 猜测结果，也不会重发修改请求。当前浏览器会尝试安全退出；请重新登录并用预期密码确认。
            </p>
          </v-alert>

          <v-alert
            v-if="navigationFailed"
            class="mb-5"
            type="warning"
            variant="tonal"
            role="alert"
            data-testid="password-navigation-error"
          >
            页面跳转失败。敏感字段已经清空，修改请求不会自动重放。
            <div v-if="accepted" class="mt-3">
              <v-btn type="button" size="small" variant="outlined" @click="navigateAfterAcceptance">
                重试页面跳转
              </v-btn>
            </div>
            <div v-else-if="failure === 'recent-auth-required'" class="mt-3">
              <v-btn
                type="button"
                size="small"
                variant="outlined"
                @click="requireFreshAuthentication"
              >
                重试前往身份确认
              </v-btn>
            </div>
          </v-alert>

          <v-btn
            type="submit"
            color="primary"
            size="large"
            block
            :disabled="!canSubmit"
            :loading="session.passwordChangePending"
          >
            修改密码并轮换会话
          </v-btn>
        </v-form>
      </v-card-text>
    </v-card>
  </div>
</template>
