use crate::error::TaskForgeError;
use crate::models::task::Task;
use crate::storage::repository::TaskRepository;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FileStorage {
    #[serde(skip)]
    path: Option<PathBuf>,
    tasks: Vec<Task>,
}

/// Backward compatibility alias for `FileStorage`.
pub type TaskStorage = FileStorage;

impl FileStorage {
    pub fn new() -> Self {
        Self {
            path: None,
            tasks: Vec::new(),
        }
    }

    pub fn with_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: Some(path.as_ref().to_path_buf()),
            tasks: Vec::new(),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, TaskForgeError> {
        let path_ref = path.as_ref();

        // If the file doesn't exist, return an empty storage with the path recorded
        if !path_ref.exists() {
            return Ok(Self::with_path(path_ref));
        }

        let file = File::open(path_ref)?;
        if file.metadata()?.len() == 0 {
            return Ok(Self::with_path(path_ref));
        }

        let reader = BufReader::new(file);

        // Try to deserialize the JSON content
        let tasks: Vec<Task> = serde_json::from_reader(reader)?;

        Ok(Self {
            path: Some(path_ref.to_path_buf()),
            tasks,
        })
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), TaskForgeError> {
        let path = path.as_ref();

        // Ensure the directory exists
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }

        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        serde_json::to_writer_pretty(&mut writer, &self.tasks)?;
        writer.flush()?;

        Ok(())
    }

    pub fn save(&self) -> Result<(), TaskForgeError> {
        if let Some(ref path) = self.path {
            self.save_to_file(path)
        } else {
            Ok(())
        }
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn get_task(&self, id: &Uuid) -> Option<&Task> {
        self.tasks.iter().find(|task| task.get_id() == id)
    }

    pub fn get_task_mut(&mut self, id: &Uuid) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.get_id() == id)
    }

    pub fn remove_task(&mut self, id: &Uuid) -> Result<(), TaskForgeError> {
        let index = self
            .tasks
            .iter()
            .position(|task| task.get_id() == id)
            .ok_or(TaskForgeError::TaskNotFound)?;

        self.tasks.remove(index);
        Ok(())
    }

    pub fn complete_task(&mut self, id: &Uuid) -> Result<(), TaskForgeError> {
        let task = self.get_task_mut(id).ok_or(TaskForgeError::TaskNotFound)?;

        task.complete();
        Ok(())
    }

    pub fn clear_all(&mut self) {
        self.tasks.clear();
    }

    pub fn get_all_tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

impl TaskRepository for FileStorage {
    fn get_task(&self, id: &Uuid) -> Result<Option<Task>, TaskForgeError> {
        Ok(self.get_task(id).cloned())
    }

    fn get_all_tasks(&self) -> Result<Vec<Task>, TaskForgeError> {
        Ok(self.tasks.clone())
    }

    fn add_task(&mut self, task: Task) -> Result<(), TaskForgeError> {
        self.add_task(task);
        self.save()
    }

    fn update_task(&mut self, task: Task) -> Result<(), TaskForgeError> {
        let index = self
            .tasks
            .iter()
            .position(|t| t.get_id() == task.get_id())
            .ok_or(TaskForgeError::TaskNotFound)?;
        self.tasks[index] = task;
        self.save()
    }

    fn remove_task(&mut self, id: &Uuid) -> Result<(), TaskForgeError> {
        self.remove_task(id)?;
        self.save()
    }

    fn clear_all(&mut self) -> Result<(), TaskForgeError> {
        self.clear_all();
        self.save()
    }
}
