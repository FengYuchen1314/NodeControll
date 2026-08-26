//! Application use-case boundary for the NodeControll control plane.
//!
//! HTTP adapters depend on [`ControlPlane`]; persistence and cryptographic details stay behind the
//! concrete [`ControlPlaneApplication`]. The current authenticated slice includes password login,
//! password recent-auth, self-service password rotation, and server-side session management. MFA,
//! API tokens, full authorization, and user administration extend this boundary later in WP-02.

use std::{
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use nodecontroll_domain::{
    BaselineCapabilities, EntityId, Instance, InstanceName, PasswordHash, PrincipalLabel, Revision,
    SubscriptionBehaviorSettings, UserAccount, UserRole, UserStatus, Username,
};
use nodecontroll_identity::{
    CsrfToken, LoginPasswordWorkPlan, PasswordError, PasswordService, SessionToken,
    SessionTokenPair, SetupCapability,
};
use nodecontroll_persistence::{
    AuthLevel, AuthSessionSummary, AuthenticatedSession, Database, LoginAttemptReservation,
    LoginRateDecision, LoginSecurityReason, NewAuthSession, NewLoginSecurityEvent, NewRecoveryCode,
    NewRecoveryCodeSet, NewSecretRecord, PasswordChangeRotation, PersistenceError,
    RecoveryCodeConsumption, RecoveryCodeReplacement, SessionAuthentication,
    SessionAuthenticationOutcome, SessionRevocationReason, UserSessionRevocation,
};
use nodecontroll_secrets::{
    KeyedDigestPurpose, Keyring, RecoveryCode, SecretBinding, SecretError, generate_recovery_codes,
};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use tokio::sync::{Mutex, Semaphore};
use zeroize::Zeroizing;

mod auth_challenge;

pub use auth_challenge::{
    AuthChallengePort, AuthChallengePortError, AuthChallengeReservationOutcome,
    AuthChallengeRotationResumeOutcome, AuthChallengeRotationTransactionClaim,
    AuthChallengeRotationTransactionPort, AuthChallengeService, AuthChallengeServiceError,
    AuthChallengeVerificationClaim, IssueAuthChallengeCommand, IssuedAuthChallenge,
    PresentAuthChallengeCommand, VerifiedAuthChallengeEvidence, VerifiedAuthChallengeOutcome,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthPolicy {
    pub session_idle: Duration,
    pub session_absolute: Duration,
    pub session_touch_interval: Duration,
    pub recent_auth: Duration,
    pub login_window: Duration,
    pub login_block: Duration,
    pub login_account_limit: u32,
    pub login_ip_limit: u32,
    pub login_global_limit: u32,
    pub password_hash_concurrency: usize,
}

impl AuthPolicy {
    fn validate(self) -> Result<Self, AuthPolicyError> {
        if self.session_touch_interval.is_zero()
            || self.session_touch_interval >= self.session_idle
            || self.session_idle > self.session_absolute
            || self.recent_auth < Duration::from_secs(60)
            || self.recent_auth > Duration::from_secs(3_600)
            || self.recent_auth > self.session_absolute
            || self.login_window.is_zero()
            || self.login_block.is_zero()
            || self.login_block < self.login_window
            || self.login_account_limit == 0
            || self.login_account_limit > self.login_ip_limit
            || self.login_ip_limit > self.login_global_limit
            || !(1..=64).contains(&self.password_hash_concurrency)
            || duration_ms(self.session_touch_interval).is_err()
            || duration_ms(self.session_idle).is_err()
            || duration_ms(self.session_absolute).is_err()
            || duration_ms(self.recent_auth).is_err()
            || duration_ms(self.login_window).is_err()
            || duration_ms(self.login_block).is_err()
        {
            return Err(AuthPolicyError::Invalid);
        }
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum AuthPolicyError {
    #[error("authentication policy violates a resource or lifetime invariant")]
    Invalid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientNetwork {
    address: IpAddr,
    prefix_length: u8,
}

impl ClientNetwork {
    pub fn new(address: IpAddr, prefix_length: u8) -> Result<Self, ClientNetworkError> {
        let address = match address {
            IpAddr::V4(address) => {
                if prefix_length > 32 {
                    return Err(ClientNetworkError::InvalidPrefixLength);
                }
                let mask = if prefix_length == 0 {
                    0
                } else {
                    u32::MAX << (32 - prefix_length)
                };
                IpAddr::V4((u32::from(address) & mask).into())
            }
            IpAddr::V6(address) => {
                if prefix_length > 128 {
                    return Err(ClientNetworkError::InvalidPrefixLength);
                }
                let mask = if prefix_length == 0 {
                    0
                } else {
                    u128::MAX << (128 - prefix_length)
                };
                IpAddr::V6((u128::from(address) & mask).into())
            }
        };
        Ok(Self {
            address,
            prefix_length,
        })
    }

    pub fn from_client_ip(address: IpAddr) -> Self {
        match address {
            IpAddr::V4(address) => Self {
                address: IpAddr::V4(address),
                prefix_length: 32,
            },
            IpAddr::V6(address) => Self {
                address: IpAddr::V6((u128::from(address) & (u128::MAX << 64)).into()),
                prefix_length: 64,
            },
        }
    }

    #[must_use]
    pub const fn address(&self) -> IpAddr {
        self.address
    }

    #[must_use]
    pub const fn prefix_length(&self) -> u8 {
        self.prefix_length
    }

    #[must_use]
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut encoded = Vec::with_capacity(18);
        match self.address {
            IpAddr::V4(address) => {
                encoded.push(4);
                encoded.extend_from_slice(&address.octets());
            }
            IpAddr::V6(address) => {
                encoded.push(6);
                encoded.extend_from_slice(&address.octets());
            }
        }
        encoded.push(self.prefix_length);
        encoded
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClientNetworkError {
    InvalidPrefixLength,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestContext {
    pub request_id: String,
    pub client: ClientNetwork,
    pub user_agent: String,
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
    pub one_time_recovery_codes: Vec<Zeroizing<String>>,
}

pub struct LoginCommand {
    pub username: String,
    pub password: Zeroizing<String>,
    pub context: RequestContext,
}

pub struct SessionCredential {
    pub session_token: Zeroizing<String>,
    pub csrf_token: Option<Zeroizing<String>>,
    pub context: RequestContext,
}

pub struct MutatingSessionCredential {
    pub session_token: Zeroizing<String>,
    pub csrf_token: Zeroizing<String>,
    pub context: RequestContext,
}

pub struct LoginOutcome {
    pub actor: ActorProjection,
    pub session: SessionProjection,
    pub session_token: SessionToken,
    pub csrf_token: CsrfToken,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActorProjection {
    pub id: EntityId,
    pub username: String,
    pub role: UserRole,
    pub capabilities: BaselineCapabilities,
    pub force_password_change: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionProjection {
    pub id: EntityId,
    pub auth_level: String,
    pub created_at_ms: i64,
    pub last_seen_at_ms: i64,
    pub idle_expires_at_ms: i64,
    pub absolute_expires_at_ms: i64,
    pub recent_auth_expires_at_ms: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProbeError {
    DatabaseUnavailable,
    SecretUnavailable,
    BootstrapStateInconsistent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthServiceError {
    InvalidCredentials,
    InvalidProof,
    InvalidNewPassword,
    PasswordUnchanged,
    RecentAuthRequired,
    PasswordChangeRequired,
    RateLimited { retry_after_seconds: u64 },
    SessionInvalid,
    CsrfInvalid,
    NotInitialized,
    RecoveryCodesUnavailable,
    Unavailable,
}

/// Every application use case that consumes a browser session must declare its action class.
/// This keeps forced-password-change enforcement at the authenticated boundary instead of relying
/// on individual HTTP handlers or hidden navigation items.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthenticatedAction {
    ReadOwnIdentity,
    Reauthenticate,
    ChangePassword,
    ManageOwnSessions,
    ManageRecoveryCodes,
    SignOut,
    ProductAccess,
}

impl AuthenticatedAction {
    const fn allowed_during_forced_password_change(self) -> bool {
        matches!(
            self,
            Self::ReadOwnIdentity
                | Self::Reauthenticate
                | Self::ChangePassword
                | Self::ManageOwnSessions
                | Self::SignOut
        )
    }
}

pub struct ReauthenticateCommand {
    pub credential: MutatingSessionCredential,
    pub password: Zeroizing<String>,
}

pub struct ChangePasswordCommand {
    pub credential: MutatingSessionCredential,
    pub new_password: Zeroizing<String>,
}

pub struct PasswordChangeOutcome {
    pub login: LoginOutcome,
    pub revoked_sessions: u64,
}

pub struct LogoutAllCommand {
    pub credential: MutatingSessionCredential,
    pub keep_current: bool,
}

pub enum LogoutAllOutcome {
    CurrentRetained {
        login: LoginOutcome,
        revoked_sessions: u64,
    },
    SignedOut {
        revoked_sessions: u64,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserSessionProjection {
    pub session: SessionProjection,
    pub is_current: bool,
}

pub struct RevokeSessionCommand {
    pub credential: MutatingSessionCredential,
    pub target_session_id: EntityId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecoveryCodeSummary {
    pub set_version: u64,
    pub total_count: u8,
    pub remaining_count: u8,
    pub created_at_ms: i64,
}

pub struct RegenerateRecoveryCodesCommand {
    pub credential: MutatingSessionCredential,
}

pub struct RecoveryCodesCreated {
    pub set_version: u64,
    pub one_time_recovery_codes: Vec<Zeroizing<String>>,
    pub created_at_ms: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RevokeSessionOutcome {
    pub revoked_current: bool,
}

#[async_trait]
pub trait ControlPlane: Send + Sync {
    async fn database_ready(&self) -> Result<(), ProbeError>;
    async fn secret_ready(&self) -> Result<(), ProbeError>;
    async fn is_initialized(&self) -> Result<bool, ProbeError>;
    async fn initialize(
        &self,
        command: BootstrapCommand,
    ) -> Result<BootstrapOutcome, BootstrapServiceError>;
    async fn login(&self, command: LoginCommand) -> Result<LoginOutcome, AuthServiceError>;
    async fn reauthenticate(
        &self,
        command: ReauthenticateCommand,
    ) -> Result<LoginOutcome, AuthServiceError>;
    async fn change_password(
        &self,
        command: ChangePasswordCommand,
    ) -> Result<PasswordChangeOutcome, AuthServiceError>;
    async fn current_actor(
        &self,
        credential: SessionCredential,
    ) -> Result<(ActorProjection, SessionProjection), AuthServiceError>;
    async fn logout(&self, credential: MutatingSessionCredential) -> Result<(), AuthServiceError>;
    async fn logout_all(
        &self,
        command: LogoutAllCommand,
    ) -> Result<LogoutAllOutcome, AuthServiceError>;
    async fn list_sessions(
        &self,
        credential: SessionCredential,
    ) -> Result<Vec<UserSessionProjection>, AuthServiceError>;
    async fn revoke_session(
        &self,
        command: RevokeSessionCommand,
    ) -> Result<RevokeSessionOutcome, AuthServiceError>;
    async fn recovery_code_summary(
        &self,
        credential: SessionCredential,
    ) -> Result<RecoveryCodeSummary, AuthServiceError>;
    async fn regenerate_recovery_codes(
        &self,
        command: RegenerateRecoveryCodesCommand,
    ) -> Result<RecoveryCodesCreated, AuthServiceError>;
}

pub struct ControlPlaneApplication {
    database: Database,
    keyring: Keyring,
    password_service: PasswordService,
    dummy_password_hash: PasswordHash,
    setup_capability: Option<SetupCapability>,
    last_bootstrap_attempt: Mutex<Option<std::time::Instant>>,
    auth_policy: AuthPolicy,
    password_hash_slots: Arc<Semaphore>,
}

#[derive(Debug, thiserror::Error)]
pub enum RootKeyCanaryError {
    #[error("persistent root-key canary storage failed: {0}")]
    Persistence(#[from] PersistenceError),
    #[error("persistent root-key canary verification failed: {0}")]
    Secret(#[from] SecretError),
    #[error("system clock is outside the supported timestamp range")]
    Clock,
}

/// Creates or verifies the persisted root-key canary before HTTP bind. If the canary is encrypted
/// by a configured old key, it is atomically rotated to the current key so the old key can later be
/// removed from the finite keyring.
pub async fn initialize_root_key_canary(
    database: &Database,
    keyring: &Keyring,
) -> Result<(), RootKeyCanaryError> {
    let now_ms = unix_time_ms().map_err(|_| RootKeyCanaryError::Clock)?;
    let candidate = NewSecretRecord {
        id: EntityId::new(),
        binding: SecretBinding::root_key_canary(),
        envelope: keyring.new_canary_envelope()?,
        created_at_ms: now_ms,
        rotated_from: None,
    };
    let stored = database.ensure_secret_record(&candidate).await?;
    keyring.verify_canary(&stored.envelope)?;
    if stored.envelope.key_version == keyring.key_version() {
        return Ok(());
    }
    let replacement = NewSecretRecord {
        id: EntityId::new(),
        binding: stored.binding,
        envelope: keyring.new_canary_envelope()?,
        created_at_ms: now_ms,
        rotated_from: Some(stored.id),
    };
    match database
        .rotate_secret_record(&stored, &replacement, now_ms)
        .await
    {
        Ok(rotated) => keyring.verify_canary(&rotated.envelope)?,
        Err(PersistenceError::SecretRecordConflict) => {
            let winner = database
                .active_secret_record(SecretBinding::root_key_canary())
                .await?
                .ok_or(PersistenceError::InvalidStoredSecretRecord)?;
            keyring.verify_canary(&winner.envelope)?;
        }
        Err(error) => return Err(error.into()),
    }
    Ok(())
}

struct PasswordAttemptDigests {
    key_version: u32,
    account_hmac: [u8; 32],
    ip_prefix_hmac: [u8; 32],
}

struct PlannedLoginPasswordWork {
    plan: LoginPasswordWorkPlan,
    selected_credential: PasswordHash,
}

fn plan_login_password_work(
    selected_credential: Option<&PasswordHash>,
    current_dummy: &PasswordHash,
) -> PlannedLoginPasswordWork {
    PlannedLoginPasswordWork {
        plan: PasswordService::login_work_plan(),
        selected_credential: selected_credential
            .cloned()
            .unwrap_or_else(|| current_dummy.clone()),
    }
}

struct PreparedSessionRotation {
    session: NewAuthSession,
    token_pair: SessionTokenPair,
}

impl ControlPlaneApplication {
    pub fn new(
        database: Database,
        keyring: Keyring,
        password_service: PasswordService,
        dummy_password_hash: PasswordHash,
        setup_capability: Option<SetupCapability>,
        auth_policy: AuthPolicy,
    ) -> Result<Arc<Self>, AuthPolicyError> {
        let auth_policy = auth_policy.validate()?;
        if !matches!(
            password_service.needs_rehash(&dummy_password_hash),
            Ok(false)
        ) {
            return Err(AuthPolicyError::Invalid);
        }
        let password_hash_slots = Arc::new(Semaphore::new(auth_policy.password_hash_concurrency));
        Ok(Arc::new(Self {
            database,
            keyring,
            password_service,
            dummy_password_hash,
            setup_capability,
            last_bootstrap_attempt: Mutex::new(None),
            auth_policy,
            password_hash_slots,
        }))
    }

    fn actor_projection(session: &AuthenticatedSession) -> ActorProjection {
        ActorProjection {
            id: session.user_id,
            username: session.username.as_str().to_owned(),
            role: session.role,
            capabilities: actor_capabilities(session.role, session.force_password_change),
            force_password_change: session.force_password_change,
        }
    }

    fn prepare_recovery_code_set(
        &self,
        created_at_ms: i64,
    ) -> Result<(NewRecoveryCodeSet, Vec<Zeroizing<String>>), AuthServiceError> {
        let generated = generate_recovery_codes().map_err(|_| AuthServiceError::Unavailable)?;
        let mut records = Vec::with_capacity(generated.len());
        let mut presented = Vec::with_capacity(generated.len());
        for code in generated {
            let digest = self
                .keyring
                .keyed_digest(KeyedDigestPurpose::RecoveryCode, code.normalized_bytes())
                .map_err(|_| AuthServiceError::Unavailable)?;
            records.push(NewRecoveryCode {
                id: EntityId::new(),
                digest_key_version: digest.key_version,
                code_hmac: digest.digest,
            });
            presented.push(Zeroizing::new(code.presented().to_owned()));
        }
        Ok((
            NewRecoveryCodeSet {
                created_at_ms,
                codes: records,
            },
            presented,
        ))
    }

    /// Application boundary used by the later password-recovery flow. The repository performs the
    /// conditional consume, so two concurrent submissions of the same code cannot both succeed.
    pub async fn consume_recovery_code(
        &self,
        user_id: EntityId,
        presented: &str,
    ) -> Result<bool, AuthServiceError> {
        let code =
            RecoveryCode::parse_presented(presented).map_err(|_| AuthServiceError::InvalidProof)?;
        let now_ms = unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?;
        for key_version in self.keyring.key_versions() {
            let digest = self
                .keyring
                .keyed_digest_for_version(
                    key_version,
                    KeyedDigestPurpose::RecoveryCode,
                    code.normalized_bytes(),
                )
                .map_err(|_| AuthServiceError::Unavailable)?;
            if self
                .database
                .consume_recovery_code(&RecoveryCodeConsumption {
                    user_id,
                    digest_key_version: digest.key_version,
                    code_hmac: digest.digest,
                    now_ms,
                })
                .await
                .map_err(|_| AuthServiceError::Unavailable)?
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn session_projection(&self, session: &AuthSessionSummary) -> SessionProjection {
        let recent_auth_milliseconds =
            i64::try_from(self.auth_policy.recent_auth.as_millis()).unwrap_or(i64::MAX);
        SessionProjection {
            id: session.id,
            auth_level: session.auth_level.as_str().to_owned(),
            created_at_ms: session.created_at_ms,
            last_seen_at_ms: session.last_seen_at_ms,
            idle_expires_at_ms: session.idle_expires_at_ms,
            absolute_expires_at_ms: session.absolute_expires_at_ms,
            recent_auth_expires_at_ms: session
                .recent_auth_at_ms
                .saturating_add(recent_auth_milliseconds)
                .min(session.absolute_expires_at_ms),
        }
    }

    fn new_session_projection(&self, session: &NewAuthSession) -> SessionProjection {
        let recent_auth_milliseconds =
            i64::try_from(self.auth_policy.recent_auth.as_millis()).unwrap_or(i64::MAX);
        SessionProjection {
            id: session.id,
            auth_level: session.auth_level.as_str().to_owned(),
            created_at_ms: session.created_at_ms,
            last_seen_at_ms: session.last_seen_at_ms,
            idle_expires_at_ms: session.idle_expires_at_ms,
            absolute_expires_at_ms: session.absolute_expires_at_ms,
            recent_auth_expires_at_ms: session
                .recent_auth_at_ms
                .saturating_add(recent_auth_milliseconds)
                .min(session.absolute_expires_at_ms),
        }
    }

    fn recent_auth_is_valid(&self, session: &AuthSessionSummary, now_ms: i64) -> bool {
        let Ok(window_ms) = duration_ms(self.auth_policy.recent_auth) else {
            return false;
        };
        now_ms >= session.recent_auth_at_ms
            && now_ms.saturating_sub(session.recent_auth_at_ms) < window_ms
    }

    async fn reserve_password_attempt(
        &self,
        normalized_subject: &str,
        context: &RequestContext,
        now_ms: i64,
    ) -> Result<PasswordAttemptDigests, AuthServiceError> {
        let account_digest = self
            .keyring
            .keyed_digest(
                KeyedDigestPurpose::LoginAccount,
                normalized_subject.as_bytes(),
            )
            .map_err(|_| AuthServiceError::Unavailable)?;
        let ip_digest = self
            .keyring
            .keyed_digest(
                KeyedDigestPurpose::LoginIp,
                &context.client.canonical_bytes(),
            )
            .map_err(|_| AuthServiceError::Unavailable)?;
        let global_digest = self
            .keyring
            .keyed_digest(KeyedDigestPurpose::LoginGlobal, b"control-plane-login-v1")
            .map_err(|_| AuthServiceError::Unavailable)?;
        let user_agent_hash: [u8; 32] = Sha256::digest(context.user_agent.as_bytes()).into();
        let reservation = LoginAttemptReservation {
            key_version: account_digest.key_version,
            account_hmac: account_digest.digest,
            ip_prefix_hmac: ip_digest.digest,
            global_hmac: global_digest.digest,
            user_agent_hash,
            request_id: context.request_id.clone(),
            now_ms,
            window_ms: duration_ms(self.auth_policy.login_window)
                .map_err(|_| AuthServiceError::Unavailable)?,
            account_max_attempts: self.auth_policy.login_account_limit,
            ip_max_attempts: self.auth_policy.login_ip_limit,
            global_max_attempts: self.auth_policy.login_global_limit,
            lockout_ms: duration_ms(self.auth_policy.login_block)
                .map_err(|_| AuthServiceError::Unavailable)?,
        };
        let rate_limited_event = NewLoginSecurityEvent {
            id: EntityId::new(),
            occurred_at_ms: now_ms,
            request_id: context.request_id.clone(),
            reason: LoginSecurityReason::RateLimited,
            digest_key_version: account_digest.key_version,
            account_hmac: Some(account_digest.digest),
            ip_prefix_hmac: Some(ip_digest.digest),
            user_agent_hash: Some(user_agent_hash),
        };
        match self
            .database
            .reserve_login_attempt(&reservation, &rate_limited_event)
            .await
            .map_err(|_| AuthServiceError::Unavailable)?
        {
            LoginRateDecision::Allowed { .. } => Ok(PasswordAttemptDigests {
                key_version: account_digest.key_version,
                account_hmac: account_digest.digest,
                ip_prefix_hmac: ip_digest.digest,
            }),
            LoginRateDecision::Limited { retry_after_ms, .. } => {
                Err(AuthServiceError::RateLimited {
                    retry_after_seconds: milliseconds_to_retry_seconds(retry_after_ms),
                })
            }
        }
    }

    fn prepare_session_rotation(
        &self,
        authenticated: &AuthenticatedSession,
        context: &RequestContext,
        auth_revision: Revision,
        authenticated_at_ms: i64,
        recent_auth_at_ms: i64,
        now_ms: i64,
    ) -> Result<PreparedSessionRotation, AuthServiceError> {
        let token_pair = SessionTokenPair::generate().map_err(|_| AuthServiceError::Unavailable)?;
        let session_digest = self
            .keyring
            .keyed_digest(KeyedDigestPurpose::Session, token_pair.session().as_bytes())
            .map_err(|_| AuthServiceError::Unavailable)?;
        let csrf_digest = self
            .keyring
            .keyed_digest(KeyedDigestPurpose::Csrf, token_pair.csrf().as_bytes())
            .map_err(|_| AuthServiceError::Unavailable)?;
        let ip_digest = self
            .keyring
            .keyed_digest(
                KeyedDigestPurpose::LoginIp,
                &context.client.canonical_bytes(),
            )
            .map_err(|_| AuthServiceError::Unavailable)?;
        let idle_expires_at_ms = checked_add_duration(now_ms, self.auth_policy.session_idle)
            .map_err(|_| AuthServiceError::Unavailable)?
            .min(authenticated.session.absolute_expires_at_ms);
        if idle_expires_at_ms <= now_ms {
            return Err(AuthServiceError::SessionInvalid);
        }
        Ok(PreparedSessionRotation {
            session: NewAuthSession {
                id: EntityId::new(),
                user_id: authenticated.user_id,
                token_key_version: session_digest.key_version,
                token_hmac: session_digest.digest,
                csrf_key_version: csrf_digest.key_version,
                csrf_hmac: csrf_digest.digest,
                auth_revision,
                auth_level: authenticated.session.auth_level,
                created_at_ms: now_ms,
                authenticated_at_ms,
                recent_auth_at_ms,
                last_seen_at_ms: now_ms,
                idle_expires_at_ms,
                absolute_expires_at_ms: authenticated.session.absolute_expires_at_ms,
                ip_prefix_key_version: Some(ip_digest.key_version),
                ip_prefix_hmac: Some(ip_digest.digest),
                user_agent_hash: Some(Sha256::digest(context.user_agent.as_bytes()).into()),
                revision: Revision::initial(),
            },
            token_pair,
        })
    }

    fn security_event(
        &self,
        authenticated: &AuthenticatedSession,
        context: &RequestContext,
        reason: LoginSecurityReason,
        now_ms: i64,
    ) -> Result<NewLoginSecurityEvent, AuthServiceError> {
        let account_digest = self
            .keyring
            .keyed_digest(
                KeyedDigestPurpose::LoginAccount,
                authenticated.username.normalized().as_bytes(),
            )
            .map_err(|_| AuthServiceError::Unavailable)?;
        let ip_digest = self
            .keyring
            .keyed_digest(
                KeyedDigestPurpose::LoginIp,
                &context.client.canonical_bytes(),
            )
            .map_err(|_| AuthServiceError::Unavailable)?;
        Ok(NewLoginSecurityEvent {
            id: EntityId::new(),
            occurred_at_ms: now_ms,
            request_id: context.request_id.clone(),
            reason,
            digest_key_version: account_digest.key_version,
            account_hmac: Some(account_digest.digest),
            ip_prefix_hmac: Some(ip_digest.digest),
            user_agent_hash: Some(Sha256::digest(context.user_agent.as_bytes()).into()),
        })
    }

    fn rotated_login_outcome(
        &self,
        authenticated: &AuthenticatedSession,
        session: SessionProjection,
        token_pair: SessionTokenPair,
        force_password_change: bool,
    ) -> LoginOutcome {
        let mut actor = Self::actor_projection(authenticated);
        actor.force_password_change = force_password_change;
        actor.capabilities = actor_capabilities(actor.role, force_password_change);
        let (session_token, csrf_token) = token_pair.into_tokens();
        LoginOutcome {
            actor,
            session,
            session_token,
            csrf_token,
        }
    }

    async fn authenticate_credential(
        &self,
        credential: SessionCredential,
        action: AuthenticatedAction,
        touch_session: bool,
    ) -> Result<AuthenticatedSession, AuthServiceError> {
        let session_token = SessionToken::parse_presented(credential.session_token.as_str())
            .map_err(|_| AuthServiceError::SessionInvalid)?;
        let csrf_token = credential
            .csrf_token
            .map(|csrf| CsrfToken::parse_presented(csrf.as_str()))
            .transpose()
            .map_err(|_| AuthServiceError::CsrfInvalid)?;
        let now_ms = unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?;
        let touch_interval_ms = duration_ms(self.auth_policy.session_touch_interval)
            .map_err(|_| AuthServiceError::Unavailable)?;
        let idle_timeout_ms = duration_ms(self.auth_policy.session_idle)
            .map_err(|_| AuthServiceError::Unavailable)?;
        let touch_session = touch_session && action.allowed_during_forced_password_change();
        for key_version in self.keyring.key_versions() {
            let token_hmac = self
                .keyring
                .keyed_digest_for_version(
                    key_version,
                    KeyedDigestPurpose::Session,
                    session_token.as_bytes(),
                )
                .map_err(|_| AuthServiceError::Unavailable)?;
            let csrf_hmac = csrf_token
                .as_ref()
                .map(|csrf| {
                    self.keyring.keyed_digest_for_version(
                        key_version,
                        KeyedDigestPurpose::Csrf,
                        csrf.as_bytes(),
                    )
                })
                .transpose()
                .map_err(|_| AuthServiceError::Unavailable)?;
            let authentication = SessionAuthentication {
                token_key_version: token_hmac.key_version,
                token_hmac: token_hmac.digest,
                csrf_key_version: csrf_hmac.as_ref().map(|digest| digest.key_version),
                csrf_hmac: csrf_hmac.map(|digest| digest.digest),
                now_ms,
                touch_interval_ms,
                idle_timeout_ms,
            };
            let outcome = if touch_session {
                self.database.authenticate_session(&authentication).await
            } else {
                self.database
                    .authenticate_session_read_only(&authentication)
                    .await
            }
            .map_err(|_| AuthServiceError::Unavailable)?;
            match outcome {
                SessionAuthenticationOutcome::Authenticated(session) => {
                    if session.force_password_change
                        && !action.allowed_during_forced_password_change()
                    {
                        return Err(AuthServiceError::PasswordChangeRequired);
                    }
                    return Ok(session);
                }
                SessionAuthenticationOutcome::InvalidSession => {}
                SessionAuthenticationOutcome::InvalidCsrf => {
                    return Err(AuthServiceError::CsrfInvalid);
                }
            }
        }
        Err(AuthServiceError::SessionInvalid)
    }

    async fn authenticate_mutating_credential(
        &self,
        credential: MutatingSessionCredential,
        action: AuthenticatedAction,
    ) -> Result<AuthenticatedSession, AuthServiceError> {
        self.authenticate_credential(
            SessionCredential {
                session_token: credential.session_token,
                csrf_token: Some(credential.csrf_token),
                context: credential.context,
            },
            action,
            false,
        )
        .await
    }

    async fn record_login_event(
        &self,
        context: &RequestContext,
        reason: LoginSecurityReason,
        account_hmac: Option<[u8; 32]>,
        ip_prefix_hmac: Option<[u8; 32]>,
    ) -> Result<(), AuthServiceError> {
        let event = NewLoginSecurityEvent {
            id: EntityId::new(),
            occurred_at_ms: unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?,
            request_id: context.request_id.clone(),
            reason,
            digest_key_version: self.keyring.key_version(),
            account_hmac,
            ip_prefix_hmac,
            user_agent_hash: Some(Sha256::digest(context.user_agent.as_bytes()).into()),
        };
        self.database
            .record_login_security_event(&event)
            .await
            .map_err(|_| AuthServiceError::Unavailable)
    }
}

#[async_trait]
impl ControlPlane for ControlPlaneApplication {
    async fn database_ready(&self) -> Result<(), ProbeError> {
        self.database
            .probe()
            .await
            .map_err(|_| ProbeError::DatabaseUnavailable)
    }

    async fn secret_ready(&self) -> Result<(), ProbeError> {
        let record = self
            .database
            .active_secret_record(SecretBinding::root_key_canary())
            .await
            .map_err(|_| ProbeError::SecretUnavailable)?
            .ok_or(ProbeError::SecretUnavailable)?;
        self.keyring
            .verify_canary(&record.envelope)
            .map_err(|_| ProbeError::SecretUnavailable)
    }

    async fn is_initialized(&self) -> Result<bool, ProbeError> {
        self.database
            .is_initialized()
            .await
            .map_err(|error| match error {
                PersistenceError::InconsistentBootstrapState => {
                    ProbeError::BootstrapStateInconsistent
                }
                _ => ProbeError::DatabaseUnavailable,
            })
    }

    async fn initialize(
        &self,
        command: BootstrapCommand,
    ) -> Result<BootstrapOutcome, BootstrapServiceError> {
        if self
            .database
            .is_initialized()
            .await
            .map_err(map_bootstrap_state_read_error)?
        {
            return Err(BootstrapServiceError::AlreadyInitialized);
        }
        let capability = self
            .setup_capability
            .as_ref()
            .ok_or(BootstrapServiceError::CapabilityInvalid)?;
        if !capability.authorize(command.setup_token.as_str()) {
            return Err(BootstrapServiceError::CapabilityInvalid);
        }
        let instance_name = InstanceName::parse(command.instance_name)
            .map_err(|_| BootstrapServiceError::InvalidInstanceName)?;
        let username = Username::parse(command.username)
            .map_err(|_| BootstrapServiceError::InvalidUsername)?;
        self.password_service
            .validate(command.password.as_str())
            .map_err(map_password_error)?;
        let mut attempt_guard = self.last_bootstrap_attempt.lock().await;
        if attempt_guard
            .as_ref()
            .is_some_and(|last_attempt| last_attempt.elapsed() < Duration::from_secs(2))
        {
            return Err(BootstrapServiceError::RateLimited);
        }
        *attempt_guard = Some(Instant::now());
        let password_service = self.password_service.clone();
        let password = command.password;
        let password_hash =
            tokio::task::spawn_blocking(move || password_service.hash(password.as_str()))
                .await
                .map_err(|_| BootstrapServiceError::Unavailable)?
                .map_err(map_password_error)?;
        let created_at_ms = unix_time_ms().map_err(|_| BootstrapServiceError::Unavailable)?;
        let instance_id = EntityId::new();
        let owner_id = EntityId::new();
        let principal_label = PrincipalLabel::parse(format!("usr_{owner_id}"))
            .map_err(|_| BootstrapServiceError::Unavailable)?;
        let instance = Instance {
            id: instance_id,
            public_id: EntityId::new(),
            name: instance_name,
            created_at_ms,
            revision: Revision::initial(),
        };
        let owner = UserAccount {
            id: owner_id,
            username,
            password_hash,
            role: UserRole::Owner,
            principal_label,
            force_password_change: false,
            revision: Revision::initial(),
            created_at_ms,
        };
        let (recovery_codes, one_time_recovery_codes) = self
            .prepare_recovery_code_set(created_at_ms)
            .map_err(|_| BootstrapServiceError::Unavailable)?;
        let persisted_instance_id = self
            .database
            .bootstrap_control_plane_with_recovery(
                &instance,
                &owner,
                &SubscriptionBehaviorSettings::default(),
                &recovery_codes,
            )
            .await
            .map_err(map_bootstrap_write_error)?;
        capability.consume();
        Ok(BootstrapOutcome {
            instance_id: persisted_instance_id.to_string(),
            owner_id: owner_id.to_string(),
            one_time_recovery_codes,
        })
    }

    async fn login(&self, command: LoginCommand) -> Result<LoginOutcome, AuthServiceError> {
        if !self
            .database
            .is_initialized()
            .await
            .map_err(|_| AuthServiceError::Unavailable)?
        {
            return Err(AuthServiceError::NotInitialized);
        }
        let password_hash_permit = self
            .password_hash_slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| AuthServiceError::RateLimited {
                retry_after_seconds: 1,
            })?;
        let now_ms = unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?;
        let parsed_username = Username::parse(command.username.clone()).ok();
        let normalized_subject = parsed_username
            .as_ref()
            .map(Username::normalized)
            .unwrap_or_else(|| bounded_login_subject(&command.username));
        let attempt_digests = self
            .reserve_password_attempt(&normalized_subject, &command.context, now_ms)
            .await?;

        let credentials = match parsed_username {
            Some(ref username) => self
                .database
                .user_credentials_by_normalized_username(&username.normalized())
                .await
                .map_err(|_| AuthServiceError::Unavailable)?,
            None => None,
        };
        let planned_password_work = plan_login_password_work(
            credentials
                .as_ref()
                .map(|credentials| &credentials.password_hash),
            &self.dummy_password_hash,
        );
        let current_dummy = self.dummy_password_hash.clone();
        let upgrade_verified_credential = credentials
            .as_ref()
            .is_some_and(|credentials| credentials.status == UserStatus::Active);
        let password_service = self.password_service.clone();
        let password = command.password;
        let password_verification = tokio::task::spawn_blocking(move || {
            let result = password_service.execute_login_work_plan(
                planned_password_work.plan,
                password.as_str(),
                &planned_password_work.selected_credential,
                &current_dummy,
                upgrade_verified_credential,
            );
            drop(password_hash_permit);
            result
        })
        .await
        .map_err(|_| AuthServiceError::Unavailable)?;
        let password_verification = match password_verification {
            Ok(verification) => verification,
            Err(PasswordError::TooLong) => return Err(AuthServiceError::InvalidCredentials),
            Err(_) => return Err(AuthServiceError::Unavailable),
        };
        let password_matches = password_verification.verified();
        let account_is_active = credentials
            .as_ref()
            .is_some_and(|credentials| credentials.status == UserStatus::Active);
        if !password_matches || !account_is_active {
            let reason = if password_matches && credentials.is_some() {
                LoginSecurityReason::AccountInactive
            } else {
                LoginSecurityReason::InvalidCredentials
            };
            self.record_login_event(
                &command.context,
                reason,
                Some(attempt_digests.account_hmac),
                Some(attempt_digests.ip_prefix_hmac),
            )
            .await?;
            return Err(AuthServiceError::InvalidCredentials);
        }
        let credentials = credentials.ok_or(AuthServiceError::InvalidCredentials)?;
        let upgraded_hash = password_verification.into_upgraded_hash();
        let token_pair = SessionTokenPair::generate().map_err(|_| AuthServiceError::Unavailable)?;
        let session_digest = self
            .keyring
            .keyed_digest(KeyedDigestPurpose::Session, token_pair.session().as_bytes())
            .map_err(|_| AuthServiceError::Unavailable)?;
        let csrf_digest = self
            .keyring
            .keyed_digest(KeyedDigestPurpose::Csrf, token_pair.csrf().as_bytes())
            .map_err(|_| AuthServiceError::Unavailable)?;
        let idle_expires_at_ms = checked_add_duration(now_ms, self.auth_policy.session_idle)
            .map_err(|_| AuthServiceError::Unavailable)?;
        let absolute_expires_at_ms =
            checked_add_duration(now_ms, self.auth_policy.session_absolute)
                .map_err(|_| AuthServiceError::Unavailable)?;
        let session = NewAuthSession {
            id: EntityId::new(),
            user_id: credentials.user_id,
            token_key_version: session_digest.key_version,
            token_hmac: session_digest.digest,
            csrf_key_version: csrf_digest.key_version,
            csrf_hmac: csrf_digest.digest,
            auth_revision: credentials.auth_revision,
            auth_level: AuthLevel::Password,
            created_at_ms: now_ms,
            authenticated_at_ms: now_ms,
            recent_auth_at_ms: now_ms,
            last_seen_at_ms: now_ms,
            idle_expires_at_ms,
            absolute_expires_at_ms,
            ip_prefix_key_version: Some(attempt_digests.key_version),
            ip_prefix_hmac: Some(attempt_digests.ip_prefix_hmac),
            user_agent_hash: Some(Sha256::digest(command.context.user_agent.as_bytes()).into()),
            revision: Revision::initial(),
        };
        let success_event = NewLoginSecurityEvent {
            id: EntityId::new(),
            occurred_at_ms: now_ms,
            request_id: command.context.request_id,
            reason: LoginSecurityReason::LoginSucceeded,
            digest_key_version: attempt_digests.key_version,
            account_hmac: Some(attempt_digests.account_hmac),
            ip_prefix_hmac: Some(attempt_digests.ip_prefix_hmac),
            user_agent_hash: session.user_agent_hash,
        };
        let stored_session = self
            .database
            .create_auth_session_with_optional_password_upgrade(
                &session,
                &success_event,
                &credentials,
                upgraded_hash.as_ref(),
            )
            .await
            .map_err(map_login_write_error)?;
        let actor = ActorProjection {
            id: credentials.user_id,
            username: credentials.username.as_str().to_owned(),
            role: credentials.role,
            capabilities: actor_capabilities(credentials.role, credentials.force_password_change),
            force_password_change: credentials.force_password_change,
        };
        let session = self.session_projection(&stored_session);
        let (session_token, csrf_token) = token_pair.into_tokens();
        Ok(LoginOutcome {
            actor,
            session,
            session_token,
            csrf_token,
        })
    }

    async fn reauthenticate(
        &self,
        command: ReauthenticateCommand,
    ) -> Result<LoginOutcome, AuthServiceError> {
        let context = command.credential.context.clone();
        let authenticated = self
            .authenticate_mutating_credential(
                command.credential,
                AuthenticatedAction::Reauthenticate,
            )
            .await?;
        let password_hash_permit = self
            .password_hash_slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| AuthServiceError::RateLimited {
                retry_after_seconds: 1,
            })?;
        let now_ms = unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?;
        let attempt_digests = self
            .reserve_password_attempt(&authenticated.username.normalized(), &context, now_ms)
            .await?;
        let credentials = self
            .database
            .user_credentials_by_normalized_username(&authenticated.username.normalized())
            .await
            .map_err(|_| AuthServiceError::Unavailable)?
            .ok_or(AuthServiceError::SessionInvalid)?;
        if credentials.user_id != authenticated.user_id
            || credentials.auth_revision != authenticated.session.auth_revision
            || credentials.status != UserStatus::Active
        {
            return Err(AuthServiceError::SessionInvalid);
        }
        let password_service = self.password_service.clone();
        let password = command.password;
        let expected_hash = credentials.password_hash;
        let verification = tokio::task::spawn_blocking(move || {
            let result = password_service.verify(password.as_str(), &expected_hash);
            drop(password_hash_permit);
            result
        })
        .await
        .map_err(|_| AuthServiceError::Unavailable)?;
        let verified = match verification {
            Ok(verified) => verified,
            Err(PasswordError::TooLong) => false,
            Err(_) => return Err(AuthServiceError::Unavailable),
        };
        if !verified {
            self.record_login_event(
                &context,
                LoginSecurityReason::InvalidCredentials,
                Some(attempt_digests.account_hmac),
                Some(attempt_digests.ip_prefix_hmac),
            )
            .await?;
            return Err(AuthServiceError::InvalidProof);
        }

        let prepared = self.prepare_session_rotation(
            &authenticated,
            &context,
            authenticated.session.auth_revision,
            now_ms,
            now_ms,
            now_ms,
        )?;
        let event = self.security_event(
            &authenticated,
            &context,
            LoginSecurityReason::ReauthenticationSucceeded,
            now_ms,
        )?;
        let stored_session = self
            .database
            .rotate_current_session(
                authenticated.user_id,
                authenticated.session.id,
                authenticated.user_revision,
                &prepared.session,
                &event,
                now_ms,
            )
            .await
            .map_err(map_session_write_error)?;
        Ok(self.rotated_login_outcome(
            &authenticated,
            self.session_projection(&stored_session),
            prepared.token_pair,
            authenticated.force_password_change,
        ))
    }

    async fn change_password(
        &self,
        command: ChangePasswordCommand,
    ) -> Result<PasswordChangeOutcome, AuthServiceError> {
        let context = command.credential.context.clone();
        let authenticated = self
            .authenticate_mutating_credential(
                command.credential,
                AuthenticatedAction::ChangePassword,
            )
            .await?;
        let now_ms = unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?;
        if !self.recent_auth_is_valid(&authenticated.session, now_ms) {
            return Err(AuthServiceError::RecentAuthRequired);
        }
        let password_hash_permit = self
            .password_hash_slots
            .clone()
            .try_acquire_owned()
            .map_err(|_| AuthServiceError::RateLimited {
                retry_after_seconds: 1,
            })?;
        let credentials = self
            .database
            .user_credentials_by_normalized_username(&authenticated.username.normalized())
            .await
            .map_err(|_| AuthServiceError::Unavailable)?
            .ok_or(AuthServiceError::SessionInvalid)?;
        if credentials.user_id != authenticated.user_id
            || credentials.auth_revision != authenticated.session.auth_revision
            || credentials.status != UserStatus::Active
        {
            return Err(AuthServiceError::SessionInvalid);
        }
        let password_service = self.password_service.clone();
        let new_password = command.new_password;
        let current_hash = credentials.password_hash;
        let password_hash = tokio::task::spawn_blocking(move || {
            let result = (|| {
                password_service.validate(new_password.as_str())?;
                if password_service.verify(new_password.as_str(), &current_hash)? {
                    return Ok(None);
                }
                password_service.hash(new_password.as_str()).map(Some)
            })();
            drop(password_hash_permit);
            result
        })
        .await
        .map_err(|_| AuthServiceError::Unavailable)?;
        let password_hash: PasswordHash = match password_hash {
            Ok(Some(password_hash)) => password_hash,
            Ok(None) => return Err(AuthServiceError::PasswordUnchanged),
            Err(
                PasswordError::TooShort | PasswordError::TooLong | PasswordError::ControlCharacter,
            ) => return Err(AuthServiceError::InvalidNewPassword),
            Err(_) => return Err(AuthServiceError::Unavailable),
        };
        let next_auth_revision = authenticated
            .session
            .auth_revision
            .next()
            .map_err(|_| AuthServiceError::Unavailable)?;
        let prepared = self.prepare_session_rotation(
            &authenticated,
            &context,
            next_auth_revision,
            authenticated.session.authenticated_at_ms,
            authenticated.session.recent_auth_at_ms,
            now_ms,
        )?;
        let event = self.security_event(
            &authenticated,
            &context,
            LoginSecurityReason::PasswordChanged,
            now_ms,
        )?;
        let changed = self
            .database
            .change_password_and_rotate(PasswordChangeRotation {
                user_id: authenticated.user_id,
                current_session_id: authenticated.session.id,
                expected_user_revision: authenticated.user_revision,
                new_hash: &password_hash,
                replacement: &prepared.session,
                event: &event,
                now_ms,
            })
            .await
            .map_err(map_session_write_error)?;
        Ok(PasswordChangeOutcome {
            login: self.rotated_login_outcome(
                &authenticated,
                self.session_projection(&changed.session),
                prepared.token_pair,
                false,
            ),
            revoked_sessions: changed.revoked_sessions,
        })
    }

    async fn current_actor(
        &self,
        credential: SessionCredential,
    ) -> Result<(ActorProjection, SessionProjection), AuthServiceError> {
        let authenticated = self
            .authenticate_credential(credential, AuthenticatedAction::ReadOwnIdentity, true)
            .await?;
        Ok((
            Self::actor_projection(&authenticated),
            self.session_projection(&authenticated.session),
        ))
    }

    async fn logout(&self, credential: MutatingSessionCredential) -> Result<(), AuthServiceError> {
        let context = credential.context.clone();
        let authenticated = match self
            .authenticate_mutating_credential(credential, AuthenticatedAction::SignOut)
            .await
        {
            Ok(authenticated) => authenticated,
            Err(AuthServiceError::SessionInvalid) => return Ok(()),
            Err(error) => return Err(error),
        };
        let now_ms = unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?;
        let account_digest = self
            .keyring
            .keyed_digest(
                KeyedDigestPurpose::LoginAccount,
                authenticated.username.normalized().as_bytes(),
            )
            .map_err(|_| AuthServiceError::Unavailable)?;
        let ip_digest = self
            .keyring
            .keyed_digest(
                KeyedDigestPurpose::LoginIp,
                &context.client.canonical_bytes(),
            )
            .map_err(|_| AuthServiceError::Unavailable)?;
        let event = NewLoginSecurityEvent {
            id: EntityId::new(),
            occurred_at_ms: now_ms,
            request_id: context.request_id,
            reason: LoginSecurityReason::Logout,
            digest_key_version: account_digest.key_version,
            account_hmac: Some(account_digest.digest),
            ip_prefix_hmac: Some(ip_digest.digest),
            user_agent_hash: Some(Sha256::digest(context.user_agent.as_bytes()).into()),
        };
        self.database
            .revoke_current_session_with_event(
                authenticated.user_id,
                authenticated.session.id,
                now_ms,
                SessionRevocationReason::Logout,
                &event,
            )
            .await
            .map(|_| ())
            .map_err(|_| AuthServiceError::Unavailable)
    }

    async fn logout_all(
        &self,
        command: LogoutAllCommand,
    ) -> Result<LogoutAllOutcome, AuthServiceError> {
        let context = command.credential.context.clone();
        let authenticated = self
            .authenticate_mutating_credential(command.credential, AuthenticatedAction::SignOut)
            .await?;
        let now_ms = unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?;
        if !self.recent_auth_is_valid(&authenticated.session, now_ms) {
            return Err(AuthServiceError::RecentAuthRequired);
        }
        let event = self.security_event(
            &authenticated,
            &context,
            LoginSecurityReason::LogoutAll,
            now_ms,
        )?;
        if !command.keep_current {
            let result = self
                .database
                .logout_all_sessions_with_event(
                    authenticated.user_id,
                    authenticated.session.id,
                    authenticated.session.recent_auth_at_ms,
                    &event,
                    now_ms,
                )
                .await
                .map_err(map_session_write_error)?;
            return Ok(LogoutAllOutcome::SignedOut {
                revoked_sessions: result.revoked_sessions,
            });
        }

        let next_auth_revision = authenticated
            .session
            .auth_revision
            .next()
            .map_err(|_| AuthServiceError::Unavailable)?;
        let prepared = self.prepare_session_rotation(
            &authenticated,
            &context,
            next_auth_revision,
            authenticated.session.authenticated_at_ms,
            authenticated.session.recent_auth_at_ms,
            now_ms,
        )?;
        let result = self
            .database
            .logout_all_sessions_and_rotate(
                authenticated.user_id,
                authenticated.session.id,
                authenticated.user_revision,
                &prepared.session,
                &event,
                now_ms,
            )
            .await
            .map_err(map_session_write_error)?;
        let login = self.rotated_login_outcome(
            &authenticated,
            self.new_session_projection(&prepared.session),
            prepared.token_pair,
            authenticated.force_password_change,
        );
        Ok(LogoutAllOutcome::CurrentRetained {
            login,
            revoked_sessions: result.revoked_sessions,
        })
    }

    async fn list_sessions(
        &self,
        credential: SessionCredential,
    ) -> Result<Vec<UserSessionProjection>, AuthServiceError> {
        let authenticated = self
            .authenticate_credential(credential, AuthenticatedAction::ManageOwnSessions, true)
            .await?;
        let now_ms = unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?;
        self.database
            .list_active_user_sessions(authenticated.user_id, now_ms)
            .await
            .map_err(|_| AuthServiceError::Unavailable)
            .map(|sessions| {
                sessions
                    .into_iter()
                    .map(|session| UserSessionProjection {
                        is_current: session.id == authenticated.session.id,
                        session: self.session_projection(&session),
                    })
                    .collect()
            })
    }

    async fn revoke_session(
        &self,
        command: RevokeSessionCommand,
    ) -> Result<RevokeSessionOutcome, AuthServiceError> {
        let context = command.credential.context.clone();
        let authenticated = self
            .authenticate_mutating_credential(
                command.credential,
                AuthenticatedAction::ManageOwnSessions,
            )
            .await?;
        let now_ms = unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?;
        if !self.recent_auth_is_valid(&authenticated.session, now_ms) {
            return Err(AuthServiceError::RecentAuthRequired);
        }
        let event = self.security_event(
            &authenticated,
            &context,
            LoginSecurityReason::SessionRevoked,
            now_ms,
        )?;
        self.database
            .revoke_user_session_with_event(UserSessionRevocation {
                user_id: authenticated.user_id,
                actor_session_id: authenticated.session.id,
                target_session_id: command.target_session_id,
                expected_user_revision: authenticated.user_revision,
                expected_auth_revision: authenticated.session.auth_revision,
                expected_recent_auth_at_ms: authenticated.session.recent_auth_at_ms,
                event: &event,
                now_ms,
            })
            .await
            .map_err(map_session_write_error)?;
        Ok(RevokeSessionOutcome {
            revoked_current: command.target_session_id == authenticated.session.id,
        })
    }

    async fn recovery_code_summary(
        &self,
        credential: SessionCredential,
    ) -> Result<RecoveryCodeSummary, AuthServiceError> {
        let authenticated = self
            .authenticate_credential(credential, AuthenticatedAction::ManageRecoveryCodes, true)
            .await?;
        self.database
            .recovery_code_summary(authenticated.user_id)
            .await
            .map_err(|_| AuthServiceError::Unavailable)?
            .map(|summary| RecoveryCodeSummary {
                set_version: summary.set_version,
                total_count: summary.total_count,
                remaining_count: summary.remaining_count,
                created_at_ms: summary.created_at_ms,
            })
            .ok_or(AuthServiceError::RecoveryCodesUnavailable)
    }

    async fn regenerate_recovery_codes(
        &self,
        command: RegenerateRecoveryCodesCommand,
    ) -> Result<RecoveryCodesCreated, AuthServiceError> {
        let authenticated = self
            .authenticate_mutating_credential(
                command.credential,
                AuthenticatedAction::ManageRecoveryCodes,
            )
            .await?;
        let now_ms = unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?;
        if !self.recent_auth_is_valid(&authenticated.session, now_ms) {
            return Err(AuthServiceError::RecentAuthRequired);
        }
        let (replacement, one_time_recovery_codes) = self.prepare_recovery_code_set(now_ms)?;
        let summary = self
            .database
            .replace_recovery_codes(RecoveryCodeReplacement {
                user_id: authenticated.user_id,
                actor_session_id: authenticated.session.id,
                expected_user_revision: authenticated.user_revision,
                expected_auth_revision: authenticated.session.auth_revision,
                expected_recent_auth_at_ms: authenticated.session.recent_auth_at_ms,
                replacement: &replacement,
                now_ms,
            })
            .await
            .map_err(map_session_write_error)?;
        Ok(RecoveryCodesCreated {
            set_version: summary.set_version,
            one_time_recovery_codes,
            created_at_ms: summary.created_at_ms,
        })
    }
}

const fn actor_capabilities(role: UserRole, force_password_change: bool) -> BaselineCapabilities {
    if force_password_change {
        BaselineCapabilities::for_forced_password_change()
    } else {
        BaselineCapabilities::for_role(role)
    }
}

fn map_password_error(error: PasswordError) -> BootstrapServiceError {
    match error {
        PasswordError::TooShort | PasswordError::TooLong | PasswordError::ControlCharacter => {
            BootstrapServiceError::InvalidPassword
        }
        _ => BootstrapServiceError::Unavailable,
    }
}

fn map_bootstrap_state_read_error(error: PersistenceError) -> BootstrapServiceError {
    match error {
        PersistenceError::InconsistentBootstrapState => BootstrapServiceError::InconsistentState,
        _ => BootstrapServiceError::Unavailable,
    }
}

fn map_bootstrap_write_error(error: PersistenceError) -> BootstrapServiceError {
    match error {
        PersistenceError::AlreadyInitialized => BootstrapServiceError::AlreadyInitialized,
        PersistenceError::IdentityConflict => BootstrapServiceError::IdentityConflict,
        PersistenceError::InconsistentBootstrapState => BootstrapServiceError::InconsistentState,
        _ => BootstrapServiceError::Unavailable,
    }
}

fn map_session_write_error(error: PersistenceError) -> AuthServiceError {
    match error {
        PersistenceError::SessionPrincipalUnavailable
        | PersistenceError::SessionRevisionConflict
        | PersistenceError::AuthStateUnavailable => AuthServiceError::SessionInvalid,
        _ => AuthServiceError::Unavailable,
    }
}

fn map_login_write_error(error: PersistenceError) -> AuthServiceError {
    match error {
        PersistenceError::SessionPrincipalUnavailable
        | PersistenceError::SessionRevisionConflict
        | PersistenceError::AuthStateUnavailable => AuthServiceError::InvalidCredentials,
        _ => AuthServiceError::Unavailable,
    }
}

fn unix_time_ms() -> Result<i64, ()> {
    i64::try_from(OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000).map_err(|_| ())
}

fn duration_ms(duration: Duration) -> Result<i64, ()> {
    i64::try_from(duration.as_millis()).map_err(|_| ())
}

fn checked_add_duration(timestamp_ms: i64, duration: Duration) -> Result<i64, ()> {
    timestamp_ms.checked_add(duration_ms(duration)?).ok_or(())
}

fn milliseconds_to_retry_seconds(milliseconds: i64) -> u64 {
    let milliseconds = u64::try_from(milliseconds).unwrap_or(1);
    (milliseconds.saturating_add(999) / 1_000).max(1)
}

fn bounded_login_subject(username: &str) -> String {
    const MAX_SUBJECT_BYTES: usize = 128;
    if username.len() <= MAX_SUBJECT_BYTES {
        username.to_ascii_lowercase()
    } else {
        "__invalid_overlong_login_subject__".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use nodecontroll_domain::{PasswordHash, UserRole};
    use nodecontroll_identity::{LoginPasswordTimingBucket, PasswordService};
    use std::{
        net::{IpAddr, Ipv4Addr, Ipv6Addr},
        time::Duration,
    };

    use super::{
        AuthPolicy, AuthPolicyError, AuthenticatedAction, ClientNetwork, ClientNetworkError,
        actor_capabilities, bounded_login_subject, milliseconds_to_retry_seconds,
        plan_login_password_work,
    };

    #[test]
    fn client_network_canonicalizes_ipv4_and_ipv6_prefixes() {
        let ipv4 = ClientNetwork::new(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 129)), 24);
        assert!(matches!(
            ipv4,
            Ok(ref network)
                if network.address() == IpAddr::V4(Ipv4Addr::new(192, 0, 2, 0))
                    && network.prefix_length() == 24
        ));

        let ipv6 = ClientNetwork::new(
            IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 1, 2, 3, 4, 5, 6)),
            64,
        );
        assert!(matches!(
            ipv6,
            Ok(ref network)
                if network.address()
                    == IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 1, 2, 0, 0, 0, 0))
                    && network.prefix_length() == 64
        ));
    }

    #[test]
    fn client_network_rejects_cross_family_prefix_lengths() {
        assert_eq!(
            ClientNetwork::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 33),
            Err(ClientNetworkError::InvalidPrefixLength)
        );
        assert_eq!(
            ClientNetwork::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 129),
            Err(ClientNetworkError::InvalidPrefixLength)
        );
    }

    #[test]
    fn invalid_login_subjects_are_bounded_before_hmac() {
        assert_eq!(bounded_login_subject("MixedCase"), "mixedcase");
        assert_eq!(
            bounded_login_subject(&"a".repeat(129)),
            "__invalid_overlong_login_subject__"
        );
    }

    #[test]
    fn retry_after_rounds_up_and_never_returns_zero() {
        assert_eq!(milliseconds_to_retry_seconds(0), 1);
        assert_eq!(milliseconds_to_retry_seconds(1), 1);
        assert_eq!(milliseconds_to_retry_seconds(1_001), 2);
        assert_eq!(milliseconds_to_retry_seconds(-1), 1);
    }

    #[test]
    fn unknown_current_and_cheap_legacy_credentials_share_one_login_work_plan() {
        let dummy = PasswordHash::parse(
            "$argon2id$v=19$m=19456,t=2,p=1$QkJCQkJCQkJCQkJCQkJCQg$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        );
        let current = PasswordHash::parse(
            "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        );
        let cheap_legacy = PasswordHash::parse(
            "$argon2i$v=16$m=8192,t=1,p=1$c29tZXNhbHQxMjM0NTY3OA$AAAAAAAAAAAAAAAAAAAAAA",
        );
        assert!(dummy.is_ok());
        assert!(current.is_ok());
        assert!(cheap_legacy.is_ok());

        if let (Ok(dummy), Ok(current), Ok(cheap_legacy)) = (dummy, current, cheap_legacy) {
            let unknown = plan_login_password_work(None, &dummy);
            let current = plan_login_password_work(Some(&current), &dummy);
            let cheap_legacy = plan_login_password_work(Some(&cheap_legacy), &dummy);

            assert_eq!(unknown.plan, PasswordService::login_work_plan());
            assert_eq!(unknown.plan, current.plan);
            assert_eq!(current.plan, cheap_legacy.plan);
            assert_eq!(
                unknown.plan.timing_bucket(),
                LoginPasswordTimingBucket::CalibratedTwoCurrentPolicyCosts
            );
            assert_eq!(unknown.selected_credential.as_str(), dummy.as_str());
            assert_ne!(current.selected_credential.as_str(), dummy.as_str());
            assert_ne!(cheap_legacy.selected_credential.as_str(), dummy.as_str());
        }
    }

    #[test]
    fn forced_password_change_is_a_backend_allowlist_and_restricts_projected_scopes() {
        for action in [
            AuthenticatedAction::ReadOwnIdentity,
            AuthenticatedAction::Reauthenticate,
            AuthenticatedAction::ChangePassword,
            AuthenticatedAction::ManageOwnSessions,
            AuthenticatedAction::SignOut,
        ] {
            assert!(action.allowed_during_forced_password_change());
        }
        assert!(!AuthenticatedAction::ProductAccess.allowed_during_forced_password_change());
        assert!(!AuthenticatedAction::ManageRecoveryCodes.allowed_during_forced_password_change());
        let capabilities = actor_capabilities(UserRole::Owner, true);
        assert!(capabilities.allows_scope_name("credentials:manage"));
        assert!(!capabilities.allows_scope_name("users:write"));
        assert!(!capabilities.allows_scope_name("system:execute"));
        assert!(actor_capabilities(UserRole::Owner, false).allows_scope_name("users:write"));
    }

    #[test]
    fn authentication_policy_rejects_unbounded_hashing_and_invalid_lifetimes() {
        let policy = AuthPolicy {
            session_idle: Duration::from_secs(60),
            session_absolute: Duration::from_secs(300),
            session_touch_interval: Duration::from_secs(15),
            recent_auth: Duration::from_secs(60),
            login_window: Duration::from_secs(10),
            login_block: Duration::from_secs(20),
            login_account_limit: 3,
            login_ip_limit: 30,
            login_global_limit: 300,
            password_hash_concurrency: 4,
        };
        assert_eq!(policy.validate(), Ok(policy));
        assert_eq!(
            AuthPolicy {
                password_hash_concurrency: 0,
                ..policy
            }
            .validate(),
            Err(AuthPolicyError::Invalid)
        );
        assert_eq!(
            AuthPolicy {
                session_touch_interval: Duration::from_secs(60),
                ..policy
            }
            .validate(),
            Err(AuthPolicyError::Invalid)
        );
        assert_eq!(
            AuthPolicy {
                login_block: Duration::from_secs(9),
                ..policy
            }
            .validate(),
            Err(AuthPolicyError::Invalid)
        );
        assert_eq!(
            AuthPolicy {
                recent_auth: Duration::from_secs(301),
                ..policy
            }
            .validate(),
            Err(AuthPolicyError::Invalid)
        );
        assert_eq!(
            AuthPolicy {
                recent_auth: Duration::from_secs(59),
                ..policy
            }
            .validate(),
            Err(AuthPolicyError::Invalid)
        );
    }
}
