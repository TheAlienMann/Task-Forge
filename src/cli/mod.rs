pub mod formatter;
pub mod handlers;
pub mod prompt;

use clap::{Parser, Subcommand};
pub use formatter::{create_formatter, OutputFormat};
pub use handlers::execute_command;
pub use prompt::ConsolePrompter;
use taskforge::error::TaskForgeError;
use taskforge::{Priority, TaskManager};

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
        /// Task priority (low, medium, high, urgent)
        #[clap(short, long, value_enum, default_value_t = Priority::Medium)]
        priority: Priority,
    },
    /// List tasks with optional filtering
    List {
        /// Show only completed tasks
        #[clap(long, conflicts_with = "pending")]
        completed: bool,
        /// Show only pending tasks
        #[clap(long, conflicts_with = "completed")]
        pending: bool,
        /// Filter by priority (low, medium, high, urgent)
        #[clap(short, long, value_enum)]
        priority: Option<Priority>,
    },
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
        /// New priority
        #[clap(short, long, value_enum)]
        priority: Option<Priority>,
    },
    /// Delete a task
    Delete {
        /// Task ID
        id: String,
    },
    /// Clear all tasks
    Clear {
        /// Skip confirmation prompt and clear immediately
        #[clap(short, long)]
        yes: bool,
    },
}

/// Runs the CLI application, resolving storage and executing the command.
pub fn run(cli: Cli) -> Result<(), TaskForgeError> {
    let formatter = create_formatter(cli.output_format());
    let prompter = ConsolePrompter;

    let mut manager = TaskManager::from_file(&cli.storage_path)?;
    execute_command(&cli.command, &mut manager, formatter.as_ref(), &prompter)
}
