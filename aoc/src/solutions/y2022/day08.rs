use std::collections::HashMap;

use aoc_harness::*;
use utils::cartesian::{as_point_map, render_char_map_w, Point, Dir};

aoc_main!(2022 day 8, part1 [p1] => 1835, part2 [p2] => 263670, example both EG => (21,8));

const EG: &str = "30373
25512
65332
33549
35390
";
fn try_see(grid: &HashMap<Point<usize>, char>, p: Point::<usize>, dir: Dir) -> bool {
    //is there no taller tree in direction d until grid edge?
    let mut c = p.step(dir);
    let my_height = grid[&p];
    while let Some(t) = grid.get(&c) {
        if t >= &my_height {
            return false;
        }
        c = c.step(dir);
    }
    true
}
fn scenic_score_dir(grid: &HashMap<Point<usize>, char>, p: Point::<usize>, dir: Dir) -> usize {
    let mut c = p.step(dir);
    let my_height = grid[&p];
    let mut count = 0;
    while let Some(t) = grid.get(&c) {
        count +=1;
        if t >= &my_height {
            break;
        }
        c = c.step(dir);
    }
    count
}
fn p1(input: &str) -> usize {
    let grid = as_point_map::<usize>(input, false);
    let r = render_char_map_w(&grid, 1, ".", false);
    let mut count = 0;
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let g2 = grid.clone();
    let mut vismap = HashMap::new();
    for (p, t) in g2 {
        let vis = [Dir::Up, Dir::Down, Dir::Left, Dir::Right].iter().any(|&d| try_see(&grid, p, d));
        if vis {
            vismap.insert(p, t);
            count += 1;
        }
    }
    count
}

fn p2(input: &str) -> usize {
    let grid = as_point_map::<usize>(input, false);
    let mut count = 0;
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let g2 = grid.clone();
    let mut best = 0;
    for (p, t) in g2 {
        let vis = [Dir::Up, Dir::Down, Dir::Left, Dir::Right].iter().map(|&d| scenic_score_dir(&grid, p, d)).collect_vec();
        let prod = vis.iter().product();
        if prod > best {
            best = prod;
        }
    }
    best
}
