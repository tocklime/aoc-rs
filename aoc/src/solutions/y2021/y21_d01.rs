use aoc_harness::*;

aoc_main!(2021 day 1, generator lines::<usize>, part1 [solve::<1>] => 1616, part2 [solve::<3>] => 1645,
          example both EG => (7,5));
fn solve<const N: usize>(input: &[usize]) -> usize {
    input.windows(N + 1).filter(|x| x[N] > x[0]).count()
}
const EG: &str = "199
200
208
210
200
207
240
269
260
263";
