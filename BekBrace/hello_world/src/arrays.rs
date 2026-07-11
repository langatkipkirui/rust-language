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
    println!("My mix tuple: {:?}", my_mix_tuple);

    // slices: [1,2,3,4,5]
    let number_slices = &[1, 2, 3, 4, 5];
    println!("Number Slice: {:?}", number_slices);
    let animal_slices = &["Lion", "Cheetah", "Elephant", "Crocodile", "Dog"];
    println!("Animal Slice: {:?}", animal_slices);
    let book_slices = &[
        &"IT".to_string(),
        &"Harry Potter".to_string(),
        &"Kidagaa Kimemwozea".to_string(),
    ];
    println!("Number Slice: {:?}", book_slices);

    // Strings VS String Slices (&str)
    // Strings[growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");

    println!("Stone cold says: {}", stone_cold);

    // B- &str (String Slice)
    let string: String = String::from("Hello world!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice)
}
