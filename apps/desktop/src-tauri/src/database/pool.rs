use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;
use std::sync::Arc;
use tauri::State;
use crate::errors::{AppError, ErrorCode, ErrorSeverity, ErrorContext};

pub type DbPool = Pool<SqliteConnectionManager>;
pub type DbConnection = PooledConnection<SqliteConnectionManager>;

/// Application state containing the database pool
pub struct AppState {
    pub db_pool: Arc<DbPool>,
}

/// Initialize a connection pool for the database
pub fn init_pool(db_path: &Path) -> Result<DbPool, AppError> {
    let manager = SqliteConnectionManager::file(db_path);
    
    Pool::builder()
        .max_size(10) // Maximum 10 connections
        .min_idle(Some(2)) // Keep at least 2 idle connections
        .connection_timeout(std::time::Duration::from_secs(10))
        .build(manager)
        .map_err(|e| AppError::Operation {
            message: format!("Failed to create database pool: {}", e),
            code: ErrorCode::DatabaseConnection,
            severity: ErrorSeverity::Critical,
            context: Some(ErrorContext {
                user_action: "Initializing database connection pool".to_string(),
                recovery_suggestions: vec![
                    "Check if the database file exists".to_string(),
                    "Verify file permissions".to_string(),
                    "Ensure there's enough system resources".to_string(),
                ],
                recoverable: false,
                help_url: None,
            }),
        })
}

/// Get a connection from the pool
pub fn get_connection(pool: &DbPool) -> Result<DbConnection, AppError> {
    pool.get()
        .map_err(|e| AppError::Operation {
            message: format!("Failed to get database connection from pool: {}", e),
            code: ErrorCode::DatabaseConnection,
            severity: ErrorSeverity::Error,
            context: Some(ErrorContext {
                user_action: "Accessing database".to_string(),
                recovery_suggestions: vec![
                    "The connection pool might be exhausted".to_string(),
                    "Try again in a moment".to_string(),
                    "Check system resources".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        })
}

/// Helper to get a connection from Tauri state
pub fn get_db_connection_from_state(state: &State<AppState>) -> Result<DbConnection, AppError> {
    get_connection(&state.db_pool)
}