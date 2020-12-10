use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;

#[aoc(day10, part1)]
pub fn p1(input: &str) -> Option<usize> {
    let mut xs: BTreeSet<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    let mut one_jumps = 0;
    let mut three_jumps = 1;
    xs.insert(0);
    for (a, b) in xs.iter().tuple_windows() {
        match b - a {
            3 => three_jumps += 1,
            1 => one_jumps += 1,
            2 => {}
            _ => {
                panic!("No");
            }
        }
    }
    Some(three_jumps * one_jumps)
}

pub fn count_routes(memo: &mut HashMap<usize,usize>, map: &BTreeSet<usize>, from: usize, to: usize) -> usize {
    if memo.contains_key(&from) {
        return memo[&from];
    }
    if from == to {
        return 1;
    }
    if from > to {
        return 0;
    }
    let a = (from + 1..=from + 3)
        .filter(|m| map.contains(m))
        .map(|m| count_routes(memo, map, m, to))
        .sum();
    memo.insert(from,a);
    a
}

#[aoc(day10, part2)]
pub fn p2(input: &str) -> usize {
    let mut xs: BTreeSet<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    let target = xs.iter().max().unwrap() + 3_usize;
    xs.insert(target);
    let mut memo = HashMap::new();
    count_routes(&mut memo, &xs, 0, target)
}
