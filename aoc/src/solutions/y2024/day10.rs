use std::collections::HashSet;

use utils::{cartesian::Point, grid2d::Grid2d};

aoc_harness::aoc_main!(2024 day 10,
    generator Grid2d::from_str_as_bytes,
    part1 [set_fringe] => 811, part2 [list_fringe] => 1794,
    example both EG => (36,81));

macro_rules! solve {
    ($func_name:ident, $ty:ty) => {
        fn $func_name(g: &Grid2d<u8>) -> usize {
            //Loop through all grid positions
            g.indexed_iter()
                // ...only want the trailheads with height '0'.
                .filter(|x| x.1 == &b'0')
                // ...foreach trailhead,
                .map(|(trailhead, _)| {
                    // start with the fringe as just the start point.
                    let fringe: $ty = [trailhead].into_iter().collect();
                    // iterate through heights 1 through 9, and replace the
                    // fringe with the set of all neighbours which have the correct height.
                    (b'1'..=b'9')
                        .fold(fringe, |fringe, h| {
                            fringe.into_iter()
                                .flat_map(|x| g.neighbours(x).filter(|&n| g[n] == h))
                                .collect()
                        })
                        // give the length of the final set (reachable '9's)
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
