use async_trait::async_trait;
use nodecontroll_domain::{
    AuthChallenge, AuthChallengePolicy, AuthChallengePurpose, AuthenticationAssurance,
    AuthenticationMethod, EntityId, Revision,
};
use nodecontroll_persistence::{
    AuthChallengeAccess, AuthChallengeAttemptFailure, AuthChallengeAttemptOutcome,
    AuthChallengeAttemptReservation, AuthChallengeAttemptReservationOutcome,
    AuthChallengeClientContext, AuthChallengeConsumption, AuthChallengeConsumptionOutcome,
    AuthChallengeRotationReservation, AuthChallengeRotationReservationOutcome,
    AuthChallengeTokenLookup, CreateAuthChallengeOutcome, Database, NewAuthChallenge,
    PersistenceError,
};
use nodecontroll_secrets::{AuthChallengeToken, KeyedDigest, Keyring};
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IssueAuthChallengeCommand {
    pub purpose: AuthChallengePurpose,
    pub user_id: EntityId,
    pub session_id: Option<EntityId>,
    pub auth_revision: Revision,
    pub allowed_methods: Vec<AuthenticationMethod>,
    pub rotation_required: bool,
    pub client_context: AuthChallengeClientContext,
    pub created_at_ms: i64,
}

/// The plaintext bearer exists only in this one-shot result and is zeroized when dropped.
/// It is never part of a persistence model or a cloneable/debuggable application result.
pub struct IssuedAuthChallenge {
    pub challenge: AuthChallenge,
    pub token: AuthChallengeToken,
}

/// A canonical bearer plus the request context calculated by the trusted HTTP boundary.
/// `AuthChallengeToken` is neither cloneable nor debuggable and zeroizes its plaintext.
pub struct PresentAuthChallengeCommand {
    pub id: EntityId,
    pub token: AuthChallengeToken,
    pub client_context: AuthChallengeClientContext,
    pub now_ms: i64,
}

/// An unforgeable application capability proving that this request atomically reserved one slot.
/// Fields remain private so an HTTP DTO cannot manufacture a claim token or revision.
pub struct AuthChallengeVerificationClaim {
    access: AuthChallengeAccess,
    claim_id: EntityId,
    method: AuthenticationMethod,
    challenge: AuthChallenge,
}

impl AuthChallengeVerificationClaim {
    #[must_use]
    pub const fn method(&self) -> AuthenticationMethod {
        self.method
    }

    #[must_use]
    pub const fn challenge(&self) -> &AuthChallenge {
        &self.challenge
    }

    /// Trusted method verifiers must reject a clock value older than the durable reservation
    /// before consuming method-specific replay state. This remains crate-private so transport
    /// callers cannot use it to synthesize or alter a verification claim.
    #[must_use]
    pub(crate) const fn reserved_at_ms(&self) -> i64 {
        self.access.now_ms
    }

    #[cfg(test)]
    pub(crate) fn test_fixture(
        challenge: AuthChallenge,
        method: AuthenticationMethod,
        reserved_at_ms: i64,
    ) -> Self {
        Self {
            access: AuthChallengeAccess {
                id: challenge.id,
                token_key_version: 1,
                token_hmac: [7; 32],
                client_context: AuthChallengeClientContext::unbound(),
                now_ms: reserved_at_ms,
            },
            claim_id: EntityId::new(),
            method,
            challenge,
        }
    }
}

/// Accepted evidence can only be emitted by a method verifier inside the application crate.
/// In particular, no public constructor accepts an arbitrary method/assurance pair.
pub struct VerifiedAuthChallengeEvidence {
    claim: AuthChallengeVerificationClaim,
    achieved_assurance: AuthenticationAssurance,
}

impl VerifiedAuthChallengeEvidence {
    pub(crate) fn from_method_verifier(
        claim: AuthChallengeVerificationClaim,
        achieved_assurance: AuthenticationAssurance,
    ) -> Result<Self, AuthChallengeServiceError> {
        if !claim.method.permits_assurance(achieved_assurance) {
            return Err(AuthChallengeServiceError::InvalidEvidence);
        }
        Ok(Self {
            claim,
            achieved_assurance,
        })
    }
}

pub enum AuthChallengeReservationOutcome {
    Reserved(Box<AuthChallengeVerificationClaim>),
    NotFound,
    Stale,
}

/// Successful recovery-code proof cannot be detached from replacement-session creation.
/// The handle is created only from the durable `rotation_pending` transition.
pub struct AuthChallengeRotationTransactionClaim {
    access: AuthChallengeAccess,
    transaction_claim_id: EntityId,
    expected_revision: Revision,
    challenge: AuthChallenge,
}

impl AuthChallengeRotationTransactionClaim {
    #[must_use]
    pub const fn challenge(&self) -> &AuthChallenge {
        &self.challenge
    }

    /// Infrastructure implementations must CAS all three values before writing the replacement
    /// session/event, and commit that CAS with those writes. A lease expiry or competing resume
    /// changes the revision/claim and makes the whole transaction lose. There is intentionally no
    /// standalone completion API.
    #[must_use]
    pub const fn transaction_binding(&self) -> (&AuthChallengeAccess, EntityId, Revision) {
        (
            &self.access,
            self.transaction_claim_id,
            self.expected_revision,
        )
    }
}

pub enum AuthChallengeRotationResumeOutcome {
    Ready(Box<AuthChallengeRotationTransactionClaim>),
    NotFound,
    Stale,
}

pub enum VerifiedAuthChallengeOutcome {
    Consumed(AuthChallenge),
    RotationRequired(AuthChallengeRotationTransactionClaim),
    Stale,
}

/// Future session infrastructure implements this seam by first claiming the bound revision/token,
/// then creating the replacement session and event and consuming `rotation_pending`, all under one
/// database transaction. C3 deliberately provides no `Database` implementation and no API that
/// can mark rotation complete by itself.
#[async_trait]
pub trait AuthChallengeRotationTransactionPort: Send + Sync {
    type ReplacementSession: Send;
    type Output: Send;

    async fn replace_session_and_consume_atomically(
        &self,
        claim: AuthChallengeRotationTransactionClaim,
        replacement: Self::ReplacementSession,
        completed_at_ms: i64,
    ) -> Result<Self::Output, AuthChallengePortError>;
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum AuthChallengePortError {
    #[error("authentication challenge input is invalid")]
    InvalidInput,
    #[error("authentication challenge persistence is unavailable")]
    Unavailable,
}

#[async_trait]
pub trait AuthChallengePort: Send + Sync {
    async fn create(
        &self,
        challenge: NewAuthChallenge,
    ) -> Result<CreateAuthChallengeOutcome, AuthChallengePortError>;

    async fn token_digest(
        &self,
        lookup: AuthChallengeTokenLookup,
    ) -> Result<Option<KeyedDigest>, AuthChallengePortError>;

    async fn load(
        &self,
        access: AuthChallengeAccess,
    ) -> Result<Option<AuthChallenge>, AuthChallengePortError>;

    async fn reserve_attempt(
        &self,
        reservation: AuthChallengeAttemptReservation,
    ) -> Result<AuthChallengeAttemptReservationOutcome, AuthChallengePortError>;

    async fn record_failure(
        &self,
        failure: AuthChallengeAttemptFailure,
    ) -> Result<AuthChallengeAttemptOutcome, AuthChallengePortError>;

    async fn begin_consumption(
        &self,
        consumption: AuthChallengeConsumption,
    ) -> Result<AuthChallengeConsumptionOutcome, AuthChallengePortError>;

    async fn reserve_rotation(
        &self,
        reservation: AuthChallengeRotationReservation,
    ) -> Result<AuthChallengeRotationReservationOutcome, AuthChallengePortError>;
}

#[async_trait]
impl AuthChallengePort for Database {
    async fn create(
        &self,
        challenge: NewAuthChallenge,
    ) -> Result<CreateAuthChallengeOutcome, AuthChallengePortError> {
        self.create_auth_challenge(&challenge)
            .await
            .map_err(map_persistence_error)
    }

    async fn token_digest(
        &self,
        lookup: AuthChallengeTokenLookup,
    ) -> Result<Option<KeyedDigest>, AuthChallengePortError> {
        self.auth_challenge_token_digest(&lookup)
            .await
            .map_err(map_persistence_error)
    }

    async fn load(
        &self,
        access: AuthChallengeAccess,
    ) -> Result<Option<AuthChallenge>, AuthChallengePortError> {
        self.auth_challenge(&access)
            .await
            .map_err(map_persistence_error)
    }

    async fn reserve_attempt(
        &self,
        reservation: AuthChallengeAttemptReservation,
    ) -> Result<AuthChallengeAttemptReservationOutcome, AuthChallengePortError> {
        self.reserve_auth_challenge_attempt(&reservation)
            .await
            .map_err(map_persistence_error)
    }

    async fn record_failure(
        &self,
        failure: AuthChallengeAttemptFailure,
    ) -> Result<AuthChallengeAttemptOutcome, AuthChallengePortError> {
        self.record_auth_challenge_failure(&failure)
            .await
            .map_err(map_persistence_error)
    }

    async fn begin_consumption(
        &self,
        consumption: AuthChallengeConsumption,
    ) -> Result<AuthChallengeConsumptionOutcome, AuthChallengePortError> {
        self.begin_auth_challenge_consumption(&consumption)
            .await
            .map_err(map_persistence_error)
    }

    async fn reserve_rotation(
        &self,
        reservation: AuthChallengeRotationReservation,
    ) -> Result<AuthChallengeRotationReservationOutcome, AuthChallengePortError> {
        self.reserve_auth_challenge_rotation(&reservation)
            .await
            .map_err(map_persistence_error)
    }
}

fn map_persistence_error(error: PersistenceError) -> AuthChallengePortError {
    match error {
        PersistenceError::InvalidAuthChallenge
        | PersistenceError::InvalidTimestamp
        | PersistenceError::InvalidKeyVersion
        | PersistenceError::RevisionOutOfRange => AuthChallengePortError::InvalidInput,
        _ => AuthChallengePortError::Unavailable,
    }
}

pub struct AuthChallengeService<Port> {
    port: Port,
    keyring: Keyring,
    policy: AuthChallengePolicy,
}

impl<Port> AuthChallengeService<Port>
where
    Port: AuthChallengePort,
{
    #[must_use]
    pub const fn new(port: Port, keyring: Keyring, policy: AuthChallengePolicy) -> Self {
        Self {
            port,
            keyring,
            policy,
        }
    }

    pub async fn issue(
        &self,
        command: IssueAuthChallengeCommand,
    ) -> Result<IssuedAuthChallenge, AuthChallengeServiceError> {
        let expires_at_ms = self
            .policy
            .expires_at_ms(command.created_at_ms)
            .map_err(|_| AuthChallengeServiceError::InvalidCommand)?;
        let generated = self
            .keyring
            .generate_auth_challenge()
            .map_err(|_| AuthChallengeServiceError::Unavailable)?;
        let challenge = NewAuthChallenge {
            id: EntityId::new(),
            token_key_version: generated.digest.key_version,
            token_hmac: generated.digest.digest,
            purpose: command.purpose,
            user_id: command.user_id,
            session_id: command.session_id,
            auth_revision: command.auth_revision,
            allowed_methods: command.allowed_methods,
            max_attempts: self.policy.max_attempts(),
            created_at_ms: command.created_at_ms,
            expires_at_ms,
            rotation_required: command.rotation_required,
            client_context: command.client_context,
            revision: Revision::initial(),
        };
        match self.port.create(challenge).await? {
            CreateAuthChallengeOutcome::Created(challenge) => Ok(IssuedAuthChallenge {
                challenge,
                token: generated.token,
            }),
            CreateAuthChallengeOutcome::AlreadyOpen => {
                Err(AuthChallengeServiceError::AlreadyPending)
            }
            CreateAuthChallengeOutcome::PrincipalUnavailable => {
                Err(AuthChallengeServiceError::Unauthorized)
            }
        }
    }

    pub async fn load(
        &self,
        presentation: PresentAuthChallengeCommand,
    ) -> Result<Option<AuthChallenge>, AuthChallengeServiceError> {
        let Some(access) = self.authorize(presentation).await? else {
            return Ok(None);
        };
        self.port.load(access).await.map_err(Into::into)
    }

    /// Reserves a durable attempt before the caller invokes its method-specific verifier.
    /// Only the CAS winner receives an unforgeable claim; stale callers must not run verification.
    pub async fn reserve_attempt(
        &self,
        presentation: PresentAuthChallengeCommand,
        method: AuthenticationMethod,
        expected_revision: Revision,
    ) -> Result<AuthChallengeReservationOutcome, AuthChallengeServiceError> {
        let now_ms = presentation.now_ms;
        let Some(access) = self.authorize(presentation).await? else {
            return Ok(AuthChallengeReservationOutcome::NotFound);
        };
        let verification_expires_at_ms = self
            .policy
            .verification_expires_at_ms(now_ms)
            .map_err(|_| AuthChallengeServiceError::InvalidCommand)?;
        let claim_id = EntityId::new();
        match self
            .port
            .reserve_attempt(AuthChallengeAttemptReservation {
                access: access.clone(),
                claim_id,
                method,
                expected_revision,
                verification_expires_at_ms,
            })
            .await?
        {
            AuthChallengeAttemptReservationOutcome::Reserved(challenge) => {
                Ok(AuthChallengeReservationOutcome::Reserved(Box::new(
                    AuthChallengeVerificationClaim {
                        access,
                        claim_id,
                        method,
                        challenge,
                    },
                )))
            }
            AuthChallengeAttemptReservationOutcome::Stale => {
                Ok(AuthChallengeReservationOutcome::Stale)
            }
        }
    }

    pub async fn reject_attempt(
        &self,
        mut claim: AuthChallengeVerificationClaim,
        completed_at_ms: i64,
    ) -> Result<AuthChallengeAttemptOutcome, AuthChallengeServiceError> {
        validate_completion_time(claim.reserved_at_ms(), completed_at_ms)?;
        claim.access.now_ms = completed_at_ms;
        self.port
            .record_failure(AuthChallengeAttemptFailure {
                access: claim.access,
                claim_id: claim.claim_id,
                method: claim.method,
                expected_revision: claim.challenge.revision,
            })
            .await
            .map_err(Into::into)
    }

    pub async fn accept_verified_method(
        &self,
        mut evidence: VerifiedAuthChallengeEvidence,
        completed_at_ms: i64,
    ) -> Result<VerifiedAuthChallengeOutcome, AuthChallengeServiceError> {
        validate_completion_time(evidence.claim.reserved_at_ms(), completed_at_ms)?;
        if !evidence
            .claim
            .method
            .permits_assurance(evidence.achieved_assurance)
        {
            return Err(AuthChallengeServiceError::InvalidEvidence);
        }
        evidence.claim.access.now_ms = completed_at_ms;
        let access = evidence.claim.access.clone();
        match self
            .port
            .begin_consumption(AuthChallengeConsumption {
                access,
                claim_id: evidence.claim.claim_id,
                method: evidence.claim.method,
                achieved_assurance: evidence.achieved_assurance,
                expected_revision: evidence.claim.challenge.revision,
            })
            .await?
        {
            AuthChallengeConsumptionOutcome::Consumed(challenge) => {
                Ok(VerifiedAuthChallengeOutcome::Consumed(challenge))
            }
            AuthChallengeConsumptionOutcome::RotationPending(challenge) => {
                match self
                    .reserve_rotation_claim(evidence.claim.access, challenge.revision)
                    .await?
                {
                    Some(claim) => Ok(VerifiedAuthChallengeOutcome::RotationRequired(claim)),
                    None => Ok(VerifiedAuthChallengeOutcome::Stale),
                }
            }
            AuthChallengeConsumptionOutcome::Stale => Ok(VerifiedAuthChallengeOutcome::Stale),
        }
    }

    /// Rehydrates the replacement-session transaction handoff after a process crash. The bearer,
    /// exact client context and caller-observed revision are checked again. A durable lease/CAS
    /// ensures that concurrent resumptions yield at most one transaction claim.
    pub async fn resume_rotation(
        &self,
        presentation: PresentAuthChallengeCommand,
        expected_revision: Revision,
    ) -> Result<AuthChallengeRotationResumeOutcome, AuthChallengeServiceError> {
        let Some(access) = self.authorize(presentation).await? else {
            return Ok(AuthChallengeRotationResumeOutcome::NotFound);
        };
        match self
            .reserve_rotation_claim(access, expected_revision)
            .await?
        {
            Some(claim) => Ok(AuthChallengeRotationResumeOutcome::Ready(Box::new(claim))),
            None => Ok(AuthChallengeRotationResumeOutcome::Stale),
        }
    }

    async fn reserve_rotation_claim(
        &self,
        access: AuthChallengeAccess,
        expected_revision: Revision,
    ) -> Result<Option<AuthChallengeRotationTransactionClaim>, AuthChallengeServiceError> {
        let transaction_expires_at_ms = self
            .policy
            .verification_expires_at_ms(access.now_ms)
            .map_err(|_| AuthChallengeServiceError::InvalidCommand)?;
        let transaction_claim_id = EntityId::new();
        match self
            .port
            .reserve_rotation(AuthChallengeRotationReservation {
                access: access.clone(),
                claim_id: transaction_claim_id,
                expected_revision,
                transaction_expires_at_ms,
            })
            .await?
        {
            AuthChallengeRotationReservationOutcome::Reserved(challenge) => {
                Ok(Some(AuthChallengeRotationTransactionClaim {
                    access,
                    transaction_claim_id,
                    expected_revision: challenge.revision,
                    challenge,
                }))
            }
            AuthChallengeRotationReservationOutcome::Stale => Ok(None),
        }
    }

    async fn authorize(
        &self,
        presentation: PresentAuthChallengeCommand,
    ) -> Result<Option<AuthChallengeAccess>, AuthChallengeServiceError> {
        let lookup = AuthChallengeTokenLookup {
            id: presentation.id,
            client_context: presentation.client_context.clone(),
            now_ms: presentation.now_ms,
        };
        let Some(expected) = self.port.token_digest(lookup).await? else {
            return Ok(None);
        };
        let verified = self
            .keyring
            .verify_auth_challenge(presentation.token.presented(), &expected)
            .map_err(|_| AuthChallengeServiceError::Unavailable)?;
        if !verified {
            return Ok(None);
        }
        Ok(Some(AuthChallengeAccess {
            id: presentation.id,
            token_key_version: expected.key_version,
            token_hmac: expected.digest,
            client_context: presentation.client_context,
            now_ms: presentation.now_ms,
        }))
    }
}

fn validate_completion_time(
    reserved_at_ms: i64,
    completed_at_ms: i64,
) -> Result<(), AuthChallengeServiceError> {
    if completed_at_ms < reserved_at_ms {
        return Err(AuthChallengeServiceError::InvalidCommand);
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum AuthChallengeServiceError {
    #[error("authentication challenge command is invalid")]
    InvalidCommand,
    #[error("authentication evidence is invalid for its method")]
    InvalidEvidence,
    #[error("an authentication challenge for this user and purpose is already pending")]
    AlreadyPending,
    #[error("the authentication challenge principal or session is unavailable")]
    Unauthorized,
    #[error("authentication challenge service is unavailable")]
    Unavailable,
}

impl From<AuthChallengePortError> for AuthChallengeServiceError {
    fn from(error: AuthChallengePortError) -> Self {
        match error {
            AuthChallengePortError::InvalidInput => Self::InvalidCommand,
            AuthChallengePortError::Unavailable => Self::Unavailable,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use nodecontroll_domain::{AuthChallengeRotationState, AuthChallengeStatus};
    use nodecontroll_secrets::EnvelopeCipher;

    use super::*;

    #[derive(Clone, Default)]
    struct FakePort {
        created: Arc<Mutex<Option<NewAuthChallenge>>>,
        reserved: Arc<Mutex<Option<AuthChallengeAttemptReservation>>>,
        consumed: Arc<Mutex<Option<AuthChallengeConsumption>>>,
        rotation_revision: Arc<Mutex<Option<Revision>>>,
    }

    #[async_trait]
    impl AuthChallengePort for FakePort {
        async fn create(
            &self,
            challenge: NewAuthChallenge,
        ) -> Result<CreateAuthChallengeOutcome, AuthChallengePortError> {
            if let Ok(mut stored) = self.created.lock() {
                *stored = Some(challenge.clone());
            }
            Ok(CreateAuthChallengeOutcome::Created(project_new(&challenge)))
        }

        async fn token_digest(
            &self,
            _lookup: AuthChallengeTokenLookup,
        ) -> Result<Option<KeyedDigest>, AuthChallengePortError> {
            let stored = self.created.lock();
            Ok(match stored {
                Ok(guard) => guard.as_ref().map(|challenge| KeyedDigest {
                    key_version: challenge.token_key_version,
                    digest: challenge.token_hmac,
                }),
                Err(_) => None,
            })
        }

        async fn load(
            &self,
            _access: AuthChallengeAccess,
        ) -> Result<Option<AuthChallenge>, AuthChallengePortError> {
            let stored = self.created.lock();
            Ok(match stored {
                Ok(guard) => guard.as_ref().map(project_new),
                Err(_) => None,
            })
        }

        async fn reserve_attempt(
            &self,
            reservation: AuthChallengeAttemptReservation,
        ) -> Result<AuthChallengeAttemptReservationOutcome, AuthChallengePortError> {
            if let Ok(mut stored) = self.reserved.lock() {
                *stored = Some(reservation.clone());
            }
            let created = self.created.lock();
            let Some(challenge) = created
                .ok()
                .and_then(|guard| guard.as_ref().map(project_new))
            else {
                return Ok(AuthChallengeAttemptReservationOutcome::Stale);
            };
            let mut reserved = challenge;
            reserved.status = AuthChallengeStatus::VerificationPending;
            reserved.attempts_used = 1;
            reserved.verification_in_progress = true;
            reserved.revision = Revision::from_value(1);
            Ok(AuthChallengeAttemptReservationOutcome::Reserved(reserved))
        }

        async fn record_failure(
            &self,
            _failure: AuthChallengeAttemptFailure,
        ) -> Result<AuthChallengeAttemptOutcome, AuthChallengePortError> {
            Ok(AuthChallengeAttemptOutcome::Stale)
        }

        async fn begin_consumption(
            &self,
            consumption: AuthChallengeConsumption,
        ) -> Result<AuthChallengeConsumptionOutcome, AuthChallengePortError> {
            if let Ok(mut stored) = self.consumed.lock() {
                *stored = Some(consumption);
            }
            Ok(AuthChallengeConsumptionOutcome::Stale)
        }

        async fn reserve_rotation(
            &self,
            reservation: AuthChallengeRotationReservation,
        ) -> Result<AuthChallengeRotationReservationOutcome, AuthChallengePortError> {
            let mut revision = match self.rotation_revision.lock() {
                Ok(revision) => revision,
                Err(_) => return Err(AuthChallengePortError::Unavailable),
            };
            if *revision != Some(reservation.expected_revision) {
                return Ok(AuthChallengeRotationReservationOutcome::Stale);
            }
            let next = reservation
                .expected_revision
                .next()
                .map_err(|_| AuthChallengePortError::InvalidInput)?;
            *revision = Some(next);
            let created = self.created.lock();
            let Some(mut challenge) = created
                .ok()
                .and_then(|guard| guard.as_ref().map(project_new))
            else {
                return Ok(AuthChallengeRotationReservationOutcome::Stale);
            };
            challenge.status = AuthChallengeStatus::RotationPending;
            challenge.rotation_state = AuthChallengeRotationState::Pending;
            challenge.attempts_used = 1;
            challenge.verified_method = Some(AuthenticationMethod::RecoveryCode);
            challenge.achieved_assurance = Some(AuthenticationAssurance::Recovery);
            challenge.rotation_transaction_in_progress = true;
            challenge.revision = next;
            Ok(AuthChallengeRotationReservationOutcome::Reserved(challenge))
        }
    }

    fn keyring() -> Keyring {
        let cipher = EnvelopeCipher::from_hex(&"11".repeat(32), 1);
        let Ok(cipher) = cipher else {
            panic!("test key must be valid");
        };
        let keyring = Keyring::from_ciphers(cipher, Vec::new());
        let Ok(keyring) = keyring else {
            panic!("single-key keyring must be valid");
        };
        keyring
    }

    fn policy() -> AuthChallengePolicy {
        let policy = AuthChallengePolicy::new(120_000, 10_000, 5);
        let Ok(policy) = policy else {
            panic!("test policy must be valid");
        };
        policy
    }

    fn issue_command(created_at_ms: i64) -> IssueAuthChallengeCommand {
        IssueAuthChallengeCommand {
            purpose: AuthChallengePurpose::Login,
            user_id: EntityId::new(),
            session_id: None,
            auth_revision: Revision::from_value(9),
            allowed_methods: vec![AuthenticationMethod::Totp, AuthenticationMethod::WebAuthn],
            rotation_required: true,
            client_context: AuthChallengeClientContext {
                key_version: Some(1),
                client_network_hmac: Some([2; 32]),
                user_agent_hash: Some([3; 32]),
            },
            created_at_ms,
        }
    }

    fn project_new(challenge: &NewAuthChallenge) -> AuthChallenge {
        AuthChallenge {
            id: challenge.id,
            purpose: challenge.purpose,
            user_id: challenge.user_id,
            session_id: challenge.session_id,
            auth_revision: challenge.auth_revision,
            allowed_methods: challenge.allowed_methods.clone(),
            status: AuthChallengeStatus::Pending,
            rotation_state: if challenge.rotation_required {
                AuthChallengeRotationState::Required
            } else {
                AuthChallengeRotationState::NotRequired
            },
            attempts_used: 0,
            max_attempts: challenge.max_attempts,
            created_at_ms: challenge.created_at_ms,
            expires_at_ms: challenge.expires_at_ms,
            verified_method: None,
            achieved_assurance: None,
            consumed_at_ms: None,
            verification_in_progress: false,
            rotation_transaction_in_progress: false,
            has_client_network_context: challenge.client_context.client_network_hmac.is_some(),
            has_user_agent_context: challenge.client_context.user_agent_hash.is_some(),
            revision: challenge.revision,
        }
    }

    #[tokio::test]
    async fn issue_generates_an_opaque_bearer_and_persists_only_its_digest() {
        let port = FakePort::default();
        let observed = port.created.clone();
        let keyring = keyring();
        let verifier = keyring.clone();
        let service = AuthChallengeService::new(port, keyring, policy());
        let result = service.issue(issue_command(10_000)).await;
        let Ok(issued) = result else {
            panic!("challenge issuance must succeed");
        };
        assert_eq!(issued.challenge.expires_at_ms, 130_000);
        assert_eq!(issued.challenge.max_attempts, 5);
        let stored = observed.lock();
        assert!(matches!(
            stored,
            Ok(ref guard)
                if matches!(guard.as_ref(), Some(challenge)
                    if challenge.id == issued.challenge.id
                        && challenge.token_key_version == 1
                        && challenge.token_hmac != [0; 32]
                        && challenge.revision == Revision::initial())
        ));
        if let Ok(guard) = stored
            && let Some(challenge) = guard.as_ref()
        {
            let expected = KeyedDigest {
                key_version: challenge.token_key_version,
                digest: challenge.token_hmac,
            };
            assert!(matches!(
                verifier.verify_auth_challenge(issued.token.presented(), &expected),
                Ok(true)
            ));
        }
    }

    #[tokio::test]
    async fn reservation_claim_and_verified_evidence_cross_typed_ports() {
        let port = FakePort::default();
        let observed_reservation = port.reserved.clone();
        let observed_consumption = port.consumed.clone();
        let service = AuthChallengeService::new(port, keyring(), policy());
        let issued = service.issue(issue_command(10_000)).await;
        let Ok(issued) = issued else {
            panic!("challenge issuance must succeed");
        };
        let presentation = PresentAuthChallengeCommand {
            id: issued.challenge.id,
            token: issued.token,
            client_context: AuthChallengeClientContext {
                key_version: Some(1),
                client_network_hmac: Some([2; 32]),
                user_agent_hash: Some([3; 32]),
            },
            now_ms: 10_001,
        };
        let reserved = service
            .reserve_attempt(
                presentation,
                AuthenticationMethod::WebAuthn,
                Revision::initial(),
            )
            .await;
        let Ok(AuthChallengeReservationOutcome::Reserved(claim)) = reserved else {
            panic!("one attempt must reserve a typed claim");
        };
        assert_eq!(claim.method(), AuthenticationMethod::WebAuthn);
        assert_eq!(claim.challenge().attempts_used, 1);
        let evidence = VerifiedAuthChallengeEvidence::from_method_verifier(
            *claim,
            AuthenticationAssurance::PhishingResistant,
        );
        let Ok(evidence) = evidence else {
            panic!("WebAuthn may establish phishing-resistant assurance");
        };
        assert!(matches!(
            service.accept_verified_method(evidence, 10_002).await,
            Ok(VerifiedAuthChallengeOutcome::Stale)
        ));
        assert!(matches!(
            observed_reservation.lock(),
            Ok(ref guard)
                if matches!(guard.as_ref(), Some(reservation)
                    if reservation.method == AuthenticationMethod::WebAuthn
                        && reservation.expected_revision == Revision::initial()
                        && reservation.verification_expires_at_ms == 20_001)
        ));
        assert!(matches!(
            observed_consumption.lock(),
            Ok(ref guard)
                if matches!(guard.as_ref(), Some(consumption)
                    if consumption.method == AuthenticationMethod::WebAuthn
                        && consumption.achieved_assurance
                            == AuthenticationAssurance::PhishingResistant
                        && consumption.expected_revision == Revision::from_value(1))
        ));
    }

    #[tokio::test]
    async fn concurrent_crash_resume_yields_one_rotation_transaction_claim() {
        let port = FakePort::default();
        let rotation_revision = port.rotation_revision.clone();
        let service = AuthChallengeService::new(port, keyring(), policy());
        let mut command = issue_command(30_000);
        command.allowed_methods = vec![AuthenticationMethod::RecoveryCode];
        let issued = service.issue(command).await;
        let Ok(issued) = issued else {
            panic!("challenge issuance must succeed");
        };
        if let Ok(mut revision) = rotation_revision.lock() {
            *revision = Some(Revision::from_value(2));
        }
        let presented = issued.token.presented().to_owned();
        let first_token = AuthChallengeToken::parse_presented(&presented);
        let second_token = AuthChallengeToken::parse_presented(&presented);
        let (Ok(first_token), Ok(second_token)) = (first_token, second_token) else {
            panic!("generated challenge bearer must retain its canonical form");
        };
        let context = AuthChallengeClientContext {
            key_version: Some(1),
            client_network_hmac: Some([2; 32]),
            user_agent_hash: Some([3; 32]),
        };
        let (first, second) = tokio::join!(
            service.resume_rotation(
                PresentAuthChallengeCommand {
                    id: issued.challenge.id,
                    token: first_token,
                    client_context: context.clone(),
                    now_ms: 30_010,
                },
                Revision::from_value(2),
            ),
            service.resume_rotation(
                PresentAuthChallengeCommand {
                    id: issued.challenge.id,
                    token: second_token,
                    client_context: context,
                    now_ms: 30_010,
                },
                Revision::from_value(2),
            ),
        );
        match (first, second) {
            (
                Ok(AuthChallengeRotationResumeOutcome::Ready(claim)),
                Ok(AuthChallengeRotationResumeOutcome::Stale),
            )
            | (
                Ok(AuthChallengeRotationResumeOutcome::Stale),
                Ok(AuthChallengeRotationResumeOutcome::Ready(claim)),
            ) => {
                assert_eq!(claim.challenge().revision, Revision::from_value(3));
                assert!(claim.challenge().rotation_transaction_in_progress);
                let (_, _, expected_revision) = claim.transaction_binding();
                assert_eq!(expected_revision, Revision::from_value(3));
            }
            _ => panic!("exactly one concurrent resume must own the transaction claim"),
        }
    }

    #[test]
    fn method_verifier_evidence_rejects_every_cross_method_assurance() {
        for (method, assurance, accepted) in [
            (
                AuthenticationMethod::Password,
                AuthenticationAssurance::Password,
                true,
            ),
            (
                AuthenticationMethod::Password,
                AuthenticationAssurance::Mfa,
                false,
            ),
            (
                AuthenticationMethod::Totp,
                AuthenticationAssurance::Mfa,
                true,
            ),
            (
                AuthenticationMethod::Totp,
                AuthenticationAssurance::PhishingResistant,
                false,
            ),
            (
                AuthenticationMethod::WebAuthn,
                AuthenticationAssurance::PhishingResistant,
                true,
            ),
            (
                AuthenticationMethod::RecoveryCode,
                AuthenticationAssurance::Recovery,
                true,
            ),
            (
                AuthenticationMethod::RecoveryCode,
                AuthenticationAssurance::Mfa,
                false,
            ),
        ] {
            let challenge = AuthChallenge {
                id: EntityId::new(),
                purpose: AuthChallengePurpose::Login,
                user_id: EntityId::new(),
                session_id: None,
                auth_revision: Revision::initial(),
                allowed_methods: vec![method],
                status: AuthChallengeStatus::VerificationPending,
                rotation_state: AuthChallengeRotationState::NotRequired,
                attempts_used: 1,
                max_attempts: 3,
                created_at_ms: 1,
                expires_at_ms: 100,
                verified_method: None,
                achieved_assurance: None,
                consumed_at_ms: None,
                verification_in_progress: true,
                rotation_transaction_in_progress: false,
                has_client_network_context: false,
                has_user_agent_context: false,
                revision: Revision::from_value(1),
            };
            let claim = AuthChallengeVerificationClaim {
                access: AuthChallengeAccess {
                    id: challenge.id,
                    token_key_version: 1,
                    token_hmac: [1; 32],
                    client_context: AuthChallengeClientContext::unbound(),
                    now_ms: 2,
                },
                claim_id: EntityId::new(),
                method,
                challenge,
            };
            assert_eq!(
                VerifiedAuthChallengeEvidence::from_method_verifier(claim, assurance).is_ok(),
                accepted,
                "unexpected evidence result for {method:?}/{assurance:?}"
            );
        }
    }
}
