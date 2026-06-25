// Operators are used to perform operations on values and variables.

// Rust supports many common operators, like:

// Comparison Operators
// Logical Operators

pub fn run() {
    // Arithmetic Operators
    println!("{}", "------Arithmetic operators-------");
    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);

    // Assignment Operators
    println!("{}", "------Assignment operators-------");
    let mut x = 10;
    println!("Start: {}", x);

    x += 5;
    println!("After += 5: {}", x);

    x -= 2;
    println!("After -= 2: {}", x);

    x *= 2;
    println!("After *= 2: {}", x);

    x /= 3;
    println!("After /= 3: {}", x);

    x %= 4;
    println!("After %= 4: {}", x);

    // Comparison operators
    // they return true or false: ==, !=, >, <, >=, <=

    let a = 5;
    let b = 10;

    println!("{}", "------Comparison operators-------");
    println!("5 == 10: {}", a == b);
    println!("5 < 10: {}", a < b);
    println!("5 >= 10: {}", a >= b);

    // Logical Operators
    // used to work with boolean values:
    // && AND true is both values are true
    // || OR  inverts the boolean
}
