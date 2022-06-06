use aoc_harness::aoc_main;

aoc_main!(2020 day 6, part1 [p1, p1_binop], part2 [p2_binop]);

use std::collections::HashSet;

pub fn p1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| g.lines().flat_map(str::chars).collect::<HashSet<_>>().len())
        .sum()
}

pub fn solve<F>(input: &str, f: F) -> usize
where
    F: Fn(&HashSet<char>, &HashSet<char>) -> HashSet<char>,
{
    input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| l.chars().collect())
                .fold(None, |acc: Option<HashSet<char>>, s| {
                    Some(match acc {
                        None => s,
                        Some(x) => f(&x, &s),
                    })
                })
                .unwrap()
                .len()
        })
        .sum()
}
pub fn p1_binop(input: &str) -> usize {
    solve(input, |a, b| a | b)
}
pub fn p2_binop(input: &str) -> usize {
    solve(input, |a, b| a & b)
}
