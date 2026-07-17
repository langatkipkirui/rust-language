// shadowing
// shadowing is a feature of Rs
// shadowing is not the same as making a variable as mut

pub fn run() {
    example_one();
    example_two();
    example_three()
}

// Example 1: Basic Shadowing
fn example_one() {
    let name = "Kevin"; // Immutable string
    println!("My name is {}", name); // Kevin

    {
        let name = "Rustacee"; // Shadows the outer variable
        println!("Inside shadowed block, my name is {}", name); // Rustacee
    }

    println!("Back to original: {}", name); // Kevin (shadow ends)
}

// Example 2: Changing Types with Shadowing
fn example_two() {
    let x = 5; // Immutable integer
    println!("{}", x); // Works because `x` is still an integer

    {
        let x = "five".to_string(); // Shadows and changes type to String
        println!("{}", x); // Now a string (e.g., "Kevin")
    }
    // Cannot use the original integer here unless shadowing ends earlier.
}

// Example 3: Shadowing with Mutability Changes

fn example_three() {
    let mut count = 0; // Mutable integer

    {
        let count = true; // Shadows and changes type to bool (mutability irrelevant)
        println!("Shadowed count is {}", count); // Prints "true"
    }

    // Original `count` remains mutable outside the shadow.
}
