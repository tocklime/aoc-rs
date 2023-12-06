
use utils::span::Span;

aoc_harness::aoc_main!(2022 day 4, generator gen, part1 [p1], part2 [p2], example part1 EG => 2);

const EG: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";
type T = u8;
fn gen(input: &str) -> Vec<(Span<T>, Span<T>)> {
    input
        .lines()
        .map(|l| {
            let mut nums = l.split(&[',', '-']).map(|n| n.parse::<T>().unwrap());
            (
                Span::new(nums.next().unwrap(), nums.next().unwrap() + 1),
                Span::new(nums.next().unwrap(), nums.next().unwrap() + 1),
            )
        })
        .collect()
}
fn p1(input: &[(Span<T>, Span<T>)]) -> usize {
    input
        .iter()
        .filter(|(r1, r2)| r1.is_entirely_within(r2) || r2.is_entirely_within(r1))
        .count()
}

fn p2(input: &[(Span<T>, Span<T>)]) -> usize {
    input.iter().filter(|(r1, r2)| !r1.is_disjoint(r2)).count()
}
