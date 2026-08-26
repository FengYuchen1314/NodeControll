<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { isNavigationFailure, onBeforeRouteLeave, useRoute, useRouter } from 'vue-router'

import type { UserSessionResponse } from '../api/generated/types.gen'
import type { RecoveryCodeStatus } from '../api/recovery-codes'
import OneTimeRecoveryCodes from '../components/security/OneTimeRecoveryCodes.vue'
import { safeRedirectPath } from '../router'
import { useOneTimeRecoveryCodeStore } from '../stores/one-time-recovery'
import {
  RecoveryCodeFailure,
  SessionManagementFailure,
  type RecoveryCodeFailureReason,
  type SessionManagementFailureReason,
  useSessionStore,
} from '../stores/session'

const route = useRoute()
const router = useRouter()
const session = useSessionStore()
const oneTimeRecoveryCodes = useOneTimeRecoveryCodeStore()

const sessions = computed(() => session.managedSessions)
const failure = ref<SessionManagementFailureReason | RecoveryCodeFailureReason>()
const navigationFailed = ref(false)
const revokeCandidate = ref<UserSessionResponse>()
const revokeDialog = ref(false)
const logoutOthersDialog = ref(false)
const logoutEverywhereDialog = ref(false)
const revokedOne = ref(false)
const revokedOthersCount = ref<number>()
const signedOutEverywhere = ref(false)
const recoveryCodeStatus = ref<RecoveryCodeStatus>()
const recoveryCodeRegenerationDialog = ref(false)
const recoveryCodeDownloadFailed = ref(false)
const recoveryCodesSaved = ref(false)

const clearCodeArray = (codes: string[]) => {
  for (let index = 0; index < codes.length; index += 1) codes[index] = ''
}

const abandonRecoveryCodeOwnership = () => {
  oneTimeRecoveryCodes.abandonOperation()
}

const otherSessionCount = computed(
  () => sessions.value.filter((candidate) => !candidate.is_current).length,
)
const operationPending = computed(
  () =>
    session.logoutAllPending ||
    session.revokingSessionIds.length > 0 ||
    session.recoveryCodeRegenerationPending,
)

const formatTimestamp = (value: number) =>
  new Intl.DateTimeFormat('zh-CN', {
    dateStyle: 'medium',
    timeStyle: 'short',
  }).format(new Date(value))

const authLevelLabel = (value: string) => {
  switch (value) {
    case 'password':
      return '密码'
    case 'mfa':
      return '多因素认证'
    case 'phishing_resistant':
      return '抗钓鱼认证'
    case 'recovery':
      return '恢复码'
    default:
      return '已认证'
  }
}

const failureMessage = computed(() => {
  switch (failure.value) {
    case 'csrf-unavailable':
      return '当前页面缺少有效的安全校验 Cookie，请刷新页面后重试。'
    case 'recent-auth-required':
      return '近期身份确认已经过期，请先再次确认身份。'
    case 'request-rejected':
      return '这项安全操作已被策略拒绝，请确认当前访问地址。'
    case 'session-invalid':
      return '当前会话已经失效，请重新登录。'
    case 'outcome-unknown':
      return '请求结果无法确认。系统不会自动重放这项会话操作，请重新登录后核对。'
    case 'unavailable':
      return '暂时无法读取或更新会话，请检查网络与控制面状态后重试。'
    default:
      return ''
  }
})

const loadSessions = async () => {
  failure.value = undefined
  try {
    await session.listSessions()
  } catch (error) {
    const sessionFailure =
      error instanceof SessionManagementFailure
        ? error
        : new SessionManagementFailure('unavailable')
    failure.value = sessionFailure.reason
  }
}

const loadRecoveryCodeStatus = async () => {
  failure.value = undefined
  try {
    recoveryCodeStatus.value = await session.getRecoveryCodeStatus()
  } catch (error) {
    recoveryCodeStatus.value = undefined
    const recoveryFailure =
      error instanceof RecoveryCodeFailure ? error : new RecoveryCodeFailure('unavailable')
    failure.value = recoveryFailure.reason
  }
}

const goToReauthentication = async () => {
  navigationFailed.value = false
  try {
    const result = await router.replace({
      name: 'reauth',
      query: { redirect: safeRedirectPath(route.fullPath) },
    })
    if (isNavigationFailure(result)) navigationFailed.value = true
  } catch {
    navigationFailed.value = true
  }
}

const recentAuthenticationIsReady = async () => {
  session.syncRecentAuthClock()
  if (session.recentAuthValid) return true
  failure.value = 'recent-auth-required'
  await goToReauthentication()
  return false
}

const acceptOperationFailure = async (error: unknown) => {
  const sessionFailure =
    error instanceof SessionManagementFailure || error instanceof RecoveryCodeFailure
      ? error
      : new SessionManagementFailure('outcome-unknown')
  failure.value = sessionFailure.reason
  if (sessionFailure.reason === 'recent-auth-required') await goToReauthentication()
}

const openRecoveryCodeRegenerationDialog = async () => {
  recoveryCodesSaved.value = false
  recoveryCodeDownloadFailed.value = false
  if (!(await recentAuthenticationIsReady())) return
  recoveryCodeRegenerationDialog.value = true
}

const regenerateRecoveryCodeSet = async () => {
  recoveryCodeRegenerationDialog.value = false
  const operationOwner = oneTimeRecoveryCodes.beginOperation()
  const operationRoute = router.currentRoute.value
  const stillOwnsOperation = () =>
    oneTimeRecoveryCodes.ownsOperation(operationOwner) &&
    router.currentRoute.value === operationRoute
  if (
    !(await recentAuthenticationIsReady()) ||
    !stillOwnsOperation()
  ) {
    oneTimeRecoveryCodes.releaseOperation(operationOwner)
    return
  }

  failure.value = undefined
  try {
    const codes = await session.regenerateRecoveryCodes()
    if (!stillOwnsOperation()) {
      clearCodeArray(codes)
      return
    }
    let accepted = false
    try {
      accepted = oneTimeRecoveryCodes.acceptForOperation(operationOwner, codes)
    } finally {
      clearCodeArray(codes)
    }
    if (!accepted) {
      throw new RecoveryCodeFailure('outcome-unknown')
    }
    await loadRecoveryCodeStatus()
  } catch (error) {
    if (!stillOwnsOperation()) return
    await acceptOperationFailure(error)
  } finally {
    oneTimeRecoveryCodes.releaseOperation(operationOwner)
  }
}

const confirmRecoveryCodesSaved = () => {
  oneTimeRecoveryCodes.clear()
  recoveryCodeDownloadFailed.value = false
  recoveryCodesSaved.value = true
}

const openRevokeDialog = (candidate: UserSessionResponse) => {
  revokeCandidate.value = candidate
  revokeDialog.value = true
}

const revokeSelectedSession = async () => {
  const candidate = revokeCandidate.value
  revokeDialog.value = false
  revokeCandidate.value = undefined
  if (!candidate || candidate.is_current || !(await recentAuthenticationIsReady())) return

  failure.value = undefined
  revokedOne.value = false
  try {
    await session.revokeSession(candidate.id)
    revokedOne.value = true
  } catch (error) {
    await acceptOperationFailure(error)
  }
}

const logoutOtherSessions = async () => {
  logoutOthersDialog.value = false
  if (!(await recentAuthenticationIsReady())) return

  failure.value = undefined
  revokedOthersCount.value = undefined
  try {
    const revoked = await session.logoutAll(true)
    revokedOthersCount.value = revoked
    scheduleRecentAuthExpiration()
  } catch (error) {
    await acceptOperationFailure(error)
  }
}

const logoutEverywhere = async () => {
  logoutEverywhereDialog.value = false
  if (!(await recentAuthenticationIsReady())) return

  failure.value = undefined
  signedOutEverywhere.value = false
  try {
    await session.logoutAll(false)
    signedOutEverywhere.value = true
    const result = await router.replace({ name: 'login' })
    if (isNavigationFailure(result)) navigationFailed.value = true
  } catch (error) {
    await acceptOperationFailure(error)
  }
}

let recentAuthExpirationTimer: ReturnType<typeof globalThis.setTimeout> | undefined

const cancelRecentAuthExpirationTimer = () => {
  if (recentAuthExpirationTimer !== undefined) {
    globalThis.clearTimeout(recentAuthExpirationTimer)
    recentAuthExpirationTimer = undefined
  }
}

const scheduleRecentAuthExpiration = () => {
  cancelRecentAuthExpirationTimer()
  const deadline = session.session?.recent_auth_expires_at_ms
  if (deadline === undefined) return
  const delay = deadline - Date.now()
  if (delay <= 0) {
    session.syncRecentAuthClock()
    return
  }
  recentAuthExpirationTimer = globalThis.setTimeout(
    () => {
      recentAuthExpirationTimer = undefined
      session.syncRecentAuthClock()
    },
    Math.min(delay, 2_147_483_647),
  )
}

const syncRecentAuthAfterVisibilityChange = () => {
  if (globalThis.document.visibilityState !== 'visible') return
  session.syncRecentAuthClock()
  scheduleRecentAuthExpiration()
}

const clearRecoveryCodesForPageHide = () => {
  abandonRecoveryCodeOwnership()
}

onMounted(() => {
  session.syncRecentAuthClock()
  scheduleRecentAuthExpiration()
  globalThis.document.addEventListener('visibilitychange', syncRecentAuthAfterVisibilityChange)
  globalThis.addEventListener('pagehide', clearRecoveryCodesForPageHide)
  void loadSessions()
  void loadRecoveryCodeStatus()
})

onBeforeUnmount(() => {
  oneTimeRecoveryCodes.clear()
  cancelRecentAuthExpirationTimer()
  globalThis.document.removeEventListener('visibilitychange', syncRecentAuthAfterVisibilityChange)
  globalThis.removeEventListener('pagehide', clearRecoveryCodesForPageHide)
})

onBeforeRouteLeave(() => {
  abandonRecoveryCodeOwnership()
})

watch(
  () => route.fullPath,
  () => {
    abandonRecoveryCodeOwnership()
  },
)
</script>

<template>
  <div>
    <div class="d-flex flex-column flex-md-row align-md-start justify-space-between ga-4 mb-7">
      <div class="page-lead">
        <p class="text-overline text-primary mb-2">ACCOUNT SECURITY</p>
        <h1 class="text-h4 font-weight-bold mb-2">账户安全</h1>
        <p class="text-body-2 text-medium-emphasis mb-0">
          管理密码和服务端会话。这里只显示认证方式与时间，不暴露网络地址、浏览器标识或凭据材料。
        </p>
      </div>
      <v-chip
        :color="session.recentAuthValid ? 'success' : 'warning'"
        :prepend-icon="
          session.recentAuthValid ? 'mdi-shield-check-outline' : 'mdi-shield-alert-outline'
        "
        variant="tonal"
      >
        {{ session.recentAuthValid ? '近期身份已确认' : '敏感操作前需确认身份' }}
      </v-chip>
    </div>

    <v-alert
      v-if="failure"
      class="mb-5"
      type="error"
      variant="tonal"
      role="alert"
      data-testid="security-error"
    >
      {{ failureMessage }}
    </v-alert>
    <v-alert
      v-if="navigationFailed"
      class="mb-5"
      type="warning"
      variant="tonal"
      role="alert"
      data-testid="security-navigation-error"
    >
      页面跳转失败，刚才的敏感操作不会自动重放。
    </v-alert>
    <v-alert v-if="revokedOne" class="mb-5" type="success" variant="tonal" role="status">
      已撤销所选会话。
    </v-alert>
    <v-alert
      v-if="revokedOthersCount !== undefined"
      class="mb-5"
      type="success"
      variant="tonal"
      role="status"
      data-testid="logout-others-result"
    >
      已退出其他会话，共撤销 {{ revokedOthersCount }} 个会话；数量包含轮换前的当前会话。当前浏览器会话已轮换。
    </v-alert>
    <v-alert
      v-if="signedOutEverywhere && navigationFailed"
      class="mb-5"
      type="success"
      variant="tonal"
      role="status"
      data-testid="logout-everywhere-result"
    >
      所有会话均已退出。请手动前往登录页。
    </v-alert>
    <v-alert
      v-if="recoveryCodesSaved"
      class="mb-5"
      type="success"
      variant="tonal"
      role="status"
      data-testid="recovery-codes-saved"
    >
      新恢复码已确认保存，页面中的明文副本已清除。
    </v-alert>
    <v-alert
      v-if="recoveryCodeDownloadFailed"
      class="mb-5"
      type="error"
      variant="tonal"
      role="alert"
    >
      浏览器未能创建下载文件。请直接从窗口抄录恢复码，并保存到受保护位置。
    </v-alert>

    <v-row>
      <v-col cols="12" lg="4">
        <v-card border flat height="100%">
          <v-card-item prepend-icon="mdi-lock-outline">
            <v-card-title>密码</v-card-title>
            <v-card-subtitle>轮换账户密码和当前会话凭据</v-card-subtitle>
          </v-card-item>
          <v-card-text>
            <p class="text-body-2 text-medium-emphasis">
              修改密码会撤销其他登录位置。该操作需要近期身份确认。
            </p>
            <v-btn
              to="/profile/security/password"
              color="primary"
              variant="tonal"
              prepend-icon="mdi-lock-reset"
            >
              修改密码
            </v-btn>
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12" lg="8">
        <v-card border flat height="100%">
          <v-card-item prepend-icon="mdi-key-chain-variant">
            <template #append>
              <v-btn
                icon="mdi-refresh"
                variant="text"
                aria-label="刷新恢复码状态"
                :loading="session.recoveryCodeStatusPending"
                :disabled="operationPending"
                @click="loadRecoveryCodeStatus"
              />
            </template>
            <v-card-title>账户恢复码</v-card-title>
            <v-card-subtitle>离线保存的单次使用凭据</v-card-subtitle>
          </v-card-item>
          <v-card-text>
            <v-skeleton-loader
              v-if="session.recoveryCodeStatusPending && !recoveryCodeStatus"
              type="list-item-two-line"
              role="status"
            />
            <div v-else-if="recoveryCodeStatus" data-testid="recovery-code-status">
              <p class="text-h5 font-weight-bold mb-1">
                剩余 {{ recoveryCodeStatus.remaining_count }} / {{ recoveryCodeStatus.total_count }} 枚
              </p>
              <p class="text-body-2 text-medium-emphasis mb-4">
                第 {{ recoveryCodeStatus.set_version }} 组 · 创建于
                {{ formatTimestamp(recoveryCodeStatus.created_at_ms) }}
              </p>
            </div>
            <p v-else class="text-body-2 text-medium-emphasis mb-4">
              暂未读取到恢复码状态。状态接口只返回数量和创建时间，不返回明文。
            </p>
            <v-btn
              color="warning"
              variant="tonal"
              prepend-icon="mdi-autorenew"
              :loading="session.recoveryCodeRegenerationPending"
              :disabled="operationPending"
              data-testid="regenerate-recovery-codes"
              @click="openRecoveryCodeRegenerationDialog"
            >
              重新生成恢复码
            </v-btn>
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="12">
        <v-card border flat>
          <v-card-item prepend-icon="mdi-devices">
            <template #append>
              <v-btn
                icon="mdi-refresh"
                variant="text"
                aria-label="刷新会话列表"
                :loading="session.sessionListPending"
                :disabled="operationPending"
                @click="loadSessions"
              />
            </template>
            <v-card-title>登录会话</v-card-title>
            <v-card-subtitle>共 {{ sessions.length }} 个活动会话</v-card-subtitle>
          </v-card-item>

          <v-card-text v-if="session.sessionListPending" role="status" aria-live="polite">
            <v-skeleton-loader type="list-item-two-line@2" />
            <span class="sr-only">正在读取会话列表</span>
          </v-card-text>
          <v-card-text v-else-if="sessions.length === 0" class="text-medium-emphasis">
            暂无可显示的活动会话。
          </v-card-text>
          <v-list v-else lines="three" class="pb-2">
            <template v-for="candidate in sessions" :key="candidate.id">
              <v-divider />
              <v-list-item>
                <template #prepend>
                  <v-avatar :color="candidate.is_current ? 'primary' : 'surface-variant'" size="40">
                    <v-icon icon="mdi-monitor-cellphone" />
                  </v-avatar>
                </template>
                <v-list-item-title class="d-flex align-center flex-wrap ga-2">
                  {{ candidate.is_current ? '当前浏览器' : '其他会话' }}
                  <v-chip
                    v-if="candidate.is_current"
                    color="primary"
                    size="x-small"
                    variant="tonal"
                  >
                    当前
                  </v-chip>
                  <v-chip size="x-small" variant="outlined">
                    {{ authLevelLabel(candidate.auth_level) }}
                  </v-chip>
                </v-list-item-title>
                <v-list-item-subtitle class="session-time-grid mt-1">
                  <span>最近活动：{{ formatTimestamp(candidate.last_seen_at_ms) }}</span>
                  <span>登录时间：{{ formatTimestamp(candidate.created_at_ms) }}</span>
                  <span>最晚失效：{{ formatTimestamp(candidate.absolute_expires_at_ms) }}</span>
                </v-list-item-subtitle>
                <template v-if="!candidate.is_current" #append>
                  <v-btn
                    color="error"
                    variant="text"
                    size="small"
                    :loading="session.revokingSessionIds.includes(candidate.id)"
                    :disabled="operationPending"
                    @click="openRevokeDialog(candidate)"
                  >
                    撤销
                  </v-btn>
                </template>
              </v-list-item>
            </template>
          </v-list>

          <v-divider />
          <v-card-actions class="pa-4 flex-wrap ga-2">
            <v-btn
              color="warning"
              variant="tonal"
              :disabled="otherSessionCount === 0 || operationPending"
              @click="logoutOthersDialog = true"
            >
              退出其他会话
            </v-btn>
            <v-btn
              color="error"
              variant="tonal"
              :disabled="operationPending"
              @click="logoutEverywhereDialog = true"
            >
              退出所有会话
            </v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>

    <v-dialog v-model="recoveryCodeRegenerationDialog" max-width="520">
      <v-card>
        <v-card-title>重新生成恢复码？</v-card-title>
        <v-card-text>
          新的一组恢复码生成后，现有恢复码会立即全部失效。新明文只会在下一步显示一次，操作不会自动重放。
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn autofocus @click="recoveryCodeRegenerationDialog = false">取消</v-btn>
          <v-btn color="warning" variant="flat" @click="regenerateRecoveryCodeSet">
            确认重新生成
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog :model-value="oneTimeRecoveryCodes.hasCodes" persistent max-width="760">
      <OneTimeRecoveryCodes
        :codes="oneTimeRecoveryCodes.codes"
        context="regenerated"
        @download-failed="recoveryCodeDownloadFailed = true"
        @confirmed="confirmRecoveryCodesSaved"
      />
    </v-dialog>

    <v-dialog v-model="revokeDialog" max-width="480">
      <v-card>
        <v-card-title>撤销所选会话？</v-card-title>
        <v-card-text>该会话会立即失效。此操作不会影响当前浏览器。</v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn autofocus @click="revokeDialog = false">取消</v-btn>
          <v-btn color="error" variant="flat" @click="revokeSelectedSession">确认撤销</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="logoutOthersDialog" max-width="480">
      <v-card>
        <v-card-title>退出其他会话？</v-card-title>
        <v-card-text>
          除当前浏览器外的所有会话都会失效；当前浏览器会换发新的会话凭据。
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn autofocus @click="logoutOthersDialog = false">取消</v-btn>
          <v-btn color="warning" variant="flat" @click="logoutOtherSessions"> 退出其他会话 </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <v-dialog v-model="logoutEverywhereDialog" max-width="480">
      <v-card>
        <v-card-title>退出所有会话？</v-card-title>
        <v-card-text> 包括当前浏览器在内的所有会话都会立即失效，你需要重新登录。 </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn autofocus @click="logoutEverywhereDialog = false">取消</v-btn>
          <v-btn color="error" variant="flat" @click="logoutEverywhere"> 确认全部退出 </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>
