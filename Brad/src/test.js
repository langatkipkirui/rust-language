console.time("JavaScript Execution Time");

const iterations = 100000000;
let insideCircle = 0;

for (let i = 0; i < iterations; i++) {
  // Basic linear congruential generator for fast pseudo-random numbers
  let x = ((i * 32719 + 3) % 32749) / 32749;
  let y = ((i * 40009 + 7) % 40009) / 40009;

  if (x * x + y * y <= 1.0) {
    insideCircle++;
  }
}

const pi = (4.0 * insideCircle) / iterations;
console.log(`Estimated Pi: ${pi}`);
console.timeEnd("JavaScript Execution Time");
