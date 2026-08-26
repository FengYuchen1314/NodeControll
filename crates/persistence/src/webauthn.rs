use nodecontroll_domain::{
    AuthChallengePurpose, EntityId, Revision, WebAuthnAaguid, WebAuthnCredential,
    WebAuthnCredentialId, WebAuthnCredentialStatus, WebAuthnNickname, WebAuthnOrigin,
    WebAuthnTransport, WebAuthnUserHandle,
};
use nodecontroll_secrets::SecretEnvelope;
use sqlx::{FromRow, PgPool, Row, SqlitePool};

use super::{
    AuthChallengeClientContext, Database, PersistenceError, database_key_version,
    database_revision, decode_revision, validate_non_negative_timestamp,
};

const MAX_WEBAUTHN_ENCRYPTED_BYTES: usize = 262_144;

#[derive(FromRow)]
struct WebAuthnGuardSnapshot {
    user_revision: i64,
    auth_revision: i64,
    session_auth_revision: i64,
    force_password_change: bool,
    session_status: String,
    revoked_at_ms: Option<i64>,
    recent_auth_at_ms: i64,
    last_seen_at_ms: i64,
    idle_expires_at_ms: i64,
    absolute_expires_at_ms: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WebAuthnSessionGuard {
    pub user_id: EntityId,
    pub actor_session_id: EntityId,
    pub expected_user_revision: Revision,
    pub expected_auth_revision: Revision,
    pub expected_recent_auth_at_ms: i64,
    pub now_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebAuthnChallengeBinding {
    pub auth_challenge_id: EntityId,
    pub claim_id: EntityId,
    pub purpose: AuthChallengePurpose,
    pub user_id: EntityId,
    pub session_id: Option<EntityId>,
    pub auth_revision: Revision,
    pub reserved_at_ms: i64,
    pub verification_expires_at_ms: i64,
    pub client_context: AuthChallengeClientContext,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewWebAuthnRegistrationCeremony {
    pub id: EntityId,
    pub guard: WebAuthnSessionGuard,
    pub origin: WebAuthnOrigin,
    pub expires_at_ms: i64,
    pub state: SecretEnvelope,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredWebAuthnRegistrationCeremony {
    pub id: EntityId,
    pub user_id: EntityId,
    pub session_id: EntityId,
    pub origin: WebAuthnOrigin,
    pub user_revision: Revision,
    pub auth_revision: Revision,
    pub recent_auth_at_ms: i64,
    pub state: SecretEnvelope,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
    pub revision: Revision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BeginWebAuthnRegistrationOutcome {
    Created(StoredWebAuthnRegistrationCeremony),
    AlreadyPending,
    Stale,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewWebAuthnCredential {
    pub credential: WebAuthnCredential,
    pub material: SecretEnvelope,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredWebAuthnCredential {
    pub credential: WebAuthnCredential,
    pub material: SecretEnvelope,
}

#[derive(Clone)]
pub struct CompleteWebAuthnRegistration<'a> {
    pub ceremony_id: EntityId,
    pub expected_ceremony_revision: Revision,
    pub guard: WebAuthnSessionGuard,
    pub origin: &'a WebAuthnOrigin,
    pub credential: &'a NewWebAuthnCredential,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebAuthnRegistrationResult {
    pub credential: WebAuthnCredential,
    pub auth_revision: Revision,
    pub revoked_other_sessions: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompleteWebAuthnRegistrationOutcome {
    Registered(WebAuthnRegistrationResult),
    DuplicateCredential,
    Stale,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewWebAuthnAuthenticationCeremony {
    pub id: EntityId,
    pub binding: WebAuthnChallengeBinding,
    pub origin: WebAuthnOrigin,
    pub state: SecretEnvelope,
    pub created_at_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredWebAuthnAuthenticationCeremony {
    pub id: EntityId,
    pub binding: WebAuthnChallengeBinding,
    pub origin: WebAuthnOrigin,
    pub state: SecretEnvelope,
    pub created_at_ms: i64,
    pub revision: Revision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BeginWebAuthnAuthenticationOutcome {
    Created(StoredWebAuthnAuthenticationCeremony),
    AlreadyPending,
    Stale,
}

#[derive(Clone)]
pub struct WebAuthnAuthenticationCommit<'a> {
    pub ceremony_id: EntityId,
    pub expected_ceremony_revision: Revision,
    pub binding: &'a WebAuthnChallengeBinding,
    pub origin: &'a WebAuthnOrigin,
    pub credential_id: EntityId,
    pub expected_credential_revision: Revision,
    pub expected_sign_counter: u32,
    pub expected_backup_eligible: bool,
    pub expected_backup_state: bool,
    /// Counter returned by the fully verified library result before monotonic persistence policy.
    pub observed_sign_counter: u32,
    pub sign_counter: u32,
    pub backup_eligible: bool,
    pub backup_state: bool,
    /// Records an accepted non-monotonic result from a backup-eligible synced passkey.
    pub backup_counter_anomaly: bool,
    pub material: &'a SecretEnvelope,
    pub now_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WebAuthnAuthenticationCommitOutcome {
    Committed(WebAuthnCredential),
    Stale,
}

/// Durable handoff left after the WebAuthn commit point and before C3 consumes or rejects the
/// verifier claim. The caller must first re-authorize the original C3 bearer and exact context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WebAuthnAuthenticationHandoff {
    Verified,
    Rejected,
}

#[derive(Clone)]
pub struct WebAuthnCloneSuspected<'a> {
    pub ceremony_id: EntityId,
    pub expected_ceremony_revision: Revision,
    pub binding: &'a WebAuthnChallengeBinding,
    pub origin: &'a WebAuthnOrigin,
    pub credential_id: EntityId,
    pub expected_credential_revision: Revision,
    pub expected_sign_counter: u32,
    pub now_ms: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WebAuthnCloneSuspectedOutcome {
    Recorded {
        auth_revision: Revision,
        revoked_sessions: u64,
    },
    Stale,
}

#[derive(Clone)]
pub struct RenameWebAuthnCredential<'a> {
    pub credential_id: EntityId,
    pub expected_credential_revision: Revision,
    pub nickname: &'a WebAuthnNickname,
    pub guard: WebAuthnSessionGuard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RevokeWebAuthnCredential {
    pub credential_id: EntityId,
    pub expected_credential_revision: Revision,
    pub guard: WebAuthnSessionGuard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RevokeWebAuthnCredentialOutcome {
    Revoked {
        auth_revision: Revision,
        revoked_other_sessions: u64,
    },
    Stale,
}

impl Database {
    pub async fn begin_webauthn_registration(
        &self,
        ceremony: &NewWebAuthnRegistrationCeremony,
    ) -> Result<BeginWebAuthnRegistrationOutcome, PersistenceError> {
        validate_new_registration(ceremony)?;
        match self {
            Self::Sqlite(pool) => begin_registration_sqlite(pool, ceremony).await,
            Self::Postgres(pool) => begin_registration_postgres(pool, ceremony).await,
        }
    }

    pub async fn webauthn_registration_ceremony(
        &self,
        ceremony_id: EntityId,
        expected_revision: Revision,
        guard: &WebAuthnSessionGuard,
        origin: &WebAuthnOrigin,
    ) -> Result<Option<StoredWebAuthnRegistrationCeremony>, PersistenceError> {
        validate_guard(guard)?;
        database_revision(expected_revision)?;
        match self {
            Self::Sqlite(pool) => {
                load_registration_sqlite(pool, ceremony_id, expected_revision, guard, origin).await
            }
            Self::Postgres(pool) => {
                load_registration_postgres(pool, ceremony_id, expected_revision, guard, origin)
                    .await
            }
        }
    }

    pub async fn complete_webauthn_registration(
        &self,
        command: &CompleteWebAuthnRegistration<'_>,
    ) -> Result<CompleteWebAuthnRegistrationOutcome, PersistenceError> {
        validate_complete_registration(command)?;
        match self {
            Self::Sqlite(pool) => complete_registration_sqlite(pool, command).await,
            Self::Postgres(pool) => complete_registration_postgres(pool, command).await,
        }
    }

    pub async fn reject_webauthn_registration(
        &self,
        ceremony_id: EntityId,
        expected_revision: Revision,
        guard: &WebAuthnSessionGuard,
        origin: &WebAuthnOrigin,
    ) -> Result<bool, PersistenceError> {
        validate_guard(guard)?;
        database_revision(expected_revision)?;
        match self {
            Self::Sqlite(pool) => {
                reject_registration_sqlite(pool, ceremony_id, expected_revision, guard, origin)
                    .await
            }
            Self::Postgres(pool) => {
                reject_registration_postgres(pool, ceremony_id, expected_revision, guard, origin)
                    .await
            }
        }
    }

    pub async fn active_webauthn_credentials_for_challenge(
        &self,
        binding: &WebAuthnChallengeBinding,
        now_ms: i64,
    ) -> Result<Vec<StoredWebAuthnCredential>, PersistenceError> {
        validate_challenge_binding(binding, now_ms)?;
        match self {
            Self::Sqlite(pool) => active_for_challenge_sqlite(pool, binding, now_ms).await,
            Self::Postgres(pool) => active_for_challenge_postgres(pool, binding, now_ms).await,
        }
    }

    pub async fn active_webauthn_credentials_for_registration(
        &self,
        guard: &WebAuthnSessionGuard,
    ) -> Result<Vec<StoredWebAuthnCredential>, PersistenceError> {
        validate_guard(guard)?;
        match self {
            Self::Sqlite(pool) => active_for_registration_sqlite(pool, guard).await,
            Self::Postgres(pool) => active_for_registration_postgres(pool, guard).await,
        }
    }

    pub async fn begin_webauthn_authentication(
        &self,
        ceremony: &NewWebAuthnAuthenticationCeremony,
    ) -> Result<BeginWebAuthnAuthenticationOutcome, PersistenceError> {
        validate_new_authentication(ceremony)?;
        match self {
            Self::Sqlite(pool) => begin_authentication_sqlite(pool, ceremony).await,
            Self::Postgres(pool) => begin_authentication_postgres(pool, ceremony).await,
        }
    }

    pub async fn webauthn_authentication_context(
        &self,
        ceremony_id: EntityId,
        expected_ceremony_revision: Revision,
        binding: &WebAuthnChallengeBinding,
        origin: &WebAuthnOrigin,
        credential_id: &WebAuthnCredentialId,
        now_ms: i64,
    ) -> Result<Option<(StoredWebAuthnAuthenticationCeremony, StoredWebAuthnCredential)>, PersistenceError>
    {
        validate_challenge_binding(binding, now_ms)?;
        database_revision(expected_ceremony_revision)?;
        match self {
            Self::Sqlite(pool) => {
                authentication_context_sqlite(
                    pool,
                    ceremony_id,
                    expected_ceremony_revision,
                    binding,
                    origin,
                    credential_id,
                    now_ms,
                )
                .await
            }
            Self::Postgres(pool) => {
                authentication_context_postgres(
                    pool,
                    ceremony_id,
                    expected_ceremony_revision,
                    binding,
                    origin,
                    credential_id,
                    now_ms,
                )
                .await
            }
        }
    }

    pub async fn commit_webauthn_authentication(
        &self,
        command: &WebAuthnAuthenticationCommit<'_>,
    ) -> Result<WebAuthnAuthenticationCommitOutcome, PersistenceError> {
        validate_authentication_commit(command)?;
        match self {
            Self::Sqlite(pool) => commit_authentication_sqlite(pool, command).await,
            Self::Postgres(pool) => commit_authentication_postgres(pool, command).await,
        }
    }

    pub async fn webauthn_authentication_handoff(
        &self,
        ceremony_id: EntityId,
        expected_ceremony_revision: Revision,
        binding: &WebAuthnChallengeBinding,
        origin: &WebAuthnOrigin,
        now_ms: i64,
    ) -> Result<Option<WebAuthnAuthenticationHandoff>, PersistenceError> {
        validate_terminal_handoff_binding(binding, now_ms)?;
        let terminal_revision = expected_ceremony_revision
            .next()
            .map_err(|_| PersistenceError::RevisionOutOfRange)?;
        match self {
            Self::Sqlite(pool) => {
                authentication_handoff_sqlite(
                    pool,
                    ceremony_id,
                    terminal_revision,
                    binding,
                    origin,
                    now_ms,
                )
                .await
            }
            Self::Postgres(pool) => {
                authentication_handoff_postgres(
                    pool,
                    ceremony_id,
                    terminal_revision,
                    binding,
                    origin,
                    now_ms,
                )
                .await
            }
        }
    }

    pub async fn reject_webauthn_authentication(
        &self,
        ceremony_id: EntityId,
        expected_ceremony_revision: Revision,
        binding: &WebAuthnChallengeBinding,
        origin: &WebAuthnOrigin,
        now_ms: i64,
    ) -> Result<bool, PersistenceError> {
        validate_challenge_binding(binding, now_ms)?;
        database_revision(expected_ceremony_revision)?;
        match self {
            Self::Sqlite(pool) => {
                reject_authentication_sqlite(
                    pool,
                    ceremony_id,
                    expected_ceremony_revision,
                    binding,
                    origin,
                    now_ms,
                )
                .await
            }
            Self::Postgres(pool) => {
                reject_authentication_postgres(
                    pool,
                    ceremony_id,
                    expected_ceremony_revision,
                    binding,
                    origin,
                    now_ms,
                )
                .await
            }
        }
    }

    pub async fn record_webauthn_clone_suspected(
        &self,
        command: &WebAuthnCloneSuspected<'_>,
    ) -> Result<WebAuthnCloneSuspectedOutcome, PersistenceError> {
        validate_clone_suspected(command)?;
        match self {
            Self::Sqlite(pool) => clone_suspected_sqlite(pool, command).await,
            Self::Postgres(pool) => clone_suspected_postgres(pool, command).await,
        }
    }

    pub async fn rename_webauthn_credential(
        &self,
        command: &RenameWebAuthnCredential<'_>,
    ) -> Result<Option<WebAuthnCredential>, PersistenceError> {
        validate_guard(&command.guard)?;
        database_revision(command.expected_credential_revision)?;
        match self {
            Self::Sqlite(pool) => rename_credential_sqlite(pool, command).await,
            Self::Postgres(pool) => rename_credential_postgres(pool, command).await,
        }
    }

    pub async fn revoke_webauthn_credential(
        &self,
        command: &RevokeWebAuthnCredential,
    ) -> Result<RevokeWebAuthnCredentialOutcome, PersistenceError> {
        validate_guard(&command.guard)?;
        database_revision(command.expected_credential_revision)?;
        match self {
            Self::Sqlite(pool) => revoke_credential_sqlite(pool, command).await,
            Self::Postgres(pool) => revoke_credential_postgres(pool, command).await,
        }
    }
}

fn validate_guard(guard: &WebAuthnSessionGuard) -> Result<(), PersistenceError> {
    validate_non_negative_timestamp(guard.now_ms)?;
    validate_non_negative_timestamp(guard.expected_recent_auth_at_ms)?;
    database_revision(guard.expected_user_revision)?;
    database_revision(guard.expected_auth_revision)?;
    if guard.expected_recent_auth_at_ms > guard.now_ms {
        return Err(PersistenceError::InvalidWebAuthnCeremony);
    }
    Ok(())
}

fn validate_guard_snapshot(
    snapshot: Option<WebAuthnGuardSnapshot>,
    guard: &WebAuthnSessionGuard,
) -> Result<(), PersistenceError> {
    let Some(snapshot) = snapshot else {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    };
    if snapshot.user_revision != database_revision(guard.expected_user_revision)?
        || snapshot.auth_revision != database_revision(guard.expected_auth_revision)?
        || snapshot.session_auth_revision != database_revision(guard.expected_auth_revision)?
        || snapshot.force_password_change
        || snapshot.session_status != "active"
        || snapshot.revoked_at_ms.is_some()
        || snapshot.recent_auth_at_ms != guard.expected_recent_auth_at_ms
        || snapshot.last_seen_at_ms > guard.now_ms
        || snapshot.idle_expires_at_ms <= guard.now_ms
        || snapshot.absolute_expires_at_ms <= guard.now_ms
    {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    Ok(())
}

async fn lock_guard_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    guard: &WebAuthnSessionGuard,
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
    let snapshot = sqlx::query_as(
        "SELECT u.revision AS user_revision,uas.auth_revision AS auth_revision,s.auth_revision AS session_auth_revision,u.force_password_change,s.status AS session_status,s.revoked_at_ms,s.recent_auth_at_ms,s.last_seen_at_ms,s.idle_expires_at_ms,s.absolute_expires_at_ms FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=? AND s.id=? AND u.status='active' AND u.deleted_at_ms IS NULL",
    )
    .bind(guard.user_id.to_string())
    .bind(guard.actor_session_id.to_string())
    .fetch_optional(&mut **transaction)
    .await?;
    validate_guard_snapshot(snapshot, guard)
}

async fn lock_guard_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    guard: &WebAuthnSessionGuard,
) -> Result<(), PersistenceError> {
    let locked_auth_revision: Option<i64> = sqlx::query_scalar(
        "SELECT auth_revision FROM user_auth_state WHERE user_id=$1 AND auth_revision=$2 FOR UPDATE",
    )
    .bind(guard.user_id.into_uuid())
    .bind(database_revision(guard.expected_auth_revision)?)
    .fetch_optional(&mut **transaction)
    .await?;
    if locked_auth_revision.is_none() {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    let snapshot = sqlx::query_as(
        "SELECT u.revision AS user_revision,uas.auth_revision AS auth_revision,s.auth_revision AS session_auth_revision,u.force_password_change,s.status AS session_status,s.revoked_at_ms,s.recent_auth_at_ms,s.last_seen_at_ms,s.idle_expires_at_ms,s.absolute_expires_at_ms FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=$1 AND s.id=$2 AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=$3 FOR NO KEY UPDATE OF u,s",
    )
    .bind(guard.user_id.into_uuid())
    .bind(guard.actor_session_id.into_uuid())
    .bind(database_revision(guard.expected_auth_revision)?)
    .fetch_optional(&mut **transaction)
    .await?;
    validate_guard_snapshot(snapshot, guard)
}

/// Serializes every PostgreSQL WebAuthn mutation for one user on `user_auth_state` before it can
/// lock a C3 challenge. The user and optional bound session are then locked `FOR NO KEY UPDATE`:
/// this blocks status/deletion races but remains compatible with C3 challenge INSERT's foreign-key
/// `KEY SHARE`, preventing its challenge -> principal order from forming a deadlock cycle.
async fn lock_authentication_principal_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    binding: &WebAuthnChallengeBinding,
    now_ms: i64,
) -> Result<bool, PersistenceError> {
    let locked_auth_revision: Option<i64> = sqlx::query_scalar(
        "SELECT auth_revision FROM user_auth_state WHERE user_id=$1 AND auth_revision=$2 FOR UPDATE",
    )
    .bind(binding.user_id.into_uuid())
    .bind(database_revision(binding.auth_revision)?)
    .fetch_optional(&mut **transaction)
    .await?;
    if locked_auth_revision.is_none() {
        return Ok(false);
    }
    let principal_is_active: Option<i64> = if let Some(session_id) = binding.session_id {
        sqlx::query_scalar(
            "SELECT 1::BIGINT FROM users u JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=$1 AND s.id=$2 AND u.status='active' AND u.deleted_at_ms IS NULL AND s.status='active' AND s.auth_revision=$3 AND s.last_seen_at_ms<=$4 AND s.idle_expires_at_ms>$4 AND s.absolute_expires_at_ms>$4 FOR NO KEY UPDATE OF u,s",
        )
        .bind(binding.user_id.into_uuid())
        .bind(session_id.into_uuid())
        .bind(database_revision(binding.auth_revision)?)
        .bind(now_ms)
        .fetch_optional(&mut **transaction)
        .await?
    } else {
        sqlx::query_scalar(
            "SELECT 1::BIGINT FROM users u WHERE u.id=$1 AND u.status='active' AND u.deleted_at_ms IS NULL FOR NO KEY UPDATE OF u",
        )
        .bind(binding.user_id.into_uuid())
        .fetch_optional(&mut **transaction)
        .await?
    };
    Ok(principal_is_active.is_some())
}

/// Locks every open C3 challenge for one auth revision in canonical UUID order.
///
/// PostgreSQL may otherwise choose a different physical row order for a bulk `UPDATE` than the
/// ordered lock used by challenge refresh. Callers take this lock after principal/auth-state
/// locks and before any credential or ceremony lock.
async fn lock_open_user_challenges_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: EntityId,
    auth_revision: Revision,
) -> Result<(), PersistenceError> {
    let _: Vec<uuid::Uuid> = sqlx::query_scalar(
        "SELECT id FROM auth_challenges WHERE user_id=$1 AND auth_revision=$2 AND status IN ('pending','verification_pending','rotation_pending','exhausted') ORDER BY id FOR UPDATE",
    )
    .bind(user_id.into_uuid())
    .bind(database_revision(auth_revision)?)
    .fetch_all(&mut **transaction)
    .await?;
    Ok(())
}

async fn advance_management_auth_state_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    guard: &WebAuthnSessionGuard,
) -> Result<(Revision, u64), PersistenceError> {
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
    let revoked = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=?,revoked_reason='security_policy',revision=revision+1 WHERE user_id=? AND id<>? AND status='active' AND auth_revision=?",
    )
    .bind(guard.now_ms)
    .bind(guard.user_id.to_string())
    .bind(guard.actor_session_id.to_string())
    .bind(database_revision(guard.expected_auth_revision)?)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    let actor = sqlx::query(
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
    .rows_affected();
    if actor != 1 {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    Ok((next, revoked))
}

async fn advance_management_auth_state_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    guard: &WebAuthnSessionGuard,
) -> Result<(Revision, u64), PersistenceError> {
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
    let revoked = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=$1,revoked_reason='security_policy',revision=revision+1 WHERE user_id=$2 AND id<>$3 AND status='active' AND auth_revision=$4",
    )
    .bind(guard.now_ms)
    .bind(guard.user_id.into_uuid())
    .bind(guard.actor_session_id.into_uuid())
    .bind(database_revision(guard.expected_auth_revision)?)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    let actor = sqlx::query(
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
    .rows_affected();
    if actor != 1 {
        return Err(PersistenceError::SessionPrincipalUnavailable);
    }
    Ok((next, revoked))
}

fn validate_envelope(envelope: &SecretEnvelope) -> Result<(), PersistenceError> {
    database_key_version(envelope.key_version)?;
    if !(17..=MAX_WEBAUTHN_ENCRYPTED_BYTES).contains(&envelope.ciphertext.len()) {
        return Err(PersistenceError::InvalidWebAuthnCeremony);
    }
    Ok(())
}

fn validate_new_registration(
    ceremony: &NewWebAuthnRegistrationCeremony,
) -> Result<(), PersistenceError> {
    validate_guard(&ceremony.guard)?;
    validate_envelope(&ceremony.state)?;
    if ceremony.expires_at_ms <= ceremony.guard.now_ms {
        return Err(PersistenceError::InvalidWebAuthnCeremony);
    }
    Ok(())
}

fn validate_new_credential(credential: &NewWebAuthnCredential) -> Result<(), PersistenceError> {
    let value = &credential.credential;
    validate_envelope(&credential.material)
        .map_err(|_| PersistenceError::InvalidWebAuthnCredential)?;
    database_revision(value.revision)?;
    if value.status != WebAuthnCredentialStatus::Active
        || value.revision != Revision::initial()
        || !value.user_verified
        || value.user_handle != WebAuthnUserHandle::for_user(value.user_id)
        || (value.backup_state && !value.backup_eligible)
        || value.created_at_ms < 0
        || value.last_used_at_ms.is_some()
        || value.backup_counter_anomaly_at_ms.is_some()
        || value.revoked_at_ms.is_some()
        || value.clone_suspected_at_ms.is_some()
    {
        return Err(PersistenceError::InvalidWebAuthnCredential);
    }
    let mut transports = value.transports.clone();
    transports.sort_by_key(|transport| transport.as_str());
    transports.dedup();
    if transports.len() != value.transports.len() {
        return Err(PersistenceError::InvalidWebAuthnCredential);
    }
    Ok(())
}

fn validate_complete_registration(
    command: &CompleteWebAuthnRegistration<'_>,
) -> Result<(), PersistenceError> {
    validate_guard(&command.guard)?;
    database_revision(command.expected_ceremony_revision)?;
    validate_new_credential(command.credential)?;
    if command.credential.credential.user_id != command.guard.user_id
        || command.credential.credential.created_at_ms != command.guard.now_ms
    {
        return Err(PersistenceError::InvalidWebAuthnCredential);
    }
    Ok(())
}

fn validate_challenge_binding(
    binding: &WebAuthnChallengeBinding,
    now_ms: i64,
) -> Result<(), PersistenceError> {
    validate_non_negative_timestamp(now_ms)?;
    validate_non_negative_timestamp(binding.reserved_at_ms)?;
    database_revision(binding.auth_revision)?;
    validate_context(&binding.client_context)?;
    if now_ms < binding.reserved_at_ms
        || now_ms >= binding.verification_expires_at_ms
        || binding.verification_expires_at_ms <= binding.reserved_at_ms
    {
        return Err(PersistenceError::InvalidWebAuthnCeremony);
    }
    Ok(())
}

/// A terminal WebAuthn row is allowed to outlive the short verifier lease, but never the
/// enclosing C3 challenge (enforced by the joined handoff query). This relaxed time check is used
/// only for replaying an already-committed terminal handoff; pending verification still uses
/// `validate_challenge_binding` and must finish inside the verifier lease.
fn validate_terminal_handoff_binding(
    binding: &WebAuthnChallengeBinding,
    now_ms: i64,
) -> Result<(), PersistenceError> {
    validate_non_negative_timestamp(now_ms)?;
    validate_non_negative_timestamp(binding.reserved_at_ms)?;
    database_revision(binding.auth_revision)?;
    validate_context(&binding.client_context)?;
    if now_ms < binding.reserved_at_ms
        || binding.verification_expires_at_ms <= binding.reserved_at_ms
    {
        return Err(PersistenceError::InvalidWebAuthnCeremony);
    }
    Ok(())
}

fn validate_context(context: &AuthChallengeClientContext) -> Result<(), PersistenceError> {
    match (
        context.key_version,
        context.client_network_hmac,
        context.user_agent_hash,
    ) {
        (None, None, None) => Ok(()),
        (Some(version), Some(_), Some(_)) => {
            database_key_version(version)?;
            Ok(())
        }
        _ => Err(PersistenceError::InvalidWebAuthnCeremony),
    }
}

fn validate_new_authentication(
    ceremony: &NewWebAuthnAuthenticationCeremony,
) -> Result<(), PersistenceError> {
    validate_challenge_binding(&ceremony.binding, ceremony.created_at_ms)?;
    validate_envelope(&ceremony.state)?;
    Ok(())
}

fn validate_authentication_commit(
    command: &WebAuthnAuthenticationCommit<'_>,
) -> Result<(), PersistenceError> {
    validate_challenge_binding(command.binding, command.now_ms)?;
    database_revision(command.expected_ceremony_revision)?;
    database_revision(command.expected_credential_revision)?;
    validate_envelope(command.material)
        .map_err(|_| PersistenceError::InvalidWebAuthnCredential)?;
    let persisted_counter = command
        .expected_sign_counter
        .max(command.observed_sign_counter);
    let counter_has_signal =
        command.expected_sign_counter > 0 || command.observed_sign_counter > 0;
    let counter_non_monotonic =
        counter_has_signal && command.observed_sign_counter <= command.expected_sign_counter;
    let backup_counter_anomaly = command.backup_eligible && counter_non_monotonic;
    if command.backup_state && !command.backup_eligible
        || command.backup_eligible != command.expected_backup_eligible
        || (!command.backup_eligible && counter_non_monotonic)
        || command.sign_counter != persisted_counter
        || command.backup_counter_anomaly != backup_counter_anomaly
    {
        return Err(PersistenceError::InvalidWebAuthnCredential);
    }
    Ok(())
}

fn validate_clone_suspected(command: &WebAuthnCloneSuspected<'_>) -> Result<(), PersistenceError> {
    validate_challenge_binding(command.binding, command.now_ms)?;
    database_revision(command.expected_ceremony_revision)?;
    database_revision(command.expected_credential_revision)?;
    if command.expected_sign_counter == 0 {
        return Err(PersistenceError::InvalidWebAuthnCredential);
    }
    Ok(())
}

fn envelope_from_columns(
    key_version: i64,
    nonce: Vec<u8>,
    ciphertext: Vec<u8>,
    aad_hash: Vec<u8>,
) -> Result<SecretEnvelope, PersistenceError> {
    let key_version = u32::try_from(key_version)
        .map_err(|_| PersistenceError::InvalidStoredWebAuthnCeremony)?;
    database_key_version(key_version)
        .map_err(|_| PersistenceError::InvalidStoredWebAuthnCeremony)?;
    let nonce = nonce
        .try_into()
        .map_err(|_| PersistenceError::InvalidStoredWebAuthnCeremony)?;
    let aad_hash = aad_hash
        .try_into()
        .map_err(|_| PersistenceError::InvalidStoredWebAuthnCeremony)?;
    let envelope = SecretEnvelope {
        key_version,
        nonce,
        ciphertext,
        aad_hash,
    };
    validate_envelope(&envelope).map_err(|_| PersistenceError::InvalidStoredWebAuthnCeremony)?;
    Ok(envelope)
}

#[allow(clippy::too_many_arguments)]
fn decode_credential(
    id: uuid::Uuid,
    user_id: uuid::Uuid,
    credential_id: Vec<u8>,
    user_handle: Vec<u8>,
    aaguid: Option<[u8; 16]>,
    user_verified: bool,
    backup_eligible: bool,
    backup_state: bool,
    sign_counter: i64,
    nickname: String,
    status: String,
    material_key_version: i64,
    material_nonce: Vec<u8>,
    material_ciphertext: Vec<u8>,
    material_aad_hash: Vec<u8>,
    created_at_ms: i64,
    last_used_at_ms: Option<i64>,
    backup_counter_anomaly_at_ms: Option<i64>,
    revoked_at_ms: Option<i64>,
    clone_suspected_at_ms: Option<i64>,
    revision: i64,
    transports: Vec<WebAuthnTransport>,
) -> Result<StoredWebAuthnCredential, PersistenceError> {
    let sign_counter = u32::try_from(sign_counter)
        .map_err(|_| PersistenceError::InvalidStoredWebAuthnCredential)?;
    let credential = WebAuthnCredential {
        id: EntityId::from_uuid(id),
        user_id: EntityId::from_uuid(user_id),
        credential_id: WebAuthnCredentialId::parse(credential_id)
            .map_err(|_| PersistenceError::InvalidStoredWebAuthnCredential)?,
        user_handle: WebAuthnUserHandle::parse(user_handle)
            .map_err(|_| PersistenceError::InvalidStoredWebAuthnCredential)?,
        aaguid: aaguid.map(WebAuthnAaguid::from_bytes),
        transports,
        user_verified,
        backup_eligible,
        backup_state,
        sign_counter,
        nickname: WebAuthnNickname::parse(nickname)
            .map_err(|_| PersistenceError::InvalidStoredWebAuthnCredential)?,
        status: WebAuthnCredentialStatus::parse(&status)
            .map_err(|_| PersistenceError::InvalidStoredWebAuthnCredential)?,
        created_at_ms,
        last_used_at_ms,
        backup_counter_anomaly_at_ms,
        revoked_at_ms,
        clone_suspected_at_ms,
        revision: decode_revision(revision)
            .map_err(|_| PersistenceError::InvalidStoredWebAuthnCredential)?,
    };
    if !credential.user_verified
        || credential.user_handle != WebAuthnUserHandle::for_user(credential.user_id)
        || (credential.backup_state && !credential.backup_eligible)
        || credential.created_at_ms < 0
        || credential
            .last_used_at_ms
            .is_some_and(|value| value < credential.created_at_ms)
        || credential.backup_counter_anomaly_at_ms.is_some_and(|value| {
            !credential.backup_eligible || value < credential.created_at_ms
        })
        || match credential.status {
            WebAuthnCredentialStatus::Active => {
                credential.revoked_at_ms.is_some() || credential.clone_suspected_at_ms.is_some()
            }
            WebAuthnCredentialStatus::Revoked => {
                credential.revoked_at_ms.is_none() || credential.clone_suspected_at_ms.is_some()
            }
            WebAuthnCredentialStatus::CloneSuspected => {
                credential.revoked_at_ms.is_some() || credential.clone_suspected_at_ms.is_none()
            }
        }
    {
        return Err(PersistenceError::InvalidStoredWebAuthnCredential);
    }
    let material = envelope_from_columns(
        material_key_version,
        material_nonce,
        material_ciphertext,
        material_aad_hash,
    )
    .map_err(|_| PersistenceError::InvalidStoredWebAuthnCredential)?;
    Ok(StoredWebAuthnCredential {
        credential,
        material,
    })
}

async fn transports_sqlite(
    pool: &SqlitePool,
    credential_id: EntityId,
) -> Result<Vec<WebAuthnTransport>, PersistenceError> {
    let values: Vec<String> = sqlx::query_scalar(
        "SELECT transport FROM webauthn_credential_transports WHERE credential_id=? ORDER BY transport",
    )
    .bind(credential_id.to_string())
    .fetch_all(pool)
    .await?;
    values
        .into_iter()
        .map(|value| {
            WebAuthnTransport::parse(&value)
                .map_err(|_| PersistenceError::InvalidStoredWebAuthnCredential)
        })
        .collect()
}

async fn transports_postgres(
    pool: &PgPool,
    credential_id: EntityId,
) -> Result<Vec<WebAuthnTransport>, PersistenceError> {
    let values: Vec<String> = sqlx::query_scalar(
        "SELECT transport FROM webauthn_credential_transports WHERE credential_id=$1 ORDER BY transport",
    )
    .bind(credential_id.into_uuid())
    .fetch_all(pool)
    .await?;
    values
        .into_iter()
        .map(|value| {
            WebAuthnTransport::parse(&value)
                .map_err(|_| PersistenceError::InvalidStoredWebAuthnCredential)
        })
        .collect()
}

async fn decode_sqlite_credential_row(
    pool: &SqlitePool,
    row: sqlx::sqlite::SqliteRow,
) -> Result<StoredWebAuthnCredential, PersistenceError> {
    let id = EntityId::from_uuid(uuid::Uuid::parse_str(row.try_get("id")?)?);
    let transports = transports_sqlite(pool, id).await?;
    let aaguid = row
        .try_get::<Option<Vec<u8>>, _>("aaguid")?
        .map(|value| {
            value
                .try_into()
                .map_err(|_| PersistenceError::InvalidStoredWebAuthnCredential)
        })
        .transpose()?;
    decode_credential(
        id.into_uuid(),
        uuid::Uuid::parse_str(row.try_get("user_id")?)?,
        row.try_get("credential_id")?,
        row.try_get("user_handle")?,
        aaguid,
        row.try_get("user_verified")?,
        row.try_get("backup_eligible")?,
        row.try_get("backup_state")?,
        row.try_get("sign_counter")?,
        row.try_get("nickname")?,
        row.try_get("status")?,
        row.try_get("material_key_version")?,
        row.try_get("material_nonce")?,
        row.try_get("material_ciphertext")?,
        row.try_get("material_aad_hash")?,
        row.try_get("created_at_ms")?,
        row.try_get("last_used_at_ms")?,
        row.try_get("backup_counter_anomaly_at_ms")?,
        row.try_get("revoked_at_ms")?,
        row.try_get("clone_suspected_at_ms")?,
        row.try_get("revision")?,
        transports,
    )
}

async fn decode_postgres_credential_row(
    pool: &PgPool,
    row: sqlx::postgres::PgRow,
) -> Result<StoredWebAuthnCredential, PersistenceError> {
    let id = EntityId::from_uuid(row.try_get("id")?);
    let transports = transports_postgres(pool, id).await?;
    let aaguid = row
        .try_get::<Option<uuid::Uuid>, _>("aaguid")?
        .map(|value| *value.as_bytes());
    decode_credential(
        id.into_uuid(),
        row.try_get("user_id")?,
        row.try_get("credential_id")?,
        row.try_get("user_handle")?,
        aaguid,
        row.try_get("user_verified")?,
        row.try_get("backup_eligible")?,
        row.try_get("backup_state")?,
        row.try_get("sign_counter")?,
        row.try_get("nickname")?,
        row.try_get("status")?,
        i64::from(row.try_get::<i32, _>("material_key_version")?),
        row.try_get("material_nonce")?,
        row.try_get("material_ciphertext")?,
        row.try_get("material_aad_hash")?,
        row.try_get("created_at_ms")?,
        row.try_get("last_used_at_ms")?,
        row.try_get("backup_counter_anomaly_at_ms")?,
        row.try_get("revoked_at_ms")?,
        row.try_get("clone_suspected_at_ms")?,
        row.try_get("revision")?,
        transports,
    )
}

fn stored_registration_from_new(
    value: &NewWebAuthnRegistrationCeremony,
) -> StoredWebAuthnRegistrationCeremony {
    StoredWebAuthnRegistrationCeremony {
        id: value.id,
        user_id: value.guard.user_id,
        session_id: value.guard.actor_session_id,
        origin: value.origin.clone(),
        user_revision: value.guard.expected_user_revision,
        auth_revision: value.guard.expected_auth_revision,
        recent_auth_at_ms: value.guard.expected_recent_auth_at_ms,
        state: value.state.clone(),
        created_at_ms: value.guard.now_ms,
        expires_at_ms: value.expires_at_ms,
        revision: Revision::initial(),
    }
}

fn stored_authentication_from_new(
    value: &NewWebAuthnAuthenticationCeremony,
) -> StoredWebAuthnAuthenticationCeremony {
    StoredWebAuthnAuthenticationCeremony {
        id: value.id,
        binding: value.binding.clone(),
        origin: value.origin.clone(),
        state: value.state.clone(),
        created_at_ms: value.created_at_ms,
        revision: Revision::initial(),
    }
}

async fn expire_registration_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    user_id: EntityId,
    session_id: EntityId,
    now_ms: i64,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "UPDATE webauthn_ceremonies SET status='expired',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=expires_at_ms,revision=revision+1 WHERE kind='registration' AND status='pending' AND user_id=? AND session_id=? AND expires_at_ms<=?",
    )
    .bind(user_id.to_string())
    .bind(session_id.to_string())
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn expire_registration_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: EntityId,
    session_id: EntityId,
    now_ms: i64,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "UPDATE webauthn_ceremonies SET status='expired',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=expires_at_ms,revision=revision+1 WHERE kind='registration' AND status='pending' AND user_id=$1 AND session_id=$2 AND expires_at_ms<=$3",
    )
    .bind(user_id.into_uuid())
    .bind(session_id.into_uuid())
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn begin_registration_sqlite(
    pool: &SqlitePool,
    ceremony: &NewWebAuthnRegistrationCeremony,
) -> Result<BeginWebAuthnRegistrationOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    expire_registration_sqlite(
        &mut transaction,
        ceremony.guard.user_id,
        ceremony.guard.actor_session_id,
        ceremony.guard.now_ms,
    )
    .await?;
    let inserted = sqlx::query(
        "INSERT INTO webauthn_ceremonies (id,kind,status,user_id,session_id,purpose,rp_id,origin,user_revision,auth_revision,recent_auth_at_ms,auth_challenge_id,claim_id,reserved_at_ms,verification_expires_at_ms,context_key_version,client_network_hmac,user_agent_hash,state_schema_version,state_key_version,state_nonce,state_ciphertext,state_aad_hash,created_at_ms,expires_at_ms,finished_at_ms,revision) SELECT ?,'registration','pending',u.id,s.id,'credential_enrollment',?,?,?,?,?,NULL,NULL,NULL,NULL,NULL,NULL,NULL,1,?,?,?,?,?,?,NULL,0 FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=? AND s.id=? AND u.status='active' AND u.deleted_at_ms IS NULL AND u.force_password_change=0 AND u.revision=? AND uas.auth_revision=? AND s.status='active' AND s.auth_revision=uas.auth_revision AND s.recent_auth_at_ms=? AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>? ON CONFLICT DO NOTHING",
    )
    .bind(ceremony.id.to_string())
    .bind(ceremony.origin.rp_id())
    .bind(ceremony.origin.as_str())
    .bind(database_revision(ceremony.guard.expected_user_revision)?)
    .bind(database_revision(ceremony.guard.expected_auth_revision)?)
    .bind(ceremony.guard.expected_recent_auth_at_ms)
    .bind(i64::from(ceremony.state.key_version))
    .bind(ceremony.state.nonce.as_slice())
    .bind(ceremony.state.ciphertext.as_slice())
    .bind(ceremony.state.aad_hash.as_slice())
    .bind(ceremony.guard.now_ms)
    .bind(ceremony.expires_at_ms)
    .bind(ceremony.guard.user_id.to_string())
    .bind(ceremony.guard.actor_session_id.to_string())
    .bind(database_revision(ceremony.guard.expected_user_revision)?)
    .bind(database_revision(ceremony.guard.expected_auth_revision)?)
    .bind(ceremony.guard.expected_recent_auth_at_ms)
    .bind(ceremony.guard.now_ms)
    .bind(ceremony.guard.now_ms)
    .bind(ceremony.guard.now_ms)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if inserted == 1 {
        transaction.commit().await?;
        return Ok(BeginWebAuthnRegistrationOutcome::Created(
            stored_registration_from_new(ceremony),
        ));
    }
    let pending: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM webauthn_ceremonies WHERE kind='registration' AND status='pending' AND user_id=? AND session_id=?",
    )
    .bind(ceremony.guard.user_id.to_string())
    .bind(ceremony.guard.actor_session_id.to_string())
    .fetch_one(&mut *transaction)
    .await?;
    transaction.commit().await?;
    if pending > 0 {
        Ok(BeginWebAuthnRegistrationOutcome::AlreadyPending)
    } else {
        Ok(BeginWebAuthnRegistrationOutcome::Stale)
    }
}

async fn begin_registration_postgres(
    pool: &PgPool,
    ceremony: &NewWebAuthnRegistrationCeremony,
) -> Result<BeginWebAuthnRegistrationOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    match lock_guard_postgres(&mut transaction, &ceremony.guard).await {
        Ok(()) => {}
        Err(PersistenceError::SessionPrincipalUnavailable) => {
            return Ok(BeginWebAuthnRegistrationOutcome::Stale);
        }
        Err(error) => return Err(error),
    }
    expire_registration_postgres(
        &mut transaction,
        ceremony.guard.user_id,
        ceremony.guard.actor_session_id,
        ceremony.guard.now_ms,
    )
    .await?;
    let inserted = sqlx::query(
        "INSERT INTO webauthn_ceremonies (id,kind,status,user_id,session_id,purpose,rp_id,origin,user_revision,auth_revision,recent_auth_at_ms,auth_challenge_id,claim_id,reserved_at_ms,verification_expires_at_ms,context_key_version,client_network_hmac,user_agent_hash,state_schema_version,state_key_version,state_nonce,state_ciphertext,state_aad_hash,created_at_ms,expires_at_ms,finished_at_ms,revision) SELECT $1,'registration','pending',u.id,s.id,'credential_enrollment',$2,$3,$4,$5,$6,NULL,NULL,NULL,NULL,NULL,NULL,NULL,1,$7,$8,$9,$10,$11,$12,NULL,0 FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=$13 AND s.id=$14 AND u.status='active' AND u.deleted_at_ms IS NULL AND NOT u.force_password_change AND u.revision=$4 AND uas.auth_revision=$5 AND s.status='active' AND s.auth_revision=uas.auth_revision AND s.recent_auth_at_ms=$6 AND s.last_seen_at_ms<=$11 AND s.idle_expires_at_ms>$11 AND s.absolute_expires_at_ms>$11 ON CONFLICT DO NOTHING",
    )
    .bind(ceremony.id.into_uuid())
    .bind(ceremony.origin.rp_id())
    .bind(ceremony.origin.as_str())
    .bind(database_revision(ceremony.guard.expected_user_revision)?)
    .bind(database_revision(ceremony.guard.expected_auth_revision)?)
    .bind(ceremony.guard.expected_recent_auth_at_ms)
    .bind(database_key_version(ceremony.state.key_version)?)
    .bind(ceremony.state.nonce.as_slice())
    .bind(ceremony.state.ciphertext.as_slice())
    .bind(ceremony.state.aad_hash.as_slice())
    .bind(ceremony.guard.now_ms)
    .bind(ceremony.expires_at_ms)
    .bind(ceremony.guard.user_id.into_uuid())
    .bind(ceremony.guard.actor_session_id.into_uuid())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if inserted == 1 {
        transaction.commit().await?;
        return Ok(BeginWebAuthnRegistrationOutcome::Created(
            stored_registration_from_new(ceremony),
        ));
    }
    let pending: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM webauthn_ceremonies WHERE kind='registration' AND status='pending' AND user_id=$1 AND session_id=$2",
    )
    .bind(ceremony.guard.user_id.into_uuid())
    .bind(ceremony.guard.actor_session_id.into_uuid())
    .fetch_one(&mut *transaction)
    .await?;
    transaction.commit().await?;
    if pending > 0 {
        Ok(BeginWebAuthnRegistrationOutcome::AlreadyPending)
    } else {
        Ok(BeginWebAuthnRegistrationOutcome::Stale)
    }
}

async fn load_registration_sqlite(
    pool: &SqlitePool,
    ceremony_id: EntityId,
    expected_revision: Revision,
    guard: &WebAuthnSessionGuard,
    origin: &WebAuthnOrigin,
) -> Result<Option<StoredWebAuthnRegistrationCeremony>, PersistenceError> {
    let mut transaction = pool.begin().await?;
    expire_registration_sqlite(
        &mut transaction,
        guard.user_id,
        guard.actor_session_id,
        guard.now_ms,
    )
    .await?;
    let row = sqlx::query(
        "SELECT id,user_id,session_id,origin,user_revision,auth_revision,recent_auth_at_ms,state_key_version,state_nonce,state_ciphertext,state_aad_hash,created_at_ms,expires_at_ms,revision FROM webauthn_ceremonies c WHERE c.id=? AND c.kind='registration' AND c.status='pending' AND c.user_id=? AND c.session_id=? AND c.purpose='credential_enrollment' AND c.rp_id=? AND c.origin=? AND c.user_revision=? AND c.auth_revision=? AND c.recent_auth_at_ms=? AND c.revision=? AND c.created_at_ms<=? AND c.expires_at_ms>? AND EXISTS(SELECT 1 FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=c.user_id AND s.id=c.session_id AND u.status='active' AND u.deleted_at_ms IS NULL AND u.force_password_change=0 AND u.revision=c.user_revision AND uas.auth_revision=c.auth_revision AND s.status='active' AND s.auth_revision=uas.auth_revision AND s.recent_auth_at_ms=c.recent_auth_at_ms AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?)",
    )
    .bind(ceremony_id.to_string())
    .bind(guard.user_id.to_string())
    .bind(guard.actor_session_id.to_string())
    .bind(origin.rp_id())
    .bind(origin.as_str())
    .bind(database_revision(guard.expected_user_revision)?)
    .bind(database_revision(guard.expected_auth_revision)?)
    .bind(guard.expected_recent_auth_at_ms)
    .bind(database_revision(expected_revision)?)
    .bind(guard.now_ms)
    .bind(guard.now_ms)
    .bind(guard.now_ms)
    .bind(guard.now_ms)
    .bind(guard.now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    transaction.commit().await?;
    row.map(|row| {
        Ok(StoredWebAuthnRegistrationCeremony {
            id: EntityId::from_uuid(uuid::Uuid::parse_str(row.try_get("id")?)?),
            user_id: EntityId::from_uuid(uuid::Uuid::parse_str(row.try_get("user_id")?)?),
            session_id: EntityId::from_uuid(uuid::Uuid::parse_str(row.try_get("session_id")?)?),
            origin: WebAuthnOrigin::parse(row.try_get::<String, _>("origin")?)
                .map_err(|_| PersistenceError::InvalidStoredWebAuthnCeremony)?,
            user_revision: decode_revision(row.try_get("user_revision")?)?,
            auth_revision: decode_revision(row.try_get("auth_revision")?)?,
            recent_auth_at_ms: row.try_get("recent_auth_at_ms")?,
            state: envelope_from_columns(
                row.try_get("state_key_version")?,
                row.try_get("state_nonce")?,
                row.try_get("state_ciphertext")?,
                row.try_get("state_aad_hash")?,
            )?,
            created_at_ms: row.try_get("created_at_ms")?,
            expires_at_ms: row.try_get("expires_at_ms")?,
            revision: decode_revision(row.try_get("revision")?)?,
        })
    })
    .transpose()
}

async fn load_registration_postgres(
    pool: &PgPool,
    ceremony_id: EntityId,
    expected_revision: Revision,
    guard: &WebAuthnSessionGuard,
    origin: &WebAuthnOrigin,
) -> Result<Option<StoredWebAuthnRegistrationCeremony>, PersistenceError> {
    let mut transaction = pool.begin().await?;
    expire_registration_postgres(
        &mut transaction,
        guard.user_id,
        guard.actor_session_id,
        guard.now_ms,
    )
    .await?;
    let row = sqlx::query(
        "SELECT id,user_id,session_id,origin,user_revision,auth_revision,recent_auth_at_ms,state_key_version,state_nonce,state_ciphertext,state_aad_hash,created_at_ms,expires_at_ms,revision FROM webauthn_ceremonies c WHERE c.id=$1 AND c.kind='registration' AND c.status='pending' AND c.user_id=$2 AND c.session_id=$3 AND c.purpose='credential_enrollment' AND c.rp_id=$4 AND c.origin=$5 AND c.user_revision=$6 AND c.auth_revision=$7 AND c.recent_auth_at_ms=$8 AND c.revision=$9 AND c.created_at_ms<=$10 AND c.expires_at_ms>$10 AND EXISTS(SELECT 1 FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=c.user_id AND s.id=c.session_id AND u.status='active' AND u.deleted_at_ms IS NULL AND NOT u.force_password_change AND u.revision=c.user_revision AND uas.auth_revision=c.auth_revision AND s.status='active' AND s.auth_revision=uas.auth_revision AND s.recent_auth_at_ms=c.recent_auth_at_ms AND s.last_seen_at_ms<=$10 AND s.idle_expires_at_ms>$10 AND s.absolute_expires_at_ms>$10)",
    )
    .bind(ceremony_id.into_uuid())
    .bind(guard.user_id.into_uuid())
    .bind(guard.actor_session_id.into_uuid())
    .bind(origin.rp_id())
    .bind(origin.as_str())
    .bind(database_revision(guard.expected_user_revision)?)
    .bind(database_revision(guard.expected_auth_revision)?)
    .bind(guard.expected_recent_auth_at_ms)
    .bind(database_revision(expected_revision)?)
    .bind(guard.now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    transaction.commit().await?;
    row.map(|row| {
        Ok(StoredWebAuthnRegistrationCeremony {
            id: EntityId::from_uuid(row.try_get("id")?),
            user_id: EntityId::from_uuid(row.try_get("user_id")?),
            session_id: EntityId::from_uuid(row.try_get("session_id")?),
            origin: WebAuthnOrigin::parse(row.try_get::<String, _>("origin")?)
                .map_err(|_| PersistenceError::InvalidStoredWebAuthnCeremony)?,
            user_revision: decode_revision(row.try_get("user_revision")?)?,
            auth_revision: decode_revision(row.try_get("auth_revision")?)?,
            recent_auth_at_ms: row.try_get("recent_auth_at_ms")?,
            state: envelope_from_columns(
                i64::from(row.try_get::<i32, _>("state_key_version")?),
                row.try_get("state_nonce")?,
                row.try_get("state_ciphertext")?,
                row.try_get("state_aad_hash")?,
            )?,
            created_at_ms: row.try_get("created_at_ms")?,
            expires_at_ms: row.try_get("expires_at_ms")?,
            revision: decode_revision(row.try_get("revision")?)?,
        })
    })
    .transpose()
}

async fn reject_registration_sqlite(
    pool: &SqlitePool,
    ceremony_id: EntityId,
    expected_revision: Revision,
    guard: &WebAuthnSessionGuard,
    origin: &WebAuthnOrigin,
) -> Result<bool, PersistenceError> {
    let changed = sqlx::query(
        "UPDATE webauthn_ceremonies SET status='rejected',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=?,revision=revision+1 WHERE id=? AND kind='registration' AND status='pending' AND user_id=? AND session_id=? AND purpose='credential_enrollment' AND rp_id=? AND origin=? AND user_revision=? AND auth_revision=? AND recent_auth_at_ms=? AND revision=? AND created_at_ms<=? AND expires_at_ms>? AND EXISTS(SELECT 1 FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=webauthn_ceremonies.user_id AND s.id=webauthn_ceremonies.session_id AND u.status='active' AND u.deleted_at_ms IS NULL AND u.force_password_change=0 AND u.revision=webauthn_ceremonies.user_revision AND uas.auth_revision=webauthn_ceremonies.auth_revision AND s.status='active' AND s.auth_revision=uas.auth_revision AND s.recent_auth_at_ms=webauthn_ceremonies.recent_auth_at_ms AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?)",
    )
    .bind(guard.now_ms)
    .bind(ceremony_id.to_string())
    .bind(guard.user_id.to_string())
    .bind(guard.actor_session_id.to_string())
    .bind(origin.rp_id())
    .bind(origin.as_str())
    .bind(database_revision(guard.expected_user_revision)?)
    .bind(database_revision(guard.expected_auth_revision)?)
    .bind(guard.expected_recent_auth_at_ms)
    .bind(database_revision(expected_revision)?)
    .bind(guard.now_ms)
    .bind(guard.now_ms)
    .bind(guard.now_ms)
    .bind(guard.now_ms)
    .bind(guard.now_ms)
    .execute(pool)
    .await?
    .rows_affected();
    Ok(changed == 1)
}

async fn reject_registration_postgres(
    pool: &PgPool,
    ceremony_id: EntityId,
    expected_revision: Revision,
    guard: &WebAuthnSessionGuard,
    origin: &WebAuthnOrigin,
) -> Result<bool, PersistenceError> {
    let changed = sqlx::query(
        "UPDATE webauthn_ceremonies SET status='rejected',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=$1,revision=revision+1 WHERE id=$2 AND kind='registration' AND status='pending' AND user_id=$3 AND session_id=$4 AND purpose='credential_enrollment' AND rp_id=$5 AND origin=$6 AND user_revision=$7 AND auth_revision=$8 AND recent_auth_at_ms=$9 AND revision=$10 AND created_at_ms<=$1 AND expires_at_ms>$1 AND EXISTS(SELECT 1 FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=webauthn_ceremonies.user_id AND s.id=webauthn_ceremonies.session_id AND u.status='active' AND u.deleted_at_ms IS NULL AND NOT u.force_password_change AND u.revision=webauthn_ceremonies.user_revision AND uas.auth_revision=webauthn_ceremonies.auth_revision AND s.status='active' AND s.auth_revision=uas.auth_revision AND s.recent_auth_at_ms=webauthn_ceremonies.recent_auth_at_ms AND s.last_seen_at_ms<=$1 AND s.idle_expires_at_ms>$1 AND s.absolute_expires_at_ms>$1)",
    )
    .bind(guard.now_ms)
    .bind(ceremony_id.into_uuid())
    .bind(guard.user_id.into_uuid())
    .bind(guard.actor_session_id.into_uuid())
    .bind(origin.rp_id())
    .bind(origin.as_str())
    .bind(database_revision(guard.expected_user_revision)?)
    .bind(database_revision(guard.expected_auth_revision)?)
    .bind(guard.expected_recent_auth_at_ms)
    .bind(database_revision(expected_revision)?)
    .execute(pool)
    .await?
    .rows_affected();
    Ok(changed == 1)
}

const CREDENTIAL_COLUMNS: &str = "wc.id,wc.user_id,wc.credential_id,wc.user_handle,wc.aaguid,wc.user_verified,wc.backup_eligible,wc.backup_state,wc.sign_counter,wc.nickname,wc.status,wc.material_key_version,wc.material_nonce,wc.material_ciphertext,wc.material_aad_hash,wc.created_at_ms,wc.last_used_at_ms,wc.backup_counter_anomaly_at_ms,wc.revoked_at_ms,wc.clone_suspected_at_ms,wc.revision";

async fn active_for_registration_sqlite(
    pool: &SqlitePool,
    guard: &WebAuthnSessionGuard,
) -> Result<Vec<StoredWebAuthnCredential>, PersistenceError> {
    let query = format!(
        "SELECT {CREDENTIAL_COLUMNS} FROM webauthn_credentials wc WHERE wc.user_id=? AND wc.status='active' AND EXISTS(SELECT 1 FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=wc.user_id AND s.id=? AND u.status='active' AND u.deleted_at_ms IS NULL AND u.force_password_change=0 AND u.revision=? AND uas.auth_revision=? AND s.status='active' AND s.auth_revision=uas.auth_revision AND s.recent_auth_at_ms=? AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?) ORDER BY wc.created_at_ms,wc.id"
    );
    let rows = sqlx::query(&query)
        .bind(guard.user_id.to_string())
        .bind(guard.actor_session_id.to_string())
        .bind(database_revision(guard.expected_user_revision)?)
        .bind(database_revision(guard.expected_auth_revision)?)
        .bind(guard.expected_recent_auth_at_ms)
        .bind(guard.now_ms)
        .bind(guard.now_ms)
        .bind(guard.now_ms)
        .fetch_all(pool)
        .await?;
    let mut credentials = Vec::with_capacity(rows.len());
    for row in rows {
        credentials.push(decode_sqlite_credential_row(pool, row).await?);
    }
    Ok(credentials)
}

async fn active_for_registration_postgres(
    pool: &PgPool,
    guard: &WebAuthnSessionGuard,
) -> Result<Vec<StoredWebAuthnCredential>, PersistenceError> {
    let query = format!(
        "SELECT {CREDENTIAL_COLUMNS} FROM webauthn_credentials wc WHERE wc.user_id=$1 AND wc.status='active' AND EXISTS(SELECT 1 FROM users u JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.user_id=u.id WHERE u.id=wc.user_id AND s.id=$2 AND u.status='active' AND u.deleted_at_ms IS NULL AND NOT u.force_password_change AND u.revision=$3 AND uas.auth_revision=$4 AND s.status='active' AND s.auth_revision=uas.auth_revision AND s.recent_auth_at_ms=$5 AND s.last_seen_at_ms<=$6 AND s.idle_expires_at_ms>$6 AND s.absolute_expires_at_ms>$6) ORDER BY wc.created_at_ms,wc.id"
    );
    let rows = sqlx::query(&query)
        .bind(guard.user_id.into_uuid())
        .bind(guard.actor_session_id.into_uuid())
        .bind(database_revision(guard.expected_user_revision)?)
        .bind(database_revision(guard.expected_auth_revision)?)
        .bind(guard.expected_recent_auth_at_ms)
        .bind(guard.now_ms)
        .fetch_all(pool)
        .await?;
    let mut credentials = Vec::with_capacity(rows.len());
    for row in rows {
        credentials.push(decode_postgres_credential_row(pool, row).await?);
    }
    Ok(credentials)
}

fn sqlite_context_values(
    context: &AuthChallengeClientContext,
) -> (Option<i64>, Option<&[u8]>, Option<&[u8]>) {
    (
        context.key_version.map(i64::from),
        context.client_network_hmac.as_ref().map(<[u8; 32]>::as_slice),
        context.user_agent_hash.as_ref().map(<[u8; 32]>::as_slice),
    )
}

fn postgres_context_values(
    context: &AuthChallengeClientContext,
) -> Result<(Option<i32>, Option<&[u8]>, Option<&[u8]>), PersistenceError> {
    Ok((
        context.key_version.map(database_key_version).transpose()?,
        context.client_network_hmac.as_ref().map(<[u8; 32]>::as_slice),
        context.user_agent_hash.as_ref().map(<[u8; 32]>::as_slice),
    ))
}

async fn active_for_challenge_sqlite(
    pool: &SqlitePool,
    binding: &WebAuthnChallengeBinding,
    now_ms: i64,
) -> Result<Vec<StoredWebAuthnCredential>, PersistenceError> {
    let (context_version, network_hmac, agent_hash) =
        sqlite_context_values(&binding.client_context);
    let query = format!(
        "SELECT {CREDENTIAL_COLUMNS} FROM webauthn_credentials wc WHERE wc.user_id=? AND wc.status='active' AND EXISTS(SELECT 1 FROM auth_challenges ac JOIN users u ON u.id=ac.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE ac.id=? AND ac.user_id=wc.user_id AND ac.session_id IS ? AND ac.purpose=? AND ac.auth_revision=? AND ac.status='verification_pending' AND ac.attempt_claim_id=? AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=? AND ac.attempt_expires_at_ms=? AND ac.context_key_version IS ? AND ac.client_network_hmac IS ? AND ac.user_agent_hash IS ? AND ac.created_at_ms<=? AND ac.expires_at_ms>? AND ac.attempt_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?))) ORDER BY wc.created_at_ms,wc.id"
    );
    let rows = sqlx::query(&query)
        .bind(binding.user_id.to_string())
        .bind(binding.auth_challenge_id.to_string())
        .bind(binding.session_id.map(|id| id.to_string()))
        .bind(binding.purpose.as_str())
        .bind(database_revision(binding.auth_revision)?)
        .bind(binding.claim_id.to_string())
        .bind(binding.reserved_at_ms)
        .bind(binding.verification_expires_at_ms)
        .bind(context_version)
        .bind(network_hmac)
        .bind(agent_hash)
        .bind(now_ms)
        .bind(now_ms)
        .bind(now_ms)
        .bind(now_ms)
        .bind(now_ms)
        .bind(now_ms)
        .fetch_all(pool)
        .await?;
    let mut credentials = Vec::with_capacity(rows.len());
    for row in rows {
        credentials.push(decode_sqlite_credential_row(pool, row).await?);
    }
    Ok(credentials)
}

async fn active_for_challenge_postgres(
    pool: &PgPool,
    binding: &WebAuthnChallengeBinding,
    now_ms: i64,
) -> Result<Vec<StoredWebAuthnCredential>, PersistenceError> {
    let (context_version, network_hmac, agent_hash) =
        postgres_context_values(&binding.client_context)?;
    let query = format!(
        "SELECT {CREDENTIAL_COLUMNS} FROM webauthn_credentials wc WHERE wc.user_id=$1 AND wc.status='active' AND EXISTS(SELECT 1 FROM auth_challenges ac JOIN users u ON u.id=ac.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE ac.id=$2 AND ac.user_id=wc.user_id AND ac.session_id IS NOT DISTINCT FROM $3 AND ac.purpose=$4 AND ac.auth_revision=$5 AND ac.status='verification_pending' AND ac.attempt_claim_id=$6 AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=$7 AND ac.attempt_expires_at_ms=$8 AND ac.context_key_version IS NOT DISTINCT FROM $9 AND ac.client_network_hmac IS NOT DISTINCT FROM $10 AND ac.user_agent_hash IS NOT DISTINCT FROM $11 AND ac.created_at_ms<=$12 AND ac.expires_at_ms>$12 AND ac.attempt_expires_at_ms>$12 AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=$12 AND s.idle_expires_at_ms>$12 AND s.absolute_expires_at_ms>$12))) ORDER BY wc.created_at_ms,wc.id"
    );
    let rows = sqlx::query(&query)
        .bind(binding.user_id.into_uuid())
        .bind(binding.auth_challenge_id.into_uuid())
        .bind(binding.session_id.map(EntityId::into_uuid))
        .bind(binding.purpose.as_str())
        .bind(database_revision(binding.auth_revision)?)
        .bind(binding.claim_id.into_uuid())
        .bind(binding.reserved_at_ms)
        .bind(binding.verification_expires_at_ms)
        .bind(context_version)
        .bind(network_hmac)
        .bind(agent_hash)
        .bind(now_ms)
        .fetch_all(pool)
        .await?;
    let mut credentials = Vec::with_capacity(rows.len());
    for row in rows {
        credentials.push(decode_postgres_credential_row(pool, row).await?);
    }
    Ok(credentials)
}

async fn insert_new_credential_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    value: &NewWebAuthnCredential,
) -> Result<bool, PersistenceError> {
    let credential = &value.credential;
    let inserted = sqlx::query(
        "INSERT INTO webauthn_credentials (id,user_id,credential_id,user_handle,aaguid,user_verified,backup_eligible,backup_state,sign_counter,nickname,status,material_schema_version,material_key_version,material_nonce,material_ciphertext,material_aad_hash,created_at_ms,last_used_at_ms,backup_counter_anomaly_at_ms,revoked_at_ms,clone_suspected_at_ms,revision) VALUES (?,?,?,?,?,1,?,?,?,?,'active',1,?,?,?,?,?,NULL,NULL,NULL,NULL,0) ON CONFLICT DO NOTHING",
    )
    .bind(credential.id.to_string())
    .bind(credential.user_id.to_string())
    .bind(credential.credential_id.as_bytes())
    .bind(credential.user_handle.as_bytes())
    .bind(credential.aaguid.map(|value| value.as_bytes().to_vec()))
    .bind(credential.backup_eligible)
    .bind(credential.backup_state)
    .bind(i64::from(credential.sign_counter))
    .bind(credential.nickname.as_str())
    .bind(i64::from(value.material.key_version))
    .bind(value.material.nonce.as_slice())
    .bind(value.material.ciphertext.as_slice())
    .bind(value.material.aad_hash.as_slice())
    .bind(credential.created_at_ms)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    if inserted != 1 {
        return Ok(false);
    }
    for transport in &credential.transports {
        sqlx::query(
            "INSERT INTO webauthn_credential_transports (credential_id,transport) VALUES (?,?)",
        )
        .bind(credential.id.to_string())
        .bind(transport.as_str())
        .execute(&mut **transaction)
        .await?;
    }
    Ok(true)
}

async fn insert_new_credential_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    value: &NewWebAuthnCredential,
) -> Result<bool, PersistenceError> {
    let credential = &value.credential;
    let inserted = sqlx::query(
        "INSERT INTO webauthn_credentials (id,user_id,credential_id,user_handle,aaguid,user_verified,backup_eligible,backup_state,sign_counter,nickname,status,material_schema_version,material_key_version,material_nonce,material_ciphertext,material_aad_hash,created_at_ms,last_used_at_ms,backup_counter_anomaly_at_ms,revoked_at_ms,clone_suspected_at_ms,revision) VALUES ($1,$2,$3,$4,$5,true,$6,$7,$8,$9,'active',1,$10,$11,$12,$13,$14,NULL,NULL,NULL,NULL,0) ON CONFLICT DO NOTHING",
    )
    .bind(credential.id.into_uuid())
    .bind(credential.user_id.into_uuid())
    .bind(credential.credential_id.as_bytes())
    .bind(credential.user_handle.as_bytes())
    .bind(
        credential
            .aaguid
            .map(|value| uuid::Uuid::from_bytes(*value.as_bytes())),
    )
    .bind(credential.backup_eligible)
    .bind(credential.backup_state)
    .bind(i64::from(credential.sign_counter))
    .bind(credential.nickname.as_str())
    .bind(database_key_version(value.material.key_version)?)
    .bind(value.material.nonce.as_slice())
    .bind(value.material.ciphertext.as_slice())
    .bind(value.material.aad_hash.as_slice())
    .bind(credential.created_at_ms)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    if inserted != 1 {
        return Ok(false);
    }
    for transport in &credential.transports {
        sqlx::query(
            "INSERT INTO webauthn_credential_transports (credential_id,transport) VALUES ($1,$2)",
        )
        .bind(credential.id.into_uuid())
        .bind(transport.as_str())
        .execute(&mut **transaction)
        .await?;
    }
    Ok(true)
}

async fn complete_registration_sqlite(
    pool: &SqlitePool,
    command: &CompleteWebAuthnRegistration<'_>,
) -> Result<CompleteWebAuthnRegistrationOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let locked = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=auth_revision WHERE user_id=? AND auth_revision=?",
    )
    .bind(command.guard.user_id.to_string())
    .bind(database_revision(command.guard.expected_auth_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if locked != 1 {
        return Ok(CompleteWebAuthnRegistrationOutcome::Stale);
    }
    let valid: Option<i64> = sqlx::query_scalar(
        "SELECT 1 FROM webauthn_ceremonies c JOIN users u ON u.id=c.user_id JOIN user_auth_state uas ON uas.user_id=u.id JOIN auth_sessions s ON s.id=c.session_id AND s.user_id=u.id WHERE c.id=? AND c.kind='registration' AND c.status='pending' AND c.user_id=? AND c.session_id=? AND c.purpose='credential_enrollment' AND c.rp_id=? AND c.origin=? AND c.user_revision=? AND c.auth_revision=? AND c.recent_auth_at_ms=? AND c.revision=? AND c.created_at_ms<=? AND c.expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND u.force_password_change=0 AND u.revision=c.user_revision AND uas.auth_revision=c.auth_revision AND s.status='active' AND s.auth_revision=uas.auth_revision AND s.recent_auth_at_ms=c.recent_auth_at_ms AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?",
    )
    .bind(command.ceremony_id.to_string())
    .bind(command.guard.user_id.to_string())
    .bind(command.guard.actor_session_id.to_string())
    .bind(command.origin.rp_id())
    .bind(command.origin.as_str())
    .bind(database_revision(command.guard.expected_user_revision)?)
    .bind(database_revision(command.guard.expected_auth_revision)?)
    .bind(command.guard.expected_recent_auth_at_ms)
    .bind(database_revision(command.expected_ceremony_revision)?)
    .bind(command.guard.now_ms)
    .bind(command.guard.now_ms)
    .bind(command.guard.now_ms)
    .bind(command.guard.now_ms)
    .bind(command.guard.now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    if valid.is_none() {
        return Ok(CompleteWebAuthnRegistrationOutcome::Stale);
    }
    if !insert_new_credential_sqlite(&mut transaction, command.credential).await? {
        let rejected = sqlx::query(
            "UPDATE webauthn_ceremonies SET status='rejected',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=?,revision=revision+1 WHERE id=? AND status='pending' AND revision=?",
        )
        .bind(command.guard.now_ms)
        .bind(command.ceremony_id.to_string())
        .bind(database_revision(command.expected_ceremony_revision)?)
        .execute(&mut *transaction)
        .await?
        .rows_affected();
        if rejected != 1 {
            return Ok(CompleteWebAuthnRegistrationOutcome::Stale);
        }
        transaction.commit().await?;
        return Ok(CompleteWebAuthnRegistrationOutcome::DuplicateCredential);
    }
    let next_auth_revision = command
        .guard
        .expected_auth_revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let advanced = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=?,updated_at_ms=MAX(updated_at_ms,?) WHERE user_id=? AND auth_revision=?",
    )
    .bind(database_revision(next_auth_revision)?)
    .bind(command.guard.now_ms)
    .bind(command.guard.user_id.to_string())
    .bind(database_revision(command.guard.expected_auth_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if advanced != 1 {
        return Ok(CompleteWebAuthnRegistrationOutcome::Stale);
    }
    let revoked_other_sessions = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=MAX(created_at_ms,?),revoked_reason='security_policy',revision=revision+1 WHERE user_id=? AND id<>? AND status='active' AND auth_revision=?",
    )
    .bind(command.guard.now_ms)
    .bind(command.guard.user_id.to_string())
    .bind(command.guard.actor_session_id.to_string())
    .bind(database_revision(command.guard.expected_auth_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    let actor_updated = sqlx::query(
        "UPDATE auth_sessions SET auth_revision=?,auth_level='phishing_resistant',recent_auth_at_ms=?,last_seen_at_ms=?,revision=revision+1 WHERE id=? AND user_id=? AND status='active' AND auth_revision=? AND recent_auth_at_ms=? AND last_seen_at_ms<=? AND idle_expires_at_ms>? AND absolute_expires_at_ms>?",
    )
    .bind(database_revision(next_auth_revision)?)
    .bind(command.guard.now_ms)
    .bind(command.guard.now_ms)
    .bind(command.guard.actor_session_id.to_string())
    .bind(command.guard.user_id.to_string())
    .bind(database_revision(command.guard.expected_auth_revision)?)
    .bind(command.guard.expected_recent_auth_at_ms)
    .bind(command.guard.now_ms)
    .bind(command.guard.now_ms)
    .bind(command.guard.now_ms)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    let consumed = sqlx::query(
        "UPDATE webauthn_ceremonies SET status='consumed',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=?,revision=revision+1 WHERE id=? AND status='pending' AND revision=?",
    )
    .bind(command.guard.now_ms)
    .bind(command.ceremony_id.to_string())
    .bind(database_revision(command.expected_ceremony_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if actor_updated != 1 || consumed != 1 {
        return Ok(CompleteWebAuthnRegistrationOutcome::Stale);
    }
    transaction.commit().await?;
    Ok(CompleteWebAuthnRegistrationOutcome::Registered(
        WebAuthnRegistrationResult {
            credential: command.credential.credential.clone(),
            auth_revision: next_auth_revision,
            revoked_other_sessions,
        },
    ))
}

async fn complete_registration_postgres(
    pool: &PgPool,
    command: &CompleteWebAuthnRegistration<'_>,
) -> Result<CompleteWebAuthnRegistrationOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    match lock_guard_postgres(&mut transaction, &command.guard).await {
        Ok(()) => {}
        Err(PersistenceError::SessionPrincipalUnavailable) => {
            return Ok(CompleteWebAuthnRegistrationOutcome::Stale);
        }
        Err(error) => return Err(error),
    }
    let valid: Option<i64> = sqlx::query_scalar(
        "SELECT 1::BIGINT FROM webauthn_ceremonies c WHERE c.id=$1 AND c.kind='registration' AND c.status='pending' AND c.user_id=$2 AND c.session_id=$3 AND c.purpose='credential_enrollment' AND c.rp_id=$4 AND c.origin=$5 AND c.user_revision=$6 AND c.auth_revision=$7 AND c.recent_auth_at_ms=$8 AND c.revision=$9 AND c.created_at_ms<=$10 AND c.expires_at_ms>$10 FOR UPDATE OF c",
    )
    .bind(command.ceremony_id.into_uuid())
    .bind(command.guard.user_id.into_uuid())
    .bind(command.guard.actor_session_id.into_uuid())
    .bind(command.origin.rp_id())
    .bind(command.origin.as_str())
    .bind(database_revision(command.guard.expected_user_revision)?)
    .bind(database_revision(command.guard.expected_auth_revision)?)
    .bind(command.guard.expected_recent_auth_at_ms)
    .bind(database_revision(command.expected_ceremony_revision)?)
    .bind(command.guard.now_ms)
    .fetch_optional(&mut *transaction)
    .await?;
    if valid.is_none() {
        return Ok(CompleteWebAuthnRegistrationOutcome::Stale);
    }
    if !insert_new_credential_postgres(&mut transaction, command.credential).await? {
        let rejected = sqlx::query(
            "UPDATE webauthn_ceremonies SET status='rejected',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=$1,revision=revision+1 WHERE id=$2 AND status='pending' AND revision=$3",
        )
        .bind(command.guard.now_ms)
        .bind(command.ceremony_id.into_uuid())
        .bind(database_revision(command.expected_ceremony_revision)?)
        .execute(&mut *transaction)
        .await?
        .rows_affected();
        if rejected != 1 {
            return Ok(CompleteWebAuthnRegistrationOutcome::Stale);
        }
        transaction.commit().await?;
        return Ok(CompleteWebAuthnRegistrationOutcome::DuplicateCredential);
    }
    let next_auth_revision = command
        .guard
        .expected_auth_revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let advanced = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=$1,updated_at_ms=GREATEST(updated_at_ms,$2) WHERE user_id=$3 AND auth_revision=$4",
    )
    .bind(database_revision(next_auth_revision)?)
    .bind(command.guard.now_ms)
    .bind(command.guard.user_id.into_uuid())
    .bind(database_revision(command.guard.expected_auth_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if advanced != 1 {
        return Ok(CompleteWebAuthnRegistrationOutcome::Stale);
    }
    let revoked_other_sessions = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=GREATEST(created_at_ms,$1),revoked_reason='security_policy',revision=revision+1 WHERE user_id=$2 AND id<>$3 AND status='active' AND auth_revision=$4",
    )
    .bind(command.guard.now_ms)
    .bind(command.guard.user_id.into_uuid())
    .bind(command.guard.actor_session_id.into_uuid())
    .bind(database_revision(command.guard.expected_auth_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    let actor_updated = sqlx::query(
        "UPDATE auth_sessions SET auth_revision=$1,auth_level='phishing_resistant',recent_auth_at_ms=$2,last_seen_at_ms=$2,revision=revision+1 WHERE id=$3 AND user_id=$4 AND status='active' AND auth_revision=$5 AND recent_auth_at_ms=$6 AND last_seen_at_ms<=$2 AND idle_expires_at_ms>$2 AND absolute_expires_at_ms>$2",
    )
    .bind(database_revision(next_auth_revision)?)
    .bind(command.guard.now_ms)
    .bind(command.guard.actor_session_id.into_uuid())
    .bind(command.guard.user_id.into_uuid())
    .bind(database_revision(command.guard.expected_auth_revision)?)
    .bind(command.guard.expected_recent_auth_at_ms)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    let consumed = sqlx::query(
        "UPDATE webauthn_ceremonies SET status='consumed',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=$1,revision=revision+1 WHERE id=$2 AND status='pending' AND revision=$3",
    )
    .bind(command.guard.now_ms)
    .bind(command.ceremony_id.into_uuid())
    .bind(database_revision(command.expected_ceremony_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if actor_updated != 1 || consumed != 1 {
        return Ok(CompleteWebAuthnRegistrationOutcome::Stale);
    }
    transaction.commit().await?;
    Ok(CompleteWebAuthnRegistrationOutcome::Registered(
        WebAuthnRegistrationResult {
            credential: command.credential.credential.clone(),
            auth_revision: next_auth_revision,
            revoked_other_sessions,
        },
    ))
}

/// Acquires the exact C3 verifier claim before any authentication ceremony or credential write.
/// SQLite's no-op update takes the database writer lock; PostgreSQL locks only the matched C3 row.
/// A concurrent lease refresh therefore either wins before this function (and the WebAuthn write
/// is stale) or waits until the terminal ceremony exists (and preserves the durable handoff).
async fn lock_authentication_claim_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    binding: &WebAuthnChallengeBinding,
    now_ms: i64,
) -> Result<bool, PersistenceError> {
    let (context_version, network_hmac, agent_hash) =
        sqlite_context_values(&binding.client_context);
    let affected = sqlx::query(
        "UPDATE auth_challenges SET updated_at_ms=updated_at_ms WHERE id=? AND user_id=? AND session_id IS ? AND purpose=? AND auth_revision=? AND status='verification_pending' AND attempt_claim_id=? AND attempted_method='webauthn' AND attempt_started_at_ms=? AND attempt_expires_at_ms=? AND context_key_version IS ? AND client_network_hmac IS ? AND user_agent_hash IS ? AND created_at_ms<=? AND expires_at_ms>? AND attempt_expires_at_ms>? AND EXISTS(SELECT 1 FROM users u JOIN user_auth_state uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) AND (session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?))",
    )
    .bind(binding.auth_challenge_id.to_string())
    .bind(binding.user_id.to_string())
    .bind(binding.session_id.map(|id| id.to_string()))
    .bind(binding.purpose.as_str())
    .bind(database_revision(binding.auth_revision)?)
    .bind(binding.claim_id.to_string())
    .bind(binding.reserved_at_ms)
    .bind(binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?
    .rows_affected();
    Ok(affected == 1)
}

async fn lock_authentication_claim_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    binding: &WebAuthnChallengeBinding,
    now_ms: i64,
) -> Result<bool, PersistenceError> {
    let (context_version, network_hmac, agent_hash) =
        postgres_context_values(&binding.client_context)?;
    let locked: Option<i64> = sqlx::query_scalar(
        "SELECT 1::BIGINT FROM auth_challenges ac JOIN users u ON u.id=ac.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE ac.id=$1 AND ac.user_id=$2 AND ac.session_id IS NOT DISTINCT FROM $3 AND ac.purpose=$4 AND ac.auth_revision=$5 AND ac.status='verification_pending' AND ac.attempt_claim_id=$6 AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=$7 AND ac.attempt_expires_at_ms=$8 AND ac.context_key_version IS NOT DISTINCT FROM $9 AND ac.client_network_hmac IS NOT DISTINCT FROM $10 AND ac.user_agent_hash IS NOT DISTINCT FROM $11 AND ac.created_at_ms<=$12 AND ac.expires_at_ms>$12 AND ac.attempt_expires_at_ms>$12 AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=$12 AND s.idle_expires_at_ms>$12 AND s.absolute_expires_at_ms>$12)) FOR UPDATE OF ac",
    )
    .bind(binding.auth_challenge_id.into_uuid())
    .bind(binding.user_id.into_uuid())
    .bind(binding.session_id.map(EntityId::into_uuid))
    .bind(binding.purpose.as_str())
    .bind(database_revision(binding.auth_revision)?)
    .bind(binding.claim_id.into_uuid())
    .bind(binding.reserved_at_ms)
    .bind(binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(now_ms)
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(locked.is_some())
}

async fn begin_authentication_sqlite(
    pool: &SqlitePool,
    ceremony: &NewWebAuthnAuthenticationCeremony,
) -> Result<BeginWebAuthnAuthenticationOutcome, PersistenceError> {
    let binding = &ceremony.binding;
    let (context_version, network_hmac, agent_hash) =
        sqlite_context_values(&binding.client_context);
    let mut transaction = pool.begin().await?;
    if !lock_authentication_claim_sqlite(
        &mut transaction,
        binding,
        ceremony.created_at_ms,
    )
    .await?
    {
        return Ok(BeginWebAuthnAuthenticationOutcome::Stale);
    }
    let inserted = sqlx::query(
        "INSERT INTO webauthn_ceremonies (id,kind,status,user_id,session_id,purpose,rp_id,origin,user_revision,auth_revision,recent_auth_at_ms,auth_challenge_id,claim_id,reserved_at_ms,verification_expires_at_ms,context_key_version,client_network_hmac,user_agent_hash,state_schema_version,state_key_version,state_nonce,state_ciphertext,state_aad_hash,created_at_ms,expires_at_ms,finished_at_ms,revision) SELECT ?,'authentication','pending',ac.user_id,ac.session_id,ac.purpose,?,?,NULL,ac.auth_revision,NULL,ac.id,ac.attempt_claim_id,ac.attempt_started_at_ms,ac.attempt_expires_at_ms,ac.context_key_version,ac.client_network_hmac,ac.user_agent_hash,1,?,?,?,?,?,ac.attempt_expires_at_ms,NULL,0 FROM auth_challenges ac JOIN users u ON u.id=ac.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE ac.id=? AND ac.user_id=? AND ac.session_id IS ? AND ac.purpose=? AND ac.auth_revision=? AND ac.status='verification_pending' AND ac.attempt_claim_id=? AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=? AND ac.attempt_expires_at_ms=? AND ac.context_key_version IS ? AND ac.client_network_hmac IS ? AND ac.user_agent_hash IS ? AND ac.created_at_ms<=? AND ac.expires_at_ms>? AND ac.attempt_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?)) AND EXISTS(SELECT 1 FROM webauthn_credentials wc WHERE wc.user_id=ac.user_id AND wc.status='active') ON CONFLICT DO NOTHING",
    )
    .bind(ceremony.id.to_string())
    .bind(ceremony.origin.rp_id())
    .bind(ceremony.origin.as_str())
    .bind(i64::from(ceremony.state.key_version))
    .bind(ceremony.state.nonce.as_slice())
    .bind(ceremony.state.ciphertext.as_slice())
    .bind(ceremony.state.aad_hash.as_slice())
    .bind(ceremony.created_at_ms)
    .bind(binding.auth_challenge_id.to_string())
    .bind(binding.user_id.to_string())
    .bind(binding.session_id.map(|id| id.to_string()))
    .bind(binding.purpose.as_str())
    .bind(database_revision(binding.auth_revision)?)
    .bind(binding.claim_id.to_string())
    .bind(binding.reserved_at_ms)
    .bind(binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(ceremony.created_at_ms)
    .bind(ceremony.created_at_ms)
    .bind(ceremony.created_at_ms)
    .bind(ceremony.created_at_ms)
    .bind(ceremony.created_at_ms)
    .bind(ceremony.created_at_ms)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if inserted == 1 {
        transaction.commit().await?;
        return Ok(BeginWebAuthnAuthenticationOutcome::Created(
            stored_authentication_from_new(ceremony),
        ));
    }
    let existing: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM webauthn_ceremonies WHERE kind='authentication' AND status='pending' AND claim_id=?",
    )
    .bind(binding.claim_id.to_string())
    .fetch_one(&mut *transaction)
    .await?;
    transaction.commit().await?;
    if existing > 0 {
        Ok(BeginWebAuthnAuthenticationOutcome::AlreadyPending)
    } else {
        Ok(BeginWebAuthnAuthenticationOutcome::Stale)
    }
}

async fn begin_authentication_postgres(
    pool: &PgPool,
    ceremony: &NewWebAuthnAuthenticationCeremony,
) -> Result<BeginWebAuthnAuthenticationOutcome, PersistenceError> {
    let binding = &ceremony.binding;
    let (context_version, network_hmac, agent_hash) =
        postgres_context_values(&binding.client_context)?;
    let mut transaction = pool.begin().await?;
    if !lock_authentication_principal_postgres(
        &mut transaction,
        binding,
        ceremony.created_at_ms,
    )
    .await?
    {
        return Ok(BeginWebAuthnAuthenticationOutcome::Stale);
    }
    if !lock_authentication_claim_postgres(
        &mut transaction,
        binding,
        ceremony.created_at_ms,
    )
    .await?
    {
        return Ok(BeginWebAuthnAuthenticationOutcome::Stale);
    }
    let inserted = sqlx::query(
        "INSERT INTO webauthn_ceremonies (id,kind,status,user_id,session_id,purpose,rp_id,origin,user_revision,auth_revision,recent_auth_at_ms,auth_challenge_id,claim_id,reserved_at_ms,verification_expires_at_ms,context_key_version,client_network_hmac,user_agent_hash,state_schema_version,state_key_version,state_nonce,state_ciphertext,state_aad_hash,created_at_ms,expires_at_ms,finished_at_ms,revision) SELECT $1,'authentication','pending',ac.user_id,ac.session_id,ac.purpose,$2,$3,NULL,ac.auth_revision,NULL,ac.id,ac.attempt_claim_id,ac.attempt_started_at_ms,ac.attempt_expires_at_ms,ac.context_key_version,ac.client_network_hmac,ac.user_agent_hash,1,$4,$5,$6,$7,$8,ac.attempt_expires_at_ms,NULL,0 FROM auth_challenges ac JOIN users u ON u.id=ac.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE ac.id=$9 AND ac.user_id=$10 AND ac.session_id IS NOT DISTINCT FROM $11 AND ac.purpose=$12 AND ac.auth_revision=$13 AND ac.status='verification_pending' AND ac.attempt_claim_id=$14 AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=$15 AND ac.attempt_expires_at_ms=$16 AND ac.context_key_version IS NOT DISTINCT FROM $17 AND ac.client_network_hmac IS NOT DISTINCT FROM $18 AND ac.user_agent_hash IS NOT DISTINCT FROM $19 AND ac.created_at_ms<=$8 AND ac.expires_at_ms>$8 AND ac.attempt_expires_at_ms>$8 AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=$8 AND s.idle_expires_at_ms>$8 AND s.absolute_expires_at_ms>$8)) AND EXISTS(SELECT 1 FROM webauthn_credentials wc WHERE wc.user_id=ac.user_id AND wc.status='active') ON CONFLICT DO NOTHING",
    )
    .bind(ceremony.id.into_uuid())
    .bind(ceremony.origin.rp_id())
    .bind(ceremony.origin.as_str())
    .bind(database_key_version(ceremony.state.key_version)?)
    .bind(ceremony.state.nonce.as_slice())
    .bind(ceremony.state.ciphertext.as_slice())
    .bind(ceremony.state.aad_hash.as_slice())
    .bind(ceremony.created_at_ms)
    .bind(binding.auth_challenge_id.into_uuid())
    .bind(binding.user_id.into_uuid())
    .bind(binding.session_id.map(EntityId::into_uuid))
    .bind(binding.purpose.as_str())
    .bind(database_revision(binding.auth_revision)?)
    .bind(binding.claim_id.into_uuid())
    .bind(binding.reserved_at_ms)
    .bind(binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if inserted == 1 {
        transaction.commit().await?;
        return Ok(BeginWebAuthnAuthenticationOutcome::Created(
            stored_authentication_from_new(ceremony),
        ));
    }
    let existing: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM webauthn_ceremonies WHERE kind='authentication' AND status='pending' AND claim_id=$1",
    )
    .bind(binding.claim_id.into_uuid())
    .fetch_one(&mut *transaction)
    .await?;
    transaction.commit().await?;
    if existing > 0 {
        Ok(BeginWebAuthnAuthenticationOutcome::AlreadyPending)
    } else {
        Ok(BeginWebAuthnAuthenticationOutcome::Stale)
    }
}

async fn expire_authentication_sqlite(
    pool: &SqlitePool,
    ceremony_id: EntityId,
    now_ms: i64,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "UPDATE webauthn_ceremonies SET status='expired',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=expires_at_ms,revision=revision+1 WHERE id=? AND kind='authentication' AND status='pending' AND expires_at_ms<=?",
    )
    .bind(ceremony_id.to_string())
    .bind(now_ms)
    .execute(pool)
    .await?;
    Ok(())
}

async fn expire_authentication_postgres(
    pool: &PgPool,
    ceremony_id: EntityId,
    now_ms: i64,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "UPDATE webauthn_ceremonies SET status='expired',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=expires_at_ms,revision=revision+1 WHERE id=$1 AND kind='authentication' AND status='pending' AND expires_at_ms<=$2",
    )
    .bind(ceremony_id.into_uuid())
    .bind(now_ms)
    .execute(pool)
    .await?;
    Ok(())
}

fn decode_authentication_handoff(
    status: &str,
) -> Result<WebAuthnAuthenticationHandoff, PersistenceError> {
    match status {
        "consumed" => Ok(WebAuthnAuthenticationHandoff::Verified),
        "rejected" => Ok(WebAuthnAuthenticationHandoff::Rejected),
        _ => Err(PersistenceError::InvalidStoredWebAuthnCeremony),
    }
}

async fn authentication_handoff_sqlite(
    pool: &SqlitePool,
    ceremony_id: EntityId,
    terminal_revision: Revision,
    binding: &WebAuthnChallengeBinding,
    origin: &WebAuthnOrigin,
    now_ms: i64,
) -> Result<Option<WebAuthnAuthenticationHandoff>, PersistenceError> {
    let (context_version, network_hmac, agent_hash) =
        sqlite_context_values(&binding.client_context);
    let status = sqlx::query_scalar::<_, String>(
        "SELECT c.status FROM webauthn_ceremonies c WHERE c.id=? AND c.kind='authentication' AND c.status IN ('consumed','rejected') AND c.user_id=? AND c.session_id IS ? AND c.purpose=? AND c.rp_id=? AND c.origin=? AND c.auth_revision=? AND c.auth_challenge_id=? AND c.claim_id=? AND c.reserved_at_ms=? AND c.verification_expires_at_ms=? AND c.context_key_version IS ? AND c.client_network_hmac IS ? AND c.user_agent_hash IS ? AND c.revision=? AND c.reserved_at_ms<=? AND EXISTS(SELECT 1 FROM auth_challenges ac JOIN users u ON u.id=ac.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE ac.id=c.auth_challenge_id AND ac.user_id=c.user_id AND ac.session_id IS c.session_id AND ac.purpose=c.purpose AND ac.auth_revision=c.auth_revision AND ac.status='verification_pending' AND ac.attempt_claim_id=c.claim_id AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=c.reserved_at_ms AND ac.attempt_expires_at_ms=c.verification_expires_at_ms AND ac.context_key_version IS c.context_key_version AND ac.client_network_hmac IS c.client_network_hmac AND ac.user_agent_hash IS c.user_agent_hash AND ac.expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?)))",
    )
    .bind(ceremony_id.to_string())
    .bind(binding.user_id.to_string())
    .bind(binding.session_id.map(|id| id.to_string()))
    .bind(binding.purpose.as_str())
    .bind(origin.rp_id())
    .bind(origin.as_str())
    .bind(database_revision(binding.auth_revision)?)
    .bind(binding.auth_challenge_id.to_string())
    .bind(binding.claim_id.to_string())
    .bind(binding.reserved_at_ms)
    .bind(binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(database_revision(terminal_revision)?)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .fetch_optional(pool)
    .await?;
    status
        .as_deref()
        .map(decode_authentication_handoff)
        .transpose()
}

async fn authentication_handoff_postgres(
    pool: &PgPool,
    ceremony_id: EntityId,
    terminal_revision: Revision,
    binding: &WebAuthnChallengeBinding,
    origin: &WebAuthnOrigin,
    now_ms: i64,
) -> Result<Option<WebAuthnAuthenticationHandoff>, PersistenceError> {
    let (context_version, network_hmac, agent_hash) =
        postgres_context_values(&binding.client_context)?;
    let status = sqlx::query_scalar::<_, String>(
        "SELECT c.status FROM webauthn_ceremonies c WHERE c.id=$1 AND c.kind='authentication' AND c.status IN ('consumed','rejected') AND c.user_id=$2 AND c.session_id IS NOT DISTINCT FROM $3 AND c.purpose=$4 AND c.rp_id=$5 AND c.origin=$6 AND c.auth_revision=$7 AND c.auth_challenge_id=$8 AND c.claim_id=$9 AND c.reserved_at_ms=$10 AND c.verification_expires_at_ms=$11 AND c.context_key_version IS NOT DISTINCT FROM $12 AND c.client_network_hmac IS NOT DISTINCT FROM $13 AND c.user_agent_hash IS NOT DISTINCT FROM $14 AND c.revision=$15 AND c.reserved_at_ms<=$16 AND EXISTS(SELECT 1 FROM auth_challenges ac JOIN users u ON u.id=ac.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE ac.id=c.auth_challenge_id AND ac.user_id=c.user_id AND ac.session_id IS NOT DISTINCT FROM c.session_id AND ac.purpose=c.purpose AND ac.auth_revision=c.auth_revision AND ac.status='verification_pending' AND ac.attempt_claim_id=c.claim_id AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=c.reserved_at_ms AND ac.attempt_expires_at_ms=c.verification_expires_at_ms AND ac.context_key_version IS NOT DISTINCT FROM c.context_key_version AND ac.client_network_hmac IS NOT DISTINCT FROM c.client_network_hmac AND ac.user_agent_hash IS NOT DISTINCT FROM c.user_agent_hash AND ac.expires_at_ms>$16 AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=$16 AND s.idle_expires_at_ms>$16 AND s.absolute_expires_at_ms>$16)))",
    )
    .bind(ceremony_id.into_uuid())
    .bind(binding.user_id.into_uuid())
    .bind(binding.session_id.map(EntityId::into_uuid))
    .bind(binding.purpose.as_str())
    .bind(origin.rp_id())
    .bind(origin.as_str())
    .bind(database_revision(binding.auth_revision)?)
    .bind(binding.auth_challenge_id.into_uuid())
    .bind(binding.claim_id.into_uuid())
    .bind(binding.reserved_at_ms)
    .bind(binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(database_revision(terminal_revision)?)
    .bind(now_ms)
    .fetch_optional(pool)
    .await?;
    status
        .as_deref()
        .map(decode_authentication_handoff)
        .transpose()
}

async fn authentication_context_sqlite(
    pool: &SqlitePool,
    ceremony_id: EntityId,
    expected_ceremony_revision: Revision,
    binding: &WebAuthnChallengeBinding,
    origin: &WebAuthnOrigin,
    credential_id: &WebAuthnCredentialId,
    now_ms: i64,
) -> Result<Option<(StoredWebAuthnAuthenticationCeremony, StoredWebAuthnCredential)>, PersistenceError>
{
    expire_authentication_sqlite(pool, ceremony_id, now_ms).await?;
    let (context_version, network_hmac, agent_hash) =
        sqlite_context_values(&binding.client_context);
    let ceremony_row = sqlx::query(
        "SELECT c.state_key_version,c.state_nonce,c.state_ciphertext,c.state_aad_hash,c.created_at_ms,c.revision FROM webauthn_ceremonies c WHERE c.id=? AND c.kind='authentication' AND c.status='pending' AND c.user_id=? AND c.session_id IS ? AND c.purpose=? AND c.rp_id=? AND c.origin=? AND c.auth_revision=? AND c.auth_challenge_id=? AND c.claim_id=? AND c.reserved_at_ms=? AND c.verification_expires_at_ms=? AND c.context_key_version IS ? AND c.client_network_hmac IS ? AND c.user_agent_hash IS ? AND c.revision=? AND c.created_at_ms<=? AND c.expires_at_ms>? AND EXISTS(SELECT 1 FROM auth_challenges ac JOIN users u ON u.id=ac.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE ac.id=c.auth_challenge_id AND ac.user_id=c.user_id AND ac.session_id IS c.session_id AND ac.purpose=c.purpose AND ac.auth_revision=c.auth_revision AND ac.status='verification_pending' AND ac.attempt_claim_id=c.claim_id AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=c.reserved_at_ms AND ac.attempt_expires_at_ms=c.verification_expires_at_ms AND ac.context_key_version IS c.context_key_version AND ac.client_network_hmac IS c.client_network_hmac AND ac.user_agent_hash IS c.user_agent_hash AND ac.expires_at_ms>? AND ac.attempt_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?)))",
    )
    .bind(ceremony_id.to_string())
    .bind(binding.user_id.to_string())
    .bind(binding.session_id.map(|id| id.to_string()))
    .bind(binding.purpose.as_str())
    .bind(origin.rp_id())
    .bind(origin.as_str())
    .bind(database_revision(binding.auth_revision)?)
    .bind(binding.auth_challenge_id.to_string())
    .bind(binding.claim_id.to_string())
    .bind(binding.reserved_at_ms)
    .bind(binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(database_revision(expected_ceremony_revision)?)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .fetch_optional(pool)
    .await?;
    let Some(row) = ceremony_row else {
        return Ok(None);
    };
    let query = format!(
        "SELECT {CREDENTIAL_COLUMNS} FROM webauthn_credentials wc WHERE wc.user_id=? AND wc.credential_id=? AND wc.status='active'"
    );
    let credential_row = sqlx::query(&query)
        .bind(binding.user_id.to_string())
        .bind(credential_id.as_bytes())
        .fetch_optional(pool)
        .await?;
    let Some(credential_row) = credential_row else {
        return Ok(None);
    };
    let ceremony = StoredWebAuthnAuthenticationCeremony {
        id: ceremony_id,
        binding: binding.clone(),
        origin: origin.clone(),
        state: envelope_from_columns(
            row.try_get("state_key_version")?,
            row.try_get("state_nonce")?,
            row.try_get("state_ciphertext")?,
            row.try_get("state_aad_hash")?,
        )?,
        created_at_ms: row.try_get("created_at_ms")?,
        revision: decode_revision(row.try_get("revision")?)?,
    };
    let credential = decode_sqlite_credential_row(pool, credential_row).await?;
    Ok(Some((ceremony, credential)))
}

async fn authentication_context_postgres(
    pool: &PgPool,
    ceremony_id: EntityId,
    expected_ceremony_revision: Revision,
    binding: &WebAuthnChallengeBinding,
    origin: &WebAuthnOrigin,
    credential_id: &WebAuthnCredentialId,
    now_ms: i64,
) -> Result<Option<(StoredWebAuthnAuthenticationCeremony, StoredWebAuthnCredential)>, PersistenceError>
{
    expire_authentication_postgres(pool, ceremony_id, now_ms).await?;
    let (context_version, network_hmac, agent_hash) =
        postgres_context_values(&binding.client_context)?;
    let ceremony_row = sqlx::query(
        "SELECT c.state_key_version,c.state_nonce,c.state_ciphertext,c.state_aad_hash,c.created_at_ms,c.revision FROM webauthn_ceremonies c WHERE c.id=$1 AND c.kind='authentication' AND c.status='pending' AND c.user_id=$2 AND c.session_id IS NOT DISTINCT FROM $3 AND c.purpose=$4 AND c.rp_id=$5 AND c.origin=$6 AND c.auth_revision=$7 AND c.auth_challenge_id=$8 AND c.claim_id=$9 AND c.reserved_at_ms=$10 AND c.verification_expires_at_ms=$11 AND c.context_key_version IS NOT DISTINCT FROM $12 AND c.client_network_hmac IS NOT DISTINCT FROM $13 AND c.user_agent_hash IS NOT DISTINCT FROM $14 AND c.revision=$15 AND c.created_at_ms<=$16 AND c.expires_at_ms>$16 AND EXISTS(SELECT 1 FROM auth_challenges ac JOIN users u ON u.id=ac.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE ac.id=c.auth_challenge_id AND ac.user_id=c.user_id AND ac.session_id IS NOT DISTINCT FROM c.session_id AND ac.purpose=c.purpose AND ac.auth_revision=c.auth_revision AND ac.status='verification_pending' AND ac.attempt_claim_id=c.claim_id AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=c.reserved_at_ms AND ac.attempt_expires_at_ms=c.verification_expires_at_ms AND ac.context_key_version IS NOT DISTINCT FROM c.context_key_version AND ac.client_network_hmac IS NOT DISTINCT FROM c.client_network_hmac AND ac.user_agent_hash IS NOT DISTINCT FROM c.user_agent_hash AND ac.expires_at_ms>$16 AND ac.attempt_expires_at_ms>$16 AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=$16 AND s.idle_expires_at_ms>$16 AND s.absolute_expires_at_ms>$16)))",
    )
    .bind(ceremony_id.into_uuid())
    .bind(binding.user_id.into_uuid())
    .bind(binding.session_id.map(EntityId::into_uuid))
    .bind(binding.purpose.as_str())
    .bind(origin.rp_id())
    .bind(origin.as_str())
    .bind(database_revision(binding.auth_revision)?)
    .bind(binding.auth_challenge_id.into_uuid())
    .bind(binding.claim_id.into_uuid())
    .bind(binding.reserved_at_ms)
    .bind(binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(database_revision(expected_ceremony_revision)?)
    .bind(now_ms)
    .fetch_optional(pool)
    .await?;
    let Some(row) = ceremony_row else {
        return Ok(None);
    };
    let query = format!(
        "SELECT {CREDENTIAL_COLUMNS} FROM webauthn_credentials wc WHERE wc.user_id=$1 AND wc.credential_id=$2 AND wc.status='active'"
    );
    let credential_row = sqlx::query(&query)
        .bind(binding.user_id.into_uuid())
        .bind(credential_id.as_bytes())
        .fetch_optional(pool)
        .await?;
    let Some(credential_row) = credential_row else {
        return Ok(None);
    };
    let ceremony = StoredWebAuthnAuthenticationCeremony {
        id: ceremony_id,
        binding: binding.clone(),
        origin: origin.clone(),
        state: envelope_from_columns(
            i64::from(row.try_get::<i32, _>("state_key_version")?),
            row.try_get("state_nonce")?,
            row.try_get("state_ciphertext")?,
            row.try_get("state_aad_hash")?,
        )?,
        created_at_ms: row.try_get("created_at_ms")?,
        revision: decode_revision(row.try_get("revision")?)?,
    };
    let credential = decode_postgres_credential_row(pool, credential_row).await?;
    Ok(Some((ceremony, credential)))
}

async fn credential_by_internal_id_sqlite(
    pool: &SqlitePool,
    credential_id: EntityId,
) -> Result<Option<StoredWebAuthnCredential>, PersistenceError> {
    let query = format!(
        "SELECT {CREDENTIAL_COLUMNS} FROM webauthn_credentials wc WHERE wc.id=?"
    );
    let row = sqlx::query(&query)
        .bind(credential_id.to_string())
        .fetch_optional(pool)
        .await?;
    match row {
        Some(row) => Ok(Some(decode_sqlite_credential_row(pool, row).await?)),
        None => Ok(None),
    }
}

async fn credential_by_internal_id_postgres(
    pool: &PgPool,
    credential_id: EntityId,
) -> Result<Option<StoredWebAuthnCredential>, PersistenceError> {
    let query = format!(
        "SELECT {CREDENTIAL_COLUMNS} FROM webauthn_credentials wc WHERE wc.id=$1"
    );
    let row = sqlx::query(&query)
        .bind(credential_id.into_uuid())
        .fetch_optional(pool)
        .await?;
    match row {
        Some(row) => Ok(Some(decode_postgres_credential_row(pool, row).await?)),
        None => Ok(None),
    }
}

async fn commit_authentication_sqlite(
    pool: &SqlitePool,
    command: &WebAuthnAuthenticationCommit<'_>,
) -> Result<WebAuthnAuthenticationCommitOutcome, PersistenceError> {
    let (context_version, network_hmac, agent_hash) =
        sqlite_context_values(&command.binding.client_context);
    let mut transaction = pool.begin().await?;
    if !lock_authentication_claim_sqlite(&mut transaction, command.binding, command.now_ms)
        .await?
    {
        return Ok(WebAuthnAuthenticationCommitOutcome::Stale);
    }
    let updated = sqlx::query(
        "UPDATE webauthn_credentials SET sign_counter=?,backup_eligible=?,backup_state=?,material_key_version=?,material_nonce=?,material_ciphertext=?,material_aad_hash=?,last_used_at_ms=?,revision=revision+1 WHERE id=? AND user_id=? AND status='active' AND user_verified=1 AND revision=? AND sign_counter=? AND backup_eligible=? AND backup_state=? AND EXISTS(SELECT 1 FROM webauthn_ceremonies c JOIN auth_challenges ac ON ac.id=c.auth_challenge_id JOIN users u ON u.id=c.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE c.id=? AND c.kind='authentication' AND c.status='pending' AND c.user_id=webauthn_credentials.user_id AND c.session_id IS ? AND c.purpose=? AND c.rp_id=? AND c.origin=? AND c.auth_revision=? AND c.auth_challenge_id=? AND c.claim_id=? AND c.reserved_at_ms=? AND c.verification_expires_at_ms=? AND c.context_key_version IS ? AND c.client_network_hmac IS ? AND c.user_agent_hash IS ? AND c.revision=? AND c.created_at_ms<=? AND c.expires_at_ms>? AND ac.user_id=c.user_id AND ac.session_id IS c.session_id AND ac.purpose=c.purpose AND ac.auth_revision=c.auth_revision AND ac.status='verification_pending' AND ac.attempt_claim_id=c.claim_id AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=c.reserved_at_ms AND ac.attempt_expires_at_ms=c.verification_expires_at_ms AND ac.context_key_version IS c.context_key_version AND ac.client_network_hmac IS c.client_network_hmac AND ac.user_agent_hash IS c.user_agent_hash AND ac.expires_at_ms>? AND ac.attempt_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?)))",
    )
    .bind(i64::from(command.sign_counter))
    .bind(command.backup_eligible)
    .bind(command.backup_state)
    .bind(i64::from(command.material.key_version))
    .bind(command.material.nonce.as_slice())
    .bind(command.material.ciphertext.as_slice())
    .bind(command.material.aad_hash.as_slice())
    .bind(command.now_ms)
    .bind(command.credential_id.to_string())
    .bind(command.binding.user_id.to_string())
    .bind(database_revision(command.expected_credential_revision)?)
    .bind(i64::from(command.expected_sign_counter))
    .bind(command.expected_backup_eligible)
    .bind(command.expected_backup_state)
    .bind(command.ceremony_id.to_string())
    .bind(command.binding.session_id.map(|id| id.to_string()))
    .bind(command.binding.purpose.as_str())
    .bind(command.origin.rp_id())
    .bind(command.origin.as_str())
    .bind(database_revision(command.binding.auth_revision)?)
    .bind(command.binding.auth_challenge_id.to_string())
    .bind(command.binding.claim_id.to_string())
    .bind(command.binding.reserved_at_ms)
    .bind(command.binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(database_revision(command.expected_ceremony_revision)?)
    .bind(command.now_ms)
    .bind(command.now_ms)
    .bind(command.now_ms)
    .bind(command.now_ms)
    .bind(command.now_ms)
    .bind(command.now_ms)
    .bind(command.now_ms)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if updated != 1 {
        return Ok(WebAuthnAuthenticationCommitOutcome::Stale);
    }
    if command.backup_counter_anomaly {
        let audited = sqlx::query(
            "UPDATE webauthn_credentials SET backup_counter_anomaly_at_ms=? WHERE id=? AND backup_eligible=1",
        )
        .bind(command.now_ms)
        .bind(command.credential_id.to_string())
        .execute(&mut *transaction)
        .await?
        .rows_affected();
        if audited != 1 {
            return Ok(WebAuthnAuthenticationCommitOutcome::Stale);
        }
    }
    let consumed = sqlx::query(
        "UPDATE webauthn_ceremonies SET status='consumed',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=?,revision=revision+1 WHERE id=? AND kind='authentication' AND status='pending' AND revision=?",
    )
    .bind(command.now_ms)
    .bind(command.ceremony_id.to_string())
    .bind(database_revision(command.expected_ceremony_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if consumed != 1 {
        return Ok(WebAuthnAuthenticationCommitOutcome::Stale);
    }
    transaction.commit().await?;
    let stored = credential_by_internal_id_sqlite(pool, command.credential_id)
        .await?
        .ok_or(PersistenceError::InvalidStoredWebAuthnCredential)?;
    Ok(WebAuthnAuthenticationCommitOutcome::Committed(
        stored.credential,
    ))
}

async fn commit_authentication_postgres(
    pool: &PgPool,
    command: &WebAuthnAuthenticationCommit<'_>,
) -> Result<WebAuthnAuthenticationCommitOutcome, PersistenceError> {
    let (context_version, network_hmac, agent_hash) =
        postgres_context_values(&command.binding.client_context)?;
    let mut transaction = pool.begin().await?;
    if !lock_authentication_principal_postgres(
        &mut transaction,
        command.binding,
        command.now_ms,
    )
    .await?
    {
        return Ok(WebAuthnAuthenticationCommitOutcome::Stale);
    }
    if !lock_authentication_claim_postgres(&mut transaction, command.binding, command.now_ms)
        .await?
    {
        return Ok(WebAuthnAuthenticationCommitOutcome::Stale);
    }
    let updated = sqlx::query(
        "UPDATE webauthn_credentials SET sign_counter=$1,backup_eligible=$2,backup_state=$3,material_key_version=$4,material_nonce=$5,material_ciphertext=$6,material_aad_hash=$7,last_used_at_ms=$8,revision=revision+1 WHERE id=$9 AND user_id=$10 AND status='active' AND user_verified AND revision=$11 AND sign_counter=$12 AND backup_eligible=$13 AND backup_state=$14 AND EXISTS(SELECT 1 FROM webauthn_ceremonies c JOIN auth_challenges ac ON ac.id=c.auth_challenge_id JOIN users u ON u.id=c.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE c.id=$15 AND c.kind='authentication' AND c.status='pending' AND c.user_id=webauthn_credentials.user_id AND c.session_id IS NOT DISTINCT FROM $16 AND c.purpose=$17 AND c.rp_id=$18 AND c.origin=$19 AND c.auth_revision=$20 AND c.auth_challenge_id=$21 AND c.claim_id=$22 AND c.reserved_at_ms=$23 AND c.verification_expires_at_ms=$24 AND c.context_key_version IS NOT DISTINCT FROM $25 AND c.client_network_hmac IS NOT DISTINCT FROM $26 AND c.user_agent_hash IS NOT DISTINCT FROM $27 AND c.revision=$28 AND c.created_at_ms<=$29 AND c.expires_at_ms>$29 AND ac.user_id=c.user_id AND ac.session_id IS NOT DISTINCT FROM c.session_id AND ac.purpose=c.purpose AND ac.auth_revision=c.auth_revision AND ac.status='verification_pending' AND ac.attempt_claim_id=c.claim_id AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=c.reserved_at_ms AND ac.attempt_expires_at_ms=c.verification_expires_at_ms AND ac.context_key_version IS NOT DISTINCT FROM c.context_key_version AND ac.client_network_hmac IS NOT DISTINCT FROM c.client_network_hmac AND ac.user_agent_hash IS NOT DISTINCT FROM c.user_agent_hash AND ac.expires_at_ms>$29 AND ac.attempt_expires_at_ms>$29 AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=$29 AND s.idle_expires_at_ms>$29 AND s.absolute_expires_at_ms>$29)))",
    )
    .bind(i64::from(command.sign_counter))
    .bind(command.backup_eligible)
    .bind(command.backup_state)
    .bind(database_key_version(command.material.key_version)?)
    .bind(command.material.nonce.as_slice())
    .bind(command.material.ciphertext.as_slice())
    .bind(command.material.aad_hash.as_slice())
    .bind(command.now_ms)
    .bind(command.credential_id.into_uuid())
    .bind(command.binding.user_id.into_uuid())
    .bind(database_revision(command.expected_credential_revision)?)
    .bind(i64::from(command.expected_sign_counter))
    .bind(command.expected_backup_eligible)
    .bind(command.expected_backup_state)
    .bind(command.ceremony_id.into_uuid())
    .bind(command.binding.session_id.map(EntityId::into_uuid))
    .bind(command.binding.purpose.as_str())
    .bind(command.origin.rp_id())
    .bind(command.origin.as_str())
    .bind(database_revision(command.binding.auth_revision)?)
    .bind(command.binding.auth_challenge_id.into_uuid())
    .bind(command.binding.claim_id.into_uuid())
    .bind(command.binding.reserved_at_ms)
    .bind(command.binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(database_revision(command.expected_ceremony_revision)?)
    .bind(command.now_ms)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if updated != 1 {
        return Ok(WebAuthnAuthenticationCommitOutcome::Stale);
    }
    if command.backup_counter_anomaly {
        let audited = sqlx::query(
            "UPDATE webauthn_credentials SET backup_counter_anomaly_at_ms=$1 WHERE id=$2 AND backup_eligible",
        )
        .bind(command.now_ms)
        .bind(command.credential_id.into_uuid())
        .execute(&mut *transaction)
        .await?
        .rows_affected();
        if audited != 1 {
            return Ok(WebAuthnAuthenticationCommitOutcome::Stale);
        }
    }
    let consumed = sqlx::query(
        "UPDATE webauthn_ceremonies SET status='consumed',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=$1,revision=revision+1 WHERE id=$2 AND kind='authentication' AND status='pending' AND revision=$3",
    )
    .bind(command.now_ms)
    .bind(command.ceremony_id.into_uuid())
    .bind(database_revision(command.expected_ceremony_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if consumed != 1 {
        return Ok(WebAuthnAuthenticationCommitOutcome::Stale);
    }
    transaction.commit().await?;
    let stored = credential_by_internal_id_postgres(pool, command.credential_id)
        .await?
        .ok_or(PersistenceError::InvalidStoredWebAuthnCredential)?;
    Ok(WebAuthnAuthenticationCommitOutcome::Committed(
        stored.credential,
    ))
}

async fn reject_authentication_sqlite(
    pool: &SqlitePool,
    ceremony_id: EntityId,
    expected_ceremony_revision: Revision,
    binding: &WebAuthnChallengeBinding,
    origin: &WebAuthnOrigin,
    now_ms: i64,
) -> Result<bool, PersistenceError> {
    let (context_version, network_hmac, agent_hash) =
        sqlite_context_values(&binding.client_context);
    let mut transaction = pool.begin().await?;
    if !lock_authentication_claim_sqlite(&mut transaction, binding, now_ms).await? {
        return Ok(false);
    }
    let affected = sqlx::query(
        "UPDATE webauthn_ceremonies SET status='rejected',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=?,revision=revision+1 WHERE id=? AND kind='authentication' AND status='pending' AND user_id=? AND session_id IS ? AND purpose=? AND rp_id=? AND origin=? AND auth_revision=? AND auth_challenge_id=? AND claim_id=? AND reserved_at_ms=? AND verification_expires_at_ms=? AND context_key_version IS ? AND client_network_hmac IS ? AND user_agent_hash IS ? AND revision=? AND created_at_ms<=? AND expires_at_ms>? AND EXISTS(SELECT 1 FROM auth_challenges ac WHERE ac.id=webauthn_ceremonies.auth_challenge_id AND ac.status='verification_pending' AND ac.attempt_claim_id=webauthn_ceremonies.claim_id AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=webauthn_ceremonies.reserved_at_ms AND ac.attempt_expires_at_ms=webauthn_ceremonies.verification_expires_at_ms AND ac.expires_at_ms>? AND ac.attempt_expires_at_ms>?)",
    )
    .bind(now_ms)
    .bind(ceremony_id.to_string())
    .bind(binding.user_id.to_string())
    .bind(binding.session_id.map(|id| id.to_string()))
    .bind(binding.purpose.as_str())
    .bind(origin.rp_id())
    .bind(origin.as_str())
    .bind(database_revision(binding.auth_revision)?)
    .bind(binding.auth_challenge_id.to_string())
    .bind(binding.claim_id.to_string())
    .bind(binding.reserved_at_ms)
    .bind(binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(database_revision(expected_ceremony_revision)?)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if affected != 1 {
        return Ok(false);
    }
    transaction.commit().await?;
    Ok(true)
}

async fn reject_authentication_postgres(
    pool: &PgPool,
    ceremony_id: EntityId,
    expected_ceremony_revision: Revision,
    binding: &WebAuthnChallengeBinding,
    origin: &WebAuthnOrigin,
    now_ms: i64,
) -> Result<bool, PersistenceError> {
    let (context_version, network_hmac, agent_hash) =
        postgres_context_values(&binding.client_context)?;
    let mut transaction = pool.begin().await?;
    if !lock_authentication_principal_postgres(&mut transaction, binding, now_ms).await? {
        return Ok(false);
    }
    if !lock_authentication_claim_postgres(&mut transaction, binding, now_ms).await? {
        return Ok(false);
    }
    let affected = sqlx::query(
        "UPDATE webauthn_ceremonies SET status='rejected',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=$1,revision=revision+1 WHERE id=$2 AND kind='authentication' AND status='pending' AND user_id=$3 AND session_id IS NOT DISTINCT FROM $4 AND purpose=$5 AND rp_id=$6 AND origin=$7 AND auth_revision=$8 AND auth_challenge_id=$9 AND claim_id=$10 AND reserved_at_ms=$11 AND verification_expires_at_ms=$12 AND context_key_version IS NOT DISTINCT FROM $13 AND client_network_hmac IS NOT DISTINCT FROM $14 AND user_agent_hash IS NOT DISTINCT FROM $15 AND revision=$16 AND created_at_ms<=$1 AND expires_at_ms>$1 AND EXISTS(SELECT 1 FROM auth_challenges ac WHERE ac.id=webauthn_ceremonies.auth_challenge_id AND ac.status='verification_pending' AND ac.attempt_claim_id=webauthn_ceremonies.claim_id AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=webauthn_ceremonies.reserved_at_ms AND ac.attempt_expires_at_ms=webauthn_ceremonies.verification_expires_at_ms AND ac.expires_at_ms>$1 AND ac.attempt_expires_at_ms>$1)",
    )
    .bind(now_ms)
    .bind(ceremony_id.into_uuid())
    .bind(binding.user_id.into_uuid())
    .bind(binding.session_id.map(EntityId::into_uuid))
    .bind(binding.purpose.as_str())
    .bind(origin.rp_id())
    .bind(origin.as_str())
    .bind(database_revision(binding.auth_revision)?)
    .bind(binding.auth_challenge_id.into_uuid())
    .bind(binding.claim_id.into_uuid())
    .bind(binding.reserved_at_ms)
    .bind(binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(database_revision(expected_ceremony_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if affected != 1 {
        return Ok(false);
    }
    transaction.commit().await?;
    Ok(true)
}

async fn clone_suspected_sqlite(
    pool: &SqlitePool,
    command: &WebAuthnCloneSuspected<'_>,
) -> Result<WebAuthnCloneSuspectedOutcome, PersistenceError> {
    let (context_version, network_hmac, agent_hash) =
        sqlite_context_values(&command.binding.client_context);
    let mut transaction = pool.begin().await?;
    if !lock_authentication_claim_sqlite(&mut transaction, command.binding, command.now_ms)
        .await?
    {
        return Ok(WebAuthnCloneSuspectedOutcome::Stale);
    }
    let credential_changed = sqlx::query(
        "UPDATE webauthn_credentials SET status='clone_suspected',clone_suspected_at_ms=?,revision=revision+1 WHERE id=? AND user_id=? AND status='active' AND backup_eligible=0 AND revision=? AND sign_counter=? AND EXISTS(SELECT 1 FROM webauthn_ceremonies c JOIN auth_challenges ac ON ac.id=c.auth_challenge_id JOIN users u ON u.id=c.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE c.id=? AND c.kind='authentication' AND c.status='pending' AND c.user_id=webauthn_credentials.user_id AND c.session_id IS ? AND c.purpose=? AND c.rp_id=? AND c.origin=? AND c.auth_revision=? AND c.auth_challenge_id=? AND c.claim_id=? AND c.reserved_at_ms=? AND c.verification_expires_at_ms=? AND c.context_key_version IS ? AND c.client_network_hmac IS ? AND c.user_agent_hash IS ? AND c.revision=? AND c.created_at_ms<=? AND c.expires_at_ms>? AND ac.user_id=c.user_id AND ac.session_id IS c.session_id AND ac.purpose=c.purpose AND ac.auth_revision=c.auth_revision AND ac.status='verification_pending' AND ac.attempt_claim_id=c.claim_id AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=c.reserved_at_ms AND ac.attempt_expires_at_ms=c.verification_expires_at_ms AND ac.context_key_version IS c.context_key_version AND ac.client_network_hmac IS c.client_network_hmac AND ac.user_agent_hash IS c.user_agent_hash AND ac.expires_at_ms>? AND ac.attempt_expires_at_ms>? AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?)))",
    )
    .bind(command.now_ms)
    .bind(command.credential_id.to_string())
    .bind(command.binding.user_id.to_string())
    .bind(database_revision(command.expected_credential_revision)?)
    .bind(i64::from(command.expected_sign_counter))
    .bind(command.ceremony_id.to_string())
    .bind(command.binding.session_id.map(|id| id.to_string()))
    .bind(command.binding.purpose.as_str())
    .bind(command.origin.rp_id())
    .bind(command.origin.as_str())
    .bind(database_revision(command.binding.auth_revision)?)
    .bind(command.binding.auth_challenge_id.to_string())
    .bind(command.binding.claim_id.to_string())
    .bind(command.binding.reserved_at_ms)
    .bind(command.binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(database_revision(command.expected_ceremony_revision)?)
    .bind(command.now_ms)
    .bind(command.now_ms)
    .bind(command.now_ms)
    .bind(command.now_ms)
    .bind(command.now_ms)
    .bind(command.now_ms)
    .bind(command.now_ms)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if credential_changed != 1 {
        transaction.rollback().await?;
        return Ok(WebAuthnCloneSuspectedOutcome::Stale);
    }
    let ceremony_burned = sqlx::query(
        "UPDATE webauthn_ceremonies SET status='rejected',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=?,revision=revision+1 WHERE id=? AND kind='authentication' AND status='pending' AND revision=?",
    )
    .bind(command.now_ms)
    .bind(command.ceremony_id.to_string())
    .bind(database_revision(command.expected_ceremony_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if ceremony_burned != 1 {
        transaction.rollback().await?;
        return Ok(WebAuthnCloneSuspectedOutcome::Stale);
    }
    let challenge_burned = sqlx::query(
        "UPDATE auth_challenges SET status='invalidated',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=? WHERE id=? AND user_id=? AND session_id IS ? AND purpose=? AND auth_revision=? AND status='verification_pending' AND attempt_claim_id=? AND attempted_method='webauthn'",
    )
    .bind(command.now_ms)
    .bind(command.binding.auth_challenge_id.to_string())
    .bind(command.binding.user_id.to_string())
    .bind(command.binding.session_id.map(|id| id.to_string()))
    .bind(command.binding.purpose.as_str())
    .bind(database_revision(command.binding.auth_revision)?)
    .bind(command.binding.claim_id.to_string())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if challenge_burned != 1 {
        transaction.rollback().await?;
        return Ok(WebAuthnCloneSuspectedOutcome::Stale);
    }
    let next = command
        .binding
        .auth_revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let auth_changed = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=?,updated_at_ms=? WHERE user_id=? AND auth_revision=?",
    )
    .bind(database_revision(next)?)
    .bind(command.now_ms)
    .bind(command.binding.user_id.to_string())
    .bind(database_revision(command.binding.auth_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if auth_changed != 1 {
        transaction.rollback().await?;
        return Ok(WebAuthnCloneSuspectedOutcome::Stale);
    }
    let revoked_sessions = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=?,revoked_reason='security_policy',revision=revision+1 WHERE user_id=? AND status='active' AND auth_revision=?",
    )
    .bind(command.now_ms)
    .bind(command.binding.user_id.to_string())
    .bind(database_revision(command.binding.auth_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    sqlx::query(
        "UPDATE auth_challenges SET status='invalidated',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=? WHERE user_id=? AND auth_revision=? AND status IN ('pending','verification_pending','rotation_pending','exhausted')",
    )
    .bind(command.now_ms)
    .bind(command.binding.user_id.to_string())
    .bind(database_revision(command.binding.auth_revision)?)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(WebAuthnCloneSuspectedOutcome::Recorded {
        auth_revision: next,
        revoked_sessions,
    })
}

async fn clone_suspected_postgres(
    pool: &PgPool,
    command: &WebAuthnCloneSuspected<'_>,
) -> Result<WebAuthnCloneSuspectedOutcome, PersistenceError> {
    let (context_version, network_hmac, agent_hash) =
        postgres_context_values(&command.binding.client_context)?;
    let mut transaction = pool.begin().await?;
    if !lock_authentication_principal_postgres(
        &mut transaction,
        command.binding,
        command.now_ms,
    )
    .await?
    {
        return Ok(WebAuthnCloneSuspectedOutcome::Stale);
    }
    lock_open_user_challenges_postgres(
        &mut transaction,
        command.binding.user_id,
        command.binding.auth_revision,
    )
    .await?;
    if !lock_authentication_claim_postgres(&mut transaction, command.binding, command.now_ms)
        .await?
    {
        return Ok(WebAuthnCloneSuspectedOutcome::Stale);
    }
    let credential_changed = sqlx::query(
        "UPDATE webauthn_credentials SET status='clone_suspected',clone_suspected_at_ms=$1,revision=revision+1 WHERE id=$2 AND user_id=$3 AND status='active' AND NOT backup_eligible AND revision=$4 AND sign_counter=$5 AND EXISTS(SELECT 1 FROM webauthn_ceremonies c JOIN auth_challenges ac ON ac.id=c.auth_challenge_id JOIN users u ON u.id=c.user_id JOIN user_auth_state uas ON uas.user_id=u.id WHERE c.id=$6 AND c.kind='authentication' AND c.status='pending' AND c.user_id=webauthn_credentials.user_id AND c.session_id IS NOT DISTINCT FROM $7 AND c.purpose=$8 AND c.rp_id=$9 AND c.origin=$10 AND c.auth_revision=$11 AND c.auth_challenge_id=$12 AND c.claim_id=$13 AND c.reserved_at_ms=$14 AND c.verification_expires_at_ms=$15 AND c.context_key_version IS NOT DISTINCT FROM $16 AND c.client_network_hmac IS NOT DISTINCT FROM $17 AND c.user_agent_hash IS NOT DISTINCT FROM $18 AND c.revision=$19 AND c.created_at_ms<=$20 AND c.expires_at_ms>$20 AND ac.user_id=c.user_id AND ac.session_id IS NOT DISTINCT FROM c.session_id AND ac.purpose=c.purpose AND ac.auth_revision=c.auth_revision AND ac.status='verification_pending' AND ac.attempt_claim_id=c.claim_id AND ac.attempted_method='webauthn' AND ac.attempt_started_at_ms=c.reserved_at_ms AND ac.attempt_expires_at_ms=c.verification_expires_at_ms AND ac.context_key_version IS NOT DISTINCT FROM c.context_key_version AND ac.client_network_hmac IS NOT DISTINCT FROM c.client_network_hmac AND ac.user_agent_hash IS NOT DISTINCT FROM c.user_agent_hash AND ac.expires_at_ms>$20 AND ac.attempt_expires_at_ms>$20 AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=ac.auth_revision AND (ac.session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions s WHERE s.id=ac.session_id AND s.user_id=ac.user_id AND s.status='active' AND s.auth_revision=ac.auth_revision AND s.last_seen_at_ms<=$20 AND s.idle_expires_at_ms>$20 AND s.absolute_expires_at_ms>$20)))",
    )
    .bind(command.now_ms)
    .bind(command.credential_id.into_uuid())
    .bind(command.binding.user_id.into_uuid())
    .bind(database_revision(command.expected_credential_revision)?)
    .bind(i64::from(command.expected_sign_counter))
    .bind(command.ceremony_id.into_uuid())
    .bind(command.binding.session_id.map(EntityId::into_uuid))
    .bind(command.binding.purpose.as_str())
    .bind(command.origin.rp_id())
    .bind(command.origin.as_str())
    .bind(database_revision(command.binding.auth_revision)?)
    .bind(command.binding.auth_challenge_id.into_uuid())
    .bind(command.binding.claim_id.into_uuid())
    .bind(command.binding.reserved_at_ms)
    .bind(command.binding.verification_expires_at_ms)
    .bind(context_version)
    .bind(network_hmac)
    .bind(agent_hash)
    .bind(database_revision(command.expected_ceremony_revision)?)
    .bind(command.now_ms)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if credential_changed != 1 {
        transaction.rollback().await?;
        return Ok(WebAuthnCloneSuspectedOutcome::Stale);
    }
    let ceremony_burned = sqlx::query(
        "UPDATE webauthn_ceremonies SET status='rejected',state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=$1,revision=revision+1 WHERE id=$2 AND kind='authentication' AND status='pending' AND revision=$3",
    )
    .bind(command.now_ms)
    .bind(command.ceremony_id.into_uuid())
    .bind(database_revision(command.expected_ceremony_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if ceremony_burned != 1 {
        transaction.rollback().await?;
        return Ok(WebAuthnCloneSuspectedOutcome::Stale);
    }
    let challenge_burned = sqlx::query(
        "UPDATE auth_challenges SET status='invalidated',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=$1 WHERE id=$2 AND user_id=$3 AND session_id IS NOT DISTINCT FROM $4 AND purpose=$5 AND auth_revision=$6 AND status='verification_pending' AND attempt_claim_id=$7 AND attempted_method='webauthn'",
    )
    .bind(command.now_ms)
    .bind(command.binding.auth_challenge_id.into_uuid())
    .bind(command.binding.user_id.into_uuid())
    .bind(command.binding.session_id.map(EntityId::into_uuid))
    .bind(command.binding.purpose.as_str())
    .bind(database_revision(command.binding.auth_revision)?)
    .bind(command.binding.claim_id.into_uuid())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if challenge_burned != 1 {
        transaction.rollback().await?;
        return Ok(WebAuthnCloneSuspectedOutcome::Stale);
    }
    let next = command
        .binding
        .auth_revision
        .next()
        .map_err(|_| PersistenceError::RevisionOutOfRange)?;
    let auth_changed = sqlx::query(
        "UPDATE user_auth_state SET auth_revision=$1,updated_at_ms=$2 WHERE user_id=$3 AND auth_revision=$4",
    )
    .bind(database_revision(next)?)
    .bind(command.now_ms)
    .bind(command.binding.user_id.into_uuid())
    .bind(database_revision(command.binding.auth_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if auth_changed != 1 {
        transaction.rollback().await?;
        return Ok(WebAuthnCloneSuspectedOutcome::Stale);
    }
    let revoked_sessions = sqlx::query(
        "UPDATE auth_sessions SET status='revoked',revoked_at_ms=$1,revoked_reason='security_policy',revision=revision+1 WHERE user_id=$2 AND status='active' AND auth_revision=$3",
    )
    .bind(command.now_ms)
    .bind(command.binding.user_id.into_uuid())
    .bind(database_revision(command.binding.auth_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    sqlx::query(
        "UPDATE auth_challenges SET status='invalidated',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=$1 WHERE user_id=$2 AND auth_revision=$3 AND status IN ('pending','verification_pending','rotation_pending','exhausted')",
    )
    .bind(command.now_ms)
    .bind(command.binding.user_id.into_uuid())
    .bind(database_revision(command.binding.auth_revision)?)
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;
    Ok(WebAuthnCloneSuspectedOutcome::Recorded {
        auth_revision: next,
        revoked_sessions,
    })
}

async fn rename_credential_sqlite(
    pool: &SqlitePool,
    command: &RenameWebAuthnCredential<'_>,
) -> Result<Option<WebAuthnCredential>, PersistenceError> {
    let mut transaction = pool.begin().await?;
    lock_guard_sqlite(&mut transaction, &command.guard).await?;
    let changed = sqlx::query(
        "UPDATE webauthn_credentials SET nickname=?,revision=revision+1 WHERE id=? AND user_id=? AND status='active' AND revision=?",
    )
    .bind(command.nickname.as_str())
    .bind(command.credential_id.to_string())
    .bind(command.guard.user_id.to_string())
    .bind(database_revision(command.expected_credential_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if changed != 1 {
        transaction.rollback().await?;
        return Ok(None);
    }
    transaction.commit().await?;
    let stored = credential_by_internal_id_sqlite(pool, command.credential_id)
        .await?
        .ok_or(PersistenceError::InvalidStoredWebAuthnCredential)?;
    Ok(Some(stored.credential))
}

async fn rename_credential_postgres(
    pool: &PgPool,
    command: &RenameWebAuthnCredential<'_>,
) -> Result<Option<WebAuthnCredential>, PersistenceError> {
    let mut transaction = pool.begin().await?;
    lock_guard_postgres(&mut transaction, &command.guard).await?;
    let changed = sqlx::query(
        "UPDATE webauthn_credentials SET nickname=$1,revision=revision+1 WHERE id=$2 AND user_id=$3 AND status='active' AND revision=$4",
    )
    .bind(command.nickname.as_str())
    .bind(command.credential_id.into_uuid())
    .bind(command.guard.user_id.into_uuid())
    .bind(database_revision(command.expected_credential_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if changed != 1 {
        transaction.rollback().await?;
        return Ok(None);
    }
    transaction.commit().await?;
    let stored = credential_by_internal_id_postgres(pool, command.credential_id)
        .await?
        .ok_or(PersistenceError::InvalidStoredWebAuthnCredential)?;
    Ok(Some(stored.credential))
}

async fn revoke_credential_sqlite(
    pool: &SqlitePool,
    command: &RevokeWebAuthnCredential,
) -> Result<RevokeWebAuthnCredentialOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    lock_guard_sqlite(&mut transaction, &command.guard).await?;
    // Match the authentication-terminal lock order: principal guard, challenges, credential,
    // ceremony, then auth/session effects. A stale credential rolls this invalidation back.
    sqlx::query(
        "UPDATE auth_challenges SET status='invalidated',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=? WHERE user_id=? AND auth_revision=? AND status IN ('pending','verification_pending','rotation_pending','exhausted')",
    )
    .bind(command.guard.now_ms)
    .bind(command.guard.user_id.to_string())
    .bind(database_revision(command.guard.expected_auth_revision)?)
    .execute(&mut *transaction)
    .await?;
    let changed = sqlx::query(
        "UPDATE webauthn_credentials SET status='revoked',revoked_at_ms=?,revision=revision+1 WHERE id=? AND user_id=? AND status='active' AND revision=?",
    )
    .bind(command.guard.now_ms)
    .bind(command.credential_id.to_string())
    .bind(command.guard.user_id.to_string())
    .bind(database_revision(command.expected_credential_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if changed != 1 {
        transaction.rollback().await?;
        return Ok(RevokeWebAuthnCredentialOutcome::Stale);
    }
    sqlx::query(
        "UPDATE webauthn_ceremonies SET status=CASE WHEN expires_at_ms<=? THEN 'expired' ELSE 'rejected' END,state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=CASE WHEN expires_at_ms<=? THEN expires_at_ms ELSE ? END,revision=revision+1 WHERE user_id=? AND status='pending'",
    )
    .bind(command.guard.now_ms)
    .bind(command.guard.now_ms)
    .bind(command.guard.now_ms)
    .bind(command.guard.user_id.to_string())
    .execute(&mut *transaction)
    .await?;
    let (auth_revision, revoked_other_sessions) =
        advance_management_auth_state_sqlite(&mut transaction, &command.guard).await?;
    transaction.commit().await?;
    Ok(RevokeWebAuthnCredentialOutcome::Revoked {
        auth_revision,
        revoked_other_sessions,
    })
}

async fn revoke_credential_postgres(
    pool: &PgPool,
    command: &RevokeWebAuthnCredential,
) -> Result<RevokeWebAuthnCredentialOutcome, PersistenceError> {
    let mut transaction = pool.begin().await?;
    lock_guard_postgres(&mut transaction, &command.guard).await?;
    lock_open_user_challenges_postgres(
        &mut transaction,
        command.guard.user_id,
        command.guard.expected_auth_revision,
    )
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET status='invalidated',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=$1 WHERE user_id=$2 AND auth_revision=$3 AND status IN ('pending','verification_pending','rotation_pending','exhausted')",
    )
    .bind(command.guard.now_ms)
    .bind(command.guard.user_id.into_uuid())
    .bind(database_revision(command.guard.expected_auth_revision)?)
    .execute(&mut *transaction)
    .await?;
    let changed = sqlx::query(
        "UPDATE webauthn_credentials SET status='revoked',revoked_at_ms=$1,revision=revision+1 WHERE id=$2 AND user_id=$3 AND status='active' AND revision=$4",
    )
    .bind(command.guard.now_ms)
    .bind(command.credential_id.into_uuid())
    .bind(command.guard.user_id.into_uuid())
    .bind(database_revision(command.expected_credential_revision)?)
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if changed != 1 {
        transaction.rollback().await?;
        return Ok(RevokeWebAuthnCredentialOutcome::Stale);
    }
    sqlx::query(
        "UPDATE webauthn_ceremonies SET status=CASE WHEN expires_at_ms<=$1 THEN 'expired' ELSE 'rejected' END,state_schema_version=NULL,state_key_version=NULL,state_nonce=NULL,state_ciphertext=NULL,state_aad_hash=NULL,finished_at_ms=CASE WHEN expires_at_ms<=$1 THEN expires_at_ms ELSE $1 END,revision=revision+1 WHERE user_id=$2 AND status='pending'",
    )
    .bind(command.guard.now_ms)
    .bind(command.guard.user_id.into_uuid())
    .execute(&mut *transaction)
    .await?;
    let (auth_revision, revoked_other_sessions) =
        advance_management_auth_state_postgres(&mut transaction, &command.guard).await?;
    transaction.commit().await?;
    Ok(RevokeWebAuthnCredentialOutcome::Revoked {
        auth_revision,
        revoked_other_sessions,
    })
}
