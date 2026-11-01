use crate::cli::formatter::OutputFormatter;
use crate::cli::prompt::Prompter;
use crate::cli::Commands;
use taskforge::error::TaskForgeError;
use taskforge::storage::TaskRepository;
use taskforge::TaskManager;

/// Dispatches CLI commands to the domain service (`TaskManager`) and formats output.
pub fn execute_command<R: TaskRepository, P: Prompter>(
    command: &Commands,
    manager: &mut TaskManager<R>,
    formatter: &dyn OutputFormatter,
    prompter: &P,
) -> Result<(), TaskForgeError> {
    match command {
        Commands::Add {
            title,
            description,
            priority,
        } => {
            let task =
                manager.add_task_with_priority(title.clone(), description.clone(), *priority)?;
            formatter.format_task_created(&task);
        }
        Commands::List {
            completed,
            pending,
            priority,
        } => {
            let tasks = manager.filter_tasks(*completed, *pending, *priority);
            formatter.format_task_list(&tasks);
        }
        Commands::Show { id } => {
            let task = manager.show_task(id)?;
            formatter.format_task_detail(&task);
        }
        Commands::Complete { id } => {
            manager.complete_task(id)?;
            formatter.format_success("Task completed successfully.");
        }
        Commands::Edit {
            id,
            title,
            description,
            priority,
        } => {
            manager.edit_task_full(id, title.clone(), description.clone().map(Some), *priority)?;
            formatter.format_success("Task updated successfully.");
        }
        Commands::Delete { id } => {
            manager.delete_task(id)?;
            formatter.format_success("Task deleted successfully.");
        }
        Commands::Clear { yes } => {
            let confirmed = *yes || prompter.confirm("Are you sure you want to clear all tasks?");
            if confirmed {
                manager.clear_all()?;
                formatter.format_success("All tasks cleared successfully.");
            } else {
                formatter.format_success("Operation cancelled.");
            }
        }
    }
    Ok(())
}
