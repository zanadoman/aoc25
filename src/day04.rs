#![forbid(unsafe_code)]
#![deny(warnings)]
#![warn(clippy::nursery, clippy::pedantic)]

fn main() {
    let mut grid: Vec<Vec<char>> = aoc25::read_input(env!("CARGO_BIN_NAME"))
        .expect("input")
        .iter()
        .map(|l| l.chars().collect())
        .collect();
    let mut count = remove_rolls(&mut grid);
    let mut removed = count;

    while 0 < count {
        count = remove_rolls(&mut grid);
        removed += count;
    }

    println!("Removed rolls: {removed}");
}

fn remove_rolls(grid: &mut [Vec<char>]) -> i32 {
    let mut removed = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '.' {
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
                    if row.get(col) == Some(&'@') {
                        adjacents += 1;
                    }
                }
            }
            if adjacents < 4 {
                grid[y][x] = '.';
                removed += 1;
            }
        }
    }

    removed
}
