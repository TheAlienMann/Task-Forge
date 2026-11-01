pub mod error;
pub mod manager;
pub mod models;
pub mod storage;

// Re-exports for convenient top-level access
pub use error::TaskForgeError;
pub use manager::TaskManager;
pub use models::task::Task;
pub use storage::file_storage::TaskStorage;

// Backward-compatibility module alias
pub mod commands {
    pub mod task_commands {
        pub use crate::manager::TaskManager;
    }
}
