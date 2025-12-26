use crate::config;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};
use std::cmp::max;
use std::time::Duration;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub async fn init_database() -> anyhow::Result<DatabaseConnection> {
    let app_config = config::AppConfig::get();
    let db_config = app_config.database();
    let pool_config = app_config.pool();
    let mut options = ConnectOptions::new(db_config.database_url());

    let num_cpus = num_cpus::get() as u32;
    options
        .min_connections(max(num_cpus * 4, pool_config.min_connections()))
        .max_connections(max(num_cpus * 8, pool_config.max_connections()))
        .connect_timeout(Duration::from_secs(pool_config.connect_timeout()))
        .acquire_timeout(Duration::from_secs(pool_config.read_timeout())) // read timeout
        .idle_timeout(Duration::from_secs(pool_config.idle_timeout()))
        .max_lifetime(Duration::from_secs(3600 * pool_config.max_lifetime()))
        .sqlx_logging(false)
        .set_schema_search_path(db_config.schema());

    let db_connection = Database::connect(options).await?;
    db_connection.ping().await?;
    tracing::info!("Database connection through pool is established");
    print_db_version(&db_connection).await?;

    Ok(db_connection)
}

pub fn init_logger() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(false),
        )
        .init();
}

async fn print_db_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version = db
        .query_one(Statement::from_string(
            DbBackend::Postgres,
            "SELECT version()",
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to get database version!"))?;
    tracing::info!(
        "Database version is: {}",
        version.try_get_by_index::<String>(0)?
    );

    Ok(())
}
