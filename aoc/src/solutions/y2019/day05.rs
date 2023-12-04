use std::str::FromStr;
use utils::intcode::Computer;

use aoc_harness::*;
aoc_main!(2019 day 5, part1 [solve::<1>] => 5_577_461, part2 [solve::<5>] => 7_161_591);

pub fn solve<const I: isize>(input: &str) -> isize {
    Computer::from_str(input)
        .unwrap()
        .with_input(I)
        .run()
        .get_last_output()
}
