use std::cmp::Ordering;

use aoc_harness::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, u8},
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
        map(u8, Packet::Single),
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
    let a: Packet = Packet::List(vec![Packet::List(vec![Packet::Single(2)])]);
    let b: Packet = Packet::List(vec![Packet::List(vec![Packet::Single(6)])]);
    let mut a_pos = 1;
    let mut b_pos = 2; //a is also before b.
    for p in input.iter().flatten() {
        if p < &a {
            a_pos += 1;
            b_pos += 1;
        } else if p < &b {
            b_pos += 1;
        }
    }
    a_pos * b_pos
}
