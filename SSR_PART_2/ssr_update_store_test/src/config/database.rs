use serde::Deserialize;

/// Database configuration for PostgreSQL connection.
///
/// All fields are optional with sensible defaults for development environment.
/// Configuration can be loaded from YAML files or overridden by environment variables.
#[derive(Debug, Deserialize)]
pub struct DbConfig {
    /// Database server hostname or IP address
    host: Option<String>,
    /// Database server port
    port: Option<u16>,
    /// Database authentication username
    user: Option<String>,
    /// Database authentication password
    password: Option<String>,
    /// Name of the database to connect to
    db_name: Option<String>,
    /// PostgreSQL schema name (namespace)
    schema: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DbPoolConfig {
    /// Minimum number of connections to maintain in the pool.
    ///
    /// Default: `5`
    min_connections: Option<u32>,
    /// Maximum number of connections allowed in the pool.
    ///
    /// Default: `20`
    max_connections: Option<u32>,
    /// Maximum time to wait when establishing a new connection (seconds).
    ///
    /// Default: `10`
    connect_timeout: Option<u64>,
    /// Maximum time to wait for database query responses (seconds).
    ///
    /// Default: `20`
    read_timeout: Option<u64>,
    /// Maximum time a connection can remain idle in the pool before being closed (seconds).
    ///
    /// Default: `300` (5 minutes)
    idle_timeout: Option<u64>,
    /// Maximum lifetime of a connection in the pool (hours).
    ///
    /// Default: `24`
    max_lifetime: Option<u64>,
}

impl DbConfig {
    /// Returns the database host with fallback to localhost.
    ///
    /// Default: `127.0.0.1`
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
    }

    /// Returns the database port with fallback to PostgreSQL default.
    ///
    /// Default: `5432`
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(5432)
    }

    /// Returns the database username with fallback to 'postgres'.
    ///
    /// Default: `postgres`
    pub fn user(&self) -> &str {
        self.user.as_deref().unwrap_or("postgres")
    }

    /// Returns the database password with fallback to 'postgres'.
    ///
    /// Default: `postgres`
    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("postgres")
    }

    /// Returns the database name with fallback to 'axum_template'.
    ///
    /// Default: `axum_template`
    pub fn db_name(&self) -> &str {
        self.db_name.as_deref().unwrap_or("axum_template")
    }

    /// Returns the PostgreSQL schema name with fallback to 'public'.
    ///
    /// Default: `public`
    pub fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or("public")
    }

    /// Constructs a PostgreSQL connection URL from the configuration.
    ///
    /// Format: `postgres://user:password@host:port/database`
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user(),
            self.password(),
            self.host(),
            self.port(),
            self.db_name()
        )
    }
}

impl DbPoolConfig {
    pub fn min_connections(&self) -> u32 {
        self.min_connections.unwrap_or(5)
    }

    pub fn max_connections(&self) -> u32 {
        self.max_connections.unwrap_or(20)
    }

    pub fn connect_timeout(&self) -> u64 {
        self.connect_timeout.unwrap_or(10)
    }

    pub fn read_timeout(&self) -> u64 {
        self.read_timeout.unwrap_or(20)
    }

    pub fn idle_timeout(&self) -> u64 {
        self.idle_timeout.unwrap_or(300)
    }

    pub fn max_lifetime(&self) -> u64 {
        self.max_lifetime.unwrap_or(24)
    }
}
