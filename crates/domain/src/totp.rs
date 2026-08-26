use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{EntityId, Revision};

pub const TOTP_DIGITS: u8 = 6;
pub const TOTP_PERIOD_SECONDS: u32 = 30;
pub const TOTP_VALIDATION_WINDOW: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TotpCredentialStatus {
    Pending,
    Active,
    Disabled,
}

impl TotpCredentialStatus {
    pub fn parse(value: &str) -> Result<Self, TotpCredentialStatusParseError> {
        match value {
            "pending" => Ok(Self::Pending),
            "active" => Ok(Self::Active),
            "disabled" => Ok(Self::Disabled),
            _ => Err(TotpCredentialStatusParseError),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::Disabled => "disabled",
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("unknown TOTP credential status")]
pub struct TotpCredentialStatusParseError;

/// Non-secret projection of a TOTP credential. The encrypted seed is held by persistence and is
/// never part of this domain object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TotpCredential {
    pub id: EntityId,
    pub user_id: EntityId,
    pub secret_record_id: EntityId,
    pub status: TotpCredentialStatus,
    pub created_at_ms: i64,
    pub pending_expires_at_ms: Option<i64>,
    pub activated_at_ms: Option<i64>,
    pub disabled_at_ms: Option<i64>,
    pub last_accepted_step: Option<u64>,
    pub revision: Revision,
}

impl TotpCredential {
    #[must_use]
    pub fn is_pending_at(&self, now_ms: i64) -> bool {
        self.status == TotpCredentialStatus::Pending
            && self
                .pending_expires_at_ms
                .is_some_and(|expires_at_ms| now_ms >= 0 && now_ms < expires_at_ms)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TotpEnrollmentPolicy {
    pending_ttl_ms: i64,
}

impl TotpEnrollmentPolicy {
    pub fn new(pending_ttl_ms: i64) -> Result<Self, TotpEnrollmentPolicyError> {
        if !(30_000..=3_600_000).contains(&pending_ttl_ms) {
            return Err(TotpEnrollmentPolicyError);
        }
        Ok(Self { pending_ttl_ms })
    }

    #[must_use]
    pub const fn pending_ttl_ms(self) -> i64 {
        self.pending_ttl_ms
    }

    pub fn expires_at_ms(self, now_ms: i64) -> Result<i64, TotpEnrollmentPolicyError> {
        if now_ms < 0 {
            return Err(TotpEnrollmentPolicyError);
        }
        now_ms
            .checked_add(self.pending_ttl_ms)
            .ok_or(TotpEnrollmentPolicyError)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("TOTP enrollment policy is invalid")]
pub struct TotpEnrollmentPolicyError;

#[cfg(test)]
mod tests {
    use super::{
        TOTP_DIGITS, TOTP_PERIOD_SECONDS, TOTP_VALIDATION_WINDOW, TotpCredentialStatus,
        TotpEnrollmentPolicy,
    };

    #[test]
    fn totp_profile_is_closed_and_fixed() {
        assert_eq!(TOTP_DIGITS, 6);
        assert_eq!(TOTP_PERIOD_SECONDS, 30);
        assert_eq!(TOTP_VALIDATION_WINDOW, 1);
        for (wire, status) in [
            ("pending", TotpCredentialStatus::Pending),
            ("active", TotpCredentialStatus::Active),
            ("disabled", TotpCredentialStatus::Disabled),
        ] {
            assert_eq!(TotpCredentialStatus::parse(wire), Ok(status));
            assert_eq!(status.as_str(), wire);
        }
        assert!(TotpCredentialStatus::parse("enabled").is_err());
    }

    #[test]
    fn enrollment_ttl_is_bounded_and_checked() {
        let policy = TotpEnrollmentPolicy::new(600_000);
        assert!(matches!(policy, Ok(value) if value.expires_at_ms(1_000) == Ok(601_000)));
        assert!(TotpEnrollmentPolicy::new(29_999).is_err());
        assert!(TotpEnrollmentPolicy::new(3_600_001).is_err());
        assert!(matches!(
            TotpEnrollmentPolicy::new(30_000),
            Ok(value) if value.expires_at_ms(i64::MAX).is_err()
        ));
    }
}
