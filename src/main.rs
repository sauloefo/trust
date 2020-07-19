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

struct Domain {
    elements: Vec<Task>
}

fn main() {
    let my_tasks= Domain {
        elements: vec![
            Task::new("Task 1"),
            Task::new("Task 2")
        ]
    };

    for task in my_tasks.elements.iter() {
        println!("{}", task.description);
    }
}
