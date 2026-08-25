//! Domain primitives shared by Master application modules.
//!
//! This crate deliberately has no async runtime, HTTP, SQL, or filesystem dependency.

use std::fmt;

use password_hash::PasswordHash as ParsedPasswordHash;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Stable internal identity. New values are UUIDv7 so indexed writes remain time ordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EntityId(Uuid);

impl EntityId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    #[must_use]
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }

    #[must_use]
    pub const fn from_uuid(value: Uuid) -> Self {
        Self(value)
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

/// Optimistic-concurrency revision for editable aggregate roots.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Revision(u64);

impl Revision {
    #[must_use]
    pub const fn initial() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn from_value(value: u64) -> Self {
        Self(value)
    }

    pub fn next(self) -> Result<Self, RevisionError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(RevisionError::Exhausted)
    }

    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum RevisionError {
    #[error("revision space is exhausted")]
    Exhausted,
}

/// Non-negative byte quantity. Arithmetic is checked at domain boundaries.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ByteCount(u64);

impl ByteCount {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn checked_add(self, other: Self) -> Result<Self, ByteCountError> {
        self.0
            .checked_add(other.0)
            .map(Self)
            .ok_or(ByteCountError::Overflow)
    }

    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ByteCountError {
    #[error("byte count overflow")]
    Overflow,
}

/// Human-readable instance name with bounded storage and no control characters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct InstanceName(String);

impl InstanceName {
    pub fn parse(value: impl Into<String>) -> Result<Self, InstanceNameError> {
        let value = value.into();
        let trimmed = value.trim();
        let length = trimmed.chars().count();
        if length == 0 {
            return Err(InstanceNameError::Empty);
        }
        if length > 80 {
            return Err(InstanceNameError::TooLong);
        }
        if trimmed.chars().any(char::is_control) {
            return Err(InstanceNameError::ControlCharacter);
        }
        Ok(Self(trimmed.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for InstanceName {
    type Error = InstanceNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<InstanceName> for String {
    fn from(value: InstanceName) -> Self {
        value.0
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum InstanceNameError {
    #[error("instance name cannot be empty")]
    Empty,
    #[error("instance name cannot exceed 80 Unicode scalar values")]
    TooLong,
    #[error("instance name cannot contain control characters")]
    ControlCharacter,
}

/// Master control-plane identity. Only one is active in the initial single-instance mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Instance {
    pub id: EntityId,
    pub public_id: EntityId,
    pub name: InstanceName,
    pub created_at_ms: i64,
    pub revision: Revision,
}

/// Policy for refreshing externally sourced subscriptions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalSyncStrategy {
    #[default]
    Scheduled,
    OnRequest,
    Manual,
}

/// Compatibility behavior is explicit; it is never inferred from a license tier.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientCompatibilityMode {
    #[default]
    Strict,
    Legacy,
}

/// Typed WP-01 settings section. Script/template references are separate resources.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SubscriptionBehaviorSettings {
    pub external_sync: ExternalSyncStrategy,
    pub silent_mode: bool,
    pub short_links_enabled: bool,
    pub client_compatibility: ClientCompatibilityMode,
    pub response_headers_enabled: bool,
    pub info_node_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Username(String);

impl Username {
    pub fn parse(value: impl Into<String>) -> Result<Self, UsernameError> {
        let value = value.into();
        let length = value.len();
        if !(3..=32).contains(&length) {
            return Err(UsernameError::InvalidLength);
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
        {
            return Err(UsernameError::InvalidCharacter);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn normalized(&self) -> String {
        self.0.to_ascii_lowercase()
    }
}

impl TryFrom<String> for Username {
    type Error = UsernameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<Username> for String {
    fn from(value: Username) -> Self {
        value.0
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum UsernameError {
    #[error("username must contain 3 to 32 ASCII characters")]
    InvalidLength,
    #[error("username may only contain ASCII letters, digits, underscore, hyphen, or dot")]
    InvalidCharacter,
}

#[derive(Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn parse(value: impl Into<String>) -> Result<Self, PasswordHashError> {
        let value = value.into();
        if value.is_empty() || value.len() > 512 {
            return Err(PasswordHashError::InvalidEncoding);
        }
        let parsed =
            ParsedPasswordHash::new(&value).map_err(|_| PasswordHashError::InvalidEncoding)?;
        let memory_cost = parsed
            .params
            .get_decimal("m")
            .ok_or(PasswordHashError::InvalidEncoding)?;
        let time_cost = parsed
            .params
            .get_decimal("t")
            .ok_or(PasswordHashError::InvalidEncoding)?;
        let parallelism = parsed
            .params
            .get_decimal("p")
            .ok_or(PasswordHashError::InvalidEncoding)?;
        let salt = parsed.salt.ok_or(PasswordHashError::InvalidEncoding)?;
        let output = parsed.hash.ok_or(PasswordHashError::InvalidEncoding)?;
        let mut decoded_salt = [0_u8; 64];
        let decoded_salt_len = salt
            .decode_b64(&mut decoded_salt)
            .map_err(|_| PasswordHashError::InvalidEncoding)?
            .len();
        if parsed.algorithm.as_str() != "argon2id"
            || parsed.version != Some(19)
            || parsed.params.iter().count() != 3
            || !(8_192..=262_144).contains(&memory_cost)
            || !(1..=10).contains(&time_cost)
            || !(1..=8).contains(&parallelism)
            || decoded_salt_len < 8
            || output.len() != 32
        {
            return Err(PasswordHashError::InvalidEncoding);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PasswordHashError {
    #[error("password hash is not a complete, resource-bounded Argon2id v19 PHC string")]
    InvalidEncoding,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrincipalLabel(String);

impl PrincipalLabel {
    pub fn parse(value: impl Into<String>) -> Result<Self, PrincipalLabelError> {
        let value = value.into();
        if value.is_empty() || value.len() > 80 {
            return Err(PrincipalLabelError::InvalidLength);
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
        {
            return Err(PrincipalLabelError::InvalidCharacter);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PrincipalLabelError {
    #[error("principal label must contain 1 to 80 ASCII characters")]
    InvalidLength,
    #[error("principal label may only contain ASCII letters, digits, underscore, hyphen, or dot")]
    InvalidCharacter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserRole {
    Owner,
}

impl UserRole {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Owner => "owner",
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct UserAccount {
    pub id: EntityId,
    pub username: Username,
    pub password_hash: PasswordHash,
    pub role: UserRole,
    pub principal_label: PrincipalLabel,
    pub force_password_change: bool,
    pub revision: Revision,
    pub created_at_ms: i64,
}

#[cfg(test)]
mod tests {
    use super::{
        ByteCount, ByteCountError, EntityId, InstanceName, InstanceNameError, PasswordHash,
        PasswordHashError, PrincipalLabel, PrincipalLabelError, Revision, RevisionError,
    };

    #[test]
    fn generated_entity_ids_are_version_seven() {
        assert_eq!(EntityId::new().into_uuid().get_version_num(), 7);
    }

    #[test]
    fn revision_increment_is_checked() {
        assert_eq!(Revision::initial().next().map(Revision::value), Ok(1));
        assert_eq!(Revision(u64::MAX).next(), Err(RevisionError::Exhausted));
    }

    #[test]
    fn byte_addition_is_checked() {
        assert_eq!(
            ByteCount::new(20)
                .checked_add(ByteCount::new(22))
                .map(ByteCount::value),
            Ok(42)
        );
        assert_eq!(
            ByteCount::new(u64::MAX).checked_add(ByteCount::new(1)),
            Err(ByteCountError::Overflow)
        );
    }

    #[test]
    fn instance_name_is_trimmed_and_bounded() {
        let parsed = InstanceName::parse("  My NodeControll  ");
        assert_eq!(
            parsed.as_ref().map(InstanceName::as_str),
            Ok("My NodeControll")
        );
        assert_eq!(InstanceName::parse("\n"), Err(InstanceNameError::Empty));
        assert_eq!(
            InstanceName::parse("x".repeat(81)),
            Err(InstanceNameError::TooLong)
        );
    }

    #[test]
    fn password_hash_requires_complete_bounded_argon2id_phc() {
        let valid = "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        assert!(PasswordHash::parse(valid).is_ok());
        for invalid in [
            "$argon2id$fixture",
            "$argon2i$v=19$m=19456,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            "$argon2id$v=19$m=1048576,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA",
        ] {
            assert!(matches!(
                PasswordHash::parse(invalid),
                Err(PasswordHashError::InvalidEncoding)
            ));
        }
    }

    #[test]
    fn principal_label_is_bounded_and_machine_safe() {
        assert!(PrincipalLabel::parse("usr_01900000-0000-7000-8000-000000000001").is_ok());
        assert_eq!(
            PrincipalLabel::parse("owner label"),
            Err(PrincipalLabelError::InvalidCharacter)
        );
    }
}
