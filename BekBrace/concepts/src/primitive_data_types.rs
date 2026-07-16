// primitve data types
// int, float, char

// Integer
// Rust has signed (+ and -) and usigned integer(onl+) types of different sizes.
// i8, i16, i32 i64, i128: signed integers
// u8, u16, u32 u64, u128: unsigned integers

pub fn run() {
    let x: i32 = -42;
    let y: u64 = 100;

    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    // diff btw 132 and i64;
    // i32-2,147,483,647;
    // i64-9,223,372,036,854,775,807

    let e: i32 = 2147483647;
    let i: i64 = 9223372036854775807;

    println!("max value of i32:{}", e);
    println!("max value of i32:{}", i);

    // Floats [Floating Point Types]
    // f32, f64
    let pi: f64 = 3.14;
    println!("Float {}", pi);

    // Boolean Values: true, false
    let is_snowing: bool = true;

    println!("Is it snowing? {}", is_snowing);

    // character type - chars
    let letter: char = 'a';
    println!("{} is the first letter of alphapet", letter)
}
