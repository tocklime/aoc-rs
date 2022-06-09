use aoc_harness::aoc_main;

aoc_main!(2017 day 2, part1 [p1], part2 [p2]);
use itertools::Itertools;
use regex::Regex;

fn p1(input: &str) -> u32 {
    let re = Regex::new(r"\s+").unwrap();
    input
        .trim()
        .lines()
        .map(|l| {
            let (a, b) = re
                .split(l)
                .map(|n| n.parse::<u32>().unwrap())
                .minmax()
                .into_option()
                .unwrap();
            b - a
        })
        .sum()
}

fn p2(input: &str) -> u32 {
    let re = Regex::new(r"\s+").unwrap();
    input
        .trim()
        .lines()
        .map(|l| {
            re.split(l)
                .map(|n| n.parse::<u32>().unwrap())
                .permutations(2)
                .filter_map(|v| {
                    if v[1] % v[0] == 0 {
                        Some(v[1] / v[0])
                    } else {
                        None
                    }
                })
                .next()
                .unwrap()
        })
        .sum()
}
