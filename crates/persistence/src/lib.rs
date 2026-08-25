use std::{str::FromStr, time::Duration};

use nodecontroll_domain::{
    EntityId, Instance, InstanceName, Revision, SubscriptionBehaviorSettings, UserAccount,
};
use sqlx::{
    PgPool, Row, SqlitePool,
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPoolOptions},
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use thiserror::Error;

static SQLITE_MIGRATOR: Migrator = sqlx::migrate!("./migrations/sqlite");
static POSTGRES_MIGRATOR: Migrator = sqlx::migrate!("./migrations/postgres");
const SUBSCRIPTION_SETTINGS_KEY: &str = "subscription.behavior";
const SUBSCRIPTION_SETTINGS_SCHEMA: i32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootstrapState {
    Uninitialized,
    LegacyNeedsOwner,
    Ready,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DatabaseEngine {
    Sqlite,
    Postgres,
}

impl DatabaseEngine {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sqlite => "sqlite",
            Self::Postgres => "postgres",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ConnectionSettings {
    pub max_connections: u32,
    pub acquire_timeout: Duration,
    pub statement_timeout: Duration,
    pub lock_timeout: Duration,
}

#[derive(Clone)]
pub enum Database {
    Sqlite(SqlitePool),
    Postgres(PgPool),
}

impl Database {
    pub fn validate_url(url: &str) -> Result<DatabaseEngine, PersistenceError> {
        if url.starts_with("sqlite:") {
            SqliteConnectOptions::from_str(url)?;
            Ok(DatabaseEngine::Sqlite)
        } else if url.starts_with("postgres://") || url.starts_with("postgresql://") {
            PgConnectOptions::from_str(url)?;
            Ok(DatabaseEngine::Postgres)
        } else {
            Err(PersistenceError::UnsupportedDatabaseUrl)
        }
    }

    pub async fn connect(
        url: &str,
        settings: ConnectionSettings,
    ) -> Result<Self, PersistenceError> {
        let engine = Self::validate_url(url)?;
        if engine == DatabaseEngine::Sqlite {
            let options = SqliteConnectOptions::from_str(url)?
                .create_if_missing(true)
                .foreign_keys(true)
                .journal_mode(SqliteJournalMode::Wal)
                .synchronous(SqliteSynchronous::Normal)
                .busy_timeout(settings.lock_timeout);
            let pool = SqlitePoolOptions::new()
                .max_connections(1)
                .acquire_timeout(settings.acquire_timeout)
                .connect_with(options)
                .await?;
            Ok(Self::Sqlite(pool))
        } else {
            let statement_timeout = duration_millis_string(settings.statement_timeout)?;
            let lock_timeout = duration_millis_string(settings.lock_timeout)?;
            let options = PgConnectOptions::from_str(url)?;
            let pool = PgPoolOptions::new()
                .max_connections(settings.max_connections)
                .acquire_timeout(settings.acquire_timeout)
                .after_connect(move |connection, _metadata| {
                    let statement_timeout = statement_timeout.clone();
                    let lock_timeout = lock_timeout.clone();
                    Box::pin(async move {
                        sqlx::query("SELECT set_config('statement_timeout', $1, false)")
                            .bind(statement_timeout)
                            .execute(&mut *connection)
                            .await?;
                        sqlx::query("SELECT set_config('lock_timeout', $1, false)")
                            .bind(lock_timeout)
                            .execute(&mut *connection)
                            .await?;
                        Ok(())
                    })
                })
                .connect_with(options)
                .await?;
            Ok(Self::Postgres(pool))
        }
    }

    #[must_use]
    pub const fn engine(&self) -> DatabaseEngine {
        match self {
            Self::Sqlite(_) => DatabaseEngine::Sqlite,
            Self::Postgres(_) => DatabaseEngine::Postgres,
        }
    }

    pub async fn migrate(&self) -> Result<(), PersistenceError> {
        match self {
            Self::Sqlite(pool) => SQLITE_MIGRATOR.run(pool).await?,
            Self::Postgres(pool) => POSTGRES_MIGRATOR.run(pool).await?,
        }
        Ok(())
    }

    pub async fn probe(&self) -> Result<(), PersistenceError> {
        match self {
            Self::Sqlite(pool) => {
                sqlx::query("SELECT 1").execute(pool).await?;
            }
            Self::Postgres(pool) => {
                sqlx::query("SELECT 1").execute(pool).await?;
            }
        }
        Ok(())
    }

    pub async fn bootstrap_state(&self) -> Result<BootstrapState, PersistenceError> {
        match self {
            Self::Sqlite(pool) => {
                let row: Option<(String, Option<String>, bool, bool, bool)> = sqlx::query_as(
                    "SELECT status, instance_id, EXISTS(SELECT 1 FROM instances), EXISTS(SELECT 1 FROM users), EXISTS(SELECT 1 FROM users WHERE role='owner' AND status='active' AND deleted_at_ms IS NULL) FROM control_plane_bootstrap WHERE singleton_key=1",
                )
                .fetch_optional(pool)
                .await?;
                let row = row.ok_or(PersistenceError::InconsistentBootstrapState)?;
                classify_bootstrap_record(&row.0, row.1.is_some(), (row.2, row.3, row.4))
            }
            Self::Postgres(pool) => {
                let row: Option<(String, Option<uuid::Uuid>, bool, bool, bool)> = sqlx::query_as(
                    "SELECT status, instance_id, EXISTS(SELECT 1 FROM instances), EXISTS(SELECT 1 FROM users), EXISTS(SELECT 1 FROM users WHERE role='owner' AND status='active' AND deleted_at_ms IS NULL) FROM control_plane_bootstrap WHERE singleton_key=1",
                )
                .fetch_optional(pool)
                .await?;
                let row = row.ok_or(PersistenceError::InconsistentBootstrapState)?;
                classify_bootstrap_record(&row.0, row.1.is_some(), (row.2, row.3, row.4))
            }
        }
    }

    pub async fn is_initialized(&self) -> Result<bool, PersistenceError> {
        Ok(self.bootstrap_state().await? == BootstrapState::Ready)
    }

    pub async fn bootstrap_control_plane(
        &self,
        instance: &Instance,
        owner: &UserAccount,
        settings: &SubscriptionBehaviorSettings,
    ) -> Result<EntityId, PersistenceError> {
        if instance.created_at_ms < 0 || owner.created_at_ms < 0 {
            return Err(PersistenceError::InvalidTimestamp);
        }
        let instance_revision = i64::try_from(instance.revision.value())
            .map_err(|_| PersistenceError::RevisionOutOfRange)?;
        let owner_revision = i64::try_from(owner.revision.value())
            .map_err(|_| PersistenceError::RevisionOutOfRange)?;
        let settings_json = serde_json::to_string(settings)?;

        match self {
            Self::Sqlite(pool) => {
                bootstrap_sqlite(
                    pool,
                    instance,
                    owner,
                    &settings_json,
                    instance_revision,
                    owner_revision,
                )
                .await
            }
            Self::Postgres(pool) => {
                bootstrap_postgres(
                    pool,
                    instance,
                    owner,
                    &settings_json,
                    instance_revision,
                    owner_revision,
                )
                .await
            }
        }
    }

    pub async fn active_owner_count(&self) -> Result<i64, PersistenceError> {
        match self {
            Self::Sqlite(pool) => Ok(sqlx::query_scalar(
                "SELECT COUNT(*) FROM users WHERE role='owner' AND status='active' AND deleted_at_ms IS NULL",
            )
            .fetch_one(pool)
            .await?),
            Self::Postgres(pool) => Ok(sqlx::query_scalar(
                "SELECT COUNT(*) FROM users WHERE role='owner' AND status='active' AND deleted_at_ms IS NULL",
            )
            .fetch_one(pool)
            .await?),
        }
    }

    pub async fn instance(&self) -> Result<Option<Instance>, PersistenceError> {
        match self {
            Self::Sqlite(pool) => {
                let row = sqlx::query(
                    "SELECT id, public_id, name, created_at_ms, revision FROM instances LIMIT 1",
                )
                .fetch_optional(pool)
                .await?;
                row.map(decode_sqlite_instance).transpose()
            }
            Self::Postgres(pool) => {
                let row = sqlx::query(
                    "SELECT id, public_id, name, created_at_ms, revision FROM instances LIMIT 1",
                )
                .fetch_optional(pool)
                .await?;
                row.map(decode_postgres_instance).transpose()
            }
        }
    }

    pub async fn subscription_settings(
        &self,
        instance_id: EntityId,
    ) -> Result<Option<(SubscriptionBehaviorSettings, Revision)>, PersistenceError> {
        let row: Option<(String, i64)> = match self {
            Self::Sqlite(pool) => sqlx::query_as(
                "SELECT value_json, revision FROM instance_settings WHERE instance_id=? AND key=? AND schema_version=?",
            )
            .bind(instance_id.to_string())
            .bind(SUBSCRIPTION_SETTINGS_KEY)
            .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
            .fetch_optional(pool)
            .await?,
            Self::Postgres(pool) => sqlx::query_as(
                "SELECT value_json::text, revision FROM instance_settings WHERE instance_id=$1 AND key=$2 AND schema_version=$3",
            )
            .bind(instance_id.into_uuid())
            .bind(SUBSCRIPTION_SETTINGS_KEY)
            .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
            .fetch_optional(pool)
            .await?,
        };
        row.map(|(json, revision)| {
            let revision =
                u64::try_from(revision).map_err(|_| PersistenceError::RevisionOutOfRange)?;
            Ok((serde_json::from_str(&json)?, Revision::from_value(revision)))
        })
        .transpose()
    }

    pub async fn save_subscription_settings(
        &self,
        instance_id: EntityId,
        settings: &SubscriptionBehaviorSettings,
        expected_revision: Option<Revision>,
        actor_id: EntityId,
        updated_at_ms: i64,
    ) -> Result<Revision, PersistenceError> {
        if updated_at_ms < 0 {
            return Err(PersistenceError::InvalidTimestamp);
        }
        let json = serde_json::to_string(settings)?;
        match expected_revision {
            None => {
                let result = match self {
                    Self::Sqlite(pool) => sqlx::query(
                        "INSERT INTO instance_settings (instance_id,key,schema_version,value_json,revision,updated_by,updated_at_ms) VALUES (?,?,?,?,0,?,?) ON CONFLICT(instance_id,key) DO NOTHING",
                    )
                    .bind(instance_id.to_string())
                    .bind(SUBSCRIPTION_SETTINGS_KEY)
                    .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
                    .bind(json)
                    .bind(actor_id.to_string())
                    .bind(updated_at_ms)
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected()),
                    Self::Postgres(pool) => sqlx::query(
                        "INSERT INTO instance_settings (instance_id,key,schema_version,value_json,revision,updated_by,updated_at_ms) VALUES ($1,$2,$3,$4::jsonb,0,$5,$6) ON CONFLICT(instance_id,key) DO NOTHING",
                    )
                    .bind(instance_id.into_uuid())
                    .bind(SUBSCRIPTION_SETTINGS_KEY)
                    .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
                    .bind(json)
                    .bind(actor_id.into_uuid())
                    .bind(updated_at_ms)
                    .execute(pool)
                    .await
                    .map(|result| result.rows_affected()),
                };
                match result {
                    Ok(1) => Ok(Revision::initial()),
                    Ok(_) => Err(PersistenceError::RevisionConflict),
                    Err(error) => Err(PersistenceError::Sql(error)),
                }
            }
            Some(expected) => {
                let next = expected
                    .next()
                    .map_err(|_| PersistenceError::RevisionOutOfRange)?;
                let expected_value = i64::try_from(expected.value())
                    .map_err(|_| PersistenceError::RevisionOutOfRange)?;
                let next_value = i64::try_from(next.value())
                    .map_err(|_| PersistenceError::RevisionOutOfRange)?;
                let affected = match self {
                    Self::Sqlite(pool) => sqlx::query(
                        "UPDATE instance_settings SET value_json=?,revision=?,updated_by=?,updated_at_ms=? WHERE instance_id=? AND key=? AND schema_version=? AND revision=?",
                    )
                    .bind(json)
                    .bind(next_value)
                    .bind(actor_id.to_string())
                    .bind(updated_at_ms)
                    .bind(instance_id.to_string())
                    .bind(SUBSCRIPTION_SETTINGS_KEY)
                    .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
                    .bind(expected_value)
                    .execute(pool)
                    .await?
                    .rows_affected(),
                    Self::Postgres(pool) => sqlx::query(
                        "UPDATE instance_settings SET value_json=$1::jsonb,revision=$2,updated_by=$3,updated_at_ms=$4 WHERE instance_id=$5 AND key=$6 AND schema_version=$7 AND revision=$8",
                    )
                    .bind(json)
                    .bind(next_value)
                    .bind(actor_id.into_uuid())
                    .bind(updated_at_ms)
                    .bind(instance_id.into_uuid())
                    .bind(SUBSCRIPTION_SETTINGS_KEY)
                    .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
                    .bind(expected_value)
                    .execute(pool)
                    .await?
                    .rows_affected(),
                };
                if affected == 1 {
                    Ok(next)
                } else {
                    Err(PersistenceError::RevisionConflict)
                }
            }
        }
    }
}

async fn bootstrap_sqlite(
    pool: &SqlitePool,
    instance: &Instance,
    owner: &UserAccount,
    settings_json: &str,
    instance_revision: i64,
    owner_revision: i64,
) -> Result<EntityId, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let claimed = sqlx::query(
        "UPDATE control_plane_bootstrap SET status=status WHERE singleton_key=1 AND status='pending'",
    )
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    let record: Option<(String, Option<String>, bool, bool, bool)> = sqlx::query_as(
        "SELECT status, instance_id, EXISTS(SELECT 1 FROM instances), EXISTS(SELECT 1 FROM users), EXISTS(SELECT 1 FROM users WHERE role='owner' AND status='active' AND deleted_at_ms IS NULL) FROM control_plane_bootstrap WHERE singleton_key=1",
    )
    .fetch_optional(&mut *transaction)
    .await?;
    let state = record
        .as_ref()
        .ok_or(PersistenceError::InconsistentBootstrapState)
        .and_then(|row| {
            classify_bootstrap_record(&row.0, row.1.is_some(), (row.2, row.3, row.4))
        })?;
    if claimed != 1 {
        return if state == BootstrapState::Ready {
            Err(PersistenceError::AlreadyInitialized)
        } else {
            Err(PersistenceError::InconsistentBootstrapState)
        };
    }
    let (instance_id, needs_default_settings) = match state {
        BootstrapState::Uninitialized => {
            let result = sqlx::query(
                "INSERT INTO instances (singleton_key,id,name,public_id,created_at_ms,revision) VALUES (1,?,?,?,?,?)",
            )
            .bind(instance.id.to_string())
            .bind(instance.name.as_str())
            .bind(instance.public_id.to_string())
            .bind(instance.created_at_ms)
            .bind(instance_revision)
            .execute(&mut *transaction)
            .await;
            match result {
                Ok(_) => (instance.id, true),
                Err(error) => return Err(PersistenceError::Sql(error)),
            }
        }
        BootstrapState::LegacyNeedsOwner => {
            let stored_id = record
                .as_ref()
                .and_then(|row| row.1.as_deref())
                .ok_or(PersistenceError::InconsistentBootstrapState)?;
            let stored_id = uuid::Uuid::parse_str(stored_id)?;
            let stored_id = EntityId::from_uuid(stored_id);
            let stored_settings: Option<(i32, String)> = sqlx::query_as(
                "SELECT schema_version,value_json FROM instance_settings WHERE instance_id=? AND key=?",
            )
            .bind(stored_id.to_string())
            .bind(SUBSCRIPTION_SETTINGS_KEY)
            .fetch_optional(&mut *transaction)
            .await?;
            (stored_id, legacy_needs_default_settings(stored_settings)?)
        }
        BootstrapState::Ready => return Err(PersistenceError::AlreadyInitialized),
    };
    let owner_result = sqlx::query(
        "INSERT INTO users (id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms) VALUES (?,?,?,?,?,'active',?,?,?, ?,NULL) ON CONFLICT(username_norm) WHERE deleted_at_ms IS NULL DO NOTHING",
    )
    .bind(owner.id.to_string())
    .bind(owner.username.as_str())
    .bind(owner.username.normalized())
    .bind(owner.password_hash.as_str())
    .bind(owner.role.as_str())
    .bind(owner.principal_label.as_str())
    .bind(owner.force_password_change)
    .bind(owner_revision)
    .bind(owner.created_at_ms)
    .execute(&mut *transaction)
    .await;
    match owner_result {
        Ok(result) if result.rows_affected() == 1 => {}
        Ok(_) => return Err(PersistenceError::IdentityConflict),
        Err(error) => return Err(PersistenceError::Sql(error)),
    }
    if needs_default_settings {
        sqlx::query(
            "INSERT INTO instance_settings (instance_id,key,schema_version,value_json,revision,updated_by,updated_at_ms) VALUES (?,?,?,?,0,?,?)",
        )
        .bind(instance_id.to_string())
        .bind(SUBSCRIPTION_SETTINGS_KEY)
        .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
        .bind(settings_json)
        .bind(owner.id.to_string())
        .bind(owner.created_at_ms)
        .execute(&mut *transaction)
        .await?;
    }
    let finalized = sqlx::query(
        "UPDATE control_plane_bootstrap SET status='ready',instance_id=? WHERE singleton_key=1 AND status='pending'",
    )
    .bind(instance_id.to_string())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if finalized != 1 {
        return Err(PersistenceError::InconsistentBootstrapState);
    }
    transaction.commit().await?;
    Ok(instance_id)
}

async fn bootstrap_postgres(
    pool: &PgPool,
    instance: &Instance,
    owner: &UserAccount,
    settings_json: &str,
    instance_revision: i64,
    owner_revision: i64,
) -> Result<EntityId, PersistenceError> {
    let mut transaction = pool.begin().await?;
    let claimed = sqlx::query(
        "UPDATE control_plane_bootstrap SET status=status WHERE singleton_key=1 AND status='pending'",
    )
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    let record: Option<(String, Option<uuid::Uuid>, bool, bool, bool)> = sqlx::query_as(
        "SELECT status, instance_id, EXISTS(SELECT 1 FROM instances), EXISTS(SELECT 1 FROM users), EXISTS(SELECT 1 FROM users WHERE role='owner' AND status='active' AND deleted_at_ms IS NULL) FROM control_plane_bootstrap WHERE singleton_key=1",
    )
    .fetch_optional(&mut *transaction)
    .await?;
    let state = record
        .as_ref()
        .ok_or(PersistenceError::InconsistentBootstrapState)
        .and_then(|row| {
            classify_bootstrap_record(&row.0, row.1.is_some(), (row.2, row.3, row.4))
        })?;
    if claimed != 1 {
        return if state == BootstrapState::Ready {
            Err(PersistenceError::AlreadyInitialized)
        } else {
            Err(PersistenceError::InconsistentBootstrapState)
        };
    }
    let (instance_id, needs_default_settings) = match state {
        BootstrapState::Uninitialized => {
            let result = sqlx::query(
                "INSERT INTO instances (singleton_key,id,name,public_id,created_at_ms,revision) VALUES (1,$1,$2,$3,$4,$5)",
            )
            .bind(instance.id.into_uuid())
            .bind(instance.name.as_str())
            .bind(instance.public_id.into_uuid())
            .bind(instance.created_at_ms)
            .bind(instance_revision)
            .execute(&mut *transaction)
            .await;
            match result {
                Ok(_) => (instance.id, true),
                Err(error) => return Err(PersistenceError::Sql(error)),
            }
        }
        BootstrapState::LegacyNeedsOwner => {
            let stored_id = record
                .as_ref()
                .and_then(|row| row.1)
                .ok_or(PersistenceError::InconsistentBootstrapState)?;
            let stored_id = EntityId::from_uuid(stored_id);
            let stored_settings: Option<(i32, String)> = sqlx::query_as(
                "SELECT schema_version,value_json::text FROM instance_settings WHERE instance_id=$1 AND key=$2 FOR UPDATE",
            )
            .bind(stored_id.into_uuid())
            .bind(SUBSCRIPTION_SETTINGS_KEY)
            .fetch_optional(&mut *transaction)
            .await?;
            (stored_id, legacy_needs_default_settings(stored_settings)?)
        }
        BootstrapState::Ready => return Err(PersistenceError::AlreadyInitialized),
    };
    let owner_result = sqlx::query(
        "INSERT INTO users (id,username,username_norm,password_hash,role,status,principal_label,force_password_change,revision,created_at_ms,deleted_at_ms) VALUES ($1,$2,$3,$4,$5,'active',$6,$7,$8,$9,NULL) ON CONFLICT(username_norm) WHERE deleted_at_ms IS NULL DO NOTHING",
    )
    .bind(owner.id.into_uuid())
    .bind(owner.username.as_str())
    .bind(owner.username.normalized())
    .bind(owner.password_hash.as_str())
    .bind(owner.role.as_str())
    .bind(owner.principal_label.as_str())
    .bind(owner.force_password_change)
    .bind(owner_revision)
    .bind(owner.created_at_ms)
    .execute(&mut *transaction)
    .await;
    match owner_result {
        Ok(result) if result.rows_affected() == 1 => {}
        Ok(_) => return Err(PersistenceError::IdentityConflict),
        Err(error) => return Err(PersistenceError::Sql(error)),
    }
    if needs_default_settings {
        sqlx::query(
            "INSERT INTO instance_settings (instance_id,key,schema_version,value_json,revision,updated_by,updated_at_ms) VALUES ($1,$2,$3,$4::jsonb,0,$5,$6)",
        )
        .bind(instance_id.into_uuid())
        .bind(SUBSCRIPTION_SETTINGS_KEY)
        .bind(SUBSCRIPTION_SETTINGS_SCHEMA)
        .bind(settings_json)
        .bind(owner.id.into_uuid())
        .bind(owner.created_at_ms)
        .execute(&mut *transaction)
        .await?;
    }
    let finalized = sqlx::query(
        "UPDATE control_plane_bootstrap SET status='ready',instance_id=$1 WHERE singleton_key=1 AND status='pending'",
    )
    .bind(instance_id.into_uuid())
    .execute(&mut *transaction)
    .await?
    .rows_affected();
    if finalized != 1 {
        return Err(PersistenceError::InconsistentBootstrapState);
    }
    transaction.commit().await?;
    Ok(instance_id)
}

fn legacy_needs_default_settings(stored: Option<(i32, String)>) -> Result<bool, PersistenceError> {
    let Some((schema_version, value_json)) = stored else {
        return Ok(true);
    };
    if schema_version != SUBSCRIPTION_SETTINGS_SCHEMA
        || serde_json::from_str::<SubscriptionBehaviorSettings>(&value_json).is_err()
    {
        return Err(PersistenceError::InconsistentBootstrapState);
    }
    Ok(false)
}

fn decode_sqlite_instance(row: sqlx::sqlite::SqliteRow) -> Result<Instance, PersistenceError> {
    let id = uuid::Uuid::parse_str(row.try_get("id")?)?;
    let public_id = uuid::Uuid::parse_str(row.try_get("public_id")?)?;
    decode_instance(
        id,
        public_id,
        row.try_get("name")?,
        row.try_get("created_at_ms")?,
        row.try_get("revision")?,
    )
}

fn decode_postgres_instance(row: sqlx::postgres::PgRow) -> Result<Instance, PersistenceError> {
    decode_instance(
        row.try_get("id")?,
        row.try_get("public_id")?,
        row.try_get("name")?,
        row.try_get("created_at_ms")?,
        row.try_get("revision")?,
    )
}

fn decode_instance(
    id: uuid::Uuid,
    public_id: uuid::Uuid,
    name: String,
    created_at_ms: i64,
    revision: i64,
) -> Result<Instance, PersistenceError> {
    let revision = u64::try_from(revision).map_err(|_| PersistenceError::RevisionOutOfRange)?;
    Ok(Instance {
        id: EntityId::from_uuid(id),
        public_id: EntityId::from_uuid(public_id),
        name: InstanceName::parse(name)?,
        created_at_ms,
        revision: Revision::from_value(revision),
    })
}

fn classify_bootstrap_record(
    status: &str,
    has_instance_id: bool,
    (has_instance, has_user, has_active_owner): (bool, bool, bool),
) -> Result<BootstrapState, PersistenceError> {
    match (
        status,
        has_instance_id,
        has_instance,
        has_user,
        has_active_owner,
    ) {
        ("pending", false, false, false, false) => Ok(BootstrapState::Uninitialized),
        ("pending", true, true, false, false) => Ok(BootstrapState::LegacyNeedsOwner),
        ("ready", true, true, true, true) => Ok(BootstrapState::Ready),
        _ => Err(PersistenceError::InconsistentBootstrapState),
    }
}

fn duration_millis_string(duration: Duration) -> Result<String, PersistenceError> {
    let millis =
        u64::try_from(duration.as_millis()).map_err(|_| PersistenceError::TimeoutTooLarge)?;
    Ok(format!("{millis}ms"))
}

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("unsupported database URL scheme")]
    UnsupportedDatabaseUrl,
    #[error("database timeout exceeds the supported range")]
    TimeoutTooLarge,
    #[error("timestamp must be non-negative")]
    InvalidTimestamp,
    #[error("revision does not fit the database representation")]
    RevisionOutOfRange,
    #[error("the control-plane instance has already been initialized")]
    AlreadyInitialized,
    #[error("stored bootstrap records do not form a recoverable control-plane state")]
    InconsistentBootstrapState,
    #[error("the requested owner identity conflicts with an existing identity")]
    IdentityConflict,
    #[error("the setting revision does not match the stored revision")]
    RevisionConflict,
    #[error("stored instance name is invalid: {0}")]
    InvalidInstanceName(#[from] nodecontroll_domain::InstanceNameError),
    #[error("stored UUID is invalid: {0}")]
    InvalidUuid(#[from] uuid::Error),
    #[error("database operation failed: {0}")]
    Sql(#[from] sqlx::Error),
    #[error("stored setting JSON is invalid: {0}")]
    Json(#[from] serde_json::Error),
    #[error("database migration failed: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, time::Duration};

    use nodecontroll_domain::{
        ClientCompatibilityMode, EntityId, ExternalSyncStrategy, Instance, InstanceName,
        PasswordHash, PrincipalLabel, Revision, SubscriptionBehaviorSettings, UserAccount,
        UserRole, Username,
    };
    use sqlx::{
        PgPool,
        postgres::{PgConnectOptions, PgPoolOptions},
    };

    use super::{BootstrapState, ConnectionSettings, Database, DatabaseEngine, PersistenceError};

    fn settings() -> ConnectionSettings {
        ConnectionSettings {
            max_connections: 4,
            acquire_timeout: Duration::from_secs(5),
            statement_timeout: Duration::from_secs(30),
            lock_timeout: Duration::from_secs(5),
        }
    }

    struct PostgresFixture {
        database: Database,
        admin: PgPool,
        schema: &'static str,
    }

    async fn isolated_postgres(
        url: &str,
        schema: &'static str,
    ) -> Result<PostgresFixture, sqlx::Error> {
        let admin = PgPoolOptions::new().max_connections(1).connect(url).await?;
        match schema {
            "nodecontroll_test_fresh" => {
                sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_fresh CASCADE")
                    .execute(&admin)
                    .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_fresh")
                    .execute(&admin)
                    .await?;
            }
            "nodecontroll_test_legacy" => {
                sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_legacy CASCADE")
                    .execute(&admin)
                    .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_legacy")
                    .execute(&admin)
                    .await?;
            }
            "nodecontroll_test_legacy_missing" => {
                sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_legacy_missing CASCADE")
                    .execute(&admin)
                    .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_legacy_missing")
                    .execute(&admin)
                    .await?;
            }
            "nodecontroll_test_migration_rollback" => {
                sqlx::query("DROP SCHEMA IF EXISTS nodecontroll_test_migration_rollback CASCADE")
                    .execute(&admin)
                    .await?;
                sqlx::query("CREATE SCHEMA nodecontroll_test_migration_rollback")
                    .execute(&admin)
                    .await?;
            }
            _ => panic!("unexpected PostgreSQL fixture schema"),
        }
        let options = PgConnectOptions::from_str(url)?.options([
            ("search_path", schema),
            ("statement_timeout", "30s"),
            ("lock_timeout", "5s"),
        ]);
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect_with(options)
            .await?;
        Ok(PostgresFixture {
            database: Database::Postgres(pool),
            admin,
            schema,
        })
    }

    impl PostgresFixture {
        async fn cleanup(self) -> Result<(), sqlx::Error> {
            if let Database::Postgres(pool) = self.database {
                pool.close().await;
            }
            match self.schema {
                "nodecontroll_test_fresh" => {
                    sqlx::query("DROP SCHEMA nodecontroll_test_fresh CASCADE")
                        .execute(&self.admin)
                        .await?;
                }
                "nodecontroll_test_legacy" => {
                    sqlx::query("DROP SCHEMA nodecontroll_test_legacy CASCADE")
                        .execute(&self.admin)
                        .await?;
                }
                "nodecontroll_test_legacy_missing" => {
                    sqlx::query("DROP SCHEMA nodecontroll_test_legacy_missing CASCADE")
                        .execute(&self.admin)
                        .await?;
                }
                "nodecontroll_test_migration_rollback" => {
                    sqlx::query("DROP SCHEMA nodecontroll_test_migration_rollback CASCADE")
                        .execute(&self.admin)
                        .await?;
                }
                _ => panic!("unexpected PostgreSQL fixture schema"),
            }
            self.admin.close().await;
            Ok(())
        }
    }

    fn fixture() -> Instance {
        let name = InstanceName::parse("Foundation contract");
        assert!(name.is_ok());
        Instance {
            id: EntityId::new(),
            public_id: EntityId::new(),
            name: name.unwrap_or_else(|_| unreachable!("checked above")),
            created_at_ms: 1_777_777_777_000,
            revision: Revision::initial(),
        }
    }

    fn owner_fixture() -> UserAccount {
        let username = Username::parse("InitialOwner");
        let password_hash = PasswordHash::parse(
            "$argon2id$v=19$m=19456,t=2,p=1$c29tZXNhbHQxMjM0NTY3OA$AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        );
        let principal_label = PrincipalLabel::parse(format!("owner-{}", EntityId::new()));
        assert!(username.is_ok());
        assert!(password_hash.is_ok());
        assert!(principal_label.is_ok());
        UserAccount {
            id: EntityId::new(),
            username: username.unwrap_or_else(|_| unreachable!("checked above")),
            password_hash: password_hash.unwrap_or_else(|_| unreachable!("checked above")),
            role: UserRole::Owner,
            principal_label: principal_label.unwrap_or_else(|_| unreachable!("checked above")),
            force_password_change: false,
            revision: Revision::initial(),
            created_at_ms: 1_777_777_777_000,
        }
    }

    async fn migrate_to_0001(database: &Database) {
        let result = match database {
            Database::Sqlite(pool) => super::SQLITE_MIGRATOR.run_to(1, pool).await,
            Database::Postgres(pool) => super::POSTGRES_MIGRATOR.run_to(1, pool).await,
        };
        assert!(result.is_ok());
    }

    async fn seed_v1_instance(
        database: &Database,
        instance: &Instance,
        settings: &SubscriptionBehaviorSettings,
    ) {
        let settings_json = serde_json::to_string(settings);
        assert!(settings_json.is_ok());
        let settings_json = settings_json.unwrap_or_else(|_| unreachable!("checked above"));
        match database {
            Database::Sqlite(pool) => {
                let inserted = sqlx::query(
                    "INSERT INTO instances (singleton_key,id,name,public_id,created_at_ms,revision) VALUES (1,?,?,?,?,0)",
                )
                .bind(instance.id.to_string())
                .bind(instance.name.as_str())
                .bind(instance.public_id.to_string())
                .bind(instance.created_at_ms)
                .execute(pool)
                .await;
                assert!(inserted.is_ok());
                let settings_inserted = sqlx::query(
                    "INSERT INTO instance_settings (instance_id,key,schema_version,value_json,revision,updated_by,updated_at_ms) VALUES (?,?,?,?,0,NULL,?)",
                )
                .bind(instance.id.to_string())
                .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                .bind(super::SUBSCRIPTION_SETTINGS_SCHEMA)
                .bind(settings_json)
                .bind(instance.created_at_ms)
                .execute(pool)
                .await;
                assert!(settings_inserted.is_ok());
            }
            Database::Postgres(pool) => {
                let inserted = sqlx::query(
                    "INSERT INTO instances (singleton_key,id,name,public_id,created_at_ms,revision) VALUES (1,$1,$2,$3,$4,0)",
                )
                .bind(instance.id.into_uuid())
                .bind(instance.name.as_str())
                .bind(instance.public_id.into_uuid())
                .bind(instance.created_at_ms)
                .execute(pool)
                .await;
                assert!(inserted.is_ok());
                let settings_inserted = sqlx::query(
                    "INSERT INTO instance_settings (instance_id,key,schema_version,value_json,revision,updated_by,updated_at_ms) VALUES ($1,$2,$3,$4::jsonb,0,NULL,$5)",
                )
                .bind(instance.id.into_uuid())
                .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                .bind(super::SUBSCRIPTION_SETTINGS_SCHEMA)
                .bind(settings_json)
                .bind(instance.created_at_ms)
                .execute(pool)
                .await;
                assert!(settings_inserted.is_ok());
            }
        }
    }

    async fn replace_legacy_setting(
        database: &Database,
        instance_id: EntityId,
        schema_version: i32,
        value_json: &str,
    ) {
        match database {
            Database::Sqlite(pool) => {
                let result = sqlx::query(
                    "UPDATE instance_settings SET schema_version=?,value_json=? WHERE instance_id=? AND key=?",
                )
                .bind(schema_version)
                .bind(value_json)
                .bind(instance_id.to_string())
                .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                .execute(pool)
                .await;
                assert!(matches!(result, Ok(result) if result.rows_affected() == 1));
            }
            Database::Postgres(pool) => {
                let result = sqlx::query(
                    "UPDATE instance_settings SET schema_version=$1,value_json=$2::jsonb WHERE instance_id=$3 AND key=$4",
                )
                .bind(schema_version)
                .bind(value_json)
                .bind(instance_id.into_uuid())
                .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                .execute(pool)
                .await;
                assert!(matches!(result, Ok(result) if result.rows_affected() == 1));
            }
        }
    }

    async fn install_owner_failure_trigger(database: &Database) {
        match database {
            Database::Sqlite(pool) => {
                assert!(sqlx::query("CREATE TRIGGER nodecontroll_test_fail_owner BEFORE INSERT ON users BEGIN SELECT RAISE(ABORT, 'forced owner failure'); END").execute(pool).await.is_ok());
            }
            Database::Postgres(pool) => {
                assert!(sqlx::query("CREATE OR REPLACE FUNCTION nodecontroll_test_fail_owner() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'forced owner failure'; END $$").execute(pool).await.is_ok());
                assert!(sqlx::query("CREATE TRIGGER nodecontroll_test_fail_owner BEFORE INSERT ON users FOR EACH ROW EXECUTE FUNCTION nodecontroll_test_fail_owner()").execute(pool).await.is_ok());
            }
        }
    }

    async fn remove_owner_failure_trigger(database: &Database) {
        match database {
            Database::Sqlite(pool) => {
                assert!(
                    sqlx::query("DROP TRIGGER nodecontroll_test_fail_owner")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
            Database::Postgres(pool) => {
                assert!(
                    sqlx::query("DROP TRIGGER nodecontroll_test_fail_owner ON users")
                        .execute(pool)
                        .await
                        .is_ok()
                );
                assert!(
                    sqlx::query("DROP FUNCTION nodecontroll_test_fail_owner()")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
        }
    }

    async fn settings_actor(
        database: &Database,
        instance_id: EntityId,
    ) -> Result<Option<String>, sqlx::Error> {
        match database {
            Database::Sqlite(pool) => {
                let value: Option<String> = sqlx::query_scalar(
                    "SELECT updated_by FROM instance_settings WHERE instance_id=? AND key=?",
                )
                .bind(instance_id.to_string())
                .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                .fetch_one(pool)
                .await?;
                Ok(value)
            }
            Database::Postgres(pool) => {
                let value: Option<uuid::Uuid> = sqlx::query_scalar(
                    "SELECT updated_by FROM instance_settings WHERE instance_id=$1 AND key=$2",
                )
                .bind(instance_id.into_uuid())
                .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                .fetch_one(pool)
                .await?;
                Ok(value.map(|id| id.to_string()))
            }
        }
    }

    async fn remove_bootstrap_latch(database: &Database) {
        match database {
            Database::Sqlite(pool) => {
                let result =
                    sqlx::query("DELETE FROM control_plane_bootstrap WHERE singleton_key=1")
                        .execute(pool)
                        .await;
                assert!(matches!(result, Ok(result) if result.rows_affected() == 1));
            }
            Database::Postgres(pool) => {
                let result =
                    sqlx::query("DELETE FROM control_plane_bootstrap WHERE singleton_key=1")
                        .execute(pool)
                        .await;
                assert!(matches!(result, Ok(result) if result.rows_affected() == 1));
            }
        }
    }

    async fn legacy_upgrade_contract(database: Database) {
        migrate_to_0001(&database).await;
        let legacy_instance = fixture();
        let legacy_settings = SubscriptionBehaviorSettings {
            external_sync: ExternalSyncStrategy::Manual,
            silent_mode: true,
            short_links_enabled: false,
            client_compatibility: ClientCompatibilityMode::Legacy,
            response_headers_enabled: true,
            info_node_enabled: true,
        };
        seed_v1_instance(&database, &legacy_instance, &legacy_settings).await;
        assert!(database.migrate().await.is_ok());
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::LegacyNeedsOwner)
        ));
        assert!(matches!(database.is_initialized().await, Ok(false)));
        let legacy_owner = owner_fixture();

        let valid_settings_json = serde_json::to_string(&legacy_settings);
        assert!(valid_settings_json.is_ok());
        let valid_settings_json =
            valid_settings_json.unwrap_or_else(|_| unreachable!("checked above"));
        replace_legacy_setting(&database, legacy_instance.id, 2, &valid_settings_json).await;
        assert!(matches!(
            database
                .bootstrap_control_plane(
                    &fixture(),
                    &legacy_owner,
                    &SubscriptionBehaviorSettings::default(),
                )
                .await,
            Err(PersistenceError::InconsistentBootstrapState)
        ));
        assert!(matches!(database.active_owner_count().await, Ok(0)));
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::LegacyNeedsOwner)
        ));

        replace_legacy_setting(
            &database,
            legacy_instance.id,
            super::SUBSCRIPTION_SETTINGS_SCHEMA,
            r#"{"unexpected":true}"#,
        )
        .await;
        assert!(matches!(
            database
                .bootstrap_control_plane(
                    &fixture(),
                    &legacy_owner,
                    &SubscriptionBehaviorSettings::default(),
                )
                .await,
            Err(PersistenceError::InconsistentBootstrapState)
        ));
        assert!(matches!(database.active_owner_count().await, Ok(0)));
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::LegacyNeedsOwner)
        ));
        replace_legacy_setting(
            &database,
            legacy_instance.id,
            super::SUBSCRIPTION_SETTINGS_SCHEMA,
            &valid_settings_json,
        )
        .await;

        install_owner_failure_trigger(&database).await;
        assert!(matches!(
            database
                .bootstrap_control_plane(
                    &fixture(),
                    &legacy_owner,
                    &SubscriptionBehaviorSettings::default(),
                )
                .await,
            Err(PersistenceError::Sql(_))
        ));
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::LegacyNeedsOwner)
        ));
        assert!(matches!(database.instance().await, Ok(Some(found)) if found == legacy_instance));
        assert!(matches!(
            database.subscription_settings(legacy_instance.id).await,
            Ok(Some((found, revision))) if found == legacy_settings && revision == Revision::initial()
        ));
        assert!(matches!(database.active_owner_count().await, Ok(0)));
        remove_owner_failure_trigger(&database).await;
        assert!(matches!(
            database
                .bootstrap_control_plane(
                    &fixture(),
                    &legacy_owner,
                    &SubscriptionBehaviorSettings::default(),
                )
                .await,
            Ok(id) if id == legacy_instance.id
        ));
        assert!(matches!(
            database.subscription_settings(legacy_instance.id).await,
            Ok(Some((found, revision))) if found == legacy_settings && revision == Revision::initial()
        ));
        assert!(matches!(database.active_owner_count().await, Ok(1)));
        assert!(matches!(database.is_initialized().await, Ok(true)));
        assert!(matches!(
            settings_actor(&database, legacy_instance.id).await,
            Ok(None)
        ));
    }

    async fn legacy_missing_settings_contract(database: Database) {
        migrate_to_0001(&database).await;
        let legacy_instance = fixture();
        let legacy_settings = SubscriptionBehaviorSettings::default();
        seed_v1_instance(&database, &legacy_instance, &legacy_settings).await;
        match &database {
            Database::Sqlite(pool) => {
                let deleted =
                    sqlx::query("DELETE FROM instance_settings WHERE instance_id=? AND key=?")
                        .bind(legacy_instance.id.to_string())
                        .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                        .execute(pool)
                        .await;
                assert!(matches!(deleted, Ok(result) if result.rows_affected() == 1));
            }
            Database::Postgres(pool) => {
                let deleted =
                    sqlx::query("DELETE FROM instance_settings WHERE instance_id=$1 AND key=$2")
                        .bind(legacy_instance.id.into_uuid())
                        .bind(super::SUBSCRIPTION_SETTINGS_KEY)
                        .execute(pool)
                        .await;
                assert!(matches!(deleted, Ok(result) if result.rows_affected() == 1));
            }
        }
        assert!(database.migrate().await.is_ok());
        let owner = owner_fixture();
        assert!(matches!(
            database
                .bootstrap_control_plane(&fixture(), &owner, &legacy_settings)
                .await,
            Ok(id) if id == legacy_instance.id
        ));
        assert!(matches!(
            database.subscription_settings(legacy_instance.id).await,
            Ok(Some((found, revision))) if found == legacy_settings && revision == Revision::initial()
        ));
        assert!(matches!(
            settings_actor(&database, legacy_instance.id).await,
            Ok(Some(found)) if found == owner.id.to_string()
        ));
    }

    async fn migration_atomicity_contract(database: Database) {
        migrate_to_0001(&database).await;
        match &database {
            Database::Sqlite(pool) => {
                assert!(
                    sqlx::query("CREATE TABLE control_plane_bootstrap (dummy INTEGER)")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
            Database::Postgres(pool) => {
                assert!(
                    sqlx::query("CREATE TABLE control_plane_bootstrap (dummy INTEGER)")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
        }
        assert!(database.migrate().await.is_err());
        match &database {
            Database::Sqlite(pool) => {
                let version: Result<Option<i64>, _> =
                    sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations")
                        .fetch_one(pool)
                        .await;
                assert!(matches!(version, Ok(Some(1))));
                let users: Result<i64, _> = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='users'",
                )
                .fetch_one(pool)
                .await;
                assert!(matches!(users, Ok(0)));
                assert!(
                    sqlx::query("DROP TABLE control_plane_bootstrap")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
            Database::Postgres(pool) => {
                let version: Result<Option<i64>, _> =
                    sqlx::query_scalar("SELECT MAX(version) FROM _sqlx_migrations")
                        .fetch_one(pool)
                        .await;
                assert!(matches!(version, Ok(Some(1))));
                let users_absent: Result<bool, _> =
                    sqlx::query_scalar("SELECT to_regclass('users') IS NULL")
                        .fetch_one(pool)
                        .await;
                assert!(matches!(users_absent, Ok(true)));
                assert!(
                    sqlx::query("DROP TABLE control_plane_bootstrap")
                        .execute(pool)
                        .await
                        .is_ok()
                );
            }
        }
        assert!(database.migrate().await.is_ok());
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::Uninitialized)
        ));
    }

    async fn repository_contract(database: Database) {
        assert!(database.migrate().await.is_ok());
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::Uninitialized)
        ));
        install_owner_failure_trigger(&database).await;
        assert!(matches!(
            database
                .bootstrap_control_plane(
                    &fixture(),
                    &owner_fixture(),
                    &SubscriptionBehaviorSettings::default(),
                )
                .await,
            Err(PersistenceError::Sql(_))
        ));
        assert!(matches!(database.instance().await, Ok(None)));
        assert!(matches!(database.active_owner_count().await, Ok(0)));
        assert!(matches!(
            database.bootstrap_state().await,
            Ok(BootstrapState::Uninitialized)
        ));
        remove_owner_failure_trigger(&database).await;

        let first_instance = fixture();
        let first_owner = owner_fixture();
        let second_instance = fixture();
        let second_owner = owner_fixture();
        let initial_settings = SubscriptionBehaviorSettings::default();
        let second_initial_settings = SubscriptionBehaviorSettings::default();
        let (first_result, second_result) = tokio::join!(
            database.bootstrap_control_plane(&first_instance, &first_owner, &initial_settings),
            database.bootstrap_control_plane(
                &second_instance,
                &second_owner,
                &second_initial_settings
            )
        );
        let (instance, owner) = match (first_result, second_result) {
            (Ok(id), Err(PersistenceError::AlreadyInitialized)) if id == first_instance.id => {
                (first_instance, first_owner)
            }
            (Err(PersistenceError::AlreadyInitialized), Ok(id)) if id == second_instance.id => {
                (second_instance, second_owner)
            }
            outcome => panic!("unexpected concurrent bootstrap outcome: {outcome:?}"),
        };
        assert!(matches!(database.is_initialized().await, Ok(true)));
        assert!(matches!(database.active_owner_count().await, Ok(1)));
        assert!(matches!(database.instance().await, Ok(Some(found)) if found == instance));
        assert!(matches!(
            database.subscription_settings(instance.id).await,
            Ok(Some((found, revision))) if found == initial_settings && revision == Revision::initial()
        ));
        let settings = SubscriptionBehaviorSettings {
            external_sync: ExternalSyncStrategy::OnRequest,
            silent_mode: true,
            short_links_enabled: true,
            client_compatibility: ClientCompatibilityMode::Legacy,
            response_headers_enabled: true,
            info_node_enabled: false,
        };
        let updated = SubscriptionBehaviorSettings {
            silent_mode: false,
            ..settings.clone()
        };
        let next = Revision::from_value(1);
        assert!(matches!(
            database
                .save_subscription_settings(
                    instance.id,
                    &updated,
                    Some(Revision::initial()),
                    owner.id,
                    1_777_777_777_002,
                )
                .await,
            Ok(revision) if revision == next
        ));
        assert!(matches!(
            database
                .save_subscription_settings(
                    instance.id,
                    &settings,
                    Some(Revision::initial()),
                    owner.id,
                    1_777_777_777_003,
                )
                .await,
            Err(PersistenceError::RevisionConflict)
        ));
        assert!(matches!(
            database.subscription_settings(instance.id).await,
            Ok(Some((found, revision))) if found == updated && revision == next
        ));
        assert!(matches!(
            settings_actor(&database, instance.id).await,
            Ok(Some(found)) if found == owner.id.to_string()
        ));
        assert!(matches!(
            database
                .bootstrap_control_plane(
                    &fixture(),
                    &owner_fixture(),
                    &SubscriptionBehaviorSettings::default(),
                )
                .await,
            Err(PersistenceError::AlreadyInitialized)
        ));
        assert!(matches!(database.instance().await, Ok(Some(found)) if found == instance));
        assert!(matches!(database.active_owner_count().await, Ok(1)));
        assert!(matches!(
            database.subscription_settings(instance.id).await,
            Ok(Some((found, revision))) if found == updated && revision == next
        ));
        assert!(matches!(
            settings_actor(&database, instance.id).await,
            Ok(Some(found)) if found == owner.id.to_string()
        ));
        remove_bootstrap_latch(&database).await;
        assert!(matches!(
            database.bootstrap_state().await,
            Err(PersistenceError::InconsistentBootstrapState)
        ));
    }

    #[tokio::test]
    async fn sqlite_repository_contract() {
        assert!(matches!(
            Database::validate_url("sqlite::memory:"),
            Ok(DatabaseEngine::Sqlite)
        ));
        let database = Database::connect("sqlite::memory:", settings()).await;
        assert!(database.is_ok());
        if let Ok(database) = database {
            assert_eq!(database.engine(), DatabaseEngine::Sqlite);
            repository_contract(database).await;
        }
        let legacy_database = Database::connect("sqlite::memory:", settings()).await;
        assert!(legacy_database.is_ok());
        if let Ok(database) = legacy_database {
            legacy_upgrade_contract(database).await;
        }
        let legacy_missing_database = Database::connect("sqlite::memory:", settings()).await;
        assert!(legacy_missing_database.is_ok());
        if let Ok(database) = legacy_missing_database {
            legacy_missing_settings_contract(database).await;
        }
        let rollback_database = Database::connect("sqlite::memory:", settings()).await;
        assert!(rollback_database.is_ok());
        if let Ok(database) = rollback_database {
            migration_atomicity_contract(database).await;
        }
    }

    #[tokio::test]
    async fn postgres_repository_contract() {
        let url = match std::env::var("NODECONTROLL_TEST_POSTGRES_URL") {
            Ok(url) => url,
            Err(_) => {
                panic!("NODECONTROLL_TEST_POSTGRES_URL is required for the persistence test gate")
            }
        };
        let fresh = isolated_postgres(&url, "nodecontroll_test_fresh").await;
        assert!(fresh.is_ok());
        if let Ok(fresh) = fresh {
            assert_eq!(fresh.database.engine(), DatabaseEngine::Postgres);
            repository_contract(fresh.database.clone()).await;
            assert!(fresh.cleanup().await.is_ok());
        }
        let legacy = isolated_postgres(&url, "nodecontroll_test_legacy").await;
        assert!(legacy.is_ok());
        if let Ok(legacy) = legacy {
            legacy_upgrade_contract(legacy.database.clone()).await;
            assert!(legacy.cleanup().await.is_ok());
        }
        let legacy_missing = isolated_postgres(&url, "nodecontroll_test_legacy_missing").await;
        assert!(legacy_missing.is_ok());
        if let Ok(legacy_missing) = legacy_missing {
            legacy_missing_settings_contract(legacy_missing.database.clone()).await;
            assert!(legacy_missing.cleanup().await.is_ok());
        }
        let rollback = isolated_postgres(&url, "nodecontroll_test_migration_rollback").await;
        assert!(rollback.is_ok());
        if let Ok(rollback) = rollback {
            migration_atomicity_contract(rollback.database.clone()).await;
            assert!(rollback.cleanup().await.is_ok());
        }
    }

    #[test]
    fn unsupported_database_scheme_is_rejected_without_connecting() {
        assert!(matches!(
            Database::validate_url("mysql://localhost/nodecontroll"),
            Err(PersistenceError::UnsupportedDatabaseUrl)
        ));
    }
}
