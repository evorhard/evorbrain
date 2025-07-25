use super::*;
use std::path::Component;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    // Valid filename pattern: alphanumeric, dash, underscore, dot
    static ref VALID_FILENAME: Regex = Regex::new(r"^[a-zA-Z0-9_\-\.]+$").unwrap();
    
    // Reserved filenames on Windows
    static ref RESERVED_NAMES: Vec<&'static str> = vec![
        "CON", "PRN", "AUX", "NUL",
        "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
        "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
}

pub fn validate_path(path: &Path) -> Result<()> {
    // Check for path traversal attempts
    for component in path.components() {
        match component {
            Component::ParentDir => {
                return Err(FileSystemError::PathValidation(
                    "Path traversal (..) not allowed".to_string()
                ));
            }
            Component::RootDir => {
                return Err(FileSystemError::PathValidation(
                    "Absolute paths not allowed".to_string()
                ));
            }
            Component::Normal(name) => {
                validate_filename(name)?;
            }
            _ => {}
        }
    }
    
    Ok(())
}

pub fn validate_filename(name: &std::ffi::OsStr) -> Result<()> {
    let name_str = name.to_str()
        .ok_or_else(|| FileSystemError::PathValidation(
            "Invalid UTF-8 in filename".to_string()
        ))?;
    
    // Check length
    if name_str.is_empty() {
        return Err(FileSystemError::PathValidation(
            "Empty filename not allowed".to_string()
        ));
    }
    
    if name_str.len() > 255 {
        return Err(FileSystemError::PathValidation(
            "Filename too long (max 255 characters)".to_string()
        ));
    }
    
    // Check for invalid characters
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', '\0'];
    for ch in invalid_chars {
        if name_str.contains(ch) {
            return Err(FileSystemError::PathValidation(
                format!("Invalid character '{}' in filename", ch)
            ));
        }
    }
    
    // Check for reserved names (Windows)
    let name_upper = name_str.to_uppercase();
    let base_name = name_upper.split('.').next().unwrap_or(&name_upper);
    if RESERVED_NAMES.contains(&base_name) {
        return Err(FileSystemError::PathValidation(
            format!("Reserved filename '{}' not allowed", name_str)
        ));
    }
    
    // Check for leading/trailing dots or spaces
    if name_str.starts_with('.') || name_str.ends_with('.') ||
       name_str.starts_with(' ') || name_str.ends_with(' ') {
        return Err(FileSystemError::PathValidation(
            "Filename cannot start or end with dot or space".to_string()
        ));
    }
    
    Ok(())
}

pub fn sanitize_filename(name: &str) -> String {
    let mut sanitized = name.to_string();
    
    // Replace invalid characters with underscore
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', '\0'];
    for ch in invalid_chars {
        sanitized = sanitized.replace(ch, "_");
    }
    
    // Remove leading/trailing dots and spaces
    sanitized = sanitized.trim_matches(|c| c == '.' || c == ' ').to_string();
    
    // Ensure not empty
    if sanitized.is_empty() {
        sanitized = "unnamed".to_string();
    }
    
    // Truncate if too long
    if sanitized.len() > 200 {
        sanitized.truncate(200);
    }
    
    sanitized
}

pub fn generate_unique_filename(base: &str, extension: &str, existing: &[String]) -> String {
    let sanitized_base = sanitize_filename(base);
    let mut filename = format!("{}.{}", sanitized_base, extension);
    
    let mut counter = 1;
    while existing.contains(&filename) {
        filename = format!("{}-{}.{}", sanitized_base, counter, extension);
        counter += 1;
    }
    
    filename
}

pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

pub fn is_markdown_file(path: &Path) -> bool {
    matches!(get_file_extension(path).as_deref(), Some("md" | "markdown"))
}

pub fn is_hidden_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_filename() {
        // Valid filenames
        assert!(validate_filename(std::ffi::OsStr::new("test.md")).is_ok());
        assert!(validate_filename(std::ffi::OsStr::new("my-file_123.txt")).is_ok());
        
        // Invalid filenames
        assert!(validate_filename(std::ffi::OsStr::new("test/file.md")).is_err());
        assert!(validate_filename(std::ffi::OsStr::new("test:file.md")).is_err());
        assert!(validate_filename(std::ffi::OsStr::new("CON.txt")).is_err());
        assert!(validate_filename(std::ffi::OsStr::new(".")).is_err());
        assert!(validate_filename(std::ffi::OsStr::new("")).is_err());
    }
    
    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("test/file"), "test_file");
        assert_eq!(sanitize_filename("test:file<>"), "test_file__");
        assert_eq!(sanitize_filename("  .test.  "), "test");
        assert_eq!(sanitize_filename(""), "unnamed");
    }
    
    #[test]
    fn test_generate_unique_filename() {
        let existing = vec!["test.md".to_string(), "test-1.md".to_string()];
        
        assert_eq!(generate_unique_filename("test", "md", &existing), "test-2.md");
        assert_eq!(generate_unique_filename("new", "md", &existing), "new.md");
    }
    
    #[test]
    fn test_validate_path() {
        // Valid paths
        assert!(validate_path(Path::new("test.md")).is_ok());
        assert!(validate_path(Path::new("folder/test.md")).is_ok());
        
        // Invalid paths
        assert!(validate_path(Path::new("../test.md")).is_err());
        assert!(validate_path(Path::new("/absolute/path")).is_err());
        assert!(validate_path(Path::new("folder/../test.md")).is_err());
    }
    
    #[test]
    fn test_file_helpers() {
        assert_eq!(get_file_extension(Path::new("test.MD")), Some("md".to_string()));
        assert!(is_markdown_file(Path::new("test.md")));
        assert!(is_markdown_file(Path::new("test.markdown")));
        assert!(!is_markdown_file(Path::new("test.txt")));
        
        assert!(is_hidden_file(Path::new(".hidden")));
        assert!(!is_hidden_file(Path::new("visible")));
    }
}