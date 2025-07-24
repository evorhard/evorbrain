use std::path::{Path, PathBuf};
use std::io;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use thiserror::Error;
use tokio::fs;

pub mod operations;
pub mod watcher;
pub mod validation;


#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Path validation error: {0}")]
    PathValidation(String),
    
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    
    #[error("Directory not found: {0}")]
    DirectoryNotFound(PathBuf),
    
    #[error("Invalid file content: {0}")]
    InvalidContent(String),
    
    #[error("Watch error: {0}")]
    WatchError(String),
    
    #[error("Atomic operation failed: {0}")]
    AtomicOperationFailed(String),
}

pub type Result<T> = std::result::Result<T, FileSystemError>;

#[async_trait]
pub trait FileSystemOperations: Send + Sync {
    async fn read_file(&self, path: &Path) -> Result<String>;
    async fn write_file(&self, path: &Path, content: &str) -> Result<()>;
    async fn delete_file(&self, path: &Path) -> Result<()>;
    async fn exists(&self, path: &Path) -> Result<bool>;
    async fn create_directory(&self, path: &Path) -> Result<()>;
    async fn list_directory(&self, path: &Path) -> Result<Vec<PathBuf>>;
    async fn copy_file(&self, from: &Path, to: &Path) -> Result<()>;
    async fn move_file(&self, from: &Path, to: &Path) -> Result<()>;
    async fn get_metadata(&self, path: &Path) -> Result<FileMetadata>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub size: u64,
    pub created: Option<u64>,
    pub modified: Option<u64>,
    pub is_file: bool,
    pub is_directory: bool,
    pub permissions_readonly: bool,
}

pub struct FileSystem {
    base_path: PathBuf,
}

impl FileSystem {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            base_path,
        }
    }
    
    pub fn base_path(&self) -> &Path {
        &self.base_path
    }
    
    fn resolve_path(&self, path: &Path) -> Result<PathBuf> {
        validation::validate_path(path)?;
        
        let full_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.base_path.join(path)
        };
        
        // Ensure the path is within the base directory
        let canonical_base = self.base_path.canonicalize()
            .map_err(|e| FileSystemError::Io(e))?;
        
        if let Ok(canonical_path) = full_path.canonicalize() {
            if !canonical_path.starts_with(&canonical_base) {
                return Err(FileSystemError::PermissionDenied(
                    "Path is outside of allowed directory".to_string()
                ));
            }
        }
        
        Ok(full_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_file_system_creation() {
        let temp_dir = TempDir::new().unwrap();
        let fs = FileSystem::new(temp_dir.path().to_path_buf());
        
        assert_eq!(fs.base_path(), temp_dir.path());
    }
    
    #[tokio::test]
    async fn test_path_resolution() {
        let temp_dir = TempDir::new().unwrap();
        let fs = FileSystem::new(temp_dir.path().to_path_buf());
        
        let relative_path = Path::new("test.txt");
        let resolved = fs.resolve_path(relative_path).unwrap();
        
        assert!(resolved.starts_with(temp_dir.path()));
        assert!(resolved.ends_with("test.txt"));
    }
}