use std::collections::{HashMap, HashSet};

use aoc_harness::*;

aoc_main!(2021 day 3, [p1], [p2] => 587895);

fn p1(input: &str) -> usize {
    let mut position_popularity = HashMap::new();
    let mut len = 0;
    for l in input.lines() {
        len = l.len();
        for (ix, c) in l.chars().enumerate() {
            let mut x = position_popularity.entry(ix).or_insert((0, 0));
            match c {
                '0' => x.0 += 1,
                '1' => x.1 += 1,
                _ => unreachable!(),
            };
        }
    }
    let mut eps = 0;
    let mut gamma = 0;
    for (k, v) in position_popularity {
        if v.1 > v.0 {
            eps |= 1 << (len - k - 1)
        } else {
            gamma |= 1 << (len - k - 1)
        }
    }

    eps * gamma
}

fn p2(input: &str) -> usize {
    let mut all_nums = HashSet::new();
    let mut len = 0;
    for l in input.lines() {
        len = l.len();
        let mut this_num = 0;
        for (ix, c) in l.chars().enumerate() {
            if c == '1' {
                this_num |= 1 << (len - ix - 1);
            }
        }
        all_nums.insert(this_num);
    }
    let mut co2 = all_nums.clone();
    let mut ix = 0;
    while co2.len() > 1 {
        let count = co2.len();
        let bit = 1 << len - 1 - (ix % len);
        let pos_1s = co2.iter().filter(|&&x| (x & bit) != 0).count();
        let pos_0s = count - pos_1s;
        let target_bit = if pos_1s >= pos_0s { bit } else { 0 };
        co2 = co2.drain().filter(|x| x & bit == target_bit).collect();
        ix += 1;
    }
    dbg!(&co2);
    let mut oxy = all_nums.clone();
    let mut ix = len;
    while oxy.len() > 1 {
        let count = oxy.len();
        let bit = 1 << len - 1 - (ix % len);
        let pos_1s = oxy.iter().filter(|&&x| x & bit != 0).count();
        let pos_0s = count - pos_1s;
        let target_bit = if pos_0s > pos_1s { bit } else { 0 };
        dbg!(&oxy, bit, pos_1s, pos_0s, target_bit);
        oxy = oxy.drain().filter(|x| x & bit == target_bit).collect();
        ix += 1;
    }
    dbg!(&oxy);
    co2.iter().next().unwrap() * oxy.iter().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EG: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    pub fn tp1() {
        assert_eq!(p1(EG), 198);
    }
    #[test]
    pub fn tp2() {
        assert_eq!(p2(EG), 230);
    }
}
