use thiserror::Error;
use serde::{Serialize, Deserialize};
use std::fmt;
use log::{error, warn};
use crate::filesystem::FileSystemError;

pub mod utils;
pub use utils::*;

/// Error codes for categorizing errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    // Database errors (1xxx)
    DatabaseConnection = 1001,
    DatabaseQuery = 1002,
    DatabaseConstraint = 1003,
    DatabaseMigration = 1004,
    
    // File system errors (2xxx)
    FileNotFound = 2001,
    FilePermissionDenied = 2002,
    FileAlreadyExists = 2003,
    InvalidPath = 2004,
    DirectoryNotEmpty = 2005,
    
    // Entity errors (3xxx)
    EntityNotFound = 3001,
    EntityAlreadyExists = 3002,
    InvalidEntityReference = 3003,
    OrphanedEntity = 3004,
    
    // Validation errors (4xxx)
    InvalidInput = 4001,
    MissingRequiredField = 4002,
    InvalidFormat = 4003,
    ValueOutOfRange = 4004,
    
    // Operation errors (5xxx)
    OperationFailed = 5001,
    OperationTimeout = 5002,
    OperationCancelled = 5003,
    ConcurrentModification = 5004,
    
    // System errors (9xxx)
    InternalError = 9001,
    UnknownError = 9999,
}

/// Severity levels for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Informational - user can continue
    Info,
    /// Warning - operation completed with issues
    Warning,
    /// Error - operation failed but app is stable
    Error,
    /// Critical - app stability may be affected
    Critical,
}

/// User-friendly error context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// What the user was trying to do
    pub user_action: String,
    /// Suggested recovery actions
    pub recovery_suggestions: Vec<String>,
    /// Whether the error is recoverable
    pub recoverable: bool,
    /// Additional helpful information
    pub help_url: Option<String>,
}

/// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("{}", .get_user_message())]
    Database {
        source: rusqlite::Error,
        code: ErrorCode,
        context: Option<ErrorContext>,
    },
    
    #[error("{}", .get_user_message())]
    Io {
        source: std::io::Error,
        code: ErrorCode,
        path: Option<String>,
        context: Option<ErrorContext>,
    },
    
    #[error("{}", .get_user_message())]
    Serialization {
        source: serde_json::Error,
        entity_type: String,
        context: Option<ErrorContext>,
    },
    
    #[error("{}", .get_user_message())]
    EntityNotFound {
        entity_type: String,
        id: String,
        context: Option<ErrorContext>,
    },
    
    #[error("{}", .get_user_message())]
    Validation {
        field: String,
        reason: String,
        code: ErrorCode,
        context: Option<ErrorContext>,
    },
    
    #[error("{}", .get_user_message())]
    FileSystem {
        source: FileSystemError,
        code: ErrorCode,
        context: Option<ErrorContext>,
    },
    
    #[error("{}", .get_user_message())]
    Operation {
        message: String,
        code: ErrorCode,
        severity: ErrorSeverity,
        context: Option<ErrorContext>,
    },
    
    #[error("{}", .get_user_message())]
    Unknown {
        message: String,
        context: Option<ErrorContext>,
    },
}

impl AppError {
    /// Get a user-friendly error message
    pub fn get_user_message(&self) -> String {
        match self {
            AppError::Database { code, .. } => match code {
                ErrorCode::DatabaseConnection => "Unable to connect to the database. Please try restarting the application.".to_string(),
                ErrorCode::DatabaseQuery => "Failed to retrieve data. The database might be corrupted.".to_string(),
                ErrorCode::DatabaseConstraint => "This operation would violate data integrity rules.".to_string(),
                ErrorCode::DatabaseMigration => "Database upgrade failed. Please contact support.".to_string(),
                _ => "A database error occurred.".to_string(),
            },
            
            AppError::Io { source, path, .. } => {
                let path_str = path.as_deref().unwrap_or("unknown location");
                match source.kind() {
                    std::io::ErrorKind::NotFound => format!("File not found: {}", path_str),
                    std::io::ErrorKind::PermissionDenied => format!("Permission denied accessing: {}", path_str),
                    std::io::ErrorKind::AlreadyExists => format!("File already exists: {}", path_str),
                    _ => format!("File operation failed for: {}", path_str),
                }
            },
            
            AppError::Serialization { entity_type, .. } => {
                format!("Failed to process {} data. The data might be corrupted.", entity_type)
            },
            
            AppError::EntityNotFound { entity_type, id, .. } => {
                format!("The {} '{}' could not be found.", entity_type.to_lowercase(), id)
            },
            
            AppError::Validation { field, reason, .. } => {
                format!("Invalid {}: {}", field, reason)
            },
            
            AppError::FileSystem { source, .. } => {
                format!("File system error: {}", source)
            },
            
            AppError::Operation { message, severity, .. } => {
                match severity {
                    ErrorSeverity::Info => format!("Info: {}", message),
                    ErrorSeverity::Warning => format!("Warning: {}", message),
                    ErrorSeverity::Error => format!("Error: {}", message),
                    ErrorSeverity::Critical => format!("Critical: {}", message),
                }
            },
            
            AppError::Unknown { message, .. } => {
                format!("An unexpected error occurred: {}", message)
            },
        }
    }
    
    /// Get the error code
    pub fn code(&self) -> ErrorCode {
        match self {
            AppError::Database { code, .. } => *code,
            AppError::Io { code, .. } => *code,
            AppError::Serialization { .. } => ErrorCode::InvalidFormat,
            AppError::EntityNotFound { .. } => ErrorCode::EntityNotFound,
            AppError::Validation { code, .. } => *code,
            AppError::FileSystem { code, .. } => *code,
            AppError::Operation { code, .. } => *code,
            AppError::Unknown { .. } => ErrorCode::UnknownError,
        }
    }
    
    /// Get the error severity
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            AppError::Database { code, .. } => match code {
                ErrorCode::DatabaseConnection | ErrorCode::DatabaseMigration => ErrorSeverity::Critical,
                _ => ErrorSeverity::Error,
            },
            AppError::Io { .. } => ErrorSeverity::Error,
            AppError::Serialization { .. } => ErrorSeverity::Error,
            AppError::EntityNotFound { .. } => ErrorSeverity::Warning,
            AppError::Validation { .. } => ErrorSeverity::Warning,
            AppError::FileSystem { .. } => ErrorSeverity::Error,
            AppError::Operation { severity, .. } => *severity,
            AppError::Unknown { .. } => ErrorSeverity::Error,
        }
    }
    
    /// Log the error with appropriate level
    pub fn log(&self) {
        let technical_details = self.to_string();
        let user_message = self.get_user_message();
        let code = self.code();
        
        match self.severity() {
            ErrorSeverity::Info => {
                log::info!("Error {}: {} - {}", code as u32, user_message, technical_details);
            },
            ErrorSeverity::Warning => {
                warn!("Error {}: {} - {}", code as u32, user_message, technical_details);
            },
            ErrorSeverity::Error => {
                error!("Error {}: {} - {}", code as u32, user_message, technical_details);
            },
            ErrorSeverity::Critical => {
                error!("CRITICAL Error {}: {} - {}", code as u32, user_message, technical_details);
                // In a real app, you might want to send this to a crash reporting service
            },
        }
    }
    
    /// Create an error with context
    pub fn with_context(mut self, context: ErrorContext) -> Self {
        match &mut self {
            AppError::Database { context: ctx, .. } |
            AppError::Io { context: ctx, .. } |
            AppError::Serialization { context: ctx, .. } |
            AppError::EntityNotFound { context: ctx, .. } |
            AppError::Validation { context: ctx, .. } |
            AppError::FileSystem { context: ctx, .. } |
            AppError::Operation { context: ctx, .. } |
            AppError::Unknown { context: ctx, .. } => {
                *ctx = Some(context);
            }
        }
        self
    }
}

/// Conversion from rusqlite::Error
impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        let code = match &err {
            rusqlite::Error::SqliteFailure(e, _) => match e.code {
                rusqlite::ErrorCode::ConstraintViolation => ErrorCode::DatabaseConstraint,
                _ => ErrorCode::DatabaseQuery,
            },
            rusqlite::Error::QueryReturnedNoRows => ErrorCode::EntityNotFound,
            _ => ErrorCode::DatabaseQuery,
        };
        
        AppError::Database {
            source: err,
            code,
            context: None,
        }
    }
}

/// Conversion from std::io::Error
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        let code = match err.kind() {
            std::io::ErrorKind::NotFound => ErrorCode::FileNotFound,
            std::io::ErrorKind::PermissionDenied => ErrorCode::FilePermissionDenied,
            std::io::ErrorKind::AlreadyExists => ErrorCode::FileAlreadyExists,
            _ => ErrorCode::OperationFailed,
        };
        
        AppError::Io {
            source: err,
            code,
            path: None,
            context: None,
        }
    }
}

/// Conversion from serde_json::Error
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization {
            source: err,
            entity_type: "Unknown".to_string(),
            context: None,
        }
    }
}

/// Conversion from FileSystemError
impl From<FileSystemError> for AppError {
    fn from(err: FileSystemError) -> Self {
        let code = match &err {
            FileSystemError::NotFound(_) => ErrorCode::FileNotFound,
            FileSystemError::PermissionDenied(_) => ErrorCode::FilePermissionDenied,
            FileSystemError::AlreadyExists(_) => ErrorCode::FileAlreadyExists,
            FileSystemError::InvalidPath(_) => ErrorCode::InvalidPath,
            FileSystemError::DirectoryNotEmpty(_) => ErrorCode::DirectoryNotEmpty,
            _ => ErrorCode::OperationFailed,
        };
        
        AppError::FileSystem {
            source: err,
            code,
            context: None,
        }
    }
}

/// Serialization for Tauri IPC
#[derive(Serialize)]
struct SerializedError {
    code: u32,
    message: String,
    user_message: String,
    severity: ErrorSeverity,
    recoverable: bool,
    suggestions: Vec<String>,
    help_url: Option<String>,
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Log the error before serializing
        self.log();
        
        let context = match self {
            AppError::Database { context, .. } |
            AppError::Io { context, .. } |
            AppError::Serialization { context, .. } |
            AppError::EntityNotFound { context, .. } |
            AppError::Validation { context, .. } |
            AppError::FileSystem { context, .. } |
            AppError::Operation { context, .. } |
            AppError::Unknown { context, .. } => context,
        };
        
        let serialized = SerializedError {
            code: self.code() as u32,
            message: self.to_string(),
            user_message: self.get_user_message(),
            severity: self.severity(),
            recoverable: context.as_ref().map(|c| c.recoverable).unwrap_or(true),
            suggestions: context.as_ref().map(|c| c.recovery_suggestions.clone()).unwrap_or_default(),
            help_url: context.as_ref().and_then(|c| c.help_url.clone()),
        };
        
        serialized.serialize(serializer)
    }
}

/// Helper functions for creating common errors
impl AppError {
    /// Create a validation error for a missing required field
    pub fn missing_field(field: &str) -> Self {
        AppError::Validation {
            field: field.to_string(),
            reason: "This field is required".to_string(),
            code: ErrorCode::MissingRequiredField,
            context: Some(ErrorContext {
                user_action: "Saving data".to_string(),
                recovery_suggestions: vec![
                    format!("Please provide a value for '{}'", field),
                ],
                recoverable: true,
                help_url: None,
            }),
        }
    }
    
    /// Create a validation error for invalid format
    pub fn invalid_format(field: &str, expected: &str) -> Self {
        AppError::Validation {
            field: field.to_string(),
            reason: format!("Expected format: {}", expected),
            code: ErrorCode::InvalidFormat,
            context: Some(ErrorContext {
                user_action: "Validating input".to_string(),
                recovery_suggestions: vec![
                    format!("Please ensure '{}' follows the format: {}", field, expected),
                ],
                recoverable: true,
                help_url: None,
            }),
        }
    }
    
    /// Create an entity not found error
    pub fn entity_not_found(entity_type: &str, id: &str) -> Self {
        AppError::EntityNotFound {
            entity_type: entity_type.to_string(),
            id: id.to_string(),
            context: Some(ErrorContext {
                user_action: format!("Loading {}", entity_type),
                recovery_suggestions: vec![
                    "The item may have been deleted".to_string(),
                    "Try refreshing the view".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        }
    }
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, AppError>;