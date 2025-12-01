use itertools::Itertools;
use std::collections::HashMap;

use num::{Integer, PrimInt};
use utils::nums::digit_count;

aoc_harness::aoc_main!(2024 day 11, part1 [solve::<25>] => 203_457, part2 [solve::<75>] => 241_394_363_462_435, example part1 EG => 55_312);

fn blink(n: usize) -> Vec<usize> {
    if n == 0 {
        vec![1]
    } else {
        let dc = digit_count(n);
        if dc.is_multiple_of(2) {
            let m = 10.pow((dc / 2) as u32);
            let (left,right) = n.div_mod_floor(&m);
            vec![left, right]
        } else {
            vec![n * 2024]
        }
    }
}

fn solve<const ROUNDS: usize>(input: &str) -> usize {
    let start = input.trim().split(' ').map(|x| x.parse().unwrap()).counts();
    let end = (0..ROUNDS).fold(start, |c,_| {
        c
            .into_iter()
            .flat_map(|(k, v)| blink(k).into_iter().map(move |n| (n, v)))
            .fold(HashMap::new(), |mut m, (k, v)| {
                *m.entry(k).or_default() += v;
                m
            })
    });
    end.values().sum()
}

const EG: &str = "125 17";
