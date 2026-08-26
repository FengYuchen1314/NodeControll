use std::net::SocketAddr;

use axum::{
    Json,
    extract::{ConnectInfo, State, rejection::JsonRejection},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use nodecontroll_application::{
    ActorProjection, AuthServiceError, LoginCommand, LoginOutcome, MutatingSessionCredential,
    RequestContext, SessionCredential, SessionProjection,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use zeroize::Zeroizing;

use crate::{AppState, Problem, ResponseMeta, request_id, web_security};

#[derive(Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct LoginRequest {
    #[schema(min_length = 3, max_length = 32)]
    pub username: String,
    #[schema(format = Password, min_length = 1, max_length = 1024, write_only)]
    pub password: String,
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
    pub created_at_ms: i64,
    pub last_seen_at_ms: i64,
    pub idle_expires_at_ms: i64,
    pub absolute_expires_at_ms: i64,
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
            created_at_ms: session.created_at_ms,
            last_seen_at_ms: session.last_seen_at_ms,
            idle_expires_at_ms: session.idle_expires_at_ms,
            absolute_expires_at_ms: session.absolute_expires_at_ms,
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
        (status = 200, description = "Password authentication succeeded and host-only session cookies were issued", body = AuthenticatedEnvelope),
        (status = 400, description = "The JSON request is malformed", body = Problem, content_type = "application/problem+json"),
        (status = 401, description = "The supplied credentials are invalid", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The browser origin or host does not match the configured public origin", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "The control plane has not been initialized", body = Problem, content_type = "application/problem+json"),
        (status = 429, description = "A shared login limit is active", body = Problem, content_type = "application/problem+json"),
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
    let Json(mut request) = request.map_err(|error| auth_json_problem(error, &headers))?;
    let outcome = state
        .control_plane
        .login(LoginCommand {
            username: request.username,
            password: Zeroizing::new(std::mem::take(&mut request.password)),
            context,
        })
        .await
        .map_err(|error| auth_problem(error, &headers))?;
    login_response(outcome, state.session_cookie_max_age_seconds, &headers)
}

#[utoipa::path(
    get,
    path = "/api/v1/me",
    operation_id = "getCurrentActor",
    tag = "authentication",
    security(("sessionCookie" = [])),
    responses(
        (status = 200, description = "The current active actor and server-side session projection", body = AuthenticatedEnvelope),
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
    post,
    path = "/api/v1/auth/logout",
    operation_id = "logout",
    tag = "authentication",
    security(("sessionCookie" = [], "csrfHeader" = [])),
    responses(
        (status = 204, description = "The current server-side session was revoked and browser cookies were expired"),
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
    let mut response = authenticated_response(actor, session, headers);
    let session_cookie = web_security::session_set_cookie(&session_token, max_age_seconds);
    let csrf_cookie = web_security::csrf_set_cookie(&csrf_token, max_age_seconds);
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
    Ok(response)
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
            clear_session_cookies: false,
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
            clear_session_cookies: false,
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
            clear_session_cookies: true,
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
            clear_session_cookies: false,
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
            clear_session_cookies: false,
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
            clear_session_cookies: false,
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
        clear_session_cookies: false,
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
        clear_session_cookies: false,
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
        clear_session_cookies: false,
    }
}

#[cfg(test)]
mod tests {
    use axum::http::HeaderMap;
    use nodecontroll_application::AuthServiceError;

    use super::auth_problem;

    #[test]
    fn login_errors_do_not_disclose_account_state() {
        let problem = auth_problem(AuthServiceError::InvalidCredentials, &HeaderMap::new());
        assert_eq!(problem.code, "INVALID_CREDENTIALS");
        assert_eq!(problem.status, 401);
        assert!(!problem.clear_session_cookies);
    }

    #[test]
    fn invalid_sessions_expire_both_browser_cookies() {
        let problem = auth_problem(AuthServiceError::SessionInvalid, &HeaderMap::new());
        assert!(problem.clear_session_cookies);
        assert_eq!(problem.status, 401);
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
}
