use std::io;

const SPEED_OF_LIGHT: u64 = 299_792_458;

fn time_to_travel(distance: u64) -> f64 {
    distance as f64 / SPEED_OF_LIGHT as f64
}
fn main() {
    const FIVE_HOURS_IN_SECONDS: u32 = 60 * 60 * 5;
    const PI: f64 = 3.14159;
    const CIRCUMFERENCE: f64 = 2.0 * PI * 5.0;

    println!("{FIVE_HOURS_IN_SECONDS}");
    println!("{PI}");
    println!("{CIRCUMFERENCE}");

    let distance = 1_000_000_000;
    let time = time_to_travel(distance);
    // println!("Time to travel {distance} meters at the speed of light: {time:.6} seconds");

    let spaces = "hello";
    let spaces = 8;

    // println!("{spaces}");

    let quotient = 56.7 / 32.2;
    let truncated: i32 = -5 / 3;

    let rm = 40 % 5;

    // println!("{rm}")

    let tup = (500, 56.6, 1);
    let (x, y, z) = tup;

    println!("{y}");

    let five_hundred = tup.0;
    let one = tup.2;

    println!("{one} and {five_hundred}");

    let a = [1, 2, 3, 4, 5];

    let a = [10; 5];

    println!("{:?}", a);
}
