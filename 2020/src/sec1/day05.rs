#![warn(clippy::all)]
use std::collections::HashSet;

#[aoc_generator(day5)]
pub fn gen(input: &str) -> HashSet<usize> {
    input
        .lines()
        .map(|c| {
            c.chars()
                .fold(0,|acc,x| 
                    acc * 2 + ("BR".contains(x) as usize)
                )
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn p1(input: &HashSet<usize>) -> Option<usize> {
    input.iter().max().copied()
}
#[aoc(day5, part2)]
pub fn p2(input: &HashSet<usize>) -> Option<usize> {
    input
        .iter()
        .map(|&x| x + 1)
        .find(|&x| !input.contains(&(x)) && input.contains(&(x + 1)))
}
