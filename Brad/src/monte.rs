use std::time::Instant;

pub fn run() {
    let start = Instant::now();

    // Explicitly make this a 64-bit unsigned integer (u64)
    let iterations: u64 = 100_000_000;
    let mut inside_circle = 0;

    // Force the loop counter 'i' to be a u64 to prevent multiplication overflow
    for i in 0..iterations {
        let x = (((i * 32719 + 3) % 32749) as f64) / 32749.0;
        let y = (((i * 40009 + 7) % 40009) as f64) / 40009.0;

        if x * x + y * y <= 1.0 {
            inside_circle += 1;
        }
    }

    let pi = (4.0 * (inside_circle as f64)) / (iterations as f64);
    println!("Estimated Pi: {}", pi);

    let duration = start.elapsed();
    println!("Rust Execution Time: {:?}", duration);
}
