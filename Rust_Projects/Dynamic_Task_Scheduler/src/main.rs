//Task struct to respresent a Task
struct Task {
    name: String,
    priority: u8,
    status: String,
}
//Scheduler struct to manage tasks
struct Scheduler {
    tasks: Vec<Task>,
}

impl Scheduler {
    //Add new task
    fn add_task(&mut self, name: &str, priority: u8) {
        self.tasks.push(Task {
            name: name.to_string(),
            priority,
            status: "Pending".to_string(),
        });
    }
    //Execute the highest priority task
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
    //Display all tasks
    fn show_tasks(&self) {
        for task in &self.tasks {
            println!(
                "Task: {},Priority: {},Status: {}",
                task.name, task.priority, task.status
            );
        }
    }
}
fn main() {
    let mut scheduler = Scheduler { tasks: Vec::new() };
    
    scheduler.add_task("write report",2);
    scheduler.add_task("update website",2);
    scheduler.add_task("Clean House",2);

    scheduler.show_tasks();
    scheduler.execute_task();
    scheduler.show_tasks();

}
