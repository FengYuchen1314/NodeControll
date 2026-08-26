use std::{
    fmt,
    net::{IpAddr, SocketAddr},
    path::Path,
};

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use thiserror::Error;

const DEFAULT_LISTEN: &str = "127.0.0.1:8080";
const DEFAULT_PUBLIC_ORIGIN: &str = "http://127.0.0.1:8080";
const DEFAULT_DATABASE_URL: &str = "sqlite://nodecontroll.db?mode=rwc";
const DEFAULT_SETUP_TOKEN_TTL_SECONDS: u64 = 1_800;
const MAX_SETUP_TOKEN_TTL_SECONDS: u64 = 3_600;
const DEFAULT_SESSION_IDLE_SECONDS: u64 = 1_800;
const DEFAULT_SESSION_ABSOLUTE_SECONDS: u64 = 86_400;
const DEFAULT_RECENT_AUTH_SECONDS: u64 = 300;
const MIN_SESSION_IDLE_SECONDS: u64 = 60;
const MAX_SESSION_IDLE_SECONDS: u64 = 86_400;
const MIN_SESSION_ABSOLUTE_SECONDS: u64 = 300;
const MAX_SESSION_ABSOLUTE_SECONDS: u64 = 2_592_000;
const MIN_RECENT_AUTH_SECONDS: u64 = 60;
const MAX_RECENT_AUTH_SECONDS: u64 = 3_600;
const DEFAULT_LOGIN_WINDOW_SECONDS: u64 = 300;
const DEFAULT_LOGIN_BLOCK_SECONDS: u64 = 900;
const MIN_LOGIN_WINDOW_SECONDS: u64 = 10;
const MAX_LOGIN_WINDOW_SECONDS: u64 = 3_600;
const MAX_LOGIN_BLOCK_SECONDS: u64 = 86_400;
const DEFAULT_LOGIN_ACCOUNT_LIMIT: u32 = 5;
const DEFAULT_LOGIN_IP_LIMIT: u32 = 50;
const DEFAULT_LOGIN_GLOBAL_LIMIT: u32 = 10_000;
const MAX_LOGIN_LIMIT: u32 = 1_000_000;
const DEFAULT_PASSWORD_HASH_CONCURRENCY: u32 = 4;
const MAX_PASSWORD_HASH_CONCURRENCY: u32 = 64;
const DEFAULT_DIGEST_KEY_VERSION: u32 = 1;
const MAX_DIGEST_KEY_VERSION: u32 = 65_535;

/// Canonical browser origin used for Origin checks and absolute public links.
///
/// Only a scheme and authority are accepted. HTTP is limited to a literal loopback address or
/// `localhost`; all externally reachable names and addresses require HTTPS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicOrigin(String);

impl PublicOrigin {
    pub fn parse(value: impl Into<String>) -> Result<Self, PublicOriginParseError> {
        let value = value.into();
        if value.trim() != value {
            return Err(PublicOriginParseError::InvalidShape);
        }
        let (scheme, authority) = if let Some(authority) = value.strip_prefix("http://") {
            ("http", authority)
        } else if let Some(authority) = value.strip_prefix("https://") {
            ("https", authority)
        } else {
            return Err(PublicOriginParseError::InvalidScheme);
        };
        if authority.is_empty()
            || authority.chars().any(|character| {
                matches!(character, '/' | '?' | '#' | '@') || character.is_control()
            })
        {
            return Err(PublicOriginParseError::InvalidShape);
        }

        let (canonical_host, is_loopback, parsed_port) = parse_origin_authority(authority)?;
        if scheme == "http" && !is_loopback {
            return Err(PublicOriginParseError::InsecureNonLoopback);
        }
        let port = match (scheme, parsed_port) {
            ("http", Some(80)) | ("https", Some(443)) => None,
            (_, port) => port,
        };

        let canonical = match port {
            Some(port) => format!("{scheme}://{canonical_host}:{port}"),
            None => format!("{scheme}://{canonical_host}"),
        };
        Ok(Self(canonical))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn is_https(&self) -> bool {
        self.0.starts_with("https://")
    }
}

impl Default for PublicOrigin {
    fn default() -> Self {
        Self(DEFAULT_PUBLIC_ORIGIN.to_owned())
    }
}

impl TryFrom<String> for PublicOrigin {
    type Error = PublicOriginParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<PublicOrigin> for String {
    fn from(value: PublicOrigin) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for PublicOrigin {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for PublicOrigin {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PublicOriginParseError {
    #[error("http.public_origin must use an exact http or https scheme")]
    InvalidScheme,
    #[error("http.public_origin must contain only scheme, host, and an optional non-zero port")]
    InvalidShape,
    #[error("http.public_origin must use https unless its host is loopback or localhost")]
    InsecureNonLoopback,
}

fn parse_origin_authority(
    authority: &str,
) -> Result<(String, bool, Option<u16>), PublicOriginParseError> {
    if let Some(ipv6) = authority.strip_prefix('[') {
        let closing = ipv6.find(']').ok_or(PublicOriginParseError::InvalidShape)?;
        let host = &ipv6[..closing];
        let remainder = &ipv6[closing + 1..];
        let address = host
            .parse::<std::net::Ipv6Addr>()
            .map_err(|_| PublicOriginParseError::InvalidShape)?;
        let port = parse_optional_origin_port(remainder)?;
        return Ok((format!("[{address}]"), address.is_loopback(), port));
    }

    let (host, port) = match authority.rsplit_once(':') {
        Some((host, encoded_port)) => {
            if host.contains(':') {
                return Err(PublicOriginParseError::InvalidShape);
            }
            (host, Some(parse_origin_port(encoded_port)?))
        }
        None => (authority, None),
    };
    if host.is_empty() || !host.is_ascii() {
        return Err(PublicOriginParseError::InvalidShape);
    }
    if let Ok(address) = host.parse::<IpAddr>() {
        if !address.is_ipv4() {
            return Err(PublicOriginParseError::InvalidShape);
        }
        return Ok((address.to_string(), address.is_loopback(), port));
    }
    if !valid_dns_name(host) {
        return Err(PublicOriginParseError::InvalidShape);
    }
    let canonical_host = host.to_ascii_lowercase();
    let is_loopback = canonical_host == "localhost";
    Ok((canonical_host, is_loopback, port))
}

fn parse_optional_origin_port(remainder: &str) -> Result<Option<u16>, PublicOriginParseError> {
    if remainder.is_empty() {
        return Ok(None);
    }
    let encoded = remainder
        .strip_prefix(':')
        .ok_or(PublicOriginParseError::InvalidShape)?;
    parse_origin_port(encoded).map(Some)
}

fn parse_origin_port(value: &str) -> Result<u16, PublicOriginParseError> {
    let port = value
        .parse::<u16>()
        .map_err(|_| PublicOriginParseError::InvalidShape)?;
    if port == 0 {
        return Err(PublicOriginParseError::InvalidShape);
    }
    Ok(port)
}

fn valid_dns_name(value: &str) -> bool {
    if value.is_empty() || value.len() > 253 || value.ends_with('.') {
        return false;
    }
    if value.contains('.')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || byte == b'.')
    {
        return false;
    }
    value.split('.').all(|label| {
        !label.is_empty()
            && label.len() <= 63
            && !label.starts_with('-')
            && !label.ends_with('-')
            && label
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
    })
}

/// Canonical IP network that may supply trusted reverse-proxy forwarding headers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrustedProxyCidr {
    network: IpAddr,
    prefix_length: u8,
    encoded: String,
}

impl TrustedProxyCidr {
    pub fn parse(value: impl Into<String>) -> Result<Self, TrustedProxyCidrParseError> {
        let value = value.into();
        if value.trim() != value {
            return Err(TrustedProxyCidrParseError::InvalidFormat);
        }
        let (address, prefix) = value
            .split_once('/')
            .ok_or(TrustedProxyCidrParseError::InvalidFormat)?;
        if prefix.contains('/') {
            return Err(TrustedProxyCidrParseError::InvalidFormat);
        }
        let network = address
            .parse::<IpAddr>()
            .map_err(|_| TrustedProxyCidrParseError::InvalidFormat)?;
        let prefix_length = prefix
            .parse::<u8>()
            .map_err(|_| TrustedProxyCidrParseError::InvalidFormat)?;
        let canonical = match network {
            IpAddr::V4(address) => {
                if prefix_length == 0 || prefix_length > 32 {
                    return Err(TrustedProxyCidrParseError::InvalidPrefixLength);
                }
                let bits = u32::from(address);
                let mask = ipv4_mask(prefix_length);
                if bits & mask != bits {
                    return Err(TrustedProxyCidrParseError::HostBitsSet);
                }
                IpAddr::V4(address)
            }
            IpAddr::V6(address) => {
                if prefix_length == 0 || prefix_length > 128 {
                    return Err(TrustedProxyCidrParseError::InvalidPrefixLength);
                }
                let bits = u128::from(address);
                let mask = ipv6_mask(prefix_length);
                if bits & mask != bits {
                    return Err(TrustedProxyCidrParseError::HostBitsSet);
                }
                IpAddr::V6(address)
            }
        };
        Ok(Self {
            network: canonical,
            prefix_length,
            encoded: format!("{canonical}/{prefix_length}"),
        })
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.encoded
    }

    #[must_use]
    pub fn contains(&self, candidate: IpAddr) -> bool {
        match (self.network, candidate) {
            (IpAddr::V4(network), IpAddr::V4(candidate)) => {
                let mask = ipv4_mask(self.prefix_length);
                u32::from(candidate) & mask == u32::from(network)
            }
            (IpAddr::V6(network), IpAddr::V6(candidate)) => {
                let mask = ipv6_mask(self.prefix_length);
                u128::from(candidate) & mask == u128::from(network)
            }
            _ => false,
        }
    }
}

impl TryFrom<String> for TrustedProxyCidr {
    type Error = TrustedProxyCidrParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<TrustedProxyCidr> for String {
    fn from(value: TrustedProxyCidr) -> Self {
        value.encoded
    }
}

impl<'de> Deserialize<'de> for TrustedProxyCidr {
    fn deserialize<Deserializer>(deserializer: Deserializer) -> Result<Self, Deserializer::Error>
    where
        Deserializer: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TrustedProxyCidrParseError {
    #[error("trusted proxy CIDR must contain a canonical IP address and prefix length")]
    InvalidFormat,
    #[error(
        "trusted proxy CIDR prefix length must be non-zero and within the address-family range"
    )]
    InvalidPrefixLength,
    #[error("trusted proxy CIDR network address has host bits set")]
    HostBitsSet,
}

const fn ipv4_mask(prefix_length: u8) -> u32 {
    if prefix_length == 0 {
        0
    } else {
        u32::MAX << (32 - prefix_length)
    }
}

const fn ipv6_mask(prefix_length: u8) -> u128 {
    if prefix_length == 0 {
        0
    } else {
        u128::MAX << (128 - prefix_length)
    }
}

#[derive(Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MasterConfig {
    pub http: HttpConfig,
    pub database: DatabaseConfig,
    pub secrets: SecretsConfig,
    pub bootstrap: BootstrapConfig,
    pub auth: AuthConfig,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct HttpConfig {
    pub listen: SocketAddr,
    pub public_origin: PublicOrigin,
    pub trusted_proxy_cidrs: Vec<TrustedProxyCidr>,
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            listen: SocketAddr::from(([127, 0, 0, 1], 8080)),
            public_origin: PublicOrigin::default(),
            trusted_proxy_cidrs: Vec::new(),
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
    pub previous_root_keys: Vec<PreviousRootKeyConfig>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PreviousRootKeyConfig {
    pub key_version: u32,
    pub path: std::path::PathBuf,
}

impl Default for SecretsConfig {
    fn default() -> Self {
        Self {
            root_key_file: std::path::PathBuf::from("nodecontroll.key"),
            setup_token_file: std::path::PathBuf::from("nodecontroll.setup-token"),
            previous_root_keys: Vec::new(),
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

/// Authentication/session resource ceilings and login throttling inputs.
///
/// These values configure server-side enforcement. There is intentionally no switch for insecure
/// browser cookies; non-loopback browser deployments are required to use an HTTPS public origin.
#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AuthConfig {
    pub session_idle_seconds: u64,
    pub session_absolute_seconds: u64,
    pub recent_auth_seconds: u64,
    pub login_window_seconds: u64,
    pub login_block_seconds: u64,
    pub login_account_limit: u32,
    pub login_ip_limit: u32,
    pub login_global_limit: u32,
    pub password_hash_concurrency: u32,
    pub digest_key_version: u32,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            session_idle_seconds: DEFAULT_SESSION_IDLE_SECONDS,
            session_absolute_seconds: DEFAULT_SESSION_ABSOLUTE_SECONDS,
            recent_auth_seconds: DEFAULT_RECENT_AUTH_SECONDS,
            login_window_seconds: DEFAULT_LOGIN_WINDOW_SECONDS,
            login_block_seconds: DEFAULT_LOGIN_BLOCK_SECONDS,
            login_account_limit: DEFAULT_LOGIN_ACCOUNT_LIMIT,
            login_ip_limit: DEFAULT_LOGIN_IP_LIMIT,
            login_global_limit: DEFAULT_LOGIN_GLOBAL_LIMIT,
            password_hash_concurrency: DEFAULT_PASSWORD_HASH_CONCURRENCY,
            digest_key_version: DEFAULT_DIGEST_KEY_VERSION,
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
    #[error("auth.session_idle_seconds must be between 60 and 86400")]
    InvalidSessionIdle,
    #[error(
        "auth.session_absolute_seconds must be between 300 and 2592000 and not shorter than the idle lifetime"
    )]
    InvalidSessionAbsolute,
    #[error(
        "auth.recent_auth_seconds must be between 60 and 3600 and not longer than the absolute session lifetime"
    )]
    InvalidRecentAuth,
    #[error("auth.login_window_seconds must be between 10 and 3600")]
    InvalidLoginWindow,
    #[error("auth.login_block_seconds must be between auth.login_window_seconds and 86400")]
    InvalidLoginBlock,
    #[error("auth login limits must be between 1 and 1000000 with account <= IP <= global")]
    InvalidLoginLimits,
    #[error("auth.password_hash_concurrency must be between 1 and 64")]
    InvalidPasswordHashConcurrency,
    #[error("auth.digest_key_version must be between 1 and 65535")]
    InvalidDigestKeyVersion,
    #[error(
        "secrets.previous_root_keys must contain at most three unique versions older than auth.digest_key_version"
    )]
    InvalidPreviousRootKeys,
}

pub fn load(path: Option<&Path>) -> Result<MasterConfig, ConfigError> {
    let mut builder = config::Config::builder()
        .set_default("http.listen", DEFAULT_LISTEN)?
        .set_default("http.public_origin", DEFAULT_PUBLIC_ORIGIN)?
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
        )?
        .set_default("auth.session_idle_seconds", DEFAULT_SESSION_IDLE_SECONDS)?
        .set_default(
            "auth.session_absolute_seconds",
            DEFAULT_SESSION_ABSOLUTE_SECONDS,
        )?
        .set_default("auth.recent_auth_seconds", DEFAULT_RECENT_AUTH_SECONDS)?
        .set_default("auth.login_window_seconds", DEFAULT_LOGIN_WINDOW_SECONDS)?
        .set_default("auth.login_block_seconds", DEFAULT_LOGIN_BLOCK_SECONDS)?
        .set_default("auth.login_account_limit", DEFAULT_LOGIN_ACCOUNT_LIMIT)?
        .set_default("auth.login_ip_limit", DEFAULT_LOGIN_IP_LIMIT)?
        .set_default("auth.login_global_limit", DEFAULT_LOGIN_GLOBAL_LIMIT)?
        .set_default(
            "auth.password_hash_concurrency",
            DEFAULT_PASSWORD_HASH_CONCURRENCY,
        )?
        .set_default("auth.digest_key_version", DEFAULT_DIGEST_KEY_VERSION)?;

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
    if !(MIN_SESSION_IDLE_SECONDS..=MAX_SESSION_IDLE_SECONDS)
        .contains(&loaded.auth.session_idle_seconds)
    {
        return Err(ConfigError::InvalidSessionIdle);
    }
    if !(MIN_SESSION_ABSOLUTE_SECONDS..=MAX_SESSION_ABSOLUTE_SECONDS)
        .contains(&loaded.auth.session_absolute_seconds)
        || loaded.auth.session_absolute_seconds < loaded.auth.session_idle_seconds
    {
        return Err(ConfigError::InvalidSessionAbsolute);
    }
    if !(MIN_RECENT_AUTH_SECONDS..=MAX_RECENT_AUTH_SECONDS)
        .contains(&loaded.auth.recent_auth_seconds)
        || loaded.auth.recent_auth_seconds > loaded.auth.session_absolute_seconds
    {
        return Err(ConfigError::InvalidRecentAuth);
    }
    if !(MIN_LOGIN_WINDOW_SECONDS..=MAX_LOGIN_WINDOW_SECONDS)
        .contains(&loaded.auth.login_window_seconds)
    {
        return Err(ConfigError::InvalidLoginWindow);
    }
    if !(1..=MAX_LOGIN_BLOCK_SECONDS).contains(&loaded.auth.login_block_seconds)
        || loaded.auth.login_block_seconds < loaded.auth.login_window_seconds
    {
        return Err(ConfigError::InvalidLoginBlock);
    }
    if !(1..=MAX_LOGIN_LIMIT).contains(&loaded.auth.login_account_limit)
        || !(1..=MAX_LOGIN_LIMIT).contains(&loaded.auth.login_ip_limit)
        || !(1..=MAX_LOGIN_LIMIT).contains(&loaded.auth.login_global_limit)
        || loaded.auth.login_account_limit > loaded.auth.login_ip_limit
        || loaded.auth.login_ip_limit > loaded.auth.login_global_limit
    {
        return Err(ConfigError::InvalidLoginLimits);
    }
    if !(1..=MAX_PASSWORD_HASH_CONCURRENCY).contains(&loaded.auth.password_hash_concurrency) {
        return Err(ConfigError::InvalidPasswordHashConcurrency);
    }
    if !(1..=MAX_DIGEST_KEY_VERSION).contains(&loaded.auth.digest_key_version) {
        return Err(ConfigError::InvalidDigestKeyVersion);
    }
    let mut previous_versions = std::collections::BTreeSet::new();
    if loaded.secrets.previous_root_keys.len() > 3
        || loaded.secrets.previous_root_keys.iter().any(|key| {
            !(1..loaded.auth.digest_key_version).contains(&key.key_version)
                || !previous_versions.insert(key.key_version)
        })
    {
        return Err(ConfigError::InvalidPreviousRootKeys);
    }

    Ok(loaded)
}

#[cfg(test)]
mod tests {
    use std::{fs, net::IpAddr, time::SystemTime};

    use super::{
        ConfigError, PublicOrigin, PublicOriginParseError, TrustedProxyCidr,
        TrustedProxyCidrParseError, load,
    };

    fn load_toml(label: &str, contents: &str) -> Result<super::MasterConfig, ConfigError> {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or_default();
        let path = std::env::temp_dir().join(format!(
            "nodecontroll-config-{label}-{}-{nonce}.toml",
            std::process::id()
        ));
        if let Err(error) = fs::write(&path, contents) {
            panic!("could not create config fixture: {error}");
        }
        let loaded = load(Some(&path));
        let _ = fs::remove_file(path);
        loaded
    }

    #[test]
    fn defaults_are_loopback_and_sqlite() {
        let loaded = load(None);
        assert!(loaded.is_ok());
        if let Ok(loaded) = loaded {
            assert!(loaded.http.listen.ip().is_loopback());
            assert_eq!(loaded.http.public_origin.as_str(), "http://127.0.0.1:8080");
            assert!(loaded.http.trusted_proxy_cidrs.is_empty());
            assert!(loaded.database.url().starts_with("sqlite:"));
            assert_eq!(loaded.database.redacted_url(), "[REDACTED]");
            assert_eq!(loaded.bootstrap.setup_token_ttl_seconds, 1_800);
            assert_eq!(loaded.auth.session_idle_seconds, 1_800);
            assert_eq!(loaded.auth.session_absolute_seconds, 86_400);
            assert_eq!(loaded.auth.recent_auth_seconds, 300);
            assert_eq!(loaded.auth.login_window_seconds, 300);
            assert_eq!(loaded.auth.login_block_seconds, 900);
            assert_eq!(loaded.auth.login_account_limit, 5);
            assert_eq!(loaded.auth.login_ip_limit, 50);
            assert_eq!(loaded.auth.login_global_limit, 10_000);
            assert_eq!(loaded.auth.password_hash_concurrency, 4);
            assert_eq!(loaded.auth.digest_key_version, 1);
            assert!(loaded.secrets.previous_root_keys.is_empty());
        }
    }

    #[test]
    fn public_origin_is_canonical_and_https_is_required_off_loopback() {
        for (value, canonical, is_https) in [
            ("http://localhost", "http://localhost", false),
            ("http://127.0.0.1:8080", "http://127.0.0.1:8080", false),
            ("http://[::1]:8080", "http://[::1]:8080", false),
            ("https://Example.COM", "https://example.com", true),
            ("http://localhost:80", "http://localhost", false),
            ("https://192.0.2.10:443", "https://192.0.2.10", true),
        ] {
            let parsed = PublicOrigin::parse(value);
            assert_eq!(parsed.as_ref().map(PublicOrigin::as_str), Ok(canonical));
            assert_eq!(
                parsed.as_ref().map(|origin| origin.is_https()),
                Ok(is_https)
            );
        }

        for insecure in [
            "http://example.com",
            "http://localhost.example",
            "http://192.0.2.10:8080",
        ] {
            assert_eq!(
                PublicOrigin::parse(insecure),
                Err(PublicOriginParseError::InsecureNonLoopback)
            );
        }
        for invalid in [
            "HTTP://localhost",
            "https://user@example.com",
            "https://example.com/",
            "https://example.com/path",
            "https://example.com?query=true",
            "https://example.com#fragment",
            "https://example.com:0",
            "https://127.0.0.999",
            " https://example.com",
        ] {
            assert!(matches!(
                PublicOrigin::parse(invalid),
                Err(PublicOriginParseError::InvalidScheme | PublicOriginParseError::InvalidShape)
            ));
        }
    }

    #[test]
    fn trusted_proxy_cidrs_are_canonical_and_match_only_their_address_family() {
        let ipv4 = TrustedProxyCidr::parse("10.0.0.0/8");
        assert_eq!(
            ipv4.as_ref().map(TrustedProxyCidr::as_str),
            Ok("10.0.0.0/8")
        );
        if let Ok(ipv4) = ipv4 {
            assert!(ipv4.contains(IpAddr::from([10, 20, 30, 40])));
            assert!(!ipv4.contains(IpAddr::from([11, 0, 0, 1])));
            assert!(!ipv4.contains(IpAddr::V6(std::net::Ipv6Addr::UNSPECIFIED)));
        }

        let ipv6 = TrustedProxyCidr::parse("2001:db8::/32");
        assert_eq!(
            ipv6.as_ref().map(TrustedProxyCidr::as_str),
            Ok("2001:db8::/32")
        );
        if let Ok(ipv6) = ipv6 {
            let member = "2001:db8:1::1".parse::<IpAddr>();
            assert!(member.is_ok_and(|address| ipv6.contains(address)));
        }

        assert_eq!(
            TrustedProxyCidr::parse("10.0.0.1/8"),
            Err(TrustedProxyCidrParseError::HostBitsSet)
        );
        assert_eq!(
            TrustedProxyCidr::parse("10.0.0.0/33"),
            Err(TrustedProxyCidrParseError::InvalidPrefixLength)
        );
        assert_eq!(
            TrustedProxyCidr::parse("0.0.0.0/0"),
            Err(TrustedProxyCidrParseError::InvalidPrefixLength)
        );
        assert_eq!(
            TrustedProxyCidr::parse("::/0"),
            Err(TrustedProxyCidrParseError::InvalidPrefixLength)
        );
        assert_eq!(
            TrustedProxyCidr::parse("not-an-ip/24"),
            Err(TrustedProxyCidrParseError::InvalidFormat)
        );
    }

    #[test]
    fn http_security_inputs_deserialize_from_file() {
        let loaded = load_toml(
            "http-security",
            "[http]\npublic_origin = 'https://panel.example.com:8443'\ntrusted_proxy_cidrs = ['10.0.0.0/8', '2001:db8::/32']\n",
        );
        assert!(loaded.is_ok());
        if let Ok(loaded) = loaded {
            assert_eq!(
                loaded.http.public_origin.as_str(),
                "https://panel.example.com:8443"
            );
            let cidrs = loaded
                .http
                .trusted_proxy_cidrs
                .iter()
                .map(TrustedProxyCidr::as_str)
                .collect::<Vec<_>>();
            assert_eq!(cidrs, ["10.0.0.0/8", "2001:db8::/32"]);
        }
    }

    #[test]
    fn unknown_file_keys_are_rejected() {
        let loaded = load_toml(
            "unknown",
            "[http]\nlisten = '127.0.0.1:8080'\nunknown = true\n",
        );
        assert!(matches!(loaded, Err(ConfigError::Load(_))));
    }

    #[test]
    fn setup_capability_ttl_is_bounded() {
        let loaded = load_toml(
            "bootstrap-ttl",
            "[bootstrap]\nsetup_token_ttl_seconds = 3601\n",
        );
        assert!(matches!(loaded, Err(ConfigError::InvalidSetupTokenTtl)));
    }

    #[test]
    fn authentication_bounds_and_relationships_are_enforced() {
        let valid_minimums = load_toml(
            "auth-valid-minimums",
            "[auth]\nsession_idle_seconds = 60\nsession_absolute_seconds = 300\nlogin_window_seconds = 10\nlogin_block_seconds = 10\nlogin_account_limit = 1\nlogin_ip_limit = 1\nlogin_global_limit = 1\ndigest_key_version = 1\n",
        );
        assert!(valid_minimums.is_ok());

        assert!(matches!(
            load_toml("auth-idle", "[auth]\nsession_idle_seconds = 59\n"),
            Err(ConfigError::InvalidSessionIdle)
        ));
        assert!(matches!(
            load_toml(
                "auth-absolute",
                "[auth]\nsession_idle_seconds = 3600\nsession_absolute_seconds = 3599\n"
            ),
            Err(ConfigError::InvalidSessionAbsolute)
        ));
        assert!(matches!(
            load_toml("auth-recent-short", "[auth]\nrecent_auth_seconds = 59\n"),
            Err(ConfigError::InvalidRecentAuth)
        ));
        assert!(matches!(
            load_toml(
                "auth-recent-longer-than-session",
                "[auth]\nsession_idle_seconds = 60\nsession_absolute_seconds = 300\nrecent_auth_seconds = 301\n"
            ),
            Err(ConfigError::InvalidRecentAuth)
        ));
        assert!(matches!(
            load_toml("auth-window", "[auth]\nlogin_window_seconds = 9\n"),
            Err(ConfigError::InvalidLoginWindow)
        ));
        assert!(matches!(
            load_toml("auth-block", "[auth]\nlogin_block_seconds = 0\n"),
            Err(ConfigError::InvalidLoginBlock)
        ));
        assert!(matches!(
            load_toml(
                "auth-block-shorter-than-window",
                "[auth]\nlogin_window_seconds = 10\nlogin_block_seconds = 9\n"
            ),
            Err(ConfigError::InvalidLoginBlock)
        ));
        assert!(matches!(
            load_toml(
                "auth-limits",
                "[auth]\nlogin_account_limit = 6\nlogin_ip_limit = 5\nlogin_global_limit = 100\n"
            ),
            Err(ConfigError::InvalidLoginLimits)
        ));
        assert!(matches!(
            load_toml("auth-digest", "[auth]\ndigest_key_version = 0\n"),
            Err(ConfigError::InvalidDigestKeyVersion)
        ));
        assert!(matches!(
            load_toml(
                "auth-password-hash-concurrency",
                "[auth]\npassword_hash_concurrency = 0\n"
            ),
            Err(ConfigError::InvalidPasswordHashConcurrency)
        ));
    }

    #[test]
    fn previous_root_key_ring_is_finite_unique_and_older_than_current() {
        let valid = load_toml(
            "previous-root-keys",
            "[auth]\ndigest_key_version = 3\n[[secrets.previous_root_keys]]\nkey_version = 1\npath = 'old-v1.key'\n[[secrets.previous_root_keys]]\nkey_version = 2\npath = 'old-v2.key'\n",
        );
        assert!(valid.is_ok());
        assert!(matches!(
            load_toml(
                "duplicate-previous-root-key",
                "[auth]\ndigest_key_version = 3\n[[secrets.previous_root_keys]]\nkey_version = 1\npath = 'a.key'\n[[secrets.previous_root_keys]]\nkey_version = 1\npath = 'b.key'\n"
            ),
            Err(ConfigError::InvalidPreviousRootKeys)
        ));
        assert!(matches!(
            load_toml(
                "current-as-previous-root-key",
                "[auth]\ndigest_key_version = 2\n[[secrets.previous_root_keys]]\nkey_version = 2\npath = 'current.key'\n"
            ),
            Err(ConfigError::InvalidPreviousRootKeys)
        ));
    }
}
