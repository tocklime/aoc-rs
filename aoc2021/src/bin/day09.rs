use std::collections::BinaryHeap;

use aoc_harness::*;
use ndarray::{Array2, IntoDimension};
use utils::{aabb::Aabb, cartesian::Point};

aoc_main!(2021 day 9, generator gen, part1 [p1] => 633, part2 [p2] => 1050192,
          example part1 EG => 15, example part2 EG => 1134);
const EG: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
fn gen(input: &str) -> Array2<u8> {
    let input = input.trim();
    let wid = input.lines().next().unwrap().len();
    let hei = input.lines().count();
    let mut i = input.chars().filter(char::is_ascii_digit);
    Array2::from_shape_simple_fn((hei, wid), || (i.next().unwrap() as u32 - '0' as u32) as u8)
}
fn get_low_points(grid: &Array2<u8>) -> Vec<(usize, usize)> {
    let (hei, wid) = grid.dim();
    let bb = Aabb::new(Point::new(wid - 1, hei - 1)).extend(Point::new(0, 0));
    grid.indexed_iter()
        .filter(|&(p, &here)| {
            Point::from_dim(p)
                .neighbours()
                .iter()
                .all(|x| !bb.contains(x) || grid[x.into_dimension()] > here)
        })
        .map(|x| x.0)
        .collect()
}
fn neighbours(p: (usize, usize)) -> [(usize, usize); 4] {
    [
        (p.0.wrapping_sub(1), p.1),
        (p.0, p.1.wrapping_sub(1)),
        (p.0 + 1, p.1),
        (p.0, p.1 + 1),
    ]
}
fn p1(grid: &Array2<u8>) -> usize {
    get_low_points(grid)
        .into_iter()
        .map(|x| grid[x] as usize + 1)
        .sum()
}
fn p2(grid: &Array2<u8>) -> usize {
    let mut sizes = BinaryHeap::new();
    let mut done_map = Array2::from_elem(grid.dim(), false);
    for (p, &v) in grid.indexed_iter() {
        if done_map[p] || v == 9 {
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
            for n in neighbours(p) {
                if grid.get(n).unwrap_or(&9) < &9 {
                    fringe.push(n);
                }
            }
        }
        sizes.push(count)
    }
    sizes.iter().take(3).product()
}
