use std::ops::IndexMut;

use aoc_harness::*;

use pathfinding::prelude::Grid;
use utils::grid2d::{Grid2d, ICoord};

aoc_main!(2021 day 20, part1 [p1::<2>] => 5786, part2 [p1::<50>] => 16757, example both EG => (35,3351));

const EG: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

struct Picture<'a> {
    rules: &'a [bool; 512],
    set: Grid2d<bool>,
    infinite_value: bool,
}
impl<'a> Picture<'a> {
    fn is_lit(&self, p: ICoord) -> bool {
        match self.set.get_i(p) {
            Some(x) => *x,
            None => self.infinite_value,
        }
    }
    fn lit_next_time(&self, p: ICoord) -> bool {
        //p is in the new base, not the old...
        let n: usize = (p.0 - 1..=p.0 + 1)
            .cartesian_product(p.1 - 1..=p.1 + 1)
            .map(|p| self.is_lit(p))
            .fold(0_usize, |acc, n| acc << 1 | usize::from(n));
        self.rules[n]
    }
    fn step_into(&self, target: &mut Self) {
        let (my, mx) = self.set.dim();
        let new_dim = (my + 2, mx + 2);
        target.set.grow_and_invalidate_all_data(new_dim, false);
        for ((y, x), v) in target.set.indexed_iter_mut() {
            *v = self.lit_next_time((
                isize::try_from(y).unwrap() - 1,
                isize::try_from(x).unwrap() - 1,
            ));
        }
        let new_inf_value = if self.infinite_value {
            self.rules[511]
        } else {
            self.rules[0]
        };
        target.infinite_value = new_inf_value;
    }
}
fn p1<const ITER: usize>(input: &str) -> usize {
    let mut map = [false; 512];
    let mut s = input.split("\n\n");
    s.next().unwrap().chars().enumerate().for_each(|(ix, c)| {
        map[ix] = c == '#';
    });
    let set = Grid2d::from_str(s.next().unwrap(), |x| x == '#');
    let mut a = Picture {
        rules: &map,
        set,
        infinite_value: false,
    };
    let mut b = Picture {
        rules: &map,
        set: Grid2d::from_elem((0, 0), false),
        infinite_value: false,
    };
    let ar = &mut a;
    let br = &mut b;
    for _ in 0..ITER {
        ar.step_into(br);
        std::mem::swap(ar, br);
    }
    ar.set.iter().filter(|&&x| x).count()
}
