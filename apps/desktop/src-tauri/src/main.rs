// Prevents additional console window on Windows in release, DO NOT REMOVE!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

mod commands;
mod database;
mod errors;
mod models;
mod utils;

fn main() {
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}