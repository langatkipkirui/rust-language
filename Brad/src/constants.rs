// Constant variables are used to store values that never change.
// Unlike regular variables, constants must be defined with a type (e.g. i32 or char).
pub fn run() {
    const BIRTHYEAR: i32 = 1985;

    println!("He was born in {}", BIRTHYEAR);

    // You must write the type when creating a constant. You cannot let Rust guess the type like you can with regular variables:

    // const BIRTHYEAR: i32 = 1980; --> Ok
    // const BIRTHYEAR = 1980; --> Error: missing type
}
