

aoc_harness::aoc_main!(2020 day 5, generator gen, part1 [p1] => 922, part2 [p2] => 747);
use std::collections::HashSet;

fn gen(input: &str) -> HashSet<usize> {
    input
        .lines()
        .map(|c| {
            c.chars().fold(0, |acc, x| {
                acc * 2 + usize::from("Binary go BRRRR!".contains(x))
            })
        })
        .collect()
}

fn p1(input: &HashSet<usize>) -> usize {
    input.iter().max().copied().unwrap()
}
fn p2(input: &HashSet<usize>) -> usize {
    input
        .iter()
        .map(|&x| x + 1)
        .find(|&x| !input.contains(&(x)) && input.contains(&(x + 1)))
        .unwrap()
}
