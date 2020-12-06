use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn p1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| g.lines().flat_map(str::chars).collect::<HashSet<_>>().len())
        .sum()
}

pub fn solve<F>(input: &str, f: F) -> usize
where
    F: Fn(&HashSet<char>, &HashSet<char>) -> HashSet<char>,
{
    input
        .split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| l.chars().collect())
                .fold(None, |acc: Option<HashSet<char>>, s| {
                    Some(match acc {
                        None => s,
                        Some(x) => f(&x, &s),
                    })
                })
                .unwrap()
                .len()
        })
        .sum()
}
#[aoc(day6, part1, binop)]
pub fn p1_binop(input: &str) -> usize {
    solve(input, |a, b| a | b)
}
#[aoc(day6, part2, binop)]
pub fn p2_binop(input: &str) -> usize {
    solve(input, |a, b| a & b)
}
