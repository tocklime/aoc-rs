use aoc_harness::*;

aoc_main!(2021 day 1, generator lines::<usize>, part1 [solve::<1>, solve2::<1>] => 1616, part2 [part2_fastest, solve::<3>, solve2::<3>] => 1645,
          example both EG => (7,5));

fn solve<const N: usize>(input: &[usize]) -> usize {
    input
        .windows(N)
        .map(|x| x.iter().sum::<usize>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn solve2<const N: usize>(input: &[usize]) -> usize {
    input.windows(N + 1).filter(|x| x[N] > x[0]).count()
}

pub fn part2_fastest(inputs: &[usize]) -> usize {
    let mut count = 0;

    for i in 3..inputs.len() {
        if inputs[i - 3] < inputs[i] {
            count += 1;
        }
    }

    count
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
