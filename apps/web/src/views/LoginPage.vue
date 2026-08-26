<script setup lang="ts">
import { computed, reactive, ref } from 'vue'
import { isNavigationFailure, useRoute, useRouter } from 'vue-router'

import { safeRedirectPath } from '../router'
import { LoginFailure, type LoginFailureReason, useSessionStore } from '../stores/session'

const route = useRoute()
const router = useRouter()
const session = useSessionStore()

const form = reactive({
  password: '',
  username: '',
})
const failure = ref<{ reason: LoginFailureReason; retryAfterSeconds?: number }>()
const navigationFailed = ref(false)

const canSubmit = computed(
  () =>
    form.username.trim().length > 0 &&
    form.password.length > 0 &&
    !session.loginPending &&
    !session.isAuthenticated,
)
const failureMessage = computed(() => {
  switch (failure.value?.reason) {
    case 'invalid-credentials':
      return '用户名或密码错误。'
    case 'rate-limited':
      return failure.value.retryAfterSeconds
        ? `登录尝试过于频繁，请在 ${failure.value.retryAfterSeconds} 秒后重试。`
        : '登录尝试过于频繁，请稍后重试。'
    case 'request-rejected':
      return '无法验证此登录请求，请确认正在使用管理员配置的访问地址。'
    case 'setup-required':
      return '实例尚未初始化，正在转到初始化页面。'
    case 'unavailable':
      return '暂时无法登录，请检查网络与 Master readiness 后重试。'
    default:
      return ''
  }
})

const submit = async () => {
  if (!canSubmit.value) return
  failure.value = undefined
  navigationFailed.value = false
  const password = form.password
  try {
    await session.login(form.username.trim(), password)
  } catch (error) {
    const loginFailure = error instanceof LoginFailure ? error : new LoginFailure('unavailable')
    failure.value = {
      reason: loginFailure.reason,
      retryAfterSeconds: loginFailure.retryAfterSeconds,
    }
    if (loginFailure.reason === 'setup-required') {
      try {
        const result = await router.replace({ name: 'setup' })
        if (isNavigationFailure(result)) navigationFailed.value = true
      } catch {
        navigationFailed.value = true
      }
    }
    return
  } finally {
    form.password = ''
  }

  try {
    const result = await router.replace(safeRedirectPath(route.query.redirect))
    if (isNavigationFailure(result)) navigationFailed.value = true
  } catch {
    navigationFailed.value = true
  }
}

const retryAuthenticatedNavigation = async () => {
  navigationFailed.value = false
  try {
    const result = await router.replace(safeRedirectPath(route.query.redirect))
    if (isNavigationFailure(result)) navigationFailed.value = true
  } catch {
    navigationFailed.value = true
  }
}
</script>

<template>
  <div class="login-page">
    <div class="mb-7">
      <p class="text-overline text-primary mb-2">CONTROL PLANE</p>
      <h1 class="text-h4 font-weight-bold mb-2">登录 NodeControll</h1>
      <p class="text-body-2 text-medium-emphasis mb-0">
        使用此实例的本地账户继续。凭据只发送到当前站点。
      </p>
    </div>

    <v-card class="login-card" border flat>
      <v-card-text class="pa-6 pa-sm-8">
        <v-form aria-label="登录" @submit.prevent="submit">
          <v-text-field
            v-model="form.username"
            label="用户名"
            autocomplete="username"
            autofocus
            maxlength="32"
            prepend-inner-icon="mdi-account-outline"
            required
          />
          <v-text-field
            v-model="form.password"
            label="密码"
            type="password"
            autocomplete="current-password"
            maxlength="1024"
            prepend-inner-icon="mdi-lock-outline"
            :aria-describedby="failure ? 'login-error' : undefined"
            required
          />

          <v-alert
            v-if="failure"
            id="login-error"
            class="mb-5"
            type="error"
            variant="tonal"
            role="alert"
            data-testid="login-error"
          >
            {{ failureMessage }}
          </v-alert>

          <v-alert
            v-if="navigationFailed"
            class="mb-5"
            type="warning"
            variant="tonal"
            role="alert"
            data-testid="login-navigation-error"
          >
            登录状态已经更新，但页面跳转失败。当前页面不会再次提交密码。
            <div class="mt-3">
              <v-btn
                type="button"
                size="small"
                variant="outlined"
                @click="retryAuthenticatedNavigation"
              >
                重试进入控制台
              </v-btn>
            </div>
          </v-alert>

          <v-btn
            type="submit"
            color="primary"
            size="large"
            block
            :disabled="!canSubmit"
            :loading="session.loginPending"
          >
            登录
          </v-btn>
        </v-form>
      </v-card-text>
    </v-card>

    <p class="text-caption text-medium-emphasis text-center mt-5 mb-0">
      会话使用 Secure、Host-only Cookie，不会保存到浏览器存储。
    </p>
  </div>
</template>
