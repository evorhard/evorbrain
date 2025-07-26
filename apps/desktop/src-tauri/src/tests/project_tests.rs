#[cfg(test)]
mod project_tests {
    use crate::models::{Project, ProjectStatus};
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
        let test_db_path = temp_dir.path().join("test_project_crud.db");
        
        // Initialize database
        init_database(&test_db_path).unwrap();
        let conn = Connection::open(&test_db_path).unwrap();
        
        // Create a test area
        conn.execute(
            "INSERT INTO areas (id, title, description, color, icon, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), datetime('now'), ?6, ?7)",
            params!["area-test-1", "Test Area", "Test area for projects", "#3b82f6", "briefcase", 0, "[]"],
        ).unwrap();
        
        // Create a test goal
        conn.execute(
            "INSERT INTO goals (id, area_id, title, description, target_date, status, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'), datetime('now'), ?8, ?9)",
            params!["goal-test-1", "area-test-1", "Test Goal", "Test goal for projects", None::<String>, "not-started", 0, 0, "[]"],
        ).unwrap();
        
        TestDb {
            _temp_dir: temp_dir,
            conn,
        }
    }
    
    #[test]
    fn test_project_creation() {
        let test_db = setup_test_db();
        
        // Create a project
        let project = Project::new("goal-test-1".to_string(), "Test Project".to_string(), Some("Test description".to_string()));
        
        // Verify initial state
        assert_eq!(project.title, "Test Project");
        assert_eq!(project.goal_id, "goal-test-1");
        assert!(matches!(project.status, ProjectStatus::Planning));
        assert_eq!(project.progress, 0);
        assert_eq!(project.sort_order, 0);
        
        // Validate the project
        assert!(project.validate().is_ok());
    }

    #[test]
    fn test_project_insert_and_retrieve() {
        let test_db = setup_test_db();
        
        // Create a project
        let mut project = Project::new("goal-test-1".to_string(), "Test Project".to_string(), Some("Test description".to_string()));
        project.tags = Some(vec!["tag1".to_string(), "tag2".to_string()]);
        project.sort_order = 10;
        
        // Insert into database
        let result = test_db.conn.execute(
            "INSERT INTO projects (id, goal_id, title, description, status, start_date, end_date, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                &project.id,
                &project.goal_id,
                &project.title,
                &project.description,
                "planning",
                project.start_date,
                project.end_date,
                project.progress,
                project.created_at.to_rfc3339(),
                project.updated_at.to_rfc3339(),
                project.sort_order,
                serde_json::to_string(&project.tags).unwrap(),
            ],
        );
        assert!(result.is_ok());
        
        // Retrieve and verify
        let retrieved: (String, String, String, Option<String>, String, i32, i32, Option<String>) = test_db.conn.query_row(
            "SELECT id, goal_id, title, description, status, progress, sort_order, tags FROM projects WHERE id = ?1",
            params![&project.id],
            |row| Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
            )),
        ).unwrap();
        
        assert_eq!(retrieved.0, project.id);
        assert_eq!(retrieved.1, project.goal_id);
        assert_eq!(retrieved.2, project.title);
        assert_eq!(retrieved.3, project.description);
        assert_eq!(retrieved.4, "planning");
        assert_eq!(retrieved.5, 0);
        assert_eq!(retrieved.6, 10);
    }

    #[test]
    fn test_project_update() {
        let test_db = setup_test_db();
        
        // Create and insert a project
        let project = Project::new("goal-test-1".to_string(), "Original Title".to_string(), None);
        test_db.conn.execute(
            "INSERT INTO projects (id, goal_id, title, description, status, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                &project.id,
                &project.goal_id,
                &project.title,
                project.description,
                "planning",
                project.progress,
                project.created_at.to_rfc3339(),
                project.updated_at.to_rfc3339(),
                project.sort_order,
                "[]",
            ],
        ).unwrap();
        
        // Update the project
        let result = test_db.conn.execute(
            "UPDATE projects SET title = ?1, status = ?2, progress = ?3, updated_at = ?4 WHERE id = ?5",
            params!["Updated Title", "active", 50, Utc::now().to_rfc3339(), &project.id],
        );
        assert!(result.is_ok());
        
        // Verify update
        let (title, status, progress): (String, String, i32) = test_db.conn.query_row(
            "SELECT title, status, progress FROM projects WHERE id = ?1",
            params![&project.id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        ).unwrap();
        
        assert_eq!(title, "Updated Title");
        assert_eq!(status, "active");
        assert_eq!(progress, 50);
    }

    #[test]
    fn test_project_delete() {
        let test_db = setup_test_db();
        
        // Create and insert a project
        let project = Project::new("goal-test-1".to_string(), "To Delete".to_string(), None);
        test_db.conn.execute(
            "INSERT INTO projects (id, goal_id, title, description, status, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                &project.id,
                &project.goal_id,
                &project.title,
                project.description,
                "planning",
                project.progress,
                project.created_at.to_rfc3339(),
                project.updated_at.to_rfc3339(),
                project.sort_order,
                "[]",
            ],
        ).unwrap();
        
        // Delete the project
        let result = test_db.conn.execute("DELETE FROM projects WHERE id = ?1", params![&project.id]);
        assert!(result.is_ok());
        
        // Verify deletion
        let count: i32 = test_db.conn.query_row(
            "SELECT COUNT(*) FROM projects WHERE id = ?1",
            params![&project.id],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_project_validation() {
        // Test empty title validation
        let mut project = Project::new("goal-test-1".to_string(), "".to_string(), None);
        assert!(project.validate().is_err());
        
        // Test title too long validation
        project.title = "a".repeat(256);
        assert!(project.validate().is_err());
        
        // Test invalid progress
        project.title = "Valid Title".to_string();
        project.progress = -1;
        assert!(project.validate().is_err());
        
        project.progress = 101;
        assert!(project.validate().is_err());
        
        // Test invalid date order
        project.progress = 50;
        project.start_date = Some("2025-12-31T00:00:00Z".to_string());
        project.end_date = Some("2025-01-01T00:00:00Z".to_string());
        assert!(project.validate().is_err());
        
        // Test valid project
        project.start_date = Some("2025-01-01T00:00:00Z".to_string());
        project.end_date = Some("2025-12-31T00:00:00Z".to_string());
        assert!(project.validate().is_ok());
    }

    #[test]
    fn test_project_goal_relationship() {
        let test_db = setup_test_db();
        
        // Create two projects for the same goal
        let project1 = Project::new("goal-test-1".to_string(), "Project 1".to_string(), None);
        let project2 = Project::new("goal-test-1".to_string(), "Project 2".to_string(), None);
        
        // Insert both projects
        for project in &[&project1, &project2] {
            test_db.conn.execute(
                "INSERT INTO projects (id, goal_id, title, description, status, progress, created_at, updated_at, sort_order, tags)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![
                    &project.id,
                    &project.goal_id,
                    &project.title,
                    project.description,
                    "planning",
                    project.progress,
                    project.created_at.to_rfc3339(),
                    project.updated_at.to_rfc3339(),
                    project.sort_order,
                    "[]",
                ],
            ).unwrap();
        }
        
        // Count projects for the goal
        let count: i32 = test_db.conn.query_row(
            "SELECT COUNT(*) FROM projects WHERE goal_id = ?1",
            params!["goal-test-1"],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_project_status_transitions() {
        let test_db = setup_test_db();
        
        // Create and insert a project
        let project = Project::new("goal-test-1".to_string(), "Status Test".to_string(), None);
        test_db.conn.execute(
            "INSERT INTO projects (id, goal_id, title, description, status, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                &project.id,
                &project.goal_id,
                &project.title,
                project.description,
                "planning",
                project.progress,
                project.created_at.to_rfc3339(),
                project.updated_at.to_rfc3339(),
                project.sort_order,
                "[]",
            ],
        ).unwrap();
        
        // Test status transitions
        let statuses = ["planning", "active", "on-hold", "completed", "cancelled"];
        for status in &statuses {
            test_db.conn.execute(
                "UPDATE projects SET status = ?1 WHERE id = ?2",
                params![status, &project.id],
            ).unwrap();
            
            let retrieved_status: String = test_db.conn.query_row(
                "SELECT status FROM projects WHERE id = ?1",
                params![&project.id],
                |row| row.get(0),
            ).unwrap();
            
            assert_eq!(&retrieved_status, status);
        }
    }

    #[test]
    fn test_project_with_dates() {
        let test_db = setup_test_db();
        
        // Create a project with dates
        let mut project = Project::new("goal-test-1".to_string(), "Project with Dates".to_string(), None);
        project.start_date = Some("2025-07-01T00:00:00Z".to_string());
        project.end_date = Some("2025-12-31T23:59:59Z".to_string());
        
        // Validate dates
        assert!(project.validate().is_ok());
        
        // Insert with dates
        test_db.conn.execute(
            "INSERT INTO projects (id, goal_id, title, description, status, start_date, end_date, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                &project.id,
                &project.goal_id,
                &project.title,
                project.description,
                "planning",
                &project.start_date,
                &project.end_date,
                project.progress,
                project.created_at.to_rfc3339(),
                project.updated_at.to_rfc3339(),
                project.sort_order,
                "[]",
            ],
        ).unwrap();
        
        // Retrieve and verify dates
        let (start_date, end_date): (Option<String>, Option<String>) = test_db.conn.query_row(
            "SELECT start_date, end_date FROM projects WHERE id = ?1",
            params![&project.id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        ).unwrap();
        
        assert_eq!(start_date, project.start_date);
        assert_eq!(end_date, project.end_date);
    }
}