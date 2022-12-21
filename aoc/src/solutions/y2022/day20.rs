use std::collections::VecDeque;

use aoc_harness::*;
use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

aoc_main!(2022 day 20, part1 [solve::<1,1>] => 5962, part2 [solve::<10,811589153>] => 9862431387256, example both EG => (3,1623178306));

const EG: &str = "1
2
-3
3
-2
0
4
";
fn parse_line(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(newline, complete::i64)(input)
}
fn solve<const ITER: usize, const MUL: i64>(input: &str) -> i64 {
    let (_, numbers) = parse_line(input).unwrap();
    let mut d: VecDeque<(usize, i64)> = numbers.into_iter().map(|x| x * MUL).enumerate().collect();

    for _ in 0..ITER {
        for id in 0..d.len() {
            let index = d.iter().position(|x| id == x.0).unwrap();
            d.rotate_left(index);
            let item = d.pop_front().unwrap();
            d.rotate_left((item.1).rem_euclid(d.len() as i64) as usize);
            d.push_front(item);
        }
    }

    let zero_ix = d.iter().position(|x| x.1 == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|n| d[(zero_ix + n) % d.len()].1)
        .sum::<i64>()
}
