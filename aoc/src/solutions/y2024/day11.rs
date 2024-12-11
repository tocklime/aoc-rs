use std::collections::HashMap;

use num::PrimInt;
use utils::{inputs::parse_input_from_str_sep_by, nums::digit_count};

aoc_harness::aoc_main!(2024 day 11, part1 [p1::<25>] => 203_457, part2 [p1::<75>] => 241_394_363_462_435, example part1 EG => 55312);

fn blink(n: usize) -> Vec<usize> {
    if n == 0 {
        vec![1]
    } else {
        let dc = digit_count(n);
        if dc%2 == 0 {
            let m = 10.pow((dc/2) as u32);
            let left = n / m;
            let right = n % m;
            vec![left,right]
        } else {
            vec![n*2024]
        }
    }
}
fn p1<const ROUNDS: usize>(input: &str) -> usize {
    let ns : Vec<usize> = parse_input_from_str_sep_by(input, " ");
    let mut collection : HashMap<usize,usize> = HashMap::new();
    for n in ns {
        *collection.entry(n).or_default() += 1;
    }
    for _ in 0..ROUNDS {
        let mut new_collection = HashMap::new();
        for (k,v) in collection.into_iter() {
            for n in  blink(k) {
                *new_collection.entry(n).or_default() += v;
            }
        }
        collection = new_collection;
    }

    collection.values().sum()
}

const EG: &str = "125 17";