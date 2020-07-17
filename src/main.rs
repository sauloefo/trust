struct Task {
    description: String
}

impl Task {
    fn new(description: &str) -> Task {
        Task {
            description: String::from(description)
        }
    }
}

fn main() {
    let task1 = Task::new("Task 1");

    let task2 = Task::new("Task 2");

    let my_tasks = [
        task1,
        task2
    ];

    for task in my_tasks.iter() {
        println!("{}", task.description);
    }
}
