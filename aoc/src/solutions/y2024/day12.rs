use itertools::Itertools;

use utils::{cartesian::Point, grid2d::Grid2d};

aoc_harness::aoc_main!(2024 day 12, both [solve] => (1_387_004, 844_198), example both EG => (1930,1206));

fn solve(input: &str) -> (usize,usize) {
    let g = Grid2d::from_str_as_bytes(input);
    let mut todo_grid = Grid2d::from_elem(g.dim(), true);
    let mut p1 = 0;
    let mut p2 = 0;
    for (p, _x) in g.indexed_iter() {
        if todo_grid[p] {
            let mut area_count = 0;
            let mut fringe = vec![p];
            let mut fence_count = 0;
            let mut corner_count = 0;
            while let Some(c) = fringe.pop() {
                if todo_grid[c] {
                    let ns = g.neighbours_array_ordered(c);
                    for on in &ns {
                        match on {
                            &Some(n) if g[n] == g[c] => fringe.push(n),
                            _ => fence_count += 1,
                        }
                    }
                    let corners: usize = ns
                        .iter()
                        .circular_tuple_windows()
                        .map(|(a, b)| {
                            let a_diff = a.and_then(|x| g.get(x)) != Some(&g[c]);
                            let b_diff = b.and_then(|x| g.get(x)) != Some(&g[c]);
                            let outer = usize::from(a_diff && b_diff);
                            let inner = if !a_diff && !b_diff {
                                //a and b are the same - is this an internal corner?
                                match (a, b) {
                                    (&Some(x), &Some(y)) => {
                                        let diag_x = if x.x == c.x { y.x } else { x.x };
                                        let diag_y = if x.y == c.y { y.y } else { x.y };
                                        let diag = Point::new(diag_x, diag_y);
                                        //is internal corner if the diagonal is not same.
                                        usize::from(g.get(diag) != Some(&g[c]))
                                    }
                                    _ => 0,
                                }
                            } else {
                                0
                            };
                            outer + inner
                        })
                        .sum();
                    corner_count += corners;
                    todo_grid[c] = false;
                    area_count += 1;
                }
            }
            p1 += area_count * fence_count;
            p2 += area_count * corner_count;
        }
    }
    (p1,p2)
}

const EG: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
