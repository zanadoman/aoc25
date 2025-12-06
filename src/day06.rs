#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(unused)]

fn main() {
    let mut input = aoc25::read_input(env!("CARGO_BIN_NAME")).expect("input");
    let mut results: Vec<i128> = input
        .remove(0)
        .split_whitespace()
        .map(|o| o.parse().expect("operand"))
        .collect();
    let operators = input.remove(input.len() - 1);
    let operators: Vec<_> = operators.split_whitespace().collect();

    for line in input {
        for (i, operand) in line.split_whitespace().enumerate() {
            let operand: i128 = operand.parse().expect("operand");
            match operators[i] {
                "+" => results[i] += operand,
                "*" => results[i] *= operand,
                _ => panic!("Invalid operator"),
            }
        }
    }

    println!("{results:#?}");
    println!("Result: {}", results.iter().sum::<i128>());
}
