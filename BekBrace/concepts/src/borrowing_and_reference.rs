// References and Borrowing
// Safety and Perfomance
// Borrowing and references are powerfull concepts

// Understanding References
// References: Enable you to borrow values without taking
// ownership.

// Mutable reference
// Create Refernce by add "&"
// -I- Immutable Reference

pub fn main() {
    let mut _x = 5;
    let _r = &mut _x;
    *_r += 1;
    *_r -= 3;

    println!("Value of _x: {}", _x);
}
