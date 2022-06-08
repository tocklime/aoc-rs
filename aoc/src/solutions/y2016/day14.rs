use aoc_harness::aoc_main;

aoc_main!(2016 day 14, part1 [p1], part2 [p2]);
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

fn groups(i: &str, group_size: usize) -> Vec<char> {
    let gs = i.chars().group_by(|c| *c);
    gs.into_iter()
        .filter_map(|(k, g)| {
            if g.count() >= group_size {
                Some(k)
            } else {
                None
            }
        })
        .collect()
}

fn hash(input: &str, n: usize, reps: usize) -> String {
    let mut s = String::new() + input + &n.to_string();
    for _ in 0..reps {
        s = format!("{:?}", md5::compute(&s));
    }
    s
}

fn solve(input: &str, hash_reps: usize) -> usize {
    let mut memory = HashMap::new();
    let mut hashes: Vec<String> = Vec::new();
    let mut ix = 0;
    let mut hi_ix = 0;
    let mut answers = Vec::new();
    while answers.len() <= 64 {
        while hi_ix < ix + 1000 {
            let h = hash(input, hi_ix, hash_reps);
            for c in groups(&h, 5) {
                memory
                    .entry(c)
                    .or_insert_with(VecDeque::new)
                    .push_back(hi_ix);
            }
            hashes.push(h);
            hi_ix += 1;
        }
        for v in memory.values_mut() {
            while !v.is_empty() && v[0] <= ix {
                v.pop_front();
            }
        }
        let h = &hashes[ix];
        for &t in groups(h, 3).iter().take(1) {
            if !memory.entry(t).or_insert_with(VecDeque::new).is_empty() {
                answers.push(ix);
            }
        }
        ix += 1;
    }
    answers[63]
}

fn p1(input: &str) -> usize {
    solve(input.trim(), 1)
}

fn p2(input: &str) -> usize {
    solve(input.trim(), 2017)
}

#[test]
fn testd14() {
    assert!(hash("abc", 18, 1).contains("cc38887a5"));
    assert!(hash("abc", 39, 1).contains("eee"));
    assert!(hash("abc", 92, 1).contains("999"));
    assert!(hash("abc", 200, 1).contains("99999"));
    assert_eq!(p1("abc"), 22728);
}
