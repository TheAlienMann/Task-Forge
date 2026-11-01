# TaskForge

A modern, modular task manager built in Rust with JSON persistence, decoupled library architecture, and rich CLI features.

## Features

- **Decoupled Architecture**: Pure Rust core library (`libtaskforge`) separated from the CLI binary (`taskforge`)
- **Task Management**: Add, list, show, complete, edit, delete, and clear tasks
- **Persistent Storage**: Tasks stored in JSON format with configurable path
- **UUID-based Task IDs**: Unique identifiers for all tasks
- **Rich CLI Interface**: Intuitive command-line interface with helpful error messages
- **Error Handling**: Comprehensive error handling with custom error types
- **Library API**: Can be embedded directly as a dependency in web servers, desktop apps, or other tools

## Installation

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd taskforge

# Build the project (builds both library and binary)
cargo build --release

# The binary will be available at target/release/taskforge
```

## Usage

### CLI Commands

```bash
# Add a new task
taskforge add "My Task Title" --description "Task description here"

# List all tasks
taskforge list

# Show a specific task
taskforge show <task-id>

# Complete a task
taskforge complete <task-id>

# Edit a task
taskforge edit <task-id> --title "Updated Title" --description "Updated Description"

# Delete a task
taskforge delete <task-id>

# Clear all tasks
taskforge clear

# Set custom storage path (default: ~/.taskforge/tasks.json)
taskforge --storage-path /path/to/tasks.json add "My Task"
```

### Using TaskForge as a Library

Because `taskforge` is designed with a decoupled architecture, you can use it directly in your own Rust applications without CLI overhead:

```rust
use taskforge::{TaskManager, Task, TaskForgeError};

fn main() -> Result<(), TaskForgeError> {
    // Initialize task manager with a JSON storage path
    let mut manager = TaskManager::new("tasks.json")?;

    // Add a new task
    let task = manager.add_task("Implement feature".into(), Some("Details here".into()))?;
    println!("Created task with ID: {}", task.get_id());

    // List all tasks
    for t in manager.list_tasks() {
        println!("{}: {}", t.get_id(), t.get_title());
    }

    // Complete the task
    manager.complete_task(&task.get_id().to_string())?;

    Ok(())
}
```

## Architecture & Project Structure

The project strictly separates domain logic and persistence (library) from user-interface and argument parsing (binary):

```text
.
├── Cargo.toml
├── src/
│   ├── lib.rs               # Library root & public re-exports (TaskManager, Task, etc.)
│   ├── main.rs              # Thin binary entry point (CLI application)
│   ├── cli.rs               # Clap CLI arguments, options, and subcommand definitions
│   ├── manager.rs           # TaskManager service coordinating business logic and storage
│   ├── error/               # Custom error types (TaskForgeError)
│   │   └── mod.rs
│   ├── models/              # Core domain models (Task)
│   │   ├── mod.rs
│   │   └── task.rs
│   └── storage/             # Persistence layer (TaskStorage)
│       ├── mod.rs
│       └── file_storage.rs
└── tests/                   # Independent integration test suites
    ├── task_tests.rs        # Task model unit and serialization tests
    ├── storage_tests.rs     # Storage persistence, loading, and mutation tests
    └── manager_tests.rs     # TaskManager workflow and error handling tests
```

### Layers:
1. **Domain Layer (`models`)**: `Task` entity with invariants, completion states, and serde support.
2. **Persistence Layer (`storage`)**: `TaskStorage` managing JSON serialization, buffer flushing, and file persistence.
3. **Application Service (`manager`)**: `TaskManager` providing an programmatic API for operations.
4. **Presentation Adapter (`cli` & `main.rs`)**: Clap parser handling command-line flags and console formatting.

## Development

### Running Tests

All unit and integration tests live in the `tests/` directory:

```bash
# Run all integration test suites
cargo test

# Run tests with coverage
cargo tarpaulin --verbose --workspace
```

### Code Quality

```bash
# Check formatting
cargo fmt --check

# Run linter
cargo clippy --all-targets --all-features

# Build project
cargo build
```

## Error Handling

The application handles various error conditions gracefully:

- Invalid task IDs (UUID parsing errors)
- File I/O errors during storage operations
- Task not found errors
- Serialization/deserialization errors
- Empty or malformed JSON files

## License

This project is licensed under the MIT License - see the LICENSE file for details.
