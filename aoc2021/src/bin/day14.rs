use std::{fmt::Display, str::FromStr};

use aoc_harness::*;
use itertools::MinMaxResult;
use num::Integer;

aoc_main!(2021 day 14, generator whole_input_is::<Day14>, part1 [solve::<10>] => 3284, part2 [solve::<40>] => 4_302_675_529_689, 
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

const CHARS: usize = 26;
const CHAR_PAIRS: usize = CHARS * CHARS;
struct Day14 {
    rules: Vec<((usize, usize), usize)>,
    start: Counts,
    edges: [u8; 2],
}
impl FromStr for Day14 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = s.split("\n\n");
        let line1 = x.next().unwrap();
        let edges = [
            line1.bytes().next().unwrap() - b'A',
            line1.bytes().last().unwrap() - b'A',
        ];
        let start = line1.parse()?;
        let mut rules = Vec::new();
        for l in x.next().unwrap().lines() {
            let mut i = l.split(" -> ");
            let from = i.next().unwrap().bytes().map(|a| a - b'A').collect_vec();
            let to = i.next().unwrap().bytes().collect_vec()[0] - b'A';
            rules.push(((from[0].into(), from[1].into()), to.into()));
        }
        Ok(Self {
            rules,
            start,
            edges,
        })
    }
}
#[derive(Clone)]
struct Counts {
    inner: [usize; CHAR_PAIRS],
}
impl Display for Counts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("  ")?;
        for c in 'A'..='Z' {
            f.write_fmt(format_args!("{} ", c))?;
        }
        for (ix, count) in self.inner.iter().enumerate() {
            if ix % CHARS == 0 {
                f.write_str("\n")?;
                let c: u8 = (ix / CHARS).try_into().unwrap();
                f.write_fmt(format_args!("{} ", (b'A' + c) as char))?;
            }
            f.write_fmt(format_args!("{} ", count))?;
        }
        Ok(())
    }
}
impl FromStr for Counts {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start = s.bytes().map(|x| x - b'A').collect_vec();
        let mut ans = Self::new();
        for (a, b) in start.into_iter().tuple_windows() {
            let ix = a as usize * CHARS + b as usize;
            ans.inner[ix] += 1;
        }
        Ok(ans)
    }
}
impl Counts {
    fn new() -> Self {
        Self {
            inner: [0; CHAR_PAIRS],
        }
    }
}
impl Day14 {
    fn step(&self, i: &Counts) -> Counts {
        let mut ans = Counts::new();
        for &((a, b), c) in &self.rules {
            let count: usize = i.inner[a * CHARS + b];
            ans.inner[a * CHARS + c] += count;
            ans.inner[c * CHARS + b] += count;
        }
        ans
    }
}

fn solve<const ITERS: usize>(input: &Day14) -> usize {
    let curr = (0..ITERS).fold(input.start.clone(), |x, _| input.step(&x));
    let mut counts = [0; CHARS];
    input.edges.iter().for_each(|&e| counts[e as usize] = 1);
    for (ix, count) in curr.inner.into_iter().enumerate() {
        let (a, b) = ix.div_mod_floor(&CHARS);
        counts[a] += count;
        counts[b] += count;
    }
    if let MinMaxResult::MinMax(a, b) = counts.iter().filter(|&&x| x > 0).minmax() {
        (b - a) / 2
    } else {
        unreachable!()
    }
}
