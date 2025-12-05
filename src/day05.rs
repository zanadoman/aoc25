#![feature(vec_try_remove)]
#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    let mut input = aoc25::read_input(env!("CARGO_BIN_NAME")).expect("input");
    let mut ranges: Vec<(u128, u128)> = Vec::new();

    while let Some(line) = input.try_remove(0)
        && !line.is_empty()
    {
        let (low, high) = line.split_once('-').expect("low, high");
        ranges.push((low.parse().expect("low"), high.parse().expect("high")));
    }

    println!(
        "Fresh ingredients: {}",
        input
            .iter()
            .map(|l| l.parse().expect("id"))
            .filter(|id| ranges
                .iter()
                .any(|(low, high)| (low..high).contains(&id)))
            .count()
    );
}
