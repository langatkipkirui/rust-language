// mod print;
mod var;

// fn main() {
//     // println!("Hello, world!");
//     print::run();

//     // formatting
//     println!("Number: {}", 1);
//     println!("{} is from {}", "Kevin", "Kericho");

//     // Positional arguments
//     println!("{0} is from {1} and {0} likes to {2}", { "Kevin" }, { "Kericho" }, { "code" });

//     // name arguments
//     println!("{name} likes to play {activity}", name = "John", activity = "Baseball");

//     // Placeholder Traits
//     println!("Binary {:b} Hex: {:x} Octal: {:0}", 10, 10, 10);

//     // placeholder for debug trait
//     println!("{:?}", (12, true, "hello"));

//     // Basic math
//     println!("10+10={}", 10 + 10);
// }

fn main() {
    var::run();
}
