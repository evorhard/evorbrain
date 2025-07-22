use rusqlite::{Connection, Result};
use std::path::Path;

pub fn init_database(db_path: &Path) -> Result<()> {
    let conn = Connection::open(db_path)?;
    
    // Create tables for entities
    conn.execute(
        "CREATE TABLE IF NOT EXISTS areas (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT,
            color TEXT,
            icon TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS goals (
            id TEXT PRIMARY KEY,
            area_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            target_date TEXT,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (area_id) REFERENCES areas(id)
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            goal_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL,
            start_date TEXT,
            end_date TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (goal_id) REFERENCES goals(id)
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            project_id TEXT,
            parent_task_id TEXT,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL,
            due_date TEXT,
            priority TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL,
            FOREIGN KEY (project_id) REFERENCES projects(id),
            FOREIGN KEY (parent_task_id) REFERENCES tasks(id)
        )",
        [],
    )?;
    
    // Create search index table (FTS5 will be added later)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS search_index (
            entity_id TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            content TEXT,
            PRIMARY KEY (entity_id, entity_type)
        )",
        [],
    )?;
    
    Ok(())
}