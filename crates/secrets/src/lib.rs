use std::{collections::BTreeMap, fs, io::Read, path::Path, sync::Arc};

use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, Key, KeyInit, Payload},
};
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use thiserror::Error;
use zeroize::Zeroizing;

const KEY_BYTES: usize = 32;
const ENCODED_KEY_BYTES: usize = KEY_BYTES * 2;
const NONCE_BYTES: usize = 24;
const TYPED_AAD_FORMAT: &[u8] = b"NCSECRET2\0";
const ROOT_KEY_CANARY_PLAINTEXT: &[u8] = b"nodecontroll-root-key-canary-v1";
const MAX_KEYRING_KEYS: usize = 4;
const RECOVERY_CODE_BYTES: usize = 16;
pub const RECOVERY_CODE_COUNT: usize = 8;
pub const AUTH_CHALLENGE_TOKEN_BYTES: usize = 32;
pub const AUTH_CHALLENGE_TOKEN_HEX_LENGTH: usize = AUTH_CHALLENGE_TOKEN_BYTES * 2;
const MAX_KEY_FILE_BYTES: u64 = (ENCODED_KEY_BYTES + 1) as u64;
const KEYED_DIGEST_KDF_SALT: &[u8] = b"nodecontroll/keyed-digest/hkdf-sha256/v1";
const KEYED_DIGEST_KDF_INFO: &[u8] = b"nodecontroll/keyed-digest/purpose-key/v1\0";
const KEYED_DIGEST_MAC_FORMAT: &[u8] = b"nodecontroll/keyed-digest/hmac-sha256/v1\0";

pub const KEYED_DIGEST_BYTES: usize = 32;
pub const KEYED_DIGEST_ALGORITHM_VERSION: u8 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyedDigestPurpose {
    Session,
    Csrf,
    LoginAccount,
    LoginIp,
    LoginGlobal,
    RecoveryCode,
    AuthChallenge,
}

impl KeyedDigestPurpose {
    const fn context(self) -> &'static [u8] {
        match self {
            Self::Session => b"session",
            Self::Csrf => b"csrf",
            Self::LoginAccount => b"login-account",
            Self::LoginIp => b"login-ip",
            Self::LoginGlobal => b"login-global",
            Self::RecoveryCode => b"recovery-code",
            Self::AuthChallenge => b"auth-challenge",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecretPurpose {
    RootKeyCanary,
    TotpSeed,
}

impl SecretPurpose {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RootKeyCanary => "root_key_canary",
            Self::TotpSeed => "totp_seed",
        }
    }

    pub fn parse(value: &str) -> Result<Self, SecretError> {
        match value {
            "root_key_canary" => Ok(Self::RootKeyCanary),
            "totp_seed" => Ok(Self::TotpSeed),
            _ => Err(SecretError::InvalidSecretPurpose),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecretOwnerKind {
    System,
    Instance,
    User,
}

impl SecretOwnerKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Instance => "instance",
            Self::User => "user",
        }
    }

    pub fn parse(value: &str) -> Result<Self, SecretError> {
        match value {
            "system" => Ok(Self::System),
            "instance" => Ok(Self::Instance),
            "user" => Ok(Self::User),
            _ => Err(SecretError::InvalidSecretOwnerKind),
        }
    }
}

/// A typed, versioned business binding for an encrypted secret record.
///
/// Every field is authenticated as AEAD associated data together with the root-key version. A
/// ciphertext therefore cannot be replayed under another owner, purpose, schema, or key version.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SecretBinding {
    pub purpose: SecretPurpose,
    pub owner_kind: SecretOwnerKind,
    pub owner_id: uuid::Uuid,
    pub schema_version: u32,
}

impl SecretBinding {
    pub fn new(
        purpose: SecretPurpose,
        owner_kind: SecretOwnerKind,
        owner_id: uuid::Uuid,
        schema_version: u32,
    ) -> Result<Self, SecretError> {
        if schema_version == 0 {
            return Err(SecretError::InvalidSecretSchemaVersion);
        }
        if owner_kind == SecretOwnerKind::System && !owner_id.is_nil() {
            return Err(SecretError::InvalidSecretOwner);
        }
        if owner_kind != SecretOwnerKind::System && owner_id.is_nil() {
            return Err(SecretError::InvalidSecretOwner);
        }
        if purpose == SecretPurpose::RootKeyCanary && owner_kind != SecretOwnerKind::System {
            return Err(SecretError::InvalidSecretOwner);
        }
        if purpose == SecretPurpose::TotpSeed && owner_kind != SecretOwnerKind::User {
            return Err(SecretError::InvalidSecretOwner);
        }
        Ok(Self {
            purpose,
            owner_kind,
            owner_id,
            schema_version,
        })
    }

    #[must_use]
    pub const fn root_key_canary() -> Self {
        Self {
            purpose: SecretPurpose::RootKeyCanary,
            owner_kind: SecretOwnerKind::System,
            owner_id: uuid::Uuid::nil(),
            schema_version: 1,
        }
    }
}

/// A generated recovery code. Debug and Clone are deliberately not implemented.
pub struct RecoveryCode {
    presented: Zeroizing<String>,
    normalized: Zeroizing<[u8; RECOVERY_CODE_BYTES]>,
}

impl RecoveryCode {
    pub fn generate() -> Result<Self, SecretError> {
        let mut normalized = Zeroizing::new([0_u8; RECOVERY_CODE_BYTES]);
        getrandom::fill(normalized.as_mut()).map_err(|_| SecretError::RandomUnavailable)?;
        let encoded = Zeroizing::new(hex::encode(normalized.as_ref()));
        let presented = Zeroizing::new(
            encoded
                .as_bytes()
                .chunks(4)
                .map(|chunk| std::str::from_utf8(chunk).unwrap_or_default())
                .collect::<Vec<_>>()
                .join("-"),
        );
        Ok(Self {
            presented,
            normalized,
        })
    }

    /// Accepts either eight four-hex groups or the same 32 hex digits without separators.
    /// No whitespace or alternate separators are accepted; hexadecimal case is normalized.
    pub fn parse_presented(value: &str) -> Result<Self, SecretError> {
        let compact = Zeroizing::new(
            if value.len() == 32 && value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                value.to_ascii_lowercase()
            } else if value.len() == 39
                && value.split('-').count() == 8
                && value.split('-').all(|group| {
                    group.len() == 4 && group.bytes().all(|byte| byte.is_ascii_hexdigit())
                })
            {
                value.replace('-', "").to_ascii_lowercase()
            } else {
                return Err(SecretError::InvalidRecoveryCode);
            },
        );
        let mut normalized = Zeroizing::new([0_u8; RECOVERY_CODE_BYTES]);
        hex::decode_to_slice(compact.as_bytes(), normalized.as_mut())
            .map_err(|_| SecretError::InvalidRecoveryCode)?;
        let presented = Zeroizing::new(
            compact
                .as_bytes()
                .chunks(4)
                .map(|chunk| std::str::from_utf8(chunk).unwrap_or_default())
                .collect::<Vec<_>>()
                .join("-"),
        );
        Ok(Self {
            presented,
            normalized,
        })
    }

    #[must_use]
    pub fn presented(&self) -> &str {
        self.presented.as_str()
    }

    #[must_use]
    pub fn normalized_bytes(&self) -> &[u8; RECOVERY_CODE_BYTES] {
        &self.normalized
    }
}

pub fn generate_recovery_codes() -> Result<Vec<RecoveryCode>, SecretError> {
    (0..RECOVERY_CODE_COUNT)
        .map(|_| RecoveryCode::generate())
        .collect()
}

/// An opaque, canonical bearer used by persisted authentication challenges.
///
/// Debug and Clone are deliberately not implemented. The presented token and decoded bytes are
/// zeroized on drop; callers persist only its purpose-separated keyed digest.
pub struct AuthChallengeToken {
    presented: Zeroizing<String>,
    normalized: Zeroizing<[u8; AUTH_CHALLENGE_TOKEN_BYTES]>,
}

impl AuthChallengeToken {
    pub fn generate() -> Result<Self, SecretError> {
        let mut normalized = Zeroizing::new([0_u8; AUTH_CHALLENGE_TOKEN_BYTES]);
        getrandom::fill(normalized.as_mut()).map_err(|_| SecretError::RandomUnavailable)?;
        let presented = Zeroizing::new(hex::encode(normalized.as_ref()));
        Ok(Self {
            presented,
            normalized,
        })
    }

    /// Parses the one canonical wire form: exactly 64 lowercase hexadecimal characters.
    pub fn parse_presented(value: &str) -> Result<Self, SecretError> {
        if value.len() != AUTH_CHALLENGE_TOKEN_HEX_LENGTH
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(SecretError::InvalidAuthChallengeToken);
        }
        let mut normalized = Zeroizing::new([0_u8; AUTH_CHALLENGE_TOKEN_BYTES]);
        hex::decode_to_slice(value, normalized.as_mut())
            .map_err(|_| SecretError::InvalidAuthChallengeToken)?;
        Ok(Self {
            presented: Zeroizing::new(value.to_owned()),
            normalized,
        })
    }

    #[must_use]
    pub fn presented(&self) -> &str {
        self.presented.as_str()
    }

    fn normalized_bytes(&self) -> &[u8; AUTH_CHALLENGE_TOKEN_BYTES] {
        &self.normalized
    }
}

/// A newly generated challenge bearer paired with the only value that may be persisted.
/// Debug and Clone are deliberately not implemented because this value owns the bearer plaintext.
pub struct GeneratedAuthChallenge {
    pub token: AuthChallengeToken,
    pub digest: KeyedDigest,
}

#[derive(Clone, PartialEq, Eq)]
pub struct KeyedDigest {
    pub key_version: u32,
    pub digest: [u8; KEYED_DIGEST_BYTES],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretEnvelope {
    pub key_version: u32,
    pub nonce: [u8; NONCE_BYTES],
    pub ciphertext: Vec<u8>,
    pub aad_hash: [u8; 32],
}

#[derive(Clone)]
pub struct EnvelopeCipher {
    key: Arc<Zeroizing<[u8; KEY_BYTES]>>,
    key_version: u32,
}

impl EnvelopeCipher {
    pub fn from_key_file(path: impl AsRef<Path>, key_version: u32) -> Result<Self, SecretError> {
        if key_version == 0 {
            return Err(SecretError::InvalidKeyVersion);
        }
        let encoded = read_private_root_key(path.as_ref())?;
        let encoded = encoded.strip_suffix('\n').unwrap_or(encoded.as_str());
        Self::from_hex(encoded, key_version)
    }

    pub fn from_hex(encoded: &str, key_version: u32) -> Result<Self, SecretError> {
        if key_version == 0 {
            return Err(SecretError::InvalidKeyVersion);
        }
        if encoded.len() != KEY_BYTES * 2
            || !encoded
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(SecretError::InvalidKeyEncoding);
        }
        let mut key = Zeroizing::new([0_u8; KEY_BYTES]);
        hex::decode_to_slice(encoded, key.as_mut()).map_err(|_| SecretError::InvalidKeyEncoding)?;
        Ok(Self {
            key: Arc::new(key),
            key_version,
        })
    }

    #[must_use]
    pub const fn key_version(&self) -> u32 {
        self.key_version
    }

    pub fn encrypt_bound(
        &self,
        binding: &SecretBinding,
        plaintext: &[u8],
    ) -> Result<SecretEnvelope, SecretError> {
        let aad = typed_associated_data(binding, self.key_version)?;
        self.encrypt_with_aad(&aad, plaintext)
    }

    pub fn decrypt_bound(
        &self,
        binding: &SecretBinding,
        envelope: &SecretEnvelope,
    ) -> Result<Zeroizing<Vec<u8>>, SecretError> {
        if envelope.key_version != self.key_version {
            return Err(SecretError::UnknownKeyVersion);
        }
        let aad = typed_associated_data(binding, envelope.key_version)?;
        self.decrypt_with_aad(&aad, envelope)
    }

    fn encrypt_with_aad(
        &self,
        aad: &[u8],
        plaintext: &[u8],
    ) -> Result<SecretEnvelope, SecretError> {
        let aad_hash: [u8; 32] = Sha256::digest(aad).into();
        let mut nonce = [0_u8; NONCE_BYTES];
        getrandom::fill(&mut nonce).map_err(|_| SecretError::RandomUnavailable)?;
        let key: &Key<XChaCha20Poly1305> = (&**self.key).into();
        let cipher = XChaCha20Poly1305::new(key);
        let aead_nonce = XNonce::from(nonce);
        let ciphertext = cipher
            .encrypt(
                &aead_nonce,
                Payload {
                    msg: plaintext,
                    aad,
                },
            )
            .map_err(|_| SecretError::EncryptionFailed)?;
        Ok(SecretEnvelope {
            key_version: self.key_version,
            nonce,
            ciphertext,
            aad_hash,
        })
    }

    fn decrypt_with_aad(
        &self,
        aad: &[u8],
        envelope: &SecretEnvelope,
    ) -> Result<Zeroizing<Vec<u8>>, SecretError> {
        let aad_hash: [u8; 32] = Sha256::digest(aad).into();
        if !bool::from(aad_hash.ct_eq(&envelope.aad_hash)) {
            return Err(SecretError::AssociatedDataMismatch);
        }
        let key: &Key<XChaCha20Poly1305> = (&**self.key).into();
        let cipher = XChaCha20Poly1305::new(key);
        let aead_nonce = XNonce::from(envelope.nonce);
        let plaintext = cipher
            .decrypt(
                &aead_nonce,
                Payload {
                    msg: &envelope.ciphertext,
                    aad,
                },
            )
            .map_err(|_| SecretError::AuthenticationFailed)?;
        Ok(Zeroizing::new(plaintext))
    }

    pub fn keyed_digest(
        &self,
        purpose: KeyedDigestPurpose,
        value: &[u8],
    ) -> Result<KeyedDigest, SecretError> {
        let value_length =
            u64::try_from(value.len()).map_err(|_| SecretError::DigestInputTooLarge)?;
        let purpose_context = purpose.context();
        let purpose_length =
            u8::try_from(purpose_context.len()).map_err(|_| SecretError::KeyDerivationFailed)?;

        let mut kdf_info = Vec::with_capacity(
            KEYED_DIGEST_KDF_INFO.len() + purpose_context.len() + std::mem::size_of::<u32>() + 2,
        );
        kdf_info.extend_from_slice(KEYED_DIGEST_KDF_INFO);
        kdf_info.push(KEYED_DIGEST_ALGORITHM_VERSION);
        kdf_info.extend_from_slice(&self.key_version.to_be_bytes());
        kdf_info.push(purpose_length);
        kdf_info.extend_from_slice(purpose_context);

        let hkdf = Hkdf::<Sha256>::new(Some(KEYED_DIGEST_KDF_SALT), &**self.key);
        let mut purpose_key = Zeroizing::new([0_u8; KEYED_DIGEST_BYTES]);
        hkdf.expand(&kdf_info, purpose_key.as_mut())
            .map_err(|_| SecretError::KeyDerivationFailed)?;

        let mut mac = <Hmac<Sha256> as hmac::KeyInit>::new_from_slice(purpose_key.as_ref())
            .map_err(|_| SecretError::KeyDerivationFailed)?;
        mac.update(KEYED_DIGEST_MAC_FORMAT);
        mac.update(&[KEYED_DIGEST_ALGORITHM_VERSION]);
        mac.update(&self.key_version.to_be_bytes());
        mac.update(&[purpose_length]);
        mac.update(purpose_context);
        mac.update(&value_length.to_be_bytes());
        mac.update(value);
        let digest: [u8; KEYED_DIGEST_BYTES] = mac.finalize().into_bytes().into();
        Ok(KeyedDigest {
            key_version: self.key_version,
            digest,
        })
    }

    pub fn verify_keyed_digest(
        &self,
        purpose: KeyedDigestPurpose,
        value: &[u8],
        expected: &KeyedDigest,
    ) -> Result<bool, SecretError> {
        if expected.key_version != self.key_version {
            return Err(SecretError::UnknownKeyVersion);
        }
        let presented = self.keyed_digest(purpose, value)?;
        Ok(bool::from(
            expected
                .digest
                .as_slice()
                .ct_eq(presented.digest.as_slice()),
        ))
    }
}

#[derive(Clone)]
pub struct Keyring {
    current_version: u32,
    keys: Arc<BTreeMap<u32, EnvelopeCipher>>,
}

impl Keyring {
    pub fn from_key_files(
        current_path: impl AsRef<Path>,
        current_version: u32,
        previous: &[(u32, std::path::PathBuf)],
    ) -> Result<Self, SecretError> {
        let current = EnvelopeCipher::from_key_file(current_path, current_version)?;
        let previous = previous
            .iter()
            .map(|(version, path)| EnvelopeCipher::from_key_file(path, *version))
            .collect::<Result<Vec<_>, _>>()?;
        Self::from_ciphers(current, previous)
    }

    pub fn from_ciphers(
        current: EnvelopeCipher,
        previous: Vec<EnvelopeCipher>,
    ) -> Result<Self, SecretError> {
        if previous.len().saturating_add(1) > MAX_KEYRING_KEYS {
            return Err(SecretError::KeyringTooLarge);
        }
        let current_version = current.key_version();
        let mut keys = BTreeMap::new();
        keys.insert(current_version, current);
        for cipher in previous {
            if cipher.key_version() >= current_version
                || keys.insert(cipher.key_version(), cipher).is_some()
            {
                return Err(SecretError::InvalidKeyring);
            }
        }
        Ok(Self {
            current_version,
            keys: Arc::new(keys),
        })
    }

    #[must_use]
    pub const fn key_version(&self) -> u32 {
        self.current_version
    }

    #[must_use]
    pub fn key_versions(&self) -> Vec<u32> {
        let mut versions = self.keys.keys().copied().collect::<Vec<_>>();
        versions.sort_unstable_by(|left, right| right.cmp(left));
        versions
    }

    pub fn encrypt(
        &self,
        binding: &SecretBinding,
        plaintext: &[u8],
    ) -> Result<SecretEnvelope, SecretError> {
        self.keys
            .get(&self.current_version)
            .ok_or(SecretError::InvalidKeyring)?
            .encrypt_bound(binding, plaintext)
    }

    pub fn decrypt(
        &self,
        binding: &SecretBinding,
        envelope: &SecretEnvelope,
    ) -> Result<Zeroizing<Vec<u8>>, SecretError> {
        self.keys
            .get(&envelope.key_version)
            .ok_or(SecretError::UnknownKeyVersion)?
            .decrypt_bound(binding, envelope)
    }

    pub fn keyed_digest(
        &self,
        purpose: KeyedDigestPurpose,
        value: &[u8],
    ) -> Result<KeyedDigest, SecretError> {
        self.keyed_digest_for_version(self.current_version, purpose, value)
    }

    pub fn keyed_digest_for_version(
        &self,
        key_version: u32,
        purpose: KeyedDigestPurpose,
        value: &[u8],
    ) -> Result<KeyedDigest, SecretError> {
        self.keys
            .get(&key_version)
            .ok_or(SecretError::UnknownKeyVersion)?
            .keyed_digest(purpose, value)
    }

    /// Generates a 256-bit opaque bearer and digests it with the current root-key version under
    /// the dedicated authentication-challenge purpose.
    pub fn generate_auth_challenge(&self) -> Result<GeneratedAuthChallenge, SecretError> {
        let token = AuthChallengeToken::generate()?;
        let digest = self.keyed_digest(
            KeyedDigestPurpose::AuthChallenge,
            token.normalized_bytes(),
        )?;
        Ok(GeneratedAuthChallenge { token, digest })
    }

    /// Parses the canonical bearer and verifies it using the key version stored with the digest.
    /// This keeps in-flight challenges verifiable across a bounded root-key rotation window.
    pub fn verify_auth_challenge(
        &self,
        presented: &str,
        expected: &KeyedDigest,
    ) -> Result<bool, SecretError> {
        let token = AuthChallengeToken::parse_presented(presented)?;
        let actual = self.keyed_digest_for_version(
            expected.key_version,
            KeyedDigestPurpose::AuthChallenge,
            token.normalized_bytes(),
        )?;
        Ok(bool::from(
            expected.digest.as_slice().ct_eq(actual.digest.as_slice()),
        ))
    }

    pub fn new_canary_envelope(&self) -> Result<SecretEnvelope, SecretError> {
        self.encrypt(&SecretBinding::root_key_canary(), ROOT_KEY_CANARY_PLAINTEXT)
    }

    pub fn verify_canary(&self, envelope: &SecretEnvelope) -> Result<(), SecretError> {
        let plaintext = self.decrypt(&SecretBinding::root_key_canary(), envelope)?;
        if bool::from(plaintext.as_slice().ct_eq(ROOT_KEY_CANARY_PLAINTEXT)) {
            Ok(())
        } else {
            Err(SecretError::CanaryMismatch)
        }
    }
}

fn typed_associated_data(
    binding: &SecretBinding,
    key_version: u32,
) -> Result<Vec<u8>, SecretError> {
    if binding.schema_version == 0 || key_version == 0 {
        return Err(SecretError::InvalidSecretSchemaVersion);
    }
    let purpose = binding.purpose.as_str().as_bytes();
    let owner_kind = binding.owner_kind.as_str().as_bytes();
    let purpose_length = u8::try_from(purpose.len()).map_err(|_| SecretError::BindingTooLarge)?;
    let owner_kind_length =
        u8::try_from(owner_kind.len()).map_err(|_| SecretError::BindingTooLarge)?;
    let mut aad =
        Vec::with_capacity(TYPED_AAD_FORMAT.len() + purpose.len() + owner_kind.len() + 16 + 10);
    aad.extend_from_slice(TYPED_AAD_FORMAT);
    aad.push(purpose_length);
    aad.extend_from_slice(purpose);
    aad.push(owner_kind_length);
    aad.extend_from_slice(owner_kind);
    aad.extend_from_slice(binding.owner_id.as_bytes());
    aad.extend_from_slice(&binding.schema_version.to_be_bytes());
    aad.extend_from_slice(&key_version.to_be_bytes());
    Ok(aad)
}

#[cfg(unix)]
fn ensure_private_permissions(metadata: &fs::Metadata) -> Result<(), SecretError> {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    if metadata.permissions().mode() & 0o077 != 0 {
        return Err(SecretError::InsecureKeyPermissions);
    }
    if metadata.uid() != rustix::process::geteuid().as_raw() {
        return Err(SecretError::UnexpectedKeyOwner);
    }
    Ok(())
}

#[cfg(not(unix))]
fn ensure_private_permissions(_metadata: &fs::Metadata) -> Result<(), SecretError> {
    Ok(())
}

#[cfg(unix)]
fn read_private_root_key(path: &Path) -> Result<Zeroizing<String>, SecretError> {
    use rustix::fs::{Mode, OFlags, open};

    let descriptor = match open(
        path,
        OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK,
        Mode::empty(),
    ) {
        Ok(descriptor) => descriptor,
        Err(rustix::io::Errno::LOOP) => return Err(SecretError::KeyNotRegularFile),
        Err(error) => return Err(SecretError::Io(error.into())),
    };
    let mut file = fs::File::from(descriptor);
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(SecretError::KeyNotRegularFile);
    }
    ensure_private_permissions(&metadata)?;
    if !(ENCODED_KEY_BYTES as u64..=MAX_KEY_FILE_BYTES).contains(&metadata.len()) {
        return Err(SecretError::InvalidKeyEncoding);
    }
    read_bounded_root_key(&mut file)
}

#[cfg(windows)]
fn read_private_root_key(path: &Path) -> Result<Zeroizing<String>, SecretError> {
    use std::os::windows::fs::{FileTypeExt, OpenOptionsExt};

    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;

    let mut file = fs::OpenOptions::new()
        .read(true)
        .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)?;
    let metadata = file.metadata()?;
    let file_type = metadata.file_type();
    if file_type.is_symlink_file() || file_type.is_symlink_dir() || !metadata.is_file() {
        return Err(SecretError::KeyNotRegularFile);
    }
    ensure_private_permissions(&metadata)?;
    if !(ENCODED_KEY_BYTES as u64..=MAX_KEY_FILE_BYTES).contains(&metadata.len()) {
        return Err(SecretError::InvalidKeyEncoding);
    }
    read_bounded_root_key(&mut file)
}

#[cfg(not(any(unix, windows)))]
fn read_private_root_key(path: &Path) -> Result<Zeroizing<String>, SecretError> {
    let mut file = fs::OpenOptions::new().read(true).open(path)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(SecretError::KeyNotRegularFile);
    }
    ensure_private_permissions(&metadata)?;
    if !(ENCODED_KEY_BYTES as u64..=MAX_KEY_FILE_BYTES).contains(&metadata.len()) {
        return Err(SecretError::InvalidKeyEncoding);
    }
    read_bounded_root_key(&mut file)
}

fn read_bounded_root_key(file: &mut fs::File) -> Result<Zeroizing<String>, SecretError> {
    let mut encoded = Zeroizing::new(String::with_capacity(MAX_KEY_FILE_BYTES as usize));
    file.take(MAX_KEY_FILE_BYTES + 1)
        .read_to_string(&mut encoded)?;
    if encoded.len() > MAX_KEY_FILE_BYTES as usize {
        return Err(SecretError::InvalidKeyEncoding);
    }
    Ok(encoded)
}

#[derive(Debug, Error)]
pub enum SecretError {
    #[error("secret root key version must be greater than zero")]
    InvalidKeyVersion,
    #[error("secret root key must be a regular file")]
    KeyNotRegularFile,
    #[error("secret root key file permissions allow group or other access")]
    InsecureKeyPermissions,
    #[error("secret root key file is not owned by the effective process user")]
    UnexpectedKeyOwner,
    #[error("secret root key must be 64 lowercase hexadecimal characters")]
    InvalidKeyEncoding,
    #[error("secret keyring contains duplicate, unordered, or invalid key versions")]
    InvalidKeyring,
    #[error("secret keyring supports at most four root keys")]
    KeyringTooLarge,
    #[error("secret purpose is not supported")]
    InvalidSecretPurpose,
    #[error("secret owner kind is not supported")]
    InvalidSecretOwnerKind,
    #[error("secret owner does not match its owner kind or purpose")]
    InvalidSecretOwner,
    #[error("secret schema version must be greater than zero")]
    InvalidSecretSchemaVersion,
    #[error("recovery code must be 32 hexadecimal characters, optionally in eight groups")]
    InvalidRecoveryCode,
    #[error("authentication challenge token must be 64 lowercase hexadecimal characters")]
    InvalidAuthChallengeToken,
    #[error("secret binding exceeds the supported size")]
    BindingTooLarge,
    #[error("operating-system randomness is unavailable")]
    RandomUnavailable,
    #[error("secret encryption failed")]
    EncryptionFailed,
    #[error("secret envelope uses an unavailable key version")]
    UnknownKeyVersion,
    #[error("keyed digest input exceeds the supported size")]
    DigestInputTooLarge,
    #[error("keyed digest derivation failed")]
    KeyDerivationFailed,
    #[error("secret envelope is bound to a different purpose or owner")]
    AssociatedDataMismatch,
    #[error("secret authentication failed")]
    AuthenticationFailed,
    #[error("secret canary round-trip did not reproduce its plaintext")]
    CanaryMismatch,
    #[error("secret key file operation failed: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    #[cfg(unix)]
    use std::{
        fs,
        os::unix::fs::{PermissionsExt, symlink},
        time::SystemTime,
    };

    use super::{
        AUTH_CHALLENGE_TOKEN_HEX_LENGTH, AuthChallengeToken, EnvelopeCipher, KeyedDigest,
        KeyedDigestPurpose, Keyring, RecoveryCode, SecretBinding, SecretError, SecretOwnerKind,
        SecretPurpose, generate_recovery_codes,
    };

    const KEY: &str = "f97c2563b4609f964f83ecf3c874f545698b8e360bbca06316547d2af8928f62";
    const OLD_KEY: &str = "097c2563b4609f964f83ecf3c874f545698b8e360bbca06316547d2af8928f62";

    #[test]
    fn typed_envelope_binds_owner_schema_and_key_version() {
        let cipher = EnvelopeCipher::from_hex(KEY, 2);
        assert!(cipher.is_ok());
        if let Ok(cipher) = cipher {
            let owner = uuid::Uuid::now_v7();
            let binding =
                SecretBinding::new(SecretPurpose::TotpSeed, SecretOwnerKind::User, owner, 1);
            assert!(binding.is_ok());
            if let Ok(binding) = binding {
                let envelope = cipher.encrypt_bound(&binding, b"seed");
                assert!(envelope.is_ok());
                if let Ok(envelope) = envelope {
                    assert!(matches!(
                        cipher.decrypt_bound(&binding, &envelope),
                        Ok(plaintext) if plaintext.as_slice() == b"seed"
                    ));
                    let other_owner = SecretBinding::new(
                        SecretPurpose::TotpSeed,
                        SecretOwnerKind::User,
                        uuid::Uuid::now_v7(),
                        1,
                    );
                    assert!(matches!(
                        other_owner.and_then(|binding| cipher.decrypt_bound(&binding, &envelope)),
                        Err(SecretError::AssociatedDataMismatch)
                    ));
                    let other_schema = SecretBinding::new(
                        SecretPurpose::TotpSeed,
                        SecretOwnerKind::User,
                        owner,
                        2,
                    );
                    assert!(matches!(
                        other_schema.and_then(|binding| cipher.decrypt_bound(&binding, &envelope)),
                        Err(SecretError::AssociatedDataMismatch)
                    ));
                }
            }
        }
    }

    #[test]
    fn finite_keyring_decrypts_old_canary_and_writes_only_current_version() {
        let current = EnvelopeCipher::from_hex(KEY, 2);
        let previous = EnvelopeCipher::from_hex(OLD_KEY, 1);
        assert!(current.is_ok());
        assert!(previous.is_ok());
        if let (Ok(current), Ok(previous)) = (current, previous) {
            let old_envelope = previous.encrypt_bound(
                &SecretBinding::root_key_canary(),
                super::ROOT_KEY_CANARY_PLAINTEXT,
            );
            let keyring = Keyring::from_ciphers(current, vec![previous]);
            assert!(keyring.is_ok());
            if let (Ok(old_envelope), Ok(keyring)) = (old_envelope, keyring) {
                assert!(keyring.verify_canary(&old_envelope).is_ok());
                let new_envelope = keyring.new_canary_envelope();
                assert!(matches!(new_envelope, Ok(envelope) if envelope.key_version == 2));
                assert_eq!(keyring.key_versions(), vec![2, 1]);
            }
        }
    }

    #[test]
    fn recovery_codes_have_128_bits_and_explicit_normalization() {
        let codes = generate_recovery_codes();
        assert!(codes.is_ok());
        if let Ok(codes) = codes {
            assert_eq!(codes.len(), super::RECOVERY_CODE_COUNT);
            let distinct = codes
                .iter()
                .map(RecoveryCode::normalized_bytes)
                .collect::<std::collections::HashSet<_>>();
            assert_eq!(distinct.len(), super::RECOVERY_CODE_COUNT);
            for code in codes {
                assert_eq!(code.normalized_bytes().len(), 16);
                assert_eq!(code.presented().len(), 39);
                assert!(matches!(
                    RecoveryCode::parse_presented(&code.presented().to_ascii_uppercase()),
                    Ok(parsed) if parsed.normalized_bytes() == code.normalized_bytes()
                ));
            }
        }
        for invalid in [
            "0000 0000 0000 0000 0000 0000 0000 0000",
            "0000-0000",
            " 00000000000000000000000000000000",
        ] {
            assert!(matches!(
                RecoveryCode::parse_presented(invalid),
                Err(SecretError::InvalidRecoveryCode)
            ));
        }
    }

    #[test]
    fn auth_challenge_tokens_have_256_bits_and_one_canonical_wire_form() {
        let first = AuthChallengeToken::generate();
        let second = AuthChallengeToken::generate();
        assert!(first.is_ok());
        assert!(second.is_ok());
        if let (Ok(first), Ok(second)) = (first, second) {
            assert_eq!(first.normalized_bytes().len(), 32);
            assert_eq!(first.presented().len(), AUTH_CHALLENGE_TOKEN_HEX_LENGTH);
            let canonical = first
                .presented()
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase());
            assert!(canonical);
            assert!(first.normalized_bytes() != second.normalized_bytes());
            assert!(matches!(
                AuthChallengeToken::parse_presented(first.presented()),
                Ok(parsed) if parsed.normalized_bytes() == first.normalized_bytes()
            ));
        }
        for invalid in [
            "0",
            "g000000000000000000000000000000000000000000000000000000000000000",
            " 000000000000000000000000000000000000000000000000000000000000000",
        ] {
            assert!(matches!(
                AuthChallengeToken::parse_presented(invalid),
                Err(SecretError::InvalidAuthChallengeToken)
            ));
        }
        assert!(matches!(
            AuthChallengeToken::parse_presented(&format!("A{}", "0".repeat(63))),
            Err(SecretError::InvalidAuthChallengeToken)
        ));
        assert!(matches!(
            AuthChallengeToken::parse_presented(&"0".repeat(65)),
            Err(SecretError::InvalidAuthChallengeToken)
        ));
    }

    #[test]
    fn auth_challenge_generation_uses_current_key_and_old_version_verification_is_explicit() {
        let current = EnvelopeCipher::from_hex(KEY, 2);
        let old = EnvelopeCipher::from_hex(OLD_KEY, 1);
        assert!(current.is_ok());
        assert!(old.is_ok());
        if let (Ok(current), Ok(old)) = (current, old) {
            let keyring = Keyring::from_ciphers(current, vec![old]);
            assert!(keyring.is_ok());
            if let Ok(keyring) = keyring {
                let generated = keyring.generate_auth_challenge();
                assert!(generated.is_ok());
                if let Ok(generated) = generated {
                    assert_eq!(generated.digest.key_version, 2);
                    assert!(matches!(
                        keyring.verify_auth_challenge(
                            generated.token.presented(),
                            &generated.digest
                        ),
                        Ok(true)
                    ));
                    let other = keyring.generate_auth_challenge();
                    assert!(matches!(
                        other,
                        Ok(other)
                            if keyring.verify_auth_challenge(
                                other.token.presented(),
                                &generated.digest
                            ) == Ok(false)
                    ));
                }

                let old_token = AuthChallengeToken::generate();
                assert!(old_token.is_ok());
                if let Ok(old_token) = old_token {
                    let old_digest = keyring.keyed_digest_for_version(
                        1,
                        KeyedDigestPurpose::AuthChallenge,
                        old_token.normalized_bytes(),
                    );
                    assert!(matches!(
                        old_digest.as_ref(),
                        Ok(digest) if digest.key_version == 1
                    ));
                    if let Ok(old_digest) = old_digest {
                        assert!(matches!(
                            keyring.verify_auth_challenge(old_token.presented(), &old_digest),
                            Ok(true)
                        ));
                        let unavailable = KeyedDigest {
                            key_version: 3,
                            digest: old_digest.digest,
                        };
                        assert!(matches!(
                            keyring.verify_auth_challenge(old_token.presented(), &unavailable),
                            Err(SecretError::UnknownKeyVersion)
                        ));
                    }
                }
            }
        }
    }

    #[test]
    fn encryption_round_trip_is_bound_to_owner_and_purpose() {
        let cipher = EnvelopeCipher::from_hex(KEY, 1);
        assert!(cipher.is_ok());
        if let Ok(cipher) = cipher {
            let binding = SecretBinding::new(
                SecretPurpose::TotpSeed,
                SecretOwnerKind::User,
                uuid::Uuid::now_v7(),
                1,
            );
            assert!(binding.is_ok());
            let Ok(binding) = binding else {
                return;
            };
            let envelope = cipher.encrypt_bound(&binding, b"top-secret");
            assert!(envelope.is_ok());
            if let Ok(envelope) = envelope {
                assert_ne!(envelope.ciphertext, b"top-secret");
                assert!(matches!(
                    cipher.decrypt_bound(&binding, &envelope),
                    Ok(plaintext) if plaintext.as_slice() == b"top-secret"
                ));
                let other_binding = SecretBinding::new(
                    SecretPurpose::TotpSeed,
                    SecretOwnerKind::User,
                    uuid::Uuid::now_v7(),
                    1,
                );
                assert!(matches!(
                    other_binding.and_then(|binding| cipher.decrypt_bound(&binding, &envelope)),
                    Err(SecretError::AssociatedDataMismatch)
                ));
            }
        }
    }

    #[test]
    fn tampering_is_rejected() {
        let cipher = EnvelopeCipher::from_hex(KEY, 1);
        assert!(cipher.is_ok());
        if let Ok(cipher) = cipher {
            let binding = SecretBinding::root_key_canary();
            let envelope = cipher.encrypt_bound(&binding, b"secret");
            assert!(envelope.is_ok());
            if let Ok(mut envelope) = envelope {
                if let Some(byte) = envelope.ciphertext.first_mut() {
                    *byte ^= 1;
                }
                assert!(matches!(
                    cipher.decrypt_bound(&binding, &envelope),
                    Err(SecretError::AuthenticationFailed)
                ));
            }
        }
    }

    #[test]
    fn canary_and_key_validation_are_explicit() {
        assert!(matches!(
            EnvelopeCipher::from_hex("ABC", 1),
            Err(SecretError::InvalidKeyEncoding)
        ));
        let cipher = EnvelopeCipher::from_hex(KEY, 1);
        assert!(cipher.is_ok());
        if let Ok(cipher) = cipher {
            let keyring = Keyring::from_ciphers(cipher, Vec::new());
            assert!(keyring.is_ok());
            if let Ok(keyring) = keyring {
                let envelope = keyring.new_canary_envelope();
                assert!(matches!(
                    envelope,
                    Ok(envelope) if keyring.verify_canary(&envelope).is_ok()
                ));
            }
        }
    }

    #[test]
    fn keyed_digests_are_deterministic_and_purpose_separated() {
        let cipher = EnvelopeCipher::from_hex(KEY, 7);
        assert!(cipher.is_ok());
        if let Ok(cipher) = cipher {
            let session = cipher.keyed_digest(KeyedDigestPurpose::Session, b"same-bearer");
            let session_again = cipher.keyed_digest(KeyedDigestPurpose::Session, b"same-bearer");
            let csrf = cipher.keyed_digest(KeyedDigestPurpose::Csrf, b"same-bearer");
            let account = cipher.keyed_digest(KeyedDigestPurpose::LoginAccount, b"same-bearer");
            let ip = cipher.keyed_digest(KeyedDigestPurpose::LoginIp, b"same-bearer");
            let global = cipher.keyed_digest(KeyedDigestPurpose::LoginGlobal, b"same-bearer");
            let recovery = cipher.keyed_digest(KeyedDigestPurpose::RecoveryCode, b"same-bearer");
            let challenge = cipher.keyed_digest(KeyedDigestPurpose::AuthChallenge, b"same-bearer");
            assert!(session.is_ok());
            assert!(session_again.is_ok());
            assert!(csrf.is_ok());
            assert!(account.is_ok());
            assert!(ip.is_ok());
            assert!(global.is_ok());
            assert!(recovery.is_ok());
            assert!(challenge.is_ok());
            if let (
                Ok(session),
                Ok(session_again),
                Ok(csrf),
                Ok(account),
                Ok(ip),
                Ok(global),
                Ok(recovery),
                Ok(challenge),
            ) = (
                session,
                session_again,
                csrf,
                account,
                ip,
                global,
                recovery,
                challenge,
            ) {
                assert_eq!(session.key_version, 7);
                assert_eq!(session.digest.len(), 32);
                assert!(session == session_again);
                assert!(session != csrf);
                assert!(session != account);
                assert!(session != ip);
                assert!(session != global);
                assert!(session != recovery);
                assert!(session != challenge);
                assert!(recovery != challenge);
                assert!(matches!(
                    cipher.verify_keyed_digest(
                        KeyedDigestPurpose::Session,
                        b"same-bearer",
                        &session
                    ),
                    Ok(true)
                ));
                assert!(matches!(
                    cipher.verify_keyed_digest(
                        KeyedDigestPurpose::Session,
                        b"different-bearer",
                        &session
                    ),
                    Ok(false)
                ));
                assert!(matches!(
                    cipher.verify_keyed_digest(KeyedDigestPurpose::Csrf, b"same-bearer", &session),
                    Ok(false)
                ));
            }
        }
    }

    #[test]
    fn keyed_digest_key_versions_are_cryptographically_separated() {
        let first = EnvelopeCipher::from_hex(KEY, 1);
        let second = EnvelopeCipher::from_hex(KEY, 2);
        assert!(first.is_ok());
        assert!(second.is_ok());
        if let (Ok(first), Ok(second)) = (first, second) {
            let first_digest = first.keyed_digest(KeyedDigestPurpose::Session, b"bearer");
            let second_digest = second.keyed_digest(KeyedDigestPurpose::Session, b"bearer");
            assert!(first_digest.is_ok());
            assert!(second_digest.is_ok());
            if let (Ok(first_digest), Ok(second_digest)) = (first_digest, second_digest) {
                assert_eq!(first_digest.key_version, 1);
                assert_eq!(second_digest.key_version, 2);
                assert!(first_digest.digest != second_digest.digest);
                assert!(matches!(
                    second.verify_keyed_digest(
                        KeyedDigestPurpose::Session,
                        b"bearer",
                        &first_digest
                    ),
                    Err(SecretError::UnknownKeyVersion)
                ));
            }
        }
    }

    #[cfg(unix)]
    fn temporary_key_path(label: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        std::env::temp_dir().join(format!(
            "nodecontroll-{label}-{}-{nonce}",
            std::process::id()
        ))
    }

    #[cfg(unix)]
    #[test]
    fn root_key_file_rejects_group_or_other_access() {
        let path = temporary_key_path("secret-key");
        assert!(fs::write(&path, KEY).is_ok());
        assert!(fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).is_ok());
        assert!(matches!(
            EnvelopeCipher::from_key_file(&path, 1),
            Err(SecretError::InsecureKeyPermissions)
        ));
        assert!(fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).is_ok());
        assert!(EnvelopeCipher::from_key_file(&path, 1).is_ok());
        let _ = fs::remove_file(path);
    }

    #[cfg(unix)]
    #[test]
    fn root_key_file_rejects_symlink_and_non_regular_file() {
        let target = temporary_key_path("secret-key-target");
        let link = temporary_key_path("secret-key-link");
        let directory = temporary_key_path("secret-key-directory");
        assert!(fs::write(&target, KEY).is_ok());
        assert!(fs::set_permissions(&target, fs::Permissions::from_mode(0o600)).is_ok());
        assert!(symlink(&target, &link).is_ok());
        assert!(matches!(
            EnvelopeCipher::from_key_file(&link, 1),
            Err(SecretError::KeyNotRegularFile)
        ));
        assert!(fs::create_dir(&directory).is_ok());
        assert!(matches!(
            EnvelopeCipher::from_key_file(&directory, 1),
            Err(SecretError::KeyNotRegularFile)
        ));
        let _ = fs::remove_file(link);
        let _ = fs::remove_file(target);
        let _ = fs::remove_dir(directory);
    }
}
