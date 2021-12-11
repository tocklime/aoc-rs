use std::collections::BinaryHeap;

use aoc_harness::*;
use utils::grid2d::Grid2d;

aoc_main!(2021 day 9, generator gen, part1 [p1] => 633, part2 [p2] => 1050192,
          example part1 EG => 15, example part2 EG => 1134);
const EG: &str = "
2199943210
3987894921
9856789892
8767896789
9899965678";

fn gen(input: &str) -> Grid2d<u8> {
    Grid2d::from_str(input.trim(), |c| (c as u8) - b'0')
}
fn p1(grid: &Grid2d<u8>) -> usize {
    grid.indexed_iter()
        .filter(|&(p, &here)| grid.neighbours(p).all(|p| grid[p] > here))
        .map(|x| *(x.1) as usize + 1)
        .sum()
}
fn p2(grid: &Grid2d<u8>) -> usize {
    let mut done_map = Grid2d::from_elem(grid.dim(), false);
    let mut sizes = BinaryHeap::new();
    for (p, v) in grid.indexed_iter() {
        if done_map[p] || *v == 9 {
            continue;
        }
        //flood from here.
        let mut fringe = vec![p];
        let mut count = 0;
        while let Some(p) = fringe.pop() {
            if done_map[p] {
                continue;
            }
            done_map[p] = true;
            count += 1;
            fringe.extend(grid.neighbours(p).filter(|p| grid[*p] < 9));
        }
        sizes.push(count)
    }
    sizes.iter().take(3).product()
}
