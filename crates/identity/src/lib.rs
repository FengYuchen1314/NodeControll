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
const LOGIN_PASSWORD_PADDING_MIN_MARGIN: Duration = Duration::from_millis(25);
const LOGIN_PASSWORD_PADDING_MAX_MARGIN: Duration = Duration::from_secs(1);
const LOGIN_CALIBRATION_PASSWORD: &str =
    "nodecontroll-login-resource-bound-rejection-calibration-password";
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

/// The password work performed for every password-login attempt, independent of whether the
/// selected credential belongs to an account or uses the dummy credential.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoginPasswordWorkStep {
    CalibrateCurrentPolicy,
    VerifySelectedCredentialAndUpgrade,
    PadToCalibratedDeadline,
}

/// The timing target shared by unknown accounts, current hashes, and accepted legacy hashes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoginPasswordTimingBucket {
    CalibratedTwoCurrentPolicyCosts,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoginPasswordWorkPlan {
    steps: [LoginPasswordWorkStep; 3],
    timing_bucket: LoginPasswordTimingBucket,
}

#[derive(Clone, Copy)]
struct LoginVerificationInput<'a> {
    password: &'a str,
    resource_bounded: bool,
}

impl LoginPasswordWorkPlan {
    #[must_use]
    pub const fn steps(self) -> [LoginPasswordWorkStep; 3] {
        self.steps
    }

    #[must_use]
    pub const fn timing_bucket(self) -> LoginPasswordTimingBucket {
        self.timing_bucket
    }
}

pub struct PasswordVerification {
    verified: bool,
    upgraded_hash: Option<PasswordHash>,
}

impl PasswordVerification {
    #[must_use]
    pub const fn verified(&self) -> bool {
        self.verified
    }

    #[must_use]
    pub fn into_upgraded_hash(self) -> Option<PasswordHash> {
        self.upgraded_hash
    }
}

impl PasswordService {
    pub fn recommended() -> Result<Self, PasswordError> {
        let params = Params::new(MEMORY_COST_KIB, TIME_COST, PARALLELISM, Some(32))
            .map_err(|_| PasswordError::InvalidParameters)?;
        Ok(Self { params })
    }

    pub fn hash(&self, password: &str) -> Result<PasswordHash, PasswordError> {
        validate_password(password)?;
        self.hash_resource_bounded(password)
    }

    fn hash_resource_bounded(&self, password: &str) -> Result<PasswordHash, PasswordError> {
        validate_password_resource_bound(password)?;
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

    /// Returns the fixed login work plan. Callers select either the account credential or the
    /// current-policy dummy credential before executing this plan; that selection never changes
    /// the steps or timing bucket.
    #[must_use]
    pub const fn login_work_plan() -> LoginPasswordWorkPlan {
        LoginPasswordWorkPlan {
            steps: [
                LoginPasswordWorkStep::CalibrateCurrentPolicy,
                LoginPasswordWorkStep::VerifySelectedCredentialAndUpgrade,
                LoginPasswordWorkStep::PadToCalibratedDeadline,
            ],
            timing_bucket: LoginPasswordTimingBucket::CalibratedTwoCurrentPolicyCosts,
        }
    }

    /// Executes the fixed password-login work plan while retaining successful legacy-hash
    /// upgrades when the caller has already established that the account may authenticate.
    /// `current_dummy` must use the active policy; the application constructor enforces that
    /// invariant before accepting traffic.
    ///
    /// An overlong presented value is replaced only for the bounded Argon2 work and can never be
    /// accepted or upgraded. Both Argon2 verifications and deadline padding still occur, so this
    /// rejection does not create a cheap path. Operational errors are returned only after the
    /// remaining fixed work and padding have completed.
    pub fn execute_login_work_plan(
        &self,
        plan: LoginPasswordWorkPlan,
        password: &str,
        selected_credential: &PasswordHash,
        current_dummy: &PasswordHash,
        upgrade_verified_credential: bool,
    ) -> Result<PasswordVerification, PasswordError> {
        let started_at = Instant::now();
        let verification_input = login_verification_input(password);
        execute_login_password_orchestration(
            plan,
            || {
                let calibration_started_at = Instant::now();
                let result = self.verify(LOGIN_CALIBRATION_PASSWORD, current_dummy);
                (result, calibration_started_at.elapsed())
            },
            || {
                if !verification_input.resource_bounded {
                    self.verify(verification_input.password, selected_credential)
                        .map(|_| PasswordVerification {
                            verified: false,
                            upgraded_hash: None,
                        })
                } else if upgrade_verified_credential {
                    self.verify_with_upgrade(verification_input.password, selected_credential)
                } else {
                    self.verify(verification_input.password, selected_credential)
                        .map(|verified| PasswordVerification {
                            verified,
                            upgraded_hash: None,
                        })
                }
            },
            |target_elapsed| {
                std::thread::sleep(login_password_padding_remaining(
                    target_elapsed,
                    started_at.elapsed(),
                ));
            },
        )
    }

    /// Verifies a password and, only after success, prepares a fresh PHC string when the stored
    /// Argon2id parameters differ from the active policy.
    ///
    /// Existing passwords are not re-evaluated against enrollment composition rules during a
    /// transparent upgrade. This avoids locking out a valid legacy account; a separate forced
    /// password-change policy can still require a new password at the application boundary.
    pub fn verify_with_upgrade(
        &self,
        password: &str,
        expected: &PasswordHash,
    ) -> Result<PasswordVerification, PasswordError> {
        let verified = self.verify(password, expected)?;
        if !verified {
            return Ok(PasswordVerification {
                verified: false,
                upgraded_hash: None,
            });
        }
        let upgraded_hash = self
            .needs_rehash(expected)?
            .then(|| self.hash_resource_bounded(password))
            .transpose()?;
        Ok(PasswordVerification {
            verified: true,
            upgraded_hash,
        })
    }

    /// Reports whether a successfully verified password should be re-hashed with the active
    /// Argon2id policy.
    ///
    /// This function never verifies a password and must therefore only be used after
    /// [`Self::verify`] returned `true`. The salt bytes are random and are not compared by value;
    /// only their decoded length is part of the active policy, so a short legacy salt is upgraded
    /// after successful verification when persistence commits the compare-and-swap replacement.
    pub fn needs_rehash(&self, expected: &PasswordHash) -> Result<bool, PasswordError> {
        let parsed = ParsedPasswordHash::new(expected.as_str())
            .map_err(|_| PasswordError::StoredHashInvalid)?;
        let memory_cost = parsed
            .params
            .get_decimal("m")
            .ok_or(PasswordError::StoredHashInvalid)?;
        let time_cost = parsed
            .params
            .get_decimal("t")
            .ok_or(PasswordError::StoredHashInvalid)?;
        let parallelism = parsed
            .params
            .get_decimal("p")
            .ok_or(PasswordError::StoredHashInvalid)?;
        let salt = parsed.salt.ok_or(PasswordError::StoredHashInvalid)?;
        let mut decoded_salt = Zeroizing::new([0_u8; 64]);
        let salt_length = salt
            .decode_b64(decoded_salt.as_mut())
            .map_err(|_| PasswordError::StoredHashInvalid)?
            .len();
        let output_length = parsed.hash.ok_or(PasswordError::StoredHashInvalid)?.len();

        Ok(parsed.algorithm.as_str() != "argon2id"
            || parsed.version != Some(19)
            || memory_cost != self.params.m_cost()
            || time_cost != self.params.t_cost()
            || parallelism != self.params.p_cost()
            || salt_length != SALT_BYTES
            || output_length != self.params.output_len().unwrap_or(32))
    }
}

fn login_verification_input(password: &str) -> LoginVerificationInput<'_> {
    if password.len() <= MAX_PASSWORD_BYTES {
        LoginVerificationInput {
            password,
            resource_bounded: true,
        }
    } else {
        LoginVerificationInput {
            password: LOGIN_CALIBRATION_PASSWORD,
            resource_bounded: false,
        }
    }
}

fn execute_login_password_orchestration<Calibrate, VerifySelected, Pad>(
    plan: LoginPasswordWorkPlan,
    mut calibrate: Calibrate,
    mut verify_selected: VerifySelected,
    mut pad: Pad,
) -> Result<PasswordVerification, PasswordError>
where
    Calibrate: FnMut() -> (Result<bool, PasswordError>, Duration),
    VerifySelected: FnMut() -> Result<PasswordVerification, PasswordError>,
    Pad: FnMut(Duration),
{
    let mut calibration_result = None;
    let mut selected_result = None;
    let mut target_elapsed = None;

    for step in plan.steps() {
        match step {
            LoginPasswordWorkStep::CalibrateCurrentPolicy => {
                let (result, calibration_elapsed) = calibrate();
                calibration_result = Some(result);
                target_elapsed = Some(login_password_padding_target(calibration_elapsed));
            }
            LoginPasswordWorkStep::VerifySelectedCredentialAndUpgrade => {
                selected_result = Some(verify_selected());
            }
            LoginPasswordWorkStep::PadToCalibratedDeadline => {
                pad(target_elapsed.unwrap_or(Duration::ZERO));
            }
        }
    }

    let calibration_result = calibration_result.ok_or(PasswordError::VerificationFailed)?;
    calibration_result?;
    selected_result.ok_or(PasswordError::VerificationFailed)?
}

fn login_password_padding_target(calibration_elapsed: Duration) -> Duration {
    let safety_margin = (calibration_elapsed / 2)
        .max(LOGIN_PASSWORD_PADDING_MIN_MARGIN)
        .min(LOGIN_PASSWORD_PADDING_MAX_MARGIN);
    calibration_elapsed
        .saturating_mul(2)
        .saturating_add(safety_margin)
}

fn login_password_padding_remaining(target_elapsed: Duration, elapsed: Duration) -> Duration {
    target_elapsed.saturating_sub(elapsed)
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
    use argon2::{Algorithm, Argon2, PasswordHasher, Version, password_hash::SaltString};
    use nodecontroll_domain::PasswordHash;
    use std::{
        cell::RefCell,
        fs,
        time::{Duration, SystemTime},
    };

    #[cfg(unix)]
    use std::os::unix::fs::{PermissionsExt, symlink};

    use super::{
        BearerTokenError, CsrfToken, LoginPasswordTimingBucket, LoginPasswordWorkStep,
        MAX_PRESENTED_BEARER_TOKEN_BYTES, Params, PasswordError, PasswordService,
        PasswordVerification, SessionToken, SessionTokenPair, SetupCapability,
        SetupCapabilityError, constant_time_bounded_string_equal,
        execute_login_password_orchestration, login_password_padding_remaining,
        login_password_padding_target, login_verification_input,
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
    fn login_work_plan_and_dynamic_padding_are_deterministic() {
        let unknown_account = PasswordService::login_work_plan();
        let current_hash = PasswordService::login_work_plan();
        let cheap_legacy_hash = PasswordService::login_work_plan();
        let expected_steps = [
            LoginPasswordWorkStep::CalibrateCurrentPolicy,
            LoginPasswordWorkStep::VerifySelectedCredentialAndUpgrade,
            LoginPasswordWorkStep::PadToCalibratedDeadline,
        ];

        assert_eq!(unknown_account, current_hash);
        assert_eq!(current_hash, cheap_legacy_hash);
        assert_eq!(unknown_account.steps(), expected_steps);
        assert_eq!(
            unknown_account.timing_bucket(),
            LoginPasswordTimingBucket::CalibratedTwoCurrentPolicyCosts
        );
        assert_eq!(
            login_password_padding_target(Duration::ZERO),
            Duration::from_millis(25)
        );
        assert_eq!(
            login_password_padding_target(Duration::from_millis(10)),
            Duration::from_millis(45)
        );
        assert_eq!(
            login_password_padding_target(Duration::from_millis(100)),
            Duration::from_millis(250)
        );
        assert_eq!(
            login_password_padding_target(Duration::from_secs(20)),
            Duration::from_secs(41)
        );
        assert_eq!(
            login_password_padding_remaining(
                Duration::from_millis(250),
                Duration::from_millis(175)
            ),
            Duration::from_millis(75)
        );
        assert_eq!(
            login_password_padding_remaining(
                Duration::from_millis(250),
                Duration::from_millis(300)
            ),
            Duration::ZERO
        );
    }

    #[test]
    fn login_orchestration_delays_calibration_and_selected_errors_until_after_padding() {
        #[derive(Debug, PartialEq, Eq)]
        enum Trace {
            Calibrate,
            VerifySelected,
            Pad(Duration),
        }

        let calibration_error_trace = RefCell::new(Vec::new());
        let calibration_error = execute_login_password_orchestration(
            PasswordService::login_work_plan(),
            || {
                calibration_error_trace.borrow_mut().push(Trace::Calibrate);
                (
                    Err(PasswordError::StoredHashInvalid),
                    Duration::from_millis(100),
                )
            },
            || {
                calibration_error_trace
                    .borrow_mut()
                    .push(Trace::VerifySelected);
                Ok(PasswordVerification {
                    verified: false,
                    upgraded_hash: None,
                })
            },
            |target| {
                calibration_error_trace
                    .borrow_mut()
                    .push(Trace::Pad(target));
            },
        );
        assert!(matches!(
            calibration_error,
            Err(PasswordError::StoredHashInvalid)
        ));
        assert_eq!(
            calibration_error_trace.into_inner(),
            [
                Trace::Calibrate,
                Trace::VerifySelected,
                Trace::Pad(Duration::from_millis(250)),
            ]
        );

        let selected_error_trace = RefCell::new(Vec::new());
        let selected_error = execute_login_password_orchestration(
            PasswordService::login_work_plan(),
            || {
                selected_error_trace.borrow_mut().push(Trace::Calibrate);
                (Ok(false), Duration::from_millis(10))
            },
            || {
                selected_error_trace
                    .borrow_mut()
                    .push(Trace::VerifySelected);
                Err(PasswordError::VerificationFailed)
            },
            |target| {
                selected_error_trace.borrow_mut().push(Trace::Pad(target));
            },
        );
        assert!(matches!(
            selected_error,
            Err(PasswordError::VerificationFailed)
        ));
        assert_eq!(
            selected_error_trace.into_inner(),
            [
                Trace::Calibrate,
                Trace::VerifySelected,
                Trace::Pad(Duration::from_millis(45)),
            ]
        );
    }

    #[test]
    fn overlong_login_input_uses_bounded_work_but_can_never_be_accepted() {
        let ordinary = login_verification_input("ordinary-password");
        assert!(ordinary.resource_bounded);
        assert_eq!(ordinary.password, "ordinary-password");

        let overlong_password = "x".repeat(1_025);
        let overlong = login_verification_input(&overlong_password);
        assert!(!overlong.resource_bounded);
        assert_ne!(overlong.password, overlong_password);
        assert!(overlong.password.len() <= 1_024);
    }

    #[test]
    fn password_hash_upgrade_decision_compares_the_complete_active_policy() {
        let recommended = PasswordService::recommended();
        let legacy_params = Params::new(8_192, 1, 1, Some(32));
        assert!(recommended.is_ok());
        assert!(legacy_params.is_ok());
        if let (Ok(recommended), Ok(legacy_params)) = (recommended, legacy_params) {
            let current_hash = recommended.hash("a long correct horse password");
            let legacy = PasswordService {
                params: legacy_params,
            };
            let legacy_hash = legacy.hash("a long correct horse password");
            assert!(matches!(
                current_hash,
                Ok(ref hash) if recommended.needs_rehash(hash) == Ok(false)
            ));
            assert!(matches!(
                legacy_hash,
                Ok(ref hash) if recommended.needs_rehash(hash) == Ok(true)
            ));
            if let Ok(legacy_hash) = legacy_hash {
                let upgraded =
                    recommended.verify_with_upgrade("a long correct horse password", &legacy_hash);
                assert!(matches!(
                    upgraded,
                    Ok(ref verification) if verification.verified()
                ));
                if let Ok(upgraded) = upgraded {
                    assert!(matches!(
                        upgraded.into_upgraded_hash(),
                        Some(ref hash)
                            if recommended.needs_rehash(hash) == Ok(false)
                                && recommended.verify(
                                    "a long correct horse password",
                                    hash,
                                ) == Ok(true)
                    ));
                }

                let mismatch = recommended.verify_with_upgrade("wrong password", &legacy_hash);
                assert!(mismatch.is_ok());
                if let Ok(mismatch) = mismatch {
                    assert!(!mismatch.verified());
                    assert!(mismatch.into_upgraded_hash().is_none());
                }
            }
        }
    }

    #[test]
    fn bounded_legacy_algorithm_version_output_and_salt_are_verified_then_upgraded() {
        let recommended = PasswordService::recommended();
        let legacy_params = Params::new(8_192, 1, 1, Some(16));
        let legacy_salt = SaltString::encode_b64(&[0x42_u8; 8]);
        assert!(recommended.is_ok());
        assert!(legacy_params.is_ok());
        assert!(legacy_salt.is_ok());
        if let (Ok(recommended), Ok(legacy_params), Ok(legacy_salt)) =
            (recommended, legacy_params, legacy_salt)
        {
            let legacy_engine = Argon2::new(Algorithm::Argon2i, Version::V0x10, legacy_params);
            let legacy_phc =
                legacy_engine.hash_password(b"a long correct horse password", &legacy_salt);
            assert!(legacy_phc.is_ok());
            if let Ok(legacy_phc) = legacy_phc {
                let stored = PasswordHash::parse(legacy_phc.to_string());
                assert!(stored.is_ok());
                if let Ok(stored) = stored {
                    let verified =
                        recommended.verify_with_upgrade("a long correct horse password", &stored);
                    assert!(matches!(
                        verified,
                        Ok(ref result) if result.verified()
                    ));
                    if let Ok(verified) = verified {
                        assert!(verified.into_upgraded_hash().is_some());
                    }
                }
            }

            let short_salt_engine = Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                recommended.params.clone(),
            );
            let short_salt_phc =
                short_salt_engine.hash_password(b"a long correct horse password", &legacy_salt);
            assert!(short_salt_phc.is_ok());
            if let Ok(short_salt_phc) = short_salt_phc {
                let stored = PasswordHash::parse(short_salt_phc.to_string());
                assert!(matches!(
                    stored,
                    Ok(ref hash) if recommended.needs_rehash(hash) == Ok(true)
                ));
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
