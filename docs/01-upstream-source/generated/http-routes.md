# 妙妙屋顶层 HTTP 路由注册表

> 从 `cmd/server/main.go` 的 `http.ServeMux` 注册语句自动提取。前缀路由的子路径和方法由对应 Handler 再分派；详细业务方法见人工 HTTP API 文档。

共 87 条顶层注册。

| 路径/前缀 | 访问边界 | Handler/构造器 | 注册方式 | 源码行 |
|---|---|---|---|---:|
| `/api/setup/status` | 公开或 Handler 内校验 | `NewSetupStatusHandler` | `Handle` | 156 |
| `/api/setup/init` | 公开或 Handler 内校验 | `NewInitialSetupHandler` | `Handle` | 157 |
| `/api/setup/restore-backup` | 公开或 Handler 内校验 | `NewSetupRestoreBackupHandler` | `Handle` | 158 |
| `/api/captcha/config` | 公开或 Handler 内校验 | `inline HandlerFunc` | `HandleFunc` | 160 |
| `/api/login` | 公开或 Handler 内校验 | `NewLoginHandler` | `Handle` | 168 |
| `/api/login/2fa` | 公开或 Handler 内校验 | `NewTwoFactorLoginHandler` | `Handle` | 169 |
| `/api/login/recovery` | 公开或 Handler 内校验 | `NewRecoveryLoginHandler` | `Handle` | 170 |
| `/api/admin/credentials` | 管理员 UI 会话 | `NewCredentialsHandler` | `Handle` | 173 |
| `/api/admin/users` | 管理员 UI 会话 | `NewUserListHandler` | `Handle` | 174 |
| `/api/admin/users/create` | 管理员 UI 会话 | `NewUserCreateHandler` | `Handle` | 175 |
| `/api/admin/users/delete` | 管理员 UI 会话 | `NewUserDeleteHandler` | `Handle` | 176 |
| `/api/admin/users/status` | 管理员 UI 会话 | `NewUserStatusHandler` | `Handle` | 177 |
| `/api/admin/users/reset-password` | 管理员 UI 会话 | `NewUserResetPasswordHandler` | `Handle` | 178 |
| `/api/admin/users/remark` | 管理员 UI 会话 | `NewUserRemarkHandler` | `Handle` | 179 |
| `/api/admin/users/custom-short-code` | 管理员 UI 会话 | `NewUserCustomShortCodeHandler` | `Handle` | 180 |
| `/api/admin/users/` | 管理员 UI 会话 | `NewUserSubscriptionsHandler` | `Handle` | 181 |
| `/api/admin/security/` | 管理员 UI 会话 | `securityLogHandler` | `Handle` | 183 |
| `/api/admin/security/turnstile` | 管理员 UI 会话 | `NewTurnstileSettingsHandler` | `Handle` | 184 |
| `/api/admin/tasks/` | 管理员 UI 会话 | `NewTaskLogHandler` | `Handle` | 185 |
| `/api/admin/operations` | 管理员 UI 会话 | `NewOperationLogHandler` | `Handle` | 186 |
| `/api/admin/subscriptions` | 管理员 UI 会话 | `NewSubscriptionAdminHandler` | `Handle` | 187 |
| `/api/admin/subscriptions/` | 管理员 UI 会话 | `NewSubscriptionAdminHandler` | `Handle` | 188 |
| `/api/admin/subscribe-files` | 管理员 UI 会话 | `NewSubscribeFilesHandler` | `Handle` | 189 |
| `/api/admin/subscribe-files/` | 管理员 UI 会话 | `NewSubscribeFilesHandler` | `Handle` | 190 |
| `/api/admin/probe-config` | 管理员 UI 会话 | `NewProbeConfigHandler` | `Handle` | 191 |
| `/api/admin/probe-sync` | 管理员 UI 会话 | `NewProbeSyncHandler` | `Handle` | 192 |
| `/api/admin/rules/` | 管理员 UI 会话 | `NewRuleEditorHandler` | `Handle` | 193 |
| `/api/admin/rule-templates` | 已登录 UI 会话 | `NewRuleTemplatesHandler` | `Handle` | 194 |
| `/api/admin/rule-templates/` | 已登录 UI 会话 | `NewRuleTemplatesHandler` | `Handle` | 195 |
| `/api/user/default-template` | 已登录 UI 会话 | `NewUserDefaultTemplateHandler` | `Handle` | 196 |
| `/api/admin/template-v3/` | 管理员 UI 会话 | `NewTemplateV3Handler` | `Handle` | 197 |
| `/api/admin/nodes` | 管理员 UI 会话 | `NewNodesHandler` | `Handle` | 198 |
| `/api/admin/nodes/` | 管理员 UI 会话 | `NewNodesHandler` | `Handle` | 199 |
| `/api/admin/sync-external-subscriptions` | 管理员 UI 会话 | `NewSyncExternalSubscriptionsHandler` | `Handle` | 200 |
| `/api/admin/sync-external-subscription` | 管理员 UI 会话 | `NewSyncSingleExternalSubscriptionHandler` | `Handle` | 201 |
| `/api/admin/sync-external-subscriptions/confirm` | 管理员 UI 会话 | `NewConfirmExternalSyncHandler` | `Handle` | 202 |
| `/api/admin/rules/latest` | 管理员 UI 会话 | `NewRuleMetadataHandler` | `Handle` | 203 |
| `/api/admin/custom-rules` | 管理员 UI 会话 | `NewCustomRulesHandler` | `Handle` | 204 |
| `/api/admin/custom-rules/` | 管理员 UI 会话 | `NewCustomRuleHandler` | `Handle` | 205 |
| `/api/admin/apply-custom-rules` | 管理员 UI 会话 | `NewApplyCustomRulesHandler` | `Handle` | 206 |
| `/api/admin/override-scripts` | 管理员 UI 会话 | `NewOverrideScriptsHandler` | `Handle` | 207 |
| `/api/admin/override-scripts/` | 管理员 UI 会话 | `NewOverrideScriptsHandler` | `Handle` | 208 |
| `/api/admin/templates` | 管理员 UI 会话 | `NewTemplatesHandler` | `Handle` | 209 |
| `/api/admin/templates/` | 管理员 UI 会话 | `NewTemplateHandler` | `Handle` | 210 |
| `/api/admin/templates/convert` | 管理员 UI 会话 | `NewTemplateConvertHandler` | `Handle` | 211 |
| `/api/admin/templates/fetch-source` | 管理员 UI 会话 | `NewTemplateFetchSourceHandler` | `Handle` | 212 |
| `/api/admin/backup/download` | 管理员 UI 会话 | `NewBackupDownloadHandler` | `Handle` | 213 |
| `/api/admin/backup/restore` | 管理员 UI 会话 | `NewBackupRestoreHandler` | `Handle` | 214 |
| `/api/admin/update/check` | 管理员 UI 会话 | `NewUpdateCheckHandler` | `Handle` | 215 |
| `/api/admin/update/apply` | 管理员 UI 会话 | `NewUpdateApplyHandler` | `Handle` | 216 |
| `/api/admin/update/apply-sse` | 管理员 UI 会话 | `NewUpdateApplySSEHandler` | `Handle` | 217 |
| `/api/admin/proxy-groups/sync` | 管理员 UI 会话 | `NewProxyGroupsSyncHandler` | `Handle` | 218 |
| `/api/admin/notify-config` | 管理员 UI 会话 | `NewNotifyConfigHandler` | `Handle` | 219 |
| `/api/admin/notify-config/` | 管理员 UI 会话 | `NewNotifyConfigHandler` | `Handle` | 220 |
| `/api/admin/tcping` | 管理员 UI 会话 | `NewTCPingHandler` | `Handle` | 223 |
| `/api/admin/tcping/batch` | 管理员 UI 会话 | `NewTCPingBatchHandler` | `Handle` | 224 |
| `/api/proxy-groups` | 已登录 UI 会话 | `NewProxyGroupsHandler` | `Handle` | 227 |
| `/api/user/password` | 已登录 UI 会话 | `NewPasswordHandler` | `Handle` | 228 |
| `/api/user/profile` | 已登录 UI 会话 | `NewProfileHandler` | `Handle` | 229 |
| `/api/user/settings` | 已登录 UI 会话 | `NewUserSettingsHandler` | `Handle` | 230 |
| `/api/user/config` | 已登录 UI 会话 | `NewUserConfigHandler` | `Handle` | 231 |
| `/api/user/2fa/status` | 已登录 UI 会话 | `NewTwoFactorStatusHandler` | `Handle` | 232 |
| `/api/user/2fa/setup` | 已登录 UI 会话 | `NewTwoFactorSetupHandler` | `Handle` | 233 |
| `/api/user/2fa/verify-setup` | 已登录 UI 会话 | `NewTwoFactorVerifySetupHandler` | `Handle` | 234 |
| `/api/user/2fa/disable` | 已登录 UI 会话 | `NewTwoFactorDisableHandler` | `Handle` | 235 |
| `/api/user/token` | 已登录 UI 会话 | `NewUserTokenHandler` | `Handle` | 236 |
| `/api/user/external-subscriptions` | 已登录 UI 会话 | `NewExternalSubscriptionsHandler` | `Handle` | 237 |
| `/api/user/external-subscriptions/nodes` | 已登录 UI 会话 | `NewExternalSubscriptionNodesHandler` | `Handle` | 238 |
| `/api/user/external-subscriptions/check-filter` | 已登录 UI 会话 | `NewExternalSubscriptionCheckFilterHandler` | `Handle` | 239 |
| `/api/user/proxy-provider-configs` | 已登录 UI 会话 | `NewProxyProviderConfigsHandler` | `Handle` | 240 |
| `/api/user/proxy-provider-cache/refresh` | 已登录 UI 会话 | `NewProxyProviderCacheRefreshHandler` | `Handle` | 241 |
| `/api/user/proxy-provider-cache/status` | 已登录 UI 会话 | `NewProxyProviderCacheStatusHandler` | `Handle` | 242 |
| `/api/user/proxy-provider-nodes` | 已登录 UI 会话 | `NewProxyProviderNodesHandler` | `Handle` | 243 |
| `/api/proxy-provider/` | 端点内订阅鉴权 | `NewProxyProviderServeHandler` | `Handle` | 244 |
| `/api/user/debug/` | 已登录 UI 会话 | `NewDebugHandler` | `Handle` | 247 |
| `/api/traffic/summary` | 已登录 UI 会话 | `trafficHandler` | `Handle` | 249 |
| `/api/traffic/subscribe` | 已登录 UI 会话 | `trafficHandler, http.HandlerFunc` | `Handle` | 250 |
| `/api/subscriptions` | 已登录 UI 会话 | `NewSubscriptionListHandler` | `Handle` | 251 |
| `/api/dns/resolve` | 已登录 UI 会话 | `NewDNSHandler` | `Handle` | 252 |
| `/api/subscribe-files` | 已登录 UI 会话 | `NewSubscribeFilesListHandler` | `Handle` | 253 |
| `/api/clash/subscribe` | 端点内订阅鉴权 | `NewSubscriptionEndpoint` | `Handle` | 257 |
| `/api/user/short-link` | 已登录 UI 会话 | `NewShortLinkResetHandler` | `Handle` | 260 |
| `/api/user/custom-short-code` | 已登录 UI 会话 | `NewUserCustomShortCodeSelfHandler` | `Handle` | 261 |
| `/api/admin/speedtest/` | 管理员 UI 会话 | `speedTestHandler` | `Handle` | 267 |
| `/api/speedtest/tester/ws` | 端点内 tester 鉴权 | `speedTesterWS` | `Handle` | 268 |
| `/api/admin/temp-subscription` | 管理员 UI 会话 | `NewTempSubscriptionHandler` | `Handle` | 271 |
| `/` | 混合：SPA/短链/临时订阅 | `tempSubAccessHandler, shortLinkHandler, web.Handler` | `HandleFunc` | 286 |
