use sea_orm::DatabaseConnection;

/// Application state shared across all request handlers.
///
/// Contains database connection pool and other shared resources.
#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    /// Creates a new application state with the given database connection.
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Returns a reference to the database connection.
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}
