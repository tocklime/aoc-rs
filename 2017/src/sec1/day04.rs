use std::collections::HashSet;
use itertools::Itertools;

#[aoc(day4,part1)]
fn p1(input: &str) -> usize {
    input.lines().filter(|l| {
        let mut seen = HashSet::new();
        for w in l.split(' ') {
            if !seen.insert(w) {
                return false;
            }
        }
        true
    }).count()
}

#[aoc(day4,part2)]
fn p2(input: &str) -> usize {
    input.lines().filter(|l| {
        let mut seen = HashSet::new();
        for w in l.split(' ') {
            let mut s = w.chars().collect_vec();
            s.sort();
            if !seen.insert(s) {
                return false;
            }
        }
        true
    }).count()
}
