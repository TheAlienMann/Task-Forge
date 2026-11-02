pub mod error;
pub mod manager;
pub mod models;
pub mod storage;

// Re-exports for convenient top-level access
pub use error::TaskForgeError;
pub use manager::TaskManager;
pub use models::priority::Priority;
pub use models::task::Task;
pub use storage::file_storage::{resolve_path, FileLock, FileStorage, TaskStorage};
pub use storage::mem_storage::InMemoryStorage;
pub use storage::repository::TaskRepository;

// Backward-compatibility module alias
pub mod commands {
    pub mod task_commands {
        pub use crate::manager::TaskManager;
    }
}
