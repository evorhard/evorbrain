// Prevents additional console window on Windows in release, DO NOT REMOVE!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod commands;
mod database;
mod errors;
mod models;
mod utils;

fn main() {
    // Check if we're running tests
    if std::env::args().any(|arg| arg == "--test-db") {
        test_database_operations();
        return;
    }

    // Initialize logging
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            // Initialize database on app startup
            let app_handle = app.handle();
            let app_dir = app_handle.path().app_data_dir().unwrap();
            
            // Ensure app directory exists
            std::fs::create_dir_all(&app_dir).expect("Failed to create app directory");
            
            // Initialize database
            let db_path = app_dir.join("evorbrain.db");
            database::init_database(&db_path)?;
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Add command handlers here
            commands::greet,
            commands::test_database,
            commands::search,
            commands::test_fts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn test_database_operations() {
    use rusqlite::Connection;
    use std::path::Path;
    
    println!("🧪 Testing EvorBrain Database Operations\n");
    
    // Create a test database
    let db_path = Path::new("test_evorbrain.db");
    
    // Initialize database
    println!("📦 Initializing database...");
    database::init_database(&db_path).expect("Failed to initialize database");
    println!("✅ Database initialized successfully!\n");
    
    // Open connection for tests
    let conn = Connection::open(&db_path).expect("Failed to open connection");
    
    // Test 1: Insert a test area
    println!("🔧 Test 1: Insert a test area");
    let test_id = "test-area-1";
    let result = conn.execute(
        "INSERT OR REPLACE INTO areas (id, title, description, color, icon, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), datetime('now'))",
        &[test_id, "Test Area", "This is a test area", "#3b82f6", "home"],
    ).expect("Failed to insert area");
    println!("✅ Inserted {} row(s)\n", result);
    
    // Test 2: Query the inserted area
    println!("🔧 Test 2: Query the inserted area");
    {
        let mut stmt = conn.prepare("SELECT id, title, description FROM areas WHERE id = ?1")
            .expect("Failed to prepare statement");
        
        let area: (String, String, Option<String>) = stmt.query_row([test_id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            ))
        }).expect("Failed to query area");
        
        println!("✅ Queried area:");
        println!("   ID: {}", area.0);
        println!("   Title: {}", area.1);
        println!("   Description: {}\n", area.2.unwrap_or_default());
    }
    
    // Test 3: Update the area
    println!("🔧 Test 3: Update the area");
    let result = conn.execute(
        "UPDATE areas SET description = ?1, updated_at = datetime('now') WHERE id = ?2",
        &["Updated test description", test_id],
    ).expect("Failed to update area");
    println!("✅ Updated {} row(s)\n", result);
    
    // Test 4: Delete the test area
    println!("🔧 Test 4: Delete the test area");
    let result = conn.execute(
        "DELETE FROM areas WHERE id = ?1",
        &[test_id],
    ).expect("Failed to delete area");
    println!("✅ Deleted {} row(s)\n", result);
    
    // Test 5: Verify tables exist
    println!("🔧 Test 5: Verify tables exist");
    let table_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('areas', 'goals', 'projects', 'tasks', 'search_index')",
        [],
        |row| row.get(0),
    ).expect("Failed to count tables");
    println!("✅ Verified {} tables exist\n", table_count);
    
    // Clean up
    drop(conn);
    std::fs::remove_file(&db_path).expect("Failed to remove test database");
    
    println!("🎉 All database tests passed successfully!");
}