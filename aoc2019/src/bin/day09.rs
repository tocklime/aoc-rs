use aoc_harness::aoc_main;
use std::str::FromStr;
use utils::intcode::Computer;

aoc_main!(2019 day 9, part1 [p1] => 4261108180, part2 [p2] => 77944,
    example part1 E1 => 1219070632396864,
    example part1 E2 => 1125899906842624
);

pub fn p1(input: &str) -> i64 {
    let mut c = Computer::from_str(input).unwrap();
    c.with_input(1).run().get_last_output()
}

pub fn p2(input: &str) -> i64 {
    let mut c = Computer::from_str(input).unwrap();
    c.with_input(2).run().get_last_output()
}

const E1: &str = "1102,34915192,34915192,7,4,7,99,0";
const E2: &str = "104,1125899906842624,99";
#[test]
pub fn p1tests() {
    use itertools::Itertools;

    const E0: &str = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let mut c0: Computer<i64> = Computer::from_str(E0).unwrap();
    c0.run();
    let output0 = format!("{}", c0.get_output().iter().format(","));
    assert_eq!(output0, E0);
}
