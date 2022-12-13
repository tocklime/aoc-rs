use aoc_harness::*;
use nom::{IResult, sequence::{delimited, separated_pair}, bytes::complete::tag, multi::{separated_list1, separated_list0, many1}, combinator::map, branch::alt, character::complete::newline};

aoc_main!(2022 day 13, part1 [p1], example both EG => (13,140), part2 [p2]);

const EG : &str = "[1,1,3,1,1]
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

#[derive(Clone,Debug, PartialEq,Eq)]
enum List {
    List(Vec<List>),
    Single(u8)
}

fn parse_list(input: &str) -> IResult<&str, List> {
    let (input, l) = alt((
        map(nom::character::complete::u8, List::Single),
        map(delimited(tag("["), separated_list0(tag(","), parse_list), tag("]")), List::List)
    ))(input)?;
    Ok((input, l))
}
fn parse_input(input: &str) -> IResult<&str, Vec<(List,List)>> {
    separated_list1(tag("\n\n"), separated_pair(parse_list, newline, parse_list))(input)
}

fn cmp_list(a: &List, b: &List) -> std::cmp::Ordering {
    match (a, b) {
        (List::Single(l), List::Single(r)) => l.cmp(r),
        (List::List(l), List::List(r)) => l.cmp(r),
        (List::List(l), List::Single(r)) => l.cmp(&vec![List::Single(*r)]),
        (List::Single(l), List::List(r)) => vec![List::Single(*l)].cmp(r)
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        cmp_list(self, other)
    }
}
impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(cmp_list(self,other))
    }
}
fn p1(input: &str) -> usize {
    let (_, l) = parse_input(input).unwrap();
    l.into_iter().zip(1..).filter(|((a,b),_)| a < b).map(|(_, ix)|ix).sum()
}
fn p2(input: &str) -> usize {
    let (_, mut l) = separated_list1(many1(newline), parse_list)(input).unwrap();
    let (_, a) = parse_list("[[2]]").unwrap();
    let (_, b) = parse_list("[[6]]").unwrap();
    l.push(a.clone());
    l.push(b.clone());
    l.sort();
    let (div1,_) = l.iter().find_position(|&l| l == &a).unwrap();
    let (div2,_) = l.iter().find_position(|&l| l == &b).unwrap();
    (1+div1)*(1+div2)
}