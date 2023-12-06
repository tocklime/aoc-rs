use std::collections::BTreeMap;


use utils::grid2d::Grid2d;

aoc_harness::aoc_main!(2022 day 8, generator gen, part1 [p1,p1a] => 1835, part2 [p2, p2a] => 263_670, example both EG => (21,8));

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
        .all(|(_, &h)| h < my_height)
}
fn visible_tree_count(grid: &Grid2d<char>, p: (usize, usize), dir: (isize, isize)) -> usize {
    let my_height = grid[p];
    let mut count = 0;
    for (_, c) in grid.values_in_direction(p, dir).skip(1) {
        count += 1;
        if c >= &my_height {
            break;
        }
    }
    count
}
const ALL_DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
fn gen(input: &str) -> Grid2d<char> {
    Grid2d::from_str(input, |c| c)
}
fn p1(grid: &Grid2d<char>) -> usize {
    grid.indexes()
        .filter(|p| {
            ALL_DIRS
                .iter()
                .any(|&d| is_visible_in_direction(grid, *p, d))
        })
        .count()
}
fn p1a(grid: &Grid2d<char>) -> usize {
    let (h, w) = grid.dim();
    let mut out_grid = Grid2d::from_elem((h, w), false);

    //from top and bottom
    for x in 0..w {
        let line = grid.values_in_direction((0, x), (1, 0));
        for r in utils::iter::all_new_greatest_with(line, |&(_, &c)| c) {
            out_grid[r.0] = true;
        }
        let line = grid.values_in_direction((h - 1, x), (-1, 0));
        for r in utils::iter::all_new_greatest_with(line, |&(_, &c)| c) {
            out_grid[r.0] = true;
        }
    }
    //from left and right
    for y in 0..h {
        let row = grid.get_row(y);
        for r in utils::iter::all_new_greatest_with(row.iter().enumerate(), |(_, c)| **c) {
            out_grid[(y, r.0)] = true;
        }
        for r in utils::iter::all_new_greatest_with(row.iter().enumerate().rev(), |(_, c)| **c) {
            out_grid[(y, r.0)] = true;
        }
    }
    out_grid.iter().filter(|x| **x).count()
}

fn p2(grid: &Grid2d<char>) -> usize {
    grid.indexes()
        .map(|p| {
            ALL_DIRS
                .iter()
                .map(|&d| visible_tree_count(grid, p, d))
                .product()
        })
        .max()
        .unwrap()
}
/// TODO: try pruning the maps when you see a '9'.
fn p2a(grid: &Grid2d<char>) -> usize {
    let (h, w) = grid.dim();
    let mut product_grid = Grid2d::from_elem((h, w), 1);
    //from left and right.
    // let mut scratch = vec![0; w];
    let mut map: BTreeMap<char, usize> = BTreeMap::new();
    for y in 0..h {
        let row = grid.get_row(y);
        for (ix, &c) in row.iter().enumerate() {
            //find max index in map where height >= c.
            let view_score = ix - map.range(c..).map(|x| *x.1).max().unwrap_or(0);
            product_grid[(y, ix)] *= view_score;
            map.insert(c, ix);
        }
        // println!("{}", product_grid);
        map.clear();
        for (ix, &c) in row.iter().enumerate().rev() {
            let view_score = map.range(c..).map(|x| *x.1).min().unwrap_or(w - 1) - ix;
            product_grid[(y, ix)] *= view_score;
            map.insert(c, ix);
        }
        map.clear();
        // println!("{}", product_grid);
    }
    for x in 0..w {
        let line = grid.values_in_direction((0, x), (1, 0));
        for (pos, &c) in line {
            let view_score = pos.0 - map.range(c..).map(|x| *x.1).max().unwrap_or(0);
            product_grid[pos] *= view_score;
            map.insert(c, pos.0);
        }
        map.clear();
        let line = grid.values_in_direction((h - 1, x), (-1, 0));
        for (pos, &c) in line {
            let view_score = map.range(c..).map(|x| *x.1).min().unwrap_or(h - 1) - pos.0;
            product_grid[pos] *= view_score;
            map.insert(c, pos.0);
        }
        map.clear();
    }
    *product_grid.iter().max().unwrap()
}
