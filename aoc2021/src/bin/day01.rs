use aoc_harness::*;

aoc_main!(2021 day 1, generator lines::<u32>, [solve::<1>] => 1616, [solve::<3>] => 1645);

fn solve<const N: usize>(input: &[u32]) -> usize {
    input
        .windows(N)
        .map(|x| x.iter().sum::<u32>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}