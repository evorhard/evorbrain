use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errors::AppError;

/// Base entity trait with common fields
pub trait Entity {
    fn id(&self) -> &str;
    fn created_at(&self) -> &DateTime<Utc>;
    fn updated_at(&self) -> &DateTime<Utc>;
    fn title(&self) -> &str;
    fn description(&self) -> Option<&str>;
    fn tags(&self) -> Option<&Vec<String>>;
}

/// Area entity - Top level organizational unit
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub sort_order: i32,
}

/// Goal entity - Belongs to an Area
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub area_id: String,
    pub target_date: Option<String>,
    pub progress: i32, // 0-100
    pub status: GoalStatus,
    pub sort_order: i32,
}

/// Project entity - Belongs to a Goal
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub goal_id: String,
    pub status: ProjectStatus,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub progress: i32, // 0-100
    pub sort_order: i32,
}

/// Task entity - Can belong to a Project or be standalone
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub project_id: Option<String>,
    pub parent_task_id: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<String>,
    pub completed_at: Option<String>,
    pub estimated_minutes: Option<i32>,
    pub actual_minutes: Option<i32>,
    pub sort_order: i32,
    // Recurring task properties
    pub recurrence: Option<RecurrenceRule>,
    pub recurrence_id: Option<String>,
    pub recurrence_date: Option<String>,
}

/// Recurrence rule for tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurrenceRule {
    pub frequency: RecurrenceFrequency,
    pub interval: i32,
    pub end_date: Option<String>,
    pub end_count: Option<i32>,
    pub week_days: Option<Vec<i32>>, // 0-6, Sunday to Saturday
    pub month_day: Option<i32>, // 1-31
    pub exceptions: Option<Vec<String>>, // ISO date strings
}

/// Recurrence frequency options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecurrenceFrequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

/// Goal status values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum GoalStatus {
    NotStarted,
    InProgress,
    Completed,
    Abandoned,
}

/// Project status values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Planning,
    Active,
    #[serde(rename = "on-hold")]
    OnHold,
    Completed,
    Cancelled,
}

/// Task-specific status values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Completed,
    Cancelled,
}

/// Task priority values
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

// Implement Entity trait for each type
impl Entity for Area {
    fn id(&self) -> &str { &self.id }
    fn created_at(&self) -> &DateTime<Utc> { &self.created_at }
    fn updated_at(&self) -> &DateTime<Utc> { &self.updated_at }
    fn title(&self) -> &str { &self.title }
    fn description(&self) -> Option<&str> { self.description.as_deref() }
    fn tags(&self) -> Option<&Vec<String>> { self.tags.as_ref() }
}

impl Entity for Goal {
    fn id(&self) -> &str { &self.id }
    fn created_at(&self) -> &DateTime<Utc> { &self.created_at }
    fn updated_at(&self) -> &DateTime<Utc> { &self.updated_at }
    fn title(&self) -> &str { &self.title }
    fn description(&self) -> Option<&str> { self.description.as_deref() }
    fn tags(&self) -> Option<&Vec<String>> { self.tags.as_ref() }
}

impl Entity for Project {
    fn id(&self) -> &str { &self.id }
    fn created_at(&self) -> &DateTime<Utc> { &self.created_at }
    fn updated_at(&self) -> &DateTime<Utc> { &self.updated_at }
    fn title(&self) -> &str { &self.title }
    fn description(&self) -> Option<&str> { self.description.as_deref() }
    fn tags(&self) -> Option<&Vec<String>> { self.tags.as_ref() }
}

impl Entity for Task {
    fn id(&self) -> &str { &self.id }
    fn created_at(&self) -> &DateTime<Utc> { &self.created_at }
    fn updated_at(&self) -> &DateTime<Utc> { &self.updated_at }
    fn title(&self) -> &str { &self.title }
    fn description(&self) -> Option<&str> { self.description.as_deref() }
    fn tags(&self) -> Option<&Vec<String>> { self.tags.as_ref() }
}

// Helper functions for creating new entities
impl Area {
    pub fn new(title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            title,
            description,
            tags: None,
            entity_type: "area".to_string(),
            color: None,
            icon: None,
            sort_order: 0,
        }
    }
}

impl Goal {
    pub fn new(area_id: String, title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            title,
            description,
            tags: None,
            entity_type: "goal".to_string(),
            area_id,
            target_date: None,
            progress: 0,
            status: GoalStatus::NotStarted,
            sort_order: 0,
        }
    }
}

impl Project {
    pub fn new(goal_id: String, title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            title,
            description,
            tags: None,
            entity_type: "project".to_string(),
            goal_id,
            status: ProjectStatus::Planning,
            start_date: None,
            end_date: None,
            progress: 0,
            sort_order: 0,
        }
    }
}

impl Task {
    pub fn new(title: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            title,
            description,
            tags: None,
            entity_type: "task".to_string(),
            project_id: None,
            parent_task_id: None,
            status: TaskStatus::NotStarted,
            priority: TaskPriority::Medium,
            due_date: None,
            completed_at: None,
            estimated_minutes: None,
            actual_minutes: None,
            sort_order: 0,
            recurrence: None,
            recurrence_id: None,
            recurrence_date: None,
        }
    }
}

// Validation implementations
impl Area {
    /// Validate the area entity
    pub fn validate(&self) -> Result<(), AppError> {
        if self.title.trim().is_empty() {
            return Err(AppError::validation(
                "Area title cannot be empty",
                Some("title".to_string()),
            ));
        }

        if self.title.len() > 255 {
            return Err(AppError::validation(
                "Area title cannot exceed 255 characters",
                Some("title".to_string()),
            ));
        }

        if let Some(desc) = &self.description {
            if desc.len() > 10000 {
                return Err(AppError::validation(
                    "Area description cannot exceed 10000 characters",
                    Some("description".to_string()),
                ));
            }
        }

        if let Some(color) = &self.color {
            if !color.starts_with('#') || (color.len() != 7 && color.len() != 4) {
                return Err(AppError::validation(
                    "Area color must be a valid hex color (e.g., #FF0000 or #F00)",
                    Some("color".to_string()),
                ));
            }
        }

        Ok(())
    }
}

impl Goal {
    /// Validate the goal entity
    pub fn validate(&self) -> Result<(), AppError> {
        if self.title.trim().is_empty() {
            return Err(AppError::validation(
                "Goal title cannot be empty",
                Some("title".to_string()),
            ));
        }

        if self.title.len() > 255 {
            return Err(AppError::validation(
                "Goal title cannot exceed 255 characters",
                Some("title".to_string()),
            ));
        }

        if self.area_id.trim().is_empty() {
            return Err(AppError::validation(
                "Goal must belong to an area",
                Some("area_id".to_string()),
            ));
        }

        if self.progress < 0 || self.progress > 100 {
            return Err(AppError::validation(
                "Goal progress must be between 0 and 100",
                Some("progress".to_string()),
            ));
        }

        Ok(())
    }
}

impl Project {
    /// Validate the project entity
    pub fn validate(&self) -> Result<(), AppError> {
        if self.title.trim().is_empty() {
            return Err(AppError::validation(
                "Project title cannot be empty",
                Some("title".to_string()),
            ));
        }

        if self.title.len() > 255 {
            return Err(AppError::validation(
                "Project title cannot exceed 255 characters",
                Some("title".to_string()),
            ));
        }

        if self.goal_id.trim().is_empty() {
            return Err(AppError::validation(
                "Project must belong to a goal",
                Some("goal_id".to_string()),
            ));
        }

        if self.progress < 0 || self.progress > 100 {
            return Err(AppError::validation(
                "Project progress must be between 0 and 100",
                Some("progress".to_string()),
            ));
        }

        // Validate date constraints
        if let (Some(start), Some(end)) = (&self.start_date, &self.end_date) {
            if start > end {
                return Err(AppError::validation(
                    "Project start date must be before end date",
                    Some("dates".to_string()),
                ));
            }
        }

        Ok(())
    }
}

impl Task {
    /// Validate the task entity
    pub fn validate(&self) -> Result<(), AppError> {
        if self.title.trim().is_empty() {
            return Err(AppError::validation(
                "Task title cannot be empty",
                Some("title".to_string()),
            ));
        }

        if self.title.len() > 255 {
            return Err(AppError::validation(
                "Task title cannot exceed 255 characters",
                Some("title".to_string()),
            ));
        }

        // Validate time tracking
        if let Some(estimated) = self.estimated_minutes {
            if estimated < 0 {
                return Err(AppError::validation(
                    "Estimated minutes cannot be negative",
                    Some("estimated_minutes".to_string()),
                ));
            }
        }

        if let Some(actual) = self.actual_minutes {
            if actual < 0 {
                return Err(AppError::validation(
                    "Actual minutes cannot be negative",
                    Some("actual_minutes".to_string()),
                ));
            }
        }

        // Validate recurrence
        if let Some(recurrence) = &self.recurrence {
            recurrence.validate()?;
        }

        // Validate completed tasks have completion date
        if self.status == TaskStatus::Completed && self.completed_at.is_none() {
            return Err(AppError::validation(
                "Completed tasks must have a completion date",
                Some("completed_at".to_string()),
            ));
        }

        Ok(())
    }
}

impl RecurrenceRule {
    /// Validate the recurrence rule
    pub fn validate(&self) -> Result<(), AppError> {
        if self.interval < 1 {
            return Err(AppError::validation(
                "Recurrence interval must be at least 1",
                Some("interval".to_string()),
            ));
        }

        if let Some(count) = self.end_count {
            if count < 1 {
                return Err(AppError::validation(
                    "End count must be at least 1",
                    Some("end_count".to_string()),
                ));
            }
        }

        if let Some(days) = &self.week_days {
            for day in days {
                if *day < 0 || *day > 6 {
                    return Err(AppError::validation(
                        "Week days must be between 0 (Sunday) and 6 (Saturday)",
                        Some("week_days".to_string()),
                    ));
                }
            }
        }

        if let Some(day) = self.month_day {
            if day < 1 || day > 31 {
                return Err(AppError::validation(
                    "Month day must be between 1 and 31",
                    Some("month_day".to_string()),
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_validation() {
        // Valid area
        let area = Area::new("Test Area".to_string(), Some("Description".to_string()));
        assert!(area.validate().is_ok());

        // Empty title
        let mut area = Area::new("".to_string(), None);
        assert!(area.validate().is_err());

        // Title too long
        area.title = "a".repeat(256);
        assert!(area.validate().is_err());

        // Valid hex color
        area.title = "Valid Title".to_string();
        area.color = Some("#FF0000".to_string());
        assert!(area.validate().is_ok());

        // Invalid hex color
        area.color = Some("not-a-color".to_string());
        assert!(area.validate().is_err());
    }

    #[test]
    fn test_goal_validation() {
        // Valid goal
        let goal = Goal::new("area-123".to_string(), "Test Goal".to_string(), None);
        assert!(goal.validate().is_ok());

        // Empty area_id
        let mut goal = Goal::new("".to_string(), "Test Goal".to_string(), None);
        assert!(goal.validate().is_err());

        // Invalid progress
        goal.area_id = "area-123".to_string();
        goal.progress = 150;
        assert!(goal.validate().is_err());

        // Valid progress
        goal.progress = 50;
        assert!(goal.validate().is_ok());
    }

    #[test]
    fn test_project_validation() {
        // Valid project
        let project = Project::new("goal-123".to_string(), "Test Project".to_string(), None);
        assert!(project.validate().is_ok());

        // Invalid date order
        let mut project = Project::new("goal-123".to_string(), "Test Project".to_string(), None);
        project.start_date = Some("2025-07-25".to_string());
        project.end_date = Some("2025-07-24".to_string());
        assert!(project.validate().is_err());

        // Valid date order
        project.end_date = Some("2025-07-26".to_string());
        assert!(project.validate().is_ok());
    }

    #[test]
    fn test_task_validation() {
        // Valid task
        let task = Task::new("Test Task".to_string(), None);
        assert!(task.validate().is_ok());

        // Negative estimated minutes
        let mut task = Task::new("Test Task".to_string(), None);
        task.estimated_minutes = Some(-10);
        assert!(task.validate().is_err());

        // Completed task without completion date
        task.estimated_minutes = Some(30);
        task.status = TaskStatus::Completed;
        assert!(task.validate().is_err());

        // Completed task with completion date
        task.completed_at = Some("2025-07-25T10:00:00Z".to_string());
        assert!(task.validate().is_ok());
    }

    #[test]
    fn test_recurrence_validation() {
        // Valid recurrence
        let mut recurrence = RecurrenceRule {
            frequency: RecurrenceFrequency::Daily,
            interval: 1,
            end_date: None,
            end_count: None,
            week_days: None,
            month_day: None,
            exceptions: None,
        };
        assert!(recurrence.validate().is_ok());

        // Invalid interval
        recurrence.interval = 0;
        assert!(recurrence.validate().is_err());

        // Valid week days
        recurrence.interval = 1;
        recurrence.week_days = Some(vec![0, 1, 2, 3, 4, 5, 6]);
        assert!(recurrence.validate().is_ok());

        // Invalid week day
        recurrence.week_days = Some(vec![0, 1, 7]);
        assert!(recurrence.validate().is_err());

        // Valid month day
        recurrence.week_days = None;
        recurrence.month_day = Some(15);
        assert!(recurrence.validate().is_ok());

        // Invalid month day
        recurrence.month_day = Some(32);
        assert!(recurrence.validate().is_err());
    }

    #[test]
    fn test_entity_trait() {
        let area = Area::new("Test Area".to_string(), Some("Description".to_string()));
        assert_eq!(area.title(), "Test Area");
        assert_eq!(area.description(), Some("Description"));
        assert!(area.tags().is_none());

        let goal = Goal::new("area-123".to_string(), "Test Goal".to_string(), None);
        assert_eq!(goal.title(), "Test Goal");
        assert!(goal.description().is_none());
    }
}