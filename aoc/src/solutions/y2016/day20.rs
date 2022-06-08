use aoc_harness::aoc_main;

aoc_main!(2016 day 20, part1 [p1], part2 [p2]);
use itertools::Itertools;
use std::cmp::max;
use std::collections::BTreeSet;

fn p1(input: &str) -> u32 {
    let limits: BTreeSet<(u32, u32)> = input
        .lines()
        .map(|x| {
            let s = x
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec();
            (s[0], s[1])
        })
        .collect();
    let mut candidate = 0;
    for &(lo, hi) in &limits {
        if candidate < lo {
            return candidate;
        }
        candidate = hi + 1;
    }
    panic!("Not found");
}

fn p2(input: &str) -> u32 {
    let limits: BTreeSet<(u32, u32)> = input
        .lines()
        .map(|x| {
            let s = x
                .split('-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect_vec();
            (s[0], s[1])
        })
        .collect();
    let mut count = 0;
    let mut candidate = 0;
    for &(lo, hi) in &limits {
        if candidate < lo {
            count += lo - candidate;
        }
        candidate = max(candidate, hi.saturating_add(1));
    }
    count
}
