use serde::{Serialize, Deserialize};
use std::fmt;

// Enum to define the status of a task
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

// Struct to define a Task
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
}

// Display implementations for better output
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Completed => write!(f, "Completed"),
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {}, Description: {}, Status: {}",
            self.id, self.description, self.status
        )
    }
}

// CRUD Functions

// Add a new task to the list
pub fn add_task(tasks: &mut Vec<Task>, description: String) {
    let new_id = tasks.len() as u32 + 1; // Auto-generate an ID
    let new_task = Task {
        id: new_id,
        description,
        status: TaskStatus::Pending,
    };
    tasks.push(new_task);
    println!("Task added successfully.");
}

// Display all tasks
pub fn display_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        for task in tasks {
            println!("{}", task);
        }
    }
}

// Update a task status
pub fn update_task(tasks: &mut Vec<Task>, task_id: u32, new_status: TaskStatus) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
        task.status = new_status;
        println!("Task updated successfully.");
    } else {
        println!("Task not found.");
    }
}

// Delete a task by ID
pub fn delete_task(tasks: &mut Vec<Task>, task_id: u32) {
    if let Some(pos) = tasks.iter().position(|t| t.id == task_id) {
        tasks.remove(pos);
        println!("Task deleted successfully.");
    } else {
        println!("Task not found.");
    }
}
