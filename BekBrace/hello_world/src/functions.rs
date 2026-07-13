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
    let x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    human_id("Kevin", 22, 182.0, x);
}

//  you can insert more than on e parameter
fn human_id(name: &str, age: u32, height: f32, x: i32) {
    println!("My name is {}, I am {} years old and my height is {}", name, age, height);
    println!("The X value is {}", x);
}

// Expressions and statements
// An expression is anything that returns a Value
// A statement is anything that does not return a value
