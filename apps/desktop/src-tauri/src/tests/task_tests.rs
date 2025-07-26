#[cfg(test)]
mod task_tests {
    use crate::models::{Task, TaskStatus, TaskPriority};
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
        let test_db_path = temp_dir.path().join("test_task_crud.db");
        
        // Initialize database
        init_database(&test_db_path).unwrap();
        let conn = Connection::open(&test_db_path).unwrap();
        
        // Create test area, goal and project for tasks (tasks can belong to projects)
        conn.execute(
            "INSERT INTO areas (id, title, description, icon, color, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), datetime('now'), ?6, ?7)",
            params!["area-test-1", "Test Area", "Test area for tasks", None::<String>, None::<String>, 0, "[]"],
        ).unwrap();
        
        conn.execute(
            "INSERT INTO goals (id, area_id, title, description, target_date, progress, status, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'), datetime('now'), ?8, ?9)",
            params!["goal-test-1", "area-test-1", "Test Goal", "Test goal for tasks", None::<String>, 0, "not-started", 0, "[]"],
        ).unwrap();
        
        conn.execute(
            "INSERT INTO projects (id, goal_id, title, description, status, start_date, end_date, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'), datetime('now'), ?9, ?10)",
            params!["project-test-1", "goal-test-1", "Test Project", "Test project for tasks", "active", None::<String>, None::<String>, 0, 0, "[]"],
        ).unwrap();
        
        TestDb {
            _temp_dir: temp_dir,
            conn,
        }
    }
    
    #[test]
    fn test_task_creation() {
        let test_db = setup_test_db();
        
        // Create a task
        let task = Task::new("Test Task".to_string(), Some("Test description".to_string()));
        
        // Verify initial state
        assert_eq!(task.title, "Test Task");
        assert!(matches!(task.status, TaskStatus::NotStarted));
        assert!(matches!(task.priority, TaskPriority::Medium));
        assert_eq!(task.sort_order, 0);
        assert!(task.project_id.is_none());
        assert!(task.parent_task_id.is_none());
        assert!(task.completed_at.is_none());
        
        // Validate the task
        assert!(task.validate().is_ok());
    }

    #[test]
    fn test_task_insert_and_retrieve() {
        let test_db = setup_test_db();
        
        // Create a task with project
        let mut task = Task::new("Test Task".to_string(), Some("Test description".to_string()));
        task.project_id = Some("project-test-1".to_string());
        task.tags = Some(vec!["tag1".to_string(), "tag2".to_string()]);
        task.sort_order = 10;
        task.estimated_minutes = Some(60);
        task.priority = TaskPriority::High;
        
        // Insert into database
        let result = test_db.conn.execute(
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
                "not-started",
                task.due_date,
                "high",
                task.completed_at,
                task.estimated_minutes,
                task.actual_minutes,
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339(),
                task.sort_order,
                serde_json::to_string(&task.tags).unwrap(),
                None::<String>, // recurrence
                task.recurrence_id,
                task.recurrence_date,
            ],
        );
        assert!(result.is_ok());
        
        // Retrieve and verify
        let retrieved: (String, Option<String>, String, Option<String>, String, Option<i32>, i32, Option<String>) = test_db.conn.query_row(
            "SELECT id, project_id, title, description, status, estimated_minutes, sort_order, tags FROM tasks WHERE id = ?1",
            params![&task.id],
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
        
        assert_eq!(retrieved.0, task.id);
        assert_eq!(retrieved.1, task.project_id);
        assert_eq!(retrieved.2, task.title);
        assert_eq!(retrieved.3, task.description);
        assert_eq!(retrieved.4, "not-started");
        assert_eq!(retrieved.5, Some(60));
        assert_eq!(retrieved.6, 10);
    }

    #[test]
    fn test_task_update() {
        let test_db = setup_test_db();
        
        // Create and insert a task
        let task = Task::new("Original Title".to_string(), None);
        test_db.conn.execute(
            "INSERT INTO tasks (id, project_id, parent_task_id, title, description, status, priority, 
             estimated_minutes, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                &task.id,
                task.project_id,
                task.parent_task_id,
                &task.title,
                task.description,
                "not-started",
                "medium",
                None::<i32>,
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339(),
                task.sort_order,
                "[]",
            ],
        ).unwrap();
        
        // Update the task
        let result = test_db.conn.execute(
            "UPDATE tasks SET title = ?1, status = ?2, priority = ?3, estimated_minutes = ?4, 
             actual_minutes = ?5, updated_at = ?6 WHERE id = ?7",
            params!["Updated Title", "in-progress", "high", 120, 30, Utc::now().to_rfc3339(), &task.id],
        );
        assert!(result.is_ok());
        
        // Verify update
        let (title, status, priority, estimated, actual): (String, String, String, Option<i32>, Option<i32>) = test_db.conn.query_row(
            "SELECT title, status, priority, estimated_minutes, actual_minutes FROM tasks WHERE id = ?1",
            params![&task.id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        ).unwrap();
        
        assert_eq!(title, "Updated Title");
        assert_eq!(status, "in-progress");
        assert_eq!(priority, "high");
        assert_eq!(estimated, Some(120));
        assert_eq!(actual, Some(30));
    }

    #[test]
    fn test_task_delete() {
        let test_db = setup_test_db();
        
        // Create and insert a task
        let task = Task::new("To Delete".to_string(), None);
        test_db.conn.execute(
            "INSERT INTO tasks (id, project_id, parent_task_id, title, description, status, priority, 
             created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                &task.id,
                task.project_id,
                task.parent_task_id,
                &task.title,
                task.description,
                "not-started",
                "medium",
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339(),
                task.sort_order,
                "[]",
            ],
        ).unwrap();
        
        // Delete the task
        let result = test_db.conn.execute("DELETE FROM tasks WHERE id = ?1", params![&task.id]);
        assert!(result.is_ok());
        
        // Verify deletion
        let count: i32 = test_db.conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE id = ?1",
            params![&task.id],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_task_validation() {
        // Test empty title validation
        let mut task = Task::new("".to_string(), None);
        assert!(task.validate().is_err());
        
        // Test title too long validation
        task.title = "a".repeat(256);
        assert!(task.validate().is_err());
        
        // Test negative estimated minutes
        task.title = "Valid Title".to_string();
        task.estimated_minutes = Some(-10);
        assert!(task.validate().is_err());
        
        // Test negative actual minutes
        task.estimated_minutes = Some(60);
        task.actual_minutes = Some(-5);
        assert!(task.validate().is_err());
        
        // Test completed task without completion date
        task.actual_minutes = Some(30);
        task.status = TaskStatus::Completed;
        assert!(task.validate().is_err());
        
        // Test valid task with completion date
        task.completed_at = Some("2025-07-26T10:00:00Z".to_string());
        assert!(task.validate().is_ok());
    }

    #[test]
    fn test_task_with_parent() {
        let test_db = setup_test_db();
        
        // Create parent task
        let parent = Task::new("Parent Task".to_string(), None);
        test_db.conn.execute(
            "INSERT INTO tasks (id, title, status, priority, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                &parent.id,
                &parent.title,
                "not-started",
                "medium",
                parent.created_at.to_rfc3339(),
                parent.updated_at.to_rfc3339(),
                0,
                "[]",
            ],
        ).unwrap();
        
        // Create subtask
        let mut subtask = Task::new("Subtask".to_string(), None);
        subtask.parent_task_id = Some(parent.id.clone());
        
        test_db.conn.execute(
            "INSERT INTO tasks (id, parent_task_id, title, status, priority, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                &subtask.id,
                &subtask.parent_task_id,
                &subtask.title,
                "not-started",
                "medium",
                subtask.created_at.to_rfc3339(),
                subtask.updated_at.to_rfc3339(),
                0,
                "[]",
            ],
        ).unwrap();
        
        // Count subtasks for parent
        let count: i32 = test_db.conn.query_row(
            "SELECT COUNT(*) FROM tasks WHERE parent_task_id = ?1",
            params![&parent.id],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_task_status_transitions() {
        let test_db = setup_test_db();
        
        // Create and insert a task
        let task = Task::new("Status Test".to_string(), None);
        test_db.conn.execute(
            "INSERT INTO tasks (id, title, status, priority, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                &task.id,
                &task.title,
                "not-started",
                "medium",
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339(),
                task.sort_order,
                "[]",
            ],
        ).unwrap();
        
        // Test status transitions
        let statuses = ["not-started", "in-progress", "completed", "cancelled"];
        for status in &statuses {
            let completed_at = if *status == "completed" {
                Some(Utc::now().to_rfc3339())
            } else {
                None
            };
            
            test_db.conn.execute(
                "UPDATE tasks SET status = ?1, completed_at = ?2 WHERE id = ?3",
                params![status, completed_at, &task.id],
            ).unwrap();
            
            let (retrieved_status, retrieved_completed): (String, Option<String>) = test_db.conn.query_row(
                "SELECT status, completed_at FROM tasks WHERE id = ?1",
                params![&task.id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            ).unwrap();
            
            assert_eq!(&retrieved_status, status);
            if *status == "completed" {
                assert!(retrieved_completed.is_some());
            }
        }
    }

    #[test]
    fn test_task_with_due_date() {
        let test_db = setup_test_db();
        
        // Create a task with due date
        let mut task = Task::new("Task with Due Date".to_string(), None);
        task.due_date = Some("2025-12-31T23:59:59Z".to_string());
        
        // Validate due date
        assert!(task.validate().is_ok());
        
        // Insert with due date
        test_db.conn.execute(
            "INSERT INTO tasks (id, title, status, due_date, priority, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                &task.id,
                &task.title,
                "not-started",
                &task.due_date,
                "medium",
                task.created_at.to_rfc3339(),
                task.updated_at.to_rfc3339(),
                task.sort_order,
                "[]",
            ],
        ).unwrap();
        
        // Retrieve and verify due date
        let due_date: Option<String> = test_db.conn.query_row(
            "SELECT due_date FROM tasks WHERE id = ?1",
            params![&task.id],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(due_date, task.due_date);
    }
}