use std::cmp::min;

use aoc_harness::*;

aoc_main!(2021 day 7, generator input::<isize,','>, part1 [p1] => 352997, part2 [p2] => 101571302,
  example part1 EG => 37,
  example part2 EG => 168,
);
const EG: &str = "16,1,2,0,4,2,7,1,2,14";

fn p1(input: &[isize]) -> isize {
    let mut input = input.to_vec();
    input.sort_unstable();
    let median = input[input.len() / 2];
    input
        .iter()
        .map(|&x| (x-median).abs())
        .sum()
}

fn fuel_to_pos2(input: &[isize], pos: isize) -> isize {
    input
        .iter()
        .map(|&x| {
            let steps = (x-pos).abs();
            steps * (steps + 1) / 2
        })
        .sum()
}

fn p2(input: &[isize]) -> isize {
    let mean: isize = input.iter().sum::<isize>() / (input.len() as isize);
    min(fuel_to_pos2(input, mean), fuel_to_pos2(input, mean + 1))
}
