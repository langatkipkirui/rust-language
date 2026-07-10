// Compound Data Types
// arrays, tuples, slices, and strings (slice string)

pub fn run() {
    // arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("An array: {:?}", numbers);

    // string array
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits array 1st element: {}", fruits[0]);
    println!("Fruits array 2nd element: {}", fruits[1]);
    println!("Fruits array 3rd element: {}", fruits[2]);

    // tuples
    let human = ("Alice", 30, false);
    println!("Human tuple {:?}", human);

    // --> mix tuples
    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My mix tuple: {:?}", my_mix_tuple)
}
