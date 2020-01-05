#![allow(clippy::redundant_pattern_matching)]

use reformation::Reformation;
use std::cmp::min;
use nom::lib::std::collections::HashMap;

#[derive(Reformation, Debug)]
#[reformation(r"{name} can fly {speed} km/s for {active} seconds, but then must rest for {rest} seconds.")]
struct Line<'a> {
    name: &'a str,
    speed: u32,
    active: u32,
    rest: u32,
}

impl Line<'_> {
    fn distance_after(&self, t: u32) -> u32 {
        let cycle = self.active + self.rest;
        let full_cycles = t / cycle;
        let last_cycle_active = min(t % cycle, self.active);
        full_cycles * self.speed * self.active + self.speed * last_cycle_active
    }
}

fn gen(input: &str) -> Vec<Line> {
    input.lines().map(|x| Line::parse(x).unwrap()).collect()
}

#[aoc(day14, part1)]
pub fn p1(input: &str) -> u32 {
    let l = gen(input);
    l.into_iter().map(|x| x.distance_after(2503)).max().unwrap()
}

#[aoc(day14, part2)]
pub fn p2(input: &str) -> u32 {
    let l = gen(input);
    let mut scores: HashMap<&str, u32> = HashMap::new();
    for s in 1..=2503 {
        let distances: HashMap<_, _> = l.iter().map(|x| (x.name, x.distance_after(s))).collect();
        let max = *distances.values().max().unwrap();
        for (n, v) in distances {
            if v == max {
                *scores.entry(n).or_default() += 1;
            }
        }
    }
    *scores.values().max().unwrap()
}
