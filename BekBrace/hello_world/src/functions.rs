// functions at the entry level should be main function
// a function or variable should be written in snake case
// snake case: hello_world
// kebab case: hello-world

pub fn run() {
    // println!("Hello Rust");
    tell_height(182);
}

fn tell_height(height: i32) {
    println!("My height is  {}cm", height)
}
