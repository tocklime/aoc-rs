use std::collections::HashMap;

use aoc_harness::*;
use utils::{grid2d::Grid2d, aabb::Aabb, cartesian::Point};

aoc_main!(2023 day 3, generator gen, part1 [p1] => 527364, part2 [p2] => 79026871, example both EG => (4361, 467835));

#[derive(Debug)]
struct FoundNum {
    value: u32,
    row: usize,
    col_start: usize,
    col_end: usize,
}
impl FoundNum {
    fn search_box(&self) -> Aabb<usize> {
        let bottom_left = Point::new(self.col_start.saturating_sub(1), self.row.saturating_sub(1));
        let top_right = Point::new(self.col_end+1, self.row +1);
        Aabb {
            bottom_left, top_right
        }
    }
}

fn gen(input: &str) -> Grid2d<char> {
    Grid2d::from_str(input, |x| x)
}

fn find_numbers(g: &Grid2d<char>) -> impl Iterator<Item = FoundNum> + '_ {
    let mut curr: Option<FoundNum> = None;
    let width = g.dim().1 - 1;
    g.indexed_iter()
        //Find all the nums.
        .filter_map(move |((row, col), c)| {
            if let Some(d) = c.to_digit(10) {
                if let Some(n) = curr.as_mut() {
                    n.value = n.value * 10 + d;
                    n.col_end = col;
                } else {
                    curr = Some(FoundNum {
                        value: d,
                        row,
                        col_start: col,
                        col_end: col,
                    });
                }
            }
            if col == width || !c.is_ascii_digit() {
                curr.take()
            } else {
                None
            }
        })
}

fn p1(g: &Grid2d<char>) -> u32 {
    find_numbers(g)
        //filter to just those that are adjacent to symbols
        .filter(|f| {
            f.search_box().all_points().any(|p| {
                g.get((p.y,p.x)).map(|s| s != &'.' && !s.is_ascii_digit()).unwrap_or_default()
            })
        })
        //add them up.
        .map(|x| x.value)
        .sum()
}

const EG: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

fn p2(g: &Grid2d<char>) -> u32 {
    let mut gears: HashMap<(usize, usize), Vec<u32>> = g
        .indexed_iter()
        .filter_map(|((row, col), c)| (c == &'*').then_some(((row, col), Vec::new())))
        .collect();
    for f in find_numbers(g) {
        // Add into the gears object.
        for p in f.search_box().all_points() {
            if let Some(s) = gears.get_mut(&(p.y, p.x)) {
                s.push(f.value);
            }
        }
    }
    gears
        .values()
        .filter_map(|x| (x.len() == 2).then_some(x.iter().product::<u32>()))
        .sum()
}
