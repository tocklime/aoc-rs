use itertools::Itertools;

aoc_harness_macros::aoc_main!(2021 day 1, generator parse_input, [solve::<1>] => 1616, [solve::<3>] => 1645);

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn solve<const N: usize>(input: &[u32]) -> usize {
    input
        .windows(N)
        .map(|x| x.iter().sum::<u32>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}
