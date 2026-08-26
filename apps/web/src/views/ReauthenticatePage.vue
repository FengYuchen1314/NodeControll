<script setup lang="ts">
import { computed, reactive, ref } from 'vue'
import { isNavigationFailure, useRoute, useRouter } from 'vue-router'

import { safeRedirectPath } from '../router'
import {
  ReauthenticationFailure,
  type ReauthenticationFailureReason,
  useSessionStore,
} from '../stores/session'

const route = useRoute()
const router = useRouter()
const session = useSessionStore()

const form = reactive({ password: '' })
const failure = ref<{
  reason: ReauthenticationFailureReason
  retryAfterSeconds?: number
}>()
const accepted = ref(false)
const navigationFailed = ref(false)

const canSubmit = computed(
  () =>
    form.password.length > 0 &&
    session.isAuthenticated &&
    !session.reauthenticationPending &&
    !accepted.value,
)

const failureMessage = computed(() => {
  switch (failure.value?.reason) {
    case 'invalid-proof':
      return '密码不正确，身份确认未通过。'
    case 'outcome-unknown':
      return '身份确认请求的结果无法确认。系统不会重放证明，并会尝试退出当前浏览器；请重新登录。'
    case 'rate-limited':
      return failure.value.retryAfterSeconds
        ? `尝试过于频繁，请在 ${failure.value.retryAfterSeconds} 秒后重试。`
        : '尝试过于频繁，请稍后重试。'
    case 'csrf-unavailable':
      return '当前页面缺少有效的安全校验 Cookie，请刷新页面后重试。'
    case 'request-rejected':
      return '此身份确认请求已被安全策略拒绝，请确认当前访问地址。'
    case 'session-invalid':
      return '当前会话已经失效，请重新登录。'
    case 'unavailable':
      return '暂时无法确认身份，请检查网络与控制面状态后重试。'
    default:
      return ''
  }
})

const destination = () => {
  const redirect = safeRedirectPath(route.query.redirect)
  return redirect === '/' && session.passwordChangeRequired
    ? '/profile/security/password'
    : redirect
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
  failure.value = undefined
  navigationFailed.value = false
  const password = form.password
  let succeeded = false
  try {
    await session.reauthenticate(password)
    succeeded = true
    accepted.value = true
  } catch (error) {
    const reauthenticationFailure =
      error instanceof ReauthenticationFailure ? error : new ReauthenticationFailure('unavailable')
    failure.value = {
      reason: reauthenticationFailure.reason,
      retryAfterSeconds: reauthenticationFailure.retryAfterSeconds,
    }
  } finally {
    form.password = ''
  }

  if (succeeded) await navigateAfterAcceptance()
}
</script>

<template>
  <div class="security-form-page">
    <div class="page-lead mb-7">
      <p class="text-overline text-primary mb-2">RECENT AUTHENTICATION</p>
      <h1 class="text-h4 font-weight-bold mb-2">再次确认身份</h1>
      <p class="text-body-2 text-medium-emphasis mb-0">
        敏感操作需要近期身份确认。密码仅用于这一次校验，成功后会轮换当前会话凭据。
      </p>
    </div>

    <v-card class="security-form-card" border flat>
      <v-card-text class="pa-6 pa-sm-8">
        <v-form aria-label="再次确认身份" @submit.prevent="submit">
          <v-text-field
            v-model="form.password"
            label="当前密码"
            type="password"
            autocomplete="current-password"
            maxlength="1024"
            prepend-inner-icon="mdi-lock-check-outline"
            :aria-describedby="failure ? 'reauth-error' : undefined"
            autofocus
            required
          />

          <v-alert
            v-if="failure"
            id="reauth-error"
            class="mb-5"
            type="error"
            variant="tonal"
            role="alert"
            data-testid="reauth-error"
          >
            {{ failureMessage }}
          </v-alert>

          <v-alert
            v-if="navigationFailed"
            class="mb-5"
            type="warning"
            variant="tonal"
            role="alert"
            data-testid="reauth-navigation-error"
          >
            身份状态已经更新，但页面跳转失败。此页面不会再次提交密码。
            <div class="mt-3">
              <v-btn type="button" size="small" variant="outlined" @click="navigateAfterAcceptance">
                重试页面跳转
              </v-btn>
            </div>
          </v-alert>

          <v-btn
            type="submit"
            color="primary"
            size="large"
            block
            :disabled="!canSubmit"
            :loading="session.reauthenticationPending"
          >
            确认身份
          </v-btn>
        </v-form>
      </v-card-text>
    </v-card>
  </div>
</template>
