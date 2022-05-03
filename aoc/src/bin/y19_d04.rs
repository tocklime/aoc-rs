use aoc_harness::*;
use itertools::Itertools;
use rayon::prelude::*;
use utils::nums::int_to_digits_big_endian;

aoc_main!(2019 day 4, generator gen, part1 [p1] => 921, part2 [p2] => 603);

pub fn check_groups(input: usize, check: fn(usize) -> bool) -> bool {
    let groups = int_to_digits_big_endian::<6>(input)
        .into_iter()
        .group_by(|x| *x);
    let mut last_key: Option<u8> = None;
    let mut saw_any_match = false;
    for (k, g) in &groups {
        if last_key.map_or(false, |l| l > k) {
            return false; //key decreased!
        }
        last_key = Some(k);
        saw_any_match |= check(g.count());
    }
    saw_any_match
}

pub fn find(input: &[usize], group_check: fn(usize) -> bool) -> usize {
    (input[0]..=input[1])
        .into_par_iter()
        .filter(|&x| check_groups(x, group_check))
        .count()
}

pub fn gen(input: &str) -> Vec<usize> {
    input
        .trim()
        .split('-')
        .map(|x| x.parse().unwrap())
        .collect()
}
pub fn p1(input: &[usize]) -> usize {
    find(input, |x| x > 1)
}
pub fn p2(input: &[usize]) -> usize {
    find(input, |x| x == 2)
}
