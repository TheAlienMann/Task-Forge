# TaskForge

A modern, modular task manager built in Rust with JSON persistence, decoupled library architecture, dependency inversion, resilient atomic writes, and rich CLI features.

## Features

- **Decoupled Architecture**: Pure Rust core library (`libtaskforge`) separated from the CLI binary (`taskforge`)
- **Dependency Inversion**: Domain service (`TaskManager`) depends on an abstract `TaskRepository` trait rather than concrete file storage
- **Pluggable Storage**: Built-in support for `FileStorage` (JSON persistence) and `InMemoryStorage` (fast, zero-I/O testing)
- **Multiple Output Formats**: Human-readable text with status icons, structured JSON (`--json`), and quiet ID-only mode (`-q` / `--quiet`)
- **Resilient Persistence**: Atomic file writes (`tempfile` + `sync_all` + `rename`) guarantee zero data corruption or partial file loss during crashes
- **Safe Path Resolution**: Automatic `~` and `~/` home directory path expansion avoiding literal directory pollution
- **Lifetime-Free Service**: `TaskManager` owns its repository without borrowed lifetime contagion
- **Task Management**: Add, list, show, complete, edit, delete, and clear tasks
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

# Set custom storage path (default: ~/.taskforge/tasks.json, automatically expanded to your home directory)
taskforge --storage-path /path/to/tasks.json add "My Task"
```

### Scripting & Output Formats

TaskForge supports multiple output formats for easy scripting and terminal piping:

```bash
# JSON Output (useful for piping into jq or other tools)
taskforge list --json
taskforge show <task-id> --json
taskforge add "New Task" --json

# Quiet Mode (outputs only UUIDs, ideal for shell script loops)
taskforge list -q
taskforge add "New Task" -q

# Format option flag
taskforge --format json list
taskforge --format quiet list
```

### Using TaskForge as a Library

Because `taskforge` is designed with dependency inversion, you can choose between persistent file storage or in-memory storage, or even implement your own `TaskRepository`:

#### File-backed Task Manager:
```rust
use taskforge::{TaskManager, Task, TaskForgeError};

fn main() -> Result<(), TaskForgeError> {
    // Initialize task manager backed by a JSON file (supports atomic writes and '~' expansion)
    let mut manager = TaskManager::from_file("tasks.json")?;

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

#### In-Memory Task Manager (e.g. for testing or ephemeral workloads):
```rust
use taskforge::{TaskManager, InMemoryStorage, TaskForgeError};

fn main() -> Result<(), TaskForgeError> {
    // Zero disk I/O, fast in-memory repository
    let mut manager = TaskManager::new(InMemoryStorage::new());

    let task = manager.add_task("Quick test".into(), None)?;
    assert_eq!(manager.list_tasks().len(), 1);

    Ok(())
}
```

## Architecture & Project Structure

The project strictly follows clean separation of concerns and the Dependency Inversion Principle (DIP):

```text
.
├── Cargo.toml
├── src/
│   ├── lib.rs               # Library root & public re-exports (TaskManager, TaskRepository, etc.)
│   ├── main.rs              # Thin binary entry point (wiring CLI to library services)
│   ├── cli/                 # CLI presentation & argument handling
│   │   ├── mod.rs           # Clap CLI arguments, options, and subcommand definitions
│   │   └── formatter.rs     # Output formatters (Text, JSON, Quiet)
│   ├── manager.rs           # Generic TaskManager<R: TaskRepository> business service
│   ├── error/               # Custom error types (TaskForgeError)
│   │   └── mod.rs
│   ├── models/              # Core domain models (Task)
│   │   ├── mod.rs
│   │   └── task.rs
│   └── storage/             # Persistence layer & abstractions
│       ├── mod.rs           # Module re-exports
│       ├── repository.rs    # TaskRepository trait definition
│       ├── file_storage.rs  # FileStorage implementation (atomic writes & path resolution)
│       └── mem_storage.rs   # InMemoryStorage implementation (ephemeral / testing)
└── tests/                   # Independent integration test suites
    ├── task_tests.rs        # Task model unit and serialization tests
    ├── storage_tests.rs     # Storage persistence, atomic writes, and mutation tests
    └── manager_tests.rs     # TaskManager workflow with both File and InMemory repositories
```

### Layers:
1. **Domain Layer (`models`)**: `Task` entity with invariants, completion states, and serde support.
2. **Repository Abstraction (`storage::repository`)**: `TaskRepository` trait defining standard persistence contracts (`get_task`, `get_all_tasks`, `add_task`, `update_task`, `remove_task`, `clear_all`).
3. **Storage Implementations (`storage::file_storage`, `storage::mem_storage`)**: Pluggable adapters implementing `TaskRepository`. Includes crash-resilient atomic writes and path resolution.
4. **Application Service (`manager`)**: Generic `TaskManager<R: TaskRepository>` providing high-level task coordination without coupling to storage details.
5. **Presentation Adapter (`cli/`)**: Clap parser handling command-line flags and `TaskFormatter` handling multi-format output (Text, JSON, Quiet).

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
