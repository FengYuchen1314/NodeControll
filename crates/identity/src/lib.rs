use argon2::{
    Algorithm, Argon2, Params, PasswordHash as ParsedPasswordHash, PasswordHasher,
    PasswordVerifier, Version, password_hash::SaltString,
};
use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use nodecontroll_domain::PasswordHash;
use sha2::{Digest, Sha256};
use thiserror::Error;
use zeroize::Zeroizing;

const SALT_BYTES: usize = 16;
const MIN_PASSWORD_CHARS: usize = 12;
const MAX_PASSWORD_BYTES: usize = 1024;
const MEMORY_COST_KIB: u32 = 19_456;
const TIME_COST: u32 = 2;
const PARALLELISM: u32 = 1;
const SETUP_TOKEN_BYTES: usize = 32;

pub struct SetupCapability {
    expected_digest: Zeroizing<[u8; 32]>,
    expires_at: Instant,
    consumed: AtomicBool,
}

impl SetupCapability {
    pub fn from_file(path: impl AsRef<Path>, ttl: Duration) -> Result<Self, SetupCapabilityError> {
        if ttl.is_zero() {
            return Err(SetupCapabilityError::InvalidTtl);
        }
        let metadata = fs::symlink_metadata(path.as_ref())?;
        if !metadata.is_file() {
            return Err(SetupCapabilityError::TokenNotRegularFile);
        }
        if !(64..=65).contains(&metadata.len()) {
            return Err(SetupCapabilityError::InvalidTokenEncoding);
        }
        ensure_private_permissions(&metadata)?;
        let encoded = Zeroizing::new(fs::read_to_string(path)?);
        let encoded = encoded.strip_suffix('\n').unwrap_or(encoded.as_str());
        let token = decode_setup_token(encoded)?;
        let digest: [u8; 32] = Sha256::digest(token.as_ref()).into();
        let expires_at = Instant::now()
            .checked_add(ttl)
            .ok_or(SetupCapabilityError::InvalidTtl)?;
        Ok(Self {
            expected_digest: Zeroizing::new(digest),
            expires_at,
            consumed: AtomicBool::new(false),
        })
    }

    #[must_use]
    pub fn authorize(&self, presented: &str) -> bool {
        if self.consumed.load(Ordering::Acquire) || Instant::now() >= self.expires_at {
            return false;
        }
        let Ok(token) = decode_setup_token(presented) else {
            return false;
        };
        let presented_digest: [u8; 32] = Sha256::digest(token.as_ref()).into();
        constant_time_equal(&self.expected_digest, &presented_digest)
    }

    pub fn consume(&self) {
        self.consumed.store(true, Ordering::Release);
    }
}

fn decode_setup_token(
    value: &str,
) -> Result<Zeroizing<[u8; SETUP_TOKEN_BYTES]>, SetupCapabilityError> {
    if value.len() != SETUP_TOKEN_BYTES * 2
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(SetupCapabilityError::InvalidTokenEncoding);
    }
    let mut decoded = Zeroizing::new([0_u8; SETUP_TOKEN_BYTES]);
    hex::decode_to_slice(value, decoded.as_mut())
        .map_err(|_| SetupCapabilityError::InvalidTokenEncoding)?;
    Ok(decoded)
}

fn constant_time_equal(expected: &[u8; 32], presented: &[u8; 32]) -> bool {
    expected
        .iter()
        .zip(presented)
        .fold(0_u8, |difference, (left, right)| {
            difference | (left ^ right)
        })
        == 0
}

#[cfg(unix)]
fn ensure_private_permissions(metadata: &fs::Metadata) -> Result<(), SetupCapabilityError> {
    use std::os::unix::fs::PermissionsExt;

    if metadata.permissions().mode() & 0o077 == 0 {
        Ok(())
    } else {
        Err(SetupCapabilityError::InsecureTokenPermissions)
    }
}

#[cfg(not(unix))]
fn ensure_private_permissions(_metadata: &fs::Metadata) -> Result<(), SetupCapabilityError> {
    Ok(())
}

#[derive(Clone)]
pub struct PasswordService {
    params: Params,
}

impl PasswordService {
    pub fn recommended() -> Result<Self, PasswordError> {
        let params = Params::new(MEMORY_COST_KIB, TIME_COST, PARALLELISM, Some(32))
            .map_err(|_| PasswordError::InvalidParameters)?;
        Ok(Self { params })
    }

    pub fn hash(&self, password: &str) -> Result<PasswordHash, PasswordError> {
        validate_password(password)?;
        let mut salt_bytes = Zeroizing::new([0_u8; SALT_BYTES]);
        getrandom::fill(salt_bytes.as_mut()).map_err(|_| PasswordError::RandomUnavailable)?;
        let salt = SaltString::encode_b64(salt_bytes.as_ref())
            .map_err(|_| PasswordError::SaltEncodingFailed)?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, self.params.clone());
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| PasswordError::HashFailed)?
            .to_string();
        PasswordHash::parse(hash).map_err(|_| PasswordError::HashFailed)
    }

    pub fn validate(&self, password: &str) -> Result<(), PasswordError> {
        validate_password(password)
    }

    pub fn verify(&self, password: &str, expected: &PasswordHash) -> Result<bool, PasswordError> {
        validate_password_resource_bound(password)?;
        let parsed = ParsedPasswordHash::new(expected.as_str())
            .map_err(|_| PasswordError::StoredHashInvalid)?;
        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, self.params.clone());
        match argon2.verify_password(password.as_bytes(), &parsed) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(_) => Err(PasswordError::VerificationFailed),
        }
    }
}

#[derive(Debug, Error)]
pub enum SetupCapabilityError {
    #[error("setup capability TTL must be greater than zero")]
    InvalidTtl,
    #[error("setup token must be a regular file")]
    TokenNotRegularFile,
    #[error("setup token file permissions allow group or other access")]
    InsecureTokenPermissions,
    #[error("setup token must contain exactly 64 lowercase hexadecimal characters")]
    InvalidTokenEncoding,
    #[error("setup token file operation failed: {0}")]
    Io(#[from] std::io::Error),
}

fn validate_password(password: &str) -> Result<(), PasswordError> {
    validate_password_resource_bound(password)?;
    if password.chars().count() < MIN_PASSWORD_CHARS {
        return Err(PasswordError::TooShort);
    }
    if password.chars().any(char::is_control) {
        return Err(PasswordError::ControlCharacter);
    }
    Ok(())
}

fn validate_password_resource_bound(password: &str) -> Result<(), PasswordError> {
    if password.len() > MAX_PASSWORD_BYTES {
        return Err(PasswordError::TooLong);
    }
    Ok(())
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PasswordError {
    #[error("password must contain at least 12 Unicode scalar values")]
    TooShort,
    #[error("password cannot exceed 1024 UTF-8 bytes")]
    TooLong,
    #[error("password cannot contain control characters")]
    ControlCharacter,
    #[error("Argon2id parameters are invalid")]
    InvalidParameters,
    #[error("operating-system randomness is unavailable")]
    RandomUnavailable,
    #[error("password salt encoding failed")]
    SaltEncodingFailed,
    #[error("password hashing failed")]
    HashFailed,
    #[error("stored password hash is invalid")]
    StoredHashInvalid,
    #[error("password verification failed for an operational reason")]
    VerificationFailed,
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{Duration, SystemTime},
    };

    #[cfg(unix)]
    use std::os::unix::fs::{PermissionsExt, symlink};

    use super::{PasswordError, PasswordService, SetupCapability, SetupCapabilityError};

    const SETUP_TOKEN: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn temporary_token_path(label: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        std::env::temp_dir().join(format!(
            "nodecontroll-{label}-{}-{nonce}",
            std::process::id()
        ))
    }

    #[test]
    fn password_hash_round_trip_and_mismatch() {
        let service = PasswordService::recommended();
        assert!(service.is_ok());
        if let Ok(service) = service {
            let hash = service.hash("a long correct horse password");
            assert!(hash.is_ok());
            if let Ok(hash) = hash {
                assert!(matches!(
                    service.verify("a long correct horse password", &hash),
                    Ok(true)
                ));
                assert!(matches!(service.verify("wrong password", &hash), Ok(false)));
                assert!(!hash.as_str().contains("correct horse"));
            }
        }
    }

    #[test]
    fn password_policy_rejects_short_and_control_input() {
        let service = PasswordService::recommended();
        assert!(service.is_ok());
        if let Ok(service) = service {
            assert!(matches!(
                service.hash("too short"),
                Err(PasswordError::TooShort)
            ));
            assert!(matches!(
                service.hash("long enough\npassword"),
                Err(PasswordError::ControlCharacter)
            ));
            assert!(matches!(
                service.hash(&"x".repeat(1025)),
                Err(PasswordError::TooLong)
            ));
        }
    }

    #[test]
    fn setup_capability_is_exact_and_one_time() {
        let path = temporary_token_path("setup-token");
        assert!(fs::write(&path, format!("{SETUP_TOKEN}\n")).is_ok());
        #[cfg(unix)]
        assert!(fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).is_ok());
        let capability = SetupCapability::from_file(&path, Duration::from_secs(60));
        let _ = fs::remove_file(path);
        assert!(capability.is_ok());
        if let Ok(capability) = capability {
            assert!(capability.authorize(SETUP_TOKEN));
            assert!(
                !capability
                    .authorize("1123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef")
            );
            assert!(!capability.authorize("not-a-token"));
            capability.consume();
            assert!(!capability.authorize(SETUP_TOKEN));
        }
    }

    #[test]
    fn setup_capability_rejects_zero_ttl_and_invalid_encoding() {
        let path = temporary_token_path("bad-setup-token");
        assert!(fs::write(&path, "ABC").is_ok());
        #[cfg(unix)]
        assert!(fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).is_ok());
        assert!(matches!(
            SetupCapability::from_file(&path, Duration::ZERO),
            Err(SetupCapabilityError::InvalidTtl)
        ));
        assert!(matches!(
            SetupCapability::from_file(&path, Duration::from_secs(60)),
            Err(SetupCapabilityError::InvalidTokenEncoding)
        ));
        let _ = fs::remove_file(path);
    }

    #[cfg(unix)]
    #[test]
    fn setup_capability_rejects_group_readable_file() {
        let path = temporary_token_path("public-setup-token");
        assert!(fs::write(&path, SETUP_TOKEN).is_ok());
        assert!(fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).is_ok());
        assert!(matches!(
            SetupCapability::from_file(&path, Duration::from_secs(60)),
            Err(SetupCapabilityError::InsecureTokenPermissions)
        ));
        let _ = fs::remove_file(path);
    }

    #[cfg(unix)]
    #[test]
    fn setup_capability_rejects_symlink() {
        let target = temporary_token_path("setup-token-target");
        let link = temporary_token_path("setup-token-link");
        assert!(fs::write(&target, SETUP_TOKEN).is_ok());
        assert!(fs::set_permissions(&target, fs::Permissions::from_mode(0o600)).is_ok());
        assert!(symlink(&target, &link).is_ok());
        assert!(matches!(
            SetupCapability::from_file(&link, Duration::from_secs(60)),
            Err(SetupCapabilityError::TokenNotRegularFile)
        ));
        let _ = fs::remove_file(link);
        let _ = fs::remove_file(target);
    }
}
