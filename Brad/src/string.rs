// primitive str- Imutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure -Use when you need to modify or own string data

pub fn run() {
    let hello = "Hello";
    let mut great = String::from("Hello ");

    println!("{}", hello);

    // Get length
    println!("{:?}", (hello, great.len()));

    // push a single character to a string var
    great.push('W');

    // push multiple characters into a string
    great.push_str("orld!");

    // capacity in bytes
    println!("Capacity: {}", great.capacity());

    // check if empty
    println!("Is empty: {}", great.is_empty());

    // check if it contains: "world"
    println!("Contsins World: {}", great.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("Hello", "World"));

    // loop through string by whitespace
    let hello_world = "Hello World!";
    for word in hello_world.split_whitespace() {
        println!("{}", word);
    }

    // Create a string with  capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
}
