use aoc_harness::aoc_main;
use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    Parser, sequence::tuple,
};
use nom_supreme::{ParserExt, tag::complete::tag};
use utils::nom::IResult;

aoc_main!(2023 day 6, part1 [p1], part2 [p2], example both EG => (288, 71503));

fn parse(input: &str) -> IResult<Vec<(u64, u64)>> {
    let (input, times) = separated_list1(space1, complete::u64)
        .preceded_by(tuple((tag("Time:"), space1)))
        .terminated(newline)
        .parse(input)?;
    let (input, distance) = separated_list1(space1, complete::u64)
        .preceded_by(tuple((tag("Distance:"), space1)))
        .terminated(newline)
        .parse(input)?;
    let zipped = times.into_iter().zip(distance.into_iter()).collect();
    Ok((input, zipped))
}

fn solve(time: u64, record: u64) -> usize {
    (0..time).filter(|t| {
        let speed = t;
        let remaining = time - t;
        let distance = speed * remaining;
        distance > record
    }).count()
}
fn p1(input: &str) -> usize {
    let t = parse(input).unwrap().1;
    t.into_iter().map(|(a,b)|solve(a,b)).product()
}
fn p2(input: &str) -> usize {
    let mut l = input.lines();
    let time : u64 = l.next().unwrap().chars().filter(char::is_ascii_digit).collect::<String>().parse().unwrap();
    let dist : u64 = l.next().unwrap().chars().filter(char::is_ascii_digit).collect::<String>().parse().unwrap();
    solve(time,dist)
}

const EG: &str = "Time:      7  15   30
Distance:  9  40  200
";
