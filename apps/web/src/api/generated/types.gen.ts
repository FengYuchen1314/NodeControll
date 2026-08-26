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

export type ResponseMeta = {
    api_version: string;
    request_id: string;
};

export type SessionResponse = {
    absolute_expires_at_ms: number;
    created_at_ms: number;
    id: string;
    idle_expires_at_ms: number;
    last_seen_at_ms: number;
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
