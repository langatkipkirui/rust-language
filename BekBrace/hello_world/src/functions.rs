// functions at the entry level should be main function
// a function or variable should be written in snake case
// snake case: hello_world
// kebab case: hello-world

pub fn run() {
    // println!("Hello Rust");
    tell_height(182);
}

fn tell_height(height: i32) {
    // println!("My height is  {}cm", height)
    human_id("Kevin", 22, 182.0);
}

//  you can insert more than on e parameter
fn human_id(name: &str, age: u32, height: f32) {
    print!("My name is {}, I am {} years old and my height is {}", name, age, height)
}
