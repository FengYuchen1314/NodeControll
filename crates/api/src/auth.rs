use std::net::SocketAddr;

use axum::{
    Json,
    extract::{ConnectInfo, Path, State, rejection::JsonRejection},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use nodecontroll_application::{
    ActorProjection, AuthServiceError, ChangePasswordCommand, LoginCommand, LoginOutcome,
    LogoutAllCommand, LogoutAllOutcome, MutatingSessionCredential, PasswordChangeOutcome,
    ReauthenticateCommand, RegenerateRecoveryCodesCommand, RequestContext, RevokeSessionCommand,
    SessionCredential, SessionProjection, UserSessionProjection,
};
use nodecontroll_domain::EntityId;
use serde::{Deserialize, Deserializer, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use zeroize::Zeroizing;

use crate::{AppState, Problem, ResponseMeta, request_id, web_security};

#[derive(Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    #[schema(min_length = 3, max_length = 32)]
    pub username: String,
    #[serde(deserialize_with = "deserialize_secret_string")]
    #[schema(value_type = String, format = Password, min_length = 1, max_length = 1024, write_only)]
    pub password: Zeroizing<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReauthenticationMethod {
    Password,
}

#[derive(Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ReauthenticateRequest {
    pub method: ReauthenticationMethod,
    #[serde(deserialize_with = "deserialize_secret_string")]
    #[schema(value_type = String, format = Password, min_length = 1, max_length = 1024, write_only)]
    pub password: Zeroizing<String>,
}

#[derive(Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ChangePasswordRequest {
    /// At least 12 Unicode scalar values and at most 1024 UTF-8 bytes.
    #[serde(deserialize_with = "deserialize_secret_string")]
    #[schema(value_type = String, format = Password, min_length = 12, max_length = 1024, write_only)]
    pub new_password: Zeroizing<String>,
}

fn deserialize_secret_string<'de, D>(deserializer: D) -> Result<Zeroizing<String>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(Zeroizing::new)
}

#[derive(Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct LogoutAllRequest {
    pub keep_current: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ActorResponse {
    pub id: String,
    pub username: String,
    pub role: String,
    pub capabilities: Vec<String>,
    pub force_password_change: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionResponse {
    pub id: String,
    pub auth_level: String,
    pub created_at_ms: i64,
    pub last_seen_at_ms: i64,
    pub idle_expires_at_ms: i64,
    pub absolute_expires_at_ms: i64,
    pub recent_auth_expires_at_ms: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthenticatedData {
    pub actor: ActorResponse,
    pub session: SessionResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthenticatedEnvelope {
    pub data: AuthenticatedData,
    pub meta: ResponseMeta,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PasswordChangedData {
    pub actor: ActorResponse,
    pub session: SessionResponse,
    pub revoked_sessions: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PasswordChangedEnvelope {
    pub data: PasswordChangedData,
    pub meta: ResponseMeta,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutAllRetainedData {
    pub actor: ActorResponse,
    pub session: SessionResponse,
    pub revoked_sessions: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutAllRetainedEnvelope {
    pub data: LogoutAllRetainedData,
    pub meta: ResponseMeta,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserSessionResponse {
    pub id: String,
    pub is_current: bool,
    pub auth_level: String,
    pub created_at_ms: i64,
    pub last_seen_at_ms: i64,
    pub idle_expires_at_ms: i64,
    pub absolute_expires_at_ms: i64,
    pub recent_auth_expires_at_ms: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserSessionsData {
    pub sessions: Vec<UserSessionResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserSessionsEnvelope {
    pub data: UserSessionsData,
    pub meta: ResponseMeta,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecoveryCodeSummaryData {
    #[schema(minimum = 1, maximum = 9_007_199_254_740_991_i64)]
    pub set_version: u64,
    #[schema(minimum = 8, maximum = 8)]
    pub total_count: u8,
    #[schema(minimum = 0, maximum = 8)]
    pub remaining_count: u8,
    #[schema(minimum = 0, maximum = 9_007_199_254_740_991_i64)]
    pub created_at_ms: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecoveryCodeSummaryEnvelope {
    pub data: RecoveryCodeSummaryData,
    pub meta: ResponseMeta,
}

#[derive(Serialize, ToSchema)]
pub struct RecoveryCodesCreatedData {
    #[schema(minimum = 1, maximum = 9_007_199_254_740_991_i64)]
    pub set_version: u64,
    #[schema(schema_with = crate::one_time_recovery_codes_schema)]
    pub one_time_recovery_codes: Vec<String>,
    #[schema(minimum = 0, maximum = 9_007_199_254_740_991_i64)]
    pub created_at_ms: i64,
}

#[derive(Serialize, ToSchema)]
pub struct RecoveryCodesCreatedEnvelope {
    pub data: RecoveryCodesCreatedData,
    pub meta: ResponseMeta,
}

impl From<ActorProjection> for ActorResponse {
    fn from(actor: ActorProjection) -> Self {
        Self {
            id: actor.id.to_string(),
            username: actor.username,
            role: actor.role.as_str().to_owned(),
            capabilities: actor
                .capabilities
                .scopes()
                .iter()
                .map(|scope| scope.as_str().to_owned())
                .collect(),
            force_password_change: actor.force_password_change,
        }
    }
}

impl From<SessionProjection> for SessionResponse {
    fn from(session: SessionProjection) -> Self {
        Self {
            id: session.id.to_string(),
            auth_level: session.auth_level,
            created_at_ms: session.created_at_ms,
            last_seen_at_ms: session.last_seen_at_ms,
            idle_expires_at_ms: session.idle_expires_at_ms,
            absolute_expires_at_ms: session.absolute_expires_at_ms,
            recent_auth_expires_at_ms: session.recent_auth_expires_at_ms,
        }
    }
}

impl From<UserSessionProjection> for UserSessionResponse {
    fn from(projection: UserSessionProjection) -> Self {
        let session = projection.session;
        Self {
            id: session.id.to_string(),
            is_current: projection.is_current,
            auth_level: session.auth_level,
            created_at_ms: session.created_at_ms,
            last_seen_at_ms: session.last_seen_at_ms,
            idle_expires_at_ms: session.idle_expires_at_ms,
            absolute_expires_at_ms: session.absolute_expires_at_ms,
            recent_auth_expires_at_ms: session.recent_auth_expires_at_ms,
        }
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    operation_id = "login",
    tag = "authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Password authentication succeeded and host-only session cookies were issued", body = AuthenticatedEnvelope, headers(("Set-Cookie" = String, description = "Two separate host-only header fields issue the session and CSRF cookies"))),
        (status = 400, description = "The JSON request is malformed", body = Problem, content_type = "application/problem+json"),
        (status = 413, description = "The JSON request exceeds the configured body limit", body = Problem, content_type = "application/problem+json"),
        (status = 415, description = "The request media type is unsupported", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The JSON value does not match the login schema", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "The supplied credentials are invalid", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The browser origin or host does not match the configured public origin", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "The control plane has not been initialized", body = Problem, content_type = "application/problem+json"),
        (status = 429, description = "A shared login limit is active", body = Problem, content_type = "application/problem+json", headers(("Retry-After" = u64, description = "Bounded delay in seconds"))),
        (status = 503, description = "Authentication dependencies are unavailable", body = Problem, content_type = "application/problem+json")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    request: Result<Json<LoginRequest>, JsonRejection>,
) -> Result<Response, Problem> {
    state
        .web_security
        .validate_browser_origin(&headers)
        .map_err(|_| browser_security_problem(&headers))?;
    let context = request_context(&state, peer, &headers)?;
    let Json(request) = request.map_err(|error| auth_json_problem(error, &headers))?;
    let outcome = state
        .control_plane
        .login(LoginCommand {
            username: request.username,
            password: request.password,
            context,
        })
        .await
        .map_err(|error| auth_problem(error, &headers))?;
    login_response(outcome, state.session_cookie_max_age_seconds, &headers)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/reauth",
    operation_id = "reauthenticate",
    tag = "authentication",
    security(("sessionCookie" = [], "csrfHeader" = [])),
    request_body = ReauthenticateRequest,
    responses(
        (status = 200, description = "The recent-auth proof succeeded and both browser credentials were rotated", body = AuthenticatedEnvelope, headers(("Set-Cookie" = String, description = "Two separate host-only header fields rotate the session and CSRF cookies"))),
        (status = 400, description = "The request body is malformed", body = Problem, content_type = "application/problem+json"),
        (status = 413, description = "The request body exceeds the configured limit", body = Problem, content_type = "application/problem+json"),
        (status = 415, description = "The request media type is unsupported", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The JSON value does not match the reauthentication schema", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "The current session is invalid", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "Origin, CSRF, or the reauthentication proof is invalid", body = Problem, content_type = "application/problem+json"),
        (status = 429, description = "A shared authentication limit is active", body = Problem, content_type = "application/problem+json", headers(("Retry-After" = u64, description = "Bounded delay in seconds"))),
        (status = 503, description = "Authentication dependencies are unavailable", body = Problem, content_type = "application/problem+json")
    )
)]
pub async fn reauthenticate(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    request: Result<Json<ReauthenticateRequest>, JsonRejection>,
) -> Result<Response, Problem> {
    state
        .web_security
        .validate_browser_origin(&headers)
        .map_err(|_| browser_security_problem(&headers))?;
    let credential = required_mutating_credential(&state, peer, &headers)?;
    let Json(request) = request.map_err(|error| protected_auth_json_problem(error, &headers))?;
    let ReauthenticationMethod::Password = request.method;
    let outcome = state
        .control_plane
        .reauthenticate(ReauthenticateCommand {
            credential,
            password: request.password,
        })
        .await
        .map_err(|error| auth_problem(error, &headers))?;
    login_response(outcome, state.session_cookie_max_age_seconds, &headers)
}

#[utoipa::path(
    post,
    path = "/api/v1/me/password",
    operation_id = "changeCurrentPassword",
    tag = "authentication",
    security(("sessionCookie" = [], "csrfHeader" = [])),
    request_body = ChangePasswordRequest,
    responses(
        (status = 200, description = "The password changed, all sessions were revoked, and this browser received a replacement session", body = PasswordChangedEnvelope, headers(("Set-Cookie" = String, description = "Two separate host-only header fields issue the replacement session and CSRF cookies"))),
        (status = 400, description = "The request body is malformed", body = Problem, content_type = "application/problem+json"),
        (status = 413, description = "The request body exceeds the configured limit", body = Problem, content_type = "application/problem+json"),
        (status = 415, description = "The request media type is unsupported", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "The current session is invalid", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "Origin, CSRF, or recent-auth verification failed", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The new password is rejected by policy or is unchanged", body = Problem, content_type = "application/problem+json"),
        (status = 429, description = "Password hashing capacity is temporarily exhausted", body = Problem, content_type = "application/problem+json", headers(("Retry-After" = u64, description = "Bounded delay in seconds"))),
        (status = 503, description = "Authentication dependencies are unavailable", body = Problem, content_type = "application/problem+json")
    )
)]
pub async fn change_password(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    request: Result<Json<ChangePasswordRequest>, JsonRejection>,
) -> Result<Response, Problem> {
    state
        .web_security
        .validate_browser_origin(&headers)
        .map_err(|_| browser_security_problem(&headers))?;
    let credential = required_mutating_credential(&state, peer, &headers)?;
    let Json(request) = request.map_err(|error| protected_auth_json_problem(error, &headers))?;
    let outcome = state
        .control_plane
        .change_password(ChangePasswordCommand {
            credential,
            new_password: request.new_password,
        })
        .await
        .map_err(|error| auth_problem(error, &headers))?;
    password_changed_response(outcome, state.session_cookie_max_age_seconds, &headers)
}

#[utoipa::path(
    get,
    path = "/api/v1/me",
    operation_id = "getCurrentActor",
    tag = "authentication",
    security(("sessionCookie" = [])),
    responses(
        (status = 200, description = "The current active actor and server-side session projection", body = AuthenticatedEnvelope),
        (status = 400, description = "Required request metadata is malformed", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "The session is absent, invalid, revoked, inactive, or expired", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The request host does not match the configured public origin", body = Problem, content_type = "application/problem+json"),
        (status = 503, description = "Authentication dependencies are unavailable", body = Problem, content_type = "application/problem+json")
    )
)]
pub async fn current_actor(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Result<Response, Problem> {
    state
        .web_security
        .validate_request_host(&headers)
        .map_err(|_| browser_security_problem(&headers))?;
    let context = request_context(&state, peer, &headers)?;
    let session_token = web_security::security_cookie(&headers, web_security::SESSION_COOKIE_NAME)
        .map_err(|_| auth_problem(AuthServiceError::SessionInvalid, &headers))?
        .ok_or_else(|| auth_problem(AuthServiceError::SessionInvalid, &headers))?;
    let (actor, session) = state
        .control_plane
        .current_actor(SessionCredential {
            session_token,
            csrf_token: None,
            context,
        })
        .await
        .map_err(|error| auth_problem(error, &headers))?;
    Ok(authenticated_response(actor, session, &headers))
}

#[utoipa::path(
    get,
    path = "/api/v1/me/sessions",
    operation_id = "listCurrentSessions",
    tag = "authentication",
    security(("sessionCookie" = [])),
    responses(
        (status = 200, description = "Active server-side sessions with coarse, secret-free projections", body = UserSessionsEnvelope),
        (status = 400, description = "Required request metadata is malformed", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "The current session is invalid", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The request host is invalid", body = Problem, content_type = "application/problem+json"),
        (status = 503, description = "Authentication dependencies are unavailable", body = Problem, content_type = "application/problem+json")
    )
)]
pub async fn list_sessions(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Result<Response, Problem> {
    state
        .web_security
        .validate_request_host(&headers)
        .map_err(|_| browser_security_problem(&headers))?;
    let credential = required_session_credential(&state, peer, &headers)?;
    let sessions = state
        .control_plane
        .list_sessions(credential)
        .await
        .map_err(|error| auth_problem(error, &headers))?;
    let envelope = UserSessionsEnvelope {
        data: UserSessionsData {
            sessions: sessions.into_iter().map(Into::into).collect(),
        },
        meta: ResponseMeta {
            api_version: "v1",
            request_id: request_id(&headers),
        },
    };
    let mut response = (StatusCode::OK, Json(envelope)).into_response();
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    Ok(response)
}

#[utoipa::path(
    get,
    path = "/api/v1/me/recovery-codes",
    operation_id = "getCurrentRecoveryCodes",
    tag = "authentication",
    security(("sessionCookie" = [])),
    responses(
        (status = 200, description = "Secret-free summary of the active recovery-code set", body = RecoveryCodeSummaryEnvelope, headers(("Cache-Control" = String, description = "Always no-store"))),
        (status = 401, description = "The current session is invalid", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The request host is invalid or password change is required", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "No active recovery-code set exists", body = Problem, content_type = "application/problem+json"),
        (status = 503, description = "Authentication dependencies are unavailable", body = Problem, content_type = "application/problem+json")
    )
)]
pub async fn get_recovery_codes(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Result<Response, Problem> {
    state
        .web_security
        .validate_request_host(&headers)
        .map_err(|_| browser_security_problem(&headers))?;
    let credential = required_session_credential(&state, peer, &headers)?;
    let summary = state
        .control_plane
        .recovery_code_summary(credential)
        .await
        .map_err(|error| auth_problem(error, &headers))?;
    let envelope = RecoveryCodeSummaryEnvelope {
        data: RecoveryCodeSummaryData {
            set_version: summary.set_version,
            total_count: summary.total_count,
            remaining_count: summary.remaining_count,
            created_at_ms: summary.created_at_ms,
        },
        meta: ResponseMeta {
            api_version: "v1",
            request_id: request_id(&headers),
        },
    };
    let mut response = (StatusCode::OK, Json(envelope)).into_response();
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    Ok(response)
}

#[utoipa::path(
    post,
    path = "/api/v1/me/recovery-codes",
    operation_id = "regenerateCurrentRecoveryCodes",
    tag = "authentication",
    security(("sessionCookie" = [], "csrfHeader" = [])),
    responses(
        (status = 200, description = "The old set was atomically invalidated and eight replacement codes are returned once", body = RecoveryCodesCreatedEnvelope, headers(("Cache-Control" = String, description = "Always no-store because recovery codes are returned once"))),
        (status = 401, description = "The current session is invalid", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "Origin, CSRF, recent-auth, or password-change policy rejected the request", body = Problem, content_type = "application/problem+json"),
        (status = 503, description = "Authentication dependencies are unavailable", body = Problem, content_type = "application/problem+json")
    )
)]
pub async fn regenerate_recovery_codes(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Result<Response, Problem> {
    state
        .web_security
        .validate_browser_origin(&headers)
        .map_err(|_| browser_security_problem(&headers))?;
    let credential = required_mutating_credential(&state, peer, &headers)?;
    let created = state
        .control_plane
        .regenerate_recovery_codes(RegenerateRecoveryCodesCommand { credential })
        .await
        .map_err(|error| auth_problem(error, &headers))?;
    let envelope = RecoveryCodesCreatedEnvelope {
        data: RecoveryCodesCreatedData {
            set_version: created.set_version,
            one_time_recovery_codes: created
                .one_time_recovery_codes
                .into_iter()
                .map(|code| code.as_str().to_owned())
                .collect(),
            created_at_ms: created.created_at_ms,
        },
        meta: ResponseMeta {
            api_version: "v1",
            request_id: request_id(&headers),
        },
    };
    let mut response = (StatusCode::OK, Json(envelope)).into_response();
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    Ok(response)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    operation_id = "logout",
    tag = "authentication",
    security(("sessionCookie" = [], "csrfHeader" = [])),
    responses(
        (status = 204, description = "The current server-side session was revoked and browser cookies were expired", headers(("Set-Cookie" = String, description = "Two separate host-only header fields expire the session and CSRF cookies"))),
        (status = 400, description = "Required request metadata is malformed", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "The Cookie header is oversized, ambiguous, or structurally malformed", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "Origin, host, or double-submit CSRF verification failed", body = Problem, content_type = "application/problem+json"),
        (status = 503, description = "Authentication dependencies are unavailable", body = Problem, content_type = "application/problem+json")
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Result<Response, Problem> {
    state
        .web_security
        .validate_browser_origin(&headers)
        .map_err(|_| browser_security_problem(&headers))?;
    let context = request_context(&state, peer, &headers)?;
    let session_token = web_security::security_cookie(&headers, web_security::SESSION_COOKIE_NAME)
        .map_err(|_| auth_problem(AuthServiceError::SessionInvalid, &headers))?;
    let Some(session_token) = session_token else {
        return Ok(logout_response());
    };
    let (_, csrf_token) = web_security::csrf_header_and_cookie(&headers)
        .map_err(|_| auth_problem(AuthServiceError::CsrfInvalid, &headers))?;
    state
        .control_plane
        .logout(MutatingSessionCredential {
            session_token,
            csrf_token,
            context,
        })
        .await
        .map_err(|error| auth_problem(error, &headers))?;
    Ok(logout_response())
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout-all",
    operation_id = "logoutAll",
    tag = "authentication",
    security(("sessionCookie" = [], "csrfHeader" = [])),
    request_body = LogoutAllRequest,
    responses(
        (status = 200, description = "All sessions were revoked and this browser received a replacement session", body = LogoutAllRetainedEnvelope, headers(("Set-Cookie" = String, description = "Two separate host-only header fields issue the replacement session and CSRF cookies"))),
        (status = 204, description = "All sessions, including this browser, were revoked and cookies were expired", headers(("Set-Cookie" = String, description = "Two separate host-only header fields expire the session and CSRF cookies"))),
        (status = 400, description = "The request body is malformed", body = Problem, content_type = "application/problem+json"),
        (status = 413, description = "The request body exceeds the configured limit", body = Problem, content_type = "application/problem+json"),
        (status = 415, description = "The request media type is unsupported", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The JSON value does not match the logout-all schema", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "The current session is invalid", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "Origin, CSRF, or recent-auth verification failed", body = Problem, content_type = "application/problem+json"),
        (status = 503, description = "Authentication dependencies are unavailable", body = Problem, content_type = "application/problem+json")
    )
)]
pub async fn logout_all(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    request: Result<Json<LogoutAllRequest>, JsonRejection>,
) -> Result<Response, Problem> {
    state
        .web_security
        .validate_browser_origin(&headers)
        .map_err(|_| browser_security_problem(&headers))?;
    let credential = required_mutating_credential(&state, peer, &headers)?;
    let Json(request) = request.map_err(|error| protected_auth_json_problem(error, &headers))?;
    let outcome = state
        .control_plane
        .logout_all(LogoutAllCommand {
            credential,
            keep_current: request.keep_current,
        })
        .await
        .map_err(|error| auth_problem(error, &headers))?;
    match outcome {
        LogoutAllOutcome::CurrentRetained {
            login,
            revoked_sessions,
        } => logout_all_retained_response(
            login,
            revoked_sessions,
            state.session_cookie_max_age_seconds,
            &headers,
        ),
        LogoutAllOutcome::SignedOut { .. } => Ok(logout_response()),
    }
}

#[utoipa::path(
    delete,
    path = "/api/v1/me/sessions/{session_id}",
    operation_id = "revokeCurrentUserSession",
    tag = "authentication",
    security(("sessionCookie" = [], "csrfHeader" = [])),
    params(("session_id" = String, Path, description = "Session UUID")),
    responses(
        (status = 204, description = "While the caller session remains valid, the selected session is revoked or was already unavailable", headers(("Set-Cookie" = String, description = "Two separate header fields expire the session and CSRF cookies when the current session was selected"))),
        (status = 400, description = "The session identifier or required request metadata is invalid", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "The current session is invalid", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "Origin, CSRF, or recent-auth verification failed", body = Problem, content_type = "application/problem+json"),
        (status = 503, description = "Authentication dependencies are unavailable", body = Problem, content_type = "application/problem+json")
    )
)]
pub async fn revoke_session(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    Path(session_id): Path<String>,
    headers: HeaderMap,
) -> Result<Response, Problem> {
    state
        .web_security
        .validate_browser_origin(&headers)
        .map_err(|_| browser_security_problem(&headers))?;
    let credential = required_mutating_credential(&state, peer, &headers)?;
    let parsed_session_id =
        uuid::Uuid::parse_str(&session_id).map_err(|_| session_id_problem(&headers))?;
    if parsed_session_id.to_string() != session_id {
        return Err(session_id_problem(&headers));
    }
    let target_session_id = EntityId::from_uuid(parsed_session_id);
    let outcome = state
        .control_plane
        .revoke_session(RevokeSessionCommand {
            credential,
            target_session_id,
        })
        .await
        .map_err(|error| auth_problem(error, &headers))?;
    Ok(if outcome.revoked_current {
        logout_response()
    } else {
        no_content_response()
    })
}

fn request_context(
    state: &AppState,
    peer: SocketAddr,
    headers: &HeaderMap,
) -> Result<RequestContext, Problem> {
    let client = state
        .web_security
        .resolve_client_network(peer, headers)
        .map_err(|_| request_metadata_problem(headers))?;
    let user_agent =
        web_security::bounded_user_agent(headers).map_err(|_| request_metadata_problem(headers))?;
    Ok(RequestContext {
        request_id: request_id(headers),
        client,
        user_agent,
    })
}

fn required_session_credential(
    state: &AppState,
    peer: SocketAddr,
    headers: &HeaderMap,
) -> Result<SessionCredential, Problem> {
    let context = request_context(state, peer, headers)?;
    let session_token = web_security::security_cookie(headers, web_security::SESSION_COOKIE_NAME)
        .map_err(|_| auth_problem(AuthServiceError::SessionInvalid, headers))?
        .ok_or_else(|| auth_problem(AuthServiceError::SessionInvalid, headers))?;
    Ok(SessionCredential {
        session_token,
        csrf_token: None,
        context,
    })
}

fn required_mutating_credential(
    state: &AppState,
    peer: SocketAddr,
    headers: &HeaderMap,
) -> Result<MutatingSessionCredential, Problem> {
    let context = request_context(state, peer, headers)?;
    let session_token = web_security::security_cookie(headers, web_security::SESSION_COOKIE_NAME)
        .map_err(|_| auth_problem(AuthServiceError::SessionInvalid, headers))?
        .ok_or_else(|| auth_problem(AuthServiceError::SessionInvalid, headers))?;
    let (_, csrf_token) = web_security::csrf_header_and_cookie(headers)
        .map_err(|_| auth_problem(AuthServiceError::CsrfInvalid, headers))?;
    Ok(MutatingSessionCredential {
        session_token,
        csrf_token,
        context,
    })
}

fn login_response(
    outcome: LoginOutcome,
    max_age_seconds: u64,
    headers: &HeaderMap,
) -> Result<Response, Problem> {
    let LoginOutcome {
        actor,
        session,
        session_token,
        csrf_token,
    } = outcome;
    let cookie_max_age_seconds =
        remaining_cookie_max_age_seconds(max_age_seconds, session.absolute_expires_at_ms);
    let mut response = authenticated_response(actor, session, headers);
    append_session_cookies(
        &mut response,
        &session_token,
        &csrf_token,
        cookie_max_age_seconds,
        headers,
    )?;
    Ok(response)
}

fn password_changed_response(
    outcome: PasswordChangeOutcome,
    max_age_seconds: u64,
    headers: &HeaderMap,
) -> Result<Response, Problem> {
    let PasswordChangeOutcome {
        login:
            LoginOutcome {
                actor,
                session,
                session_token,
                csrf_token,
            },
        revoked_sessions,
    } = outcome;
    let cookie_max_age_seconds =
        remaining_cookie_max_age_seconds(max_age_seconds, session.absolute_expires_at_ms);
    let envelope = PasswordChangedEnvelope {
        data: PasswordChangedData {
            actor: actor.into(),
            session: session.into(),
            revoked_sessions,
        },
        meta: ResponseMeta {
            api_version: "v1",
            request_id: request_id(headers),
        },
    };
    let mut response = (StatusCode::OK, Json(envelope)).into_response();
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    append_session_cookies(
        &mut response,
        &session_token,
        &csrf_token,
        cookie_max_age_seconds,
        headers,
    )?;
    Ok(response)
}

fn logout_all_retained_response(
    login: LoginOutcome,
    revoked_sessions: u64,
    max_age_seconds: u64,
    headers: &HeaderMap,
) -> Result<Response, Problem> {
    let LoginOutcome {
        actor,
        session,
        session_token,
        csrf_token,
    } = login;
    let cookie_max_age_seconds =
        remaining_cookie_max_age_seconds(max_age_seconds, session.absolute_expires_at_ms);
    let envelope = LogoutAllRetainedEnvelope {
        data: LogoutAllRetainedData {
            actor: actor.into(),
            session: session.into(),
            revoked_sessions,
        },
        meta: ResponseMeta {
            api_version: "v1",
            request_id: request_id(headers),
        },
    };
    let mut response = (StatusCode::OK, Json(envelope)).into_response();
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    append_session_cookies(
        &mut response,
        &session_token,
        &csrf_token,
        cookie_max_age_seconds,
        headers,
    )?;
    Ok(response)
}

fn append_session_cookies(
    response: &mut Response,
    session_token: &nodecontroll_identity::SessionToken,
    csrf_token: &nodecontroll_identity::CsrfToken,
    max_age_seconds: u64,
    headers: &HeaderMap,
) -> Result<(), Problem> {
    let session_cookie = web_security::session_set_cookie(session_token, max_age_seconds);
    let csrf_cookie = web_security::csrf_set_cookie(csrf_token, max_age_seconds);
    let session_cookie = HeaderValue::from_str(&session_cookie)
        .map_err(|_| auth_problem(AuthServiceError::Unavailable, headers))?;
    let csrf_cookie = HeaderValue::from_str(&csrf_cookie)
        .map_err(|_| auth_problem(AuthServiceError::Unavailable, headers))?;
    response
        .headers_mut()
        .append(header::SET_COOKIE, session_cookie);
    response
        .headers_mut()
        .append(header::SET_COOKIE, csrf_cookie);
    Ok(())
}

fn remaining_cookie_max_age_seconds(
    configured_max_age_seconds: u64,
    absolute_expires_at_ms: i64,
) -> u64 {
    let now_ms = i64::try_from(OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000)
        .unwrap_or(i64::MAX);
    remaining_cookie_max_age_seconds_at(configured_max_age_seconds, absolute_expires_at_ms, now_ms)
}

fn remaining_cookie_max_age_seconds_at(
    configured_max_age_seconds: u64,
    absolute_expires_at_ms: i64,
    now_ms: i64,
) -> u64 {
    let remaining_ms = absolute_expires_at_ms.saturating_sub(now_ms).max(0);
    let remaining_seconds = u64::try_from(remaining_ms / 1_000).unwrap_or(0);
    configured_max_age_seconds.min(remaining_seconds)
}

fn authenticated_response(
    actor: ActorProjection,
    session: SessionProjection,
    headers: &HeaderMap,
) -> Response {
    let envelope = AuthenticatedEnvelope {
        data: AuthenticatedData {
            actor: actor.into(),
            session: session.into(),
        },
        meta: ResponseMeta {
            api_version: "v1",
            request_id: request_id(headers),
        },
    };
    let mut response = (StatusCode::OK, Json(envelope)).into_response();
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

fn logout_response() -> Response {
    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().append(
        header::SET_COOKIE,
        HeaderValue::from_static(web_security::clear_session_cookie()),
    );
    response.headers_mut().append(
        header::SET_COOKIE,
        HeaderValue::from_static(web_security::clear_csrf_cookie()),
    );
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

fn no_content_response() -> Response {
    let mut response = StatusCode::NO_CONTENT.into_response();
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response
}

fn auth_problem(error: AuthServiceError, headers: &HeaderMap) -> Problem {
    let request_id = request_id(headers);
    match error {
        AuthServiceError::InvalidCredentials => Problem {
            type_uri: "urn:nodecontroll:problem:invalid-credentials",
            title: "Authentication failed",
            status: StatusCode::UNAUTHORIZED.as_u16(),
            code: "INVALID_CREDENTIALS",
            detail: "The supplied username or password is invalid",
            request_id,
            errors: Box::default(),
            retry_after_seconds: None,
        },
        AuthServiceError::InvalidProof => Problem {
            type_uri: "urn:nodecontroll:problem:reauthentication-failed",
            title: "Reauthentication failed",
            status: StatusCode::FORBIDDEN.as_u16(),
            code: "REAUTHENTICATION_FAILED",
            detail: "The supplied authentication proof is invalid",
            request_id,
            errors: Box::default(),
            retry_after_seconds: None,
        },
        AuthServiceError::InvalidNewPassword => Problem {
            type_uri: "urn:nodecontroll:problem:password-policy-rejected",
            title: "Password rejected",
            status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            code: "PASSWORD_POLICY_REJECTED",
            detail: "The new password does not satisfy the server-side password policy",
            request_id,
            errors: vec![crate::FieldError {
                pointer: "/new_password".to_owned(),
                code: "password_policy_rejected".to_owned(),
                message: "Choose a password accepted by the server-side policy".to_owned(),
            }]
            .into_boxed_slice(),
            retry_after_seconds: None,
        },
        AuthServiceError::PasswordUnchanged => Problem {
            type_uri: "urn:nodecontroll:problem:password-unchanged",
            title: "Password unchanged",
            status: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            code: "PASSWORD_UNCHANGED",
            detail: "The new password must differ from the current password",
            request_id,
            errors: vec![crate::FieldError {
                pointer: "/new_password".to_owned(),
                code: "password_unchanged".to_owned(),
                message: "Choose a password different from the current password".to_owned(),
            }]
            .into_boxed_slice(),
            retry_after_seconds: None,
        },
        AuthServiceError::RecentAuthRequired => Problem {
            type_uri: "urn:nodecontroll:problem:recent-auth-required",
            title: "Recent authentication required",
            status: StatusCode::FORBIDDEN.as_u16(),
            code: "RECENT_AUTH_REQUIRED",
            detail: "Complete a recent-auth challenge before performing this action",
            request_id,
            errors: Box::default(),
            retry_after_seconds: None,
        },
        AuthServiceError::PasswordChangeRequired => Problem {
            type_uri: "urn:nodecontroll:problem:password-change-required",
            title: "Password change required",
            status: StatusCode::FORBIDDEN.as_u16(),
            code: "PASSWORD_CHANGE_REQUIRED",
            detail: "Change the current password before accessing product functions",
            request_id,
            errors: Box::default(),
            retry_after_seconds: None,
        },
        AuthServiceError::RateLimited {
            retry_after_seconds,
        } => Problem {
            type_uri: "urn:nodecontroll:problem:rate-limited",
            title: "Too many requests",
            status: StatusCode::TOO_MANY_REQUESTS.as_u16(),
            code: "LOGIN_RATE_LIMITED",
            detail: "Wait before attempting to authenticate again",
            request_id,
            errors: Box::default(),
            retry_after_seconds: Some(
                u32::try_from(retry_after_seconds.max(1)).unwrap_or(u32::MAX),
            ),
        },
        AuthServiceError::SessionInvalid => Problem {
            type_uri: "urn:nodecontroll:problem:session-invalid",
            title: "Authentication required",
            status: StatusCode::UNAUTHORIZED.as_u16(),
            code: "SESSION_INVALID",
            detail: "The session is absent, invalid, revoked, inactive, or expired",
            request_id,
            errors: Box::default(),
            retry_after_seconds: None,
        },
        AuthServiceError::CsrfInvalid => Problem {
            type_uri: "urn:nodecontroll:problem:csrf-invalid",
            title: "Request verification failed",
            status: StatusCode::FORBIDDEN.as_u16(),
            code: "CSRF_INVALID",
            detail: "The double-submit CSRF proof is missing or invalid",
            request_id,
            errors: Box::default(),
            retry_after_seconds: None,
        },
        AuthServiceError::NotInitialized => Problem {
            type_uri: "urn:nodecontroll:problem:not-initialized",
            title: "Control plane not initialized",
            status: StatusCode::CONFLICT.as_u16(),
            code: "CONTROL_PLANE_NOT_INITIALIZED",
            detail: "Complete one-time control-plane initialization before authenticating",
            request_id,
            errors: Box::default(),
            retry_after_seconds: None,
        },
        AuthServiceError::RecoveryCodesUnavailable => Problem {
            type_uri: "urn:nodecontroll:problem:recovery-codes-unavailable",
            title: "Recovery codes unavailable",
            status: StatusCode::CONFLICT.as_u16(),
            code: "RECOVERY_CODES_UNAVAILABLE",
            detail: "The current user does not have an active recovery-code set",
            request_id,
            errors: Box::default(),
            retry_after_seconds: None,
        },
        AuthServiceError::Unavailable => Problem {
            type_uri: "urn:nodecontroll:problem:authentication-unavailable",
            title: "Authentication unavailable",
            status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
            code: "AUTHENTICATION_UNAVAILABLE",
            detail: "The authentication service could not complete the request",
            request_id,
            errors: Box::default(),
            retry_after_seconds: None,
        },
    }
}

pub(crate) fn browser_security_problem(headers: &HeaderMap) -> Problem {
    Problem {
        type_uri: "urn:nodecontroll:problem:browser-origin-invalid",
        title: "Browser request rejected",
        status: StatusCode::FORBIDDEN.as_u16(),
        code: "BROWSER_ORIGIN_INVALID",
        detail: "The request origin or host does not match the configured public origin",
        request_id: request_id(headers),
        errors: Box::default(),
        retry_after_seconds: None,
    }
}

fn request_metadata_problem(headers: &HeaderMap) -> Problem {
    Problem {
        type_uri: "urn:nodecontroll:problem:request-metadata-invalid",
        title: "Request metadata invalid",
        status: StatusCode::BAD_REQUEST.as_u16(),
        code: "REQUEST_METADATA_INVALID",
        detail: "Forwarding or user-agent metadata is malformed or exceeds its bound",
        request_id: request_id(headers),
        errors: Box::default(),
        retry_after_seconds: None,
    }
}

fn auth_json_problem(error: JsonRejection, headers: &HeaderMap) -> Problem {
    let status = match error.status() {
        StatusCode::PAYLOAD_TOO_LARGE => StatusCode::PAYLOAD_TOO_LARGE,
        StatusCode::UNSUPPORTED_MEDIA_TYPE => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        StatusCode::UNPROCESSABLE_ENTITY => StatusCode::UNPROCESSABLE_ENTITY,
        _ => StatusCode::BAD_REQUEST,
    };
    Problem {
        type_uri: "urn:nodecontroll:problem:login-json-invalid",
        title: "Login request invalid",
        status: status.as_u16(),
        code: "LOGIN_JSON_INVALID",
        detail: "The login request must be a bounded JSON object containing username and password",
        request_id: request_id(headers),
        errors: Box::default(),
        retry_after_seconds: None,
    }
}

fn protected_auth_json_problem(error: JsonRejection, headers: &HeaderMap) -> Problem {
    let status = match error.status() {
        StatusCode::PAYLOAD_TOO_LARGE => StatusCode::PAYLOAD_TOO_LARGE,
        StatusCode::UNSUPPORTED_MEDIA_TYPE => StatusCode::UNSUPPORTED_MEDIA_TYPE,
        StatusCode::UNPROCESSABLE_ENTITY => StatusCode::UNPROCESSABLE_ENTITY,
        _ => StatusCode::BAD_REQUEST,
    };
    Problem {
        type_uri: "urn:nodecontroll:problem:auth-action-json-invalid",
        title: "Authentication action request invalid",
        status: status.as_u16(),
        code: "AUTH_ACTION_JSON_INVALID",
        detail: "The request must be a bounded JSON object matching the authentication action schema",
        request_id: request_id(headers),
        errors: Box::default(),
        retry_after_seconds: None,
    }
}

fn session_id_problem(headers: &HeaderMap) -> Problem {
    Problem {
        type_uri: "urn:nodecontroll:problem:session-id-invalid",
        title: "Session identifier invalid",
        status: StatusCode::BAD_REQUEST.as_u16(),
        code: "SESSION_ID_INVALID",
        detail: "The session identifier must be a canonical UUID",
        request_id: request_id(headers),
        errors: Box::default(),
        retry_after_seconds: None,
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        http::{HeaderMap, StatusCode, header},
        response::IntoResponse,
    };
    use nodecontroll_application::AuthServiceError;

    use super::{auth_problem, remaining_cookie_max_age_seconds_at};

    #[test]
    fn login_errors_do_not_disclose_account_state() {
        let problem = auth_problem(AuthServiceError::InvalidCredentials, &HeaderMap::new());
        assert_eq!(problem.code, "INVALID_CREDENTIALS");
        assert_eq!(problem.status, 401);
    }

    #[test]
    fn invalid_session_problems_never_clear_shared_browser_cookies() {
        let response =
            auth_problem(AuthServiceError::SessionInvalid, &HeaderMap::new()).into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert!(
            response
                .headers()
                .get_all(header::SET_COOKIE)
                .iter()
                .next()
                .is_none()
        );
    }

    #[test]
    fn retry_after_is_never_zero() {
        let problem = auth_problem(
            AuthServiceError::RateLimited {
                retry_after_seconds: 0,
            },
            &HeaderMap::new(),
        );
        assert_eq!(problem.retry_after_seconds, Some(1));
    }

    #[test]
    fn rotated_cookie_never_outlives_the_server_absolute_deadline() {
        assert_eq!(
            remaining_cookie_max_age_seconds_at(3_600, 11_500, 10_000),
            1
        );
        assert_eq!(
            remaining_cookie_max_age_seconds_at(3_600, 10_999, 10_000),
            0
        );
        assert_eq!(
            remaining_cookie_max_age_seconds_at(60, 1_000_000, 10_000),
            60
        );
        assert_eq!(remaining_cookie_max_age_seconds_at(60, 9_000, 10_000), 0);
    }
}
