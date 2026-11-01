use crate::error::TaskForgeError;
use crate::models::task::Task;
use crate::storage::repository::TaskRepository;
use uuid::Uuid;

/// In-memory implementation of `TaskRepository`.
///
/// Ideal for testing, temporary workspaces, or environments without disk I/O.
#[derive(Debug, Default, Clone)]
pub struct InMemoryStorage {
    tasks: Vec<Task>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

impl TaskRepository for InMemoryStorage {
    fn get_task(&self, id: &Uuid) -> Result<Option<Task>, TaskForgeError> {
        Ok(self.tasks.iter().find(|t| t.get_id() == id).cloned())
    }

    fn get_all_tasks(&self) -> Result<Vec<Task>, TaskForgeError> {
        Ok(self.tasks.clone())
    }

    fn add_task(&mut self, task: Task) -> Result<(), TaskForgeError> {
        self.tasks.push(task);
        Ok(())
    }

    fn update_task(&mut self, task: Task) -> Result<(), TaskForgeError> {
        let index = self
            .tasks
            .iter()
            .position(|t| t.get_id() == task.get_id())
            .ok_or(TaskForgeError::TaskNotFound)?;
        self.tasks[index] = task;
        Ok(())
    }

    fn remove_task(&mut self, id: &Uuid) -> Result<(), TaskForgeError> {
        let index = self
            .tasks
            .iter()
            .position(|t| t.get_id() == id)
            .ok_or(TaskForgeError::TaskNotFound)?;
        self.tasks.remove(index);
        Ok(())
    }

    fn clear_all(&mut self) -> Result<(), TaskForgeError> {
        self.tasks.clear();
        Ok(())
    }
}
