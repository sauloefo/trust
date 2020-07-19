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

impl Domain {
    // TODO: create a version to receive n elements
    fn new (task1: Task, task2: Task) -> Domain {
        Domain {
            elements: vec![task1, task2]
        }
    }
}

fn main() {
    let my_tasks= Domain::new(
        Task::new("Task 1"),
        Task::new("Task 2")
    );

    for task in my_tasks.elements {
        println!("{}", task.description);
    }
}
