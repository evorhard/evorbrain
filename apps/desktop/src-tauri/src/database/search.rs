use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub entity_type: String,
    pub title: String,
    pub description: Option<String>,
    pub excerpt: String,
    pub score: f64,
}

/// Initialize FTS5 full-text search tables
pub fn init_fts5(conn: &Connection) -> Result<()> {
    // Enable FTS5 extension (it's built into SQLite by default since 3.9.0)
    
    // Create the main entities table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entities (
            id TEXT PRIMARY KEY,
            type TEXT NOT NULL CHECK (type IN ('area', 'goal', 'project', 'task')),
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL,
            parent_id TEXT,
            file_path TEXT NOT NULL UNIQUE,
            frontmatter TEXT NOT NULL,  -- JSON
            content TEXT,
            content_hash TEXT NOT NULL,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL,
            indexed_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    
    // Create relationships table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS relationships (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            source_id TEXT NOT NULL,
            target_id TEXT NOT NULL,
            relationship_type TEXT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (source_id) REFERENCES entities(id) ON DELETE CASCADE,
            FOREIGN KEY (target_id) REFERENCES entities(id) ON DELETE CASCADE,
            UNIQUE(source_id, target_id, relationship_type)
        )",
        [],
    )?;
    
    // Create tags table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            color TEXT,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    
    // Create entity-tag junction table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS entity_tags (
            entity_id TEXT NOT NULL,
            tag_id INTEGER NOT NULL,
            FOREIGN KEY (entity_id) REFERENCES entities(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE,
            PRIMARY KEY (entity_id, tag_id)
        )",
        [],
    )?;
    
    // Create FTS5 virtual table for full-text search
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS entities_fts USING fts5(
            id UNINDEXED,
            title,
            description,
            content,
            tags,
            type UNINDEXED,
            status UNINDEXED,
            parent_id UNINDEXED,
            tokenize = 'porter unicode61',
            content_rowid = 'rowid'
        )",
        [],
    )?;
    
    // Create triggers to keep FTS in sync with entities table
    
    // After INSERT trigger
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS entities_ai 
         AFTER INSERT ON entities BEGIN
            INSERT INTO entities_fts(
                rowid, id, title, description, content, tags, type, status, parent_id
            ) 
            SELECT 
                new.rowid,
                new.id,
                new.title,
                new.description,
                new.content,
                COALESCE((
                    SELECT GROUP_CONCAT(t.name, ' ') 
                    FROM entity_tags et 
                    JOIN tags t ON et.tag_id = t.id 
                    WHERE et.entity_id = new.id
                ), ''),
                new.type,
                new.status,
                new.parent_id;
        END",
        [],
    )?;
    
    // After UPDATE trigger
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS entities_au 
         AFTER UPDATE ON entities BEGIN
            UPDATE entities_fts SET
                title = new.title,
                description = new.description,
                content = new.content,
                tags = COALESCE((
                    SELECT GROUP_CONCAT(t.name, ' ') 
                    FROM entity_tags et 
                    JOIN tags t ON et.tag_id = t.id 
                    WHERE et.entity_id = new.id
                ), ''),
                type = new.type,
                status = new.status,
                parent_id = new.parent_id
            WHERE id = new.id;
        END",
        [],
    )?;
    
    // After DELETE trigger
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS entities_ad 
         AFTER DELETE ON entities BEGIN
            DELETE FROM entities_fts WHERE id = old.id;
        END",
        [],
    )?;
    
    // Create indexes for better performance
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_entities_type ON entities(type)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_entities_status ON entities(status)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_entities_parent_id ON entities(parent_id)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_relationships_source ON relationships(source_id)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_relationships_target ON relationships(target_id)",
        [],
    )?;
    
    Ok(())
}

/// Search entities using FTS5
pub fn search_entities(
    conn: &Connection,
    query: &str,
    entity_type: Option<&str>,
    parent_id: Option<&str>,
    limit: usize,
    offset: usize,
) -> Result<Vec<SearchResult>> {
    let mut sql = String::from(
        "SELECT 
            e.id,
            e.type,
            e.title,
            e.description,
            snippet(entities_fts, 3, '<mark>', '</mark>', '...', 30) as excerpt,
            bm25(entities_fts) as score
        FROM entities e
        JOIN entities_fts fts ON e.id = fts.id
        WHERE entities_fts MATCH ?"
    );
    
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = vec![Box::new(query.to_string())];
    let mut param_index = 2;
    
    if let Some(entity_type) = entity_type {
        sql.push_str(&format!(" AND e.type = ?{}", param_index));
        params.push(Box::new(entity_type.to_string()));
        param_index += 1;
    }
    
    if let Some(parent_id) = parent_id {
        sql.push_str(&format!(" AND e.parent_id = ?{}", param_index));
        params.push(Box::new(parent_id.to_string()));
        param_index += 1;
    }
    
    sql.push_str(&format!(" ORDER BY score DESC LIMIT ?{} OFFSET ?{}", param_index, param_index + 1));
    params.push(Box::new(limit));
    params.push(Box::new(offset));
    
    let mut stmt = conn.prepare(&sql)?;
    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let results = stmt.query_map(&param_refs[..], |row| {
        Ok(SearchResult {
            id: row.get(0)?,
            entity_type: row.get(1)?,
            title: row.get(2)?,
            description: row.get(3)?,
            excerpt: row.get(4)?,
            score: row.get(5)?,
        })
    })?;
    
    let mut search_results = Vec::new();
    for result in results {
        search_results.push(result?);
    }
    
    Ok(search_results)
}

/// Index or re-index an entity for search
pub fn index_entity(conn: &Connection, entity_id: &str) -> Result<()> {
    // The triggers handle this automatically when entities are inserted/updated
    // This function can be used to force re-indexing if needed
    
    // First, delete existing FTS entry
    conn.execute(
        "DELETE FROM entities_fts WHERE id = ?",
        params![entity_id],
    )?;
    
    // Then re-insert from entities table
    conn.execute(
        "INSERT INTO entities_fts(id, title, description, content, tags, type, status, parent_id)
         SELECT 
            e.id,
            e.title,
            e.description,
            e.content,
            COALESCE((
                SELECT GROUP_CONCAT(t.name, ' ') 
                FROM entity_tags et 
                JOIN tags t ON et.tag_id = t.id 
                WHERE et.entity_id = e.id
            ), ''),
            e.type,
            e.status,
            e.parent_id
         FROM entities e
         WHERE e.id = ?",
        params![entity_id],
    )?;
    
    Ok(())
}

/// Test FTS5 functionality
pub fn test_fts5(conn: &Connection) -> Result<String> {
    // Insert test data
    let test_id = "test_entity_fts5";
    
    conn.execute(
        "INSERT OR REPLACE INTO entities (
            id, type, title, description, status, content, 
            file_path, frontmatter, content_hash, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'), datetime('now'))",
        params![
            test_id,
            "task",
            "Test FTS5 Search Implementation",
            "Testing full-text search with SQLite FTS5",
            "todo",
            "This is a test entity for searching. It contains keywords like implementation, search, and database.",
            "test/path.md",
            "{}",
            "test_hash"
        ],
    )?;
    
    // Perform search
    let results = search_entities(conn, "search implementation", None, None, 10, 0)?;
    
    // Clean up
    conn.execute("DELETE FROM entities WHERE id = ?", params![test_id])?;
    
    if results.is_empty() {
        Ok("FTS5 test completed but no results found. This might indicate an issue.".to_string())
    } else {
        Ok(format!(
            "FTS5 test successful!\n\
            - Created test entity\n\
            - Performed search for 'search implementation'\n\
            - Found {} results\n\
            - First result: {} (score: {:.2})\n\
            - Cleaned up test data",
            results.len(),
            results[0].title,
            results[0].score
        ))
    }
}