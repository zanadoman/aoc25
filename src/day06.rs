#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]
#![allow(unused)]

fn main() {
    let mut input = aoc25::read_input(env!("CARGO_BIN_NAME")).expect("input");
    let matrix: Vec<Vec<char>> =
        input.iter().map(|l| l.chars().collect()).collect();
    let mut exprs: Vec<String> = Vec::new();

    for i in (0..matrix
        .iter()
        .max_by(|l, r| l.len().cmp(&r.len()))
        .expect("max")
        .len())
        .rev()
    {
        let expr = matrix
            .iter()
            .filter_map(|r| {
                r.get(i)
                    .and_then(|c| if *c == ' ' { None } else { Some(c) })
            })
            .fold(String::new(), |s, c| format!("{s}{c}"))
            .trim()
            .to_owned();
        if !expr.is_empty() {
            let last = expr.chars().last().expect("last");
            match last {
                '+' | '*' => {
                    exprs.push(expr[..expr.len() - 1].to_owned());
                    exprs.push(last.to_string());
                }
                _ => exprs.push(expr),
            }
        }
    }

    let mut results = Vec::new();
    let mut operands: Vec<i128> = Vec::new();

    for expr in exprs {
        match expr.as_str() {
            "+" => {
                results.push(operands.pop().expect("operand"));
                while let Some(operand) = operands.pop() {
                    *results.last_mut().expect("last") += operand;
                }
            }
            "*" => {
                results.push(operands.pop().expect("operand"));
                while let Some(operand) = operands.pop() {
                    *results.last_mut().expect("last") *= operand;
                }
            }
            _ => operands.push(expr.parse().expect("operand")),
        }
    }

    println!("Result: {}", results.iter().sum::<i128>());
}
