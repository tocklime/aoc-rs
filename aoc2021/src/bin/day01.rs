use itertools::Itertools;
use std::collections::HashSet;

aoc_harness_macros::aoc_main!(2021 day 1, generator parse_input, [part1], [part2]);

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn part1(input: &Vec<u32>) -> usize {
    input.len()
}
fn part2(input: &Vec<u32>) -> usize {
    input.len()
}
