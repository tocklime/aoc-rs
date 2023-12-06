use std::str::FromStr;

use aoc_harness::*;
use utils::numset::NumSet;

aoc_harness::aoc_main!(2022 day 3, part1 [p1], part2 [p2], example both EG => (157,70));

const EG: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

fn score(c: char) -> u8 {
    let cu8 = c as u8;
    (27 + cu8 - b'A') - ((b'a' - b'A' + 26) * u8::from(cu8 & 0b10_0000 > 0))
}

struct ScoreSet(NumSet<u64>);

impl FromStr for ScoreSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ScoreSet(s.chars().map(score).collect()))
    }
}
impl ScoreSet {
    fn intersection(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }
    fn single_score(&self) -> usize {
        self.0.iter().next().unwrap() as usize
    }
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.len() / 2);
            let a_set = ScoreSet::from_str(a).unwrap();
            let b_set = ScoreSet::from_str(b).unwrap();
            a_set.intersection(&b_set).single_score()
        })
        .sum()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .map(|l| ScoreSet::from_str(l).unwrap())
        .chunks(3)
        .into_iter()
        .map(|ch| ch.reduce(|a, b| a.intersection(&b)).unwrap().single_score())
        .sum()
}
