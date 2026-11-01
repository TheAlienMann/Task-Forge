use crate::error::TaskForgeError;
use crate::models::task::Task;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TaskStorage {
    tasks: Vec<Task>,
}

impl TaskStorage {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, TaskForgeError> {
        let path = path.as_ref();

        // If the file doesn't exist, return an empty storage
        if !path.exists() {
            return Ok(Self::new());
        }

        let file = File::open(path)?;
        if file.metadata()?.len() == 0 {
            return Ok(Self::new());
        }

        let reader = BufReader::new(file);

        // Try to deserialize the JSON content
        let tasks: Vec<Task> = serde_json::from_reader(reader)?;

        Ok(Self { tasks })
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

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn get_task(&self, id: &uuid::Uuid) -> Option<&Task> {
        self.tasks.iter().find(|task| task.get_id() == id)
    }

    pub fn get_task_mut(&mut self, id: &uuid::Uuid) -> Option<&mut Task> {
        self.tasks.iter_mut().find(|task| task.get_id() == id)
    }

    pub fn remove_task(&mut self, id: &uuid::Uuid) -> Result<(), TaskForgeError> {
        let index = self
            .tasks
            .iter()
            .position(|task| task.get_id() == id)
            .ok_or(TaskForgeError::TaskNotFound)?;

        self.tasks.remove(index);
        Ok(())
    }

    pub fn complete_task(&mut self, id: &uuid::Uuid) -> Result<(), TaskForgeError> {
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
