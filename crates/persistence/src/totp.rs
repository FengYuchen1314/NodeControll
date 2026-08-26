use nodecontroll_domain::{
    EntityId, Revision, TOTP_DIGITS, TOTP_PERIOD_SECONDS, TotpCredential, TotpCredentialStatus,
};
use nodecontroll_secrets::{
    SecretBinding, SecretOwnerKind, SecretPurpose, TOTP_SEED_SCHEMA_VERSION,
};
use sqlx::{PgPool, Row, SqlitePool};

use super::{
    Database, NewRecoveryCodeSet, NewSecretRecord, PersistenceError, RecoveryCodeSetSummary,
    StoredSecretRecord, database_revision, decode_recovery_code_summary, decode_secret_record_row,
    insert_recovery_code_set_postgres, insert_recovery_code_set_sqlite,
    insert_secret_record_postgres, insert_secret_record_sqlite, validate_non_negative_timestamp,
    validate_recovery_code_set, validate_secret_record,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TotpSessionGuard {
    pub user_id: EntityId,
    pub actor_session_id: EntityId,
    pub expected_user_revision: Revision,
    pub expected_auth_revision: Revision,
    pub expected_recent_auth_at_ms: i64,
    pub now_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewTotpEnrollment {
    pub credential_id: EntityId,
    pub secret: NewSecretRecord,
    pub guard: TotpSessionGuard,
    pub pending_expires_at_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredTotpCredential {
    pub credential: TotpCredential,
    pub secret: StoredSecretRecord,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BeginTotpEnrollmentOutcome {
    Created(StoredTotpCredential),
    AlreadyPending,
    Stale,
}

#[derive(Clone, Copy)]
pub struct ActivateTotpCredential<'a> {
    pub credential_id: EntityId,
    pub expected_credential_revision: Revision,
    pub accepted_step: u64,
    pub guard: TotpSessionGuard,
    pub recovery_codes: &'a NewRecoveryCodeSet,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TotpActivationResult {
    pub credential: TotpCredential,
    pub recovery_codes: RecoveryCodeSetSummary,
    pub auth_revision: Revision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ActivateTotpCredentialOutcome {
    Activated(TotpActivationResult),
    Stale,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TotpStepAdvance {
    pub credential_id: EntityId,
    pub user_id: EntityId,
    pub expected_credential_revision: Revision,
    pub expected_last_accepted_step: Option<u64>,
    pub accepted_step: u64,
    pub expected_auth_revision: Revision,
    pub session_id: Option<EntityId>,
    /// UTC time captured by C3 when the durable attempt slot was reserved. The TOTP window is
    /// permanently bound to this value even if verification crosses a 30-second boundary.
    pub verification_time_ms: i64,
    /// Repository commit time used only for principal/session expiry and revocation checks.
    pub now_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TotpStepAdvanceOutcome {
    Advanced(TotpCredential),
    Stale,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DisableTotpCredential {
    pub credential_id: EntityId,
    pub expected_credential_revision: Revision,
    pub guard: TotpSessionGuard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisableTotpCredentialOutcome {
    Disabled {
        disabled_credentials: u64,
        auth_revision: Revision,
    },
    Stale,
}

impl Database {
    /// Persists the pending credential and encrypted seed in one transaction. A crash before
    /// commit leaves no enrollment; a crash after commit may lose the one-shot seed response, but
    /// any previous active credential remains usable and the pending row can safely expire.
    pub async fn begin_totp_enrollment(
        &self,
        enrollment: &NewTotpEnrollment,
    ) -> Result<BeginTotpEnrollmentOutcome, PersistenceError> {
        validate_new_enrollment(enrollment)?;
        match self {
            Self::Sqlite(pool) => begin_enrollment_sqlite(pool, enrollment).await,
            Self::Postgres(pool) => begin_enrollment_postgres(pool, enrollment).await,
        }
    }

    pub async fn pending_totp_credential(
        &self,
        user_id: EntityId,
        now_ms: i64,
    ) -> Result<Option<StoredTotpCredential>, PersistenceError> {
        validate_non_negative_timestamp(now_ms)?;
        load_credential(self, user_id, TotpCredentialStatus::Pending, now_ms).await
    }

    pub async fn active_totp_credential(
        &self,
        user_id: EntityId,
        now_ms: i64,
    ) -> Result<Option<StoredTotpCredential>, PersistenceError> {
        validate_non_negative_timestamp(now_ms)?;
        load_credential(self, user_id, TotpCredentialStatus::Active, now_ms).await
    }

    /// The pending-to-active swap, old-seed tombstone, recovery-code replacement, authentication
    /// revision advance, actor-session promotion, and other-session revocation commit atomically.
    pub async fn activate_totp_credential(
        &self,
        activation: ActivateTotpCredential<'_>,
    ) -> Result<ActivateTotpCredentialOutcome, PersistenceError> {
        validate_activation(&activation)?;
        match self {
            Self::Sqlite(pool) => activate_sqlite(pool, &activation).await,
            Self::Postgres(pool) => activate_postgres(pool, &activation).await,
        }
    }

    /// The conditional update is the replay boundary. A crash after commit may burn one valid
    /// code, but can never recreate evidence or make that UTC step acceptable again.
    pub async fn advance_totp_step(
        &self,
        advance: &TotpStepAdvance,
    ) -> Result<TotpStepAdvanceOutcome, PersistenceError> {
        validate_step_advance(advance)?;
        match self {
            Self::Sqlite(pool) => advance_step_sqlite(pool, advance).await,
            Self::Postgres(pool) => advance_step_postgres(pool, advance).await,
        }
    }

    /// Seed tombstones, credential disablement, recovery-code revocation and authentication-state
    /// rotation are all-or-nothing.
    pub async fn disable_totp_credential(
        &self,
        disable: &DisableTotpCredential,
    ) -> Result<DisableTotpCredentialOutcome, PersistenceError> {
        validate_guard(&disable.guard)?;
        database_revision(disable.expected_credential_revision)?;
        match self {
            Self::Sqlite(pool) => disable_sqlite(pool, disable).await,
            Self::Postgres(pool) => disable_postgres(pool, disable).await,
        }
    }
}

fn validate_guard(guard: &TotpSessionGuard) -> Result<(), PersistenceError> {
    validate_non_negative_timestamp(guard.now_ms)?;
    validate_non_negative_timestamp(guard.expected_recent_auth_at_ms)?;
    database_revision(guard.expected_user_revision)?;
    database_revision(guard.expected_auth_revision)?;
    if guard.expected_recent_auth_at_ms > guard.now_ms {
        return Err(PersistenceError::InvalidTotpCredential);
    }
    Ok(())
}

fn validate_new_enrollment(enrollment: &NewTotpEnrollment) -> Result<(), PersistenceError> {
    validate_guard(&enrollment.guard)?;
    validate_secret_record(&enrollment.secret)?;
    let expected_binding = SecretBinding::new(
        SecretPurpose::TotpSeed,
        SecretOwnerKind::User,
        enrollment.guard.user_id.into_uuid(),
        TOTP_SEED_SCHEMA_VERSION,
    )
    .map_err(|_| PersistenceError::InvalidTotpCredential)?;
    if enrollment.secret.binding != expected_binding
        || enrollment.secret.created_at_ms != enrollment.guard.now_ms
        || enrollment.secret.rotated_from.is_some()
        || enrollment.pending_expires_at_ms <= enrollment.guard.now_ms
    {
        return Err(PersistenceError::InvalidTotpCredential);
    }
    Ok(())
}

fn validate_activation(activation: &ActivateTotpCredential<'_>) -> Result<(), PersistenceError> {
    validate_guard(&activation.guard)?;
    validate_recovery_code_set(activation.recovery_codes)?;
    database_revision(activation.expected_credential_revision)?;
    if activation.recovery_codes.created_at_ms != activation.guard.now_ms
        || activation.accepted_step > i64::MAX as u64
        || !step_is_in_validation_window(activation.accepted_step, activation.guard.now_ms)
    {
        return Err(PersistenceError::InvalidTotpCredential);
    }
    Ok(())
}

fn validate_step_advance(advance: &TotpStepAdvance) -> Result<(), PersistenceError> {
    validate_non_negative_timestamp(advance.now_ms)?;
    validate_non_negative_timestamp(advance.verification_time_ms)?;
    database_revision(advance.expected_credential_revision)?;
    database_revision(advance.expected_auth_revision)?;
    if advance.accepted_step > i64::MAX as u64
        || advance.verification_time_ms > advance.now_ms
        || !step_is_in_validation_window(advance.accepted_step, advance.verification_time_ms)
        || advance
            .expected_last_accepted_step
            .is_some_and(|step| step > i64::MAX as u64 || advance.accepted_step <= step)
    {
        return Err(PersistenceError::InvalidTotpCredential);
    }
    Ok(())
}

fn step_is_in_validation_window(step: u64, now_ms: i64) -> bool {
    let period_ms = i64::from(TOTP_PERIOD_SECONDS) * 1_000;
    let Ok(current) = u64::try_from(now_ms / period_ms) else {
        return false;
    };
    current.abs_diff(step) <= 1
}

type GuardSnapshot = (i64, i64, i64, String, Option<i64>, i64, i64, i64, i64);

fn validate_guard_snapshot(
    snapshot: Option<GuardSnapshot>,
    guard: &TotpSessionGuard,
) -> Result<(), PersistenceError> {
    let Some((
        user_revision,
        auth_revision,
        session_auth_revision,
        session_status,
        revoked_at_ms,
        recent_auth_at_ms,
        last_seen_at_ms,
        idle_expires_at_ms,
        absolute_expires_at_ms,
    )) = snapshot
    else {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    };
    if user_revision != database_revision(guard.expected_user_revision)?
        || auth_revision != database_revision(guard.expected_auth_revision)?
        || session_auth_revision != database_revision(guard.expected_auth_revision)?
        || session_status != "active"
        || revoked_at_ms.is_some()
        || recent_auth_at_ms != guard.expected_recent_auth_at_ms
        || last_seen_at_ms > guard.now_ms
        || idle_expires_at_ms <= guard.now_ms
        || absolute_expires_at_ms <= guard.now_ms
    {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    Ok(())
}

async fn lock_guard_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    guard: &TotpSessionGuard,
) -> Result<(), PersistenceError> {
    let locked = sqlx::query(
        "UPDATE user_auth_state SET updated_at_ms=updated_at_ms WHERE user_id=? AND auth_revision=?",
    )
    .bind(guard.user_id.to_string())
    .bind(database_revision(guard.expected_auth_revision)?)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    if locked != 1 {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let snapshot: Option<GuardSnapshot> = sqlx::query_as(
        "SELECT u.revision,uas.auth_revision,s.auth_revision,s.status,s.revoked_at_ms,s.recent_auth_at_ms,s.last_seen_at_ms,s.idle_expires_at_ms,s.absolute_expires_at_ms FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=? AND s.id=? AND u.status='active' AND u.deleted_at_ms IS NULL",
    )
    .bind(guard.user_id.to_string())
    .bind(guard.actor_session_id.to_string())
    .fetch_optional(&mut **transaction)
    .await?;
    validate_guard_snapshot(snapshot, guard)
}

async fn lock_guard_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    guard: &TotpSessionGuard,
) -> Result<(), PersistenceError> {
    let snapshot: Option<GuardSnapshot> = sqlx::query_as(
        "SELECT u.revision,uas.auth_revision,s.auth_revision,s.status,s.revoked_at_ms,s.recent_auth_at_ms,s.last_seen_at_ms,s.idle_expires_at_ms,s.absolute_expires_at_ms FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=$1 AND s.id=$2 AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=$3 FOR UPDATE OF u,uas,s",
    )
    .bind(guard.user_id.into_uuid())
    .bind(guard.actor_session_id.into_uuid())
    .bind(database_revision(guard.expected_auth_revision)?)
    .fetch_optional(&mut **transaction)
    .await?;
    validate_guard_snapshot(snapshot, guard)
}

async fn begin_enrollment_sqlite(
    pool: &SqlitePool,
    enrollment: &NewTotpEnrollment,
) -> Result<BeginTotpEnrollmentOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    lock_guard_sqlite(&mut transaction, &enrollment.guard).await?;
    expire_pending_sqlite(
        &mut transaction,
        enrollment.guard.user_id,
        enrollment.guard.now_ms,
    )
    .await?;
    let pending: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM totp_credentials WHERE user_id=? AND status='pending' AND pending_expires_at_ms>?)",
    )
    .bind(enrollment.guard.user_id.to_string())
    .bind(enrollment.guard.now_ms)
    .fetch_one(&mut *transaction)
    .await?;
    if pending {
        transaction.rollback().await?;
        return Ok(BeginTotpEnrollmentOutcome::AlreadyPending);
    }
    insert_secret_record_sqlite(&mut transaction, &enrollment.secret).await?;
    sqlx::query(
        "INSERT INTO totp_credentials (id,user_id,secret_record_id,status,algorithm,digits,period_seconds,created_at_ms,pending_expires_at_ms,activated_at_ms,disabled_at_ms,last_accepted_step,revision) VALUES (?,?,?,'pending','sha1',6,30,?,?,NULL,NULL,NULL,0)",
    )
    .bind(enrollment.credential_id.to_string())
    .bind(enrollment.guard.user_id.to_string())
    .bind(enrollment.secret.id.to_string())
    .bind(enrollment.guard.now_ms)
    .bind(enrollment.pending_expires_at_ms)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(BeginTotpEnrollmentOutcome::Created(project_new_enrollment(
        enrollment,
    )))
}

async fn begin_enrollment_postgres(
    pool: &PgPool,
    enrollment: &NewTotpEnrollment,
) -> Result<BeginTotpEnrollmentOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    lock_guard_postgres(&mut transaction, &enrollment.guard).await?;
    expire_pending_postgres(
        &mut transaction,
        enrollment.guard.user_id,
        enrollment.guard.now_ms,
    )
    .await?;
    let pending: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM totp_credentials WHERE user_id=$1 AND status='pending' AND pending_expires_at_ms>$2)",
    )
    .bind(enrollment.guard.user_id.into_uuid())
    .bind(enrollment.guard.now_ms)
    .fetch_one(&mut *transaction)
    .await?;
    if pending {
        transaction.rollback().await?;
        return Ok(BeginTotpEnrollmentOutcome::AlreadyPending);
    }
    insert_secret_record_postgres(&mut transaction, &enrollment.secret).await?;
    sqlx::query(
        "INSERT INTO totp_credentials (id,user_id,secret_record_id,status,algorithm,digits,period_seconds,created_at_ms,pending_expires_at_ms,activated_at_ms,disabled_at_ms,last_accepted_step,revision) VALUES ($1,$2,$3,'pending','sha1',6,30,$4,$5,NULL,NULL,NULL,0)",
    )
    .bind(enrollment.credential_id.into_uuid())
    .bind(enrollment.guard.user_id.into_uuid())
    .bind(enrollment.secret.id.into_uuid())
    .bind(enrollment.guard.now_ms)
    .bind(enrollment.pending_expires_at_ms)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(BeginTotpEnrollmentOutcome::Created(project_new_enrollment(
        enrollment,
    )))
}

fn project_new_enrollment(enrollment: &NewTotpEnrollment) -> StoredTotpCredential {
    StoredTotpCredential {
        credential: TotpCredential {
            id: enrollment.credential_id,
            user_id: enrollment.guard.user_id,
            secret_record_id: enrollment.secret.id,
            status: TotpCredentialStatus::Pending,
            created_at_ms: enrollment.guard.now_ms,
            pending_expires_at_ms: Some(enrollment.pending_expires_at_ms),
            activated_at_ms: None,
            disabled_at_ms: None,
            last_accepted_step: None,
            revision: Revision::initial(),
        },
        secret: StoredSecretRecord {
            id: enrollment.secret.id,
            binding: enrollment.secret.binding,
            envelope: enrollment.secret.envelope.clone(),
            created_at_ms: enrollment.secret.created_at_ms,
            rotated_from: enrollment.secret.rotated_from,
            revision: Revision::initial(),
        },
    }
}

async fn expire_pending_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    user_id: EntityId,
    now_ms: i64,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "UPDATE secret_records SET deleted_at_ms=?,revision=revision+1 WHERE id IN (SELECT secret_record_id FROM totp_credentials WHERE user_id=? AND status='pending' AND pending_expires_at_ms<=?) AND deleted_at_ms IS NULL",
    )
    .bind(now_ms)
    .bind(user_id.to_string())
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE totp_credentials SET status='disabled',pending_expires_at_ms=NULL,disabled_at_ms=?,revision=revision+1 WHERE user_id=? AND status='pending' AND pending_expires_at_ms<=?",
    )
    .bind(now_ms)
    .bind(user_id.to_string())
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn expire_pending_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: EntityId,
    now_ms: i64,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "UPDATE secret_records SET deleted_at_ms=$1,revision=revision+1 WHERE id IN (SELECT secret_record_id FROM totp_credentials WHERE user_id=$2 AND status='pending' AND pending_expires_at_ms<=$1) AND deleted_at_ms IS NULL",
    )
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE totp_credentials SET status='disabled',pending_expires_at_ms=NULL,disabled_at_ms=$1,revision=revision+1 WHERE user_id=$2 AND status='pending' AND pending_expires_at_ms<=$1",
    )
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn load_credential(
    database: &Database,
    user_id: EntityId,
    status: TotpCredentialStatus,
    now_ms: i64,
) -> Result<Option<StoredTotpCredential>, PersistenceError> {
    let row = match database {
        Database::Sqlite(pool) => sqlx::query(
            "SELECT tc.id AS credential_id,tc.user_id,tc.secret_record_id,tc.status,tc.algorithm,tc.digits,tc.period_seconds,tc.created_at_ms,tc.pending_expires_at_ms,tc.activated_at_ms,tc.disabled_at_ms,tc.last_accepted_step,tc.revision AS credential_revision,sr.id AS secret_id,sr.owner_type,sr.owner_id,sr.purpose,sr.schema_version,sr.key_version,sr.nonce,sr.ciphertext,sr.aad_hash,sr.created_at_ms AS secret_created_at_ms,sr.rotated_from,sr.revision AS secret_revision FROM totp_credentials tc JOIN secret_records sr ON sr.id=tc.secret_record_id AND sr.deleted_at_ms IS NULL JOIN users u ON u.id=tc.user_id WHERE tc.user_id=? AND tc.status=? AND u.status='active' AND u.deleted_at_ms IS NULL AND (tc.status<>'pending' OR tc.pending_expires_at_ms>?)",
        )
        .bind(user_id.to_string())
        .bind(status.as_str())
        .bind(now_ms)
        .fetch_optional(pool)
        .await?
        .map(decode_sqlite_totp_row)
        .transpose()?,
        Database::Postgres(pool) => sqlx::query(
            "SELECT tc.id AS credential_id,tc.user_id,tc.secret_record_id,tc.status,tc.algorithm,tc.digits,tc.period_seconds,tc.created_at_ms,tc.pending_expires_at_ms,tc.activated_at_ms,tc.disabled_at_ms,tc.last_accepted_step,tc.revision AS credential_revision,sr.id AS secret_id,sr.owner_type,sr.owner_id,sr.purpose,sr.schema_version,sr.key_version,sr.nonce,sr.ciphertext,sr.aad_hash,sr.created_at_ms AS secret_created_at_ms,sr.rotated_from,sr.revision AS secret_revision FROM totp_credentials tc JOIN secret_records sr ON sr.id=tc.secret_record_id AND sr.deleted_at_ms IS NULL JOIN users u ON u.id=tc.user_id WHERE tc.user_id=$1 AND tc.status=$2 AND u.status='active' AND u.deleted_at_ms IS NULL AND (tc.status<>'pending' OR tc.pending_expires_at_ms>$3)",
        )
        .bind(user_id.into_uuid())
        .bind(status.as_str())
        .bind(now_ms)
        .fetch_optional(pool)
        .await?
        .map(decode_postgres_totp_row)
        .transpose()?,
    };
    if row
        .as_ref()
        .is_some_and(|stored| stored.credential.status != status)
    {
        return Err(PersistenceError::InvalidStoredTotpCredential);
    }
    Ok(row)
}

fn decode_sqlite_totp_row(
    row: sqlx::sqlite::SqliteRow,
) -> Result<StoredTotpCredential, PersistenceError> {
    let credential_id = uuid::Uuid::parse_str(row.try_get::<&str, _>("credential_id")?)?;
    let user_id = uuid::Uuid::parse_str(row.try_get::<&str, _>("user_id")?)?;
    let secret_record_id = uuid::Uuid::parse_str(row.try_get::<&str, _>("secret_record_id")?)?;
    let secret_id = uuid::Uuid::parse_str(row.try_get::<&str, _>("secret_id")?)?;
    let owner_id = uuid::Uuid::parse_str(row.try_get::<&str, _>("owner_id")?)?;
    let rotated_from = row
        .try_get::<Option<&str>, _>("rotated_from")?
        .map(uuid::Uuid::parse_str)
        .transpose()?;
    decode_totp_row(
        credential_id,
        user_id,
        secret_record_id,
        row.try_get("status")?,
        row.try_get("algorithm")?,
        row.try_get("digits")?,
        row.try_get("period_seconds")?,
        row.try_get("created_at_ms")?,
        row.try_get("pending_expires_at_ms")?,
        row.try_get("activated_at_ms")?,
        row.try_get("disabled_at_ms")?,
        row.try_get("last_accepted_step")?,
        row.try_get("credential_revision")?,
        secret_id,
        row.try_get("owner_type")?,
        owner_id,
        row.try_get("purpose")?,
        row.try_get("schema_version")?,
        row.try_get("key_version")?,
        row.try_get("nonce")?,
        row.try_get("ciphertext")?,
        row.try_get("aad_hash")?,
        row.try_get("secret_created_at_ms")?,
        rotated_from,
        row.try_get("secret_revision")?,
    )
}

fn decode_postgres_totp_row(
    row: sqlx::postgres::PgRow,
) -> Result<StoredTotpCredential, PersistenceError> {
    decode_totp_row(
        row.try_get("credential_id")?,
        row.try_get("user_id")?,
        row.try_get("secret_record_id")?,
        row.try_get("status")?,
        row.try_get("algorithm")?,
        row.try_get::<i16, _>("digits")?.into(),
        row.try_get("period_seconds")?,
        row.try_get("created_at_ms")?,
        row.try_get("pending_expires_at_ms")?,
        row.try_get("activated_at_ms")?,
        row.try_get("disabled_at_ms")?,
        row.try_get("last_accepted_step")?,
        row.try_get("credential_revision")?,
        row.try_get("secret_id")?,
        row.try_get("owner_type")?,
        row.try_get("owner_id")?,
        row.try_get("purpose")?,
        row.try_get("schema_version")?,
        row.try_get("key_version")?,
        row.try_get("nonce")?,
        row.try_get("ciphertext")?,
        row.try_get("aad_hash")?,
        row.try_get("secret_created_at_ms")?,
        row.try_get("rotated_from")?,
        row.try_get("secret_revision")?,
    )
}

#[allow(clippy::too_many_arguments)]
fn decode_totp_row(
    credential_id: uuid::Uuid,
    user_id: uuid::Uuid,
    secret_record_id: uuid::Uuid,
    status: String,
    algorithm: String,
    digits: i32,
    period_seconds: i32,
    created_at_ms: i64,
    pending_expires_at_ms: Option<i64>,
    activated_at_ms: Option<i64>,
    disabled_at_ms: Option<i64>,
    last_accepted_step: Option<i64>,
    credential_revision: i64,
    secret_id: uuid::Uuid,
    owner_type: String,
    owner_id: uuid::Uuid,
    purpose: String,
    schema_version: i32,
    key_version: i32,
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
    aad_hash: Vec<u8>,
    secret_created_at_ms: i64,
    rotated_from: Option<uuid::Uuid>,
    secret_revision: i64,
) -> Result<StoredTotpCredential, PersistenceError> {
    let status = TotpCredentialStatus::parse(&status)
        .map_err(|_| PersistenceError::InvalidStoredTotpCredential)?;
    let last_accepted_step = last_accepted_step
        .map(u64::try_from)
        .transpose()
        .map_err(|_| PersistenceError::InvalidStoredTotpCredential)?;
    let credential_revision = u64::try_from(credential_revision)
        .map_err(|_| PersistenceError::InvalidStoredTotpCredential)?;
    let valid_state = match status {
        TotpCredentialStatus::Pending => {
            pending_expires_at_ms.is_some_and(|expires| expires > created_at_ms)
                && activated_at_ms.is_none()
                && disabled_at_ms.is_none()
                && last_accepted_step.is_none()
        }
        TotpCredentialStatus::Active => {
            pending_expires_at_ms.is_none()
                && activated_at_ms.is_some_and(|activated| activated >= created_at_ms)
                && disabled_at_ms.is_none()
                && last_accepted_step.is_some()
        }
        TotpCredentialStatus::Disabled => {
            pending_expires_at_ms.is_none()
                && disabled_at_ms.is_some_and(|disabled| disabled >= created_at_ms)
                && ((activated_at_ms.is_none() && last_accepted_step.is_none())
                    || (activated_at_ms.is_some() && last_accepted_step.is_some()))
        }
    };
    if secret_id != secret_record_id
        || algorithm != "sha1"
        || digits != i32::from(TOTP_DIGITS)
        || period_seconds != i32::try_from(TOTP_PERIOD_SECONDS).unwrap_or_default()
        || created_at_ms < 0
        || secret_created_at_ms != created_at_ms
        || !valid_state
    {
        return Err(PersistenceError::InvalidStoredTotpCredential);
    }
    let secret = decode_secret_record_row(
        secret_id,
        owner_type,
        owner_id,
        purpose,
        schema_version,
        key_version,
        nonce,
        ciphertext,
        aad_hash,
        secret_created_at_ms,
        rotated_from,
        secret_revision,
    )?;
    let expected_binding = SecretBinding::new(
        SecretPurpose::TotpSeed,
        SecretOwnerKind::User,
        user_id,
        TOTP_SEED_SCHEMA_VERSION,
    )
    .map_err(|_| PersistenceError::InvalidStoredTotpCredential)?;
    if secret.id.into_uuid() != secret_record_id || secret.binding != expected_binding {
        return Err(PersistenceError::InvalidStoredTotpCredential);
    }
    Ok(StoredTotpCredential {
        credential: TotpCredential {
            id: EntityId::from_uuid(credential_id),
            user_id: EntityId::from_uuid(user_id),
            secret_record_id: EntityId::from_uuid(secret_record_id),
            status,
            created_at_ms,
            pending_expires_at_ms,
            activated_at_ms,
            disabled_at_ms,
            last_accepted_step,
            revision: Revision::from_value(credential_revision),
        },
        secret,
    })
}

async fn activate_sqlite(
    pool: &SqlitePool,
    activation: &ActivateTotpCredential<'_>,
) -> Result<ActivateTotpCredentialOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    lock_guard_sqlite(&mut transaction, &activation.guard).await?;
    let pending: Option<(String, i64)> = sqlx::query_as(
        "SELECT secret_record_id,created_at_ms FROM totp_credentials WHERE id=? AND user_id=? AND status='pending' AND revision=? AND pending_expires_at_ms>?",
    )
    .bind(activation.credential_id.to_string())
    .bind(activation.guard.user_id.to_string())
    .bind(database_revision(activation.expected_credential_revision)?)
    .bind(activation.guard.now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    let Some((secret_record_id, created_at_ms)) = pending else {
        transaction.rollback().await?;
        return Ok(ActivateTotpCredentialOutcome::Stale);
    };
    sqlx::query(
        "UPDATE secret_records SET deleted_at_ms=?,revision=revision+1 WHERE id IN (SELECT secret_record_id FROM totp_credentials WHERE user_id=? AND status='active') AND deleted_at_ms IS NULL",
    )
    .bind(activation.guard.now_ms)
    .bind(activation.guard.user_id.to_string())
    .execute(&mut *transaction)
    .await?;
    sqlx::query(
        "UPDATE totp_credentials SET status='disabled',disabled_at_ms=?,revision=revision+1 WHERE user_id=? AND status='active'",
    )
    .bind(activation.guard.now_ms)
    .bind(activation.guard.user_id.to_string())
    .execute(&mut *transaction)
    .await?;
    let activated = sqlx::query(
        "UPDATE totp_credentials SET status='active',pending_expires_at_ms=NULL,activated_at_ms=?,last_accepted_step=?,revision=revision+1 WHERE id=? AND user_id=? AND status='pending' AND revision=? AND pending_expires_at_ms>?",
    )
    .bind(activation.guard.now_ms)
    .bind(i64::try_from(activation.accepted_step).map_err(|_| PersistenceError::InvalidTotpCredential)?)
    .bind(activation.credential_id.to_string())
    .bind(activation.guard.user_id.to_string())
    .bind(database_revision(activation.expected_credential_revision)?)
    .bind(activation.guard.now_ms)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if activated != 1 {
        transaction.rollback().await?;
        return Ok(ActivateTotpCredentialOutcome::Stale);
    }
    let recovery = replace_recovery_codes_in_sqlite_transaction(
        &mut transaction,
        activation.guard.user_id,
        activation.recovery_codes,
        activation.guard.now_ms,
    )
    .await?;
    let auth_revision =
        advance_auth_state_sqlite(&mut transaction, &activation.guard, true).await?;
    transaction.commit().await?;
    Ok(ActivateTotpCredentialOutcome::Activated(
        TotpActivationResult {
            credential: TotpCredential {
                id: activation.credential_id,
                user_id: activation.guard.user_id,
                secret_record_id: EntityId::from_uuid(uuid::Uuid::parse_str(&secret_record_id)?),
                status: TotpCredentialStatus::Active,
                created_at_ms,
                pending_expires_at_ms: None,
                activated_at_ms: Some(activation.guard.now_ms),
                disabled_at_ms: None,
                last_accepted_step: Some(activation.accepted_step),
                revision: activation
                    .expected_credential_revision
                    .next()
                    .map_err(|_| PersistenceError::RevisionOutOfRange)?,
            },
            recovery_codes: recovery,
            auth_revision,
        },
    ))
}

async fn activate_postgres(
    pool: &PgPool,
    activation: &ActivateTotpCredential<'_>,
) -> Result<ActivateTotpCredentialOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    lock_guard_postgres(&mut transaction, &activation.guard).await?;
    let pending: Option<(uuid::Uuid, i64)> = sqlx::query_as(
        "SELECT secret_record_id,created_at_ms FROM totp_credentials WHERE id=$1 AND user_id=$2 AND status='pending' AND revision=$3 AND pending_expires_at_ms>$4 FOR UPDATE",
    )
    .bind(activation.credential_id.into_uuid())
    .bind(activation.guard.user_id.into_uuid())
    .bind(database_revision(activation.expected_credential_revision)?)
    .bind(activation.guard.now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    let Some((secret_record_id, created_at_ms)) = pending else {
        transaction.rollback().await?;
        return Ok(ActivateTotpCredentialOutcome::Stale);
    };
    sqlx::query(
        "UPDATE secret_records SET deleted_at_ms=$1,revision=revision+1 WHERE id IN (SELECT secret_record_id FROM totp_credentials WHERE user_id=$2 AND status='active') AND deleted_at_ms IS NULL",
    )
    .bind(activation.guard.now_ms)
    .bind(activation.guard.user_id.into_uuid())
    .execute(&mut *transaction)
    .await?;
    sqlx::query(
        "UPDATE totp_credentials SET status='disabled',disabled_at_ms=$1,revision=revision+1 WHERE user_id=$2 AND status='active'",
    )
    .bind(activation.guard.now_ms)
    .bind(activation.guard.user_id.into_uuid())
    .execute(&mut *transaction)
    .await?;
    let activated = sqlx::query(
        "UPDATE totp_credentials SET status='active',pending_expires_at_ms=NULL,activated_at_ms=$1,last_accepted_step=$2,revision=revision+1 WHERE id=$3 AND user_id=$4 AND status='pending' AND revision=$5 AND pending_expires_at_ms>$1",
    )
    .bind(activation.guard.now_ms)
    .bind(i64::try_from(activation.accepted_step).map_err(|_| PersistenceError::InvalidTotpCredential)?)
    .bind(activation.credential_id.into_uuid())
    .bind(activation.guard.user_id.into_uuid())
    .bind(database_revision(activation.expected_credential_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if activated != 1 {
        transaction.rollback().await?;
        return Ok(ActivateTotpCredentialOutcome::Stale);
    }
    let recovery = replace_recovery_codes_in_postgres_transaction(
        &mut transaction,
        activation.guard.user_id,
        activation.recovery_codes,
        activation.guard.now_ms,
    )
    .await?;
    let auth_revision =
        advance_auth_state_postgres(&mut transaction, &activation.guard, true).await?;
    transaction.commit().await?;
    Ok(ActivateTotpCredentialOutcome::Activated(
        TotpActivationResult {
            credential: TotpCredential {
                id: activation.credential_id,
                user_id: activation.guard.user_id,
                secret_record_id: EntityId::from_uuid(secret_record_id),
                status: TotpCredentialStatus::Active,
                created_at_ms,
                pending_expires_at_ms: None,
                activated_at_ms: Some(activation.guard.now_ms),
                disabled_at_ms: None,
                last_accepted_step: Some(activation.accepted_step),
                revision: activation
                    .expected_credential_revision
                    .next()
                    .map_err(|_| PersistenceError::RevisionOutOfRange)?,
            },
            recovery_codes: recovery,
            auth_revision,
        },
    ))
}

async fn replace_recovery_codes_in_sqlite_transaction(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    user_id: EntityId,
    replacement: &NewRecoveryCodeSet,
    now_ms: i64,
) -> Result<RecoveryCodeSetSummary, PersistenceError> {
    let next_version: i64 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(set_version),0)+1 FROM recovery_code_sets WHERE user_id=?",
    )
    .bind(user_id.to_string())
    .fetch_one(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE recovery_code_sets SET status='replaced',replaced_at_ms=? WHERE user_id=? AND status='active' AND created_at_ms<=?",
    )
    .bind(now_ms)
    .bind(user_id.to_string())
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    insert_recovery_code_set_sqlite(transaction, user_id, next_version, replacement).await?;
    decode_recovery_code_summary((next_version, 8, 8, now_ms))
}

async fn replace_recovery_codes_in_postgres_transaction(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: EntityId,
    replacement: &NewRecoveryCodeSet,
    now_ms: i64,
) -> Result<RecoveryCodeSetSummary, PersistenceError> {
    let next_version: i64 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(set_version),0)+1 FROM recovery_code_sets WHERE user_id=$1",
    )
    .bind(user_id.into_uuid())
    .fetch_one(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE recovery_code_sets SET status='replaced',replaced_at_ms=$1 WHERE user_id=$2 AND status='active' AND created_at_ms<=$1",
    )
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .execute(&mut **transaction)
    .await?;
    insert_recovery_code_set_postgres(transaction, user_id, next_version, replacement).await?;
    decode_recovery_code_summary((next_version, 8, 8, now_ms))
}

async fn advance_auth_state_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    guard: &TotpSessionGuard,
    promote_mfa: bool,
) -> Result<Revision, PersistenceError> {
    let next = guard
        .expected_auth_revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let changed = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=?,updated_at_ms=? WHERE user_id=? AND auth_revision=?",
    )
    .bind(database_revision(next)?)
    .bind(guard.now_ms)
    .bind(guard.user_id.to_string())
    .bind(database_revision(guard.expected_auth_revision)?)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    if changed != 1 {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=?,revoked_reason='security_policy',revision=revision+1 WHERE user_id=? AND id<>? AND status='active' AND auth_revision=?",
    )
    .bind(guard.now_ms)
    .bind(guard.user_id.to_string())
    .bind(guard.actor_session_id.to_string())
    .bind(database_revision(guard.expected_auth_revision)?)
    .execute(&mut **transaction)
    .await?;
    let actor_changed = if promote_mfa {
        sqlx::query(
            "UPDATE auth_sessions SET auth_revision=?,auth_level='mfa',recent_auth_at_ms=?,last_seen_at_ms=?,revision=revision+1 WHERE id=? AND user_id=? AND status='active' AND auth_revision=? AND recent_auth_at_ms=? AND last_seen_at_ms<=? AND idle_expires_at_ms>? AND absolute_expires_at_ms>?",
        )
        .bind(database_revision(next)?)
        .bind(guard.now_ms)
        .bind(guard.now_ms)
        .bind(guard.actor_session_id.to_string())
        .bind(guard.user_id.to_string())
        .bind(database_revision(guard.expected_auth_revision)?)
        .bind(guard.expected_recent_auth_at_ms)
        .bind(guard.now_ms)
        .bind(guard.now_ms)
        .bind(guard.now_ms)
        .execute(&mut **transaction)
        .await?
        .rows_affected()
    } else {
        sqlx::query(
            "UPDATE auth_sessions SET auth_revision=?,last_seen_at_ms=?,revision=revision+1 WHERE id=? AND user_id=? AND status='active' AND auth_revision=? AND recent_auth_at_ms=? AND last_seen_at_ms<=? AND idle_expires_at_ms>? AND absolute_expires_at_ms>?",
        )
        .bind(database_revision(next)?)
        .bind(guard.now_ms)
        .bind(guard.actor_session_id.to_string())
        .bind(guard.user_id.to_string())
        .bind(database_revision(guard.expected_auth_revision)?)
        .bind(guard.expected_recent_auth_at_ms)
        .bind(guard.now_ms)
        .bind(guard.now_ms)
        .bind(guard.now_ms)
        .execute(&mut **transaction)
        .await?
        .rows_affected()
    };
    if actor_changed != 1 {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    Ok(next)
}

async fn advance_auth_state_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    guard: &TotpSessionGuard,
    promote_mfa: bool,
) -> Result<Revision, PersistenceError> {
    let next = guard
        .expected_auth_revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let changed = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=$1,updated_at_ms=$2 WHERE user_id=$3 AND auth_revision=$4",
    )
    .bind(database_revision(next)?)
    .bind(guard.now_ms)
    .bind(guard.user_id.into_uuid())
    .bind(database_revision(guard.expected_auth_revision)?)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    if changed != 1 {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=$1,revoked_reason='security_policy',revision=revision+1 WHERE user_id=$2 AND id<>$3 AND status='active' AND auth_revision=$4",
    )
    .bind(guard.now_ms)
    .bind(guard.user_id.into_uuid())
    .bind(guard.actor_session_id.into_uuid())
    .bind(database_revision(guard.expected_auth_revision)?)
    .execute(&mut **transaction)
    .await?;
    let actor_changed = if promote_mfa {
        sqlx::query(
            "UPDATE auth_sessions SET auth_revision=$1,auth_level='mfa',recent_auth_at_ms=$2,last_seen_at_ms=$2,revision=revision+1 WHERE id=$3 AND user_id=$4 AND status='active' AND auth_revision=$5 AND recent_auth_at_ms=$6 AND last_seen_at_ms<=$2 AND idle_expires_at_ms>$2 AND absolute_expires_at_ms>$2",
        )
        .bind(database_revision(next)?)
        .bind(guard.now_ms)
        .bind(guard.actor_session_id.into_uuid())
        .bind(guard.user_id.into_uuid())
        .bind(database_revision(guard.expected_auth_revision)?)
        .bind(guard.expected_recent_auth_at_ms)
        .execute(&mut **transaction)
        .await?
        .rows_affected()
    } else {
        sqlx::query(
            "UPDATE auth_sessions SET auth_revision=$1,last_seen_at_ms=$2,revision=revision+1 WHERE id=$3 AND user_id=$4 AND status='active' AND auth_revision=$5 AND recent_auth_at_ms=$6 AND last_seen_at_ms<=$2 AND idle_expires_at_ms>$2 AND absolute_expires_at_ms>$2",
        )
        .bind(database_revision(next)?)
        .bind(guard.now_ms)
        .bind(guard.actor_session_id.into_uuid())
        .bind(guard.user_id.into_uuid())
        .bind(database_revision(guard.expected_auth_revision)?)
        .bind(guard.expected_recent_auth_at_ms)
        .execute(&mut **transaction)
        .await?
        .rows_affected()
    };
    if actor_changed != 1 {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    Ok(next)
}

async fn advance_step_sqlite(
    pool: &SqlitePool,
    advance: &TotpStepAdvance,
) -> Result<TotpStepAdvanceOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let row = sqlx::query(
        "UPDATE totp_credentials SET last_accepted_step=?,revision=revision+1 WHERE id=? AND user_id=? AND status='active' AND revision=? AND last_accepted_step IS ? AND last_accepted_step<? AND EXISTS(SELECT 1 FROM users u JOIN user_auth_state uas ON uas.user_id=u.id WHERE u.id=totp_credentials.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=?) AND (? IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=? AND s.user_id=totp_credentials.user_id AND s.status='active' AND s.auth_revision=? AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?)) RETURNING secret_record_id,created_at_ms,activated_at_ms,revision",
    )
    .bind(i64::try_from(advance.accepted_step).map_err(|_| PersistenceError::InvalidTotpCredential)?)
    .bind(advance.credential_id.to_string())
    .bind(advance.user_id.to_string())
    .bind(database_revision(advance.expected_credential_revision)?)
    .bind(advance.expected_last_accepted_step.map(|step| i64::try_from(step).unwrap_or(i64::MAX)))
    .bind(i64::try_from(advance.accepted_step).map_err(|_| PersistenceError::InvalidTotpCredential)?)
    .bind(database_revision(advance.expected_auth_revision)?)
    .bind(advance.session_id.map(|id| id.to_string()))
    .bind(advance.session_id.map(|id| id.to_string()))
    .bind(database_revision(advance.expected_auth_revision)?)
    .bind(advance.now_ms)
    .bind(advance.now_ms)
    .bind(advance.now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    let Some(row) = row else {
        transaction.rollback().await?;
        return Ok(TotpStepAdvanceOutcome::Stale);
    };
    let secret_record_id = uuid::Uuid::parse_str(row.try_get::<&str, _>("secret_record_id")?)?;
    let created_at_ms: i64 = row.try_get("created_at_ms")?;
    let activated_at_ms: Option<i64> = row.try_get("activated_at_ms")?;
    let revision = u64::try_from(row.try_get::<i64, _>("revision")?)
        .map_err(|_| PersistenceError::InvalidStoredTotpCredential)?;
    transaction.commit().await?;
    Ok(TotpStepAdvanceOutcome::Advanced(TotpCredential {
        id: advance.credential_id,
        user_id: advance.user_id,
        secret_record_id: EntityId::from_uuid(secret_record_id),
        status: TotpCredentialStatus::Active,
        created_at_ms,
        pending_expires_at_ms: None,
        activated_at_ms,
        disabled_at_ms: None,
        last_accepted_step: Some(advance.accepted_step),
        revision: Revision::from_value(revision),
    }))
}

async fn advance_step_postgres(
    pool: &PgPool,
    advance: &TotpStepAdvance,
) -> Result<TotpStepAdvanceOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let row = sqlx::query(
        "UPDATE totp_credentials SET last_accepted_step=$1,revision=revision+1 WHERE id=$2 AND user_id=$3 AND status='active' AND revision=$4 AND last_accepted_step IS NOT DISTINCT FROM $5 AND last_accepted_step<$1 AND EXISTS(SELECT 1 FROM users u JOIN user_auth_state uas ON uas.user_id=u.id WHERE u.id=totp_credentials.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=$6) AND ($7::uuid IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=$8 AND s.user_id=totp_credentials.user_id AND s.status='active' AND s.auth_revision=$6 AND s.last_seen_at_ms<=$9 AND s.idle_expires_at_ms>$9 AND s.absolute_expires_at_ms>$9)) RETURNING secret_record_id,created_at_ms,activated_at_ms,revision",
    )
    .bind(i64::try_from(advance.accepted_step).map_err(|_| PersistenceError::InvalidTotpCredential)?)
    .bind(advance.credential_id.into_uuid())
    .bind(advance.user_id.into_uuid())
    .bind(database_revision(advance.expected_credential_revision)?)
    .bind(advance.expected_last_accepted_step.map(|step| i64::try_from(step).unwrap_or(i64::MAX)))
    .bind(database_revision(advance.expected_auth_revision)?)
    .bind(advance.session_id.map(EntityId::into_uuid))
    .bind(advance.session_id.map(EntityId::into_uuid))
    .bind(advance.now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    let Some(row) = row else {
        transaction.rollback().await?;
        return Ok(TotpStepAdvanceOutcome::Stale);
    };
    let credential = TotpCredential {
        id: advance.credential_id,
        user_id: advance.user_id,
        secret_record_id: EntityId::from_uuid(row.try_get("secret_record_id")?),
        status: TotpCredentialStatus::Active,
        created_at_ms: row.try_get("created_at_ms")?,
        pending_expires_at_ms: None,
        activated_at_ms: row.try_get("activated_at_ms")?,
        disabled_at_ms: None,
        last_accepted_step: Some(advance.accepted_step),
        revision: Revision::from_value(
            u64::try_from(row.try_get::<i64, _>("revision")?)
                .map_err(|_| PersistenceError::InvalidStoredTotpCredential)?,
        ),
    };
    transaction.commit().await?;
    Ok(TotpStepAdvanceOutcome::Advanced(credential))
}

async fn disable_sqlite(
    pool: &SqlitePool,
    disable: &DisableTotpCredential,
) -> Result<DisableTotpCredentialOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    lock_guard_sqlite(&mut transaction, &disable.guard).await?;
    let target_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM totp_credentials WHERE id=? AND user_id=? AND status IN ('pending','active') AND revision=?)",
    )
    .bind(disable.credential_id.to_string())
    .bind(disable.guard.user_id.to_string())
    .bind(database_revision(disable.expected_credential_revision)?)
    .fetch_one(&mut *transaction)
    .await?;
    if !target_exists {
        transaction.rollback().await?;
        return Ok(DisableTotpCredentialOutcome::Stale);
    }
    sqlx::query(
        "UPDATE secret_records SET deleted_at_ms=?,revision=revision+1 WHERE id IN (SELECT secret_record_id FROM totp_credentials WHERE user_id=? AND status IN ('pending','active')) AND deleted_at_ms IS NULL",
    )
    .bind(disable.guard.now_ms)
    .bind(disable.guard.user_id.to_string())
    .execute(&mut *transaction)
    .await?;
    let disabled_credentials = sqlx::query(
        "UPDATE totp_credentials SET status='disabled',pending_expires_at_ms=NULL,disabled_at_ms=?,revision=revision+1 WHERE user_id=? AND status IN ('pending','active')",
    )
    .bind(disable.guard.now_ms)
    .bind(disable.guard.user_id.to_string())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    sqlx::query(
        "UPDATE recovery_code_sets SET status='replaced',replaced_at_ms=? WHERE user_id=? AND status='active' AND created_at_ms<=?",
    )
    .bind(disable.guard.now_ms)
    .bind(disable.guard.user_id.to_string())
    .bind(disable.guard.now_ms)
    .execute(&mut *transaction)
    .await?;
    let auth_revision = advance_auth_state_sqlite(&mut transaction, &disable.guard, false).await?;
    transaction.commit().await?;
    Ok(DisableTotpCredentialOutcome::Disabled {
        disabled_credentials,
        auth_revision,
    })
}

async fn disable_postgres(
    pool: &PgPool,
    disable: &DisableTotpCredential,
) -> Result<DisableTotpCredentialOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    lock_guard_postgres(&mut transaction, &disable.guard).await?;
    let target_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM totp_credentials WHERE id=$1 AND user_id=$2 AND status IN ('pending','active') AND revision=$3)",
    )
    .bind(disable.credential_id.into_uuid())
    .bind(disable.guard.user_id.into_uuid())
    .bind(database_revision(disable.expected_credential_revision)?)
    .fetch_one(&mut *transaction)
    .await?;
    if !target_exists {
        transaction.rollback().await?;
        return Ok(DisableTotpCredentialOutcome::Stale);
    }
    sqlx::query(
        "UPDATE secret_records SET deleted_at_ms=$1,revision=revision+1 WHERE id IN (SELECT secret_record_id FROM totp_credentials WHERE user_id=$2 AND status IN ('pending','active')) AND deleted_at_ms IS NULL",
    )
    .bind(disable.guard.now_ms)
    .bind(disable.guard.user_id.into_uuid())
    .execute(&mut *transaction)
    .await?;
    let disabled_credentials = sqlx::query(
        "UPDATE totp_credentials SET status='disabled',pending_expires_at_ms=NULL,disabled_at_ms=$1,revision=revision+1 WHERE user_id=$2 AND status IN ('pending','active')",
    )
    .bind(disable.guard.now_ms)
    .bind(disable.guard.user_id.into_uuid())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    sqlx::query(
        "UPDATE recovery_code_sets SET status='replaced',replaced_at_ms=$1 WHERE user_id=$2 AND status='active' AND created_at_ms<=$1",
    )
    .bind(disable.guard.now_ms)
    .bind(disable.guard.user_id.into_uuid())
    .execute(&mut *transaction)
    .await?;
    let auth_revision =
        advance_auth_state_postgres(&mut transaction, &disable.guard, false).await?;
    transaction.commit().await?;
    Ok(DisableTotpCredentialOutcome::Disabled {
        disabled_credentials,
        auth_revision,
    })
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, time::Duration};

    use nodecontroll_domain::{EntityId, Revision, TotpCredentialStatus};
    use nodecontroll_secrets::{
        EnvelopeCipher, Keyring, SecretBinding, SecretOwnerKind, SecretPurpose,
        TOTP_SEED_SCHEMA_VERSION, TotpSeed,
    };
    use sqlx::{PgPool, Row, postgres::PgConnectOptions, postgres::PgPoolOptions};

    use super::{
        ActivateTotpCredential, ActivateTotpCredentialOutcome, BeginTotpEnrollmentOutcome,
        Database, DisableTotpCredential, DisableTotpCredentialOutcome, NewRecoveryCodeSet,
        NewSecretRecord, NewTotpEnrollment, PersistenceError, StoredSecretRecord, TotpSessionGuard,
        TotpStepAdvance, TotpStepAdvanceOutcome, insert_recovery_code_set_postgres,
        insert_recovery_code_set_sqlite,
    };
    use crate::{ConnectionSettings, NewRecoveryCode};

    const KEY: &str = "5c7c2563b4609f964f83ecf3c874f545698b8e360bbca06316547d2af8928f62";

    fn settings() -> ConnectionSettings {
        ConnectionSettings {
            max_connections: 4,
            acquire_timeout: Duration::from_secs(5),
            statement_timeout: Duration::from_secs(30),
            lock_timeout: Duration::from_secs(5),
        }
    }

    fn keyring() -> Keyring {
        let cipher = EnvelopeCipher::from_hex(KEY, 1);
        assert!(cipher.is_ok());
        let Ok(cipher) = cipher else {
            panic!("fixture key must be valid");
        };
        let keyring = Keyring::from_ciphers(cipher, Vec::new());
        assert!(keyring.is_ok());
        let Ok(keyring) = keyring else {
            panic!("fixture keyring must be valid");
        };
        keyring
    }

    async fn insert_principal(
        database: &Database,
        user_id: EntityId,
        session_id: EntityId,
        recent_auth_at_ms: i64,
    ) {
        match database {
            Database::Sqlite(pool) => {
                let inserted = sqlx::query(
                    "INSERT INTO users (id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms) VALUES (?,?,?,'$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA','owner','active','totp-fixture',0,0,100,NULL)",
                )
                .bind(user_id.to_string())
                .bind(format!("u-{}", &user_id.to_string()[..8]))
                .bind(format!("u-{}", &user_id.to_string()[..8]))
                .execute(pool)
                .await;
                assert!(inserted.is_ok());
                assert!(
                    sqlx::query(
                        "INSERT INTO user_auth_state (user_id,auth_revision,password_changed_at_ms,updated_at_ms) VALUES (?,0,100,100)",
                    )
                    .bind(user_id.to_string())
                    .execute(pool)
                    .await
                    .is_ok()
                );
                assert!(
                    sqlx::query(
                        "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES (?,?,1,?,1,?,0,'password','active',100,100,?,?,2000000,3000000,NULL,NULL,NULL,NULL,NULL,0)",
                    )
                    .bind(session_id.to_string())
                    .bind(user_id.to_string())
                    .bind([1_u8; 32].as_slice())
                    .bind([2_u8; 32].as_slice())
                    .bind(recent_auth_at_ms)
                    .bind(recent_auth_at_ms)
                    .execute(pool)
                    .await
                    .is_ok()
                );
            }
            Database::Postgres(pool) => {
                let inserted = sqlx::query(
                    "INSERT INTO users (id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms) VALUES ($1,$2,$2,'$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA','owner','active','totp-fixture',false,0,100,NULL)",
                )
                .bind(user_id.into_uuid())
                .bind(format!("u-{}", &user_id.to_string()[..8]))
                .execute(pool)
                .await;
                assert!(inserted.is_ok());
                assert!(
                    sqlx::query(
                        "INSERT INTO user_auth_state (user_id,auth_revision,password_changed_at_ms,updated_at_ms) VALUES ($1,0,100,100)",
                    )
                    .bind(user_id.into_uuid())
                    .execute(pool)
                    .await
                    .is_ok()
                );
                assert!(
                    sqlx::query(
                        "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES ($1,$2,1,$3,1,$4,0,'password','active',100,100,$5,$5,2000000,3000000,NULL,NULL,NULL,NULL,NULL,0)",
                    )
                    .bind(session_id.into_uuid())
                    .bind(user_id.into_uuid())
                    .bind([1_u8; 32].as_slice())
                    .bind([2_u8; 32].as_slice())
                    .bind(recent_auth_at_ms)
                    .execute(pool)
                    .await
                    .is_ok()
                );
            }
        }
    }

    fn guard(
        user_id: EntityId,
        session_id: EntityId,
        auth_revision: u64,
        recent_auth_at_ms: i64,
        now_ms: i64,
    ) -> TotpSessionGuard {
        TotpSessionGuard {
            user_id,
            actor_session_id: session_id,
            expected_user_revision: Revision::initial(),
            expected_auth_revision: Revision::from_value(auth_revision),
            expected_recent_auth_at_ms: recent_auth_at_ms,
            now_ms,
        }
    }

    fn enrollment(
        keyring: &Keyring,
        user_id: EntityId,
        session_id: EntityId,
        now_ms: i64,
        marker: u8,
    ) -> NewTotpEnrollment {
        let seed = TotpSeed::from_bytes(&[marker; 20]);
        assert!(seed.is_ok());
        let Ok(seed) = seed else {
            panic!("fixture seed must be valid");
        };
        let envelope = keyring.encrypt_totp_seed(user_id.into_uuid(), &seed);
        assert!(envelope.is_ok());
        let Ok(envelope) = envelope else {
            panic!("fixture seed encryption must succeed");
        };
        let binding = SecretBinding::new(
            SecretPurpose::TotpSeed,
            SecretOwnerKind::User,
            user_id.into_uuid(),
            TOTP_SEED_SCHEMA_VERSION,
        );
        assert!(binding.is_ok());
        let Ok(binding) = binding else {
            panic!("fixture binding must be valid");
        };
        NewTotpEnrollment {
            credential_id: EntityId::new(),
            secret: NewSecretRecord {
                id: EntityId::new(),
                binding,
                envelope,
                created_at_ms: now_ms,
                rotated_from: None,
            },
            guard: guard(user_id, session_id, 0, now_ms - 1_000, now_ms),
            pending_expires_at_ms: now_ms + 600_000,
        }
    }

    fn recovery_set(created_at_ms: i64, marker: u8) -> NewRecoveryCodeSet {
        NewRecoveryCodeSet {
            created_at_ms,
            codes: (0_u8..8)
                .map(|index| NewRecoveryCode {
                    id: EntityId::new(),
                    digest_key_version: 1,
                    code_hmac: [marker.wrapping_add(index); 32],
                })
                .collect(),
        }
    }

    async fn insert_recovery_set(
        database: &Database,
        user_id: EntityId,
        version: i64,
        set: &NewRecoveryCodeSet,
    ) {
        match database {
            Database::Sqlite(pool) => {
                let transaction = pool.begin().await;
                assert!(transaction.is_ok());
                if let Ok(mut transaction) = transaction {
                    assert!(
                        insert_recovery_code_set_sqlite(&mut transaction, user_id, version, set,)
                            .await
                            .is_ok()
                    );
                    assert!(transaction.commit().await.is_ok());
                }
            }
            Database::Postgres(pool) => {
                let transaction = pool.begin().await;
                assert!(transaction.is_ok());
                if let Ok(mut transaction) = transaction {
                    assert!(
                        insert_recovery_code_set_postgres(&mut transaction, user_id, version, set,)
                            .await
                            .is_ok()
                    );
                    assert!(transaction.commit().await.is_ok());
                }
            }
        }
    }

    async fn auth_revision(database: &Database, user_id: EntityId) -> i64 {
        match database {
            Database::Sqlite(pool) => {
                sqlx::query_scalar("SELECT auth_revision FROM user_auth_state WHERE user_id=?")
                    .bind(user_id.to_string())
                    .fetch_one(pool)
                    .await
                    .unwrap_or(-1)
            }
            Database::Postgres(pool) => {
                sqlx::query_scalar("SELECT auth_revision FROM user_auth_state WHERE user_id=$1")
                    .bind(user_id.into_uuid())
                    .fetch_one(pool)
                    .await
                    .unwrap_or(-1)
            }
        }
    }

    async fn active_recovery_version(database: &Database, user_id: EntityId) -> Option<i64> {
        match database {
            Database::Sqlite(pool) => sqlx::query_scalar(
                "SELECT set_version FROM recovery_code_sets WHERE user_id=? AND status='active'",
            )
            .bind(user_id.to_string())
            .fetch_optional(pool)
            .await
            .ok()
            .flatten(),
            Database::Postgres(pool) => sqlx::query_scalar(
                "SELECT set_version FROM recovery_code_sets WHERE user_id=$1 AND status='active'",
            )
            .bind(user_id.into_uuid())
            .fetch_optional(pool)
            .await
            .ok()
            .flatten(),
        }
    }

    async fn secret_is_deleted(database: &Database, secret_id: EntityId) -> bool {
        match database {
            Database::Sqlite(pool) => sqlx::query_scalar::<_, bool>(
                "SELECT deleted_at_ms IS NOT NULL FROM secret_records WHERE id=?",
            )
            .bind(secret_id.to_string())
            .fetch_one(pool)
            .await
            .unwrap_or(true),
            Database::Postgres(pool) => sqlx::query_scalar::<_, bool>(
                "SELECT deleted_at_ms IS NOT NULL FROM secret_records WHERE id=$1",
            )
            .bind(secret_id.into_uuid())
            .fetch_one(pool)
            .await
            .unwrap_or(true),
        }
    }

    async fn set_secret_deleted_at(
        database: &Database,
        secret_id: EntityId,
        deleted_at_ms: Option<i64>,
    ) {
        let result: Result<u64, sqlx::Error> = match database {
            Database::Sqlite(pool) => {
                sqlx::query("UPDATE secret_records SET deleted_at_ms=? WHERE id=?")
                    .bind(deleted_at_ms)
                    .bind(secret_id.to_string())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
            Database::Postgres(pool) => {
                sqlx::query("UPDATE secret_records SET deleted_at_ms=$1 WHERE id=$2")
                    .bind(deleted_at_ms)
                    .bind(secret_id.into_uuid())
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected())
            }
        };
        assert_eq!(result, Ok(1));
    }

    async fn set_session_revoked(
        database: &Database,
        session_id: EntityId,
        revoked: bool,
        now_ms: i64,
    ) {
        let result: Result<u64, sqlx::Error> = match database {
            Database::Sqlite(pool) if revoked => sqlx::query(
                "UPDATE auth_sessions SET status='revoked',revoked_at_ms=?,revoked_reason='security_policy',revision=revision+1 WHERE id=?",
            )
            .bind(now_ms)
            .bind(session_id.to_string())
            .execute(pool)
            .await
            .map(|result| result.rows_affected()),
            Database::Sqlite(pool) => sqlx::query(
                "UPDATE auth_sessions SET status='active',revoked_at_ms=NULL,revoked_reason=NULL,revision=revision+1 WHERE id=?",
            )
            .bind(session_id.to_string())
            .execute(pool)
            .await
            .map(|result| result.rows_affected()),
            Database::Postgres(pool) if revoked => sqlx::query(
                "UPDATE auth_sessions SET status='revoked',revoked_at_ms=$1,revoked_reason='security_policy',revision=revision+1 WHERE id=$2",
            )
            .bind(now_ms)
            .bind(session_id.into_uuid())
            .execute(pool)
            .await
            .map(|result| result.rows_affected()),
            Database::Postgres(pool) => sqlx::query(
                "UPDATE auth_sessions SET status='active',revoked_at_ms=NULL,revoked_reason=NULL,revision=revision+1 WHERE id=$1",
            )
            .bind(session_id.into_uuid())
            .execute(pool)
            .await
            .map(|result| result.rows_affected()),
        };
        assert_eq!(result, Ok(1));
    }

    async fn schema_invariant_contract(database: &Database) {
        match database {
            Database::Sqlite(pool) => {
                let sql: String = sqlx::query_scalar(
                    "SELECT sql FROM sqlite_master WHERE type='table' AND name='totp_credentials'",
                )
                .fetch_one(pool)
                .await
                .unwrap_or_default();
                assert!(sql.contains("typeof(id) = 'text'"));
                assert!(sql.contains("typeof(last_accepted_step) = 'integer'"));
                let columns = sqlx::query("PRAGMA table_info(totp_credentials)")
                    .fetch_all(pool)
                    .await
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|row| row.try_get::<String, _>("name").ok())
                    .collect::<Vec<_>>();
                assert!(!columns.iter().any(|name| {
                    name == "seed" || name == "recovery_code" || name == "recovery_codes"
                }));
            }
            Database::Postgres(pool) => {
                let constraints = sqlx::query_scalar::<_, String>(
                    "SELECT pg_get_constraintdef(oid) FROM pg_constraint WHERE conrelid='totp_credentials'::regclass ORDER BY conname",
                )
                .fetch_all(pool)
                .await
                .unwrap_or_default()
                .join(" ");
                assert!(constraints.contains("sha1"));
                assert!(constraints.contains("digits"));
                assert!(constraints.contains("period_seconds"));
                let columns = sqlx::query_scalar::<_, String>(
                    "SELECT column_name FROM information_schema.columns WHERE table_schema=current_schema() AND table_name='totp_credentials'",
                )
                .fetch_all(pool)
                .await
                .unwrap_or_default();
                assert!(!columns.iter().any(|name| {
                    name == "seed" || name == "recovery_code" || name == "recovery_codes"
                }));
            }
        }
    }

    async fn repository_contract(database: Database) {
        assert!(database.migrate().await.is_ok());
        schema_invariant_contract(&database).await;
        let user_id = EntityId::new();
        let session_id = EntityId::new();
        let enrollment_at_ms = 300_000_i64;
        insert_principal(&database, user_id, session_id, enrollment_at_ms - 1_000).await;
        let keyring = keyring();
        let first = enrollment(&keyring, user_id, session_id, enrollment_at_ms, 1);
        let second = enrollment(&keyring, user_id, session_id, enrollment_at_ms, 2);
        let mut invalid_rotation_enrollment = first.clone();
        invalid_rotation_enrollment.secret.rotated_from = Some(EntityId::new());
        assert!(matches!(
            database
                .begin_totp_enrollment(&invalid_rotation_enrollment)
                .await,
            Err(PersistenceError::InvalidTotpCredential)
        ));

        // Generic secret helpers intentionally model one live row per binding. TOTP permits an
        // active seed and a pending replacement, so every TOTP access must carry a credential ID.
        // Keep this boundary fail-closed in both database implementations so future key rotation
        // cannot silently select an arbitrary seed.
        assert!(matches!(
            database.ensure_secret_record(&first.secret).await,
            Err(PersistenceError::InvalidSecretRecord)
        ));
        assert!(matches!(
            database.active_secret_record(first.secret.binding).await,
            Err(PersistenceError::InvalidSecretRecord)
        ));
        let expected_secret = StoredSecretRecord {
            id: first.secret.id,
            binding: first.secret.binding,
            envelope: first.secret.envelope.clone(),
            created_at_ms: first.secret.created_at_ms,
            rotated_from: first.secret.rotated_from,
            revision: Revision::initial(),
        };
        let mut replacement_secret = second.secret.clone();
        replacement_secret.rotated_from = Some(expected_secret.id);
        replacement_secret.created_at_ms = enrollment_at_ms + 1;
        assert!(matches!(
            database
                .rotate_secret_record(
                    &expected_secret,
                    &replacement_secret,
                    replacement_secret.created_at_ms,
                )
                .await,
            Err(PersistenceError::InvalidSecretRecord)
        ));

        let (first_result, second_result) = tokio::join!(
            database.begin_totp_enrollment(&first),
            database.begin_totp_enrollment(&second),
        );
        let created = match (first_result, second_result) {
            (
                Ok(BeginTotpEnrollmentOutcome::Created(created)),
                Ok(BeginTotpEnrollmentOutcome::AlreadyPending),
            )
            | (
                Ok(BeginTotpEnrollmentOutcome::AlreadyPending),
                Ok(BeginTotpEnrollmentOutcome::Created(created)),
            ) => created,
            _ => panic!("exactly one concurrent enrollment may create the pending credential"),
        };
        assert_eq!(created.credential.status, TotpCredentialStatus::Pending);
        assert!(
            database
                .active_totp_credential(user_id, enrollment_at_ms)
                .await
                .is_ok_and(|value| value.is_none())
        );

        let original_recovery = recovery_set(enrollment_at_ms + 1, 10);
        insert_recovery_set(&database, user_id, 1, &original_recovery).await;
        let activation_at_ms = enrollment_at_ms + 2;
        let mut colliding_recovery = recovery_set(activation_at_ms, 30);
        colliding_recovery.codes[0].code_hmac = original_recovery.codes[0].code_hmac;
        let failed = database
            .activate_totp_credential(ActivateTotpCredential {
                credential_id: created.credential.id,
                expected_credential_revision: Revision::initial(),
                accepted_step: u64::try_from(activation_at_ms / 30_000).unwrap_or_default(),
                guard: guard(
                    user_id,
                    session_id,
                    0,
                    enrollment_at_ms - 1_000,
                    activation_at_ms,
                ),
                recovery_codes: &colliding_recovery,
            })
            .await;
        assert!(matches!(failed, Err(PersistenceError::Sql(_))));
        assert_eq!(auth_revision(&database, user_id).await, 0);
        assert_eq!(active_recovery_version(&database, user_id).await, Some(1));
        assert!(
            database
                .pending_totp_credential(user_id, activation_at_ms)
                .await
                .is_ok_and(|value| value.is_some())
        );

        let activation_at_ms = enrollment_at_ms + 3;
        let accepted_step = u64::try_from(activation_at_ms / 30_000).unwrap_or_default();
        let replacement = recovery_set(activation_at_ms, 50);
        let activated = database
            .activate_totp_credential(ActivateTotpCredential {
                credential_id: created.credential.id,
                expected_credential_revision: Revision::initial(),
                accepted_step,
                guard: guard(
                    user_id,
                    session_id,
                    0,
                    enrollment_at_ms - 1_000,
                    activation_at_ms,
                ),
                recovery_codes: &replacement,
            })
            .await;
        let activated = match activated {
            Ok(ActivateTotpCredentialOutcome::Activated(activated)) => activated,
            _ => panic!("valid first code must atomically activate TOTP and recovery codes"),
        };
        assert_eq!(activated.auth_revision, Revision::from_value(1));
        assert_eq!(activated.recovery_codes.set_version, 2);
        assert_eq!(active_recovery_version(&database, user_id).await, Some(2));
        let old_active_id = activated.credential.id;
        let old_active_secret_id = activated.credential.secret_record_id;

        let reenrollment_at_ms = 400_000;
        let mut reenrollment = enrollment(&keyring, user_id, session_id, reenrollment_at_ms, 3);
        reenrollment.guard = guard(user_id, session_id, 1, activation_at_ms, reenrollment_at_ms);
        let reenrolled = database.begin_totp_enrollment(&reenrollment).await;
        let reenrolled = match reenrolled {
            Ok(BeginTotpEnrollmentOutcome::Created(created)) => created,
            _ => panic!("one replacement enrollment must be created"),
        };
        assert!(
            database
                .active_totp_credential(user_id, reenrollment_at_ms)
                .await
                .is_ok_and(|value| value.is_some_and(|stored| {
                    stored.credential.id == old_active_id
                        && stored.credential.secret_record_id == old_active_secret_id
                }))
        );
        assert!(!secret_is_deleted(&database, old_active_secret_id).await);

        let failed_reenrollment_at_ms = reenrollment_at_ms + 1;
        let mut colliding_reenrollment_recovery = recovery_set(failed_reenrollment_at_ms, 70);
        colliding_reenrollment_recovery.codes[0].code_hmac = replacement.codes[0].code_hmac;
        let failed_reenrollment = database
            .activate_totp_credential(ActivateTotpCredential {
                credential_id: reenrolled.credential.id,
                expected_credential_revision: Revision::initial(),
                accepted_step: u64::try_from(failed_reenrollment_at_ms / 30_000)
                    .unwrap_or_default(),
                guard: guard(
                    user_id,
                    session_id,
                    1,
                    activation_at_ms,
                    failed_reenrollment_at_ms,
                ),
                recovery_codes: &colliding_reenrollment_recovery,
            })
            .await;
        assert!(matches!(failed_reenrollment, Err(PersistenceError::Sql(_))));
        assert!(!secret_is_deleted(&database, old_active_secret_id).await);
        assert!(
            database
                .active_totp_credential(user_id, failed_reenrollment_at_ms)
                .await
                .is_ok_and(
                    |value| value.is_some_and(|stored| stored.credential.id == old_active_id)
                )
        );

        let expired_at_ms = reenrollment.pending_expires_at_ms + 1;
        assert!(
            database
                .active_totp_credential(user_id, expired_at_ms)
                .await
                .is_ok_and(
                    |value| value.is_some_and(|stored| stored.credential.id == old_active_id)
                )
        );
        let mut replacement_enrollment =
            enrollment(&keyring, user_id, session_id, expired_at_ms, 4);
        replacement_enrollment.guard =
            guard(user_id, session_id, 1, activation_at_ms, expired_at_ms);
        let replacement_pending = database
            .begin_totp_enrollment(&replacement_enrollment)
            .await;
        let replacement_pending = match replacement_pending {
            Ok(BeginTotpEnrollmentOutcome::Created(created)) => created,
            _ => panic!("expired pending enrollment must be safely replaceable"),
        };
        assert!(secret_is_deleted(&database, reenrolled.credential.secret_record_id).await);
        assert!(!secret_is_deleted(&database, old_active_secret_id).await);

        let replacement_activation_at_ms = expired_at_ms + 2;
        let replacement_step =
            u64::try_from(replacement_activation_at_ms / 30_000).unwrap_or_default();
        let second_recovery = recovery_set(replacement_activation_at_ms, 90);
        let replacement_activated = database
            .activate_totp_credential(ActivateTotpCredential {
                credential_id: replacement_pending.credential.id,
                expected_credential_revision: Revision::initial(),
                accepted_step: replacement_step,
                guard: guard(
                    user_id,
                    session_id,
                    1,
                    activation_at_ms,
                    replacement_activation_at_ms,
                ),
                recovery_codes: &second_recovery,
            })
            .await;
        let replacement_activated = match replacement_activated {
            Ok(ActivateTotpCredentialOutcome::Activated(activated)) => activated,
            _ => panic!("successful replacement must atomically swap the active seed"),
        };
        assert_eq!(replacement_activated.auth_revision, Revision::from_value(2));
        assert_eq!(replacement_activated.recovery_codes.set_version, 3);
        assert!(secret_is_deleted(&database, old_active_secret_id).await);
        assert!(
            database
                .active_totp_credential(user_id, replacement_activation_at_ms)
                .await
                .is_ok_and(|value| value.is_some_and(|stored| {
                    stored.credential.id == replacement_activated.credential.id
                        && stored.credential.secret_record_id
                            == replacement_activated.credential.secret_record_id
                }))
        );

        // A credential row never makes a tombstoned envelope readable. The raw mutation models
        // corruption or an interrupted future rewrap and is immediately restored for replay tests.
        let replacement_secret_id = replacement_activated.credential.secret_record_id;
        set_secret_deleted_at(
            &database,
            replacement_secret_id,
            Some(replacement_activation_at_ms + 1),
        )
        .await;
        assert!(
            database
                .active_totp_credential(user_id, replacement_activation_at_ms + 1)
                .await
                .is_ok_and(|value| value.is_none())
        );
        set_secret_deleted_at(&database, replacement_secret_id, None).await;
        assert!(
            database
                .active_totp_credential(user_id, replacement_activation_at_ms + 1)
                .await
                .is_ok_and(|value| value.is_some())
        );

        // Reservation occurs in step 35 and commit crosses into step 36. Step 34 is accepted
        // because the proof window is bound to the durable reservation, not commit time.
        let verification_time_ms = 1_079_999;
        let replay_at_ms = 1_080_001;
        let next_step = 34;
        let advance = TotpStepAdvance {
            credential_id: replacement_activated.credential.id,
            user_id,
            expected_credential_revision: replacement_activated.credential.revision,
            expected_last_accepted_step: Some(replacement_step),
            accepted_step: next_step,
            expected_auth_revision: Revision::from_value(2),
            session_id: Some(session_id),
            verification_time_ms,
            now_ms: replay_at_ms,
        };
        let stale_auth_revision = TotpStepAdvance {
            expected_auth_revision: Revision::from_value(1),
            ..advance
        };
        assert!(matches!(
            database.advance_totp_step(&stale_auth_revision).await,
            Ok(TotpStepAdvanceOutcome::Stale)
        ));
        let expired_session = TotpStepAdvance {
            now_ms: 2_000_000,
            ..advance
        };
        assert!(matches!(
            database.advance_totp_step(&expired_session).await,
            Ok(TotpStepAdvanceOutcome::Stale)
        ));
        set_session_revoked(&database, session_id, true, replay_at_ms).await;
        assert!(matches!(
            database.advance_totp_step(&advance).await,
            Ok(TotpStepAdvanceOutcome::Stale)
        ));
        set_session_revoked(&database, session_id, false, replay_at_ms).await;
        let (first_advance, second_advance) = tokio::join!(
            database.advance_totp_step(&advance),
            database.advance_totp_step(&advance),
        );
        assert!(matches!(
            (first_advance, second_advance),
            (
                Ok(TotpStepAdvanceOutcome::Advanced(_)),
                Ok(TotpStepAdvanceOutcome::Stale)
            ) | (
                Ok(TotpStepAdvanceOutcome::Stale),
                Ok(TotpStepAdvanceOutcome::Advanced(_))
            )
        ));
        assert!(matches!(
            database.advance_totp_step(&advance).await,
            Ok(TotpStepAdvanceOutcome::Stale)
        ));

        let stale_recent = database
            .disable_totp_credential(&DisableTotpCredential {
                credential_id: replacement_activated.credential.id,
                expected_credential_revision: Revision::from_value(2),
                guard: guard(
                    user_id,
                    session_id,
                    2,
                    replacement_activation_at_ms - 1,
                    replay_at_ms + 1,
                ),
            })
            .await;
        assert!(matches!(
            stale_recent,
            Err(PersistenceError::SessionPrincipalUnavailable)
        ));
        let disabled = database
            .disable_totp_credential(&DisableTotpCredential {
                credential_id: replacement_activated.credential.id,
                expected_credential_revision: Revision::from_value(2),
                guard: guard(
                    user_id,
                    session_id,
                    2,
                    replacement_activation_at_ms,
                    replay_at_ms + 1,
                ),
            })
            .await;
        assert!(matches!(
            disabled,
            Ok(DisableTotpCredentialOutcome::Disabled {
                disabled_credentials: 1,
                auth_revision: Revision::from_value(3),
            })
        ));
        assert_eq!(active_recovery_version(&database, user_id).await, None);
        assert!(
            database
                .active_totp_credential(user_id, replay_at_ms + 1)
                .await
                .is_ok_and(|value| value.is_none())
        );
    }

    #[tokio::test]
    async fn sqlite_totp_repository_contract() {
        let database = Database::connect("sqlite::memory:", settings()).await;
        assert!(database.is_ok());
        if let Ok(database) = database {
            repository_contract(database).await;
        }
    }

    struct PostgresFixture {
        database: Database,
        admin: PgPool,
    }

    async fn postgres_fixture(url: &str) -> Result<PostgresFixture, sqlx::Error> {
        let admin = PgPoolOptions::new().max_connections(1).connect(url).await?;
        sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_totp_c4 CASCADE")
            .execute(&admin)
            .await?;
        sqlx::query("CREATE SCHEMA nodecontroll_test_totp_c4")
            .execute(&admin)
            .await?;
        let options = PgConnectOptions::from_str(url)?.options([
            ("search_path", "nodecontroll_test_totp_c4"),
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
        })
    }

    #[tokio::test]
    async fn postgres_totp_repository_contract() {
        let url = match std::env::var("NODECONTROLL_TEST_POSTGRES_URL") {
            Ok(url) => url,
            Err(_) => panic!("NODECONTROLL_TEST_POSTGRES_URL is required for the persistence gate"),
        };
        let fixture = postgres_fixture(&url).await;
        assert!(fixture.is_ok());
        if let Ok(fixture) = fixture {
            repository_contract(fixture.database.clone()).await;
            if let Database::Postgres(pool) = &fixture.database {
                pool.close().await;
            }
            assert!(
                sqlx::query("DROP SCHEMA nodecontroll_test_totp_c4 CASCADE")
                    .execute(&fixture.admin)
                    .await
                    .is_ok()
            );
            fixture.admin.close().await;
        }
    }
}
