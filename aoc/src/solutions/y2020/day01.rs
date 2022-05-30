use std::collections::HashSet;
use itertools::Itertools;

use aoc_harness::aoc_main;

aoc_main!(2020 day 1, generator gen, part1 [p1, p1_sets] => 802_011, part2 [p2,p2_sets] => 248607374);

use utils::inputs::parse_input_from_str_sep_by;
pub fn gen(input: &str) -> Vec<i64> {
    parse_input_from_str_sep_by(input, "\n")
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

pub fn p1(input: &[i64]) -> i64 {
    solve(input, 2)
}

pub fn p2(input: &[i64]) -> i64 {
    solve(input, 3)
}

pub fn p1_sets(input: &[i64]) -> i64 {
    let s: HashSet<i64> = input.iter().cloned().collect();
    let n = *s.iter().find(|&&x| s.contains(&(2020 - x))).unwrap();
    n * (2020 - n)
}

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