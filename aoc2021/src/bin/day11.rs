use std::{collections::HashSet, str::FromStr};

use aoc_harness::*;
use pathfinding::prelude::Grid;
use utils::grid2d::Grid2d;

aoc_main!(2021 day 11, generator gen, part1 [p1], part2 [p2],
          example part1 EG => 1656, example part2 EG => 195);

const EG: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

fn gen(input: &str) -> Grid2d<u8> {
    Grid2d::from_str(input, |c| ((c as u32) as u8) - b'0')
}
fn incr(grid: &mut Grid2d<u8>, pos: (usize, usize)) -> bool {
    grid[pos] += 1;
    grid[pos] > 9
}
fn step(grid: &mut Grid2d<u8>) -> usize {
    let mut flashing_this_turn = Vec::new();
    let mut flashed_this_turn = HashSet::new();

    for p in grid.indexes().collect_vec() {
        if incr(grid, p) {
            flashing_this_turn.push(p);
        }
    }
    while let Some(p) = flashing_this_turn.pop() {
        if !flashed_this_turn.contains(&p) {
            flashed_this_turn.insert(p);
            for n in grid.neighbours_with_diagonals(p).collect_vec() {
                if incr(grid, n) {
                    flashing_this_turn.push(n);
                }
            }
        }
    }
    let ans = flashed_this_turn.len();
    for x in flashed_this_turn {
        grid[x] = 0;
    }
    ans
}
fn p1(input: &Grid2d<u8>) -> usize {
    let mut t = input.clone();
    let a = step(&mut t);
    let b = step(&mut t);
    dbg!(a, b, t);
    let mut g = input.clone();
    (0..100).fold(0, |c, _| c + step(&mut g))
}

fn p2(input: &Grid2d<u8>) -> usize {
    let mut t = input.clone();
    for s in 0.. {
        let f = step(&mut t);
        if f == t.len() {
            return s + 1;
        }
    }
    0
}
