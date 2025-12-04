#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    let grid = aoc25::read_input(env!("CARGO_BIN_NAME")).expect("input");
    let mut count = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y].chars().nth(x).expect("cell") == '.' {
                continue;
            }
            let mut adjacents = 0;
            for i in -1..=1 {
                let Ok(row): Result<usize, _> = (y as i128 + i).try_into()
                else {
                    continue;
                };
                let Some(row) = grid.get(row) else {
                    continue;
                };
                for j in -1..=1 {
                    if i == 0 && j == 0 {
                        continue;
                    }
                    let Ok(col): Result<usize, _> = (x as i128 + j).try_into()
                    else {
                        continue;
                    };
                    if row.chars().nth(col) == Some('@') {
                        adjacents += 1;
                    }
                }
            }
            if adjacents < 4 {
                count += 1;
            }
        }
    }

    println!("Accessible rolls: {count}");
}
