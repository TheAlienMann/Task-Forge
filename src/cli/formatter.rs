use clap::ValueEnum;
use taskforge::error::TaskForgeError;
use taskforge::Task;

/// Supported CLI output formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default)]
pub enum OutputFormat {
    /// Human-readable text format with status icons and badges
    #[default]
    Text,
    /// Structured JSON format suitable for piping (e.g. into jq)
    Json,
    /// Quiet format outputting only identifiers, suitable for shell scripting
    Quiet,
}

/// Interface for rendering task domain events and data to the console.
pub trait OutputFormatter {
    fn format_task_created(&self, task: &Task);
    fn format_task_list(&self, tasks: &[Task]);
    fn format_task_detail(&self, task: &Task);
    fn format_success(&self, message: &str);
    fn format_error(&self, error: &TaskForgeError);
}

pub struct TextFormatter;

impl OutputFormatter for TextFormatter {
    fn format_task_created(&self, task: &Task) {
        println!(
            "Task added successfully with ID: {} {}",
            task.get_id(),
            task.priority.badge()
        );
    }

    fn format_task_list(&self, tasks: &[Task]) {
        if tasks.is_empty() {
            println!("No tasks found.");
        } else {
            for task in tasks {
                let status = if task.is_completed() { "✓" } else { "○" };
                println!(
                    "{} {} {} - {}{}",
                    status,
                    task.priority.badge(),
                    task.get_id(),
                    task.get_title(),
                    task.get_description()
                        .map_or(String::new(), |desc| { format!(" ({})", desc) })
                );
            }
        }
    }

    fn format_task_detail(&self, task: &Task) {
        let status = if task.is_completed() {
            "Completed"
        } else {
            "Pending"
        };
        println!("ID: {}", task.get_id());
        println!("Title: {}", task.get_title());
        println!("Description: {}", task.get_description().unwrap_or("None"));
        println!("Priority: {}", task.priority);
        println!("Status: {}", status);
        println!("Created at: {}", task.created_at);
        println!("Updated at: {}", task.updated_at);
    }

    fn format_success(&self, message: &str) {
        println!("{}", message);
    }

    fn format_error(&self, error: &TaskForgeError) {
        eprintln!("Error: {}", error);
    }
}

pub struct JsonFormatter;

impl OutputFormatter for JsonFormatter {
    fn format_task_created(&self, task: &Task) {
        if let Ok(json) = serde_json::to_string_pretty(task) {
            println!("{}", json);
        }
    }

    fn format_task_list(&self, tasks: &[Task]) {
        if let Ok(json) = serde_json::to_string_pretty(tasks) {
            println!("{}", json);
        }
    }

    fn format_task_detail(&self, task: &Task) {
        if let Ok(json) = serde_json::to_string_pretty(task) {
            println!("{}", json);
        }
    }

    fn format_success(&self, message: &str) {
        println!(r#"{{"status":"ok","message":"{}"}}"#, message);
    }

    fn format_error(&self, error: &TaskForgeError) {
        eprintln!(r#"{{"status":"error","message":"{}"}}"#, error);
    }
}

pub struct QuietFormatter;

impl OutputFormatter for QuietFormatter {
    fn format_task_created(&self, task: &Task) {
        println!("{}", task.get_id());
    }

    fn format_task_list(&self, tasks: &[Task]) {
        for task in tasks {
            println!("{}", task.get_id());
        }
    }

    fn format_task_detail(&self, task: &Task) {
        println!("{}", task.get_id());
    }

    fn format_success(&self, _message: &str) {}

    fn format_error(&self, error: &TaskForgeError) {
        eprintln!("Error: {}", error);
    }
}

/// Factory function to instantiate the appropriate formatter for an `OutputFormat`.
pub fn create_formatter(format: OutputFormat) -> Box<dyn OutputFormatter> {
    match format {
        OutputFormat::Text => Box::new(TextFormatter),
        OutputFormat::Json => Box::new(JsonFormatter),
        OutputFormat::Quiet => Box::new(QuietFormatter),
    }
}
