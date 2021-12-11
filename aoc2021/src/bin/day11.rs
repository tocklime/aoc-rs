use std::collections::HashSet;

use aoc_harness::*;
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
    let mut flashed_this_turn = Grid2d::from_elem(grid.dim(), false);

    for p in grid.indexes().collect_vec() {
        if incr(grid, p) {
            flashing_this_turn.push(p);
        }
    }
    while let Some(p) = flashing_this_turn.pop() {
        if !flashed_this_turn[p] {
            flashed_this_turn[p] = true;
            for n in grid.neighbours_with_diagonals(p).collect_vec() {
                if incr(grid, n) {
                    flashing_this_turn.push(n);
                }
            }
        }
    }
    flashed_this_turn
        .indexed_iter()
        .filter(|&(p, &v)| {
            if v {
                grid[p] = 0;
            }
            v
        })
        .count()
}
fn p1(input: &Grid2d<u8>) -> usize {
    let mut g = input.clone();
    (0..100).fold(0, |c, _| c + step(&mut g))
}

fn p2(input: &Grid2d<u8>) -> usize {
    let mut t = input.clone();
    (1..).find(|_| step(&mut t) == t.len()).unwrap()
}
