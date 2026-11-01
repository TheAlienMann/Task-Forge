use std::fs;
use taskforge::error::TaskForgeError;
use taskforge::storage::InMemoryStorage;
use taskforge::TaskManager;
use uuid::Uuid;

fn get_temp_storage_path() -> (std::path::PathBuf, String) {
    let temp_dir = std::env::temp_dir().join(format!("taskforge_tm_test_{}", Uuid::new_v4()));
    let file_path = temp_dir.join("tasks.json");
    let path_str = file_path.to_str().unwrap().to_string();
    (temp_dir, path_str)
}

#[test]
fn test_task_manager_add_and_list() {
    let (temp_dir, path) = get_temp_storage_path();
    {
        let mut manager = TaskManager::from_file(&path).expect("Failed to create manager");
        assert!(manager.list_tasks().is_empty());

        let task = manager
            .add_task("Test task".to_string(), Some("Desc".to_string()))
            .expect("Failed to add task");
        assert_eq!(task.get_title(), "Test task");
        assert_eq!(manager.list_tasks().len(), 1);
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_task_manager_show_task() {
    let (temp_dir, path) = get_temp_storage_path();
    {
        let mut manager = TaskManager::from_file(&path).expect("Failed to create manager");
        let task = manager
            .add_task("Show me".to_string(), None)
            .expect("Failed to add task");
        let id_str = task.get_id().to_string();

        let shown = manager.show_task(&id_str).expect("Failed to show task");
        assert_eq!(shown.get_title(), "Show me");

        // Invalid ID
        assert!(matches!(
            manager.show_task("invalid-uuid"),
            Err(TaskForgeError::InvalidTaskId)
        ));

        // Non-existent ID
        let random_uuid = Uuid::new_v4().to_string();
        assert!(matches!(
            manager.show_task(&random_uuid),
            Err(TaskForgeError::TaskNotFound)
        ));
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_task_manager_complete_task() {
    let (temp_dir, path) = get_temp_storage_path();
    {
        let mut manager = TaskManager::from_file(&path).expect("Failed to create manager");
        let task = manager
            .add_task("Complete me".to_string(), None)
            .expect("Failed to add task");
        let id_str = task.get_id().to_string();

        manager
            .complete_task(&id_str)
            .expect("Failed to complete task");
        let shown = manager.show_task(&id_str).expect("Failed to show task");
        assert!(shown.is_completed());

        // Invalid ID
        assert!(matches!(
            manager.complete_task("bad-id"),
            Err(TaskForgeError::InvalidTaskId)
        ));

        // Not found
        let random_uuid = Uuid::new_v4().to_string();
        assert!(matches!(
            manager.complete_task(&random_uuid),
            Err(TaskForgeError::TaskNotFound)
        ));
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_task_manager_edit_task() {
    let (temp_dir, path) = get_temp_storage_path();
    {
        let mut manager = TaskManager::from_file(&path).expect("Failed to create manager");
        let task = manager
            .add_task("Old title".to_string(), Some("Old desc".to_string()))
            .expect("Failed to add task");
        let id_str = task.get_id().to_string();

        // Edit title only
        manager
            .edit_task(&id_str, Some("New title".to_string()), None)
            .expect("Failed to edit title");
        let shown = manager.show_task(&id_str).unwrap();
        assert_eq!(shown.get_title(), "New title");
        assert_eq!(shown.get_description(), Some("Old desc"));

        // Edit description only
        manager
            .edit_task(&id_str, None, Some(Some("New desc".to_string())))
            .expect("Failed to edit desc");
        let shown = manager.show_task(&id_str).unwrap();
        assert_eq!(shown.get_description(), Some("New desc"));

        // Clear description
        manager
            .edit_task(&id_str, None, Some(None))
            .expect("Failed to clear desc");
        let shown = manager.show_task(&id_str).unwrap();
        assert_eq!(shown.get_description(), None);

        // Invalid ID
        assert!(matches!(
            manager.edit_task("bad", Some("T".to_string()), None),
            Err(TaskForgeError::InvalidTaskId)
        ));

        // Not found
        let random_uuid = Uuid::new_v4().to_string();
        assert!(matches!(
            manager.edit_task(&random_uuid, Some("T".to_string()), None),
            Err(TaskForgeError::TaskNotFound)
        ));
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_task_manager_delete_and_clear() {
    let (temp_dir, path) = get_temp_storage_path();
    {
        let mut manager = TaskManager::from_file(&path).expect("Failed to create manager");
        let task1 = manager
            .add_task("Task 1".to_string(), None)
            .expect("Failed to add task");
        let _task2 = manager
            .add_task("Task 2".to_string(), None)
            .expect("Failed to add task");

        assert_eq!(manager.list_tasks().len(), 2);

        manager
            .delete_task(&task1.get_id().to_string())
            .expect("Failed to delete task");
        assert_eq!(manager.list_tasks().len(), 1);

        manager.clear_all().expect("Failed to clear tasks");
        assert!(manager.list_tasks().is_empty());

        // Delete not found
        let random_uuid = Uuid::new_v4().to_string();
        assert!(matches!(
            manager.delete_task(&random_uuid),
            Err(TaskForgeError::TaskNotFound)
        ));

        // Delete invalid ID
        assert!(matches!(
            manager.delete_task("invalid"),
            Err(TaskForgeError::InvalidTaskId)
        ));
    }
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_task_manager_with_in_memory_repository() {
    let memory_repo = InMemoryStorage::new();
    let mut manager = TaskManager::new(memory_repo);

    assert!(manager.list_tasks().is_empty());

    let task = manager
        .add_task(
            "In-memory task".to_string(),
            Some("No disk I/O".to_string()),
        )
        .expect("Failed to add task");
    let id_str = task.get_id().to_string();

    assert_eq!(manager.list_tasks().len(), 1);
    assert_eq!(
        manager.show_task(&id_str).unwrap().get_title(),
        "In-memory task"
    );

    manager
        .complete_task(&id_str)
        .expect("Failed to complete task");
    assert!(manager.show_task(&id_str).unwrap().is_completed());

    manager.delete_task(&id_str).expect("Failed to delete task");
    assert!(manager.list_tasks().is_empty());
}

#[test]
fn test_task_manager_priority_and_filtering() {
    let mut manager = TaskManager::new(InMemoryStorage::new());

    let t1 = manager
        .add_task_with_priority("Urgent task".to_string(), None, taskforge::Priority::Urgent)
        .expect("Failed to add");
    let _t2 = manager
        .add_task_with_priority("Low task".to_string(), None, taskforge::Priority::Low)
        .expect("Failed to add");
    let _t3 = manager
        .add_task("Medium task".to_string(), None)
        .expect("Failed to add");

    // Complete urgent task
    manager
        .complete_task(&t1.get_id().to_string())
        .expect("Failed to complete");

    // Filter completed
    let completed = manager.filter_tasks(true, false, None);
    assert_eq!(completed.len(), 1);
    assert_eq!(completed[0].get_title(), "Urgent task");

    // Filter pending
    let pending = manager.filter_tasks(false, true, None);
    assert_eq!(pending.len(), 2);

    // Filter by priority
    let low_priority = manager.filter_tasks(false, false, Some(taskforge::Priority::Low));
    assert_eq!(low_priority.len(), 1);
    assert_eq!(low_priority[0].get_title(), "Low task");
}

#[test]
fn test_task_manager_validation() {
    let mut manager = TaskManager::new(InMemoryStorage::new());

    // Empty title
    assert!(matches!(
        manager.add_task("   ".to_string(), None),
        Err(TaskForgeError::EmptyTitle)
    ));

    // Valid task
    let task = manager
        .add_task("Valid task".to_string(), None)
        .expect("Failed to add");

    // Edit to empty title
    assert!(matches!(
        manager.edit_task(&task.get_id().to_string(), Some("".to_string()), None),
        Err(TaskForgeError::EmptyTitle)
    ));
}
