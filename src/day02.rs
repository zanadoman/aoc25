#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    println!(
        "Sum of invalid IDs: {}",
        aoc25::read_input(env!("CARGO_BIN_NAME"))
            .expect("input")
            .iter()
            .filter_map(|l| l.split_once('-'))
            .map(|(l, h)| {
                (l.parse().expect("l")..=h.parse().expect("h"))
                    .filter(|v: &u128| {
                        let s = v.to_string();
                        (1..s.len()).filter(|v| s.len().is_multiple_of(*v)).any(
                            |d| {
                                (1..s.len() / d)
                                    .all(|i| s[d * i..d * (i + 1)] == s[..d])
                            },
                        )
                    })
                    .sum::<u128>()
            })
            .sum::<u128>()
    );
}
