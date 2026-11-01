use crate::error::TaskForgeError;
use crate::models::priority::Priority;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    #[serde(default)]
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

impl Task {
    /// Creates a new task with validation and default Priority::Medium.
    pub fn new(title: String, description: Option<String>) -> Result<Self, TaskForgeError> {
        Self::with_priority(title, description, Priority::default())
    }

    /// Creates a new task with a specific priority and validation.
    pub fn with_priority(
        title: String,
        description: Option<String>,
        priority: Priority,
    ) -> Result<Self, TaskForgeError> {
        let trimmed_title = title.trim();
        if trimmed_title.is_empty() {
            return Err(TaskForgeError::EmptyTitle);
        }

        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            title: trimmed_title.to_string(),
            description: description
                .map(|d| d.trim().to_string())
                .filter(|d| !d.is_empty()),
            completed: false,
            priority,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn complete(&mut self) {
        self.completed = true;
        self.updated_at = Utc::now();
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

    pub fn get_priority(&self) -> Priority {
        self.priority
    }
}
