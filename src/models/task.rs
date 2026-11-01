use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(title: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            completed: false,
            created_at: Utc::now(),
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new("Buy milk".to_string(), Some("2% milk".to_string()));
        assert_eq!(task.get_title(), "Buy milk");
        assert_eq!(task.get_description(), Some("2% milk"));
        assert!(!task.is_completed());
        assert_eq!(task.get_id(), &task.id);
    }

    #[test]
    fn test_task_creation_without_description() {
        let task = Task::new("Learn Rust".to_string(), None);
        assert_eq!(task.get_title(), "Learn Rust");
        assert_eq!(task.get_description(), None);
        assert!(!task.is_completed());
    }

    #[test]
    fn test_task_completion() {
        let mut task = Task::new("Read book".to_string(), None);
        assert!(!task.is_completed());
        task.complete();
        assert!(task.is_completed());
    }

    #[test]
    fn test_task_serialization_deserialization() {
        let task = Task::new("Serialize test".to_string(), Some("Details".to_string()));
        let json = serde_json::to_string(&task).expect("Failed to serialize task");
        let deserialized: Task = serde_json::from_str(&json).expect("Failed to deserialize task");
        assert_eq!(task, deserialized);
    }
}
