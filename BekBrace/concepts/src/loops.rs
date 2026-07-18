pub fn run() {
    // loop {
    //     println!("Hello World!");
    // }
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is: {}", result);
    multiple_loops();
    while_loop()
}
// loop labels to disambiguate between multiple loops

fn multiple_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count= {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}
fn while_loop() {
    let mut number = 5;
    while number != 0 {
        println!("Current no: {number}");
        number -= 1;
        // break;
    }
}
