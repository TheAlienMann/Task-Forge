use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskForgeError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Task not found")]
    TaskNotFound,

    #[error("Invalid task ID")]
    InvalidTaskId,

    #[allow(dead_code)]
    #[error("Storage error: {0}")]
    StorageError(String),
}
