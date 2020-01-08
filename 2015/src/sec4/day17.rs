use itertools::Itertools;

const TARGET: usize = 150;

#[aoc(day17, part1)]
fn p1(input: &str) -> usize {
    let caps = input.lines().map(|l| l.parse::<usize>().unwrap()).collect_vec();
    (1..=caps.len())
        .map(|lim| caps.iter().combinations(lim)
            .filter(|x| x.iter().cloned().sum::<usize>() == TARGET).count())
        .sum()
}

#[aoc(day17, part2)]
fn p2(input: &str) -> usize {
    let caps = input.lines().map(|l| l.parse::<usize>().unwrap()).collect_vec();
    (1..=caps.len())
        .map(|lim| caps.iter().combinations(lim)
            .filter(|x| x.iter().cloned().sum::<usize>() == TARGET).count())
        .find(|&x| x > 0).unwrap()
}
