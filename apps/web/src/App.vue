<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { isNavigationFailure, useRoute, useRouter } from 'vue-router'
import { useDisplay } from 'vuetify'

import { useSessionStore } from './stores/session'

const route = useRoute()
const router = useRouter()
const session = useSessionStore()
const { mobile } = useDisplay()

const drawer = ref(true)
const logoutErrorVisible = ref(false)
const guestLayout = computed(() => route.meta.guestOnly === true)
const protectedRouteDenied = computed(
  () => route.meta.requiresAuth === true && !session.isAuthenticated,
)
const passwordRestrictedRouteDenied = computed(
  () =>
    route.meta.requiresAuth === true &&
    session.passwordChangeRequired &&
    route.meta.allowDuringPasswordChange !== true,
)
const actorInitial = computed(() => session.actor?.username.trim().charAt(0).toUpperCase() || 'U')
const pageTitle = computed(() => route.meta.title ?? 'NodeControll')

watch(
  mobile,
  (isMobile) => {
    drawer.value = !isMobile
  },
  { immediate: true },
)

const retrySession = async () => {
  await session.refresh()
  if (session.status === 'setup-required') {
    const result = await router.replace({ name: 'setup' })
    if (isNavigationFailure(result)) logoutErrorVisible.value = true
  } else if (session.status === 'anonymous') {
    const result = await router.replace({ name: 'login' })
    if (isNavigationFailure(result)) logoutErrorVisible.value = true
  } else if (session.status === 'authenticated' && guestLayout.value) {
    const result = await router.replace({ name: 'dashboard' })
    if (isNavigationFailure(result)) logoutErrorVisible.value = true
  }
}

const logout = async () => {
  logoutErrorVisible.value = false
  try {
    await session.logout()
    const result = await router.replace({ name: 'login' })
    if (isNavigationFailure(result)) logoutErrorVisible.value = true
  } catch {
    logoutErrorVisible.value = true
  }
}

const recoverProtectedRoute = async () => {
  logoutErrorVisible.value = false
  try {
    const result = await router.replace({
      name: session.status === 'setup-required' ? 'setup' : 'login',
    })
    if (isNavigationFailure(result)) logoutErrorVisible.value = true
  } catch {
    logoutErrorVisible.value = true
  }
}

const goToPasswordChange = async () => {
  logoutErrorVisible.value = false
  try {
    const result = await router.replace({
      name: 'password-change',
      query: route.meta.requiresAuth ? { redirect: route.fullPath } : undefined,
    })
    if (isNavigationFailure(result)) logoutErrorVisible.value = true
  } catch {
    logoutErrorVisible.value = true
  }
}
</script>

<template>
  <v-app>
    <div v-if="session.isResolving" class="session-gate" role="status" aria-live="polite">
      <v-progress-circular color="primary" indeterminate size="42" width="4" />
      <div class="text-body-2 text-medium-emphasis mt-4">正在确认实例与会话状态…</div>
    </div>

    <div
      v-else-if="session.hasResolutionError"
      class="session-gate px-5"
      data-testid="session-resolution-error-gate"
    >
      <v-card class="session-error-card" border flat role="alert">
        <v-card-text class="pa-7 text-center">
          <v-icon icon="mdi-cloud-alert-outline" color="error" size="44" class="mb-4" />
          <h1 class="text-h5 font-weight-bold mb-2">暂时无法连接控制面</h1>
          <p class="text-body-2 text-medium-emphasis mb-5">
            无法安全确认初始化或登录状态。请检查网络、反向代理和 Master readiness。
          </p>
          <v-btn color="primary" :loading="session.isResolving" @click="retrySession">
            重新连接
          </v-btn>
        </v-card-text>
      </v-card>
    </div>

    <div
      v-else-if="protectedRouteDenied"
      class="session-gate px-5"
      role="status"
      data-testid="protected-route-session-gate"
    >
      <v-card class="session-error-card" border flat>
        <v-card-text class="pa-7 text-center">
          <v-icon icon="mdi-lock-outline" color="primary" size="44" class="mb-4" />
          <h1 class="text-h5 font-weight-bold mb-2">受保护页面已关闭</h1>
          <p class="text-body-2 text-medium-emphasis mb-5">
            当前会话不能继续显示此页面。请返回登录页重新确认身份。
          </p>
          <v-btn color="primary" @click="recoverProtectedRoute">前往登录</v-btn>
        </v-card-text>
      </v-card>
    </div>

    <template v-else-if="guestLayout">
      <div class="guest-shell">
        <div class="guest-orb guest-orb-one" aria-hidden="true" />
        <div class="guest-orb guest-orb-two" aria-hidden="true" />
        <v-container class="guest-container">
          <header class="guest-header">
            <router-link to="/" class="guest-brand" aria-label="NodeControll 首页">
              <span class="brand-mark" aria-hidden="true">N</span>
              <span>
                <span class="d-block text-subtitle-1 font-weight-bold text-high-emphasis">
                  {{ $t('app.name') }}
                </span>
                <span class="d-block text-caption text-medium-emphasis">{{ $t('app.stage') }}</span>
              </span>
            </router-link>
          </header>
          <main class="guest-main">
            <router-view />
          </main>
          <footer class="guest-footer text-caption text-medium-emphasis">
            Self-hosted · Same-origin security boundary
          </footer>
        </v-container>
      </div>
    </template>

    <template v-else>
      <v-navigation-drawer v-model="drawer" :temporary="mobile" :permanent="!mobile" width="264">
        <div class="brand-lockup px-5 py-4">
          <span class="brand-mark" aria-hidden="true">N</span>
          <div class="ml-3">
            <div class="text-subtitle-1 font-weight-bold">{{ $t('app.name') }}</div>
            <div class="text-caption text-medium-emphasis">{{ $t('app.stage') }}</div>
          </div>
        </div>

        <v-divider />
        <v-list nav density="comfortable" class="px-3 py-4" aria-label="主导航">
          <template v-if="session.passwordChangeRequired">
            <v-list-item
              to="/profile/security/password"
              prepend-icon="mdi-lock-reset"
              :title="$t('nav.password')"
              value="password-change"
              rounded="lg"
            />
            <v-list-item
              to="/profile/security"
              prepend-icon="mdi-shield-account-outline"
              :title="$t('nav.security')"
              value="profile-security"
              rounded="lg"
            />
          </template>
          <template v-else>
            <v-list-item
              to="/"
              prepend-icon="mdi-view-dashboard-outline"
              :title="$t('nav.dashboard')"
              value="dashboard"
              rounded="lg"
            />
            <v-list-item
              to="/system"
              prepend-icon="mdi-cog-outline"
              :title="$t('nav.system')"
              value="system"
              rounded="lg"
            />
          </template>
        </v-list>

        <template #append>
          <div class="pa-3">
            <v-card color="surface-variant" flat>
              <v-card-text class="pa-3 d-flex align-center ga-3">
                <v-avatar color="primary" size="36" aria-hidden="true">
                  <span class="text-body-2 font-weight-bold">{{ actorInitial }}</span>
                </v-avatar>
                <div class="min-width-0">
                  <div class="text-body-2 font-weight-medium text-truncate">
                    {{ session.actor?.username }}
                  </div>
                  <div class="text-caption text-medium-emphasis text-truncate">
                    {{ session.actor?.role }}
                  </div>
                </div>
              </v-card-text>
            </v-card>
          </div>
        </template>
      </v-navigation-drawer>

      <v-app-bar flat border="b" height="64">
        <v-app-bar-nav-icon v-if="mobile" aria-label="打开主导航" @click="drawer = !drawer" />
        <v-app-bar-title class="text-subtitle-1 font-weight-semibold">{{
          pageTitle
        }}</v-app-bar-title>
        <template #append>
          <v-menu location="bottom end">
            <template #activator="{ props }">
              <v-btn v-bind="props" variant="text" class="account-button" aria-label="账户菜单">
                <v-avatar color="primary" size="32" class="mr-sm-2" aria-hidden="true">
                  <span class="text-caption font-weight-bold">{{ actorInitial }}</span>
                </v-avatar>
                <span class="account-label text-body-2">{{ session.actor?.username }}</span>
                <v-icon icon="mdi-chevron-down" size="18" class="ml-1" />
              </v-btn>
            </template>
            <v-list min-width="220" density="comfortable">
              <v-list-item
                to="/profile/security"
                prepend-icon="mdi-shield-account-outline"
                title="安全设置"
              />
              <v-divider />
              <v-list-item
                prepend-icon="mdi-logout"
                title="退出登录"
                :disabled="session.logoutPending"
                @click="logout"
              />
            </v-list>
          </v-menu>
        </template>
      </v-app-bar>

      <v-main>
        <v-container class="page-container py-6 py-md-8">
          <div
            v-if="passwordRestrictedRouteDenied"
            class="restricted-route-gate"
            role="alert"
            data-testid="password-restricted-route-gate"
          >
            <v-card class="session-error-card" border flat>
              <v-card-text class="pa-7 text-center">
                <v-icon icon="mdi-lock-reset" color="warning" size="44" class="mb-4" />
                <h1 class="text-h5 font-weight-bold mb-2">请先修改密码</h1>
                <p class="text-body-2 text-medium-emphasis mb-5">
                  普通控制台内容已经关闭。完成密码修改后才能继续访问。
                </p>
                <v-btn color="primary" @click="goToPasswordChange">前往修改密码</v-btn>
              </v-card-text>
            </v-card>
          </div>
          <router-view v-else />
        </v-container>
      </v-main>
    </template>

    <v-snackbar v-model="logoutErrorVisible" color="error" location="bottom end" timeout="6000">
      无法完成页面跳转。当前身份状态仍会保持关闭。
      <template #actions>
        <v-btn variant="text" @click="logoutErrorVisible = false">关闭</v-btn>
      </template>
    </v-snackbar>
  </v-app>
</template>
