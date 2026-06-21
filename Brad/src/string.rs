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

    println!("{}", great);
}
