use crate::error::TaskForgeError;
use crate::models::priority::Priority;
use crate::models::task::Task;
use crate::storage::file_storage::FileStorage;
use crate::storage::repository::TaskRepository;
use std::path::Path;
use uuid::Uuid;

pub struct TaskManager<R: TaskRepository = FileStorage> {
    repository: R,
}

impl<R: TaskRepository> TaskManager<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn add_task(
        &mut self,
        title: String,
        description: Option<String>,
    ) -> Result<Task, TaskForgeError> {
        self.add_task_with_priority(title, description, Priority::default())
    }

    pub fn add_task_with_priority(
        &mut self,
        title: String,
        description: Option<String>,
        priority: Priority,
    ) -> Result<Task, TaskForgeError> {
        let task = Task::with_priority(title, description, priority)?;
        self.repository.add_task(task.clone())?;
        Ok(task)
    }

    pub fn list_tasks(&self) -> Vec<Task> {
        self.repository.get_all_tasks().unwrap_or_default()
    }

    pub fn filter_tasks(
        &self,
        completed_only: bool,
        pending_only: bool,
        priority: Option<Priority>,
    ) -> Vec<Task> {
        self.list_tasks()
            .into_iter()
            .filter(|t| {
                if completed_only && !t.is_completed() {
                    return false;
                }
                if pending_only && t.is_completed() {
                    return false;
                }
                if let Some(p) = priority {
                    if t.priority != p {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    pub fn get_all_tasks(&self) -> Result<Vec<Task>, TaskForgeError> {
        self.repository.get_all_tasks()
    }

    pub fn show_task(&self, id: &str) -> Result<Task, TaskForgeError> {
        let task_id = Uuid::parse_str(id).map_err(|_| TaskForgeError::InvalidTaskId)?;
        self.repository
            .get_task(&task_id)?
            .ok_or(TaskForgeError::TaskNotFound)
    }

    pub fn complete_task(&mut self, id: &str) -> Result<(), TaskForgeError> {
        let task_id = Uuid::parse_str(id).map_err(|_| TaskForgeError::InvalidTaskId)?;
        let mut task = self
            .repository
            .get_task(&task_id)?
            .ok_or(TaskForgeError::TaskNotFound)?;
        task.complete();
        self.repository.update_task(task)
    }

    pub fn edit_task(
        &mut self,
        id: &str,
        title: Option<String>,
        description: Option<Option<String>>,
    ) -> Result<(), TaskForgeError> {
        self.edit_task_full(id, title, description, None)
    }

    pub fn edit_task_full(
        &mut self,
        id: &str,
        title: Option<String>,
        description: Option<Option<String>>,
        priority: Option<Priority>,
    ) -> Result<(), TaskForgeError> {
        let task_id = Uuid::parse_str(id).map_err(|_| TaskForgeError::InvalidTaskId)?;
        let mut task = self
            .repository
            .get_task(&task_id)?
            .ok_or(TaskForgeError::TaskNotFound)?;

        if let Some(t) = title {
            let trimmed = t.trim();
            if trimmed.is_empty() {
                return Err(TaskForgeError::EmptyTitle);
            }
            task.title = trimmed.to_string();
        }
        if let Some(d) = description {
            task.description = d.map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        }
        if let Some(p) = priority {
            task.priority = p;
        }
        task.updated_at = chrono::Utc::now();
        self.repository.update_task(task)
    }

    pub fn delete_task(&mut self, id: &str) -> Result<(), TaskForgeError> {
        let task_id = Uuid::parse_str(id).map_err(|_| TaskForgeError::InvalidTaskId)?;
        self.repository.remove_task(&task_id)
    }

    pub fn clear_all(&mut self) -> Result<(), TaskForgeError> {
        self.repository.clear_all()
    }

    pub fn repository(&self) -> &R {
        &self.repository
    }

    pub fn repository_mut(&mut self) -> &mut R {
        &mut self.repository
    }
}

impl TaskManager<FileStorage> {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, TaskForgeError> {
        let storage = FileStorage::load_from_file(path)?;
        Ok(Self::new(storage))
    }
}
