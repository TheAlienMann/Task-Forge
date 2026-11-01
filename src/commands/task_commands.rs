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
