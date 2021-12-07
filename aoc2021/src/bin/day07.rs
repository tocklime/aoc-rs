use aoc_harness::*;

aoc_main!(2021 day 7, generator input::<isize,','>, part1 [p1] => 352997, part2 [p2] => 101571302,
  example part1 EG => 37,
  example part2 EG => 168,
);
const EG: &str = "16,1,2,0,4,2,7,1,2,14";

fn fuel_to_pos(input: &[isize], pos: isize) -> isize {
    input.iter().map(|x| (x - pos).abs()).sum()
}

fn fuel_to_pos2(input: &[isize], pos: isize) -> isize {
    input
        .iter()
        .map(|x| {
            let steps = (x - pos).abs();
            steps * (steps + 1) / 2
        })
        .sum()
}
fn p1(input: &[isize]) -> isize {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    (min..=max).map(|p| fuel_to_pos(input, p)).min().unwrap()
}

fn p2(input: &[isize]) -> isize {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    (min..=max).map(|p| fuel_to_pos2(input, p)).min().unwrap()
}

