#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    let input = aoc25::read_input(env!("CARGO_BIN_NAME")).expect("input");
    let mut dial = 50;
    let mut password = 0;

    for line in &input {
        let old = dial;
        let value = line[1..].parse::<i128>().expect("value");
        if line.starts_with('L') {
            dial -= value;
        } else if line.starts_with('R') {
            dial += value;
        } else {
            panic!("Invalid direction");
        }
        password += (dial / 100 - old / 100).abs();
    }

    println!("Password: {password}");
}
