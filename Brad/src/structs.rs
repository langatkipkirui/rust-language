//A struct (short for "structure") is a custom data structure that lets you group related values together.
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    // we can directly mutate the struct
    c.blue = 2;

    // tuple struct
    struct Color_2(u8, u8, u8);

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c = Color_2(255, 0, 0);
    c.0 = 200;

    println!("Color: {} {} {}", c.0, c.1, c.2)
}
