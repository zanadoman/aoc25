#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    let mut joltage = 0;

    for line in aoc25::read_input(env!("CARGO_BIN_NAME")).expect("input") {
        let mut begin = 0;
        let mut number = String::new();
        while number.len() != 12 {
            let remaining = &line[begin..line.len() - 11 + number.len()];
            let (position, max) = remaining
                .chars()
                .rev()
                .enumerate()
                .max_by_key(|(_, max)| *max)
                .expect("position, max");
            begin += remaining.len() - position;
            number.push(max);
        }
        joltage += number.parse::<u128>().expect("joltage");
    }

    println!("Joltage: {joltage}");
}
