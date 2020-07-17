struct Task {
    description: String
}

fn main() {
    let task1 = Task {
        description: String::from("Task 1")
    };

    let task2 = Task {
        description: String::from("Task 2")
    };

    let my_tasks = [
        task1,
        task2
    ];

    for task in my_tasks.iter() {
        println!("{}", task.description);
    }
}
