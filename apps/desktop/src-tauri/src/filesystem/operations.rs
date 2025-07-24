use super::*;
use tokio::io::AsyncWriteExt;
use std::time::UNIX_EPOCH;

#[async_trait]
impl FileSystemOperations for FileSystem {
    async fn read_file(&self, path: &Path) -> Result<String> {
        let full_path = self.resolve_path(path)?;
        
        if !full_path.exists() {
            return Err(FileSystemError::FileNotFound(full_path));
        }
        
        let content = fs::read_to_string(&full_path).await?;
        Ok(content)
    }
    
    async fn write_file(&self, path: &Path, content: &str) -> Result<()> {
        let full_path = self.resolve_path(path)?;
        
        // Ensure parent directory exists
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        // Write atomically using a temporary file
        atomic_write(&full_path, content).await
    }
    
    async fn delete_file(&self, path: &Path) -> Result<()> {
        let full_path = self.resolve_path(path)?;
        
        if !full_path.exists() {
            return Err(FileSystemError::FileNotFound(full_path));
        }
        
        fs::remove_file(&full_path).await?;
        Ok(())
    }
    
    async fn exists(&self, path: &Path) -> Result<bool> {
        let full_path = self.resolve_path(path)?;
        Ok(full_path.exists())
    }
    
    async fn create_directory(&self, path: &Path) -> Result<()> {
        let full_path = self.resolve_path(path)?;
        fs::create_dir_all(&full_path).await?;
        Ok(())
    }
    
    async fn list_directory(&self, path: &Path) -> Result<Vec<PathBuf>> {
        let full_path = self.resolve_path(path)?;
        
        if !full_path.exists() {
            return Err(FileSystemError::DirectoryNotFound(full_path));
        }
        
        let mut entries = Vec::new();
        let mut dir = fs::read_dir(&full_path).await?;
        
        while let Some(entry) = dir.next_entry().await? {
            entries.push(entry.path());
        }
        
        Ok(entries)
    }
    
    async fn copy_file(&self, from: &Path, to: &Path) -> Result<()> {
        let from_path = self.resolve_path(from)?;
        let to_path = self.resolve_path(to)?;
        
        if !from_path.exists() {
            return Err(FileSystemError::FileNotFound(from_path));
        }
        
        // Ensure parent directory exists
        if let Some(parent) = to_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        fs::copy(&from_path, &to_path).await?;
        Ok(())
    }
    
    async fn move_file(&self, from: &Path, to: &Path) -> Result<()> {
        let from_path = self.resolve_path(from)?;
        let to_path = self.resolve_path(to)?;
        
        if !from_path.exists() {
            return Err(FileSystemError::FileNotFound(from_path));
        }
        
        // Ensure parent directory exists
        if let Some(parent) = to_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        fs::rename(&from_path, &to_path).await?;
        Ok(())
    }
    
    async fn get_metadata(&self, path: &Path) -> Result<FileMetadata> {
        let full_path = self.resolve_path(path)?;
        
        if !full_path.exists() {
            return Err(FileSystemError::FileNotFound(full_path));
        }
        
        let metadata = fs::metadata(&full_path).await?;
        
        let created = metadata.created()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs());
            
        let modified = metadata.modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs());
        
        Ok(FileMetadata {
            path: full_path,
            size: metadata.len(),
            created,
            modified,
            is_file: metadata.is_file(),
            is_directory: metadata.is_dir(),
            permissions_readonly: metadata.permissions().readonly(),
        })
    }
}

pub async fn atomic_write(path: &Path, content: &str) -> Result<()> {
    let temp_path = path.with_extension("tmp");
    
    // Write to temporary file
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&temp_path)
        .await?;
    
    file.write_all(content.as_bytes()).await?;
    file.sync_all().await?;
    drop(file);
    
    // Atomically rename
    fs::rename(&temp_path, path).await.map_err(|e| {
        // Clean up temp file on error
        let _ = std::fs::remove_file(&temp_path);
        FileSystemError::AtomicOperationFailed(format!("Failed to rename: {}", e))
    })?;
    
    Ok(())
}

pub async fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path).await?;
    }
    Ok(())
}

pub async fn read_file_with_backup(path: &Path) -> Result<String> {
    // Try to read the main file
    match fs::read_to_string(path).await {
        Ok(content) => Ok(content),
        Err(_) => {
            // Try backup file
            let backup_path = path.with_extension("bak");
            fs::read_to_string(&backup_path).await.map_err(|_| {
                FileSystemError::FileNotFound(path.to_path_buf())
            })
        }
    }
}

pub async fn write_file_with_backup(path: &Path, content: &str) -> Result<()> {
    // Create backup if file exists
    if path.exists() {
        let backup_path = path.with_extension("bak");
        fs::copy(path, &backup_path).await?;
    }
    
    // Write new content
    atomic_write(path, content).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_read_write_file() {
        let temp_dir = TempDir::new().unwrap();
        let fs = FileSystem::new(temp_dir.path().to_path_buf());
        
        let test_path = Path::new("test.txt");
        let content = "Hello, World!";
        
        // Write file
        fs.write_file(test_path, content).await.unwrap();
        
        // Read file
        let read_content = fs.read_file(test_path).await.unwrap();
        assert_eq!(read_content, content);
    }
    
    #[tokio::test]
    async fn test_atomic_write() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("atomic.txt");
        
        atomic_write(&file_path, "Atomic content").await.unwrap();
        
        let content = fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(content, "Atomic content");
    }
    
    #[tokio::test]
    async fn test_file_operations() {
        let temp_dir = TempDir::new().unwrap();
        let fs = FileSystem::new(temp_dir.path().to_path_buf());
        
        // Test exists
        let test_path = Path::new("test.txt");
        assert!(!fs.exists(test_path).await.unwrap());
        
        // Create and test
        fs.write_file(test_path, "Test").await.unwrap();
        assert!(fs.exists(test_path).await.unwrap());
        
        // Test metadata
        let metadata = fs.get_metadata(test_path).await.unwrap();
        assert!(metadata.is_file);
        assert_eq!(metadata.size, 4);
        
        // Test copy
        let copy_path = Path::new("copy.txt");
        fs.copy_file(test_path, copy_path).await.unwrap();
        assert!(fs.exists(copy_path).await.unwrap());
        
        // Test move
        let move_path = Path::new("moved.txt");
        fs.move_file(copy_path, move_path).await.unwrap();
        assert!(!fs.exists(copy_path).await.unwrap());
        assert!(fs.exists(move_path).await.unwrap());
        
        // Test delete
        fs.delete_file(test_path).await.unwrap();
        assert!(!fs.exists(test_path).await.unwrap());
    }
}