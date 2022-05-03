use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc_harness::*;
use itertools::MinMaxResult;
use nalgebra::{DMatrix, DVector};
use num::Integer;
use utils::nums::exp_by_squares;

aoc_main!(2021 day 14, generator whole_input_is::<Day14>, part1 [solve::<10>] => 3284,
    part2 [solve::<40>] => 4_302_675_529_689,
    example both EG => (1588, 2_188_189_693_529_u64)
);

const EG: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

struct Day14 {
    mat: DMatrix<u64>,
    char_count: usize,
    start: DVector<u64>,
}
impl FromStr for Day14 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let all_chars: HashSet<char> = s.chars().filter(|c| char::is_alphabetic(*c)).collect();
        let lookup: HashMap<char, usize> = all_chars.into_iter().zip(0..).collect();
        let char_count = lookup.len();
        let pair_count = char_count * char_count;
        let mut x = s.split("\n\n");
        let line1 = x.next().unwrap();
        let mut start = DVector::from_element(pair_count, 0_u64);
        for (a, b) in line1.chars().map(|x| lookup[&x]).tuple_windows() {
            let ix = a * char_count + b;
            start[ix] += 1;
        }
        let mut mat: DMatrix<u64> = DMatrix::from_element(pair_count, pair_count, 0);
        for l in x.next().unwrap().lines() {
            let mut i = l.split(" -> ");
            let from = i.next().unwrap().chars().map(|a| lookup[&a]).collect_vec();
            let to = lookup[&i.next().unwrap().chars().collect_vec()[0]];
            let ab_ix = from[0] * char_count + from[1];
            mat[(from[0] * char_count + to, ab_ix)] = 1;
            mat[(to * char_count + from[1], ab_ix)] = 1;
        }
        Ok(Self {
            mat,
            char_count,
            start,
        })
    }
}
fn solve<const ITERS: usize>(input: &Day14) -> u64 {
    let mat_pow = exp_by_squares(&input.mat, ITERS - 1);
    let end_vec = mat_pow * &input.start;
    let mut counts = vec![1; input.char_count];
    for (ix, count) in end_vec.into_iter().enumerate() {
        let (a, b) = ix.div_mod_floor(&input.char_count);
        counts[a] += count;
        counts[b] += count;
    }
    if let MinMaxResult::MinMax(&a, &b) = counts.iter().filter(|&&x| x > 0).minmax() {
        b / 2 - a / 2
    } else {
        unreachable!()
    }
}
