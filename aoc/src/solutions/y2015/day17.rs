use aoc_harness::aoc_main;

aoc_main!(2015 day 17, part1 [p1], part2 [p2]);
use itertools::Itertools;

const TARGET: usize = 150;


fn p1(input: &str) -> usize {
    let caps = input.lines().map(|l| l.parse::<usize>().unwrap()).collect_vec();
    (1..=caps.len())
        .map(|lim| caps.iter().combinations(lim)
            .filter(|x| x.iter().cloned().sum::<usize>() == TARGET).count())
        .sum()
}


fn p2(input: &str) -> usize {
    let caps = input.lines().map(|l| l.parse::<usize>().unwrap()).collect_vec();
    (1..=caps.len())
        .map(|lim| caps.iter().combinations(lim)
            .filter(|x| x.iter().cloned().sum::<usize>() == TARGET).count())
        .find(|&x| x > 0).unwrap()
}
