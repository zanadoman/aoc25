#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    for line in aoc25::read_input(env!("CARGO_BIN_NAME")).expect("input") {
        println!("{line}");
    }
}
