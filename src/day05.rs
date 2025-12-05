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
        let low = low.parse().expect("low");
        let high = high.parse().expect("high");
        if !ranges.iter().any(|(l, h)| *l <= low && high <= *h) {
            ranges.retain_mut(|(l, h)| {
                if low <= *l && *h <= high {
                    false
                } else {
                    if low <= *l && *l <= high {
                        *l = high + 1;
                    } else if low <= *h && *h <= high {
                        *h = low - 1;
                    }
                    true
                }
            });
            ranges.push((low, high));
        }
    }

    println!(
        "Fresh ingredient IDs: {}",
        ranges.iter().map(|(l, h)| h - l + 1).sum::<u128>()
    );
}
