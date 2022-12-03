use std::{collections::HashSet, hash::Hash};

use aoc_harness::*;

aoc_main!(2022 day 3, part1 [p1], part2 [p2], example both EG => (157,70));

const EG: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

fn score(c: char) -> usize {
    (if c.is_ascii_lowercase() {
        1 + (c as u8) - b'a'
    } else {
        27 + (c as u8) - b'A'
    }) as usize
}
fn p1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let len = l.len();
            let (a, b) = l.split_at(len / 2);
            let a_set: HashSet<char> = a.chars().collect();
            let b_set: HashSet<char> = b.chars().collect();
            let common = a_set.intersection(&b_set).next().unwrap();
            score(*common)
        })
        .sum()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|ch| {
            let sets = ch
                .map(|l| l.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();
            let i = sets[0]
                .intersection(&sets[1])
                .copied()
                .collect::<HashSet<char>>();
            let i = i.intersection(&sets[2]).copied().collect::<HashSet<char>>();
            let c = i.into_iter().next().unwrap();
            score(c)
        })
        .sum()
}
