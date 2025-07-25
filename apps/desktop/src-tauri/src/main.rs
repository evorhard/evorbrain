// Prevents additional console window on Windows in release, DO NOT REMOVE!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;

mod commands;
mod database;
mod errors;
mod filesystem;
mod models;
mod utils;

#[cfg(test)]
mod tests;

/// Set up logging with appropriate configuration
fn setup_logging() {
    let mut builder = Builder::from_default_env();
    
    builder
        .target(Target::Stdout)
        .filter_level(LevelFilter::Info)
        .filter_module("evorbrain", LevelFilter::Debug)
        .format(|buf, record| {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            let level = record.level();
            let target = record.target();
            let message = record.args();
            
            // Color coding for different log levels
            let level_str = match level {
                log::Level::Error => format!("\x1b[31m{}\x1b[0m", level), // Red
                log::Level::Warn => format!("\x1b[33m{}\x1b[0m", level),  // Yellow
                log::Level::Info => format!("\x1b[32m{}\x1b[0m", level),  // Green
                log::Level::Debug => format!("\x1b[36m{}\x1b[0m", level), // Cyan
                log::Level::Trace => format!("\x1b[35m{}\x1b[0m", level), // Magenta
            };
            
            writeln!(
                buf,
                "[{}] {} {} - {}",
                timestamp,
                level_str,
                target,
                message
            )
        })
        .init();
}

fn main() {
    // Check if we're running tests
    if std::env::args().any(|arg| arg == "--test-db") {
        test_database_operations();
        return;
    }

    // Initialize logging
    setup_logging();

    tauri::Builder::default()
        .setup(|app| {
            log::info!("Starting EvorBrain application");
            
            // Initialize database on app startup
            let app_handle = app.handle();
            let app_dir = app_handle.path().app_data_dir()
                .map_err(|e| {
                    let err = errors::AppError::Operation {
                        message: format!("Failed to get app data directory: {}", e),
                        code: errors::ErrorCode::OperationFailed,
                        severity: errors::ErrorSeverity::Critical,
                        context: Some(errors::ErrorContext {
                            user_action: "Starting application".to_string(),
                            recovery_suggestions: vec![
                                "Check if the application has permission to access its data directory".to_string(),
                                "Try reinstalling the application".to_string(),
                            ],
                            recoverable: false,
                            help_url: None,
                        }),
                    };
                    err.log();
                    err
                })?;
            
            log::info!("App data directory: {:?}", app_dir);
            
            // Ensure app directory exists
            std::fs::create_dir_all(&app_dir)
                .map_err(|e| {
                    let err = errors::AppError::Io {
                        source: e,
                        code: errors::ErrorCode::OperationFailed,
                        path: Some(app_dir.display().to_string()),
                        context: Some(errors::ErrorContext {
                            user_action: "Creating application data directory".to_string(),
                            recovery_suggestions: vec![
                                "Check if you have write permissions to the directory".to_string(),
                                "Ensure there is enough disk space".to_string(),
                            ],
                            recoverable: false,
                            help_url: None,
                        }),
                    };
                    err.log();
                    err
                })?;
            
            // Initialize database
            let db_path = app_dir.join("evorbrain.db");
            log::info!("Initializing database at: {:?}", db_path);
            
            database::init_database(&db_path)
                .map_err(|e| {
                    let context = errors::ErrorContext {
                        user_action: "Initializing database".to_string(),
                        recovery_suggestions: vec![
                            "Try deleting the database file and restarting".to_string(),
                            "Check if the database file is corrupted".to_string(),
                            "Ensure you have write permissions to the data directory".to_string(),
                        ],
                        recoverable: false,
                        help_url: None,
                    };
                    errors::AppError::Database {
                        source: e,
                        code: errors::ErrorCode::DatabaseConnection,
                        context: Some(context),
                    }
                })?;
            
            log::info!("Database initialized successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Basic commands
            commands::greet,
            commands::test_database,
            commands::search,
            commands::test_fts,
            commands::test_error_handling,
            
            // File system commands
            commands::read_file,
            commands::write_file,
            commands::delete_file,
            commands::list_directory,
            commands::get_file_metadata,
            commands::file_exists,
            commands::create_directory,
            commands::copy_file,
            commands::move_file,
            
            // Entity commands - Areas
            commands::create_area,
            commands::get_area,
            commands::get_all_areas,
            commands::update_area,
            commands::delete_area,
            
            // Entity commands - Goals
            commands::create_goal,
            commands::get_goal,
            commands::get_all_goals,
            commands::get_goals_by_area,
            commands::update_goal,
            commands::delete_goal,
            
            // Entity commands - Projects
            commands::create_project,
            commands::get_project,
            commands::get_all_projects,
            commands::get_projects_by_goal,
            commands::update_project,
            commands::delete_project,
            
            // Entity commands - Tasks
            commands::create_task,
            commands::get_task,
            commands::get_all_tasks,
            commands::get_tasks_by_project,
            commands::get_tasks_by_status,
            commands::get_upcoming_tasks,
            commands::get_subtasks,
            commands::update_task,
            commands::update_task_status,
            commands::delete_task,
        ])
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                log::info!("Window close requested");
                // Add any cleanup code here
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    log::info!("EvorBrain application shutting down");
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