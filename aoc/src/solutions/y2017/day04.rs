use aoc_harness::aoc_main;

aoc_main!(2017 day 4, part1 [p1], part2 [p2]);
use itertools::Itertools;
use std::collections::HashSet;

fn p1(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let mut seen = HashSet::new();
            for w in l.split(' ') {
                if !seen.insert(w) {
                    return false;
                }
            }
            true
        })
        .count()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .filter(|l| {
            let mut seen = HashSet::new();
            for w in l.split(' ') {
                let mut s = w.chars().collect_vec();
                s.sort_unstable();
                if !seen.insert(s) {
                    return false;
                }
            }
            true
        })
        .count()
}
