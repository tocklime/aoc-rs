use counter::Counter;
use std::iter::once;

#[aoc(day6, part1)]
fn p1a(input: &str) -> String {
    let width = input.lines().nth(0).unwrap().len();
    let mut counts = vec![Counter::<char>::new(); width];
    for l in input.lines() {
        for (p, c) in l.chars().enumerate() {
            counts[p].update(once(c));
        }
    }
    counts.into_iter().map(|c| c.most_common()[0].0).collect()
}

#[aoc(day6, part2)]
fn p2a(input: &str) -> String {
    let width = input.lines().nth(0).unwrap().len();
    let mut counts = vec![Counter::<char>::new(); width];
    for l in input.lines() {
        for (p, c) in l.chars().enumerate() {
            counts[p].update(once(c));
        }
    }
    counts.into_iter().map(|c| c.most_common().last().unwrap().0).collect()
}
