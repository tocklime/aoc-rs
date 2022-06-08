#![allow(clippy::redundant_pattern_matching)]

use reformation::Reformation;
use itertools::Itertools;

#[derive(Reformation, Debug)]
enum Instr {
    #[reformation(r"rect {}x{}")]
    Rect(usize,usize),
    #[reformation(r"rotate row y={} by {}")]
    RotateRow(usize,usize),
    #[reformation(r"rotate column x={} by {}")]
    RotateCol(usize,usize)
}

const WIDTH:usize = 50;
const HEIGHT:usize = 6;
fn screen(input: &str) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; WIDTH]; HEIGHT];
    for i in input.lines().map(|x| Instr::parse(x).unwrap()) {
        match i {
            Instr::Rect(wid, height) =>
                (0..wid).cartesian_product(0..height).for_each(|(x, y)| grid[y][x] = true),
            Instr::RotateRow(row, dist) =>
                {
                    let before = grid[row].clone();
                    for i in 0..WIDTH {
                        grid[row][i] = before[(i + WIDTH - (dist % WIDTH)) % WIDTH]
                    }
                }
            Instr::RotateCol(col, dist) =>
                {
                    let before = grid.iter().map(|row| row[col]).collect_vec();
                    for i in 0..HEIGHT {
                        grid[i][col] = before[(i + HEIGHT - (dist % HEIGHT)) % HEIGHT]
                    }
                }
        }
    }
    grid
}

#[aoc(day8,part1)]

fn p1(input: &str) -> usize {
    screen(input).iter().flat_map(|x| x.iter()).filter(|&&x| x).count()
}
#[aoc(day8,part2)]
fn p2(input: &str) -> String {
    let grid = screen(input);
    let lines : Vec<String> = grid.iter()
        .map(|l|
            format!("\n{}",l.iter().map(|&c| if c {'#'} else {' '}).collect::<String>())
        ).collect();
    lines.join("")
}
