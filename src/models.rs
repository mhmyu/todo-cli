use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubTask {
    pub id: usize,
    pub title: String,
    pub time_allocation_minutes: u32,
    pub completion_history: HashMap<String, bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub time_allocation_minutes: u32,
    pub subtasks: Vec<SubTask>,
    pub is_globally_completed: bool,
}

impl Task {
    pub fn new(id: usize, title: String, time_allocation_minutes: u32) -> Self {
        Self {
            id,
            title,
            time_allocation_minutes,
            subtasks: Vec::new(),
            is_globally_completed: false,
        }
    }

    pub fn current_subtasks_total_time(&self) -> u32 {
        self.subtasks.iter().map(|s| s.time_allocation_minutes).sum()
    }

    pub fn add_subtask(&mut self, title: String, time_allocation: u32) -> Result<(), String> {
        let current_total = self.current_subtasks_total_time();
        
        if current_total + time_allocation > self.time_allocation_minutes {
            let available_time = self.time_allocation_minutes - current_total;
            return Err(format!(
                "Gagal menambah sub-task! Alokasi waktu tugas utama adalah {}m. Total sub-task saat ini: {}m. Sisa waktu tersedia hanya {}m.",
                self.time_allocation_minutes, current_total, available_time
            ));
        }

        let subtask_id = self.subtasks.len() + 1;
        self.subtasks.push(SubTask {
            id: subtask_id,
            title,
            time_allocation_minutes: time_allocation,
            completion_history: HashMap::new(),
        });

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Storage {
    pub tasks: Vec<Task>,
}
