use itertools::Itertools;
use std::collections::HashSet;

aoc_harness_macros::aoc_main!(2021 day 1, generator parse_input, [part1], [part2]);

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn part1(input: &Vec<u32>) -> usize {
    let mut a = 0;
    for (x, b) in input.iter().tuple_windows() {
        if b > x {
            a += 1;
        }
    }
    a
}
fn part2(input: &Vec<u32>) -> usize {
    let mut ans = 0;
    let mut v = Vec::new();
    for (x, y, z) in input.iter().tuple_windows() {
        v.push(x + y + z);
    }
    for (a, b) in v.iter().tuple_windows() {
        if b > a {
            ans += 1;
        }
    }
    ans
}
