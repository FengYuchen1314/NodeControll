use std::{fs, io::Read, path::Path, sync::Arc};

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
const AAD_FORMAT: &[u8] = b"NCSECRET1\0";
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
}

impl KeyedDigestPurpose {
    const fn context(self) -> &'static [u8] {
        match self {
            Self::Session => b"session",
            Self::Csrf => b"csrf",
            Self::LoginAccount => b"login-account",
            Self::LoginIp => b"login-ip",
            Self::LoginGlobal => b"login-global",
        }
    }
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

    pub fn encrypt(
        &self,
        purpose: &str,
        owner: &str,
        plaintext: &[u8],
    ) -> Result<SecretEnvelope, SecretError> {
        let aad = associated_data(purpose, owner)?;
        let aad_hash: [u8; 32] = Sha256::digest(&aad).into();
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
                    aad: &aad,
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

    pub fn decrypt(
        &self,
        purpose: &str,
        owner: &str,
        envelope: &SecretEnvelope,
    ) -> Result<Zeroizing<Vec<u8>>, SecretError> {
        if envelope.key_version != self.key_version {
            return Err(SecretError::UnknownKeyVersion);
        }
        let aad = associated_data(purpose, owner)?;
        let aad_hash: [u8; 32] = Sha256::digest(&aad).into();
        if aad_hash != envelope.aad_hash {
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
                    aad: &aad,
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

    pub fn canary(&self) -> Result<(), SecretError> {
        let envelope = self.encrypt("system.canary", "local-master", b"nodecontroll-canary-v1")?;
        let plaintext = self.decrypt("system.canary", "local-master", &envelope)?;
        if plaintext.as_slice() == b"nodecontroll-canary-v1" {
            Ok(())
        } else {
            Err(SecretError::CanaryMismatch)
        }
    }
}

fn associated_data(purpose: &str, owner: &str) -> Result<Vec<u8>, SecretError> {
    if purpose.is_empty() || owner.is_empty() {
        return Err(SecretError::EmptyBinding);
    }
    let purpose_length = u32::try_from(purpose.len()).map_err(|_| SecretError::BindingTooLarge)?;
    let owner_length = u32::try_from(owner.len()).map_err(|_| SecretError::BindingTooLarge)?;
    let mut aad = Vec::with_capacity(AAD_FORMAT.len() + purpose.len() + owner.len() + 8);
    aad.extend_from_slice(AAD_FORMAT);
    aad.extend_from_slice(&purpose_length.to_be_bytes());
    aad.extend_from_slice(purpose.as_bytes());
    aad.extend_from_slice(&owner_length.to_be_bytes());
    aad.extend_from_slice(owner.as_bytes());
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
    #[error("secret purpose and owner bindings cannot be empty")]
    EmptyBinding,
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

    use super::{EnvelopeCipher, KeyedDigestPurpose, SecretError};

    const KEY: &str = "f97c2563b4609f964f83ecf3c874f545698b8e360bbca06316547d2af8928f62";

    #[test]
    fn encryption_round_trip_is_bound_to_owner_and_purpose() {
        let cipher = EnvelopeCipher::from_hex(KEY, 1);
        assert!(cipher.is_ok());
        if let Ok(cipher) = cipher {
            let envelope = cipher.encrypt("telegram.bot", "instance-1", b"top-secret");
            assert!(envelope.is_ok());
            if let Ok(envelope) = envelope {
                assert_ne!(envelope.ciphertext, b"top-secret");
                assert!(matches!(
                    cipher.decrypt("telegram.bot", "instance-1", &envelope),
                    Ok(plaintext) if plaintext.as_slice() == b"top-secret"
                ));
                assert!(matches!(
                    cipher.decrypt("telegram.bot", "instance-2", &envelope),
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
            let envelope = cipher.encrypt("test", "owner", b"secret");
            assert!(envelope.is_ok());
            if let Ok(mut envelope) = envelope {
                if let Some(byte) = envelope.ciphertext.first_mut() {
                    *byte ^= 1;
                }
                assert!(matches!(
                    cipher.decrypt("test", "owner", &envelope),
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
        assert!(matches!(cipher, Ok(ref value) if value.canary().is_ok()));
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
            assert!(session.is_ok());
            assert!(session_again.is_ok());
            assert!(csrf.is_ok());
            assert!(account.is_ok());
            assert!(ip.is_ok());
            assert!(global.is_ok());
            if let (Ok(session), Ok(session_again), Ok(csrf), Ok(account), Ok(ip), Ok(global)) =
                (session, session_again, csrf, account, ip, global)
            {
                assert_eq!(session.key_version, 7);
                assert_eq!(session.digest.len(), 32);
                assert!(session == session_again);
                assert!(session != csrf);
                assert!(session != account);
                assert!(session != ip);
                assert!(session != global);
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
