/* Generated from the Rust OpenAPI document. Do not edit. */

export type ClientOptions = {
    baseUrl: `${string}://${string}` | (string & {});
};

export type ActorResponse = {
    capabilities: Array<string>;
    force_password_change: boolean;
    id: string;
    role: string;
    username: string;
};

export type AuthenticatedData = {
    actor: ActorResponse;
    session: SessionResponse;
};

export type AuthenticatedEnvelope = {
    data: AuthenticatedData;
    meta: ResponseMeta;
};

export type BootstrapCreated = {
    instance_id: string;
    owner_id: string;
};

export type BootstrapCreatedEnvelope = {
    data: BootstrapCreated;
    meta: ResponseMeta;
};

export type BootstrapEnvelope = {
    data: BootstrapInfo;
    meta: ResponseMeta;
};

export type BootstrapInfo = {
    initialized: boolean;
    login_methods: Array<string>;
    product: string;
    setup_capability_required: boolean;
};

export type BootstrapRequest = {
    instance_name: string;
    username: string;
};

export type ChangePasswordRequest = {
    /**
     * At least 12 Unicode scalar values and at most 1024 UTF-8 bytes.
     */
    new_password: string;
};

export type DependencyCheck = {
    code?: string | null;
    name: string;
    status: string;
};

export type FieldError = {
    code: string;
    message: string;
    pointer: string;
};

export type HealthResponse = {
    status: string;
};

export type LoginRequest = {
    username: string;
};

export type LogoutAllRequest = {
    keep_current: boolean;
};

export type LogoutAllRetainedData = {
    actor: ActorResponse;
    revoked_sessions: number;
    session: SessionResponse;
};

export type LogoutAllRetainedEnvelope = {
    data: LogoutAllRetainedData;
    meta: ResponseMeta;
};

export type PasswordChangedData = {
    actor: ActorResponse;
    revoked_sessions: number;
    session: SessionResponse;
};

export type PasswordChangedEnvelope = {
    data: PasswordChangedData;
    meta: ResponseMeta;
};

export type Problem = {
    code: string;
    detail: string;
    errors?: Array<FieldError>;
    request_id: string;
    status: number;
    title: string;
    type: string;
};

export type ReadinessResponse = {
    checks: Array<DependencyCheck>;
    status: string;
};

export type ReauthenticateRequest = {
    method: ReauthenticationMethod;
};

export type ReauthenticationMethod = 'password';

export type ResponseMeta = {
    api_version: string;
    request_id: string;
};

export type SessionResponse = {
    absolute_expires_at_ms: number;
    auth_level: string;
    created_at_ms: number;
    id: string;
    idle_expires_at_ms: number;
    last_seen_at_ms: number;
    recent_auth_expires_at_ms: number;
};

export type UserSessionResponse = {
    absolute_expires_at_ms: number;
    auth_level: string;
    created_at_ms: number;
    id: string;
    idle_expires_at_ms: number;
    is_current: boolean;
    last_seen_at_ms: number;
    recent_auth_expires_at_ms: number;
};

export type UserSessionsData = {
    sessions: Array<UserSessionResponse>;
};

export type UserSessionsEnvelope = {
    data: UserSessionsData;
    meta: ResponseMeta;
};

export type VersionEnvelope = {
    data: VersionInfo;
    meta: ResponseMeta;
};

export type VersionInfo = {
    product: string;
    started_at: string;
    version: string;
};

export type BootstrapRequestWritable = {
    instance_name: string;
    /**
     * At least 12 Unicode scalar values and at most 1024 UTF-8 bytes.
     */
    password: string;
    username: string;
};

export type LoginRequestWritable = {
    password: string;
    username: string;
};

export type ReauthenticateRequestWritable = {
    method: ReauthenticationMethod;
    password: string;
};

export type LoginData = {
    body: LoginRequestWritable;
    path?: never;
    query?: never;
    url: '/api/v1/auth/login';
};

export type LoginErrors = {
    /**
     * The JSON request is malformed
     */
    400: Problem;
    /**
     * The supplied credentials are invalid
     */
    401: Problem;
    /**
     * The browser origin or host does not match the configured public origin
     */
    403: Problem;
    /**
     * The control plane has not been initialized
     */
    409: Problem;
    /**
     * The JSON request exceeds the configured body limit
     */
    413: Problem;
    /**
     * The request media type is unsupported
     */
    415: Problem;
    /**
     * The JSON value does not match the login schema
     */
    422: Problem;
    /**
     * A shared login limit is active
     */
    429: Problem;
    /**
     * Authentication dependencies are unavailable
     */
    503: Problem;
};

export type LoginError = LoginErrors[keyof LoginErrors];

export type LoginResponses = {
    /**
     * Password authentication succeeded and host-only session cookies were issued
     */
    200: AuthenticatedEnvelope;
};

export type LoginResponse = LoginResponses[keyof LoginResponses];

export type LogoutData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/api/v1/auth/logout';
};

export type LogoutErrors = {
    /**
     * Required request metadata is malformed
     */
    400: Problem;
    /**
     * The Cookie header is oversized, ambiguous, or structurally malformed
     */
    401: Problem;
    /**
     * Origin, host, or double-submit CSRF verification failed
     */
    403: Problem;
    /**
     * Authentication dependencies are unavailable
     */
    503: Problem;
};

export type LogoutError = LogoutErrors[keyof LogoutErrors];

export type LogoutResponses = {
    /**
     * The current server-side session was revoked and browser cookies were expired
     */
    204: void;
};

export type LogoutResponse = LogoutResponses[keyof LogoutResponses];

export type LogoutAllData = {
    body: LogoutAllRequest;
    path?: never;
    query?: never;
    url: '/api/v1/auth/logout-all';
};

export type LogoutAllErrors = {
    /**
     * The request body is malformed
     */
    400: Problem;
    /**
     * The current session is invalid
     */
    401: Problem;
    /**
     * Origin, CSRF, or recent-auth verification failed
     */
    403: Problem;
    /**
     * The request body exceeds the configured limit
     */
    413: Problem;
    /**
     * The request media type is unsupported
     */
    415: Problem;
    /**
     * The JSON value does not match the logout-all schema
     */
    422: Problem;
    /**
     * Authentication dependencies are unavailable
     */
    503: Problem;
};

export type LogoutAllError = LogoutAllErrors[keyof LogoutAllErrors];

export type LogoutAllResponses = {
    /**
     * All sessions were revoked and this browser received a replacement session
     */
    200: LogoutAllRetainedEnvelope;
    /**
     * All sessions, including this browser, were revoked and cookies were expired
     */
    204: void;
};

export type LogoutAllResponse = LogoutAllResponses[keyof LogoutAllResponses];

export type ReauthenticateData = {
    body: ReauthenticateRequestWritable;
    path?: never;
    query?: never;
    url: '/api/v1/auth/reauth';
};

export type ReauthenticateErrors = {
    /**
     * The request body is malformed
     */
    400: Problem;
    /**
     * The current session is invalid
     */
    401: Problem;
    /**
     * Origin, CSRF, or the reauthentication proof is invalid
     */
    403: Problem;
    /**
     * The request body exceeds the configured limit
     */
    413: Problem;
    /**
     * The request media type is unsupported
     */
    415: Problem;
    /**
     * The JSON value does not match the reauthentication schema
     */
    422: Problem;
    /**
     * A shared authentication limit is active
     */
    429: Problem;
    /**
     * Authentication dependencies are unavailable
     */
    503: Problem;
};

export type ReauthenticateError = ReauthenticateErrors[keyof ReauthenticateErrors];

export type ReauthenticateResponses = {
    /**
     * The recent-auth proof succeeded and both browser credentials were rotated
     */
    200: AuthenticatedEnvelope;
};

export type ReauthenticateResponse = ReauthenticateResponses[keyof ReauthenticateResponses];

export type GetBootstrapStateData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/api/v1/bootstrap';
};

export type GetBootstrapStateErrors = {
    /**
     * The initialization projection is unavailable or stored bootstrap state is inconsistent
     */
    503: Problem;
};

export type GetBootstrapStateError = GetBootstrapStateErrors[keyof GetBootstrapStateErrors];

export type GetBootstrapStateResponses = {
    /**
     * Public initialization and login-method projection
     */
    200: BootstrapEnvelope;
};

export type GetBootstrapStateResponse = GetBootstrapStateResponses[keyof GetBootstrapStateResponses];

export type InitializeControlPlaneData = {
    body: BootstrapRequestWritable;
    headers: {
        /**
         * Short-lived setup capability read from the deployment token file
         */
        'x-nodecontroll-setup-token': string;
    };
    path?: never;
    query?: never;
    url: '/api/v1/bootstrap';
};

export type InitializeControlPlaneErrors = {
    /**
     * A bootstrap field is invalid or the JSON syntax is malformed
     */
    400: Problem;
    /**
     * The setup capability is missing, invalid, expired, or consumed
     */
    403: Problem;
    /**
     * The control plane is already initialized or the requested owner conflicts with stored identity data
     */
    409: Problem;
    /**
     * The bootstrap request exceeds the 16 KiB body limit
     */
    413: Problem;
    /**
     * The request does not use application/json
     */
    415: Problem;
    /**
     * The JSON object does not match the bootstrap request schema
     */
    422: Problem;
    /**
     * Bootstrap attempts are rate limited in this Master process
     */
    429: Problem;
    /**
     * Bootstrap dependencies are unavailable or stored bootstrap state is inconsistent
     */
    503: Problem;
};

export type InitializeControlPlaneError = InitializeControlPlaneErrors[keyof InitializeControlPlaneErrors];

export type InitializeControlPlaneResponses = {
    /**
     * Control-plane bootstrap completed atomically
     */
    201: BootstrapCreatedEnvelope;
};

export type InitializeControlPlaneResponse = InitializeControlPlaneResponses[keyof InitializeControlPlaneResponses];

export type GetCurrentActorData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/api/v1/me';
};

export type GetCurrentActorErrors = {
    /**
     * Required request metadata is malformed
     */
    400: Problem;
    /**
     * The session is absent, invalid, revoked, inactive, or expired
     */
    401: Problem;
    /**
     * The request host does not match the configured public origin
     */
    403: Problem;
    /**
     * Authentication dependencies are unavailable
     */
    503: Problem;
};

export type GetCurrentActorError = GetCurrentActorErrors[keyof GetCurrentActorErrors];

export type GetCurrentActorResponses = {
    /**
     * The current active actor and server-side session projection
     */
    200: AuthenticatedEnvelope;
};

export type GetCurrentActorResponse = GetCurrentActorResponses[keyof GetCurrentActorResponses];

export type ChangeCurrentPasswordData = {
    body: ChangePasswordRequest;
    path?: never;
    query?: never;
    url: '/api/v1/me/password';
};

export type ChangeCurrentPasswordErrors = {
    /**
     * The request body is malformed
     */
    400: Problem;
    /**
     * The current session is invalid
     */
    401: Problem;
    /**
     * Origin, CSRF, or recent-auth verification failed
     */
    403: Problem;
    /**
     * The request body exceeds the configured limit
     */
    413: Problem;
    /**
     * The request media type is unsupported
     */
    415: Problem;
    /**
     * The new password is rejected by policy or is unchanged
     */
    422: Problem;
    /**
     * Password hashing capacity is temporarily exhausted
     */
    429: Problem;
    /**
     * Authentication dependencies are unavailable
     */
    503: Problem;
};

export type ChangeCurrentPasswordError = ChangeCurrentPasswordErrors[keyof ChangeCurrentPasswordErrors];

export type ChangeCurrentPasswordResponses = {
    /**
     * The password changed, all sessions were revoked, and this browser received a replacement session
     */
    200: PasswordChangedEnvelope;
};

export type ChangeCurrentPasswordResponse = ChangeCurrentPasswordResponses[keyof ChangeCurrentPasswordResponses];

export type ListCurrentSessionsData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/api/v1/me/sessions';
};

export type ListCurrentSessionsErrors = {
    /**
     * Required request metadata is malformed
     */
    400: Problem;
    /**
     * The current session is invalid
     */
    401: Problem;
    /**
     * The request host is invalid
     */
    403: Problem;
    /**
     * Authentication dependencies are unavailable
     */
    503: Problem;
};

export type ListCurrentSessionsError = ListCurrentSessionsErrors[keyof ListCurrentSessionsErrors];

export type ListCurrentSessionsResponses = {
    /**
     * Active server-side sessions with coarse, secret-free projections
     */
    200: UserSessionsEnvelope;
};

export type ListCurrentSessionsResponse = ListCurrentSessionsResponses[keyof ListCurrentSessionsResponses];

export type RevokeCurrentUserSessionData = {
    body?: never;
    path: {
        /**
         * Session UUID
         */
        session_id: string;
    };
    query?: never;
    url: '/api/v1/me/sessions/{session_id}';
};

export type RevokeCurrentUserSessionErrors = {
    /**
     * The session identifier or required request metadata is invalid
     */
    400: Problem;
    /**
     * The current session is invalid
     */
    401: Problem;
    /**
     * Origin, CSRF, or recent-auth verification failed
     */
    403: Problem;
    /**
     * Authentication dependencies are unavailable
     */
    503: Problem;
};

export type RevokeCurrentUserSessionError = RevokeCurrentUserSessionErrors[keyof RevokeCurrentUserSessionErrors];

export type RevokeCurrentUserSessionResponses = {
    /**
     * While the caller session remains valid, the selected session is revoked or was already unavailable
     */
    204: void;
};

export type RevokeCurrentUserSessionResponse = RevokeCurrentUserSessionResponses[keyof RevokeCurrentUserSessionResponses];

export type GetSystemVersionData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/api/v1/system/version';
};

export type GetSystemVersionResponses = {
    /**
     * Product and API version
     */
    200: VersionEnvelope;
};

export type GetSystemVersionResponse = GetSystemVersionResponses[keyof GetSystemVersionResponses];

export type GetLivenessData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/healthz';
};

export type GetLivenessResponses = {
    /**
     * Master process is alive
     */
    200: HealthResponse;
};

export type GetLivenessResponse = GetLivenessResponses[keyof GetLivenessResponses];

export type GetReadinessData = {
    body?: never;
    path?: never;
    query?: never;
    url: '/readyz';
};

export type GetReadinessErrors = {
    /**
     * A required local dependency is unavailable
     */
    503: ReadinessResponse;
};

export type GetReadinessError = GetReadinessErrors[keyof GetReadinessErrors];

export type GetReadinessResponses = {
    /**
     * Required local dependencies are ready
     */
    200: ReadinessResponse;
};

export type GetReadinessResponse = GetReadinessResponses[keyof GetReadinessResponses];
