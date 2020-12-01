#![warn(clippy::all)]
use std::collections::HashSet;

use itertools::Itertools;
#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .map(|l| l.parse().expect("Integer"))
        .collect()
}
pub fn solve(input: &[i64], size: usize) -> i64 {
    input
        .iter()
        .combinations(size)
        .find(|x| x.iter().copied().sum::<i64>() == 2020)
        .unwrap()
        .into_iter()
        .product()
}

#[aoc(day1, part1, brute)]
pub fn p1(input: &[i64]) -> i64 {
    solve(input, 2)
}

#[aoc(day1, part2, brute)]
pub fn p2(input: &[i64]) -> i64 {
    solve(input, 3)
}

#[aoc(day1, part1, sets)]
pub fn p1_sets(input: &[i64]) -> i64 {
    let s: HashSet<i64> = input.iter().cloned().collect();
    let n = *s.iter().find(|&&x| s.contains(&(2020 - x))).unwrap();
    n * (2020 - n)
}

#[aoc(day1, part2, sets)]
pub fn p2_sets(input: &[i64]) -> i64 {
    let s: HashSet<i64> = input.iter().cloned().collect();
    let c = s
        .iter()
        .combinations(2)
        .map(|x| (x.iter().copied().sum::<i64>(), x))
        .find(|(sum, _)| s.contains(&(2020 - *sum)))
        .unwrap();
    c.1.iter().copied().product::<i64>() * (2020 - c.0)
}
