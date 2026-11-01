mod cli;

use clap::Parser;
use cli::{Cli, Commands, TaskFormatter};
use std::process;
use taskforge::TaskManager;

fn main() {
    let cli = Cli::parse();
    let formatter = TaskFormatter::new(cli.output_format());

    let mut task_manager = match TaskManager::from_file(&cli.storage_path) {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Error initializing task manager: {}", e);
            process::exit(1);
        }
    };

    match &cli.command {
        Commands::Add {
            title,
            description,
            priority,
        } => {
            match task_manager.add_task_with_priority(title.clone(), description.clone(), *priority)
            {
                Ok(task) => formatter.format_task_created(&task),
                Err(e) => {
                    eprintln!("Error adding task: {}", e);
                    process::exit(1);
                }
            }
        }
        Commands::List {
            completed,
            pending,
            priority,
        } => {
            let tasks = task_manager.filter_tasks(*completed, *pending, *priority);
            formatter.format_task_list(&tasks);
        }
        Commands::Show { id } => match task_manager.show_task(id) {
            Ok(task) => formatter.format_task_detail(&task),
            Err(e) => {
                eprintln!("Error showing task: {}", e);
                process::exit(1);
            }
        },
        Commands::Complete { id } => match task_manager.complete_task(id) {
            Ok(()) => formatter.format_success("Task completed successfully."),
            Err(e) => {
                eprintln!("Error completing task: {}", e);
                process::exit(1);
            }
        },
        Commands::Edit {
            id,
            title,
            description,
            priority,
        } => {
            match task_manager.edit_task_full(
                id,
                title.clone(),
                description.clone().map(Some),
                *priority,
            ) {
                Ok(()) => formatter.format_success("Task updated successfully."),
                Err(e) => {
                    eprintln!("Error updating task: {}", e);
                    process::exit(1);
                }
            }
        }
        Commands::Delete { id } => match task_manager.delete_task(id) {
            Ok(()) => formatter.format_success("Task deleted successfully."),
            Err(e) => {
                eprintln!("Error deleting task: {}", e);
                process::exit(1);
            }
        },
        Commands::Clear => {
            print!("Are you sure you want to clear all tasks? (y/N): ");
            use std::io::{self, Write};
            let mut input = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() == "y" {
                match task_manager.clear_all() {
                    Ok(()) => formatter.format_success("All tasks cleared successfully."),
                    Err(e) => eprintln!("Error clearing tasks: {}", e),
                }
            } else {
                formatter.format_success("Operation cancelled.");
            }
        }
    }
}
