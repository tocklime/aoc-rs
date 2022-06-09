use aoc_harness::aoc_main;

aoc_main!(2017 day 5, part1 [p1], part2 [p2]);
use itertools::Itertools;
use std::convert::TryInto;

fn p1(input: &str) -> usize {
    let mut mem = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect_vec();
    let mut ip: i32 = 0;
    let mut steps = 0;
    loop {
        let as_u: Option<usize> = ip.try_into().ok();
        match as_u.and_then(|u| mem.get_mut(u)) {
            Some(x) => {
                ip += *x;
                *x += 1;
            }
            None => break,
        }
        steps += 1;
    }
    steps
}

fn p2(input: &str) -> usize {
    let mut mem = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect_vec();
    let mut ip: i32 = 0;
    let mut steps = 0;
    loop {
        let as_u: Option<usize> = ip.try_into().ok();
        match as_u.and_then(|u| mem.get_mut(u)) {
            Some(x) => {
                ip += *x;
                *x += if *x >= 3 { -1 } else { 1 };
            }
            None => break,
        }
        steps += 1;
    }
    steps
}
//too low: 18227952
