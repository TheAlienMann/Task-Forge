use clap::ValueEnum;
use taskforge::Task;

/// Supported CLI output formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable text format with status icons
    Text,
    /// Structured JSON format suitable for piping (e.g. into jq)
    Json,
    /// Quiet format outputting only identifiers, suitable for shell scripting
    Quiet,
}

/// Handles formatting of task domain entities for console output.
pub struct TaskFormatter {
    format: OutputFormat,
}

impl TaskFormatter {
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }

    pub fn format_task_created(&self, task: &Task) {
        match self.format {
            OutputFormat::Text => println!(
                "Task added successfully with ID: {} {}",
                task.get_id(),
                task.priority.badge()
            ),
            OutputFormat::Json => {
                if let Ok(json) = serde_json::to_string_pretty(task) {
                    println!("{}", json);
                }
            }
            OutputFormat::Quiet => println!("{}", task.get_id()),
        }
    }

    pub fn format_task_list(&self, tasks: &[Task]) {
        match self.format {
            OutputFormat::Text => {
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
            OutputFormat::Json => {
                if let Ok(json) = serde_json::to_string_pretty(tasks) {
                    println!("{}", json);
                }
            }
            OutputFormat::Quiet => {
                for task in tasks {
                    println!("{}", task.get_id());
                }
            }
        }
    }

    pub fn format_task_detail(&self, task: &Task) {
        match self.format {
            OutputFormat::Text => {
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
            OutputFormat::Json => {
                if let Ok(json) = serde_json::to_string_pretty(task) {
                    println!("{}", json);
                }
            }
            OutputFormat::Quiet => println!("{}", task.get_id()),
        }
    }

    pub fn format_success(&self, message: &str) {
        match self.format {
            OutputFormat::Text => println!("{}", message),
            OutputFormat::Json => println!(r#"{{"status":"ok","message":"{}"}}"#, message),
            OutputFormat::Quiet => {}
        }
    }
}
