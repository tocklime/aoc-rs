use advent_of_code_traits::{days::Day1, ParseInput, Solution};

use crate::AdventOfCode2021;

impl ParseInput<Day1> for AdventOfCode2021 {
    type Parsed = Vec<u32>;

    fn parse_input(input: &str) -> Self::Parsed {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }
}

impl Solution<Day1> for AdventOfCode2021 {
    type Part1Output = u32;

    type Part2Output = u32;

    fn part1(input: &Vec<u32>) -> Self::Part1Output {
        for (ix, &i) in input.iter().enumerate() {
            for &j in &input[ix + 1..] {
                if i + j == 2020 {
                    return i * j;
                }
            }
        }
        0
    }
    fn part2(input: &Vec<u32>) -> Self::Part2Output {
        for (ix, &i) in input.iter().enumerate() {
            for (ix2, &j) in input.iter().enumerate().skip(ix) {
                for &k in input.iter().skip(ix2) {
                    if i + j + k == 2020 {
                        return i * j * k;
                    }
                }
            }
        }
        0
    }
}
