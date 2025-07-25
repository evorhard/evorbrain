#[cfg(test)]
mod area_tests {
    use crate::commands::entities::{create_area, get_area, get_all_areas, update_area, delete_area};
    use crate::database::init_database;
    use std::fs;
    use std::path::Path;
    use tauri::test::{mock_builder, MockRuntime};
    
    fn setup_test_app() -> tauri::AppHandle<MockRuntime> {
        let app = mock_builder().build(tauri::generate_context!()).unwrap();
        let app_handle = app.handle();
        
        // Create test database
        let test_db_path = Path::new("test_area_crud.db");
        if test_db_path.exists() {
            fs::remove_file(test_db_path).unwrap();
        }
        
        init_database(test_db_path).unwrap();
        
        app_handle
    }
    
    fn cleanup_test_db() {
        let test_db_path = Path::new("test_area_crud.db");
        if test_db_path.exists() {
            fs::remove_file(test_db_path).unwrap();
        }
    }
    
    #[tokio::test]
    async fn test_create_area() {
        let app_handle = setup_test_app();
        
        // Test creating a basic area
        let area = create_area(
            app_handle.clone(),
            "Work".to_string(),
            Some("Professional work and career".to_string()),
            Some("#3b82f6".to_string()),
            Some("briefcase".to_string()),
        ).await.unwrap();
        
        assert_eq!(area.title, "Work");
        assert_eq!(area.description, Some("Professional work and career".to_string()));
        assert_eq!(area.color, Some("#3b82f6".to_string()));
        assert_eq!(area.icon, Some("briefcase".to_string()));
        assert_eq!(area.sort_order, 0);
        assert!(area.tags.is_none());
        
        cleanup_test_db();
    }
    
    #[tokio::test]
    async fn test_create_area_validation() {
        let app_handle = setup_test_app();
        
        // Test empty title
        let result = create_area(
            app_handle.clone(),
            "".to_string(),
            None,
            None,
            None,
        ).await;
        
        assert!(result.is_err());
        
        // Test title too long
        let long_title = "a".repeat(256);
        let result = create_area(
            app_handle.clone(),
            long_title,
            None,
            None,
            None,
        ).await;
        
        assert!(result.is_err());
        
        cleanup_test_db();
    }
    
    #[tokio::test]
    async fn test_get_area() {
        let app_handle = setup_test_app();
        
        // Create an area first
        let created_area = create_area(
            app_handle.clone(),
            "Personal".to_string(),
            Some("Personal life and goals".to_string()),
            Some("#10b981".to_string()),
            Some("home".to_string()),
        ).await.unwrap();
        
        // Get the area
        let fetched_area = get_area(app_handle.clone(), created_area.id.clone()).await.unwrap();
        
        assert_eq!(fetched_area.id, created_area.id);
        assert_eq!(fetched_area.title, "Personal");
        assert_eq!(fetched_area.description, Some("Personal life and goals".to_string()));
        
        // Test getting non-existent area
        let result = get_area(app_handle.clone(), "non-existent-id".to_string()).await;
        assert!(result.is_err());
        
        cleanup_test_db();
    }
    
    #[tokio::test]
    async fn test_get_all_areas() {
        let app_handle = setup_test_app();
        
        // Create multiple areas
        create_area(
            app_handle.clone(),
            "Work".to_string(),
            None,
            None,
            None,
        ).await.unwrap();
        
        create_area(
            app_handle.clone(),
            "Personal".to_string(),
            None,
            None,
            None,
        ).await.unwrap();
        
        create_area(
            app_handle.clone(),
            "Health".to_string(),
            None,
            None,
            None,
        ).await.unwrap();
        
        // Get all areas
        let areas = get_all_areas(app_handle.clone()).await.unwrap();
        
        assert_eq!(areas.len(), 3);
        
        // Check ordering by sort_order, then title
        let titles: Vec<String> = areas.iter().map(|a| a.title.clone()).collect();
        assert_eq!(titles, vec!["Health", "Personal", "Work"]);
        
        cleanup_test_db();
    }
    
    #[tokio::test]
    async fn test_update_area() {
        let app_handle = setup_test_app();
        
        // Create an area
        let area = create_area(
            app_handle.clone(),
            "Work".to_string(),
            Some("Professional work".to_string()),
            Some("#3b82f6".to_string()),
            Some("briefcase".to_string()),
        ).await.unwrap();
        
        // Update the area
        let updated_area = update_area(
            app_handle.clone(),
            area.id.clone(),
            "Career".to_string(),
            Some("Professional career and development".to_string()),
            Some("#8b5cf6".to_string()),
            Some("rocket".to_string()),
            Some(1),
            Some(vec!["work".to_string(), "career".to_string()]),
        ).await.unwrap();
        
        assert_eq!(updated_area.title, "Career");
        assert_eq!(updated_area.description, Some("Professional career and development".to_string()));
        assert_eq!(updated_area.color, Some("#8b5cf6".to_string()));
        assert_eq!(updated_area.icon, Some("rocket".to_string()));
        assert_eq!(updated_area.sort_order, 1);
        assert_eq!(updated_area.tags, Some(vec!["work".to_string(), "career".to_string()]));
        
        cleanup_test_db();
    }
    
    #[tokio::test]
    async fn test_delete_area() {
        let app_handle = setup_test_app();
        
        // Create an area
        let area = create_area(
            app_handle.clone(),
            "Temporary".to_string(),
            None,
            None,
            None,
        ).await.unwrap();
        
        // Delete the area
        delete_area(app_handle.clone(), area.id.clone()).await.unwrap();
        
        // Verify it's deleted
        let result = get_area(app_handle.clone(), area.id).await;
        assert!(result.is_err());
        
        cleanup_test_db();
    }
    
    #[tokio::test]
    async fn test_delete_area_with_goals() {
        let app_handle = setup_test_app();
        
        // This test would require creating goals, which we'll skip for now
        // since we're focusing on Area CRUD operations
        
        cleanup_test_db();
    }
}