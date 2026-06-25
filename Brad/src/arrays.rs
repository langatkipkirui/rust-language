use std::mem;
// fixed list where elements are the same data types
pub fn run() {
    let numbers: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", numbers);

    // get a single value
    println!("single value:{}", numbers[2]);

    // get the length of the array
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers))
}
