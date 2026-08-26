use std::{str::FromStr, time::Duration};

use nodecontroll_domain::{
    EntityId, Instance, InstanceName, PasswordHash, PrincipalLabel, Revision,
    SubscriptionBehaviorSettings, UserAccount, UserRole, UserStatus, Username,
};
use sqlx::{
    PgPool, Row, SqlitePool,
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPoolOptions},
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use thiserror::Error;

static SQLITE_MIGRATOR: Migrator = sqlx::migrate!("./migrations/sqlite");
static POSTGRES_MIGRATOR: Migrator = sqlx::migrate!("./migrations/postgres");
const SUBSCRIPTION_SETTINGS_KEY: &str = "subscription.behavior";
const SUBSCRIPTION_SETTINGS_SCHEMA: i32 = 1;
pub const AUTH_HMAC_LENGTH: usize = 32;

pub type AuthHmac = [u8; AUTH_HMAC_LENGTH];

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
    Webauthn,
    Recovery,
}

impl AuthLevel {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Password => "password",
            Self::Mfa => "mfa",
            Self::Webauthn => "webauthn",
            Self::Recovery => "recovery",
        }
    }

    fn parse(value: &str) -> Result<Self, PersistenceError> {
        match value {
            "password" => Ok(Self::Password),
            "mfa" => Ok(Self::Mfa),
            "webauthn" => Ok(Self::Webauthn),
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

    pub async fn bootstrap_control_plane(
        &self,
        instance: &Instance,
        owner: &UserAccount,
        settings: &SubscriptionBehaviorSettings,
    ) -> Result<EntityId, PersistenceError> {
        if instance.created_at_ms < 0 || owner.created_at_ms < 0 {
            return Err(PersistenceError::InvalidTimestamp);
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
                )
                .await
            }
        }
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

    pub async fn create_auth_session(
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

    pub async fn authenticate_session(
        &self,
        authentication: &SessionAuthentication,
    ) -> Result<SessionAuthenticationOutcome, PersistenceError> {
        validate_session_authentication(authentication)?;
        match self {
            Self::Sqlite(pool) => authenticate_session_sqlite(pool, authentication).await,
            Self::Postgres(pool) => authenticate_session_postgres(pool, authentication).await,
        }
    }

    pub async fn revoke_current_session(
        &self,
        user_id: EntityId,
        session_id: EntityId,
        now_ms: i64,
        reason: SessionRevocationReason,
    ) -> Result<bool, PersistenceError> {
        validate_non_negative_timestamp(now_ms)?;
        let affected = match self {
            Self::Sqlite(pool) => sqlx::query(
                "UPDATE auth_sessions SET status='revoked',revoked_at_ms=?,revoked_reason=?,revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE id=? AND user_id=? AND status='active' AND created_at_ms<=?",
            )
            .bind(now_ms)
            .bind(reason.as_str())
            .bind(session_id.to_string())
            .bind(user_id.to_string())
            .bind(now_ms)
            .execute(pool)
            .await?
            .rows_affected(),
            Self::Postgres(pool) => sqlx::query(
                "UPDATE auth_sessions SET status='revoked',revoked_at_ms=$1,revoked_reason=$2,revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE id=$3 AND user_id=$4 AND status='active' AND created_at_ms<=$1",
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
        if reason != SessionRevocationReason::Logout
            || event.reason != LoginSecurityReason::Logout
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

    pub async fn logout_all_sessions(
        &self,
        user_id: EntityId,
        now_ms: i64,
    ) -> Result<LogoutAllResult, PersistenceError> {
        self.revoke_all_sessions_inner(user_id, now_ms, SessionRevocationReason::LogoutAll)
            .await
    }

    pub async fn logout_all_sessions_and_rotate(
        &self,
        user_id: EntityId,
        current_session_id: EntityId,
        replacement: &NewAuthSession,
        event: &NewLoginSecurityEvent,
        now_ms: i64,
    ) -> Result<LogoutAllResult, PersistenceError> {
        validate_non_negative_timestamp(now_ms)?;
        validate_new_session(replacement)?;
        validate_security_event(event)?;
        if replacement.user_id != user_id
            || replacement.id == current_session_id
            || event.reason != LoginSecurityReason::LogoutAll
            || event.occurred_at_ms != now_ms
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
                    replacement,
                    event,
                    now_ms,
                )
                .await
            }
        }
    }

    pub async fn advance_auth_revision_and_revoke_sessions(
        &self,
        user_id: EntityId,
        now_ms: i64,
        reason: SessionRevocationReason,
    ) -> Result<LogoutAllResult, PersistenceError> {
        self.revoke_all_sessions_inner(user_id, now_ms, reason)
            .await
    }

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

    pub async fn reserve_login_attempt(
        &self,
        reservation: &LoginAttemptReservation,
    ) -> Result<LoginRateDecision, PersistenceError> {
        validate_login_attempt_reservation(reservation)?;
        match self {
            Self::Sqlite(pool) => reserve_login_attempt_sqlite(pool, reservation).await,
            Self::Postgres(pool) => reserve_login_attempt_postgres(pool, reservation).await,
        }
    }

    pub async fn clear_login_account_bucket(
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
        || session.authenticated_at_ms < session.created_at_ms
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
        "SELECT revision FROM auth_sessions WHERE id=? AND user_id=? AND status='active' AND created_at_ms<=?",
    )
    .bind(session_id.to_string())
    .bind(user_id.to_string())
    .bind(now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    let Some(row) = row else {
        transaction.commit().await?;
        return Ok(false);
    };
    let revision: i64 = row.try_get("revision")?;
    let affected = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=?,revoked_reason=?,revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE id=? AND user_id=? AND status='active' AND revision=?",
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
        "SELECT revision FROM auth_sessions WHERE id=$1 AND user_id=$2 AND status='active' AND created_at_ms<=$3 FOR UPDATE",
    )
    .bind(session_id.into_uuid())
    .bind(user_id.into_uuid())
    .bind(now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    let Some(row) = row else {
        transaction.commit().await?;
        return Ok(false);
    };
    let revision: i64 = row.try_get("revision")?;
    let affected = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=$1,revoked_reason=$2,revision=CASE WHEN revision<9223372036854775807 THEN revision+1 ELSE revision END WHERE id=$3 AND user_id=$4 AND status='active' AND revision=$5",
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

async fn create_session_postgres(
    pool: &PgPool,
    session: &NewAuthSession,
    event: &NewLoginSecurityEvent,
) -> Result<AuthSessionSummary, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let auth_revision: Option<i64> = sqlx::query_scalar(
        "SELECT a.auth_revision FROM user_auth_state AS a JOIN users AS u ON u.id=a.user_id WHERE a.user_id=$1 AND u.status='active' AND u.deleted_at_ms IS NULL FOR UPDATE OF a,u",
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
        "SELECT CASE WHEN ? IS NULL THEN 1 WHEN s.csrf_key_version=? AND s.csrf_hmac=? THEN 1 ELSE 0 END AS csrf_matches,s.id AS session_id,s.user_id AS user_id,u.username AS username,u.role AS role,u.principal_label AS principal_label,u.force_password_change AS force_password_change,u.revision AS user_revision,s.auth_revision AS auth_revision,s.auth_level AS auth_level,s.created_at_ms AS created_at_ms,s.authenticated_at_ms AS authenticated_at_ms,s.recent_auth_at_ms AS recent_auth_at_ms,s.last_seen_at_ms AS last_seen_at_ms,s.idle_expires_at_ms AS idle_expires_at_ms,s.absolute_expires_at_ms AS absolute_expires_at_ms,s.ip_prefix_hmac IS NOT NULL AS has_ip_context,s.user_agent_hash IS NOT NULL AS has_user_agent_context,s.revision AS session_revision FROM auth_sessions AS s JOIN users AS u ON u.id=s.user_id JOIN user_auth_state AS a ON a.user_id=s.user_id WHERE s.token_key_version=? AND s.token_hmac=? AND s.status='active' AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND a.auth_revision=s.auth_revision",
    )
    .bind(csrf_key_version)
    .bind(csrf_key_version)
    .bind(csrf_hmac)
    .bind(database_key_version(authentication.token_key_version)?)
    .bind(authentication.token_hmac.as_slice())
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
    touch_authenticated_session_sqlite(&mut transaction, authentication, &mut authenticated)
        .await?;
    transaction.commit().await?;
    Ok(SessionAuthenticationOutcome::Authenticated(authenticated))
}

async fn authenticate_session_postgres(
    pool: &PgPool,
    authentication: &SessionAuthentication,
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
        "SELECT ($1::integer IS NULL OR (s.csrf_key_version=$1 AND s.csrf_hmac=$2)) AS csrf_matches,s.id AS session_id,s.user_id AS user_id,u.username AS username,u.role AS role,u.principal_label AS principal_label,u.force_password_change AS force_password_change,u.revision AS user_revision,s.auth_revision AS auth_revision,s.auth_level AS auth_level,s.created_at_ms AS created_at_ms,s.authenticated_at_ms AS authenticated_at_ms,s.recent_auth_at_ms AS recent_auth_at_ms,s.last_seen_at_ms AS last_seen_at_ms,s.idle_expires_at_ms AS idle_expires_at_ms,s.absolute_expires_at_ms AS absolute_expires_at_ms,s.ip_prefix_hmac IS NOT NULL AS has_ip_context,s.user_agent_hash IS NOT NULL AS has_user_agent_context,s.revision AS session_revision FROM auth_sessions AS s JOIN users AS u ON u.id=s.user_id JOIN user_auth_state AS a ON a.user_id=s.user_id WHERE s.token_key_version=$3 AND s.token_hmac=$4 AND s.status='active' AND s.idle_expires_at_ms>$5 AND s.absolute_expires_at_ms>$5 AND u.status='active' AND u.deleted_at_ms IS NULL AND a.auth_revision=s.auth_revision FOR UPDATE OF s,u,a",
    )
    .bind(csrf_key_version)
    .bind(csrf_hmac)
    .bind(database_key_version(authentication.token_key_version)?)
    .bind(authentication.token_hmac.as_slice())
    .bind(authentication.now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
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
    touch_authenticated_session_postgres(&mut transaction, authentication, &mut authenticated)
        .await?;
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

#[allow(clippy::too_many_arguments)]
fn validate_rotation_replacement(
    replacement: &NewAuthSession,
    current_auth_revision: Revision,
    current_auth_level: AuthLevel,
    current_absolute_expires_at_ms: i64,
    now_ms: i64,
) -> Result<Revision, PersistenceError> {
    let next = current_auth_revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    if replacement.auth_revision != next
        || replacement.auth_level != current_auth_level
        || replacement.created_at_ms != now_ms
        || replacement.authenticated_at_ms != now_ms
        || replacement.recent_auth_at_ms != now_ms
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
    let current: Option<(i64, String, i64)> = sqlx::query_as(
        "SELECT s.auth_revision,s.auth_level,s.absolute_expires_at_ms FROM auth_sessions AS s JOIN users AS u ON u.id=s.user_id JOIN user_auth_state AS a ON a.user_id=s.user_id WHERE s.id=? AND s.user_id=? AND s.status='active' AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND a.auth_revision=s.auth_revision",
    )
    .bind(current_session_id.to_string())
    .bind(user_id.to_string())
    .bind(now_ms)
    .bind(now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    let (current_auth_revision, current_auth_level, absolute_expires_at_ms) =
        current.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    let next = validate_rotation_replacement(
        replacement,
        decode_revision(current_auth_revision)?,
        AuthLevel::parse(&current_auth_level)?,
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
    replacement: &NewAuthSession,
    event: &NewLoginSecurityEvent,
    now_ms: i64,
) -> Result<LogoutAllResult, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let current: Option<(i64, String, i64)> = sqlx::query_as(
        "SELECT s.auth_revision,s.auth_level,s.absolute_expires_at_ms FROM auth_sessions AS s JOIN users AS u ON u.id=s.user_id JOIN user_auth_state AS a ON a.user_id=s.user_id WHERE s.id=$1 AND s.user_id=$2 AND s.status='active' AND s.idle_expires_at_ms>$3 AND s.absolute_expires_at_ms>$3 AND u.status='active' AND u.deleted_at_ms IS NULL AND a.auth_revision=s.auth_revision FOR UPDATE OF s,u,a",
    )
    .bind(current_session_id.into_uuid())
    .bind(user_id.into_uuid())
    .bind(now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    let (current_auth_revision, current_auth_level, absolute_expires_at_ms) =
        current.ok_or(PersistenceError::SessionPrincipalUnavailable)?;
    let next = validate_rotation_replacement(
        replacement,
        decode_revision(current_auth_revision)?,
        AuthLevel::parse(&current_auth_level)?,
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
        transaction.commit().await?;
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
        transaction.commit().await?;
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
    transaction.commit().await?;
    Ok(combine_rate_bucket_outcomes(
        &[account, ip, global],
        reservation.now_ms,
    ))
}

async fn reserve_login_attempt_postgres(
    pool: &PgPool,
    reservation: &LoginAttemptReservation,
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
        transaction.commit().await?;
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
        transaction.commit().await?;
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
) -> RateBucketOutcome {
    let attempts = u32::try_from(attempt_count).unwrap_or(u32::MAX);
    RateBucketOutcome {
        remaining_attempts: max_attempts.saturating_sub(attempts),
        reset_at_ms,
        blocked_until_ms,
    }
}

async fn bootstrap_sqlite(
    pool: &SqlitePool,
    instance: &Instance,
    owner: &UserAccount,
    settings_json: &str,
    instance_revision: i64,
    owner_revision: i64,
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
    #[error("session creation must atomically record a successful login event")]
    SessionEventMustRecordLoginSuccess,
    #[error("the session principal is unavailable or its authentication revision changed")]
    SessionPrincipalUnavailable,
    #[error("the user's authentication state is unavailable")]
    AuthStateUnavailable,
    #[error("the session revision changed during the operation")]
    SessionRevisionConflict,
    #[error("the requested logout-all rotation is invalid")]
    InvalidSessionRotation,
    #[error("the current-session revocation event does not match the logout transition")]
    InvalidSessionRevocationEvent,
    #[error("the stored session status is invalid")]
    InvalidStoredSessionStatus,
    #[error("the stored authentication level is invalid")]
    InvalidStoredAuthLevel,
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
    use sqlx::{
        PgPool,
        postgres::{PgConnectOptions, PgPoolOptions},
    };

    use super::{
        AuthLevel, AuthSessionStatus, BootstrapState, ConnectionSettings, Database, DatabaseEngine,
        LoginAttemptReservation, LoginRateDecision, LoginSecurityReason, NewAuthSession,
        NewLoginSecurityEvent, PersistenceError, SessionAuthentication,
        SessionAuthenticationOutcome, SessionRevocationReason,
    };

    fn settings() -> ConnectionSettings {
        ConnectionSettings {
            max_connections: 4,
            acquire_timeout: Duration::from_secs(5),
            statement_timeout: Duration::from_secs(30),
            lock_timeout: Duration::from_secs(5),
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
            "nodecontroll_test_auth_rollback" => {
                sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_auth_rollback CASCADE")
                    .execute(&admin)
                    .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_auth_rollback")
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
                "nodecontroll_test_auth_rollback" => {
                    sqlx::query("DROP SCHEMA nodecontroll_test_auth_rollback CASCADE")
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

    fn persisted_authentication_fixture() -> SessionAuthentication {
        SessionAuthentication {
            token_key_version: 1,
            token_hmac: [6; 32],
            csrf_key_version: None,
            csrf_hmac: None,
            now_ms: 1_777_777_777_506,
            touch_interval_ms: 50,
            idle_timeout_ms: 1_000,
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
                "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES (?,?,1,?,1,?,0,'password','active',100,99,100,100,200,300,NULL,NULL,NULL,NULL,NULL,0)",
            )
            .bind(EntityId::new().to_string())
            .bind(user.id.to_string())
            .bind(vec![1_u8; 32])
            .bind(vec![2_u8; 32])
            .execute(pool)
            .await
            .map(|_| ()),
            Database::Postgres(pool) => sqlx::query(
                "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES ($1,$2,1,$3,1,$4,0,'password','active',100,99,100,100,200,300,NULL,NULL,NULL,NULL,NULL,0)",
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
        assert!(database.migrate().await.is_ok());
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
        let wrong_reason_event =
            session_security_event(&first, 21, base + 299, LoginSecurityReason::LogoutAll);
        assert!(matches!(
            database
                .revoke_current_session_with_event(
                    owner.id,
                    first.id,
                    base + 299,
                    SessionRevocationReason::Logout,
                    &wrong_reason_event,
                )
                .await,
            Err(PersistenceError::InvalidSessionRevocationEvent)
        ));
        assert!(matches!(
            login_security_event_count(database, wrong_reason_event.id).await,
            Ok(0)
        ));
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
        let rotate_at_ms = base + 400;
        let replacement = auth_session_fixture(
            owner.id,
            5,
            Revision::from_value(1),
            rotate_at_ms,
            base + 10_000,
        );
        let rotation = database
            .logout_all_sessions_and_rotate(
                owner.id,
                current.id,
                &replacement,
                &login_security_event(5, rotate_at_ms, LoginSecurityReason::LogoutAll),
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
            Ok(SessionAuthenticationOutcome::Authenticated(_))
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

        let rate = LoginAttemptReservation {
            key_version: 1,
            account_hmac: [0x31; 32],
            ip_prefix_hmac: [0x32; 32],
            global_hmac: [0x33; 32],
            now_ms: base + 20_000,
            window_ms: 60_000,
            account_max_attempts: 2,
            ip_max_attempts: 10,
            global_max_attempts: 100,
            lockout_ms: 60_000,
        };
        let (first_rate, second_rate, third_rate, fourth_rate) = tokio::join!(
            database.reserve_login_attempt(&rate),
            database.reserve_login_attempt(&rate),
            database.reserve_login_attempt(&rate),
            database.reserve_login_attempt(&rate),
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

        let ip_limited = LoginAttemptReservation {
            key_version: 2,
            account_hmac: [0x51; 32],
            ip_prefix_hmac: [0x52; 32],
            global_hmac: [0x53; 32],
            now_ms: base + 30_000,
            window_ms: 60_000,
            account_max_attempts: 10,
            ip_max_attempts: 1,
            global_max_attempts: 100,
            lockout_ms: 60_000,
        };
        assert!(matches!(
            database.reserve_login_attempt(&ip_limited).await,
            Ok(LoginRateDecision::Allowed { .. })
        ));
        let second_account_same_ip = LoginAttemptReservation {
            account_hmac: [0x54; 32],
            now_ms: ip_limited.now_ms + 1,
            ..ip_limited.clone()
        };
        assert!(matches!(
            database
                .reserve_login_attempt(&second_account_same_ip)
                .await,
            Ok(LoginRateDecision::Limited { .. })
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
            now_ms: ip_limited.now_ms + 2,
            ..ip_limited.clone()
        };
        assert!(matches!(
            database
                .reserve_login_attempt(&rotated_account_while_ip_blocked)
                .await,
            Ok(LoginRateDecision::Limited { .. })
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
                now_ms: ip_limited.now_ms + 3 + i64::from(offset),
                ..ip_limited.clone()
            };
            assert!(matches!(
                database.reserve_login_attempt(&rotated).await,
                Ok(LoginRateDecision::Limited { .. })
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
            now_ms: base + 40_000,
            window_ms: 60_000,
            account_max_attempts: 10,
            ip_max_attempts: 10,
            global_max_attempts: 1,
            lockout_ms: 60_000,
        };
        assert!(matches!(
            database.reserve_login_attempt(&global_limited).await,
            Ok(LoginRateDecision::Allowed { .. })
        ));
        let second_origin_same_global = LoginAttemptReservation {
            account_hmac: [0x64; 32],
            ip_prefix_hmac: [0x65; 32],
            now_ms: global_limited.now_ms + 1,
            ..global_limited.clone()
        };
        assert!(matches!(
            database
                .reserve_login_attempt(&second_origin_same_global)
                .await,
            Ok(LoginRateDecision::Limited { .. })
        ));
        let rotated_origin_while_global_blocked = LoginAttemptReservation {
            account_hmac: [0x66; 32],
            ip_prefix_hmac: [0x67; 32],
            now_ms: global_limited.now_ms + 2,
            ..global_limited.clone()
        };
        assert!(matches!(
            database
                .reserve_login_attempt(&rotated_origin_while_global_blocked)
                .await,
            Ok(LoginRateDecision::Limited { .. })
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
                now_ms: global_limited.now_ms + 3 + i64::from(offset),
                ..global_limited.clone()
            };
            assert!(matches!(
                database.reserve_login_attempt(&rotated).await,
                Ok(LoginRateDecision::Limited { .. })
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
            now_ms: rate.now_ms + 1,
            ..rate
        };
        assert!(matches!(
            database.reserve_login_attempt(&after_clear).await,
            Ok(LoginRateDecision::Allowed {
                remaining_attempts: 1,
                ..
            })
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
        assert!(database.migrate().await.is_ok());
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::Uninitialized)
        ));
    }

    async fn repository_contract(database: Database) {
        assert!(database.migrate().await.is_ok());
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
        let (first_result, second_result) = tokio::join!(
            database.bootstrap_control_plane(&first_instance, &first_owner, &initial_settings),
            database.bootstrap_control_plane(
                &second_instance,
                &second_owner,
                &second_initial_settings
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
        auth_core_contract(&database, &owner).await;
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
        let auth_rollback_database = Database::connect("sqlite::memory:", settings()).await;
        assert!(auth_rollback_database.is_ok());
        if let Ok(database) = auth_rollback_database {
            auth_migration_rollback_contract(database).await;
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
                        .authenticate_session(&persisted_authentication_fixture())
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
                        .authenticate_session(&persisted_authentication_fixture())
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
        let auth_rollback = isolated_postgres(&url, "nodecontroll_test_auth_rollback").await;
        assert!(auth_rollback.is_ok());
        if let Ok(auth_rollback) = auth_rollback {
            auth_migration_rollback_contract(auth_rollback.database.clone()).await;
            assert!(auth_rollback.cleanup().await.is_ok());
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
