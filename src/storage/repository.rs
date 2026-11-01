use crate::error::TaskForgeError;
use crate::models::task::Task;
use uuid::Uuid;

/// Storage abstraction interface for task persistence.
///
/// Implementations of this trait can store tasks in files, databases (e.g. SQLite),
/// in-memory maps, or remote services.
pub trait TaskRepository: Send + Sync {
    /// Retrieve a task by its unique ID.
    fn get_task(&self, id: &Uuid) -> Result<Option<Task>, TaskForgeError>;

    /// Retrieve all tasks.
    fn get_all_tasks(&self) -> Result<Vec<Task>, TaskForgeError>;

    /// Add a new task to the repository.
    fn add_task(&mut self, task: Task) -> Result<(), TaskForgeError>;

    /// Update an existing task in the repository.
    fn update_task(&mut self, task: Task) -> Result<(), TaskForgeError>;

    /// Remove a task by its unique ID.
    fn remove_task(&mut self, id: &Uuid) -> Result<(), TaskForgeError>;

    /// Clear all tasks from the repository.
    fn clear_all(&mut self) -> Result<(), TaskForgeError>;
}
