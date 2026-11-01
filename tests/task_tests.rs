use taskforge::models::task::Task;

#[test]
fn test_task_creation() {
    let task = Task::new("Buy milk".to_string(), Some("2% milk".to_string()));
    assert_eq!(task.get_title(), "Buy milk");
    assert_eq!(task.get_description(), Some("2% milk"));
    assert!(!task.is_completed());
    assert_eq!(task.get_id(), &task.id);
}

#[test]
fn test_task_creation_without_description() {
    let task = Task::new("Learn Rust".to_string(), None);
    assert_eq!(task.get_title(), "Learn Rust");
    assert_eq!(task.get_description(), None);
    assert!(!task.is_completed());
}

#[test]
fn test_task_completion() {
    let mut task = Task::new("Read book".to_string(), None);
    assert!(!task.is_completed());
    task.complete();
    assert!(task.is_completed());
}

#[test]
fn test_task_serialization_deserialization() {
    let task = Task::new("Serialize test".to_string(), Some("Details".to_string()));
    let json = serde_json::to_string(&task).expect("Failed to serialize task");
    let deserialized: Task = serde_json::from_str(&json).expect("Failed to deserialize task");
    assert_eq!(task, deserialized);
}
