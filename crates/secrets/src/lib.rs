use std::{fs, path::Path, sync::Arc};

use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, Key, KeyInit, Payload},
};
use sha2::{Digest, Sha256};
use thiserror::Error;
use zeroize::Zeroizing;

const KEY_BYTES: usize = 32;
const NONCE_BYTES: usize = 24;
const AAD_FORMAT: &[u8] = b"NCSECRET1\0";

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
        let path = path.as_ref();
        let metadata = fs::metadata(path)?;
        if !metadata.is_file() {
            return Err(SecretError::KeyNotRegularFile);
        }
        ensure_private_permissions(&metadata)?;
        let encoded = Zeroizing::new(fs::read_to_string(path)?);
        Self::from_hex(encoded.trim(), key_version)
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
    use std::os::unix::fs::PermissionsExt;

    if metadata.permissions().mode() & 0o077 == 0 {
        Ok(())
    } else {
        Err(SecretError::InsecureKeyPermissions)
    }
}

#[cfg(not(unix))]
fn ensure_private_permissions(_metadata: &fs::Metadata) -> Result<(), SecretError> {
    Ok(())
}

#[derive(Debug, Error)]
pub enum SecretError {
    #[error("secret root key version must be greater than zero")]
    InvalidKeyVersion,
    #[error("secret root key must be a regular file")]
    KeyNotRegularFile,
    #[error("secret root key file permissions allow group or other access")]
    InsecureKeyPermissions,
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
    use std::{fs, os::unix::fs::PermissionsExt, time::SystemTime};

    use super::{EnvelopeCipher, SecretError};

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

    #[cfg(unix)]
    #[test]
    fn root_key_file_rejects_group_or_other_access() {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        let path = std::env::temp_dir().join(format!(
            "nodecontroll-secret-key-{}-{nonce}",
            std::process::id()
        ));
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
}
