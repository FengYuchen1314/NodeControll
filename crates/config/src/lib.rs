use std::{net::SocketAddr, path::Path};

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use thiserror::Error;

const DEFAULT_LISTEN: &str = "127.0.0.1:8080";
const DEFAULT_DATABASE_URL: &str = "sqlite://nodecontroll.db?mode=rwc";
const DEFAULT_SETUP_TOKEN_TTL_SECONDS: u64 = 1_800;
const MAX_SETUP_TOKEN_TTL_SECONDS: u64 = 3_600;

#[derive(Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MasterConfig {
    pub http: HttpConfig,
    pub database: DatabaseConfig,
    pub secrets: SecretsConfig,
    pub bootstrap: BootstrapConfig,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct HttpConfig {
    pub listen: SocketAddr,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            listen: SocketAddr::from(([127, 0, 0, 1], 8080)),
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DatabaseConfig {
    pub url: SecretString,
    pub max_connections: u32,
    pub acquire_timeout_ms: u64,
    pub statement_timeout_ms: u64,
    pub lock_timeout_ms: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: SecretString::from(DEFAULT_DATABASE_URL),
            max_connections: 8,
            acquire_timeout_ms: 5_000,
            statement_timeout_ms: 30_000,
            lock_timeout_ms: 5_000,
        }
    }
}

impl DatabaseConfig {
    #[must_use]
    pub fn url(&self) -> &str {
        self.url.expose_secret()
    }

    #[must_use]
    pub fn redacted_url(&self) -> &'static str {
        "[REDACTED]"
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SecretsConfig {
    pub root_key_file: std::path::PathBuf,
    pub setup_token_file: std::path::PathBuf,
}

impl Default for SecretsConfig {
    fn default() -> Self {
        Self {
            root_key_file: std::path::PathBuf::from("nodecontroll.key"),
            setup_token_file: std::path::PathBuf::from("nodecontroll.setup-token"),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct BootstrapConfig {
    pub setup_token_ttl_seconds: u64,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        Self {
            setup_token_ttl_seconds: DEFAULT_SETUP_TOKEN_TTL_SECONDS,
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("could not load configuration: {0}")]
    Load(#[from] config::ConfigError),
    #[error("database.max_connections must be greater than zero")]
    ZeroDatabaseConnections,
    #[error("database timeout values must be greater than zero")]
    ZeroDatabaseTimeout,
    #[error("bootstrap.setup_token_ttl_seconds must be between 1 and 3600")]
    InvalidSetupTokenTtl,
}

pub fn load(path: Option<&Path>) -> Result<MasterConfig, ConfigError> {
    let mut builder = config::Config::builder()
        .set_default("http.listen", DEFAULT_LISTEN)?
        .set_default("database.url", DEFAULT_DATABASE_URL)?
        .set_default("database.max_connections", 8_u32)?
        .set_default("database.acquire_timeout_ms", 5_000_u64)?
        .set_default("database.statement_timeout_ms", 30_000_u64)?
        .set_default("database.lock_timeout_ms", 5_000_u64)?
        .set_default("secrets.root_key_file", "nodecontroll.key")?
        .set_default("secrets.setup_token_file", "nodecontroll.setup-token")?
        .set_default(
            "bootstrap.setup_token_ttl_seconds",
            DEFAULT_SETUP_TOKEN_TTL_SECONDS,
        )?;

    if let Some(path) = path {
        builder = builder.add_source(config::File::from(path).required(true));
    }

    let loaded: MasterConfig = builder
        .add_source(
            config::Environment::with_prefix("NODECONTROLL")
                .prefix_separator("__")
                .separator("__")
                .try_parsing(true),
        )
        .build()?
        .try_deserialize()?;

    if loaded.database.max_connections == 0 {
        return Err(ConfigError::ZeroDatabaseConnections);
    }
    if loaded.database.acquire_timeout_ms == 0
        || loaded.database.statement_timeout_ms == 0
        || loaded.database.lock_timeout_ms == 0
    {
        return Err(ConfigError::ZeroDatabaseTimeout);
    }
    if !(1..=MAX_SETUP_TOKEN_TTL_SECONDS).contains(&loaded.bootstrap.setup_token_ttl_seconds) {
        return Err(ConfigError::InvalidSetupTokenTtl);
    }

    Ok(loaded)
}

#[cfg(test)]
mod tests {
    use std::{fs, time::SystemTime};

    use super::{ConfigError, load};

    #[test]
    fn defaults_are_loopback_and_sqlite() {
        let loaded = load(None);
        assert!(loaded.is_ok());
        if let Ok(loaded) = loaded {
            assert!(loaded.http.listen.ip().is_loopback());
            assert!(loaded.database.url().starts_with("sqlite:"));
            assert_eq!(loaded.database.redacted_url(), "[REDACTED]");
            assert_eq!(loaded.bootstrap.setup_token_ttl_seconds, 1_800);
        }
    }

    #[test]
    fn unknown_file_keys_are_rejected() {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        let path = std::env::temp_dir().join(format!(
            "nodecontroll-config-{}-{nonce}.toml",
            std::process::id()
        ));
        let write_result = fs::write(&path, "[http]\nlisten = '127.0.0.1:8080'\nunknown = true\n");
        assert!(write_result.is_ok());
        let loaded = load(Some(&path));
        let _ = fs::remove_file(path);
        assert!(matches!(loaded, Err(ConfigError::Load(_))));
    }

    #[test]
    fn setup_capability_ttl_is_bounded() {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        let path = std::env::temp_dir().join(format!(
            "nodecontroll-config-ttl-{}-{nonce}.toml",
            std::process::id()
        ));
        assert!(fs::write(&path, "[bootstrap]\nsetup_token_ttl_seconds = 3601\n").is_ok());
        let loaded = load(Some(&path));
        let _ = fs::remove_file(path);
        assert!(matches!(loaded, Err(ConfigError::InvalidSetupTokenTtl)));
    }
}
