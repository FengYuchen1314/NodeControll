use std::{
    env,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use async_trait::async_trait;
use nodecontroll_api::{
    AppState, BootstrapCommand, BootstrapOutcome, BootstrapServiceError, FoundationProbe,
    ProbeError,
};
use nodecontroll_config::MasterConfig;
use nodecontroll_domain::{
    EntityId, Instance, InstanceName, PrincipalLabel, Revision, SubscriptionBehaviorSettings,
    UserAccount, UserRole, Username,
};
use nodecontroll_identity::{PasswordError, PasswordService, SetupCapability};
use nodecontroll_persistence::{ConnectionSettings, Database};
use nodecontroll_secrets::EnvelopeCipher;
use time::OffsetDateTime;
use tokio::{net::TcpListener, signal, sync::Mutex};
use tracing_subscriber::EnvFilter;

struct DatabaseProbe {
    database: Database,
    cipher: EnvelopeCipher,
    password_service: PasswordService,
    setup_capability: Option<SetupCapability>,
    last_bootstrap_attempt: Mutex<Option<Instant>>,
}

#[async_trait]
impl FoundationProbe for DatabaseProbe {
    async fn database_ready(&self) -> Result<(), ProbeError> {
        self.database
            .probe()
            .await
            .map_err(|_| ProbeError::database_unavailable())
    }

    async fn is_initialized(&self) -> Result<bool, ProbeError> {
        self.database
            .is_initialized()
            .await
            .map_err(|error| match error {
                nodecontroll_persistence::PersistenceError::InconsistentBootstrapState => {
                    ProbeError::bootstrap_state_inconsistent()
                }
                _ => ProbeError::database_unavailable(),
            })
    }

    async fn secret_ready(&self) -> Result<(), ProbeError> {
        self.cipher
            .canary()
            .map_err(|_| ProbeError::secret_unavailable())
    }

    async fn initialize(
        &self,
        command: BootstrapCommand,
    ) -> Result<BootstrapOutcome, BootstrapServiceError> {
        if self
            .database
            .is_initialized()
            .await
            .map_err(map_bootstrap_state_read_error)?
        {
            return Err(BootstrapServiceError::AlreadyInitialized);
        }
        let capability = self
            .setup_capability
            .as_ref()
            .ok_or(BootstrapServiceError::CapabilityInvalid)?;
        if !capability.authorize(command.setup_token.as_str()) {
            return Err(BootstrapServiceError::CapabilityInvalid);
        }
        let instance_name = InstanceName::parse(command.instance_name)
            .map_err(|_| BootstrapServiceError::InvalidInstanceName)?;
        let username = Username::parse(command.username)
            .map_err(|_| BootstrapServiceError::InvalidUsername)?;
        self.password_service
            .validate(command.password.as_str())
            .map_err(map_password_error)?;
        let mut attempt_guard = self.last_bootstrap_attempt.lock().await;
        if attempt_guard
            .as_ref()
            .is_some_and(|last_attempt| last_attempt.elapsed() < Duration::from_secs(2))
        {
            return Err(BootstrapServiceError::RateLimited);
        }
        *attempt_guard = Some(Instant::now());
        let password_service = self.password_service.clone();
        let password = command.password;
        let password_hash =
            tokio::task::spawn_blocking(move || password_service.hash(password.as_str()))
                .await
                .map_err(|_| BootstrapServiceError::Unavailable)?
                .map_err(map_password_error)?;
        let created_at_ms =
            i64::try_from(OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000)
                .map_err(|_| BootstrapServiceError::Unavailable)?;
        let instance_id = EntityId::new();
        let owner_id = EntityId::new();
        let principal_label = PrincipalLabel::parse(format!("usr_{owner_id}"))
            .map_err(|_| BootstrapServiceError::Unavailable)?;
        let instance = Instance {
            id: instance_id,
            public_id: EntityId::new(),
            name: instance_name,
            created_at_ms,
            revision: Revision::initial(),
        };
        let owner = UserAccount {
            id: owner_id,
            username,
            password_hash,
            role: UserRole::Owner,
            principal_label,
            force_password_change: false,
            revision: Revision::initial(),
            created_at_ms,
        };
        let persisted_instance_id = self
            .database
            .bootstrap_control_plane(&instance, &owner, &SubscriptionBehaviorSettings::default())
            .await
            .map_err(|error| match error {
                nodecontroll_persistence::PersistenceError::AlreadyInitialized => {
                    BootstrapServiceError::AlreadyInitialized
                }
                nodecontroll_persistence::PersistenceError::IdentityConflict => {
                    BootstrapServiceError::IdentityConflict
                }
                nodecontroll_persistence::PersistenceError::InconsistentBootstrapState => {
                    BootstrapServiceError::InconsistentState
                }
                _ => BootstrapServiceError::Unavailable,
            })?;
        capability.consume();
        Ok(BootstrapOutcome {
            instance_id: persisted_instance_id.to_string(),
            owner_id: owner_id.to_string(),
        })
    }
}

fn map_password_error(error: PasswordError) -> BootstrapServiceError {
    match error {
        PasswordError::TooShort | PasswordError::TooLong | PasswordError::ControlCharacter => {
            BootstrapServiceError::InvalidPassword
        }
        _ => BootstrapServiceError::Unavailable,
    }
}

fn map_bootstrap_state_read_error(
    error: nodecontroll_persistence::PersistenceError,
) -> BootstrapServiceError {
    match error {
        nodecontroll_persistence::PersistenceError::InconsistentBootstrapState => {
            BootstrapServiceError::InconsistentState
        }
        _ => BootstrapServiceError::Unavailable,
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    init_telemetry();

    let config_path = env::var_os("NODECONTROLL_CONFIG").map(PathBuf::from);
    let config = nodecontroll_config::load(config_path.as_deref())?;

    if env::args().any(|argument| argument == "--check-config") {
        let database_engine = Database::validate_url(config.database.url())?;
        println!(
            "{}",
            serde_json::json!({
                "status": "ok",
                "database_engine": database_engine.as_str(),
                "database_url": config.database.redacted_url(),
                "listen": config.http.listen.to_string()
            })
        );
        return Ok(());
    }

    let database = connect_database(&config).await?;
    database
        .migrate()
        .await
        .context("database migration failed")?;
    let cipher = EnvelopeCipher::from_key_file(&config.secrets.root_key_file, 1)
        .context("secret root key could not be loaded")?;
    cipher
        .canary()
        .context("secret root key canary failed before HTTP startup")?;
    let password_service =
        PasswordService::recommended().context("Argon2id password parameters are invalid")?;
    let initialized = database
        .is_initialized()
        .await
        .context("bootstrap state could not be read before HTTP startup")?;
    let setup_capability = if initialized {
        None
    } else {
        Some(
            SetupCapability::from_file(
                &config.secrets.setup_token_file,
                Duration::from_secs(config.bootstrap.setup_token_ttl_seconds),
            )
            .context("setup capability could not be loaded")?,
        )
    };

    let listener = TcpListener::bind(config.http.listen)
        .await
        .with_context(|| format!("could not bind {}", config.http.listen))?;
    let state = AppState::new(
        env!("CARGO_PKG_VERSION"),
        Arc::new(DatabaseProbe {
            database: database.clone(),
            cipher,
            password_service,
            setup_capability,
            last_bootstrap_attempt: Mutex::new(None),
        }),
    )?;

    tracing::info!(
        address = %config.http.listen,
        database_engine = database.engine().as_str(),
        version = env!("CARGO_PKG_VERSION"),
        "master listening"
    );
    axum::serve(listener, nodecontroll_api::router(state))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("HTTP server failed")
}

async fn connect_database(config: &MasterConfig) -> Result<Database> {
    Database::connect(
        config.database.url(),
        ConnectionSettings {
            max_connections: config.database.max_connections,
            acquire_timeout: Duration::from_millis(config.database.acquire_timeout_ms),
            statement_timeout: Duration::from_millis(config.database.statement_timeout_ms),
            lock_timeout: Duration::from_millis(config.database.lock_timeout_ms),
        },
    )
    .await
    .context("database connection failed")
}

fn init_telemetry() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .json()
        .try_init();
}

async fn shutdown_signal() {
    let ctrl_c = signal::ctrl_c();
    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut signal) = signal::unix::signal(signal::unix::SignalKind::terminate()) {
            signal.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

#[cfg(test)]
mod tests {
    use nodecontroll_api::BootstrapServiceError;
    use nodecontroll_persistence::PersistenceError;

    use super::map_bootstrap_state_read_error;

    #[test]
    fn bootstrap_precheck_preserves_inconsistent_state() {
        assert_eq!(
            map_bootstrap_state_read_error(PersistenceError::InconsistentBootstrapState),
            BootstrapServiceError::InconsistentState
        );
        assert_eq!(
            map_bootstrap_state_read_error(PersistenceError::InvalidTimestamp),
            BootstrapServiceError::Unavailable
        );
    }
}
