use aoc_harness::aoc_main;

aoc_main!(2016 day 3, generator gen, part1 [p1], part2 [p2]);
use itertools::Itertools;
use regex::Regex;

fn gen(input: &str) -> Vec<Vec<usize>> {
    let re = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .map(|l| {
            re.captures_iter(l)
                .map(|x| x[0].parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect()
}

fn p1(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .filter(|&l| {
            let mut t: Vec<usize> = l.clone();
            t.sort_unstable();
            t[0] + t[1] > t[2]
        })
        .count()
}

fn p2(input: &[Vec<usize>]) -> usize {
    input
        .chunks(3)
        .flat_map(|ch| {
            let mut ts = vec![
                vec![ch[0][0], ch[1][0], ch[2][0]],
                vec![ch[0][1], ch[1][1], ch[2][1]],
                vec![ch[0][2], ch[1][2], ch[2][2]],
            ];
            for t in &mut ts {
                t.sort_unstable();
            }
            ts
        })
        .filter(|t| t[0] + t[1] > t[2])
        .count()
}
