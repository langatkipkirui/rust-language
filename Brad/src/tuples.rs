// Tuples group together values different types
// max 12 elements
pub fn run() {
    let person: (&str, &str, i8) = ("Vanessa", "Bogota", 26);

    println!("{} is from {} and she is {}", person.0, person.1, person.2)
}
