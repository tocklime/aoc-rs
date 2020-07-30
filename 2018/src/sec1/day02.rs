use std::collections::HashMap;
use std::collections::HashSet;

//#[aoc(day2, part1)]
#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut c2 = 0;
    let mut c3 = 0;
    for i in input.lines() {
        let mut hm = HashMap::<char, u32>::new();
        for c in i.chars() {
            *hm.entry(c).or_insert(0) += 1;
        }
        if hm.values().any(|x| *x == 2) {
            c2 += 1;
        }
        if hm.values().any(|x| *x == 3) {
            c3 += 1;
        }
    }
    c2 * c3
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    let mut hs = HashSet::new();
    for l in input.lines() {
        for i in 0..l.len() {
            let (a, b) = l.split_at(i);
            let c = String::from(a) + "_" + &b[1..];
            if !hs.insert(c) {
                return String::from(a) + &b[1..];
            }
        }
    }
    panic!("No answer found");
}
