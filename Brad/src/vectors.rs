use std::mem;
// fixed list where elements are the same data types
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}", numbers);

    // Reassign a value to an vectors
    numbers[2] = 20;

    // Add on to vectors
    numbers.push(5);
    numbers.push(6);

    println!("Numbers: {:?}", numbers);

    // get a single value
    println!("single value:{}", numbers[2]);

    // get the length of the vectors
    println!("vectors length: {}", numbers.len());

    // Vectors are stack allocated
    println!("vectors occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..3];
    println!("Sliced numbers vectors: {:?}", slice);

    // loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;

        println!("Numbers Vec: {:?}", x);
    }
}
