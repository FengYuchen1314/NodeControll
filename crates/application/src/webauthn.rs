use std::time::Duration;

use nodecontroll_domain::{
    AuthenticationAssurance, AuthenticationMethod, EntityId, Revision, WebAuthnCredential,
    PrincipalLabel, Username, WebAuthnCredentialId, WebAuthnCredentialStatus, WebAuthnNickname,
    WebAuthnOrigin, WebAuthnTransport, WebAuthnUserHandle,
};
use nodecontroll_persistence::{
    AuthSessionStatus, AuthenticatedSession, BeginWebAuthnAuthenticationOutcome,
    BeginWebAuthnRegistrationOutcome, CompleteWebAuthnRegistration,
    CompleteWebAuthnRegistrationOutcome, Database, NewWebAuthnAuthenticationCeremony,
    NewWebAuthnCredential,
    NewWebAuthnRegistrationCeremony, RenameWebAuthnCredential, RevokeWebAuthnCredential,
    RevokeWebAuthnCredentialOutcome, StoredWebAuthnCredential,
    WebAuthnAuthenticationCommit, WebAuthnAuthenticationCommitOutcome,
    WebAuthnAuthenticationHandoff, WebAuthnChallengeBinding, WebAuthnCloneSuspected,
    WebAuthnCloneSuspectedOutcome, WebAuthnRegistrationResult, WebAuthnSessionGuard,
};
use nodecontroll_secrets::Keyring;
use thiserror::Error;
use webauthn_rs::prelude::{
    AttestationFormat, AttestationMetadata, CreationChallengeResponse, Credential,
    Passkey, PasskeyAuthentication, PasskeyRegistration, PublicKeyCredential,
    RegisterPublicKeyCredential, RequestChallengeResponse, Url, Webauthn, WebauthnBuilder,
};
use zeroize::Zeroizing;

use crate::{
    AuthChallengePort, AuthChallengeService, AuthChallengeVerificationClaim,
    PresentAuthChallengeCommand, VerifiedAuthChallengeEvidence,
};

pub trait WebAuthnClock: Send + Sync {
    fn now_utc_ms(&self) -> Result<i64, WebAuthnServiceError>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct SystemWebAuthnClock;

impl WebAuthnClock for SystemWebAuthnClock {
    fn now_utc_ms(&self) -> Result<i64, WebAuthnServiceError> {
        super::unix_time_ms().map_err(|_| WebAuthnServiceError::ClockUnavailable)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WebAuthnPolicy {
    registration_ttl_ms: i64,
    recent_auth_ttl_ms: i64,
    authenticator_timeout_ms: u64,
}

impl WebAuthnPolicy {
    pub fn new(
        registration_ttl_ms: i64,
        recent_auth_ttl_ms: i64,
        authenticator_timeout_ms: u64,
    ) -> Result<Self, WebAuthnServiceError> {
        if !(60_000..=900_000).contains(&registration_ttl_ms)
            || !(60_000..=3_600_000).contains(&recent_auth_ttl_ms)
            || !(60_000..=900_000).contains(&authenticator_timeout_ms)
        {
            return Err(WebAuthnServiceError::InvalidCommand);
        }
        Ok(Self {
            registration_ttl_ms,
            recent_auth_ttl_ms,
            authenticator_timeout_ms,
        })
    }
}

/// A management capability projected from one authenticated, recent-auth session. The private
/// fields prevent an HTTP DTO from manufacturing user/session revision bindings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebAuthnManagementBinding {
    user_id: EntityId,
    actor_session_id: EntityId,
    expected_user_revision: Revision,
    expected_auth_revision: Revision,
    expected_recent_auth_at_ms: i64,
    canonical_username: Username,
    canonical_display_label: PrincipalLabel,
}

impl WebAuthnManagementBinding {
    /// Captures the immutable persistence guard only from the ordinary authenticated-session
    /// projection. Field-wise construction is intentionally unavailable to transport adapters.
    pub fn from_authenticated_session(
        authenticated: &AuthenticatedSession,
    ) -> Result<Self, WebAuthnManagementBindingError> {
        if authenticated.force_password_change {
            return Err(WebAuthnManagementBindingError::PasswordChangeRequired);
        }
        if authenticated.session.status != AuthSessionStatus::Active
            || authenticated.session.revoked_at_ms.is_some()
            || authenticated.session.revoked_reason.is_some()
        {
            return Err(WebAuthnManagementBindingError::InvalidSession);
        }
        Ok(Self {
            user_id: authenticated.user_id,
            actor_session_id: authenticated.session.id,
            expected_user_revision: authenticated.user_revision,
            expected_auth_revision: authenticated.session.auth_revision,
            expected_recent_auth_at_ms: authenticated.session.recent_auth_at_ms,
            canonical_username: authenticated.username.clone(),
            canonical_display_label: authenticated.principal_label.clone(),
        })
    }

    fn guard(&self, now_ms: i64) -> WebAuthnSessionGuard {
        WebAuthnSessionGuard {
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
pub enum WebAuthnManagementBindingError {
    #[error("WebAuthn management is unavailable until the required password change is complete")]
    PasswordChangeRequired,
    #[error("WebAuthn management requires an active authenticated session")]
    InvalidSession,
}

pub struct BeginWebAuthnRegistrationCommand {
    pub binding: WebAuthnManagementBinding,
    pub request_origin: WebAuthnOrigin,
}

pub struct BegunWebAuthnRegistration {
    pub ceremony_id: EntityId,
    pub ceremony_revision: Revision,
    pub expires_at_ms: i64,
    pub options: CreationChallengeResponse,
}

/// Contains raw browser response bytes and therefore intentionally implements neither Debug nor
/// Clone. Those bytes go directly to webauthn-rs and never cross a persistence or logging model.
pub struct FinishWebAuthnRegistrationCommand {
    pub binding: WebAuthnManagementBinding,
    pub ceremony_id: EntityId,
    pub expected_ceremony_revision: Revision,
    pub request_origin: WebAuthnOrigin,
    pub nickname: WebAuthnNickname,
    pub response: RegisterPublicKeyCredential,
}

pub struct BegunWebAuthnAuthentication {
    pub ceremony_id: EntityId,
    pub ceremony_revision: Revision,
    pub claim_id: EntityId,
    pub expires_at_ms: i64,
    pub options: RequestChallengeResponse,
}

/// The original C3 bearer must be presented again at finish so a process restart cannot turn a
/// database identifier into an authentication capability. This also contains raw browser response
/// bytes and intentionally implements neither Debug nor Clone.
pub struct FinishWebAuthnAuthenticationCommand {
    pub challenge: PresentAuthChallengeCommand,
    pub claim_id: EntityId,
    pub ceremony_id: EntityId,
    pub expected_ceremony_revision: Revision,
    pub request_origin: WebAuthnOrigin,
    pub response: PublicKeyCredential,
}

pub enum WebAuthnChallengeProofOutcome {
    Verified(VerifiedAuthChallengeEvidence),
    Rejected(AuthChallengeVerificationClaim),
    CloneSuspected {
        auth_revision: Revision,
        revoked_sessions: u64,
    },
    Stale,
}

pub struct RenameWebAuthnCredentialCommand {
    pub binding: WebAuthnManagementBinding,
    pub request_origin: WebAuthnOrigin,
    pub credential_id: EntityId,
    pub expected_credential_revision: Revision,
    pub nickname: WebAuthnNickname,
}

pub struct RevokeWebAuthnCredentialCommand {
    pub binding: WebAuthnManagementBinding,
    pub request_origin: WebAuthnOrigin,
    pub credential_id: EntityId,
    pub expected_credential_revision: Revision,
}

struct DatabasePasskey {
    passkey: Passkey,
}

#[derive(Clone, Copy)]
enum WebAuthnLibraryOperation {
    RegistrationStart,
    RegistrationFinish,
    AuthenticationStart,
    AuthenticationFinish,
}

impl WebAuthnLibraryOperation {
    const fn as_str(self) -> &'static str {
        match self {
            Self::RegistrationStart => "registration_start",
            Self::RegistrationFinish => "registration_finish",
            Self::AuthenticationStart => "authentication_start",
            Self::AuthenticationFinish => "authentication_finish",
        }
    }
}

struct WebAuthnLibraryFailure;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CounterDecision {
    persisted_counter: u32,
    backup_counter_anomaly: bool,
    clone_suspected: bool,
}

pub struct WebAuthnService<Clock> {
    database: Database,
    keyring: Keyring,
    origin: WebAuthnOrigin,
    webauthn: Webauthn,
    policy: WebAuthnPolicy,
    clock: Clock,
}

impl<Clock> WebAuthnService<Clock>
where
    Clock: WebAuthnClock,
{
    pub fn new(
        database: Database,
        keyring: Keyring,
        origin: WebAuthnOrigin,
        rp_name: &str,
        policy: WebAuthnPolicy,
        clock: Clock,
    ) -> Result<Self, WebAuthnServiceError> {
        if rp_name.trim() != rp_name
            || rp_name.is_empty()
            || rp_name.chars().count() > 100
            || rp_name.chars().any(char::is_control)
        {
            return Err(WebAuthnServiceError::InvalidCommand);
        }
        let rp_origin = Url::parse(origin.as_str())
            .map_err(|_| WebAuthnServiceError::InvalidConfiguration)?;
        // Do not enable allow_subdomains, allow_any_port, alternate origins, or metadata services.
        let webauthn = WebauthnBuilder::new(origin.rp_id(), &rp_origin)
            .map_err(|_| WebAuthnServiceError::InvalidConfiguration)?
            .rp_name(rp_name)
            .timeout(Duration::from_millis(policy.authenticator_timeout_ms))
            .build()
            .map_err(|_| WebAuthnServiceError::InvalidConfiguration)?;
        Ok(Self {
            database,
            keyring,
            origin,
            webauthn,
            policy,
            clock,
        })
    }

    pub async fn begin_registration(
        &self,
        command: BeginWebAuthnRegistrationCommand,
    ) -> Result<BegunWebAuthnRegistration, WebAuthnServiceError> {
        self.require_origin(&command.request_origin)?;
        let now_ms = self.management_now(&command.binding)?;
        let guard = command.binding.guard(now_ms);
        let credentials = self
            .database
            .active_webauthn_credentials_for_registration(&guard)
            .await?;
        let mut exclude = Vec::with_capacity(credentials.len());
        for stored in &credentials {
            let unsealed = self.unseal_database_passkey(stored)?;
            exclude.push(unsealed.passkey.cred_id().clone());
        }
        let (options, state) = invoke_webauthn_library(
            WebAuthnLibraryOperation::RegistrationStart,
            || {
                self.webauthn.start_passkey_registration(
                    command.binding.user_id.into_uuid(),
                    command.binding.canonical_username.as_str(),
                    command.binding.canonical_display_label.as_str(),
                    (!exclude.is_empty()).then_some(exclude),
                )
            },
        )
        .map_err(|_| WebAuthnServiceError::Unavailable)?;
        let ceremony_id = EntityId::new();
        let state_bytes = serialize_zeroizing(&state)?;
        let state = self
            .keyring
            .encrypt_webauthn_registration_state(
                ceremony_id.into_uuid(),
                state_bytes.as_slice(),
            )
            .map_err(|_| WebAuthnServiceError::Unavailable)?;
        let expires_at_ms = now_ms
            .checked_add(self.policy.registration_ttl_ms)
            .ok_or(WebAuthnServiceError::InvalidCommand)?;
        match self
            .database
            .begin_webauthn_registration(&NewWebAuthnRegistrationCeremony {
                id: ceremony_id,
                guard,
                origin: self.origin.clone(),
                expires_at_ms,
                state,
            })
            .await?
        {
            BeginWebAuthnRegistrationOutcome::Created(stored) => {
                Ok(BegunWebAuthnRegistration {
                    ceremony_id: stored.id,
                    ceremony_revision: stored.revision,
                    expires_at_ms: stored.expires_at_ms,
                    options,
                })
            }
            BeginWebAuthnRegistrationOutcome::AlreadyPending => {
                Err(WebAuthnServiceError::AlreadyPending)
            }
            BeginWebAuthnRegistrationOutcome::Stale => Err(WebAuthnServiceError::Stale),
        }
    }

    pub async fn finish_registration(
        &self,
        command: FinishWebAuthnRegistrationCommand,
    ) -> Result<WebAuthnRegistrationResult, WebAuthnServiceError> {
        self.require_origin(&command.request_origin)?;
        let preflight_now_ms = self.management_now(&command.binding)?;
        let preflight_guard = command.binding.guard(preflight_now_ms);
        let Some(ceremony) = self
            .database
            .webauthn_registration_ceremony(
                command.ceremony_id,
                command.expected_ceremony_revision,
                &preflight_guard,
                &self.origin,
            )
            .await?
        else {
            return Err(WebAuthnServiceError::Stale);
        };
        let state_bytes = self
            .keyring
            .decrypt_webauthn_registration_state(
                ceremony.id.into_uuid(),
                &ceremony.state,
            )
            .map_err(|_| WebAuthnServiceError::Unavailable)?;
        let state: PasskeyRegistration = deserialize_typed(state_bytes.as_slice())?;
        let passkey = match invoke_webauthn_library(
            WebAuthnLibraryOperation::RegistrationFinish,
            || {
                self.webauthn
                    .finish_passkey_registration(&command.response, &state)
            },
        ) {
            Ok(passkey) => passkey,
            Err(_) => {
                self.burn_invalid_registration(
                    &ceremony,
                    &command.binding,
                    preflight_now_ms,
                )
                .await?;
                return Err(WebAuthnServiceError::InvalidProof);
            }
        };
        let library_credential: Credential = passkey.clone().into();
        if !library_credential.user_verified
            || library_credential.backup_state && !library_credential.backup_eligible
            || !matches!(
                &library_credential.attestation_format,
                AttestationFormat::None
            )
            || !matches!(
                &library_credential.attestation.metadata,
                AttestationMetadata::None
            )
        {
            self.burn_invalid_registration(&ceremony, &command.binding, preflight_now_ms)
                .await?;
            return Err(WebAuthnServiceError::InvalidProof);
        }
        let credential_id = match WebAuthnCredentialId::parse(
            library_credential.cred_id.as_slice().to_vec(),
        ) {
            Ok(value) => value,
            Err(_) => {
                self.burn_invalid_registration(
                    &ceremony,
                    &command.binding,
                    preflight_now_ms,
                )
                .await?;
                return Err(WebAuthnServiceError::InvalidProof);
            }
        };
        let mut transports = command
            .response
            .response
            .transports
            .as_ref()
            .map(|values| {
                values
                    .iter()
                    .filter_map(|transport| WebAuthnTransport::parse(transport.as_ref()).ok())
                    .collect()
            })
            .unwrap_or_default();
        transports.sort_by_key(|transport| transport.as_str());
        transports.dedup();
        let internal_id = EntityId::new();
        let material_bytes = serialize_zeroizing(&passkey)?;
        let material = self
            .keyring
            .encrypt_webauthn_credential_material(
                internal_id.into_uuid(),
                material_bytes.as_slice(),
            )
            .map_err(|_| WebAuthnServiceError::Unavailable)?;
        let terminal_now_ms = self.fresh_management_terminal_now(
            &command.binding,
            preflight_now_ms,
            ceremony.expires_at_ms,
        )?;
        let terminal_guard = command.binding.guard(terminal_now_ms);
        let credential = NewWebAuthnCredential {
            credential: WebAuthnCredential {
                id: internal_id,
                user_id: command.binding.user_id,
                credential_id,
                user_handle: WebAuthnUserHandle::for_user(command.binding.user_id),
                // The safe attestation-none API intentionally exposes no AAGUID metadata.
                aaguid: None,
                transports,
                user_verified: true,
                backup_eligible: library_credential.backup_eligible,
                backup_state: library_credential.backup_state,
                sign_counter: library_credential.counter,
                nickname: command.nickname,
                status: WebAuthnCredentialStatus::Active,
                created_at_ms: terminal_now_ms,
                last_used_at_ms: None,
                backup_counter_anomaly_at_ms: None,
                revoked_at_ms: None,
                clone_suspected_at_ms: None,
                revision: Revision::initial(),
            },
            material,
        };
        match self
            .database
            .complete_webauthn_registration(&CompleteWebAuthnRegistration {
                ceremony_id: ceremony.id,
                expected_ceremony_revision: ceremony.revision,
                guard: terminal_guard,
                origin: &self.origin,
                credential: &credential,
            })
            .await?
        {
            CompleteWebAuthnRegistrationOutcome::Registered(result) => Ok(result),
            CompleteWebAuthnRegistrationOutcome::DuplicateCredential => {
                Err(WebAuthnServiceError::DuplicateCredential)
            }
            CompleteWebAuthnRegistrationOutcome::Stale => Err(WebAuthnServiceError::Stale),
        }
    }

    pub async fn begin_authentication(
        &self,
        claim: AuthChallengeVerificationClaim,
        request_origin: WebAuthnOrigin,
    ) -> Result<BegunWebAuthnAuthentication, WebAuthnServiceError> {
        self.require_origin(&request_origin)?;
        if claim.method() != AuthenticationMethod::WebAuthn {
            return Err(WebAuthnServiceError::InvalidChallengeClaim);
        }
        let now_ms = self.clock.now_utc_ms()?;
        let binding = challenge_binding(&claim);
        if now_ms < binding.reserved_at_ms || now_ms >= binding.verification_expires_at_ms {
            return Err(WebAuthnServiceError::ClockUnavailable);
        }
        let stored = self
            .database
            .active_webauthn_credentials_for_challenge(&binding, now_ms)
            .await?;
        if stored.is_empty() {
            return Err(WebAuthnServiceError::NoCredentials);
        }
        let mut verifier_copies = Vec::with_capacity(stored.len());
        for credential in &stored {
            let unsealed = self.unseal_database_passkey(credential)?;
            verifier_copies.push(counter_normalized_verifier_copy(unsealed));
        }
        let (options, state) = invoke_webauthn_library(
            WebAuthnLibraryOperation::AuthenticationStart,
            || self.webauthn.start_passkey_authentication(&verifier_copies),
        )
        .map_err(|_| WebAuthnServiceError::Unavailable)?;
        let ceremony_id = EntityId::new();
        let state_bytes = serialize_zeroizing(&state)?;
        let state = self
            .keyring
            .encrypt_webauthn_authentication_state(
                ceremony_id.into_uuid(),
                state_bytes.as_slice(),
            )
            .map_err(|_| WebAuthnServiceError::Unavailable)?;
        match self
            .database
            .begin_webauthn_authentication(&NewWebAuthnAuthenticationCeremony {
                id: ceremony_id,
                binding: binding.clone(),
                origin: self.origin.clone(),
                state,
                created_at_ms: now_ms,
            })
            .await?
        {
            BeginWebAuthnAuthenticationOutcome::Created(stored) => {
                Ok(BegunWebAuthnAuthentication {
                    ceremony_id: stored.id,
                    ceremony_revision: stored.revision,
                    claim_id: binding.claim_id,
                    expires_at_ms: binding.verification_expires_at_ms,
                    options,
                })
            }
            BeginWebAuthnAuthenticationOutcome::AlreadyPending => {
                Err(WebAuthnServiceError::AlreadyPending)
            }
            BeginWebAuthnAuthenticationOutcome::Stale => Err(WebAuthnServiceError::Stale),
        }
    }

    pub async fn finish_authentication<AuthPort>(
        &self,
        auth_challenges: &AuthChallengeService<AuthPort>,
        command: FinishWebAuthnAuthenticationCommand,
    ) -> Result<WebAuthnChallengeProofOutcome, WebAuthnServiceError>
    where
        AuthPort: AuthChallengePort,
    {
        self.require_origin(&command.request_origin)?;
        let preflight_now_ms = self.clock.now_utc_ms()?;
        let PresentAuthChallengeCommand {
            id,
            token,
            client_context,
            now_ms: _,
        } = command.challenge;
        let Some(claim) = auth_challenges
            .resume_verification_claim(
                PresentAuthChallengeCommand {
                    id,
                    token,
                    client_context,
                    now_ms: preflight_now_ms,
                },
                command.claim_id,
                AuthenticationMethod::WebAuthn,
            )
            .await
            .map_err(|_| WebAuthnServiceError::Unavailable)?
        else {
            return Ok(WebAuthnChallengeProofOutcome::Stale);
        };
        let binding = challenge_binding(&claim);
        if let Some(handoff) = self
            .database
            .webauthn_authentication_handoff(
                command.ceremony_id,
                command.expected_ceremony_revision,
                &binding,
                &self.origin,
                preflight_now_ms,
            )
            .await?
        {
            return match handoff {
                WebAuthnAuthenticationHandoff::Verified => verified_outcome(claim),
                WebAuthnAuthenticationHandoff::Rejected => {
                    Ok(WebAuthnChallengeProofOutcome::Rejected(claim))
                }
            };
        }
        let response_credential_id = match WebAuthnCredentialId::parse(
            command.response.get_credential_id().to_vec(),
        ) {
            Ok(value) => value,
            Err(_) => {
                return self
                    .reject_authentication(
                        command.ceremony_id,
                        command.expected_ceremony_revision,
                        binding,
                        claim,
                        preflight_now_ms,
                    )
                    .await;
            }
        };
        let Some((ceremony, stored)) = self
            .database
            .webauthn_authentication_context(
                command.ceremony_id,
                command.expected_ceremony_revision,
                &binding,
                &self.origin,
                &response_credential_id,
                preflight_now_ms,
            )
            .await?
        else {
            return self
                .reject_authentication(
                    command.ceremony_id,
                    command.expected_ceremony_revision,
                    binding,
                    claim,
                    preflight_now_ms,
                )
                .await;
        };
        let state_bytes = self
            .keyring
            .decrypt_webauthn_authentication_state(
                ceremony.id.into_uuid(),
                &ceremony.state,
            )
            .map_err(|_| WebAuthnServiceError::Unavailable)?;
        let state: PasskeyAuthentication = deserialize_typed(state_bytes.as_slice())?;
        let result = match invoke_webauthn_library(
            WebAuthnLibraryOperation::AuthenticationFinish,
            || {
                self.webauthn
                    .finish_passkey_authentication(&command.response, &state)
            },
        ) {
            Ok(result) => result,
            Err(_) => {
                return self
                    .reject_authentication(
                        ceremony.id,
                        ceremony.revision,
                        binding,
                        claim,
                        preflight_now_ms,
                    )
                    .await;
            }
        };
        let expected_user_handle = stored.credential.user_handle.as_bytes();
        if !result.user_verified()
            || result.cred_id().as_slice() != stored.credential.credential_id.as_bytes()
            || command
                .response
                .get_user_unique_id()
                .is_some_and(|value| value != expected_user_handle)
            || !verified_backup_flags(
                stored.credential.backup_eligible,
                result.backup_eligible(),
                result.backup_state(),
            )
        {
            return self
                .reject_authentication(
                    ceremony.id,
                    ceremony.revision,
                    binding,
                    claim,
                    preflight_now_ms,
                )
                .await;
        }
        let decision = counter_decision(
            stored.credential.sign_counter,
            stored.credential.backup_eligible,
            result.counter(),
        );
        if decision.clone_suspected {
            let terminal_now_ms = self
                .fresh_authentication_terminal_now(preflight_now_ms, &binding)?;
            return match self
                .database
                .record_webauthn_clone_suspected(&WebAuthnCloneSuspected {
                    ceremony_id: ceremony.id,
                    expected_ceremony_revision: ceremony.revision,
                    binding: &binding,
                    origin: &self.origin,
                    credential_id: stored.credential.id,
                    expected_credential_revision: stored.credential.revision,
                    expected_sign_counter: stored.credential.sign_counter,
                    now_ms: terminal_now_ms,
                })
                .await?
            {
                WebAuthnCloneSuspectedOutcome::Recorded {
                    auth_revision,
                    revoked_sessions,
                } => Ok(WebAuthnChallengeProofOutcome::CloneSuspected {
                    auth_revision,
                    revoked_sessions,
                }),
                WebAuthnCloneSuspectedOutcome::Stale => {
                    Ok(WebAuthnChallengeProofOutcome::Stale)
                }
            };
        }
        let mut persisted_passkey = self.unseal_database_passkey(&stored)?.passkey;
        if persisted_passkey.update_credential(&result).is_none() {
            return self
                .reject_authentication(
                    ceremony.id,
                    ceremony.revision,
                    binding,
                    claim,
                    preflight_now_ms,
                )
                .await;
        }
        let updated: Credential = persisted_passkey.clone().into();
        if updated.counter != decision.persisted_counter
            || updated.backup_eligible != result.backup_eligible()
            || updated.backup_state != result.backup_state()
        {
            return self
                .reject_authentication(
                    ceremony.id,
                    ceremony.revision,
                    binding,
                    claim,
                    preflight_now_ms,
                )
                .await;
        }
        let material_bytes = serialize_zeroizing(&persisted_passkey)?;
        let material = self
            .keyring
            .encrypt_webauthn_credential_material(
                stored.credential.id.into_uuid(),
                material_bytes.as_slice(),
            )
            .map_err(|_| WebAuthnServiceError::Unavailable)?;
        let terminal_now_ms =
            self.fresh_authentication_terminal_now(preflight_now_ms, &binding)?;
        match self
            .database
            .commit_webauthn_authentication(&WebAuthnAuthenticationCommit {
                ceremony_id: ceremony.id,
                expected_ceremony_revision: ceremony.revision,
                binding: &binding,
                origin: &self.origin,
                credential_id: stored.credential.id,
                expected_credential_revision: stored.credential.revision,
                expected_sign_counter: stored.credential.sign_counter,
                expected_backup_eligible: stored.credential.backup_eligible,
                expected_backup_state: stored.credential.backup_state,
                observed_sign_counter: result.counter(),
                sign_counter: decision.persisted_counter,
                backup_eligible: result.backup_eligible(),
                backup_state: result.backup_state(),
                backup_counter_anomaly: decision.backup_counter_anomaly,
                material: &material,
                now_ms: terminal_now_ms,
            })
            .await?
        {
            WebAuthnAuthenticationCommitOutcome::Committed(_) => {
                verified_outcome(claim)
            }
            WebAuthnAuthenticationCommitOutcome::Stale => {
                Ok(WebAuthnChallengeProofOutcome::Stale)
            }
        }
    }

    pub async fn rename_credential(
        &self,
        command: RenameWebAuthnCredentialCommand,
    ) -> Result<WebAuthnCredential, WebAuthnServiceError> {
        self.require_origin(&command.request_origin)?;
        let now_ms = self.management_now(&command.binding)?;
        self.database
            .rename_webauthn_credential(&RenameWebAuthnCredential {
                credential_id: command.credential_id,
                expected_credential_revision: command.expected_credential_revision,
                nickname: &command.nickname,
                guard: command.binding.guard(now_ms),
            })
            .await?
            .ok_or(WebAuthnServiceError::Stale)
    }

    pub async fn revoke_credential(
        &self,
        command: RevokeWebAuthnCredentialCommand,
    ) -> Result<RevokeWebAuthnCredentialOutcome, WebAuthnServiceError> {
        self.require_origin(&command.request_origin)?;
        let now_ms = self.management_now(&command.binding)?;
        match self
            .database
            .revoke_webauthn_credential(&RevokeWebAuthnCredential {
                credential_id: command.credential_id,
                expected_credential_revision: command.expected_credential_revision,
                guard: command.binding.guard(now_ms),
            })
            .await?
        {
            RevokeWebAuthnCredentialOutcome::Stale => Err(WebAuthnServiceError::Stale),
            outcome => Ok(outcome),
        }
    }

    async fn burn_invalid_registration(
        &self,
        ceremony: &nodecontroll_persistence::StoredWebAuthnRegistrationCeremony,
        binding: &WebAuthnManagementBinding,
        preflight_now_ms: i64,
    ) -> Result<(), WebAuthnServiceError> {
        let terminal_now_ms = self.fresh_management_terminal_now(
            binding,
            preflight_now_ms,
            ceremony.expires_at_ms,
        )?;
        let guard = binding.guard(terminal_now_ms);
        if self
            .database
            .reject_webauthn_registration(
                ceremony.id,
                ceremony.revision,
                &guard,
                &self.origin,
            )
            .await?
        {
            Ok(())
        } else {
            Err(WebAuthnServiceError::Stale)
        }
    }

    async fn reject_authentication(
        &self,
        ceremony_id: EntityId,
        expected_ceremony_revision: Revision,
        binding: WebAuthnChallengeBinding,
        claim: AuthChallengeVerificationClaim,
        preflight_now_ms: i64,
    ) -> Result<WebAuthnChallengeProofOutcome, WebAuthnServiceError> {
        let terminal_now_ms =
            self.fresh_authentication_terminal_now(preflight_now_ms, &binding)?;
        if self
            .database
            .reject_webauthn_authentication(
                ceremony_id,
                expected_ceremony_revision,
                &binding,
                &self.origin,
                terminal_now_ms,
            )
            .await?
        {
            Ok(WebAuthnChallengeProofOutcome::Rejected(claim))
        } else {
            Ok(WebAuthnChallengeProofOutcome::Stale)
        }
    }

    fn unseal_database_passkey(
        &self,
        stored: &StoredWebAuthnCredential,
    ) -> Result<DatabasePasskey, WebAuthnServiceError> {
        let bytes = self
            .keyring
            .decrypt_webauthn_credential_material(
                stored.credential.id.into_uuid(),
                &stored.material,
            )
            .map_err(|_| WebAuthnServiceError::Unavailable)?;
        let passkey: Passkey = deserialize_typed(bytes.as_slice())?;
        let library: Credential = passkey.clone().into();
        if library.cred_id.as_slice() != stored.credential.credential_id.as_bytes()
            || library.user_verified != stored.credential.user_verified
            || library.backup_eligible != stored.credential.backup_eligible
            || library.backup_state != stored.credential.backup_state
            || library.counter != stored.credential.sign_counter
            || !matches!(&library.attestation_format, AttestationFormat::None)
            || !matches!(&library.attestation.metadata, AttestationMetadata::None)
        {
            return Err(WebAuthnServiceError::CorruptCredential);
        }
        Ok(DatabasePasskey { passkey })
    }

    fn management_now(
        &self,
        binding: &WebAuthnManagementBinding,
    ) -> Result<i64, WebAuthnServiceError> {
        let now_ms = self.clock.now_utc_ms()?;
        self.validate_management_time(binding, now_ms)?;
        Ok(now_ms)
    }

    fn fresh_management_terminal_now(
        &self,
        binding: &WebAuthnManagementBinding,
        preflight_now_ms: i64,
        ceremony_expires_at_ms: i64,
    ) -> Result<i64, WebAuthnServiceError> {
        let terminal_now_ms = sample_fresh_terminal_time(
            &self.clock,
            preflight_now_ms,
            ceremony_expires_at_ms,
        )?;
        self.validate_management_time(binding, terminal_now_ms)?;
        Ok(terminal_now_ms)
    }

    fn fresh_authentication_terminal_now(
        &self,
        preflight_now_ms: i64,
        binding: &WebAuthnChallengeBinding,
    ) -> Result<i64, WebAuthnServiceError> {
        sample_fresh_terminal_time(
            &self.clock,
            preflight_now_ms,
            binding.verification_expires_at_ms,
        )
    }

    fn validate_management_time(
        &self,
        binding: &WebAuthnManagementBinding,
        now_ms: i64,
    ) -> Result<(), WebAuthnServiceError> {
        let expires_at_ms = binding
            .expected_recent_auth_at_ms
            .checked_add(self.policy.recent_auth_ttl_ms)
            .ok_or(WebAuthnServiceError::InvalidCommand)?;
        if binding.expected_recent_auth_at_ms < 0
            || now_ms < binding.expected_recent_auth_at_ms
            || now_ms >= expires_at_ms
        {
            return Err(WebAuthnServiceError::RecentAuthRequired);
        }
        Ok(())
    }

    fn require_origin(&self, request_origin: &WebAuthnOrigin) -> Result<(), WebAuthnServiceError> {
        require_exact_origin(&self.origin, request_origin)
    }
}

fn sample_fresh_terminal_time<Clock: WebAuthnClock>(
    clock: &Clock,
    preflight_now_ms: i64,
    expires_at_ms: i64,
) -> Result<i64, WebAuthnServiceError> {
    validate_fresh_terminal_time(preflight_now_ms, clock.now_utc_ms()?, expires_at_ms)
}

fn validate_fresh_terminal_time(
    preflight_now_ms: i64,
    terminal_now_ms: i64,
    expires_at_ms: i64,
) -> Result<i64, WebAuthnServiceError> {
    if terminal_now_ms < preflight_now_ms {
        Err(WebAuthnServiceError::ClockUnavailable)
    } else if terminal_now_ms >= expires_at_ms {
        Err(WebAuthnServiceError::Stale)
    } else {
        Ok(terminal_now_ms)
    }
}

fn require_exact_origin(
    configured_origin: &WebAuthnOrigin,
    request_origin: &WebAuthnOrigin,
) -> Result<(), WebAuthnServiceError> {
    if request_origin == configured_origin {
        Ok(())
    } else {
        Err(WebAuthnServiceError::InvalidOrigin)
    }
}

fn challenge_binding(claim: &AuthChallengeVerificationClaim) -> WebAuthnChallengeBinding {
    WebAuthnChallengeBinding {
        auth_challenge_id: claim.challenge().id,
        claim_id: claim.claim_id(),
        purpose: claim.challenge().purpose,
        user_id: claim.challenge().user_id,
        session_id: claim.challenge().session_id,
        auth_revision: claim.challenge().auth_revision,
        reserved_at_ms: claim.reserved_at_ms(),
        verification_expires_at_ms: claim.verification_expires_at_ms(),
        client_context: claim.client_context().clone(),
    }
}

/// Only `unseal_database_passkey` can construct this wrapper. Counter normalization therefore
/// cannot be applied to caller-supplied material or to the passkey that is persisted after finish.
fn counter_normalized_verifier_copy(unsealed: DatabasePasskey) -> Passkey {
    let mut credential: Credential = unsealed.passkey.into();
    credential.counter = 0;
    Passkey::from(credential)
}

/// Runs the synchronous library boundary with a thread-local no-op subscriber. webauthn-rs 0.5.5
/// contains tracing calls that may format parser inputs; the outer application event records only
/// a stable operation/reason pair after the library error and its source have been dropped.
fn invoke_webauthn_library<T, E>(
    operation: WebAuthnLibraryOperation,
    invoke: impl FnOnce() -> Result<T, E>,
) -> Result<T, WebAuthnLibraryFailure> {
    let result =
        tracing::subscriber::with_default(tracing::subscriber::NoSubscriber::new(), invoke);
    match result {
        Ok(value) => Ok(value),
        Err(library_error) => {
            drop(library_error);
            tracing::warn!(
                target: "nodecontroll_application::webauthn",
                operation = operation.as_str(),
                reason = "library_rejected",
                "WebAuthn library operation failed"
            );
            Err(WebAuthnLibraryFailure)
        }
    }
}

fn counter_decision(
    stored_counter: u32,
    backup_eligible: bool,
    result_counter: u32,
) -> CounterDecision {
    if !backup_eligible
        && (stored_counter > 0 || result_counter > 0)
        && result_counter <= stored_counter
    {
        return CounterDecision {
            persisted_counter: stored_counter,
            backup_counter_anomaly: false,
            clone_suspected: true,
        };
    }
    CounterDecision {
        persisted_counter: stored_counter.max(result_counter),
        backup_counter_anomaly: backup_eligible
            && (stored_counter > 0 || result_counter > 0)
            && result_counter <= stored_counter,
        clone_suspected: false,
    }
}

fn verified_backup_flags(
    stored_backup_eligible: bool,
    result_backup_eligible: bool,
    result_backup_state: bool,
) -> bool {
    result_backup_eligible == stored_backup_eligible
        && (!result_backup_state || result_backup_eligible)
}

fn verified_outcome(
    claim: AuthChallengeVerificationClaim,
) -> Result<WebAuthnChallengeProofOutcome, WebAuthnServiceError> {
    let evidence = VerifiedAuthChallengeEvidence::from_method_verifier(
        claim,
        AuthenticationAssurance::PhishingResistant,
    )
    .map_err(|_| WebAuthnServiceError::InvalidChallengeClaim)?;
    Ok(WebAuthnChallengeProofOutcome::Verified(evidence))
}

fn serialize_zeroizing<T: serde::Serialize>(
    value: &T,
) -> Result<Zeroizing<Vec<u8>>, WebAuthnServiceError> {
    let mut bytes = Zeroizing::new(Vec::new());
    serde_json::to_writer(&mut *bytes, value).map_err(|_| WebAuthnServiceError::Unavailable)?;
    Ok(bytes)
}

fn deserialize_typed<T: serde::de::DeserializeOwned>(
    value: &[u8],
) -> Result<T, WebAuthnServiceError> {
    serde_json::from_slice(value).map_err(|_| WebAuthnServiceError::Unavailable)
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum WebAuthnServiceError {
    #[error("WebAuthn configuration is invalid")]
    InvalidConfiguration,
    #[error("WebAuthn command is invalid")]
    InvalidCommand,
    #[error("the request Origin does not exactly match the configured public origin")]
    InvalidOrigin,
    #[error("recent authentication is required")]
    RecentAuthRequired,
    #[error("another WebAuthn ceremony is already pending")]
    AlreadyPending,
    #[error("no active WebAuthn credential is available")]
    NoCredentials,
    #[error("the WebAuthn proof is invalid")]
    InvalidProof,
    #[error("the credential ID is already registered")]
    DuplicateCredential,
    #[error("the WebAuthn credential or ceremony changed concurrently")]
    Stale,
    #[error("the C3 claim is for another verifier")]
    InvalidChallengeClaim,
    #[error("encrypted WebAuthn credential material is inconsistent with its projection")]
    CorruptCredential,
    #[error("the controlled UTC clock is unavailable or inconsistent")]
    ClockUnavailable,
    #[error("the WebAuthn service is unavailable")]
    Unavailable,
}

impl From<nodecontroll_persistence::PersistenceError> for WebAuthnServiceError {
    fn from(error: nodecontroll_persistence::PersistenceError) -> Self {
        use nodecontroll_persistence::PersistenceError;
        match error {
            PersistenceError::InvalidTimestamp
            | PersistenceError::RevisionOutOfRange
            | PersistenceError::InvalidKeyVersion
            | PersistenceError::InvalidWebAuthnCeremony
            | PersistenceError::InvalidWebAuthnCredential => Self::InvalidCommand,
            PersistenceError::SessionPrincipalUnavailable
            | PersistenceError::AuthStateUnavailable
            | PersistenceError::SessionRevisionConflict => Self::Stale,
            PersistenceError::InvalidStoredWebAuthnCeremony
            | PersistenceError::InvalidStoredWebAuthnCredential => Self::CorruptCredential,
            _ => Self::Unavailable,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::VecDeque,
        io::{self, Write},
        sync::{Arc, Mutex},
    };

    use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt};

    use nodecontroll_domain::{PrincipalLabel, Revision, UserRole, Username, WebAuthnOrigin};
    use nodecontroll_persistence::{
        AuthLevel, AuthSessionStatus, AuthSessionSummary, AuthenticatedSession,
        SessionRevocationReason, WebAuthnSessionGuard,
    };

    use super::{
        CounterDecision, WebAuthnClock, WebAuthnLibraryOperation, WebAuthnManagementBinding,
        WebAuthnManagementBindingError, WebAuthnServiceError, counter_decision,
        invoke_webauthn_library, require_exact_origin, sample_fresh_terminal_time,
        verified_backup_flags,
    };

    #[derive(Clone, Default)]
    struct CapturedWriter {
        bytes: Arc<Mutex<Vec<u8>>>,
    }

    impl CapturedWriter {
        fn output(&self) -> String {
            match self.bytes.lock() {
                Ok(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
                Err(_) => String::new(),
            }
        }
    }

    impl Write for CapturedWriter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            let mut bytes = self
                .bytes
                .lock()
                .map_err(|_| io::Error::other("captured WebAuthn log lock poisoned"))?;
            bytes.extend_from_slice(buffer);
            Ok(buffer.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl<'writer> MakeWriter<'writer> for CapturedWriter {
        type Writer = Self;

        fn make_writer(&'writer self) -> Self::Writer {
            self.clone()
        }
    }

    struct ScriptedClock {
        readings: Mutex<VecDeque<i64>>,
    }

    impl ScriptedClock {
        fn new(readings: [i64; 2]) -> Self {
            Self {
                readings: Mutex::new(VecDeque::from(readings)),
            }
        }
    }

    impl WebAuthnClock for ScriptedClock {
        fn now_utc_ms(&self) -> Result<i64, WebAuthnServiceError> {
            self.readings
                .lock()
                .map_err(|_| WebAuthnServiceError::ClockUnavailable)?
                .pop_front()
                .ok_or(WebAuthnServiceError::ClockUnavailable)
        }
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
                id: nodecontroll_domain::EntityId::new(),
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
            user_id: nodecontroll_domain::EntityId::new(),
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
        let binding = WebAuthnManagementBinding::from_authenticated_session(&authenticated);
        assert!(binding.is_ok());
        let Ok(binding) = binding else {
            panic!("eligible session must create a management binding");
        };
        assert_eq!(
            binding.canonical_username.as_str(),
            authenticated.username.as_str()
        );
        assert_eq!(
            binding.canonical_display_label.as_str(),
            authenticated.principal_label.as_str()
        );
        assert_eq!(
            binding.guard(10),
            WebAuthnSessionGuard {
                user_id: authenticated.user_id,
                actor_session_id: authenticated.session.id,
                expected_user_revision: authenticated.user_revision,
                expected_auth_revision: authenticated.session.auth_revision,
                expected_recent_auth_at_ms: authenticated.session.recent_auth_at_ms,
                now_ms: 10,
            }
        );
        assert_eq!(
            WebAuthnManagementBinding::from_authenticated_session(&authenticated_session(true)),
            Err(WebAuthnManagementBindingError::PasswordChangeRequired)
        );

        let mut revoked = authenticated_session(false);
        revoked.session.status = AuthSessionStatus::Revoked;
        revoked.session.revoked_at_ms = Some(11);
        revoked.session.revoked_reason = Some(SessionRevocationReason::SecurityPolicy);
        assert_eq!(
            WebAuthnManagementBinding::from_authenticated_session(&revoked),
            Err(WebAuthnManagementBindingError::InvalidSession)
        );
    }

    #[test]
    fn every_management_mutation_uses_the_exact_public_origin_guard() {
        let configured = WebAuthnOrigin::parse("https://control.example.test");
        let same = WebAuthnOrigin::parse("https://control.example.test");
        let different = WebAuthnOrigin::parse("https://other.example.test");
        assert!(configured.is_ok() && same.is_ok() && different.is_ok());
        let (Ok(configured), Ok(same), Ok(different)) = (configured, same, different) else {
            panic!("origin fixtures must be valid");
        };
        assert_eq!(require_exact_origin(&configured, &same), Ok(()));
        assert_eq!(
            require_exact_origin(&configured, &different),
            Err(super::WebAuthnServiceError::InvalidOrigin)
        );
    }

    #[test]
    fn second_controlled_clock_sample_must_remain_monotonic_and_before_terminal_expiry() {
        let within_window = ScriptedClock::new([100, 199]);
        let first = within_window.now_utc_ms();
        assert_eq!(first, Ok(100));
        assert_eq!(
            sample_fresh_terminal_time(&within_window, 100, 200),
            Ok(199)
        );

        let crossed_expiry = ScriptedClock::new([100, 200]);
        assert_eq!(crossed_expiry.now_utc_ms(), Ok(100));
        assert_eq!(
            sample_fresh_terminal_time(&crossed_expiry, 100, 200),
            Err(WebAuthnServiceError::Stale)
        );

        let moved_backwards = ScriptedClock::new([100, 99]);
        assert_eq!(moved_backwards.now_utc_ms(), Ok(100));
        assert_eq!(
            sample_fresh_terminal_time(&moved_backwards, 100, 200),
            Err(WebAuthnServiceError::ClockUnavailable)
        );
    }

    #[test]
    fn counter_policy_distinguishes_device_bound_and_synced_passkeys() {
        assert_eq!(
            counter_decision(8, false, 8),
            CounterDecision {
                persisted_counter: 8,
                backup_counter_anomaly: false,
                clone_suspected: true,
            }
        );
        assert_eq!(
            counter_decision(8, true, 7),
            CounterDecision {
                persisted_counter: 8,
                backup_counter_anomaly: true,
                clone_suspected: false,
            }
        );
        assert_eq!(
            counter_decision(0, true, 0),
            CounterDecision {
                persisted_counter: 0,
                backup_counter_anomaly: false,
                clone_suspected: false,
            }
        );
        assert_eq!(
            counter_decision(8, true, 9),
            CounterDecision {
                persisted_counter: 9,
                backup_counter_anomaly: false,
                clone_suspected: false,
            }
        );
    }

    #[test]
    fn backup_eligibility_is_registration_time_invariant_and_state_implies_eligibility() {
        assert!(verified_backup_flags(false, false, false));
        assert!(verified_backup_flags(true, true, false));
        assert!(verified_backup_flags(true, true, true));
        assert!(!verified_backup_flags(false, true, false));
        assert!(!verified_backup_flags(true, false, false));
        assert!(!verified_backup_flags(false, false, true));
    }

    #[test]
    fn library_boundary_suppresses_raw_dependency_event_and_audits_only_stable_reason() {
        let writer = CapturedWriter::default();
        let subscriber = tracing_subscriber::registry().with(
            tracing_subscriber::fmt::layer()
                .without_time()
                .with_ansi(false)
                .with_writer(writer.clone()),
        );
        tracing::subscriber::with_default(subscriber, || {
            let result = invoke_webauthn_library(
                WebAuthnLibraryOperation::AuthenticationFinish,
                || {
                    tracing::error!(
                        target: "webauthn_rs_core::core",
                        client_data_json = "raw-client-data-secret",
                        "dependency-parser-error"
                    );
                    Err::<(), ()>(())
                },
            );
            assert!(result.is_err());
        });

        let output = writer.output();
        assert!(output.contains("authentication_finish"));
        assert!(output.contains("library_rejected"));
        assert!(!output.contains("raw-client-data-secret"));
        assert!(!output.contains("dependency-parser-error"));
    }
}
