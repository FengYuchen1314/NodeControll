use nodecontroll_domain::{
    AuthChallenge, AuthChallengePurpose, AuthChallengeRotationState, AuthChallengeStatus,
    AuthenticationAssurance, AuthenticationMethod, EntityId, Revision,
};
use nodecontroll_secrets::KeyedDigest;
use sqlx::FromRow;

use super::{AuthHmac, Database, PersistenceError, database_key_version, database_revision};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthChallengeClientContext {
    pub key_version: Option<u32>,
    pub client_network_hmac: Option<AuthHmac>,
    pub user_agent_hash: Option<AuthHmac>,
}

impl AuthChallengeClientContext {
    #[must_use]
    pub const fn unbound() -> Self {
        Self {
            key_version: None,
            client_network_hmac: None,
            user_agent_hash: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewAuthChallenge {
    pub id: EntityId,
    pub token_key_version: u32,
    pub token_hmac: AuthHmac,
    pub purpose: AuthChallengePurpose,
    pub user_id: EntityId,
    pub session_id: Option<EntityId>,
    pub auth_revision: Revision,
    pub allowed_methods: Vec<AuthenticationMethod>,
    pub max_attempts: u32,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
    pub rotation_required: bool,
    pub client_context: AuthChallengeClientContext,
    pub revision: Revision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CreateAuthChallengeOutcome {
    Created(AuthChallenge),
    AlreadyOpen,
    PrincipalUnavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthChallengeTokenLookup {
    pub id: EntityId,
    pub client_context: AuthChallengeClientContext,
    pub now_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthChallengeAccess {
    pub id: EntityId,
    pub token_key_version: u32,
    pub token_hmac: AuthHmac,
    pub client_context: AuthChallengeClientContext,
    pub now_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthChallengeAttemptReservation {
    pub access: AuthChallengeAccess,
    pub claim_id: EntityId,
    pub method: AuthenticationMethod,
    pub expected_revision: Revision,
    pub verification_expires_at_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthChallengeAttemptReservationOutcome {
    Reserved(AuthChallenge),
    Stale,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthChallengeAttemptFailure {
    pub access: AuthChallengeAccess,
    pub claim_id: EntityId,
    pub method: AuthenticationMethod,
    pub expected_revision: Revision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthChallengeAttemptOutcome {
    Retryable(AuthChallenge),
    Exhausted(AuthChallenge),
    Stale,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthChallengeConsumption {
    pub access: AuthChallengeAccess,
    pub claim_id: EntityId,
    pub method: AuthenticationMethod,
    pub achieved_assurance: AuthenticationAssurance,
    pub expected_revision: Revision,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthChallengeConsumptionOutcome {
    Consumed(AuthChallenge),
    RotationPending(AuthChallenge),
    Stale,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthChallengeRotationReservation {
    pub access: AuthChallengeAccess,
    pub claim_id: EntityId,
    pub expected_revision: Revision,
    pub transaction_expires_at_ms: i64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthChallengeRotationReservationOutcome {
    Reserved(AuthChallenge),
    Stale,
}

#[derive(Debug, FromRow)]
struct ChallengeRow {
    id: String,
    purpose: String,
    user_id: String,
    session_id: Option<String>,
    auth_revision: i64,
    status: String,
    rotation_state: String,
    attempts_used: i64,
    max_attempts: i64,
    created_at_ms: i64,
    expires_at_ms: i64,
    verified_method: Option<String>,
    achieved_assurance: Option<String>,
    consumed_at_ms: Option<i64>,
    has_attempt_claim: i64,
    has_client_network_context: i64,
    has_user_agent_context: i64,
    revision: i64,
}

#[derive(Debug, FromRow)]
struct TokenDigestRow {
    token_key_version: i32,
    token_hmac: Vec<u8>,
}

impl Database {
    pub async fn create_auth_challenge(
        &self,
        challenge: &NewAuthChallenge,
    ) -> Result<CreateAuthChallengeOutcome, PersistenceError> {
        validate_new_challenge(challenge)?;
        let token_key_version = database_key_version(challenge.token_key_version)?;
        let auth_revision = database_revision(challenge.auth_revision)?;
        let max_attempts = i32::try_from(challenge.max_attempts)
            .map_err(|_| PersistenceError::InvalidAuthChallenge)?;
        let revision = database_revision(challenge.revision)?;
        let context_key_version = challenge
            .client_context
            .key_version
            .map(database_key_version)
            .transpose()?;
        let rotation_state = if challenge.rotation_required {
            AuthChallengeRotationState::Required
        } else {
            AuthChallengeRotationState::NotRequired
        };
        let methods = canonical_methods(&challenge.allowed_methods)?;

        let rejected = match self {
            Self::Sqlite(pool) => {
                let mut transaction = pool.begin().await?;
                refresh_user_purpose_sqlite(
                    &mut transaction,
                    challenge.user_id,
                    challenge.purpose,
                    challenge.created_at_ms,
                )
                .await?;
                let session_id = challenge.session_id.map(|id| id.to_string());
                let result = sqlx::query(
                    "INSERT INTO auth_challenges (id,token_key_version,token_hmac,purpose,user_id,session_id,auth_revision,status,rotation_state,attempts_used,max_attempts,created_at_ms,expires_at_ms,attempt_claim_id,attempted_method,attempt_started_at_ms,attempt_expires_at_ms,verified_method,achieved_assurance,consumed_at_ms,context_key_version,client_network_hmac,user_agent_hash,revision,updated_at_ms) SELECT ?,?,?,?,?,?,?,'pending',?,0,?,?,?,NULL,NULL,NULL,NULL,NULL,NULL,NULL,?,?,?,?,? FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=? AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=? AND (? IS NULL OR EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=? AND s.user_id=u.id AND s.status='active' AND s.auth_revision=uas.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?)) ON CONFLICT DO NOTHING",
                )
                .bind(challenge.id.to_string())
                .bind(token_key_version)
                .bind(challenge.token_hmac.as_slice())
                .bind(challenge.purpose.as_str())
                .bind(challenge.user_id.to_string())
                .bind(session_id.clone())
                .bind(auth_revision)
                .bind(rotation_state.as_str())
                .bind(max_attempts)
                .bind(challenge.created_at_ms)
                .bind(challenge.expires_at_ms)
                .bind(context_key_version)
                .bind(
                    challenge
                        .client_context
                        .client_network_hmac
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(
                    challenge
                        .client_context
                        .user_agent_hash
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(revision)
                .bind(challenge.created_at_ms)
                .bind(challenge.user_id.to_string())
                .bind(auth_revision)
                .bind(session_id.clone())
                .bind(session_id)
                .bind(challenge.created_at_ms)
                .bind(challenge.created_at_ms)
                .bind(challenge.created_at_ms)
                .execute(&mut *transaction)
                .await?;
                if result.rows_affected() == 1 {
                    for method in &methods {
                        sqlx::query(
                            "INSERT INTO auth_challenge_methods (challenge_id,method) VALUES (?,?)",
                        )
                        .bind(challenge.id.to_string())
                        .bind(method.as_str())
                        .execute(&mut *transaction)
                        .await?;
                    }
                    transaction.commit().await?;
                    None
                } else {
                    let open: Option<i64> = sqlx::query_scalar(
                        "SELECT 1 FROM auth_challenges WHERE user_id=? AND purpose=? AND status IN ('pending','verification_pending','rotation_pending','exhausted') LIMIT 1"
                    )
                    .bind(challenge.user_id.to_string())
                    .bind(challenge.purpose.as_str())
                    .fetch_optional(&mut *transaction)
                    .await?;
                    transaction.commit().await?;
                    Some(if open.is_some() {
                        CreateAuthChallengeOutcome::AlreadyOpen
                    } else {
                        CreateAuthChallengeOutcome::PrincipalUnavailable
                    })
                }
            }
            Self::Postgres(pool) => {
                let mut transaction = pool.begin().await?;
                refresh_user_purpose_postgres(
                    &mut transaction,
                    challenge.user_id,
                    challenge.purpose,
                    challenge.created_at_ms,
                )
                .await?;
                let session_id = challenge.session_id.map(EntityId::into_uuid);
                let result = sqlx::query(
                    "INSERT INTO auth_challenges (id,token_key_version,token_hmac,purpose,user_id,session_id,auth_revision,status,rotation_state,attempts_used,max_attempts,created_at_ms,expires_at_ms,attempt_claim_id,attempted_method,attempt_started_at_ms,attempt_expires_at_ms,verified_method,achieved_assurance,consumed_at_ms,context_key_version,client_network_hmac,user_agent_hash,revision,updated_at_ms) SELECT $1,$2,$3,$4,$5,$6,$7,'pending',$8,0,$9,$10,$11,NULL,NULL,NULL,NULL,NULL,NULL,NULL,$12,$13,$14,$15,$16 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=$17 AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=$18 AND ($19::uuid IS NULL OR EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=$20 AND s.user_id=u.id AND s.status='active' AND s.auth_revision=uas.auth_revision AND s.last_seen_at_ms<=$21 AND s.idle_expires_at_ms>$22 AND s.absolute_expires_at_ms>$23)) ON CONFLICT DO NOTHING",
                )
                .bind(challenge.id.into_uuid())
                .bind(token_key_version)
                .bind(challenge.token_hmac.as_slice())
                .bind(challenge.purpose.as_str())
                .bind(challenge.user_id.into_uuid())
                .bind(session_id)
                .bind(auth_revision)
                .bind(rotation_state.as_str())
                .bind(max_attempts)
                .bind(challenge.created_at_ms)
                .bind(challenge.expires_at_ms)
                .bind(context_key_version)
                .bind(
                    challenge
                        .client_context
                        .client_network_hmac
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(
                    challenge
                        .client_context
                        .user_agent_hash
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(revision)
                .bind(challenge.created_at_ms)
                .bind(challenge.user_id.into_uuid())
                .bind(auth_revision)
                .bind(session_id)
                .bind(session_id)
                .bind(challenge.created_at_ms)
                .bind(challenge.created_at_ms)
                .bind(challenge.created_at_ms)
                .execute(&mut *transaction)
                .await?;
                if result.rows_affected() == 1 {
                    for method in &methods {
                        sqlx::query(
                            "INSERT INTO auth_challenge_methods (challenge_id,method) VALUES ($1,$2)",
                        )
                        .bind(challenge.id.into_uuid())
                        .bind(method.as_str())
                        .execute(&mut *transaction)
                        .await?;
                    }
                    transaction.commit().await?;
                    None
                } else {
                    let open: Option<i64> = sqlx::query_scalar(
                        "SELECT 1::BIGINT FROM auth_challenges WHERE user_id=$1 AND purpose=$2 AND status IN ('pending','verification_pending','rotation_pending','exhausted') LIMIT 1"
                    )
                    .bind(challenge.user_id.into_uuid())
                    .bind(challenge.purpose.as_str())
                    .fetch_optional(&mut *transaction)
                    .await?;
                    transaction.commit().await?;
                    Some(if open.is_some() {
                        CreateAuthChallengeOutcome::AlreadyOpen
                    } else {
                        CreateAuthChallengeOutcome::PrincipalUnavailable
                    })
                }
            }
        };

        if let Some(rejected) = rejected {
            return Ok(rejected);
        }
        let access = AuthChallengeAccess {
            id: challenge.id,
            token_key_version: challenge.token_key_version,
            token_hmac: challenge.token_hmac,
            client_context: challenge.client_context.clone(),
            now_ms: challenge.created_at_ms,
        };
        self.auth_challenge(&access)
            .await?
            .map(CreateAuthChallengeOutcome::Created)
            .ok_or(PersistenceError::InvalidStoredAuthChallenge)
    }

    pub async fn auth_challenge_token_digest(
        &self,
        lookup: &AuthChallengeTokenLookup,
    ) -> Result<Option<KeyedDigest>, PersistenceError> {
        validate_lookup(lookup)?;
        let context_key_version = lookup
            .client_context
            .key_version
            .map(database_key_version)
            .transpose()?;
        let row = match self {
            Self::Sqlite(pool) => sqlx::query_as::<_, TokenDigestRow>(
                "SELECT token_key_version,token_hmac FROM auth_challenges WHERE id=? AND context_key_version IS ? AND client_network_hmac IS ? AND user_agent_hash IS ? AND status IN ('pending','verification_pending','rotation_pending','exhausted') AND created_at_ms<=? AND expires_at_ms>? AND EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) AND (session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?))"
            )
            .bind(lookup.id.to_string())
            .bind(context_key_version)
            .bind(
                lookup
                    .client_context
                    .client_network_hmac
                    .as_ref()
                    .map(|value| value.as_slice()),
            )
            .bind(
                lookup
                    .client_context
                    .user_agent_hash
                    .as_ref()
                    .map(|value| value.as_slice()),
            )
            .bind(lookup.now_ms)
            .bind(lookup.now_ms)
            .bind(lookup.now_ms)
            .bind(lookup.now_ms)
            .bind(lookup.now_ms)
            .fetch_optional(pool)
            .await?,
            Self::Postgres(pool) => sqlx::query_as::<_, TokenDigestRow>(
                "SELECT token_key_version,token_hmac FROM auth_challenges WHERE id=$1 AND context_key_version IS NOT DISTINCT FROM $2 AND client_network_hmac IS NOT DISTINCT FROM $3 AND user_agent_hash IS NOT DISTINCT FROM $4 AND status IN ('pending','verification_pending','rotation_pending','exhausted') AND created_at_ms<=$5 AND expires_at_ms>$6 AND EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) AND (session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=$7 AND s.idle_expires_at_ms>$8 AND s.absolute_expires_at_ms>$9))"
            )
            .bind(lookup.id.into_uuid())
            .bind(context_key_version)
            .bind(
                lookup
                    .client_context
                    .client_network_hmac
                    .as_ref()
                    .map(|value| value.as_slice()),
            )
            .bind(
                lookup
                    .client_context
                    .user_agent_hash
                    .as_ref()
                    .map(|value| value.as_slice()),
            )
            .bind(lookup.now_ms)
            .bind(lookup.now_ms)
            .bind(lookup.now_ms)
            .bind(lookup.now_ms)
            .bind(lookup.now_ms)
            .fetch_optional(pool)
            .await?,
        };
        row.map(decode_token_digest).transpose()
    }

    pub async fn auth_challenge(
        &self,
        access: &AuthChallengeAccess,
    ) -> Result<Option<AuthChallenge>, PersistenceError> {
        validate_access(access)?;
        let key_version = database_key_version(access.token_key_version)?;
        let context_key_version = access
            .client_context
            .key_version
            .map(database_key_version)
            .transpose()?;
        match self {
            Self::Sqlite(pool) => {
                let mut transaction = pool.begin().await?;
                refresh_access_sqlite(&mut transaction, access, key_version).await?;
                let row = sqlx::query_as::<_, ChallengeRow>(
                    "SELECT id,purpose,user_id,session_id,auth_revision,status,rotation_state,attempts_used,max_attempts,created_at_ms,expires_at_ms,verified_method,achieved_assurance,consumed_at_ms,CASE WHEN attempt_claim_id IS NULL THEN 0 ELSE 1 END AS has_attempt_claim,CASE WHEN client_network_hmac IS NULL THEN 0 ELSE 1 END AS has_client_network_context,CASE WHEN user_agent_hash IS NULL THEN 0 ELSE 1 END AS has_user_agent_context,revision FROM auth_challenges WHERE id=? AND token_key_version=? AND token_hmac=? AND context_key_version IS ? AND client_network_hmac IS ? AND user_agent_hash IS ? AND created_at_ms<=?",
                )
                .bind(access.id.to_string())
                .bind(key_version)
                .bind(access.token_hmac.as_slice())
                .bind(context_key_version)
                .bind(
                    access
                        .client_context
                        .client_network_hmac
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(
                    access
                        .client_context
                        .user_agent_hash
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(access.now_ms)
                .fetch_optional(&mut *transaction)
                .await?;
                let challenge = match row {
                    Some(row) => {
                        let methods: Vec<String> = sqlx::query_scalar(
                            "SELECT method FROM auth_challenge_methods WHERE challenge_id=? ORDER BY method",
                        )
                        .bind(access.id.to_string())
                        .fetch_all(&mut *transaction)
                        .await?;
                        Some(decode_challenge(row, methods)?)
                    }
                    None => None,
                };
                transaction.commit().await?;
                Ok(challenge)
            }
            Self::Postgres(pool) => {
                let mut transaction = pool.begin().await?;
                refresh_access_postgres(&mut transaction, access, key_version).await?;
                let row = sqlx::query_as::<_, ChallengeRow>(
                    "SELECT id::text AS id,purpose,user_id::text AS user_id,session_id::text AS session_id,auth_revision,status,rotation_state,attempts_used::BIGINT AS attempts_used,max_attempts::BIGINT AS max_attempts,created_at_ms,expires_at_ms,verified_method,achieved_assurance,consumed_at_ms,CASE WHEN attempt_claim_id IS NULL THEN 0::BIGINT ELSE 1::BIGINT END AS has_attempt_claim,CASE WHEN client_network_hmac IS NULL THEN 0::BIGINT ELSE 1::BIGINT END AS has_client_network_context,CASE WHEN user_agent_hash IS NULL THEN 0::BIGINT ELSE 1::BIGINT END AS has_user_agent_context,revision FROM auth_challenges WHERE id=$1 AND token_key_version=$2 AND token_hmac=$3 AND context_key_version IS NOT DISTINCT FROM $4 AND client_network_hmac IS NOT DISTINCT FROM $5 AND user_agent_hash IS NOT DISTINCT FROM $6 AND created_at_ms<=$7",
                )
                .bind(access.id.into_uuid())
                .bind(key_version)
                .bind(access.token_hmac.as_slice())
                .bind(context_key_version)
                .bind(
                    access
                        .client_context
                        .client_network_hmac
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(
                    access
                        .client_context
                        .user_agent_hash
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(access.now_ms)
                .fetch_optional(&mut *transaction)
                .await?;
                let challenge = match row {
                    Some(row) => {
                        let methods: Vec<String> = sqlx::query_scalar(
                            "SELECT method FROM auth_challenge_methods WHERE challenge_id=$1 ORDER BY method",
                        )
                        .bind(access.id.into_uuid())
                        .fetch_all(&mut *transaction)
                        .await?;
                        Some(decode_challenge(row, methods)?)
                    }
                    None => None,
                };
                transaction.commit().await?;
                Ok(challenge)
            }
        }
    }
}

impl Database {
    /// Reserves the restart-safe handoff to the replacement-session transaction. The claim lease
    /// prevents parallel transaction work; expiry clears only the handoff claim, so a verified
    /// `rotation_pending` challenge can be resumed after a process crash.
    pub async fn reserve_auth_challenge_rotation(
        &self,
        reservation: &AuthChallengeRotationReservation,
    ) -> Result<AuthChallengeRotationReservationOutcome, PersistenceError> {
        validate_access(&reservation.access)?;
        if reservation.transaction_expires_at_ms <= reservation.access.now_ms {
            return Err(PersistenceError::InvalidAuthChallenge);
        }
        let key_version = database_key_version(reservation.access.token_key_version)?;
        let context_key_version = reservation
            .access
            .client_context
            .key_version
            .map(database_key_version)
            .transpose()?;
        let expected_revision = database_revision(reservation.expected_revision)?;
        let rows_affected = match self {
            Self::Sqlite(pool) => {
                let mut transaction = pool.begin().await?;
                refresh_access_sqlite(&mut transaction, &reservation.access, key_version).await?;
                let rows = sqlx::query(
                    "UPDATE auth_challenges SET attempt_claim_id=?,attempted_method=verified_method,attempt_started_at_ms=?,attempt_expires_at_ms=MIN(expires_at_ms,?),revision=revision+1,updated_at_ms=? WHERE id=? AND token_key_version=? AND token_hmac=? AND context_key_version IS ? AND client_network_hmac IS ? AND user_agent_hash IS ? AND revision=? AND status='rotation_pending' AND rotation_state='pending' AND attempt_claim_id IS NULL AND verified_method IS NOT NULL AND achieved_assurance IS NOT NULL AND created_at_ms<=? AND expires_at_ms>? AND EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) AND (session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?))",
                )
                .bind(reservation.claim_id.to_string())
                .bind(reservation.access.now_ms)
                .bind(reservation.transaction_expires_at_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.id.to_string())
                .bind(key_version)
                .bind(reservation.access.token_hmac.as_slice())
                .bind(context_key_version)
                .bind(
                    reservation
                        .access
                        .client_context
                        .client_network_hmac
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(
                    reservation
                        .access
                        .client_context
                        .user_agent_hash
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(expected_revision)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
                transaction.commit().await?;
                rows
            }
            Self::Postgres(pool) => {
                let mut transaction = pool.begin().await?;
                refresh_access_postgres(&mut transaction, &reservation.access, key_version).await?;
                let rows = sqlx::query(
                    "UPDATE auth_challenges SET attempt_claim_id=$1,attempted_method=verified_method,attempt_started_at_ms=$2,attempt_expires_at_ms=LEAST(expires_at_ms,$3),revision=revision+1,updated_at_ms=$4 WHERE id=$5 AND token_key_version=$6 AND token_hmac=$7 AND context_key_version IS NOT DISTINCT FROM $8 AND client_network_hmac IS NOT DISTINCT FROM $9 AND user_agent_hash IS NOT DISTINCT FROM $10 AND revision=$11 AND status='rotation_pending' AND rotation_state='pending' AND attempt_claim_id IS NULL AND verified_method IS NOT NULL AND achieved_assurance IS NOT NULL AND created_at_ms<=$12 AND expires_at_ms>$13 AND EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) AND (session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=$14 AND s.idle_expires_at_ms>$15 AND s.absolute_expires_at_ms>$16))",
                )
                .bind(reservation.claim_id.into_uuid())
                .bind(reservation.access.now_ms)
                .bind(reservation.transaction_expires_at_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.id.into_uuid())
                .bind(key_version)
                .bind(reservation.access.token_hmac.as_slice())
                .bind(context_key_version)
                .bind(
                    reservation
                        .access
                        .client_context
                        .client_network_hmac
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(
                    reservation
                        .access
                        .client_context
                        .user_agent_hash
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(expected_revision)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
                transaction.commit().await?;
                rows
            }
        };
        if rows_affected != 1 {
            return Ok(AuthChallengeRotationReservationOutcome::Stale);
        }
        match self.auth_challenge(&reservation.access).await? {
            Some(challenge)
                if challenge.status == AuthChallengeStatus::RotationPending
                    && challenge.rotation_transaction_in_progress =>
            {
                Ok(AuthChallengeRotationReservationOutcome::Reserved(challenge))
            }
            _ => Err(PersistenceError::InvalidStoredAuthChallenge),
        }
    }
}

#[cfg(test)]
pub(crate) async fn auth_challenge_contract(
    database: &Database,
    user_id: EntityId,
    auth_revision: Revision,
    base_ms: i64,
) {
    let context_for = |seed: u8| AuthChallengeClientContext {
        key_version: Some(1),
        client_network_hmac: Some([seed.wrapping_add(32); 32]),
        user_agent_hash: Some([seed.wrapping_add(64); 32]),
    };
    let challenge_fixture =
        |seed: u8,
         purpose: AuthChallengePurpose,
         session_id: Option<EntityId>,
         created_at_ms: i64,
         expires_at_ms: i64,
         rotation_required: bool,
         max_attempts: u32,
         allowed_methods: Vec<AuthenticationMethod>| NewAuthChallenge {
            id: EntityId::new(),
            token_key_version: 1,
            token_hmac: [seed; 32],
            purpose,
            user_id,
            session_id,
            auth_revision,
            allowed_methods,
            max_attempts,
            created_at_ms,
            expires_at_ms,
            rotation_required,
            client_context: context_for(seed),
            revision: Revision::initial(),
        };
    let access_for = |challenge: &NewAuthChallenge, now_ms: i64| AuthChallengeAccess {
        id: challenge.id,
        token_key_version: challenge.token_key_version,
        token_hmac: challenge.token_hmac,
        client_context: challenge.client_context.clone(),
        now_ms,
    };

    let attempts = challenge_fixture(
        101,
        AuthChallengePurpose::Login,
        None,
        base_ms,
        base_ms + 1_000,
        false,
        3,
        vec![AuthenticationMethod::Totp, AuthenticationMethod::WebAuthn],
    );
    assert!(matches!(
        database.create_auth_challenge(&attempts).await,
        Ok(CreateAuthChallengeOutcome::Created(ref challenge))
            if challenge.status == AuthChallengeStatus::Pending
                && challenge.attempts_used == 0
                && challenge.remaining_attempts() == 3
                && challenge.has_client_network_context
                && challenge.has_user_agent_context
    ));

    let digest = database
        .auth_challenge_token_digest(&AuthChallengeTokenLookup {
            id: attempts.id,
            client_context: attempts.client_context.clone(),
            now_ms: base_ms + 1,
        })
        .await;
    assert!(matches!(
        digest,
        Ok(Some(ref value)) if value.key_version == 1 && value.digest == attempts.token_hmac
    ));
    let mut wrong_context = attempts.client_context.clone();
    wrong_context.user_agent_hash = Some([250; 32]);
    assert!(matches!(
        database
            .auth_challenge_token_digest(&AuthChallengeTokenLookup {
                id: attempts.id,
                client_context: wrong_context.clone(),
                now_ms: base_ms + 1,
            })
            .await,
        Ok(None)
    ));
    let mut mismatched_access = access_for(&attempts, base_ms + 1);
    mismatched_access.client_context = wrong_context;
    assert!(matches!(
        database.auth_challenge(&mismatched_access).await,
        Ok(None)
    ));
    assert!(matches!(
        database
            .reserve_auth_challenge_attempt(&AuthChallengeAttemptReservation {
                access: mismatched_access,
                claim_id: EntityId::new(),
                method: AuthenticationMethod::Totp,
                expected_revision: Revision::initial(),
                verification_expires_at_ms: base_ms + 20,
            })
            .await,
        Ok(AuthChallengeAttemptReservationOutcome::Stale)
    ));

    let access = access_for(&attempts, base_ms + 2);
    let claims = [
        EntityId::new(),
        EntityId::new(),
        EntityId::new(),
        EntityId::new(),
    ];
    let reservations = [
        AuthChallengeAttemptReservation {
            access: access.clone(),
            claim_id: claims[0],
            method: AuthenticationMethod::WebAuthn,
            expected_revision: Revision::initial(),
            verification_expires_at_ms: base_ms + 30,
        },
        AuthChallengeAttemptReservation {
            access: access.clone(),
            claim_id: claims[1],
            method: AuthenticationMethod::WebAuthn,
            expected_revision: Revision::initial(),
            verification_expires_at_ms: base_ms + 30,
        },
        AuthChallengeAttemptReservation {
            access: access.clone(),
            claim_id: claims[2],
            method: AuthenticationMethod::WebAuthn,
            expected_revision: Revision::initial(),
            verification_expires_at_ms: base_ms + 30,
        },
        AuthChallengeAttemptReservation {
            access,
            claim_id: claims[3],
            method: AuthenticationMethod::WebAuthn,
            expected_revision: Revision::initial(),
            verification_expires_at_ms: base_ms + 30,
        },
    ];
    let (one, two, three, four) = tokio::join!(
        database.reserve_auth_challenge_attempt(&reservations[0]),
        database.reserve_auth_challenge_attempt(&reservations[1]),
        database.reserve_auth_challenge_attempt(&reservations[2]),
        database.reserve_auth_challenge_attempt(&reservations[3]),
    );
    let outcomes = [one, two, three, four];
    let mut winning_claim = None;
    let mut stale_count = 0;
    for (claim_id, outcome) in claims.into_iter().zip(outcomes) {
        match outcome {
            Ok(AuthChallengeAttemptReservationOutcome::Reserved(challenge)) => {
                assert_eq!(challenge.attempts_used, 1);
                assert_eq!(challenge.revision, Revision::from_value(1));
                assert!(challenge.verification_in_progress);
                assert!(winning_claim.replace(claim_id).is_none());
            }
            Ok(AuthChallengeAttemptReservationOutcome::Stale) => stale_count += 1,
            result => panic!("unexpected concurrent reservation outcome: {result:?}"),
        }
    }
    assert_eq!(stale_count, 3);
    let Some(winning_claim) = winning_claim else {
        panic!("one reservation must own the verifier claim");
    };
    let failure_access = access_for(&attempts, base_ms + 3);
    assert!(matches!(
        database
            .record_auth_challenge_failure(&AuthChallengeAttemptFailure {
                access: failure_access,
                claim_id: winning_claim,
                method: AuthenticationMethod::WebAuthn,
                expected_revision: Revision::from_value(1),
            })
            .await,
        Ok(AuthChallengeAttemptOutcome::Retryable(ref challenge))
            if challenge.attempts_used == 1
                && challenge.revision == Revision::from_value(2)
                && !challenge.verification_in_progress
    ));

    let second_claim = EntityId::new();
    let second_access = access_for(&attempts, base_ms + 4);
    assert!(matches!(
        database
            .reserve_auth_challenge_attempt(&AuthChallengeAttemptReservation {
                access: second_access.clone(),
                claim_id: second_claim,
                method: AuthenticationMethod::Totp,
                expected_revision: Revision::from_value(2),
                verification_expires_at_ms: base_ms + 30,
            })
            .await,
        Ok(AuthChallengeAttemptReservationOutcome::Reserved(ref challenge))
            if challenge.attempts_used == 2 && challenge.revision == Revision::from_value(3)
    ));
    assert!(matches!(
        database
            .record_auth_challenge_failure(&AuthChallengeAttemptFailure {
                access: second_access,
                claim_id: second_claim,
                method: AuthenticationMethod::Totp,
                expected_revision: Revision::from_value(3),
            })
            .await,
        Ok(AuthChallengeAttemptOutcome::Retryable(ref challenge))
            if challenge.attempts_used == 2 && challenge.revision == Revision::from_value(4)
    ));

    let final_claim = EntityId::new();
    let final_access = access_for(&attempts, base_ms + 5);
    assert!(matches!(
        database
            .reserve_auth_challenge_attempt(&AuthChallengeAttemptReservation {
                access: final_access.clone(),
                claim_id: final_claim,
                method: AuthenticationMethod::WebAuthn,
                expected_revision: Revision::from_value(4),
                verification_expires_at_ms: base_ms + 30,
            })
            .await,
        Ok(AuthChallengeAttemptReservationOutcome::Reserved(ref challenge))
            if challenge.attempts_used == 3
                && challenge.remaining_attempts() == 0
                && challenge.status == AuthChallengeStatus::VerificationPending
    ));
    assert!(matches!(
        database
            .begin_auth_challenge_consumption(&AuthChallengeConsumption {
                access: final_access.clone(),
                claim_id: final_claim,
                method: AuthenticationMethod::WebAuthn,
                achieved_assurance: AuthenticationAssurance::Password,
                expected_revision: Revision::from_value(5),
            })
            .await,
        Err(PersistenceError::InvalidAuthChallenge)
    ));
    assert!(matches!(
        database
            .begin_auth_challenge_consumption(&AuthChallengeConsumption {
                access: final_access,
                claim_id: final_claim,
                method: AuthenticationMethod::WebAuthn,
                achieved_assurance: AuthenticationAssurance::PhishingResistant,
                expected_revision: Revision::from_value(5),
            })
            .await,
        Ok(AuthChallengeConsumptionOutcome::Consumed(ref challenge))
            if challenge.attempts_used == 3
                && challenge.status == AuthChallengeStatus::Consumed
                && challenge.verified_method == Some(AuthenticationMethod::WebAuthn)
                && challenge.achieved_assurance
                    == Some(AuthenticationAssurance::PhishingResistant)
    ));

    let crash = challenge_fixture(
        102,
        AuthChallengePurpose::Reauthenticate,
        None,
        base_ms + 100,
        base_ms + 1_100,
        false,
        2,
        vec![AuthenticationMethod::Totp],
    );
    assert!(matches!(
        database.create_auth_challenge(&crash).await,
        Ok(CreateAuthChallengeOutcome::Created(_))
    ));
    let first_crash_claim = EntityId::new();
    assert!(matches!(
        database
            .reserve_auth_challenge_attempt(&AuthChallengeAttemptReservation {
                access: access_for(&crash, base_ms + 101),
                claim_id: first_crash_claim,
                method: AuthenticationMethod::Totp,
                expected_revision: Revision::initial(),
                verification_expires_at_ms: base_ms + 103,
            })
            .await,
        Ok(AuthChallengeAttemptReservationOutcome::Reserved(_))
    ));
    assert!(matches!(
        database
            .auth_challenge(&access_for(&crash, base_ms + 103))
            .await,
        Ok(Some(ref challenge))
            if challenge.status == AuthChallengeStatus::Pending
                && challenge.attempts_used == 1
                && challenge.revision == Revision::from_value(2)
    ));
    assert!(matches!(
        database
            .record_auth_challenge_failure(&AuthChallengeAttemptFailure {
                access: access_for(&crash, base_ms + 104),
                claim_id: first_crash_claim,
                method: AuthenticationMethod::Totp,
                expected_revision: Revision::from_value(1),
            })
            .await,
        Ok(AuthChallengeAttemptOutcome::Stale)
    ));
    let final_crash_claim = EntityId::new();
    assert!(matches!(
        database
            .reserve_auth_challenge_attempt(&AuthChallengeAttemptReservation {
                access: access_for(&crash, base_ms + 104),
                claim_id: final_crash_claim,
                method: AuthenticationMethod::Totp,
                expected_revision: Revision::from_value(2),
                verification_expires_at_ms: base_ms + 106,
            })
            .await,
        Ok(AuthChallengeAttemptReservationOutcome::Reserved(_))
    ));
    assert!(matches!(
        database
            .auth_challenge(&access_for(&crash, base_ms + 106))
            .await,
        Ok(Some(ref challenge))
            if challenge.status == AuthChallengeStatus::Exhausted
                && challenge.attempts_used == 2
                && challenge.revision == Revision::from_value(4)
    ));
    let blocked_reissue = challenge_fixture(
        103,
        AuthChallengePurpose::Reauthenticate,
        None,
        base_ms + 107,
        base_ms + 2_000,
        false,
        2,
        vec![AuthenticationMethod::Totp],
    );
    assert!(matches!(
        database.create_auth_challenge(&blocked_reissue).await,
        Ok(CreateAuthChallengeOutcome::AlreadyOpen)
    ));
    let expired_replacement = challenge_fixture(
        104,
        AuthChallengePurpose::Reauthenticate,
        None,
        base_ms + 1_100,
        base_ms + 2_100,
        false,
        2,
        vec![AuthenticationMethod::Totp],
    );
    assert!(matches!(
        database.create_auth_challenge(&expired_replacement).await,
        Ok(CreateAuthChallengeOutcome::Created(_))
    ));

    let concurrent_one = challenge_fixture(
        105,
        AuthChallengePurpose::SensitiveAction,
        None,
        base_ms + 200,
        base_ms + 500,
        false,
        2,
        vec![AuthenticationMethod::Password],
    );
    let concurrent_two = NewAuthChallenge {
        id: EntityId::new(),
        token_hmac: [106; 32],
        ..concurrent_one.clone()
    };
    let (issued_one, issued_two) = tokio::join!(
        database.create_auth_challenge(&concurrent_one),
        database.create_auth_challenge(&concurrent_two),
    );
    assert!(matches!(
        (&issued_one, &issued_two),
        (
            Ok(CreateAuthChallengeOutcome::Created(_)),
            Ok(CreateAuthChallengeOutcome::AlreadyOpen)
        ) | (
            Ok(CreateAuthChallengeOutcome::AlreadyOpen),
            Ok(CreateAuthChallengeOutcome::Created(_))
        )
    ));
    let concurrent_replacement = challenge_fixture(
        107,
        AuthChallengePurpose::SensitiveAction,
        None,
        base_ms + 500,
        base_ms + 800,
        false,
        2,
        vec![AuthenticationMethod::Password],
    );
    assert!(matches!(
        database
            .create_auth_challenge(&concurrent_replacement)
            .await,
        Ok(CreateAuthChallengeOutcome::Created(_))
    ));

    let revision_bound = challenge_fixture(
        108,
        AuthChallengePurpose::CredentialEnrollment,
        None,
        base_ms + 600,
        base_ms + 1_000,
        false,
        2,
        vec![AuthenticationMethod::Password],
    );
    assert!(matches!(
        database.create_auth_challenge(&revision_bound).await,
        Ok(CreateAuthChallengeOutcome::Created(_))
    ));
    set_auth_revision(database, user_id, Revision::from_value(1)).await;
    assert!(matches!(
        database
            .auth_challenge(&access_for(&revision_bound, base_ms + 601))
            .await,
        Ok(Some(ref challenge)) if challenge.status == AuthChallengeStatus::Invalidated
    ));
    set_auth_revision(database, user_id, auth_revision).await;

    let expiring_session = EntityId::new();
    insert_test_session(
        database,
        expiring_session,
        user_id,
        auth_revision,
        201,
        base_ms + 700,
        base_ms + 705,
        base_ms + 900,
    )
    .await;
    let session_expiry = challenge_fixture(
        109,
        AuthChallengePurpose::Login,
        Some(expiring_session),
        base_ms + 701,
        base_ms + 850,
        false,
        2,
        vec![AuthenticationMethod::Password],
    );
    assert!(matches!(
        database.create_auth_challenge(&session_expiry).await,
        Ok(CreateAuthChallengeOutcome::Created(_))
    ));
    assert!(matches!(
        database
            .auth_challenge(&access_for(&session_expiry, base_ms + 705))
            .await,
        Ok(Some(ref challenge)) if challenge.status == AuthChallengeStatus::Invalidated
    ));

    let revoked_session = EntityId::new();
    insert_test_session(
        database,
        revoked_session,
        user_id,
        auth_revision,
        202,
        base_ms + 710,
        base_ms + 800,
        base_ms + 900,
    )
    .await;
    let session_revoke = challenge_fixture(
        110,
        AuthChallengePurpose::CredentialEnrollment,
        Some(revoked_session),
        base_ms + 711,
        base_ms + 850,
        true,
        2,
        vec![AuthenticationMethod::RecoveryCode],
    );
    assert!(matches!(
        database.create_auth_challenge(&session_revoke).await,
        Ok(CreateAuthChallengeOutcome::Created(_))
    ));
    revoke_test_session(database, revoked_session, base_ms + 712).await;
    assert!(matches!(
        database
            .auth_challenge(&access_for(&session_revoke, base_ms + 713))
            .await,
        Ok(Some(ref challenge)) if challenge.status == AuthChallengeStatus::Invalidated
    ));

    let invalid_context_write = match database {
        Database::Sqlite(pool) => {
            sqlx::query("UPDATE auth_challenges SET user_agent_hash=NULL WHERE id=?")
                .bind(expired_replacement.id.to_string())
                .execute(pool)
                .await
                .map(|_| ())
        }
        Database::Postgres(pool) => {
            sqlx::query("UPDATE auth_challenges SET user_agent_hash=NULL WHERE id=$1")
                .bind(expired_replacement.id.into_uuid())
                .execute(pool)
                .await
                .map(|_| ())
        }
    };
    assert!(invalid_context_write.is_err());

    let invalid_assurance_write = match database {
        Database::Sqlite(pool) => {
            sqlx::query("UPDATE auth_challenges SET achieved_assurance='password' WHERE id=?")
                .bind(attempts.id.to_string())
                .execute(pool)
                .await
                .map(|_| ())
        }
        Database::Postgres(pool) => {
            sqlx::query("UPDATE auth_challenges SET achieved_assurance='password' WHERE id=$1")
                .bind(attempts.id.into_uuid())
                .execute(pool)
                .await
                .map(|_| ())
        }
    };
    assert!(invalid_assurance_write.is_err());

    let mut malformed_context = challenge_fixture(
        111,
        AuthChallengePurpose::Login,
        None,
        base_ms + 720,
        base_ms + 900,
        false,
        2,
        vec![AuthenticationMethod::Password],
    );
    malformed_context.client_context.user_agent_hash = None;
    assert!(matches!(
        database.create_auth_challenge(&malformed_context).await,
        Err(PersistenceError::InvalidAuthChallenge)
    ));

    let storage_probe = challenge_fixture(
        112,
        AuthChallengePurpose::CredentialEnrollment,
        None,
        base_ms + 720,
        base_ms + 900,
        false,
        2,
        vec![AuthenticationMethod::Password],
    );
    assert!(matches!(
        database.create_auth_challenge(&storage_probe).await,
        Ok(CreateAuthChallengeOutcome::Created(_))
    ));
    let storage_claim = EntityId::new();
    assert!(matches!(
        database
            .reserve_auth_challenge_attempt(&AuthChallengeAttemptReservation {
                access: access_for(&storage_probe, base_ms + 721),
                claim_id: storage_claim,
                method: AuthenticationMethod::Password,
                expected_revision: Revision::initial(),
                verification_expires_at_ms: base_ms + 730,
            })
            .await,
        Ok(AuthChallengeAttemptReservationOutcome::Reserved(_))
    ));
    match database {
        Database::Sqlite(pool) => {
            let invalid_uuid = sqlx::query(
                "UPDATE auth_challenges SET attempt_claim_id='00000000-0000-7000-8000-00000000000A' WHERE id=?",
            )
            .bind(storage_probe.id.to_string())
            .execute(pool)
            .await;
            assert!(invalid_uuid.is_err());
            let invalid_storage_class =
                sqlx::query("UPDATE auth_challenges SET attempts_used='not-an-integer' WHERE id=?")
                    .bind(storage_probe.id.to_string())
                    .execute(pool)
                    .await;
            assert!(invalid_storage_class.is_err());
        }
        Database::Postgres(pool) => {
            let invalid_uuid =
                sqlx::query("UPDATE auth_challenges SET attempt_claim_id='not-a-uuid' WHERE id=$1")
                    .bind(storage_probe.id.into_uuid())
                    .execute(pool)
                    .await;
            assert!(invalid_uuid.is_err());
        }
    }
    assert!(matches!(
        database
            .begin_auth_challenge_consumption(&AuthChallengeConsumption {
                access: access_for(&storage_probe, base_ms + 722),
                claim_id: storage_claim,
                method: AuthenticationMethod::Password,
                achieved_assurance: AuthenticationAssurance::Password,
                expected_revision: Revision::from_value(1),
            })
            .await,
        Ok(AuthChallengeConsumptionOutcome::Consumed(_))
    ));

    let rotating = challenge_fixture(
        113,
        AuthChallengePurpose::CredentialEnrollment,
        None,
        base_ms + 723,
        base_ms + 900,
        true,
        2,
        vec![AuthenticationMethod::RecoveryCode],
    );
    assert!(matches!(
        database.create_auth_challenge(&rotating).await,
        Ok(CreateAuthChallengeOutcome::Created(_))
    ));
    let proof_claim = EntityId::new();
    assert!(matches!(
        database
            .reserve_auth_challenge_attempt(&AuthChallengeAttemptReservation {
                access: access_for(&rotating, base_ms + 724),
                claim_id: proof_claim,
                method: AuthenticationMethod::RecoveryCode,
                expected_revision: Revision::initial(),
                verification_expires_at_ms: base_ms + 730,
            })
            .await,
        Ok(AuthChallengeAttemptReservationOutcome::Reserved(_))
    ));
    assert!(matches!(
        database
            .begin_auth_challenge_consumption(&AuthChallengeConsumption {
                access: access_for(&rotating, base_ms + 725),
                claim_id: proof_claim,
                method: AuthenticationMethod::RecoveryCode,
                achieved_assurance: AuthenticationAssurance::Recovery,
                expected_revision: Revision::from_value(1),
            })
            .await,
        Ok(AuthChallengeConsumptionOutcome::RotationPending(ref challenge))
            if challenge.revision == Revision::from_value(2)
                && !challenge.rotation_transaction_in_progress
    ));
    let rotation_claims = [
        EntityId::new(),
        EntityId::new(),
        EntityId::new(),
        EntityId::new(),
    ];
    let rotation_reservations = rotation_claims.map(|claim_id| AuthChallengeRotationReservation {
        access: access_for(&rotating, base_ms + 726),
        claim_id,
        expected_revision: Revision::from_value(2),
        transaction_expires_at_ms: base_ms + 728,
    });
    let (rotation_one, rotation_two, rotation_three, rotation_four) = tokio::join!(
        database.reserve_auth_challenge_rotation(&rotation_reservations[0]),
        database.reserve_auth_challenge_rotation(&rotation_reservations[1]),
        database.reserve_auth_challenge_rotation(&rotation_reservations[2]),
        database.reserve_auth_challenge_rotation(&rotation_reservations[3]),
    );
    let rotation_outcomes = [rotation_one, rotation_two, rotation_three, rotation_four];
    let mut rotation_winners = 0;
    let mut rotation_stale = 0;
    for outcome in rotation_outcomes {
        match outcome {
            Ok(AuthChallengeRotationReservationOutcome::Reserved(challenge)) => {
                rotation_winners += 1;
                assert_eq!(challenge.revision, Revision::from_value(3));
                assert!(challenge.rotation_transaction_in_progress);
            }
            Ok(AuthChallengeRotationReservationOutcome::Stale) => rotation_stale += 1,
            result => panic!("unexpected concurrent rotation resume outcome: {result:?}"),
        }
    }
    assert_eq!(rotation_winners, 1);
    assert_eq!(rotation_stale, 3);
    assert!(matches!(
        database
            .auth_challenge(&access_for(&rotating, base_ms + 728))
            .await,
        Ok(Some(ref challenge))
            if challenge.status == AuthChallengeStatus::RotationPending
                && challenge.revision == Revision::from_value(4)
                && !challenge.rotation_transaction_in_progress
    ));
    assert!(matches!(
        database
            .reserve_auth_challenge_rotation(&AuthChallengeRotationReservation {
                access: access_for(&rotating, base_ms + 729),
                claim_id: EntityId::new(),
                expected_revision: Revision::from_value(4),
                transaction_expires_at_ms: base_ms + 735,
            })
            .await,
        Ok(AuthChallengeRotationReservationOutcome::Reserved(ref challenge))
            if challenge.revision == Revision::from_value(5)
                && challenge.rotation_transaction_in_progress
    ));

    cleanup_test_session(database, expiring_session).await;
    cleanup_test_session(database, revoked_session).await;
}

#[cfg(test)]
async fn set_auth_revision(database: &Database, user_id: EntityId, revision: Revision) {
    let value = database_revision(revision);
    assert!(value.is_ok());
    let Ok(value) = value else {
        return;
    };
    let result: Result<u64, sqlx::Error> = match database {
        Database::Sqlite(pool) => {
            sqlx::query("UPDATE user_auth_state SET auth_revision=? WHERE user_id=?")
                .bind(value)
                .bind(user_id.to_string())
                .execute(pool)
                .await
                .map(|changed| changed.rows_affected())
        }
        Database::Postgres(pool) => {
            sqlx::query("UPDATE user_auth_state SET auth_revision=$1 WHERE user_id=$2")
                .bind(value)
                .bind(user_id.into_uuid())
                .execute(pool)
                .await
                .map(|changed| changed.rows_affected())
        }
    };
    assert!(matches!(result, Ok(1)));
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
async fn insert_test_session(
    database: &Database,
    session_id: EntityId,
    user_id: EntityId,
    auth_revision: Revision,
    seed: u8,
    created_at_ms: i64,
    idle_expires_at_ms: i64,
    absolute_expires_at_ms: i64,
) {
    let auth_revision = database_revision(auth_revision);
    assert!(auth_revision.is_ok());
    let Ok(auth_revision) = auth_revision else {
        return;
    };
    let token_hmac = [seed; 32];
    let csrf_hmac = [seed.wrapping_add(1); 32];
    let result: Result<u64, sqlx::Error> = match database {
        Database::Sqlite(pool) => sqlx::query(
            "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES (?,?,1,?,1,?,?,'password','active',?,?,?,?,?,?,NULL,NULL,NULL,NULL,NULL,0)",
        )
        .bind(session_id.to_string())
        .bind(user_id.to_string())
        .bind(token_hmac.as_slice())
        .bind(csrf_hmac.as_slice())
        .bind(auth_revision)
        .bind(created_at_ms)
        .bind(created_at_ms)
        .bind(created_at_ms)
        .bind(created_at_ms)
        .bind(idle_expires_at_ms)
        .bind(absolute_expires_at_ms)
        .execute(pool)
        .await
        .map(|changed| changed.rows_affected()),
        Database::Postgres(pool) => sqlx::query(
            "INSERT INTO auth_sessions (id,user_id,token_key_version,token_hmac,csrf_key_version,csrf_hmac,auth_revision,auth_level,status,created_at_ms,authenticated_at_ms,recent_auth_at_ms,last_seen_at_ms,idle_expires_at_ms,absolute_expires_at_ms,ip_prefix_key_version,ip_prefix_hmac,user_agent_hash,revoked_at_ms,revoked_reason,revision) VALUES ($1,$2,1,$3,1,$4,$5,'password','active',$6,$7,$8,$9,$10,$11,NULL,NULL,NULL,NULL,NULL,0)",
        )
        .bind(session_id.into_uuid())
        .bind(user_id.into_uuid())
        .bind(token_hmac.as_slice())
        .bind(csrf_hmac.as_slice())
        .bind(auth_revision)
        .bind(created_at_ms)
        .bind(created_at_ms)
        .bind(created_at_ms)
        .bind(created_at_ms)
        .bind(idle_expires_at_ms)
        .bind(absolute_expires_at_ms)
        .execute(pool)
        .await
        .map(|changed| changed.rows_affected()),
    };
    assert!(matches!(result, Ok(1)));
}

#[cfg(test)]
async fn revoke_test_session(database: &Database, session_id: EntityId, revoked_at_ms: i64) {
    let result: Result<u64, sqlx::Error> = match database {
        Database::Sqlite(pool) => sqlx::query(
            "UPDATE auth_sessions SET status='revoked',revoked_at_ms=?,revoked_reason='administrator',revision=revision+1 WHERE id=?",
        )
        .bind(revoked_at_ms)
        .bind(session_id.to_string())
        .execute(pool)
        .await
        .map(|changed| changed.rows_affected()),
        Database::Postgres(pool) => sqlx::query(
            "UPDATE auth_sessions SET status='revoked',revoked_at_ms=$1,revoked_reason='administrator',revision=revision+1 WHERE id=$2",
        )
        .bind(revoked_at_ms)
        .bind(session_id.into_uuid())
        .execute(pool)
        .await
        .map(|changed| changed.rows_affected()),
    };
    assert!(matches!(result, Ok(1)));
}

#[cfg(test)]
async fn cleanup_test_session(database: &Database, session_id: EntityId) {
    let result = match database {
        Database::Sqlite(pool) => {
            let mut transaction = match pool.begin().await {
                Ok(transaction) => transaction,
                Err(error) => panic!("test cleanup transaction failed: {error}"),
            };
            let challenge_delete = sqlx::query("DELETE FROM auth_challenges WHERE session_id=?")
                .bind(session_id.to_string())
                .execute(&mut *transaction)
                .await;
            assert!(challenge_delete.is_ok());
            let session_delete = sqlx::query("DELETE FROM auth_sessions WHERE id=?")
                .bind(session_id.to_string())
                .execute(&mut *transaction)
                .await;
            assert!(session_delete.is_ok());
            transaction.commit().await
        }
        Database::Postgres(pool) => {
            let mut transaction = match pool.begin().await {
                Ok(transaction) => transaction,
                Err(error) => panic!("test cleanup transaction failed: {error}"),
            };
            let challenge_delete = sqlx::query("DELETE FROM auth_challenges WHERE session_id=$1")
                .bind(session_id.into_uuid())
                .execute(&mut *transaction)
                .await;
            assert!(challenge_delete.is_ok());
            let session_delete = sqlx::query("DELETE FROM auth_sessions WHERE id=$1")
                .bind(session_id.into_uuid())
                .execute(&mut *transaction)
                .await;
            assert!(session_delete.is_ok());
            transaction.commit().await
        }
    };
    assert!(result.is_ok());
}

fn validate_new_challenge(challenge: &NewAuthChallenge) -> Result<(), PersistenceError> {
    database_key_version(challenge.token_key_version)?;
    database_revision(challenge.auth_revision)?;
    database_revision(challenge.revision)?;
    validate_client_context(&challenge.client_context)?;
    if challenge.revision != Revision::initial()
        || challenge.created_at_ms < 0
        || challenge.expires_at_ms <= challenge.created_at_ms
        || challenge.max_attempts == 0
        || challenge.max_attempts > i32::MAX as u32
        || canonical_methods(&challenge.allowed_methods).is_err()
    {
        return Err(PersistenceError::InvalidAuthChallenge);
    }
    Ok(())
}

fn validate_lookup(lookup: &AuthChallengeTokenLookup) -> Result<(), PersistenceError> {
    validate_client_context(&lookup.client_context)?;
    if lookup.now_ms < 0 {
        return Err(PersistenceError::InvalidTimestamp);
    }
    Ok(())
}

fn validate_access(access: &AuthChallengeAccess) -> Result<(), PersistenceError> {
    database_key_version(access.token_key_version)?;
    validate_client_context(&access.client_context)?;
    if access.now_ms < 0 {
        return Err(PersistenceError::InvalidTimestamp);
    }
    Ok(())
}

fn validate_client_context(context: &AuthChallengeClientContext) -> Result<(), PersistenceError> {
    if !matches!(
        (
            context.key_version,
            context.client_network_hmac,
            context.user_agent_hash
        ),
        (None, None, None) | (Some(_), Some(_), Some(_))
    ) {
        return Err(PersistenceError::InvalidAuthChallenge);
    }
    if let Some(key_version) = context.key_version {
        database_key_version(key_version)?;
    }
    Ok(())
}

fn canonical_methods(
    methods: &[AuthenticationMethod],
) -> Result<Vec<AuthenticationMethod>, PersistenceError> {
    if methods.is_empty() || methods.len() > 4 {
        return Err(PersistenceError::InvalidAuthChallenge);
    }
    let mut canonical = methods.to_vec();
    canonical.sort_unstable_by_key(|method| method.as_str());
    let original_len = canonical.len();
    canonical.dedup();
    if canonical.len() != original_len {
        return Err(PersistenceError::InvalidAuthChallenge);
    }
    Ok(canonical)
}

fn decode_token_digest(row: TokenDigestRow) -> Result<KeyedDigest, PersistenceError> {
    let key_version = u32::try_from(row.token_key_version)
        .map_err(|_| PersistenceError::InvalidStoredAuthChallenge)?;
    database_key_version(key_version)?;
    let digest = row
        .token_hmac
        .try_into()
        .map_err(|_| PersistenceError::InvalidStoredAuthChallenge)?;
    Ok(KeyedDigest {
        key_version,
        digest,
    })
}

fn decode_challenge(
    row: ChallengeRow,
    encoded_methods: Vec<String>,
) -> Result<AuthChallenge, PersistenceError> {
    let allowed_methods = encoded_methods
        .into_iter()
        .map(|method| {
            AuthenticationMethod::parse(&method)
                .map_err(|_| PersistenceError::InvalidStoredAuthChallenge)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let canonical = canonical_methods(&allowed_methods)
        .map_err(|_| PersistenceError::InvalidStoredAuthChallenge)?;
    if canonical != allowed_methods {
        return Err(PersistenceError::InvalidStoredAuthChallenge);
    }
    let attempts_used = u32::try_from(row.attempts_used)
        .map_err(|_| PersistenceError::InvalidStoredAuthChallenge)?;
    let max_attempts = u32::try_from(row.max_attempts)
        .map_err(|_| PersistenceError::InvalidStoredAuthChallenge)?;
    let status = AuthChallengeStatus::parse(&row.status)
        .map_err(|_| PersistenceError::InvalidStoredAuthChallenge)?;
    let verified_method = row
        .verified_method
        .as_deref()
        .map(AuthenticationMethod::parse)
        .transpose()
        .map_err(|_| PersistenceError::InvalidStoredAuthChallenge)?;
    let achieved_assurance = row
        .achieved_assurance
        .as_deref()
        .map(AuthenticationAssurance::parse)
        .transpose()
        .map_err(|_| PersistenceError::InvalidStoredAuthChallenge)?;
    let has_attempt_claim = row.has_attempt_claim == 1;
    if attempts_used > max_attempts
        || (status == AuthChallengeStatus::VerificationPending && !has_attempt_claim)
        || (!matches!(
            status,
            AuthChallengeStatus::VerificationPending | AuthChallengeStatus::RotationPending
        ) && has_attempt_claim)
        || !matches!(
            (verified_method, achieved_assurance),
            (None, None) | (Some(_), Some(_))
        )
        || verified_method.is_some_and(|method| !allowed_methods.contains(&method))
        || matches!(
            (verified_method, achieved_assurance),
            (Some(method), Some(assurance)) if !method.permits_assurance(assurance)
        )
    {
        return Err(PersistenceError::InvalidStoredAuthChallenge);
    }
    Ok(AuthChallenge {
        id: EntityId::from_uuid(uuid::Uuid::parse_str(&row.id)?),
        purpose: AuthChallengePurpose::parse(&row.purpose)
            .map_err(|_| PersistenceError::InvalidStoredAuthChallenge)?,
        user_id: EntityId::from_uuid(uuid::Uuid::parse_str(&row.user_id)?),
        session_id: row
            .session_id
            .as_deref()
            .map(uuid::Uuid::parse_str)
            .transpose()?
            .map(EntityId::from_uuid),
        auth_revision: super::decode_revision(row.auth_revision)?,
        allowed_methods,
        status,
        rotation_state: AuthChallengeRotationState::parse(&row.rotation_state)
            .map_err(|_| PersistenceError::InvalidStoredAuthChallenge)?,
        attempts_used,
        max_attempts,
        created_at_ms: row.created_at_ms,
        expires_at_ms: row.expires_at_ms,
        verified_method,
        achieved_assurance,
        consumed_at_ms: row.consumed_at_ms,
        verification_in_progress: status == AuthChallengeStatus::VerificationPending,
        rotation_transaction_in_progress: status == AuthChallengeStatus::RotationPending
            && has_attempt_claim,
        has_client_network_context: row.has_client_network_context == 1,
        has_user_agent_context: row.has_user_agent_context == 1,
        revision: super::decode_revision(row.revision)?,
    })
}

async fn refresh_access_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    access: &AuthChallengeAccess,
    token_key_version: i32,
) -> Result<(), PersistenceError> {
    let context_key_version = access
        .client_context
        .key_version
        .map(database_key_version)
        .transpose()?;
    let network = access
        .client_context
        .client_network_hmac
        .as_ref()
        .map(|value| value.as_slice());
    let user_agent = access
        .client_context
        .user_agent_hash
        .as_ref()
        .map(|value| value.as_slice());
    sqlx::query(
        "UPDATE auth_challenges SET status='expired',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=? WHERE id=? AND token_key_version=? AND token_hmac=? AND context_key_version IS ? AND client_network_hmac IS ? AND user_agent_hash IS ? AND status IN ('pending','verification_pending','rotation_pending','exhausted') AND created_at_ms<=? AND expires_at_ms<=?"
    )
    .bind(access.now_ms)
    .bind(access.id.to_string())
    .bind(token_key_version)
    .bind(access.token_hmac.as_slice())
    .bind(context_key_version)
    .bind(network)
    .bind(user_agent)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET status='invalidated',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=? WHERE id=? AND token_key_version=? AND token_hmac=? AND context_key_version IS ? AND client_network_hmac IS ? AND user_agent_hash IS ? AND status IN ('pending','verification_pending','rotation_pending','exhausted') AND created_at_ms<=? AND expires_at_ms>? AND (NOT EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) OR (session_id IS NOT NULL AND NOT EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?)))"
    )
    .bind(access.now_ms)
    .bind(access.id.to_string())
    .bind(token_key_version)
    .bind(access.token_hmac.as_slice())
    .bind(context_key_version)
    .bind(network)
    .bind(user_agent)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET status=CASE WHEN attempts_used=max_attempts THEN 'exhausted' ELSE 'pending' END,attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=? WHERE id=? AND token_key_version=? AND token_hmac=? AND context_key_version IS ? AND client_network_hmac IS ? AND user_agent_hash IS ? AND status='verification_pending' AND created_at_ms<=? AND expires_at_ms>? AND attempt_expires_at_ms<=?",
    )
    .bind(access.now_ms)
    .bind(access.id.to_string())
    .bind(token_key_version)
    .bind(access.token_hmac.as_slice())
    .bind(context_key_version)
    .bind(network)
    .bind(user_agent)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=? WHERE id=? AND token_key_version=? AND token_hmac=? AND context_key_version IS ? AND client_network_hmac IS ? AND user_agent_hash IS ? AND status='rotation_pending' AND attempt_claim_id IS NOT NULL AND created_at_ms<=? AND expires_at_ms>? AND attempt_expires_at_ms<=?",
    )
    .bind(access.now_ms)
    .bind(access.id.to_string())
    .bind(token_key_version)
    .bind(access.token_hmac.as_slice())
    .bind(context_key_version)
    .bind(network)
    .bind(user_agent)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn refresh_access_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    access: &AuthChallengeAccess,
    token_key_version: i32,
) -> Result<(), PersistenceError> {
    let context_key_version = access
        .client_context
        .key_version
        .map(database_key_version)
        .transpose()?;
    let network = access
        .client_context
        .client_network_hmac
        .as_ref()
        .map(|value| value.as_slice());
    let user_agent = access
        .client_context
        .user_agent_hash
        .as_ref()
        .map(|value| value.as_slice());
    sqlx::query(
        "UPDATE auth_challenges SET status='expired',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=$1 WHERE id=$2 AND token_key_version=$3 AND token_hmac=$4 AND context_key_version IS NOT DISTINCT FROM $5 AND client_network_hmac IS NOT DISTINCT FROM $6 AND user_agent_hash IS NOT DISTINCT FROM $7 AND status IN ('pending','verification_pending','rotation_pending','exhausted') AND created_at_ms<=$8 AND expires_at_ms<=$9"
    )
    .bind(access.now_ms)
    .bind(access.id.into_uuid())
    .bind(token_key_version)
    .bind(access.token_hmac.as_slice())
    .bind(context_key_version)
    .bind(network)
    .bind(user_agent)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET status='invalidated',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=$1 WHERE id=$2 AND token_key_version=$3 AND token_hmac=$4 AND context_key_version IS NOT DISTINCT FROM $5 AND client_network_hmac IS NOT DISTINCT FROM $6 AND user_agent_hash IS NOT DISTINCT FROM $7 AND status IN ('pending','verification_pending','rotation_pending','exhausted') AND created_at_ms<=$8 AND expires_at_ms>$9 AND (NOT EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) OR (session_id IS NOT NULL AND NOT EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=$10 AND s.idle_expires_at_ms>$11 AND s.absolute_expires_at_ms>$12)))"
    )
    .bind(access.now_ms)
    .bind(access.id.into_uuid())
    .bind(token_key_version)
    .bind(access.token_hmac.as_slice())
    .bind(context_key_version)
    .bind(network)
    .bind(user_agent)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET status=CASE WHEN attempts_used=max_attempts THEN 'exhausted' ELSE 'pending' END,attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=$1 WHERE id=$2 AND token_key_version=$3 AND token_hmac=$4 AND context_key_version IS NOT DISTINCT FROM $5 AND client_network_hmac IS NOT DISTINCT FROM $6 AND user_agent_hash IS NOT DISTINCT FROM $7 AND status='verification_pending' AND created_at_ms<=$8 AND expires_at_ms>$9 AND attempt_expires_at_ms<=$10",
    )
    .bind(access.now_ms)
    .bind(access.id.into_uuid())
    .bind(token_key_version)
    .bind(access.token_hmac.as_slice())
    .bind(context_key_version)
    .bind(network)
    .bind(user_agent)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=$1 WHERE id=$2 AND token_key_version=$3 AND token_hmac=$4 AND context_key_version IS NOT DISTINCT FROM $5 AND client_network_hmac IS NOT DISTINCT FROM $6 AND user_agent_hash IS NOT DISTINCT FROM $7 AND status='rotation_pending' AND attempt_claim_id IS NOT NULL AND created_at_ms<=$8 AND expires_at_ms>$9 AND attempt_expires_at_ms<=$10",
    )
    .bind(access.now_ms)
    .bind(access.id.into_uuid())
    .bind(token_key_version)
    .bind(access.token_hmac.as_slice())
    .bind(context_key_version)
    .bind(network)
    .bind(user_agent)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .bind(access.now_ms)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn refresh_user_purpose_sqlite(
    transaction: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    user_id: EntityId,
    purpose: AuthChallengePurpose,
    now_ms: i64,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "UPDATE auth_challenges SET status='expired',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=? WHERE user_id=? AND purpose=? AND status IN ('pending','verification_pending','rotation_pending','exhausted') AND created_at_ms<=? AND expires_at_ms<=?"
    )
    .bind(now_ms)
    .bind(user_id.to_string())
    .bind(purpose.as_str())
    .bind(now_ms)
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET status='invalidated',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=? WHERE user_id=? AND purpose=? AND status IN ('pending','verification_pending','rotation_pending','exhausted') AND created_at_ms<=? AND expires_at_ms>? AND (NOT EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) OR (session_id IS NOT NULL AND NOT EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?)))"
    )
    .bind(now_ms)
    .bind(user_id.to_string())
    .bind(purpose.as_str())
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET status=CASE WHEN attempts_used=max_attempts THEN 'exhausted' ELSE 'pending' END,attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=? WHERE user_id=? AND purpose=? AND status='verification_pending' AND created_at_ms<=? AND expires_at_ms>? AND attempt_expires_at_ms<=?",
    )
    .bind(now_ms)
    .bind(user_id.to_string())
    .bind(purpose.as_str())
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=? WHERE user_id=? AND purpose=? AND status='rotation_pending' AND attempt_claim_id IS NOT NULL AND created_at_ms<=? AND expires_at_ms>? AND attempt_expires_at_ms<=?",
    )
    .bind(now_ms)
    .bind(user_id.to_string())
    .bind(purpose.as_str())
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

async fn refresh_user_purpose_postgres(
    transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    user_id: EntityId,
    purpose: AuthChallengePurpose,
    now_ms: i64,
) -> Result<(), PersistenceError> {
    sqlx::query(
        "UPDATE auth_challenges SET status='expired',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=$1 WHERE user_id=$2 AND purpose=$3 AND status IN ('pending','verification_pending','rotation_pending','exhausted') AND created_at_ms<=$4 AND expires_at_ms<=$5"
    )
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .bind(purpose.as_str())
    .bind(now_ms)
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET status='invalidated',attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=$1 WHERE user_id=$2 AND purpose=$3 AND status IN ('pending','verification_pending','rotation_pending','exhausted') AND created_at_ms<=$4 AND expires_at_ms>$5 AND (NOT EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) OR (session_id IS NOT NULL AND NOT EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=$6 AND s.idle_expires_at_ms>$7 AND s.absolute_expires_at_ms>$8)))"
    )
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .bind(purpose.as_str())
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET status=CASE WHEN attempts_used=max_attempts THEN 'exhausted' ELSE 'pending' END,attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=$1 WHERE user_id=$2 AND purpose=$3 AND status='verification_pending' AND created_at_ms<=$4 AND expires_at_ms>$5 AND attempt_expires_at_ms<=$6",
    )
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .bind(purpose.as_str())
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    sqlx::query(
        "UPDATE auth_challenges SET attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=$1 WHERE user_id=$2 AND purpose=$3 AND status='rotation_pending' AND attempt_claim_id IS NOT NULL AND created_at_ms<=$4 AND expires_at_ms>$5 AND attempt_expires_at_ms<=$6",
    )
    .bind(now_ms)
    .bind(user_id.into_uuid())
    .bind(purpose.as_str())
    .bind(now_ms)
    .bind(now_ms)
    .bind(now_ms)
    .execute(&mut **transaction)
    .await?;
    Ok(())
}

impl Database {
    /// Atomically owns one attempt slot before a method verifier sees the proof.
    ///
    /// A single revision can produce at most one live claim. Losing callers receive `Stale` and
    /// must not verify or replay their proof. A timed-out claim is recovered by `refresh_access_*`;
    /// its already-reserved slot remains consumed.
    pub async fn reserve_auth_challenge_attempt(
        &self,
        reservation: &AuthChallengeAttemptReservation,
    ) -> Result<AuthChallengeAttemptReservationOutcome, PersistenceError> {
        validate_access(&reservation.access)?;
        if reservation.verification_expires_at_ms <= reservation.access.now_ms {
            return Err(PersistenceError::InvalidAuthChallenge);
        }
        let key_version = database_key_version(reservation.access.token_key_version)?;
        let context_key_version = reservation
            .access
            .client_context
            .key_version
            .map(database_key_version)
            .transpose()?;
        let expected_revision = database_revision(reservation.expected_revision)?;
        let rows_affected = match self {
            Self::Sqlite(pool) => {
                let mut transaction = pool.begin().await?;
                refresh_access_sqlite(&mut transaction, &reservation.access, key_version).await?;
                let rows = sqlx::query(
                    "UPDATE auth_challenges SET status='verification_pending',attempts_used=attempts_used+1,attempt_claim_id=?,attempted_method=?,attempt_started_at_ms=?,attempt_expires_at_ms=MIN(expires_at_ms,?),revision=revision+1,updated_at_ms=? WHERE id=? AND token_key_version=? AND token_hmac=? AND context_key_version IS ? AND client_network_hmac IS ? AND user_agent_hash IS ? AND revision=? AND status='pending' AND attempts_used<max_attempts AND created_at_ms<=? AND expires_at_ms>? AND EXISTS(SELECT 1 FROM auth_challenge_methods AS m WHERE m.challenge_id=auth_challenges.id AND m.method=?) AND EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) AND (session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?))",
                )
                .bind(reservation.claim_id.to_string())
                .bind(reservation.method.as_str())
                .bind(reservation.access.now_ms)
                .bind(reservation.verification_expires_at_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.id.to_string())
                .bind(key_version)
                .bind(reservation.access.token_hmac.as_slice())
                .bind(context_key_version)
                .bind(
                    reservation
                        .access
                        .client_context
                        .client_network_hmac
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(
                    reservation
                        .access
                        .client_context
                        .user_agent_hash
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(expected_revision)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.method.as_str())
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
                transaction.commit().await?;
                rows
            }
            Self::Postgres(pool) => {
                let mut transaction = pool.begin().await?;
                refresh_access_postgres(&mut transaction, &reservation.access, key_version).await?;
                let rows = sqlx::query(
                    "UPDATE auth_challenges SET status='verification_pending',attempts_used=attempts_used+1,attempt_claim_id=$1,attempted_method=$2,attempt_started_at_ms=$3,attempt_expires_at_ms=LEAST(expires_at_ms,$4),revision=revision+1,updated_at_ms=$5 WHERE id=$6 AND token_key_version=$7 AND token_hmac=$8 AND context_key_version IS NOT DISTINCT FROM $9 AND client_network_hmac IS NOT DISTINCT FROM $10 AND user_agent_hash IS NOT DISTINCT FROM $11 AND revision=$12 AND status='pending' AND attempts_used<max_attempts AND created_at_ms<=$13 AND expires_at_ms>$14 AND EXISTS(SELECT 1 FROM auth_challenge_methods AS m WHERE m.challenge_id=auth_challenges.id AND m.method=$15) AND EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) AND (session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=$16 AND s.idle_expires_at_ms>$17 AND s.absolute_expires_at_ms>$18))",
                )
                .bind(reservation.claim_id.into_uuid())
                .bind(reservation.method.as_str())
                .bind(reservation.access.now_ms)
                .bind(reservation.verification_expires_at_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.id.into_uuid())
                .bind(key_version)
                .bind(reservation.access.token_hmac.as_slice())
                .bind(context_key_version)
                .bind(
                    reservation
                        .access
                        .client_context
                        .client_network_hmac
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(
                    reservation
                        .access
                        .client_context
                        .user_agent_hash
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(expected_revision)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.method.as_str())
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .bind(reservation.access.now_ms)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
                transaction.commit().await?;
                rows
            }
        };
        if rows_affected != 1 {
            return Ok(AuthChallengeAttemptReservationOutcome::Stale);
        }
        match self.auth_challenge(&reservation.access).await? {
            Some(challenge)
                if challenge.status == AuthChallengeStatus::VerificationPending
                    && challenge.verification_in_progress =>
            {
                Ok(AuthChallengeAttemptReservationOutcome::Reserved(challenge))
            }
            _ => Err(PersistenceError::InvalidStoredAuthChallenge),
        }
    }

    /// Finalizes only the claim that actually ran the verifier. A final-slot failure transitions
    /// to exhausted; a final-slot success is handled separately and remains valid.
    pub async fn record_auth_challenge_failure(
        &self,
        failure: &AuthChallengeAttemptFailure,
    ) -> Result<AuthChallengeAttemptOutcome, PersistenceError> {
        validate_access(&failure.access)?;
        let key_version = database_key_version(failure.access.token_key_version)?;
        let context_key_version = failure
            .access
            .client_context
            .key_version
            .map(database_key_version)
            .transpose()?;
        let expected_revision = database_revision(failure.expected_revision)?;
        let rows_affected = match self {
            Self::Sqlite(pool) => {
                let mut transaction = pool.begin().await?;
                refresh_access_sqlite(&mut transaction, &failure.access, key_version).await?;
                let rows = sqlx::query(
                    "UPDATE auth_challenges SET status=CASE WHEN attempts_used=max_attempts THEN 'exhausted' ELSE 'pending' END,attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=? WHERE id=? AND token_key_version=? AND token_hmac=? AND context_key_version IS ? AND client_network_hmac IS ? AND user_agent_hash IS ? AND revision=? AND status='verification_pending' AND attempt_claim_id=? AND attempted_method=? AND attempt_expires_at_ms>? AND expires_at_ms>? AND EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) AND (session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?))",
                )
                .bind(failure.access.now_ms)
                .bind(failure.access.id.to_string())
                .bind(key_version)
                .bind(failure.access.token_hmac.as_slice())
                .bind(context_key_version)
                .bind(
                    failure
                        .access
                        .client_context
                        .client_network_hmac
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(
                    failure
                        .access
                        .client_context
                        .user_agent_hash
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(expected_revision)
                .bind(failure.claim_id.to_string())
                .bind(failure.method.as_str())
                .bind(failure.access.now_ms)
                .bind(failure.access.now_ms)
                .bind(failure.access.now_ms)
                .bind(failure.access.now_ms)
                .bind(failure.access.now_ms)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
                transaction.commit().await?;
                rows
            }
            Self::Postgres(pool) => {
                let mut transaction = pool.begin().await?;
                refresh_access_postgres(&mut transaction, &failure.access, key_version).await?;
                let rows = sqlx::query(
                    "UPDATE auth_challenges SET status=CASE WHEN attempts_used=max_attempts THEN 'exhausted' ELSE 'pending' END,attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,revision=revision+1,updated_at_ms=$1 WHERE id=$2 AND token_key_version=$3 AND token_hmac=$4 AND context_key_version IS NOT DISTINCT FROM $5 AND client_network_hmac IS NOT DISTINCT FROM $6 AND user_agent_hash IS NOT DISTINCT FROM $7 AND revision=$8 AND status='verification_pending' AND attempt_claim_id=$9 AND attempted_method=$10 AND attempt_expires_at_ms>$11 AND expires_at_ms>$12 AND EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) AND (session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=$13 AND s.idle_expires_at_ms>$14 AND s.absolute_expires_at_ms>$15))",
                )
                .bind(failure.access.now_ms)
                .bind(failure.access.id.into_uuid())
                .bind(key_version)
                .bind(failure.access.token_hmac.as_slice())
                .bind(context_key_version)
                .bind(
                    failure
                        .access
                        .client_context
                        .client_network_hmac
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(
                    failure
                        .access
                        .client_context
                        .user_agent_hash
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(expected_revision)
                .bind(failure.claim_id.into_uuid())
                .bind(failure.method.as_str())
                .bind(failure.access.now_ms)
                .bind(failure.access.now_ms)
                .bind(failure.access.now_ms)
                .bind(failure.access.now_ms)
                .bind(failure.access.now_ms)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
                transaction.commit().await?;
                rows
            }
        };
        if rows_affected != 1 {
            return Ok(AuthChallengeAttemptOutcome::Stale);
        }
        let Some(challenge) = self.auth_challenge(&failure.access).await? else {
            return Err(PersistenceError::InvalidStoredAuthChallenge);
        };
        match challenge.status {
            AuthChallengeStatus::Pending => Ok(AuthChallengeAttemptOutcome::Retryable(challenge)),
            AuthChallengeStatus::Exhausted => Ok(AuthChallengeAttemptOutcome::Exhausted(challenge)),
            _ => Err(PersistenceError::InvalidStoredAuthChallenge),
        }
    }

    pub async fn begin_auth_challenge_consumption(
        &self,
        consumption: &AuthChallengeConsumption,
    ) -> Result<AuthChallengeConsumptionOutcome, PersistenceError> {
        validate_access(&consumption.access)?;
        if !consumption
            .method
            .permits_assurance(consumption.achieved_assurance)
        {
            return Err(PersistenceError::InvalidAuthChallenge);
        }
        let key_version = database_key_version(consumption.access.token_key_version)?;
        let context_key_version = consumption
            .access
            .client_context
            .key_version
            .map(database_key_version)
            .transpose()?;
        let expected_revision = database_revision(consumption.expected_revision)?;
        let rows_affected = match self {
            Self::Sqlite(pool) => {
                let mut transaction = pool.begin().await?;
                refresh_access_sqlite(&mut transaction, &consumption.access, key_version).await?;
                let rows = sqlx::query(
                    "UPDATE auth_challenges SET status=CASE WHEN rotation_state='required' THEN 'rotation_pending' ELSE 'consumed' END,rotation_state=CASE WHEN rotation_state='required' THEN 'pending' ELSE 'not_required' END,attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,verified_method=?,achieved_assurance=?,consumed_at_ms=CASE WHEN rotation_state='required' THEN NULL ELSE ? END,revision=revision+1,updated_at_ms=? WHERE id=? AND token_key_version=? AND token_hmac=? AND context_key_version IS ? AND client_network_hmac IS ? AND user_agent_hash IS ? AND revision=? AND status='verification_pending' AND attempt_claim_id=? AND attempted_method=? AND attempt_expires_at_ms>? AND expires_at_ms>? AND EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) AND (session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=? AND s.idle_expires_at_ms>? AND s.absolute_expires_at_ms>?))",
                )
                .bind(consumption.method.as_str())
                .bind(consumption.achieved_assurance.as_str())
                .bind(consumption.access.now_ms)
                .bind(consumption.access.now_ms)
                .bind(consumption.access.id.to_string())
                .bind(key_version)
                .bind(consumption.access.token_hmac.as_slice())
                .bind(context_key_version)
                .bind(
                    consumption
                        .access
                        .client_context
                        .client_network_hmac
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(
                    consumption
                        .access
                        .client_context
                        .user_agent_hash
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(expected_revision)
                .bind(consumption.claim_id.to_string())
                .bind(consumption.method.as_str())
                .bind(consumption.access.now_ms)
                .bind(consumption.access.now_ms)
                .bind(consumption.access.now_ms)
                .bind(consumption.access.now_ms)
                .bind(consumption.access.now_ms)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
                transaction.commit().await?;
                rows
            }
            Self::Postgres(pool) => {
                let mut transaction = pool.begin().await?;
                refresh_access_postgres(&mut transaction, &consumption.access, key_version).await?;
                let rows = sqlx::query(
                    "UPDATE auth_challenges SET status=CASE WHEN rotation_state='required' THEN 'rotation_pending' ELSE 'consumed' END,rotation_state=CASE WHEN rotation_state='required' THEN 'pending' ELSE 'not_required' END,attempt_claim_id=NULL,attempted_method=NULL,attempt_started_at_ms=NULL,attempt_expires_at_ms=NULL,verified_method=$1,achieved_assurance=$2,consumed_at_ms=CASE WHEN rotation_state='required' THEN NULL ELSE $3 END,revision=revision+1,updated_at_ms=$4 WHERE id=$5 AND token_key_version=$6 AND token_hmac=$7 AND context_key_version IS NOT DISTINCT FROM $8 AND client_network_hmac IS NOT DISTINCT FROM $9 AND user_agent_hash IS NOT DISTINCT FROM $10 AND revision=$11 AND status='verification_pending' AND attempt_claim_id=$12 AND attempted_method=$13 AND attempt_expires_at_ms>$14 AND expires_at_ms>$15 AND EXISTS(SELECT 1 FROM users AS u JOIN user_auth_state AS uas ON uas.user_id=u.id WHERE u.id=auth_challenges.user_id AND u.status='active' AND u.deleted_at_ms IS NULL AND uas.auth_revision=auth_challenges.auth_revision) AND (session_id IS NULL OR EXISTS(SELECT 1 FROM auth_sessions AS s WHERE s.id=auth_challenges.session_id AND s.user_id=auth_challenges.user_id AND s.status='active' AND s.auth_revision=auth_challenges.auth_revision AND s.last_seen_at_ms<=$16 AND s.idle_expires_at_ms>$17 AND s.absolute_expires_at_ms>$18))",
                )
                .bind(consumption.method.as_str())
                .bind(consumption.achieved_assurance.as_str())
                .bind(consumption.access.now_ms)
                .bind(consumption.access.now_ms)
                .bind(consumption.access.id.into_uuid())
                .bind(key_version)
                .bind(consumption.access.token_hmac.as_slice())
                .bind(context_key_version)
                .bind(
                    consumption
                        .access
                        .client_context
                        .client_network_hmac
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(
                    consumption
                        .access
                        .client_context
                        .user_agent_hash
                        .as_ref()
                        .map(|value| value.as_slice()),
                )
                .bind(expected_revision)
                .bind(consumption.claim_id.into_uuid())
                .bind(consumption.method.as_str())
                .bind(consumption.access.now_ms)
                .bind(consumption.access.now_ms)
                .bind(consumption.access.now_ms)
                .bind(consumption.access.now_ms)
                .bind(consumption.access.now_ms)
                .execute(&mut *transaction)
                .await?
                .rows_affected();
                transaction.commit().await?;
                rows
            }
        };
        if rows_affected != 1 {
            return Ok(AuthChallengeConsumptionOutcome::Stale);
        }
        let Some(challenge) = self.auth_challenge(&consumption.access).await? else {
            return Err(PersistenceError::InvalidStoredAuthChallenge);
        };
        match challenge.status {
            AuthChallengeStatus::Consumed => {
                Ok(AuthChallengeConsumptionOutcome::Consumed(challenge))
            }
            AuthChallengeStatus::RotationPending => {
                Ok(AuthChallengeConsumptionOutcome::RotationPending(challenge))
            }
            _ => Err(PersistenceError::InvalidStoredAuthChallenge),
        }
    }
}
