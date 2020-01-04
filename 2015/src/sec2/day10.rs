use itertools::Itertools;
use std::str::FromStr;

fn step(input: &[usize]) -> Vec<usize> {
    let mut ans = Vec::new();
    let mut pos = 0;
    while pos < input.len() {
        let c = input[pos];
        let len = input[pos..].iter().take_while(|x| **x == c).count();
        ans.push(len);
        ans.push(c);
        pos += len;
    }
    ans
}

#[aoc(day10, part1)]
fn p1(input: &str) -> usize {
    let as_digits = input.chars().map(|x| usize::from_str(&x.to_string()).unwrap()).collect_vec();
    let ans = (0..40).fold(as_digits, |a,_| step(&a));
    ans.len()
}
#[aoc(day10, part2)]
fn p2(input: &str) -> usize {
    let as_digits = input.chars().map(|x| usize::from_str(&x.to_string()).unwrap()).collect_vec();
    let ans = (0..50).fold(as_digits, |a,_| step(&a));
    ans.len()
}
