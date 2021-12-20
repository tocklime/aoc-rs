use std::collections::{HashMap, HashSet};

use aoc_harness::*;
use utils::{aabb::Aabb, cartesian::Point, nums::NumBitExt};

aoc_main!(2021 day 20, part1 [p1::<2>] => 5786, part2 [p1::<50>] => 16757, example both EG => (35,3351));

const EG: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

struct Picture<'a> {
    rules: &'a [bool; 512],
    set: HashSet<Point<isize>>,
    infinite_value: bool,
    bb: Aabb<isize>,
}
impl<'a> Picture<'a> {
    fn is_lit(&self, p: &Point<isize>) -> bool {
        if self.bb.contains(p) {
            self.set.contains(p)
        } else {
            self.infinite_value
        }
    }
    fn step1(&self) -> Self {
        let mut bb = self.bb;
        let new_inf_value = if self.infinite_value {
            self.rules[511]
        } else {
            self.rules[0]
        };
        bb.top_right += Point::new(1, 1);
        bb.bottom_left += Point::new(-1, -1);
        let set = bb
            .all_points()
            .filter(|p| {
                let n = p
                    .neighbours_and_self_with_diagonals_in_order()
                    .iter()
                    .map(|n| self.is_lit(n))
                    .fold(0_usize, |acc, n| acc << 1 | usize::from(n));
                // dbg!(p, bools, n);
                self.rules[n]
            })
            .collect();
        Self {
            rules: self.rules,
            set,
            infinite_value: new_inf_value,
            bb,
        }
    }
}
fn p1<const ITER: usize>(input: &str) -> usize {
    let mut map = [false; 512];
    let mut s = input.split("\n\n");
    s.next().unwrap().chars().enumerate().for_each(|(ix, c)| {
        map[ix] = c == '#';
    });
    let char_map = utils::cartesian::as_point_map::<isize>(s.next().unwrap(), false);
    let lit_set: HashSet<Point<isize>> = char_map
        .into_iter()
        .filter(|x| x.1 == '#')
        .map(|x| x.0)
        .collect();
    let bb: Aabb<isize> = lit_set.iter().collect();
    let mut p = Picture {
        rules: &map,
        set: lit_set,
        infinite_value: false,
        bb,
    };
    for _ in 0..ITER {
        // println!("{}", utils::cartesian::render_set_w(&p.set, '#', '.', true));
        p = p.step1();
    }
    p.set.len()
}
