pub fn run() {
    greeting("Hello", "Kevin");
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice meeting you", greet, name);
}
