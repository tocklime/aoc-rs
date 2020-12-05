use std::collections::HashSet;

#[aoc_generator(day5)]
pub fn gen(input: &str) -> HashSet<usize> {
    input
        .lines()
        .map(|c| {
            c.chars()
                .rev()
                .enumerate()
                .map(|(ix, c)| ("BR".contains(c) as usize) << ix)
                .sum()
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn p1(input: &HashSet<usize>) -> Option<usize> {
    input.iter().cloned().max()
}
#[aoc(day5, part2)]
pub fn p2(input: &HashSet<usize>) -> Option<usize> {
    input
        .iter()
        .find(|&&x| input.contains(&(x + 2)) && !input.contains(&(x + 1)))
        .map(|&x| x + 1)
}
