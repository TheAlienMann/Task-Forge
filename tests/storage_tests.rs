use std::fs::{self, File};
use taskforge::error::TaskForgeError;
use taskforge::models::task::Task;
use taskforge::storage::file_storage::TaskStorage;

#[test]
fn test_storage_new_and_empty() {
    let storage = TaskStorage::new();
    assert!(storage.is_empty());
    assert_eq!(storage.len(), 0);
    assert_eq!(storage.get_all_tasks().len(), 0);
}

#[test]
fn test_add_and_get_task() {
    let mut storage = TaskStorage::new();
    let task = Task::new("Test task".to_string(), None);
    let id = *task.get_id();

    storage.add_task(task);
    assert_eq!(storage.len(), 1);
    assert!(!storage.is_empty());

    let retrieved = storage.get_task(&id);
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().get_title(), "Test task");
}

#[test]
fn test_get_task_mut() {
    let mut storage = TaskStorage::new();
    let task = Task::new("Original title".to_string(), None);
    let id = *task.get_id();

    storage.add_task(task);
    let task_mut = storage.get_task_mut(&id).unwrap();
    task_mut.title = "Updated title".to_string();

    assert_eq!(storage.get_task(&id).unwrap().get_title(), "Updated title");
}

#[test]
fn test_complete_task() {
    let mut storage = TaskStorage::new();
    let task = Task::new("Complete me".to_string(), None);
    let id = *task.get_id();

    storage.add_task(task);
    assert!(!storage.get_task(&id).unwrap().is_completed());

    storage.complete_task(&id).expect("Failed to complete task");
    assert!(storage.get_task(&id).unwrap().is_completed());
}

#[test]
fn test_complete_non_existent_task() {
    let mut storage = TaskStorage::new();
    let random_id = uuid::Uuid::new_v4();
    let result = storage.complete_task(&random_id);
    assert!(matches!(result, Err(TaskForgeError::TaskNotFound)));
}

#[test]
fn test_remove_task() {
    let mut storage = TaskStorage::new();
    let task = Task::new("Remove me".to_string(), None);
    let id = *task.get_id();

    storage.add_task(task);
    assert_eq!(storage.len(), 1);

    storage.remove_task(&id).expect("Failed to remove task");
    assert_eq!(storage.len(), 0);
    assert!(storage.get_task(&id).is_none());
}

#[test]
fn test_remove_non_existent_task() {
    let mut storage = TaskStorage::new();
    let random_id = uuid::Uuid::new_v4();
    let result = storage.remove_task(&random_id);
    assert!(matches!(result, Err(TaskForgeError::TaskNotFound)));
}

#[test]
fn test_clear_all() {
    let mut storage = TaskStorage::new();
    storage.add_task(Task::new("Task 1".to_string(), None));
    storage.add_task(Task::new("Task 2".to_string(), None));
    assert_eq!(storage.len(), 2);

    storage.clear_all();
    assert_eq!(storage.len(), 0);
    assert!(storage.is_empty());
}

#[test]
fn test_save_and_load() {
    let temp_dir = std::env::temp_dir().join(format!("taskforge_test_{}", uuid::Uuid::new_v4()));
    let file_path = temp_dir.join("tasks.json");

    let mut storage = TaskStorage::new();
    let task = Task::new("Saved task".to_string(), Some("Desc".to_string()));
    let id = *task.get_id();
    storage.add_task(task);

    storage
        .save_to_file(&file_path)
        .expect("Failed to save to file");

    let loaded = TaskStorage::load_from_file(&file_path).expect("Failed to load from file");
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded.get_task(&id).unwrap().get_title(), "Saved task");

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_load_non_existent_file() {
    let non_existent =
        std::env::temp_dir().join(format!("non_existent_{}.json", uuid::Uuid::new_v4()));
    let storage = TaskStorage::load_from_file(&non_existent).expect("Should return empty storage");
    assert!(storage.is_empty());
}

#[test]
fn test_load_empty_file() {
    let empty_file_path = std::env::temp_dir().join(format!("empty_{}.json", uuid::Uuid::new_v4()));
    File::create(&empty_file_path).expect("Failed to create empty file");

    let storage = TaskStorage::load_from_file(&empty_file_path)
        .expect("Should return empty storage for empty file");
    assert!(storage.is_empty());

    let _ = fs::remove_file(&empty_file_path);
}
