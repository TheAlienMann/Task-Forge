mod cli;

use clap::Parser;
use cli::{Cli, Commands};
use std::process;
use taskforge::TaskManager;

fn main() {
    let cli = Cli::parse();

    let mut task_manager = match TaskManager::from_file(&cli.storage_path) {
        Ok(manager) => manager,
        Err(e) => {
            eprintln!("Error initializing task manager: {}", e);
            process::exit(1);
        }
    };

    match &cli.command {
        Commands::Add { title, description } => {
            match task_manager.add_task(title.clone(), description.clone()) {
                Ok(task) => {
                    println!("Task added successfully with ID: {}", task.get_id());
                }
                Err(e) => {
                    eprintln!("Error adding task: {}", e);
                    process::exit(1);
                }
            }
        }
        Commands::List => {
            let tasks = task_manager.list_tasks();
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                for task in tasks {
                    let status = if task.is_completed() { "✓" } else { "○" };
                    println!(
                        "{} {} - {}{}",
                        status,
                        task.get_id(),
                        task.get_title(),
                        task.get_description()
                            .map_or(String::new(), |desc| { format!(" ({})", desc) })
                    );
                }
            }
        }
        Commands::Show { id } => match task_manager.show_task(id) {
            Ok(task) => {
                let status = if task.is_completed() {
                    "Completed"
                } else {
                    "Pending"
                };
                println!("ID: {}", task.get_id());
                println!("Title: {}", task.get_title());
                println!("Description: {}", task.get_description().unwrap_or("None"));
                println!("Status: {}", status);
                println!("Created at: {}", task.created_at);
            }
            Err(e) => {
                eprintln!("Error showing task: {}", e);
                process::exit(1);
            }
        },
        Commands::Complete { id } => match task_manager.complete_task(id) {
            Ok(()) => {
                println!("Task completed successfully.");
            }
            Err(e) => {
                eprintln!("Error completing task: {}", e);
                process::exit(1);
            }
        },
        Commands::Edit {
            id,
            title,
            description,
        } => match task_manager.edit_task(id, title.clone(), description.clone().map(Some)) {
            Ok(()) => {
                println!("Task updated successfully.");
            }
            Err(e) => {
                eprintln!("Error updating task: {}", e);
                process::exit(1);
            }
        },
        Commands::Delete { id } => match task_manager.delete_task(id) {
            Ok(()) => {
                println!("Task deleted successfully.");
            }
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
                    Ok(()) => println!("All tasks cleared successfully."),
                    Err(e) => eprintln!("Error clearing tasks: {}", e),
                }
            } else {
                println!("Operation cancelled.");
            }
        }
    }
}
