use aoc_harness::aoc_main;

aoc_main!(2017 day 1, part1 [p1], part2 [p2]);
use itertools::Itertools;

fn p1(input: &str) -> u32 {
    let mut digs = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect_vec();
    digs.push(digs[0]);
    digs.iter()
        .tuple_windows()
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .sum()
}

fn p2(input: &str) -> u32 {
    let digs = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect_vec();
    let (a, b) = digs.split_at(digs.len() / 2);
    a.iter()
        .zip(b)
        .filter_map(|(a, b)| if a == b { Some(*a) } else { None })
        .sum::<u32>()
        * 2
}
