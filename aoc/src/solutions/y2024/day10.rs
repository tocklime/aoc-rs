use std::collections::HashSet;

use utils::{cartesian::Point, grid2d::Grid2d};

aoc_harness::aoc_main!(2024 day 10,
    generator Grid2d::from_str_as_bytes,
    part1 [set_fringe] => 811, part2 [list_fringe] => 1794,
    example both EG => (36,81));

macro_rules! solve {
    ($func_name:ident, $ty:ty) => {
        fn $func_name(g: &Grid2d<u8>) -> usize {
            g.indexed_iter()
                .filter(|x| x.1 == &b'0')
                .map(|(trailhead, _)| {
                    let set: $ty = [trailhead].into_iter().collect();
                    (b'1'..=b'9')
                        .fold(set, |set, h| {
                            set.into_iter()
                                .flat_map(|x| g.neighbours(x).filter(|&n| g[n] == h))
                                .collect()
                        })
                        .len()
                })
                .sum()
        }
    };
}

solve!(set_fringe, HashSet<Point<usize>>);
solve!(list_fringe, Vec<Point<usize>>);

const EG: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
