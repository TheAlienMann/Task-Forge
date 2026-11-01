# TaskForge

A modern, modular command-line task manager built in Rust with JSON persistence and rich CLI features.

## Features

- **Task Management**: Add, list, show, complete, delete, and clear tasks
- **Persistent Storage**: Tasks stored in JSON format with configurable path
- **UUID-based Task IDs**: Unique identifiers for all tasks
- **Rich CLI Interface**: Intuitive command-line interface with helpful error messages
- **Error Handling**: Comprehensive error handling with custom error types
- **Modular Architecture**: Well-organized codebase following Rust best practices

## Installation

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd taskforge

# Build the project
cargo build --release

# The binary will be available at target/release/taskforge
```

## Usage

### Basic Commands

```bash
# Add a new task
taskforge add "My Task Title" --description "Task description here"

# List all tasks
taskforge list

# Show a specific task
taskforge show <task-id>

# Complete a task
taskforge complete <task-id>

# Delete a task
taskforge delete <task-id>

# Clear all tasks
taskforge clear

# Set custom storage path (default: ~/.taskforge/tasks.json)
taskforge --storage-path /path/to/tasks.json add "My Task"
```

### Examples

```bash
# Add tasks with descriptions
taskforge add "Buy groceries" --description "Milk, eggs, bread"
taskforge add "Write documentation"

# List all tasks
taskforge list

# Complete a task
taskforge complete 550e8400-e29b-41d4-a716-446655440000

# Show specific task details
taskforge show 550e8400-e29b-41d4-a716-446655440000

# Delete a task
taskforge delete 550e8400-e29b-41d4-a716-446655440000

# Clear all tasks
taskforge clear
```

## Configuration

### Storage Path

By default, tasks are stored in `~/.taskforge/tasks.json`. You can specify a custom storage path using the `--storage-path` option:

```bash
taskforge --storage-path /custom/path/tasks.json add "My Task"
```

## Project Structure

```
src/
├── main.rs              # Entry point with CLI setup
├── error/               # Custom error types
│   └── mod.rs
├── models/              # Data models
│   └── task.rs
├── storage/             # Storage module
│   ├── file_storage.rs
│   └── mod.rs
└── commands/            # Command handling
    └── task_commands.rs
```

## Development

### Running Tests

```bash
# Run all tests
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