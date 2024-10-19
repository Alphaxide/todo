mod task;
mod io;

use crate::task::{add_task, display_tasks, update_task, delete_task, TaskStatus};
use crate::io::{save_tasks_to_file, load_tasks_from_file};

fn main() {
    let file_name = "tasks.json";
    let mut tasks = load_tasks_from_file(file_name);

    println!("-- To-Do List Manager --");

    // Add tasks
    add_task(&mut tasks, "Buy groceries".to_string());
    add_task(&mut tasks, "Finish homework".to_string());

    // Display tasks
    println!("\nTasks List:");
    display_tasks(&tasks);

    // Update a task
    println!("\nUpdating task status for task with ID 1:");
    update_task(&mut tasks, 1, TaskStatus::InProgress);
    display_tasks(&tasks);

    // Delete a task
    println!("\nDeleting task with ID 2:");
    delete_task(&mut tasks, 2);
    display_tasks(&tasks);

    // Save tasks to file
    save_tasks_to_file(&tasks, file_name);
}
