use crate::error::TaskForgeError;
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
        let task = Task::new(title, description);
        self.repository.add_task(task.clone())?;
        Ok(task)
    }

    pub fn list_tasks(&self) -> Vec<Task> {
        self.repository.get_all_tasks().unwrap_or_default()
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
        let task_id = Uuid::parse_str(id).map_err(|_| TaskForgeError::InvalidTaskId)?;
        let mut task = self
            .repository
            .get_task(&task_id)?
            .ok_or(TaskForgeError::TaskNotFound)?;

        if let Some(t) = title {
            task.title = t;
        }
        if let Some(d) = description {
            task.description = d;
        }
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
