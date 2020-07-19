struct Task {
    description: String,
    done: bool
}

impl Task {
    fn not_done(description: &str) -> Task {
        Task::new(description, false)
    }

    fn done(description: &str) -> Task {
        Task::new(description, false)
    }

    fn new(description: &str, done: bool) -> Task {
        Task {
            description: String::from(description),
            done: done
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
        Task::not_done("Task 1"),
        Task::done("Task 2")
    )
}

fn show_tasks(tasks: &Tasks) -> () {
    for task in tasks.elements.iter() {
        let done_description = if task.done { "done" } else { "not done" };
        println!("[{}] {}", done_description, task.description);
    }
}
