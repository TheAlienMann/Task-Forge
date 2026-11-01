pub mod formatter;

use clap::{Parser, Subcommand};
pub use formatter::{OutputFormat, TaskFormatter};

#[derive(Parser)]
#[clap(
    name = "taskforge",
    version = "0.1.0",
    about = "A modern command-line task manager"
)]
pub struct Cli {
    #[clap(long, default_value = "~/.taskforge/tasks.json")]
    pub storage_path: String,

    /// Output format (text, json, quiet)
    #[clap(
        short,
        long,
        value_enum,
        default_value_t = OutputFormat::Text,
        global = true
    )]
    pub format: OutputFormat,

    /// Output in JSON format (equivalent to --format json)
    #[clap(long, global = true)]
    pub json: bool,

    /// Quiet output: prints only task IDs (equivalent to --format quiet)
    #[clap(short, long, global = true)]
    pub quiet: bool,

    #[clap(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn output_format(&self) -> OutputFormat {
        if self.json {
            OutputFormat::Json
        } else if self.quiet {
            OutputFormat::Quiet
        } else {
            self.format
        }
    }
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
