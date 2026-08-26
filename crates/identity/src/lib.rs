use argon2::{
    Algorithm, Argon2, Params, PasswordHash as ParsedPasswordHash, PasswordHasher,
    PasswordVerifier, Version, password_hash::SaltString,
};
use std::{
    fs,
    io::Read,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use nodecontroll_domain::PasswordHash;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use thiserror::Error;
use zeroize::Zeroizing;

const SALT_BYTES: usize = 16;
const MIN_PASSWORD_CHARS: usize = 12;
const MAX_PASSWORD_BYTES: usize = 1024;
const MEMORY_COST_KIB: u32 = 19_456;
const TIME_COST: u32 = 2;
const PARALLELISM: u32 = 1;
const SETUP_TOKEN_BYTES: usize = 32;
const ENCODED_SETUP_TOKEN_BYTES: usize = SETUP_TOKEN_BYTES * 2;
const MAX_SETUP_TOKEN_FILE_BYTES: u64 = (ENCODED_SETUP_TOKEN_BYTES + 1) as u64;
const BEARER_TOKEN_BYTES: usize = 32;
const ENCODED_BEARER_SECRET_BYTES: usize = BEARER_TOKEN_BYTES * 2;
const SESSION_TOKEN_PREFIX: &str = "ncs1_";
const CSRF_TOKEN_PREFIX: &str = "ncc1_";
pub const MAX_PRESENTED_BEARER_TOKEN_BYTES: usize = 96;

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
        let encoded = read_private_setup_token(path.as_ref())?;
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
    bool::from(expected.as_slice().ct_eq(presented.as_slice()))
}

#[cfg(unix)]
fn ensure_private_permissions(metadata: &fs::Metadata) -> Result<(), SetupCapabilityError> {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    if metadata.permissions().mode() & 0o077 != 0 {
        return Err(SetupCapabilityError::InsecureTokenPermissions);
    }
    if metadata.uid() != rustix::process::geteuid().as_raw() {
        return Err(SetupCapabilityError::UnexpectedTokenOwner);
    }
    Ok(())
}

#[cfg(not(unix))]
fn ensure_private_permissions(_metadata: &fs::Metadata) -> Result<(), SetupCapabilityError> {
    Ok(())
}

#[cfg(unix)]
fn read_private_setup_token(path: &Path) -> Result<Zeroizing<String>, SetupCapabilityError> {
    use rustix::fs::{Mode, OFlags, open};

    let descriptor = match open(
        path,
        OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK,
        Mode::empty(),
    ) {
        Ok(descriptor) => descriptor,
        Err(rustix::io::Errno::LOOP) => {
            return Err(SetupCapabilityError::TokenNotRegularFile);
        }
        Err(error) => return Err(SetupCapabilityError::Io(error.into())),
    };
    let mut file = fs::File::from(descriptor);
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(SetupCapabilityError::TokenNotRegularFile);
    }
    ensure_private_permissions(&metadata)?;
    if !(ENCODED_SETUP_TOKEN_BYTES as u64..=MAX_SETUP_TOKEN_FILE_BYTES).contains(&metadata.len()) {
        return Err(SetupCapabilityError::InvalidTokenEncoding);
    }
    read_bounded_setup_token(&mut file)
}

#[cfg(windows)]
fn read_private_setup_token(path: &Path) -> Result<Zeroizing<String>, SetupCapabilityError> {
    use std::os::windows::fs::{FileTypeExt, OpenOptionsExt};

    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;

    let mut file = fs::OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)?;
    let metadata = file.metadata()?;
    let file_type = metadata.file_type();
    if file_type.is_symlink_file() || file_type.is_symlink_dir() || !metadata.is_file() {
        return Err(SetupCapabilityError::TokenNotRegularFile);
    }
    ensure_private_permissions(&metadata)?;
    if !(ENCODED_SETUP_TOKEN_BYTES as u64..=MAX_SETUP_TOKEN_FILE_BYTES).contains(&metadata.len()) {
        return Err(SetupCapabilityError::InvalidTokenEncoding);
    }
    read_bounded_setup_token(&mut file)
}

#[cfg(not(any(unix, windows)))]
fn read_private_setup_token(path: &Path) -> Result<Zeroizing<String>, SetupCapabilityError> {
    let mut file = fs::OpenOptions::new().read(true).open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(SetupCapabilityError::TokenNotRegularFile);
    }
    ensure_private_permissions(&metadata)?;
    if !(ENCODED_SETUP_TOKEN_BYTES as u64..=MAX_SETUP_TOKEN_FILE_BYTES).contains(&metadata.len()) {
        return Err(SetupCapabilityError::InvalidTokenEncoding);
    }
    read_bounded_setup_token(&mut file)
}

fn read_bounded_setup_token(
    file: &mut fs::File,
) -> Result<Zeroizing<String>, SetupCapabilityError> {
    let mut encoded = Zeroizing::new(String::with_capacity(MAX_SETUP_TOKEN_FILE_BYTES as usize));
    file.take(MAX_SETUP_TOKEN_FILE_BYTES + 1)
        .read_to_string(&mut encoded)?;
    if encoded.len() > MAX_SETUP_TOKEN_FILE_BYTES as usize {
        return Err(SetupCapabilityError::InvalidTokenEncoding);
    }
    Ok(encoded)
}

pub struct SessionToken(Zeroizing<String>);

impl SessionToken {
    pub fn generate() -> Result<Self, BearerTokenError> {
        generate_bearer_token(SESSION_TOKEN_PREFIX).map(Self)
    }

    pub fn parse_presented(value: &str) -> Result<Self, BearerTokenError> {
        parse_bearer_token(value, SESSION_TOKEN_PREFIX)
            .map(Self)
            .map_err(|error| match error {
                BearerTokenError::InvalidTokenFormat => BearerTokenError::InvalidSessionTokenFormat,
                other => other,
            })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    #[must_use]
    pub fn constant_time_eq_presented(&self, presented: &str) -> bool {
        Self::parse_presented(presented).is_ok()
            && constant_time_bounded_string_equal(self.as_str(), presented)
    }
}

pub struct CsrfToken(Zeroizing<String>);

impl CsrfToken {
    pub fn generate() -> Result<Self, BearerTokenError> {
        generate_bearer_token(CSRF_TOKEN_PREFIX).map(Self)
    }

    pub fn parse_presented(value: &str) -> Result<Self, BearerTokenError> {
        parse_bearer_token(value, CSRF_TOKEN_PREFIX)
            .map(Self)
            .map_err(|error| match error {
                BearerTokenError::InvalidTokenFormat => BearerTokenError::InvalidCsrfTokenFormat,
                other => other,
            })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    #[must_use]
    pub fn constant_time_eq_presented(&self, presented: &str) -> bool {
        Self::parse_presented(presented).is_ok()
            && constant_time_bounded_string_equal(self.as_str(), presented)
    }
}

pub struct SessionTokenPair {
    session: SessionToken,
    csrf: CsrfToken,
}

impl SessionTokenPair {
    pub fn generate() -> Result<Self, BearerTokenError> {
        Ok(Self {
            session: SessionToken::generate()?,
            csrf: CsrfToken::generate()?,
        })
    }

    #[must_use]
    pub fn session(&self) -> &SessionToken {
        &self.session
    }

    #[must_use]
    pub fn csrf(&self) -> &CsrfToken {
        &self.csrf
    }

    #[must_use]
    pub fn into_tokens(self) -> (SessionToken, CsrfToken) {
        (self.session, self.csrf)
    }
}

fn generate_bearer_token(prefix: &str) -> Result<Zeroizing<String>, BearerTokenError> {
    let mut secret = Zeroizing::new([0_u8; BEARER_TOKEN_BYTES]);
    getrandom::fill(secret.as_mut()).map_err(|_| BearerTokenError::RandomUnavailable)?;
    let mut encoded_secret = Zeroizing::new([0_u8; ENCODED_BEARER_SECRET_BYTES]);
    hex::encode_to_slice(secret.as_ref(), encoded_secret.as_mut())
        .map_err(|_| BearerTokenError::TokenEncodingFailed)?;
    let encoded_secret = std::str::from_utf8(encoded_secret.as_ref())
        .map_err(|_| BearerTokenError::TokenEncodingFailed)?;
    let mut encoded = Zeroizing::new(String::with_capacity(
        prefix.len() + ENCODED_BEARER_SECRET_BYTES,
    ));
    encoded.push_str(prefix);
    encoded.push_str(encoded_secret);
    Ok(encoded)
}

fn parse_bearer_token(
    value: &str,
    expected_prefix: &str,
) -> Result<Zeroizing<String>, BearerTokenError> {
    if value.len() > MAX_PRESENTED_BEARER_TOKEN_BYTES {
        return Err(BearerTokenError::PresentedTokenTooLong);
    }
    let expected_length = expected_prefix.len() + ENCODED_BEARER_SECRET_BYTES;
    if value.len() != expected_length || !value.starts_with(expected_prefix) {
        return Err(BearerTokenError::InvalidTokenFormat);
    }
    let encoded_secret = &value[expected_prefix.len()..];
    if !encoded_secret
        .bytes()
        .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(BearerTokenError::InvalidTokenFormat);
    }
    let mut decoded = Zeroizing::new([0_u8; BEARER_TOKEN_BYTES]);
    hex::decode_to_slice(encoded_secret, decoded.as_mut())
        .map_err(|_| BearerTokenError::InvalidTokenFormat)?;
    Ok(Zeroizing::new(value.to_owned()))
}

#[must_use]
pub fn constant_time_bounded_string_equal(expected: &str, presented: &str) -> bool {
    if expected.len() > MAX_PRESENTED_BEARER_TOKEN_BYTES
        || presented.len() > MAX_PRESENTED_BEARER_TOKEN_BYTES
    {
        return false;
    }
    let mut expected_padded = Zeroizing::new([0_u8; MAX_PRESENTED_BEARER_TOKEN_BYTES]);
    let mut presented_padded = Zeroizing::new([0_u8; MAX_PRESENTED_BEARER_TOKEN_BYTES]);
    expected_padded[..expected.len()].copy_from_slice(expected.as_bytes());
    presented_padded[..presented.len()].copy_from_slice(presented.as_bytes());
    let contents_equal = expected_padded[..].ct_eq(&presented_padded[..]);
    let lengths_equal = expected.len().ct_eq(&presented.len());
    bool::from(contents_equal & lengths_equal)
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BearerTokenError {
    #[error("operating-system randomness is unavailable")]
    RandomUnavailable,
    #[error("bearer token encoding failed")]
    TokenEncodingFailed,
    #[error("presented bearer token exceeds the supported size")]
    PresentedTokenTooLong,
    #[error("session token must use the ncs1_ prefix and 64 lowercase hexadecimal characters")]
    InvalidSessionTokenFormat,
    #[error("CSRF token must use the ncc1_ prefix and 64 lowercase hexadecimal characters")]
    InvalidCsrfTokenFormat,
    #[error("bearer token has an invalid format")]
    InvalidTokenFormat,
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
    #[error("setup token file is not owned by the effective process user")]
    UnexpectedTokenOwner,
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

    use super::{
        BearerTokenError, CsrfToken, MAX_PRESENTED_BEARER_TOKEN_BYTES, PasswordError,
        PasswordService, SessionToken, SessionTokenPair, SetupCapability, SetupCapabilityError,
        constant_time_bounded_string_equal,
    };

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
    fn session_and_csrf_tokens_have_strict_versioned_formats_and_fresh_entropy() {
        let first = SessionTokenPair::generate();
        let second = SessionTokenPair::generate();
        assert!(first.is_ok());
        assert!(second.is_ok());
        if let (Ok(first), Ok(second)) = (first, second) {
            assert!(first.session().as_str().starts_with("ncs1_"));
            assert!(first.csrf().as_str().starts_with("ncc1_"));
            assert_eq!(first.session().as_str().len(), 69);
            assert_eq!(first.csrf().as_str().len(), 69);
            assert!(
                first.session().as_str()[5..]
                    .bytes()
                    .all(|byte| { byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase() })
            );
            assert!(
                first.csrf().as_str()[5..]
                    .bytes()
                    .all(|byte| { byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase() })
            );
            assert_ne!(&first.session().as_str()[5..], &first.csrf().as_str()[5..]);
            assert_ne!(first.session().as_str(), second.session().as_str());
            assert_ne!(first.csrf().as_str(), second.csrf().as_str());
        }
    }

    #[test]
    fn presented_bearer_tokens_are_strict_and_resource_bounded() {
        let session = SessionToken::generate();
        let csrf = CsrfToken::generate();
        assert!(session.is_ok());
        assert!(csrf.is_ok());
        if let (Ok(session), Ok(csrf)) = (session, csrf) {
            assert!(SessionToken::parse_presented(session.as_str()).is_ok());
            assert!(CsrfToken::parse_presented(csrf.as_str()).is_ok());
            assert!(matches!(
                SessionToken::parse_presented(csrf.as_str()),
                Err(BearerTokenError::InvalidSessionTokenFormat)
            ));
            assert!(matches!(
                SessionToken::parse_presented(&format!("ncs2_{}", "0".repeat(64))),
                Err(BearerTokenError::InvalidSessionTokenFormat)
            ));
            assert!(matches!(
                SessionToken::parse_presented(&format!("ncs1_{}", "A".repeat(64))),
                Err(BearerTokenError::InvalidSessionTokenFormat)
            ));
            assert!(matches!(
                SessionToken::parse_presented(&format!("ncs1_{}", "g".repeat(64))),
                Err(BearerTokenError::InvalidSessionTokenFormat)
            ));
            assert!(matches!(
                SessionToken::parse_presented(&"x".repeat(MAX_PRESENTED_BEARER_TOKEN_BYTES + 1)),
                Err(BearerTokenError::PresentedTokenTooLong)
            ));
        }
    }

    #[test]
    fn bearer_comparison_is_exact_and_accepts_only_bounded_inputs() {
        let expected = format!("ncs1_{}", "0".repeat(64));
        let token = SessionToken::parse_presented(&expected);
        assert!(token.is_ok());
        if let Ok(token) = token {
            assert!(token.constant_time_eq_presented(token.as_str()));
            let changed = format!("ncs1_{}1", "0".repeat(63));
            assert!(!token.constant_time_eq_presented(&changed));
            assert!(!token.constant_time_eq_presented("ncs1_short"));
        }
        assert!(constant_time_bounded_string_equal("same", "same"));
        assert!(!constant_time_bounded_string_equal("same", "different"));
        assert!(!constant_time_bounded_string_equal(
            &"x".repeat(MAX_PRESENTED_BEARER_TOKEN_BYTES + 1),
            &"x".repeat(MAX_PRESENTED_BEARER_TOKEN_BYTES + 1)
        ));
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

    #[cfg(unix)]
    #[test]
    fn setup_capability_rejects_non_regular_file() {
        let path = temporary_token_path("setup-token-directory");
        assert!(fs::create_dir(&path).is_ok());
        assert!(matches!(
            SetupCapability::from_file(&path, Duration::from_secs(60)),
            Err(SetupCapabilityError::TokenNotRegularFile)
        ));
        let _ = fs::remove_dir(path);
    }
}
