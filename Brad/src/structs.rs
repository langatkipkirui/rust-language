//A struct (short for "structure") is a custom data structure that lets you group related values together.
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
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
    struct Colorsec(u8, u8, u8);

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c = Colorsec(255, 0, 0);
    c.0 = 200;

    println!("Color: {} {} {}", c.0, c.1, c.2);

    let p = Person::new("John", "Doe");

    println!("person {} {}", p.first_name, p.last_name);
    println!("Person full name: {}", p.full_name());
}
