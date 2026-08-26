use async_trait::async_trait;
use nodecontroll_domain::{
    AuthenticationAssurance, AuthenticationMethod, EntityId, Revision, TotpCredential,
    TotpEnrollmentPolicy,
};
use nodecontroll_persistence::{
    ActivateTotpCredential, ActivateTotpCredentialOutcome, AuthSessionStatus, AuthenticatedSession,
    BeginTotpEnrollmentOutcome, Database, DisableTotpCredential, DisableTotpCredentialOutcome,
    NewRecoveryCode, NewRecoveryCodeSet, NewSecretRecord, NewTotpEnrollment, PersistenceError,
    StoredTotpCredential, TotpActivationResult, TotpSessionGuard, TotpStepAdvance,
    TotpStepAdvanceOutcome,
};
use nodecontroll_secrets::{
    KeyedDigestPurpose, Keyring, RecoveryCode, SecretBinding, SecretOwnerKind, SecretPurpose,
    TOTP_SEED_SCHEMA_VERSION, TotpCode, TotpSeed, generate_recovery_codes, verify_totp_at_utc_ms,
};
use thiserror::Error;

use crate::{AuthChallengeVerificationClaim, VerifiedAuthChallengeEvidence};

pub trait TotpClock: Send + Sync {
    fn now_utc_ms(&self) -> Result<i64, TotpServiceError>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemTotpClock;

impl TotpClock for SystemTotpClock {
    fn now_utc_ms(&self) -> Result<i64, TotpServiceError> {
        super::unix_time_ms().map_err(|_| TotpServiceError::ClockUnavailable)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TotpManagementBinding {
    user_id: EntityId,
    actor_session_id: EntityId,
    expected_user_revision: Revision,
    expected_auth_revision: Revision,
    expected_recent_auth_at_ms: i64,
}

impl TotpManagementBinding {
    /// Derives the immutable persistence guard from an authenticated session projection.
    ///
    /// The binding deliberately has no field-wise constructor. TOTP management callers must
    /// first pass the ordinary session-authentication boundary, and a forced password change
    /// cannot be bypassed by entering the credential-management flow. The repository still
    /// revalidates the captured user/session/auth revisions, exact recent-auth timestamp, and
    /// session lifetime inside every management transaction.
    pub fn from_authenticated_session(
        authenticated: &AuthenticatedSession,
    ) -> Result<Self, TotpManagementBindingError> {
        if authenticated.force_password_change {
            return Err(TotpManagementBindingError::PasswordChangeRequired);
        }
        if authenticated.session.status != AuthSessionStatus::Active
            || authenticated.session.revoked_at_ms.is_some()
            || authenticated.session.revoked_reason.is_some()
        {
            return Err(TotpManagementBindingError::InvalidSession);
        }
        Ok(Self {
            user_id: authenticated.user_id,
            actor_session_id: authenticated.session.id,
            expected_user_revision: authenticated.user_revision,
            expected_auth_revision: authenticated.session.auth_revision,
            expected_recent_auth_at_ms: authenticated.session.recent_auth_at_ms,
        })
    }

    fn guard(self, now_ms: i64) -> TotpSessionGuard {
        TotpSessionGuard {
            user_id: self.user_id,
            actor_session_id: self.actor_session_id,
            expected_user_revision: self.expected_user_revision,
            expected_auth_revision: self.expected_auth_revision,
            expected_recent_auth_at_ms: self.expected_recent_auth_at_ms,
            now_ms,
        }
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum TotpManagementBindingError {
    #[error("TOTP management is unavailable until the required password change is complete")]
    PasswordChangeRequired,
    #[error("TOTP management requires an active authenticated session")]
    InvalidSession,
}

pub struct BeginTotpEnrollmentCommand {
    pub binding: TotpManagementBinding,
}

/// One-shot enrollment result. The seed has no Debug/Clone implementation and zeroizes on drop.
pub struct BegunTotpEnrollment {
    pub credential: TotpCredential,
    seed: TotpSeed,
}

impl BegunTotpEnrollment {
    #[must_use]
    pub const fn seed(&self) -> &TotpSeed {
        &self.seed
    }
}

pub struct ActivateTotpEnrollmentCommand<'a> {
    pub binding: TotpManagementBinding,
    pub credential_id: EntityId,
    pub expected_credential_revision: Revision,
    pub presented_code: &'a str,
}

/// Recovery-code plaintext exists only in this non-cloneable, non-debuggable one-shot outcome.
/// If delivery is lost after the database commit, these values cannot be reconstructed; the still
/// authenticated actor must use the recovery-code replacement flow to obtain a fresh set.
pub struct ActivatedTotpCredential {
    pub result: TotpActivationResult,
    one_time_recovery_codes: Vec<RecoveryCode>,
}

impl ActivatedTotpCredential {
    #[must_use]
    pub fn one_time_recovery_codes(&self) -> &[RecoveryCode] {
        &self.one_time_recovery_codes
    }
}

pub struct DisableTotpCredentialCommand {
    pub binding: TotpManagementBinding,
    pub credential_id: EntityId,
    pub expected_credential_revision: Revision,
}

pub enum TotpChallengeProofOutcome {
    Verified(VerifiedAuthChallengeEvidence),
    Rejected(AuthChallengeVerificationClaim),
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum TotpPortError {
    #[error("TOTP command is invalid")]
    InvalidInput,
    #[error("the authenticated TOTP management principal is stale")]
    Unauthorized,
    #[error("TOTP persistence is unavailable")]
    Unavailable,
}

#[async_trait]
pub trait TotpPort: Send + Sync {
    async fn begin_enrollment(
        &self,
        enrollment: NewTotpEnrollment,
    ) -> Result<BeginTotpEnrollmentOutcome, TotpPortError>;

    async fn pending_credential(
        &self,
        user_id: EntityId,
        now_ms: i64,
    ) -> Result<Option<StoredTotpCredential>, TotpPortError>;

    async fn active_credential(
        &self,
        user_id: EntityId,
        now_ms: i64,
    ) -> Result<Option<StoredTotpCredential>, TotpPortError>;

    async fn activate(
        &self,
        activation: ActivateTotpCredential<'_>,
    ) -> Result<ActivateTotpCredentialOutcome, TotpPortError>;

    async fn advance_step(
        &self,
        advance: TotpStepAdvance,
    ) -> Result<TotpStepAdvanceOutcome, TotpPortError>;

    async fn disable(
        &self,
        disable: DisableTotpCredential,
    ) -> Result<DisableTotpCredentialOutcome, TotpPortError>;
}

#[async_trait]
impl TotpPort for Database {
    async fn begin_enrollment(
        &self,
        enrollment: NewTotpEnrollment,
    ) -> Result<BeginTotpEnrollmentOutcome, TotpPortError> {
        self.begin_totp_enrollment(&enrollment)
            .await
            .map_err(map_persistence_error)
    }

    async fn pending_credential(
        &self,
        user_id: EntityId,
        now_ms: i64,
    ) -> Result<Option<StoredTotpCredential>, TotpPortError> {
        self.pending_totp_credential(user_id, now_ms)
            .await
            .map_err(map_persistence_error)
    }

    async fn active_credential(
        &self,
        user_id: EntityId,
        now_ms: i64,
    ) -> Result<Option<StoredTotpCredential>, TotpPortError> {
        self.active_totp_credential(user_id, now_ms)
            .await
            .map_err(map_persistence_error)
    }

    async fn activate(
        &self,
        activation: ActivateTotpCredential<'_>,
    ) -> Result<ActivateTotpCredentialOutcome, TotpPortError> {
        self.activate_totp_credential(activation)
            .await
            .map_err(map_persistence_error)
    }

    async fn advance_step(
        &self,
        advance: TotpStepAdvance,
    ) -> Result<TotpStepAdvanceOutcome, TotpPortError> {
        self.advance_totp_step(&advance)
            .await
            .map_err(map_persistence_error)
    }

    async fn disable(
        &self,
        disable: DisableTotpCredential,
    ) -> Result<DisableTotpCredentialOutcome, TotpPortError> {
        self.disable_totp_credential(&disable)
            .await
            .map_err(map_persistence_error)
    }
}

fn map_persistence_error(error: PersistenceError) -> TotpPortError {
    match error {
        PersistenceError::InvalidTimestamp
        | PersistenceError::RevisionOutOfRange
        | PersistenceError::InvalidKeyVersion
        | PersistenceError::InvalidSecretRecord
        | PersistenceError::InvalidRecoveryCodeSet
        | PersistenceError::InvalidTotpCredential => TotpPortError::InvalidInput,
        PersistenceError::SessionPrincipalUnavailable
        | PersistenceError::AuthStateUnavailable
        | PersistenceError::SessionRevisionConflict => TotpPortError::Unauthorized,
        _ => TotpPortError::Unavailable,
    }
}

pub struct TotpService<Port, Clock> {
    port: Port,
    keyring: Keyring,
    enrollment_policy: TotpEnrollmentPolicy,
    recent_auth_ttl_ms: i64,
    clock: Clock,
}

impl<Port, Clock> TotpService<Port, Clock>
where
    Port: TotpPort,
    Clock: TotpClock,
{
    pub fn new(
        port: Port,
        keyring: Keyring,
        enrollment_policy: TotpEnrollmentPolicy,
        recent_auth_ttl_ms: i64,
        clock: Clock,
    ) -> Result<Self, TotpServiceError> {
        if !(60_000..=3_600_000).contains(&recent_auth_ttl_ms) {
            return Err(TotpServiceError::InvalidCommand);
        }
        Ok(Self {
            port,
            keyring,
            enrollment_policy,
            recent_auth_ttl_ms,
            clock,
        })
    }

    pub async fn begin_enrollment(
        &self,
        command: BeginTotpEnrollmentCommand,
    ) -> Result<BegunTotpEnrollment, TotpServiceError> {
        let now_ms = self.management_now(command.binding)?;
        let expires_at_ms = self
            .enrollment_policy
            .expires_at_ms(now_ms)
            .map_err(|_| TotpServiceError::InvalidCommand)?;
        let seed = TotpSeed::generate().map_err(|_| TotpServiceError::Unavailable)?;
        let binding = SecretBinding::new(
            SecretPurpose::TotpSeed,
            SecretOwnerKind::User,
            command.binding.user_id.into_uuid(),
            TOTP_SEED_SCHEMA_VERSION,
        )
        .map_err(|_| TotpServiceError::InvalidCommand)?;
        let envelope = self
            .keyring
            .encrypt_totp_seed(command.binding.user_id.into_uuid(), &seed)
            .map_err(|_| TotpServiceError::Unavailable)?;
        let secret_id = EntityId::new();
        match self
            .port
            .begin_enrollment(NewTotpEnrollment {
                credential_id: EntityId::new(),
                secret: NewSecretRecord {
                    id: secret_id,
                    binding,
                    envelope,
                    created_at_ms: now_ms,
                    rotated_from: None,
                },
                guard: command.binding.guard(now_ms),
                pending_expires_at_ms: expires_at_ms,
            })
            .await?
        {
            BeginTotpEnrollmentOutcome::Created(stored) => Ok(BegunTotpEnrollment {
                credential: stored.credential,
                seed,
            }),
            BeginTotpEnrollmentOutcome::AlreadyPending => Err(TotpServiceError::AlreadyPending),
            BeginTotpEnrollmentOutcome::Stale => Err(TotpServiceError::Stale),
        }
    }

    pub async fn activate_enrollment(
        &self,
        command: ActivateTotpEnrollmentCommand<'_>,
    ) -> Result<ActivatedTotpCredential, TotpServiceError> {
        let now_ms = self.management_now(command.binding)?;
        let Some(stored) = self
            .port
            .pending_credential(command.binding.user_id, now_ms)
            .await?
        else {
            return Err(TotpServiceError::Stale);
        };
        if stored.credential.id != command.credential_id
            || stored.credential.revision != command.expected_credential_revision
        {
            return Err(TotpServiceError::Stale);
        }
        let code =
            TotpCode::parse(command.presented_code).map_err(|_| TotpServiceError::InvalidProof)?;
        let seed = self
            .keyring
            .decrypt_totp_seed(command.binding.user_id.into_uuid(), &stored.secret.envelope)
            .map_err(|_| TotpServiceError::Unavailable)?;
        let accepted_step = verify_totp_at_utc_ms(&seed, &code, now_ms, None)
            .map_err(|_| TotpServiceError::Unavailable)?
            .ok_or(TotpServiceError::InvalidProof)?;
        let (recovery_codes, recovery_records) = self.prepare_recovery_codes(now_ms)?;
        match self
            .port
            .activate(ActivateTotpCredential {
                credential_id: command.credential_id,
                expected_credential_revision: command.expected_credential_revision,
                accepted_step,
                guard: command.binding.guard(now_ms),
                recovery_codes: &recovery_records,
            })
            .await?
        {
            ActivateTotpCredentialOutcome::Activated(result) => Ok(ActivatedTotpCredential {
                result,
                one_time_recovery_codes: recovery_codes,
            }),
            ActivateTotpCredentialOutcome::Stale => Err(TotpServiceError::Stale),
        }
    }

    /// Verifies a claim already reserved by C3. The accepted step is durably advanced before
    /// evidence is emitted; a process crash therefore burns at most that one code and can never
    /// reopen it. The C3 claim lease recovers independently and no evidence survives the crash.
    pub async fn verify_challenge(
        &self,
        claim: AuthChallengeVerificationClaim,
        presented_code: &str,
    ) -> Result<TotpChallengeProofOutcome, TotpServiceError> {
        if claim.method() != AuthenticationMethod::Totp {
            return Err(TotpServiceError::InvalidChallengeClaim);
        }
        let now_ms = self.clock.now_utc_ms()?;
        if now_ms < claim.reserved_at_ms() || now_ms >= claim.challenge().expires_at_ms {
            return Err(TotpServiceError::ClockUnavailable);
        }
        let Ok(code) = TotpCode::parse(presented_code) else {
            return Ok(TotpChallengeProofOutcome::Rejected(claim));
        };
        let user_id = claim.challenge().user_id;
        let Some(stored) = self.port.active_credential(user_id, now_ms).await? else {
            return Ok(TotpChallengeProofOutcome::Rejected(claim));
        };
        let seed = self
            .keyring
            .decrypt_totp_seed(user_id.into_uuid(), &stored.secret.envelope)
            .map_err(|_| TotpServiceError::Unavailable)?;
        let verification_time_ms = claim.reserved_at_ms();
        let Some(accepted_step) = verify_totp_at_utc_ms(
            &seed,
            &code,
            verification_time_ms,
            stored.credential.last_accepted_step,
        )
        .map_err(|_| TotpServiceError::Unavailable)?
        else {
            return Ok(TotpChallengeProofOutcome::Rejected(claim));
        };
        let advance = self
            .port
            .advance_step(TotpStepAdvance {
                credential_id: stored.credential.id,
                user_id,
                expected_credential_revision: stored.credential.revision,
                expected_last_accepted_step: stored.credential.last_accepted_step,
                accepted_step,
                expected_auth_revision: claim.challenge().auth_revision,
                session_id: claim.challenge().session_id,
                verification_time_ms,
                now_ms,
            })
            .await?;
        match advance {
            TotpStepAdvanceOutcome::Advanced(_) => {
                let evidence = VerifiedAuthChallengeEvidence::from_method_verifier(
                    claim,
                    AuthenticationAssurance::Mfa,
                )
                .map_err(|_| TotpServiceError::InvalidChallengeClaim)?;
                Ok(TotpChallengeProofOutcome::Verified(evidence))
            }
            TotpStepAdvanceOutcome::Stale => Ok(TotpChallengeProofOutcome::Rejected(claim)),
        }
    }

    pub async fn disable(
        &self,
        command: DisableTotpCredentialCommand,
    ) -> Result<DisableTotpCredentialOutcome, TotpServiceError> {
        let now_ms = self.management_now(command.binding)?;
        match self
            .port
            .disable(DisableTotpCredential {
                credential_id: command.credential_id,
                expected_credential_revision: command.expected_credential_revision,
                guard: command.binding.guard(now_ms),
            })
            .await?
        {
            DisableTotpCredentialOutcome::Stale => Err(TotpServiceError::Stale),
            outcome => Ok(outcome),
        }
    }

    fn management_now(&self, binding: TotpManagementBinding) -> Result<i64, TotpServiceError> {
        let now_ms = self.clock.now_utc_ms()?;
        let expires_at_ms = binding
            .expected_recent_auth_at_ms
            .checked_add(self.recent_auth_ttl_ms)
            .ok_or(TotpServiceError::InvalidCommand)?;
        if binding.expected_recent_auth_at_ms < 0
            || now_ms < binding.expected_recent_auth_at_ms
            || now_ms >= expires_at_ms
        {
            return Err(TotpServiceError::RecentAuthRequired);
        }
        Ok(now_ms)
    }

    fn prepare_recovery_codes(
        &self,
        now_ms: i64,
    ) -> Result<(Vec<RecoveryCode>, NewRecoveryCodeSet), TotpServiceError> {
        let generated = generate_recovery_codes().map_err(|_| TotpServiceError::Unavailable)?;
        let mut records = Vec::with_capacity(generated.len());
        for code in &generated {
            let digest = self
                .keyring
                .keyed_digest(KeyedDigestPurpose::RecoveryCode, code.normalized_bytes())
                .map_err(|_| TotpServiceError::Unavailable)?;
            records.push(NewRecoveryCode {
                id: EntityId::new(),
                digest_key_version: digest.key_version,
                code_hmac: digest.digest,
            });
        }
        Ok((
            generated,
            NewRecoveryCodeSet {
                created_at_ms: now_ms,
                codes: records,
            },
        ))
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum TotpServiceError {
    #[error("TOTP command is invalid")]
    InvalidCommand,
    #[error("TOTP proof is invalid")]
    InvalidProof,
    #[error("recent authentication is required")]
    RecentAuthRequired,
    #[error("another TOTP enrollment is already pending")]
    AlreadyPending,
    #[error("the TOTP credential changed concurrently")]
    Stale,
    #[error("the authentication challenge claim is for another verifier")]
    InvalidChallengeClaim,
    #[error("the controlled UTC clock is unavailable or inconsistent")]
    ClockUnavailable,
    #[error("TOTP service is unavailable")]
    Unavailable,
}

impl From<TotpPortError> for TotpServiceError {
    fn from(error: TotpPortError) -> Self {
        match error {
            TotpPortError::InvalidInput => Self::InvalidCommand,
            TotpPortError::Unauthorized => Self::Stale,
            TotpPortError::Unavailable => Self::Unavailable,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;
    use nodecontroll_domain::{
        AuthChallenge, AuthChallengePurpose, AuthChallengeRotationState, AuthChallengeStatus,
        AuthenticationMethod, EntityId, PrincipalLabel, Revision, TotpCredential,
        TotpCredentialStatus, TotpEnrollmentPolicy, UserRole, Username,
    };
    use nodecontroll_persistence::{
        ActivateTotpCredential, ActivateTotpCredentialOutcome, AuthLevel, AuthSessionStatus,
        AuthSessionSummary, AuthenticatedSession, BeginTotpEnrollmentOutcome,
        DisableTotpCredential, DisableTotpCredentialOutcome, NewTotpEnrollment,
        SessionRevocationReason, StoredSecretRecord, StoredTotpCredential, TotpStepAdvance,
        TotpStepAdvanceOutcome,
    };
    use nodecontroll_secrets::{
        EnvelopeCipher, Keyring, SecretBinding, SecretOwnerKind, SecretPurpose,
        TOTP_SEED_SCHEMA_VERSION, TotpSeed,
    };

    use super::{
        TotpChallengeProofOutcome, TotpClock, TotpManagementBinding, TotpManagementBindingError,
        TotpPort, TotpPortError, TotpService, TotpServiceError,
    };
    use crate::AuthChallengeVerificationClaim;

    #[derive(Clone, Copy)]
    struct FixedClock(i64);

    impl TotpClock for FixedClock {
        fn now_utc_ms(&self) -> Result<i64, TotpServiceError> {
            Ok(self.0)
        }
    }

    #[derive(Clone)]
    struct FakePort {
        active: StoredTotpCredential,
        advanced: Arc<Mutex<Option<TotpStepAdvance>>>,
    }

    fn authenticated_session(force_password_change: bool) -> AuthenticatedSession {
        let username = Username::parse("owner");
        assert!(username.is_ok());
        let Ok(username) = username else {
            panic!("fixture username must be valid");
        };
        let principal_label = PrincipalLabel::parse("owner");
        assert!(principal_label.is_ok());
        let Ok(principal_label) = principal_label else {
            panic!("fixture principal label must be valid");
        };
        AuthenticatedSession {
            session: AuthSessionSummary {
                id: EntityId::new(),
                status: AuthSessionStatus::Active,
                auth_revision: Revision::from_value(7),
                auth_level: AuthLevel::Password,
                created_at_ms: 1,
                authenticated_at_ms: 2,
                recent_auth_at_ms: 3,
                last_seen_at_ms: 4,
                idle_expires_at_ms: 5,
                absolute_expires_at_ms: 6,
                has_ip_context: false,
                has_user_agent_context: false,
                revoked_at_ms: None,
                revoked_reason: None,
                revision: Revision::from_value(8),
            },
            user_id: EntityId::new(),
            username,
            role: UserRole::Owner,
            principal_label,
            force_password_change,
            user_revision: Revision::from_value(9),
        }
    }

    #[test]
    fn management_binding_only_derives_from_an_eligible_authenticated_session() {
        let authenticated = authenticated_session(false);
        let binding = TotpManagementBinding::from_authenticated_session(&authenticated);
        assert!(binding.is_ok());
        let Ok(binding) = binding else {
            panic!("eligible session must create a management binding");
        };
        assert_eq!(
            binding.guard(10),
            nodecontroll_persistence::TotpSessionGuard {
                user_id: authenticated.user_id,
                actor_session_id: authenticated.session.id,
                expected_user_revision: authenticated.user_revision,
                expected_auth_revision: authenticated.session.auth_revision,
                expected_recent_auth_at_ms: authenticated.session.recent_auth_at_ms,
                now_ms: 10,
            }
        );

        assert_eq!(
            TotpManagementBinding::from_authenticated_session(&authenticated_session(true)),
            Err(TotpManagementBindingError::PasswordChangeRequired)
        );

        let mut revoked = authenticated_session(false);
        revoked.session.status = AuthSessionStatus::Revoked;
        revoked.session.revoked_at_ms = Some(11);
        revoked.session.revoked_reason = Some(SessionRevocationReason::SecurityPolicy);
        assert_eq!(
            TotpManagementBinding::from_authenticated_session(&revoked),
            Err(TotpManagementBindingError::InvalidSession)
        );
    }

    #[async_trait]
    impl TotpPort for FakePort {
        async fn begin_enrollment(
            &self,
            _enrollment: NewTotpEnrollment,
        ) -> Result<BeginTotpEnrollmentOutcome, TotpPortError> {
            Err(TotpPortError::InvalidInput)
        }

        async fn pending_credential(
            &self,
            _user_id: EntityId,
            _now_ms: i64,
        ) -> Result<Option<StoredTotpCredential>, TotpPortError> {
            Ok(None)
        }

        async fn active_credential(
            &self,
            user_id: EntityId,
            _now_ms: i64,
        ) -> Result<Option<StoredTotpCredential>, TotpPortError> {
            Ok((self.active.credential.user_id == user_id).then(|| self.active.clone()))
        }

        async fn activate(
            &self,
            _activation: ActivateTotpCredential<'_>,
        ) -> Result<ActivateTotpCredentialOutcome, TotpPortError> {
            Ok(ActivateTotpCredentialOutcome::Stale)
        }

        async fn advance_step(
            &self,
            advance: TotpStepAdvance,
        ) -> Result<TotpStepAdvanceOutcome, TotpPortError> {
            if let Ok(mut observed) = self.advanced.lock() {
                *observed = Some(advance);
            }
            let mut credential = self.active.credential.clone();
            credential.last_accepted_step = Some(1);
            credential.revision = Revision::from_value(6);
            Ok(TotpStepAdvanceOutcome::Advanced(credential))
        }

        async fn disable(
            &self,
            _disable: DisableTotpCredential,
        ) -> Result<DisableTotpCredentialOutcome, TotpPortError> {
            Ok(DisableTotpCredentialOutcome::Stale)
        }
    }

    fn keyring() -> Keyring {
        let cipher = EnvelopeCipher::from_hex(&"19".repeat(32), 1);
        assert!(cipher.is_ok());
        let Ok(cipher) = cipher else {
            panic!("test key must be valid");
        };
        let keyring = Keyring::from_ciphers(cipher, Vec::new());
        assert!(keyring.is_ok());
        let Ok(keyring) = keyring else {
            panic!("test keyring must be valid");
        };
        keyring
    }

    fn active_fixture(keyring: &Keyring, user_id: EntityId) -> StoredTotpCredential {
        let seed = TotpSeed::from_bytes(b"12345678901234567890");
        assert!(seed.is_ok());
        let Ok(seed) = seed else {
            panic!("RFC seed must be valid");
        };
        let envelope = keyring.encrypt_totp_seed(user_id.into_uuid(), &seed);
        assert!(envelope.is_ok());
        let Ok(envelope) = envelope else {
            panic!("seed encryption must succeed");
        };
        let binding = SecretBinding::new(
            SecretPurpose::TotpSeed,
            SecretOwnerKind::User,
            user_id.into_uuid(),
            TOTP_SEED_SCHEMA_VERSION,
        );
        assert!(binding.is_ok());
        let Ok(binding) = binding else {
            panic!("binding must be valid");
        };
        let secret_record_id = EntityId::new();
        StoredTotpCredential {
            credential: TotpCredential {
                id: EntityId::new(),
                user_id,
                secret_record_id,
                status: TotpCredentialStatus::Active,
                created_at_ms: 1,
                pending_expires_at_ms: None,
                activated_at_ms: Some(2),
                disabled_at_ms: None,
                last_accepted_step: Some(0),
                revision: Revision::from_value(5),
            },
            secret: StoredSecretRecord {
                id: secret_record_id,
                binding,
                envelope,
                created_at_ms: 1,
                rotated_from: None,
                revision: Revision::initial(),
            },
        }
    }

    fn claim(user_id: EntityId, reserved_at_ms: i64) -> AuthChallengeVerificationClaim {
        AuthChallengeVerificationClaim::test_fixture(
            AuthChallenge {
                id: EntityId::new(),
                purpose: AuthChallengePurpose::Login,
                user_id,
                session_id: None,
                auth_revision: Revision::from_value(7),
                allowed_methods: vec![AuthenticationMethod::Totp],
                status: AuthChallengeStatus::VerificationPending,
                rotation_state: AuthChallengeRotationState::NotRequired,
                attempts_used: 1,
                max_attempts: 5,
                created_at_ms: 1,
                expires_at_ms: 120_000,
                verified_method: None,
                achieved_assurance: None,
                consumed_at_ms: None,
                verification_in_progress: true,
                rotation_transaction_in_progress: false,
                has_client_network_context: false,
                has_user_agent_context: false,
                revision: Revision::from_value(1),
            },
            AuthenticationMethod::Totp,
            reserved_at_ms,
        )
    }

    #[tokio::test]
    async fn challenge_window_is_bound_to_reservation_across_a_period_boundary() {
        let keyring = keyring();
        let user_id = EntityId::new();
        let active = active_fixture(&keyring, user_id);
        let advanced = Arc::new(Mutex::new(None));
        let port = FakePort {
            active,
            advanced: advanced.clone(),
        };
        let policy = TotpEnrollmentPolicy::new(600_000);
        assert!(policy.is_ok());
        let Ok(policy) = policy else {
            panic!("policy must be valid");
        };
        let service = TotpService::new(port, keyring, policy, 300_000, FixedClock(90_001));
        assert!(service.is_ok());
        let Ok(service) = service else {
            panic!("service must be valid");
        };
        // Reservation is in step 2, so the RFC step-1 code is valid. Commit is in step 3, where
        // that code would already be outside a window incorrectly anchored to commit time.
        let result = service
            .verify_challenge(claim(user_id, 89_999), "287082")
            .await;
        assert!(matches!(result, Ok(TotpChallengeProofOutcome::Verified(_))));
        assert!(matches!(
            advanced.lock(),
            Ok(ref observed) if matches!(observed.as_ref(), Some(advance)
                if advance.accepted_step == 1
                    && advance.verification_time_ms == 89_999
                    && advance.now_ms == 90_001
                    && advance.expected_auth_revision == Revision::from_value(7))
        ));
    }

    #[tokio::test]
    async fn clock_rollback_never_consumes_totp_replay_state() {
        let keyring = keyring();
        let user_id = EntityId::new();
        let advanced = Arc::new(Mutex::new(None));
        let port = FakePort {
            active: active_fixture(&keyring, user_id),
            advanced: advanced.clone(),
        };
        let policy = TotpEnrollmentPolicy::new(600_000);
        assert!(policy.is_ok());
        let Ok(policy) = policy else {
            panic!("policy must be valid");
        };
        let service = TotpService::new(port, keyring, policy, 300_000, FixedClock(89_998));
        assert!(service.is_ok());
        if let Ok(service) = service {
            assert!(matches!(
                service
                    .verify_challenge(claim(user_id, 89_999), "287082")
                    .await,
                Err(TotpServiceError::ClockUnavailable)
            ));
            assert!(matches!(advanced.lock(), Ok(ref observed) if observed.is_none()));
        }
    }
}
