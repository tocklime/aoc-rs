

aoc_harness::aoc_main!(2017 day 6, part1 [p1], part2 [p2]);
use itertools::Itertools;
use nom::lib::std::collections::HashSet;
use std::collections::HashMap;

fn p1(input: &str) -> usize {
    let mut mem = input
        .trim()
        .split('\t')
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();
    let mut seen = HashSet::new();
    let mut steps = 0;
    let len = mem.len();
    loop {
        if !seen.insert(mem.clone()) {
            return steps;
        }
        steps += 1;
        let mut curr: usize = mem
            .iter()
            .enumerate()
            .max_by_key(|(a, &b)| (b, len - a))
            .unwrap()
            .0;
        let mut in_hand = mem[curr];
        mem[curr] = 0;
        while in_hand > 0 {
            curr += 1;
            *mem.get_mut(curr % len).unwrap() += 1;
            in_hand -= 1;
        }
    }
}

fn p2(input: &str) -> usize {
    let mut mem = input
        .trim()
        .split('\t')
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();
    let mut seen = HashMap::new();
    let mut steps: usize = 0;
    let len = mem.len();
    loop {
        if seen.contains_key(&mem) {
            return steps - seen[&mem];
        }
        seen.insert(mem.clone(), steps);
        steps += 1;
        let mut curr: usize = mem
            .iter()
            .enumerate()
            .max_by_key(|(a, &b)| (b, len - a))
            .unwrap()
            .0;
        let mut in_hand = mem[curr];
        mem[curr] = 0;
        while in_hand > 0 {
            curr += 1;
            *mem.get_mut(curr % len).unwrap() += 1;
            in_hand -= 1;
        }
    }
}

#[test]
fn test_day_6() {
    assert_eq!(p1("0\t2\t7\t0"), 5);
}
