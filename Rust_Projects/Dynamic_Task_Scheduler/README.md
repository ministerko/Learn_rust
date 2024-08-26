# Task Management System in Rust
This code implements a simple task management system in Rust, consisting of two main structs: Task and Scheduler.

## Task Struct
The Task struct represents a single task with the following attributes:

`name`: a string representing the task's name
`priority`: an unsigned 8-bit integer (u8) representing the task's priority
`status`: a string representing the task's status (initially set to "Pending")
```rust

struct Task {
    name: String,
    priority: u8,
    status: String,
}
```
## Scheduler Struct
The Scheduler struct manages a collection of tasks and provides methods to add, execute, and display tasks.

tasks: a `vector` of Task instances
```rust

struct Scheduler {
    tasks: Vec<Task>,
}
```
## Scheduler Methods
#### Add Task
The add_task method adds a new task to the scheduler's task list.

`name`: a string slice (&str) representing the task's name
`priority`: an unsigned 8-bit integer (u8) representing the task's priority
```rust

Copy code
fn add_task(&mut self, name: &str, priority: u8) {
    self.tasks.push(Task {
        name: name.to_string(),
        priority,
        status: "Pending".to_string(),
    });
}
```
#### Execute Task
The execute_task method executes the task with the highest priority.

It finds the task with the highest priority using the max_by_key method.
If a task is found, it updates the task's status to "Completed" and prints a message indicating the task has been executed.
If no tasks are found, it prints a message indicating that there are no tasks to execute.
```rust

fn execute_task(&mut self) {
    if let Some((index, _)) = self
        .tasks
        .iter()
        .enumerate()
        .max_by_key(|(_, task)| task.priority)
    {
        let task = &mut self.tasks[index];
        task.status = "Completed".to_string();
        println!("Executed task: {}", task.name);
    } else {
        println!("No tasks to execute");
    }
}
```
#### Show Tasks
The show_tasks method displays all tasks in the scheduler's task list.

```rust

fn show_tasks(&self) {
    for task in &self.tasks {
        println!(
            "Task: {}, Priority: {}, Status: {}",
            task.name, task.priority, task.status
        );
    }
}
```
## Main Function
The main function demonstrates the usage of the Scheduler struct and its methods.

It creates a new Scheduler instance with an empty task list.
It adds three tasks with the same priority (2).
It displays all tasks using the show_tasks method.
It executes the task with the highest priority using the execute_task method.
It displays all tasks again to show the updated status of the executed task.
```rust

fn main() {
    let mut scheduler = Scheduler { tasks: Vec::new() };
    
    scheduler.add_task("write report", 2);
    scheduler.add_task("update website", 2);
    scheduler.add_task("Clean House", 2);

    scheduler.show_tasks();
    scheduler.execute_task();
    scheduler.show_tasks();
}
```
This code provides a basic task management system with a simple priority-based execution mechanism. However, it does not handle cases where multiple tasks have the same highest priority, and it does not provide any error handling or validation for task names or priorities.