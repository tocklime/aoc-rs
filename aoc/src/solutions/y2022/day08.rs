use aoc_harness::*;
use utils::grid2d::Grid2d;

aoc_main!(2022 day 8, generator gen, part1 [p1] => 1835, part2 [p2] => 263670, example both EG => (21,8));

const EG: &str = "30373
25512
65332
33549
35390
";
fn is_visible_in_direction(grid: &Grid2d<char>, p: (usize, usize), dir: (isize, isize)) -> bool {
    //is there no taller tree in direction d until grid edge?
    let my_height = grid[p];
    grid.values_in_direction(p, dir)
        .skip(1)
        .all(|&h| h < my_height)
}
fn visible_tree_count(grid: &Grid2d<char>, p: (usize, usize), dir: (isize, isize)) -> usize {
    let my_height = grid[p];
    let mut count = 0;
    for c in grid.values_in_direction(p, dir).skip(1) {
        count += 1;
        if c >= &my_height {
            break;
        }
    }
    count
}
const ALL_DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
fn gen(input: &str) ->Grid2d<char> {
    Grid2d::from_str(input, |c| c)
}
fn p1(grid: &Grid2d<char>) -> usize {
    grid.indexes().filter(|p| {
        ALL_DIRS
            .iter()
            .any(|&d| is_visible_in_direction(grid, *p, d))
    }).count()
}
fn p2(grid: &Grid2d<char>) -> usize {
    grid.indexes().map(|p| {
        ALL_DIRS
            .iter()
            .map(|&d| visible_tree_count(grid, p, d))
            .product()
    }).max().unwrap()
}
