pub fn run() {
    // primitive array

    let arr = [1, 2, 3, 4, 5];
    let arr2 = arr;

    println!("Values: {:?}", (arr, arr2));

    // with none-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. you will need to use a reference (&) to point to the resource

    let vec = vec![1, 2, 3, 4];
    let vec2 = &vec;

    println!("Values: {:?}", (&vec, vec2))
}
