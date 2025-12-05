use std::collections::BTreeSet;

use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};
use utils::{nom::NomError, span::Span};

aoc_harness::aoc_main!(2025 day 5, generator parse, part1 [p1] => 761, part2 [p2] => 345_755_049_374_932, example part1 EG => 3, example part2 EG => 14);


fn parse(input: &str) -> (Vec<Span<u64>>, Vec<u64>) {
    let (ranges, ingredients) = input.trim().split_once("\n\n").unwrap();
    let ranges = all_consuming(separated_list1(
        newline::<_, NomError>,
        separated_pair(complete::u64, tag("-"), complete::u64).map(|(a, b)| Span::new(a, b+1)),
    ))
    .parse(ranges)
    .unwrap()
    .1;
    let ingredients = all_consuming(separated_list1(newline::<_, NomError>, complete::u64))
        .parse(ingredients)
        .unwrap()
        .1;
    (ranges, ingredients)
}

fn p1(input: &(Vec<Span<u64>>, Vec<u64>)) -> usize {
    input.1.iter().filter(|&&x| input.0.iter().any(|r| r.contains(x))).count()
}
fn p2(input: &(Vec<Span<u64>>, Vec<u64>)) -> u64 {
    let mut ranges = input.0.clone();

    let mut cut_ranges = BTreeSet::new();
    while let Some(r) = ranges.pop() {
        if cut_ranges.contains(&r) {
            continue;
        }
        if let Some(first_collider) = cut_ranges.iter().find(|or| r.intersects(or)) {
            ranges.extend(r.subtract(first_collider));
        } else {
            cut_ranges.insert(r);
        }
    }
    cut_ranges.into_iter().map(utils::span::Span::size).sum()
}

const EG: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
