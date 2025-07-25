use crate::errors::AppError;
use crate::filesystem::{FileMetadata, FileSystem, FileSystemOperations};
use std::path::Path;
use tauri::Manager;

// Initialize a FileSystem instance for the app data directory
fn get_filesystem(app_handle: &tauri::AppHandle) -> Result<FileSystem, AppError> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Unknown {
            message: format!("Failed to get app directory: {}", e),
            context: None,
        })?;
    
    let data_dir = app_dir.join("data");
    std::fs::create_dir_all(&data_dir)
        .map_err(|e| AppError::Io {
            source: e,
            code: crate::errors::ErrorCode::OperationFailed,
            path: Some(data_dir.to_string_lossy().to_string()),
            context: None,
        })?;
    
    Ok(FileSystem::new(data_dir))
}

#[tauri::command]
pub async fn read_file(
    app_handle: tauri::AppHandle,
    path: String,
) -> Result<String, AppError> {
    let fs = get_filesystem(&app_handle)?;
    let file_path = Path::new(&path);
    
    Ok(fs.read_file(file_path).await?)
}

#[tauri::command]
pub async fn write_file(
    app_handle: tauri::AppHandle,
    path: String,
    content: String,
) -> Result<(), AppError> {
    let fs = get_filesystem(&app_handle)?;
    let file_path = Path::new(&path);
    
    Ok(fs.write_file(file_path, &content).await?)
}

#[tauri::command]
pub async fn delete_file(
    app_handle: tauri::AppHandle,
    path: String,
) -> Result<(), AppError> {
    let fs = get_filesystem(&app_handle)?;
    let file_path = Path::new(&path);
    
    Ok(fs.delete_file(file_path).await?)
}

#[tauri::command]
pub async fn list_directory(
    app_handle: tauri::AppHandle,
    path: String,
) -> Result<Vec<String>, AppError> {
    let fs = get_filesystem(&app_handle)?;
    let dir_path = Path::new(&path);
    
    let paths = fs.list_directory(dir_path).await?;
    
    // Convert PathBuf to String for serialization
    Ok(paths
        .into_iter()
        .filter_map(|p| p.to_str().map(|s| s.to_string()))
        .collect())
}

#[tauri::command]
pub async fn get_file_metadata(
    app_handle: tauri::AppHandle,
    path: String,
) -> Result<FileMetadata, AppError> {
    let fs = get_filesystem(&app_handle)?;
    let file_path = Path::new(&path);
    
    Ok(fs.get_metadata(file_path).await?)
}

#[tauri::command]
pub async fn file_exists(
    app_handle: tauri::AppHandle,
    path: String,
) -> Result<bool, AppError> {
    let fs = get_filesystem(&app_handle)?;
    let file_path = Path::new(&path);
    
    Ok(fs.exists(file_path).await?)
}

#[tauri::command]
pub async fn create_directory(
    app_handle: tauri::AppHandle,
    path: String,
) -> Result<(), AppError> {
    let fs = get_filesystem(&app_handle)?;
    let dir_path = Path::new(&path);
    
    Ok(fs.create_directory(dir_path).await?)
}

#[tauri::command]
pub async fn copy_file(
    app_handle: tauri::AppHandle,
    from: String,
    to: String,
) -> Result<(), AppError> {
    let fs = get_filesystem(&app_handle)?;
    let from_path = Path::new(&from);
    let to_path = Path::new(&to);
    
    Ok(fs.copy_file(from_path, to_path).await?)
}

#[tauri::command]
pub async fn move_file(
    app_handle: tauri::AppHandle,
    from: String,
    to: String,
) -> Result<(), AppError> {
    let fs = get_filesystem(&app_handle)?;
    let from_path = Path::new(&from);
    let to_path = Path::new(&to);
    
    Ok(fs.move_file(from_path, to_path).await?)
}