use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
};

use aoc_harness::*;
use itertools::MinMaxResult;

aoc_main!(2021 day 14, generator whole_input_is::<Day14>, part1 [solve::<10>] => 3284, part2 [ solve::<40>] => 4_302_675_529_689,
        example both EG => (1588, 2_188_189_693_529_usize));

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

const CHAR_COUNT: usize = 10;
const PAIR_COUNT: usize = 100;
struct Day14 {
    rules: [(usize, usize); PAIR_COUNT],
    start: [usize; PAIR_COUNT],
}
impl FromStr for Day14 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lookup = HashMap::new();
        let mut char_count = 0;
        for c in s.chars().filter(|c| char::is_alphabetic(*c)) {
            if let Entry::Vacant(e) = lookup.entry(c) {
                e.insert(char_count);
                char_count += 1;
            }
        }
        let mut x = s.split("\n\n");
        let line1 = x.next().unwrap();
        let mut start = [0; PAIR_COUNT];
        for (a, b) in line1.chars().map(|x| lookup[&x]).tuple_windows() {
            start[(a * CHAR_COUNT + b)] += 1;
        }
        let mut rules = [(0, 0); PAIR_COUNT];
        for l in x.next().unwrap().lines() {
            let mut i = l.split(" -> ");
            let from = i.next().unwrap().chars().map(|a| lookup[&a]).collect_vec();
            let to: usize = lookup[&i.next().unwrap().chars().collect_vec()[0]];
            rules[from[0] * CHAR_COUNT + from[1]] =
                (from[0] * CHAR_COUNT + to, to * CHAR_COUNT + from[1]);
        }
        Ok(Self { rules, start })
    }
}
impl Day14 {
    fn step(&self, i: &[usize]) -> [usize; PAIR_COUNT] {
        let mut ans = [0; PAIR_COUNT];
        for (&(n1, n2), &count) in self.rules.iter().zip(i) {
            ans[n1] += count;
            ans[n2] += count;
        }
        ans
    }
}

fn solve<const ITERS: usize>(input: &Day14) -> usize {
    let curr = (0..ITERS).fold(input.start, |x, _| input.step(&x));
    let mut counts = [0; CHAR_COUNT];
    //we initialise the first to 1, because that was the first elem, and we're about to start only counting the 2nd of each pair.
    counts[0] = 1;
    let minmax = (0..CHAR_COUNT)
        .map(|x| (x == 0) as usize + curr.iter().skip(x).step_by(CHAR_COUNT).sum::<usize>())
        .filter(|&x| x > 0)
        .minmax();
    if let MinMaxResult::MinMax(a, b) = minmax {
        b - a
    } else {
        unreachable!()
    }
}
