// Tauri command handlers

pub mod filesystem;
pub mod entities;
pub mod relationships;

use tauri::State;
use crate::database::search::{search_entities, test_fts5, SearchResult};
use crate::database::pool::{AppState, get_db_connection_from_state};

// Re-export all commands
pub use filesystem::*;
pub use entities::*;
pub use relationships::*;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to EvorBrain.", name)
}

// Test command to demonstrate error handling
#[tauri::command]
pub async fn test_error_handling(error_type: String) -> Result<String, crate::errors::AppError> {
    use crate::errors::{AppError, ErrorCode, ErrorContext, ErrorSeverity};
    
    match error_type.as_str() {
        "validation" => {
            Err(AppError::missing_field("test_field"))
        },
        "not_found" => {
            Err(AppError::entity_not_found("test_entity", "test-id-123"))
        },
        "database" => {
            Err(AppError::Operation {
                message: "Database connection failed".to_string(),
                code: ErrorCode::DatabaseConnection,
                severity: ErrorSeverity::Critical,
                context: Some(ErrorContext {
                    user_action: "Testing database connection".to_string(),
                    recovery_suggestions: vec![
                        "Check if the database file exists".to_string(),
                        "Verify file permissions".to_string(),
                        "Try restarting the application".to_string(),
                    ],
                    recoverable: false,
                    help_url: Some("https://evorbrain.dev/help/database-errors".to_string()),
                }),
            })
        },
        "file" => {
            Err(AppError::Io {
                source: std::io::Error::new(std::io::ErrorKind::NotFound, "Test file not found"),
                code: ErrorCode::FileNotFound,
                path: Some("/path/to/test/file.md".to_string()),
                context: Some(ErrorContext {
                    user_action: "Reading file".to_string(),
                    recovery_suggestions: vec![
                        "Check if the file exists".to_string(),
                        "Verify the file path is correct".to_string(),
                    ],
                    recoverable: true,
                    help_url: None,
                }),
            })
        },
        _ => {
            Ok("No error - test successful!".to_string())
        }
    }
}

#[tauri::command]
pub fn test_database(state: State<AppState>) -> Result<String, String> {
    // Get connection from pool
    let conn = get_db_connection_from_state(&state)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;
    
    // Test 1: Insert a test area
    let test_id = "test-area-1";
    let _result = conn.execute(
        "INSERT OR REPLACE INTO areas (id, title, description, color, icon, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), datetime('now'))",
        &[test_id, "Test Area", "This is a test area", "#3b82f6", "home"],
    ).map_err(|e| format!("Failed to insert test area: {}", e))?;
    
    // Test 2: Query the inserted area
    let mut stmt = conn.prepare("SELECT id, title, description FROM areas WHERE id = ?1")
        .map_err(|e| format!("Failed to prepare query: {}", e))?;
    
    let area: (String, String, Option<String>) = stmt.query_row([test_id], |row| {
        Ok((
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
        ))
    }).map_err(|e| format!("Failed to query area: {}", e))?;
    
    // Test 3: Update the area
    conn.execute(
        "UPDATE areas SET description = ?1, updated_at = datetime('now') WHERE id = ?2",
        &["Updated test description", test_id],
    ).map_err(|e| format!("Failed to update area: {}", e))?;
    
    // Test 4: Delete the test area
    conn.execute(
        "DELETE FROM areas WHERE id = ?1",
        &[test_id],
    ).map_err(|e| format!("Failed to delete area: {}", e))?;
    
    // Test 5: Verify tables exist
    let table_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('areas', 'goals', 'projects', 'tasks', 'search_index')",
        [],
        |row| row.get(0),
    ).map_err(|e| format!("Failed to count tables: {}", e))?;
    
    Ok(format!(
        "Database tests completed successfully!\n\
        - Connected to database\n\
        - Inserted test area with ID: {}\n\
        - Queried area: {} - {}\n\
        - Updated area description\n\
        - Deleted test area\n\
        - Verified {} tables exist",
        test_id, area.1, area.2.unwrap_or_default(), table_count
    ))
}

#[tauri::command]
pub fn search(
    state: State<AppState>,
    query: String,
    entity_type: Option<String>,
    parent_id: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<SearchResult>, String> {
    let conn = get_db_connection_from_state(&state)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;
    
    let results = search_entities(
        &conn,
        &query,
        entity_type.as_deref(),
        parent_id.as_deref(),
        limit.unwrap_or(20),
        offset.unwrap_or(0),
    ).map_err(|e| format!("Search failed: {}", e))?;
    
    Ok(results)
}

#[tauri::command]
pub fn test_fts(state: State<AppState>) -> Result<String, String> {
    let conn = get_db_connection_from_state(&state)
        .map_err(|e| format!("Failed to get database connection: {}", e))?;
    
    test_fts5(&conn)
        .map_err(|e| format!("FTS5 test failed: {}", e))
}