

aoc_harness::aoc_main!(2016 day 6, part1 [p1] => "qrqlznrl", part2 [p2] => "kgzdfaon");
use counter::Counter;
use std::iter::once;

fn p1(input: &str) -> String {
    let width = input.lines().next().unwrap().len();
    let mut counts = vec![Counter::<char>::new(); width];
    for l in input.lines() {
        for (p, c) in l.chars().enumerate() {
            counts[p].update(once(c));
        }
    }
    counts.into_iter().map(|c| c.most_common()[0].0).collect()
}

fn p2(input: &str) -> String {
    let width = input.lines().next().unwrap().len();
    let mut counts = vec![Counter::<char>::new(); width];
    for l in input.lines() {
        for (p, c) in l.chars().enumerate() {
            counts[p].update(once(c));
        }
    }
    counts
        .into_iter()
        .map(|c| c.most_common().last().unwrap().0)
        .collect()
}
