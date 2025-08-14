use std::io::{self, Write};

#[derive(Debug)]
struct Task {
    description: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n--- To-Do List ---");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Mark task as done");
        println!("4. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                print!("Enter task description: ");
                io::stdout().flush().unwrap();
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();
                tasks.push(Task {
                    description: desc.trim().to_string(),
                    done: false,
                });
                println!("Task added!");
            }
            "2" => {
                println!("\nYour Tasks:");
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.done { "[x]" } else { "[ ]" };
                    println!("{} {} {}", i + 1, status, task.description);
                }
            }
            "3" => {
                print!("Enter task number to mark done: ");
                io::stdout().flush().unwrap();
                let mut num = String::new();
                io::stdin().read_line(&mut num).unwrap();
                if let Ok(index) = num.trim().parse::<usize>() {
                    if index > 0 && index <= tasks.len() {
                        tasks[index - 1].done = true;
                        println!("Task marked as done!");
                    } else {
                        println!("Invalid task number.");
                    }
                }
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}
