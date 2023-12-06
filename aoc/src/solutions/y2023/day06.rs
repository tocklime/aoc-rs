
use nom::{
    character::complete::{self, newline, space0},
    Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt, multi::collect_separated_terminated};
use num::integer::Roots;
use utils::nom::IResult;

aoc_harness::aoc_main!(2023 day 6, part1 [p::<false>] => 2_756_160, part2 [p::<true>] => 34_788_142, example both EG => (288, 71503));

fn parse(input: &str) -> IResult<Vec<(u64, u64)>> {
    let (input, times): (&str, Vec<u64>) = tag("Time:")
        .precedes(space0)
        .precedes(collect_separated_terminated(
            complete::u64,
            space0,
            newline,
        ))
        .parse(input)?;
    let (input, distance): (&str, Vec<u64>) = tag("Distance:")
        .precedes(space0)
        .precedes(collect_separated_terminated(
            complete::u64,
            space0,
            newline,
        ))
        .parse(input)?;
    let zipped = times.into_iter().zip(distance).collect();
    Ok((input, zipped))
}

fn solve(time_allowed: u64, record: u64) -> u64 {
    let dist_f = |x| x * time_allowed - x * x;
    let intermediate = (time_allowed*time_allowed-4*record).sqrt();
    let root1 = (time_allowed - intermediate) / 2;
    let root2 = (time_allowed + intermediate) / 2;

    //roots may be 1 too big or too small. Find values that are both to the right of the root.
    let root1 = (root1-1..=root1+1).find(|x| dist_f(*x) > record).unwrap();
    let root2 = (root2-1..=root2+1).find(|x| dist_f(*x) <= record).unwrap();
    root2 - root1
}
fn p<const NO_SPACE: bool>(input: &str) -> u64 {
    let t = if NO_SPACE {
        parse(&input.replace(' ', "")).unwrap().1
    } else {
        parse(input).unwrap().1
    };
    t.into_iter().map(|(a, b)| solve(a, b)).product()
}

const EG: &str = "Time:      7  15   30
Distance:  9  40  200
";
