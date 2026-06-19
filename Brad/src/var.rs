// variables hold primitive data or referneces to data
// variables are immutable by default
// Rust is block-scoped
pub fn run() {
    let name = "Langat";

    // let age = 21;
    // age = 21; --> we cannot mutate a var directly in Rust
    let mut age = 21;
    println!("My name is {} and I am {}", name, age);
    age = 22;

    println!("My name is {} and I am {}", name, age)
}
