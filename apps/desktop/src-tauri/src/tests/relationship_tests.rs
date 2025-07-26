#[cfg(test)]
mod relationship_tests {
    use crate::database::init_database;
    use crate::commands::relationships::{
        get_entity_relationships_internal, 
        validate_entity_relationship_internal, check_cascade_impact_internal,
        update_children_on_parent_change_internal
    };
    use rusqlite::{Connection, params};
    use tempfile::TempDir;
    
    struct TestDb {
        _temp_dir: TempDir,
        conn: Connection,
    }
    
    fn setup_test_db() -> TestDb {
        // Create a temporary directory for the test database
        let temp_dir = TempDir::new().unwrap();
        let test_db_path = temp_dir.path().join("test_relationships.db");
        
        // Initialize database
        init_database(&test_db_path).unwrap();
        let conn = Connection::open(&test_db_path).unwrap();
        
        // Create test hierarchy: Area -> Goal -> Project -> Task -> Subtask
        // Test Area
        conn.execute(
            "INSERT INTO areas (id, title, description, icon, color, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), datetime('now'), ?6, ?7)",
            params!["area-test-1", "Test Area", "Test area description", None::<String>, None::<String>, 0, "[]"],
        ).unwrap();
        
        // Test Goals
        conn.execute(
            "INSERT INTO goals (id, area_id, title, description, target_date, progress, status, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'), datetime('now'), ?8, ?9)",
            params!["goal-test-1", "area-test-1", "Test Goal 1", "First test goal", None::<String>, 0, "not-started", 0, "[]"],
        ).unwrap();
        
        conn.execute(
            "INSERT INTO goals (id, area_id, title, description, target_date, progress, status, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'), datetime('now'), ?8, ?9)",
            params!["goal-test-2", "area-test-1", "Test Goal 2", "Second test goal", None::<String>, 50, "in-progress", 1, "[]"],
        ).unwrap();
        
        // Test Projects
        conn.execute(
            "INSERT INTO projects (id, goal_id, title, description, status, start_date, end_date, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'), datetime('now'), ?9, ?10)",
            params!["project-test-1", "goal-test-1", "Test Project 1", "First test project", "planning", None::<String>, None::<String>, 0, 0, "[]"],
        ).unwrap();
        
        conn.execute(
            "INSERT INTO projects (id, goal_id, title, description, status, start_date, end_date, progress, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, datetime('now'), datetime('now'), ?9, ?10)",
            params!["project-test-2", "goal-test-1", "Test Project 2", "Second test project", "active", None::<String>, None::<String>, 25, 1, "[]"],
        ).unwrap();
        
        // Test Tasks
        conn.execute(
            "INSERT INTO tasks (id, project_id, parent_task_id, title, description, status, priority, 
             created_at, updated_at, sort_order, tags, estimated_minutes, actual_minutes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'), datetime('now'), ?8, ?9, ?10, ?11)",
            params![
                "task-test-1", "project-test-1", None::<String>, "Test Task 1", "First test task", 
                "not-started", "medium", 0, "[]", None::<i32>, None::<i32>
            ],
        ).unwrap();
        
        conn.execute(
            "INSERT INTO tasks (id, project_id, parent_task_id, title, description, status, priority, 
             created_at, updated_at, sort_order, tags, estimated_minutes, actual_minutes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'), datetime('now'), ?8, ?9, ?10, ?11)",
            params![
                "task-test-2", "project-test-1", None::<String>, "Test Task 2", "Second test task", 
                "in-progress", "high", 1, "[]", 60, 30
            ],
        ).unwrap();
        
        // Test Subtasks
        conn.execute(
            "INSERT INTO tasks (id, project_id, parent_task_id, title, description, status, priority, 
             created_at, updated_at, sort_order, tags, estimated_minutes, actual_minutes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'), datetime('now'), ?8, ?9, ?10, ?11)",
            params![
                "subtask-test-1", None::<String>, "task-test-1", "Test Subtask 1", "First subtask", 
                "not-started", "low", 0, "[]", 30, None::<i32>
            ],
        ).unwrap();
        
        conn.execute(
            "INSERT INTO tasks (id, project_id, parent_task_id, title, description, status, priority, 
             created_at, updated_at, sort_order, tags, estimated_minutes, actual_minutes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'), datetime('now'), ?8, ?9, ?10, ?11)",
            params![
                "subtask-test-2", None::<String>, "task-test-1", "Test Subtask 2", "Second subtask", 
                "completed", "medium", 1, "[]", 45, 50
            ],
        ).unwrap();
        
        TestDb {
            _temp_dir: temp_dir,
            conn,
        }
    }
    
    #[test]
    fn test_get_entity_relationships_area() {
        let test_db = setup_test_db();
        
        let relationships = get_entity_relationships_internal(
            &test_db.conn,
            "area-test-1".to_string(),
            "area".to_string()
        ).unwrap();
        
        assert_eq!(relationships.entity_id, "area-test-1");
        assert_eq!(relationships.entity_type, "area");
        assert_eq!(relationships.parents.len(), 0); // Areas have no parents
        assert_eq!(relationships.children.len(), 2); // Two goals
        
        // Check children are goals
        assert!(relationships.children.iter().all(|c| c.entity_type == "goal"));
        assert!(relationships.children.iter().any(|c| c.title == "Test Goal 1"));
        assert!(relationships.children.iter().any(|c| c.title == "Test Goal 2"));
    }
    
    #[test]
    fn test_get_entity_relationships_goal() {
        let test_db = setup_test_db();
        
        let relationships = get_entity_relationships_internal(
            &test_db.conn,
            "goal-test-1".to_string(),
            "goal".to_string()
        ).unwrap();
        
        assert_eq!(relationships.entity_id, "goal-test-1");
        assert_eq!(relationships.entity_type, "goal");
        assert_eq!(relationships.parents.len(), 1); // One parent area
        assert_eq!(relationships.children.len(), 2); // Two projects
        
        // Check parent is area
        assert_eq!(relationships.parents[0].entity_type, "area");
        assert_eq!(relationships.parents[0].title, "Test Area");
        
        // Check children are projects
        assert!(relationships.children.iter().all(|c| c.entity_type == "project"));
    }
    
    #[test]
    fn test_get_entity_relationships_task() {
        let test_db = setup_test_db();
        
        let relationships = get_entity_relationships_internal(
            &test_db.conn,
            "task-test-1".to_string(),
            "task".to_string()
        ).unwrap();
        
        assert_eq!(relationships.entity_id, "task-test-1");
        assert_eq!(relationships.entity_type, "task");
        assert_eq!(relationships.parents.len(), 1); // One parent project
        assert_eq!(relationships.children.len(), 2); // Two subtasks
        
        // Check parent is project
        assert_eq!(relationships.parents[0].entity_type, "project");
        assert_eq!(relationships.parents[0].title, "Test Project 1");
        
        // Check children are tasks
        assert!(relationships.children.iter().all(|c| c.entity_type == "task"));
        assert!(relationships.children.iter().any(|c| c.title == "Test Subtask 1"));
        assert!(relationships.children.iter().any(|c| c.title == "Test Subtask 2"));
    }
    
    #[test]
    fn test_validate_entity_relationship() {
        let test_db = setup_test_db();
        
        // Valid relationships
        assert!(validate_entity_relationship_internal(
            &test_db.conn,
            "goal".to_string(),
            "goal-test-1".to_string(),
            "area".to_string(),
            "area-test-1".to_string()
        ).unwrap());
        
        assert!(validate_entity_relationship_internal(
            &test_db.conn,
            "project".to_string(),
            "project-test-1".to_string(),
            "goal".to_string(),
            "goal-test-1".to_string()
        ).unwrap());
        
        assert!(validate_entity_relationship_internal(
            &test_db.conn,
            "task".to_string(),
            "task-test-1".to_string(),
            "project".to_string(),
            "project-test-1".to_string()
        ).unwrap());
        
        // Invalid relationships
        assert!(!validate_entity_relationship_internal(
            &test_db.conn,
            "area".to_string(),
            "area-test-1".to_string(),
            "goal".to_string(),
            "goal-test-1".to_string()
        ).unwrap());
        
        // Non-existent parent
        assert!(!validate_entity_relationship_internal(
            &test_db.conn,
            "goal".to_string(),
            "goal-test-1".to_string(),
            "area".to_string(),
            "area-non-existent".to_string()
        ).unwrap());
        
        // Circular task reference
        assert!(!validate_entity_relationship_internal(
            &test_db.conn,
            "task".to_string(),
            "task-test-1".to_string(),
            "task".to_string(),
            "task-test-1".to_string()
        ).unwrap());
    }
    
    #[test]
    fn test_check_cascade_impact() {
        let test_db = setup_test_db();
        
        // Check cascade impact for area
        let area_impact = check_cascade_impact_internal(
            &test_db.conn,
            "area-test-1".to_string(),
            "area".to_string()
        ).unwrap();
        
        assert_eq!(area_impact.entity_id, "area-test-1");
        assert_eq!(area_impact.entity_type, "area");
        assert_eq!(area_impact.affected_entities["goals"], 2);
        assert_eq!(area_impact.affected_entities["projects"], 2);
        assert_eq!(area_impact.affected_entities["tasks"], 2);
        assert_eq!(area_impact.total_affected, 6);
        
        // Check cascade impact for goal
        let goal_impact = check_cascade_impact_internal(
            &test_db.conn,
            "goal-test-1".to_string(),
            "goal".to_string()
        ).unwrap();
        
        assert_eq!(goal_impact.entity_id, "goal-test-1");
        assert_eq!(goal_impact.entity_type, "goal");
        assert_eq!(goal_impact.affected_entities["projects"], 2);
        assert_eq!(goal_impact.affected_entities["tasks"], 2);
        assert_eq!(goal_impact.total_affected, 4);
        
        // Check cascade impact for project
        let project_impact = check_cascade_impact_internal(
            &test_db.conn,
            "project-test-1".to_string(),
            "project".to_string()
        ).unwrap();
        
        assert_eq!(project_impact.entity_id, "project-test-1");
        assert_eq!(project_impact.entity_type, "project");
        assert_eq!(project_impact.affected_entities["tasks"], 2);
        assert_eq!(project_impact.total_affected, 2);
        
        // Check cascade impact for task with subtasks
        let task_impact = check_cascade_impact_internal(
            &test_db.conn,
            "task-test-1".to_string(),
            "task".to_string()
        ).unwrap();
        
        assert_eq!(task_impact.entity_id, "task-test-1");
        assert_eq!(task_impact.entity_type, "task");
        assert_eq!(task_impact.affected_entities["subtasks"], 2);
        assert_eq!(task_impact.total_affected, 2);
    }
    
    #[test]
    fn test_update_children_on_parent_change() {
        let test_db = setup_test_db();
        
        // Test updating children when goal is completed
        let affected = update_children_on_parent_change_internal(
            &test_db.conn,
            "goal-test-1".to_string(),
            "goal".to_string(),
            "status".to_string(),
            Some("completed".to_string())
        ).unwrap();
        
        assert_eq!(affected, 2); // Two projects should be updated
        
        // Verify projects were updated to completed
        let project_status: String = test_db.conn.query_row(
            "SELECT status FROM projects WHERE id = ?1",
            params!["project-test-1"],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(project_status, "completed");
        
        // Test updating children when project is cancelled
        let affected = update_children_on_parent_change_internal(
            &test_db.conn,
            "project-test-1".to_string(),
            "project".to_string(),
            "status".to_string(),
            Some("cancelled".to_string())
        ).unwrap();
        
        assert_eq!(affected, 2); // Two tasks should be updated
        
        // Verify tasks were updated to cancelled
        let task_status: String = test_db.conn.query_row(
            "SELECT status FROM tasks WHERE id = ?1",
            params!["task-test-1"],
            |row| row.get(0),
        ).unwrap();
        
        assert_eq!(task_status, "cancelled");
    }
    
    #[test]
    fn test_circular_reference_prevention() {
        let test_db = setup_test_db();
        
        // Create a chain of tasks: A -> B -> C
        test_db.conn.execute(
            "INSERT INTO tasks (id, parent_task_id, title, status, priority, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), datetime('now'), ?6, ?7)",
            params!["task-a", None::<String>, "Task A", "not-started", "medium", 0, "[]"],
        ).unwrap();
        
        test_db.conn.execute(
            "INSERT INTO tasks (id, parent_task_id, title, status, priority, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), datetime('now'), ?6, ?7)",
            params!["task-b", "task-a", "Task B", "not-started", "medium", 0, "[]"],
        ).unwrap();
        
        test_db.conn.execute(
            "INSERT INTO tasks (id, parent_task_id, title, status, priority, created_at, updated_at, sort_order, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), datetime('now'), ?6, ?7)",
            params!["task-c", "task-b", "Task C", "not-started", "medium", 0, "[]"],
        ).unwrap();
        
        // Try to make A a child of C (would create circular reference)
        let is_valid = validate_entity_relationship_internal(
            &test_db.conn,
            "task".to_string(),
            "task-a".to_string(),
            "task".to_string(),
            "task-c".to_string()
        ).unwrap();
        
        assert!(!is_valid, "Should not allow circular task reference");
    }
    
    #[test]
    fn test_references_in_descriptions() {
        let test_db = setup_test_db();
        
        // Add references in descriptions
        test_db.conn.execute(
            "UPDATE goals SET description = 'This goal references [[task:task-test-1]] for implementation' WHERE id = 'goal-test-1'",
            params![],
        ).unwrap();
        
        test_db.conn.execute(
            "UPDATE projects SET description = 'This project depends on [[goal:goal-test-2]] completion' WHERE id = 'project-test-1'",
            params![],
        ).unwrap();
        
        // Test will verify that references are found correctly
        // The actual test would use the get_entity_relationships function
        // which includes reference tracking
    }
}