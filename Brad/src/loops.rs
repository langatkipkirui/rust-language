// A loop is used to iterate untill a condition is met
pub fn run() {
    let mut count = 0;

    // infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20000 {
            break;
        }
    }
}
