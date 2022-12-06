use std::collections::{VecDeque, HashSet};

use itertools::*;
use aoc_harness::*;

aoc_main!(2022 day 6, part1 [p1], part2 [p2], example both EG => (7,19));

const EG : &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

fn p1(input: &str) -> usize {
    input.chars().tuple_windows().enumerate().find(|(ix, (a,b,c,d))| {
        a != b && a != c && a != d && b != c && b != d && c != d
    }).unwrap().0 + 4 
}

fn p2(input: &str) -> usize {
    for ix in 0..(input.len() - 14) {
        let set : HashSet<char> = input.chars().skip(ix).take(14).collect();
        if set.len() == 14 {
            dbg!(set);
            return ix + 14
        }
    }
    0
}

