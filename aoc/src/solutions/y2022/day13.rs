use std::cmp::Ordering;

use aoc_harness::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, self},
    combinator::{all_consuming, map},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, terminated},
    IResult,
};

aoc_main!(2022 day 13, generator gen, part1 [p1] => 5625, example both EG => (13,140), part2 [p2] => 23111);

const EG: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Single(u8),
}

fn parse_list(input: &str) -> IResult<&str, Packet> {
    alt((
        map(complete::u8, Packet::Single),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_list), tag("]")),
            Packet::List,
        ),
    ))(input)
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Single(l), Packet::Single(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
            (Packet::List(l), Packet::Single(r)) => l[..].cmp(&[Packet::Single(*r)][..]),
            (Packet::Single(l), Packet::List(r)) => [Packet::Single(*l)][..].cmp(r),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn gen(input: &str) -> Vec<Vec<Packet>> {
    all_consuming(terminated(
        separated_list1(tag("\n\n"), separated_list1(newline, parse_list)),
        newline,
    ))(input)
    .unwrap()
    .1
}

fn p1(input: &[Vec<Packet>]) -> usize {
    input
        .iter()
        .zip(1..)
        .filter(|(l, _)| l[0] < l[1])
        .map(|(_, ix)| ix)
        .sum()
}
fn p2(input: &[Vec<Packet>]) -> usize {
    let mut l = input.iter().flatten().collect::<Vec<_>>();
    let a = parse_list("[[2]]").unwrap().1;
    let b = parse_list("[[6]]").unwrap().1;
    l.push(&a);
    l.push(&b);
    l.sort_unstable();
    let div1 = l.iter().position(|&l| l == &a).unwrap();
    let div2 = l.iter().position(|&l| l == &b).unwrap();
    (1 + div1) * (1 + div2)
}
