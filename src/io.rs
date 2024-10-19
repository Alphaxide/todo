use std::fs::{File, read_to_string};
use std::io::Write;
use crate::task::Task;

// Function to save tasks to a JSON file
pub fn save_tasks_to_file(tasks: &Vec<Task>, file_name: &str) {
    let json = serde_json::to_string(&tasks).expect("Error serializing tasks");
    let mut file = File::create(file_name).expect("Error creating file");
    file.write_all(json.as_bytes()).expect("Error writing to file");
    println!("Tasks saved successfully.");
}

// Function to load tasks from a JSON file
pub fn load_tasks_from_file(file_name: &str) -> Vec<Task> {
    let file_content = read_to_string(file_name).unwrap_or_else(|_| String::from("[]"));
    let tasks: Vec<Task> = serde_json::from_str(&file_content).expect("Error deserializing tasks");
    tasks
}
