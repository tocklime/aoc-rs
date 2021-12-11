use utils::intcode::Computer;
use std::str::FromStr;
//#[aoc(day9, part1)]
pub fn p1(input: &str) -> i64 {
    let mut c = Computer::from_str(input).unwrap();
    c.with_input(1).run().get_last_output()
}
//#[aoc(day9, part2)]
pub fn p2(input: &str) -> i64 {
    let mut c = Computer::from_str(input).unwrap();
    c.with_input(2).run().get_last_output()
}
#[test]
pub fn p1tests() {
    use itertools::Itertools;
    let e0 = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let e1 = "1102,34915192,34915192,7,4,7,99,0";
    let e2 = "104,1125899906842624,99";

    let mut c0: Computer<i64> = Computer::from_str(e0).unwrap();
    c0.run();
    let output0 = format!("{}", c0.get_output().iter().format(","));
    assert_eq!(output0, e0);
    assert_eq!(p1(e1), 1219070632396864);
    assert_eq!(p1(e2), 1125899906842624);
}
