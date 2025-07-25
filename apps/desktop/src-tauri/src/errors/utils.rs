use super::{AppError, ErrorCode, ErrorContext, ErrorSeverity};
use log::{error, debug};

/// Extension trait for Result types to add context
pub trait ResultExt<T> {
    /// Add context to an error
    fn with_user_context(self, action: &str, suggestions: Vec<String>) -> Result<T, AppError>;
    
    /// Log and convert the error
    fn log_error(self, message: &str) -> Result<T, AppError>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<AppError>,
{
    fn with_user_context(self, action: &str, suggestions: Vec<String>) -> Result<T, AppError> {
        self.map_err(|e| {
            let err: AppError = e.into();
            err.with_context(ErrorContext {
                user_action: action.to_string(),
                recovery_suggestions: suggestions,
                recoverable: true,
                help_url: None,
            })
        })
    }
    
    fn log_error(self, message: &str) -> Result<T, AppError> {
        self.map_err(|e| {
            let err: AppError = e.into();
            error!("{}: {}", message, err);
            err
        })
    }
}

/// Helper macro for creating validation errors
#[macro_export]
macro_rules! validation_error {
    ($field:expr, $reason:expr) => {
        $crate::errors::AppError::Validation {
            field: $field.to_string(),
            reason: $reason.to_string(),
            code: $crate::errors::ErrorCode::InvalidInput,
            context: None,
        }
    };
    ($field:expr, $reason:expr, $($suggestion:expr),+) => {
        $crate::errors::AppError::Validation {
            field: $field.to_string(),
            reason: $reason.to_string(),
            code: $crate::errors::ErrorCode::InvalidInput,
            context: Some($crate::errors::ErrorContext {
                user_action: "Validating input".to_string(),
                recovery_suggestions: vec![$($suggestion.to_string()),+],
                recoverable: true,
                help_url: None,
            }),
        }
    };
}

/// Helper function to validate string length
pub fn validate_string_length(
    value: &str,
    field_name: &str,
    min: Option<usize>,
    max: Option<usize>,
) -> Result<(), AppError> {
    let trimmed = value.trim();
    
    if let Some(min_len) = min {
        if trimmed.len() < min_len {
            return Err(AppError::Validation {
                field: field_name.to_string(),
                reason: format!("Must be at least {} characters", min_len),
                code: ErrorCode::ValueOutOfRange,
                context: Some(ErrorContext {
                    user_action: "Entering data".to_string(),
                    recovery_suggestions: vec![
                        format!("Please enter at least {} characters for {}", min_len, field_name),
                    ],
                    recoverable: true,
                    help_url: None,
                }),
            });
        }
    }
    
    if let Some(max_len) = max {
        if value.len() > max_len {
            return Err(AppError::Validation {
                field: field_name.to_string(),
                reason: format!("Must be no more than {} characters", max_len),
                code: ErrorCode::ValueOutOfRange,
                context: Some(ErrorContext {
                    user_action: "Entering data".to_string(),
                    recovery_suggestions: vec![
                        format!("Please shorten {} to no more than {} characters", field_name, max_len),
                    ],
                    recoverable: true,
                    help_url: None,
                }),
            });
        }
    }
    
    Ok(())
}

/// Helper function to validate required fields
pub fn validate_required(value: &str, field_name: &str) -> Result<(), AppError> {
    if value.trim().is_empty() {
        Err(AppError::missing_field(field_name))
    } else {
        Ok(())
    }
}

/// Log a database operation
pub fn log_db_operation(operation: &str, entity_type: &str, id: Option<&str>) {
    match id {
        Some(id) => debug!("Database operation: {} {} with ID: {}", operation, entity_type, id),
        None => debug!("Database operation: {} {}", operation, entity_type),
    }
}

/// Create a generic database error with context
pub fn database_error(operation: &str, entity_type: &str) -> impl Fn(rusqlite::Error) -> AppError {
    let operation = operation.to_string();
    let entity_type = entity_type.to_string();
    
    move |e: rusqlite::Error| {
        error!("Database error during {} {}: {}", operation, entity_type, e);
        
        match &e {
            rusqlite::Error::SqliteFailure(err, _) => match err.code {
                rusqlite::ErrorCode::ConstraintViolation => AppError::Operation {
                    message: format!("The {} violates data integrity rules", entity_type),
                    code: ErrorCode::DatabaseConstraint,
                    severity: ErrorSeverity::Warning,
                    context: Some(ErrorContext {
                        user_action: format!("{} {}", operation, entity_type),
                        recovery_suggestions: vec![
                            "Check if the item already exists".to_string(),
                            "Ensure all required fields are filled".to_string(),
                        ],
                        recoverable: true,
                        help_url: None,
                    }),
                },
                _ => AppError::from(e),
            },
            rusqlite::Error::QueryReturnedNoRows => AppError::entity_not_found(&entity_type, "unknown"),
            _ => AppError::from(e),
        }
    }
}