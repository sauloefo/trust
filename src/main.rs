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

    fn get_description(&self) -> Vec<&str> {
        let mut descriptions: Vec<&str> = Vec::new();

        for task in &self.elements {
            descriptions.push(&task.description);
        }

        // TODO: remove duplications
        descriptions
    }
}

fn main() {
    let my_tasks= Domain::new(
        Task::new("Task 1"),
        Task::new("Task 2")
    );

    for description in my_tasks.get_description() {
        println!("{}", description);
    }
}
