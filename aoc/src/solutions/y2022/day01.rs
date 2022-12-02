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
    let mut input = input.to_vec();
    input.select_nth_unstable_by(N, |a, b| b.cmp(a));
    input.into_iter().take(N).sum()
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
