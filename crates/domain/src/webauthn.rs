use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

use crate::{EntityId, Revision};

pub const WEBAUTHN_CREDENTIAL_ID_MIN_BYTES: usize = 16;
pub const WEBAUTHN_CREDENTIAL_ID_MAX_BYTES: usize = 1_023;
pub const WEBAUTHN_USER_HANDLE_MAX_BYTES: usize = 64;
pub const WEBAUTHN_NICKNAME_MAX_CHARS: usize = 80;

/// Canonical public WebAuthn origin and its exact-host RP ID.
///
/// NodeControll intentionally does not accept a separately configured RP ID. Deriving it from the
/// canonical HTTPS origin prevents a typo or later config split from silently binding credentials
/// to a broader parent domain. Request origins are parsed through this same type and compared for
/// exact equality; subdomains and alternate ports are never implicit.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WebAuthnOrigin {
    canonical: String,
    rp_id: String,
}

impl WebAuthnOrigin {
    pub fn parse(value: impl Into<String>) -> Result<Self, WebAuthnOriginError> {
        let value = value.into();
        if value.trim() != value || value.is_empty() || value.len() > 2_048 {
            return Err(WebAuthnOriginError);
        }
        let parsed = Url::parse(&value).map_err(|_| WebAuthnOriginError)?;
        let rp_id = parsed.domain().ok_or(WebAuthnOriginError)?.to_owned();
        if parsed.scheme() != "https"
            || !parsed.username().is_empty()
            || parsed.password().is_some()
            || parsed.path() != "/"
            || parsed.query().is_some()
            || parsed.fragment().is_some()
        {
            return Err(WebAuthnOriginError);
        }
        let canonical = parsed.origin().ascii_serialization();
        if canonical.len() > 2_048 {
            return Err(WebAuthnOriginError);
        }
        Ok(Self { canonical, rp_id })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.canonical
    }

    #[must_use]
    pub fn rp_id(&self) -> &str {
        &self.rp_id
    }
}

impl TryFrom<String> for WebAuthnOrigin {
    type Error = WebAuthnOriginError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<WebAuthnOrigin> for String {
    fn from(value: WebAuthnOrigin) -> Self {
        value.canonical
    }
}

impl fmt::Display for WebAuthnOrigin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.canonical.fmt(formatter)
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
#[error("WebAuthn origin must be a canonical pathless HTTPS domain origin")]
pub struct WebAuthnOriginError;

/// Opaque authenticator credential ID. It is never interpreted by NodeControll.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Vec<u8>", into = "Vec<u8>")]
pub struct WebAuthnCredentialId(Vec<u8>);

impl WebAuthnCredentialId {
    pub fn parse(value: Vec<u8>) -> Result<Self, WebAuthnCredentialIdError> {
        if !(WEBAUTHN_CREDENTIAL_ID_MIN_BYTES..=WEBAUTHN_CREDENTIAL_ID_MAX_BYTES)
            .contains(&value.len())
        {
            return Err(WebAuthnCredentialIdError);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<Vec<u8>> for WebAuthnCredentialId {
    type Error = WebAuthnCredentialIdError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<WebAuthnCredentialId> for Vec<u8> {
    fn from(value: WebAuthnCredentialId) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
#[error("WebAuthn credential ID length is outside the supported range")]
pub struct WebAuthnCredentialIdError;

/// Stable RP-supplied user handle. NodeControll uses the 16 raw UUID bytes, never a username.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "Vec<u8>", into = "Vec<u8>")]
pub struct WebAuthnUserHandle(Vec<u8>);

impl WebAuthnUserHandle {
    pub fn parse(value: Vec<u8>) -> Result<Self, WebAuthnUserHandleError> {
        if value.is_empty() || value.len() > WEBAUTHN_USER_HANDLE_MAX_BYTES {
            return Err(WebAuthnUserHandleError);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn for_user(user_id: EntityId) -> Self {
        Self(user_id.into_uuid().as_bytes().to_vec())
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<Vec<u8>> for WebAuthnUserHandle {
    type Error = WebAuthnUserHandleError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<WebAuthnUserHandle> for Vec<u8> {
    fn from(value: WebAuthnUserHandle) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
#[error("WebAuthn user handle length is outside the supported range")]
pub struct WebAuthnUserHandleError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WebAuthnAaguid([u8; 16]);

impl WebAuthnAaguid {
    #[must_use]
    pub const fn from_bytes(value: [u8; 16]) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebAuthnTransport {
    Usb,
    Nfc,
    Ble,
    Internal,
    Hybrid,
    Test,
}

impl WebAuthnTransport {
    pub fn parse(value: &str) -> Result<Self, WebAuthnTransportParseError> {
        match value {
            "usb" => Ok(Self::Usb),
            "nfc" => Ok(Self::Nfc),
            "ble" => Ok(Self::Ble),
            "internal" => Ok(Self::Internal),
            "hybrid" => Ok(Self::Hybrid),
            "test" => Ok(Self::Test),
            _ => Err(WebAuthnTransportParseError),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Usb => "usb",
            Self::Nfc => "nfc",
            Self::Ble => "ble",
            Self::Internal => "internal",
            Self::Hybrid => "hybrid",
            Self::Test => "test",
        }
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
#[error("unknown WebAuthn transport")]
pub struct WebAuthnTransportParseError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WebAuthnNickname(String);

impl WebAuthnNickname {
    pub fn parse(value: impl Into<String>) -> Result<Self, WebAuthnNicknameError> {
        let value = value.into();
        let trimmed = value.trim();
        let length = trimmed.chars().count();
        if length == 0 || length > WEBAUTHN_NICKNAME_MAX_CHARS {
            return Err(WebAuthnNicknameError);
        }
        if trimmed.chars().any(char::is_control) {
            return Err(WebAuthnNicknameError);
        }
        Ok(Self(trimmed.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for WebAuthnNickname {
    type Error = WebAuthnNicknameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<WebAuthnNickname> for String {
    fn from(value: WebAuthnNickname) -> Self {
        value.0
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
#[error("WebAuthn credential nickname must contain 1 to 80 non-control characters")]
pub struct WebAuthnNicknameError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebAuthnCredentialStatus {
    Active,
    Revoked,
    CloneSuspected,
}

impl WebAuthnCredentialStatus {
    pub fn parse(value: &str) -> Result<Self, WebAuthnCredentialStatusParseError> {
        match value {
            "active" => Ok(Self::Active),
            "revoked" => Ok(Self::Revoked),
            "clone_suspected" => Ok(Self::CloneSuspected),
            _ => Err(WebAuthnCredentialStatusParseError),
        }
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Revoked => "revoked",
            Self::CloneSuspected => "clone_suspected",
        }
    }
}

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
#[error("unknown WebAuthn credential status")]
pub struct WebAuthnCredentialStatusParseError;

/// Non-secret credential projection. The encrypted library-owned passkey material is carried by
/// the persistence aggregate rather than exposed through the domain object.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebAuthnCredential {
    pub id: EntityId,
    pub user_id: EntityId,
    pub credential_id: WebAuthnCredentialId,
    pub user_handle: WebAuthnUserHandle,
    pub aaguid: Option<WebAuthnAaguid>,
    pub transports: Vec<WebAuthnTransport>,
    pub user_verified: bool,
    pub backup_eligible: bool,
    pub backup_state: bool,
    pub sign_counter: u32,
    pub nickname: WebAuthnNickname,
    pub status: WebAuthnCredentialStatus,
    pub created_at_ms: i64,
    pub last_used_at_ms: Option<i64>,
    /// Most recent accepted non-monotonic counter observation for a backup-eligible credential.
    /// Synced passkeys are not disabled for this signal, but it remains visible to local audit/UI.
    pub backup_counter_anomaly_at_ms: Option<i64>,
    pub revoked_at_ms: Option<i64>,
    pub clone_suspected_at_ms: Option<i64>,
    pub revision: Revision,
}

#[cfg(test)]
mod tests {
    use super::{
        WebAuthnCredentialId, WebAuthnNickname, WebAuthnOrigin, WebAuthnUserHandle,
    };

    #[test]
    fn origin_is_https_pathless_and_derives_exact_host_rp_id() {
        let origin = WebAuthnOrigin::parse("https://Login.Example.com:8443");
        assert!(origin.is_ok());
        if let Ok(origin) = origin {
            assert_eq!(origin.as_str(), "https://login.example.com:8443");
            assert_eq!(origin.rp_id(), "login.example.com");
        }
        for rejected in [
            "http://login.example.com",
            "https://login.example.com/path",
            "https://user@login.example.com",
            "https://127.0.0.1",
            " https://login.example.com",
        ] {
            assert!(WebAuthnOrigin::parse(rejected).is_err(), "accepted {rejected}");
        }
    }

    #[test]
    fn opaque_identifiers_and_nicknames_are_bounded() {
        assert!(WebAuthnCredentialId::parse(vec![7; 15]).is_err());
        assert!(WebAuthnCredentialId::parse(vec![7; 16]).is_ok());
        assert!(WebAuthnCredentialId::parse(vec![7; 1_023]).is_ok());
        assert!(WebAuthnCredentialId::parse(vec![7; 1_024]).is_err());
        assert!(WebAuthnUserHandle::parse(vec![1; 64]).is_ok());
        assert!(WebAuthnUserHandle::parse(vec![1; 65]).is_err());
        assert!(WebAuthnNickname::parse(" Security key ").is_ok_and(|v| v.as_str() == "Security key"));
        assert!(WebAuthnNickname::parse("bad\nname").is_err());
    }
}
