use itertools::Itertools;
use utils::grid2d::Grid2d;

aoc_harness::aoc_main!(2023 day 13, part1 [solve::<0>] => 36041, part2[solve::<1>] => 35915, example both EG => (405,400));

/// Returns the number of errors in the reflection about a given row.
fn mirror_before_row(grid: &Grid2d<char>, row_split: usize) -> usize {
    let gs = grid.dim();
    let y_height = row_split.min(gs.y - row_split);
    (0..y_height)
        .cartesian_product(0..gs.x)
        .filter(|&(y, x)| grid[(row_split + y, x)] != grid[(row_split - (y + 1), x)])
        .count()
}

/// Returns the number of errors in the reflection about a given column.
fn mirror_before_col(grid: &Grid2d<char>, col_split: usize) -> usize {
    let gs = grid.dim();
    let x_height = col_split.min(gs.x - col_split);
    (0..gs.y)
        .cartesian_product(0..x_height)
        .filter(|&(y, x)| grid[(y, x + col_split)] != grid[(y, col_split - (x + 1))])
        .count()
}

fn solve<const ERROR_COUNT: usize>(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|l| {
            let g = Grid2d::from_str(l, |a| a);
            let s = g.dim();

            if let Some(y) = (1..s.y).find(|&x| mirror_before_row(&g, x) == ERROR_COUNT) {
                return y * 100;
            }
            if let Some(x) = (1..s.x).find(|&x| mirror_before_col(&g, x) == ERROR_COUNT) {
                return x;
            }
            panic!("Can't find symmetry in {g}");
        })
        .sum()
}

const EG: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
