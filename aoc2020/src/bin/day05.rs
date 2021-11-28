use std::collections::HashSet;

use aoc_harness_macros::aoc_main;

pub fn gen(input: &str) -> HashSet<usize> {
    input
        .lines()
        .map(|c| {
            c.chars().fold(0, |acc, x| {
                acc * 2 + ("Binary go BRRRR!".contains(x) as usize)
            })
        })
        .collect()
}

pub fn p1(input: &HashSet<usize>) -> Option<usize> {
    input.iter().max().copied()
}
pub fn p2(input: &HashSet<usize>) -> Option<usize> {
    input
        .iter()
        .map(|&x| x + 1)
        .find(|&x| !input.contains(&(x)) && input.contains(&(x + 1)))
}

aoc_main!(2020 day 5, generator gen, [p1] => Some(922), [p2] => Some(747));
