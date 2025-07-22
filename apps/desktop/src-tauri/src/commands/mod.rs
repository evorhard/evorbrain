// Tauri command handlers

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to EvorBrain.", name)
}