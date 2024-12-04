use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(grid: &Vec<Vec<u8>>) -> usize {
    let mut res: usize = 0;

    for row in 0..grid.len() {
        for col in 0..grid[1].len() {
            if grid[row][col] != b'X' {
                continue;
            }
            // up
            if row + 3 < grid.len()
                && grid[row + 1][col] == b'M'
                && grid[row + 2][col] == b'A'
                && grid[row + 3][col] == b'S'
            {
                res += 1;
            }
            // down
            if row as isize - 3 >= 0
                && grid[row - 1][col] == b'M'
                && grid[row - 2][col] == b'A'
                && grid[row - 3][col] == b'S'
            {
                res += 1;
            }
            // right
            if col + 3 < grid[1].len()
                && grid[row][col + 1] == b'M'
                && grid[row][col + 2] == b'A'
                && grid[row][col + 3] == b'S'
            {
                res += 1;
            }
            // left
            if col as isize - 3 >= 0
                && grid[row][col - 1] == b'M'
                && grid[row][col - 2] == b'A'
                && grid[row][col - 3] == b'S'
            {
                res += 1;
            }
            // up-right
            if row + 3 < grid.len()
                && col + 3 < grid[1].len()
                && grid[row + 1][col + 1] == b'M'
                && grid[row + 2][col + 2] == b'A'
                && grid[row + 3][col + 3] == b'S'
            {
                res += 1;
            }
            // up-left
            if row + 3 < grid.len()
                && col as isize - 3 >= 0
                && grid[row + 1][col - 1] == b'M'
                && grid[row + 2][col - 2] == b'A'
                && grid[row + 3][col - 3] == b'S'
            {
                res += 1;
            }
            // down-right
            if row as isize - 3 >= 0
                && col + 3 < grid[1].len()
                && grid[row - 1][col + 1] == b'M'
                && grid[row - 2][col + 2] == b'A'
                && grid[row - 3][col + 3] == b'S'
            {
                res += 1;
            }
            // down-left
            if row as isize - 3 >= 0
                && col as isize - 3 >= 0
                && grid[row - 1][col - 1] == b'M'
                && grid[row - 2][col - 2] == b'A'
                && grid[row - 3][col - 3] == b'S'
            {
                res += 1;
            }
        }
    }
    res
}

fn part2(grid: &Vec<Vec<u8>>) -> usize {
    let mut res: usize = 0;

    for row in 0..grid.len() {
        for col in 0..grid[1].len() {
            if grid[row][col] != b'A' {
                continue;
            }
            if row == 0 || col == 0 || row == grid.len() - 1 || col == grid[1].len() - 1 {
                continue;
            }
            //diagonal
            if grid[row - 1][col - 1] == b'M'
                && grid[row + 1][col + 1] == b'S'
                && grid[row - 1][col + 1] == b'M'
                && grid[row + 1][col - 1] == b'S'
            {
                res += 1;
            }
            if grid[row - 1][col - 1] == b'M'
                && grid[row + 1][col + 1] == b'S'
                && grid[row - 1][col + 1] == b'S'
                && grid[row + 1][col - 1] == b'M'
            {
                res += 1;
            }
            if grid[row - 1][col - 1] == b'S'
                && grid[row + 1][col + 1] == b'M'
                && grid[row - 1][col + 1] == b'M'
                && grid[row + 1][col - 1] == b'S'
            {
                res += 1;
            }
            if grid[row - 1][col - 1] == b'S'
                && grid[row + 1][col + 1] == b'M'
                && grid[row - 1][col + 1] == b'S'
                && grid[row + 1][col - 1] == b'M'
            {
                res += 1;
            }
        }
    }
    res
}

fn main() -> std::io::Result<()> {
    let file: File = File::open("input.txt")?;
    let rb = BufReader::new(file);
    let lines = rb.lines();

    let mut grid: Vec<Vec<u8>> = Vec::new();
    lines.for_each(|line| grid.push(line.unwrap().as_bytes().to_vec()));

    println!("{}", part1(&grid));
    println!("{}", part2(&grid));

    Ok(())
}
