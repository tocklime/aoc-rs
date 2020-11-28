#![warn(clippy::all)]
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| {
            let sign = if l.starts_with('-') { -1 } else { 1 };
            let n: i64 = l[1..].parse().expect("Integer");
            sign * n
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn p1(input: &[i64]) -> i64 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn p2(input: &[i64]) -> i64 {
    let mut seen: HashSet<i64> = HashSet::new();
    for i in input.iter().cycle().scan(0, |s, &x| {
        *s += x;
        Some(*s)
    }) {
        if seen.contains(&i) {
            return i;
        }
        seen.insert(i);
    }
    -1
}
