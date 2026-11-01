use taskforge::error::TaskForgeError;
use taskforge::models::priority::Priority;
use taskforge::models::task::Task;

#[test]
fn test_task_creation() {
    let task = Task::new("Buy milk".to_string(), Some("2% milk".to_string()))
        .expect("Task creation failed");
    assert_eq!(task.get_title(), "Buy milk");
    assert_eq!(task.get_description(), Some("2% milk"));
    assert!(!task.is_completed());
    assert_eq!(task.get_priority(), Priority::Medium);
    assert_eq!(task.get_id(), &task.id);
}

#[test]
fn test_task_creation_without_description() {
    let task = Task::new("Learn Rust".to_string(), None).expect("Task creation failed");
    assert_eq!(task.get_title(), "Learn Rust");
    assert_eq!(task.get_description(), None);
    assert!(!task.is_completed());
}

#[test]
fn test_task_empty_title_validation() {
    assert!(matches!(
        Task::new("".to_string(), None),
        Err(TaskForgeError::EmptyTitle)
    ));

    assert!(matches!(
        Task::new("   \t\n  ".to_string(), None),
        Err(TaskForgeError::EmptyTitle)
    ));
}

#[test]
fn test_task_with_priority() {
    let task = Task::with_priority(
        "Critical bugfix".to_string(),
        Some("Fix prod issue".to_string()),
        Priority::Urgent,
    )
    .expect("Task creation failed");

    assert_eq!(task.get_priority(), Priority::Urgent);
    assert_eq!(task.priority.badge(), "[URGENT]");
}

#[test]
fn test_task_completion() {
    let mut task = Task::new("Read book".to_string(), None).expect("Task creation failed");
    assert!(!task.is_completed());
    task.complete();
    assert!(task.is_completed());
}

#[test]
fn test_task_serialization_deserialization() {
    let task = Task::with_priority(
        "Serialize test".to_string(),
        Some("Details".to_string()),
        Priority::High,
    )
    .expect("Task creation failed");

    let json = serde_json::to_string(&task).expect("Failed to serialize task");
    let deserialized: Task = serde_json::from_str(&json).expect("Failed to deserialize task");
    assert_eq!(task, deserialized);
    assert_eq!(deserialized.priority, Priority::High);
}
