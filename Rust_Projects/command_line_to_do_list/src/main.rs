use std::io;

struct Task {
    description: String,
    completed: bool,
}
impl Task {
    fn new(description: &str) -> Task {
        Task {
            description: description.to_string(),
            completed: false,
        }
    }
    fn complete(&mut self) {
        self.completed = true;
    }
    fn display(&self) {
        let status = if self.completed {
            "Completed"
        } else {
            "Pending"
        };
        println!("{}-{}", self.description, status);
    }
}
fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    loop {
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Exit");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        match choice.trim() {
            "1" => {
                let mut description = String::new();
                println!("Enter task description");
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read line");
                tasks.push(Task::new(&description.trim()))
            }
            "2" => {
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}:", i + 1);
                    task.display();
                }
            }
            "3" => {
                let mut index = String::new();
                println!("Enter task number to mark as completed:");
                io::stdin()
                    .read_line(&mut index)
                    .expect("Failed to read line");
                if let Ok(i) = index.trim().parse::<usize>() {
                    if i > 0 && i <= tasks.len() {
                        tasks[i - 1].complete();
                    } else {
                        println!("Invalid task number");
                    }
                }
            }
            "4" => break,
            _ => println!("Invalid choice"),
        }
    }
}
