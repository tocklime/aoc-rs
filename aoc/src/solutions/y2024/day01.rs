use itertools::Itertools;

aoc_harness::aoc_main!(2024 day 1, generator gen, part1 [p1] => 2_196_996, part2 [p2] => 23_655_822, example part1 EG => 11, example part2 EG => 31);


fn gen(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();
    for l in input.lines() {
        let (a, b) = l.split_once("   ").unwrap();
        left.push(a.parse().unwrap());
        right.push(b.parse().unwrap());
    }
    (left, right)
}
fn p1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let left = input.0.iter().copied().sorted().collect_vec();
    let right = input.1.iter().copied().sorted().collect_vec();
    left.into_iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}
fn p2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let r_counts = input.1.iter().copied().counts();
    input.0
        .iter()
        .map(|l| l * r_counts.get(l).copied().unwrap_or_default())
        .sum()
}

const EG: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
