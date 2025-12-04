#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

fn remove_rolls(grid: &mut [String]) -> i32 {
    let mut removed = 0;

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
                grid[y].replace_range(x..=x, ".");
                removed += 1;
            }
        }
    }

    removed
}

fn main() {
    let mut grid = aoc25::read_input(env!("CARGO_BIN_NAME")).expect("input");
    let mut count = remove_rolls(&mut grid);
    let mut removed = count;

    while 0 < count {
        count = remove_rolls(&mut grid);
        removed += count;
    }

    println!("Removed rolls: {removed}");
}
