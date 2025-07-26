use crate::errors::{AppError, ErrorCode, ErrorSeverity};
use crate::models::{Area, Goal, Project, Task, GoalStatus, ProjectStatus, TaskStatus, TaskPriority};
use rusqlite::{params, Row};
use tauri::State;
use chrono::{DateTime, Utc};
use serde_json;
use serde::{Deserialize, Serialize};
use crate::database::pool::{AppState, get_db_connection_from_state};

/// Pagination parameters
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

/// Paginated response
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u32,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(50),
        }
    }
}

impl PaginationParams {
    fn offset(&self) -> u32 {
        let page = self.page.unwrap_or(1).max(1);
        let per_page = self.per_page.unwrap_or(50);
        (page - 1) * per_page
    }
    
    fn limit(&self) -> u32 {
        self.per_page.unwrap_or(50).min(100) // Max 100 items per page
    }
}

// Helper to parse DateTime from database string
pub fn parse_datetime(datetime_str: &str, column_index: usize) -> Result<DateTime<Utc>, rusqlite::Error> {
    DateTime::parse_from_rfc3339(datetime_str)
        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(
            column_index,
            rusqlite::types::Type::Text,
            Box::new(e)
        ))
        .map(|dt| dt.with_timezone(&Utc))
}


// Helper functions to parse database rows
fn parse_area(row: &Row) -> Result<Area, rusqlite::Error> {
    let tags_json: Option<String> = row.get(8)?;
    let tags = tags_json.and_then(|json| serde_json::from_str(&json).ok());
    
    let created_at_str: String = row.get(5)?;
    let updated_at_str: String = row.get(6)?;
    
    Ok(Area {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        color: row.get(3)?,
        icon: row.get(4)?,
        created_at: parse_datetime(&created_at_str, 5)?,
        updated_at: parse_datetime(&updated_at_str, 6)?,
        sort_order: row.get(7)?,
        tags,
        entity_type: "area".to_string(),
    })
}

pub fn parse_goal_status(status: &str) -> GoalStatus {
    match status {
        "not-started" => GoalStatus::NotStarted,
        "in-progress" => GoalStatus::InProgress,
        "completed" => GoalStatus::Completed,
        "abandoned" => GoalStatus::Abandoned,
        _ => {
            log::warn!("Unknown goal status: {}, defaulting to NotStarted", status);
            GoalStatus::NotStarted
        }
    }
}

pub fn parse_project_status(status: &str) -> ProjectStatus {
    match status {
        "planning" => ProjectStatus::Planning,
        "active" => ProjectStatus::Active,
        "completed" => ProjectStatus::Completed,
        "on-hold" => ProjectStatus::OnHold,
        "cancelled" => ProjectStatus::Cancelled,
        _ => {
            log::warn!("Unknown project status: {}, defaulting to Planning", status);
            ProjectStatus::Planning
        }
    }
}

pub fn parse_task_status(status: &str) -> TaskStatus {
    match status {
        "not-started" => TaskStatus::NotStarted,
        "in-progress" => TaskStatus::InProgress,
        "completed" => TaskStatus::Completed,
        "cancelled" => TaskStatus::Cancelled,
        _ => {
            log::warn!("Unknown task status: {}, defaulting to NotStarted", status);
            TaskStatus::NotStarted
        }
    }
}

pub fn parse_task_priority(priority: Option<String>) -> TaskPriority {
    priority.map(|p| match p.as_str() {
        "urgent" => TaskPriority::Urgent,
        "high" => TaskPriority::High,
        "medium" => TaskPriority::Medium,
        "low" => TaskPriority::Low,
        _ => TaskPriority::Medium,
    }).unwrap_or(TaskPriority::Medium)
}

fn goal_status_to_string(status: &GoalStatus) -> &'static str {
    match status {
        GoalStatus::NotStarted => "not-started",
        GoalStatus::InProgress => "in-progress",
        GoalStatus::Completed => "completed",
        GoalStatus::Abandoned => "abandoned",
    }
}

fn project_status_to_string(status: &ProjectStatus) -> &'static str {
    match status {
        ProjectStatus::Planning => "planning",
        ProjectStatus::Active => "active",
        ProjectStatus::Completed => "completed",
        ProjectStatus::OnHold => "on-hold",
        ProjectStatus::Cancelled => "cancelled",
    }
}

fn task_status_to_string(status: &TaskStatus) -> &'static str {
    match status {
        TaskStatus::NotStarted => "not-started",
        TaskStatus::InProgress => "in-progress",
        TaskStatus::Completed => "completed",
        TaskStatus::Cancelled => "cancelled",
    }
}

fn task_priority_to_string(priority: &TaskPriority) -> &'static str {
    match priority {
        TaskPriority::Urgent => "urgent",
        TaskPriority::High => "high",
        TaskPriority::Medium => "medium",
        TaskPriority::Low => "low",
    }
}

// Area Commands
#[tauri::command]
pub async fn create_area(
    state: State<'_, AppState>,
    title: String,
    description: Option<String>,
    color: Option<String>,
    icon: Option<String>,
) -> Result<Area, AppError> {
    // Validate input
    if title.trim().is_empty() {
        return Err(AppError::missing_field("title"));
    }
    
    if title.len() > 255 {
        return Err(AppError::Validation {
            field: "title".to_string(),
            reason: "Title must be less than 255 characters".to_string(),
            code: crate::errors::ErrorCode::ValueOutOfRange,
            context: Some(crate::errors::ErrorContext {
                user_action: "Creating area".to_string(),
                recovery_suggestions: vec![
                    "Please shorten the title to less than 255 characters".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    let mut area = Area::new(title, description);
    area.color = color;
    area.icon = icon;
    
    // Validate the area
    area.validate()?;
    
    let conn = get_db_connection_from_state(&state)?;
    
    let tags_json = area.tags.as_ref()
        .and_then(|tags| serde_json::to_string(tags).ok());
    
    conn.execute(
        "INSERT INTO areas (id, title, description, color, icon, created_at, updated_at, sort_order, tags)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            &area.id,
            &area.title,
            &area.description,
            &area.color,
            &area.icon,
            area.created_at.to_rfc3339(),
            area.updated_at.to_rfc3339(),
            area.sort_order,
            &tags_json,
        ],
    ).map_err(|e| {
        log::error!("Failed to create area: {}", e);
        match &e {
            rusqlite::Error::SqliteFailure(err, _) if err.code == rusqlite::ErrorCode::ConstraintViolation => {
                AppError::Operation {
                    message: "An area with this ID already exists".to_string(),
                    code: crate::errors::ErrorCode::EntityAlreadyExists,
                    severity: crate::errors::ErrorSeverity::Warning,
                    context: Some(crate::errors::ErrorContext {
                        user_action: "Creating area".to_string(),
                        recovery_suggestions: vec![
                            "Try using a different title".to_string(),
                            "This might be a temporary issue, please try again".to_string(),
                        ],
                        recoverable: true,
                        help_url: None,
                    }),
                }
            },
            _ => AppError::from(e).with_context(crate::errors::ErrorContext {
                user_action: "Creating area".to_string(),
                recovery_suggestions: vec![
                    "Check if the database is accessible".to_string(),
                    "Try restarting the application".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        }
    })?;
    
    log::info!("Created area with ID: {}", area.id);
    Ok(area)
}

#[tauri::command]
pub async fn get_area(
    state: State<'_, AppState>,
    id: String,
) -> Result<Area, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let area = conn.query_row(
        "SELECT id, title, description, color, icon, created_at, updated_at, sort_order, tags 
         FROM areas WHERE id = ?1",
        params![&id],
        parse_area,
    ).map_err(|e| {
        match e {
            rusqlite::Error::QueryReturnedNoRows => {
                log::debug!("Area not found with ID: {}", id);
                AppError::entity_not_found("area", &id)
            },
            _ => {
                log::error!("Failed to get area {}: {}", id, e);
                AppError::from(e)
            }
        }
    })?;
    
    Ok(area)
}

#[tauri::command]
pub async fn get_all_areas(
    state: State<'_, AppState>,
) -> Result<Vec<Area>, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, title, description, color, icon, created_at, updated_at, sort_order, tags 
         FROM areas ORDER BY sort_order, title"
    )?;
    
    let areas = stmt.query_map([], parse_area)?
        .collect::<Result<Vec<_>, _>>()?;
    
    Ok(areas)
}

#[tauri::command]
pub async fn update_area(
    state: State<'_, AppState>,
    id: String,
    title: String,
    description: Option<String>,
    color: Option<String>,
    icon: Option<String>,
    sort_order: Option<i32>,
    tags: Option<Vec<String>>,
) -> Result<Area, AppError> {
    // Validate input
    if title.trim().is_empty() {
        return Err(AppError::missing_field("title"));
    }
    
    if title.len() > 255 {
        return Err(AppError::Validation {
            field: "title".to_string(),
            reason: "Title must be less than 255 characters".to_string(),
            code: crate::errors::ErrorCode::ValueOutOfRange,
            context: Some(crate::errors::ErrorContext {
                user_action: "Updating area".to_string(),
                recovery_suggestions: vec![
                    "Please shorten the title to less than 255 characters".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    let conn = get_db_connection_from_state(&state)?;
    
    // Get existing area to preserve fields that aren't being updated
    let existing_area = get_area(state.clone(), id.clone()).await?;
    
    let final_sort_order = sort_order.unwrap_or(existing_area.sort_order);
    let final_tags = tags.or(existing_area.tags);
    let tags_json = final_tags.as_ref()
        .and_then(|tags| serde_json::to_string(tags).ok());
    
    conn.execute(
        "UPDATE areas SET title = ?1, description = ?2, color = ?3, icon = ?4, 
         updated_at = ?5, sort_order = ?6, tags = ?7 WHERE id = ?8",
        params![
            &title,
            &description,
            &color,
            &icon,
            Utc::now().to_rfc3339(),
            final_sort_order,
            &tags_json,
            &id,
        ],
    ).map_err(|e| {
        log::error!("Failed to update area {}: {}", id, e);
        AppError::from(e).with_context(crate::errors::ErrorContext {
            user_action: "Updating area".to_string(),
            recovery_suggestions: vec![
                "Check if the area exists".to_string(),
                "Try refreshing and attempting the update again".to_string(),
            ],
            recoverable: true,
            help_url: None,
        })
    })?;
    
    log::info!("Updated area with ID: {}", id);
    get_area(state, id).await
}

#[tauri::command]
pub async fn delete_area(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    // First check if the area exists
    let area_exists = conn.query_row(
        "SELECT 1 FROM areas WHERE id = ?1",
        params![&id],
        |_| Ok(()),
    ).is_ok();
    
    if !area_exists {
        return Err(AppError::entity_not_found("area", &id));
    }
    
    // Check if area has any goals
    let goal_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM goals WHERE area_id = ?1",
        params![&id],
        |row| row.get(0),
    ).map_err(|e| {
        log::error!("Failed to check goals for area {}: {}", id, e);
        AppError::from(e)
    })?;
    
    if goal_count > 0 {
        return Err(AppError::Validation {
            field: "area_id".to_string(),
            reason: format!("Cannot delete area with {} associated goal(s). Please delete or reassign the goals first.", goal_count),
            code: ErrorCode::InvalidEntityReference,
            context: Some(crate::errors::ErrorContext {
                user_action: "Deleting area".to_string(),
                recovery_suggestions: vec![
                    "Delete all goals associated with this area first".to_string(),
                    "Move the goals to a different area before deleting".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    // Also check for any standalone tasks that might be associated with the area
    // (tasks without projects but conceptually under this area - future enhancement)
    
    conn.execute("DELETE FROM areas WHERE id = ?1", params![&id])
        .map_err(|e| {
            log::error!("Failed to delete area {}: {}", id, e);
            AppError::from(e).with_context(crate::errors::ErrorContext {
                user_action: "Deleting area".to_string(),
                recovery_suggestions: vec![
                    "The area may be in use by another process".to_string(),
                    "Try again after a moment".to_string(),
                ],
                recoverable: true,
                help_url: None,
            })
        })?;
    
    log::info!("Deleted area with ID: {}", id);
    Ok(())
}

// Goal Commands
#[tauri::command]
pub async fn create_goal(
    state: State<'_, AppState>,
    area_id: String,
    title: String,
    description: Option<String>,
    target_date: Option<String>,
    tags: Option<Vec<String>>,
    sort_order: Option<i32>,
) -> Result<Goal, AppError> {
    use crate::errors::ErrorContext;
    use log::{info, error};
    
    // Validate input
    if title.trim().is_empty() {
        return Err(AppError::missing_field("title"));
    }
    
    if title.len() > 255 {
        return Err(AppError::Validation {
            field: "title".to_string(),
            reason: "Title must be less than 255 characters".to_string(),
            code: ErrorCode::ValueOutOfRange,
            context: Some(ErrorContext {
                user_action: "Creating goal".to_string(),
                recovery_suggestions: vec![
                    "Please shorten the title to less than 255 characters".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    info!("Creating new goal: {} for area: {}", title, area_id);
    
    let conn = get_db_connection_from_state(&state)?;
    
    // Verify that the area exists
    let area_exists = conn.query_row(
        "SELECT 1 FROM areas WHERE id = ?1",
        params![&area_id],
        |_| Ok(()),
    ).is_ok();
    
    if !area_exists {
        return Err(AppError::entity_not_found("area", &area_id));
    }
    
    let mut goal = Goal::new(area_id.clone(), title, description);
    
    // Validate target date if provided
    if let Some(date_str) = &target_date {
        // Try to parse to validate the format
        DateTime::parse_from_rfc3339(date_str)
            .map_err(|_| AppError::invalid_format("target_date", "ISO 8601 date (e.g., 2024-01-01T00:00:00Z)"))?;
    }
    goal.target_date = target_date;
    goal.tags = tags;
    
    // Calculate sort_order if not provided
    if let Some(order) = sort_order {
        goal.sort_order = order;
    } else {
        // Get the maximum sort_order for goals in this area
        let max_order: Option<i32> = conn.query_row(
            "SELECT MAX(sort_order) FROM goals WHERE area_id = ?1",
            params![&area_id],
            |row| row.get(0),
        ).unwrap_or(None);
        goal.sort_order = max_order.unwrap_or(0) + 1;
    }
    
    // Validate the goal
    goal.validate()?;
    
    let tags_json = goal.tags.as_ref()
        .and_then(|tags| serde_json::to_string(tags).ok());
    
    conn.execute(
        "INSERT INTO goals (id, area_id, title, description, target_date, status, progress, created_at, updated_at, sort_order, tags)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            &goal.id,
            &goal.area_id,
            &goal.title,
            &goal.description,
            &goal.target_date,
            goal_status_to_string(&goal.status),
            goal.progress,
            goal.created_at.to_rfc3339(),
            goal.updated_at.to_rfc3339(),
            goal.sort_order,
            &tags_json,
        ],
    ).map_err(|e| {
        error!("Failed to create goal: {}", e);
        match &e {
            rusqlite::Error::SqliteFailure(err, _) if err.code == rusqlite::ErrorCode::ConstraintViolation => {
                AppError::Operation {
                    message: "A goal with this ID already exists".to_string(),
                    code: ErrorCode::EntityAlreadyExists,
                    severity: ErrorSeverity::Warning,
                    context: Some(ErrorContext {
                        user_action: "Creating goal".to_string(),
                        recovery_suggestions: vec![
                            "Try using a different title".to_string(),
                            "This might be a temporary issue, please try again".to_string(),
                        ],
                        recoverable: true,
                        help_url: None,
                    }),
                }
            },
            _ => AppError::from(e).with_context(ErrorContext {
                user_action: "Creating goal".to_string(),
                recovery_suggestions: vec![
                    "Check if the database is accessible".to_string(),
                    "Try restarting the application".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        }
    })?;
    
    // Update search index
    if let Err(e) = crate::database::search::index_entity(&conn, &goal.id) {
        error!("Failed to index goal in search: {}", e);
        // Don't fail the operation if search indexing fails
    }
    
    info!("Successfully created goal with ID: {}", goal.id);
    Ok(goal)
}

#[tauri::command]
pub async fn get_goals_by_area(
    state: State<'_, AppState>,
    area_id: String,
) -> Result<Vec<Goal>, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, area_id, title, description, target_date, status, progress, created_at, updated_at, sort_order, tags 
         FROM goals WHERE area_id = ?1 ORDER BY sort_order, title"
    )?;
    
    let goals = stmt.query_map(params![&area_id], |row| {
        let tags_json: Option<String> = row.get(10)?;
        let tags = tags_json.and_then(|json| serde_json::from_str(&json).ok());
        
        Ok(Goal {
            id: row.get(0)?,
            area_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            target_date: row.get(4)?,
            status: parse_goal_status(&row.get::<_, String>(5)?),
            progress: row.get(6)?,
            created_at: parse_datetime(&row.get::<_, String>(7)?, 7)?,
            updated_at: parse_datetime(&row.get::<_, String>(8)?, 8)?,
            sort_order: row.get(9)?,
            tags,
            entity_type: "goal".to_string(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(goals)
}

// Project Commands
#[tauri::command]
pub async fn create_project(
    state: State<'_, AppState>,
    goal_id: String,
    title: String,
    description: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    tags: Option<Vec<String>>,
    sort_order: Option<i32>,
) -> Result<Project, AppError> {
    use crate::errors::ErrorContext;
    use log::{info, error};
    
    // Validate input
    if title.trim().is_empty() {
        return Err(AppError::missing_field("title"));
    }
    
    if title.len() > 255 {
        return Err(AppError::Validation {
            field: "title".to_string(),
            reason: "Title must be less than 255 characters".to_string(),
            code: ErrorCode::ValueOutOfRange,
            context: Some(ErrorContext {
                user_action: "Creating project".to_string(),
                recovery_suggestions: vec![
                    "Please shorten the title to less than 255 characters".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    info!("Creating new project for goal {}: {}", goal_id, title);
    
    let conn = get_db_connection_from_state(&state)?;
    
    // Check if goal exists
    let goal_exists = conn.query_row(
        "SELECT 1 FROM goals WHERE id = ?1",
        params![&goal_id],
        |_| Ok(()),
    ).is_ok();
    
    if !goal_exists {
        return Err(AppError::entity_not_found("goal", &goal_id)
            .with_context(ErrorContext {
                user_action: "Creating project".to_string(),
                recovery_suggestions: vec![
                    "Ensure the goal exists before creating a project".to_string(),
                    "Check if you have the correct goal ID".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }));
    }
    
    // Calculate sort_order if not provided
    let final_sort_order = if let Some(order) = sort_order {
        order
    } else {
        // Get the maximum sort_order for projects in this goal
        let max_order: Option<i32> = conn.query_row(
            "SELECT MAX(sort_order) FROM projects WHERE goal_id = ?1",
            params![&goal_id],
            |row| row.get(0),
        ).unwrap_or(None);
        max_order.unwrap_or(0) + 10
    };
    
    // Validate dates if provided
    if let Some(date_str) = &start_date {
        DateTime::parse_from_rfc3339(date_str)
            .map_err(|_| AppError::invalid_format("start_date", "ISO 8601 date (e.g., 2024-01-01T00:00:00Z)"))?;
    }
    if let Some(date_str) = &end_date {
        DateTime::parse_from_rfc3339(date_str)
            .map_err(|_| AppError::invalid_format("end_date", "ISO 8601 date (e.g., 2024-01-01T00:00:00Z)"))?;
    }
    
    // Create project with provided values
    let mut project = Project::new(goal_id, title, description);
    project.start_date = start_date;
    project.end_date = end_date;
    project.tags = tags;
    project.sort_order = final_sort_order;
    
    // Validate the project
    project.validate()?;
    
    let tags_json = project.tags.as_ref()
        .and_then(|tags| serde_json::to_string(tags).ok());
    
    conn.execute(
        "INSERT INTO projects (id, goal_id, title, description, status, start_date, end_date, progress, created_at, updated_at, sort_order, tags)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            &project.id,
            &project.goal_id,
            &project.title,
            &project.description,
            project_status_to_string(&project.status),
            &project.start_date,
            &project.end_date,
            project.progress,
            project.created_at.to_rfc3339(),
            project.updated_at.to_rfc3339(),
            project.sort_order,
            &tags_json,
        ],
    ).map_err(|e| {
        error!("Failed to create project: {}", e);
        match &e {
            rusqlite::Error::SqliteFailure(err, _) if err.code == rusqlite::ErrorCode::ConstraintViolation => {
                AppError::Operation {
                    message: "A project with this ID already exists".to_string(),
                    code: ErrorCode::EntityAlreadyExists,
                    severity: ErrorSeverity::Warning,
                    context: Some(ErrorContext {
                        user_action: "Creating project".to_string(),
                        recovery_suggestions: vec![
                            "Try using a different title".to_string(),
                            "This might be a temporary issue, please try again".to_string(),
                        ],
                        recoverable: true,
                        help_url: None,
                    }),
                }
            },
            _ => AppError::from(e).with_context(ErrorContext {
                user_action: "Creating project".to_string(),
                recovery_suggestions: vec![
                    "Check if the goal exists".to_string(),
                    "Ensure dates are valid".to_string(),
                    "Try restarting the application".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        }
    })?;
    
    // Index for search
    if let Err(e) = crate::database::search::index_entity(&conn, &project.id) {
        error!("Failed to index new project: {}", e);
        // Don't fail the operation if indexing fails
    }
    
    info!("Successfully created project with ID: {}", project.id);
    Ok(project)
}

#[tauri::command]
pub async fn get_projects_by_goal(
    state: State<'_, AppState>,
    goal_id: String,
) -> Result<Vec<Project>, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, goal_id, title, description, status, start_date, end_date, progress, created_at, updated_at, sort_order, tags 
         FROM projects WHERE goal_id = ?1 ORDER BY sort_order, title"
    )?;
    
    let projects = stmt.query_map(params![&goal_id], |row| {
        let tags_json: Option<String> = row.get(11)?;
        let tags = tags_json.and_then(|json| serde_json::from_str(&json).ok());
        
        Ok(Project {
            id: row.get(0)?,
            goal_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            status: parse_project_status(&row.get::<_, String>(4)?),
            start_date: row.get(5)?,
            end_date: row.get(6)?,
            progress: row.get(7)?,
            created_at: parse_datetime(&row.get::<_, String>(8)?, 8)?,
            updated_at: parse_datetime(&row.get::<_, String>(9)?, 9)?,
            sort_order: row.get(10)?,
            tags,
            entity_type: "project".to_string(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(projects)
}

// Update Goal
#[tauri::command]
pub async fn update_goal(
    state: State<'_, AppState>,
    id: String,
    title: String,
    description: Option<String>,
    target_date: Option<String>,
    status: Option<String>,
    progress: Option<i32>,
    sort_order: Option<i32>,
    tags: Option<Vec<String>>,
) -> Result<Goal, AppError> {
    use crate::errors::ErrorContext;
    use log::{info, error};
    
    // Validate input
    if title.trim().is_empty() {
        return Err(AppError::missing_field("title"));
    }
    
    if title.len() > 255 {
        return Err(AppError::Validation {
            field: "title".to_string(),
            reason: "Title must be less than 255 characters".to_string(),
            code: ErrorCode::ValueOutOfRange,
            context: Some(ErrorContext {
                user_action: "Updating goal".to_string(),
                recovery_suggestions: vec![
                    "Please shorten the title to less than 255 characters".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    info!("Updating goal with ID: {}", id);
    
    let conn = get_db_connection_from_state(&state)?;
    
    // Get existing goal to preserve fields that aren't being updated
    let existing_goal = get_goal(state.clone(), id.clone()).await?;
    
    // Use provided values or fall back to existing ones
    let final_status = status.map(|s| parse_goal_status(&s)).unwrap_or(existing_goal.status);
    let final_progress = progress.unwrap_or(existing_goal.progress);
    let final_sort_order = sort_order.unwrap_or(existing_goal.sort_order);
    let final_tags = tags.or(existing_goal.tags);
    
    // Validate target date if provided
    if let Some(date_str) = &target_date {
        DateTime::parse_from_rfc3339(date_str)
            .map_err(|_| AppError::invalid_format("target_date", "ISO 8601 date (e.g., 2024-01-01T00:00:00Z)"))?;
    }
    
    // Validate progress is within range
    if final_progress < 0 || final_progress > 100 {
        return Err(AppError::Validation {
            field: "progress".to_string(),
            reason: "Progress must be between 0 and 100".to_string(),
            code: ErrorCode::ValueOutOfRange,
            context: Some(ErrorContext {
                user_action: "Updating goal progress".to_string(),
                recovery_suggestions: vec![
                    "Set progress to a value between 0 and 100".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    let tags_json = final_tags.as_ref()
        .and_then(|tags| serde_json::to_string(tags).ok());
    
    conn.execute(
        "UPDATE goals SET title = ?1, description = ?2, target_date = ?3, status = ?4, 
         progress = ?5, sort_order = ?6, tags = ?7, updated_at = ?8 WHERE id = ?9",
        params![
            &title,
            &description,
            &target_date,
            goal_status_to_string(&final_status),
            final_progress,
            final_sort_order,
            &tags_json,
            Utc::now().to_rfc3339(),
            &id,
        ],
    ).map_err(|e| {
        error!("Failed to update goal {}: {}", id, e);
        AppError::from(e).with_context(ErrorContext {
            user_action: "Updating goal".to_string(),
            recovery_suggestions: vec![
                "Check if the goal exists".to_string(),
                "Try refreshing and attempting the update again".to_string(),
            ],
            recoverable: true,
            help_url: None,
        })
    })?;
    
    // Update search index
    if let Err(e) = crate::database::search::index_entity(&conn, &id) {
        error!("Failed to update goal in search index: {}", e);
        // Don't fail the operation if search indexing fails
    }
    
    info!("Successfully updated goal with ID: {}", id);
    get_goal(state, id).await
}

// Delete Goal
#[tauri::command]
pub async fn delete_goal(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    use crate::errors::ErrorContext;
    use log::{info, error};
    
    info!("Attempting to delete goal with ID: {}", id);
    
    let conn = get_db_connection_from_state(&state)?;
    
    // First check if the goal exists
    let goal_exists = conn.query_row(
        "SELECT 1 FROM goals WHERE id = ?1",
        params![&id],
        |_| Ok(()),
    ).is_ok();
    
    if !goal_exists {
        return Err(AppError::entity_not_found("goal", &id));
    }
    
    // Check if goal has any projects
    let project_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM projects WHERE goal_id = ?1",
        params![&id],
        |row| row.get(0),
    ).map_err(|e| {
        error!("Failed to check projects for goal {}: {}", id, e);
        AppError::from(e)
    })?;
    
    if project_count > 0 {
        return Err(AppError::Validation {
            field: "goal_id".to_string(),
            reason: format!("Cannot delete goal with {} associated project(s). Please delete or reassign the projects first.", project_count),
            code: ErrorCode::InvalidEntityReference,
            context: Some(ErrorContext {
                user_action: "Deleting goal".to_string(),
                recovery_suggestions: vec![
                    "Delete all projects associated with this goal first".to_string(),
                    "Move the projects to a different goal before deleting".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    // Delete from search index first
    if let Err(e) = conn.execute(
        "DELETE FROM search_index WHERE entity_id = ?1",
        params![&id],
    ) {
        error!("Failed to remove goal from search index: {}", e);
        // Continue with deletion even if search index cleanup fails
    }
    
    conn.execute("DELETE FROM goals WHERE id = ?1", params![&id])
        .map_err(|e| {
            error!("Failed to delete goal {}: {}", id, e);
            AppError::from(e).with_context(ErrorContext {
                user_action: "Deleting goal".to_string(),
                recovery_suggestions: vec![
                    "The goal may be in use by another process".to_string(),
                    "Try again after a moment".to_string(),
                ],
                recoverable: true,
                help_url: None,
            })
        })?;
    
    info!("Successfully deleted goal with ID: {}", id);
    Ok(())
}

// Get single Goal
#[tauri::command]
pub async fn get_goal(
    state: State<'_, AppState>,
    id: String,
) -> Result<Goal, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let goal = conn.query_row(
        "SELECT id, area_id, title, description, target_date, status, progress, created_at, updated_at, sort_order, tags 
         FROM goals WHERE id = ?1",
        params![&id],
        |row| {
            let tags_json: Option<String> = row.get(10)?;
            let tags = tags_json.and_then(|json| serde_json::from_str(&json).ok());
            
            Ok(Goal {
                id: row.get(0)?,
                area_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                target_date: row.get(4)?,
                status: parse_goal_status(&row.get::<_, String>(5)?),
                progress: row.get(6)?,
                created_at: parse_datetime(&row.get::<_, String>(7)?, 7)?,
                updated_at: parse_datetime(&row.get::<_, String>(8)?, 8)?,
                sort_order: row.get(9)?,
                tags,
                entity_type: "goal".to_string(),
            })
        },
    )?;
    
    Ok(goal)
}

// Get all Goals
#[tauri::command]
pub async fn get_all_goals(
    state: State<'_, AppState>,
) -> Result<Vec<Goal>, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, area_id, title, description, target_date, status, progress, created_at, updated_at, sort_order, tags 
         FROM goals ORDER BY sort_order, title"
    )?;
    
    let goals = stmt.query_map([], |row| {
        let tags_json: Option<String> = row.get(10)?;
        let tags = tags_json.and_then(|json| serde_json::from_str(&json).ok());
        
        Ok(Goal {
            id: row.get(0)?,
            area_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            target_date: row.get(4)?,
            status: parse_goal_status(&row.get::<_, String>(5)?),
            progress: row.get(6)?,
            created_at: parse_datetime(&row.get::<_, String>(7)?, 7)?,
            updated_at: parse_datetime(&row.get::<_, String>(8)?, 8)?,
            sort_order: row.get(9)?,
            tags,
            entity_type: "goal".to_string(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(goals)
}

// Update Project
#[tauri::command]
pub async fn update_project(
    state: State<'_, AppState>,
    id: String,
    title: String,
    description: Option<String>,
    status: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
    progress: Option<i32>,
    sort_order: Option<i32>,
    tags: Option<Vec<String>>,
) -> Result<Project, AppError> {
    use crate::errors::ErrorContext;
    use log::{info, error};
    
    // Validate input
    if title.trim().is_empty() {
        return Err(AppError::missing_field("title"));
    }
    
    if title.len() > 255 {
        return Err(AppError::Validation {
            field: "title".to_string(),
            reason: "Title must be less than 255 characters".to_string(),
            code: ErrorCode::ValueOutOfRange,
            context: Some(ErrorContext {
                user_action: "Updating project".to_string(),
                recovery_suggestions: vec![
                    "Please shorten the title to less than 255 characters".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    info!("Updating project with ID: {}", id);
    
    let conn = get_db_connection_from_state(&state)?;
    
    // Get existing project to preserve fields that aren't being updated
    let existing_project = get_project(state.clone(), id.clone()).await?;
    
    // Use provided values or fall back to existing ones
    let final_status = status.map(|s| parse_project_status(&s)).unwrap_or(existing_project.status);
    let final_progress = progress.unwrap_or(existing_project.progress);
    let final_sort_order = sort_order.unwrap_or(existing_project.sort_order);
    let final_tags = tags.or(existing_project.tags);
    
    // Validate dates if provided
    if let Some(date_str) = &start_date {
        DateTime::parse_from_rfc3339(date_str)
            .map_err(|_| AppError::invalid_format("start_date", "ISO 8601 date (e.g., 2024-01-01T00:00:00Z)"))?;
    }
    if let Some(date_str) = &end_date {
        DateTime::parse_from_rfc3339(date_str)
            .map_err(|_| AppError::invalid_format("end_date", "ISO 8601 date (e.g., 2024-01-01T00:00:00Z)"))?;
    }
    
    // Validate date ordering
    if let (Some(start), Some(end)) = (&start_date, &end_date) {
        if start > end {
            return Err(AppError::Validation {
                field: "dates".to_string(),
                reason: "Start date must be before end date".to_string(),
                code: ErrorCode::ValueOutOfRange,
                context: Some(ErrorContext {
                    user_action: "Updating project dates".to_string(),
                    recovery_suggestions: vec![
                        "Ensure start date is before end date".to_string(),
                        "Check your date values".to_string(),
                    ],
                    recoverable: true,
                    help_url: None,
                }),
            });
        }
    }
    
    // Validate progress is within range
    if final_progress < 0 || final_progress > 100 {
        return Err(AppError::Validation {
            field: "progress".to_string(),
            reason: "Progress must be between 0 and 100".to_string(),
            code: ErrorCode::ValueOutOfRange,
            context: Some(ErrorContext {
                user_action: "Updating project progress".to_string(),
                recovery_suggestions: vec![
                    "Set progress to a value between 0 and 100".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    let tags_json = final_tags.as_ref()
        .and_then(|tags| serde_json::to_string(tags).ok());
    
    conn.execute(
        "UPDATE projects SET title = ?1, description = ?2, status = ?3, start_date = ?4, 
         end_date = ?5, progress = ?6, sort_order = ?7, tags = ?8, updated_at = ?9 WHERE id = ?10",
        params![
            &title,
            &description,
            project_status_to_string(&final_status),
            &start_date,
            &end_date,
            final_progress,
            final_sort_order,
            &tags_json,
            Utc::now().to_rfc3339(),
            &id,
        ],
    ).map_err(|e| {
        error!("Failed to update project {}: {}", id, e);
        AppError::from(e).with_context(ErrorContext {
            user_action: "Updating project".to_string(),
            recovery_suggestions: vec![
                "Check if the project exists".to_string(),
                "Try refreshing and attempting the update again".to_string(),
            ],
            recoverable: true,
            help_url: None,
        })
    })?;
    
    // Update search index
    if let Err(e) = crate::database::search::index_entity(&conn, &id) {
        error!("Failed to update project in search index: {}", e);
        // Don't fail the operation if search indexing fails
    }
    
    info!("Successfully updated project with ID: {}", id);
    get_project(state, id).await
}

// Delete Project
#[tauri::command]
pub async fn delete_project(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    use crate::errors::ErrorContext;
    use log::{info, error};
    
    info!("Attempting to delete project with ID: {}", id);
    
    let conn = get_db_connection_from_state(&state)?;
    
    // First check if the project exists
    let project_exists = conn.query_row(
        "SELECT 1 FROM projects WHERE id = ?1",
        params![&id],
        |_| Ok(()),
    ).is_ok();
    
    if !project_exists {
        return Err(AppError::entity_not_found("project", &id));
    }
    
    // Check if project has any tasks
    let task_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM tasks WHERE project_id = ?1",
        params![&id],
        |row| row.get(0),
    ).map_err(|e| {
        error!("Failed to check tasks for project {}: {}", id, e);
        AppError::from(e)
    })?;
    
    if task_count > 0 {
        return Err(AppError::Validation {
            field: "project_id".to_string(),
            reason: format!("Cannot delete project with {} associated task(s). Please delete or reassign the tasks first.", task_count),
            code: ErrorCode::InvalidEntityReference,
            context: Some(ErrorContext {
                user_action: "Deleting project".to_string(),
                recovery_suggestions: vec![
                    "Delete all tasks associated with this project first".to_string(),
                    "Move the tasks to a different project before deleting".to_string(),
                    "Mark the project as completed instead of deleting it".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    // Delete from search index first
    if let Err(e) = conn.execute(
        "DELETE FROM search_index WHERE entity_id = ?1",
        params![&id],
    ) {
        error!("Failed to remove project from search index: {}", e);
        // Continue with deletion even if search index cleanup fails
    }
    
    conn.execute("DELETE FROM projects WHERE id = ?1", params![&id])
        .map_err(|e| {
            error!("Failed to delete project {}: {}", id, e);
            AppError::from(e).with_context(ErrorContext {
                user_action: "Deleting project".to_string(),
                recovery_suggestions: vec![
                    "The project may be in use by another process".to_string(),
                    "Try again after a moment".to_string(),
                ],
                recoverable: true,
                help_url: None,
            })
        })?;
    
    info!("Successfully deleted project with ID: {}", id);
    Ok(())
}

// Get single Project
#[tauri::command]
pub async fn get_project(
    state: State<'_, AppState>,
    id: String,
) -> Result<Project, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let project = conn.query_row(
        "SELECT id, goal_id, title, description, status, start_date, end_date, progress, created_at, updated_at, sort_order, tags 
         FROM projects WHERE id = ?1",
        params![&id],
        |row| {
            let tags_json: Option<String> = row.get(11)?;
            let tags = tags_json.and_then(|json| serde_json::from_str(&json).ok());
            
            Ok(Project {
                id: row.get(0)?,
                goal_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                status: parse_project_status(&row.get::<_, String>(4)?),
                start_date: row.get(5)?,
                end_date: row.get(6)?,
                progress: row.get(7)?,
                created_at: parse_datetime(&row.get::<_, String>(8)?, 8)?,
                updated_at: parse_datetime(&row.get::<_, String>(9)?, 9)?,
                sort_order: row.get(10)?,
                tags,
                entity_type: "project".to_string(),
            })
        },
    ).map_err(|e| {
        match e {
            rusqlite::Error::QueryReturnedNoRows => {
                log::debug!("Project not found with ID: {}", id);
                AppError::entity_not_found("project", &id)
            },
            _ => {
                log::error!("Failed to get project {}: {}", id, e);
                AppError::from(e)
            }
        }
    })?;
    
    Ok(project)
}

// Get all Projects
#[tauri::command]
pub async fn get_all_projects(
    state: State<'_, AppState>,
) -> Result<Vec<Project>, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, goal_id, title, description, status, start_date, end_date, progress, created_at, updated_at, sort_order, tags 
         FROM projects ORDER BY sort_order, title"
    )?;
    
    let projects = stmt.query_map([], |row| {
        let tags_json: Option<String> = row.get(11)?;
        let tags = tags_json.and_then(|json| serde_json::from_str(&json).ok());
        
        Ok(Project {
            id: row.get(0)?,
            goal_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            status: parse_project_status(&row.get::<_, String>(4)?),
            start_date: row.get(5)?,
            end_date: row.get(6)?,
            progress: row.get(7)?,
            created_at: parse_datetime(&row.get::<_, String>(8)?, 8)?,
            updated_at: parse_datetime(&row.get::<_, String>(9)?, 9)?,
            sort_order: row.get(10)?,
            tags,
            entity_type: "project".to_string(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(projects)
}

// Task Commands
#[tauri::command]
pub async fn create_task(
    state: State<'_, AppState>,
    title: String,
    description: Option<String>,
    project_id: Option<String>,
    parent_task_id: Option<String>,
    due_date: Option<String>,
    priority: Option<String>,
    estimated_minutes: Option<i32>,
    tags: Option<Vec<String>>,
    sort_order: Option<i32>,
) -> Result<Task, AppError> {
    use crate::errors::ErrorContext;
    use log::{info, error};
    
    // Validate input
    if title.trim().is_empty() {
        return Err(AppError::missing_field("title"));
    }
    
    if title.len() > 255 {
        return Err(AppError::Validation {
            field: "title".to_string(),
            reason: "Title must be less than 255 characters".to_string(),
            code: ErrorCode::ValueOutOfRange,
            context: Some(ErrorContext {
                user_action: "Creating task".to_string(),
                recovery_suggestions: vec![
                    "Please shorten the title to less than 255 characters".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    info!("Creating new task: {}", title);
    
    let mut task = Task::new(title, description);
    task.project_id = project_id.clone();
    task.parent_task_id = parent_task_id.clone();
    task.tags = tags;
    task.estimated_minutes = estimated_minutes;
    
    // Calculate sort_order if not provided
    let conn = get_db_connection_from_state(&state)?;
    
    if let Some(order) = sort_order {
        task.sort_order = order;
    } else {
        // Calculate next sort order based on context
        let query = if let Some(ref _parent_id) = parent_task_id {
            // For subtasks, get max sort_order within parent
            "SELECT MAX(sort_order) FROM tasks WHERE parent_task_id = ?1"
        } else if let Some(ref _proj_id) = project_id {
            // For project tasks, get max sort_order within project
            "SELECT MAX(sort_order) FROM tasks WHERE project_id = ?1 AND parent_task_id IS NULL"
        } else {
            // For standalone tasks
            "SELECT MAX(sort_order) FROM tasks WHERE project_id IS NULL AND parent_task_id IS NULL"
        };
        
        let max_order: Option<i32> = if let Some(ref parent_id) = parent_task_id {
            conn.query_row(query, params![parent_id], |row| row.get(0)).ok().flatten()
        } else if let Some(ref proj_id) = project_id {
            conn.query_row(query, params![proj_id], |row| row.get(0)).ok().flatten()
        } else {
            conn.query_row(query, [], |row| row.get(0)).ok().flatten()
        };
        
        task.sort_order = max_order.unwrap_or(0) + 1;
    }
    
    // Validate due date if provided
    if let Some(date_str) = &due_date {
        DateTime::parse_from_rfc3339(date_str)
            .map_err(|_| AppError::invalid_format("due_date", "ISO 8601 date (e.g., 2024-01-01T00:00:00Z)"))?;
    }
    task.due_date = due_date;
    
    // Parse priority
    if let Some(p) = priority {
        task.priority = match p.as_str() {
            "urgent" => TaskPriority::Urgent,
            "high" => TaskPriority::High,
            "medium" => TaskPriority::Medium,
            "low" => TaskPriority::Low,
            _ => {
                error!("Invalid priority value: {}, defaulting to medium", p);
                TaskPriority::Medium
            }
        };
    }
    
    // Validate parent relationships if provided
    if let Some(ref proj_id) = task.project_id {
        let project_exists = conn.query_row(
            "SELECT 1 FROM projects WHERE id = ?1",
            params![proj_id],
            |_| Ok(()),
        ).is_ok();
        
        if !project_exists {
            return Err(AppError::entity_not_found("project", proj_id));
        }
    }
    
    if let Some(ref parent_id) = task.parent_task_id {
        let parent_exists = conn.query_row(
            "SELECT 1 FROM tasks WHERE id = ?1",
            params![parent_id],
            |_| Ok(()),
        ).is_ok();
        
        if !parent_exists {
            return Err(AppError::entity_not_found("parent task", parent_id));
        }
    }
    
    // Validate the task
    task.validate()?;
    
    let tags_json = task.tags.as_ref()
        .and_then(|tags| serde_json::to_string(tags).ok());
    
    conn.execute(
        "INSERT INTO tasks (id, project_id, parent_task_id, title, description, status, due_date, priority, 
         completed_at, estimated_minutes, actual_minutes, created_at, updated_at, sort_order, tags, 
         recurrence, recurrence_id, recurrence_date)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
        params![
            &task.id,
            &task.project_id,
            &task.parent_task_id,
            &task.title,
            &task.description,
            task_status_to_string(&task.status),
            &task.due_date,
            task_priority_to_string(&task.priority),
            &task.completed_at,
            task.estimated_minutes,
            task.actual_minutes,
            task.created_at.to_rfc3339(),
            task.updated_at.to_rfc3339(),
            task.sort_order,
            &tags_json,
            task.recurrence.as_ref()
                .and_then(|r| serde_json::to_string(r).ok()),
            &task.recurrence_id,
            &task.recurrence_date,
        ],
    ).map_err(|e| {
        error!("Failed to create task: {}", e);
        AppError::from(e).with_context(ErrorContext {
            user_action: "Creating task".to_string(),
            recovery_suggestions: vec![
                "Check that all parent entities exist".to_string(),
                "Ensure the data is valid and try again".to_string(),
            ],
            recoverable: true,
            help_url: None,
        })
    })?;
    
    // Index for search
    if let Err(e) = crate::database::search::index_entity(&conn, &task.id) {
        error!("Failed to index task in search: {}", e);
        // Don't fail the operation if search indexing fails
    }
    
    info!("Successfully created task with ID: {}", task.id);
    Ok(task)
}

#[tauri::command]
pub async fn get_tasks_by_project(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<Vec<Task>, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, 
         completed_at, estimated_minutes, actual_minutes, created_at, updated_at, sort_order, tags, 
         recurrence, recurrence_id, recurrence_date 
         FROM tasks WHERE project_id = ?1 ORDER BY sort_order, created_at"
    )?;
    
    let tasks = stmt.query_map(params![&project_id], |row| {
        let tags_json: Option<String> = row.get(14)?;
        let tags = tags_json.and_then(|json| serde_json::from_str(&json).ok());
        let recurrence_json: Option<String> = row.get(15)?;
        let recurrence = recurrence_json.and_then(|json| serde_json::from_str(&json).ok());
        
        Ok(Task {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_task_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            status: parse_task_status(&row.get::<_, String>(5)?),
            due_date: row.get(6)?,
            priority: parse_task_priority(row.get(7)?),
            completed_at: row.get(8)?,
            estimated_minutes: row.get(9)?,
            actual_minutes: row.get(10)?,
            created_at: parse_datetime(&row.get::<_, String>(11)?, 11)?,
            updated_at: parse_datetime(&row.get::<_, String>(12)?, 12)?,
            sort_order: row.get(13)?,
            tags,
            recurrence,
            recurrence_id: row.get(16)?,
            recurrence_date: row.get(17)?,
            entity_type: "task".to_string(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(tasks)
}

#[tauri::command]
pub async fn update_task_status(
    state: State<'_, AppState>,
    id: String,
    status: String,
) -> Result<(), AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    // Validate status
    let task_status = parse_task_status(&status);
    
    // Set completed_at if completing the task
    let completed_at = if task_status == TaskStatus::Completed {
        Some(Utc::now().to_rfc3339())
    } else {
        None
    };
    
    conn.execute(
        "UPDATE tasks SET status = ?1, completed_at = ?2, updated_at = ?3 WHERE id = ?4",
        params![&status, &completed_at, Utc::now().to_rfc3339(), &id],
    )?;
    
    Ok(())
}

#[tauri::command]
pub async fn delete_task(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), AppError> {
    use crate::errors::ErrorContext;
    use log::{info, error};
    
    info!("Attempting to delete task with ID: {}", id);
    
    let conn = get_db_connection_from_state(&state)?;
    
    // First check if the task exists
    let task_exists = conn.query_row(
        "SELECT 1 FROM tasks WHERE id = ?1",
        params![&id],
        |_| Ok(()),
    ).is_ok();
    
    if !task_exists {
        return Err(AppError::entity_not_found("task", &id));
    }
    
    // Check if task has subtasks
    let subtask_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM tasks WHERE parent_task_id = ?1",
        params![&id],
        |row| row.get(0),
    ).map_err(|e| {
        error!("Failed to check subtasks for task {}: {}", id, e);
        AppError::from(e)
    })?;
    
    if subtask_count > 0 {
        return Err(AppError::Validation {
            field: "task_id".to_string(),
            reason: format!("Cannot delete task with {} subtask(s). Please delete or reassign the subtasks first.", subtask_count),
            code: ErrorCode::InvalidEntityReference,
            context: Some(ErrorContext {
                user_action: "Deleting task".to_string(),
                recovery_suggestions: vec![
                    "Delete all subtasks first".to_string(),
                    "Move the subtasks to a different parent task before deleting".to_string(),
                    format!("This task has {} subtask(s) that depend on it", subtask_count),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    // Delete from search index first
    if let Err(e) = conn.execute(
        "DELETE FROM search_index WHERE entity_id = ?1",
        params![&id],
    ) {
        error!("Failed to remove task from search index: {}", e);
        // Continue with deletion even if search index cleanup fails
    }
    
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![&id])
        .map_err(|e| {
            error!("Failed to delete task {}: {}", id, e);
            AppError::from(e).with_context(ErrorContext {
                user_action: "Deleting task".to_string(),
                recovery_suggestions: vec![
                    "The task may be in use by another process".to_string(),
                    "Try again after a moment".to_string(),
                ],
                recoverable: true,
                help_url: None,
            })
        })?;
    
    info!("Successfully deleted task with ID: {}", id);
    Ok(())
}

// Get single Task
#[tauri::command]
pub async fn get_task(
    state: State<'_, AppState>,
    id: String,
) -> Result<Task, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let task = conn.query_row(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, 
         completed_at, estimated_minutes, actual_minutes, created_at, updated_at, sort_order, tags,
         recurrence, recurrence_id, recurrence_date
         FROM tasks WHERE id = ?1",
        params![&id],
        |row| {
            Ok(Task {
                id: row.get(0)?,
                project_id: row.get(1)?,
                parent_task_id: row.get(2)?,
                title: row.get(3)?,
                description: row.get(4)?,
                status: parse_task_status(&row.get::<_, String>(5)?),
                due_date: row.get(6)?,
                priority: parse_task_priority(row.get::<_, Option<String>>(7)?),
                completed_at: row.get(8)?,
                estimated_minutes: row.get(9)?,
                actual_minutes: row.get(10)?,
                created_at: parse_datetime(&row.get::<_, String>(11)?, 11)?,
                updated_at: parse_datetime(&row.get::<_, String>(12)?, 12)?,
                sort_order: row.get(13)?,
                tags: row.get::<_, Option<String>>(14)?
                    .and_then(|s| serde_json::from_str(&s).ok()),
                entity_type: "task".to_string(),
                recurrence: row.get::<_, Option<String>>(15)?
                    .and_then(|s| serde_json::from_str(&s).ok()),
                recurrence_id: row.get(16)?,
                recurrence_date: row.get(17)?,
            })
        },
    ).map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => AppError::entity_not_found("task", &id),
        _ => AppError::from(e),
    })?;
    
    Ok(task)
}

// Get all Tasks
#[tauri::command]
pub async fn get_all_tasks(
    state: State<'_, AppState>,
) -> Result<Vec<Task>, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, 
         completed_at, estimated_minutes, actual_minutes, created_at, updated_at, sort_order, tags,
         recurrence, recurrence_id, recurrence_date
         FROM tasks ORDER BY sort_order, created_at DESC"
    )?;
    
    let tasks = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_task_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            status: parse_task_status(&row.get::<_, String>(5)?),
            due_date: row.get(6)?,
            priority: parse_task_priority(row.get::<_, Option<String>>(7)?),
            completed_at: row.get(8)?,
            estimated_minutes: row.get(9)?,
            actual_minutes: row.get(10)?,
            created_at: parse_datetime(&row.get::<_, String>(11)?, 11)?,
            updated_at: parse_datetime(&row.get::<_, String>(12)?, 12)?,
            sort_order: row.get(13)?,
            tags: row.get::<_, Option<String>>(14)?
                .and_then(|s| serde_json::from_str(&s).ok()),
            entity_type: "task".to_string(),
            recurrence: row.get::<_, Option<String>>(15)?
                .and_then(|s| serde_json::from_str(&s).ok()),
            recurrence_id: row.get(16)?,
            recurrence_date: row.get(17)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(tasks)
}

// Get all Tasks with pagination
#[tauri::command]
pub async fn get_all_tasks_paginated(
    state: State<'_, AppState>,
    pagination: Option<PaginationParams>,
) -> Result<PaginatedResponse<Task>, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    let pagination = pagination.unwrap_or_default();
    
    // First get the total count
    let total: u32 = conn.query_row(
        "SELECT COUNT(*) FROM tasks",
        [],
        |row| row.get(0),
    )?;
    
    // Then get the paginated results
    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, 
         completed_at, estimated_minutes, actual_minutes, created_at, updated_at, sort_order, tags,
         recurrence, recurrence_id, recurrence_date
         FROM tasks 
         ORDER BY sort_order, created_at DESC
         LIMIT ?1 OFFSET ?2"
    )?;
    
    let tasks = stmt.query_map(params![pagination.limit(), pagination.offset()], |row| {
        Ok(Task {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_task_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            status: parse_task_status(&row.get::<_, String>(5)?),
            due_date: row.get(6)?,
            priority: parse_task_priority(row.get::<_, Option<String>>(7)?),
            completed_at: row.get(8)?,
            estimated_minutes: row.get(9)?,
            actual_minutes: row.get(10)?,
            created_at: parse_datetime(&row.get::<_, String>(11)?, 11)?,
            updated_at: parse_datetime(&row.get::<_, String>(12)?, 12)?,
            sort_order: row.get(13)?,
            tags: row.get::<_, Option<String>>(14)?
                .and_then(|s| serde_json::from_str(&s).ok()),
            entity_type: "task".to_string(),
            recurrence: row.get::<_, Option<String>>(15)?
                .and_then(|s| serde_json::from_str(&s).ok()),
            recurrence_id: row.get(16)?,
            recurrence_date: row.get(17)?,
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    let page = pagination.page.unwrap_or(1).max(1);
    let per_page = pagination.limit();
    let total_pages = (total as f32 / per_page as f32).ceil() as u32;
    
    Ok(PaginatedResponse {
        items: tasks,
        total,
        page,
        per_page,
        total_pages,
    })
}

// Update Task
#[tauri::command]
pub async fn update_task(
    state: State<'_, AppState>,
    id: String,
    title: String,
    description: Option<String>,
    status: Option<String>,
    due_date: Option<String>,
    priority: Option<String>,
    estimated_minutes: Option<i32>,
    actual_minutes: Option<i32>,
    sort_order: Option<i32>,
    tags: Option<Vec<String>>,
) -> Result<Task, AppError> {
    use crate::errors::ErrorContext;
    use log::{info, error};
    
    // Validate input
    if title.trim().is_empty() {
        return Err(AppError::missing_field("title"));
    }
    
    if title.len() > 255 {
        return Err(AppError::Validation {
            field: "title".to_string(),
            reason: "Title must be less than 255 characters".to_string(),
            code: ErrorCode::ValueOutOfRange,
            context: Some(ErrorContext {
                user_action: "Updating task".to_string(),
                recovery_suggestions: vec![
                    "Please shorten the title to less than 255 characters".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        });
    }
    
    info!("Updating task with ID: {}", id);
    
    let conn = get_db_connection_from_state(&state)?;
    
    // Get existing task to preserve fields that aren't being updated
    let existing_task = get_task(state.clone(), id.clone()).await?;
    
    // Use provided values or fall back to existing ones
    let final_status = status.map(|s| parse_task_status(&s)).unwrap_or(existing_task.status.clone());
    let final_priority = priority.map(|p| parse_task_priority(Some(p))).unwrap_or(existing_task.priority.clone());
    let final_estimated = estimated_minutes.or(existing_task.estimated_minutes);
    let final_actual = actual_minutes.or(existing_task.actual_minutes);
    let final_sort_order = sort_order.unwrap_or(existing_task.sort_order);
    let final_tags = tags.or(existing_task.tags);
    
    // Validate due date if provided
    if let Some(date_str) = &due_date {
        DateTime::parse_from_rfc3339(date_str)
            .map_err(|_| AppError::invalid_format("due_date", "ISO 8601 date (e.g., 2024-01-01T00:00:00Z)"))?;
    }
    
    // Validate time estimates
    if let Some(est) = final_estimated {
        if est < 0 {
            return Err(AppError::Validation {
                field: "estimated_minutes".to_string(),
                reason: "Estimated minutes cannot be negative".to_string(),
                code: ErrorCode::ValueOutOfRange,
                context: Some(ErrorContext {
                    user_action: "Updating task estimates".to_string(),
                    recovery_suggestions: vec![
                        "Set estimated minutes to a positive value or 0".to_string(),
                    ],
                    recoverable: true,
                    help_url: None,
                }),
            });
        }
    }
    
    if let Some(act) = final_actual {
        if act < 0 {
            return Err(AppError::Validation {
                field: "actual_minutes".to_string(),
                reason: "Actual minutes cannot be negative".to_string(),
                code: ErrorCode::ValueOutOfRange,
                context: Some(ErrorContext {
                    user_action: "Updating task actuals".to_string(),
                    recovery_suggestions: vec![
                        "Set actual minutes to a positive value or 0".to_string(),
                    ],
                    recoverable: true,
                    help_url: None,
                }),
            });
        }
    }
    
    // Set completed_at if completing the task
    let completed_at = if final_status == TaskStatus::Completed {
        if existing_task.completed_at.is_none() {
            Some(Utc::now().to_rfc3339())
        } else {
            existing_task.completed_at
        }
    } else {
        None
    };
    
    let tags_json = final_tags.as_ref()
        .and_then(|tags| serde_json::to_string(tags).ok());
    
    conn.execute(
        "UPDATE tasks SET title = ?1, description = ?2, status = ?3, due_date = ?4, 
         priority = ?5, estimated_minutes = ?6, actual_minutes = ?7, completed_at = ?8,
         sort_order = ?9, tags = ?10, updated_at = ?11 WHERE id = ?12",
        params![
            &title,
            &description,
            task_status_to_string(&final_status),
            &due_date,
            task_priority_to_string(&final_priority),
            final_estimated,
            final_actual,
            &completed_at,
            final_sort_order,
            &tags_json,
            Utc::now().to_rfc3339(),
            &id,
        ],
    ).map_err(|e| {
        error!("Failed to update task {}: {}", id, e);
        AppError::from(e).with_context(ErrorContext {
            user_action: "Updating task".to_string(),
            recovery_suggestions: vec![
                "Check if the task exists".to_string(),
                "Try refreshing and attempting the update again".to_string(),
            ],
            recoverable: true,
            help_url: None,
        })
    })?;
    
    // Update search index
    if let Err(e) = crate::database::search::index_entity(&conn, &id) {
        error!("Failed to update task in search index: {}", e);
        // Don't fail the operation if search indexing fails
    }
    
    info!("Successfully updated task with ID: {}", id);
    get_task(state, id).await
}

// Task filtering commands
#[tauri::command]
pub async fn get_tasks_by_status(
    state: State<'_, AppState>,
    status: String,
) -> Result<Vec<Task>, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    // Validate status
    let _ = parse_task_status(&status); // Validate status format
    
    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, 
         completed_at, estimated_minutes, actual_minutes, created_at, updated_at, sort_order, tags, 
         recurrence, recurrence_id, recurrence_date 
         FROM tasks WHERE status = ?1 ORDER BY sort_order, created_at DESC"
    )?;
    
    let tasks = stmt.query_map(params![&status], |row| {
        let tags_json: Option<String> = row.get(14)?;
        let tags = tags_json.and_then(|json| serde_json::from_str(&json).ok());
        let recurrence_json: Option<String> = row.get(15)?;
        let recurrence = recurrence_json.and_then(|json| serde_json::from_str(&json).ok());
        
        Ok(Task {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_task_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            status: parse_task_status(&row.get::<_, String>(5)?),
            due_date: row.get(6)?,
            priority: parse_task_priority(row.get(7)?),
            completed_at: row.get(8)?,
            estimated_minutes: row.get(9)?,
            actual_minutes: row.get(10)?,
            created_at: parse_datetime(&row.get::<_, String>(11)?, 11)?,
            updated_at: parse_datetime(&row.get::<_, String>(12)?, 12)?,
            sort_order: row.get(13)?,
            tags,
            recurrence,
            recurrence_id: row.get(16)?,
            recurrence_date: row.get(17)?,
            entity_type: "task".to_string(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(tasks)
}

#[tauri::command]
pub async fn get_upcoming_tasks(
    state: State<'_, AppState>,
    days: i64,
) -> Result<Vec<Task>, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let future_date = Utc::now() + chrono::Duration::days(days);
    
    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, 
         completed_at, estimated_minutes, actual_minutes, created_at, updated_at, sort_order, tags, 
         recurrence, recurrence_id, recurrence_date 
         FROM tasks 
         WHERE due_date IS NOT NULL 
         AND date(due_date) <= date(?)
         AND status != 'completed'
         AND status != 'cancelled'
         ORDER BY due_date ASC"
    )?;
    
    let tasks = stmt.query_map(params![future_date.to_rfc3339()], |row| {
        let tags_json: Option<String> = row.get(14)?;
        let tags = tags_json.and_then(|json| serde_json::from_str(&json).ok());
        let recurrence_json: Option<String> = row.get(15)?;
        let recurrence = recurrence_json.and_then(|json| serde_json::from_str(&json).ok());
        
        Ok(Task {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_task_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            status: parse_task_status(&row.get::<_, String>(5)?),
            due_date: row.get(6)?,
            priority: parse_task_priority(row.get(7)?),
            completed_at: row.get(8)?,
            estimated_minutes: row.get(9)?,
            actual_minutes: row.get(10)?,
            created_at: parse_datetime(&row.get::<_, String>(11)?, 11)?,
            updated_at: parse_datetime(&row.get::<_, String>(12)?, 12)?,
            sort_order: row.get(13)?,
            tags,
            recurrence,
            recurrence_id: row.get(16)?,
            recurrence_date: row.get(17)?,
            entity_type: "task".to_string(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(tasks)
}

#[tauri::command]
pub async fn get_subtasks(
    state: State<'_, AppState>,
    parent_task_id: String,
) -> Result<Vec<Task>, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, 
         completed_at, estimated_minutes, actual_minutes, created_at, updated_at, sort_order, tags, 
         recurrence, recurrence_id, recurrence_date 
         FROM tasks WHERE parent_task_id = ?1 ORDER BY sort_order, created_at"
    )?;
    
    let tasks = stmt.query_map(params![&parent_task_id], |row| {
        let tags_json: Option<String> = row.get(14)?;
        let tags = tags_json.and_then(|json| serde_json::from_str(&json).ok());
        let recurrence_json: Option<String> = row.get(15)?;
        let recurrence = recurrence_json.and_then(|json| serde_json::from_str(&json).ok());
        
        Ok(Task {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_task_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            status: parse_task_status(&row.get::<_, String>(5)?),
            due_date: row.get(6)?,
            priority: parse_task_priority(row.get(7)?),
            completed_at: row.get(8)?,
            estimated_minutes: row.get(9)?,
            actual_minutes: row.get(10)?,
            created_at: parse_datetime(&row.get::<_, String>(11)?, 11)?,
            updated_at: parse_datetime(&row.get::<_, String>(12)?, 12)?,
            sort_order: row.get(13)?,
            tags,
            recurrence,
            recurrence_id: row.get(16)?,
            recurrence_date: row.get(17)?,
            entity_type: "task".to_string(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(tasks)
}