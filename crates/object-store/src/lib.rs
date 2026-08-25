use std::{
    io,
    path::{Path, PathBuf},
    sync::Arc,
};

use async_trait::async_trait;
use sha2::{Digest, Sha256};
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

const HASH_HEX_LENGTH: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredObject {
    pub sha256: String,
    pub size_bytes: u64,
    pub storage_key: String,
}

#[async_trait]
pub trait ObjectStore: Send + Sync {
    async fn put(&self, content: &[u8]) -> Result<StoredObject, ObjectStoreError>;
    async fn get(&self, object: &StoredObject) -> Result<Vec<u8>, ObjectStoreError>;
}

#[derive(Clone)]
pub struct FilesystemObjectStore {
    root: Arc<PathBuf>,
    max_object_bytes: u64,
}

impl FilesystemObjectStore {
    pub async fn open(
        root: impl AsRef<Path>,
        max_object_bytes: u64,
    ) -> Result<Self, ObjectStoreError> {
        if max_object_bytes == 0 {
            return Err(ObjectStoreError::ZeroSizeLimit);
        }
        tokio::fs::create_dir_all(root.as_ref()).await?;
        let root = tokio::fs::canonicalize(root.as_ref()).await?;
        Ok(Self {
            root: Arc::new(root),
            max_object_bytes,
        })
    }

    fn path_for_hash(&self, hash: &str) -> Result<(String, PathBuf), ObjectStoreError> {
        if hash.len() != HASH_HEX_LENGTH
            || !hash
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err(ObjectStoreError::InvalidHash);
        }
        let key = format!("sha256/{}/{}/{}", &hash[..2], &hash[2..4], hash);
        Ok((key.clone(), self.root.join(key)))
    }

    async fn verify_existing(
        &self,
        path: &Path,
        expected_hash: &str,
        expected_size: u64,
    ) -> Result<Vec<u8>, ObjectStoreError> {
        let content = tokio::fs::read(path).await?;
        let actual_size =
            u64::try_from(content.len()).map_err(|_| ObjectStoreError::SizeOverflow)?;
        let actual_hash = digest_hex(&content);
        if actual_size != expected_size || actual_hash != expected_hash {
            return Err(ObjectStoreError::IntegrityMismatch);
        }
        Ok(content)
    }
}

#[async_trait]
impl ObjectStore for FilesystemObjectStore {
    async fn put(&self, content: &[u8]) -> Result<StoredObject, ObjectStoreError> {
        let size_bytes =
            u64::try_from(content.len()).map_err(|_| ObjectStoreError::SizeOverflow)?;
        if size_bytes > self.max_object_bytes {
            return Err(ObjectStoreError::ObjectTooLarge {
                size_bytes,
                max_bytes: self.max_object_bytes,
            });
        }

        let sha256 = digest_hex(content);
        let (storage_key, target) = self.path_for_hash(&sha256)?;
        if tokio::fs::try_exists(&target).await? {
            self.verify_existing(&target, &sha256, size_bytes).await?;
            return Ok(StoredObject {
                sha256,
                size_bytes,
                storage_key,
            });
        }

        let parent = target.parent().ok_or(ObjectStoreError::InvalidHash)?;
        tokio::fs::create_dir_all(parent).await?;
        let temporary = parent.join(format!(".{}.{}.tmp", sha256, Uuid::now_v7()));
        let write_result = async {
            let mut file = tokio::fs::OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&temporary)
                .await?;
            file.write_all(content).await?;
            file.flush().await?;
            file.sync_all().await?;
            drop(file);
            tokio::fs::rename(&temporary, &target).await?;
            sync_directory(parent.to_owned()).await?;
            Ok::<(), io::Error>(())
        }
        .await;

        if let Err(error) = write_result {
            let _ = tokio::fs::remove_file(&temporary).await;
            if tokio::fs::try_exists(&target).await.unwrap_or(false) {
                self.verify_existing(&target, &sha256, size_bytes).await?;
            } else {
                return Err(ObjectStoreError::Io(error));
            }
        }

        Ok(StoredObject {
            sha256,
            size_bytes,
            storage_key,
        })
    }

    async fn get(&self, object: &StoredObject) -> Result<Vec<u8>, ObjectStoreError> {
        let (expected_key, path) = self.path_for_hash(&object.sha256)?;
        if expected_key != object.storage_key {
            return Err(ObjectStoreError::InvalidStorageKey);
        }
        self.verify_existing(&path, &object.sha256, object.size_bytes)
            .await
    }
}

fn digest_hex(content: &[u8]) -> String {
    hex::encode(Sha256::digest(content))
}

async fn sync_directory(path: PathBuf) -> Result<(), io::Error> {
    tokio::task::spawn_blocking(move || std::fs::File::open(path)?.sync_all())
        .await
        .map_err(io::Error::other)?
}

#[derive(Debug, Error)]
pub enum ObjectStoreError {
    #[error("max object size must be greater than zero")]
    ZeroSizeLimit,
    #[error("object size cannot be represented")]
    SizeOverflow,
    #[error("object size {size_bytes} exceeds configured maximum {max_bytes}")]
    ObjectTooLarge { size_bytes: u64, max_bytes: u64 },
    #[error("object hash is not canonical lowercase SHA-256")]
    InvalidHash,
    #[error("object storage key does not match its content hash")]
    InvalidStorageKey,
    #[error("stored object failed size or hash verification")]
    IntegrityMismatch,
    #[error("filesystem object operation failed: {0}")]
    Io(#[from] io::Error),
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::{FilesystemObjectStore, ObjectStore, ObjectStoreError};

    fn test_root(label: &str) -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        std::env::temp_dir().join(format!(
            "nodecontroll-object-{label}-{}-{nonce}",
            std::process::id()
        ))
    }

    #[tokio::test]
    async fn put_is_content_addressed_atomic_and_idempotent() {
        let root = test_root("roundtrip");
        let store = FilesystemObjectStore::open(&root, 1024).await;
        assert!(store.is_ok());
        if let Ok(store) = store {
            let first = store.put(b"NodeControll").await;
            assert!(first.is_ok());
            if let Ok(first) = first {
                let second = store.put(b"NodeControll").await;
                assert!(matches!(second, Ok(ref value) if value == &first));
                assert!(matches!(store.get(&first).await, Ok(value) if value == b"NodeControll"));
                assert!(first.storage_key.starts_with("sha256/"));
                assert!(!first.storage_key.contains(".."));
            }
        }
        let _ = tokio::fs::remove_dir_all(root).await;
    }

    #[tokio::test]
    async fn size_limit_is_enforced_before_writing() {
        let root = test_root("limit");
        let store = FilesystemObjectStore::open(&root, 3).await;
        assert!(store.is_ok());
        if let Ok(store) = store {
            assert!(matches!(
                store.put(b"four").await,
                Err(ObjectStoreError::ObjectTooLarge { .. })
            ));
        }
        let _ = tokio::fs::remove_dir_all(root).await;
    }

    #[tokio::test]
    async fn read_detects_on_disk_corruption() {
        let root = test_root("corruption");
        let store = FilesystemObjectStore::open(&root, 1024).await;
        assert!(store.is_ok());
        if let Ok(store) = store {
            let object = store.put(b"correct").await;
            assert!(object.is_ok());
            if let Ok(object) = object {
                let path = root.join(&object.storage_key);
                assert!(tokio::fs::write(path, b"corrupt").await.is_ok());
                assert!(matches!(
                    store.get(&object).await,
                    Err(ObjectStoreError::IntegrityMismatch)
                ));
            }
        }
        let _ = tokio::fs::remove_dir_all(root).await;
    }
}
