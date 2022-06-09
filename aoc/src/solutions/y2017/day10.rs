use std::str::FromStr;

use aoc_harness::aoc_main;

aoc_main!(2017 day 10, part1 [p1], part2 [p2]);
use itertools::Itertools;
use utils::knot_hash::*;

fn p1(input: &str) -> u64 {
    let ns = input
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();
    let mut st = KnotHash::new();
    st.step(&ns);
    let (a, b): (u64, u64) = st.data().iter().map(|&x| x.into()).next_tuple().unwrap();
    a * b
}

fn p2(input: &str) -> String {
    KnotHash::from_str(input).unwrap().dense_hash()
}
