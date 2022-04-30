use std::rc::Rc;

use aoc_harness::*;

use utils::grid2d::{Grid2d, ICoord};

aoc_main!(2021 day 20, generator gen, part1 [solve::<1>] => 5786, part2 [solve::<25>] => 16757, example both EG => (35,3351));

const EG: &str = "
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#.
.#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..
#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....
#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####
.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.
#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..
#.##.#....##..#.####....##...##..#...#......#.#.......#.......##
..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

#[derive(Clone)]
struct Picture {
    rules: Rc<[bool; 512]>,
    grid: Grid2d<bool>,
    infinite_value: bool,
}
impl Picture {
    fn is_lit(&self, p: ICoord) -> bool {
        self.grid.get_i(p).copied().unwrap_or(self.infinite_value)
    }
    fn lit_next_time(&self, p: ICoord) -> bool {
        let n: usize = (p.0 - 1..=p.0 + 1)
            .cartesian_product(p.1 - 1..=p.1 + 1)
            .map(|p| self.is_lit(p))
            .fold(0_usize, |acc, n| acc << 1 | usize::from(n));
        self.rules[n]
    }
    fn step_into(&self, target: &mut Self) {
        let (my, mx) = self.grid.dim();
        let new_dim = (my + 2, mx + 2);
        target.grid.grow_and_invalidate_all_data(new_dim, false);
        for ((y, x), v) in target.grid.indexed_iter_mut() {
            *v = self.lit_next_time((
                isize::try_from(y).unwrap() - 1,
                isize::try_from(x).unwrap() - 1,
            ));
        }
        target.infinite_value = if self.infinite_value {
            self.rules[0b1_1111_1111]
        } else {
            self.rules[0]
        };
    }
}
fn gen(input: &str) -> Picture {
    let mut map = [false; 512];
    let mut s = input.split("\n\n");
    s.next()
        .unwrap()
        .chars()
        .filter(|&x| !char::is_whitespace(x))
        .enumerate()
        .for_each(|(ix, c)| {
            map[ix] = c == '#';
        });
    let set = Grid2d::from_str(s.next().unwrap(), |x| x == '#');
    Picture {
        rules: Rc::new(map),
        grid: set,
        infinite_value: false,
    }
}
fn solve<const ITER: usize>(input: &Picture) -> usize {
    let mut a = input.clone();
    let mut b = input.clone();
    for _ in 0..ITER {
        a.step_into(&mut b);
        b.step_into(&mut a);
    }
    a.grid.iter().filter(|&&x| x).count()
}
