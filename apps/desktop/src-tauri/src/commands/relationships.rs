// Entity relationship management commands

use tauri::State;
use rusqlite::{params, OptionalExtension};
use crate::database::pool::{AppState, get_db_connection_from_state};
use crate::errors::{AppError, ErrorCode, ErrorContext};
use log::{info, warn};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Note: Helper functions from entities module are available but not used in this implementation
// They could be used for more complex entity parsing if needed in the future

/// Structure representing entity relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityRelationships {
    pub entity_id: String,
    pub entity_type: String,
    pub parents: Vec<RelatedEntity>,
    pub children: Vec<RelatedEntity>,
    pub references: Vec<RelatedEntity>,
    pub referenced_by: Vec<RelatedEntity>,
}

/// Structure representing a related entity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedEntity {
    pub id: String,
    pub entity_type: String,
    pub title: String,
    pub relationship_type: String,
}

// Internal function that works with a database connection directly
pub fn get_entity_relationships_internal(
    conn: &rusqlite::Connection,
    entity_id: String,
    entity_type: String,
) -> Result<EntityRelationships, AppError> {
    info!("Getting relationships for {} with ID: {}", entity_type, entity_id);
    
    let mut relationships = EntityRelationships {
        entity_id: entity_id.clone(),
        entity_type: entity_type.clone(),
        parents: Vec::new(),
        children: Vec::new(),
        references: Vec::new(),
        referenced_by: Vec::new(),
    };
    
    match entity_type.as_str() {
        "area" => {
            // Areas have no parents, only children (goals)
            relationships.children = get_goals_for_area(&conn, &entity_id)?;
        },
        "goal" => {
            // Goals have area as parent and projects as children
            relationships.parents = get_area_for_goal(&conn, &entity_id)?;
            relationships.children = get_projects_for_goal(&conn, &entity_id)?;
        },
        "project" => {
            // Projects have goal as parent and tasks as children
            relationships.parents = get_goal_for_project(&conn, &entity_id)?;
            relationships.children = get_tasks_for_project(&conn, &entity_id)?;
        },
        "task" => {
            // Tasks can have project or parent task as parent, and subtasks as children
            relationships.parents = get_parents_for_task(&conn, &entity_id)?;
            relationships.children = get_subtasks_for_task(&conn, &entity_id)?;
        },
        _ => {
            return Err(AppError::Validation {
                field: "entity_type".to_string(),
                reason: format!("Invalid entity type: {}", entity_type),
                code: ErrorCode::InvalidEntityType,
                context: Some(ErrorContext {
                    user_action: "Getting entity relationships".to_string(),
                    recovery_suggestions: vec![
                        "Valid entity types are: area, goal, project, task".to_string(),
                    ],
                    recoverable: true,
                    help_url: None,
                }),
            });
        }
    }
    
    // Get cross-references (currently using description field, future: dedicated references table)
    relationships.references = get_entity_references(&conn, &entity_id, &entity_type)?;
    relationships.referenced_by = get_entity_referenced_by(&conn, &entity_id, &entity_type)?;
    
    Ok(relationships)
}

/// Get all relationships for an entity (Tauri command)
#[tauri::command]
pub async fn get_entity_relationships(
    state: State<'_, AppState>,
    entity_id: String,
    entity_type: String,
) -> Result<EntityRelationships, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    get_entity_relationships_internal(&conn, entity_id, entity_type)
}

// Internal function for getting children entities
pub fn get_children_entities_internal(
    conn: &rusqlite::Connection,
    parent_id: String,
    parent_type: String,
) -> Result<Vec<RelatedEntity>, AppError> {
    info!("Getting children for {} with ID: {}", parent_type, parent_id);
    
    match parent_type.as_str() {
        "area" => get_goals_for_area(&conn, &parent_id),
        "goal" => get_projects_for_goal(&conn, &parent_id),
        "project" => get_tasks_for_project(&conn, &parent_id),
        "task" => get_subtasks_for_task(&conn, &parent_id),
        _ => Err(AppError::Validation {
            field: "parent_type".to_string(),
            reason: format!("Invalid parent type: {}", parent_type),
            code: ErrorCode::InvalidEntityType,
            context: Some(ErrorContext {
                user_action: "Getting child entities".to_string(),
                recovery_suggestions: vec![
                    "Valid parent types are: area, goal, project, task".to_string(),
                ],
                recoverable: true,
                help_url: None,
            }),
        })
    }
}

/// Get children entities for a parent (Tauri command)
#[tauri::command]
pub async fn get_children_entities(
    state: State<'_, AppState>,
    parent_id: String,
    parent_type: String,
) -> Result<Vec<RelatedEntity>, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    get_children_entities_internal(&conn, parent_id, parent_type)
}

// Internal function for validating entity relationships
pub fn validate_entity_relationship_internal(
    conn: &rusqlite::Connection,
    child_type: String,
    child_id: String,
    parent_type: String,
    parent_id: String,
) -> Result<bool, AppError> {
    info!("Validating relationship: {} {} -> {} {}", 
        child_type, child_id, parent_type, parent_id);
    
    // Check if parent exists
    let parent_exists = check_entity_exists(&conn, &parent_type, &parent_id)?;
    if !parent_exists {
        return Ok(false);
    }
    
    // Validate relationship type
    let valid = match (parent_type.as_str(), child_type.as_str()) {
        ("area", "goal") => true,
        ("goal", "project") => true,
        ("project", "task") => true,
        ("task", "task") => {
            // Ensure no circular references
            !check_circular_task_reference(&conn, &parent_id, &child_id)?
        },
        _ => false,
    };
    
    Ok(valid)
}

/// Validate parent-child relationship (Tauri command)
#[tauri::command]
pub async fn validate_entity_relationship(
    state: State<'_, AppState>,
    child_type: String,
    child_id: String,
    parent_type: String,
    parent_id: String,
) -> Result<bool, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    validate_entity_relationship_internal(&conn, child_type, child_id, parent_type, parent_id)
}

// Internal function for checking cascade impact
pub fn check_cascade_impact_internal(
    conn: &rusqlite::Connection,
    entity_id: String,
    entity_type: String,
) -> Result<CascadeImpact, AppError> {
    info!("Checking cascade impact for {} with ID: {}", entity_type, entity_id);
    
    let mut impact = CascadeImpact {
        entity_id: entity_id.clone(),
        entity_type: entity_type.clone(),
        total_affected: 0,
        affected_entities: HashMap::new(),
    };
    
    match entity_type.as_str() {
        "area" => {
            let goals = count_entity_children(&conn, "goals", "area_id", &entity_id)?;
            let projects = count_descendant_entities(&conn, "area", &entity_id, "projects")?;
            let tasks = count_descendant_entities(&conn, "area", &entity_id, "tasks")?;
            
            impact.affected_entities.insert("goals".to_string(), goals);
            impact.affected_entities.insert("projects".to_string(), projects);
            impact.affected_entities.insert("tasks".to_string(), tasks);
            impact.total_affected = goals + projects + tasks;
        },
        "goal" => {
            let projects = count_entity_children(&conn, "projects", "goal_id", &entity_id)?;
            let tasks = count_descendant_entities(&conn, "goal", &entity_id, "tasks")?;
            
            impact.affected_entities.insert("projects".to_string(), projects);
            impact.affected_entities.insert("tasks".to_string(), tasks);
            impact.total_affected = projects + tasks;
        },
        "project" => {
            let tasks = count_entity_children(&conn, "tasks", "project_id", &entity_id)?;
            
            impact.affected_entities.insert("tasks".to_string(), tasks);
            impact.total_affected = tasks;
        },
        "task" => {
            let subtasks = count_entity_children(&conn, "tasks", "parent_task_id", &entity_id)?;
            
            impact.affected_entities.insert("subtasks".to_string(), subtasks);
            impact.total_affected = subtasks;
        },
        _ => {
            return Err(AppError::Validation {
                field: "entity_type".to_string(),
                reason: format!("Invalid entity type: {}", entity_type),
                code: ErrorCode::InvalidEntityType,
                context: Some(ErrorContext {
                    user_action: "Checking cascade impact".to_string(),
                    recovery_suggestions: vec![
                        "Valid entity types are: area, goal, project, task".to_string(),
                    ],
                    recoverable: true,
                    help_url: None,
                }),
            });
        }
    }
    
    Ok(impact)
}

/// Check for cascading impacts before deletion (Tauri command)
#[tauri::command]
pub async fn check_cascade_impact(
    state: State<'_, AppState>,
    entity_id: String,
    entity_type: String,
) -> Result<CascadeImpact, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    check_cascade_impact_internal(&conn, entity_id, entity_type)
}

/// Structure representing cascade deletion impact
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CascadeImpact {
    pub entity_id: String,
    pub entity_type: String,
    pub total_affected: usize,
    pub affected_entities: HashMap<String, usize>,
}

// Helper functions

fn get_goals_for_area(conn: &rusqlite::Connection, area_id: &str) -> Result<Vec<RelatedEntity>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, title FROM goals WHERE area_id = ?1 ORDER BY sort_order, created_at DESC"
    )?;
    
    let goals = stmt.query_map(params![area_id], |row| {
        Ok(RelatedEntity {
            id: row.get(0)?,
            entity_type: "goal".to_string(),
            title: row.get(1)?,
            relationship_type: "child".to_string(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(goals)
}

fn get_area_for_goal(conn: &rusqlite::Connection, goal_id: &str) -> Result<Vec<RelatedEntity>, AppError> {
    let area = conn.query_row(
        "SELECT a.id, a.title FROM areas a 
         JOIN goals g ON g.area_id = a.id 
         WHERE g.id = ?1",
        params![goal_id],
        |row| {
            Ok(RelatedEntity {
                id: row.get(0)?,
                entity_type: "area".to_string(),
                title: row.get(1)?,
                relationship_type: "parent".to_string(),
            })
        },
    ).optional()?;
    
    Ok(area.into_iter().collect())
}

fn get_projects_for_goal(conn: &rusqlite::Connection, goal_id: &str) -> Result<Vec<RelatedEntity>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, title FROM projects WHERE goal_id = ?1 ORDER BY sort_order, created_at DESC"
    )?;
    
    let projects = stmt.query_map(params![goal_id], |row| {
        Ok(RelatedEntity {
            id: row.get(0)?,
            entity_type: "project".to_string(),
            title: row.get(1)?,
            relationship_type: "child".to_string(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(projects)
}

fn get_goal_for_project(conn: &rusqlite::Connection, project_id: &str) -> Result<Vec<RelatedEntity>, AppError> {
    let goal = conn.query_row(
        "SELECT g.id, g.title FROM goals g 
         JOIN projects p ON p.goal_id = g.id 
         WHERE p.id = ?1",
        params![project_id],
        |row| {
            Ok(RelatedEntity {
                id: row.get(0)?,
                entity_type: "goal".to_string(),
                title: row.get(1)?,
                relationship_type: "parent".to_string(),
            })
        },
    ).optional()?;
    
    Ok(goal.into_iter().collect())
}

fn get_tasks_for_project(conn: &rusqlite::Connection, project_id: &str) -> Result<Vec<RelatedEntity>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, title FROM tasks 
         WHERE project_id = ?1 AND parent_task_id IS NULL 
         ORDER BY sort_order, created_at DESC"
    )?;
    
    let tasks = stmt.query_map(params![project_id], |row| {
        Ok(RelatedEntity {
            id: row.get(0)?,
            entity_type: "task".to_string(),
            title: row.get(1)?,
            relationship_type: "child".to_string(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(tasks)
}

fn get_parents_for_task(conn: &rusqlite::Connection, task_id: &str) -> Result<Vec<RelatedEntity>, AppError> {
    let mut parents = Vec::new();
    
    // Get task details first
    let (project_id, parent_task_id): (Option<String>, Option<String>) = conn.query_row(
        "SELECT project_id, parent_task_id FROM tasks WHERE id = ?1",
        params![task_id],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    
    // Get project parent if exists
    if let Some(proj_id) = project_id {
        if let Ok(project) = conn.query_row(
            "SELECT id, title FROM projects WHERE id = ?1",
            params![proj_id],
            |row| {
                Ok(RelatedEntity {
                    id: row.get(0)?,
                    entity_type: "project".to_string(),
                    title: row.get(1)?,
                    relationship_type: "parent".to_string(),
                })
            },
        ) {
            parents.push(project);
        }
    }
    
    // Get parent task if exists
    if let Some(parent_id) = parent_task_id {
        if let Ok(parent_task) = conn.query_row(
            "SELECT id, title FROM tasks WHERE id = ?1",
            params![parent_id],
            |row| {
                Ok(RelatedEntity {
                    id: row.get(0)?,
                    entity_type: "task".to_string(),
                    title: row.get(1)?,
                    relationship_type: "parent".to_string(),
                })
            },
        ) {
            parents.push(parent_task);
        }
    }
    
    Ok(parents)
}

fn get_subtasks_for_task(conn: &rusqlite::Connection, parent_task_id: &str) -> Result<Vec<RelatedEntity>, AppError> {
    let mut stmt = conn.prepare(
        "SELECT id, title FROM tasks 
         WHERE parent_task_id = ?1 
         ORDER BY sort_order, created_at DESC"
    )?;
    
    let subtasks = stmt.query_map(params![parent_task_id], |row| {
        Ok(RelatedEntity {
            id: row.get(0)?,
            entity_type: "task".to_string(),
            title: row.get(1)?,
            relationship_type: "child".to_string(),
        })
    })?
    .collect::<Result<Vec<_>, _>>()?;
    
    Ok(subtasks)
}

fn get_entity_references(conn: &rusqlite::Connection, entity_id: &str, entity_type: &str) -> Result<Vec<RelatedEntity>, AppError> {
    // For now, we'll search for references in descriptions using a simple pattern
    // In the future, this could be a dedicated references table
    let pattern = format!("[[{}:{}]]", entity_type, entity_id);
    
    let mut references = Vec::new();
    
    // Search in all entity types
    let tables = vec!["areas", "goals", "projects", "tasks"];
    
    for table in tables {
        let query = format!(
            "SELECT id, title, '{}' as entity_type FROM {} 
             WHERE description LIKE ?1 AND id != ?2",
            table.trim_end_matches('s'), // Convert plural to singular
            table
        );
        
        let mut stmt = conn.prepare(&query)?;
        let results: Vec<RelatedEntity> = stmt.query_map(params![format!("%{}%", pattern), entity_id], |row| {
            Ok(RelatedEntity {
                id: row.get(0)?,
                entity_type: row.get(2)?,
                title: row.get(1)?,
                relationship_type: "reference".to_string(),
            })
        })?
        .filter_map(Result::ok)
        .collect();
        
        references.extend(results);
    }
    
    Ok(references)
}

fn get_entity_referenced_by(conn: &rusqlite::Connection, entity_id: &str, entity_type: &str) -> Result<Vec<RelatedEntity>, AppError> {
    // Search for entities that reference this entity
    let table = match entity_type {
        "area" => "areas",
        "goal" => "goals",
        "project" => "projects",
        "task" => "tasks",
        _ => return Ok(Vec::new()),
    };
    
    let query = format!(
        "SELECT id, title FROM {} 
         WHERE description LIKE ?1",
        table
    );
    
    let mut stmt = conn.prepare(&query)?;
    let references = stmt.query_map(params![format!("%[[{}:{}]]%", entity_type, entity_id)], |row| {
        Ok(RelatedEntity {
            id: row.get(0)?,
            entity_type: entity_type.to_string(),
            title: row.get(1)?,
            relationship_type: "referenced_by".to_string(),
        })
    })?
    .filter_map(Result::ok)
    .collect();
    
    Ok(references)
}

fn check_entity_exists(conn: &rusqlite::Connection, entity_type: &str, entity_id: &str) -> Result<bool, AppError> {
    let table = match entity_type {
        "area" => "areas",
        "goal" => "goals",
        "project" => "projects",
        "task" => "tasks",
        _ => return Ok(false),
    };
    
    let count: i32 = conn.query_row(
        &format!("SELECT COUNT(*) FROM {} WHERE id = ?1", table),
        params![entity_id],
        |row| row.get(0),
    )?;
    
    Ok(count > 0)
}

fn check_circular_task_reference(conn: &rusqlite::Connection, parent_id: &str, child_id: &str) -> Result<bool, AppError> {
    // Check if making child_id a subtask of parent_id would create a circular reference
    
    // If parent and child are the same, it's circular
    if parent_id == child_id {
        return Ok(true);
    }
    
    // Check if parent_id is already a descendant of child_id
    let mut current_id = parent_id.to_string();
    let mut visited = std::collections::HashSet::new();
    
    while let Some(parent) = conn.query_row(
        "SELECT parent_task_id FROM tasks WHERE id = ?1",
        params![current_id],
        |row| row.get::<_, Option<String>>(0),
    ).optional()? {
        if let Some(parent_task_id) = parent {
            if parent_task_id == child_id {
                return Ok(true);
            }
            
            // Prevent infinite loops
            if visited.contains(&parent_task_id) {
                warn!("Detected circular reference in task hierarchy");
                return Ok(true);
            }
            
            visited.insert(current_id.clone());
            current_id = parent_task_id;
        } else {
            break;
        }
    }
    
    Ok(false)
}

fn count_entity_children(conn: &rusqlite::Connection, table: &str, parent_column: &str, parent_id: &str) -> Result<usize, AppError> {
    let count: i32 = conn.query_row(
        &format!("SELECT COUNT(*) FROM {} WHERE {} = ?1", table, parent_column),
        params![parent_id],
        |row| row.get(0),
    )?;
    
    Ok(count as usize)
}

fn count_descendant_entities(conn: &rusqlite::Connection, ancestor_type: &str, ancestor_id: &str, descendant_table: &str) -> Result<usize, AppError> {
    let count = match (ancestor_type, descendant_table) {
        ("area", "projects") => {
            conn.query_row(
                "SELECT COUNT(*) FROM projects p 
                 JOIN goals g ON p.goal_id = g.id 
                 WHERE g.area_id = ?1",
                params![ancestor_id],
                |row| row.get::<_, i32>(0),
            )? as usize
        },
        ("area", "tasks") => {
            conn.query_row(
                "SELECT COUNT(*) FROM tasks t 
                 JOIN projects p ON t.project_id = p.id 
                 JOIN goals g ON p.goal_id = g.id 
                 WHERE g.area_id = ?1",
                params![ancestor_id],
                |row| row.get::<_, i32>(0),
            )? as usize
        },
        ("goal", "tasks") => {
            conn.query_row(
                "SELECT COUNT(*) FROM tasks t 
                 JOIN projects p ON t.project_id = p.id 
                 WHERE p.goal_id = ?1",
                params![ancestor_id],
                |row| row.get::<_, i32>(0),
            )? as usize
        },
        _ => 0,
    };
    
    Ok(count)
}

// Internal function for updating children on parent change
pub fn update_children_on_parent_change_internal(
    conn: &rusqlite::Connection,
    parent_id: String,
    parent_type: String,
    update_type: String,
    new_value: Option<String>,
) -> Result<usize, AppError> {
    info!("Updating children for {} {} with update type: {}", parent_type, parent_id, update_type);
    
    let affected = match (parent_type.as_str(), update_type.as_str()) {
        ("area", "status") => {
            // If area is archived, archive all its goals
            if new_value.as_deref() == Some("archived") {
                conn.execute(
                    "UPDATE goals SET status = 'abandoned', updated_at = datetime('now') 
                     WHERE area_id = ?1 AND status NOT IN ('completed', 'abandoned')",
                    params![parent_id],
                )?
            } else {
                0
            }
        },
        ("goal", "status") => {
            // If goal is completed/abandoned, update project status
            match new_value.as_deref() {
                Some("completed") => {
                    conn.execute(
                        "UPDATE projects SET status = 'completed', updated_at = datetime('now') 
                         WHERE goal_id = ?1 AND status NOT IN ('completed', 'cancelled')",
                        params![parent_id],
                    )?
                },
                Some("abandoned") => {
                    conn.execute(
                        "UPDATE projects SET status = 'cancelled', updated_at = datetime('now') 
                         WHERE goal_id = ?1 AND status NOT IN ('completed', 'cancelled')",
                        params![parent_id],
                    )?
                },
                _ => 0
            }
        },
        ("project", "status") => {
            // If project is completed/cancelled, update task status
            match new_value.as_deref() {
                Some("completed") => {
                    conn.execute(
                        "UPDATE tasks SET status = 'completed', completed_at = datetime('now'), updated_at = datetime('now') 
                         WHERE project_id = ?1 AND status NOT IN ('completed', 'cancelled')",
                        params![parent_id],
                    )?
                },
                Some("cancelled") => {
                    conn.execute(
                        "UPDATE tasks SET status = 'cancelled', updated_at = datetime('now') 
                         WHERE project_id = ?1 AND status NOT IN ('completed', 'cancelled')",
                        params![parent_id],
                    )?
                },
                _ => 0
            }
        },
        _ => 0,
    };
    
    Ok(affected)
}

/// Update all children when parent is moved or status changes (Tauri command)
#[tauri::command]
pub async fn update_children_on_parent_change(
    state: State<'_, AppState>,
    parent_id: String,
    parent_type: String,
    update_type: String,
    new_value: Option<String>,
) -> Result<usize, AppError> {
    let conn = get_db_connection_from_state(&state)?;
    update_children_on_parent_change_internal(&conn, parent_id, parent_type, update_type, new_value)
}