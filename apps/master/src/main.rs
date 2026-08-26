use std::{env, net::SocketAddr, path::PathBuf, time::Duration};

use anyhow::{Context, Result};
use nodecontroll_api::{AppState, web_security::WebSecurityPolicy};
use nodecontroll_application::{AuthPolicy, ControlPlaneApplication};
use nodecontroll_config::{AuthConfig, MasterConfig};
use nodecontroll_identity::{PasswordService, SetupCapability};
use nodecontroll_persistence::{ConnectionSettings, Database};
use nodecontroll_secrets::EnvelopeCipher;
use tokio::{net::TcpListener, signal};
use tracing_subscriber::EnvFilter;

const DUMMY_PASSWORD: &str = "nodecontroll-dummy-password-v1";
const SESSION_TOUCH_INTERVAL_MAX_SECONDS: u64 = 60;
const SESSION_TOUCH_INTERVAL_DIVISOR: u64 = 4;

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
    let cipher = EnvelopeCipher::from_key_file(
        &config.secrets.root_key_file,
        config.auth.digest_key_version,
    )
    .context("secret root key could not be loaded")?;
    cipher
        .canary()
        .context("secret root key canary failed before HTTP startup")?;
    let password_service =
        PasswordService::recommended().context("Argon2id password parameters are invalid")?;
    let dummy_password_service = password_service.clone();
    let dummy_password_hash =
        tokio::task::spawn_blocking(move || dummy_password_service.hash(DUMMY_PASSWORD))
            .await
            .context("dummy password PHC task failed")?
            .context("dummy password PHC could not be generated")?;
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

    let listen = config.http.listen;
    let database_engine = database.engine();
    let control_plane = ControlPlaneApplication::new(
        database,
        cipher,
        password_service,
        dummy_password_hash,
        setup_capability,
        auth_policy(&config.auth),
    )
    .context("authentication policy is invalid")?;
    let web_security = WebSecurityPolicy::new(
        config.http.public_origin.clone(),
        config.http.trusted_proxy_cidrs.clone(),
    );
    let state = AppState::new(
        env!("CARGO_PKG_VERSION"),
        control_plane,
        web_security,
        config.auth.session_absolute_seconds,
    )?;
    let listener = TcpListener::bind(listen)
        .await
        .with_context(|| format!("could not bind {listen}"))?;

    tracing::info!(
        address = %listen,
        database_engine = database_engine.as_str(),
        version = env!("CARGO_PKG_VERSION"),
        "master listening"
    );
    axum::serve(
        listener,
        nodecontroll_api::router(state).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .context("HTTP server failed")
}

fn auth_policy(config: &AuthConfig) -> AuthPolicy {
    let session_touch_interval_seconds = (config.session_idle_seconds
        / SESSION_TOUCH_INTERVAL_DIVISOR)
        .clamp(1, SESSION_TOUCH_INTERVAL_MAX_SECONDS);
    AuthPolicy {
        session_idle: Duration::from_secs(config.session_idle_seconds),
        session_absolute: Duration::from_secs(config.session_absolute_seconds),
        session_touch_interval: Duration::from_secs(session_touch_interval_seconds),
        login_window: Duration::from_secs(config.login_window_seconds),
        login_block: Duration::from_secs(config.login_block_seconds),
        login_account_limit: config.login_account_limit,
        login_ip_limit: config.login_ip_limit,
        login_global_limit: config.login_global_limit,
        password_hash_concurrency: usize::try_from(config.password_hash_concurrency).unwrap_or(1),
    }
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
    use std::time::Duration;

    use nodecontroll_config::AuthConfig;

    use super::auth_policy;

    #[test]
    fn auth_policy_maps_validated_config_values() {
        let config = AuthConfig {
            session_idle_seconds: 60,
            session_absolute_seconds: 300,
            login_window_seconds: 10,
            login_block_seconds: 20,
            login_account_limit: 3,
            login_ip_limit: 30,
            login_global_limit: 300,
            ..AuthConfig::default()
        };

        let policy = auth_policy(&config);

        assert_eq!(policy.session_idle, Duration::from_secs(60));
        assert_eq!(policy.session_absolute, Duration::from_secs(300));
        assert_eq!(policy.session_touch_interval, Duration::from_secs(15));
        assert_eq!(policy.login_window, Duration::from_secs(10));
        assert_eq!(policy.login_block, Duration::from_secs(20));
        assert_eq!(policy.login_account_limit, 3);
        assert_eq!(policy.login_ip_limit, 30);
        assert_eq!(policy.login_global_limit, 300);
        assert_eq!(policy.password_hash_concurrency, 4);
    }

    #[test]
    fn default_idle_session_caps_touch_writes_at_one_minute() {
        let policy = auth_policy(&AuthConfig::default());

        assert_eq!(policy.session_touch_interval, Duration::from_secs(60));
    }
}
