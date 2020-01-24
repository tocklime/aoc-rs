use itertools::Itertools;
use std::collections::{VecDeque,HashMap};
use crypto::md5::Md5;
use crypto::digest::Digest;

fn groups(i: &str, group_size: usize) -> Vec<char> {
    let gs = i.chars().group_by(|c| *c);
    gs.into_iter().filter_map(|(k, g)| if g.count() >= group_size { Some(k) } else { None })
        .collect()
}

fn hash(input: &str, n: usize,reps:usize) -> String {
    let mut s = String::new() + input + &n.to_string();
    let mut m = Md5::new();
    for _ in 0..reps {
        m.input_str(&s);
        s = m.result_str();
        m.reset();
    }
    s
}

fn solve(input: &str,hash_reps:usize) -> usize {
    let mut memory = HashMap::new();
    let mut hashes : Vec<String> = Vec::new();
    let mut ix = 0;
    let mut hi_ix = 0;
    let mut answers = Vec::new();
    while answers.len() <= 64 {
        while hi_ix < ix + 1000 {
            let h = hash(input, hi_ix,hash_reps);
            for c in groups(&h, 5) {
                memory.entry(c).or_insert_with(VecDeque::new).push_back(hi_ix);
            }
            hashes.push(h);
            hi_ix += 1;
        }
        for (_, v) in memory.iter_mut() {
            while !v.is_empty() && v[0] <= ix {
                v.pop_front();
            }
        }
        let h = &hashes[ix];
        for &t in groups(&h, 3).iter().take(1) {
            if memory.entry(t).or_insert_with(VecDeque::new).len() > 0 {
                answers.push(ix);
            }
        }
        ix += 1;
    }
    answers[63]
}

#[aoc(day14, part1)]
#[post(ret == 23769)]
fn p1(input: &str) -> usize {
    solve(input,1)
}
#[aoc(day14, part2)]
#[post(ret == 20606)]
fn p2(input: &str) -> usize {
    solve(input,2017)
}


#[test]
fn testd14() {
    assert_eq!(p1("abc"), 22728);
}