use itertools::Itertools;
use std::collections::HashSet;

use advent_of_code_traits::{days::Day1, ParseInput, Solution};

use crate::AdventOfCode2021;

impl ParseInput<Day1> for AdventOfCode2021 {
    type Parsed = HashSet<u32>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }
}

impl Solution<Day1> for AdventOfCode2021 {
    type Part1Output = u32;

    type Part2Output = u32;

    fn part1(input: &HashSet<u32>) -> Self::Part1Output {
        let &n = input
            .iter()
            .find(|&&x| input.contains(&(2020 - x)))
            .unwrap();
        n * (2020 - n)
    }
    fn part2(input: &HashSet<u32>) -> Self::Part2Output {
        input
            .iter()
            .combinations(2)
            .find_map(|x| {
                let sum = x[0] + x[1];
                if sum <= 2020 && input.contains(&(2020 - sum)) {
                    Some(x[0] * x[1] * (2020 - sum))
                } else {
                    None
                }
            })
            .unwrap()
    }
}
