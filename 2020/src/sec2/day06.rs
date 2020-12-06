use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn p1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| {
            g.chars()
                .filter(char::is_ascii_lowercase)
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}
#[aoc(day6, part2)]
pub fn p2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| l.chars().collect::<HashSet<char>>())
                .collect::<Vec<_>>()
                .iter()
                .fold(None, |acc, s| match acc {
                    None => Some(s.clone()),
                    Some(x) => Some(x.intersection(s).copied().collect::<HashSet<char>>()),
                })
                .unwrap()
                .len()
        })
        .sum()
}
