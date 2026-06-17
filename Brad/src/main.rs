mod print;

fn main() {
    // println!("Hello, world!");
    print::run();

    // formatting
    println!("Number: {}", 1);
    println!("{} is from {}", "Kevin","Kericho");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", {"Kevin"},{"Kericho"}, {"code"} );
}
