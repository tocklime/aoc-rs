use std::collections::HashSet;

use aoc_harness::*;
use utils::{aabb::Aabb, cartesian::Point};

aoc_main!(2021 day 20, part1 [p1::<2>], part2 [p1::<50>], example part1 EG => 35);

const EG: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

fn step(map: &[bool; 512], lit: &HashSet<Point<isize>>, outside: bool) -> HashSet<Point<isize>> {
    let mut ans = HashSet::new();
    let bb1: Aabb<isize> = lit.iter().collect();
    let mut bb2 = bb1;
    bb2.top_right += Point::new(1, 1);
    bb2.bottom_left += Point::new(-1, -1);
    for p in bb2.all_points() {
        let ns = p.neighbours_and_self_with_diagonals_in_order();
        let bools = ns
            .iter()
            .map(|n| lit.contains(n) || (outside && !bb1.contains(n)))
            .collect_vec();
        let n = bools
            .iter()
            .fold(0_usize, |acc, n| acc << 1 | usize::from(*n));
        // dbg!(p, bools, n);
        let is_set = map[n];
        if is_set {
            ans.insert(p);
        }
    }
    ans
}
fn p1<const ITER: usize>(input: &str) -> usize {
    let mut map = [false; 512];
    let mut s = input.split("\n\n");
    s.next().unwrap().chars().enumerate().for_each(|(ix, c)| {
        map[ix] = c == '#';
    });
    let char_map = utils::cartesian::as_point_map::<isize>(s.next().unwrap(), true);
    let mut lit_set: HashSet<Point<isize>> = char_map
        .into_iter()
        .filter(|x| x.1 == '#')
        .map(|x| x.0)
        .collect();
    let flip_hack = map[0];
    for s in 0..ITER {
        lit_set = step(&map, &lit_set, flip_hack && s % 2 == 1);
    }
    lit_set.len()
}
//10410: wrong.
//5960 wrong
