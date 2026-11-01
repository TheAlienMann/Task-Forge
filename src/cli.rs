use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "taskforge",
    version = "0.1.0",
    about = "A modern command-line task manager"
)]
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
