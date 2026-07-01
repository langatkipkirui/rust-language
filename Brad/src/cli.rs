use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    let command = args[1].clone();
    let name = "Kevin";

    // println!("Args: {:?}", args);
    // println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you", name)
    } else if command == "status" {
        println!("Status is {}", "100 %")
    } else {
        println!("The term '{}' is not recognized as the name of a cmdlet, function, script file, or operable program. Check the spelling of the name.", command);
    }
}
