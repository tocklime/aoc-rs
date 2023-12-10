use std::rc::Rc;

use utils::{grid2d::{Grid2d, ICoord}, cartesian::Point};

aoc_harness::aoc_main!(2021 day 20, generator gen, part1 [solve::<1>] => 5786, part2 [solve::<25>] => 16757, example both EG => (35,3351));

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
        let n: usize = p.neighbours_and_self_with_diagonals_in_order()
            .map(|p| self.is_lit(p))
            .into_iter()
            .fold(0_usize, |acc, n| acc << 1 | usize::from(n));
        self.rules[n]
    }
    fn step_into(&self, target: &mut Self) {
        let m = self.grid.dim();
        let new_dim = (m.y + 2, m.x + 2).into();
        target.grid.grow_and_invalidate_all_data(new_dim, false);
        for (Point{y, x}, v) in target.grid.indexed_iter_mut() {
            *v = self.lit_next_time((
                isize::try_from(y).unwrap() - 1,
                isize::try_from(x).unwrap() - 1,
            ).into());
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
