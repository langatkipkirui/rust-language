pub fn run() {
    let age: u16 = 19;
    if age >= 18 {
        println!("You are old enough to vote");
    } else {
        println!("You are not allowed to vote");
    }

    // multiple conditions with else if:
    let number = 7;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number not divisible by 4,3,2");
    }

    // using if in a let statement
    let condition = false;
    let result = if condition { 4 } else { 3 };
    println!("Result: {result}")
}
