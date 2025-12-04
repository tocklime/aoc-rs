use rustc_hash::FxHashSet;
use utils::{cartesian::Point, grid2d::{Coord, Grid2d}};

aoc_harness::aoc_main!(2025 day 4, part1 [p1] => 1351, part2 [p2, p2_fast] => 8345, both [by_counts], example part1 EG => 13, example part2 EG => 43);

fn is_removable(g: &Grid2d<char>, p: Coord) -> bool {
    let Some(c) = g.get(p) else {return false;};
    c == &'@'
        && g.neighbours_with_diagonals(p)
            .filter(|n| g[*n] == '@')
            .count()
            < 4
}

fn p1(input: &str) -> usize {
    let g = Grid2d::from_str(input, |x| x);
    g.indexes().filter(|p| is_removable(&g, *p)).count()
}

fn p2(input: &str) -> usize {
    let mut g = Grid2d::from_str(input, |x| x);
    let mut removed = 0;
    loop {
        let to_remove = g
            .indexes()
            .filter(|p| is_removable(&g, *p))
            .collect::<Vec<_>>();
        if to_remove.is_empty() {
            return removed;
        }
        removed += to_remove.len();
        for r in to_remove {
            g[r] = '.';
        }
    }
}

fn p2_fast(input: &str) -> usize {
    let mut g = Grid2d::from_str(input, |x| x);
    let mut removed = 0;
    let mut to_remove : FxHashSet<Point<usize>> = g
        .indexes()
        .filter(|p| is_removable(&g, *p))
        .collect();
    while !to_remove.is_empty() {
        removed += to_remove.len();
        for r in &to_remove {
            g[*r] = '.';
        }
        to_remove = to_remove
            .iter()
            .flat_map(Point::neighbours_with_diagonals)
            .filter(|p| is_removable(&g, *p))
            .collect();
    }
    removed
}

/// In this solution to part 2 we represent the grid as a grid of numbers.
/// If the cell has a roll in it, the number is how many rolls of paper there are in this cell and all 8 neighbours.
/// If the cell is empty, the value in the cell is 0, regardless of neighbours.
/// 
/// As the grid is constructed, we note any removable cells in `to_remove`.
/// 
/// On each iteration of the loop, we remove the to_remove nodes, and as we do each one,
/// decrement the counts (saturating sub) of all neighbours. If we cause a neighbour to have value `4` 
/// (that is, cell has a roll, and it has precisely 3 neighbours), then we add it to `new_to_remove`.
/// 
/// This way, we scan the whole grid once, during construction, and from then only consider 
/// neighbours of removed nodes.
fn by_counts(input: &str) -> (usize,usize) {
    let g = Grid2d::from_str(input, |x| x);
    let mut to_remove = FxHashSet::default();
    let mut g2 = Grid2d::from_fn(g.dim(), |p| {
        if g[p] == '@' {
            let count = 1 + g.neighbours_with_diagonals(p).filter(|n| g[*n] == '@').count() as u8;
            if count < 5 {
                to_remove.insert(p);
            }
            count
        } else {
            0
        }
    });
    let mut removed = 0;
    let p1 = to_remove.len();
    while !to_remove.is_empty() {
        removed += to_remove.len();
        let mut new_to_remove = FxHashSet::default();
        for r in to_remove {
            g2[r] = 0;
            for n in g.neighbours_with_diagonals(r) {
                g2[n] = g2[n].saturating_sub(1);
                if 4 == g2[n] {
                    new_to_remove.insert(n);
                }
            }
        }
        to_remove = new_to_remove;
    }
    (p1, removed)
}

const EG: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
