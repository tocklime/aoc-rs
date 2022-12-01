use std::cmp::Reverse;

use aoc_harness::*;

aoc_main!(2022 day 1, generator gen, part1 [solve::<1>] => 67633, part2 [solve::<3>] => 199_628,
        example both EG => (24000, 45000));

fn gen(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|x| x.lines().map(|y| y.parse::<usize>().unwrap()).sum())
        .collect()
}
fn solve<const N: usize>(input: &[usize]) -> usize {
    input.iter().sorted_by_key(|&&k| Reverse(k)).take(N).sum()
}

const EG: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
