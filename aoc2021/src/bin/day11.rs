use aoc_harness::*;
use utils::grid2d::Grid2d;

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

fn incr(n: &mut u8) -> bool {
    *n = (*n + 1) % 10;
    *n == 0
}
fn step(grid: &mut Grid2d<u8>) -> usize {
    let mut flashing = grid.indexes().filter(|&x| incr(&mut grid[x])).collect_vec();
    let mut flash_count = 0;
    while let Some(&p) = flashing.get(flash_count) {
        flash_count += 1;
        flashing.extend(
            grid.neighbours_with_diagonals(p)
                .filter(|&n| grid[n] > 0 && incr(&mut grid[n])),
        );
    }
    flash_count
}
#[allow(clippy::maybe_infinite_iter)]
fn both(input: &str) -> (usize, usize) {
    let mut g = Grid2d::from_str(input, |c| u8::try_from(c as u32).unwrap() - b'0');
    let p1 = (0..100).fold(0, |c, _| c + step(&mut g));
    let p2 = (101..).find(|_| step(&mut g) == g.len()).unwrap();
    (p1, p2)
}
