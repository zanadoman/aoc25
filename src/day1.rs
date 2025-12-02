#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    let mut dial = 50;
    let mut password = 0;

    for line in aoc25::read_input(env!("CARGO_BIN_NAME")).expect("input") {
        let old = dial;
        let value: i128 = line[1..].parse().expect("value");
        dial += match line.chars().next() {
            Some('L') => -value,
            Some('R') => value,
            _ => panic!("Invalid direction"),
        };
        password += (dial / 100 - old / 100).abs();
    }

    println!("Password: {password}");
}
