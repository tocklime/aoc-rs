use aoc_harness::*;

aoc_main!(2021 day 1, generator lines::<u32>, [solve::<1>, solve2::<1>] => 1616, [solve::<3>, solve2::<3>] => 1645, bench);

fn solve<const N: usize>(input: &[u32]) -> usize {
    input
        .windows(N)
        .map(|x| x.iter().sum::<u32>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn solve2<const N: usize>(input: &[u32]) -> usize {
    input.windows(N + 1).filter(|x| x[N] > x[0]).count()
}
