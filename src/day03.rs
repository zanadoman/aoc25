#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    let mut joltage = 0;

    for line in aoc25::read_input(env!("CARGO_BIN_NAME")).expect("input") {
        let max = line.chars().max().expect("max");
        let position = line.chars().position(|c| c == max).expect("position");
        joltage += if position == 0 {
            format!("{max}{}", line[1..].chars().max().expect("max"))
        } else if position == line.len() - 1 {
            format!("{}{max}", line[..position].chars().max().expect("max"))
        } else {
            let low = line[..position].chars().max().expect("low");
            if max < low {
                format!("{low}{max}")
            } else {
                format!(
                    "{max}{}",
                    line[position + 1..].chars().max().expect("high")
                )
            }
        }
        .parse::<u128>()
        .expect("joltage");
    }

    println!("Joltage: {joltage}");
}
