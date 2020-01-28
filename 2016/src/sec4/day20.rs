use std::collections::BTreeSet;
use itertools::Itertools;
use std::cmp::max;

#[aoc(day20, part1)]
#[post(ret == 32_259_706)]
fn p1(input: &str) -> u32 {
    let limits: BTreeSet<(u32, u32)> = input.lines().map(|x| {
        let s = x.split('-').map(|x| x.parse::<u32>().unwrap()).collect_vec();
        (s[0], s[1])
    }).collect();
    let mut candidate = 0;
    for &(lo, hi) in &limits {
        if candidate < lo {
            return candidate;
        }
        candidate = hi + 1;
    }
    panic!("Not found");
}

#[aoc(day20, part2)]
#[post(ret == 113)]
fn p2(input: &str) -> u32 {
    let limits: BTreeSet<(u32, u32)> = input.lines().map(|x| {
        let s = x.split('-').map(|x| x.parse::<u32>().unwrap()).collect_vec();
        (s[0], s[1])
    }).collect();
    let mut count = 0;
    let mut candidate = 0;
    for &(lo, hi) in &limits {
        if candidate < lo {
            count += lo - candidate;
        }
        candidate = max(candidate,hi.saturating_add(1));
    }
    count
}
