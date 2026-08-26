use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{EntityId, Revision};

/// Concrete proof mechanism. A method never implies an assurance level by itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "totp")]
    Totp,
    #[serde(rename = "webauthn")]
    WebAuthn,
    #[serde(rename = "recovery_code")]
    RecoveryCode,
}

impl AuthenticationMethod {
    pub fn parse(value: &str) -> Result<Self, AuthenticationMethodParseError> {
        match value {
            "password" => Ok(Self::Password),
            "totp" => Ok(Self::Totp),
            "webauthn" => Ok(Self::WebAuthn),
            "recovery_code" => Ok(Self::RecoveryCode),
            _ => Err(AuthenticationMethodParseError),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Password => "password",
            Self::Totp => "totp",
            Self::WebAuthn => "webauthn",
            Self::RecoveryCode => "recovery_code",
        }
    }

    /// Defense-in-depth boundary for evidence emitted by a method-specific verifier.
    ///
    /// The verifier still decides which WebAuthn assurance was actually achieved. This matrix
    /// only prevents a caller from attaching an assurance that the concrete method can never
    /// establish.
    #[must_use]
    pub const fn permits_assurance(self, assurance: AuthenticationAssurance) -> bool {
        matches!(
            (self, assurance),
            (Self::Password, AuthenticationAssurance::Password)
                | (Self::Totp, AuthenticationAssurance::Mfa)
                | (Self::WebAuthn, AuthenticationAssurance::Mfa)
                | (Self::WebAuthn, AuthenticationAssurance::PhishingResistant)
                | (Self::RecoveryCode, AuthenticationAssurance::Recovery)
        )
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("unknown authentication method")]
pub struct AuthenticationMethodParseError;

/// Assurance attached to an accepted proof and, later, to the resulting session.
///
/// This is deliberately separate from [`AuthenticationMethod`]. For example, a WebAuthn
/// ceremony can yield different assurance depending on its verified properties. Only the
/// method-specific verifier may select the achieved assurance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthenticationAssurance {
    Password,
    Mfa,
    PhishingResistant,
    Recovery,
}

impl AuthenticationAssurance {
    pub fn parse(value: &str) -> Result<Self, AuthenticationAssuranceParseError> {
        match value {
            "password" => Ok(Self::Password),
            "mfa" => Ok(Self::Mfa),
            "phishing_resistant" => Ok(Self::PhishingResistant),
            "recovery" => Ok(Self::Recovery),
            _ => Err(AuthenticationAssuranceParseError),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Password => "password",
            Self::Mfa => "mfa",
            Self::PhishingResistant => "phishing_resistant",
            Self::Recovery => "recovery",
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("unknown authentication assurance")]
pub struct AuthenticationAssuranceParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthChallengePurpose {
    Login,
    Reauthenticate,
    SensitiveAction,
    CredentialEnrollment,
}

impl AuthChallengePurpose {
    pub fn parse(value: &str) -> Result<Self, AuthChallengePurposeParseError> {
        match value {
            "login" => Ok(Self::Login),
            "reauthenticate" => Ok(Self::Reauthenticate),
            "sensitive_action" => Ok(Self::SensitiveAction),
            "credential_enrollment" => Ok(Self::CredentialEnrollment),
            _ => Err(AuthChallengePurposeParseError),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Login => "login",
            Self::Reauthenticate => "reauthenticate",
            Self::SensitiveAction => "sensitive_action",
            Self::CredentialEnrollment => "credential_enrollment",
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("unknown authentication challenge purpose")]
pub struct AuthChallengePurposeParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthChallengeStatus {
    Pending,
    VerificationPending,
    RotationPending,
    Consumed,
    Exhausted,
    Expired,
    Invalidated,
}

impl AuthChallengeStatus {
    pub fn parse(value: &str) -> Result<Self, AuthChallengeStatusParseError> {
        match value {
            "pending" => Ok(Self::Pending),
            "verification_pending" => Ok(Self::VerificationPending),
            "rotation_pending" => Ok(Self::RotationPending),
            "consumed" => Ok(Self::Consumed),
            "exhausted" => Ok(Self::Exhausted),
            "expired" => Ok(Self::Expired),
            "invalidated" => Ok(Self::Invalidated),
            _ => Err(AuthChallengeStatusParseError),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::VerificationPending => "verification_pending",
            Self::RotationPending => "rotation_pending",
            Self::Consumed => "consumed",
            Self::Exhausted => "exhausted",
            Self::Expired => "expired",
            Self::Invalidated => "invalidated",
        }
    }

    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Consumed | Self::Exhausted | Self::Expired | Self::Invalidated
        )
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("unknown authentication challenge status")]
pub struct AuthChallengeStatusParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthChallengeRotationState {
    NotRequired,
    Required,
    Pending,
    Completed,
}

impl AuthChallengeRotationState {
    pub fn parse(value: &str) -> Result<Self, AuthChallengeRotationStateParseError> {
        match value {
            "not_required" => Ok(Self::NotRequired),
            "required" => Ok(Self::Required),
            "pending" => Ok(Self::Pending),
            "completed" => Ok(Self::Completed),
            _ => Err(AuthChallengeRotationStateParseError),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Required => "required",
            Self::Pending => "pending",
            Self::Completed => "completed",
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("unknown authentication challenge rotation state")]
pub struct AuthChallengeRotationStateParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthChallengePolicy {
    ttl_ms: i64,
    verification_timeout_ms: i64,
    max_attempts: u32,
}

impl AuthChallengePolicy {
    pub fn new(
        ttl_ms: i64,
        verification_timeout_ms: i64,
        max_attempts: u32,
    ) -> Result<Self, AuthChallengePolicyError> {
        if ttl_ms <= 0
            || verification_timeout_ms <= 0
            || verification_timeout_ms > ttl_ms
            || max_attempts == 0
            || max_attempts > i32::MAX as u32
        {
            return Err(AuthChallengePolicyError);
        }
        Ok(Self {
            ttl_ms,
            verification_timeout_ms,
            max_attempts,
        })
    }

    #[must_use]
    pub const fn ttl_ms(self) -> i64 {
        self.ttl_ms
    }

    #[must_use]
    pub const fn max_attempts(self) -> u32 {
        self.max_attempts
    }

    #[must_use]
    pub const fn verification_timeout_ms(self) -> i64 {
        self.verification_timeout_ms
    }

    pub fn expires_at_ms(self, created_at_ms: i64) -> Result<i64, AuthChallengePolicyError> {
        if created_at_ms < 0 {
            return Err(AuthChallengePolicyError);
        }
        created_at_ms
            .checked_add(self.ttl_ms)
            .ok_or(AuthChallengePolicyError)
    }

    pub fn verification_expires_at_ms(
        self,
        started_at_ms: i64,
    ) -> Result<i64, AuthChallengePolicyError> {
        if started_at_ms < 0 {
            return Err(AuthChallengePolicyError);
        }
        started_at_ms
            .checked_add(self.verification_timeout_ms)
            .ok_or(AuthChallengePolicyError)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("authentication challenge policy is invalid")]
pub struct AuthChallengePolicyError;

/// Non-secret projection of one durable challenge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthChallenge {
    pub id: EntityId,
    pub purpose: AuthChallengePurpose,
    pub user_id: EntityId,
    pub session_id: Option<EntityId>,
    pub auth_revision: Revision,
    pub allowed_methods: Vec<AuthenticationMethod>,
    pub status: AuthChallengeStatus,
    pub rotation_state: AuthChallengeRotationState,
    /// Attempt slots are reserved before proof verification begins. A successful final slot is
    /// therefore valid even when this value equals `max_attempts`.
    pub attempts_used: u32,
    pub max_attempts: u32,
    pub created_at_ms: i64,
    pub expires_at_ms: i64,
    pub verified_method: Option<AuthenticationMethod>,
    pub achieved_assurance: Option<AuthenticationAssurance>,
    pub consumed_at_ms: Option<i64>,
    pub verification_in_progress: bool,
    pub rotation_transaction_in_progress: bool,
    pub has_client_network_context: bool,
    pub has_user_agent_context: bool,
    pub revision: Revision,
}

impl AuthChallenge {
    #[must_use]
    pub fn remaining_attempts(&self) -> u32 {
        self.max_attempts.saturating_sub(self.attempts_used)
    }

    #[must_use]
    pub fn is_expired_at(&self, now_ms: i64) -> bool {
        now_ms >= self.expires_at_ms
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AuthChallengePolicy, AuthChallengePurpose, AuthChallengeRotationState, AuthChallengeStatus,
        AuthenticationAssurance, AuthenticationMethod,
    };

    #[test]
    fn authentication_methods_and_assurances_have_independent_canonical_vocabularies() {
        for (encoded, method) in [
            ("password", AuthenticationMethod::Password),
            ("totp", AuthenticationMethod::Totp),
            ("webauthn", AuthenticationMethod::WebAuthn),
            ("recovery_code", AuthenticationMethod::RecoveryCode),
        ] {
            assert_eq!(AuthenticationMethod::parse(encoded), Ok(method));
            assert_eq!(method.as_str(), encoded);
        }
        for (encoded, assurance) in [
            ("password", AuthenticationAssurance::Password),
            ("mfa", AuthenticationAssurance::Mfa),
            (
                "phishing_resistant",
                AuthenticationAssurance::PhishingResistant,
            ),
            ("recovery", AuthenticationAssurance::Recovery),
        ] {
            assert_eq!(AuthenticationAssurance::parse(encoded), Ok(assurance));
            assert_eq!(assurance.as_str(), encoded);
        }
        assert!(AuthenticationMethod::parse("mfa").is_err());
        assert!(AuthenticationAssurance::parse("webauthn").is_err());
    }

    #[test]
    fn challenge_state_vocabularies_are_closed() {
        for (encoded, value) in [
            ("login", AuthChallengePurpose::Login),
            ("reauthenticate", AuthChallengePurpose::Reauthenticate),
            ("sensitive_action", AuthChallengePurpose::SensitiveAction),
            (
                "credential_enrollment",
                AuthChallengePurpose::CredentialEnrollment,
            ),
        ] {
            assert_eq!(AuthChallengePurpose::parse(encoded), Ok(value));
            assert_eq!(value.as_str(), encoded);
        }
        for (encoded, value) in [
            ("pending", AuthChallengeStatus::Pending),
            (
                "verification_pending",
                AuthChallengeStatus::VerificationPending,
            ),
            ("rotation_pending", AuthChallengeStatus::RotationPending),
            ("consumed", AuthChallengeStatus::Consumed),
            ("exhausted", AuthChallengeStatus::Exhausted),
            ("expired", AuthChallengeStatus::Expired),
            ("invalidated", AuthChallengeStatus::Invalidated),
        ] {
            assert_eq!(AuthChallengeStatus::parse(encoded), Ok(value));
            assert_eq!(value.as_str(), encoded);
        }
        for (encoded, value) in [
            ("not_required", AuthChallengeRotationState::NotRequired),
            ("required", AuthChallengeRotationState::Required),
            ("pending", AuthChallengeRotationState::Pending),
            ("completed", AuthChallengeRotationState::Completed),
        ] {
            assert_eq!(AuthChallengeRotationState::parse(encoded), Ok(value));
            assert_eq!(value.as_str(), encoded);
        }
    }

    #[test]
    fn challenge_policy_checks_ttl_attempts_and_overflow() {
        let policy = AuthChallengePolicy::new(120_000, 10_000, 5);
        assert!(matches!(policy, Ok(value)
            if value.expires_at_ms(1_000) == Ok(121_000)
                && value.verification_expires_at_ms(1_000) == Ok(11_000)));
        assert!(AuthChallengePolicy::new(0, 1, 5).is_err());
        assert!(AuthChallengePolicy::new(10, 0, 5).is_err());
        assert!(AuthChallengePolicy::new(10, 11, 5).is_err());
        assert!(AuthChallengePolicy::new(1, 1, 0).is_err());
        assert!(AuthChallengePolicy::new(1, 1, i32::MAX as u32 + 1).is_err());
        assert!(matches!(
            AuthChallengePolicy::new(i64::MAX, 1, 1),
            Ok(value) if value.expires_at_ms(1).is_err()
        ));
    }

    #[test]
    fn method_assurance_matrix_rejects_escalation_and_cross_method_labels() {
        for (method, allowed) in [
            (
                AuthenticationMethod::Password,
                vec![AuthenticationAssurance::Password],
            ),
            (
                AuthenticationMethod::Totp,
                vec![AuthenticationAssurance::Mfa],
            ),
            (
                AuthenticationMethod::WebAuthn,
                vec![
                    AuthenticationAssurance::Mfa,
                    AuthenticationAssurance::PhishingResistant,
                ],
            ),
            (
                AuthenticationMethod::RecoveryCode,
                vec![AuthenticationAssurance::Recovery],
            ),
        ] {
            for assurance in [
                AuthenticationAssurance::Password,
                AuthenticationAssurance::Mfa,
                AuthenticationAssurance::PhishingResistant,
                AuthenticationAssurance::Recovery,
            ] {
                assert_eq!(
                    method.permits_assurance(assurance),
                    allowed.contains(&assurance),
                    "unexpected matrix result for {method:?}/{assurance:?}"
                );
            }
        }
    }
}
