use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    Json, Router,
    body::Body,
    extract::{DefaultBodyLimit, State, rejection::JsonRejection},
    http::{HeaderMap, HeaderName, Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use utoipa::{OpenApi, ToSchema};
use zeroize::Zeroizing;

const REQUEST_ID_HEADER: HeaderName = HeaderName::from_static("x-request-id");
const SETUP_TOKEN_HEADER: HeaderName = HeaderName::from_static("x-nodecontroll-setup-token");

#[async_trait]
pub trait FoundationProbe: Send + Sync {
    async fn database_ready(&self) -> Result<(), ProbeError>;
    async fn secret_ready(&self) -> Result<(), ProbeError>;
    async fn is_initialized(&self) -> Result<bool, ProbeError>;
    async fn initialize(
        &self,
        command: BootstrapCommand,
    ) -> Result<BootstrapOutcome, BootstrapServiceError>;
}

pub struct BootstrapCommand {
    pub instance_name: String,
    pub username: String,
    pub password: Zeroizing<String>,
    pub setup_token: Zeroizing<String>,
}

pub struct BootstrapOutcome {
    pub instance_id: String,
    pub owner_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootstrapServiceError {
    InvalidInstanceName,
    InvalidUsername,
    InvalidPassword,
    CapabilityInvalid,
    AlreadyInitialized,
    IdentityConflict,
    InconsistentState,
    RateLimited,
    Unavailable,
}

#[derive(Debug, Clone, Copy)]
pub struct ProbeError {
    pub code: &'static str,
}

impl ProbeError {
    #[must_use]
    pub const fn database_unavailable() -> Self {
        Self {
            code: "DATABASE_UNAVAILABLE",
        }
    }

    #[must_use]
    pub const fn secret_unavailable() -> Self {
        Self {
            code: "SECRET_STORE_UNAVAILABLE",
        }
    }

    #[must_use]
    pub const fn bootstrap_state_inconsistent() -> Self {
        Self {
            code: "BOOTSTRAP_STATE_INCONSISTENT",
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    started_at: String,
    version: &'static str,
    probe: Arc<dyn FoundationProbe>,
}

impl AppState {
    pub fn new(
        version: &'static str,
        probe: Arc<dyn FoundationProbe>,
    ) -> Result<Self, time::error::Format> {
        Ok(Self {
            started_at: OffsetDateTime::now_utc().format(&Rfc3339)?,
            version,
            probe,
        })
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: &'static str,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DependencyCheck {
    pub name: &'static str,
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<&'static str>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReadinessResponse {
    pub status: &'static str,
    pub checks: Vec<DependencyCheck>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResponseMeta {
    pub api_version: &'static str,
    pub request_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VersionInfo {
    pub product: &'static str,
    pub version: &'static str,
    pub started_at: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VersionEnvelope {
    pub data: VersionInfo,
    pub meta: ResponseMeta,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BootstrapInfo {
    pub initialized: bool,
    pub product: &'static str,
    pub login_methods: Vec<&'static str>,
    pub setup_capability_required: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BootstrapEnvelope {
    pub data: BootstrapInfo,
    pub meta: ResponseMeta,
}

#[derive(Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct BootstrapRequest {
    #[schema(min_length = 1, max_length = 80)]
    pub instance_name: String,
    #[schema(min_length = 3, max_length = 32, pattern = "^[A-Za-z0-9_.-]{3,32}$")]
    pub username: String,
    /// At least 12 Unicode scalar values and at most 1024 UTF-8 bytes.
    #[schema(format = Password, min_length = 12, max_length = 1024, write_only)]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BootstrapCreated {
    pub instance_id: String,
    pub owner_id: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BootstrapCreatedEnvelope {
    pub data: BootstrapCreated,
    pub meta: ResponseMeta,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FieldError {
    pub pointer: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Problem {
    #[serde(rename = "type")]
    pub type_uri: &'static str,
    pub title: &'static str,
    pub status: u16,
    pub code: &'static str,
    pub detail: &'static str,
    pub request_id: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<FieldError>,
}

impl IntoResponse for Problem {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let retry_after = self.code == "BOOTSTRAP_RATE_LIMITED";
        let mut response = (status, Json(self)).into_response();
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("application/problem+json"),
        );
        if retry_after {
            response.headers_mut().insert(
                axum::http::header::RETRY_AFTER,
                axum::http::HeaderValue::from_static("2"),
            );
        }
        response
    }
}

#[utoipa::path(
    get,
    path = "/healthz",
    operation_id = "getLiveness",
    responses((status = 200, description = "Master process is alive", body = HealthResponse))
)]
pub async fn healthz() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[utoipa::path(
    get,
    path = "/readyz",
    operation_id = "getReadiness",
    responses(
        (status = 200, description = "Required local dependencies are ready", body = ReadinessResponse),
        (status = 503, description = "A required local dependency is unavailable", body = ReadinessResponse)
    )
)]
pub async fn readyz(State(state): State<AppState>) -> (StatusCode, Json<ReadinessResponse>) {
    let database = state.probe.database_ready().await;
    let secret = state.probe.secret_ready().await;
    let ready = database.is_ok() && secret.is_ok();
    let checks = vec![
        dependency_check("database", database),
        dependency_check("secret_store", secret),
    ];
    if ready {
        (
            StatusCode::OK,
            Json(ReadinessResponse {
                status: "ready",
                checks,
            }),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(ReadinessResponse {
                status: "not_ready",
                checks,
            }),
        )
    }
}

fn dependency_check(name: &'static str, result: Result<(), ProbeError>) -> DependencyCheck {
    match result {
        Ok(()) => DependencyCheck {
            name,
            status: "ready",
            code: None,
        },
        Err(error) => DependencyCheck {
            name,
            status: "unavailable",
            code: Some(error.code),
        },
    }
}

fn bootstrap_projection_problem(error: ProbeError, headers: &HeaderMap) -> Problem {
    if error.code == "BOOTSTRAP_STATE_INCONSISTENT" {
        bootstrap_problem(BootstrapServiceError::InconsistentState, headers)
    } else {
        Problem {
            type_uri: "urn:nodecontroll:problem:dependency-unavailable",
            title: "Dependency unavailable",
            status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
            code: error.code,
            detail: "The bootstrap projection is temporarily unavailable",
            request_id: request_id(headers),
            errors: Vec::new(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/bootstrap",
    operation_id = "getBootstrapState",
    tag = "system",
    responses(
        (status = 200, description = "Public initialization and login-method projection", body = BootstrapEnvelope),
        (status = 503, description = "The initialization projection is unavailable or stored bootstrap state is inconsistent", body = Problem, content_type = "application/problem+json")
    )
)]
pub async fn get_bootstrap(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<BootstrapEnvelope>, Problem> {
    let initialized = state
        .probe
        .is_initialized()
        .await
        .map_err(|error| bootstrap_projection_problem(error, &headers))?;
    Ok(Json(BootstrapEnvelope {
        data: BootstrapInfo {
            initialized,
            product: "NodeControll",
            login_methods: Vec::new(),
            setup_capability_required: !initialized,
        },
        meta: ResponseMeta {
            api_version: "v1",
            request_id: request_id(&headers),
        },
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/bootstrap",
    operation_id = "initializeControlPlane",
    tag = "system",
    request_body = BootstrapRequest,
    params(
        ("x-nodecontroll-setup-token" = String, Header, description = "Short-lived setup capability read from the deployment token file")
    ),
    responses(
        (status = 201, description = "Control-plane bootstrap completed atomically", body = BootstrapCreatedEnvelope),
        (status = 400, description = "A bootstrap field is invalid or the JSON syntax is malformed", body = Problem, content_type = "application/problem+json"),
        (status = 403, description = "The setup capability is missing, invalid, expired, or consumed", body = Problem, content_type = "application/problem+json"),
        (status = 409, description = "The control plane is already initialized or the requested owner conflicts with stored identity data", body = Problem, content_type = "application/problem+json"),
        (status = 413, description = "The bootstrap request exceeds the 16 KiB body limit", body = Problem, content_type = "application/problem+json"),
        (status = 415, description = "The request does not use application/json", body = Problem, content_type = "application/problem+json"),
        (status = 422, description = "The JSON object does not match the bootstrap request schema", body = Problem, content_type = "application/problem+json"),
        (status = 429, description = "Bootstrap attempts are rate limited in this Master process", body = Problem, content_type = "application/problem+json"),
        (status = 503, description = "Bootstrap dependencies are unavailable or stored bootstrap state is inconsistent", body = Problem, content_type = "application/problem+json")
    )
)]
pub async fn initialize_control_plane(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Result<Json<BootstrapRequest>, JsonRejection>,
) -> Result<(StatusCode, Json<BootstrapCreatedEnvelope>), Problem> {
    let Json(mut request) = request.map_err(|error| bootstrap_json_problem(error, &headers))?;
    let command = BootstrapCommand {
        instance_name: request.instance_name,
        username: request.username,
        password: Zeroizing::new(std::mem::take(&mut request.password)),
        setup_token: Zeroizing::new(
            headers
                .get(&SETUP_TOKEN_HEADER)
                .and_then(|value| value.to_str().ok())
                .unwrap_or_default()
                .to_owned(),
        ),
    };
    let outcome = state
        .probe
        .initialize(command)
        .await
        .map_err(|error| bootstrap_problem(error, &headers))?;
    Ok((
        StatusCode::CREATED,
        Json(BootstrapCreatedEnvelope {
            data: BootstrapCreated {
                instance_id: outcome.instance_id,
                owner_id: outcome.owner_id,
            },
            meta: ResponseMeta {
                api_version: "v1",
                request_id: request_id(&headers),
            },
        }),
    ))
}

fn bootstrap_problem(error: BootstrapServiceError, headers: &HeaderMap) -> Problem {
    let request_id = request_id(headers);
    match error {
        BootstrapServiceError::InvalidInstanceName => validation_problem(
            request_id,
            "/instance_name",
            "invalid_instance_name",
            "Instance name must contain 1 to 80 Unicode scalar values after trimming and no control characters",
        ),
        BootstrapServiceError::InvalidUsername => validation_problem(
            request_id,
            "/username",
            "invalid_username",
            "Username must contain 3 to 32 allowed characters",
        ),
        BootstrapServiceError::InvalidPassword => validation_problem(
            request_id,
            "/password",
            "invalid_password",
            "Password does not satisfy the configured policy",
        ),
        BootstrapServiceError::CapabilityInvalid => Problem {
            type_uri: "urn:nodecontroll:problem:setup-capability-invalid",
            title: "Setup capability invalid",
            status: StatusCode::FORBIDDEN.as_u16(),
            code: "SETUP_CAPABILITY_INVALID",
            detail: "Supply the unexpired one-time capability from the deployment setup-token file",
            request_id,
            errors: Vec::new(),
        },
        BootstrapServiceError::AlreadyInitialized => Problem {
            type_uri: "urn:nodecontroll:problem:already-initialized",
            title: "Already initialized",
            status: StatusCode::CONFLICT.as_u16(),
            code: "ALREADY_INITIALIZED",
            detail: "Control-plane bootstrap has already completed",
            request_id,
            errors: Vec::new(),
        },
        BootstrapServiceError::IdentityConflict => Problem {
            type_uri: "urn:nodecontroll:problem:identity-conflict",
            title: "Identity conflict",
            status: StatusCode::CONFLICT.as_u16(),
            code: "IDENTITY_CONFLICT",
            detail: "The requested initial owner conflicts with stored identity data",
            request_id,
            errors: Vec::new(),
        },
        BootstrapServiceError::InconsistentState => Problem {
            type_uri: "urn:nodecontroll:problem:bootstrap-state-inconsistent",
            title: "Bootstrap state inconsistent",
            status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
            code: "BOOTSTRAP_STATE_INCONSISTENT",
            detail: "Stored control-plane records require operator recovery",
            request_id,
            errors: Vec::new(),
        },
        BootstrapServiceError::RateLimited => Problem {
            type_uri: "urn:nodecontroll:problem:rate-limited",
            title: "Too many requests",
            status: StatusCode::TOO_MANY_REQUESTS.as_u16(),
            code: "BOOTSTRAP_RATE_LIMITED",
            detail: "Wait before attempting control-plane initialization again",
            request_id,
            errors: Vec::new(),
        },
        BootstrapServiceError::Unavailable => Problem {
            type_uri: "urn:nodecontroll:problem:dependency-unavailable",
            title: "Dependency unavailable",
            status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
            code: "BOOTSTRAP_UNAVAILABLE",
            detail: "The control plane could not complete initialization",
            request_id,
            errors: Vec::new(),
        },
    }
}

fn bootstrap_json_problem(error: JsonRejection, headers: &HeaderMap) -> Problem {
    let (status, type_uri, title, code, detail) = match error.status() {
        StatusCode::PAYLOAD_TOO_LARGE => (
            StatusCode::PAYLOAD_TOO_LARGE,
            "urn:nodecontroll:problem:payload-too-large",
            "Payload too large",
            "PAYLOAD_TOO_LARGE",
            "The bootstrap request exceeds the 16 KiB body limit",
        ),
        StatusCode::UNSUPPORTED_MEDIA_TYPE => (
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "urn:nodecontroll:problem:unsupported-media-type",
            "Unsupported media type",
            "UNSUPPORTED_MEDIA_TYPE",
            "Bootstrap requests require application/json",
        ),
        StatusCode::UNPROCESSABLE_ENTITY => (
            StatusCode::UNPROCESSABLE_ENTITY,
            "urn:nodecontroll:problem:json-shape-invalid",
            "JSON shape invalid",
            "BOOTSTRAP_JSON_SHAPE_INVALID",
            "The JSON object is missing required fields, contains unknown fields, or has invalid field types",
        ),
        _ => (
            StatusCode::BAD_REQUEST,
            "urn:nodecontroll:problem:json-invalid",
            "JSON invalid",
            "BOOTSTRAP_JSON_INVALID",
            "The bootstrap request body is not valid JSON",
        ),
    };
    Problem {
        type_uri,
        title,
        status: status.as_u16(),
        code,
        detail,
        request_id: request_id(headers),
        errors: Vec::new(),
    }
}

fn validation_problem(
    request_id: String,
    pointer: &'static str,
    code: &'static str,
    message: &'static str,
) -> Problem {
    Problem {
        type_uri: "urn:nodecontroll:problem:validation",
        title: "Validation failed",
        status: StatusCode::BAD_REQUEST.as_u16(),
        code: "VALIDATION_FAILED",
        detail: "One or more request fields are invalid",
        request_id,
        errors: vec![FieldError {
            pointer: pointer.to_owned(),
            code: code.to_owned(),
            message: message.to_owned(),
        }],
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/system/version",
    operation_id = "getSystemVersion",
    tag = "system",
    responses((status = 200, description = "Product and API version", body = VersionEnvelope))
)]
pub async fn system_version(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Json<VersionEnvelope> {
    Json(VersionEnvelope {
        data: VersionInfo {
            product: "NodeControll",
            version: state.version,
            started_at: state.started_at,
        },
        meta: ResponseMeta {
            api_version: "v1",
            request_id: request_id(&headers),
        },
    })
}

#[derive(OpenApi)]
#[openapi(
    info(title = "NodeControll API", version = "0.1.0"),
    paths(healthz, readyz, get_bootstrap, initialize_control_plane, system_version),
    components(schemas(
        HealthResponse, DependencyCheck, ReadinessResponse, ResponseMeta, VersionInfo,
        VersionEnvelope, BootstrapInfo, BootstrapEnvelope, BootstrapRequest, BootstrapCreated,
        BootstrapCreatedEnvelope, FieldError, Problem
    )),
    tags((name = "system", description = "Instance liveness and compatibility"))
)]
struct ApiDocument;

#[must_use]
pub fn openapi() -> utoipa::openapi::OpenApi {
    ApiDocument::openapi()
}

async fn openapi_json() -> Json<serde_json::Value> {
    let document = serde_json::to_value(openapi()).unwrap_or_else(|error| {
        serde_json::json!({
            "openapi": "3.1.0",
            "info": {"title": "NodeControll API serialization error", "version": "0"},
            "x-error": error.to_string()
        })
    });
    Json(document)
}

async fn not_found(headers: HeaderMap) -> Problem {
    Problem {
        type_uri: "urn:nodecontroll:problem:not-found",
        title: "Not found",
        status: StatusCode::NOT_FOUND.as_u16(),
        code: "ROUTE_NOT_FOUND",
        detail: "The requested route does not exist",
        request_id: request_id(&headers),
        errors: Vec::new(),
    }
}

fn request_id(headers: &HeaderMap) -> String {
    headers
        .get(&REQUEST_ID_HEADER)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unavailable")
        .to_owned()
}

async fn discard_untrusted_request_id(mut request: Request<Body>, next: Next) -> Response {
    request.headers_mut().remove(&REQUEST_ID_HEADER);
    next.run(request).await
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route(
            "/api/v1/bootstrap",
            get(get_bootstrap).post(initialize_control_plane),
        )
        .route("/api/v1/system/version", get(system_version))
        .route("/api-docs/openapi.json", get(openapi_json))
        .fallback(not_found)
        .with_state(state)
        .layer(DefaultBodyLimit::max(16 * 1024))
        .layer(PropagateRequestIdLayer::new(REQUEST_ID_HEADER))
        .layer(SetRequestIdLayer::new(REQUEST_ID_HEADER, MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                tracing::info_span!(
                    "http.request",
                    method = %request.method(),
                    path = %request.uri().path()
                )
            }),
        )
        .layer(middleware::from_fn(discard_untrusted_request_id))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;
    use axum::{Json, extract::State, http::HeaderMap};

    use super::{
        AppState, BootstrapCommand, BootstrapOutcome, BootstrapRequest, BootstrapServiceError,
        FoundationProbe, ProbeError, bootstrap_problem, bootstrap_projection_problem,
        get_bootstrap, healthz, initialize_control_plane, openapi, readyz, system_version,
    };

    struct TestProbe {
        ready: bool,
        initialized: bool,
    }

    #[async_trait]
    impl FoundationProbe for TestProbe {
        async fn database_ready(&self) -> Result<(), ProbeError> {
            self.ready
                .then_some(())
                .ok_or_else(ProbeError::database_unavailable)
        }

        async fn is_initialized(&self) -> Result<bool, ProbeError> {
            Ok(self.initialized)
        }

        async fn secret_ready(&self) -> Result<(), ProbeError> {
            self.ready
                .then_some(())
                .ok_or_else(ProbeError::secret_unavailable)
        }

        async fn initialize(
            &self,
            _command: BootstrapCommand,
        ) -> Result<BootstrapOutcome, BootstrapServiceError> {
            if self.initialized {
                Err(BootstrapServiceError::AlreadyInitialized)
            } else {
                Ok(BootstrapOutcome {
                    instance_id: "01900000-0000-7000-8000-000000000001".to_owned(),
                    owner_id: "01900000-0000-7000-8000-000000000002".to_owned(),
                })
            }
        }
    }

    fn state(ready: bool, initialized: bool) -> AppState {
        let state = AppState::new("test-version", Arc::new(TestProbe { ready, initialized }));
        assert!(state.is_ok());
        state.unwrap_or_else(|_| unreachable!("checked above"))
    }

    #[tokio::test]
    async fn health_is_explicit() {
        assert_eq!(healthz().await.0.status, "ok");
    }

    #[tokio::test]
    async fn readiness_reflects_dependency_failure() {
        assert_eq!(
            readyz(State(state(true, false))).await.0,
            axum::http::StatusCode::OK
        );
        assert_eq!(
            readyz(State(state(false, false))).await.0,
            axum::http::StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[tokio::test]
    async fn bootstrap_projection_is_public_and_minimal() {
        let response = get_bootstrap(State(state(true, false)), HeaderMap::new()).await;
        assert!(response.is_ok());
        if let Ok(response) = response {
            assert!(!response.0.data.initialized);
            assert!(response.0.data.login_methods.is_empty());
        }
    }

    #[tokio::test]
    async fn first_bootstrap_returns_created_ids() {
        let response = initialize_control_plane(
            State(state(true, false)),
            HeaderMap::new(),
            Ok(Json(BootstrapRequest {
                instance_name: "My instance".to_owned(),
                username: "owner".to_owned(),
                password: "a sufficiently long password".to_owned(),
            })),
        )
        .await;
        assert!(matches!(
            response,
            Ok((axum::http::StatusCode::CREATED, envelope))
                if envelope.0.data.owner_id.ends_with("0002")
        ));
    }

    #[tokio::test]
    async fn version_endpoint_uses_v1_envelope() {
        let response = system_version(State(state(true, false)), HeaderMap::new())
            .await
            .0;
        assert_eq!(response.data.version, "test-version");
        assert_eq!(response.meta.api_version, "v1");
    }

    #[test]
    fn openapi_has_all_foundation_paths() {
        let document = openapi();
        let paths = document.paths.paths;
        assert!(paths.contains_key("/healthz"));
        assert!(paths.contains_key("/readyz"));
        assert!(paths.contains_key("/api/v1/bootstrap"));
        assert!(paths.contains_key("/api/v1/system/version"));
    }

    #[test]
    fn openapi_declares_problem_details_media_type() {
        const EXPECTED_CONFLICT_DESCRIPTION: &str = "The control plane is already initialized or the requested owner conflicts with stored identity data";
        let document = serde_json::to_value(openapi());
        assert!(document.is_ok());
        if let Ok(document) = document {
            for pointer in [
                "/paths/~1api~1v1~1bootstrap/get/responses/503/content/application~1problem+json",
                "/paths/~1api~1v1~1bootstrap/post/responses/400/content/application~1problem+json",
                "/paths/~1api~1v1~1bootstrap/post/responses/403/content/application~1problem+json",
                "/paths/~1api~1v1~1bootstrap/post/responses/409/content/application~1problem+json",
                "/paths/~1api~1v1~1bootstrap/post/responses/413/content/application~1problem+json",
                "/paths/~1api~1v1~1bootstrap/post/responses/415/content/application~1problem+json",
                "/paths/~1api~1v1~1bootstrap/post/responses/422/content/application~1problem+json",
                "/paths/~1api~1v1~1bootstrap/post/responses/429/content/application~1problem+json",
                "/paths/~1api~1v1~1bootstrap/post/responses/503/content/application~1problem+json",
            ] {
                assert!(
                    document.pointer(pointer).is_some(),
                    "missing Problem Details content at {pointer}"
                );
            }
            let conflict_description = document
                .pointer("/paths/~1api~1v1~1bootstrap/post/responses/409/description")
                .and_then(serde_json::Value::as_str);
            assert_eq!(conflict_description, Some(EXPECTED_CONFLICT_DESCRIPTION));
        }
    }

    #[test]
    fn inconsistent_bootstrap_problem_keeps_stable_code() {
        let post_problem =
            bootstrap_problem(BootstrapServiceError::InconsistentState, &HeaderMap::new());
        assert_eq!(
            post_problem.status,
            axum::http::StatusCode::SERVICE_UNAVAILABLE.as_u16()
        );
        assert_eq!(post_problem.code, "BOOTSTRAP_STATE_INCONSISTENT");
        let get_problem = bootstrap_projection_problem(
            ProbeError::bootstrap_state_inconsistent(),
            &HeaderMap::new(),
        );
        assert_eq!(get_problem.code, "BOOTSTRAP_STATE_INCONSISTENT");
        assert_eq!(get_problem.type_uri, post_problem.type_uri);
    }
}
