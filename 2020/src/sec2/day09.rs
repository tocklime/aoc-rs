use std::collections::VecDeque;
use itertools::Itertools;
use crate::utils::collections::de_prefixsum;

#[aoc(day9,part1)]
pub fn p1(input: &str) -> usize {
    let is = input.lines().map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let mut window = is.iter().copied().take(25).collect::<VecDeque<_>>();
    for x in is.iter().skip(25) {
        let mut found = false;
        for v in window.iter().combinations(2) {
            if v[0] + v[1] == *x {
                found = true;
                break; //found it OK!
            }
        }
        if !found {
            return *x;
        }
        window.push_back(*x);
        window.pop_front();
    }
    0
}

#[aoc(day9,part2)]
pub fn p2(input: &str) -> usize {
    let is = input.lines().map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let target = p1(input);
    let ps = de_prefixsum(&is);
    for v in (0..is.len()).combinations(2) {
        let ix_1 = *v.iter().min().unwrap();
        let ix_2 = *v.iter().max().unwrap();
        if ps[ix_2] - ps[ix_1] == target {
            let cands = is[ix_1+1..=ix_2].iter().collect_vec();
            let a = cands.iter().min().unwrap();
            let b = cands.iter().max().unwrap();
            println!("{}-{}, minmax: {} {}",ix_1,ix_2,a,b);
                return *a+*b;
        }
    }
    0
}