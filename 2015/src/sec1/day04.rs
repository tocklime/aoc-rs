#![allow(clippy::maybe_infinite_iter)]
use md5;

#[aoc(day4,part1)]
pub fn p1(input: &str) -> usize {
    (0..).find(|i| {
        let str = format!("{}{}",input,i);
        let md5 = md5::compute(&str);
        md5[0] == 0 && md5[1] == 0 && (md5[2] & 0xF0) == 0
    }).unwrap()
}
#[aoc(day4,part2)]
pub fn p2(input: &str) -> usize {
    (0..).find(|i| {
        let str = format!("{}{}",input,i);
        let md5 = md5::compute(&str);
        md5[0] == 0 && md5[1] == 0 && md5[2] == 0
    }).unwrap()
}
