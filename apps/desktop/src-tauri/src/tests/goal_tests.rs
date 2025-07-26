#[cfg(test)]
mod goal_tests {
    use crate::models::{Goal, GoalStatus};
    use crate::database::init_database;
    use rusqlite::{Connection, params};
    use chrono::Utc;
    use tempfile::TempDir;
    
    struct TestDb {
        _temp_dir: TempDir,
        conn: Connection,
    }
    
    fn setup_test_db() -> TestDb {
        // Create a temporary directory for the test database
        let temp_dir = TempDir::new().unwrap();
        let test_db_path = temp_dir.path().join("test_goal_crud.db");
        
        // Initialize database
        init_database(&test_db_path).unwrap();
        let conn = Connection::open(&test_db_path).unwrap();
        
        // Create a test area
        conn.execute(
            "INSERT INTO areas (id, title, description, color, icon, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), datetime('now'), ?6, ?7)",
            params!["area-test-1", "Test Area", "Test area for goals", "#3b82f6", "briefcase", 0, "[]"],
        ).unwrap();
        
        TestDb {
            _temp_dir: temp_dir,
            conn,
        }
    }
    
    #[test]
    fn test_goal_creation() {
        let test_db = setup_test_db();
        
        // Create a goal
        let goal = Goal::new("area-test-1".to_string(), "Test Goal".to_string(), Some("Test description".to_string()));
        
        // Verify initial state
        assert_eq!(goal.title, "Test Goal");
        assert_eq!(goal.area_id, "area-test-1");
        assert!(matches!(goal.status, GoalStatus::NotStarted));
        assert_eq!(goal.progress, 0);
        
        // Insert into database
        let result = test_db.conn.execute(
            "INSERT INTO goals (id, area_id, title, description, target_date, status, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                &goal.id,
                &goal.area_id,
                &goal.title,
                &goal.description,
                &goal.target_date,
                "not-started",
                goal.progress,
                goal.created_at.to_rfc3339(),
                goal.updated_at.to_rfc3339(),
                goal.sort_order,
                "[]",
            ],
        );
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_goal_area_validation() {
        let test_db = setup_test_db();
        
        // Try to create goal with non-existent area
        let result = test_db.conn.execute(
            "INSERT INTO goals (id, area_id, title, description, target_date, status, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                "goal-test-1",
                "non-existent-area",
                "Test Goal",
                "Test description",
                None::<String>,
                "not-started",
                0,
                Utc::now().to_rfc3339(),
                Utc::now().to_rfc3339(),
                0,
                "[]",
            ],
        );
        
        // Should fail due to foreign key constraint
        assert!(result.is_err());
    }
    
    #[test]
    fn test_goal_status_values() {
        let test_db = setup_test_db();
        
        // Test all valid status values
        let statuses = vec!["not-started", "in-progress", "completed", "abandoned"];
        
        for (i, status) in statuses.iter().enumerate() {
            let result = test_db.conn.execute(
                "INSERT INTO goals (id, area_id, title, description, target_date, status, progress, created_at, updated_at, sort_order, tags)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    format!("goal-test-{}", i),
                    "area-test-1",
                    format!("Goal {}", i),
                    None::<String>,
                    None::<String>,
                    status,
                    0,
                    Utc::now().to_rfc3339(),
                    Utc::now().to_rfc3339(),
                    i as i32,
                    "[]",
                ],
            );
            
            assert!(result.is_ok(), "Failed to insert goal with status: {}", status);
        }
    }
    
    #[test]
    fn test_goal_update() {
        let test_db = setup_test_db();
        
        // Create a goal
        let goal_id = "goal-test-1";
        test_db.conn.execute(
            "INSERT INTO goals (id, area_id, title, description, target_date, status, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                goal_id,
                "area-test-1",
                "Original Title",
                "Original description",
                None::<String>,
                "not-started",
                0,
                Utc::now().to_rfc3339(),
                Utc::now().to_rfc3339(),
                0,
                "[]",
            ],
        ).unwrap();
        
        // Update the goal
        let result = test_db.conn.execute(
            "UPDATE goals SET title = ?1, description = ?2, status = ?3, progress = ?4, updated_at = ?5 WHERE id = ?6",
            params![
                "Updated Title",
                "Updated description",
                "in-progress",
                50,
                Utc::now().to_rfc3339(),
                goal_id,
            ],
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1); // Should update 1 row
        
        // Verify update
        let goal: (String, String, i32) = test_db.conn.query_row(
            "SELECT title, status, progress FROM goals WHERE id = ?1",
            params![goal_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        ).unwrap();
        
        assert_eq!(goal.0, "Updated Title");
        assert_eq!(goal.1, "in-progress");
        assert_eq!(goal.2, 50);
    }
    
    #[test]
    fn test_goal_deletion_with_projects() {
        let test_db = setup_test_db();
        
        // Create a goal
        let goal_id = "goal-test-1";
        test_db.conn.execute(
            "INSERT INTO goals (id, area_id, title, description, target_date, status, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                goal_id,
                "area-test-1",
                "Goal with Projects",
                None::<String>,
                None::<String>,
                "not-started",
                0,
                Utc::now().to_rfc3339(),
                Utc::now().to_rfc3339(),
                0,
                "[]",
            ],
        ).unwrap();
        
        // Create a project under this goal
        test_db.conn.execute(
            "INSERT INTO projects (id, goal_id, title, description, status, start_date, end_date, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                "project-test-1",
                goal_id,
                "Test Project",
                None::<String>,
                "planning",
                None::<String>,
                None::<String>,
                0,
                Utc::now().to_rfc3339(),
                Utc::now().to_rfc3339(),
                0,
                "[]",
            ],
        ).unwrap();
        
        // Try to delete goal (should fail due to foreign key constraint)
        let result = test_db.conn.execute("DELETE FROM goals WHERE id = ?1", params![goal_id]);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_goal_sort_order() {
        let test_db = setup_test_db();
        
        // Get the maximum sort_order (should be None for empty table)
        let max_order: Option<i32> = test_db.conn.query_row(
            "SELECT MAX(sort_order) FROM goals WHERE area_id = ?1",
            params!["area-test-1"],
            |row| row.get(0),
        ).unwrap_or(None);
        
        assert_eq!(max_order, None);
        
        // Create goals with different sort orders
        for i in 1..=3 {
            test_db.conn.execute(
                "INSERT INTO goals (id, area_id, title, description, target_date, status, progress, created_at, updated_at, sort_order, tags)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    format!("goal-test-{}", i),
                    "area-test-1",
                    format!("Goal {}", i),
                    None::<String>,
                    None::<String>,
                    "not-started",
                    0,
                    Utc::now().to_rfc3339(),
                    Utc::now().to_rfc3339(),
                    i * 10,
                    "[]",
                ],
            ).unwrap();
        }
        
        // Get goals ordered by sort_order
        let mut stmt = test_db.conn.prepare("SELECT title, sort_order FROM goals WHERE area_id = ?1 ORDER BY sort_order").unwrap();
        let goals: Vec<(String, i32)> = stmt.query_map(params!["area-test-1"], |row| {
            Ok((row.get(0)?, row.get(1)?))
        }).unwrap().collect::<Result<Vec<_>, _>>().unwrap();
        
        assert_eq!(goals.len(), 3);
        assert_eq!(goals[0].1, 10);
        assert_eq!(goals[1].1, 20);
        assert_eq!(goals[2].1, 30);
    }
    
    #[test]
    fn test_goal_progress_validation() {
        let test_db = setup_test_db();
        
        // Test valid progress values
        for progress in [0, 25, 50, 75, 100].iter() {
            let goal_id = format!("goal-test-{}", progress);
            let result = test_db.conn.execute(
                "INSERT INTO goals (id, area_id, title, description, target_date, status, progress, created_at, updated_at, sort_order, tags)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    goal_id,
                    "area-test-1",
                    format!("Goal with {} progress", progress),
                    None::<String>,
                    None::<String>,
                    "not-started",
                    progress,
                    Utc::now().to_rfc3339(),
                    Utc::now().to_rfc3339(),
                    0,
                    "[]",
                ],
            );
            
            assert!(result.is_ok(), "Failed to insert goal with progress: {}", progress);
        }
    }
}