#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    let input = aoc25::read_input(env!("CARGO_BIN_NAME")).expect("input");
    let mut dial = 50;
    let mut password = 0;

    for line in &input {
        let value = line[1..].parse::<i16>().expect("value");
        if line.starts_with('L') {
            dial -= value;
        } else if line.starts_with('R') {
            dial += value;
        } else {
            panic!("Invalid direction");
        }
        dial %= 100;
        if dial == 0 {
            password += 1;
        }
    }

    println!("Password: {password}");
}
