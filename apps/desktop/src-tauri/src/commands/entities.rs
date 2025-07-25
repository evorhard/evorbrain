use crate::errors::AppError;
use crate::models::{Area, Goal, Project, Task, GoalStatus, ProjectStatus, TaskStatus, Priority};
use rusqlite::{Connection, params, Row};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use chrono::{DateTime, Utc};

// Helper to get database connection
fn get_db_connection(app_handle: &tauri::AppHandle) -> Result<Connection, AppError> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Operation {
            message: format!("Failed to get app directory: {}", e),
            code: crate::errors::ErrorCode::OperationFailed,
            severity: crate::errors::ErrorSeverity::Error,
            context: Some(crate::errors::ErrorContext {
                user_action: "Accessing database".to_string(),
                recovery_suggestions: vec![
                    "Ensure the application has proper permissions".to_string(),
                ],
                recoverable: false,
                help_url: None,
            }),
        })?;
    let db_path = app_dir.join("evorbrain.db");
    
    Connection::open(&db_path)
        .map_err(|e| {
            let err = AppError::from(e);
            err.with_context(crate::errors::ErrorContext {
                user_action: "Opening database connection".to_string(),
                recovery_suggestions: vec![
                    "Check if the database file exists".to_string(),
                    "Verify file permissions".to_string(),
                    "Try restarting the application".to_string(),
                ],
                recoverable: true,
                help_url: None,
            })
        })
}

// Helper functions to parse database rows
fn parse_area(row: &Row) -> Result<Area, rusqlite::Error> {
    Ok(Area {
        id: row.get(0)?,
        title: row.get(1)?,
        description: row.get(2)?,
        color: row.get(3)?,
        icon: row.get(4)?,
        created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
            .unwrap()
            .with_timezone(&Utc),
        updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
            .unwrap()
            .with_timezone(&Utc),
    })
}

fn parse_goal_status(status: &str) -> GoalStatus {
    match status {
        "not-started" => GoalStatus::NotStarted,
        "in-progress" => GoalStatus::InProgress,
        "completed" => GoalStatus::Completed,
        "on-hold" => GoalStatus::OnHold,
        _ => GoalStatus::NotStarted,
    }
}

fn parse_project_status(status: &str) -> ProjectStatus {
    match status {
        "planning" => ProjectStatus::Planning,
        "active" => ProjectStatus::Active,
        "completed" => ProjectStatus::Completed,
        "on-hold" => ProjectStatus::OnHold,
        "cancelled" => ProjectStatus::Cancelled,
        _ => ProjectStatus::Planning,
    }
}

fn parse_task_status(status: &str) -> TaskStatus {
    match status {
        "not-started" => TaskStatus::NotStarted,
        "in-progress" => TaskStatus::InProgress,
        "completed" => TaskStatus::Completed,
        "cancelled" => TaskStatus::Cancelled,
        _ => TaskStatus::NotStarted,
    }
}

fn parse_priority(priority: Option<String>) -> Option<Priority> {
    priority.and_then(|p| match p.as_str() {
        "high" => Some(Priority::High),
        "medium" => Some(Priority::Medium),
        "low" => Some(Priority::Low),
        _ => None,
    })
}

fn goal_status_to_string(status: &GoalStatus) -> &'static str {
    match status {
        GoalStatus::NotStarted => "not-started",
        GoalStatus::InProgress => "in-progress",
        GoalStatus::Completed => "completed",
        GoalStatus::OnHold => "on-hold",
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

fn priority_to_string(priority: &Option<Priority>) -> Option<&'static str> {
    priority.as_ref().map(|p| match p {
        Priority::High => "high",
        Priority::Medium => "medium",
        Priority::Low => "low",
    })
}

// Area Commands
#[tauri::command]
pub async fn create_area(
    app_handle: tauri::AppHandle,
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
    
    let area = Area::new(title, description);
    let mut area = Area { color, icon, ..area };
    
    let conn = get_db_connection(&app_handle)?;
    
    conn.execute(
        "INSERT INTO areas (id, title, description, color, icon, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            &area.id,
            &area.title,
            &area.description,
            &area.color,
            &area.icon,
            area.created_at.to_rfc3339(),
            area.updated_at.to_rfc3339(),
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
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Area, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let area = conn.query_row(
        "SELECT id, title, description, color, icon, created_at, updated_at 
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
    app_handle: tauri::AppHandle,
) -> Result<Vec<Area>, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, title, description, color, icon, created_at, updated_at 
         FROM areas ORDER BY title"
    )?;
    
    let areas = stmt.query_map([], parse_area)?
        .collect::<Result<Vec<_>, _>>()?;
    
    Ok(areas)
}

#[tauri::command]
pub async fn update_area(
    app_handle: tauri::AppHandle,
    id: String,
    title: String,
    description: Option<String>,
    color: Option<String>,
    icon: Option<String>,
) -> Result<Area, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    conn.execute(
        "UPDATE areas SET title = ?1, description = ?2, color = ?3, icon = ?4, 
         updated_at = ?5 WHERE id = ?6",
        params![
            &title,
            &description,
            &color,
            &icon,
            Utc::now().to_rfc3339(),
            &id,
        ],
    )?;
    
    get_area(app_handle, id).await
}

#[tauri::command]
pub async fn delete_area(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<(), AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    // Check if area has any goals
    let goal_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM goals WHERE area_id = ?1",
        params![&id],
        |row| row.get(0),
    )?;
    
    if goal_count > 0 {
        return Err(AppError::Validation(
            "Cannot delete area with associated goals".to_string()
        ));
    }
    
    conn.execute("DELETE FROM areas WHERE id = ?1", params![&id])?;
    
    Ok(())
}

// Goal Commands
#[tauri::command]
pub async fn create_goal(
    app_handle: tauri::AppHandle,
    area_id: String,
    title: String,
    description: Option<String>,
    target_date: Option<String>,
) -> Result<Goal, AppError> {
    let mut goal = Goal::new(area_id, title, description);
    
    if let Some(date_str) = target_date {
        goal.target_date = DateTime::parse_from_rfc3339(&date_str)
            .ok()
            .map(|dt| dt.with_timezone(&Utc));
    }
    
    let conn = get_db_connection(&app_handle)?;
    
    conn.execute(
        "INSERT INTO goals (id, area_id, title, description, target_date, status, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            &goal.id,
            &goal.area_id,
            &goal.title,
            &goal.description,
            goal.target_date.map(|dt| dt.to_rfc3339()),
            goal_status_to_string(&goal.status),
            goal.created_at.to_rfc3339(),
            goal.updated_at.to_rfc3339(),
        ],
    )?;
    
    Ok(goal)
}

#[tauri::command]
pub async fn get_goals_by_area(
    app_handle: tauri::AppHandle,
    area_id: String,
) -> Result<Vec<Goal>, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, area_id, title, description, target_date, status, created_at, updated_at 
         FROM goals WHERE area_id = ?1 ORDER BY title"
    )?;
    
    let goals = stmt.query_map(params![&area_id], |row| {
        Ok(Goal {
            id: row.get(0)?,
            area_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            target_date: row.get::<_, Option<String>>(4)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            status: parse_goal_status(&row.get::<_, String>(5)?),
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                .unwrap()
                .with_timezone(&Utc),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(goals)
}

// Project Commands
#[tauri::command]
pub async fn create_project(
    app_handle: tauri::AppHandle,
    goal_id: String,
    title: String,
    description: Option<String>,
    start_date: Option<String>,
    end_date: Option<String>,
) -> Result<Project, AppError> {
    let mut project = Project::new(goal_id, title, description);
    
    if let Some(date_str) = start_date {
        project.start_date = DateTime::parse_from_rfc3339(&date_str)
            .ok()
            .map(|dt| dt.with_timezone(&Utc));
    }
    
    if let Some(date_str) = end_date {
        project.end_date = DateTime::parse_from_rfc3339(&date_str)
            .ok()
            .map(|dt| dt.with_timezone(&Utc));
    }
    
    let conn = get_db_connection(&app_handle)?;
    
    conn.execute(
        "INSERT INTO projects (id, goal_id, title, description, status, start_date, end_date, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            &project.id,
            &project.goal_id,
            &project.title,
            &project.description,
            project_status_to_string(&project.status),
            project.start_date.map(|dt| dt.to_rfc3339()),
            project.end_date.map(|dt| dt.to_rfc3339()),
            project.created_at.to_rfc3339(),
            project.updated_at.to_rfc3339(),
        ],
    )?;
    
    Ok(project)
}

#[tauri::command]
pub async fn get_projects_by_goal(
    app_handle: tauri::AppHandle,
    goal_id: String,
) -> Result<Vec<Project>, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, goal_id, title, description, status, start_date, end_date, created_at, updated_at 
         FROM projects WHERE goal_id = ?1 ORDER BY title"
    )?;
    
    let projects = stmt.query_map(params![&goal_id], |row| {
        Ok(Project {
            id: row.get(0)?,
            goal_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            status: parse_project_status(&row.get::<_, String>(4)?),
            start_date: row.get::<_, Option<String>>(5)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            end_date: row.get::<_, Option<String>>(6)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .unwrap()
                .with_timezone(&Utc),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(projects)
}

// Update Goal
#[tauri::command]
pub async fn update_goal(
    app_handle: tauri::AppHandle,
    id: String,
    title: String,
    description: Option<String>,
    target_date: Option<String>,
    status: String,
) -> Result<Goal, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    // Validate status
    let goal_status = parse_goal_status(&status);
    
    let target_datetime = target_date.and_then(|date_str| {
        DateTime::parse_from_rfc3339(&date_str)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });
    
    conn.execute(
        "UPDATE goals SET title = ?1, description = ?2, target_date = ?3, status = ?4, 
         updated_at = ?5 WHERE id = ?6",
        params![
            &title,
            &description,
            target_datetime.map(|dt| dt.to_rfc3339()),
            goal_status_to_string(&goal_status),
            Utc::now().to_rfc3339(),
            &id,
        ],
    )?;
    
    get_goal(app_handle, id).await
}

// Delete Goal
#[tauri::command]
pub async fn delete_goal(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<(), AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    // Check if goal has any projects
    let project_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM projects WHERE goal_id = ?1",
        params![&id],
        |row| row.get(0),
    )?;
    
    if project_count > 0 {
        return Err(AppError::Validation(
            "Cannot delete goal with associated projects".to_string()
        ));
    }
    
    conn.execute("DELETE FROM goals WHERE id = ?1", params![&id])?;
    
    Ok(())
}

// Get single Goal
#[tauri::command]
pub async fn get_goal(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Goal, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let goal = conn.query_row(
        "SELECT id, area_id, title, description, target_date, status, created_at, updated_at 
         FROM goals WHERE id = ?1",
        params![&id],
        |row| {
            Ok(Goal {
                id: row.get(0)?,
                area_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                target_date: row.get::<_, Option<String>>(4)?
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                status: parse_goal_status(&row.get::<_, String>(5)?),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        },
    )?;
    
    Ok(goal)
}

// Get all Goals
#[tauri::command]
pub async fn get_all_goals(
    app_handle: tauri::AppHandle,
) -> Result<Vec<Goal>, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, area_id, title, description, target_date, status, created_at, updated_at 
         FROM goals ORDER BY title"
    )?;
    
    let goals = stmt.query_map([], |row| {
        Ok(Goal {
            id: row.get(0)?,
            area_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            target_date: row.get::<_, Option<String>>(4)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            status: parse_goal_status(&row.get::<_, String>(5)?),
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                .unwrap()
                .with_timezone(&Utc),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(goals)
}

// Update Project
#[tauri::command]
pub async fn update_project(
    app_handle: tauri::AppHandle,
    id: String,
    title: String,
    description: Option<String>,
    status: String,
    start_date: Option<String>,
    end_date: Option<String>,
) -> Result<Project, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    // Validate status
    let project_status = parse_project_status(&status);
    
    let start_datetime = start_date.and_then(|date_str| {
        DateTime::parse_from_rfc3339(&date_str)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });
    
    let end_datetime = end_date.and_then(|date_str| {
        DateTime::parse_from_rfc3339(&date_str)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });
    
    conn.execute(
        "UPDATE projects SET title = ?1, description = ?2, status = ?3, start_date = ?4, 
         end_date = ?5, updated_at = ?6 WHERE id = ?7",
        params![
            &title,
            &description,
            project_status_to_string(&project_status),
            start_datetime.map(|dt| dt.to_rfc3339()),
            end_datetime.map(|dt| dt.to_rfc3339()),
            Utc::now().to_rfc3339(),
            &id,
        ],
    )?;
    
    get_project(app_handle, id).await
}

// Delete Project
#[tauri::command]
pub async fn delete_project(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<(), AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    // Check if project has any tasks
    let task_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM tasks WHERE project_id = ?1",
        params![&id],
        |row| row.get(0),
    )?;
    
    if task_count > 0 {
        return Err(AppError::Validation(
            "Cannot delete project with associated tasks".to_string()
        ));
    }
    
    conn.execute("DELETE FROM projects WHERE id = ?1", params![&id])?;
    
    Ok(())
}

// Get single Project
#[tauri::command]
pub async fn get_project(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Project, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let project = conn.query_row(
        "SELECT id, goal_id, title, description, status, start_date, end_date, created_at, updated_at 
         FROM projects WHERE id = ?1",
        params![&id],
        |row| {
            Ok(Project {
                id: row.get(0)?,
                goal_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                status: parse_project_status(&row.get::<_, String>(4)?),
                start_date: row.get::<_, Option<String>>(5)?
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                end_date: row.get::<_, Option<String>>(6)?
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        },
    )?;
    
    Ok(project)
}

// Get all Projects
#[tauri::command]
pub async fn get_all_projects(
    app_handle: tauri::AppHandle,
) -> Result<Vec<Project>, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, goal_id, title, description, status, start_date, end_date, created_at, updated_at 
         FROM projects ORDER BY title"
    )?;
    
    let projects = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            goal_id: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            status: parse_project_status(&row.get::<_, String>(4)?),
            start_date: row.get::<_, Option<String>>(5)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            end_date: row.get::<_, Option<String>>(6)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .unwrap()
                .with_timezone(&Utc),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(projects)
}

// Task Commands
#[tauri::command]
pub async fn create_task(
    app_handle: tauri::AppHandle,
    title: String,
    description: Option<String>,
    project_id: Option<String>,
    parent_task_id: Option<String>,
    due_date: Option<String>,
    priority: Option<String>,
) -> Result<Task, AppError> {
    let mut task = Task::new(title, description);
    task.project_id = project_id;
    task.parent_task_id = parent_task_id;
    
    if let Some(date_str) = due_date {
        task.due_date = DateTime::parse_from_rfc3339(&date_str)
            .ok()
            .map(|dt| dt.with_timezone(&Utc));
    }
    
    task.priority = priority.and_then(|p| match p.as_str() {
        "high" => Some(Priority::High),
        "medium" => Some(Priority::Medium),
        "low" => Some(Priority::Low),
        _ => None,
    });
    
    let conn = get_db_connection(&app_handle)?;
    
    conn.execute(
        "INSERT INTO tasks (id, project_id, parent_task_id, title, description, status, due_date, priority, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            &task.id,
            &task.project_id,
            &task.parent_task_id,
            &task.title,
            &task.description,
            task_status_to_string(&task.status),
            task.due_date.map(|dt| dt.to_rfc3339()),
            priority_to_string(&task.priority),
            task.created_at.to_rfc3339(),
            task.updated_at.to_rfc3339(),
        ],
    )?;
    
    Ok(task)
}

#[tauri::command]
pub async fn get_tasks_by_project(
    app_handle: tauri::AppHandle,
    project_id: String,
) -> Result<Vec<Task>, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, created_at, updated_at 
         FROM tasks WHERE project_id = ?1 ORDER BY created_at"
    )?;
    
    let tasks = stmt.query_map(params![&project_id], |row| {
        Ok(Task {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_task_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            status: parse_task_status(&row.get::<_, String>(5)?),
            due_date: row.get::<_, Option<String>>(6)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            priority: parse_priority(row.get(7)?),
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                .unwrap()
                .with_timezone(&Utc),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(tasks)
}

#[tauri::command]
pub async fn update_task_status(
    app_handle: tauri::AppHandle,
    id: String,
    status: String,
) -> Result<(), AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    // Validate status
    let _ = parse_task_status(&status);
    
    conn.execute(
        "UPDATE tasks SET status = ?1, updated_at = ?2 WHERE id = ?3",
        params![&status, Utc::now().to_rfc3339(), &id],
    )?;
    
    Ok(())
}

#[tauri::command]
pub async fn delete_task(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<(), AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    // Check if task has subtasks
    let subtask_count: i32 = conn.query_row(
        "SELECT COUNT(*) FROM tasks WHERE parent_task_id = ?1",
        params![&id],
        |row| row.get(0),
    )?;
    
    if subtask_count > 0 {
        return Err(AppError::Validation(
            "Cannot delete task with subtasks".to_string()
        ));
    }
    
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![&id])?;
    
    Ok(())
}

// Get single Task
#[tauri::command]
pub async fn get_task(
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<Task, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let task = conn.query_row(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, created_at, updated_at 
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
                due_date: row.get::<_, Option<String>>(6)?
                    .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                    .map(|dt| dt.with_timezone(&Utc)),
                priority: parse_priority(row.get(7)?),
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                    .unwrap()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                    .unwrap()
                    .with_timezone(&Utc),
            })
        },
    )?;
    
    Ok(task)
}

// Get all Tasks
#[tauri::command]
pub async fn get_all_tasks(
    app_handle: tauri::AppHandle,
) -> Result<Vec<Task>, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, created_at, updated_at 
         FROM tasks ORDER BY created_at DESC"
    )?;
    
    let tasks = stmt.query_map([], |row| {
        Ok(Task {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_task_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            status: parse_task_status(&row.get::<_, String>(5)?),
            due_date: row.get::<_, Option<String>>(6)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            priority: parse_priority(row.get(7)?),
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                .unwrap()
                .with_timezone(&Utc),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(tasks)
}

// Update Task
#[tauri::command]
pub async fn update_task(
    app_handle: tauri::AppHandle,
    id: String,
    title: String,
    description: Option<String>,
    status: String,
    due_date: Option<String>,
    priority: Option<String>,
) -> Result<Task, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    // Validate status
    let task_status = parse_task_status(&status);
    
    let due_datetime = due_date.and_then(|date_str| {
        DateTime::parse_from_rfc3339(&date_str)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });
    
    let task_priority = priority.and_then(|p| match p.as_str() {
        "high" => Some(Priority::High),
        "medium" => Some(Priority::Medium),
        "low" => Some(Priority::Low),
        _ => None,
    });
    
    conn.execute(
        "UPDATE tasks SET title = ?1, description = ?2, status = ?3, due_date = ?4, 
         priority = ?5, updated_at = ?6 WHERE id = ?7",
        params![
            &title,
            &description,
            task_status_to_string(&task_status),
            due_datetime.map(|dt| dt.to_rfc3339()),
            priority_to_string(&task_priority),
            Utc::now().to_rfc3339(),
            &id,
        ],
    )?;
    
    get_task(app_handle, id).await
}

// Task filtering commands
#[tauri::command]
pub async fn get_tasks_by_status(
    app_handle: tauri::AppHandle,
    status: String,
) -> Result<Vec<Task>, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    // Validate status
    let _ = parse_task_status(&status);
    
    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, created_at, updated_at 
         FROM tasks WHERE status = ?1 ORDER BY created_at DESC"
    )?;
    
    let tasks = stmt.query_map(params![&status], |row| {
        Ok(Task {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_task_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            status: parse_task_status(&row.get::<_, String>(5)?),
            due_date: row.get::<_, Option<String>>(6)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            priority: parse_priority(row.get(7)?),
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                .unwrap()
                .with_timezone(&Utc),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(tasks)
}

#[tauri::command]
pub async fn get_upcoming_tasks(
    app_handle: tauri::AppHandle,
    days: i64,
) -> Result<Vec<Task>, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let future_date = Utc::now() + chrono::Duration::days(days);
    
    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, created_at, updated_at 
         FROM tasks 
         WHERE due_date IS NOT NULL 
         AND date(due_date) <= date(?)
         AND status != 'completed'
         AND status != 'cancelled'
         ORDER BY due_date ASC"
    )?;
    
    let tasks = stmt.query_map(params![future_date.to_rfc3339()], |row| {
        Ok(Task {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_task_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            status: parse_task_status(&row.get::<_, String>(5)?),
            due_date: row.get::<_, Option<String>>(6)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            priority: parse_priority(row.get(7)?),
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                .unwrap()
                .with_timezone(&Utc),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(tasks)
}

#[tauri::command]
pub async fn get_subtasks(
    app_handle: tauri::AppHandle,
    parent_task_id: String,
) -> Result<Vec<Task>, AppError> {
    let conn = get_db_connection(&app_handle)?;
    
    let mut stmt = conn.prepare(
        "SELECT id, project_id, parent_task_id, title, description, status, due_date, priority, created_at, updated_at 
         FROM tasks WHERE parent_task_id = ?1 ORDER BY created_at"
    )?;
    
    let tasks = stmt.query_map(params![&parent_task_id], |row| {
        Ok(Task {
            id: row.get(0)?,
            project_id: row.get(1)?,
            parent_task_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            status: parse_task_status(&row.get::<_, String>(5)?),
            due_date: row.get::<_, Option<String>>(6)?
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            priority: parse_priority(row.get(7)?),
            created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(8)?)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(9)?)
                .unwrap()
                .with_timezone(&Utc),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(tasks)
}