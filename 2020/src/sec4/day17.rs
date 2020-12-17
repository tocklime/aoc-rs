use std::collections::{HashMap, HashSet};

use itertools::{repeat_n, Itertools};

use crate::utils::cartesian::as_point_map;

#[allow(clippy::needless_pass_by_value)] //I want a specific type signature here to fit in with the fold where it's used.
pub fn step(world: HashSet<Vec<i64>>, _step: usize) -> HashSet<Vec<i64>> {
    let mut counts: HashMap<Vec<i64>, usize> = HashMap::new();
    for p in &world {
        for n in (0..p.len()).map(|_| &[-1, 0, 1]).multi_cartesian_product() {
            let t = p.iter().zip(&n).map(|(a, &b)| a + b).collect();
            *counts.entry(t).or_default() += 1;
        }
    }
    //Rule in spec: (active && alive neighbours is 2 or 3) || (!active && alive neighbours == 3)
    //but we've calculated number of alive neighbours + self
    //so we use rule: (active && count is 3 or 4) || (!active && count is 3)
    counts
        .into_iter()
        .filter(|(k, c)| *c == 3 || *c == 4 && world.contains(k))
        .map(|(k, _)| k)
        .collect()
}

pub fn solve(input: &str, dimensions: usize) -> usize {
    let input = as_point_map(input, false);
    let world: HashSet<Vec<i64>> = input
        .iter()
        .filter(|(_, c)| **c == '#')
        .map(|(p, _)| {
            let mut p = vec![p.x, p.y];
            p.extend(repeat_n(0, dimensions - 2));
            p
        })
        .collect();

    (0..6).fold(world, step).len()
}

#[aoc(day17, part1)]
pub fn p1(input: &str) -> usize {
    solve(input, 3)
}

#[aoc(day17, part2)]
pub fn p2(input: &str) -> usize {
    solve(input, 4)
}
