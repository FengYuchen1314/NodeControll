//! Application use-case boundary for the NodeControll control plane.
//!
//! HTTP adapters depend on [`ControlPlane`]; persistence and cryptographic details stay behind the
//! concrete [`ControlPlaneApplication`]. The first authenticated vertical slice deliberately
//! exposes only password login, server-side session restoration, and revocation. MFA, recent-auth
//! elevation, API tokens, and user administration extend this boundary in later WP-02 slices.

use std::{
    net::IpAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use nodecontroll_domain::{
    BaselineCapabilities, EntityId, Instance, InstanceName, PrincipalLabel, Revision,
    SubscriptionBehaviorSettings, UserAccount, UserRole, UserStatus, Username,
};
use nodecontroll_identity::{
    CsrfToken, PasswordError, PasswordService, SessionToken, SessionTokenPair, SetupCapability,
};
use nodecontroll_persistence::{
    AuthLevel, AuthSessionSummary, AuthenticatedSession, Database, LoginAttemptReservation,
    LoginRateDecision, LoginSecurityReason, NewAuthSession, NewLoginSecurityEvent,
    PersistenceError, SessionAuthentication, SessionAuthenticationOutcome, SessionRevocationReason,
};
use nodecontroll_secrets::{EnvelopeCipher, KeyedDigestPurpose};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use tokio::sync::{Mutex, Semaphore};
use zeroize::Zeroizing;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AuthPolicy {
    pub session_idle: Duration,
    pub session_absolute: Duration,
    pub session_touch_interval: Duration,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BootstrapOutcome {
    pub instance_id: String,
    pub owner_id: String,
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
    pub created_at_ms: i64,
    pub last_seen_at_ms: i64,
    pub idle_expires_at_ms: i64,
    pub absolute_expires_at_ms: i64,
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
    RateLimited { retry_after_seconds: u64 },
    SessionInvalid,
    CsrfInvalid,
    NotInitialized,
    Unavailable,
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
    async fn current_actor(
        &self,
        credential: SessionCredential,
    ) -> Result<(ActorProjection, SessionProjection), AuthServiceError>;
    async fn logout(&self, credential: MutatingSessionCredential) -> Result<(), AuthServiceError>;
}

pub struct ControlPlaneApplication {
    database: Database,
    cipher: EnvelopeCipher,
    password_service: PasswordService,
    dummy_password_hash: nodecontroll_domain::PasswordHash,
    setup_capability: Option<SetupCapability>,
    last_bootstrap_attempt: Mutex<Option<std::time::Instant>>,
    auth_policy: AuthPolicy,
    password_hash_slots: Arc<Semaphore>,
}

impl ControlPlaneApplication {
    pub fn new(
        database: Database,
        cipher: EnvelopeCipher,
        password_service: PasswordService,
        dummy_password_hash: nodecontroll_domain::PasswordHash,
        setup_capability: Option<SetupCapability>,
        auth_policy: AuthPolicy,
    ) -> Result<Arc<Self>, AuthPolicyError> {
        let auth_policy = auth_policy.validate()?;
        let password_hash_slots = Arc::new(Semaphore::new(auth_policy.password_hash_concurrency));
        Ok(Arc::new(Self {
            database,
            cipher,
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
            capabilities: BaselineCapabilities::for_role(session.role),
            force_password_change: session.force_password_change,
        }
    }

    fn session_projection(session: &AuthSessionSummary) -> SessionProjection {
        SessionProjection {
            id: session.id,
            created_at_ms: session.created_at_ms,
            last_seen_at_ms: session.last_seen_at_ms,
            idle_expires_at_ms: session.idle_expires_at_ms,
            absolute_expires_at_ms: session.absolute_expires_at_ms,
        }
    }

    async fn authenticate_credential(
        &self,
        credential: SessionCredential,
    ) -> Result<AuthenticatedSession, AuthServiceError> {
        let session_token = SessionToken::parse_presented(credential.session_token.as_str())
            .map_err(|_| AuthServiceError::SessionInvalid)?;
        let token_hmac = self
            .cipher
            .keyed_digest(KeyedDigestPurpose::Session, session_token.as_bytes())
            .map_err(|_| AuthServiceError::Unavailable)?;
        let (csrf_key_version, csrf_hmac) = match credential.csrf_token {
            Some(csrf) => {
                let csrf = CsrfToken::parse_presented(csrf.as_str())
                    .map_err(|_| AuthServiceError::CsrfInvalid)?;
                let digest = self
                    .cipher
                    .keyed_digest(KeyedDigestPurpose::Csrf, csrf.as_bytes())
                    .map_err(|_| AuthServiceError::Unavailable)?;
                (Some(digest.key_version), Some(digest.digest))
            }
            None => (None, None),
        };
        let authentication = SessionAuthentication {
            token_key_version: token_hmac.key_version,
            token_hmac: token_hmac.digest,
            csrf_key_version,
            csrf_hmac,
            now_ms: unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?,
            touch_interval_ms: duration_ms(self.auth_policy.session_touch_interval)
                .map_err(|_| AuthServiceError::Unavailable)?,
            idle_timeout_ms: duration_ms(self.auth_policy.session_idle)
                .map_err(|_| AuthServiceError::Unavailable)?,
        };
        match self
            .database
            .authenticate_session(&authentication)
            .await
            .map_err(|_| AuthServiceError::Unavailable)?
        {
            SessionAuthenticationOutcome::Authenticated(session) => Ok(session),
            SessionAuthenticationOutcome::InvalidSession => Err(AuthServiceError::SessionInvalid),
            SessionAuthenticationOutcome::InvalidCsrf => Err(AuthServiceError::CsrfInvalid),
        }
    }

    async fn authenticate_mutating_credential(
        &self,
        credential: MutatingSessionCredential,
    ) -> Result<AuthenticatedSession, AuthServiceError> {
        self.authenticate_credential(SessionCredential {
            session_token: credential.session_token,
            csrf_token: Some(credential.csrf_token),
            context: credential.context,
        })
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
            digest_key_version: self.cipher.key_version(),
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
        self.cipher
            .canary()
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
        let persisted_instance_id = self
            .database
            .bootstrap_control_plane(&instance, &owner, &SubscriptionBehaviorSettings::default())
            .await
            .map_err(map_bootstrap_write_error)?;
        capability.consume();
        Ok(BootstrapOutcome {
            instance_id: persisted_instance_id.to_string(),
            owner_id: owner_id.to_string(),
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
        let account_digest = self
            .cipher
            .keyed_digest(
                KeyedDigestPurpose::LoginAccount,
                normalized_subject.as_bytes(),
            )
            .map_err(|_| AuthServiceError::Unavailable)?;
        let ip_digest = self
            .cipher
            .keyed_digest(
                KeyedDigestPurpose::LoginIp,
                &command.context.client.canonical_bytes(),
            )
            .map_err(|_| AuthServiceError::Unavailable)?;
        let global_digest = self
            .cipher
            .keyed_digest(KeyedDigestPurpose::LoginGlobal, b"control-plane-login-v1")
            .map_err(|_| AuthServiceError::Unavailable)?;
        let reservation = LoginAttemptReservation {
            key_version: account_digest.key_version,
            account_hmac: account_digest.digest,
            ip_prefix_hmac: ip_digest.digest,
            global_hmac: global_digest.digest,
            now_ms,
            window_ms: duration_ms(self.auth_policy.login_window)
                .map_err(|_| AuthServiceError::Unavailable)?,
            account_max_attempts: self.auth_policy.login_account_limit,
            ip_max_attempts: self.auth_policy.login_ip_limit,
            global_max_attempts: self.auth_policy.login_global_limit,
            lockout_ms: duration_ms(self.auth_policy.login_block)
                .map_err(|_| AuthServiceError::Unavailable)?,
        };
        match self
            .database
            .reserve_login_attempt(&reservation)
            .await
            .map_err(|_| AuthServiceError::Unavailable)?
        {
            LoginRateDecision::Allowed { .. } => {}
            LoginRateDecision::Limited { retry_after_ms, .. } => {
                return Err(AuthServiceError::RateLimited {
                    retry_after_seconds: milliseconds_to_retry_seconds(retry_after_ms),
                });
            }
        }

        let credentials = match parsed_username {
            Some(ref username) => self
                .database
                .user_credentials_by_normalized_username(&username.normalized())
                .await
                .map_err(|_| AuthServiceError::Unavailable)?,
            None => None,
        };
        let expected_hash = credentials
            .as_ref()
            .map(|credentials| credentials.password_hash.clone())
            .unwrap_or_else(|| self.dummy_password_hash.clone());
        let password_service = self.password_service.clone();
        let password = command.password;
        let password_verification = tokio::task::spawn_blocking(move || {
            let result = password_service.verify(password.as_str(), &expected_hash);
            drop(password_hash_permit);
            result
        })
        .await
        .map_err(|_| AuthServiceError::Unavailable)?;
        let password_matches = match password_verification {
            Ok(matches) => matches,
            Err(PasswordError::TooLong) => return Err(AuthServiceError::InvalidCredentials),
            Err(_) => return Err(AuthServiceError::Unavailable),
        };
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
                Some(account_digest.digest),
                Some(ip_digest.digest),
            )
            .await?;
            return Err(AuthServiceError::InvalidCredentials);
        }
        let credentials = credentials.ok_or(AuthServiceError::InvalidCredentials)?;
        let token_pair = SessionTokenPair::generate().map_err(|_| AuthServiceError::Unavailable)?;
        let session_digest = self
            .cipher
            .keyed_digest(KeyedDigestPurpose::Session, token_pair.session().as_bytes())
            .map_err(|_| AuthServiceError::Unavailable)?;
        let csrf_digest = self
            .cipher
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
            ip_prefix_key_version: Some(ip_digest.key_version),
            ip_prefix_hmac: Some(ip_digest.digest),
            user_agent_hash: Some(Sha256::digest(command.context.user_agent.as_bytes()).into()),
            revision: Revision::initial(),
        };
        let success_event = NewLoginSecurityEvent {
            id: EntityId::new(),
            occurred_at_ms: now_ms,
            request_id: command.context.request_id,
            reason: LoginSecurityReason::LoginSucceeded,
            digest_key_version: account_digest.key_version,
            account_hmac: Some(account_digest.digest),
            ip_prefix_hmac: Some(ip_digest.digest),
            user_agent_hash: session.user_agent_hash,
        };
        let stored_session = self
            .database
            .create_auth_session(&session, &success_event)
            .await
            .map_err(|_| AuthServiceError::Unavailable)?;
        let actor = ActorProjection {
            id: credentials.user_id,
            username: credentials.username.as_str().to_owned(),
            role: credentials.role,
            capabilities: BaselineCapabilities::for_role(credentials.role),
            force_password_change: credentials.force_password_change,
        };
        let session = Self::session_projection(&stored_session);
        let (session_token, csrf_token) = token_pair.into_tokens();
        Ok(LoginOutcome {
            actor,
            session,
            session_token,
            csrf_token,
        })
    }

    async fn current_actor(
        &self,
        credential: SessionCredential,
    ) -> Result<(ActorProjection, SessionProjection), AuthServiceError> {
        let authenticated = self.authenticate_credential(credential).await?;
        Ok((
            Self::actor_projection(&authenticated),
            Self::session_projection(&authenticated.session),
        ))
    }

    async fn logout(&self, credential: MutatingSessionCredential) -> Result<(), AuthServiceError> {
        let context = credential.context.clone();
        let authenticated = match self.authenticate_mutating_credential(credential).await {
            Ok(authenticated) => authenticated,
            Err(AuthServiceError::SessionInvalid) => return Ok(()),
            Err(error) => return Err(error),
        };
        let now_ms = unix_time_ms().map_err(|_| AuthServiceError::Unavailable)?;
        let account_digest = self
            .cipher
            .keyed_digest(
                KeyedDigestPurpose::LoginAccount,
                authenticated.username.normalized().as_bytes(),
            )
            .map_err(|_| AuthServiceError::Unavailable)?;
        let ip_digest = self
            .cipher
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
    use std::{
        net::{IpAddr, Ipv4Addr, Ipv6Addr},
        time::Duration,
    };

    use super::{
        AuthPolicy, AuthPolicyError, ClientNetwork, ClientNetworkError, bounded_login_subject,
        milliseconds_to_retry_seconds,
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
    fn authentication_policy_rejects_unbounded_hashing_and_invalid_lifetimes() {
        let policy = AuthPolicy {
            session_idle: Duration::from_secs(60),
            session_absolute: Duration::from_secs(300),
            session_touch_interval: Duration::from_secs(15),
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
    }
}
