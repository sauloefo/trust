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

struct Tasks {
    elements: Vec<Task>
}

impl Tasks {
    // TODO: create a version to receive n elements
    fn new (task1: Task, task2: Task) -> Tasks {
        Tasks {
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
    let my_tasks = get_tasks();

    show_tasks(&my_tasks);
}

fn get_tasks() -> Tasks {
    Tasks::new(
        Task::new("Task 1"),
        Task::new("Task 2")
    )
}

fn show_tasks(tasks: &Tasks) -> () {
    for description in tasks.get_description() {
        println!("{}", description);
    }
}
