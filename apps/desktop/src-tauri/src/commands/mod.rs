// Tauri command handlers

use rusqlite::Connection;
use tauri::Manager;
use crate::database::search::{search_entities, test_fts5, SearchResult};

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to EvorBrain.", name)
}

#[tauri::command]
pub fn test_database(app_handle: tauri::AppHandle) -> Result<String, String> {
    // Get the database path
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Failed to get app directory: {}", e))?;
    let db_path = app_dir.join("evorbrain.db");
    
    // Open connection
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
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
        - Connected to database at: {:?}\n\
        - Inserted test area with ID: {}\n\
        - Queried area: {} - {}\n\
        - Updated area description\n\
        - Deleted test area\n\
        - Verified {} tables exist",
        db_path, test_id, area.1, area.2.unwrap_or_default(), table_count
    ))
}

#[tauri::command]
pub fn search(
    app_handle: tauri::AppHandle,
    query: String,
    entity_type: Option<String>,
    parent_id: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
) -> Result<Vec<SearchResult>, String> {
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Failed to get app directory: {}", e))?;
    let db_path = app_dir.join("evorbrain.db");
    
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
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
pub fn test_fts(app_handle: tauri::AppHandle) -> Result<String, String> {
    let app_dir = app_handle.path().app_data_dir()
        .map_err(|e| format!("Failed to get app directory: {}", e))?;
    let db_path = app_dir.join("evorbrain.db");
    
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;
    
    test_fts5(&conn)
        .map_err(|e| format!("FTS5 test failed: {}", e))
}