// ownership
// [stopping/resuming the program]
/*
Ownership in Rust is introduced by Rust to solve memory safety issues and high perfomance at the same time
What is ownership
*/
// What is ownership
// Every value has a single owner [every variable has one value and it is it's sole owner]

// Ownership rules
// > Each value in Rust has a owner
// > There can only be one owner at a time
// > When owner goes out of the scope, the value will be dropped

// Each value in Rust has a variable that's its owner
pub fn run() {
    let s1 = String::from("Rust");
    let len = calculate_len(&s1);
    println!("Length of '{}' is {}.", s1, len);
    main();
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

// 2. There can only be one owner at a time.

fn main() {
    let s1 = String::from("Rust");
    let s2 = s1;

    println!("{}", s2)
}
