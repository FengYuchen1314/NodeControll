<script setup lang="ts">
import { computed, ref } from 'vue'
import { isNavigationFailure, useRoute, useRouter } from 'vue-router'

import { routeCapabilityAllowed } from './router/access'
import { appRouteNames } from './router/route-names'
import AppShell from './shell/AppShell.vue'
import { useShellPreferences } from './shell/use-shell-preferences'
import { useSessionStore } from './stores/session'

const route = useRoute()
const router = useRouter()
const session = useSessionStore()
useShellPreferences()

const logoutErrorVisible = ref(false)
const guestLayout = computed(() => route.meta.guestOnly === true)
const protectedRouteDenied = computed(
  () => route.meta.requiresAuth === true && !session.isAuthenticated,
)
const passwordRestrictedRouteDenied = computed(
  () =>
    session.isAuthenticated &&
    session.passwordChangeRequired &&
    route.meta.allowDuringPasswordChange !== true,
)
const capabilityRestrictedRouteDenied = computed(
  () =>
    session.isAuthenticated &&
    route.meta.requiresAuth === true &&
    !routeCapabilityAllowed(route.meta, session.actor?.capabilities ?? []),
)
const retrySession = async () => {
  await session.refresh()
  if (session.status === 'setup-required') {
    const result = await router.replace({ name: appRouteNames.setup })
    if (isNavigationFailure(result)) logoutErrorVisible.value = true
  } else if (session.status === 'anonymous') {
    const result = await router.replace({ name: appRouteNames.login })
    if (isNavigationFailure(result)) logoutErrorVisible.value = true
  } else if (session.status === 'authenticated' && guestLayout.value) {
    const result = await router.replace({ name: appRouteNames.dashboard })
    if (isNavigationFailure(result)) logoutErrorVisible.value = true
  }
}

const logout = async () => {
  logoutErrorVisible.value = false
  try {
    await session.logout()
    const result = await router.replace({ name: appRouteNames.login })
    if (isNavigationFailure(result)) logoutErrorVisible.value = true
  } catch {
    logoutErrorVisible.value = true
  }
}

const recoverProtectedRoute = async () => {
  logoutErrorVisible.value = false
  try {
    const result = await router.replace({
      name:
        session.status === 'setup-required' ? appRouteNames.setup : appRouteNames.login,
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
      name: appRouteNames.passwordChange,
      query: route.meta.requiresAuth ? { redirect: route.fullPath } : undefined,
    })
    if (isNavigationFailure(result)) logoutErrorVisible.value = true
  } catch {
    logoutErrorVisible.value = true
  }
}

const recoverCapabilityRoute = async () => {
  logoutErrorVisible.value = false
  try {
    const result = await router.replace({ name: appRouteNames.dashboard })
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
      <div class="text-body-2 text-medium-emphasis mt-4">{{ $t('shell.resolvingSession') }}</div>
    </div>

    <div
      v-else-if="session.hasResolutionError"
      class="session-gate px-5"
      data-testid="session-resolution-error-gate"
    >
      <v-card class="session-error-card" border flat role="alert">
        <v-card-text class="pa-7 text-center">
          <v-icon icon="mdi-cloud-alert-outline" color="error" size="44" class="mb-4" />
          <h1 class="text-h5 font-weight-bold mb-2">{{ $t('shell.connectionUnavailable') }}</h1>
          <p class="text-body-2 text-medium-emphasis mb-5">{{ $t('shell.connectionUnavailableHelp') }}</p>
          <v-btn color="primary" :loading="session.isResolving" @click="retrySession">
            {{ $t('shell.retryConnection') }}
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
          <h1 class="text-h5 font-weight-bold mb-2">{{ $t('shell.protectedClosed') }}</h1>
          <p class="text-body-2 text-medium-emphasis mb-5">{{ $t('shell.protectedClosedHelp') }}</p>
          <v-btn color="primary" @click="recoverProtectedRoute">{{ $t('shell.goToLogin') }}</v-btn>
        </v-card-text>
      </v-card>
    </div>

    <div
      v-else-if="passwordRestrictedRouteDenied"
      class="session-gate px-5"
      role="alert"
      data-testid="password-restricted-route-gate"
    >
      <v-card class="session-error-card" border flat>
        <v-card-text class="pa-7 text-center">
          <v-icon icon="mdi-lock-reset" color="warning" size="44" class="mb-4" />
          <h1 class="text-h5 font-weight-bold mb-2">{{ $t('shell.passwordChangeRequired') }}</h1>
          <p class="text-body-2 text-medium-emphasis mb-5">
            {{ $t('shell.passwordChangeRequiredHelp') }}
          </p>
          <div class="restricted-route-actions">
            <v-btn color="primary" @click="goToPasswordChange">
              {{ $t('nav.password') }}
            </v-btn>
            <v-btn :to="{ name: appRouteNames.profileSecurity }" variant="text">
              {{ $t('nav.security') }}
            </v-btn>
          </div>
        </v-card-text>
      </v-card>
    </div>

    <div
      v-else-if="capabilityRestrictedRouteDenied"
      class="session-gate px-5"
      role="alert"
      data-testid="capability-restricted-route-gate"
    >
      <v-card class="session-error-card" border flat>
        <v-card-text class="pa-7 text-center">
          <v-icon icon="mdi-shield-lock-outline" color="warning" size="44" class="mb-4" />
          <h1 class="text-h5 font-weight-bold mb-2">{{ $t('shell.capabilityClosed') }}</h1>
          <p class="text-body-2 text-medium-emphasis mb-5">{{ $t('shell.capabilityClosedHelp') }}</p>
          <v-btn color="primary" @click="recoverCapabilityRoute">
            {{ $t('shell.goToDashboard') }}
          </v-btn>
        </v-card-text>
      </v-card>
    </div>

    <template v-else-if="guestLayout">
      <div class="guest-shell">
        <div class="guest-orb guest-orb-one" aria-hidden="true" />
        <div class="guest-orb guest-orb-two" aria-hidden="true" />
        <v-container class="guest-container">
          <header class="guest-header">
            <router-link
              :to="{ name: appRouteNames.dashboard }"
              class="guest-brand"
              :aria-label="$t('shell.homeLabel')"
            >
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
            {{ $t('shell.guestFooter') }}
          </footer>
        </v-container>
      </div>
    </template>

    <app-shell v-else :logout-pending="session.logoutPending" @logout="logout">
      <router-view />
    </app-shell>

    <v-snackbar v-model="logoutErrorVisible" color="error" location="bottom end" timeout="6000">
      {{ $t('shell.navigationFailure') }}
      <template #actions>
        <v-btn variant="text" @click="logoutErrorVisible = false">{{ $t('common.close') }}</v-btn>
      </template>
    </v-snackbar>
  </v-app>
</template>
