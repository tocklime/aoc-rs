use aoc_harness::*;
use utils::grid2d::{Coord, Grid2d};

aoc_main!(2021 day 11,  both [both] => (1675,515), example both EG => (1656,195));

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

struct Day11 {
    grid: Grid2d<u8>,
    flashing: Vec<Coord>,
}
fn incr(n: &mut u8) -> bool {
    *n = (*n + 1) % 10;
    *n == 0
}
impl Day11 {
    fn step(&mut self) -> usize {
        self.flashing.clear();
        self.flashing
            .extend(self.grid.indexes().filter(|&x| incr(&mut self.grid[x])));
        let mut flash_count = 0;
        while let Some(&p) = self.flashing.get(flash_count) {
            flash_count += 1;
            self.flashing.extend(
                self.grid
                    .neighbours_with_diagonals(p)
                    .filter(|&p| self.grid[p] > 0 && incr(&mut self.grid[p])),
            );
        }
        flash_count
    }
    fn new(input: &str) -> Self {
        let grid = Grid2d::from_str(input, |c| u8::try_from(c as u32).unwrap() - b'0');
        Self {
            grid,
            flashing: Vec::with_capacity(100),
        }
    }
}

#[allow(clippy::maybe_infinite_iter)]
fn both(input: &str) -> (usize, usize) {
    let mut g = Day11::new(input);
    let p1 = (0..100).fold(0, |c, _| c + g.step());
    let p2 = (101..).find(|_| g.step() == g.grid.len()).unwrap();
    (p1, p2)
}
