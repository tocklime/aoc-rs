#![warn(clippy::all)]
use std::collections::HashMap;
use std::collections::HashSet;

#[aoc(day2, part1)]
pub fn p1(input: &str) -> usize {
    let mut twos = 0;
    let mut threes = 0;
    for l in input.lines() {
        let mut hm: HashMap<char, usize> = HashMap::new();
        for c in l.chars() {
            *hm.entry(c).or_default() += 1;
        }
        if hm.values().any(|x| *x == 2) {
            twos += 1;
        }
        if hm.values().any(|x| *x == 3) {
            threes += 1;
        }
    }
    twos * threes
}

#[aoc(day2, part2)]
pub fn p2(input: &str) -> String {
    let mut hs = HashSet::new();
    for l in input.lines() {
        for p in 0..l.len() {
            let (pre, post) = l.split_at(p);
            let c = pre.to_owned() + "_" + &post[1..];
            if !hs.insert(c.clone()) {
                let ans = c.replace("_", "");
                return ans;
            }
        }
    }
    panic!("Not found");
}
