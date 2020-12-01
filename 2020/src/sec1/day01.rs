use itertools::Itertools;
#[aoc_generator(day1)]
pub fn gen(input:&str) -> Vec<i64>{
    input
        .trim()
        .lines()
        .map(|l| {
            let n: i64 = l[..].parse().expect("Integer");
            n
        })
        .collect()
}
pub fn solve(input: &[i64], size: usize) -> i64 {
    input.iter()
        .combinations(size)
        .filter(|x| x.iter().cloned().sum::<i64>() == 2020)
        .nth(0)
        .unwrap()
        .iter()
        .cloned()
        .product()
}

#[aoc(day1,part1)]
pub fn p1(input: &[i64]) -> i64 { solve(input,2) }

#[aoc(day1,part2)]
pub fn p2(input: &[i64]) -> i64 { solve(input,3) }