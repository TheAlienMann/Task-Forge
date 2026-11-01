use crate::error::TaskForgeError;
use crate::models::task::Task;
use crate::storage::file_storage::TaskStorage;
use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
#[clap(name = "taskforge", version = "0.1.0")]
pub struct Cli {
    #[clap(long, default_value = "~/.taskforge/tasks.json")]
    pub storage_path: String,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new task
    Add {
        /// Task title
        title: String,
        /// Task description
        #[clap(short, long)]
        description: Option<String>,
    },
    /// List all tasks
    List,
    /// Show a specific task
    Show {
        /// Task ID
        id: String,
    },
    /// Complete a task
    Complete {
        /// Task ID
        id: String,
    },
    /// Edit a task
    Edit {
        /// Task ID
        id: String,
        /// New title
        title: Option<String>,
        /// New description
        #[clap(short, long)]
        description: Option<String>,
    },
    /// Delete a task
    Delete {
        /// Task ID
        id: String,
    },
    /// Clear all tasks
    Clear,
}

pub struct TaskManager<'a> {
    storage_path: &'a str,
    storage: TaskStorage,
}

impl<'a> TaskManager<'a> {
    pub fn new(storage_path: &'a str) -> Result<Self, TaskForgeError> {
        let storage = TaskStorage::load_from_file(storage_path)?;
        Ok(Self {
            storage_path,
            storage,
        })
    }

    pub fn add_task(
        &mut self,
        title: String,
        description: Option<String>,
    ) -> Result<Task, TaskForgeError> {
        let task = Task::new(title, description);
        self.storage.add_task(task.clone());
        self.save()?;
        Ok(task)
    }

    pub fn list_tasks(&self) -> &[Task] {
        self.storage.get_all_tasks()
    }

    pub fn show_task(&self, id: &str) -> Result<Task, TaskForgeError> {
        let task_id = Uuid::parse_str(id).map_err(|_| TaskForgeError::InvalidTaskId)?;
        let task = self
            .storage
            .get_task(&task_id)
            .ok_or(TaskForgeError::TaskNotFound)?
            .clone();
        Ok(task)
    }

    pub fn complete_task(&mut self, id: &str) -> Result<(), TaskForgeError> {
        let task_id = Uuid::parse_str(id).map_err(|_| TaskForgeError::InvalidTaskId)?;
        self.storage.complete_task(&task_id)?;
        self.save()?;
        Ok(())
    }

    pub fn edit_task(
        &mut self,
        id: &str,
        title: Option<String>,
        description: Option<Option<String>>,
    ) -> Result<(), TaskForgeError> {
        let task_id = Uuid::parse_str(id).map_err(|_| TaskForgeError::InvalidTaskId)?;
        let task = self
            .storage
            .get_task_mut(&task_id)
            .ok_or(TaskForgeError::TaskNotFound)?;

        if let Some(t) = title {
            task.title = t;
        }
        if let Some(d) = description {
            task.description = d;
        }
        self.save()?;
        Ok(())
    }

    pub fn delete_task(&mut self, id: &str) -> Result<(), TaskForgeError> {
        let task_id = Uuid::parse_str(id).map_err(|_| TaskForgeError::InvalidTaskId)?;
        self.storage.remove_task(&task_id)?;
        self.save()?;
        Ok(())
    }

    pub fn clear_all(&mut self) -> Result<(), TaskForgeError> {
        self.storage.clear_all();
        self.save()?;
        Ok(())
    }

    fn save(&self) -> Result<(), TaskForgeError> {
        self.storage.save_to_file(self.storage_path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn get_temp_storage_path() -> (std::path::PathBuf, String) {
        let temp_dir = std::env::temp_dir().join(format!("taskforge_tm_test_{}", Uuid::new_v4()));
        let file_path = temp_dir.join("tasks.json");
        let path_str = file_path.to_str().unwrap().to_string();
        (temp_dir, path_str)
    }

    #[test]
    fn test_task_manager_add_and_list() {
        let (temp_dir, path) = get_temp_storage_path();
        {
            let mut manager = TaskManager::new(&path).expect("Failed to create manager");
            assert!(manager.list_tasks().is_empty());

            let task = manager
                .add_task("Test task".to_string(), Some("Desc".to_string()))
                .expect("Failed to add task");
            assert_eq!(task.get_title(), "Test task");
            assert_eq!(manager.list_tasks().len(), 1);
        }
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_task_manager_show_task() {
        let (temp_dir, path) = get_temp_storage_path();
        {
            let mut manager = TaskManager::new(&path).expect("Failed to create manager");
            let task = manager
                .add_task("Show me".to_string(), None)
                .expect("Failed to add task");
            let id_str = task.get_id().to_string();

            let shown = manager.show_task(&id_str).expect("Failed to show task");
            assert_eq!(shown.get_title(), "Show me");

            // Invalid ID
            assert!(matches!(
                manager.show_task("invalid-uuid"),
                Err(TaskForgeError::InvalidTaskId)
            ));

            // Non-existent ID
            let random_uuid = Uuid::new_v4().to_string();
            assert!(matches!(
                manager.show_task(&random_uuid),
                Err(TaskForgeError::TaskNotFound)
            ));
        }
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_task_manager_complete_task() {
        let (temp_dir, path) = get_temp_storage_path();
        {
            let mut manager = TaskManager::new(&path).expect("Failed to create manager");
            let task = manager
                .add_task("Complete me".to_string(), None)
                .expect("Failed to add task");
            let id_str = task.get_id().to_string();

            manager
                .complete_task(&id_str)
                .expect("Failed to complete task");
            let shown = manager.show_task(&id_str).expect("Failed to show task");
            assert!(shown.is_completed());

            // Invalid ID
            assert!(matches!(
                manager.complete_task("bad-id"),
                Err(TaskForgeError::InvalidTaskId)
            ));

            // Not found
            let random_uuid = Uuid::new_v4().to_string();
            assert!(matches!(
                manager.complete_task(&random_uuid),
                Err(TaskForgeError::TaskNotFound)
            ));
        }
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_task_manager_edit_task() {
        let (temp_dir, path) = get_temp_storage_path();
        {
            let mut manager = TaskManager::new(&path).expect("Failed to create manager");
            let task = manager
                .add_task("Old title".to_string(), Some("Old desc".to_string()))
                .expect("Failed to add task");
            let id_str = task.get_id().to_string();

            // Edit title only
            manager
                .edit_task(&id_str, Some("New title".to_string()), None)
                .expect("Failed to edit title");
            let shown = manager.show_task(&id_str).unwrap();
            assert_eq!(shown.get_title(), "New title");
            assert_eq!(shown.get_description(), Some("Old desc"));

            // Edit description only
            manager
                .edit_task(&id_str, None, Some(Some("New desc".to_string())))
                .expect("Failed to edit desc");
            let shown = manager.show_task(&id_str).unwrap();
            assert_eq!(shown.get_description(), Some("New desc"));

            // Clear description
            manager
                .edit_task(&id_str, None, Some(None))
                .expect("Failed to clear desc");
            let shown = manager.show_task(&id_str).unwrap();
            assert_eq!(shown.get_description(), None);

            // Invalid ID
            assert!(matches!(
                manager.edit_task("bad", Some("T".to_string()), None),
                Err(TaskForgeError::InvalidTaskId)
            ));

            // Not found
            let random_uuid = Uuid::new_v4().to_string();
            assert!(matches!(
                manager.edit_task(&random_uuid, Some("T".to_string()), None),
                Err(TaskForgeError::TaskNotFound)
            ));
        }
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_task_manager_delete_and_clear() {
        let (temp_dir, path) = get_temp_storage_path();
        {
            let mut manager = TaskManager::new(&path).expect("Failed to create manager");
            let task1 = manager
                .add_task("Task 1".to_string(), None)
                .expect("Failed to add task");
            let _task2 = manager
                .add_task("Task 2".to_string(), None)
                .expect("Failed to add task");

            assert_eq!(manager.list_tasks().len(), 2);

            manager
                .delete_task(&task1.get_id().to_string())
                .expect("Failed to delete task");
            assert_eq!(manager.list_tasks().len(), 1);

            manager.clear_all().expect("Failed to clear tasks");
            assert!(manager.list_tasks().is_empty());

            // Delete not found
            let random_uuid = Uuid::new_v4().to_string();
            assert!(matches!(
                manager.delete_task(&random_uuid),
                Err(TaskForgeError::TaskNotFound)
            ));

            // Delete invalid ID
            assert!(matches!(
                manager.delete_task("invalid"),
                Err(TaskForgeError::InvalidTaskId)
            ));
        }
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
