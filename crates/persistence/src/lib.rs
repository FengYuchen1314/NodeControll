use std::{str::FromStr, time::Duration};

use nodecontroll_domain::{
    EntityId, Instance, InstanceName, PasswordHash, PrincipalLabel, Revision,
    SubscriptionBehaviorSettings, UserAccount, UserRole, UserStatus, Username,
};
use nodecontroll_secrets::{SecretBinding, SecretEnvelope, SecretOwnerKind, SecretPurpose};
use sqlx::{
    PgPool, Row, SqlitePool,
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPoolOptions},
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use thiserror::Error;

mod auth_challenge;
mod totp;
mod webauthn;
#[cfg(test)]
mod webauthn_contract;

pub use auth_challenge::{
    AuthChallengeAccess, AuthChallengeAttemptFailure, AuthChallengeAttemptOutcome,
    AuthChallengeAttemptReservation, AuthChallengeAttemptReservationOutcome,
    AuthChallengeAttemptResume, AuthChallengeClientContext, AuthChallengeConsumption,
    AuthChallengeConsumptionOutcome, AuthChallengeRotationReservation,
    AuthChallengeRotationReservationOutcome, AuthChallengeTokenLookup, CreateAuthChallengeOutcome,
    NewAuthChallenge, ResumedAuthChallengeAttempt,
};
pub use totp::{
    ActivateTotpCredential, ActivateTotpCredentialOutcome, BeginTotpEnrollmentOutcome,
    DisableTotpCredential, DisableTotpCredentialOutcome, NewTotpEnrollment, StoredTotpCredential,
    TotpActivationResult, TotpChallengeBinding, TotpSessionGuard, TotpStepAdvance,
    TotpStepAdvanceOutcome, TotpVerifiedHandoff,
};
pub use webauthn::{
    BeginWebAuthnAuthenticationOutcome, BeginWebAuthnRegistrationOutcome,
    CompleteWebAuthnRegistration, CompleteWebAuthnRegistrationOutcome,
    NewWebAuthnAuthenticationCeremony, NewWebAuthnCredential, NewWebAuthnRegistrationCeremony,
    RenameWebAuthnCredential, RevokeWebAuthnCredential, RevokeWebAuthnCredentialOutcome,
    StoredWebAuthnAuthenticationCeremony, StoredWebAuthnCredential,
    StoredWebAuthnRegistrationCeremony, WebAuthnAuthenticationCommit,
    WebAuthnAuthenticationCommitOutcome, WebAuthnAuthenticationHandoff,
    WebAuthnChallengeBinding, WebAuthnCloneSuspected, WebAuthnCloneSuspectedOutcome,
    WebAuthnRegistrationResult, WebAuthnSessionGuard,
};

static SQLITE_MIGRATOR: Migrator = sqlx::migrate!("./migrations/sqlite");
static POSTGRES_MIGRATOR: Migrator = sqlx::migrate!("./migrations/postgres");
const SUBSCRIPTION_SETTINGS_KEY: &str = "subscription.behavior";
const SUBSCRIPTION_SETTINGS_SCHEMA: i32 = 1;
const JSON_SAFE_INTEGER_MAX_I64: i64 = 9_007_199_254_740_991;
pub const AUTH_HMAC_LENGTH: usize = 32;

pub type AuthHmac = [u8; AUTH_HMAC_LENGTH];

const fn is_transaction_owned_secret(purpose: SecretPurpose) -> bool {
    matches!(
        purpose,
        SecretPurpose::TotpSeed
            | SecretPurpose::WebAuthnRegistrationState
            | SecretPurpose::WebAuthnAuthenticationState
            | SecretPurpose::WebAuthnCredentialMaterial
    )
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewSecretRecord {
    pub id: EntityId,
    pub binding: SecretBinding,
    pub envelope: SecretEnvelope,
    pub created_at_ms: i64,
    pub rotated_from: Option<EntityId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredSecretRecord {
    pub id: EntityId,
    pub binding: SecretBinding,
    pub envelope: SecretEnvelope,
    pub created_at_ms: i64,
    pub rotated_from: Option<EntityId>,
    pub revision: Revision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewRecoveryCode {
    pub id: EntityId,
    pub digest_key_version: u32,
    pub code_hmac: AuthHmac,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewRecoveryCodeSet {
    pub created_at_ms: i64,
    pub codes: Vec<NewRecoveryCode>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecoveryCodeSetSummary {
    pub set_version: u64,
    pub total_count: u8,
    pub remaining_count: u8,
    pub created_at_ms: i64,
}

#[derive(Clone, Copy)]
pub struct RecoveryCodeReplacement<'a> {
    pub user_id: EntityId,
    pub actor_session_id: EntityId,
    pub expected_user_revision: Revision,
    pub expected_auth_revision: Revision,
    pub expected_recent_auth_at_ms: i64,
    pub replacement: &'a NewRecoveryCodeSet,
    pub now_ms: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecoveryCodeConsumption {
    pub user_id: EntityId,
    pub digest_key_version: u32,
    pub code_hmac: AuthHmac,
    pub now_ms: i64,
}

#[derive(Clone, PartialEq, Eq)]
pub struct UserCredentials {
    pub user_id: EntityId,
    pub username: Username,
    pub password_hash: PasswordHash,
    pub role: UserRole,
    pub status: UserStatus,
    pub principal_label: PrincipalLabel,
    pub force_password_change: bool,
    pub user_revision: Revision,
    pub auth_revision: Revision,
    pub password_changed_at_ms: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthSessionStatus {
    Active,
    Revoked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthLevel {
    Password,
    Mfa,
    PhishingResistant,
    Recovery,
}

impl AuthLevel {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Password => "password",
            Self::Mfa => "mfa",
            Self::PhishingResistant => "phishing_resistant",
            Self::Recovery => "recovery",
        }
    }

    fn parse(value: &str) -> Result<Self, PersistenceError> {
        match value {
            "password" => Ok(Self::Password),
            "mfa" => Ok(Self::Mfa),
            "phishing_resistant" => Ok(Self::PhishingResistant),
            "recovery" => Ok(Self::Recovery),
            _ => Err(PersistenceError::InvalidStoredAuthLevel),
        }
    }
}

impl AuthSessionStatus {
    fn parse(value: &str) -> Result<Self, PersistenceError> {
        match value {
            "active" => Ok(Self::Active),
            "revoked" => Ok(Self::Revoked),
            _ => Err(PersistenceError::InvalidStoredSessionStatus),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionRevocationReason {
    Logout,
    LogoutAll,
    PasswordChanged,
    UserDisabled,
    UserRevoked,
    Administrator,
    Rotation,
    Expired,
    SecurityPolicy,
}

impl SessionRevocationReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Logout => "logout",
            Self::LogoutAll => "logout_all",
            Self::PasswordChanged => "password_changed",
            Self::UserDisabled => "user_disabled",
            Self::UserRevoked => "user_revoked",
            Self::Administrator => "administrator",
            Self::Rotation => "rotation",
            Self::Expired => "expired",
            Self::SecurityPolicy => "security_policy",
        }
    }

    fn parse(value: &str) -> Result<Self, PersistenceError> {
        match value {
            "logout" => Ok(Self::Logout),
            "logout_all" => Ok(Self::LogoutAll),
            "password_changed" => Ok(Self::PasswordChanged),
            "user_disabled" => Ok(Self::UserDisabled),
            "user_revoked" => Ok(Self::UserRevoked),
            "administrator" => Ok(Self::Administrator),
            "rotation" => Ok(Self::Rotation),
            "expired" => Ok(Self::Expired),
            "security_policy" => Ok(Self::SecurityPolicy),
            _ => Err(PersistenceError::InvalidStoredRevocationReason),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewAuthSession {
    pub id: EntityId,
    pub user_id: EntityId,
    pub token_key_version: u32,
    pub token_hmac: AuthHmac,
    pub csrf_key_version: u32,
    pub csrf_hmac: AuthHmac,
    pub auth_revision: Revision,
    pub auth_level: AuthLevel,
    pub created_at_ms: i64,
    pub authenticated_at_ms: i64,
    pub recent_auth_at_ms: i64,
    pub last_seen_at_ms: i64,
    pub idle_expires_at_ms: i64,
    pub absolute_expires_at_ms: i64,
    pub ip_prefix_key_version: Option<u32>,
    pub ip_prefix_hmac: Option<AuthHmac>,
    pub user_agent_hash: Option<AuthHmac>,
    pub revision: Revision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionAuthentication {
    pub token_key_version: u32,
    pub token_hmac: AuthHmac,
    pub csrf_key_version: Option<u32>,
    pub csrf_hmac: Option<AuthHmac>,
    pub now_ms: i64,
    pub touch_interval_ms: i64,
    pub idle_timeout_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthSessionSummary {
    pub id: EntityId,
    pub status: AuthSessionStatus,
    pub auth_revision: Revision,
    pub auth_level: AuthLevel,
    pub created_at_ms: i64,
    pub authenticated_at_ms: i64,
    pub recent_auth_at_ms: i64,
    pub last_seen_at_ms: i64,
    pub idle_expires_at_ms: i64,
    pub absolute_expires_at_ms: i64,
    pub has_ip_context: bool,
    pub has_user_agent_context: bool,
    pub revoked_at_ms: Option<i64>,
    pub revoked_reason: Option<SessionRevocationReason>,
    pub revision: Revision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticatedSession {
    pub session: AuthSessionSummary,
    pub user_id: EntityId,
    pub username: Username,
    pub role: UserRole,
    pub principal_label: PrincipalLabel,
    pub force_password_change: bool,
    pub user_revision: Revision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SessionAuthenticationOutcome {
    Authenticated(AuthenticatedSession),
    InvalidSession,
    InvalidCsrf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogoutAllResult {
    pub revoked_sessions: u64,
    pub auth_revision: Revision,
    pub kept_current: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PasswordChangeResult {
    pub session: AuthSessionSummary,
    pub revoked_sessions: u64,
    pub auth_revision: Revision,
}

#[derive(Clone, Copy)]
pub struct PasswordChangeRotation<'a> {
    pub user_id: EntityId,
    pub current_session_id: EntityId,
    pub expected_user_revision: Revision,
    pub new_hash: &'a PasswordHash,
    pub replacement: &'a NewAuthSession,
    pub event: &'a NewLoginSecurityEvent,
    pub now_ms: i64,
}

#[derive(Clone, Copy)]
pub struct UserSessionRevocation<'a> {
    pub user_id: EntityId,
    pub actor_session_id: EntityId,
    pub target_session_id: EntityId,
    pub expected_user_revision: Revision,
    pub expected_auth_revision: Revision,
    pub expected_recent_auth_at_ms: i64,
    pub event: &'a NewLoginSecurityEvent,
    pub now_ms: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoginSecurityReason {
    LoginSucceeded,
    InvalidCredentials,
    RateLimited,
    AccountInactive,
    SessionExpired,
    SessionRevoked,
    CsrfMismatch,
    Logout,
    LogoutAll,
    AuthRevisionChanged,
    ReauthenticationSucceeded,
    PasswordChanged,
}

impl LoginSecurityReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LoginSucceeded => "login_succeeded",
            Self::InvalidCredentials => "invalid_credentials",
            Self::RateLimited => "rate_limited",
            Self::AccountInactive => "account_inactive",
            Self::SessionExpired => "session_expired",
            Self::SessionRevoked => "session_revoked",
            Self::CsrfMismatch => "csrf_mismatch",
            Self::Logout => "logout",
            Self::LogoutAll => "logout_all",
            Self::AuthRevisionChanged => "auth_revision_changed",
            Self::ReauthenticationSucceeded => "reauthentication_succeeded",
            Self::PasswordChanged => "password_changed",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewLoginSecurityEvent {
    pub id: EntityId,
    pub occurred_at_ms: i64,
    pub request_id: String,
    pub reason: LoginSecurityReason,
    pub digest_key_version: u32,
    pub account_hmac: Option<AuthHmac>,
    pub ip_prefix_hmac: Option<AuthHmac>,
    pub user_agent_hash: Option<AuthHmac>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoginAttemptReservation {
    pub key_version: u32,
    pub account_hmac: AuthHmac,
    pub ip_prefix_hmac: AuthHmac,
    pub global_hmac: AuthHmac,
    pub user_agent_hash: AuthHmac,
    pub request_id: String,
    pub now_ms: i64,
    pub window_ms: i64,
    pub account_max_attempts: u32,
    pub ip_max_attempts: u32,
    pub global_max_attempts: u32,
    pub lockout_ms: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoginRateDecision {
    Allowed {
        remaining_attempts: u32,
        reset_at_ms: i64,
    },
    Limited {
        retry_after_ms: i64,
        blocked_until_ms: i64,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootstrapState {
    Uninitialized,
    LegacyNeedsOwner,
    Ready,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseEngine {
    Sqlite,
    Postgres,
}

impl DatabaseEngine {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sqlite => "sqlite",
            Self::Postgres => "postgres",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ConnectionSettings {
    pub max_connections: u32,
    pub acquire_timeout: Duration,
    pub statement_timeout: Duration,
    pub lock_timeout: Duration,
}

#[derive(Clone)]
pub enum Database {
    Sqlite(SqlitePool),
    Postgres(PgPool),
}

impl Database {
    pub fn validate_url(url: &str) -> Result<DatabaseEngine, PersistenceError> {
        if url.starts_with("sqlite:") {
            SqliteConnectOptions::from_str(url)?;
            Ok(DatabaseEngine::Sqlite)
        } else if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            PgConnectOptions::from_str(url)?;
            Ok(DatabaseEngine::Postgres)
        } else {
            Err(PersistenceError::UnsupportedDatabaseUrl)
        }
    }

    pub async fn connect(
        url: &str,
        settings: ConnectionSettings,
    ) -> Result<Self, PersistenceError> {
        let engine = Self::validate_url(url)?;
        if engine == DatabaseEngine::Sqlite {
            let options = SqliteConnectOptions::from_str(url)?
                .create_if_missing(true)
                .foreign_keys(true)
                .journal_mode(SqliteJournalMode::Wal)
                .synchronous(SqliteSynchronous::Normal)
                .busy_timeout(settings.lock_timeout);
            let pool = SqlitePoolOptions::new()
                .max_connections(1)
                .acquire_timeout(settings.acquire_timeout)
                .connect_with(options)
                .await?;
            Ok(Self::Sqlite(pool))
        } else {
            let statement_timeout = duration_millis_string(settings.statement_timeout)?;
            let lock_timeout = duration_millis_string(settings.lock_timeout)?;
            let options = PgConnectOptions::from_str(url)?;
            let pool = PgPoolOptions::new()
                .max_connections(settings.max_connections)
                .acquire_timeout(settings.acquire_timeout)
                .after_connect(move |connection, _metadata| {
                    let statement_timeout = statement_timeout.clone();
                    let lock_timeout = lock_timeout.clone();
                    Box::pin(async move {
                        sqlx::query("SELECT set_config('statement_timeout', $1, false)")
                            .bind(statement_timeout)
                            .execute(&mut *connection)
                            .await?;
                        sqlx::query("SELECT set_config('lock_timeout', $1, false)")
                            .bind(lock_timeout)
                            .execute(&mut *connection)
                            .await?;
                        Ok(())
                    })
                })
                .connect_with(options)
                .await?;
            Ok(Self::Postgres(pool))
        }
    }

    #[must_use]
    pub const fn engine(&self) -> DatabaseEngine {
        match self {
            Self::Sqlite(_) => DatabaseEngine::Sqlite,
            Self::Postgres(_) => DatabaseEngine::Postgres,
        }
    }

    pub async fn migrate(&self) -> Result<(), PersistenceError> {
        match self {
            Self::Sqlite(pool) => SQLITE_MIGRATOR.run(pool).await?,
            Self::Postgres(pool) => POSTGRES_MIGRATOR.run(pool).await?,
        }
        Ok(())
    }

    pub async fn probe(&self) -> Result<(), PersistenceError> {
        match self {
            Self::Sqlite(pool) => {
                sqlx::query("SELECT 1").execute(pool).await?;
            }
            Self::Postgres(pool) => {
                sqlx::query("SELECT 1").execute(pool).await?;
            }
        }
        Ok(())
    }

    pub async fn bootstrap_state(&self) -> Result<BootstrapState, PersistenceError> {
        match self {
            Self::Sqlite(pool) => {
                let row: Option<(String, Option<String>, bool, bool, bool)> = sqlx::query_as(
                    "SELECT status, instance_id, EXISTS(SELECT 1 FROM instances), EXISTS(SELECT 1 FROM users), EXISTS(SELECT 1 FROM users WHERE role='owner' AND status='active' AND deleted_at_ms IS NULL) FROM control_plane_bootstrap WHERE singleton_key=1",
                )
                .fetch_optional(pool)
                .await?;
                let row = row.ok_or(PersistenceError::InconsistentBootstrapState)?;
                classify_bootstrap_record(&row.0, row.1.is_some(), (row.2, row.3, row.4))
            }
            Self::Postgres(pool) => {
                let row: Option<(String, Option<uuid::Uuid>, bool, bool, bool)> = sqlx::query_as(
                    "SELECT status, instance_id, EXISTS(SELECT 1 FROM instances), EXISTS(SELECT 1 FROM users), EXISTS(SELECT 1 FROM users WHERE role='owner' AND status='active' AND deleted_at_ms IS NULL) FROM control_plane_bootstrap WHERE singleton_key=1",
                )
                .fetch_optional(pool)
                .await?;
                let row = row.ok_or(PersistenceError::InconsistentBootstrapState)?;
                classify_bootstrap_record(&row.0, row.1.is_some(), (row.2, row.3, row.4))
            }
        }
    }

    pub async fn is_initialized(&self) -> Result<bool, PersistenceError> {
        Ok(self.bootstrap_state().await? == BootstrapState::Ready)
    }

    #[cfg(test)]
    async fn bootstrap_control_plane(
        &self,
        instance: &Instance,
        owner: &UserAccount,
        settings: &SubscriptionBehaviorSettings,
    ) -> Result<EntityId, PersistenceError> {
        self.bootstrap_control_plane_inner(instance, owner, settings, None)
            .await
    }

    pub async fn bootstrap_control_plane_with_recovery(
        &self,
        instance: &Instance,
        owner: &UserAccount,
        settings: &SubscriptionBehaviorSettings,
        recovery_codes: &NewRecoveryCodeSet,
    ) -> Result<EntityId, PersistenceError> {
        self.bootstrap_control_plane_inner(instance, owner, settings, Some(recovery_codes))
            .await
    }

    async fn bootstrap_control_plane_inner(
        &self,
        instance: &Instance,
        owner: &UserAccount,
        settings: &SubscriptionBehaviorSettings,
        recovery_codes: Option<&NewRecoveryCodeSet>,
    ) -> Result<EntityId, PersistenceError> {
        if instance.created_at_ms < 0 || owner.created_at_ms < 0 {
            return Err(PersistenceError::InvalidTimestamp);
        }
        if let Some(recovery_codes) = recovery_codes {
            validate_recovery_code_set(recovery_codes)?;
            if recovery_codes.created_at_ms != owner.created_at_ms {
                return Err(PersistenceError::InvalidRecoveryCodeSet);
            }
        }
        let instance_revision = i64::try_from(instance.revision.value())
            .map_err(|_| PersistenceError::RevisionOutOfRange)?;
        let owner_revision = i64::try_from(owner.revision.value())
            .map_err(|_| PersistenceError::RevisionOutOfRange)?;
        let settings_json = serde_json::to_string(settings)?;

        match self {
            Self::Sqlite(pool) => {
                bootstrap_sqlite(
                    pool,
                    instance,
                    owner,
                    &settings_json,
                    instance_revision,
                    owner_revision,
                    recovery_codes,
                )
                .await
            }
            Self::Postgres(pool) => {
                bootstrap_postgres(
                    pool,
                    instance,
                    owner,
                    &settings_json,
                    instance_revision,
                    owner_revision,
                    recovery_codes,
                )
                .await
            }
        }
    }

    /// Creates the typed root-key canary if absent and always returns the persisted winner. The
    /// unique active-binding index makes concurrent startup safe without overwriting ciphertext.
    pub async fn ensure_secret_record(
        &self,
        record: &NewSecretRecord,
    ) -> Result<StoredSecretRecord, PersistenceError> {
        validate_secret_record(record)?;
        // Credential-owned and ceremony-owned material has lifecycle/CAS rules that this
        // singleton escape hatch cannot enforce.
        if is_transaction_owned_secret(record.binding.purpose) {
            return Err(PersistenceError::InvalidSecretRecord);
        }
        match self {
            Self::Sqlite(pool) => ensure_secret_record_sqlite(pool, record).await,
            Self::Postgres(pool) => ensure_secret_record_postgres(pool, record).await,
        }
    }

    /// Loads a singleton secret binding. TOTP is rejected because an active credential and its
    /// pending replacement may coexist; callers must load a TOTP seed through its credential ID.
    pub async fn active_secret_record(
        &self,
        binding: SecretBinding,
    ) -> Result<Option<StoredSecretRecord>, PersistenceError> {
        if is_transaction_owned_secret(binding.purpose) {
            return Err(PersistenceError::InvalidSecretRecord);
        }
        match self {
            Self::Sqlite(pool) => {
                let row = sqlx::query(
                    "SELECT id,owner_type,owner_id,purpose,schema_version,key_version,nonce,ciphertext,aad_hash,created_at_ms,rotated_from,revision FROM secret_records WHERE owner_type=? AND owner_id=? AND purpose=? AND deleted_at_ms IS NULL",
                )
                .bind(binding.owner_kind.as_str())
                .bind(binding.owner_id.to_string())
                .bind(binding.purpose.as_str())
                .fetch_optional(pool)
                .await?;
                row.map(decode_sqlite_secret_record).transpose()
            }
            Self::Postgres(pool) => {
                let row = sqlx::query(
                    "SELECT id,owner_type,owner_id,purpose,schema_version,key_version,nonce,ciphertext,aad_hash,created_at_ms,rotated_from,revision FROM secret_records WHERE owner_type=$1 AND owner_id=$2 AND purpose=$3 AND deleted_at_ms IS NULL",
                )
                .bind(binding.owner_kind.as_str())
                .bind(binding.owner_id)
                .bind(binding.purpose.as_str())
                .fetch_optional(pool)
                .await?;
                row.map(decode_postgres_secret_record).transpose()
            }
        }
    }

    /// Rotates a singleton secret binding. TOTP is rejected until a credential-aware rewrap
    /// transaction can identify and conditionally update one exact seed record.
    pub async fn rotate_secret_record(
        &self,
        expected: &StoredSecretRecord,
        replacement: &NewSecretRecord,
        now_ms: i64,
    ) -> Result<StoredSecretRecord, PersistenceError> {
        validate_non_negative_timestamp(now_ms)?;
        validate_secret_record(replacement)?;
        if is_transaction_owned_secret(expected.binding.purpose)
            || is_transaction_owned_secret(replacement.binding.purpose)
        {
            return Err(PersistenceError::InvalidSecretRecord);
        }
        if expected.binding != replacement.binding
            || replacement.rotated_from != Some(expected.id)
            || replacement.created_at_ms != now_ms
        {
            return Err(PersistenceError::InvalidSecretRecord);
        }
        match self {
            Self::Sqlite(pool) => {
                rotate_secret_record_sqlite(pool, expected, replacement, now_ms).await
            }
            Self::Postgres(pool) => {
                rotate_secret_record_postgres(pool, expected, replacement, now_ms).await
            }
        }
    }

    pub async fn recovery_code_summary(
        &self,
        user_id: EntityId,
    ) -> Result<Option<RecoveryCodeSetSummary>, PersistenceError> {
        let row: Option<(i64, i64, i64, i64)> = match self {
            Self::Sqlite(pool) => sqlx::query_as(
                "SELECT rcs.set_version,rcs.total_count,SUM(CASE WHEN rc.consumed_at_ms IS NULL THEN 1 ELSE 0 END),rcs.created_at_ms FROM recovery_code_sets rcs JOIN recovery_codes rc ON rc.user_id=rcs.user_id AND rc.set_version=rcs.set_version WHERE rcs.user_id=? AND rcs.status='active' GROUP BY rcs.set_version,rcs.total_count,rcs.created_at_ms",
            )
            .bind(user_id.to_string())
            .fetch_optional(pool)
            .await?,
            Self::Postgres(pool) => sqlx::query_as(
                "SELECT rcs.set_version,rcs.total_count::bigint,SUM(CASE WHEN rc.consumed_at_ms IS NULL THEN 1 ELSE 0 END)::bigint,rcs.created_at_ms FROM recovery_code_sets rcs JOIN recovery_codes rc ON rc.user_id=rcs.user_id AND rc.set_version=rcs.set_version WHERE rcs.user_id=$1 AND rcs.status='active' GROUP BY rcs.set_version,rcs.total_count,rcs.created_at_ms",
            )
            .bind(user_id.into_uuid())
            .fetch_optional(pool)
            .await?,
        };
        row.map(decode_recovery_code_summary).transpose()
    }

    pub async fn replace_recovery_codes(
        &self,
        command: RecoveryCodeReplacement<'_>,
    ) -> Result<RecoveryCodeSetSummary, PersistenceError> {
        validate_recovery_code_set(command.replacement)?;
        validate_non_negative_timestamp(command.now_ms)?;
        if command.replacement.created_at_ms != command.now_ms {
            return Err(PersistenceError::InvalidRecoveryCodeSet);
        }
        match self {
            Self::Sqlite(pool) => replace_recovery_codes_sqlite(pool, &command).await,
            Self::Postgres(pool) => replace_recovery_codes_postgres(pool, &command).await,
        }
    }

    /// Atomically consumes one active-set code. A concurrent replay observes zero affected rows.
    pub async fn consume_recovery_code(
        &self,
        command: &RecoveryCodeConsumption,
    ) -> Result<bool, PersistenceError> {
        validate_non_negative_timestamp(command.now_ms)?;
        database_key_version(command.digest_key_version)?;
        let affected = match self {
            Self::Sqlite(pool) => sqlx::query(
                "UPDATE recovery_codes SET consumed_at_ms=? WHERE user_id=? AND digest_key_version=? AND code_hmac=? AND created_at_ms<=? AND consumed_at_ms IS NULL AND EXISTS (SELECT 1 FROM recovery_code_sets rcs WHERE rcs.user_id=recovery_codes.user_id AND rcs.set_version=recovery_codes.set_version AND rcs.status='active')",
            )
            .bind(command.now_ms)
            .bind(command.user_id.to_string())
            .bind(i64::from(command.digest_key_version))
            .bind(command.code_hmac.as_slice())
            .bind(command.now_ms)
            .execute(pool)
            .await?
            .rows_affected(),
            Self::Postgres(pool) => sqlx::query(
                "UPDATE recovery_codes rc SET consumed_at_ms=$1 FROM recovery_code_sets rcs WHERE rc.user_id=$2 AND rc.digest_key_version=$3 AND rc.code_hmac=$4 AND rc.created_at_ms<=$1 AND rc.consumed_at_ms IS NULL AND rcs.user_id=rc.user_id AND rcs.set_version=rc.set_version AND rcs.status='active'",
            )
            .bind(command.now_ms)
            .bind(command.user_id.into_uuid())
            .bind(i32::try_from(command.digest_key_version).map_err(|_| PersistenceError::InvalidKeyVersion)?)
            .bind(command.code_hmac.as_slice())
            .execute(pool)
            .await?
            .rows_affected(),
        };
        Ok(affected == 1)
    }

    pub async fn active_owner_count(&self) -> Result<i64, PersistenceError> {
        match self {
            Self::Sqlite(pool) => Ok(sqlx::query_scalar(
                "SELECT COUNT(*) FROM users WHERE role='owner' AND status='active' AND deleted_at_ms IS NULL",
            )
            .fetch_one(pool)
            .await?),
            Self::Postgres(pool) => Ok(sqlx::query_scalar(
                "SELECT COUNT(*) FROM users WHERE role='owner' AND status='active' AND deleted_at_ms IS NULL",
            )
            .fetch_one(pool)
            .await?),
        }
    }

    pub async fn instance(&self) -> Result<Option<Instance>, PersistenceError> {
        match self {
            Self::Sqlite(pool) => {
                let row = sqlx::query(
                    "SELECT id, public_id, name, created_at_ms, revision FROM instances LIMIT 1",
                )
                .fetch_optional(pool)
                .await?;
                row.map(decode_sqlite_instance).transpose()
            }
            Self::Postgres(pool) => {
                let row = sqlx::query(
                    "SELECT id, public_id, name, created_at_ms, revision FROM instances LIMIT 1",
                )
                .fetch_optional(pool)
                .await?;
                row.map(decode_postgres_instance).transpose()
            }
        }
    }

    pub async fn subscription_settings(
        &self,
        instance_id: EntityId,
    ) -> Result<Option<(SubscriptionBehaviorSettings, Revision)>, PersistenceError> {
        let row: Option<(String, i64)> = match self {
            Self::Sqlite(pool) => sqlx::query_as(
                "SELECT value_json, revision FROM instance_settings WHERE instance_id=? AND key=? AND schema_version=?",
            )
            .bind(instance_id.to_string())
            .bind(SUBSCRIPTION_SETTINGS_KEY)
            .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
            .fetch_optional(pool)
            .await?,
            Self::Postgres(pool) => sqlx::query_as(
                "SELECT value_json::text, revision FROM instance_settings WHERE instance_id=$1 AND key=$2 AND schema_version=$3",
            )
            .bind(instance_id.into_uuid())
            .bind(SUBSCRIPTION_SETTINGS_KEY)
            .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
            .fetch_optional(pool)
            .await?,
        };
        row.map(|(json, revision)| {
            let revision =
                u64::try_from(revision).map_err(|_| PersistenceError::RevisionOutOfRange)?;
            Ok((serde_json::from_str(&json)?, Revision::from_value(revision)))
        })
        .transpose()
    }

    pub async fn save_subscription_settings(
        &self,
        instance_id: EntityId,
        settings: &SubscriptionBehaviorSettings,
        expected_revision: Option<Revision>,
        actor_id: EntityId,
        updated_at_ms: i64,
    ) -> Result<Revision, PersistenceError> {
        if updated_at_ms < 0 {
            return Err(PersistenceError::InvalidTimestamp);
        }
        let json = serde_json::to_string(settings)?;
        match expected_revision {
            None => {
                let result = match self {
                    Self::Sqlite(pool) => sqlx::query(
                        "INSERT INTO instance_settings (instance_id,key,schema_version,value_json,revision,updated_by,updated_at_ms) VALUES (?,?,?,?,0,?,?) ON CONFLICT(instance_id,key) DO NOTHING",
                    )
                    .bind(instance_id.to_string())
                    .bind(SUBSCRIPTION_SETTINGS_KEY)
                    .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
                    .bind(json)
                    .bind(actor_id.to_string())
                    .bind(updated_at_ms)
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected()),
                    Self::Postgres(pool) => sqlx::query(
                        "INSERT INTO instance_settings (instance_id,key,schema_version,value_json,revision,updated_by,updated_at_ms) VALUES ($1,$2,$3,$4::jsonb,0,$5,$6) ON CONFLICT(instance_id,key) DO NOTHING",
                    )
                    .bind(instance_id.into_uuid())
                    .bind(SUBSCRIPTION_SETTINGS_KEY)
                    .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
                    .bind(json)
                    .bind(actor_id.into_uuid())
                    .bind(updated_at_ms)
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected()),
                };
                match result {
                    Ok(1) => Ok(Revision::initial()),
                    Ok(_) => Err(PersistenceError::RevisionConflict),
                    Err(error) => Err(PersistenceError::Sql(error)),
                }
            }
            Some(expected) => {
                let next = expected
                    .next()
                    .map_err(|_| PersistenceError::RevisionOutOfRange)?;
                let expected_value = i64::try_from(expected.value())
                    .map_err(|_| PersistenceError::RevisionOutOfRange)?;
                let next_value = i64::try_from(next.value())
                    .map_err(|_| PersistenceError::RevisionOutOfRange)?;
                let affected = match self {
                    Self::Sqlite(pool) => sqlx::query(
                        "UPDATE instance_settings SET value_json=?,revision=?,updated_by=?,updated_at_ms=? WHERE instance_id=? AND key=? AND schema_version=? AND revision=?",
                    )
                    .bind(json)
                    .bind(next_value)
                    .bind(actor_id.to_string())
                    .bind(updated_at_ms)
                    .bind(instance_id.to_string())
                    .bind(SUBSCRIPTION_SETTINGS_KEY)
                    .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
                    .bind(expected_value)
                    .execute(pool)
                    .await?
                    .rows_affected(),
                    Self::Postgres(pool) => sqlx::query(
                        "UPDATE instance_settings SET value_json=$1::jsonb,revision=$2,updated_by=$3,updated_at_ms=$4 WHERE instance_id=$5 AND key=$6 AND schema_version=$7 AND revision=$8",
                    )
                    .bind(json)
                    .bind(next_value)
                    .bind(actor_id.into_uuid())
                    .bind(updated_at_ms)
                    .bind(instance_id.into_uuid())
                    .bind(SUBSCRIPTION_SETTINGS_KEY)
                    .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
                    .bind(expected_value)
                    .execute(pool)
                    .await?
                    .rows_affected(),
                };
                if affected == 1 {
                    Ok(next)
                } else {
                    Err(PersistenceError::RevisionConflict)
                }
            }
        }
    }

    pub async fn user_credentials_by_normalized_username(
        &self,
        username_norm: &str,
    ) -> Result<Option<UserCredentials>, PersistenceError> {
        let parsed = Username::parse(username_norm.to_owned())?;
        if parsed.normalized() != username_norm {
            return Err(PersistenceError::UsernameIsNotNormalized);
        }
        match self {
            Self::Sqlite(pool) => {
                let row = sqlx::query(
                    "SELECT u.id,u.username,u.password_hash,u.role,u.status,u.principal_label,u.force_password_change,u.revision,a.auth_revision,a.password_changed_at_ms FROM users AS u JOIN user_auth_state AS a ON a.user_id=u.id WHERE u.username_norm=? AND u.deleted_at_ms IS NULL",
                )
                .bind(username_norm)
                .fetch_optional(pool)
                .await?;
                row.map(decode_sqlite_user_credentials).transpose()
            }
            Self::Postgres(pool) => {
                let row = sqlx::query(
                    "SELECT u.id,u.username,u.password_hash,u.role,u.status,u.principal_label,u.force_password_change,u.revision,a.auth_revision,a.password_changed_at_ms FROM users AS u JOIN user_auth_state AS a ON a.user_id=u.id WHERE u.username_norm=$1 AND u.deleted_at_ms IS NULL",
                )
                .bind(username_norm)
                .fetch_optional(pool)
                .await?;
                row.map(decode_postgres_user_credentials).transpose()
            }
        }
    }

    #[cfg(test)]
    async fn create_auth_session(
        &self,
        session: &NewAuthSession,
        success_event: &NewLoginSecurityEvent,
    ) -> Result<AuthSessionSummary, PersistenceError> {
        validate_new_session(session)?;
        validate_security_event(success_event)?;
        if success_event.reason != LoginSecurityReason::LoginSucceeded
            || success_event.occurred_at_ms != session.created_at_ms
            || success_event.account_hmac.is_none()
            || success_event.ip_prefix_hmac != session.ip_prefix_hmac
            || success_event.user_agent_hash != session.user_agent_hash
            || session.ip_prefix_key_version != Some(success_event.digest_key_version)
        {
            return Err(PersistenceError::SessionEventMustRecordLoginSuccess);
        }
        match self {
            Self::Sqlite(pool) => create_session_sqlite(pool, session, success_event).await,
            Self::Postgres(pool) => create_session_postgres(pool, session, success_event).await,
        }
    }

    pub async fn create_auth_session_with_optional_password_upgrade(
        &self,
        session: &NewAuthSession,
        success_event: &NewLoginSecurityEvent,
        expected: &UserCredentials,
        replacement_password_hash: Option<&PasswordHash>,
    ) -> Result<AuthSessionSummary, PersistenceError> {
        validate_new_session(session)?;
        validate_security_event(success_event)?;
        validate_non_negative_timestamp(expected.password_changed_at_ms)?;
        database_revision(expected.user_revision)?;
        database_revision(expected.auth_revision)?;
        if success_event.reason != LoginSecurityReason::LoginSucceeded
            || success_event.occurred_at_ms != session.created_at_ms
            || success_event.account_hmac.is_none()
            || success_event.ip_prefix_hmac != session.ip_prefix_hmac
            || success_event.user_agent_hash != session.user_agent_hash
            || session.ip_prefix_key_version != Some(success_event.digest_key_version)
        {
            return Err(PersistenceError::SessionEventMustRecordLoginSuccess);
        }
        if session.user_id != expected.user_id
            || session.auth_revision != expected.auth_revision
            || expected.status != UserStatus::Active
        {
            return Err(PersistenceError::SessionPrincipalUnavailable);
        }
        match self {
            Self::Sqlite(pool) => {
                create_session_with_password_upgrade_sqlite(
                    pool,
                    session,
                    success_event,
                    expected,
                    replacement_password_hash,
                )
                .await
            }
            Self::Postgres(pool) => {
                create_session_with_password_upgrade_postgres(
                    pool,
                    session,
                    success_event,
                    expected,
                    replacement_password_hash,
                )
                .await
            }
        }
    }

    #[cfg(test)]
    async fn upgrade_password_hash_if_current(
        &self,
        user_id: EntityId,
        expected_user_revision: Revision,
        expected_hash: &PasswordHash,
        replacement_hash: &PasswordHash,
        now_ms: i64,
    ) -> Result<bool, PersistenceError> {
        validate_non_negative_timestamp(now_ms)?;
        if expected_hash == replacement_hash {
            return Ok(false);
        }
        let next_user_revision = expected_user_revision
            .next()
            .map_err(|_| PersistenceError::RevisionOutOfRange)?;
        match self {
            Self::Sqlite(pool) => {
                upgrade_password_hash_sqlite(
                    pool,
                    user_id,
                    expected_user_revision,
                    next_user_revision,
                    expected_hash,
                    replacement_hash,
                    now_ms,
                )
                .await
            }
            Self::Postgres(pool) => {
                upgrade_password_hash_postgres(
                    pool,
                    user_id,
                    expected_user_revision,
                    next_user_revision,
                    expected_hash,
                    replacement_hash,
                    now_ms,
                )
                .await
            }
        }
    }

    pub async fn rotate_current_session(
        &self,
        user_id: EntityId,
        current_session_id: EntityId,
        expected_user_revision: Revision,
        replacement: &NewAuthSession,
        event: &NewLoginSecurityEvent,
        now_ms: i64,
    ) -> Result<AuthSessionSummary, PersistenceError> {
        database_revision(expected_user_revision)?;
        validate_session_rotation_request(
            user_id,
            current_session_id,
            replacement,
            event,
            now_ms,
            LoginSecurityReason::ReauthenticationSucceeded,
        )?;
        match self {
            Self::Sqlite(pool) => {
                rotate_current_session_sqlite(
                    pool,
                    user_id,
                    current_session_id,
                    expected_user_revision,
                    replacement,
                    event,
                    now_ms,
                )
                .await
            }
            Self::Postgres(pool) => {
                rotate_current_session_postgres(
                    pool,
                    user_id,
                    current_session_id,
                    expected_user_revision,
                    replacement,
                    event,
                    now_ms,
                )
                .await
            }
        }
    }

    pub async fn change_password_and_rotate(
        &self,
        rotation: PasswordChangeRotation<'_>,
    ) -> Result<PasswordChangeResult, PersistenceError> {
        database_revision(rotation.expected_user_revision)?;
        validate_session_rotation_request(
            rotation.user_id,
            rotation.current_session_id,
            rotation.replacement,
            rotation.event,
            rotation.now_ms,
            LoginSecurityReason::PasswordChanged,
        )?;
        match self {
            Self::Sqlite(pool) => change_password_and_rotate_sqlite(pool, &rotation).await,
            Self::Postgres(pool) => change_password_and_rotate_postgres(pool, &rotation).await,
        }
    }

    pub async fn authenticate_session(
        &self,
        authentication: &SessionAuthentication,
    ) -> Result<SessionAuthenticationOutcome, PersistenceError> {
        self.authenticate_session_inner(authentication, true).await
    }

    pub async fn authenticate_session_read_only(
        &self,
        authentication: &SessionAuthentication,
    ) -> Result<SessionAuthenticationOutcome, PersistenceError> {
        self.authenticate_session_inner(authentication, false).await
    }

    async fn authenticate_session_inner(
        &self,
        authentication: &SessionAuthentication,
        touch_session: bool,
    ) -> Result<SessionAuthenticationOutcome, PersistenceError> {
        validate_session_authentication(authentication)?;
        match self {
            Self::Sqlite(pool) => {
                authenticate_session_sqlite(pool, authentication, touch_session).await
            }
            Self::Postgres(pool) => {
                authenticate_session_postgres(pool, authentication, touch_session).await
            }
        }
    }

    #[cfg(test)]
    async fn revoke_current_session(
        &self,
        user_id: EntityId,
        session_id: EntityId,
        now_ms: i64,
        reason: SessionRevocationReason,
    ) -> Result<bool, PersistenceError> {
        validate_non_negative_timestamp(now_ms)?;
        let affected = match self {
            Self::Sqlite(pool) => sqlx::query(
                "UPDATE auth_sessions SET status='revoked',revoked_at_ms=MAX(created_at_ms,?),revoked_reason=?,revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE id=? AND user_id=? AND status='active'",
            )
            .bind(now_ms)
            .bind(reason.as_str())
            .bind(session_id.to_string())
            .bind(user_id.to_string())
            .execute(pool)
            .await?
            .rows_affected(),
            Self::Postgres(pool) => sqlx::query(
                "UPDATE auth_sessions SET status='revoked',revoked_at_ms=GREATEST(created_at_ms,$1),revoked_reason=$2,revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE id=$3 AND user_id=$4 AND status='active'",
            )
            .bind(now_ms)
            .bind(reason.as_str())
            .bind(session_id.into_uuid())
            .bind(user_id.into_uuid())
            .execute(pool)
            .await?
            .rows_affected(),
        };
        Ok(affected == 1)
    }

    pub async fn revoke_current_session_with_event(
        &self,
        user_id: EntityId,
        session_id: EntityId,
        now_ms: i64,
        reason: SessionRevocationReason,
        event: &NewLoginSecurityEvent,
    ) -> Result<bool, PersistenceError> {
        validate_non_negative_timestamp(now_ms)?;
        validate_security_event(event)?;
        let reason_matches_event = matches!(
            (reason, event.reason),
            (SessionRevocationReason::Logout, LoginSecurityReason::Logout)
                | (
                    SessionRevocationReason::UserRevoked,
                    LoginSecurityReason::SessionRevoked
                )
        );
        if !reason_matches_event
            || event.occurred_at_ms != now_ms
            || event.account_hmac.is_none()
            || event.ip_prefix_hmac.is_none()
        {
            return Err(PersistenceError::InvalidSessionRevocationEvent);
        }
        match self {
            Self::Sqlite(pool) => {
                revoke_current_session_with_event_sqlite(
                    pool, user_id, session_id, now_ms, reason, event,
                )
                .await
            }
            Self::Postgres(pool) => {
                revoke_current_session_with_event_postgres(
                    pool, user_id, session_id, now_ms, reason, event,
                )
                .await
            }
        }
    }

    pub async fn revoke_user_session_with_event(
        &self,
        revocation: UserSessionRevocation<'_>,
    ) -> Result<bool, PersistenceError> {
        validate_non_negative_timestamp(revocation.now_ms)?;
        validate_non_negative_timestamp(revocation.expected_recent_auth_at_ms)?;
        database_revision(revocation.expected_user_revision)?;
        database_revision(revocation.expected_auth_revision)?;
        validate_security_event(revocation.event)?;
        if revocation.event.reason != LoginSecurityReason::SessionRevoked
            || revocation.event.occurred_at_ms != revocation.now_ms
            || revocation.event.account_hmac.is_none()
            || revocation.event.ip_prefix_hmac.is_none()
        {
            return Err(PersistenceError::InvalidSessionRevocationEvent);
        }
        match self {
            Self::Sqlite(pool) => revoke_user_session_with_event_sqlite(pool, &revocation).await,
            Self::Postgres(pool) => {
                revoke_user_session_with_event_postgres(pool, &revocation).await
            }
        }
    }

    #[cfg(test)]
    async fn logout_all_sessions(
        &self,
        user_id: EntityId,
        now_ms: i64,
    ) -> Result<LogoutAllResult, PersistenceError> {
        self.revoke_all_sessions_inner(user_id, now_ms, SessionRevocationReason::LogoutAll)
            .await
    }

    pub async fn logout_all_sessions_with_event(
        &self,
        user_id: EntityId,
        current_session_id: EntityId,
        expected_recent_auth_at_ms: i64,
        event: &NewLoginSecurityEvent,
        now_ms: i64,
    ) -> Result<LogoutAllResult, PersistenceError> {
        validate_non_negative_timestamp(now_ms)?;
        validate_non_negative_timestamp(expected_recent_auth_at_ms)?;
        validate_security_event(event)?;
        if event.reason != LoginSecurityReason::LogoutAll
            || event.occurred_at_ms != now_ms
            || event.account_hmac.is_none()
            || event.ip_prefix_hmac.is_none()
        {
            return Err(PersistenceError::InvalidSessionRevocationEvent);
        }
        match self {
            Self::Sqlite(pool) => {
                logout_all_sessions_with_event_sqlite(
                    pool,
                    user_id,
                    current_session_id,
                    expected_recent_auth_at_ms,
                    event,
                    now_ms,
                )
                .await
            }
            Self::Postgres(pool) => {
                logout_all_sessions_with_event_postgres(
                    pool,
                    user_id,
                    current_session_id,
                    expected_recent_auth_at_ms,
                    event,
                    now_ms,
                )
                .await
            }
        }
    }

    pub async fn logout_all_sessions_and_rotate(
        &self,
        user_id: EntityId,
        current_session_id: EntityId,
        expected_user_revision: Revision,
        replacement: &NewAuthSession,
        event: &NewLoginSecurityEvent,
        now_ms: i64,
    ) -> Result<LogoutAllResult, PersistenceError> {
        validate_non_negative_timestamp(now_ms)?;
        database_revision(expected_user_revision)?;
        validate_new_session(replacement)?;
        validate_security_event(event)?;
        if replacement.user_id != user_id
            || replacement.id == current_session_id
            || event.reason != LoginSecurityReason::LogoutAll
            || event.occurred_at_ms != now_ms
            || event.account_hmac.is_none()
            || event.ip_prefix_hmac != replacement.ip_prefix_hmac
            || event.user_agent_hash != replacement.user_agent_hash
            || replacement.ip_prefix_key_version != Some(event.digest_key_version)
        {
            return Err(PersistenceError::InvalidSessionRotation);
        }
        match self {
            Self::Sqlite(pool) => {
                rotate_logout_all_sqlite(
                    pool,
                    user_id,
                    current_session_id,
                    expected_user_revision,
                    replacement,
                    event,
                    now_ms,
                )
                .await
            }
            Self::Postgres(pool) => {
                rotate_logout_all_postgres(
                    pool,
                    user_id,
                    current_session_id,
                    expected_user_revision,
                    replacement,
                    event,
                    now_ms,
                )
                .await
            }
        }
    }

    #[cfg(test)]
    async fn revoke_all_sessions_inner(
        &self,
        user_id: EntityId,
        now_ms: i64,
        reason: SessionRevocationReason,
    ) -> Result<LogoutAllResult, PersistenceError> {
        validate_non_negative_timestamp(now_ms)?;
        match self {
            Self::Sqlite(pool) => revoke_all_sessions_sqlite(pool, user_id, now_ms, reason).await,
            Self::Postgres(pool) => {
                revoke_all_sessions_postgres(pool, user_id, now_ms, reason).await
            }
        }
    }

    pub async fn list_user_sessions(
        &self,
        user_id: EntityId,
    ) -> Result<Vec<AuthSessionSummary>, PersistenceError> {
        match self {
            Self::Sqlite(pool) => {
                let rows = sqlx::query(
                    "SELECT id,status,auth_revision,auth_level,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_hmac IS NOT NULL AS has_ip_context,user_agent_hash IS NOT NULL AS has_user_agent_context,revoked_at_ms,revoked_reason,revision FROM auth_sessions WHERE user_id=? ORDER BY created_at_ms DESC,id DESC",
                )
                .bind(user_id.to_string())
                .fetch_all(pool)
                .await?;
                rows.into_iter()
                    .map(decode_sqlite_session_summary)
                    .collect()
            }
            Self::Postgres(pool) => {
                let rows = sqlx::query(
                    "SELECT id,status,auth_revision,auth_level,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_hmac IS NOT NULL AS has_ip_context,user_agent_hash IS NOT NULL AS has_user_agent_context,revoked_at_ms,revoked_reason,revision FROM auth_sessions WHERE user_id=$1 ORDER BY created_at_ms DESC,id DESC",
                )
                .bind(user_id.into_uuid())
                .fetch_all(pool)
                .await?;
                rows.into_iter()
                    .map(decode_postgres_session_summary)
                    .collect()
            }
        }
    }

    pub async fn list_active_user_sessions(
        &self,
        user_id: EntityId,
        now_ms: i64,
    ) -> Result<Vec<AuthSessionSummary>, PersistenceError> {
        validate_non_negative_timestamp(now_ms)?;
        match self {
            Self::Sqlite(pool) => {
                let rows = sqlx::query(
                    "SELECT s.id,s.status,s.auth_revision,s.auth_level,s.created_at_ms,s.authenticated_at_ms,s.recent_auth_at_ms,s.last_seen_at_ms,s.idle_expires_at_ms,s.absolute_expires_at_ms,s.ip_prefix_hmac IS NOT NULL AS has_ip_context,s.user_agent_hash IS NOT NULL AS has_user_agent_context,s.revoked_at_ms,s.revoked_reason,s.revision FROM auth_sessions AS s JOIN user_auth_state AS a ON a.user_id=s.user_id AND a.auth_revision=s.auth_revision WHERE s.user_id=? AND s.status='active' AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>? ORDER BY s.created_at_ms DESC,s.id DESC",
                )
                .bind(user_id.to_string())
                .bind(now_ms)
                .bind(now_ms)
                .bind(now_ms)
                .fetch_all(pool)
                .await?;
                rows.into_iter()
                    .map(decode_sqlite_session_summary)
                    .collect()
            }
            Self::Postgres(pool) => {
                let rows = sqlx::query(
                    "SELECT s.id,s.status,s.auth_revision,s.auth_level,s.created_at_ms,s.authenticated_at_ms,s.recent_auth_at_ms,s.last_seen_at_ms,s.idle_expires_at_ms,s.absolute_expires_at_ms,s.ip_prefix_hmac IS NOT NULL AS has_ip_context,s.user_agent_hash IS NOT NULL AS has_user_agent_context,s.revoked_at_ms,s.revoked_reason,s.revision FROM auth_sessions AS s JOIN user_auth_state AS a ON a.user_id=s.user_id AND a.auth_revision=s.auth_revision WHERE s.user_id=$1 AND s.status='active' AND s.last_seen_at_ms<=$2 AND s.idle_expires_at_ms>$2 AND s.absolute_expires_at_ms>$2 ORDER BY s.created_at_ms DESC,s.id DESC",
                )
                .bind(user_id.into_uuid())
                .bind(now_ms)
                .fetch_all(pool)
                .await?;
                rows.into_iter()
                    .map(decode_postgres_session_summary)
                    .collect()
            }
        }
    }

    pub async fn reserve_login_attempt(
        &self,
        reservation: &LoginAttemptReservation,
        event: &NewLoginSecurityEvent,
    ) -> Result<LoginRateDecision, PersistenceError> {
        validate_login_attempt_reservation(reservation)?;
        validate_rate_limited_event(reservation, event)?;
        match self {
            Self::Sqlite(pool) => reserve_login_attempt_sqlite(pool, reservation, event).await,
            Self::Postgres(pool) => reserve_login_attempt_postgres(pool, reservation, event).await,
        }
    }

    #[cfg(test)]
    async fn clear_login_account_bucket(
        &self,
        key_version: u32,
        account_hmac: &AuthHmac,
    ) -> Result<bool, PersistenceError> {
        let key_version = database_key_version(key_version)?;
        let affected = match self {
            Self::Sqlite(pool) => sqlx::query(
                "DELETE FROM login_rate_buckets WHERE scope='account' AND key_version=? AND bucket_hmac=?",
            )
            .bind(key_version)
            .bind(account_hmac.as_slice())
            .execute(pool)
            .await?
            .rows_affected(),
            Self::Postgres(pool) => sqlx::query(
                "DELETE FROM login_rate_buckets WHERE scope='account' AND key_version=$1 AND bucket_hmac=$2",
            )
            .bind(key_version)
            .bind(account_hmac.as_slice())
            .execute(pool)
            .await?
            .rows_affected(),
        };
        Ok(affected == 1)
    }

    pub async fn record_login_security_event(
        &self,
        event: &NewLoginSecurityEvent,
    ) -> Result<(), PersistenceError> {
        if event.reason == LoginSecurityReason::RateLimited {
            return Err(PersistenceError::RateLimitedEventMustBeAtomic);
        }
        validate_security_event(event)?;
        match self {
            Self::Sqlite(pool) => insert_security_event_sqlite(pool, event).await?,
            Self::Postgres(pool) => insert_security_event_postgres(pool, event).await?,
        }
        Ok(())
    }
}

fn database_revision(revision: Revision) -> Result<i64, PersistenceError> {
    i64::try_from(revision.value()).map_err(|_| PersistenceError::RevisionOutOfRange)
}

fn decode_revision(revision: i64) -> Result<Revision, PersistenceError> {
    let revision = u64::try_from(revision).map_err(|_| PersistenceError::RevisionOutOfRange)?;
    Ok(Revision::from_value(revision))
}

fn database_key_version(key_version: u32) -> Result<i32, PersistenceError> {
    if key_version == 0 || key_version > i32::MAX as u32 {
        return Err(PersistenceError::InvalidKeyVersion);
    }
    i32::try_from(key_version).map_err(|_| PersistenceError::InvalidKeyVersion)
}

fn validate_non_negative_timestamp(timestamp_ms: i64) -> Result<(), PersistenceError> {
    if timestamp_ms < 0 {
        return Err(PersistenceError::InvalidTimestamp);
    }
    Ok(())
}

fn validate_new_session(session: &NewAuthSession) -> Result<(), PersistenceError> {
    database_key_version(session.token_key_version)?;
    database_key_version(session.csrf_key_version)?;
    database_revision(session.auth_revision)?;
    database_revision(session.revision)?;
    match (session.ip_prefix_key_version, session.ip_prefix_hmac) {
        (Some(version), Some(_)) => {
            database_key_version(version)?;
        }
        (None, None) => {}
        _ => return Err(PersistenceError::InvalidSessionContext),
    }
    if session.created_at_ms < 0
        || session.authenticated_at_ms < 0
        || session.recent_auth_at_ms < session.authenticated_at_ms
        || session.last_seen_at_ms < session.created_at_ms
        || session.last_seen_at_ms < session.recent_auth_at_ms
        || session.idle_expires_at_ms <= session.last_seen_at_ms
        || session.absolute_expires_at_ms <= session.created_at_ms
        || session.idle_expires_at_ms > session.absolute_expires_at_ms
    {
        return Err(PersistenceError::InvalidSessionTimeline);
    }
    Ok(())
}

fn validate_session_authentication(
    authentication: &SessionAuthentication,
) -> Result<(), PersistenceError> {
    database_key_version(authentication.token_key_version)?;
    match (authentication.csrf_key_version, authentication.csrf_hmac) {
        (Some(version), Some(_)) => {
            database_key_version(version)?;
        }
        (None, None) => {}
        _ => return Err(PersistenceError::InvalidCsrfBinding),
    }
    if authentication.now_ms < 0
        || authentication.touch_interval_ms <= 0
        || authentication.idle_timeout_ms <= 0
        || authentication
            .now_ms
            .checked_add(authentication.idle_timeout_ms)
            .is_none()
    {
        return Err(PersistenceError::InvalidSessionTimingPolicy);
    }
    Ok(())
}

fn validate_security_event(event: &NewLoginSecurityEvent) -> Result<(), PersistenceError> {
    validate_non_negative_timestamp(event.occurred_at_ms)?;
    database_key_version(event.digest_key_version)?;
    if event.request_id.is_empty()
        || event.request_id.len() > 128
        || !event
            .request_id
            .bytes()
            .all(|byte| (0x21..=0x7e).contains(&byte))
    {
        return Err(PersistenceError::InvalidRequestId);
    }
    Ok(())
}

fn validate_session_rotation_request(
    user_id: EntityId,
    current_session_id: EntityId,
    replacement: &NewAuthSession,
    event: &NewLoginSecurityEvent,
    now_ms: i64,
    expected_reason: LoginSecurityReason,
) -> Result<(), PersistenceError> {
    validate_non_negative_timestamp(now_ms)?;
    validate_new_session(replacement)?;
    validate_security_event(event)?;
    if replacement.user_id != user_id
        || replacement.id == current_session_id
        || event.reason != expected_reason
        || event.occurred_at_ms != now_ms
        || event.account_hmac.is_none()
        || event.ip_prefix_hmac != replacement.ip_prefix_hmac
        || event.user_agent_hash != replacement.user_agent_hash
        || replacement.ip_prefix_key_version != Some(event.digest_key_version)
    {
        return Err(PersistenceError::InvalidSessionRotation);
    }
    Ok(())
}

fn validate_same_revision_rotation_replacement(
    replacement: &NewAuthSession,
    expected_auth_revision: Revision,
    current_auth_level: AuthLevel,
    expected_authenticated_at_ms: i64,
    expected_recent_auth_at_ms: i64,
    current_absolute_expires_at_ms: i64,
    now_ms: i64,
) -> Result<(), PersistenceError> {
    if replacement.auth_revision != expected_auth_revision
        || replacement.auth_level != current_auth_level
        || replacement.created_at_ms != now_ms
        || replacement.authenticated_at_ms != expected_authenticated_at_ms
        || replacement.recent_auth_at_ms != expected_recent_auth_at_ms
        || replacement.last_seen_at_ms != now_ms
        || replacement.absolute_expires_at_ms > current_absolute_expires_at_ms
        || replacement.revision != Revision::initial()
    {
        return Err(PersistenceError::InvalidSessionRotation);
    }
    Ok(())
}

fn validate_password_rotation_replacement(
    replacement: &NewAuthSession,
    current_auth_revision: Revision,
    current_auth_level: AuthLevel,
    current_authenticated_at_ms: i64,
    current_recent_auth_at_ms: i64,
    current_absolute_expires_at_ms: i64,
    now_ms: i64,
) -> Result<Revision, PersistenceError> {
    let next_auth_revision = current_auth_revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    validate_same_revision_rotation_replacement(
        replacement,
        next_auth_revision,
        current_auth_level,
        current_authenticated_at_ms,
        current_recent_auth_at_ms,
        current_absolute_expires_at_ms,
        now_ms,
    )?;
    Ok(next_auth_revision)
}

fn validate_login_attempt_reservation(
    reservation: &LoginAttemptReservation,
) -> Result<(), PersistenceError> {
    validate_non_negative_timestamp(reservation.now_ms)?;
    database_key_version(reservation.key_version)?;
    if reservation.window_ms <= 0
        || reservation.lockout_ms <= 0
        || reservation.lockout_ms < reservation.window_ms
        || [
            reservation.account_max_attempts,
            reservation.ip_max_attempts,
            reservation.global_max_attempts,
        ]
        .into_iter()
        .any(|limit| limit == 0 || limit >= i32::MAX as u32)
        || reservation
            .now_ms
            .checked_add(reservation.window_ms)
            .is_none()
        || reservation
            .now_ms
            .checked_add(reservation.lockout_ms)
            .is_none()
    {
        return Err(PersistenceError::InvalidLoginRatePolicy);
    }
    Ok(())
}

fn validate_rate_limited_event(
    reservation: &LoginAttemptReservation,
    event: &NewLoginSecurityEvent,
) -> Result<(), PersistenceError> {
    validate_security_event(event)?;
    if event.reason != LoginSecurityReason::RateLimited
        || event.occurred_at_ms != reservation.now_ms
        || event.digest_key_version != reservation.key_version
        || event.request_id != reservation.request_id
        || event.account_hmac != Some(reservation.account_hmac)
        || event.ip_prefix_hmac != Some(reservation.ip_prefix_hmac)
        || event.user_agent_hash != Some(reservation.user_agent_hash)
    {
        return Err(PersistenceError::InvalidLoginRateEvent);
    }
    Ok(())
}

fn decode_sqlite_user_credentials(
    row: sqlx::sqlite::SqliteRow,
) -> Result<UserCredentials, PersistenceError> {
    let id = uuid::Uuid::parse_str(row.try_get("id")?)?;
    decode_user_credentials(
        id,
        row.try_get("username")?,
        row.try_get("password_hash")?,
        row.try_get("role")?,
        row.try_get("status")?,
        row.try_get("principal_label")?,
        row.try_get("force_password_change")?,
        row.try_get("revision")?,
        row.try_get("auth_revision")?,
        row.try_get("password_changed_at_ms")?,
    )
}

fn decode_postgres_user_credentials(
    row: sqlx::postgres::PgRow,
) -> Result<UserCredentials, PersistenceError> {
    decode_user_credentials(
        row.try_get("id")?,
        row.try_get("username")?,
        row.try_get("password_hash")?,
        row.try_get("role")?,
        row.try_get("status")?,
        row.try_get("principal_label")?,
        row.try_get("force_password_change")?,
        row.try_get("revision")?,
        row.try_get("auth_revision")?,
        row.try_get("password_changed_at_ms")?,
    )
}

#[allow(clippy::too_many_arguments)]
fn decode_user_credentials(
    id: uuid::Uuid,
    username: String,
    password_hash: String,
    role: String,
    status: String,
    principal_label: String,
    force_password_change: bool,
    user_revision: i64,
    auth_revision: i64,
    password_changed_at_ms: i64,
) -> Result<UserCredentials, PersistenceError> {
    Ok(UserCredentials {
        user_id: EntityId::from_uuid(id),
        username: Username::parse(username)?,
        password_hash: PasswordHash::parse(password_hash)?,
        role: UserRole::parse(&role)?,
        status: UserStatus::parse(&status)?,
        principal_label: PrincipalLabel::parse(principal_label)?,
        force_password_change,
        user_revision: decode_revision(user_revision)?,
        auth_revision: decode_revision(auth_revision)?,
        password_changed_at_ms,
    })
}

fn new_session_summary(session: &NewAuthSession) -> AuthSessionSummary {
    AuthSessionSummary {
        id: session.id,
        status: AuthSessionStatus::Active,
        auth_revision: session.auth_revision,
        auth_level: session.auth_level,
        created_at_ms: session.created_at_ms,
        authenticated_at_ms: session.authenticated_at_ms,
        recent_auth_at_ms: session.recent_auth_at_ms,
        last_seen_at_ms: session.last_seen_at_ms,
        idle_expires_at_ms: session.idle_expires_at_ms,
        absolute_expires_at_ms: session.absolute_expires_at_ms,
        has_ip_context: session.ip_prefix_hmac.is_some(),
        has_user_agent_context: session.user_agent_hash.is_some(),
        revoked_at_ms: None,
        revoked_reason: None,
        revision: session.revision,
    }
}

async fn insert_security_event_sqlite<'executor, ExecutorType>(
    executor: ExecutorType,
    event: &NewLoginSecurityEvent,
) -> Result<(), PersistenceError>
where
    ExecutorType: sqlx::Executor<'executor, Database = sqlx::Sqlite>,
{
    sqlx::query(
        "INSERT INTO login_security_events (id,occurred_at_ms,request_id,reason,digest_key_version,account_hmac,ip_prefix_hmac,user_agent_hash) VALUES (?,?,?,?,?,?,?,?)",
    )
    .bind(event.id.to_string())
    .bind(event.occurred_at_ms)
    .bind(&event.request_id)
    .bind(event.reason.as_str())
    .bind(database_key_version(event.digest_key_version)?)
    .bind(event.account_hmac.as_ref().map(|value| value.as_slice()))
    .bind(event.ip_prefix_hmac.as_ref().map(|value| value.as_slice()))
    .bind(event.user_agent_hash.as_ref().map(|value| value.as_slice()))
    .execute(executor)
    .await?;
    Ok(())
}

async fn insert_security_event_postgres<'executor, ExecutorType>(
    executor: ExecutorType,
    event: &NewLoginSecurityEvent,
) -> Result<(), PersistenceError>
where
    ExecutorType: sqlx::Executor<'executor, Database = sqlx::Postgres>,
{
    sqlx::query(
        "INSERT INTO login_security_events (id,occurred_at_ms,request_id,reason,digest_key_version,account_hmac,ip_prefix_hmac,user_agent_hash) VALUES ($1,$2,$3,$4,$5,$6,$7,$8)",
    )
    .bind(event.id.into_uuid())
    .bind(event.occurred_at_ms)
    .bind(&event.request_id)
    .bind(event.reason.as_str())
    .bind(database_key_version(event.digest_key_version)?)
    .bind(event.account_hmac.as_ref().map(|value| value.as_slice()))
    .bind(event.ip_prefix_hmac.as_ref().map(|value| value.as_slice()))
    .bind(event.user_agent_hash.as_ref().map(|value| value.as_slice()))
    .execute(executor)
    .await?;
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PasswordUpgradeDecision {
    NoUpgrade,
    Apply,
    ConcurrentWinner,
}

type PasswordUpgradeStateRow = (i64, i64, String, String, String, String, String, bool, i64);

async fn lock_auth_revision_barrier_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: EntityId,
) -> Result<Option<i64>, PersistenceError> {
    Ok(
        sqlx::query_scalar("SELECT auth_revision FROM user_auth_state WHERE user_id=$1 FOR UPDATE")
            .bind(user_id.into_uuid())
            .fetch_optional(&mut **transaction)
            .await?,
    )
}

async fn lock_active_user_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: EntityId,
    expected_revision: Option<Revision>,
) -> Result<bool, PersistenceError> {
    let row: Option<i64> = match expected_revision {
        Some(expected_revision) => sqlx::query_scalar(
            "SELECT 1::BIGINT FROM users WHERE id=$1 AND status='active' AND deleted_at_ms IS NULL AND revision=$2 FOR UPDATE",
        )
        .bind(user_id.into_uuid())
        .bind(database_revision(expected_revision)?)
        .fetch_optional(&mut **transaction)
        .await?,
        None => sqlx::query_scalar(
            "SELECT 1::BIGINT FROM users WHERE id=$1 AND status='active' AND deleted_at_ms IS NULL FOR UPDATE",
        )
        .bind(user_id.into_uuid())
        .fetch_optional(&mut **transaction)
        .await?,
    };
    Ok(row.is_some())
}

#[allow(clippy::too_many_arguments)]
fn password_upgrade_decision(
    expected: &UserCredentials,
    replacement_password_hash: Option<&PasswordHash>,
    auth_revision: i64,
    password_changed_at_ms: i64,
    username: &str,
    current_hash: &str,
    role: &str,
    status: &str,
    principal_label: &str,
    force_password_change: bool,
    current_user_revision: i64,
) -> Result<PasswordUpgradeDecision, PersistenceError> {
    if auth_revision != database_revision(expected.auth_revision)?
        || password_changed_at_ms != expected.password_changed_at_ms
        || username != expected.username.as_str()
        || role != expected.role.as_str()
        || status != expected.status.as_str()
        || principal_label != expected.principal_label.as_str()
        || force_password_change != expected.force_password_change
    {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    if replacement_password_hash == Some(&expected.password_hash) {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }

    let current_user_revision = decode_revision(current_user_revision)?;
    if current_user_revision == expected.user_revision
        && current_hash == expected.password_hash.as_str()
    {
        return Ok(if replacement_password_hash.is_some() {
            PasswordUpgradeDecision::Apply
        } else {
            PasswordUpgradeDecision::NoUpgrade
        });
    }

    if replacement_password_hash.is_none() {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let concurrent_winner_revision = expected
        .user_revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    if current_user_revision == concurrent_winner_revision
        && current_hash != expected.password_hash.as_str()
    {
        return Ok(PasswordUpgradeDecision::ConcurrentWinner);
    }
    Err(PersistenceError::SessionPrincipalUnavailable)
}

async fn create_session_with_password_upgrade_sqlite(
    pool: &SqlitePool,
    session: &NewAuthSession,
    event: &NewLoginSecurityEvent,
    expected: &UserCredentials,
    replacement_password_hash: Option<&PasswordHash>,
) -> Result<AuthSessionSummary, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let locked =
        sqlx::query("UPDATE user_auth_state SET auth_revision=auth_revision WHERE user_id=?")
            .bind(session.user_id.to_string())
            .execute(&mut *transaction)
            .await?
            .rows_affected();
    if locked != 1 {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let state: Option<PasswordUpgradeStateRow> = sqlx::query_as(
        "SELECT a.auth_revision,a.password_changed_at_ms,u.username,u.password_hash,u.role,u.status,u.principal_label,u.force_password_change,u.revision FROM user_auth_state AS a JOIN users AS u ON u.id=a.user_id WHERE a.user_id=? AND u.deleted_at_ms IS NULL",
    )
    .bind(session.user_id.to_string())
    .fetch_optional(&mut *transaction)
    .await?;
    let (
        auth_revision,
        password_changed_at_ms,
        username,
        current_hash,
        role,
        status,
        principal_label,
        force_password_change,
        current_user_revision,
    ) = state.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    let decision = password_upgrade_decision(
        expected,
        replacement_password_hash,
        auth_revision,
        password_changed_at_ms,
        &username,
        &current_hash,
        &role,
        &status,
        &principal_label,
        force_password_change,
        current_user_revision,
    )?;
    if decision == PasswordUpgradeDecision::Apply {
        let replacement_hash =
            replacement_password_hash.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
        let next_user_revision = expected
            .user_revision
            .next()
            .map_err(|_| PersistenceError::RevisionOutOfRange)?;
        let upgraded = sqlx::query(
            "UPDATE users SET password_hash=?,revision=? WHERE id=? AND revision=? AND password_hash=? AND status='active' AND deleted_at_ms IS NULL",
        )
        .bind(replacement_hash.as_str())
        .bind(database_revision(next_user_revision)?)
        .bind(session.user_id.to_string())
        .bind(database_revision(expected.user_revision)?)
        .bind(expected.password_hash.as_str())
        .execute(&mut *transaction)
        .await?
        .rows_affected();
        if upgraded != 1 {
            return Err(PersistenceError::SessionPrincipalUnavailable);
        }
        let updated = sqlx::query(
            "UPDATE user_auth_state SET updated_at_ms=MAX(updated_at_ms,?) WHERE user_id=? AND auth_revision=? AND password_changed_at_ms=?",
        )
        .bind(session.created_at_ms)
        .bind(session.user_id.to_string())
        .bind(auth_revision)
        .bind(expected.password_changed_at_ms)
        .execute(&mut *transaction)
        .await?
        .rows_affected();
        if updated != 1 {
            return Err(PersistenceError::SessionRevisionConflict);
        }
    }
    insert_rotated_session_sqlite(&mut transaction, session).await?;
    insert_security_event_sqlite(&mut *transaction, event).await?;
    if let Some(account_hmac) = event.account_hmac.as_ref() {
        sqlx::query(
            "DELETE FROM login_rate_buckets WHERE scope='account' AND key_version=? AND bucket_hmac=?",
        )
        .bind(database_key_version(event.digest_key_version)?)
        .bind(account_hmac.as_slice())
        .execute(&mut *transaction)
        .await?;
    }
    transaction.commit().await?;
    Ok(new_session_summary(session))
}

async fn create_session_with_password_upgrade_postgres(
    pool: &PgPool,
    session: &NewAuthSession,
    event: &NewLoginSecurityEvent,
    expected: &UserCredentials,
    replacement_password_hash: Option<&PasswordHash>,
) -> Result<AuthSessionSummary, PersistenceError> {
    let mut transaction = pool.begin().await?;
    if lock_auth_revision_barrier_postgres(&mut transaction, session.user_id)
        .await?
        .is_none()
    {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let state: Option<PasswordUpgradeStateRow> = sqlx::query_as(
        "SELECT a.auth_revision,a.password_changed_at_ms,u.username,u.password_hash,u.role,u.status,u.principal_label,u.force_password_change,u.revision FROM user_auth_state AS a JOIN users AS u ON u.id=a.user_id WHERE a.user_id=$1 AND u.deleted_at_ms IS NULL FOR UPDATE OF u",
    )
    .bind(session.user_id.into_uuid())
    .fetch_optional(&mut *transaction)
    .await?;
    let (
        auth_revision,
        password_changed_at_ms,
        username,
        current_hash,
        role,
        status,
        principal_label,
        force_password_change,
        current_user_revision,
    ) = state.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    let decision = password_upgrade_decision(
        expected,
        replacement_password_hash,
        auth_revision,
        password_changed_at_ms,
        &username,
        &current_hash,
        &role,
        &status,
        &principal_label,
        force_password_change,
        current_user_revision,
    )?;
    if decision == PasswordUpgradeDecision::Apply {
        let replacement_hash =
            replacement_password_hash.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
        let next_user_revision = expected
            .user_revision
            .next()
            .map_err(|_| PersistenceError::RevisionOutOfRange)?;
        let upgraded = sqlx::query(
            "UPDATE users SET password_hash=$1,revision=$2 WHERE id=$3 AND revision=$4 AND password_hash=$5 AND status='active' AND deleted_at_ms IS NULL",
        )
        .bind(replacement_hash.as_str())
        .bind(database_revision(next_user_revision)?)
        .bind(session.user_id.into_uuid())
        .bind(database_revision(expected.user_revision)?)
        .bind(expected.password_hash.as_str())
        .execute(&mut *transaction)
        .await?
        .rows_affected();
        if upgraded != 1 {
            return Err(PersistenceError::SessionPrincipalUnavailable);
        }
        let updated = sqlx::query(
            "UPDATE user_auth_state SET updated_at_ms=GREATEST(updated_at_ms,$1) WHERE user_id=$2 AND auth_revision=$3 AND password_changed_at_ms=$4",
        )
        .bind(session.created_at_ms)
        .bind(session.user_id.into_uuid())
        .bind(auth_revision)
        .bind(expected.password_changed_at_ms)
        .execute(&mut *transaction)
        .await?
        .rows_affected();
        if updated != 1 {
            return Err(PersistenceError::SessionRevisionConflict);
        }
    }
    insert_rotated_session_postgres(&mut transaction, session).await?;
    insert_security_event_postgres(&mut *transaction, event).await?;
    if let Some(account_hmac) = event.account_hmac.as_ref() {
        sqlx::query(
            "DELETE FROM login_rate_buckets WHERE scope='account' AND key_version=$1 AND bucket_hmac=$2",
        )
        .bind(database_key_version(event.digest_key_version)?)
        .bind(account_hmac.as_slice())
        .execute(&mut *transaction)
        .await?;
    }
    transaction.commit().await?;
    Ok(new_session_summary(session))
}

#[allow(clippy::too_many_arguments)]
#[cfg(test)]
async fn upgrade_password_hash_sqlite(
    pool: &SqlitePool,
    user_id: EntityId,
    expected_user_revision: Revision,
    next_user_revision: Revision,
    expected_hash: &PasswordHash,
    replacement_hash: &PasswordHash,
    now_ms: i64,
) -> Result<bool, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let upgraded = sqlx::query(
        "UPDATE users SET password_hash=?,revision=? WHERE id=? AND revision=? AND password_hash=? AND status='active' AND deleted_at_ms IS NULL",
    )
    .bind(replacement_hash.as_str())
    .bind(database_revision(next_user_revision)?)
    .bind(user_id.to_string())
    .bind(database_revision(expected_user_revision)?)
    .bind(expected_hash.as_str())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if upgraded == 0 {
        transaction.commit().await?;
        return Ok(false);
    }
    let updated = sqlx::query(
        "UPDATE user_auth_state SET updated_at_ms=MAX(updated_at_ms,?) WHERE user_id=?",
    )
    .bind(now_ms)
    .bind(user_id.to_string())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if updated != 1 {
        return Err(PersistenceError::AuthStateUnavailable);
    }
    transaction.commit().await?;
    Ok(true)
}

#[allow(clippy::too_many_arguments)]
#[cfg(test)]
async fn upgrade_password_hash_postgres(
    pool: &PgPool,
    user_id: EntityId,
    expected_user_revision: Revision,
    next_user_revision: Revision,
    expected_hash: &PasswordHash,
    replacement_hash: &PasswordHash,
    now_ms: i64,
) -> Result<bool, PersistenceError> {
    let mut transaction = pool.begin().await?;
    if lock_auth_revision_barrier_postgres(&mut transaction, user_id)
        .await?
        .is_none()
    {
        return Err(PersistenceError::AuthStateUnavailable);
    }
    let upgraded = sqlx::query(
        "UPDATE users SET password_hash=$1,revision=$2 WHERE id=$3 AND revision=$4 AND password_hash=$5 AND status='active' AND deleted_at_ms IS NULL",
    )
    .bind(replacement_hash.as_str())
    .bind(database_revision(next_user_revision)?)
    .bind(user_id.into_uuid())
    .bind(database_revision(expected_user_revision)?)
    .bind(expected_hash.as_str())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if upgraded == 0 {
        transaction.commit().await?;
        return Ok(false);
    }
    let updated = sqlx::query(
        "UPDATE user_auth_state SET updated_at_ms=GREATEST(updated_at_ms,$1) WHERE user_id=$2",
    )
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if updated != 1 {
        return Err(PersistenceError::AuthStateUnavailable);
    }
    transaction.commit().await?;
    Ok(true)
}

async fn revoke_current_session_with_event_sqlite(
    pool: &SqlitePool,
    user_id: EntityId,
    session_id: EntityId,
    now_ms: i64,
    reason: SessionRevocationReason,
    event: &NewLoginSecurityEvent,
) -> Result<bool, PersistenceError> {
    let mut transaction = pool.begin().await?;
    sqlx::query("UPDATE auth_sessions SET revision=revision WHERE id=? AND user_id=?")
        .bind(session_id.to_string())
        .bind(user_id.to_string())
        .execute(&mut *transaction)
        .await?;
    let row = sqlx::query(
        "SELECT revision FROM auth_sessions WHERE id=? AND user_id=? AND status='active'",
    )
    .bind(session_id.to_string())
    .bind(user_id.to_string())
    .fetch_optional(&mut *transaction)
    .await?;
    let Some(row) = row else {
        transaction.commit().await?;
        return Ok(false);
    };
    let revision: i64 = row.try_get("revision")?;
    let affected = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=MAX(created_at_ms,?),revoked_reason=?,revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE id=? AND user_id=? AND status='active' AND revision=?",
    )
    .bind(now_ms)
    .bind(reason.as_str())
    .bind(session_id.to_string())
    .bind(user_id.to_string())
    .bind(revision)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if affected != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    insert_security_event_sqlite(&mut *transaction, event).await?;
    transaction.commit().await?;
    Ok(true)
}

async fn revoke_current_session_with_event_postgres(
    pool: &PgPool,
    user_id: EntityId,
    session_id: EntityId,
    now_ms: i64,
    reason: SessionRevocationReason,
    event: &NewLoginSecurityEvent,
) -> Result<bool, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let row = sqlx::query(
        "SELECT revision FROM auth_sessions WHERE id=$1 AND user_id=$2 AND status='active' FOR UPDATE",
    )
    .bind(session_id.into_uuid())
    .bind(user_id.into_uuid())
    .fetch_optional(&mut *transaction)
    .await?;
    let Some(row) = row else {
        transaction.commit().await?;
        return Ok(false);
    };
    let revision: i64 = row.try_get("revision")?;
    let affected = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=GREATEST(created_at_ms,$1),revoked_reason=$2,revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE id=$3 AND user_id=$4 AND status='active' AND revision=$5",
    )
    .bind(now_ms)
    .bind(reason.as_str())
    .bind(session_id.into_uuid())
    .bind(user_id.into_uuid())
    .bind(revision)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if affected != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    insert_security_event_postgres(&mut *transaction, event).await?;
    transaction.commit().await?;
    Ok(true)
}

async fn revoke_user_session_with_event_sqlite(
    pool: &SqlitePool,
    revocation: &UserSessionRevocation<'_>,
) -> Result<bool, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let actor_auth_state_locked =
        sqlx::query("UPDATE user_auth_state SET auth_revision=auth_revision WHERE user_id=?")
            .bind(revocation.user_id.to_string())
            .execute(&mut *transaction)
            .await?
            .rows_affected();
    if actor_auth_state_locked != 1 {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let actor_is_valid: Option<i64> = sqlx::query_scalar(
        "SELECT 1 FROM auth_sessions AS s JOIN users AS u ON u.id=s.user_id JOIN user_auth_state AS a ON a.user_id=s.user_id WHERE s.id=? AND s.user_id=? AND s.status='active' AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND u.revision=? AND a.auth_revision=? AND s.auth_revision=a.auth_revision AND s.recent_auth_at_ms=?",
    )
    .bind(revocation.actor_session_id.to_string())
    .bind(revocation.user_id.to_string())
    .bind(revocation.now_ms)
    .bind(revocation.now_ms)
    .bind(revocation.now_ms)
    .bind(database_revision(revocation.expected_user_revision)?)
    .bind(database_revision(revocation.expected_auth_revision)?)
    .bind(revocation.expected_recent_auth_at_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    if actor_is_valid.is_none() {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let revoked = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=MAX(created_at_ms,?),revoked_reason='user_revoked',revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE id=? AND user_id=? AND status='active'",
    )
    .bind(revocation.now_ms)
    .bind(revocation.target_session_id.to_string())
    .bind(revocation.user_id.to_string())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if revoked == 0 {
        transaction.commit().await?;
        return Ok(false);
    }
    insert_security_event_sqlite(&mut *transaction, revocation.event).await?;
    transaction.commit().await?;
    Ok(true)
}

async fn revoke_user_session_with_event_postgres(
    pool: &PgPool,
    revocation: &UserSessionRevocation<'_>,
) -> Result<bool, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let auth_revision = lock_auth_revision_barrier_postgres(&mut transaction, revocation.user_id)
        .await?
        .ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    if auth_revision != database_revision(revocation.expected_auth_revision)? {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    if !lock_active_user_postgres(
        &mut transaction,
        revocation.user_id,
        Some(revocation.expected_user_revision),
    )
    .await?
    {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let actor_is_valid: Option<i64> = sqlx::query_scalar(
        "SELECT 1::BIGINT FROM auth_sessions WHERE id=$1 AND user_id=$2 AND status='active' AND last_seen_at_ms<=$3 AND idle_expires_at_ms>$3 AND absolute_expires_at_ms>$3 AND auth_revision=$4 AND recent_auth_at_ms=$5 FOR UPDATE",
    )
    .bind(revocation.actor_session_id.into_uuid())
    .bind(revocation.user_id.into_uuid())
    .bind(revocation.now_ms)
    .bind(database_revision(revocation.expected_auth_revision)?)
    .bind(revocation.expected_recent_auth_at_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    if actor_is_valid.is_none() {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let revoked = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=GREATEST(created_at_ms,$1),revoked_reason='user_revoked',revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE id=$2 AND user_id=$3 AND status='active'",
    )
    .bind(revocation.now_ms)
    .bind(revocation.target_session_id.into_uuid())
    .bind(revocation.user_id.into_uuid())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if revoked == 0 {
        transaction.commit().await?;
        return Ok(false);
    }
    insert_security_event_postgres(&mut *transaction, revocation.event).await?;
    transaction.commit().await?;
    Ok(true)
}

#[cfg(test)]
async fn create_session_sqlite(
    pool: &SqlitePool,
    session: &NewAuthSession,
    event: &NewLoginSecurityEvent,
) -> Result<AuthSessionSummary, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let locked =
        sqlx::query("UPDATE user_auth_state SET auth_revision=auth_revision WHERE user_id=?")
            .bind(session.user_id.to_string())
            .execute(&mut *transaction)
            .await?
            .rows_affected();
    if locked != 1 {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let auth_revision: Option<i64> = sqlx::query_scalar(
        "SELECT a.auth_revision FROM user_auth_state AS a JOIN users AS u ON u.id=a.user_id WHERE a.user_id=? AND u.status='active' AND u.deleted_at_ms IS NULL",
    )
    .bind(session.user_id.to_string())
    .fetch_optional(&mut *transaction)
    .await?;
    if auth_revision != Some(database_revision(session.auth_revision)?) {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    sqlx::query(
        "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES (?,?,?,?,?,?,?,?,'active',?,?,?,?,?,?,?,?,?,NULL,NULL,?)",
    )
    .bind(session.id.to_string())
    .bind(session.user_id.to_string())
    .bind(database_key_version(session.token_key_version)?)
    .bind(session.token_hmac.as_slice())
    .bind(database_key_version(session.csrf_key_version)?)
    .bind(session.csrf_hmac.as_slice())
    .bind(database_revision(session.auth_revision)?)
    .bind(session.auth_level.as_str())
    .bind(session.created_at_ms)
    .bind(session.authenticated_at_ms)
    .bind(session.recent_auth_at_ms)
    .bind(session.last_seen_at_ms)
    .bind(session.idle_expires_at_ms)
    .bind(session.absolute_expires_at_ms)
    .bind(
        session
            .ip_prefix_key_version
            .map(database_key_version)
            .transpose()?,
    )
    .bind(session.ip_prefix_hmac.as_ref().map(|value| value.as_slice()))
    .bind(session.user_agent_hash.as_ref().map(|value| value.as_slice()))
    .bind(database_revision(session.revision)?)
    .execute(&mut *transaction)
    .await?;
    insert_security_event_sqlite(&mut *transaction, event).await?;
    if let Some(account_hmac) = event.account_hmac.as_ref() {
        sqlx::query(
            "DELETE FROM login_rate_buckets WHERE scope='account' AND key_version=? AND bucket_hmac=?",
        )
        .bind(database_key_version(event.digest_key_version)?)
        .bind(account_hmac.as_slice())
        .execute(&mut *transaction)
        .await?;
    }
    transaction.commit().await?;
    Ok(new_session_summary(session))
}

#[cfg(test)]
async fn create_session_postgres(
    pool: &PgPool,
    session: &NewAuthSession,
    event: &NewLoginSecurityEvent,
) -> Result<AuthSessionSummary, PersistenceError> {
    let mut transaction = pool.begin().await?;
    if lock_auth_revision_barrier_postgres(&mut transaction, session.user_id)
        .await?
        .is_none()
    {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let auth_revision: Option<i64> = sqlx::query_scalar(
        "SELECT a.auth_revision FROM user_auth_state AS a JOIN users AS u ON u.id=a.user_id WHERE a.user_id=$1 AND u.status='active' AND u.deleted_at_ms IS NULL FOR UPDATE OF u",
    )
    .bind(session.user_id.into_uuid())
    .fetch_optional(&mut *transaction)
    .await?;
    if auth_revision != Some(database_revision(session.auth_revision)?) {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    sqlx::query(
        "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,'active',$9,$10,$11,$12,$13,$14,$15,$16,$17,NULL,NULL,$18)",
    )
    .bind(session.id.into_uuid())
    .bind(session.user_id.into_uuid())
    .bind(database_key_version(session.token_key_version)?)
    .bind(session.token_hmac.as_slice())
    .bind(database_key_version(session.csrf_key_version)?)
    .bind(session.csrf_hmac.as_slice())
    .bind(database_revision(session.auth_revision)?)
    .bind(session.auth_level.as_str())
    .bind(session.created_at_ms)
    .bind(session.authenticated_at_ms)
    .bind(session.recent_auth_at_ms)
    .bind(session.last_seen_at_ms)
    .bind(session.idle_expires_at_ms)
    .bind(session.absolute_expires_at_ms)
    .bind(
        session
            .ip_prefix_key_version
            .map(database_key_version)
            .transpose()?,
    )
    .bind(session.ip_prefix_hmac.as_ref().map(|value| value.as_slice()))
    .bind(session.user_agent_hash.as_ref().map(|value| value.as_slice()))
    .bind(database_revision(session.revision)?)
    .execute(&mut *transaction)
    .await?;
    insert_security_event_postgres(&mut *transaction, event).await?;
    if let Some(account_hmac) = event.account_hmac.as_ref() {
        sqlx::query(
            "DELETE FROM login_rate_buckets WHERE scope='account' AND key_version=$1 AND bucket_hmac=$2",
        )
        .bind(database_key_version(event.digest_key_version)?)
        .bind(account_hmac.as_slice())
        .execute(&mut *transaction)
        .await?;
    }
    transaction.commit().await?;
    Ok(new_session_summary(session))
}

fn decode_sqlite_authenticated_session(
    row: sqlx::sqlite::SqliteRow,
) -> Result<AuthenticatedSession, PersistenceError> {
    let session_id = uuid::Uuid::parse_str(row.try_get("session_id")?)?;
    let user_id = uuid::Uuid::parse_str(row.try_get("user_id")?)?;
    decode_authenticated_session(
        session_id,
        user_id,
        row.try_get("username")?,
        row.try_get("role")?,
        row.try_get("principal_label")?,
        row.try_get("force_password_change")?,
        row.try_get("user_revision")?,
        row.try_get("auth_revision")?,
        row.try_get("auth_level")?,
        row.try_get("created_at_ms")?,
        row.try_get("authenticated_at_ms")?,
        row.try_get("recent_auth_at_ms")?,
        row.try_get("last_seen_at_ms")?,
        row.try_get("idle_expires_at_ms")?,
        row.try_get("absolute_expires_at_ms")?,
        row.try_get("has_ip_context")?,
        row.try_get("has_user_agent_context")?,
        row.try_get("session_revision")?,
    )
}

fn decode_postgres_authenticated_session(
    row: sqlx::postgres::PgRow,
) -> Result<AuthenticatedSession, PersistenceError> {
    decode_authenticated_session(
        row.try_get("session_id")?,
        row.try_get("user_id")?,
        row.try_get("username")?,
        row.try_get("role")?,
        row.try_get("principal_label")?,
        row.try_get("force_password_change")?,
        row.try_get("user_revision")?,
        row.try_get("auth_revision")?,
        row.try_get("auth_level")?,
        row.try_get("created_at_ms")?,
        row.try_get("authenticated_at_ms")?,
        row.try_get("recent_auth_at_ms")?,
        row.try_get("last_seen_at_ms")?,
        row.try_get("idle_expires_at_ms")?,
        row.try_get("absolute_expires_at_ms")?,
        row.try_get("has_ip_context")?,
        row.try_get("has_user_agent_context")?,
        row.try_get("session_revision")?,
    )
}

#[allow(clippy::too_many_arguments)]
fn decode_authenticated_session(
    session_id: uuid::Uuid,
    user_id: uuid::Uuid,
    username: String,
    role: String,
    principal_label: String,
    force_password_change: bool,
    user_revision: i64,
    auth_revision: i64,
    auth_level: String,
    created_at_ms: i64,
    authenticated_at_ms: i64,
    recent_auth_at_ms: i64,
    last_seen_at_ms: i64,
    idle_expires_at_ms: i64,
    absolute_expires_at_ms: i64,
    has_ip_context: bool,
    has_user_agent_context: bool,
    session_revision: i64,
) -> Result<AuthenticatedSession, PersistenceError> {
    Ok(AuthenticatedSession {
        session: AuthSessionSummary {
            id: EntityId::from_uuid(session_id),
            status: AuthSessionStatus::Active,
            auth_revision: decode_revision(auth_revision)?,
            auth_level: AuthLevel::parse(&auth_level)?,
            created_at_ms,
            authenticated_at_ms,
            recent_auth_at_ms,
            last_seen_at_ms,
            idle_expires_at_ms,
            absolute_expires_at_ms,
            has_ip_context,
            has_user_agent_context,
            revoked_at_ms: None,
            revoked_reason: None,
            revision: decode_revision(session_revision)?,
        },
        user_id: EntityId::from_uuid(user_id),
        username: Username::parse(username)?,
        role: UserRole::parse(&role)?,
        principal_label: PrincipalLabel::parse(principal_label)?,
        force_password_change,
        user_revision: decode_revision(user_revision)?,
    })
}

async fn authenticate_session_sqlite(
    pool: &SqlitePool,
    authentication: &SessionAuthentication,
    touch_session: bool,
) -> Result<SessionAuthenticationOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let csrf_key_version = authentication
        .csrf_key_version
        .map(database_key_version)
        .transpose()?;
    let csrf_hmac = authentication
        .csrf_hmac
        .as_ref()
        .map(|value| value.as_slice());
    let row = sqlx::query(
        "SELECT CASE WHEN ? IS NULL THEN 1 WHEN s.csrf_key_version=? AND s.csrf_hmac=? THEN 1 ELSE 0 END AS csrf_matches,s.id AS session_id,s.user_id AS user_id,u.username AS username,u.role AS role,u.principal_label AS principal_label,u.force_password_change AS force_password_change,u.revision AS user_revision,s.auth_revision AS auth_revision,s.auth_level AS auth_level,s.created_at_ms AS created_at_ms,s.authenticated_at_ms AS authenticated_at_ms,s.recent_auth_at_ms AS recent_auth_at_ms,s.last_seen_at_ms AS last_seen_at_ms,s.idle_expires_at_ms AS idle_expires_at_ms,s.absolute_expires_at_ms AS absolute_expires_at_ms,s.ip_prefix_hmac IS NOT NULL AS has_ip_context,s.user_agent_hash IS NOT NULL AS has_user_agent_context,s.revision AS session_revision FROM auth_sessions AS s JOIN users AS u ON u.id=s.user_id JOIN user_auth_state AS a ON a.user_id=s.user_id WHERE s.token_key_version=? AND s.token_hmac=? AND s.status='active' AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND a.auth_revision=s.auth_revision",
    )
    .bind(csrf_key_version)
    .bind(csrf_key_version)
    .bind(csrf_hmac)
    .bind(database_key_version(authentication.token_key_version)?)
    .bind(authentication.token_hmac.as_slice())
    .bind(authentication.now_ms)
    .bind(authentication.now_ms)
    .bind(authentication.now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    let Some(row) = row else {
        transaction.commit().await?;
        return Ok(SessionAuthenticationOutcome::InvalidSession);
    };
    let csrf_matches: i64 = row.try_get("csrf_matches")?;
    if csrf_matches != 1 {
        transaction.commit().await?;
        return Ok(SessionAuthenticationOutcome::InvalidCsrf);
    }
    let mut authenticated = decode_sqlite_authenticated_session(row)?;
    if touch_session {
        touch_authenticated_session_sqlite(&mut transaction, authentication, &mut authenticated)
            .await?;
    }
    transaction.commit().await?;
    Ok(SessionAuthenticationOutcome::Authenticated(authenticated))
}

async fn authenticate_session_postgres(
    pool: &PgPool,
    authentication: &SessionAuthentication,
    touch_session: bool,
) -> Result<SessionAuthenticationOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let csrf_key_version = authentication
        .csrf_key_version
        .map(database_key_version)
        .transpose()?;
    let csrf_hmac = authentication
        .csrf_hmac
        .as_ref()
        .map(|value| value.as_slice());
    let row = if touch_session {
        let candidate_user_id: Option<uuid::Uuid> = sqlx::query_scalar(
            "SELECT user_id FROM auth_sessions WHERE token_key_version=$1 AND token_hmac=$2",
        )
        .bind(database_key_version(authentication.token_key_version)?)
        .bind(authentication.token_hmac.as_slice())
        .fetch_optional(&mut *transaction)
        .await?;
        let Some(candidate_user_id) = candidate_user_id else {
            transaction.commit().await?;
            return Ok(SessionAuthenticationOutcome::InvalidSession);
        };
        let candidate_user_id = EntityId::from_uuid(candidate_user_id);
        if lock_auth_revision_barrier_postgres(&mut transaction, candidate_user_id)
            .await?
            .is_none()
            || !lock_active_user_postgres(&mut transaction, candidate_user_id, None).await?
        {
            transaction.commit().await?;
            return Ok(SessionAuthenticationOutcome::InvalidSession);
        }
        sqlx::query(
            "SELECT ($1::integer IS NULL OR (s.csrf_key_version=$1 AND s.csrf_hmac=$2)) AS csrf_matches,s.id AS session_id,s.user_id AS user_id,u.username AS username,u.role AS role,u.principal_label AS principal_label,u.force_password_change AS force_password_change,u.revision AS user_revision,s.auth_revision AS auth_revision,s.auth_level AS auth_level,s.created_at_ms AS created_at_ms,s.authenticated_at_ms AS authenticated_at_ms,s.recent_auth_at_ms AS recent_auth_at_ms,s.last_seen_at_ms AS last_seen_at_ms,s.idle_expires_at_ms AS idle_expires_at_ms,s.absolute_expires_at_ms AS absolute_expires_at_ms,s.ip_prefix_hmac IS NOT NULL AS has_ip_context,s.user_agent_hash IS NOT NULL AS has_user_agent_context,s.revision AS session_revision FROM auth_sessions AS s JOIN users AS u ON u.id=s.user_id JOIN user_auth_state AS a ON a.user_id=s.user_id WHERE s.token_key_version=$3 AND s.token_hmac=$4 AND s.user_id=$5 AND s.status='active' AND s.last_seen_at_ms<=$6 AND s.idle_expires_at_ms>$6 AND s.absolute_expires_at_ms>$6 AND u.status='active' AND u.deleted_at_ms IS NULL AND a.auth_revision=s.auth_revision FOR UPDATE OF s",
        )
        .bind(csrf_key_version)
        .bind(csrf_hmac)
        .bind(database_key_version(authentication.token_key_version)?)
        .bind(authentication.token_hmac.as_slice())
        .bind(candidate_user_id.into_uuid())
        .bind(authentication.now_ms)
        .fetch_optional(&mut *transaction)
        .await?
    } else {
        sqlx::query(
            "SELECT ($1::integer IS NULL OR (s.csrf_key_version=$1 AND s.csrf_hmac=$2)) AS csrf_matches,s.id AS session_id,s.user_id AS user_id,u.username AS username,u.role AS role,u.principal_label AS principal_label,u.force_password_change AS force_password_change,u.revision AS user_revision,s.auth_revision AS auth_revision,s.auth_level AS auth_level,s.created_at_ms AS created_at_ms,s.authenticated_at_ms AS authenticated_at_ms,s.recent_auth_at_ms AS recent_auth_at_ms,s.last_seen_at_ms AS last_seen_at_ms,s.idle_expires_at_ms AS idle_expires_at_ms,s.absolute_expires_at_ms AS absolute_expires_at_ms,s.ip_prefix_hmac IS NOT NULL AS has_ip_context,s.user_agent_hash IS NOT NULL AS has_user_agent_context,s.revision AS session_revision FROM auth_sessions AS s JOIN users AS u ON u.id=s.user_id JOIN user_auth_state AS a ON a.user_id=s.user_id WHERE s.token_key_version=$3 AND s.token_hmac=$4 AND s.status='active' AND s.last_seen_at_ms<=$5 AND s.idle_expires_at_ms>$5 AND s.absolute_expires_at_ms>$5 AND u.status='active' AND u.deleted_at_ms IS NULL AND a.auth_revision=s.auth_revision",
        )
        .bind(csrf_key_version)
        .bind(csrf_hmac)
        .bind(database_key_version(authentication.token_key_version)?)
        .bind(authentication.token_hmac.as_slice())
        .bind(authentication.now_ms)
        .fetch_optional(&mut *transaction)
        .await?
    };
    let Some(row) = row else {
        transaction.commit().await?;
        return Ok(SessionAuthenticationOutcome::InvalidSession);
    };
    let csrf_matches: bool = row.try_get("csrf_matches")?;
    if !csrf_matches {
        transaction.commit().await?;
        return Ok(SessionAuthenticationOutcome::InvalidCsrf);
    }
    let mut authenticated = decode_postgres_authenticated_session(row)?;
    if touch_session {
        touch_authenticated_session_postgres(&mut transaction, authentication, &mut authenticated)
            .await?;
    }
    transaction.commit().await?;
    Ok(SessionAuthenticationOutcome::Authenticated(authenticated))
}

async fn touch_authenticated_session_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    authentication: &SessionAuthentication,
    authenticated: &mut AuthenticatedSession,
) -> Result<(), PersistenceError> {
    if authentication.now_ms < authenticated.session.last_seen_at_ms
        || authentication.now_ms - authenticated.session.last_seen_at_ms
            < authentication.touch_interval_ms
    {
        return Ok(());
    }
    let idle_expires_at_ms = authentication
        .now_ms
        .checked_add(authentication.idle_timeout_ms)
        .ok_or(PersistenceError::InvalidSessionTimingPolicy)?
        .min(authenticated.session.absolute_expires_at_ms);
    let next_revision = authenticated
        .session
        .revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let affected = sqlx::query(
        "UPDATE auth_sessions SET last_seen_at_ms=?,idle_expires_at_ms=?,revision=? WHERE id=? AND status='active' AND revision=?",
    )
    .bind(authentication.now_ms)
    .bind(idle_expires_at_ms)
    .bind(database_revision(next_revision)?)
    .bind(authenticated.session.id.to_string())
    .bind(database_revision(authenticated.session.revision)?)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    if affected != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    authenticated.session.last_seen_at_ms = authentication.now_ms;
    authenticated.session.idle_expires_at_ms = idle_expires_at_ms;
    authenticated.session.revision = next_revision;
    Ok(())
}

async fn touch_authenticated_session_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    authentication: &SessionAuthentication,
    authenticated: &mut AuthenticatedSession,
) -> Result<(), PersistenceError> {
    if authentication.now_ms < authenticated.session.last_seen_at_ms
        || authentication.now_ms - authenticated.session.last_seen_at_ms
            < authentication.touch_interval_ms
    {
        return Ok(());
    }
    let idle_expires_at_ms = authentication
        .now_ms
        .checked_add(authentication.idle_timeout_ms)
        .ok_or(PersistenceError::InvalidSessionTimingPolicy)?
        .min(authenticated.session.absolute_expires_at_ms);
    let next_revision = authenticated
        .session
        .revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let affected = sqlx::query(
        "UPDATE auth_sessions SET last_seen_at_ms=$1,idle_expires_at_ms=$2,revision=$3 WHERE id=$4 AND status='active' AND revision=$5",
    )
    .bind(authentication.now_ms)
    .bind(idle_expires_at_ms)
    .bind(database_revision(next_revision)?)
    .bind(authenticated.session.id.into_uuid())
    .bind(database_revision(authenticated.session.revision)?)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    if affected != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    authenticated.session.last_seen_at_ms = authentication.now_ms;
    authenticated.session.idle_expires_at_ms = idle_expires_at_ms;
    authenticated.session.revision = next_revision;
    Ok(())
}

fn decode_sqlite_session_summary(
    row: sqlx::sqlite::SqliteRow,
) -> Result<AuthSessionSummary, PersistenceError> {
    let id = uuid::Uuid::parse_str(row.try_get("id")?)?;
    decode_session_summary(
        id,
        row.try_get("status")?,
        row.try_get("auth_revision")?,
        row.try_get("auth_level")?,
        row.try_get("created_at_ms")?,
        row.try_get("authenticated_at_ms")?,
        row.try_get("recent_auth_at_ms")?,
        row.try_get("last_seen_at_ms")?,
        row.try_get("idle_expires_at_ms")?,
        row.try_get("absolute_expires_at_ms")?,
        row.try_get("has_ip_context")?,
        row.try_get("has_user_agent_context")?,
        row.try_get("revoked_at_ms")?,
        row.try_get("revoked_reason")?,
        row.try_get("revision")?,
    )
}

fn decode_postgres_session_summary(
    row: sqlx::postgres::PgRow,
) -> Result<AuthSessionSummary, PersistenceError> {
    decode_session_summary(
        row.try_get("id")?,
        row.try_get("status")?,
        row.try_get("auth_revision")?,
        row.try_get("auth_level")?,
        row.try_get("created_at_ms")?,
        row.try_get("authenticated_at_ms")?,
        row.try_get("recent_auth_at_ms")?,
        row.try_get("last_seen_at_ms")?,
        row.try_get("idle_expires_at_ms")?,
        row.try_get("absolute_expires_at_ms")?,
        row.try_get("has_ip_context")?,
        row.try_get("has_user_agent_context")?,
        row.try_get("revoked_at_ms")?,
        row.try_get("revoked_reason")?,
        row.try_get("revision")?,
    )
}

#[allow(clippy::too_many_arguments)]
fn decode_session_summary(
    id: uuid::Uuid,
    status: String,
    auth_revision: i64,
    auth_level: String,
    created_at_ms: i64,
    authenticated_at_ms: i64,
    recent_auth_at_ms: i64,
    last_seen_at_ms: i64,
    idle_expires_at_ms: i64,
    absolute_expires_at_ms: i64,
    has_ip_context: bool,
    has_user_agent_context: bool,
    revoked_at_ms: Option<i64>,
    revoked_reason: Option<String>,
    revision: i64,
) -> Result<AuthSessionSummary, PersistenceError> {
    let revoked_reason = revoked_reason
        .as_deref()
        .map(SessionRevocationReason::parse)
        .transpose()?;
    Ok(AuthSessionSummary {
        id: EntityId::from_uuid(id),
        status: AuthSessionStatus::parse(&status)?,
        auth_revision: decode_revision(auth_revision)?,
        auth_level: AuthLevel::parse(&auth_level)?,
        created_at_ms,
        authenticated_at_ms,
        recent_auth_at_ms,
        last_seen_at_ms,
        idle_expires_at_ms,
        absolute_expires_at_ms,
        has_ip_context,
        has_user_agent_context,
        revoked_at_ms,
        revoked_reason,
        revision: decode_revision(revision)?,
    })
}

async fn rotate_current_session_sqlite(
    pool: &SqlitePool,
    user_id: EntityId,
    current_session_id: EntityId,
    expected_user_revision: Revision,
    replacement: &NewAuthSession,
    event: &NewLoginSecurityEvent,
    now_ms: i64,
) -> Result<AuthSessionSummary, PersistenceError> {
    let mut transaction = pool.begin().await?;
    sqlx::query("UPDATE auth_sessions SET revision=revision WHERE id=? AND user_id=?")
        .bind(current_session_id.to_string())
        .bind(user_id.to_string())
        .execute(&mut *transaction)
        .await?;
    let current: Option<(i64, String, i64, i64)> = sqlx::query_as(
        "SELECT s.auth_revision,s.auth_level,s.absolute_expires_at_ms,s.revision FROM auth_sessions AS s JOIN users AS u ON u.id=s.user_id JOIN user_auth_state AS a ON a.user_id=s.user_id WHERE s.id=? AND s.user_id=? AND s.status='active' AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND u.revision=? AND a.auth_revision=s.auth_revision",
    )
    .bind(current_session_id.to_string())
    .bind(user_id.to_string())
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(database_revision(expected_user_revision)?)
    .fetch_optional(&mut *transaction)
    .await?;
    let (auth_revision, auth_level, absolute_expires_at_ms, session_revision) =
        current.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    validate_same_revision_rotation_replacement(
        replacement,
        decode_revision(auth_revision)?,
        AuthLevel::parse(&auth_level)?,
        now_ms,
        now_ms,
        absolute_expires_at_ms,
        now_ms,
    )?;
    let next_session_revision = decode_revision(session_revision)?
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let revoked = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=MAX(created_at_ms,?),revoked_reason='rotation',revision=? WHERE id=? AND user_id=? AND status='active' AND revision=?",
    )
    .bind(now_ms)
    .bind(database_revision(next_session_revision)?)
    .bind(current_session_id.to_string())
    .bind(user_id.to_string())
    .bind(session_revision)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if revoked != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    insert_rotated_session_sqlite(&mut transaction, replacement).await?;
    insert_security_event_sqlite(&mut *transaction, event).await?;
    if let Some(account_hmac) = event.account_hmac.as_ref() {
        sqlx::query(
            "DELETE FROM login_rate_buckets WHERE scope='account' AND key_version=? AND bucket_hmac=?",
        )
        .bind(database_key_version(event.digest_key_version)?)
        .bind(account_hmac.as_slice())
        .execute(&mut *transaction)
        .await?;
    }
    transaction.commit().await?;
    Ok(new_session_summary(replacement))
}

async fn rotate_current_session_postgres(
    pool: &PgPool,
    user_id: EntityId,
    current_session_id: EntityId,
    expected_user_revision: Revision,
    replacement: &NewAuthSession,
    event: &NewLoginSecurityEvent,
    now_ms: i64,
) -> Result<AuthSessionSummary, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let auth_revision = lock_auth_revision_barrier_postgres(&mut transaction, user_id)
        .await?
        .ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    if !lock_active_user_postgres(&mut transaction, user_id, Some(expected_user_revision)).await? {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let current: Option<(i64, String, i64, i64)> = sqlx::query_as(
        "SELECT auth_revision,auth_level,absolute_expires_at_ms,revision FROM auth_sessions WHERE id=$1 AND user_id=$2 AND status='active' AND last_seen_at_ms<=$3 AND idle_expires_at_ms>$3 AND absolute_expires_at_ms>$3 AND auth_revision=$4 FOR UPDATE",
    )
    .bind(current_session_id.into_uuid())
    .bind(user_id.into_uuid())
    .bind(now_ms)
    .bind(auth_revision)
    .fetch_optional(&mut *transaction)
    .await?;
    let (auth_revision, auth_level, absolute_expires_at_ms, session_revision) =
        current.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    validate_same_revision_rotation_replacement(
        replacement,
        decode_revision(auth_revision)?,
        AuthLevel::parse(&auth_level)?,
        now_ms,
        now_ms,
        absolute_expires_at_ms,
        now_ms,
    )?;
    let next_session_revision = decode_revision(session_revision)?
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let revoked = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=GREATEST(created_at_ms,$1),revoked_reason='rotation',revision=$2 WHERE id=$3 AND user_id=$4 AND status='active' AND revision=$5",
    )
    .bind(now_ms)
    .bind(database_revision(next_session_revision)?)
    .bind(current_session_id.into_uuid())
    .bind(user_id.into_uuid())
    .bind(session_revision)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if revoked != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    insert_rotated_session_postgres(&mut transaction, replacement).await?;
    insert_security_event_postgres(&mut *transaction, event).await?;
    if let Some(account_hmac) = event.account_hmac.as_ref() {
        sqlx::query(
            "DELETE FROM login_rate_buckets WHERE scope='account' AND key_version=$1 AND bucket_hmac=$2",
        )
        .bind(database_key_version(event.digest_key_version)?)
        .bind(account_hmac.as_slice())
        .execute(&mut *transaction)
        .await?;
    }
    transaction.commit().await?;
    Ok(new_session_summary(replacement))
}

async fn change_password_and_rotate_sqlite(
    pool: &SqlitePool,
    rotation: &PasswordChangeRotation<'_>,
) -> Result<PasswordChangeResult, PersistenceError> {
    let PasswordChangeRotation {
        user_id,
        current_session_id,
        expected_user_revision,
        new_hash,
        replacement,
        event,
        now_ms,
    } = *rotation;
    let mut transaction = pool.begin().await?;
    sqlx::query("UPDATE auth_sessions SET revision=revision WHERE id=? AND user_id=?")
        .bind(current_session_id.to_string())
        .bind(user_id.to_string())
        .execute(&mut *transaction)
        .await?;
    let current: Option<(i64, String, i64, i64, i64)> = sqlx::query_as(
        "SELECT s.auth_revision,s.auth_level,s.authenticated_at_ms,s.recent_auth_at_ms,s.absolute_expires_at_ms FROM auth_sessions AS s JOIN users AS u ON u.id=s.user_id JOIN user_auth_state AS a ON a.user_id=s.user_id WHERE s.id=? AND s.user_id=? AND s.status='active' AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND u.revision=? AND a.auth_revision=s.auth_revision",
    )
    .bind(current_session_id.to_string())
    .bind(user_id.to_string())
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(database_revision(expected_user_revision)?)
    .fetch_optional(&mut *transaction)
    .await?;
    let (
        current_auth_revision,
        auth_level,
        authenticated_at_ms,
        recent_auth_at_ms,
        absolute_expires_at_ms,
    ) = current.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    let next_auth_revision = validate_password_rotation_replacement(
        replacement,
        decode_revision(current_auth_revision)?,
        AuthLevel::parse(&auth_level)?,
        authenticated_at_ms,
        recent_auth_at_ms,
        absolute_expires_at_ms,
        now_ms,
    )?;
    let next_user_revision = expected_user_revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let active_session_revisions: Vec<i64> = sqlx::query_scalar(
        "SELECT revision FROM auth_sessions WHERE user_id=? AND status='active'",
    )
    .bind(user_id.to_string())
    .fetch_all(&mut *transaction)
    .await?;
    for revision in active_session_revisions {
        let next = decode_revision(revision)?
            .next()
            .map_err(|_| PersistenceError::RevisionOutOfRange)?;
        database_revision(next)?;
    }
    let user_updated = sqlx::query(
        "UPDATE users SET password_hash=?,force_password_change=0,revision=? WHERE id=? AND revision=? AND status='active' AND deleted_at_ms IS NULL",
    )
    .bind(new_hash.as_str())
    .bind(database_revision(next_user_revision)?)
    .bind(user_id.to_string())
    .bind(database_revision(expected_user_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if user_updated != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    let auth_updated = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=?,password_changed_at_ms=MAX(password_changed_at_ms,?),updated_at_ms=MAX(updated_at_ms,?) WHERE user_id=? AND auth_revision=?",
    )
    .bind(database_revision(next_auth_revision)?)
    .bind(now_ms)
    .bind(now_ms)
    .bind(user_id.to_string())
    .bind(current_auth_revision)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if auth_updated != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    let revoked_sessions = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=MAX(created_at_ms,?),revoked_reason='password_changed',revision=revision+1 WHERE user_id=? AND status='active'",
    )
    .bind(now_ms)
    .bind(user_id.to_string())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    insert_rotated_session_sqlite(&mut transaction, replacement).await?;
    insert_security_event_sqlite(&mut *transaction, event).await?;
    if let Some(account_hmac) = event.account_hmac.as_ref() {
        sqlx::query(
            "DELETE FROM login_rate_buckets WHERE scope='account' AND key_version=? AND bucket_hmac=?",
        )
        .bind(database_key_version(event.digest_key_version)?)
        .bind(account_hmac.as_slice())
        .execute(&mut *transaction)
        .await?;
    }
    transaction.commit().await?;
    Ok(PasswordChangeResult {
        session: new_session_summary(replacement),
        revoked_sessions,
        auth_revision: next_auth_revision,
    })
}

async fn change_password_and_rotate_postgres(
    pool: &PgPool,
    rotation: &PasswordChangeRotation<'_>,
) -> Result<PasswordChangeResult, PersistenceError> {
    let PasswordChangeRotation {
        user_id,
        current_session_id,
        expected_user_revision,
        new_hash,
        replacement,
        event,
        now_ms,
    } = *rotation;
    let mut transaction = pool.begin().await?;
    let auth_revision = lock_auth_revision_barrier_postgres(&mut transaction, user_id)
        .await?
        .ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    if !lock_active_user_postgres(&mut transaction, user_id, Some(expected_user_revision)).await? {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let current: Option<(i64, String, i64, i64, i64)> = sqlx::query_as(
        "SELECT auth_revision,auth_level,authenticated_at_ms,recent_auth_at_ms,absolute_expires_at_ms FROM auth_sessions WHERE id=$1 AND user_id=$2 AND status='active' AND last_seen_at_ms<=$3 AND idle_expires_at_ms>$3 AND absolute_expires_at_ms>$3 AND auth_revision=$4 FOR UPDATE",
    )
    .bind(current_session_id.into_uuid())
    .bind(user_id.into_uuid())
    .bind(now_ms)
    .bind(auth_revision)
    .fetch_optional(&mut *transaction)
    .await?;
    let (
        current_auth_revision,
        auth_level,
        authenticated_at_ms,
        recent_auth_at_ms,
        absolute_expires_at_ms,
    ) = current.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    let next_auth_revision = validate_password_rotation_replacement(
        replacement,
        decode_revision(current_auth_revision)?,
        AuthLevel::parse(&auth_level)?,
        authenticated_at_ms,
        recent_auth_at_ms,
        absolute_expires_at_ms,
        now_ms,
    )?;
    let next_user_revision = expected_user_revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let active_session_revisions: Vec<i64> = sqlx::query_scalar(
        "SELECT revision FROM auth_sessions WHERE user_id=$1 AND status='active' ORDER BY id FOR UPDATE",
    )
    .bind(user_id.into_uuid())
    .fetch_all(&mut *transaction)
    .await?;
    for revision in active_session_revisions {
        let next = decode_revision(revision)?
            .next()
            .map_err(|_| PersistenceError::RevisionOutOfRange)?;
        database_revision(next)?;
    }
    let user_updated = sqlx::query(
        "UPDATE users SET password_hash=$1,force_password_change=false,revision=$2 WHERE id=$3 AND revision=$4 AND status='active' AND deleted_at_ms IS NULL",
    )
    .bind(new_hash.as_str())
    .bind(database_revision(next_user_revision)?)
    .bind(user_id.into_uuid())
    .bind(database_revision(expected_user_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if user_updated != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    let auth_updated = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=$1,password_changed_at_ms=GREATEST(password_changed_at_ms,$2),updated_at_ms=GREATEST(updated_at_ms,$2) WHERE user_id=$3 AND auth_revision=$4",
    )
    .bind(database_revision(next_auth_revision)?)
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .bind(current_auth_revision)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if auth_updated != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    let revoked_sessions = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=GREATEST(created_at_ms,$1),revoked_reason='password_changed',revision=revision+1 WHERE user_id=$2 AND status='active'",
    )
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    insert_rotated_session_postgres(&mut transaction, replacement).await?;
    insert_security_event_postgres(&mut *transaction, event).await?;
    if let Some(account_hmac) = event.account_hmac.as_ref() {
        sqlx::query(
            "DELETE FROM login_rate_buckets WHERE scope='account' AND key_version=$1 AND bucket_hmac=$2",
        )
        .bind(database_key_version(event.digest_key_version)?)
        .bind(account_hmac.as_slice())
        .execute(&mut *transaction)
        .await?;
    }
    transaction.commit().await?;
    Ok(PasswordChangeResult {
        session: new_session_summary(replacement),
        revoked_sessions,
        auth_revision: next_auth_revision,
    })
}

#[cfg(test)]
async fn revoke_all_sessions_sqlite(
    pool: &SqlitePool,
    user_id: EntityId,
    now_ms: i64,
    reason: SessionRevocationReason,
) -> Result<LogoutAllResult, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let locked =
        sqlx::query("UPDATE user_auth_state SET auth_revision=auth_revision WHERE user_id=?")
            .bind(user_id.to_string())
            .execute(&mut *transaction)
            .await?
            .rows_affected();
    if locked != 1 {
        return Err(PersistenceError::AuthStateUnavailable);
    }
    let current: i64 =
        sqlx::query_scalar("SELECT auth_revision FROM user_auth_state WHERE user_id=?")
            .bind(user_id.to_string())
            .fetch_one(&mut *transaction)
            .await?;
    let next = decode_revision(current)?
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    sqlx::query(
        "UPDATE user_auth_state SET auth_revision=?,updated_at_ms=MAX(updated_at_ms,?) WHERE user_id=?",
    )
    .bind(database_revision(next)?)
    .bind(now_ms)
    .bind(user_id.to_string())
    .execute(&mut *transaction)
    .await?;
    let revoked_sessions = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=MAX(created_at_ms,?),revoked_reason=?,revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE user_id=? AND status='active'",
    )
    .bind(now_ms)
    .bind(reason.as_str())
    .bind(user_id.to_string())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    transaction.commit().await?;
    Ok(LogoutAllResult {
        revoked_sessions,
        auth_revision: next,
        kept_current: false,
    })
}

#[cfg(test)]
async fn revoke_all_sessions_postgres(
    pool: &PgPool,
    user_id: EntityId,
    now_ms: i64,
    reason: SessionRevocationReason,
) -> Result<LogoutAllResult, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let current: Option<i64> =
        sqlx::query_scalar("SELECT auth_revision FROM user_auth_state WHERE user_id=$1 FOR UPDATE")
            .bind(user_id.into_uuid())
            .fetch_optional(&mut *transaction)
            .await?;
    let current = current.ok_or(PersistenceError::AuthStateUnavailable)?;
    let next = decode_revision(current)?
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    sqlx::query(
        "UPDATE user_auth_state SET auth_revision=$1,updated_at_ms=GREATEST(updated_at_ms,$2) WHERE user_id=$3",
    )
    .bind(database_revision(next)?)
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .execute(&mut *transaction)
    .await?;
    let revoked_sessions = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=GREATEST(created_at_ms,$1),revoked_reason=$2,revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE user_id=$3 AND status='active'",
    )
    .bind(now_ms)
    .bind(reason.as_str())
    .bind(user_id.into_uuid())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    transaction.commit().await?;
    Ok(LogoutAllResult {
        revoked_sessions,
        auth_revision: next,
        kept_current: false,
    })
}

async fn logout_all_sessions_with_event_sqlite(
    pool: &SqlitePool,
    user_id: EntityId,
    current_session_id: EntityId,
    expected_recent_auth_at_ms: i64,
    event: &NewLoginSecurityEvent,
    now_ms: i64,
) -> Result<LogoutAllResult, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let locked =
        sqlx::query("UPDATE user_auth_state SET auth_revision=auth_revision WHERE user_id=?")
            .bind(user_id.to_string())
            .execute(&mut *transaction)
            .await?
            .rows_affected();
    if locked != 1 {
        return Err(PersistenceError::AuthStateUnavailable);
    }
    let current: Option<i64> = sqlx::query_scalar(
        "SELECT a.auth_revision FROM auth_sessions AS s JOIN users AS u ON u.id=s.user_id JOIN user_auth_state AS a ON a.user_id=s.user_id WHERE s.id=? AND s.user_id=? AND s.status='active' AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND a.auth_revision=s.auth_revision AND s.recent_auth_at_ms=?",
    )
    .bind(current_session_id.to_string())
    .bind(user_id.to_string())
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(expected_recent_auth_at_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    let current = current.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    let next = decode_revision(current)?
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let active_session_revisions: Vec<i64> = sqlx::query_scalar(
        "SELECT revision FROM auth_sessions WHERE user_id=? AND status='active'",
    )
    .bind(user_id.to_string())
    .fetch_all(&mut *transaction)
    .await?;
    for revision in active_session_revisions {
        let next_session_revision = decode_revision(revision)?
            .next()
            .map_err(|_| PersistenceError::RevisionOutOfRange)?;
        database_revision(next_session_revision)?;
    }
    let updated = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=?,updated_at_ms=MAX(updated_at_ms,?) WHERE user_id=? AND auth_revision=?",
    )
    .bind(database_revision(next)?)
    .bind(now_ms)
    .bind(user_id.to_string())
    .bind(current)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if updated != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    let revoked_sessions = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=MAX(created_at_ms,?),revoked_reason='logout_all',revision=revision+1 WHERE user_id=? AND status='active'",
    )
    .bind(now_ms)
    .bind(user_id.to_string())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    insert_security_event_sqlite(&mut *transaction, event).await?;
    transaction.commit().await?;
    Ok(LogoutAllResult {
        revoked_sessions,
        auth_revision: next,
        kept_current: false,
    })
}

async fn logout_all_sessions_with_event_postgres(
    pool: &PgPool,
    user_id: EntityId,
    current_session_id: EntityId,
    expected_recent_auth_at_ms: i64,
    event: &NewLoginSecurityEvent,
    now_ms: i64,
) -> Result<LogoutAllResult, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let auth_revision = lock_auth_revision_barrier_postgres(&mut transaction, user_id)
        .await?
        .ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    if !lock_active_user_postgres(&mut transaction, user_id, None).await? {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let current: Option<i64> = sqlx::query_scalar(
        "SELECT auth_revision FROM auth_sessions WHERE id=$1 AND user_id=$2 AND status='active' AND last_seen_at_ms<=$3 AND idle_expires_at_ms>$3 AND absolute_expires_at_ms>$3 AND auth_revision=$4 AND recent_auth_at_ms=$5 FOR UPDATE",
    )
    .bind(current_session_id.into_uuid())
    .bind(user_id.into_uuid())
    .bind(now_ms)
    .bind(auth_revision)
    .bind(expected_recent_auth_at_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    let current = current.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    let next = decode_revision(current)?
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let active_session_revisions: Vec<i64> = sqlx::query_scalar(
        "SELECT revision FROM auth_sessions WHERE user_id=$1 AND status='active' ORDER BY id FOR UPDATE",
    )
    .bind(user_id.into_uuid())
    .fetch_all(&mut *transaction)
    .await?;
    for revision in active_session_revisions {
        let next_session_revision = decode_revision(revision)?
            .next()
            .map_err(|_| PersistenceError::RevisionOutOfRange)?;
        database_revision(next_session_revision)?;
    }
    let updated = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=$1,updated_at_ms=GREATEST(updated_at_ms,$2) WHERE user_id=$3 AND auth_revision=$4",
    )
    .bind(database_revision(next)?)
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .bind(current)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if updated != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    let revoked_sessions = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=GREATEST(created_at_ms,$1),revoked_reason='logout_all',revision=revision+1 WHERE user_id=$2 AND status='active'",
    )
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    insert_security_event_postgres(&mut *transaction, event).await?;
    transaction.commit().await?;
    Ok(LogoutAllResult {
        revoked_sessions,
        auth_revision: next,
        kept_current: false,
    })
}

#[allow(clippy::too_many_arguments)]
fn validate_rotation_replacement(
    replacement: &NewAuthSession,
    current_auth_revision: Revision,
    current_auth_level: AuthLevel,
    current_authenticated_at_ms: i64,
    current_recent_auth_at_ms: i64,
    current_absolute_expires_at_ms: i64,
    now_ms: i64,
) -> Result<Revision, PersistenceError> {
    let next = current_auth_revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    if replacement.auth_revision != next
        || replacement.auth_level != current_auth_level
        || replacement.created_at_ms != now_ms
        || replacement.authenticated_at_ms != current_authenticated_at_ms
        || replacement.recent_auth_at_ms != current_recent_auth_at_ms
        || replacement.last_seen_at_ms != now_ms
        || replacement.absolute_expires_at_ms > current_absolute_expires_at_ms
        || replacement.revision != Revision::initial()
    {
        return Err(PersistenceError::InvalidSessionRotation);
    }
    Ok(next)
}

async fn rotate_logout_all_sqlite(
    pool: &SqlitePool,
    user_id: EntityId,
    current_session_id: EntityId,
    expected_user_revision: Revision,
    replacement: &NewAuthSession,
    event: &NewLoginSecurityEvent,
    now_ms: i64,
) -> Result<LogoutAllResult, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let locked =
        sqlx::query("UPDATE user_auth_state SET auth_revision=auth_revision WHERE user_id=?")
            .bind(user_id.to_string())
            .execute(&mut *transaction)
            .await?
            .rows_affected();
    if locked != 1 {
        return Err(PersistenceError::AuthStateUnavailable);
    }
    let current: Option<(i64, String, i64, i64, i64)> = sqlx::query_as(
        "SELECT s.auth_revision,s.auth_level,s.authenticated_at_ms,s.recent_auth_at_ms,s.absolute_expires_at_ms FROM auth_sessions AS s JOIN users AS u ON u.id=s.user_id JOIN user_auth_state AS a ON a.user_id=s.user_id WHERE s.id=? AND s.user_id=? AND s.status='active' AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND u.revision=? AND a.auth_revision=s.auth_revision",
    )
    .bind(current_session_id.to_string())
    .bind(user_id.to_string())
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(database_revision(expected_user_revision)?)
    .fetch_optional(&mut *transaction)
    .await?;
    let (
        current_auth_revision,
        current_auth_level,
        authenticated_at_ms,
        recent_auth_at_ms,
        absolute_expires_at_ms,
    ) = current.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    let next = validate_rotation_replacement(
        replacement,
        decode_revision(current_auth_revision)?,
        AuthLevel::parse(&current_auth_level)?,
        authenticated_at_ms,
        recent_auth_at_ms,
        absolute_expires_at_ms,
        now_ms,
    )?;
    let updated = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=?,updated_at_ms=MAX(updated_at_ms,?) WHERE user_id=? AND auth_revision=?",
    )
    .bind(database_revision(next)?)
    .bind(now_ms)
    .bind(user_id.to_string())
    .bind(current_auth_revision)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if updated != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    let revoked_sessions = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=MAX(created_at_ms,?),revoked_reason='logout_all',revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE user_id=? AND status='active'",
    )
    .bind(now_ms)
    .bind(user_id.to_string())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    insert_rotated_session_sqlite(&mut transaction, replacement).await?;
    insert_security_event_sqlite(&mut *transaction, event).await?;
    transaction.commit().await?;
    Ok(LogoutAllResult {
        revoked_sessions,
        auth_revision: next,
        kept_current: true,
    })
}

async fn rotate_logout_all_postgres(
    pool: &PgPool,
    user_id: EntityId,
    current_session_id: EntityId,
    expected_user_revision: Revision,
    replacement: &NewAuthSession,
    event: &NewLoginSecurityEvent,
    now_ms: i64,
) -> Result<LogoutAllResult, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let auth_revision = lock_auth_revision_barrier_postgres(&mut transaction, user_id)
        .await?
        .ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    if !lock_active_user_postgres(&mut transaction, user_id, Some(expected_user_revision)).await? {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let current: Option<(i64, String, i64, i64, i64)> = sqlx::query_as(
        "SELECT auth_revision,auth_level,authenticated_at_ms,recent_auth_at_ms,absolute_expires_at_ms FROM auth_sessions WHERE id=$1 AND user_id=$2 AND status='active' AND last_seen_at_ms<=$3 AND idle_expires_at_ms>$3 AND absolute_expires_at_ms>$3 AND auth_revision=$4 FOR UPDATE",
    )
    .bind(current_session_id.into_uuid())
    .bind(user_id.into_uuid())
    .bind(now_ms)
    .bind(auth_revision)
    .fetch_optional(&mut *transaction)
    .await?;
    let (
        current_auth_revision,
        current_auth_level,
        authenticated_at_ms,
        recent_auth_at_ms,
        absolute_expires_at_ms,
    ) = current.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    let next = validate_rotation_replacement(
        replacement,
        decode_revision(current_auth_revision)?,
        AuthLevel::parse(&current_auth_level)?,
        authenticated_at_ms,
        recent_auth_at_ms,
        absolute_expires_at_ms,
        now_ms,
    )?;
    let updated = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=$1,updated_at_ms=GREATEST(updated_at_ms,$2) WHERE user_id=$3 AND auth_revision=$4",
    )
    .bind(database_revision(next)?)
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .bind(current_auth_revision)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if updated != 1 {
        return Err(PersistenceError::SessionRevisionConflict);
    }
    let revoked_sessions = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=GREATEST(created_at_ms,$1),revoked_reason='logout_all',revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE user_id=$2 AND status='active'",
    )
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    insert_rotated_session_postgres(&mut transaction, replacement).await?;
    insert_security_event_postgres(&mut *transaction, event).await?;
    transaction.commit().await?;
    Ok(LogoutAllResult {
        revoked_sessions,
        auth_revision: next,
        kept_current: true,
    })
}

async fn insert_rotated_session_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    session: &NewAuthSession,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES (?,?,?,?,?,?,?,?,'active',?,?,?,?,?,?,?,?,?,NULL,NULL,?)",
    )
    .bind(session.id.to_string())
    .bind(session.user_id.to_string())
    .bind(database_key_version(session.token_key_version)?)
    .bind(session.token_hmac.as_slice())
    .bind(database_key_version(session.csrf_key_version)?)
    .bind(session.csrf_hmac.as_slice())
    .bind(database_revision(session.auth_revision)?)
    .bind(session.auth_level.as_str())
    .bind(session.created_at_ms)
    .bind(session.authenticated_at_ms)
    .bind(session.recent_auth_at_ms)
    .bind(session.last_seen_at_ms)
    .bind(session.idle_expires_at_ms)
    .bind(session.absolute_expires_at_ms)
    .bind(
        session
            .ip_prefix_key_version
            .map(database_key_version)
            .transpose()?,
    )
    .bind(session.ip_prefix_hmac.as_ref().map(|value| value.as_slice()))
    .bind(session.user_agent_hash.as_ref().map(|value| value.as_slice()))
    .bind(database_revision(session.revision)?)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn insert_rotated_session_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    session: &NewAuthSession,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,'active',$9,$10,$11,$12,$13,$14,$15,$16,$17,NULL,NULL,$18)",
    )
    .bind(session.id.into_uuid())
    .bind(session.user_id.into_uuid())
    .bind(database_key_version(session.token_key_version)?)
    .bind(session.token_hmac.as_slice())
    .bind(database_key_version(session.csrf_key_version)?)
    .bind(session.csrf_hmac.as_slice())
    .bind(database_revision(session.auth_revision)?)
    .bind(session.auth_level.as_str())
    .bind(session.created_at_ms)
    .bind(session.authenticated_at_ms)
    .bind(session.recent_auth_at_ms)
    .bind(session.last_seen_at_ms)
    .bind(session.idle_expires_at_ms)
    .bind(session.absolute_expires_at_ms)
    .bind(
        session
            .ip_prefix_key_version
            .map(database_key_version)
            .transpose()?,
    )
    .bind(session.ip_prefix_hmac.as_ref().map(|value| value.as_slice()))
    .bind(session.user_agent_hash.as_ref().map(|value| value.as_slice()))
    .bind(database_revision(session.revision)?)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

#[derive(Clone, Copy)]
struct RateBucketOutcome {
    remaining_attempts: u32,
    reset_at_ms: i64,
    blocked_until_ms: Option<i64>,
    entered_blocked: bool,
}

fn combine_rate_bucket_outcomes(outcomes: &[RateBucketOutcome], now_ms: i64) -> LoginRateDecision {
    let remaining_attempts = outcomes
        .iter()
        .map(|outcome| outcome.remaining_attempts)
        .min()
        .unwrap_or(0);
    let reset_at_ms = outcomes
        .iter()
        .map(|outcome| outcome.reset_at_ms)
        .max()
        .unwrap_or(now_ms);
    let blocked_until_ms = outcomes
        .iter()
        .filter_map(|outcome| outcome.blocked_until_ms)
        .filter(|blocked_until_ms| *blocked_until_ms > now_ms)
        .max();
    match blocked_until_ms {
        Some(blocked_until_ms) => LoginRateDecision::Limited {
            retry_after_ms: blocked_until_ms - now_ms,
            blocked_until_ms,
        },
        None => LoginRateDecision::Allowed {
            remaining_attempts,
            reset_at_ms,
        },
    }
}

fn rate_bucket_is_limited(outcome: RateBucketOutcome, now_ms: i64) -> bool {
    outcome
        .blocked_until_ms
        .is_some_and(|blocked_until_ms| blocked_until_ms > now_ms)
}

async fn reserve_login_attempt_sqlite(
    pool: &SqlitePool,
    reservation: &LoginAttemptReservation,
    event: &NewLoginSecurityEvent,
) -> Result<LoginRateDecision, PersistenceError> {
    if let Some(decision) = preflight_existing_rate_limit_sqlite(pool, reservation).await? {
        return Ok(decision);
    }
    let mut transaction = pool.begin().await?;
    let account = reserve_rate_bucket_sqlite(
        &mut transaction,
        "account",
        &reservation.account_hmac,
        reservation.account_max_attempts,
        reservation,
    )
    .await?;
    if rate_bucket_is_limited(account, reservation.now_ms) {
        let decision = combine_rate_bucket_outcomes(&[account], reservation.now_ms);
        if account.entered_blocked {
            if let Err(error) = insert_security_event_sqlite(&mut *transaction, event).await {
                transaction.rollback().await?;
                return Err(error);
            }
            transaction.commit().await?;
        } else {
            transaction.rollback().await?;
        }
        return Ok(decision);
    }
    let ip = reserve_rate_bucket_sqlite(
        &mut transaction,
        "ip",
        &reservation.ip_prefix_hmac,
        reservation.ip_max_attempts,
        reservation,
    )
    .await?;
    if rate_bucket_is_limited(ip, reservation.now_ms) {
        let decision = combine_rate_bucket_outcomes(&[account, ip], reservation.now_ms);
        if ip.entered_blocked {
            if let Err(error) = insert_security_event_sqlite(&mut *transaction, event).await {
                transaction.rollback().await?;
                return Err(error);
            }
            transaction.commit().await?;
        } else {
            transaction.rollback().await?;
        }
        return Ok(decision);
    }
    let global = reserve_rate_bucket_sqlite(
        &mut transaction,
        "global",
        &reservation.global_hmac,
        reservation.global_max_attempts,
        reservation,
    )
    .await?;
    if rate_bucket_is_limited(global, reservation.now_ms) {
        let decision = combine_rate_bucket_outcomes(&[account, ip, global], reservation.now_ms);
        if global.entered_blocked {
            if let Err(error) = insert_security_event_sqlite(&mut *transaction, event).await {
                transaction.rollback().await?;
                return Err(error);
            }
            transaction.commit().await?;
        } else {
            transaction.rollback().await?;
        }
        return Ok(decision);
    }
    transaction.commit().await?;
    Ok(combine_rate_bucket_outcomes(
        &[account, ip, global],
        reservation.now_ms,
    ))
}

async fn reserve_login_attempt_postgres(
    pool: &PgPool,
    reservation: &LoginAttemptReservation,
    event: &NewLoginSecurityEvent,
) -> Result<LoginRateDecision, PersistenceError> {
    if let Some(decision) = preflight_existing_rate_limit_postgres(pool, reservation).await? {
        return Ok(decision);
    }
    let mut transaction = pool.begin().await?;
    let account = reserve_rate_bucket_postgres(
        &mut transaction,
        "account",
        &reservation.account_hmac,
        reservation.account_max_attempts,
        reservation,
    )
    .await?;
    if rate_bucket_is_limited(account, reservation.now_ms) {
        let decision = combine_rate_bucket_outcomes(&[account], reservation.now_ms);
        if account.entered_blocked {
            if let Err(error) = insert_security_event_postgres(&mut *transaction, event).await {
                transaction.rollback().await?;
                return Err(error);
            }
            transaction.commit().await?;
        } else {
            transaction.rollback().await?;
        }
        return Ok(decision);
    }
    let ip = reserve_rate_bucket_postgres(
        &mut transaction,
        "ip",
        &reservation.ip_prefix_hmac,
        reservation.ip_max_attempts,
        reservation,
    )
    .await?;
    if rate_bucket_is_limited(ip, reservation.now_ms) {
        let decision = combine_rate_bucket_outcomes(&[account, ip], reservation.now_ms);
        if ip.entered_blocked {
            if let Err(error) = insert_security_event_postgres(&mut *transaction, event).await {
                transaction.rollback().await?;
                return Err(error);
            }
            transaction.commit().await?;
        } else {
            transaction.rollback().await?;
        }
        return Ok(decision);
    }
    let global = reserve_rate_bucket_postgres(
        &mut transaction,
        "global",
        &reservation.global_hmac,
        reservation.global_max_attempts,
        reservation,
    )
    .await?;
    if rate_bucket_is_limited(global, reservation.now_ms) {
        let decision = combine_rate_bucket_outcomes(&[account, ip, global], reservation.now_ms);
        if global.entered_blocked {
            if let Err(error) = insert_security_event_postgres(&mut *transaction, event).await {
                transaction.rollback().await?;
                return Err(error);
            }
            transaction.commit().await?;
        } else {
            transaction.rollback().await?;
        }
        return Ok(decision);
    }
    transaction.commit().await?;
    Ok(combine_rate_bucket_outcomes(
        &[account, ip, global],
        reservation.now_ms,
    ))
}

async fn preflight_existing_rate_limit_sqlite(
    pool: &SqlitePool,
    reservation: &LoginAttemptReservation,
) -> Result<Option<LoginRateDecision>, PersistenceError> {
    let blocked_until_ms: Option<i64> = sqlx::query_scalar(
        "SELECT MAX(blocked_until_ms) FROM login_rate_buckets WHERE key_version=? AND blocked_until_ms>? AND ((scope='account' AND bucket_hmac=?) OR (scope='ip' AND bucket_hmac=?) OR (scope='global' AND bucket_hmac=?))",
    )
    .bind(database_key_version(reservation.key_version)?)
    .bind(reservation.now_ms)
    .bind(reservation.account_hmac.as_slice())
    .bind(reservation.ip_prefix_hmac.as_slice())
    .bind(reservation.global_hmac.as_slice())
    .fetch_one(pool)
    .await?;
    Ok(
        blocked_until_ms.map(|blocked_until_ms| LoginRateDecision::Limited {
            retry_after_ms: blocked_until_ms - reservation.now_ms,
            blocked_until_ms,
        }),
    )
}

async fn preflight_existing_rate_limit_postgres(
    pool: &PgPool,
    reservation: &LoginAttemptReservation,
) -> Result<Option<LoginRateDecision>, PersistenceError> {
    let blocked_until_ms: Option<i64> = sqlx::query_scalar(
        "SELECT MAX(blocked_until_ms) FROM login_rate_buckets WHERE key_version=$1 AND blocked_until_ms>$2 AND ((scope='account' AND bucket_hmac=$3) OR (scope='ip' AND bucket_hmac=$4) OR (scope='global' AND bucket_hmac=$5))",
    )
    .bind(database_key_version(reservation.key_version)?)
    .bind(reservation.now_ms)
    .bind(reservation.account_hmac.as_slice())
    .bind(reservation.ip_prefix_hmac.as_slice())
    .bind(reservation.global_hmac.as_slice())
    .fetch_one(pool)
    .await?;
    Ok(
        blocked_until_ms.map(|blocked_until_ms| LoginRateDecision::Limited {
            retry_after_ms: blocked_until_ms - reservation.now_ms,
            blocked_until_ms,
        }),
    )
}

async fn reserve_rate_bucket_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    scope: &'static str,
    bucket_hmac: &AuthHmac,
    max_attempts: u32,
    reservation: &LoginAttemptReservation,
) -> Result<RateBucketOutcome, PersistenceError> {
    let key_version = database_key_version(reservation.key_version)?;
    let initial_window_expires_at_ms = reservation
        .now_ms
        .checked_add(reservation.window_ms)
        .ok_or(PersistenceError::InvalidLoginRatePolicy)?;
    let inserted = sqlx::query(
        "INSERT INTO login_rate_buckets (scope,key_version,bucket_hmac,window_started_at_ms,window_expires_at_ms,attempt_count,blocked_until_ms,updated_at_ms) VALUES (?,?,?,?,?,1,NULL,?) ON CONFLICT(scope,key_version,bucket_hmac) DO NOTHING",
    )
    .bind(scope)
    .bind(key_version)
    .bind(bucket_hmac.as_slice())
    .bind(reservation.now_ms)
    .bind(initial_window_expires_at_ms)
    .bind(reservation.now_ms)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    if inserted == 1 {
        return Ok(RateBucketOutcome {
            remaining_attempts: max_attempts - 1,
            reset_at_ms: initial_window_expires_at_ms,
            blocked_until_ms: None,
            entered_blocked: false,
        });
    }
    let row = sqlx::query(
        "SELECT window_started_at_ms,window_expires_at_ms,attempt_count,blocked_until_ms,updated_at_ms FROM login_rate_buckets WHERE scope=? AND key_version=? AND bucket_hmac=?",
    )
    .bind(scope)
    .bind(key_version)
    .bind(bucket_hmac.as_slice())
    .fetch_one(&mut **transaction)
    .await?;
    let window_started_at_ms: i64 = row.try_get("window_started_at_ms")?;
    let window_expires_at_ms: i64 = row.try_get("window_expires_at_ms")?;
    let attempt_count: i64 = row.try_get("attempt_count")?;
    let blocked_until_ms: Option<i64> = row.try_get("blocked_until_ms")?;
    let updated_at_ms: i64 = row.try_get("updated_at_ms")?;
    if blocked_until_ms.is_some_and(|blocked_until_ms| blocked_until_ms > reservation.now_ms) {
        return Ok(rate_bucket_outcome(
            window_expires_at_ms,
            attempt_count,
            blocked_until_ms,
            max_attempts,
            false,
        ));
    }
    let (
        new_window_started_at_ms,
        new_window_expires_at_ms,
        new_attempt_count,
        new_blocked_until_ms,
    ) = next_rate_bucket_state(
        window_started_at_ms,
        window_expires_at_ms,
        attempt_count,
        blocked_until_ms,
        max_attempts,
        reservation,
    )?;
    sqlx::query(
        "UPDATE login_rate_buckets SET window_started_at_ms=?,window_expires_at_ms=?,attempt_count=?,blocked_until_ms=?,updated_at_ms=? WHERE scope=? AND key_version=? AND bucket_hmac=?",
    )
    .bind(new_window_started_at_ms)
    .bind(new_window_expires_at_ms)
    .bind(new_attempt_count)
    .bind(new_blocked_until_ms)
    .bind(updated_at_ms.max(reservation.now_ms).max(new_window_started_at_ms))
    .bind(scope)
    .bind(key_version)
    .bind(bucket_hmac.as_slice())
    .execute(&mut **transaction)
    .await?;
    Ok(rate_bucket_outcome(
        new_window_expires_at_ms,
        new_attempt_count,
        new_blocked_until_ms,
        max_attempts,
        !blocked_until_ms.is_some_and(|blocked_until_ms| blocked_until_ms > reservation.now_ms)
            && new_blocked_until_ms
                .is_some_and(|blocked_until_ms| blocked_until_ms > reservation.now_ms),
    ))
}

async fn reserve_rate_bucket_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    scope: &'static str,
    bucket_hmac: &AuthHmac,
    max_attempts: u32,
    reservation: &LoginAttemptReservation,
) -> Result<RateBucketOutcome, PersistenceError> {
    let key_version = database_key_version(reservation.key_version)?;
    let initial_window_expires_at_ms = reservation
        .now_ms
        .checked_add(reservation.window_ms)
        .ok_or(PersistenceError::InvalidLoginRatePolicy)?;
    let inserted = sqlx::query(
        "INSERT INTO login_rate_buckets (scope,key_version,bucket_hmac,window_started_at_ms,window_expires_at_ms,attempt_count,blocked_until_ms,updated_at_ms) VALUES ($1,$2,$3,$4,$5,1,NULL,$4) ON CONFLICT(scope,key_version,bucket_hmac) DO NOTHING",
    )
    .bind(scope)
    .bind(key_version)
    .bind(bucket_hmac.as_slice())
    .bind(reservation.now_ms)
    .bind(initial_window_expires_at_ms)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    if inserted == 1 {
        return Ok(RateBucketOutcome {
            remaining_attempts: max_attempts - 1,
            reset_at_ms: initial_window_expires_at_ms,
            blocked_until_ms: None,
            entered_blocked: false,
        });
    }
    let row = sqlx::query(
        "SELECT window_started_at_ms,window_expires_at_ms,attempt_count,blocked_until_ms,updated_at_ms FROM login_rate_buckets WHERE scope=$1 AND key_version=$2 AND bucket_hmac=$3 FOR UPDATE",
    )
    .bind(scope)
    .bind(key_version)
    .bind(bucket_hmac.as_slice())
    .fetch_one(&mut **transaction)
    .await?;
    let window_started_at_ms: i64 = row.try_get("window_started_at_ms")?;
    let window_expires_at_ms: i64 = row.try_get("window_expires_at_ms")?;
    let attempt_count: i32 = row.try_get("attempt_count")?;
    let blocked_until_ms: Option<i64> = row.try_get("blocked_until_ms")?;
    let updated_at_ms: i64 = row.try_get("updated_at_ms")?;
    if blocked_until_ms.is_some_and(|blocked_until_ms| blocked_until_ms > reservation.now_ms) {
        return Ok(rate_bucket_outcome(
            window_expires_at_ms,
            i64::from(attempt_count),
            blocked_until_ms,
            max_attempts,
            false,
        ));
    }
    let (
        new_window_started_at_ms,
        new_window_expires_at_ms,
        new_attempt_count,
        new_blocked_until_ms,
    ) = next_rate_bucket_state(
        window_started_at_ms,
        window_expires_at_ms,
        i64::from(attempt_count),
        blocked_until_ms,
        max_attempts,
        reservation,
    )?;
    let new_attempt_count =
        i32::try_from(new_attempt_count).map_err(|_| PersistenceError::InvalidLoginRatePolicy)?;
    sqlx::query(
        "UPDATE login_rate_buckets SET window_started_at_ms=$1,window_expires_at_ms=$2,attempt_count=$3,blocked_until_ms=$4,updated_at_ms=$5 WHERE scope=$6 AND key_version=$7 AND bucket_hmac=$8",
    )
    .bind(new_window_started_at_ms)
    .bind(new_window_expires_at_ms)
    .bind(new_attempt_count)
    .bind(new_blocked_until_ms)
    .bind(updated_at_ms.max(reservation.now_ms).max(new_window_started_at_ms))
    .bind(scope)
    .bind(key_version)
    .bind(bucket_hmac.as_slice())
    .execute(&mut **transaction)
    .await?;
    Ok(rate_bucket_outcome(
        new_window_expires_at_ms,
        i64::from(new_attempt_count),
        new_blocked_until_ms,
        max_attempts,
        !blocked_until_ms.is_some_and(|blocked_until_ms| blocked_until_ms > reservation.now_ms)
            && new_blocked_until_ms
                .is_some_and(|blocked_until_ms| blocked_until_ms > reservation.now_ms),
    ))
}

#[allow(clippy::too_many_arguments)]
fn next_rate_bucket_state(
    window_started_at_ms: i64,
    window_expires_at_ms: i64,
    attempt_count: i64,
    blocked_until_ms: Option<i64>,
    max_attempts: u32,
    reservation: &LoginAttemptReservation,
) -> Result<(i64, i64, i64, Option<i64>), PersistenceError> {
    let new_attempt_count = attempt_count.saturating_add(1).min(i64::from(i32::MAX));
    if blocked_until_ms.is_some_and(|blocked_until_ms| blocked_until_ms > reservation.now_ms) {
        return Ok((
            window_started_at_ms,
            window_expires_at_ms,
            new_attempt_count,
            blocked_until_ms,
        ));
    }
    if window_expires_at_ms <= reservation.now_ms {
        let reset_at_ms = reservation
            .now_ms
            .checked_add(reservation.window_ms)
            .ok_or(PersistenceError::InvalidLoginRatePolicy)?;
        return Ok((reservation.now_ms, reset_at_ms, 1, None));
    }
    let new_blocked_until_ms = if new_attempt_count > i64::from(max_attempts) {
        Some(
            reservation
                .now_ms
                .checked_add(reservation.lockout_ms)
                .ok_or(PersistenceError::InvalidLoginRatePolicy)?
                .max(window_started_at_ms),
        )
    } else {
        None
    };
    Ok((
        window_started_at_ms,
        window_expires_at_ms,
        new_attempt_count,
        new_blocked_until_ms,
    ))
}

fn rate_bucket_outcome(
    reset_at_ms: i64,
    attempt_count: i64,
    blocked_until_ms: Option<i64>,
    max_attempts: u32,
    entered_blocked: bool,
) -> RateBucketOutcome {
    let attempts = u32::try_from(attempt_count).unwrap_or(u32::MAX);
    RateBucketOutcome {
        remaining_attempts: max_attempts.saturating_sub(attempts),
        reset_at_ms,
        blocked_until_ms,
        entered_blocked,
    }
}

fn validate_secret_record(record: &NewSecretRecord) -> Result<(), PersistenceError> {
    validate_non_negative_timestamp(record.created_at_ms)?;
    database_key_version(record.envelope.key_version)?;
    if record.binding.schema_version == 0
        || record.envelope.nonce.len() != 24
        || record.envelope.ciphertext.is_empty()
        || (record.binding.purpose == SecretPurpose::RootKeyCanary
            && record.binding != SecretBinding::root_key_canary())
    {
        return Err(PersistenceError::InvalidSecretRecord);
    }
    Ok(())
}

fn validate_recovery_code_set(set: &NewRecoveryCodeSet) -> Result<(), PersistenceError> {
    validate_non_negative_timestamp(set.created_at_ms)?;
    if set.created_at_ms > JSON_SAFE_INTEGER_MAX_I64 || set.codes.len() != 8 {
        return Err(PersistenceError::InvalidRecoveryCodeSet);
    }
    let mut ids = std::collections::HashSet::new();
    let mut digests = std::collections::HashSet::new();
    for code in &set.codes {
        database_key_version(code.digest_key_version)?;
        if !ids.insert(code.id) || !digests.insert((code.digest_key_version, code.code_hmac)) {
            return Err(PersistenceError::InvalidRecoveryCodeSet);
        }
    }
    Ok(())
}

fn decode_recovery_code_summary(
    row: (i64, i64, i64, i64),
) -> Result<RecoveryCodeSetSummary, PersistenceError> {
    let set_version = u64::try_from(row.0).map_err(|_| PersistenceError::InvalidRecoveryCodeSet)?;
    let total_count = u8::try_from(row.1).map_err(|_| PersistenceError::InvalidRecoveryCodeSet)?;
    let remaining_count =
        u8::try_from(row.2).map_err(|_| PersistenceError::InvalidRecoveryCodeSet)?;
    if set_version == 0
        || set_version > JSON_SAFE_INTEGER_MAX_I64 as u64
        || total_count != 8
        || remaining_count > total_count
        || !(0..=JSON_SAFE_INTEGER_MAX_I64).contains(&row.3)
    {
        return Err(PersistenceError::InvalidRecoveryCodeSet);
    }
    Ok(RecoveryCodeSetSummary {
        set_version,
        total_count,
        remaining_count,
        created_at_ms: row.3,
    })
}

async fn ensure_secret_record_sqlite(
    pool: &SqlitePool,
    record: &NewSecretRecord,
) -> Result<StoredSecretRecord, PersistenceError> {
    let mut transaction = pool.begin().await?;
    sqlx::query(
        "INSERT INTO secret_records (id,owner_type,owner_id,purpose,schema_version,key_version,nonce,ciphertext,aad_hash,created_at_ms,rotated_from,deleted_at_ms,revision) VALUES (?,?,?,?,?,?,?,?,?,?,?,NULL,0) ON CONFLICT DO NOTHING",
    )
    .bind(record.id.to_string())
    .bind(record.binding.owner_kind.as_str())
    .bind(record.binding.owner_id.to_string())
    .bind(record.binding.purpose.as_str())
    .bind(i64::from(record.binding.schema_version))
    .bind(i64::from(record.envelope.key_version))
    .bind(record.envelope.nonce.as_slice())
    .bind(record.envelope.ciphertext.as_slice())
    .bind(record.envelope.aad_hash.as_slice())
    .bind(record.created_at_ms)
    .bind(record.rotated_from.map(|id| id.to_string()))
    .execute(&mut *transaction)
    .await?;
    let row = sqlx::query(
        "SELECT id,owner_type,owner_id,purpose,schema_version,key_version,nonce,ciphertext,aad_hash,created_at_ms,rotated_from,revision FROM secret_records WHERE owner_type=? AND owner_id=? AND purpose=? AND deleted_at_ms IS NULL",
    )
    .bind(record.binding.owner_kind.as_str())
    .bind(record.binding.owner_id.to_string())
    .bind(record.binding.purpose.as_str())
    .fetch_one(&mut *transaction)
    .await?;
    let stored = decode_secret_record_row(
        uuid::Uuid::parse_str(row.try_get::<&str, _>("id")?)?,
        row.try_get("owner_type")?,
        uuid::Uuid::parse_str(row.try_get::<&str, _>("owner_id")?)?,
        row.try_get("purpose")?,
        row.try_get("schema_version")?,
        row.try_get("key_version")?,
        row.try_get("nonce")?,
        row.try_get("ciphertext")?,
        row.try_get("aad_hash")?,
        row.try_get("created_at_ms")?,
        row.try_get::<Option<&str>, _>("rotated_from")?
            .map(uuid::Uuid::parse_str)
            .transpose()?,
        row.try_get("revision")?,
    )?;
    transaction.commit().await?;
    Ok(stored)
}

async fn ensure_secret_record_postgres(
    pool: &PgPool,
    record: &NewSecretRecord,
) -> Result<StoredSecretRecord, PersistenceError> {
    let mut transaction = pool.begin().await?;
    sqlx::query(
        "INSERT INTO secret_records (id,owner_type,owner_id,purpose,schema_version,key_version,nonce,ciphertext,aad_hash,created_at_ms,rotated_from,deleted_at_ms,revision) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,NULL,0) ON CONFLICT DO NOTHING",
    )
    .bind(record.id.into_uuid())
    .bind(record.binding.owner_kind.as_str())
    .bind(record.binding.owner_id)
    .bind(record.binding.purpose.as_str())
    .bind(i32::try_from(record.binding.schema_version).map_err(|_| PersistenceError::InvalidSecretRecord)?)
    .bind(i32::try_from(record.envelope.key_version).map_err(|_| PersistenceError::InvalidKeyVersion)?)
    .bind(record.envelope.nonce.as_slice())
    .bind(record.envelope.ciphertext.as_slice())
    .bind(record.envelope.aad_hash.as_slice())
    .bind(record.created_at_ms)
    .bind(record.rotated_from.map(EntityId::into_uuid))
    .execute(&mut *transaction)
    .await?;
    let row = sqlx::query(
        "SELECT id,owner_type,owner_id,purpose,schema_version,key_version,nonce,ciphertext,aad_hash,created_at_ms,rotated_from,revision FROM secret_records WHERE owner_type=$1 AND owner_id=$2 AND purpose=$3 AND deleted_at_ms IS NULL FOR UPDATE",
    )
    .bind(record.binding.owner_kind.as_str())
    .bind(record.binding.owner_id)
    .bind(record.binding.purpose.as_str())
    .fetch_one(&mut *transaction)
    .await?;
    let stored = decode_secret_record_row(
        row.try_get("id")?,
        row.try_get("owner_type")?,
        row.try_get("owner_id")?,
        row.try_get("purpose")?,
        row.try_get("schema_version")?,
        row.try_get("key_version")?,
        row.try_get("nonce")?,
        row.try_get("ciphertext")?,
        row.try_get("aad_hash")?,
        row.try_get("created_at_ms")?,
        row.try_get("rotated_from")?,
        row.try_get("revision")?,
    )?;
    transaction.commit().await?;
    Ok(stored)
}

fn decode_sqlite_secret_record(
    row: sqlx::sqlite::SqliteRow,
) -> Result<StoredSecretRecord, PersistenceError> {
    decode_secret_record_row(
        uuid::Uuid::parse_str(row.try_get::<&str, _>("id")?)?,
        row.try_get("owner_type")?,
        uuid::Uuid::parse_str(row.try_get::<&str, _>("owner_id")?)?,
        row.try_get("purpose")?,
        row.try_get("schema_version")?,
        row.try_get("key_version")?,
        row.try_get("nonce")?,
        row.try_get("ciphertext")?,
        row.try_get("aad_hash")?,
        row.try_get("created_at_ms")?,
        row.try_get::<Option<&str>, _>("rotated_from")?
            .map(uuid::Uuid::parse_str)
            .transpose()?,
        row.try_get("revision")?,
    )
}

fn decode_postgres_secret_record(
    row: sqlx::postgres::PgRow,
) -> Result<StoredSecretRecord, PersistenceError> {
    decode_secret_record_row(
        row.try_get("id")?,
        row.try_get("owner_type")?,
        row.try_get("owner_id")?,
        row.try_get("purpose")?,
        row.try_get("schema_version")?,
        row.try_get("key_version")?,
        row.try_get("nonce")?,
        row.try_get("ciphertext")?,
        row.try_get("aad_hash")?,
        row.try_get("created_at_ms")?,
        row.try_get("rotated_from")?,
        row.try_get("revision")?,
    )
}

#[allow(clippy::too_many_arguments)]
fn decode_secret_record_row(
    id: uuid::Uuid,
    owner_type: String,
    owner_id: uuid::Uuid,
    purpose: String,
    schema_version: i32,
    key_version: i32,
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
    aad_hash: Vec<u8>,
    created_at_ms: i64,
    rotated_from: Option<uuid::Uuid>,
    revision: i64,
) -> Result<StoredSecretRecord, PersistenceError> {
    let owner_kind = SecretOwnerKind::parse(&owner_type)
        .map_err(|_| PersistenceError::InvalidStoredSecretRecord)?;
    let purpose =
        SecretPurpose::parse(&purpose).map_err(|_| PersistenceError::InvalidStoredSecretRecord)?;
    let schema_version =
        u32::try_from(schema_version).map_err(|_| PersistenceError::InvalidStoredSecretRecord)?;
    let key_version =
        u32::try_from(key_version).map_err(|_| PersistenceError::InvalidStoredSecretRecord)?;
    let binding = SecretBinding::new(purpose, owner_kind, owner_id, schema_version)
        .map_err(|_| PersistenceError::InvalidStoredSecretRecord)?;
    let nonce = nonce
        .try_into()
        .map_err(|_| PersistenceError::InvalidStoredSecretRecord)?;
    let aad_hash = aad_hash
        .try_into()
        .map_err(|_| PersistenceError::InvalidStoredSecretRecord)?;
    let revision =
        u64::try_from(revision).map_err(|_| PersistenceError::InvalidStoredSecretRecord)?;
    if created_at_ms < 0 || ciphertext.is_empty() || key_version == 0 {
        return Err(PersistenceError::InvalidStoredSecretRecord);
    }
    Ok(StoredSecretRecord {
        id: EntityId::from_uuid(id),
        binding,
        envelope: SecretEnvelope {
            key_version,
            nonce,
            ciphertext,
            aad_hash,
        },
        created_at_ms,
        rotated_from: rotated_from.map(EntityId::from_uuid),
        revision: Revision::from_value(revision),
    })
}

async fn rotate_secret_record_sqlite(
    pool: &SqlitePool,
    expected: &StoredSecretRecord,
    replacement: &NewSecretRecord,
    now_ms: i64,
) -> Result<StoredSecretRecord, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let affected = sqlx::query(
        "UPDATE secret_records SET deleted_at_ms=?,revision=revision+1 WHERE id=? AND owner_type=? AND owner_id=? AND purpose=? AND schema_version=? AND key_version=? AND revision=? AND deleted_at_ms IS NULL",
    )
    .bind(now_ms)
    .bind(expected.id.to_string())
    .bind(expected.binding.owner_kind.as_str())
    .bind(expected.binding.owner_id.to_string())
    .bind(expected.binding.purpose.as_str())
    .bind(i64::from(expected.binding.schema_version))
    .bind(i64::from(expected.envelope.key_version))
    .bind(i64::try_from(expected.revision.value()).map_err(|_| PersistenceError::RevisionOutOfRange)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if affected != 1 {
        return Err(PersistenceError::SecretRecordConflict);
    }
    insert_secret_record_sqlite(&mut transaction, replacement).await?;
    transaction.commit().await?;
    Ok(StoredSecretRecord {
        id: replacement.id,
        binding: replacement.binding,
        envelope: replacement.envelope.clone(),
        created_at_ms: replacement.created_at_ms,
        rotated_from: replacement.rotated_from,
        revision: Revision::initial(),
    })
}

async fn rotate_secret_record_postgres(
    pool: &PgPool,
    expected: &StoredSecretRecord,
    replacement: &NewSecretRecord,
    now_ms: i64,
) -> Result<StoredSecretRecord, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let affected = sqlx::query(
        "UPDATE secret_records SET deleted_at_ms=$1,revision=revision+1 WHERE id=$2 AND owner_type=$3 AND owner_id=$4 AND purpose=$5 AND schema_version=$6 AND key_version=$7 AND revision=$8 AND deleted_at_ms IS NULL",
    )
    .bind(now_ms)
    .bind(expected.id.into_uuid())
    .bind(expected.binding.owner_kind.as_str())
    .bind(expected.binding.owner_id)
    .bind(expected.binding.purpose.as_str())
    .bind(
        i32::try_from(expected.binding.schema_version)
            .map_err(|_| PersistenceError::InvalidSecretRecord)?,
    )
    .bind(
        i32::try_from(expected.envelope.key_version)
            .map_err(|_| PersistenceError::InvalidKeyVersion)?,
    )
    .bind(i64::try_from(expected.revision.value()).map_err(|_| PersistenceError::RevisionOutOfRange)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if affected != 1 {
        return Err(PersistenceError::SecretRecordConflict);
    }
    insert_secret_record_postgres(&mut transaction, replacement).await?;
    transaction.commit().await?;
    Ok(StoredSecretRecord {
        id: replacement.id,
        binding: replacement.binding,
        envelope: replacement.envelope.clone(),
        created_at_ms: replacement.created_at_ms,
        rotated_from: replacement.rotated_from,
        revision: Revision::initial(),
    })
}

async fn insert_secret_record_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    record: &NewSecretRecord,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "INSERT INTO secret_records (id,owner_type,owner_id,purpose,schema_version,key_version,nonce,ciphertext,aad_hash,created_at_ms,rotated_from,deleted_at_ms,revision) VALUES (?,?,?,?,?,?,?,?,?,?,?,NULL,0)",
    )
    .bind(record.id.to_string())
    .bind(record.binding.owner_kind.as_str())
    .bind(record.binding.owner_id.to_string())
    .bind(record.binding.purpose.as_str())
    .bind(i64::from(record.binding.schema_version))
    .bind(i64::from(record.envelope.key_version))
    .bind(record.envelope.nonce.as_slice())
    .bind(record.envelope.ciphertext.as_slice())
    .bind(record.envelope.aad_hash.as_slice())
    .bind(record.created_at_ms)
    .bind(record.rotated_from.map(|id| id.to_string()))
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn insert_secret_record_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    record: &NewSecretRecord,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "INSERT INTO secret_records (id,owner_type,owner_id,purpose,schema_version,key_version,nonce,ciphertext,aad_hash,created_at_ms,rotated_from,deleted_at_ms,revision) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,NULL,0)",
    )
    .bind(record.id.into_uuid())
    .bind(record.binding.owner_kind.as_str())
    .bind(record.binding.owner_id)
    .bind(record.binding.purpose.as_str())
    .bind(i32::try_from(record.binding.schema_version).map_err(|_| PersistenceError::InvalidSecretRecord)?)
    .bind(i32::try_from(record.envelope.key_version).map_err(|_| PersistenceError::InvalidKeyVersion)?)
    .bind(record.envelope.nonce.as_slice())
    .bind(record.envelope.ciphertext.as_slice())
    .bind(record.envelope.aad_hash.as_slice())
    .bind(record.created_at_ms)
    .bind(record.rotated_from.map(EntityId::into_uuid))
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn insert_recovery_code_set_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    user_id: EntityId,
    set_version: i64,
    set: &NewRecoveryCodeSet,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "INSERT INTO recovery_code_sets (user_id,set_version,status,total_count,created_at_ms,replaced_at_ms) VALUES (?,?,'active',8,?,NULL)",
    )
    .bind(user_id.to_string())
    .bind(set_version)
    .bind(set.created_at_ms)
    .execute(&mut **transaction)
    .await?;
    for (index, code) in set.codes.iter().enumerate() {
        sqlx::query(
            "INSERT INTO recovery_codes (id,user_id,set_version,position,digest_key_version,code_hmac,created_at_ms,consumed_at_ms) VALUES (?,?,?,?,?,?,?,NULL)",
        )
        .bind(code.id.to_string())
        .bind(user_id.to_string())
        .bind(set_version)
        .bind(i64::try_from(index + 1).map_err(|_| PersistenceError::InvalidRecoveryCodeSet)?)
        .bind(i64::from(code.digest_key_version))
        .bind(code.code_hmac.as_slice())
        .bind(set.created_at_ms)
        .execute(&mut **transaction)
        .await?;
    }
    Ok(())
}

async fn insert_recovery_code_set_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: EntityId,
    set_version: i64,
    set: &NewRecoveryCodeSet,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "INSERT INTO recovery_code_sets (user_id,set_version,status,total_count,created_at_ms,replaced_at_ms) VALUES ($1,$2,'active',8,$3,NULL)",
    )
    .bind(user_id.into_uuid())
    .bind(set_version)
    .bind(set.created_at_ms)
    .execute(&mut **transaction)
    .await?;
    for (index, code) in set.codes.iter().enumerate() {
        sqlx::query(
            "INSERT INTO recovery_codes (id,user_id,set_version,position,digest_key_version,code_hmac,created_at_ms,consumed_at_ms) VALUES ($1,$2,$3,$4,$5,$6,$7,NULL)",
        )
        .bind(code.id.into_uuid())
        .bind(user_id.into_uuid())
        .bind(set_version)
        .bind(i16::try_from(index + 1).map_err(|_| PersistenceError::InvalidRecoveryCodeSet)?)
        .bind(i32::try_from(code.digest_key_version).map_err(|_| PersistenceError::InvalidKeyVersion)?)
        .bind(code.code_hmac.as_slice())
        .bind(set.created_at_ms)
        .execute(&mut **transaction)
        .await?;
    }
    Ok(())
}

type RecoveryReplacementSnapshot = (i64, i64, i64, String, Option<i64>, i64, i64, i64);

async fn replace_recovery_codes_sqlite(
    pool: &SqlitePool,
    command: &RecoveryCodeReplacement<'_>,
) -> Result<RecoveryCodeSetSummary, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let expected_auth_revision = i64::try_from(command.expected_auth_revision.value())
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let locked = sqlx::query(
        "UPDATE user_auth_state SET updated_at_ms=updated_at_ms WHERE user_id=? AND auth_revision=?",
    )
    .bind(command.user_id.to_string())
    .bind(expected_auth_revision)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if locked != 1 {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let snapshot: Option<RecoveryReplacementSnapshot> = sqlx::query_as(
        "SELECT u.revision,uas.auth_revision,s.auth_revision,s.status,s.revoked_at_ms,s.recent_auth_at_ms,s.idle_expires_at_ms,s.absolute_expires_at_ms FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=? AND s.id=? AND u.status='active' AND u.deleted_at_ms IS NULL",
    )
    .bind(command.user_id.to_string())
    .bind(command.actor_session_id.to_string())
    .fetch_optional(&mut *transaction)
    .await?;
    validate_recovery_replacement_snapshot(snapshot, command)?;
    let next_version: i64 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(set_version),0)+1 FROM recovery_code_sets WHERE user_id=?",
    )
    .bind(command.user_id.to_string())
    .fetch_one(&mut *transaction)
    .await?;
    sqlx::query(
        "UPDATE recovery_code_sets SET status='replaced',replaced_at_ms=? WHERE user_id=? AND status='active' AND created_at_ms<=?",
    )
    .bind(command.now_ms)
    .bind(command.user_id.to_string())
    .bind(command.now_ms)
    .execute(&mut *transaction)
    .await?;
    insert_recovery_code_set_sqlite(
        &mut transaction,
        command.user_id,
        next_version,
        command.replacement,
    )
    .await?;
    transaction.commit().await?;
    decode_recovery_code_summary((next_version, 8, 8, command.now_ms))
}

async fn replace_recovery_codes_postgres(
    pool: &PgPool,
    command: &RecoveryCodeReplacement<'_>,
) -> Result<RecoveryCodeSetSummary, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let expected_auth_revision = i64::try_from(command.expected_auth_revision.value())
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let snapshot: Option<RecoveryReplacementSnapshot> = sqlx::query_as(
        "SELECT u.revision,uas.auth_revision,s.auth_revision,s.status,s.revoked_at_ms,s.recent_auth_at_ms,s.idle_expires_at_ms,s.absolute_expires_at_ms FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=$1 AND s.id=$2 AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=$3 FOR UPDATE OF u,uas,s",
    )
    .bind(command.user_id.into_uuid())
    .bind(command.actor_session_id.into_uuid())
    .bind(expected_auth_revision)
    .fetch_optional(&mut *transaction)
    .await?;
    validate_recovery_replacement_snapshot(snapshot, command)?;
    let next_version: i64 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(set_version),0)+1 FROM recovery_code_sets WHERE user_id=$1",
    )
    .bind(command.user_id.into_uuid())
    .fetch_one(&mut *transaction)
    .await?;
    sqlx::query(
        "UPDATE recovery_code_sets SET status='replaced',replaced_at_ms=$1 WHERE user_id=$2 AND status='active' AND created_at_ms<=$1",
    )
    .bind(command.now_ms)
    .bind(command.user_id.into_uuid())
    .execute(&mut *transaction)
    .await?;
    insert_recovery_code_set_postgres(
        &mut transaction,
        command.user_id,
        next_version,
        command.replacement,
    )
    .await?;
    transaction.commit().await?;
    decode_recovery_code_summary((next_version, 8, 8, command.now_ms))
}

fn validate_recovery_replacement_snapshot(
    snapshot: Option<RecoveryReplacementSnapshot>,
    command: &RecoveryCodeReplacement<'_>,
) -> Result<(), PersistenceError> {
    let Some((
        user_revision,
        auth_revision,
        session_auth_revision,
        status,
        revoked_at,
        recent_auth_at,
        idle_expires_at,
        absolute_expires_at,
    )) = snapshot
    else {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    };
    let expected_user_revision = i64::try_from(command.expected_user_revision.value())
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let expected_auth_revision = i64::try_from(command.expected_auth_revision.value())
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    if user_revision != expected_user_revision
        || auth_revision != expected_auth_revision
        || session_auth_revision != expected_auth_revision
        || recent_auth_at != command.expected_recent_auth_at_ms
        || status != "active"
        || revoked_at.is_some()
        || idle_expires_at <= command.now_ms
        || absolute_expires_at <= command.now_ms
    {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    Ok(())
}

async fn bootstrap_sqlite(
    pool: &SqlitePool,
    instance: &Instance,
    owner: &UserAccount,
    settings_json: &str,
    instance_revision: i64,
    owner_revision: i64,
    recovery_codes: Option<&NewRecoveryCodeSet>,
) -> Result<EntityId, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let claimed = sqlx::query(
        "UPDATE control_plane_bootstrap SET status=status WHERE singleton_key=1 AND status='pending'",
    )
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    let record: Option<(String, Option<String>, bool, bool, bool)> = sqlx::query_as(
        "SELECT status, instance_id, EXISTS(SELECT 1 FROM instances), EXISTS(SELECT 1 FROM users), EXISTS(SELECT 1 FROM users WHERE role='owner' AND status='active' AND deleted_at_ms IS NULL) FROM control_plane_bootstrap WHERE singleton_key=1",
    )
    .fetch_optional(&mut *transaction)
    .await?;
    let state = record
        .as_ref()
        .ok_or(PersistenceError::InconsistentBootstrapState)
        .and_then(|row| {
            classify_bootstrap_record(&row.0, row.1.is_some(), (row.2, row.3, row.4))
        })?;
    if claimed != 1 {
        return if state == BootstrapState::Ready {
            Err(PersistenceError::AlreadyInitialized)
        } else {
            Err(PersistenceError::InconsistentBootstrapState)
        };
    }
    let (instance_id, needs_default_settings) = match state {
        BootstrapState::Uninitialized => {
            let result = sqlx::query(
                "INSERT INTO instances (singleton_key,id,name,public_id,created_at_ms,revision) VALUES (1,?,?,?,?,?)",
            )
            .bind(instance.id.to_string())
            .bind(instance.name.as_str())
            .bind(instance.public_id.to_string())
            .bind(instance.created_at_ms)
            .bind(instance_revision)
            .execute(&mut *transaction)
            .await;
            match result {
                Ok(_) => (instance.id, true),
                Err(error) => return Err(PersistenceError::Sql(error)),
            }
        }
        BootstrapState::LegacyNeedsOwner => {
            let stored_id = record
                .as_ref()
                .and_then(|row| row.1.as_deref())
                .ok_or(PersistenceError::InconsistentBootstrapState)?;
            let stored_id = uuid::Uuid::parse_str(stored_id)?;
            let stored_id = EntityId::from_uuid(stored_id);
            let stored_settings: Option<(i32, String)> = sqlx::query_as(
                "SELECT schema_version,value_json FROM instance_settings WHERE instance_id=? AND key=?",
            )
            .bind(stored_id.to_string())
            .bind(SUBSCRIPTION_SETTINGS_KEY)
            .fetch_optional(&mut *transaction)
            .await?;
            (stored_id, legacy_needs_default_settings(stored_settings)?)
        }
        BootstrapState::Ready => return Err(PersistenceError::AlreadyInitialized),
    };
    let owner_result = sqlx::query(
        "INSERT INTO users (id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms) VALUES (?,?,?,?,?,'active',?,?,?, ?,NULL) ON CONFLICT(username_norm) WHERE deleted_at_ms IS NULL DO NOTHING",
    )
    .bind(owner.id.to_string())
    .bind(owner.username.as_str())
    .bind(owner.username.normalized())
    .bind(owner.password_hash.as_str())
    .bind(owner.role.as_str())
    .bind(owner.principal_label.as_str())
    .bind(owner.force_password_change)
    .bind(owner_revision)
    .bind(owner.created_at_ms)
    .execute(&mut *transaction)
    .await;
    match owner_result {
        Ok(result) if result.rows_affected() == 1 => {}
        Ok(_) => return Err(PersistenceError::IdentityConflict),
        Err(error) => return Err(PersistenceError::Sql(error)),
    }
    sqlx::query(
        "INSERT INTO user_auth_state (user_id,auth_revision,password_changed_at_ms,updated_at_ms) VALUES (?,0,?,?)",
    )
    .bind(owner.id.to_string())
    .bind(owner.created_at_ms)
    .bind(owner.created_at_ms)
    .execute(&mut *transaction)
    .await?;
    if let Some(recovery_codes) = recovery_codes {
        insert_recovery_code_set_sqlite(&mut transaction, owner.id, 1, recovery_codes).await?;
    }
    if needs_default_settings {
        sqlx::query(
            "INSERT INTO instance_settings (instance_id,key,schema_version,value_json,revision,updated_by,updated_at_ms) VALUES (?,?,?,?,0,?,?)",
        )
        .bind(instance_id.to_string())
        .bind(SUBSCRIPTION_SETTINGS_KEY)
        .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
        .bind(settings_json)
        .bind(owner.id.to_string())
        .bind(owner.created_at_ms)
        .execute(&mut *transaction)
        .await?;
    }
    let finalized = sqlx::query(
        "UPDATE control_plane_bootstrap SET status='ready',instance_id=? WHERE singleton_key=1 AND status='pending'",
    )
    .bind(instance_id.to_string())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if finalized != 1 {
        return Err(PersistenceError::InconsistentBootstrapState);
    }
    transaction.commit().await?;
    Ok(instance_id)
}

async fn bootstrap_postgres(
    pool: &PgPool,
    instance: &Instance,
    owner: &UserAccount,
    settings_json: &str,
    instance_revision: i64,
    owner_revision: i64,
    recovery_codes: Option<&NewRecoveryCodeSet>,
) -> Result<EntityId, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let claimed = sqlx::query(
        "UPDATE control_plane_bootstrap SET status=status WHERE singleton_key=1 AND status='pending'",
    )
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    let record: Option<(String, Option<uuid::Uuid>, bool, bool, bool)> = sqlx::query_as(
        "SELECT status, instance_id, EXISTS(SELECT 1 FROM instances), EXISTS(SELECT 1 FROM users), EXISTS(SELECT 1 FROM users WHERE role='owner' AND status='active' AND deleted_at_ms IS NULL) FROM control_plane_bootstrap WHERE singleton_key=1",
    )
    .fetch_optional(&mut *transaction)
    .await?;
    let state = record
        .as_ref()
        .ok_or(PersistenceError::InconsistentBootstrapState)
        .and_then(|row| {
            classify_bootstrap_record(&row.0, row.1.is_some(), (row.2, row.3, row.4))
        })?;
    if claimed != 1 {
        return if state == BootstrapState::Ready {
            Err(PersistenceError::AlreadyInitialized)
        } else {
            Err(PersistenceError::InconsistentBootstrapState)
        };
    }
    let (instance_id, needs_default_settings) = match state {
        BootstrapState::Uninitialized => {
            let result = sqlx::query(
                "INSERT INTO instances (singleton_key,id,name,public_id,created_at_ms,revision) VALUES (1,$1,$2,$3,$4,$5)",
            )
            .bind(instance.id.into_uuid())
            .bind(instance.name.as_str())
            .bind(instance.public_id.into_uuid())
            .bind(instance.created_at_ms)
            .bind(instance_revision)
            .execute(&mut *transaction)
            .await;
            match result {
                Ok(_) => (instance.id, true),
                Err(error) => return Err(PersistenceError::Sql(error)),
            }
        }
        BootstrapState::LegacyNeedsOwner => {
            let stored_id = record
                .as_ref()
                .and_then(|row| row.1)
                .ok_or(PersistenceError::InconsistentBootstrapState)?;
            let stored_id = EntityId::from_uuid(stored_id);
            let stored_settings: Option<(i32, String)> = sqlx::query_as(
                "SELECT schema_version,value_json::text FROM instance_settings WHERE instance_id=$1 AND key=$2 FOR UPDATE",
            )
            .bind(stored_id.into_uuid())
            .bind(SUBSCRIPTION_SETTINGS_KEY)
            .fetch_optional(&mut *transaction)
            .await?;
            (stored_id, legacy_needs_default_settings(stored_settings)?)
        }
        BootstrapState::Ready => return Err(PersistenceError::AlreadyInitialized),
    };
    let owner_result = sqlx::query(
        "INSERT INTO users (id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms) VALUES ($1,$2,$3,$4,$5,'active',$6,$7,$8,$9,NULL) ON CONFLICT(username_norm) WHERE deleted_at_ms IS NULL DO NOTHING",
    )
    .bind(owner.id.into_uuid())
    .bind(owner.username.as_str())
    .bind(owner.username.normalized())
    .bind(owner.password_hash.as_str())
    .bind(owner.role.as_str())
    .bind(owner.principal_label.as_str())
    .bind(owner.force_password_change)
    .bind(owner_revision)
    .bind(owner.created_at_ms)
    .execute(&mut *transaction)
    .await;
    match owner_result {
        Ok(result) if result.rows_affected() == 1 => {}
        Ok(_) => return Err(PersistenceError::IdentityConflict),
        Err(error) => return Err(PersistenceError::Sql(error)),
    }
    sqlx::query(
        "INSERT INTO user_auth_state (user_id,auth_revision,password_changed_at_ms,updated_at_ms) VALUES ($1,0,$2,$2)",
    )
    .bind(owner.id.into_uuid())
    .bind(owner.created_at_ms)
    .execute(&mut *transaction)
    .await?;
    if let Some(recovery_codes) = recovery_codes {
        insert_recovery_code_set_postgres(&mut transaction, owner.id, 1, recovery_codes).await?;
    }
    if needs_default_settings {
        sqlx::query(
            "INSERT INTO instance_settings (instance_id,key,schema_version,value_json,revision,updated_by,updated_at_ms) VALUES ($1,$2,$3,$4::jsonb,0,$5,$6)",
        )
        .bind(instance_id.into_uuid())
        .bind(SUBSCRIPTION_SETTINGS_KEY)
        .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
        .bind(settings_json)
        .bind(owner.id.into_uuid())
        .bind(owner.created_at_ms)
        .execute(&mut *transaction)
        .await?;
    }
    let finalized = sqlx::query(
        "UPDATE control_plane_bootstrap SET status='ready',instance_id=$1 WHERE singleton_key=1 AND status='pending'",
    )
    .bind(instance_id.into_uuid())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if finalized != 1 {
        return Err(PersistenceError::InconsistentBootstrapState);
    }
    transaction.commit().await?;
    Ok(instance_id)
}

fn legacy_needs_default_settings(stored: Option<(i32, String)>) -> Result<bool, PersistenceError> {
    let Some((schema_version, value_json)) = stored else {
        return Ok(true);
    };
    if schema_version != SUBSCRIPTION_SETTINGS_SCHEMA
        || serde_json::from_str::<SubscriptionBehaviorSettings>(&value_json).is_err()
    {
        return Err(PersistenceError::InconsistentBootstrapState);
    }
    Ok(false)
}

fn decode_sqlite_instance(row: sqlx::sqlite::SqliteRow) -> Result<Instance, PersistenceError> {
    let id = uuid::Uuid::parse_str(row.try_get("id")?)?;
    let public_id = uuid::Uuid::parse_str(row.try_get("public_id")?)?;
    decode_instance(
        id,
        public_id,
        row.try_get("name")?,
        row.try_get("created_at_ms")?,
        row.try_get("revision")?,
    )
}

fn decode_postgres_instance(row: sqlx::postgres::PgRow) -> Result<Instance, PersistenceError> {
    decode_instance(
        row.try_get("id")?,
        row.try_get("public_id")?,
        row.try_get("name")?,
        row.try_get("created_at_ms")?,
        row.try_get("revision")?,
    )
}

fn decode_instance(
    id: uuid::Uuid,
    public_id: uuid::Uuid,
    name: String,
    created_at_ms: i64,
    revision: i64,
) -> Result<Instance, PersistenceError> {
    let revision = u64::try_from(revision).map_err(|_| PersistenceError::RevisionOutOfRange)?;
    Ok(Instance {
        id: EntityId::from_uuid(id),
        public_id: EntityId::from_uuid(public_id),
        name: InstanceName::parse(name)?,
        created_at_ms,
        revision: Revision::from_value(revision),
    })
}

fn classify_bootstrap_record(
    status: &str,
    has_instance_id: bool,
    (has_instance, has_user, has_active_owner): (bool, bool, bool),
) -> Result<BootstrapState, PersistenceError> {
    match (
        status,
        has_instance_id,
        has_instance,
        has_user,
        has_active_owner,
    ) {
        ("pending", false, false, false, false) => Ok(BootstrapState::Uninitialized),
        ("pending", true, true, false, false) => Ok(BootstrapState::LegacyNeedsOwner),
        ("ready", true, true, true, true) => Ok(BootstrapState::Ready),
        _ => Err(PersistenceError::InconsistentBootstrapState),
    }
}

fn duration_millis_string(duration: Duration) -> Result<String, PersistenceError> {
    let millis =
        u64::try_from(duration.as_millis()).map_err(|_| PersistenceError::TimeoutTooLarge)?;
    Ok(format!("{millis}ms"))
}

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("unsupported database URL scheme")]
    UnsupportedDatabaseUrl,
    #[error("database timeout exceeds the supported range")]
    TimeoutTooLarge,
    #[error("timestamp must be non-negative")]
    InvalidTimestamp,
    #[error("revision does not fit the database representation")]
    RevisionOutOfRange,
    #[error("the control-plane instance has already been initialized")]
    AlreadyInitialized,
    #[error("stored bootstrap records do not form a recoverable control-plane state")]
    InconsistentBootstrapState,
    #[error("the requested owner identity conflicts with an existing identity")]
    IdentityConflict,
    #[error("the encrypted secret record violates its typed binding or envelope contract")]
    InvalidSecretRecord,
    #[error("the stored encrypted secret record violates its typed binding or envelope contract")]
    InvalidStoredSecretRecord,
    #[error("the active encrypted secret record changed concurrently")]
    SecretRecordConflict,
    #[error("a recovery-code set must contain exactly eight distinct, versioned HMAC records")]
    InvalidRecoveryCodeSet,
    #[error("the setting revision does not match the stored revision")]
    RevisionConflict,
    #[error("the username lookup key is not normalized")]
    UsernameIsNotNormalized,
    #[error("authentication key versions must be positive 32-bit database integers")]
    InvalidKeyVersion,
    #[error("the session context digest and key version are inconsistent")]
    InvalidSessionContext,
    #[error("the session timestamps do not form a valid timeline")]
    InvalidSessionTimeline,
    #[error("the CSRF digest and key version are inconsistent")]
    InvalidCsrfBinding,
    #[error("the session timing policy is invalid")]
    InvalidSessionTimingPolicy,
    #[error("the request identifier is not bounded printable ASCII")]
    InvalidRequestId,
    #[error("the login rate policy is invalid")]
    InvalidLoginRatePolicy,
    #[error("the rate-limit event does not match the login-attempt reservation")]
    InvalidLoginRateEvent,
    #[error("rate-limit events must be committed atomically with the blocking bucket transition")]
    RateLimitedEventMustBeAtomic,
    #[error("session creation must atomically record a successful login event")]
    SessionEventMustRecordLoginSuccess,
    #[error("the session principal is unavailable or its authentication revision changed")]
    SessionPrincipalUnavailable,
    #[error("the user's authentication state is unavailable")]
    AuthStateUnavailable,
    #[error("the session revision changed during the operation")]
    SessionRevisionConflict,
    #[error("the requested session rotation is invalid")]
    InvalidSessionRotation,
    #[error("the current-session revocation event does not match the logout transition")]
    InvalidSessionRevocationEvent,
    #[error("the stored session status is invalid")]
    InvalidStoredSessionStatus,
    #[error("the stored authentication level is invalid")]
    InvalidStoredAuthLevel,
    #[error("the authentication challenge input violates a durable state invariant")]
    InvalidAuthChallenge,
    #[error("the stored authentication challenge violates its schema contract")]
    InvalidStoredAuthChallenge,
    #[error("the TOTP credential command violates a durable state invariant")]
    InvalidTotpCredential,
    #[error("the stored TOTP credential violates its schema contract")]
    InvalidStoredTotpCredential,
    #[error("the WebAuthn ceremony command violates a durable state invariant")]
    InvalidWebAuthnCeremony,
    #[error("the stored WebAuthn ceremony violates its encrypted typed-state contract")]
    InvalidStoredWebAuthnCeremony,
    #[error("the WebAuthn credential command violates a durable state invariant")]
    InvalidWebAuthnCredential,
    #[error("the stored WebAuthn credential violates its schema contract")]
    InvalidStoredWebAuthnCredential,
    #[error("the stored session revocation reason is invalid")]
    InvalidStoredRevocationReason,
    #[error("stored instance name is invalid: {0}")]
    InvalidInstanceName(#[from] nodecontroll_domain::InstanceNameError),
    #[error("stored username is invalid: {0}")]
    InvalidUsername(#[from] nodecontroll_domain::UsernameError),
    #[error("stored password hash is invalid: {0}")]
    InvalidPasswordHash(#[from] nodecontroll_domain::PasswordHashError),
    #[error("stored principal label is invalid: {0}")]
    InvalidPrincipalLabel(#[from] nodecontroll_domain::PrincipalLabelError),
    #[error("stored user role is invalid: {0}")]
    InvalidUserRole(#[from] nodecontroll_domain::UserRoleParseError),
    #[error("stored user status is invalid: {0}")]
    InvalidUserStatus(#[from] nodecontroll_domain::UserStatusParseError),
    #[error("stored UUID is invalid: {0}")]
    InvalidUuid(#[from] uuid::Error),
    #[error("database operation failed: {0}")]
    Sql(#[from] sqlx::Error),
    #[error("stored setting JSON is invalid: {0}")]
    Json(#[from] serde_json::Error),
    #[error("database migration failed: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, time::Duration};

    use nodecontroll_domain::{
        ClientCompatibilityMode, EntityId, ExternalSyncStrategy, Instance, InstanceName,
        PasswordHash, PrincipalLabel, Revision, SubscriptionBehaviorSettings, UserAccount,
        UserRole, Username,
    };
    use nodecontroll_secrets::{EnvelopeCipher, Keyring, SecretBinding};
    use sqlx::{
        PgPool, SqlitePool,
        postgres::{PgConnectOptions, PgPoolOptions},
    };

    use super::{
        AuthLevel, AuthSessionStatus, BootstrapState, ConnectionSettings, Database, DatabaseEngine,
        JSON_SAFE_INTEGER_MAX_I64, LoginAttemptReservation, LoginRateDecision, LoginSecurityReason,
        NewAuthSession, NewLoginSecurityEvent, NewRecoveryCode, NewRecoveryCodeSet,
        NewSecretRecord, PasswordChangeRotation, PersistenceError, RecoveryCodeConsumption,
        RecoveryCodeReplacement, SessionAuthentication, SessionAuthenticationOutcome,
        SessionRevocationReason, UserCredentials, UserSessionRevocation,
        decode_recovery_code_summary, validate_recovery_code_set,
    };

    fn settings() -> ConnectionSettings {
        ConnectionSettings {
            max_connections: 4,
            acquire_timeout: Duration::from_secs(5),
            statement_timeout: Duration::from_secs(30),
            lock_timeout: Duration::from_secs(5),
        }
    }

    #[test]
    fn recovery_summary_is_bounded_to_the_json_integer_contract() {
        assert!(
            validate_recovery_code_set(&recovery_set_fixture(JSON_SAFE_INTEGER_MAX_I64, 1,))
                .is_ok()
        );
        assert!(matches!(
            validate_recovery_code_set(&recovery_set_fixture(JSON_SAFE_INTEGER_MAX_I64 + 1, 2,)),
            Err(PersistenceError::InvalidRecoveryCodeSet)
        ));
        assert!(matches!(
            decode_recovery_code_summary((1, 8, 8, 0)),
            Ok(summary)
                if summary.set_version == 1
                    && summary.total_count == 8
                    && summary.remaining_count == 8
                    && summary.created_at_ms == 0
        ));
        assert!(matches!(
            decode_recovery_code_summary((
                JSON_SAFE_INTEGER_MAX_I64,
                8,
                0,
                JSON_SAFE_INTEGER_MAX_I64,
            )),
            Ok(summary)
                if summary.set_version == JSON_SAFE_INTEGER_MAX_I64 as u64
                    && summary.created_at_ms == JSON_SAFE_INTEGER_MAX_I64
        ));
        for row in [
            (0, 8, 8, 0),
            (JSON_SAFE_INTEGER_MAX_I64 + 1, 8, 8, 0),
            (1, 8, 8, -1),
            (1, 8, 8, JSON_SAFE_INTEGER_MAX_I64 + 1),
            (1, 7, 7, 0),
            (1, 8, 9, 0),
        ] {
            assert!(matches!(
                decode_recovery_code_summary(row),
                Err(PersistenceError::InvalidRecoveryCodeSet)
            ));
        }
    }

    struct PostgresFixture {
        database: Database,
        admin: PgPool,
        schema: &'static str,
        url: String,
    }

    async fn isolated_postgres(
        url: &str,
        schema: &'static str,
    ) -> Result<PostgresFixture, sqlx::Error> {
        let admin = PgPoolOptions::new().max_connections(1).connect(url).await?;
        match schema {
            "nodecontroll_test_fresh" => {
                sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_fresh CASCADE")
                    .execute(&admin)
                    .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_fresh")
                    .execute(&admin)
                    .await?;
            }
            "nodecontroll_test_legacy" => {
                sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_legacy CASCADE")
                    .execute(&admin)
                    .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_legacy")
                    .execute(&admin)
                    .await?;
            }
            "nodecontroll_test_legacy_missing" => {
                sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_legacy_missing CASCADE")
                    .execute(&admin)
                    .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_legacy_missing")
                    .execute(&admin)
                    .await?;
            }
            "nodecontroll_test_migration_rollback" => {
                sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_migration_rollback CASCADE")
                    .execute(&admin)
                    .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_migration_rollback")
                    .execute(&admin)
                    .await?;
            }
            "nodecontroll_test_auth_upgrade" => {
                sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_auth_upgrade CASCADE")
                    .execute(&admin)
                    .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_auth_upgrade")
                    .execute(&admin)
                    .await?;
            }
            "nodecontroll_test_session_timeline_upgrade" => {
                sqlx::query(
                    "DROP SCHEMA IF EXISTS nodecontroll_test_session_timeline_upgrade CASCADE",
                )
                .execute(&admin)
                .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_session_timeline_upgrade")
                    .execute(&admin)
                    .await?;
            }
            "nodecontroll_test_auth_rollback" => {
                sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_auth_rollback CASCADE")
                    .execute(&admin)
                    .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_auth_rollback")
                    .execute(&admin)
                    .await?;
            }
            "nodecontroll_test_recent_auth_migration_rollback" => {
                sqlx::query(
                    "DROP SCHEMA IF EXISTS nodecontroll_test_recent_auth_migration_rollback CASCADE",
                )
                .execute(&admin)
                .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_recent_auth_migration_rollback")
                    .execute(&admin)
                    .await?;
            }
            "nodecontroll_test_session_timeline_migration_rollback" => {
                sqlx::query(
                    "DROP SCHEMA IF EXISTS nodecontroll_test_session_timeline_migration_rollback CASCADE",
                )
                .execute(&admin)
                .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_session_timeline_migration_rollback")
                    .execute(&admin)
                    .await?;
            }
            _ => panic!("unexpected PostgreSQL fixture schema"),
        }
        let options = PgConnectOptions::from_str(url)?.options([
            ("search_path", schema),
            ("statement_timeout", "30s"),
            ("lock_timeout", "5s"),
        ]);
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect_with(options)
            .await?;
        Ok(PostgresFixture {
            database: Database::Postgres(pool),
            admin,
            schema,
            url: url.to_owned(),
        })
    }

    impl PostgresFixture {
        async fn reconnect_database(&self) -> Result<Database, sqlx::Error> {
            let options = PgConnectOptions::from_str(&self.url)?.options([
                ("search_path", self.schema),
                ("statement_timeout", "30s"),
                ("lock_timeout", "5s"),
            ]);
            let pool = PgPoolOptions::new()
                .max_connections(4)
                .connect_with(options)
                .await?;
            Ok(Database::Postgres(pool))
        }

        async fn cleanup(self) -> Result<(), sqlx::Error> {
            if let Database::Postgres(pool) = self.database {
                pool.close().await;
            }
            match self.schema {
                "nodecontroll_test_fresh" => {
                    sqlx::query("DROP SCHEMA nodecontroll_test_fresh CASCADE")
                        .execute(&self.admin)
                        .await?;
                }
                "nodecontroll_test_legacy" => {
                    sqlx::query("DROP SCHEMA nodecontroll_test_legacy CASCADE")
                        .execute(&self.admin)
                        .await?;
                }
                "nodecontroll_test_legacy_missing" => {
                    sqlx::query("DROP SCHEMA nodecontroll_test_legacy_missing CASCADE")
                        .execute(&self.admin)
                        .await?;
                }
                "nodecontroll_test_migration_rollback" => {
                    sqlx::query("DROP SCHEMA nodecontroll_test_migration_rollback CASCADE")
                        .execute(&self.admin)
                        .await?;
                }
                "nodecontroll_test_auth_upgrade" => {
                    sqlx::query("DROP SCHEMA nodecontroll_test_auth_upgrade CASCADE")
                        .execute(&self.admin)
                        .await?;
                }
                "nodecontroll_test_session_timeline_upgrade" => {
                    sqlx::query("DROP SCHEMA nodecontroll_test_session_timeline_upgrade CASCADE")
                        .execute(&self.admin)
                        .await?;
                }
                "nodecontroll_test_auth_rollback" => {
                    sqlx::query("DROP SCHEMA nodecontroll_test_auth_rollback CASCADE")
                        .execute(&self.admin)
                        .await?;
                }
                "nodecontroll_test_recent_auth_migration_rollback" => {
                    sqlx::query(
                        "DROP SCHEMA nodecontroll_test_recent_auth_migration_rollback CASCADE",
                    )
                    .execute(&self.admin)
                    .await?;
                }
                "nodecontroll_test_session_timeline_migration_rollback" => {
                    sqlx::query(
                        "DROP SCHEMA nodecontroll_test_session_timeline_migration_rollback CASCADE",
                    )
                    .execute(&self.admin)
                    .await?;
                }
                _ => panic!("unexpected PostgreSQL fixture schema"),
            }
            self.admin.close().await;
            Ok(())
        }
    }

    fn fixture() -> Instance {
        let name = InstanceName::parse("Foundation contract");
        assert!(name.is_ok());
        Instance {
            id: EntityId::new(),
            public_id: EntityId::new(),
            name: name.unwrap_or_else(|_| unreachable!("checked above")),
            created_at_ms: 1_777_777_777_000,
            revision: Revision::initial(),
        }
    }

    fn owner_fixture() -> UserAccount {
        let username = Username::parse("InitialOwner");
        let password_hash = PasswordHash::parse(
            "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        );
        let principal_label = PrincipalLabel::parse(format!("owner-{}", EntityId::new()));
        assert!(username.is_ok());
        assert!(password_hash.is_ok());
        assert!(principal_label.is_ok());
        UserAccount {
            id: EntityId::new(),
            username: username.unwrap_or_else(|_| unreachable!("checked above")),
            password_hash: password_hash.unwrap_or_else(|_| unreachable!("checked above")),
            role: UserRole::Owner,
            principal_label: principal_label.unwrap_or_else(|_| unreachable!("checked above")),
            force_password_change: false,
            revision: Revision::initial(),
            created_at_ms: 1_777_777_777_000,
        }
    }

    fn recovery_set_fixture(created_at_ms: i64, marker: u8) -> NewRecoveryCodeSet {
        NewRecoveryCodeSet {
            created_at_ms,
            codes: (0_u8..8)
                .map(|index| NewRecoveryCode {
                    id: EntityId::new(),
                    digest_key_version: 1,
                    code_hmac: [marker.saturating_add(index); 32],
                })
                .collect(),
        }
    }

    fn member_fixture(marker: u8) -> UserAccount {
        let username = Username::parse(format!("Member{marker}"));
        let password_hash = PasswordHash::parse(
            "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        );
        let principal_label = PrincipalLabel::parse(format!("member-{}", EntityId::new()));
        assert!(username.is_ok());
        assert!(password_hash.is_ok());
        assert!(principal_label.is_ok());
        UserAccount {
            id: EntityId::new(),
            username: username.unwrap_or_else(|_| unreachable!("checked above")),
            password_hash: password_hash.unwrap_or_else(|_| unreachable!("checked above")),
            role: UserRole::Member,
            principal_label: principal_label.unwrap_or_else(|_| unreachable!("checked above")),
            force_password_change: false,
            revision: Revision::initial(),
            created_at_ms: 1_777_777_780_000 + i64::from(marker) * 10,
        }
    }

    async fn insert_auth_user(database: &Database, user: &UserAccount) -> Result<(), sqlx::Error> {
        match database {
            Database::Sqlite(pool) => {
                let mut transaction = pool.begin().await?;
                sqlx::query(
                    "INSERT INTO users (id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms) VALUES (?,?,?,?,?,'active',?,?,0,?,NULL)",
                )
                .bind(user.id.to_string())
                .bind(user.username.as_str())
                .bind(user.username.normalized())
                .bind(user.password_hash.as_str())
                .bind(user.role.as_str())
                .bind(user.principal_label.as_str())
                .bind(user.force_password_change)
                .bind(user.created_at_ms)
                .execute(&mut *transaction)
                .await?;
                sqlx::query(
                    "INSERT INTO user_auth_state (user_id,auth_revision,password_changed_at_ms,updated_at_ms) VALUES (?,0,?,?)",
                )
                .bind(user.id.to_string())
                .bind(user.created_at_ms)
                .bind(user.created_at_ms)
                .execute(&mut *transaction)
                .await?;
                transaction.commit().await
            }
            Database::Postgres(pool) => {
                let mut transaction = pool.begin().await?;
                sqlx::query(
                    "INSERT INTO users (id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms) VALUES ($1,$2,$3,$4,$5,'active',$6,$7,0,$8,NULL)",
                )
                .bind(user.id.into_uuid())
                .bind(user.username.as_str())
                .bind(user.username.normalized())
                .bind(user.password_hash.as_str())
                .bind(user.role.as_str())
                .bind(user.principal_label.as_str())
                .bind(user.force_password_change)
                .bind(user.created_at_ms)
                .execute(&mut *transaction)
                .await?;
                sqlx::query(
                    "INSERT INTO user_auth_state (user_id,auth_revision,password_changed_at_ms,updated_at_ms) VALUES ($1,0,$2,$2)",
                )
                .bind(user.id.into_uuid())
                .bind(user.created_at_ms)
                .execute(&mut *transaction)
                .await?;
                transaction.commit().await
            }
        }
    }

    fn password_hash_fixture(output_prefix: char) -> PasswordHash {
        let output = format!("{output_prefix}{}", "A".repeat(42));
        let password_hash = PasswordHash::parse(format!(
            "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA${output}"
        ));
        assert!(password_hash.is_ok());
        password_hash.unwrap_or_else(|_| unreachable!("checked above"))
    }

    fn auth_session_fixture(
        user_id: EntityId,
        marker: u8,
        auth_revision: Revision,
        created_at_ms: i64,
        absolute_expires_at_ms: i64,
    ) -> NewAuthSession {
        NewAuthSession {
            id: EntityId::new(),
            user_id,
            token_key_version: 1,
            token_hmac: [marker; 32],
            csrf_key_version: 1,
            csrf_hmac: [marker.saturating_add(64); 32],
            auth_revision,
            auth_level: AuthLevel::Password,
            created_at_ms,
            authenticated_at_ms: created_at_ms,
            recent_auth_at_ms: created_at_ms,
            last_seen_at_ms: created_at_ms,
            idle_expires_at_ms: (created_at_ms + 1_000).min(absolute_expires_at_ms),
            absolute_expires_at_ms,
            ip_prefix_key_version: Some(1),
            ip_prefix_hmac: Some([marker.saturating_add(32); 32]),
            user_agent_hash: Some([marker.saturating_add(48); 32]),
            revision: Revision::initial(),
        }
    }

    fn login_security_event(
        marker: u8,
        occurred_at_ms: i64,
        reason: LoginSecurityReason,
    ) -> NewLoginSecurityEvent {
        NewLoginSecurityEvent {
            id: EntityId::new(),
            occurred_at_ms,
            request_id: format!("01900000-0000-7000-8000-{marker:012}"),
            reason,
            digest_key_version: 1,
            account_hmac: Some([marker.saturating_add(16); 32]),
            ip_prefix_hmac: Some([marker.saturating_add(32); 32]),
            user_agent_hash: Some([marker.saturating_add(48); 32]),
        }
    }

    fn rate_limited_event(reservation: &LoginAttemptReservation) -> NewLoginSecurityEvent {
        NewLoginSecurityEvent {
            id: EntityId::new(),
            occurred_at_ms: reservation.now_ms,
            request_id: reservation.request_id.clone(),
            reason: LoginSecurityReason::RateLimited,
            digest_key_version: reservation.key_version,
            account_hmac: Some(reservation.account_hmac),
            ip_prefix_hmac: Some(reservation.ip_prefix_hmac),
            user_agent_hash: Some(reservation.user_agent_hash),
        }
    }

    fn session_security_event(
        session: &NewAuthSession,
        marker: u8,
        occurred_at_ms: i64,
        reason: LoginSecurityReason,
    ) -> NewLoginSecurityEvent {
        let mut event = login_security_event(marker, occurred_at_ms, reason);
        event.digest_key_version = session.ip_prefix_key_version.unwrap_or(1);
        event.ip_prefix_hmac = session.ip_prefix_hmac;
        event.user_agent_hash = session.user_agent_hash;
        event
    }

    fn session_authentication(
        session: &NewAuthSession,
        csrf_hmac: Option<[u8; 32]>,
        now_ms: i64,
    ) -> SessionAuthentication {
        SessionAuthentication {
            token_key_version: session.token_key_version,
            token_hmac: session.token_hmac,
            csrf_key_version: csrf_hmac.map(|_| session.csrf_key_version),
            csrf_hmac,
            now_ms,
            touch_interval_ms: 50,
            idle_timeout_ms: 1_000,
        }
    }

    async fn provision_actor_and_target(
        database: &Database,
        user_marker: u8,
        actor_marker: u8,
        target_marker: u8,
    ) -> (
        UserAccount,
        NewAuthSession,
        NewAuthSession,
        super::AuthenticatedSession,
    ) {
        let user = member_fixture(user_marker);
        assert!(insert_auth_user(database, &user).await.is_ok());
        let created_at_ms = user.created_at_ms + 100;
        let absolute_expires_at_ms = created_at_ms + 10_000;
        let actor = auth_session_fixture(
            user.id,
            actor_marker,
            Revision::initial(),
            created_at_ms,
            absolute_expires_at_ms,
        );
        let target = auth_session_fixture(
            user.id,
            target_marker,
            Revision::initial(),
            created_at_ms + 1,
            absolute_expires_at_ms,
        );
        assert!(
            database
                .create_auth_session(
                    &actor,
                    &login_security_event(
                        actor_marker,
                        actor.created_at_ms,
                        LoginSecurityReason::LoginSucceeded,
                    ),
                )
                .await
                .is_ok()
        );
        assert!(
            database
                .create_auth_session(
                    &target,
                    &login_security_event(
                        target_marker,
                        target.created_at_ms,
                        LoginSecurityReason::LoginSucceeded,
                    ),
                )
                .await
                .is_ok()
        );
        let authenticated = match database
            .authenticate_session_read_only(&session_authentication(
                &actor,
                Some(actor.csrf_hmac),
                created_at_ms + 2,
            ))
            .await
        {
            Ok(SessionAuthenticationOutcome::Authenticated(authenticated)) => authenticated,
            outcome => panic!("unexpected actor authentication outcome: {outcome:?}"),
        };
        (user, actor, target, authenticated)
    }

    fn user_session_revocation<'a>(
        authenticated: &super::AuthenticatedSession,
        target_session_id: EntityId,
        event: &'a NewLoginSecurityEvent,
        now_ms: i64,
    ) -> UserSessionRevocation<'a> {
        UserSessionRevocation {
            user_id: authenticated.user_id,
            actor_session_id: authenticated.session.id,
            target_session_id,
            expected_user_revision: authenticated.user_revision,
            expected_auth_revision: authenticated.session.auth_revision,
            expected_recent_auth_at_ms: authenticated.session.recent_auth_at_ms,
            event,
            now_ms,
        }
    }

    async fn stored_session_is_active(
        database: &Database,
        user_id: EntityId,
        session_id: EntityId,
    ) -> bool {
        matches!(
            database.list_user_sessions(user_id).await,
            Ok(ref sessions)
                if sessions.iter().any(|session|
                    session.id == session_id
                        && session.status == AuthSessionStatus::Active
                        && session.revoked_at_ms.is_none()
                        && session.revoked_reason.is_none()
                )
        )
    }

    async fn set_user_status_and_revision(
        database: &Database,
        user_id: EntityId,
        status: &str,
        revision: i64,
    ) -> Result<u64, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => {
                sqlx::query("UPDATE users SET status=?,revision=? WHERE id=?")
                    .bind(status)
                    .bind(revision)
                    .bind(user_id.to_string())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
            Database::Postgres(pool) => {
                sqlx::query("UPDATE users SET status=$1,revision=$2 WHERE id=$3")
                    .bind(status)
                    .bind(revision)
                    .bind(user_id.into_uuid())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
        }
    }

    fn password_change_rotation<'a>(
        user_id: EntityId,
        current_session_id: EntityId,
        expected_user_revision: Revision,
        new_hash: &'a PasswordHash,
        replacement: &'a NewAuthSession,
        event: &'a NewLoginSecurityEvent,
        now_ms: i64,
    ) -> PasswordChangeRotation<'a> {
        PasswordChangeRotation {
            user_id,
            current_session_id,
            expected_user_revision,
            new_hash,
            replacement,
            event,
            now_ms,
        }
    }

    fn persisted_authentication_fixture(marker: u8) -> SessionAuthentication {
        SessionAuthentication {
            token_key_version: 1,
            token_hmac: [marker; 32],
            csrf_key_version: None,
            csrf_hmac: None,
            now_ms: 1_777_777_777_506,
            touch_interval_ms: 50,
            idle_timeout_ms: 1_000,
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
    struct LoginSecurityEventSnapshot {
        id: String,
        occurred_at_ms: i64,
        request_id: String,
        reason: String,
        digest_key_version: i64,
        account_hmac: Option<Vec<u8>>,
        ip_prefix_hmac: Option<Vec<u8>>,
        user_agent_hash: Option<Vec<u8>>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
    struct AuthSessionSnapshot {
        id: String,
        user_id: String,
        token_key_version: i64,
        token_hmac: Vec<u8>,
        csrf_key_version: i64,
        csrf_hmac: Vec<u8>,
        auth_revision: i64,
        auth_level: String,
        status: String,
        created_at_ms: i64,
        authenticated_at_ms: i64,
        recent_auth_at_ms: i64,
        last_seen_at_ms: i64,
        idle_expires_at_ms: i64,
        absolute_expires_at_ms: i64,
        ip_prefix_key_version: Option<i64>,
        ip_prefix_hmac: Option<Vec<u8>>,
        user_agent_hash: Option<Vec<u8>>,
        revoked_at_ms: Option<i64>,
        revoked_reason: Option<String>,
        revision: i64,
    }

    #[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
    struct IndexSnapshot {
        name: String,
        definition: String,
    }

    #[derive(Clone, Debug, PartialEq, Eq, sqlx::FromRow)]
    struct PostgresConstraintSnapshot {
        name: String,
        definition: String,
        validated: bool,
    }

    async fn login_security_event_snapshots(
        database: &Database,
    ) -> Result<Vec<LoginSecurityEventSnapshot>, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => sqlx::query_as(
                "SELECT id,occurred_at_ms,request_id,reason,digest_key_version,account_hmac,ip_prefix_hmac,user_agent_hash FROM login_security_events ORDER BY id",
            )
            .fetch_all(pool)
            .await,
            Database::Postgres(pool) => sqlx::query_as(
                "SELECT id::text AS id,occurred_at_ms,request_id,reason,digest_key_version::BIGINT AS digest_key_version,account_hmac,ip_prefix_hmac,user_agent_hash FROM login_security_events ORDER BY id",
            )
            .fetch_all(pool)
            .await,
        }
    }

    async fn auth_session_snapshots(
        database: &Database,
    ) -> Result<Vec<AuthSessionSnapshot>, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => sqlx::query_as(
                "SELECT id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision FROM auth_sessions ORDER BY id",
            )
            .fetch_all(pool)
            .await,
            Database::Postgres(pool) => sqlx::query_as(
                "SELECT id::text AS id,user_id::text AS user_id,token_key_version::BIGINT AS token_key_version,token_hmac,csrf_key_version::BIGINT AS csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version::BIGINT AS ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision FROM auth_sessions ORDER BY id",
            )
            .fetch_all(pool)
            .await,
        }
    }

    async fn sqlite_explicit_index_snapshots(
        pool: &SqlitePool,
        table: &str,
    ) -> Result<Vec<IndexSnapshot>, sqlx::Error> {
        sqlx::query_as(
            "SELECT name,sql AS definition FROM sqlite_master WHERE type='index' AND tbl_name=? AND sql IS NOT NULL ORDER BY name",
        )
        .bind(table)
        .fetch_all(pool)
        .await
    }

    async fn sqlite_table_definition(
        pool: &SqlitePool,
        table: &str,
    ) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT sql FROM sqlite_master WHERE type='table' AND name=? AND sql IS NOT NULL",
        )
        .bind(table)
        .fetch_optional(pool)
        .await
    }

    async fn postgres_index_snapshots(
        pool: &PgPool,
        table: &str,
    ) -> Result<Vec<IndexSnapshot>, sqlx::Error> {
        sqlx::query_as(
            "SELECT indexname AS name,indexdef AS definition FROM pg_indexes WHERE schemaname=current_schema() AND tablename=$1 ORDER BY indexname",
        )
        .bind(table)
        .fetch_all(pool)
        .await
    }

    async fn postgres_constraint_snapshots(
        pool: &PgPool,
        table: &str,
    ) -> Result<Vec<PostgresConstraintSnapshot>, sqlx::Error> {
        sqlx::query_as(
            "SELECT conname AS name,pg_get_constraintdef(oid) AS definition,convalidated AS validated FROM pg_constraint WHERE conrelid=to_regclass($1) ORDER BY conname",
        )
        .bind(table)
        .fetch_all(pool)
        .await
    }

    async fn migration_version(database: &Database) -> Result<Option<i64>, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => {
                sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations")
                    .fetch_one(pool)
                    .await
            }
            Database::Postgres(pool) => {
                sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations")
                    .fetch_one(pool)
                    .await
            }
        }
    }

    async fn migrate_to_0001(database: &Database) {
        let result = match database {
            Database::Sqlite(pool) => super::SQLITE_MIGRATOR.run_to(1, pool).await,
            Database::Postgres(pool) => super::POSTGRES_MIGRATOR.run_to(1, pool).await,
        };
        assert!(result.is_ok());
    }

    async fn migrate_to_0002(database: &Database) {
        let result = match database {
            Database::Sqlite(pool) => super::SQLITE_MIGRATOR.run_to(2, pool).await,
            Database::Postgres(pool) => super::POSTGRES_MIGRATOR.run_to(2, pool).await,
        };
        assert!(result.is_ok());
    }

    async fn migrate_to_0003(database: &Database) {
        let result = match database {
            Database::Sqlite(pool) => super::SQLITE_MIGRATOR.run_to(3, pool).await,
            Database::Postgres(pool) => super::POSTGRES_MIGRATOR.run_to(3, pool).await,
        };
        assert!(result.is_ok());
    }

    async fn migrate_to_0004(database: &Database) {
        let result = match database {
            Database::Sqlite(pool) => super::SQLITE_MIGRATOR.run_to(4, pool).await,
            Database::Postgres(pool) => super::POSTGRES_MIGRATOR.run_to(4, pool).await,
        };
        assert!(result.is_ok());
    }

    async fn migrate_to_0005(database: &Database) {
        let result = match database {
            Database::Sqlite(pool) => super::SQLITE_MIGRATOR.run_to(5, pool).await,
            Database::Postgres(pool) => super::POSTGRES_MIGRATOR.run_to(5, pool).await,
        };
        assert!(result.is_ok());
    }

    async fn seed_v1_instance(
        database: &Database,
        instance: &Instance,
        settings: &SubscriptionBehaviorSettings,
    ) {
        let settings_json = serde_json::to_string(settings);
        assert!(settings_json.is_ok());
        let settings_json = settings_json.unwrap_or_else(|_| unreachable!("checked above"));
        match database {
            Database::Sqlite(pool) => {
                let inserted = sqlx::query(
                    "INSERT INTO instances (singleton_key,id,name,public_id,created_at_ms,revision) VALUES (1,?,?,?,?,0)",
                )
                .bind(instance.id.to_string())
                .bind(instance.name.as_str())
                .bind(instance.public_id.to_string())
                .bind(instance.created_at_ms)
                .execute(pool)
                .await;
                assert!(inserted.is_ok());
                let settings_inserted = sqlx::query(
                    "INSERT INTO instance_settings (instance_id,key,schema_version,value_json,revision,updated_by,updated_at_ms) VALUES (?,?,?,?,0,NULL,?)",
                )
                .bind(instance.id.to_string())
                .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                .bind(super::SUBSCRIPTION_SETTINGS_SCHEMA)
                .bind(settings_json)
                .bind(instance.created_at_ms)
                .execute(pool)
                .await;
                assert!(settings_inserted.is_ok());
            }
            Database::Postgres(pool) => {
                let inserted = sqlx::query(
                    "INSERT INTO instances (singleton_key,id,name,public_id,created_at_ms,revision) VALUES (1,$1,$2,$3,$4,0)",
                )
                .bind(instance.id.into_uuid())
                .bind(instance.name.as_str())
                .bind(instance.public_id.into_uuid())
                .bind(instance.created_at_ms)
                .execute(pool)
                .await;
                assert!(inserted.is_ok());
                let settings_inserted = sqlx::query(
                    "INSERT INTO instance_settings (instance_id,key,schema_version,value_json,revision,updated_by,updated_at_ms) VALUES ($1,$2,$3,$4::jsonb,0,NULL,$5)",
                )
                .bind(instance.id.into_uuid())
                .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                .bind(super::SUBSCRIPTION_SETTINGS_SCHEMA)
                .bind(settings_json)
                .bind(instance.created_at_ms)
                .execute(pool)
                .await;
                assert!(settings_inserted.is_ok());
            }
        }
    }

    async fn seed_v2_user(database: &Database, user: &UserAccount) {
        let result = match database {
            Database::Sqlite(pool) => sqlx::query(
                "INSERT INTO users (id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms) VALUES (?,?,?,?,?,'active',?,?,0,?,NULL)",
            )
            .bind(user.id.to_string())
            .bind(user.username.as_str())
            .bind(user.username.normalized())
            .bind(user.password_hash.as_str())
            .bind(user.role.as_str())
            .bind(user.principal_label.as_str())
            .bind(user.force_password_change)
            .bind(user.created_at_ms)
            .execute(pool)
            .await
            .map(|_| ()),
            Database::Postgres(pool) => sqlx::query(
                "INSERT INTO users (id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms) VALUES ($1,$2,$3,$4,$5,'active',$6,$7,0,$8,NULL)",
            )
            .bind(user.id.into_uuid())
            .bind(user.username.as_str())
            .bind(user.username.normalized())
            .bind(user.password_hash.as_str())
            .bind(user.role.as_str())
            .bind(user.principal_label.as_str())
            .bind(user.force_password_change)
            .bind(user.created_at_ms)
            .execute(pool)
            .await
            .map(|_| ()),
        };
        assert!(result.is_ok());
    }

    async fn auth_upgrade_contract(database: Database) {
        migrate_to_0002(&database).await;
        let user = owner_fixture();
        seed_v2_user(&database, &user).await;
        assert!(database.migrate().await.is_ok());
        let state: Result<(i64, i64, i64), sqlx::Error> = match &database {
            Database::Sqlite(pool) => sqlx::query_as(
                "SELECT auth_revision,password_changed_at_ms,updated_at_ms FROM user_auth_state WHERE user_id=?",
            )
            .bind(user.id.to_string())
            .fetch_one(pool)
            .await,
            Database::Postgres(pool) => sqlx::query_as(
                "SELECT auth_revision,password_changed_at_ms,updated_at_ms FROM user_auth_state WHERE user_id=$1",
            )
            .bind(user.id.into_uuid())
            .fetch_one(pool)
            .await,
        };
        assert!(matches!(
            state,
            Ok((0, password_changed_at_ms, updated_at_ms))
                if password_changed_at_ms == user.created_at_ms
                    && updated_at_ms == user.created_at_ms
        ));

        let invalid_timeline = match &database {
            Database::Sqlite(pool) => sqlx::query(
                "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES (?,?,1,?,1,?,0,'password','active',100,-1,100,100,200,300,NULL,NULL,NULL,NULL,NULL,0)",
            )
            .bind(EntityId::new().to_string())
            .bind(user.id.to_string())
            .bind(vec![1_u8; 32])
            .bind(vec![2_u8; 32])
            .execute(pool)
            .await
            .map(|_| ()),
            Database::Postgres(pool) => sqlx::query(
                "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES ($1,$2,1,$3,1,$4,0,'password','active',100,-1,100,100,200,300,NULL,NULL,NULL,NULL,NULL,0)",
            )
            .bind(EntityId::new().into_uuid())
            .bind(user.id.into_uuid())
            .bind(vec![1_u8; 32])
            .bind(vec![2_u8; 32])
            .execute(pool)
            .await
            .map(|_| ()),
        };
        assert!(invalid_timeline.is_err());
    }

    async fn session_timeline_upgrade_contract(database: Database) {
        migrate_to_0002(&database).await;
        let user = owner_fixture();
        seed_v2_user(&database, &user).await;
        migrate_to_0004(&database).await;

        let base = user.created_at_ms + 100;
        let historical =
            auth_session_fixture(user.id, 40, Revision::initial(), base, base + 10_000);
        let historical_event = login_security_event(40, base, LoginSecurityReason::LoginSucceeded);
        assert!(
            database
                .create_auth_session(&historical, &historical_event)
                .await
                .is_ok()
        );
        let before = database.list_user_sessions(user.id).await;
        let before = match before {
            Ok(mut sessions) if sessions.len() == 1 => sessions.remove(0),
            outcome => panic!("unexpected pre-0005 session history: {outcome:?}"),
        };
        let legacy_auth_level = match &database {
            Database::Sqlite(pool) => {
                sqlx::query("UPDATE auth_sessions SET auth_level='webauthn' WHERE id=?")
                    .bind(historical.id.to_string())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
            Database::Postgres(pool) => {
                sqlx::query("UPDATE auth_sessions SET auth_level='webauthn' WHERE id=$1")
                    .bind(historical.id.into_uuid())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
        };
        assert!(matches!(legacy_auth_level, Ok(1)));

        assert!(database.migrate().await.is_ok());
        let mut expected = before;
        expected.auth_level = AuthLevel::PhishingResistant;
        assert!(matches!(
            database.list_user_sessions(user.id).await,
            Ok(ref sessions) if sessions.first() == Some(&expected) && sessions.len() == 1
        ));
        assert!(matches!(
            login_security_event_count(&database, historical_event.id).await,
            Ok(1)
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(&historical, None, base + 1))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                if authenticated.session == expected
        ));
        raw_auth_secret_absence_contract(&database, &historical).await;
        auth_schema_constraints_contract(&database, &user, &historical).await;

        let mut inherited_proof =
            auth_session_fixture(user.id, 41, Revision::initial(), base + 200, base + 10_000);
        inherited_proof.auth_level = AuthLevel::PhishingResistant;
        inherited_proof.authenticated_at_ms = historical.authenticated_at_ms;
        inherited_proof.recent_auth_at_ms = historical.recent_auth_at_ms;
        let inherited_event = login_security_event(
            41,
            inherited_proof.created_at_ms,
            LoginSecurityReason::LoginSucceeded,
        );
        assert!(matches!(
            database
                .create_auth_session(&inherited_proof, &inherited_event)
                .await,
            Ok(ref summary)
                if summary.created_at_ms == base + 200
                    && summary.authenticated_at_ms == historical.authenticated_at_ms
                    && summary.recent_auth_at_ms == historical.recent_auth_at_ms
                    && summary.auth_level == AuthLevel::PhishingResistant
        ));

        let mut duplicate_digest = inherited_proof.clone();
        duplicate_digest.id = EntityId::new();
        let duplicate_event = session_security_event(
            &duplicate_digest,
            42,
            duplicate_digest.created_at_ms,
            LoginSecurityReason::LoginSucceeded,
        );
        assert!(matches!(
            database
                .create_auth_session(&duplicate_digest, &duplicate_event)
                .await,
            Err(PersistenceError::Sql(_))
        ));
        assert!(matches!(
            login_security_event_count(&database, duplicate_event.id).await,
            Ok(0)
        ));
    }

    async fn sqlite_recent_auth_migration_rollback_contract(database: Database) {
        migrate_to_0003(&database).await;
        let first_event = login_security_event(
            90,
            1_777_777_790_000,
            LoginSecurityReason::InvalidCredentials,
        );
        let mut second_event =
            login_security_event(91, 1_777_777_790_001, LoginSecurityReason::RateLimited);
        second_event.account_hmac = None;
        second_event.ip_prefix_hmac = None;
        second_event.user_agent_hash = None;
        let Database::Sqlite(pool) = &database else {
            panic!("SQLite 0004 rollback contract requires SQLite")
        };
        assert!(
            sqlx::query(
                "INSERT INTO login_security_events (id,occurred_at_ms,request_id,reason,digest_key_version,account_hmac,ip_prefix_hmac,user_agent_hash) VALUES (?,?,?,?,?,?,?,?)",
            )
                .bind(first_event.id.to_string())
                .bind(first_event.occurred_at_ms)
                .bind(&first_event.request_id)
                .bind(first_event.reason.as_str())
                .bind(i64::from(first_event.digest_key_version))
                .bind(first_event.account_hmac.as_ref().map(|value| value.as_slice()))
                .bind(first_event.ip_prefix_hmac.as_ref().map(|value| value.as_slice()))
                .bind(first_event.user_agent_hash.as_ref().map(|value| value.as_slice()))
                .execute(pool)
                .await
                .is_ok()
        );
        assert!(
            sqlx::query(
                "INSERT INTO login_security_events (id,occurred_at_ms,request_id,reason,digest_key_version,account_hmac,ip_prefix_hmac,user_agent_hash) VALUES (?,?,?,?,?,?,?,?)",
            )
                .bind(second_event.id.to_string())
                .bind(second_event.occurred_at_ms)
                .bind(&second_event.request_id)
                .bind(second_event.reason.as_str())
                .bind(i64::from(second_event.digest_key_version))
                .bind(second_event.account_hmac.as_ref().map(|value| value.as_slice()))
                .bind(second_event.ip_prefix_hmac.as_ref().map(|value| value.as_slice()))
                .bind(second_event.user_agent_hash.as_ref().map(|value| value.as_slice()))
                .execute(pool)
                .await
                .is_ok()
        );

        let before_events = login_security_event_snapshots(&database).await;
        assert!(matches!(&before_events, Ok(rows) if rows.len() == 2));
        let before_events = before_events
            .unwrap_or_else(|error| panic!("failed to snapshot pre-0004 login events: {error}"));
        let before_table = sqlite_table_definition(pool, "login_security_events").await;
        assert!(matches!(&before_table, Ok(Some(_))));
        let before_indexes = sqlite_explicit_index_snapshots(pool, "login_security_events").await;
        assert!(matches!(&before_indexes, Ok(indexes) if indexes.len() == 2));
        let before_indexes = before_indexes
            .unwrap_or_else(|error| panic!("failed to snapshot pre-0004 indexes: {error}"));
        assert_eq!(
            before_indexes
                .iter()
                .map(|index| index.name.as_str())
                .collect::<Vec<_>>(),
            vec![
                "login_security_events_account_idx",
                "login_security_events_time_idx",
            ]
        );

        assert!(
            sqlx::query(
                "CREATE TABLE nodecontroll_test_login_event_refs (event_id TEXT PRIMARY KEY REFERENCES login_security_events(id) ON DELETE RESTRICT)",
            )
            .execute(pool)
            .await
            .is_ok()
        );
        assert!(
            sqlx::query("INSERT INTO nodecontroll_test_login_event_refs (event_id) VALUES (?)")
                .bind(first_event.id.to_string())
                .execute(pool)
                .await
                .is_ok()
        );

        let migration = super::SQLITE_MIGRATOR.run_to(4, pool).await;
        let Err(error) = migration else {
            panic!("SQLite 0004 foreign-key fixture did not fail the migration")
        };
        assert!(error.to_string().contains("FOREIGN KEY"));
        assert!(matches!(migration_version(&database).await, Ok(Some(3))));
        assert_eq!(
            login_security_event_snapshots(&database).await.ok(),
            Some(before_events.clone())
        );
        assert_eq!(
            sqlite_table_definition(pool, "login_security_events")
                .await
                .ok(),
            before_table.ok()
        );
        assert_eq!(
            sqlite_explicit_index_snapshots(pool, "login_security_events")
                .await
                .ok(),
            Some(before_indexes.clone())
        );
        let staging_count: Result<i64, sqlx::Error> = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='login_security_events_new'",
        )
        .fetch_one(pool)
        .await;
        assert!(matches!(staging_count, Ok(0)));
        let fixture_rows: Result<i64, sqlx::Error> =
            sqlx::query_scalar("SELECT COUNT(*) FROM nodecontroll_test_login_event_refs")
                .fetch_one(pool)
                .await;
        assert!(matches!(fixture_rows, Ok(1)));

        assert!(
            sqlx::query("DROP TABLE nodecontroll_test_login_event_refs")
                .execute(pool)
                .await
                .is_ok()
        );
        assert!(super::SQLITE_MIGRATOR.run_to(4, pool).await.is_ok());
        assert!(matches!(migration_version(&database).await, Ok(Some(4))));
        assert_eq!(
            login_security_event_snapshots(&database).await.ok(),
            Some(before_events)
        );
        assert_eq!(
            sqlite_explicit_index_snapshots(pool, "login_security_events")
                .await
                .ok(),
            Some(before_indexes)
        );
        let account_index_rows: Result<Vec<String>, sqlx::Error> = sqlx::query_scalar(
            "SELECT id FROM login_security_events INDEXED BY login_security_events_account_idx WHERE account_hmac IS NOT NULL ORDER BY occurred_at_ms DESC,id",
        )
        .fetch_all(pool)
        .await;
        assert!(matches!(account_index_rows, Ok(ref ids) if ids == &[first_event.id.to_string()]));
        let time_index_rows: Result<Vec<String>, sqlx::Error> = sqlx::query_scalar(
            "SELECT id FROM login_security_events INDEXED BY login_security_events_time_idx ORDER BY occurred_at_ms DESC,id",
        )
        .fetch_all(pool)
        .await;
        assert!(matches!(time_index_rows, Ok(ref ids) if ids.len() == 2));
        let staging_count: Result<i64, sqlx::Error> = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='login_security_events_new'",
        )
        .fetch_one(pool)
        .await;
        assert!(matches!(staging_count, Ok(0)));
        let reauthentication_event = login_security_event(
            95,
            1_777_777_790_002,
            LoginSecurityReason::ReauthenticationSucceeded,
        );
        let password_changed_event =
            login_security_event(96, 1_777_777_790_003, LoginSecurityReason::PasswordChanged);
        assert!(
            database
                .record_login_security_event(&reauthentication_event)
                .await
                .is_ok()
        );
        assert!(
            database
                .record_login_security_event(&password_changed_event)
                .await
                .is_ok()
        );
        let invalid_reason = sqlx::query(
            "INSERT INTO login_security_events (id,occurred_at_ms,request_id,reason,digest_key_version,account_hmac,ip_prefix_hmac,user_agent_hash) VALUES (?,?,'01900000-0000-7000-8000-000000000097','nodecontroll_test_invalid',1,NULL,NULL,NULL)",
        )
        .bind(EntityId::new().to_string())
        .bind(1_777_777_790_004_i64)
        .execute(pool)
        .await;
        assert!(invalid_reason.is_err());
    }

    async fn sqlite_session_timeline_migration_rollback_contract(database: Database) {
        migrate_to_0002(&database).await;
        let user = owner_fixture();
        seed_v2_user(&database, &user).await;
        migrate_to_0004(&database).await;
        let base = user.created_at_ms + 700;
        let session = auth_session_fixture(user.id, 92, Revision::initial(), base, base + 10_000);
        let event = login_security_event(92, base, LoginSecurityReason::LoginSucceeded);
        assert!(database.create_auth_session(&session, &event).await.is_ok());

        let Database::Sqlite(pool) = &database else {
            panic!("SQLite 0005 rollback contract requires SQLite")
        };
        let legacy_auth_level =
            sqlx::query("UPDATE auth_sessions SET auth_level='webauthn' WHERE id=?")
                .bind(session.id.to_string())
                .execute(pool)
                .await;
        assert!(matches!(legacy_auth_level, Ok(result) if result.rows_affected() == 1));
        let before_sessions = auth_session_snapshots(&database).await;
        assert!(matches!(&before_sessions, Ok(rows) if rows.len() == 1));
        let before_sessions = before_sessions
            .unwrap_or_else(|error| panic!("failed to snapshot pre-0005 sessions: {error}"));
        let before_events = login_security_event_snapshots(&database).await;
        assert!(matches!(&before_events, Ok(rows) if rows.len() == 1));
        let before_events = before_events
            .unwrap_or_else(|error| panic!("failed to snapshot pre-0005 events: {error}"));
        let before_table = sqlite_table_definition(pool, "auth_sessions").await;
        assert!(matches!(&before_table, Ok(Some(_))));
        let before_indexes = sqlite_explicit_index_snapshots(pool, "auth_sessions").await;
        assert!(matches!(&before_indexes, Ok(indexes) if indexes.len() == 2));
        let before_indexes = before_indexes
            .unwrap_or_else(|error| panic!("failed to snapshot pre-0005 indexes: {error}"));
        assert_eq!(
            before_indexes
                .iter()
                .map(|index| index.name.as_str())
                .collect::<Vec<_>>(),
            vec![
                "auth_sessions_active_user_idx",
                "auth_sessions_user_created_idx",
            ]
        );

        assert!(
            sqlx::query(
                "CREATE TABLE nodecontroll_test_auth_session_refs (session_id TEXT PRIMARY KEY REFERENCES auth_sessions(id) ON DELETE RESTRICT)",
            )
            .execute(pool)
            .await
            .is_ok()
        );
        assert!(
            sqlx::query("INSERT INTO nodecontroll_test_auth_session_refs (session_id) VALUES (?)")
                .bind(session.id.to_string())
                .execute(pool)
                .await
                .is_ok()
        );

        let migration = super::SQLITE_MIGRATOR.run_to(5, pool).await;
        let Err(error) = migration else {
            panic!("SQLite 0005 foreign-key fixture did not fail the migration")
        };
        assert!(error.to_string().contains("FOREIGN KEY"));
        assert!(matches!(migration_version(&database).await, Ok(Some(4))));
        assert_eq!(
            auth_session_snapshots(&database).await.ok(),
            Some(before_sessions.clone())
        );
        assert_eq!(
            login_security_event_snapshots(&database).await.ok(),
            Some(before_events.clone())
        );
        assert_eq!(
            sqlite_table_definition(pool, "auth_sessions").await.ok(),
            before_table.ok()
        );
        assert_eq!(
            sqlite_explicit_index_snapshots(pool, "auth_sessions")
                .await
                .ok(),
            Some(before_indexes.clone())
        );
        let staging_count: Result<i64, sqlx::Error> = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='auth_sessions_new'",
        )
        .fetch_one(pool)
        .await;
        assert!(matches!(staging_count, Ok(0)));
        let fixture_rows: Result<i64, sqlx::Error> =
            sqlx::query_scalar("SELECT COUNT(*) FROM nodecontroll_test_auth_session_refs")
                .fetch_one(pool)
                .await;
        assert!(matches!(fixture_rows, Ok(1)));

        assert!(
            sqlx::query("DROP TABLE nodecontroll_test_auth_session_refs")
                .execute(pool)
                .await
                .is_ok()
        );
        assert!(super::SQLITE_MIGRATOR.run_to(5, pool).await.is_ok());
        assert!(matches!(migration_version(&database).await, Ok(Some(5))));
        let mut expected_sessions = before_sessions;
        assert_eq!(expected_sessions.len(), 1);
        if let Some(expected) = expected_sessions.first_mut() {
            expected.auth_level = "phishing_resistant".to_owned();
        }
        assert_eq!(
            auth_session_snapshots(&database).await.ok(),
            Some(expected_sessions)
        );
        assert_eq!(
            login_security_event_snapshots(&database).await.ok(),
            Some(before_events)
        );
        assert_eq!(
            sqlite_explicit_index_snapshots(pool, "auth_sessions")
                .await
                .ok(),
            Some(before_indexes)
        );
        let staging_count: Result<i64, sqlx::Error> = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='auth_sessions_new'",
        )
        .fetch_one(pool)
        .await;
        assert!(matches!(staging_count, Ok(0)));
    }

    async fn postgres_recent_auth_migration_rollback_contract(database: Database) {
        migrate_to_0003(&database).await;
        let valid_event = login_security_event(
            93,
            1_777_777_793_000,
            LoginSecurityReason::InvalidCredentials,
        );
        assert!(
            database
                .record_login_security_event(&valid_event)
                .await
                .is_ok()
        );
        let Database::Postgres(pool) = &database else {
            panic!("PostgreSQL 0004 rollback contract requires PostgreSQL")
        };
        let original_reason_definition: String = sqlx::query_scalar(
            "SELECT pg_get_constraintdef(oid) FROM pg_constraint WHERE conrelid=to_regclass('login_security_events') AND conname='login_security_events_reason_check'",
        )
        .fetch_one(pool)
        .await
        .unwrap_or_else(|error| panic!("failed to read the version-3 reason check: {error}"));
        assert!(
            sqlx::query(
                "ALTER TABLE login_security_events DROP CONSTRAINT login_security_events_reason_check",
            )
            .execute(pool)
            .await
            .is_ok()
        );
        let poison_id = EntityId::new();
        assert!(
            sqlx::query(
                "INSERT INTO login_security_events (id,occurred_at_ms,request_id,reason,digest_key_version,account_hmac,ip_prefix_hmac,user_agent_hash) VALUES ($1,$2,$3,'nodecontroll_test_invalid',1,NULL,NULL,NULL)",
            )
            .bind(poison_id.into_uuid())
            .bind(1_777_777_793_001_i64)
            .bind("01900000-0000-7000-8000-000000000094")
            .execute(pool)
            .await
            .is_ok()
        );
        let restore_old_constraint = format!(
            "ALTER TABLE login_security_events ADD CONSTRAINT login_security_events_reason_check {original_reason_definition} NOT VALID"
        );
        // Test-only catalog round-trip: the identifier is fixed above and the
        // definition comes from PostgreSQL's own pg_get_constraintdef output.
        assert!(
            sqlx::query(sqlx::AssertSqlSafe(restore_old_constraint))
                .execute(pool)
                .await
                .is_ok()
        );

        let before_events = login_security_event_snapshots(&database)
            .await
            .unwrap_or_else(|error| {
                panic!("failed to snapshot pre-0004 PostgreSQL events: {error}")
            });
        assert_eq!(before_events.len(), 2);
        let before_indexes = postgres_index_snapshots(pool, "login_security_events")
            .await
            .unwrap_or_else(|error| {
                panic!("failed to snapshot pre-0004 PostgreSQL indexes: {error}")
            });
        let before_constraints = postgres_constraint_snapshots(pool, "login_security_events")
            .await
            .unwrap_or_else(|error| {
                panic!("failed to snapshot pre-0004 PostgreSQL constraints: {error}")
            });
        assert!(before_constraints.iter().any(|constraint| {
            constraint.name == "login_security_events_reason_check" && !constraint.validated
        }));

        let migration = super::POSTGRES_MIGRATOR.run_to(4, pool).await;
        let Err(error) = migration else {
            panic!("PostgreSQL 0004 poison fixture did not fail the migration")
        };
        assert!(
            error
                .to_string()
                .contains("login_security_events_reason_check")
        );
        assert!(matches!(migration_version(&database).await, Ok(Some(3))));
        assert_eq!(
            login_security_event_snapshots(&database).await.ok(),
            Some(before_events.clone())
        );
        assert_eq!(
            postgres_index_snapshots(pool, "login_security_events")
                .await
                .ok(),
            Some(before_indexes.clone())
        );
        assert_eq!(
            postgres_constraint_snapshots(pool, "login_security_events")
                .await
                .ok(),
            Some(before_constraints)
        );

        let deleted_poison = sqlx::query("DELETE FROM login_security_events WHERE id=$1")
            .bind(poison_id.into_uuid())
            .execute(pool)
            .await;
        assert!(matches!(deleted_poison, Ok(result) if result.rows_affected() == 1));
        // A failed sqlx migration can leave its session-level advisory lock on
        // the pooled connection. Production retries happen in a fresh process;
        // closing and reconnecting here models that boundary and releases it.
        let retry_options = pool.connect_options().as_ref().clone();
        pool.close().await;
        let retry_pool = PgPoolOptions::new()
            .max_connections(4)
            .connect_with(retry_options)
            .await
            .unwrap_or_else(|error| {
                panic!("failed to reconnect for PostgreSQL 0004 retry: {error}")
            });
        let retry_database = Database::Postgres(retry_pool.clone());
        let retry = super::POSTGRES_MIGRATOR.run_to(4, &retry_pool).await;
        assert!(retry.is_ok(), "PostgreSQL 0004 retry failed: {retry:?}");
        assert!(matches!(
            migration_version(&retry_database).await,
            Ok(Some(4))
        ));
        let poison_id_text = poison_id.to_string();
        let expected_events = before_events
            .into_iter()
            .filter(|event| event.id != poison_id_text)
            .collect::<Vec<_>>();
        assert_eq!(expected_events.len(), 1);
        assert_eq!(
            login_security_event_snapshots(&retry_database).await.ok(),
            Some(expected_events)
        );
        assert_eq!(
            postgres_index_snapshots(&retry_pool, "login_security_events")
                .await
                .ok(),
            Some(before_indexes)
        );
        let after_constraints = postgres_constraint_snapshots(&retry_pool, "login_security_events")
            .await
            .unwrap_or_else(|error| {
                panic!("failed to snapshot post-0004 PostgreSQL constraints: {error}")
            });
        assert!(after_constraints.iter().any(|constraint| {
            constraint.name == "login_security_events_reason_check"
                && constraint.validated
                && constraint.definition.contains("reauthentication_succeeded")
                && constraint.definition.contains("password_changed")
        }));
        let reauthentication_event = login_security_event(
            97,
            1_777_777_793_002,
            LoginSecurityReason::ReauthenticationSucceeded,
        );
        let password_changed_event =
            login_security_event(98, 1_777_777_793_003, LoginSecurityReason::PasswordChanged);
        assert!(
            retry_database
                .record_login_security_event(&reauthentication_event)
                .await
                .is_ok()
        );
        assert!(
            retry_database
                .record_login_security_event(&password_changed_event)
                .await
                .is_ok()
        );
        let invalid_reason = sqlx::query(
            "INSERT INTO login_security_events (id,occurred_at_ms,request_id,reason,digest_key_version,account_hmac,ip_prefix_hmac,user_agent_hash) VALUES ($1,$2,'01900000-0000-7000-8000-000000000099','nodecontroll_test_invalid',1,NULL,NULL,NULL)",
        )
        .bind(EntityId::new().into_uuid())
        .bind(1_777_777_793_004_i64)
        .execute(&retry_pool)
        .await;
        assert!(invalid_reason.is_err());
        retry_pool.close().await;
    }

    async fn postgres_session_timeline_migration_rollback_contract(database: Database) {
        migrate_to_0002(&database).await;
        let user = owner_fixture();
        seed_v2_user(&database, &user).await;
        migrate_to_0004(&database).await;
        let base = user.created_at_ms + 800;
        let session = auth_session_fixture(user.id, 94, Revision::initial(), base, base + 10_000);
        let event = login_security_event(94, base, LoginSecurityReason::LoginSucceeded);
        assert!(database.create_auth_session(&session, &event).await.is_ok());
        let Database::Postgres(pool) = &database else {
            panic!("PostgreSQL 0005 rollback contract requires PostgreSQL")
        };
        let legacy_auth_level =
            sqlx::query("UPDATE auth_sessions SET auth_level='webauthn' WHERE id=$1")
                .bind(session.id.into_uuid())
                .execute(pool)
                .await;
        assert!(matches!(legacy_auth_level, Ok(result) if result.rows_affected() == 1));
        let before_sessions = auth_session_snapshots(&database)
            .await
            .unwrap_or_else(|error| {
                panic!("failed to snapshot pre-0005 PostgreSQL sessions: {error}")
            });
        assert_eq!(before_sessions.len(), 1);
        let before_events = login_security_event_snapshots(&database)
            .await
            .unwrap_or_else(|error| {
                panic!("failed to snapshot pre-0005 PostgreSQL events: {error}")
            });
        assert_eq!(before_events.len(), 1);
        let before_indexes = postgres_index_snapshots(pool, "auth_sessions")
            .await
            .unwrap_or_else(|error| {
                panic!("failed to snapshot pre-0005 PostgreSQL indexes: {error}")
            });
        let before_constraints = postgres_constraint_snapshots(pool, "auth_sessions")
            .await
            .unwrap_or_else(|error| {
                panic!("failed to snapshot pre-0005 PostgreSQL constraints: {error}")
            });
        assert!(
            before_constraints
                .iter()
                .any(|constraint| constraint.name == "auth_sessions_check")
        );
        assert!(
            !before_constraints
                .iter()
                .any(|constraint| { constraint.name == "auth_sessions_authenticated_at_ms_check" })
        );

        assert!(
            sqlx::query(
                "CREATE FUNCTION nodecontroll_test_fail_0005_auth_level() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF OLD.auth_level='webauthn' AND NEW.auth_level='phishing_resistant' THEN RAISE EXCEPTION 'forced 0005 auth level migration failure'; END IF; RETURN NEW; END $$",
            )
            .execute(pool)
            .await
            .is_ok()
        );
        assert!(
            sqlx::query(
                "CREATE TRIGGER nodecontroll_test_fail_0005_auth_level BEFORE UPDATE OF auth_level ON auth_sessions FOR EACH ROW EXECUTE FUNCTION nodecontroll_test_fail_0005_auth_level()",
            )
            .execute(pool)
            .await
            .is_ok()
        );

        let migration = super::POSTGRES_MIGRATOR.run_to(5, pool).await;
        let Err(error) = migration else {
            panic!("PostgreSQL 0005 trigger fixture did not fail the migration")
        };
        assert!(
            error
                .to_string()
                .contains("forced 0005 auth level migration failure")
        );
        assert!(matches!(migration_version(&database).await, Ok(Some(4))));
        assert_eq!(
            auth_session_snapshots(&database).await.ok(),
            Some(before_sessions.clone())
        );
        assert_eq!(
            login_security_event_snapshots(&database).await.ok(),
            Some(before_events.clone())
        );
        assert_eq!(
            postgres_index_snapshots(pool, "auth_sessions").await.ok(),
            Some(before_indexes.clone())
        );
        assert_eq!(
            postgres_constraint_snapshots(pool, "auth_sessions")
                .await
                .ok(),
            Some(before_constraints)
        );
        let trigger_count: Result<i64, sqlx::Error> = sqlx::query_scalar(
            "SELECT COUNT(*) FROM pg_trigger WHERE tgname='nodecontroll_test_fail_0005_auth_level' AND NOT tgisinternal",
        )
        .fetch_one(pool)
        .await;
        assert!(matches!(trigger_count, Ok(1)));

        assert!(
            sqlx::query("DROP TRIGGER nodecontroll_test_fail_0005_auth_level ON auth_sessions")
                .execute(pool)
                .await
                .is_ok()
        );
        assert!(
            sqlx::query("DROP FUNCTION nodecontroll_test_fail_0005_auth_level()")
                .execute(pool)
                .await
                .is_ok()
        );
        // A failed sqlx migration can leave its session-level advisory lock on
        // the pooled connection. Production retries happen in a fresh process;
        // closing and reconnecting here models that boundary and releases it.
        let retry_options = pool.connect_options().as_ref().clone();
        pool.close().await;
        let retry_pool = PgPoolOptions::new()
            .max_connections(4)
            .connect_with(retry_options)
            .await
            .unwrap_or_else(|error| {
                panic!("failed to reconnect for PostgreSQL 0005 retry: {error}")
            });
        let retry_database = Database::Postgres(retry_pool.clone());
        let retry = super::POSTGRES_MIGRATOR.run_to(5, &retry_pool).await;
        assert!(retry.is_ok(), "PostgreSQL 0005 retry failed: {retry:?}");
        assert!(matches!(
            migration_version(&retry_database).await,
            Ok(Some(5))
        ));
        let mut expected_sessions = before_sessions;
        assert_eq!(expected_sessions.len(), 1);
        if let Some(expected) = expected_sessions.first_mut() {
            expected.auth_level = "phishing_resistant".to_owned();
        }
        assert_eq!(
            auth_session_snapshots(&retry_database).await.ok(),
            Some(expected_sessions)
        );
        assert_eq!(
            login_security_event_snapshots(&retry_database).await.ok(),
            Some(before_events)
        );
        assert_eq!(
            postgres_index_snapshots(&retry_pool, "auth_sessions")
                .await
                .ok(),
            Some(before_indexes)
        );
        let after_constraints = postgres_constraint_snapshots(&retry_pool, "auth_sessions")
            .await
            .unwrap_or_else(|error| {
                panic!("failed to snapshot post-0005 PostgreSQL constraints: {error}")
            });
        assert!(
            !after_constraints
                .iter()
                .any(|constraint| constraint.name == "auth_sessions_check")
        );
        assert!(after_constraints.iter().any(|constraint| {
            constraint.name == "auth_sessions_authenticated_at_ms_check"
                && constraint.validated
                && constraint.definition.contains("authenticated_at_ms >= 0")
        }));
        assert!(after_constraints.iter().any(|constraint| {
            constraint.name == "auth_sessions_auth_level_check"
                && constraint.validated
                && constraint.definition.contains("phishing_resistant")
                && !constraint.definition.contains("webauthn")
        }));
        assert!(after_constraints.iter().any(|constraint| {
            constraint.name == "auth_sessions_revoked_reason_check"
                && constraint.validated
                && constraint.definition.contains("user_revoked")
        }));
        let function_absent: Result<bool, sqlx::Error> = sqlx::query_scalar(
            "SELECT to_regprocedure('nodecontroll_test_fail_0005_auth_level()') IS NULL",
        )
        .fetch_one(&retry_pool)
        .await;
        assert!(matches!(function_absent, Ok(true)));
        retry_pool.close().await;
    }

    async fn auth_migration_rollback_contract(database: Database) {
        migrate_to_0002(&database).await;
        match &database {
            Database::Sqlite(pool) => {
                assert!(
                    sqlx::query("CREATE TABLE auth_sessions (sentinel INTEGER NOT NULL)")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
            Database::Postgres(pool) => {
                assert!(
                    sqlx::query("CREATE TABLE auth_sessions (sentinel INTEGER NOT NULL)")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
        }
        assert!(database.migrate().await.is_err());
        match &database {
            Database::Sqlite(pool) => {
                let version: Result<Option<i64>, _> =
                    sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations")
                        .fetch_one(pool)
                        .await;
                let auth_state_count: Result<i64, _> = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='user_auth_state'",
                )
                .fetch_one(pool)
                .await;
                assert!(matches!(version, Ok(Some(2))));
                assert!(matches!(auth_state_count, Ok(0)));
                assert!(
                    sqlx::query("DROP TABLE auth_sessions")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
            Database::Postgres(pool) => {
                let version: Result<Option<i64>, _> =
                    sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations")
                        .fetch_one(pool)
                        .await;
                let auth_state_absent: Result<bool, _> =
                    sqlx::query_scalar("SELECT to_regclass('user_auth_state') IS NULL")
                        .fetch_one(pool)
                        .await;
                assert!(matches!(version, Ok(Some(2))));
                assert!(matches!(auth_state_absent, Ok(true)));
                assert!(
                    sqlx::query("DROP TABLE auth_sessions")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
        }
        match &database {
            Database::Sqlite(_) => assert!(database.migrate().await.is_ok()),
            Database::Postgres(pool) => {
                // sqlx returns before unlocking its session advisory lock when
                // an apply step fails. A production retry starts in a fresh
                // process, so release the old sessions before retrying here.
                let retry_options = pool.connect_options().as_ref().clone();
                pool.close().await;
                let retry_pool = PgPoolOptions::new()
                    .max_connections(4)
                    .connect_with(retry_options)
                    .await
                    .unwrap_or_else(|error| {
                        panic!("failed to reconnect for PostgreSQL auth migration retry: {error}")
                    });
                let retry_database = Database::Postgres(retry_pool.clone());
                let retry = retry_database.migrate().await;
                assert!(
                    retry.is_ok(),
                    "PostgreSQL auth migration retry failed: {retry:?}"
                );
                retry_pool.close().await;
            }
        }
    }

    async fn login_security_event_count(
        database: &Database,
        event_id: EntityId,
    ) -> Result<i64, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => {
                sqlx::query_scalar("SELECT COUNT(*) FROM login_security_events WHERE id=?")
                    .bind(event_id.to_string())
                    .fetch_one(pool)
                    .await
            }
            Database::Postgres(pool) => {
                sqlx::query_scalar("SELECT COUNT(*) FROM login_security_events WHERE id=$1")
                    .bind(event_id.into_uuid())
                    .fetch_one(pool)
                    .await
            }
        }
    }

    async fn credentials_snapshot(database: &Database, username_norm: &str) -> UserCredentials {
        match database
            .user_credentials_by_normalized_username(username_norm)
            .await
        {
            Ok(Some(credentials)) => credentials,
            _ => panic!("expected a persisted credential snapshot"),
        }
    }

    async fn auth_state_timestamps(
        database: &Database,
        user_id: EntityId,
    ) -> Result<Option<(i64, i64, i64)>, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => sqlx::query_as(
                "SELECT auth_revision,password_changed_at_ms,updated_at_ms FROM user_auth_state WHERE user_id=?",
            )
            .bind(user_id.to_string())
            .fetch_optional(pool)
            .await,
            Database::Postgres(pool) => sqlx::query_as(
                "SELECT auth_revision,password_changed_at_ms,updated_at_ms FROM user_auth_state WHERE user_id=$1",
            )
            .bind(user_id.into_uuid())
            .fetch_optional(pool)
            .await,
        }
    }

    async fn set_user_auth_revision(
        database: &Database,
        user_id: EntityId,
        auth_revision: i64,
    ) -> Result<u64, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => {
                sqlx::query("UPDATE user_auth_state SET auth_revision=? WHERE user_id=?")
                    .bind(auth_revision)
                    .bind(user_id.to_string())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
            Database::Postgres(pool) => {
                sqlx::query("UPDATE user_auth_state SET auth_revision=$1 WHERE user_id=$2")
                    .bind(auth_revision)
                    .bind(user_id.into_uuid())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
        }
    }

    async fn set_session_revision(
        database: &Database,
        session_id: EntityId,
        revision: i64,
    ) -> Result<u64, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => sqlx::query("UPDATE auth_sessions SET revision=? WHERE id=?")
                .bind(revision)
                .bind(session_id.to_string())
                .execute(pool)
                .await
                .map(|result| result.rows_affected()),
            Database::Postgres(pool) => {
                sqlx::query("UPDATE auth_sessions SET revision=$1 WHERE id=$2")
                    .bind(revision)
                    .bind(session_id.into_uuid())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn set_user_login_snapshot(
        database: &Database,
        user_id: EntityId,
        username: &str,
        password_hash: &PasswordHash,
        role: &str,
        status: &str,
        principal_label: &str,
        force_password_change: bool,
        revision: i64,
    ) -> Result<u64, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => sqlx::query(
                "UPDATE users SET username=?,username_norm=lower(?),password_hash=?,role=?,status=?,principal_label=?,force_password_change=?,revision=? WHERE id=?",
            )
            .bind(username)
            .bind(username)
            .bind(password_hash.as_str())
            .bind(role)
            .bind(status)
            .bind(principal_label)
            .bind(force_password_change)
            .bind(revision)
            .bind(user_id.to_string())
            .execute(pool)
            .await
            .map(|result| result.rows_affected()),
            Database::Postgres(pool) => sqlx::query(
                "UPDATE users SET username=$1,username_norm=lower($1),password_hash=$2,role=$3,status=$4,principal_label=$5,force_password_change=$6,revision=$7 WHERE id=$8",
            )
            .bind(username)
            .bind(password_hash.as_str())
            .bind(role)
            .bind(status)
            .bind(principal_label)
            .bind(force_password_change)
            .bind(revision)
            .bind(user_id.into_uuid())
            .execute(pool)
            .await
            .map(|result| result.rows_affected()),
        }
    }

    async fn login_rate_attempt_count(
        database: &Database,
        scope: &str,
        key_version: u32,
        bucket_hmac: &[u8; 32],
    ) -> Result<Option<i64>, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => sqlx::query_scalar(
                "SELECT attempt_count FROM login_rate_buckets WHERE scope=? AND key_version=? AND bucket_hmac=?",
            )
            .bind(scope)
            .bind(i64::from(key_version))
            .bind(bucket_hmac.as_slice())
            .fetch_optional(pool)
            .await,
            Database::Postgres(pool) => sqlx::query_scalar(
                "SELECT attempt_count::BIGINT FROM login_rate_buckets WHERE scope=$1 AND key_version=$2 AND bucket_hmac=$3",
            )
            .bind(scope)
            .bind(i32::try_from(key_version).unwrap_or_default())
            .bind(bucket_hmac.as_slice())
            .fetch_optional(pool)
            .await,
        }
    }

    async fn login_rate_scope_row_count(
        database: &Database,
        scope: &str,
        key_version: u32,
    ) -> Result<i64, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => {
                sqlx::query_scalar(
                    "SELECT COUNT(*) FROM login_rate_buckets WHERE scope=? AND key_version=?",
                )
                .bind(scope)
                .bind(i64::from(key_version))
                .fetch_one(pool)
                .await
            }
            Database::Postgres(pool) => {
                sqlx::query_scalar(
                    "SELECT COUNT(*) FROM login_rate_buckets WHERE scope=$1 AND key_version=$2",
                )
                .bind(scope)
                .bind(i32::try_from(key_version).unwrap_or_default())
                .fetch_one(pool)
                .await
            }
        }
    }

    async fn raw_auth_secret_absence_contract(database: &Database, session: &NewAuthSession) {
        let raw_token = b"raw-session-token-that-must-not-be-stored";
        let raw_csrf = b"raw-csrf-token-that-must-not-be-stored";
        let (raw_matches, forbidden_columns): (Result<i64, sqlx::Error>, Result<i64, sqlx::Error>) =
            match database {
                Database::Sqlite(pool) => (
                    sqlx::query_scalar(
                        "SELECT COUNT(*) FROM auth_sessions WHERE token_hmac=? OR csrf_hmac=?",
                    )
                    .bind(raw_token.as_slice())
                    .bind(raw_csrf.as_slice())
                    .fetch_one(pool)
                    .await,
                    sqlx::query_scalar(
                        "SELECT COUNT(*) FROM (SELECT name FROM pragma_table_info('auth_sessions') UNION ALL SELECT name FROM pragma_table_info('login_rate_buckets') UNION ALL SELECT name FROM pragma_table_info('login_security_events')) WHERE name IN ('username','ip_address','ip_prefix','token','csrf_token')",
                    )
                    .fetch_one(pool)
                    .await,
                ),
                Database::Postgres(pool) => (
                    sqlx::query_scalar(
                        "SELECT COUNT(*) FROM auth_sessions WHERE token_hmac=$1 OR csrf_hmac=$2",
                    )
                    .bind(raw_token.as_slice())
                    .bind(raw_csrf.as_slice())
                    .fetch_one(pool)
                    .await,
                    sqlx::query_scalar(
                        "SELECT COUNT(*) FROM information_schema.columns WHERE table_schema=current_schema() AND table_name IN ('auth_sessions','login_rate_buckets','login_security_events') AND column_name IN ('username','ip_address','ip_prefix','token','csrf_token')",
                    )
                    .fetch_one(pool)
                    .await,
                ),
            };
        assert!(matches!(raw_matches, Ok(0)));
        assert!(matches!(forbidden_columns, Ok(0)));
        let stored_digest_count: Result<i64, sqlx::Error> = match database {
            Database::Sqlite(pool) => sqlx::query_scalar(
                "SELECT COUNT(*) FROM auth_sessions WHERE token_key_version=? AND token_hmac=? AND csrf_key_version=? AND csrf_hmac=?",
            )
            .bind(i64::from(session.token_key_version))
            .bind(session.token_hmac.as_slice())
            .bind(i64::from(session.csrf_key_version))
            .bind(session.csrf_hmac.as_slice())
            .fetch_one(pool)
            .await,
            Database::Postgres(pool) => sqlx::query_scalar(
                "SELECT COUNT(*) FROM auth_sessions WHERE token_key_version=$1 AND token_hmac=$2 AND csrf_key_version=$3 AND csrf_hmac=$4",
            )
            .bind(i32::try_from(session.token_key_version).unwrap_or_default())
            .bind(session.token_hmac.as_slice())
            .bind(i32::try_from(session.csrf_key_version).unwrap_or_default())
            .bind(session.csrf_hmac.as_slice())
            .fetch_one(pool)
            .await,
        };
        assert!(matches!(stored_digest_count, Ok(1)));
    }

    async fn auth_schema_constraints_contract(
        database: &Database,
        owner: &UserAccount,
        session: &NewAuthSession,
    ) {
        let short_digest = vec![0_u8; 31];
        let invalid_digest = match database {
            Database::Sqlite(pool) => {
                sqlx::query("UPDATE auth_sessions SET token_hmac=? WHERE id=?")
                    .bind(short_digest)
                    .bind(session.id.to_string())
                    .execute(pool)
                    .await
                    .map(|_| ())
            }
            Database::Postgres(pool) => {
                sqlx::query("UPDATE auth_sessions SET token_hmac=$1 WHERE id=$2")
                    .bind(short_digest)
                    .bind(session.id.into_uuid())
                    .execute(pool)
                    .await
                    .map(|_| ())
            }
        };
        assert!(invalid_digest.is_err());
        let invalid_enum = match database {
            Database::Sqlite(pool) => {
                sqlx::query("UPDATE auth_sessions SET auth_level='unexpected' WHERE id=?")
                    .bind(session.id.to_string())
                    .execute(pool)
                    .await
                    .map(|_| ())
            }
            Database::Postgres(pool) => {
                sqlx::query("UPDATE auth_sessions SET auth_level='unexpected' WHERE id=$1")
                    .bind(session.id.into_uuid())
                    .execute(pool)
                    .await
                    .map(|_| ())
            }
        };
        assert!(invalid_enum.is_err());

        let inherited_authenticated_at_ms = session.created_at_ms.saturating_sub(1);
        assert!(inherited_authenticated_at_ms < session.created_at_ms);
        let relaxed_authenticated_timeline = match database {
            Database::Sqlite(pool) => {
                sqlx::query("UPDATE auth_sessions SET authenticated_at_ms=? WHERE id=?")
                    .bind(inherited_authenticated_at_ms)
                    .bind(session.id.to_string())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
            Database::Postgres(pool) => {
                sqlx::query("UPDATE auth_sessions SET authenticated_at_ms=$1 WHERE id=$2")
                    .bind(inherited_authenticated_at_ms)
                    .bind(session.id.into_uuid())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
        };
        assert!(matches!(relaxed_authenticated_timeline, Ok(1)));
        let restored_authenticated_timeline = match database {
            Database::Sqlite(pool) => {
                sqlx::query("UPDATE auth_sessions SET authenticated_at_ms=? WHERE id=?")
                    .bind(session.authenticated_at_ms)
                    .bind(session.id.to_string())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
            Database::Postgres(pool) => {
                sqlx::query("UPDATE auth_sessions SET authenticated_at_ms=$1 WHERE id=$2")
                    .bind(session.authenticated_at_ms)
                    .bind(session.id.into_uuid())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
        };
        assert!(matches!(restored_authenticated_timeline, Ok(1)));

        let active_with_revocation_metadata = match database {
            Database::Sqlite(pool) => sqlx::query(
                "UPDATE auth_sessions SET revoked_at_ms=created_at_ms,revoked_reason='logout' WHERE id=?",
            )
            .bind(session.id.to_string())
            .execute(pool)
            .await
            .map(|_| ()),
            Database::Postgres(pool) => sqlx::query(
                "UPDATE auth_sessions SET revoked_at_ms=created_at_ms,revoked_reason='logout' WHERE id=$1",
            )
            .bind(session.id.into_uuid())
            .execute(pool)
            .await
            .map(|_| ()),
        };
        assert!(active_with_revocation_metadata.is_err());
        let revoked_without_revocation_metadata = match database {
            Database::Sqlite(pool) => {
                sqlx::query("UPDATE auth_sessions SET status='revoked' WHERE id=?")
                    .bind(session.id.to_string())
                    .execute(pool)
                    .await
                    .map(|_| ())
            }
            Database::Postgres(pool) => {
                sqlx::query("UPDATE auth_sessions SET status='revoked' WHERE id=$1")
                    .bind(session.id.into_uuid())
                    .execute(pool)
                    .await
                    .map(|_| ())
            }
        };
        assert!(revoked_without_revocation_metadata.is_err());

        let hard_delete = match database {
            Database::Sqlite(pool) => sqlx::query("DELETE FROM users WHERE id=?")
                .bind(owner.id.to_string())
                .execute(pool)
                .await
                .map(|_| ()),
            Database::Postgres(pool) => sqlx::query("DELETE FROM users WHERE id=$1")
                .bind(owner.id.into_uuid())
                .execute(pool)
                .await
                .map(|_| ()),
        };
        assert!(hard_delete.is_err());
        let partial_index_count: Result<i64, sqlx::Error> = match database {
            Database::Sqlite(pool) => sqlx::query_scalar(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name='auth_sessions_active_user_idx' AND sql LIKE '%WHERE status = ''active''%'",
            )
            .fetch_one(pool)
            .await,
            Database::Postgres(pool) => sqlx::query_scalar(
                "SELECT COUNT(*) FROM pg_indexes WHERE schemaname=current_schema() AND indexname='auth_sessions_active_user_idx' AND indexdef LIKE '%WHERE%' AND indexdef LIKE '%status%' AND indexdef LIKE '%active%'",
            )
            .fetch_one(pool)
            .await,
        };
        assert!(matches!(partial_index_count, Ok(1)));
    }

    async fn actor_aware_session_revocation_contract(database: &Database) {
        let (touch_user, touch_actor, touch_target, touch_snapshot) =
            provision_actor_and_target(database, 60, 100, 101).await;
        let touch_at_ms = touch_actor.created_at_ms + 100;
        let touched = database
            .authenticate_session(&session_authentication(
                &touch_actor,
                Some(touch_actor.csrf_hmac),
                touch_at_ms,
            ))
            .await;
        assert!(matches!(
            touched,
            Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                if authenticated.session.revision != touch_snapshot.session.revision
                    && authenticated.session.last_seen_at_ms == touch_at_ms
        ));
        let touch_revocation_at_ms = touch_at_ms + 1;
        let touch_revocation_event = session_security_event(
            &touch_actor,
            160,
            touch_revocation_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_user_session_with_event(user_session_revocation(
                    &touch_snapshot,
                    touch_target.id,
                    &touch_revocation_event,
                    touch_revocation_at_ms,
                ))
                .await,
            Ok(true)
        ));
        assert!(matches!(
            login_security_event_count(database, touch_revocation_event.id).await,
            Ok(1)
        ));
        assert!(matches!(
            database.list_user_sessions(touch_user.id).await,
            Ok(ref sessions)
                if sessions.iter().any(|session|
                    session.id == touch_target.id
                        && session.status == AuthSessionStatus::Revoked
                        && session.revoked_reason == Some(SessionRevocationReason::UserRevoked)
                )
        ));

        let repeated_at_ms = touch_revocation_at_ms + 1;
        let repeated_event = session_security_event(
            &touch_actor,
            161,
            repeated_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_user_session_with_event(user_session_revocation(
                    &touch_snapshot,
                    touch_target.id,
                    &repeated_event,
                    repeated_at_ms,
                ))
                .await,
            Ok(false)
        ));
        assert!(matches!(
            login_security_event_count(database, repeated_event.id).await,
            Ok(0)
        ));

        let (other_user, _other_actor, other_target, _) =
            provision_actor_and_target(database, 61, 102, 103).await;
        let wrong_user_at_ms = repeated_at_ms + 1;
        let wrong_user_event = session_security_event(
            &touch_actor,
            162,
            wrong_user_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_user_session_with_event(user_session_revocation(
                    &touch_snapshot,
                    other_target.id,
                    &wrong_user_event,
                    wrong_user_at_ms,
                ))
                .await,
            Ok(false)
        ));
        assert!(stored_session_is_active(database, other_user.id, other_target.id).await);
        assert!(matches!(
            login_security_event_count(database, wrong_user_event.id).await,
            Ok(0)
        ));

        let unknown_at_ms = wrong_user_at_ms + 1;
        let unknown_event = session_security_event(
            &touch_actor,
            163,
            unknown_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_user_session_with_event(user_session_revocation(
                    &touch_snapshot,
                    EntityId::new(),
                    &unknown_event,
                    unknown_at_ms,
                ))
                .await,
            Ok(false)
        ));
        assert!(matches!(
            login_security_event_count(database, unknown_event.id).await,
            Ok(0)
        ));

        let (_current_user, current_actor, current_sibling, current_snapshot) =
            provision_actor_and_target(database, 62, 104, 105).await;
        let current_revoke_at_ms = current_actor.created_at_ms + 10;
        let current_revoke_event = session_security_event(
            &current_actor,
            164,
            current_revoke_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_user_session_with_event(user_session_revocation(
                    &current_snapshot,
                    current_actor.id,
                    &current_revoke_event,
                    current_revoke_at_ms,
                ))
                .await,
            Ok(true)
        ));
        let repeated_current_at_ms = current_revoke_at_ms + 1;
        let repeated_current_event = session_security_event(
            &current_actor,
            165,
            repeated_current_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_user_session_with_event(user_session_revocation(
                    &current_snapshot,
                    current_actor.id,
                    &repeated_current_event,
                    repeated_current_at_ms,
                ))
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(
            stored_session_is_active(database, current_snapshot.user_id, current_sibling.id).await
        );
        assert!(matches!(
            login_security_event_count(database, repeated_current_event.id).await,
            Ok(0)
        ));

        let (rotation_user, rotation_actor, rotation_target, rotation_snapshot) =
            provision_actor_and_target(database, 63, 106, 107).await;
        let rotation_at_ms = rotation_actor.created_at_ms + 100;
        let rotation_replacement = auth_session_fixture(
            rotation_user.id,
            108,
            Revision::initial(),
            rotation_at_ms,
            rotation_actor.absolute_expires_at_ms,
        );
        let rotation_event = session_security_event(
            &rotation_replacement,
            166,
            rotation_at_ms,
            LoginSecurityReason::ReauthenticationSucceeded,
        );
        assert!(
            database
                .rotate_current_session(
                    rotation_user.id,
                    rotation_actor.id,
                    rotation_snapshot.user_revision,
                    &rotation_replacement,
                    &rotation_event,
                    rotation_at_ms,
                )
                .await
                .is_ok()
        );
        let stale_rotation_at_ms = rotation_at_ms + 1;
        let stale_rotation_event = session_security_event(
            &rotation_actor,
            167,
            stale_rotation_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_user_session_with_event(user_session_revocation(
                    &rotation_snapshot,
                    rotation_target.id,
                    &stale_rotation_event,
                    stale_rotation_at_ms,
                ))
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(stored_session_is_active(database, rotation_user.id, rotation_target.id).await);
        assert!(matches!(
            login_security_event_count(database, stale_rotation_event.id).await,
            Ok(0)
        ));

        let (revoked_user, revoked_actor, revoked_target, revoked_snapshot) =
            provision_actor_and_target(database, 64, 109, 110).await;
        let actor_revoke_at_ms = revoked_actor.created_at_ms + 100;
        let actor_revoke_event = session_security_event(
            &revoked_actor,
            168,
            actor_revoke_at_ms,
            LoginSecurityReason::Logout,
        );
        assert!(matches!(
            database
                .revoke_current_session_with_event(
                    revoked_user.id,
                    revoked_actor.id,
                    actor_revoke_at_ms,
                    SessionRevocationReason::Logout,
                    &actor_revoke_event,
                )
                .await,
            Ok(true)
        ));
        let stale_revoke_at_ms = actor_revoke_at_ms + 1;
        let stale_revoke_event = session_security_event(
            &revoked_actor,
            169,
            stale_revoke_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_user_session_with_event(user_session_revocation(
                    &revoked_snapshot,
                    revoked_target.id,
                    &stale_revoke_event,
                    stale_revoke_at_ms,
                ))
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(stored_session_is_active(database, revoked_user.id, revoked_target.id).await);
        assert!(matches!(
            login_security_event_count(database, stale_revoke_event.id).await,
            Ok(0)
        ));

        let (auth_revision_user, auth_revision_actor, auth_revision_target, auth_revision_snapshot) =
            provision_actor_and_target(database, 65, 111, 112).await;
        assert!(matches!(
            set_user_auth_revision(database, auth_revision_user.id, 1).await,
            Ok(1)
        ));
        let stale_auth_revision_at_ms = auth_revision_actor.created_at_ms + 10;
        let stale_auth_revision_event = session_security_event(
            &auth_revision_actor,
            170,
            stale_auth_revision_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_user_session_with_event(user_session_revocation(
                    &auth_revision_snapshot,
                    auth_revision_target.id,
                    &stale_auth_revision_event,
                    stale_auth_revision_at_ms,
                ))
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(
            stored_session_is_active(database, auth_revision_user.id, auth_revision_target.id)
                .await
        );
        assert!(matches!(
            login_security_event_count(database, stale_auth_revision_event.id).await,
            Ok(0)
        ));

        let (disabled_user, disabled_actor, disabled_target, disabled_snapshot) =
            provision_actor_and_target(database, 66, 113, 114).await;
        assert!(matches!(
            set_user_status_and_revision(database, disabled_user.id, "disabled", 0).await,
            Ok(1)
        ));
        let disabled_at_ms = disabled_actor.created_at_ms + 10;
        let disabled_event = session_security_event(
            &disabled_actor,
            171,
            disabled_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_user_session_with_event(user_session_revocation(
                    &disabled_snapshot,
                    disabled_target.id,
                    &disabled_event,
                    disabled_at_ms,
                ))
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(stored_session_is_active(database, disabled_user.id, disabled_target.id).await);
        assert!(matches!(
            login_security_event_count(database, disabled_event.id).await,
            Ok(0)
        ));

        let (user_revision_user, user_revision_actor, user_revision_target, user_revision_snapshot) =
            provision_actor_and_target(database, 67, 115, 116).await;
        assert!(matches!(
            set_user_status_and_revision(database, user_revision_user.id, "active", 1).await,
            Ok(1)
        ));
        let stale_user_revision_at_ms = user_revision_actor.created_at_ms + 10;
        let stale_user_revision_event = session_security_event(
            &user_revision_actor,
            172,
            stale_user_revision_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_user_session_with_event(user_session_revocation(
                    &user_revision_snapshot,
                    user_revision_target.id,
                    &stale_user_revision_event,
                    stale_user_revision_at_ms,
                ))
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(
            stored_session_is_active(database, user_revision_user.id, user_revision_target.id)
                .await
        );
        assert!(matches!(
            login_security_event_count(database, stale_user_revision_event.id).await,
            Ok(0)
        ));

        let (clock_user, clock_actor, clock_target, clock_snapshot) =
            provision_actor_and_target(database, 68, 117, 118).await;
        let rolled_back_at_ms = clock_actor.created_at_ms - 1;
        let rolled_back_event = session_security_event(
            &clock_actor,
            173,
            rolled_back_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_user_session_with_event(user_session_revocation(
                    &clock_snapshot,
                    clock_target.id,
                    &rolled_back_event,
                    rolled_back_at_ms,
                ))
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(stored_session_is_active(database, clock_user.id, clock_target.id).await);
        assert!(matches!(
            login_security_event_count(database, rolled_back_event.id).await,
            Ok(0)
        ));

        let (cross_user, cross_actor, cross_target, cross_actor_snapshot) =
            provision_actor_and_target(database, 69, 119, 120).await;
        let cross_target_snapshot = database
            .authenticate_session(&session_authentication(
                &cross_target,
                Some(cross_target.csrf_hmac),
                cross_target.created_at_ms,
            ))
            .await;
        let cross_target_snapshot = match cross_target_snapshot {
            Ok(SessionAuthenticationOutcome::Authenticated(authenticated)) => authenticated,
            outcome => panic!("unexpected cross-delete target authentication: {outcome:?}"),
        };
        let cross_delete_at_ms = cross_target.created_at_ms + 10;
        let actor_deletes_target_event = session_security_event(
            &cross_actor,
            174,
            cross_delete_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        let target_deletes_actor_event = session_security_event(
            &cross_target,
            175,
            cross_delete_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        let (actor_deletes_target, target_deletes_actor) = tokio::join!(
            database.revoke_user_session_with_event(user_session_revocation(
                &cross_actor_snapshot,
                cross_target.id,
                &actor_deletes_target_event,
                cross_delete_at_ms,
            )),
            database.revoke_user_session_with_event(user_session_revocation(
                &cross_target_snapshot,
                cross_actor.id,
                &target_deletes_actor_event,
                cross_delete_at_ms,
            )),
        );
        assert!(matches!(
            (&actor_deletes_target, &target_deletes_actor),
            (Ok(true), Err(PersistenceError::SessionPrincipalUnavailable))
                | (Err(PersistenceError::SessionPrincipalUnavailable), Ok(true))
        ));
        let winner_actor_id = if matches!(&actor_deletes_target, Ok(true)) {
            cross_actor.id
        } else {
            cross_target.id
        };
        let (actor_event_count, target_event_count) = tokio::join!(
            login_security_event_count(database, actor_deletes_target_event.id),
            login_security_event_count(database, target_deletes_actor_event.id),
        );
        assert!(matches!(
            (actor_event_count, target_event_count),
            (Ok(1), Ok(0)) | (Ok(0), Ok(1))
        ));
        assert!(matches!(
            database.list_user_sessions(cross_user.id).await,
            Ok(ref sessions)
                if sessions.iter().filter(|session| session.status == AuthSessionStatus::Active).count() == 1
                    && sessions.iter().any(|session|
                        session.id == winner_actor_id
                            && session.status == AuthSessionStatus::Active
                    )
                    && sessions.iter().filter(|session|
                        session.status == AuthSessionStatus::Revoked
                            && session.revoked_reason == Some(SessionRevocationReason::UserRevoked)
                    ).count() == 1
        ));

        let (touch_delete_user, touch_delete_actor, touch_delete_target, touch_delete_snapshot) =
            provision_actor_and_target(database, 70, 121, 122).await;
        let touch_delete_at_ms = touch_delete_target.created_at_ms + 100;
        let touch_delete_event = session_security_event(
            &touch_delete_actor,
            176,
            touch_delete_at_ms,
            LoginSecurityReason::SessionRevoked,
        );
        let touch_delete_authentication = session_authentication(
            &touch_delete_target,
            Some(touch_delete_target.csrf_hmac),
            touch_delete_at_ms,
        );
        let (touch_during_delete, delete_during_touch) = tokio::join!(
            database.authenticate_session(&touch_delete_authentication),
            database.revoke_user_session_with_event(user_session_revocation(
                &touch_delete_snapshot,
                touch_delete_target.id,
                &touch_delete_event,
                touch_delete_at_ms,
            )),
        );
        assert!(matches!(delete_during_touch, Ok(true)));
        assert!(matches!(
            touch_during_delete,
            Ok(SessionAuthenticationOutcome::Authenticated(_))
                | Ok(SessionAuthenticationOutcome::InvalidSession)
        ));
        assert!(
            !stored_session_is_active(database, touch_delete_user.id, touch_delete_target.id,)
                .await
        );
        assert!(matches!(
            login_security_event_count(database, touch_delete_event.id).await,
            Ok(1)
        ));

        let (touch_rotate_user, touch_rotate_actor, _touch_rotate_sibling, touch_rotate_snapshot) =
            provision_actor_and_target(database, 71, 123, 124).await;
        let touch_rotate_at_ms = touch_rotate_actor.created_at_ms + 100;
        let touch_rotate_replacement = auth_session_fixture(
            touch_rotate_user.id,
            125,
            touch_rotate_actor.auth_revision,
            touch_rotate_at_ms,
            touch_rotate_actor.absolute_expires_at_ms,
        );
        let touch_rotate_event = session_security_event(
            &touch_rotate_replacement,
            177,
            touch_rotate_at_ms,
            LoginSecurityReason::ReauthenticationSucceeded,
        );
        let touch_rotate_authentication = session_authentication(
            &touch_rotate_actor,
            Some(touch_rotate_actor.csrf_hmac),
            touch_rotate_at_ms,
        );
        let (touch_during_rotation, rotation_during_touch) = tokio::join!(
            database.authenticate_session(&touch_rotate_authentication),
            database.rotate_current_session(
                touch_rotate_user.id,
                touch_rotate_actor.id,
                touch_rotate_snapshot.user_revision,
                &touch_rotate_replacement,
                &touch_rotate_event,
                touch_rotate_at_ms,
            ),
        );
        assert!(rotation_during_touch.is_ok());
        assert!(matches!(
            touch_during_rotation,
            Ok(SessionAuthenticationOutcome::Authenticated(_))
                | Ok(SessionAuthenticationOutcome::InvalidSession)
        ));
        assert!(matches!(
            database.list_user_sessions(touch_rotate_user.id).await,
            Ok(ref sessions)
                if sessions.iter().any(|session|
                    session.id == touch_rotate_actor.id
                        && session.status == AuthSessionStatus::Revoked
                        && session.revoked_reason == Some(SessionRevocationReason::Rotation)
                ) && sessions.iter().any(|session|
                    session.id == touch_rotate_replacement.id
                        && session.status == AuthSessionStatus::Active
                )
        ));
        assert!(matches!(
            login_security_event_count(database, touch_rotate_event.id).await,
            Ok(1)
        ));
    }

    async fn auth_core_contract(database: &Database, owner: &UserAccount) {
        let normalized = owner.username.normalized();
        let credentials = database
            .user_credentials_by_normalized_username(&normalized)
            .await;
        assert!(matches!(
            credentials,
            Ok(Some(ref found))
                if found.user_id == owner.id
                    && found.username == owner.username
                    && found.password_hash == owner.password_hash
                    && found.role == UserRole::Owner
                    && found.status == nodecontroll_domain::UserStatus::Active
                    && found.auth_revision == Revision::initial()
        ));
        assert!(matches!(
            database
                .user_credentials_by_normalized_username(owner.username.as_str())
                .await,
            Err(PersistenceError::UsernameIsNotNormalized)
        ));

        let base = owner.created_at_ms + 100;
        let first = auth_session_fixture(owner.id, 1, Revision::initial(), base, base + 10_000);
        let first_event = login_security_event(1, base, LoginSecurityReason::LoginSucceeded);
        let created = database.create_auth_session(&first, &first_event).await;
        assert!(matches!(
            created,
            Ok(ref summary)
                if summary.id == first.id && summary.status == AuthSessionStatus::Active
        ));
        raw_auth_secret_absence_contract(database, &first).await;
        auth_schema_constraints_contract(database, owner, &first).await;
        let rolled_back_clock = session_authentication(&first, None, base - 1);
        assert!(matches!(
            database.authenticate_session(&rolled_back_clock).await,
            Ok(SessionAuthenticationOutcome::InvalidSession)
        ));
        assert!(matches!(
            database.list_active_user_sessions(owner.id, base - 1).await,
            Ok(ref sessions) if sessions.is_empty()
        ));

        let wrong_csrf = session_authentication(&first, Some([0xff; 32]), base + 100);
        assert!(matches!(
            database.authenticate_session(&wrong_csrf).await,
            Ok(SessionAuthenticationOutcome::InvalidCsrf)
        ));
        let wrong_token_version = SessionAuthentication {
            token_key_version: 2,
            ..session_authentication(&first, None, base + 10)
        };
        assert!(matches!(
            database.authenticate_session(&wrong_token_version).await,
            Ok(SessionAuthenticationOutcome::InvalidSession)
        ));
        let safe_authentication = session_authentication(&first, None, base + 10);
        assert!(matches!(
            database.authenticate_session(&safe_authentication).await,
            Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                if authenticated.user_id == owner.id
        ));
        assert!(matches!(
            database.list_user_sessions(owner.id).await,
            Ok(ref sessions)
                if sessions.len() == 1
                    && sessions[0].last_seen_at_ms == base
                    && sessions[0].revision == Revision::initial()
        ));
        let touched_at_ms = base + 100;
        let csrf_authentication =
            session_authentication(&first, Some(first.csrf_hmac), touched_at_ms);
        assert!(matches!(
            database
                .authenticate_session_read_only(&csrf_authentication)
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                if authenticated.session.last_seen_at_ms == base
                    && authenticated.session.idle_expires_at_ms == first.idle_expires_at_ms
                    && authenticated.session.revision == Revision::initial()
        ));
        assert!(matches!(
            database.list_user_sessions(owner.id).await,
            Ok(ref sessions)
                if sessions.len() == 1
                    && sessions[0].last_seen_at_ms == base
                    && sessions[0].idle_expires_at_ms == first.idle_expires_at_ms
                    && sessions[0].revision == Revision::initial()
        ));
        assert!(matches!(
            database.authenticate_session(&csrf_authentication).await,
            Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                if authenticated.session.last_seen_at_ms == touched_at_ms
                    && authenticated.session.revision == Revision::from_value(1)
        ));

        let repository_reopened = match database {
            Database::Sqlite(pool) => Database::Sqlite(pool.clone()),
            Database::Postgres(pool) => Database::Postgres(pool.clone()),
        };
        assert!(matches!(
            repository_reopened.list_user_sessions(owner.id).await,
            Ok(ref sessions) if sessions.len() == 1 && sessions[0].id == first.id
        ));
        assert!(matches!(
            repository_reopened.list_user_sessions(EntityId::new()).await,
            Ok(ref sessions) if sessions.is_empty()
        ));

        let expired =
            auth_session_fixture(owner.id, 2, Revision::initial(), base + 200, base + 250);
        let expired_event =
            login_security_event(2, base + 200, LoginSecurityReason::LoginSucceeded);
        assert!(
            database
                .create_auth_session(&expired, &expired_event)
                .await
                .is_ok()
        );
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(&expired, None, base + 250))
                .await,
            Ok(SessionAuthenticationOutcome::InvalidSession)
        ));
        assert!(matches!(
            database
                .list_active_user_sessions(owner.id, base + 249)
                .await,
            Ok(ref sessions)
                if sessions.len() == 2
                    && sessions.iter().any(|session| session.id == first.id)
                    && sessions.iter().any(|session| session.id == expired.id)
        ));
        assert!(matches!(
            database
                .list_active_user_sessions(owner.id, base + 250)
                .await,
            Ok(ref sessions) if sessions.len() == 1 && sessions[0].id == first.id
        ));
        assert!(matches!(
            database
                .list_active_user_sessions(owner.id, base + 251)
                .await,
            Ok(ref sessions) if sessions.len() == 1 && sessions[0].id == first.id
        ));
        assert!(matches!(
            database
                .list_active_user_sessions(EntityId::new(), base + 251)
                .await,
            Ok(ref sessions) if sessions.is_empty()
        ));
        assert!(matches!(
            database.list_active_user_sessions(owner.id, -1).await,
            Err(PersistenceError::InvalidTimestamp)
        ));
        assert!(matches!(
            set_user_auth_revision(database, owner.id, 1).await,
            Ok(1)
        ));
        let current_revision_sibling = auth_session_fixture(
            owner.id,
            200,
            Revision::from_value(1),
            base + 260,
            base + 10_000,
        );
        assert!(
            database
                .create_auth_session(
                    &current_revision_sibling,
                    &login_security_event(200, base + 260, LoginSecurityReason::LoginSucceeded,),
                )
                .await
                .is_ok()
        );
        assert!(matches!(
            database
                .list_active_user_sessions(owner.id, base + 261)
                .await,
            Ok(ref sessions)
                if sessions.len() == 1 && sessions[0].id == current_revision_sibling.id
        ));
        let user_revoke_event = session_security_event(
            &current_revision_sibling,
            201,
            base + 262,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_current_session_with_event(
                    owner.id,
                    current_revision_sibling.id,
                    base + 262,
                    SessionRevocationReason::UserRevoked,
                    &user_revoke_event,
                )
                .await,
            Ok(true)
        ));
        assert!(matches!(
            login_security_event_count(database, user_revoke_event.id).await,
            Ok(1)
        ));
        assert!(matches!(
            database.list_user_sessions(owner.id).await,
            Ok(ref sessions)
                if sessions.iter().any(|session|
                    session.id == current_revision_sibling.id
                        && session.revoked_reason == Some(SessionRevocationReason::UserRevoked)
                )
        ));
        let repeated_user_revoke_event = session_security_event(
            &current_revision_sibling,
            204,
            base + 263,
            LoginSecurityReason::SessionRevoked,
        );
        assert!(matches!(
            database
                .revoke_current_session_with_event(
                    owner.id,
                    current_revision_sibling.id,
                    base + 263,
                    SessionRevocationReason::UserRevoked,
                    &repeated_user_revoke_event,
                )
                .await,
            Ok(false)
        ));
        assert!(matches!(
            login_security_event_count(database, repeated_user_revoke_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            login_security_event_count(database, user_revoke_event.id).await,
            Ok(1)
        ));
        assert!(matches!(
            set_user_auth_revision(database, owner.id, 0).await,
            Ok(1)
        ));
        assert!(matches!(
            database
                .list_active_user_sessions(owner.id, base + 263)
                .await,
            Ok(ref sessions) if sessions.len() == 1 && sessions[0].id == first.id
        ));
        for (marker, revocation_reason, event_reason) in [
            (
                202,
                SessionRevocationReason::Logout,
                LoginSecurityReason::SessionRevoked,
            ),
            (
                203,
                SessionRevocationReason::UserRevoked,
                LoginSecurityReason::Logout,
            ),
        ] {
            let cross_paired_event =
                session_security_event(&first, marker, base + 299, event_reason);
            assert!(matches!(
                database
                    .revoke_current_session_with_event(
                        owner.id,
                        first.id,
                        base + 299,
                        revocation_reason,
                        &cross_paired_event,
                    )
                    .await,
                Err(PersistenceError::InvalidSessionRevocationEvent)
            ));
            assert!(matches!(
                login_security_event_count(database, cross_paired_event.id).await,
                Ok(0)
            ));
        }
        let mut missing_context_event =
            session_security_event(&first, 24, base + 299, LoginSecurityReason::Logout);
        missing_context_event.account_hmac = None;
        assert!(matches!(
            database
                .revoke_current_session_with_event(
                    owner.id,
                    first.id,
                    base + 299,
                    SessionRevocationReason::Logout,
                    &missing_context_event,
                )
                .await,
            Err(PersistenceError::InvalidSessionRevocationEvent)
        ));
        assert!(matches!(
            login_security_event_count(database, missing_context_event.id).await,
            Ok(0)
        ));
        let wrong_time_event =
            session_security_event(&first, 25, base + 298, LoginSecurityReason::Logout);
        assert!(matches!(
            database
                .revoke_current_session_with_event(
                    owner.id,
                    first.id,
                    base + 299,
                    SessionRevocationReason::Logout,
                    &wrong_time_event,
                )
                .await,
            Err(PersistenceError::InvalidSessionRevocationEvent)
        ));
        assert!(matches!(
            login_security_event_count(database, wrong_time_event.id).await,
            Ok(0)
        ));
        let mut duplicate_event =
            session_security_event(&first, 26, base + 299, LoginSecurityReason::Logout);
        duplicate_event.id = first_event.id;
        assert!(matches!(
            database
                .revoke_current_session_with_event(
                    owner.id,
                    first.id,
                    base + 299,
                    SessionRevocationReason::Logout,
                    &duplicate_event,
                )
                .await,
            Err(PersistenceError::Sql(_))
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(&first, None, base + 299))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(_))
        ));
        let mut logout_event =
            session_security_event(&first, 22, base + 300, LoginSecurityReason::Logout);
        logout_event.digest_key_version = 2;
        logout_event.ip_prefix_hmac = Some([0xee; 32]);
        logout_event.user_agent_hash = Some([0xdd; 32]);
        assert!(matches!(
            database
                .revoke_current_session_with_event(
                    owner.id,
                    first.id,
                    base + 300,
                    SessionRevocationReason::Logout,
                    &logout_event,
                )
                .await,
            Ok(true)
        ));
        assert!(matches!(
            login_security_event_count(database, logout_event.id).await,
            Ok(1)
        ));
        let idempotent_event =
            session_security_event(&first, 23, base + 301, LoginSecurityReason::Logout);
        assert!(matches!(
            database
                .revoke_current_session_with_event(
                    owner.id,
                    first.id,
                    base + 301,
                    SessionRevocationReason::Logout,
                    &idempotent_event,
                )
                .await,
            Ok(false)
        ));
        assert!(matches!(
            login_security_event_count(database, idempotent_event.id).await,
            Ok(0)
        ));

        let current =
            auth_session_fixture(owner.id, 3, Revision::initial(), base + 300, base + 10_000);
        let sibling =
            auth_session_fixture(owner.id, 4, Revision::initial(), base + 301, base + 10_000);
        assert!(
            database
                .create_auth_session(
                    &current,
                    &login_security_event(3, base + 300, LoginSecurityReason::LoginSucceeded),
                )
                .await
                .is_ok()
        );
        assert!(
            database
                .create_auth_session(
                    &sibling,
                    &login_security_event(4, base + 301, LoginSecurityReason::LoginSucceeded),
                )
                .await
                .is_ok()
        );
        let logout_rotation_guard_seen_at_ms = base + 360;
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &current,
                    None,
                    logout_rotation_guard_seen_at_ms,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                if authenticated.session.last_seen_at_ms == logout_rotation_guard_seen_at_ms
        ));
        let logout_rotation_rollback_at_ms = logout_rotation_guard_seen_at_ms - 1;
        let mut rollback_logout_rotation = auth_session_fixture(
            owner.id,
            210,
            Revision::from_value(1),
            logout_rotation_rollback_at_ms,
            base + 10_000,
        );
        rollback_logout_rotation.authenticated_at_ms = current.authenticated_at_ms;
        rollback_logout_rotation.recent_auth_at_ms = current.recent_auth_at_ms;
        let rollback_logout_rotation_event = session_security_event(
            &rollback_logout_rotation,
            210,
            logout_rotation_rollback_at_ms,
            LoginSecurityReason::LogoutAll,
        );
        assert!(matches!(
            database
                .logout_all_sessions_and_rotate(
                    owner.id,
                    current.id,
                    Revision::initial(),
                    &rollback_logout_rotation,
                    &rollback_logout_rotation_event,
                    logout_rotation_rollback_at_ms,
                )
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(matches!(
            login_security_event_count(database, rollback_logout_rotation_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &current,
                    None,
                    logout_rotation_guard_seen_at_ms + 1,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(_))
        ));
        let rotate_at_ms = base + 400;
        let mut replacement = auth_session_fixture(
            owner.id,
            5,
            Revision::from_value(1),
            rotate_at_ms,
            base + 10_000,
        );
        replacement.authenticated_at_ms = current.authenticated_at_ms;
        replacement.recent_auth_at_ms = current.recent_auth_at_ms;
        let rotation_event = login_security_event(5, rotate_at_ms, LoginSecurityReason::LogoutAll);
        let mut missing_account_rotation_event = rotation_event.clone();
        missing_account_rotation_event.account_hmac = None;
        assert!(matches!(
            database
                .logout_all_sessions_and_rotate(
                    owner.id,
                    current.id,
                    Revision::initial(),
                    &replacement,
                    &missing_account_rotation_event,
                    rotate_at_ms,
                )
                .await,
            Err(PersistenceError::InvalidSessionRotation)
        ));
        assert!(matches!(
            login_security_event_count(database, missing_account_rotation_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            set_user_login_snapshot(
                database,
                owner.id,
                owner.username.as_str(),
                &owner.password_hash,
                owner.role.as_str(),
                "active",
                owner.principal_label.as_str(),
                owner.force_password_change,
                1,
            )
            .await,
            Ok(1)
        ));
        assert!(matches!(
            database
                .logout_all_sessions_and_rotate(
                    owner.id,
                    current.id,
                    Revision::initial(),
                    &replacement,
                    &rotation_event,
                    rotate_at_ms,
                )
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(matches!(
            login_security_event_count(database, rotation_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            set_user_login_snapshot(
                database,
                owner.id,
                owner.username.as_str(),
                &owner.password_hash,
                owner.role.as_str(),
                "active",
                owner.principal_label.as_str(),
                owner.force_password_change,
                0,
            )
            .await,
            Ok(1)
        ));
        let rotation = database
            .logout_all_sessions_and_rotate(
                owner.id,
                current.id,
                Revision::initial(),
                &replacement,
                &rotation_event,
                rotate_at_ms,
            )
            .await;
        assert!(matches!(
            rotation,
            Ok(result)
                if result.kept_current
                    && result.auth_revision == Revision::from_value(1)
                    && result.revoked_sessions == 3
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(&current, None, rotate_at_ms + 1))
                .await,
            Ok(SessionAuthenticationOutcome::InvalidSession)
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &replacement,
                    Some(replacement.csrf_hmac),
                    rotate_at_ms + 1,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                if authenticated.session.authenticated_at_ms == current.authenticated_at_ms
                    && authenticated.session.recent_auth_at_ms == current.recent_auth_at_ms
        ));
        let logout_all = database
            .logout_all_sessions(owner.id, rotate_at_ms + 2)
            .await;
        assert!(matches!(
            logout_all,
            Ok(result)
                if !result.kept_current
                    && result.auth_revision == Revision::from_value(2)
                    && result.revoked_sessions == 1
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &replacement,
                    None,
                    rotate_at_ms + 3,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::InvalidSession)
        ));
        let after_logout = auth_session_fixture(
            owner.id,
            6,
            Revision::from_value(2),
            rotate_at_ms + 4,
            base + 10_000,
        );
        assert!(database
            .create_auth_session(
                &after_logout,
                &login_security_event(
                    6,
                    rotate_at_ms + 4,
                    LoginSecurityReason::LoginSucceeded,
                ),
            )
            .await
            .is_ok());
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &after_logout,
                    None,
                    rotate_at_ms + 5,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                if authenticated.session.auth_revision == Revision::from_value(2)
        ));

        let upgraded_hash = password_hash_fixture('B');
        let second_upgraded_hash = password_hash_fixture('C');
        let final_password_hash = password_hash_fixture('D');
        let competing_rehash_hash = password_hash_fixture('E');
        let third_rehash_hash = password_hash_fixture('F');
        let actor_drift_hash = password_hash_fixture('G');
        let actor_replacement_hash = password_hash_fixture('H');
        let initial_login_snapshot = credentials_snapshot(database, &normalized).await;
        assert!(
            initial_login_snapshot.password_hash == owner.password_hash
                && initial_login_snapshot.user_revision == Revision::initial()
                && initial_login_snapshot.auth_revision == Revision::from_value(2)
                && initial_login_snapshot.password_changed_at_ms == owner.created_at_ms
        );
        let rehash_at_ms = rotate_at_ms + 10;
        let rehash_session = auth_session_fixture(
            owner.id,
            7,
            Revision::from_value(2),
            rehash_at_ms,
            base + 10_000,
        );
        let mut duplicate_rehash_event =
            login_security_event(7, rehash_at_ms, LoginSecurityReason::LoginSucceeded);
        duplicate_rehash_event.id = first_event.id;
        assert!(matches!(
            database
                .create_auth_session_with_optional_password_upgrade(
                    &rehash_session,
                    &duplicate_rehash_event,
                    &initial_login_snapshot,
                    Some(&upgraded_hash),
                )
                .await,
            Err(PersistenceError::Sql(_))
        ));
        assert!(matches!(
            database
                .user_credentials_by_normalized_username(&normalized)
                .await,
            Ok(Some(ref credentials))
                if credentials.password_hash == owner.password_hash
                    && credentials.user_revision == Revision::initial()
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &rehash_session,
                    None,
                    rehash_at_ms + 1,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::InvalidSession)
        ));
        let rehash_event =
            login_security_event(7, rehash_at_ms, LoginSecurityReason::LoginSucceeded);
        assert!(
            database
                .create_auth_session_with_optional_password_upgrade(
                    &rehash_session,
                    &rehash_event,
                    &initial_login_snapshot,
                    Some(&upgraded_hash),
                )
                .await
                .is_ok()
        );
        assert!(matches!(
            database
                .user_credentials_by_normalized_username(&normalized)
                .await,
            Ok(Some(ref credentials))
                if credentials.password_hash == upgraded_hash
                    && credentials.user_revision == Revision::from_value(1)
                    && credentials.auth_revision == Revision::from_value(2)
                    && credentials.password_changed_at_ms == owner.created_at_ms
        ));
        assert!(matches!(
            auth_state_timestamps(database, owner.id).await,
            Ok(Some((2, password_changed_at_ms, updated_at_ms)))
                if password_changed_at_ms == owner.created_at_ms
                    && updated_at_ms >= rehash_at_ms
        ));
        assert!(matches!(
            database
                .upgrade_password_hash_if_current(
                    owner.id,
                    Revision::from_value(1),
                    &upgraded_hash,
                    &second_upgraded_hash,
                    rehash_at_ms + 1,
                )
                .await,
            Ok(true)
        ));
        assert!(matches!(
            database
                .upgrade_password_hash_if_current(
                    owner.id,
                    Revision::from_value(1),
                    &upgraded_hash,
                    &final_password_hash,
                    rehash_at_ms + 2,
                )
                .await,
            Ok(false)
        ));
        assert!(matches!(
            auth_state_timestamps(database, owner.id).await,
            Ok(Some((2, password_changed_at_ms, updated_at_ms)))
                if password_changed_at_ms == owner.created_at_ms
                    && updated_at_ms > rehash_at_ms
        ));
        let concurrent_login_snapshot = credentials_snapshot(database, &normalized).await;
        assert!(
            concurrent_login_snapshot.password_hash == second_upgraded_hash
                && concurrent_login_snapshot.user_revision == Revision::from_value(2)
                && concurrent_login_snapshot.auth_revision == Revision::from_value(2)
                && concurrent_login_snapshot.password_changed_at_ms == owner.created_at_ms
        );
        let rehash_race_at_ms = rehash_at_ms + 3;
        let rehash_race_session = auth_session_fixture(
            owner.id,
            8,
            Revision::from_value(2),
            rehash_race_at_ms,
            base + 10_000,
        );
        let rehash_race_event =
            login_security_event(8, rehash_race_at_ms, LoginSecurityReason::LoginSucceeded);
        let second_race_session = auth_session_fixture(
            owner.id,
            36,
            Revision::from_value(2),
            rehash_race_at_ms + 1,
            base + 10_000,
        );
        let second_race_event = login_security_event(
            36,
            rehash_race_at_ms + 1,
            LoginSecurityReason::LoginSucceeded,
        );
        let third_race_session = auth_session_fixture(
            owner.id,
            37,
            Revision::from_value(2),
            rehash_race_at_ms + 2,
            base + 10_000,
        );
        let third_race_event = login_security_event(
            37,
            rehash_race_at_ms + 2,
            LoginSecurityReason::LoginSucceeded,
        );
        let (first_race, second_race, third_race) = tokio::join!(
            database.create_auth_session_with_optional_password_upgrade(
                &rehash_race_session,
                &rehash_race_event,
                &concurrent_login_snapshot,
                Some(&final_password_hash),
            ),
            database.create_auth_session_with_optional_password_upgrade(
                &second_race_session,
                &second_race_event,
                &concurrent_login_snapshot,
                Some(&competing_rehash_hash),
            ),
            database.create_auth_session_with_optional_password_upgrade(
                &third_race_session,
                &third_race_event,
                &concurrent_login_snapshot,
                Some(&third_rehash_hash),
            ),
        );
        assert!(first_race.is_ok() && second_race.is_ok() && third_race.is_ok());
        for (session, event) in [
            (&rehash_race_session, &rehash_race_event),
            (&second_race_session, &second_race_event),
            (&third_race_session, &third_race_event),
        ] {
            assert!(matches!(
                login_security_event_count(database, event.id).await,
                Ok(1)
            ));
            assert!(matches!(
                database
                    .authenticate_session(&session_authentication(
                        session,
                        None,
                        rehash_race_at_ms + 3,
                    ))
                    .await,
                Ok(SessionAuthenticationOutcome::Authenticated(_))
            ));
        }
        let post_rehash_snapshot = credentials_snapshot(database, &normalized).await;
        assert!(
            post_rehash_snapshot.user_revision == Revision::from_value(3)
                && post_rehash_snapshot.auth_revision == Revision::from_value(2)
                && post_rehash_snapshot.password_changed_at_ms == owner.created_at_ms
                && (post_rehash_snapshot.password_hash == final_password_hash
                    || post_rehash_snapshot.password_hash == competing_rehash_hash
                    || post_rehash_snapshot.password_hash == third_rehash_hash)
        );

        for (offset, marker, username, role, status, principal_label, force_password_change) in [
            (
                1_i64,
                50_u8,
                "ConcurrentOwner",
                post_rehash_snapshot.role.as_str(),
                post_rehash_snapshot.status.as_str(),
                post_rehash_snapshot.principal_label.as_str(),
                post_rehash_snapshot.force_password_change,
            ),
            (
                2_i64,
                51_u8,
                post_rehash_snapshot.username.as_str(),
                "admin",
                post_rehash_snapshot.status.as_str(),
                post_rehash_snapshot.principal_label.as_str(),
                post_rehash_snapshot.force_password_change,
            ),
            (
                3_i64,
                52_u8,
                post_rehash_snapshot.username.as_str(),
                post_rehash_snapshot.role.as_str(),
                "disabled",
                post_rehash_snapshot.principal_label.as_str(),
                post_rehash_snapshot.force_password_change,
            ),
            (
                4_i64,
                53_u8,
                post_rehash_snapshot.username.as_str(),
                post_rehash_snapshot.role.as_str(),
                post_rehash_snapshot.status.as_str(),
                "owner-drift",
                post_rehash_snapshot.force_password_change,
            ),
            (
                5_i64,
                54_u8,
                post_rehash_snapshot.username.as_str(),
                post_rehash_snapshot.role.as_str(),
                post_rehash_snapshot.status.as_str(),
                post_rehash_snapshot.principal_label.as_str(),
                !post_rehash_snapshot.force_password_change,
            ),
        ] {
            assert!(matches!(
                set_user_login_snapshot(
                    database,
                    owner.id,
                    username,
                    &actor_drift_hash,
                    role,
                    status,
                    principal_label,
                    force_password_change,
                    4,
                )
                .await,
                Ok(1)
            ));
            let drift_at_ms = rehash_race_at_ms + offset;
            let drift_session = auth_session_fixture(
                owner.id,
                marker,
                Revision::from_value(2),
                drift_at_ms,
                base + 10_000,
            );
            let drift_event =
                login_security_event(marker, drift_at_ms, LoginSecurityReason::LoginSucceeded);
            assert!(matches!(
                database
                    .create_auth_session_with_optional_password_upgrade(
                        &drift_session,
                        &drift_event,
                        &post_rehash_snapshot,
                        Some(&actor_replacement_hash),
                    )
                    .await,
                Err(PersistenceError::SessionPrincipalUnavailable)
            ));
            assert!(matches!(
                login_security_event_count(database, drift_event.id).await,
                Ok(0)
            ));
            assert!(matches!(
                database
                    .authenticate_session(&session_authentication(
                        &drift_session,
                        None,
                        drift_at_ms + 1,
                    ))
                    .await,
                Ok(SessionAuthenticationOutcome::InvalidSession)
            ));
            assert!(matches!(
                set_user_login_snapshot(
                    database,
                    owner.id,
                    post_rehash_snapshot.username.as_str(),
                    &post_rehash_snapshot.password_hash,
                    post_rehash_snapshot.role.as_str(),
                    post_rehash_snapshot.status.as_str(),
                    post_rehash_snapshot.principal_label.as_str(),
                    post_rehash_snapshot.force_password_change,
                    3,
                )
                .await,
                Ok(1)
            ));
        }

        assert!(matches!(
            set_user_login_snapshot(
                database,
                owner.id,
                post_rehash_snapshot.username.as_str(),
                &actor_drift_hash,
                post_rehash_snapshot.role.as_str(),
                post_rehash_snapshot.status.as_str(),
                post_rehash_snapshot.principal_label.as_str(),
                post_rehash_snapshot.force_password_change,
                4,
            )
            .await,
            Ok(1)
        ));
        let none_drift_at_ms = rehash_race_at_ms + 6;
        let none_drift_session = auth_session_fixture(
            owner.id,
            55,
            Revision::from_value(2),
            none_drift_at_ms,
            base + 10_000,
        );
        let none_drift_event =
            login_security_event(55, none_drift_at_ms, LoginSecurityReason::LoginSucceeded);
        assert!(matches!(
            database
                .create_auth_session_with_optional_password_upgrade(
                    &none_drift_session,
                    &none_drift_event,
                    &post_rehash_snapshot,
                    None,
                )
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(matches!(
            login_security_event_count(database, none_drift_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            set_user_login_snapshot(
                database,
                owner.id,
                post_rehash_snapshot.username.as_str(),
                &post_rehash_snapshot.password_hash,
                post_rehash_snapshot.role.as_str(),
                post_rehash_snapshot.status.as_str(),
                post_rehash_snapshot.principal_label.as_str(),
                post_rehash_snapshot.force_password_change,
                3,
            )
            .await,
            Ok(1)
        ));

        let recent_auth_at_ms = rotate_at_ms + 100;
        let recent_auth_session = auth_session_fixture(
            owner.id,
            9,
            Revision::from_value(2),
            recent_auth_at_ms,
            base + 10_000,
        );
        let reauthentication_guard_seen_at_ms = recent_auth_at_ms - 1;
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &after_logout,
                    None,
                    reauthentication_guard_seen_at_ms,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                if authenticated.session.last_seen_at_ms == reauthentication_guard_seen_at_ms
        ));
        let reauthentication_rollback_at_ms = reauthentication_guard_seen_at_ms - 1;
        let rollback_reauthentication_session = auth_session_fixture(
            owner.id,
            211,
            Revision::from_value(2),
            reauthentication_rollback_at_ms,
            base + 10_000,
        );
        let rollback_reauthentication_event = session_security_event(
            &rollback_reauthentication_session,
            211,
            reauthentication_rollback_at_ms,
            LoginSecurityReason::ReauthenticationSucceeded,
        );
        assert!(matches!(
            database
                .rotate_current_session(
                    owner.id,
                    after_logout.id,
                    Revision::from_value(3),
                    &rollback_reauthentication_session,
                    &rollback_reauthentication_event,
                    reauthentication_rollback_at_ms,
                )
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(matches!(
            login_security_event_count(database, rollback_reauthentication_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &after_logout,
                    None,
                    recent_auth_at_ms,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(_))
        ));
        let wrong_reauthentication_event = session_security_event(
            &recent_auth_session,
            19,
            recent_auth_at_ms,
            LoginSecurityReason::PasswordChanged,
        );
        assert!(matches!(
            database
                .rotate_current_session(
                    owner.id,
                    after_logout.id,
                    Revision::from_value(3),
                    &recent_auth_session,
                    &wrong_reauthentication_event,
                    recent_auth_at_ms,
                )
                .await,
            Err(PersistenceError::InvalidSessionRotation)
        ));
        assert!(matches!(
            login_security_event_count(database, wrong_reauthentication_event.id).await,
            Ok(0)
        ));
        let mut extended_recent_auth_session = recent_auth_session.clone();
        extended_recent_auth_session.absolute_expires_at_ms = base + 10_001;
        let extended_reauthentication_event = session_security_event(
            &extended_recent_auth_session,
            20,
            recent_auth_at_ms,
            LoginSecurityReason::ReauthenticationSucceeded,
        );
        assert!(matches!(
            database
                .rotate_current_session(
                    owner.id,
                    after_logout.id,
                    Revision::from_value(3),
                    &extended_recent_auth_session,
                    &extended_reauthentication_event,
                    recent_auth_at_ms,
                )
                .await,
            Err(PersistenceError::InvalidSessionRotation)
        ));
        assert!(matches!(
            login_security_event_count(database, extended_reauthentication_event.id).await,
            Ok(0)
        ));
        let reauthentication_event = session_security_event(
            &recent_auth_session,
            21,
            recent_auth_at_ms,
            LoginSecurityReason::ReauthenticationSucceeded,
        );
        let mut missing_account_reauthentication_event = session_security_event(
            &recent_auth_session,
            212,
            recent_auth_at_ms,
            LoginSecurityReason::ReauthenticationSucceeded,
        );
        missing_account_reauthentication_event.account_hmac = None;
        assert!(matches!(
            database
                .rotate_current_session(
                    owner.id,
                    after_logout.id,
                    Revision::from_value(3),
                    &recent_auth_session,
                    &missing_account_reauthentication_event,
                    recent_auth_at_ms,
                )
                .await,
            Err(PersistenceError::InvalidSessionRotation)
        ));
        assert!(matches!(
            login_security_event_count(database, missing_account_reauthentication_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            set_user_login_snapshot(
                database,
                owner.id,
                post_rehash_snapshot.username.as_str(),
                &post_rehash_snapshot.password_hash,
                post_rehash_snapshot.role.as_str(),
                post_rehash_snapshot.status.as_str(),
                post_rehash_snapshot.principal_label.as_str(),
                post_rehash_snapshot.force_password_change,
                4,
            )
            .await,
            Ok(1)
        ));
        assert!(matches!(
            database
                .rotate_current_session(
                    owner.id,
                    after_logout.id,
                    Revision::from_value(3),
                    &recent_auth_session,
                    &reauthentication_event,
                    recent_auth_at_ms,
                )
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(matches!(
            login_security_event_count(database, reauthentication_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            set_user_login_snapshot(
                database,
                owner.id,
                post_rehash_snapshot.username.as_str(),
                &post_rehash_snapshot.password_hash,
                post_rehash_snapshot.role.as_str(),
                post_rehash_snapshot.status.as_str(),
                post_rehash_snapshot.principal_label.as_str(),
                post_rehash_snapshot.force_password_change,
                3,
            )
            .await,
            Ok(1)
        ));
        assert!(matches!(
            database
                .rotate_current_session(
                    owner.id,
                    after_logout.id,
                    Revision::from_value(3),
                    &recent_auth_session,
                    &reauthentication_event,
                    recent_auth_at_ms,
                )
                .await,
            Ok(ref summary)
                if summary.id == recent_auth_session.id
                    && summary.auth_revision == Revision::from_value(2)
                    && summary.recent_auth_at_ms == recent_auth_at_ms
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &after_logout,
                    None,
                    recent_auth_at_ms + 1,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::InvalidSession)
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &rehash_session,
                    None,
                    recent_auth_at_ms + 1,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(_))
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &recent_auth_session,
                    None,
                    recent_auth_at_ms + 1,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                if authenticated.session.recent_auth_at_ms == recent_auth_at_ms
        ));
        let repeated_reauthentication_session = auth_session_fixture(
            owner.id,
            10,
            Revision::from_value(2),
            recent_auth_at_ms + 1,
            base + 10_000,
        );
        let repeated_reauthentication_event = session_security_event(
            &repeated_reauthentication_session,
            22,
            recent_auth_at_ms + 1,
            LoginSecurityReason::ReauthenticationSucceeded,
        );
        assert!(matches!(
            database
                .rotate_current_session(
                    owner.id,
                    after_logout.id,
                    Revision::from_value(3),
                    &repeated_reauthentication_session,
                    &repeated_reauthentication_event,
                    recent_auth_at_ms + 1,
                )
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(matches!(
            login_security_event_count(database, repeated_reauthentication_event.id).await,
            Ok(0)
        ));

        let password_change_at_ms = rotate_at_ms + 200;
        let mut password_session = auth_session_fixture(
            owner.id,
            11,
            Revision::from_value(3),
            password_change_at_ms,
            base + 10_000,
        );
        password_session.authenticated_at_ms = recent_auth_session.authenticated_at_ms;
        password_session.recent_auth_at_ms = recent_auth_session.recent_auth_at_ms;
        let wrong_password_event = session_security_event(
            &password_session,
            23,
            password_change_at_ms,
            LoginSecurityReason::ReauthenticationSucceeded,
        );
        assert!(matches!(
            database
                .change_password_and_rotate(password_change_rotation(
                    owner.id,
                    recent_auth_session.id,
                    Revision::from_value(3),
                    &final_password_hash,
                    &password_session,
                    &wrong_password_event,
                    password_change_at_ms,
                ))
                .await,
            Err(PersistenceError::InvalidSessionRotation)
        ));
        assert!(matches!(
            login_security_event_count(database, wrong_password_event.id).await,
            Ok(0)
        ));
        let mut extended_password_session = password_session.clone();
        extended_password_session.absolute_expires_at_ms = base + 10_001;
        let extended_password_event = session_security_event(
            &extended_password_session,
            24,
            password_change_at_ms,
            LoginSecurityReason::PasswordChanged,
        );
        assert!(matches!(
            database
                .change_password_and_rotate(password_change_rotation(
                    owner.id,
                    recent_auth_session.id,
                    Revision::from_value(3),
                    &final_password_hash,
                    &extended_password_session,
                    &extended_password_event,
                    password_change_at_ms,
                ))
                .await,
            Err(PersistenceError::InvalidSessionRotation)
        ));
        assert!(matches!(
            database
                .authenticate_session_read_only(&session_authentication(
                    &recent_auth_session,
                    None,
                    password_change_at_ms + 1,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(_))
        ));
        let password_event = session_security_event(
            &password_session,
            25,
            password_change_at_ms,
            LoginSecurityReason::PasswordChanged,
        );
        assert!(matches!(
            set_user_login_snapshot(
                database,
                owner.id,
                post_rehash_snapshot.username.as_str(),
                &post_rehash_snapshot.password_hash,
                post_rehash_snapshot.role.as_str(),
                post_rehash_snapshot.status.as_str(),
                post_rehash_snapshot.principal_label.as_str(),
                post_rehash_snapshot.force_password_change,
                4,
            )
            .await,
            Ok(1)
        ));
        assert!(matches!(
            database
                .change_password_and_rotate(password_change_rotation(
                    owner.id,
                    recent_auth_session.id,
                    Revision::from_value(3),
                    &final_password_hash,
                    &password_session,
                    &password_event,
                    password_change_at_ms,
                ))
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(matches!(
            login_security_event_count(database, password_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            set_user_login_snapshot(
                database,
                owner.id,
                post_rehash_snapshot.username.as_str(),
                &post_rehash_snapshot.password_hash,
                post_rehash_snapshot.role.as_str(),
                post_rehash_snapshot.status.as_str(),
                post_rehash_snapshot.principal_label.as_str(),
                post_rehash_snapshot.force_password_change,
                3,
            )
            .await,
            Ok(1)
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &recent_auth_session,
                    None,
                    password_change_at_ms + 1,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                if authenticated.session.last_seen_at_ms == password_change_at_ms + 1
        ));
        assert!(matches!(
            database
                .change_password_and_rotate(password_change_rotation(
                    owner.id,
                    recent_auth_session.id,
                    Revision::from_value(3),
                    &final_password_hash,
                    &password_session,
                    &password_event,
                    password_change_at_ms,
                ))
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(matches!(
            login_security_event_count(database, password_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            database
                .user_credentials_by_normalized_username(&normalized)
                .await,
            Ok(Some(ref credentials))
                if credentials.password_hash == post_rehash_snapshot.password_hash
                    && credentials.user_revision == Revision::from_value(3)
                    && credentials.auth_revision == Revision::from_value(2)
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &recent_auth_session,
                    None,
                    password_change_at_ms + 2,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(_))
        ));

        let password_change_at_ms = password_change_at_ms + 2;
        let mut password_session = auth_session_fixture(
            owner.id,
            11,
            Revision::from_value(3),
            password_change_at_ms,
            base + 10_000,
        );
        password_session.authenticated_at_ms = recent_auth_session.authenticated_at_ms;
        password_session.recent_auth_at_ms = recent_auth_session.recent_auth_at_ms;
        let mut missing_account_password_event = session_security_event(
            &password_session,
            213,
            password_change_at_ms,
            LoginSecurityReason::PasswordChanged,
        );
        missing_account_password_event.account_hmac = None;
        assert!(matches!(
            database
                .change_password_and_rotate(password_change_rotation(
                    owner.id,
                    recent_auth_session.id,
                    Revision::from_value(3),
                    &final_password_hash,
                    &password_session,
                    &missing_account_password_event,
                    password_change_at_ms,
                ))
                .await,
            Err(PersistenceError::InvalidSessionRotation)
        ));
        assert!(matches!(
            login_security_event_count(database, missing_account_password_event.id).await,
            Ok(0)
        ));
        let password_event = session_security_event(
            &password_session,
            25,
            password_change_at_ms,
            LoginSecurityReason::PasswordChanged,
        );
        assert!(matches!(
            database
                .change_password_and_rotate(password_change_rotation(
                    owner.id,
                    recent_auth_session.id,
                    Revision::from_value(3),
                    &final_password_hash,
                    &password_session,
                    &password_event,
                    password_change_at_ms,
                ))
                .await,
            Ok(ref result)
                if result.session.id == password_session.id
                    && result.revoked_sessions == 5
                    && result.auth_revision == Revision::from_value(3)
                    && result.session.authenticated_at_ms
                        == recent_auth_session.authenticated_at_ms
                    && result.session.recent_auth_at_ms == recent_auth_session.recent_auth_at_ms
        ));
        assert!(matches!(
            database
                .user_credentials_by_normalized_username(&normalized)
                .await,
            Ok(Some(ref credentials))
                if credentials.password_hash == final_password_hash
                    && !credentials.force_password_change
                    && credentials.user_revision == Revision::from_value(4)
                    && credentials.auth_revision == Revision::from_value(3)
                    && credentials.password_changed_at_ms == password_change_at_ms
        ));
        assert!(matches!(
            login_security_event_count(database, password_event.id).await,
            Ok(1)
        ));
        for revoked in [
            &rehash_session,
            &rehash_race_session,
            &second_race_session,
            &third_race_session,
            &recent_auth_session,
        ] {
            assert!(matches!(
                database
                    .authenticate_session(&session_authentication(
                        revoked,
                        None,
                        password_change_at_ms + 1,
                    ))
                    .await,
                Ok(SessionAuthenticationOutcome::InvalidSession)
            ));
        }
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &password_session,
                    None,
                    password_change_at_ms + 1,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                if authenticated.session.authenticated_at_ms
                    == recent_auth_session.authenticated_at_ms
                    && authenticated.session.recent_auth_at_ms
                        == recent_auth_session.recent_auth_at_ms
        ));
        let stale_password_state_session = auth_session_fixture(
            owner.id,
            12,
            Revision::from_value(3),
            password_change_at_ms + 2,
            base + 10_000,
        );
        let stale_password_state_event = login_security_event(
            12,
            password_change_at_ms + 2,
            LoginSecurityReason::LoginSucceeded,
        );
        assert!(matches!(
            database
                .create_auth_session_with_optional_password_upgrade(
                    &stale_password_state_session,
                    &stale_password_state_event,
                    &post_rehash_snapshot,
                    None,
                )
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(matches!(
            login_security_event_count(database, stale_password_state_event.id).await,
            Ok(0)
        ));
        let mut rolled_back_password_session = auth_session_fixture(
            owner.id,
            12,
            Revision::from_value(4),
            password_change_at_ms + 2,
            base + 10_000,
        );
        rolled_back_password_session.authenticated_at_ms = password_session.authenticated_at_ms;
        rolled_back_password_session.recent_auth_at_ms = password_session.recent_auth_at_ms;
        let mut duplicate_password_event = session_security_event(
            &rolled_back_password_session,
            26,
            password_change_at_ms + 2,
            LoginSecurityReason::PasswordChanged,
        );
        duplicate_password_event.id = password_event.id;
        assert!(matches!(
            database
                .change_password_and_rotate(password_change_rotation(
                    owner.id,
                    password_session.id,
                    Revision::from_value(4),
                    &second_upgraded_hash,
                    &rolled_back_password_session,
                    &duplicate_password_event,
                    password_change_at_ms + 2,
                ))
                .await,
            Err(PersistenceError::Sql(_))
        ));
        assert!(matches!(
            database
                .user_credentials_by_normalized_username(&normalized)
                .await,
            Ok(Some(ref credentials))
                if credentials.password_hash == final_password_hash
                    && credentials.user_revision == Revision::from_value(4)
                    && credentials.auth_revision == Revision::from_value(3)
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &password_session,
                    None,
                    password_change_at_ms + 3,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(_))
        ));
        assert!(matches!(
            set_session_revision(database, password_session.id, i64::MAX).await,
            Ok(1)
        ));
        let mut saturated_revision_session = auth_session_fixture(
            owner.id,
            15,
            Revision::from_value(4),
            password_change_at_ms + 4,
            base + 10_000,
        );
        saturated_revision_session.authenticated_at_ms = password_session.authenticated_at_ms;
        saturated_revision_session.recent_auth_at_ms = password_session.recent_auth_at_ms;
        let saturated_revision_event = session_security_event(
            &saturated_revision_session,
            32,
            password_change_at_ms + 4,
            LoginSecurityReason::PasswordChanged,
        );
        assert!(matches!(
            database
                .change_password_and_rotate(password_change_rotation(
                    owner.id,
                    password_session.id,
                    Revision::from_value(4),
                    &second_upgraded_hash,
                    &saturated_revision_session,
                    &saturated_revision_event,
                    password_change_at_ms + 4,
                ))
                .await,
            Err(PersistenceError::RevisionOutOfRange)
        ));
        assert!(matches!(
            login_security_event_count(database, saturated_revision_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            auth_state_timestamps(database, owner.id).await,
            Ok(Some((3, changed_at_ms, _))) if changed_at_ms == password_change_at_ms
        ));
        assert!(matches!(
            database
                .user_credentials_by_normalized_username(&normalized)
                .await,
            Ok(Some(ref credentials))
                if credentials.password_hash == final_password_hash
                    && credentials.user_revision == Revision::from_value(4)
                    && credentials.auth_revision == Revision::from_value(3)
        ));
        assert!(matches!(
            set_session_revision(database, password_session.id, 0).await,
            Ok(1)
        ));
        let stale_logout_session_revision = password_session.revision;

        let audited_logout_at_ms = rotate_at_ms + 300;
        let audited_logout_sibling = auth_session_fixture(
            owner.id,
            30,
            Revision::from_value(3),
            audited_logout_at_ms - 1,
            base + 10_000,
        );
        assert!(
            database
                .create_auth_session(
                    &audited_logout_sibling,
                    &login_security_event(
                        30,
                        audited_logout_at_ms - 1,
                        LoginSecurityReason::LoginSucceeded,
                    ),
                )
                .await
                .is_ok()
        );
        let guarded_password_summary = match database
            .authenticate_session(&session_authentication(
                &password_session,
                None,
                audited_logout_at_ms - 1,
            ))
            .await
        {
            Ok(SessionAuthenticationOutcome::Authenticated(authenticated)) => {
                assert_eq!(
                    authenticated.session.last_seen_at_ms,
                    audited_logout_at_ms - 1
                );
                assert_ne!(
                    authenticated.session.revision, stale_logout_session_revision,
                    "the guard request must touch the session so logout-all exercises a stale snapshot"
                );
                authenticated.session
            }
            outcome => panic!("unexpected logout-all guard authentication: {outcome:?}"),
        };
        assert_eq!(password_session.revision, stale_logout_session_revision);
        assert_ne!(
            password_session.revision, guarded_password_summary.revision,
            "the caller snapshot must remain stale after the independent touch"
        );
        let audited_logout_rollback_at_ms = audited_logout_at_ms - 2;
        let rollback_audited_logout_event = session_security_event(
            &password_session,
            214,
            audited_logout_rollback_at_ms,
            LoginSecurityReason::LogoutAll,
        );
        assert!(matches!(
            database
                .logout_all_sessions_with_event(
                    owner.id,
                    password_session.id,
                    password_session.recent_auth_at_ms,
                    &rollback_audited_logout_event,
                    audited_logout_rollback_at_ms,
                )
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(matches!(
            login_security_event_count(database, rollback_audited_logout_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            auth_state_timestamps(database, owner.id).await,
            Ok(Some((3, _, _)))
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &password_session,
                    None,
                    audited_logout_at_ms,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(_))
        ));
        let mut missing_account_audited_logout_event = session_security_event(
            &password_session,
            215,
            audited_logout_at_ms,
            LoginSecurityReason::LogoutAll,
        );
        missing_account_audited_logout_event.account_hmac = None;
        assert!(matches!(
            database
                .logout_all_sessions_with_event(
                    owner.id,
                    password_session.id,
                    password_session.recent_auth_at_ms,
                    &missing_account_audited_logout_event,
                    audited_logout_at_ms,
                )
                .await,
            Err(PersistenceError::InvalidSessionRevocationEvent)
        ));
        assert!(matches!(
            login_security_event_count(database, missing_account_audited_logout_event.id).await,
            Ok(0)
        ));
        let audited_logout_event = session_security_event(
            &password_session,
            27,
            audited_logout_at_ms,
            LoginSecurityReason::LogoutAll,
        );
        let competing_logout_event = session_security_event(
            &password_session,
            28,
            audited_logout_at_ms,
            LoginSecurityReason::LogoutAll,
        );
        let (audited_logout, competing_logout) = tokio::join!(
            database.logout_all_sessions_with_event(
                owner.id,
                password_session.id,
                password_session.recent_auth_at_ms,
                &audited_logout_event,
                audited_logout_at_ms,
            ),
            database.logout_all_sessions_with_event(
                owner.id,
                password_session.id,
                password_session.recent_auth_at_ms,
                &competing_logout_event,
                audited_logout_at_ms,
            ),
        );
        match (&audited_logout, &competing_logout) {
            (Ok(result), Err(PersistenceError::SessionPrincipalUnavailable))
            | (Err(PersistenceError::SessionPrincipalUnavailable), Ok(result)) => {
                assert!(!result.kept_current);
                assert_eq!(result.revoked_sessions, 2);
                assert_eq!(result.auth_revision, Revision::from_value(4));
            }
            outcome => panic!("unexpected concurrent audited logout outcome: {outcome:?}"),
        }
        let persisted_logout_event_id = if audited_logout.is_ok() {
            audited_logout_event.id
        } else {
            competing_logout_event.id
        };
        let (audited_event_count, competing_event_count) = tokio::join!(
            login_security_event_count(database, audited_logout_event.id),
            login_security_event_count(database, competing_logout_event.id),
        );
        assert!(matches!(
            (audited_event_count, competing_event_count),
            (Ok(1), Ok(0)) | (Ok(0), Ok(1))
        ));
        for revoked in [&password_session, &audited_logout_sibling] {
            assert!(matches!(
                database
                    .authenticate_session(&session_authentication(
                        revoked,
                        None,
                        audited_logout_at_ms + 1,
                    ))
                    .await,
                Ok(SessionAuthenticationOutcome::InvalidSession)
            ));
        }
        let rollback_logout_session = auth_session_fixture(
            owner.id,
            13,
            Revision::from_value(4),
            audited_logout_at_ms + 10,
            base + 10_000,
        );
        assert!(
            database
                .create_auth_session(
                    &rollback_logout_session,
                    &login_security_event(
                        13,
                        audited_logout_at_ms + 10,
                        LoginSecurityReason::LoginSucceeded,
                    ),
                )
                .await
                .is_ok()
        );
        let stale_current_logout_event = session_security_event(
            &password_session,
            29,
            audited_logout_at_ms + 15,
            LoginSecurityReason::LogoutAll,
        );
        assert!(matches!(
            database
                .logout_all_sessions_with_event(
                    owner.id,
                    password_session.id,
                    password_session.recent_auth_at_ms,
                    &stale_current_logout_event,
                    audited_logout_at_ms + 15,
                )
                .await,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        assert!(matches!(
            login_security_event_count(database, stale_current_logout_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &rollback_logout_session,
                    None,
                    audited_logout_at_ms + 16,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(_))
        ));
        let mut duplicate_logout_event = session_security_event(
            &rollback_logout_session,
            31,
            audited_logout_at_ms + 20,
            LoginSecurityReason::LogoutAll,
        );
        duplicate_logout_event.id = persisted_logout_event_id;
        assert!(matches!(
            database
                .logout_all_sessions_with_event(
                    owner.id,
                    rollback_logout_session.id,
                    rollback_logout_session.recent_auth_at_ms,
                    &duplicate_logout_event,
                    audited_logout_at_ms + 20,
                )
                .await,
            Err(PersistenceError::Sql(_))
        ));
        assert!(matches!(
            auth_state_timestamps(database, owner.id).await,
            Ok(Some((4, _, _)))
        ));
        assert!(matches!(
            database
                .authenticate_session(&session_authentication(
                    &rollback_logout_session,
                    None,
                    audited_logout_at_ms + 21,
                ))
                .await,
            Ok(SessionAuthenticationOutcome::Authenticated(_))
        ));
        assert!(matches!(
            database
                .revoke_current_session(
                    owner.id,
                    rollback_logout_session.id,
                    audited_logout_at_ms + 5,
                    SessionRevocationReason::Administrator,
                )
                .await,
            Ok(true)
        ));
        assert!(matches!(
            database.list_user_sessions(owner.id).await,
            Ok(ref sessions)
                if sessions.iter().any(|session|
                    session.id == rollback_logout_session.id
                        && session.revoked_at_ms == Some(rollback_logout_session.created_at_ms)
                )
        ));
        let rollback_event_session = auth_session_fixture(
            owner.id,
            14,
            Revision::from_value(4),
            audited_logout_at_ms + 30,
            base + 10_000,
        );
        assert!(
            database
                .create_auth_session(
                    &rollback_event_session,
                    &login_security_event(
                        14,
                        audited_logout_at_ms + 30,
                        LoginSecurityReason::LoginSucceeded,
                    ),
                )
                .await
                .is_ok()
        );
        let rollback_logout_event = session_security_event(
            &rollback_event_session,
            31,
            audited_logout_at_ms + 25,
            LoginSecurityReason::Logout,
        );
        assert!(matches!(
            database
                .revoke_current_session_with_event(
                    owner.id,
                    rollback_event_session.id,
                    audited_logout_at_ms + 25,
                    SessionRevocationReason::Logout,
                    &rollback_logout_event,
                )
                .await,
            Ok(true)
        ));
        assert!(matches!(
            database.list_user_sessions(owner.id).await,
            Ok(ref sessions)
                if sessions.iter().any(|session|
                    session.id == rollback_event_session.id
                        && session.revoked_at_ms == Some(rollback_event_session.created_at_ms)
                )
        ));

        let rate = LoginAttemptReservation {
            key_version: 1,
            account_hmac: [0x31; 32],
            ip_prefix_hmac: [0x32; 32],
            global_hmac: [0x33; 32],
            user_agent_hash: [0x34; 32],
            request_id: "rate-account-0".to_owned(),
            now_ms: base + 20_000,
            window_ms: 60_000,
            account_max_attempts: 2,
            ip_max_attempts: 10,
            global_max_attempts: 100,
            lockout_ms: 60_000,
        };
        let standalone_rate_event = rate_limited_event(&rate);
        assert!(matches!(
            database
                .record_login_security_event(&standalone_rate_event)
                .await,
            Err(PersistenceError::RateLimitedEventMustBeAtomic)
        ));
        assert!(matches!(
            login_security_event_count(database, standalone_rate_event.id).await,
            Ok(0)
        ));
        for (scope, bucket_hmac) in [
            ("account", &rate.account_hmac),
            ("ip", &rate.ip_prefix_hmac),
            ("global", &rate.global_hmac),
        ] {
            assert!(matches!(
                login_rate_attempt_count(database, scope, rate.key_version, bucket_hmac).await,
                Ok(None)
            ));
        }
        let first_rate_request = LoginAttemptReservation {
            request_id: "rate-account-1".to_owned(),
            ..rate.clone()
        };
        let second_rate_request = LoginAttemptReservation {
            request_id: "rate-account-2".to_owned(),
            ..rate.clone()
        };
        let third_rate_request = LoginAttemptReservation {
            request_id: "rate-account-3".to_owned(),
            ..rate.clone()
        };
        let fourth_rate_request = LoginAttemptReservation {
            request_id: "rate-account-4".to_owned(),
            ..rate.clone()
        };
        let first_rate_event = rate_limited_event(&first_rate_request);
        let second_rate_event = rate_limited_event(&second_rate_request);
        let third_rate_event = rate_limited_event(&third_rate_request);
        let fourth_rate_event = rate_limited_event(&fourth_rate_request);
        let (first_rate, second_rate, third_rate, fourth_rate) = tokio::join!(
            database.reserve_login_attempt(&first_rate_request, &first_rate_event),
            database.reserve_login_attempt(&second_rate_request, &second_rate_event),
            database.reserve_login_attempt(&third_rate_request, &third_rate_event),
            database.reserve_login_attempt(&fourth_rate_request, &fourth_rate_event),
        );
        let outcomes = [first_rate, second_rate, third_rate, fourth_rate];
        let allowed = outcomes
            .iter()
            .filter(|outcome| matches!(outcome, Ok(LoginRateDecision::Allowed { .. })))
            .count();
        let limited = outcomes
            .iter()
            .filter(|outcome| matches!(outcome, Ok(LoginRateDecision::Limited { .. })))
            .count();
        assert_eq!(allowed, 2);
        assert_eq!(limited, 2);
        let rate_event_counts = tokio::join!(
            login_security_event_count(database, first_rate_event.id),
            login_security_event_count(database, second_rate_event.id),
            login_security_event_count(database, third_rate_event.id),
            login_security_event_count(database, fourth_rate_event.id),
        );
        let rate_event_counts = [
            rate_event_counts.0,
            rate_event_counts.1,
            rate_event_counts.2,
            rate_event_counts.3,
        ];
        assert!(
            rate_event_counts
                .iter()
                .all(|count| matches!(count, Ok(0) | Ok(1)))
        );
        assert_eq!(
            rate_event_counts
                .iter()
                .filter(|count| matches!(count, Ok(1)))
                .count(),
            1
        );
        assert!(matches!(
            login_rate_attempt_count(database, "account", rate.key_version, &rate.account_hmac,)
                .await,
            Ok(Some(3))
        ));
        assert!(matches!(
            login_rate_attempt_count(database, "ip", rate.key_version, &rate.ip_prefix_hmac).await,
            Ok(Some(2))
        ));
        assert!(matches!(
            login_rate_attempt_count(database, "global", rate.key_version, &rate.global_hmac).await,
            Ok(Some(2))
        ));

        let blocked_until_ms = rate.now_ms + rate.lockout_ms;
        let still_blocked_request = LoginAttemptReservation {
            request_id: "rate-account-still-blocked".to_owned(),
            now_ms: blocked_until_ms - 1,
            ..rate.clone()
        };
        let still_blocked_event = rate_limited_event(&still_blocked_request);
        assert!(matches!(
            database
                .reserve_login_attempt(&still_blocked_request, &still_blocked_event)
                .await,
            Ok(LoginRateDecision::Limited { .. })
        ));
        assert!(matches!(
            login_security_event_count(database, still_blocked_event.id).await,
            Ok(0)
        ));

        let reopened_request = LoginAttemptReservation {
            request_id: "rate-account-reopened".to_owned(),
            now_ms: blocked_until_ms,
            ..rate.clone()
        };
        let reopened_event = rate_limited_event(&reopened_request);
        assert!(matches!(
            database
                .reserve_login_attempt(&reopened_request, &reopened_event)
                .await,
            Ok(LoginRateDecision::Allowed { .. })
        ));
        assert!(matches!(
            login_security_event_count(database, reopened_event.id).await,
            Ok(0)
        ));

        let second_cycle_allowed = LoginAttemptReservation {
            request_id: "rate-account-second-cycle-allowed".to_owned(),
            now_ms: blocked_until_ms + 1,
            ..rate.clone()
        };
        let second_cycle_allowed_event = rate_limited_event(&second_cycle_allowed);
        assert!(matches!(
            database
                .reserve_login_attempt(&second_cycle_allowed, &second_cycle_allowed_event)
                .await,
            Ok(LoginRateDecision::Allowed { .. })
        ));
        assert!(matches!(
            login_security_event_count(database, second_cycle_allowed_event.id).await,
            Ok(0)
        ));

        let second_cycle_transition = LoginAttemptReservation {
            request_id: "rate-account-second-cycle-transition".to_owned(),
            now_ms: blocked_until_ms + 2,
            ..rate.clone()
        };
        let second_cycle_transition_event = rate_limited_event(&second_cycle_transition);
        assert!(matches!(
            database
                .reserve_login_attempt(&second_cycle_transition, &second_cycle_transition_event)
                .await,
            Ok(LoginRateDecision::Limited { .. })
        ));
        assert!(matches!(
            login_security_event_count(database, second_cycle_transition_event.id).await,
            Ok(1)
        ));

        let ip_limited = LoginAttemptReservation {
            key_version: 2,
            account_hmac: [0x51; 32],
            ip_prefix_hmac: [0x52; 32],
            global_hmac: [0x53; 32],
            user_agent_hash: [0x54; 32],
            request_id: "rate-ip-1".to_owned(),
            now_ms: base + 30_000,
            window_ms: 60_000,
            account_max_attempts: 10,
            ip_max_attempts: 1,
            global_max_attempts: 100,
            lockout_ms: 60_000,
        };
        let first_ip_event = rate_limited_event(&ip_limited);
        assert!(matches!(
            database
                .reserve_login_attempt(&ip_limited, &first_ip_event)
                .await,
            Ok(LoginRateDecision::Allowed { .. })
        ));
        let second_account_same_ip = LoginAttemptReservation {
            account_hmac: [0x54; 32],
            request_id: "rate-ip-2".to_owned(),
            now_ms: ip_limited.now_ms + 1,
            ..ip_limited.clone()
        };
        let second_ip_event = rate_limited_event(&second_account_same_ip);
        assert!(matches!(
            database
                .reserve_login_attempt(&second_account_same_ip, &second_ip_event)
                .await,
            Ok(LoginRateDecision::Limited { .. })
        ));
        assert!(matches!(
            login_security_event_count(database, first_ip_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            login_security_event_count(database, second_ip_event.id).await,
            Ok(1)
        ));
        assert!(matches!(
            login_rate_attempt_count(
                database,
                "account",
                second_account_same_ip.key_version,
                &second_account_same_ip.account_hmac,
            )
            .await,
            Ok(Some(1))
        ));
        assert!(matches!(
            login_rate_attempt_count(
                database,
                "ip",
                second_account_same_ip.key_version,
                &second_account_same_ip.ip_prefix_hmac,
            )
            .await,
            Ok(Some(2))
        ));
        assert!(matches!(
            login_rate_attempt_count(
                database,
                "global",
                second_account_same_ip.key_version,
                &second_account_same_ip.global_hmac,
            )
            .await,
            Ok(Some(1))
        ));
        let rotated_account_while_ip_blocked = LoginAttemptReservation {
            account_hmac: [0x55; 32],
            request_id: "rate-ip-blocked".to_owned(),
            now_ms: ip_limited.now_ms + 2,
            ..ip_limited.clone()
        };
        let blocked_ip_event = rate_limited_event(&rotated_account_while_ip_blocked);
        assert!(matches!(
            database
                .reserve_login_attempt(&rotated_account_while_ip_blocked, &blocked_ip_event)
                .await,
            Ok(LoginRateDecision::Limited { .. })
        ));
        assert!(matches!(
            login_security_event_count(database, blocked_ip_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            login_rate_attempt_count(
                database,
                "account",
                rotated_account_while_ip_blocked.key_version,
                &rotated_account_while_ip_blocked.account_hmac,
            )
            .await,
            Ok(None)
        ));
        assert!(matches!(
            login_rate_attempt_count(
                database,
                "ip",
                rotated_account_while_ip_blocked.key_version,
                &rotated_account_while_ip_blocked.ip_prefix_hmac,
            )
            .await,
            Ok(Some(2))
        ));
        let account_rows_before_rotation = login_rate_scope_row_count(
            database,
            "account",
            rotated_account_while_ip_blocked.key_version,
        )
        .await
        .ok();
        for offset in 0_u8..20 {
            let rotated = LoginAttemptReservation {
                account_hmac: [0x70 + offset; 32],
                request_id: format!("rate-ip-rotated-{offset}"),
                now_ms: ip_limited.now_ms + 3 + i64::from(offset),
                ..ip_limited.clone()
            };
            let rotated_event = rate_limited_event(&rotated);
            assert!(matches!(
                database
                    .reserve_login_attempt(&rotated, &rotated_event)
                    .await,
                Ok(LoginRateDecision::Limited { .. })
            ));
            assert!(matches!(
                login_security_event_count(database, rotated_event.id).await,
                Ok(0)
            ));
        }
        assert_eq!(
            login_rate_scope_row_count(
                database,
                "account",
                rotated_account_while_ip_blocked.key_version,
            )
            .await
            .ok(),
            account_rows_before_rotation
        );

        let global_limited = LoginAttemptReservation {
            key_version: 3,
            account_hmac: [0x61; 32],
            ip_prefix_hmac: [0x62; 32],
            global_hmac: [0x63; 32],
            user_agent_hash: [0x64; 32],
            request_id: "rate-global-1".to_owned(),
            now_ms: base + 40_000,
            window_ms: 60_000,
            account_max_attempts: 10,
            ip_max_attempts: 10,
            global_max_attempts: 1,
            lockout_ms: 60_000,
        };
        let first_global_event = rate_limited_event(&global_limited);
        assert!(matches!(
            database
                .reserve_login_attempt(&global_limited, &first_global_event)
                .await,
            Ok(LoginRateDecision::Allowed { .. })
        ));
        let second_origin_same_global = LoginAttemptReservation {
            account_hmac: [0x64; 32],
            ip_prefix_hmac: [0x65; 32],
            request_id: "rate-global-2".to_owned(),
            now_ms: global_limited.now_ms + 1,
            ..global_limited.clone()
        };
        let second_global_event = rate_limited_event(&second_origin_same_global);
        assert!(matches!(
            database
                .reserve_login_attempt(&second_origin_same_global, &second_global_event)
                .await,
            Ok(LoginRateDecision::Limited { .. })
        ));
        assert!(matches!(
            login_security_event_count(database, first_global_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            login_security_event_count(database, second_global_event.id).await,
            Ok(1)
        ));
        let rotated_origin_while_global_blocked = LoginAttemptReservation {
            account_hmac: [0x66; 32],
            ip_prefix_hmac: [0x67; 32],
            request_id: "rate-global-blocked".to_owned(),
            now_ms: global_limited.now_ms + 2,
            ..global_limited.clone()
        };
        let blocked_global_event = rate_limited_event(&rotated_origin_while_global_blocked);
        assert!(matches!(
            database
                .reserve_login_attempt(&rotated_origin_while_global_blocked, &blocked_global_event,)
                .await,
            Ok(LoginRateDecision::Limited { .. })
        ));
        assert!(matches!(
            login_security_event_count(database, blocked_global_event.id).await,
            Ok(0)
        ));
        assert!(matches!(
            login_rate_attempt_count(
                database,
                "account",
                rotated_origin_while_global_blocked.key_version,
                &rotated_origin_while_global_blocked.account_hmac,
            )
            .await,
            Ok(None)
        ));
        assert!(matches!(
            login_rate_attempt_count(
                database,
                "ip",
                rotated_origin_while_global_blocked.key_version,
                &rotated_origin_while_global_blocked.ip_prefix_hmac,
            )
            .await,
            Ok(None)
        ));
        assert!(matches!(
            login_rate_attempt_count(
                database,
                "global",
                rotated_origin_while_global_blocked.key_version,
                &rotated_origin_while_global_blocked.global_hmac,
            )
            .await,
            Ok(Some(2))
        ));
        let account_rows_before_global_rotation = login_rate_scope_row_count(
            database,
            "account",
            rotated_origin_while_global_blocked.key_version,
        )
        .await
        .ok();
        let ip_rows_before_global_rotation = login_rate_scope_row_count(
            database,
            "ip",
            rotated_origin_while_global_blocked.key_version,
        )
        .await
        .ok();
        for offset in 0_u8..20 {
            let rotated = LoginAttemptReservation {
                account_hmac: [0x80 + offset; 32],
                ip_prefix_hmac: [0xa0 + offset; 32],
                request_id: format!("rate-global-rotated-{offset}"),
                now_ms: global_limited.now_ms + 3 + i64::from(offset),
                ..global_limited.clone()
            };
            let rotated_event = rate_limited_event(&rotated);
            assert!(matches!(
                database
                    .reserve_login_attempt(&rotated, &rotated_event)
                    .await,
                Ok(LoginRateDecision::Limited { .. })
            ));
            assert!(matches!(
                login_security_event_count(database, rotated_event.id).await,
                Ok(0)
            ));
        }
        assert_eq!(
            login_rate_scope_row_count(
                database,
                "account",
                rotated_origin_while_global_blocked.key_version,
            )
            .await
            .ok(),
            account_rows_before_global_rotation
        );
        assert_eq!(
            login_rate_scope_row_count(
                database,
                "ip",
                rotated_origin_while_global_blocked.key_version,
            )
            .await
            .ok(),
            ip_rows_before_global_rotation
        );
        assert!(matches!(
            database
                .clear_login_account_bucket(rate.key_version, &rate.account_hmac)
                .await,
            Ok(true)
        ));
        let after_clear = LoginAttemptReservation {
            ip_prefix_hmac: [0x42; 32],
            global_hmac: [0x43; 32],
            request_id: "rate-after-clear".to_owned(),
            now_ms: rate.now_ms + 1,
            ..rate
        };
        let after_clear_event = rate_limited_event(&after_clear);
        assert!(matches!(
            database
                .reserve_login_attempt(&after_clear, &after_clear_event)
                .await,
            Ok(LoginRateDecision::Allowed {
                remaining_attempts: 1,
                ..
            })
        ));
        assert!(matches!(
            login_security_event_count(database, after_clear_event.id).await,
            Ok(0)
        ));

        let mismatched_event_rate = LoginAttemptReservation {
            key_version: 4,
            account_hmac: [0xb1; 32],
            ip_prefix_hmac: [0xb2; 32],
            global_hmac: [0xb3; 32],
            user_agent_hash: [0xb4; 32],
            request_id: "rate-mismatch".to_owned(),
            now_ms: base + 50_000,
            window_ms: 60_000,
            account_max_attempts: 1,
            ip_max_attempts: 10,
            global_max_attempts: 100,
            lockout_ms: 60_000,
        };
        let valid_rate_event = rate_limited_event(&mismatched_event_rate);
        let mut wrong_reason_event = valid_rate_event.clone();
        wrong_reason_event.reason = LoginSecurityReason::InvalidCredentials;
        let mut wrong_time_event = valid_rate_event.clone();
        wrong_time_event.occurred_at_ms += 1;
        let mut wrong_key_event = valid_rate_event.clone();
        wrong_key_event.digest_key_version += 1;
        let mut wrong_request_event = valid_rate_event.clone();
        wrong_request_event.request_id = "rate-mismatch-other".to_owned();
        let mut wrong_account_event = valid_rate_event.clone();
        wrong_account_event.account_hmac = Some([0xc1; 32]);
        let mut wrong_ip_event = valid_rate_event.clone();
        wrong_ip_event.ip_prefix_hmac = Some([0xc2; 32]);
        let mut wrong_user_agent_event = valid_rate_event;
        wrong_user_agent_event.user_agent_hash = Some([0xc3; 32]);
        for mismatched_event in [
            wrong_reason_event,
            wrong_time_event,
            wrong_key_event,
            wrong_request_event,
            wrong_account_event,
            wrong_ip_event,
            wrong_user_agent_event,
        ] {
            assert!(matches!(
                database
                    .reserve_login_attempt(&mismatched_event_rate, &mismatched_event)
                    .await,
                Err(PersistenceError::InvalidLoginRateEvent)
            ));
            assert!(matches!(
                login_security_event_count(database, mismatched_event.id).await,
                Ok(0)
            ));
        }
        for (scope, bucket_hmac) in [
            ("account", &mismatched_event_rate.account_hmac),
            ("ip", &mismatched_event_rate.ip_prefix_hmac),
            ("global", &mismatched_event_rate.global_hmac),
        ] {
            assert!(matches!(
                login_rate_attempt_count(
                    database,
                    scope,
                    mismatched_event_rate.key_version,
                    bucket_hmac,
                )
                .await,
                Ok(None)
            ));
        }

        let rollback_rate = LoginAttemptReservation {
            key_version: 5,
            account_hmac: [0xd1; 32],
            ip_prefix_hmac: [0xd2; 32],
            global_hmac: [0xd3; 32],
            user_agent_hash: [0xd4; 32],
            request_id: "rate-rollback-first".to_owned(),
            now_ms: base + 60_000,
            window_ms: 60_000,
            account_max_attempts: 1,
            ip_max_attempts: 10,
            global_max_attempts: 100,
            lockout_ms: 60_000,
        };
        let occupied_event = login_security_event(
            251,
            rollback_rate.now_ms,
            LoginSecurityReason::InvalidCredentials,
        );
        assert!(
            database
                .record_login_security_event(&occupied_event)
                .await
                .is_ok()
        );
        let rollback_first_event = rate_limited_event(&rollback_rate);
        assert!(matches!(
            database
                .reserve_login_attempt(&rollback_rate, &rollback_first_event)
                .await,
            Ok(LoginRateDecision::Allowed { .. })
        ));
        let rollback_transition = LoginAttemptReservation {
            request_id: "rate-rollback-transition".to_owned(),
            now_ms: rollback_rate.now_ms + 1,
            ..rollback_rate.clone()
        };
        let mut conflicting_rate_event = rate_limited_event(&rollback_transition);
        conflicting_rate_event.id = occupied_event.id;
        assert!(matches!(
            database
                .reserve_login_attempt(&rollback_transition, &conflicting_rate_event)
                .await,
            Err(PersistenceError::Sql(_))
        ));
        for (scope, bucket_hmac) in [
            ("account", &rollback_rate.account_hmac),
            ("ip", &rollback_rate.ip_prefix_hmac),
            ("global", &rollback_rate.global_hmac),
        ] {
            assert!(matches!(
                login_rate_attempt_count(database, scope, rollback_rate.key_version, bucket_hmac,)
                    .await,
                Ok(Some(1))
            ));
        }
        let retry_rate = LoginAttemptReservation {
            request_id: "rate-rollback-retry".to_owned(),
            ..rollback_transition
        };
        let retry_rate_event = rate_limited_event(&retry_rate);
        assert!(matches!(
            database
                .reserve_login_attempt(&retry_rate, &retry_rate_event)
                .await,
            Ok(LoginRateDecision::Limited { .. })
        ));
        assert!(matches!(
            login_rate_attempt_count(
                database,
                "account",
                retry_rate.key_version,
                &retry_rate.account_hmac,
            )
            .await,
            Ok(Some(2))
        ));
        assert!(matches!(
            login_security_event_count(database, retry_rate_event.id).await,
            Ok(1)
        ));
    }

    async fn replace_legacy_setting(
        database: &Database,
        instance_id: EntityId,
        schema_version: i32,
        value_json: &str,
    ) {
        match database {
            Database::Sqlite(pool) => {
                let result = sqlx::query(
                    "UPDATE instance_settings SET schema_version=?,value_json=? WHERE instance_id=? AND key=?",
                )
                .bind(schema_version)
                .bind(value_json)
                .bind(instance_id.to_string())
                .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                .execute(pool)
                .await;
                assert!(matches!(result, Ok(result) if result.rows_affected() == 1));
            }
            Database::Postgres(pool) => {
                let result = sqlx::query(
                    "UPDATE instance_settings SET schema_version=$1,value_json=$2::jsonb WHERE instance_id=$3 AND key=$4",
                )
                .bind(schema_version)
                .bind(value_json)
                .bind(instance_id.into_uuid())
                .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                .execute(pool)
                .await;
                assert!(matches!(result, Ok(result) if result.rows_affected() == 1));
            }
        }
    }

    async fn install_owner_failure_trigger(database: &Database) {
        match database {
            Database::Sqlite(pool) => {
                assert!(sqlx::query("CREATE TRIGGER nodecontroll_test_fail_owner BEFORE INSERT ON users BEGIN SELECT RAISE(ABORT, 'forced owner failure'); END").execute(pool).await.is_ok());
            }
            Database::Postgres(pool) => {
                assert!(sqlx::query("CREATE OR REPLACE FUNCTION nodecontroll_test_fail_owner() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'forced owner failure'; END $$").execute(pool).await.is_ok());
                assert!(sqlx::query("CREATE TRIGGER nodecontroll_test_fail_owner BEFORE INSERT ON users FOR EACH ROW EXECUTE FUNCTION nodecontroll_test_fail_owner()").execute(pool).await.is_ok());
            }
        }
    }

    async fn remove_owner_failure_trigger(database: &Database) {
        match database {
            Database::Sqlite(pool) => {
                assert!(
                    sqlx::query("DROP TRIGGER nodecontroll_test_fail_owner")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
            Database::Postgres(pool) => {
                assert!(
                    sqlx::query("DROP TRIGGER nodecontroll_test_fail_owner ON users")
                        .execute(pool)
                        .await
                        .is_ok()
                );
                assert!(
                    sqlx::query("DROP FUNCTION nodecontroll_test_fail_owner()")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
        }
    }

    async fn settings_actor(
        database: &Database,
        instance_id: EntityId,
    ) -> Result<Option<String>, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => {
                let value: Option<String> = sqlx::query_scalar(
                    "SELECT updated_by FROM instance_settings WHERE instance_id=? AND key=?",
                )
                .bind(instance_id.to_string())
                .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                .fetch_one(pool)
                .await?;
                Ok(value)
            }
            Database::Postgres(pool) => {
                let value: Option<uuid::Uuid> = sqlx::query_scalar(
                    "SELECT updated_by FROM instance_settings WHERE instance_id=$1 AND key=$2",
                )
                .bind(instance_id.into_uuid())
                .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                .fetch_one(pool)
                .await?;
                Ok(value.map(|id| id.to_string()))
            }
        }
    }

    async fn remove_bootstrap_latch(database: &Database) {
        match database {
            Database::Sqlite(pool) => {
                let result =
                    sqlx::query("DELETE FROM control_plane_bootstrap WHERE singleton_key=1")
                        .execute(pool)
                        .await;
                assert!(matches!(result, Ok(result) if result.rows_affected() == 1));
            }
            Database::Postgres(pool) => {
                let result =
                    sqlx::query("DELETE FROM control_plane_bootstrap WHERE singleton_key=1")
                        .execute(pool)
                        .await;
                assert!(matches!(result, Ok(result) if result.rows_affected() == 1));
            }
        }
    }

    async fn legacy_upgrade_contract(database: Database) {
        migrate_to_0001(&database).await;
        let legacy_instance = fixture();
        let legacy_settings = SubscriptionBehaviorSettings {
            external_sync: ExternalSyncStrategy::Manual,
            silent_mode: true,
            short_links_enabled: false,
            client_compatibility: ClientCompatibilityMode::Legacy,
            response_headers_enabled: true,
            info_node_enabled: true,
        };
        seed_v1_instance(&database, &legacy_instance, &legacy_settings).await;
        assert!(database.migrate().await.is_ok());
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::LegacyNeedsOwner)
        ));
        assert!(matches!(database.is_initialized().await, Ok(false)));
        let legacy_owner = owner_fixture();

        let valid_settings_json = serde_json::to_string(&legacy_settings);
        assert!(valid_settings_json.is_ok());
        let valid_settings_json =
            valid_settings_json.unwrap_or_else(|_| unreachable!("checked above"));
        replace_legacy_setting(&database, legacy_instance.id, 2, &valid_settings_json).await;
        assert!(matches!(
            database
                .bootstrap_control_plane(
                    &fixture(),
                    &legacy_owner,
                    &SubscriptionBehaviorSettings::default(),
                )
                .await,
            Err(PersistenceError::InconsistentBootstrapState)
        ));
        assert!(matches!(database.active_owner_count().await, Ok(0)));
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::LegacyNeedsOwner)
        ));

        replace_legacy_setting(
            &database,
            legacy_instance.id,
            super::SUBSCRIPTION_SETTINGS_SCHEMA,
            r#"{"unexpected":true}"#,
        )
        .await;
        assert!(matches!(
            database
                .bootstrap_control_plane(
                    &fixture(),
                    &legacy_owner,
                    &SubscriptionBehaviorSettings::default(),
                )
                .await,
            Err(PersistenceError::InconsistentBootstrapState)
        ));
        assert!(matches!(database.active_owner_count().await, Ok(0)));
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::LegacyNeedsOwner)
        ));
        replace_legacy_setting(
            &database,
            legacy_instance.id,
            super::SUBSCRIPTION_SETTINGS_SCHEMA,
            &valid_settings_json,
        )
        .await;

        install_owner_failure_trigger(&database).await;
        assert!(matches!(
            database
                .bootstrap_control_plane(
                    &fixture(),
                    &legacy_owner,
                    &SubscriptionBehaviorSettings::default(),
                )
                .await,
            Err(PersistenceError::Sql(_))
        ));
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::LegacyNeedsOwner)
        ));
        assert!(matches!(database.instance().await, Ok(Some(found)) if found == legacy_instance));
        assert!(matches!(
            database.subscription_settings(legacy_instance.id).await,
            Ok(Some((found, revision))) if found == legacy_settings && revision == Revision::initial()
        ));
        assert!(matches!(database.active_owner_count().await, Ok(0)));
        remove_owner_failure_trigger(&database).await;
        assert!(matches!(
            database
                .bootstrap_control_plane(
                    &fixture(),
                    &legacy_owner,
                    &SubscriptionBehaviorSettings::default(),
                )
                .await,
            Ok(id) if id == legacy_instance.id
        ));
        assert!(matches!(
            database.subscription_settings(legacy_instance.id).await,
            Ok(Some((found, revision))) if found == legacy_settings && revision == Revision::initial()
        ));
        assert!(matches!(database.active_owner_count().await, Ok(1)));
        assert!(matches!(database.is_initialized().await, Ok(true)));
        assert!(matches!(
            settings_actor(&database, legacy_instance.id).await,
            Ok(None)
        ));
    }

    async fn legacy_missing_settings_contract(database: Database) {
        migrate_to_0001(&database).await;
        let legacy_instance = fixture();
        let legacy_settings = SubscriptionBehaviorSettings::default();
        seed_v1_instance(&database, &legacy_instance, &legacy_settings).await;
        match &database {
            Database::Sqlite(pool) => {
                let deleted =
                    sqlx::query("DELETE FROM instance_settings WHERE instance_id=? AND key=?")
                        .bind(legacy_instance.id.to_string())
                        .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                        .execute(pool)
                        .await;
                assert!(matches!(deleted, Ok(result) if result.rows_affected() == 1));
            }
            Database::Postgres(pool) => {
                let deleted =
                    sqlx::query("DELETE FROM instance_settings WHERE instance_id=$1 AND key=$2")
                        .bind(legacy_instance.id.into_uuid())
                        .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                        .execute(pool)
                        .await;
                assert!(matches!(deleted, Ok(result) if result.rows_affected() == 1));
            }
        }
        assert!(database.migrate().await.is_ok());
        let owner = owner_fixture();
        assert!(matches!(
            database
                .bootstrap_control_plane(&fixture(), &owner, &legacy_settings)
                .await,
            Ok(id) if id == legacy_instance.id
        ));
        assert!(matches!(
            database.subscription_settings(legacy_instance.id).await,
            Ok(Some((found, revision))) if found == legacy_settings && revision == Revision::initial()
        ));
        assert!(matches!(
            settings_actor(&database, legacy_instance.id).await,
            Ok(Some(found)) if found == owner.id.to_string()
        ));
    }

    async fn migration_atomicity_contract(database: Database) {
        migrate_to_0001(&database).await;
        match &database {
            Database::Sqlite(pool) => {
                assert!(
                    sqlx::query("CREATE TABLE control_plane_bootstrap (dummy INTEGER)")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
            Database::Postgres(pool) => {
                assert!(
                    sqlx::query("CREATE TABLE control_plane_bootstrap (dummy INTEGER)")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
        }
        assert!(database.migrate().await.is_err());
        match &database {
            Database::Sqlite(pool) => {
                let version: Result<Option<i64>, _> =
                    sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations")
                        .fetch_one(pool)
                        .await;
                assert!(matches!(version, Ok(Some(1))));
                let users: Result<i64, _> = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'",
                )
                .fetch_one(pool)
                .await;
                assert!(matches!(users, Ok(0)));
                assert!(
                    sqlx::query("DROP TABLE control_plane_bootstrap")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
            Database::Postgres(pool) => {
                let version: Result<Option<i64>, _> =
                    sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations")
                        .fetch_one(pool)
                        .await;
                assert!(matches!(version, Ok(Some(1))));
                let users_absent: Result<bool, _> =
                    sqlx::query_scalar("SELECT to_regclass('users') IS NULL")
                        .fetch_one(pool)
                        .await;
                assert!(matches!(users_absent, Ok(true)));
                assert!(
                    sqlx::query("DROP TABLE control_plane_bootstrap")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
        }
        match &database {
            Database::Sqlite(_) => {
                assert!(database.migrate().await.is_ok());
                assert!(matches!(
                    database.bootstrap_state().await,
                    Ok(BootstrapState::Uninitialized)
                ));
            }
            Database::Postgres(pool) => {
                // Match the process-restart boundary used by production after
                // a failed sqlx migration has retained its session lock.
                let retry_options = pool.connect_options().as_ref().clone();
                pool.close().await;
                let retry_pool = PgPoolOptions::new()
                    .max_connections(4)
                    .connect_with(retry_options)
                    .await
                    .unwrap_or_else(|error| {
                        panic!("failed to reconnect for PostgreSQL atomic migration retry: {error}")
                    });
                let retry_database = Database::Postgres(retry_pool.clone());
                let retry = retry_database.migrate().await;
                assert!(
                    retry.is_ok(),
                    "PostgreSQL atomic migration retry failed: {retry:?}"
                );
                assert!(matches!(
                    retry_database.bootstrap_state().await,
                    Ok(BootstrapState::Uninitialized)
                ));
                retry_pool.close().await;
            }
        }
    }

    async fn sqlite_secret_record_migration_guard_contract(database: Database) {
        migrate_to_0005(&database).await;
        let Database::Sqlite(pool) = &database else {
            return;
        };
        let inserted = sqlx::query(
            "INSERT INTO secret_records (id,purpose,key_version,nonce,ciphertext,aad_hash,created_at_ms,rotated_from,deleted_at_ms) VALUES (?,'legacy.untyped',1,?,?,?,1,NULL,NULL)",
        )
        .bind(EntityId::new().to_string())
        .bind([1_u8; 24].as_slice())
        .bind([2_u8; 16].as_slice())
        .bind([3_u8; 32].as_slice())
        .execute(pool)
        .await;
        assert!(inserted.is_ok());
        assert!(database.migrate().await.is_err());
        assert!(matches!(migration_version(&database).await, Ok(Some(5))));
        let legacy_rows: Result<i64, _> = sqlx::query_scalar("SELECT COUNT(*) FROM secret_records")
            .fetch_one(pool)
            .await;
        assert!(matches!(legacy_rows, Ok(1)));
        assert!(
            sqlx::query("DELETE FROM secret_records")
                .execute(pool)
                .await
                .is_ok()
        );
        assert!(database.migrate().await.is_ok());
        assert!(matches!(migration_version(&database).await, Ok(Some(9))));
        let typed_owner_column: Result<i64, _> = sqlx::query_scalar(
            "SELECT COUNT(*) FROM pragma_table_info('secret_records') WHERE name='owner_type'",
        )
        .fetch_one(pool)
        .await;
        assert!(matches!(typed_owner_column, Ok(1)));
    }

    async fn secret_record_contract(database: &Database) {
        const KEY: &str = "f97c2563b4609f964f83ecf3c874f545698b8e360bbca06316547d2af8928f62";
        const WRONG_KEY: &str = "097c2563b4609f964f83ecf3c874f545698b8e360bbca06316547d2af8928f62";
        let cipher = EnvelopeCipher::from_hex(KEY, 1);
        assert!(cipher.is_ok());
        let wrong_cipher = EnvelopeCipher::from_hex(WRONG_KEY, 1);
        assert!(wrong_cipher.is_ok());
        let (Ok(cipher), Ok(wrong_cipher)) = (cipher, wrong_cipher) else {
            return;
        };
        let keyring = Keyring::from_ciphers(cipher, Vec::new());
        let wrong_keyring = Keyring::from_ciphers(wrong_cipher, Vec::new());
        assert!(keyring.is_ok());
        assert!(wrong_keyring.is_ok());
        let (Ok(keyring), Ok(wrong_keyring)) = (keyring, wrong_keyring) else {
            return;
        };
        let envelope = keyring.new_canary_envelope();
        assert!(envelope.is_ok());
        let Ok(envelope) = envelope else {
            return;
        };
        let candidate = NewSecretRecord {
            id: EntityId::new(),
            binding: SecretBinding::root_key_canary(),
            envelope,
            created_at_ms: 1_777_777_776_000,
            rotated_from: None,
        };
        let first = database.ensure_secret_record(&candidate).await;
        assert!(first.is_ok());
        let duplicate_candidate = NewSecretRecord {
            id: EntityId::new(),
            binding: SecretBinding::root_key_canary(),
            envelope: keyring
                .new_canary_envelope()
                .unwrap_or_else(|_| unreachable!("keyring was already exercised")),
            created_at_ms: candidate.created_at_ms + 1,
            rotated_from: None,
        };
        let second = database.ensure_secret_record(&duplicate_candidate).await;
        assert!(matches!(
            (&first, &second),
            (Ok(first), Ok(second)) if first.id == candidate.id && second.id == first.id
        ));
        if let Ok(first) = first {
            assert!(keyring.verify_canary(&first.envelope).is_ok());
            assert!(wrong_keyring.verify_canary(&first.envelope).is_err());
        }
    }

    async fn recovery_code_contract(database: &Database, owner: &UserAccount) {
        assert!(matches!(
            database.recovery_code_summary(owner.id).await,
            Ok(Some(summary))
                if summary.set_version == 1
                    && summary.total_count == 8
                    && summary.remaining_count == 8
                    && summary.created_at_ms == owner.created_at_ms
        ));
        let session = auth_session_fixture(
            owner.id,
            90,
            Revision::initial(),
            owner.created_at_ms + 10,
            owner.created_at_ms + 10_000,
        );
        assert!(
            database
                .create_auth_session(
                    &session,
                    &login_security_event(
                        90,
                        session.created_at_ms,
                        LoginSecurityReason::LoginSucceeded,
                    ),
                )
                .await
                .is_ok()
        );
        let replacement = recovery_set_fixture(owner.created_at_ms + 100, 50);
        let replaced = database
            .replace_recovery_codes(RecoveryCodeReplacement {
                user_id: owner.id,
                actor_session_id: session.id,
                expected_user_revision: Revision::initial(),
                expected_auth_revision: Revision::initial(),
                expected_recent_auth_at_ms: session.recent_auth_at_ms,
                replacement: &replacement,
                now_ms: replacement.created_at_ms,
            })
            .await;
        assert!(matches!(
            replaced,
            Ok(summary)
                if summary.set_version == 2
                    && summary.total_count == 8
                    && summary.remaining_count == 8
        ));
        let obsolete = RecoveryCodeConsumption {
            user_id: owner.id,
            digest_key_version: 1,
            code_hmac: [10; 32],
            now_ms: replacement.created_at_ms + 1,
        };
        assert!(matches!(
            database.consume_recovery_code(&obsolete).await,
            Ok(false)
        ));
        let one_time = RecoveryCodeConsumption {
            code_hmac: [50; 32],
            ..obsolete
        };
        let (first, second) = tokio::join!(
            database.consume_recovery_code(&one_time),
            database.consume_recovery_code(&one_time),
        );
        assert!(matches!(
            (first, second),
            (Ok(true), Ok(false)) | (Ok(false), Ok(true))
        ));
        assert!(matches!(
            database.recovery_code_summary(owner.id).await,
            Ok(Some(summary)) if summary.set_version == 2 && summary.remaining_count == 7
        ));
        assert!(matches!(
            database
                .revoke_current_session(
                    owner.id,
                    session.id,
                    replacement.created_at_ms + 2,
                    SessionRevocationReason::Logout,
                )
                .await,
            Ok(true)
        ));
        let removed_session: Result<u64, sqlx::Error> = match database {
            Database::Sqlite(pool) => sqlx::query(
                "DELETE FROM auth_sessions WHERE id=? AND user_id=? AND status='revoked'",
            )
            .bind(session.id.to_string())
            .bind(owner.id.to_string())
            .execute(pool)
            .await
            .map(|result| result.rows_affected()),
            Database::Postgres(pool) => sqlx::query(
                "DELETE FROM auth_sessions WHERE id=$1 AND user_id=$2 AND status='revoked'",
            )
            .bind(session.id.into_uuid())
            .bind(owner.id.into_uuid())
            .execute(pool)
            .await
            .map(|result| result.rows_affected()),
        };
        assert!(matches!(removed_session, Ok(1)));
        assert!(matches!(
            database.list_user_sessions(owner.id).await,
            Ok(ref sessions) if sessions.is_empty()
        ));
    }

    async fn repository_contract(database: Database) {
        let migration = database.migrate().await;
        assert!(migration.is_ok(), "fresh migration failed: {migration:?}");
        secret_record_contract(&database).await;
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::Uninitialized)
        ));
        install_owner_failure_trigger(&database).await;
        assert!(matches!(
            database
                .bootstrap_control_plane(
                    &fixture(),
                    &owner_fixture(),
                    &SubscriptionBehaviorSettings::default(),
                )
                .await,
            Err(PersistenceError::Sql(_))
        ));
        assert!(matches!(database.instance().await, Ok(None)));
        assert!(matches!(database.active_owner_count().await, Ok(0)));
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::Uninitialized)
        ));
        remove_owner_failure_trigger(&database).await;

        let first_instance = fixture();
        let first_owner = owner_fixture();
        let second_instance = fixture();
        let second_owner = owner_fixture();
        let initial_settings = SubscriptionBehaviorSettings::default();
        let second_initial_settings = SubscriptionBehaviorSettings::default();
        let first_recovery = recovery_set_fixture(first_owner.created_at_ms, 10);
        let second_recovery = recovery_set_fixture(second_owner.created_at_ms, 10);
        let (first_result, second_result) = tokio::join!(
            database.bootstrap_control_plane_with_recovery(
                &first_instance,
                &first_owner,
                &initial_settings,
                &first_recovery,
            ),
            database.bootstrap_control_plane_with_recovery(
                &second_instance,
                &second_owner,
                &second_initial_settings,
                &second_recovery,
            )
        );
        let (instance, owner) = match (first_result, second_result) {
            (Ok(id), Err(PersistenceError::AlreadyInitialized)) if id == first_instance.id => {
                (first_instance, first_owner)
            }
            (Err(PersistenceError::AlreadyInitialized), Ok(id)) if id == second_instance.id => {
                (second_instance, second_owner)
            }
            outcome => panic!("unexpected concurrent bootstrap outcome: {outcome:?}"),
        };
        assert!(matches!(database.is_initialized().await, Ok(true)));
        assert!(matches!(database.active_owner_count().await, Ok(1)));
        assert!(matches!(database.instance().await, Ok(Some(found)) if found == instance));
        assert!(matches!(
            database.subscription_settings(instance.id).await,
            Ok(Some((found, revision))) if found == initial_settings && revision == Revision::initial()
        ));
        let settings = SubscriptionBehaviorSettings {
            external_sync: ExternalSyncStrategy::OnRequest,
            silent_mode: true,
            short_links_enabled: true,
            client_compatibility: ClientCompatibilityMode::Legacy,
            response_headers_enabled: true,
            info_node_enabled: false,
        };
        let updated = SubscriptionBehaviorSettings {
            silent_mode: false,
            ..settings.clone()
        };
        let next = Revision::from_value(1);
        assert!(matches!(
            database
                .save_subscription_settings(
                    instance.id,
                    &updated,
                    Some(Revision::initial()),
                    owner.id,
                    1_777_777_777_002,
                )
                .await,
            Ok(revision) if revision == next
        ));
        assert!(matches!(
            database
                .save_subscription_settings(
                    instance.id,
                    &settings,
                    Some(Revision::initial()),
                    owner.id,
                    1_777_777_777_003,
                )
                .await,
            Err(PersistenceError::RevisionConflict)
        ));
        assert!(matches!(
            database.subscription_settings(instance.id).await,
            Ok(Some((found, revision))) if found == updated && revision == next
        ));
        assert!(matches!(
            settings_actor(&database, instance.id).await,
            Ok(Some(found)) if found == owner.id.to_string()
        ));
        assert!(matches!(
            database
                .bootstrap_control_plane(
                    &fixture(),
                    &owner_fixture(),
                    &SubscriptionBehaviorSettings::default(),
                )
                .await,
            Err(PersistenceError::AlreadyInitialized)
        ));
        assert!(matches!(database.instance().await, Ok(Some(found)) if found == instance));
        assert!(matches!(database.active_owner_count().await, Ok(1)));
        assert!(matches!(
            database.subscription_settings(instance.id).await,
            Ok(Some((found, revision))) if found == updated && revision == next
        ));
        assert!(matches!(
            settings_actor(&database, instance.id).await,
            Ok(Some(found)) if found == owner.id.to_string()
        ));
        recovery_code_contract(&database, &owner).await;
        super::auth_challenge::auth_challenge_contract(
            &database,
            owner.id,
            Revision::initial(),
            owner.created_at_ms + 50,
        )
        .await;
        auth_core_contract(&database, &owner).await;
        actor_aware_session_revocation_contract(&database).await;
        let restart_session = auth_session_fixture(
            owner.id,
            56,
            Revision::from_value(4),
            owner.created_at_ms + 500,
            owner.created_at_ms + 10_000,
        );
        assert!(
            database
                .create_auth_session(
                    &restart_session,
                    &login_security_event(
                        56,
                        restart_session.created_at_ms,
                        LoginSecurityReason::LoginSucceeded,
                    ),
                )
                .await
                .is_ok()
        );
        remove_bootstrap_latch(&database).await;
        assert!(matches!(
            database.bootstrap_state().await,
            Err(PersistenceError::InconsistentBootstrapState)
        ));
    }

    #[tokio::test]
    async fn sqlite_repository_contract() {
        assert!(matches!(
            Database::validate_url("sqlite::memory:"),
            Ok(DatabaseEngine::Sqlite)
        ));
        let database = Database::connect("sqlite::memory:", settings()).await;
        assert!(database.is_ok());
        if let Ok(database) = database {
            assert_eq!(database.engine(), DatabaseEngine::Sqlite);
            repository_contract(database).await;
        }
        let legacy_database = Database::connect("sqlite::memory:", settings()).await;
        assert!(legacy_database.is_ok());
        if let Ok(database) = legacy_database {
            legacy_upgrade_contract(database).await;
        }
        let legacy_missing_database = Database::connect("sqlite::memory:", settings()).await;
        assert!(legacy_missing_database.is_ok());
        if let Ok(database) = legacy_missing_database {
            legacy_missing_settings_contract(database).await;
        }
        let rollback_database = Database::connect("sqlite::memory:", settings()).await;
        assert!(rollback_database.is_ok());
        if let Ok(database) = rollback_database {
            migration_atomicity_contract(database).await;
        }
        let auth_upgrade_database = Database::connect("sqlite::memory:", settings()).await;
        assert!(auth_upgrade_database.is_ok());
        if let Ok(database) = auth_upgrade_database {
            auth_upgrade_contract(database).await;
        }
        let session_timeline_upgrade_database =
            Database::connect("sqlite::memory:", settings()).await;
        assert!(session_timeline_upgrade_database.is_ok());
        if let Ok(database) = session_timeline_upgrade_database {
            session_timeline_upgrade_contract(database).await;
        }
        let auth_rollback_database = Database::connect("sqlite::memory:", settings()).await;
        assert!(auth_rollback_database.is_ok());
        if let Ok(database) = auth_rollback_database {
            auth_migration_rollback_contract(database).await;
        }
        let recent_auth_migration_rollback_database =
            Database::connect("sqlite::memory:", settings()).await;
        assert!(recent_auth_migration_rollback_database.is_ok());
        if let Ok(database) = recent_auth_migration_rollback_database {
            sqlite_recent_auth_migration_rollback_contract(database).await;
        }
        let session_timeline_migration_rollback_database =
            Database::connect("sqlite::memory:", settings()).await;
        assert!(session_timeline_migration_rollback_database.is_ok());
        if let Ok(database) = session_timeline_migration_rollback_database {
            sqlite_session_timeline_migration_rollback_contract(database).await;
        }
        let secret_record_migration_guard_database =
            Database::connect("sqlite::memory:", settings()).await;
        assert!(secret_record_migration_guard_database.is_ok());
        if let Ok(database) = secret_record_migration_guard_database {
            sqlite_secret_record_migration_guard_contract(database).await;
        }
    }

    #[tokio::test]
    async fn sqlite_auth_session_survives_pool_restart() {
        let database_path = std::env::temp_dir().join(format!(
            "nodecontroll-auth-restart-{}.sqlite",
            EntityId::new()
        ));
        let normalized_path = database_path.to_string_lossy().replace('\\', "/");
        let database_url = format!("sqlite://{normalized_path}");
        let database = Database::connect(&database_url, settings()).await;
        assert!(database.is_ok());
        if let Ok(database) = database {
            assert!(database.migrate().await.is_ok());
            let instance = fixture();
            let owner = owner_fixture();
            assert!(
                database
                    .bootstrap_control_plane(
                        &instance,
                        &owner,
                        &SubscriptionBehaviorSettings::default(),
                    )
                    .await
                    .is_ok()
            );
            let session = auth_session_fixture(
                owner.id,
                6,
                Revision::initial(),
                1_777_777_777_504,
                1_777_777_787_100,
            );
            assert!(
                database
                    .create_auth_session(
                        &session,
                        &login_security_event(
                            6,
                            session.created_at_ms,
                            LoginSecurityReason::LoginSucceeded,
                        ),
                    )
                    .await
                    .is_ok()
            );
            if let Database::Sqlite(pool) = &database {
                pool.close().await;
            }
            let reopened = Database::connect(&database_url, settings()).await;
            assert!(reopened.is_ok());
            if let Ok(reopened) = reopened {
                assert!(matches!(
                    reopened
                        .authenticate_session(&persisted_authentication_fixture(6))
                        .await,
                    Ok(SessionAuthenticationOutcome::Authenticated(ref authenticated))
                        if authenticated.user_id == owner.id
                ));
                if let Database::Sqlite(pool) = &reopened {
                    pool.close().await;
                }
            }
        }
        let _ = tokio::fs::remove_file(&database_path).await;
        let mut wal_path = database_path.as_os_str().to_os_string();
        wal_path.push("-wal");
        let _ = tokio::fs::remove_file(std::path::PathBuf::from(wal_path)).await;
        let mut shm_path = database_path.as_os_str().to_os_string();
        shm_path.push("-shm");
        let _ = tokio::fs::remove_file(std::path::PathBuf::from(shm_path)).await;
    }

    #[tokio::test]
    async fn postgres_repository_contract() {
        let url = match std::env::var("NODECONTROLL_TEST_POSTGRES_URL") {
            Ok(url) => url,
            Err(_) => {
                panic!("NODECONTROLL_TEST_POSTGRES_URL is required for the persistence test gate")
            }
        };
        let fresh = isolated_postgres(&url, "nodecontroll_test_fresh").await;
        assert!(fresh.is_ok());
        if let Ok(fresh) = fresh {
            assert_eq!(fresh.database.engine(), DatabaseEngine::Postgres);
            repository_contract(fresh.database.clone()).await;
            if let Database::Postgres(pool) = &fresh.database {
                pool.close().await;
            }
            let reopened = fresh.reconnect_database().await;
            assert!(reopened.is_ok());
            if let Ok(reopened) = reopened {
                assert!(matches!(
                    reopened
                        .authenticate_session(&persisted_authentication_fixture(56))
                        .await,
                    Ok(SessionAuthenticationOutcome::Authenticated(_))
                ));
                if let Database::Postgres(pool) = &reopened {
                    pool.close().await;
                }
            }
            assert!(fresh.cleanup().await.is_ok());
        }
        let legacy = isolated_postgres(&url, "nodecontroll_test_legacy").await;
        assert!(legacy.is_ok());
        if let Ok(legacy) = legacy {
            legacy_upgrade_contract(legacy.database.clone()).await;
            assert!(legacy.cleanup().await.is_ok());
        }
        let legacy_missing = isolated_postgres(&url, "nodecontroll_test_legacy_missing").await;
        assert!(legacy_missing.is_ok());
        if let Ok(legacy_missing) = legacy_missing {
            legacy_missing_settings_contract(legacy_missing.database.clone()).await;
            assert!(legacy_missing.cleanup().await.is_ok());
        }
        let rollback = isolated_postgres(&url, "nodecontroll_test_migration_rollback").await;
        assert!(rollback.is_ok());
        if let Ok(rollback) = rollback {
            migration_atomicity_contract(rollback.database.clone()).await;
            assert!(rollback.cleanup().await.is_ok());
        }
        let auth_upgrade = isolated_postgres(&url, "nodecontroll_test_auth_upgrade").await;
        assert!(auth_upgrade.is_ok());
        if let Ok(auth_upgrade) = auth_upgrade {
            auth_upgrade_contract(auth_upgrade.database.clone()).await;
            assert!(auth_upgrade.cleanup().await.is_ok());
        }
        let session_timeline_upgrade =
            isolated_postgres(&url, "nodecontroll_test_session_timeline_upgrade").await;
        assert!(session_timeline_upgrade.is_ok());
        if let Ok(session_timeline_upgrade) = session_timeline_upgrade {
            session_timeline_upgrade_contract(session_timeline_upgrade.database.clone()).await;
            assert!(session_timeline_upgrade.cleanup().await.is_ok());
        }
        let auth_rollback = isolated_postgres(&url, "nodecontroll_test_auth_rollback").await;
        assert!(auth_rollback.is_ok());
        if let Ok(auth_rollback) = auth_rollback {
            auth_migration_rollback_contract(auth_rollback.database.clone()).await;
            assert!(auth_rollback.cleanup().await.is_ok());
        }
        let recent_auth_migration_rollback =
            isolated_postgres(&url, "nodecontroll_test_recent_auth_migration_rollback").await;
        assert!(recent_auth_migration_rollback.is_ok());
        if let Ok(recent_auth_migration_rollback) = recent_auth_migration_rollback {
            postgres_recent_auth_migration_rollback_contract(
                recent_auth_migration_rollback.database.clone(),
            )
            .await;
            assert!(recent_auth_migration_rollback.cleanup().await.is_ok());
        }
        let session_timeline_migration_rollback = isolated_postgres(
            &url,
            "nodecontroll_test_session_timeline_migration_rollback",
        )
        .await;
        assert!(session_timeline_migration_rollback.is_ok());
        if let Ok(session_timeline_migration_rollback) = session_timeline_migration_rollback {
            postgres_session_timeline_migration_rollback_contract(
                session_timeline_migration_rollback.database.clone(),
            )
            .await;
            assert!(session_timeline_migration_rollback.cleanup().await.is_ok());
        }
    }

    #[test]
    fn unsupported_database_scheme_is_rejected_without_connecting() {
        assert!(matches!(
            Database::validate_url("mysql://localhost/nodecontroll"),
            Err(PersistenceError::UnsupportedDatabaseUrl)
        ));
    }
}
