// variables hold primitive data or referneces to data
// variables are immutable by default
// Rust is block-scoped
pub fn run() {
    // LET
    let name = "Langat";

    // let age = 21;
    // age = 21; --> we cannot mutate a var directly in Rust
    let mut age = 21;
    println!("My name is {} and I am {}", name, age);
    age = 22;

    println!("My name is {} and I am {}", name, age);

    // CONST (add a type and use uppercase )
    const ID: i32 = 001;
    println!("ID {}", ID);

    // Assigning multiple vars
    let (my_name, my_age) = ("Putin", 77);
    println!("{} is the Soviet President and he is {} yrs old", my_name, my_age)
}
