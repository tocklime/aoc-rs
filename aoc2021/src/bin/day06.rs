
use std::collections::{BTreeSet, VecDeque};

use aoc_harness::*;

aoc_main!(2021 day 6, [solve::<80>], [solve::<256>],
          example part 1 EG => 5934, example part 2 EG => 26984457539);


fn solve<const GENERATIONS: usize>(input: &str) -> usize {
    dbg!(input);
    let mut v : Vec<usize> = input.trim().split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    dbg!(&v);
    let mut counts:  VecDeque<usize> = (0..=8).map(|_| 0).collect();
    dbg!(&counts);
    for f in v {
        counts[f] += 1;
        dbg!(f,&counts);
    }
    dbg!(&counts);
    for d in 0..GENERATIONS {
        // dbg!(d, &counts, counts.iter().sum::<usize>());
        let new_fish = counts.pop_front().unwrap();
        counts[6] += new_fish;
        counts.push_back(new_fish);
    }
    counts.iter().sum()
}

const EG : &str = "3,4,3,1,2";