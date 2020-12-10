use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

#[aoc_generator(day10)]
pub fn gen(input: &str) -> Vec<usize> {
    let mut a : Vec<usize> = input.lines().map(|x| x.parse::<usize>().unwrap()).collect();
    a.push(0);
    a.sort_unstable();
    a.push(a.last().unwrap() + 3);
    a
}

#[aoc(day10, part1)]
pub fn p1(input: &[usize]) -> usize {
    let (a, b) = input
        .iter()
        .tuple_windows()
        .fold((0, 0), |(a, b), (c,d)| match d - c {
            1 => (a + 1, b),
            3 => (a, b + 1),
            _ => (a, b),
        });
    a * b
}

pub fn count_routes(
    memo: &mut HashMap<usize, usize>,
    map: &HashSet<usize>,
    from: usize,
    to: usize,
) -> usize {
    match (memo.get(&from), from.cmp(&to)) {
        (Some(&x), _) => x,
        (_, Ordering::Greater) => 0,
        (_, Ordering::Equal) => 1,
        _ => {
            let a = (from + 1..=from + 3)
                .filter_map(|m| map.get(&m).map(|&x| count_routes(memo, map, x, to)))
                .sum();
            memo.insert(from, a);
            a
        }
    }
}

#[aoc(day10, part2)]
pub fn p2(input: &[usize]) -> usize {
    let mut xs: HashSet<usize> = input.iter().copied().collect();
    let target = xs.iter().max().unwrap() + 3_usize;
    xs.insert(target);
    count_routes(&mut HashMap::new(), &xs, 0, target)
}
