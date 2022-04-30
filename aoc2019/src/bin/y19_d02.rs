use aoc_harness::*;
use std::str::FromStr;
use utils::intcode::Computer;

aoc_main!(2019 day 2, part1 [p1] => 7594646, part2[p2] => 3376);

pub fn run_with_args(c: &mut Computer, noun: i32, verb: i32) -> i32 {
    c.abs_store(1, noun);
    c.abs_store(2, verb);
    c.run().abs_load(0)
}

pub fn p1(input: &str) -> i32 {
    let mut c = Computer::from_str(input).unwrap();
    run_with_args(&mut c, 12, 2)
}

pub fn p2(input: &str) -> i32 {
    let mut c = Computer::from_str(input).unwrap();
    let (n, v) = (0..100)
        .flat_map(move |n| (0..100).map(move |v| (n, v)))
        .find(|(n, v)| run_with_args(c.reset(), *n, *v) == 19_690_720)
        .unwrap();
    100 * n + v
}
