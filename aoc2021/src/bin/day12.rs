use std::collections::{HashMap, HashSet};

use aoc_harness::*;

aoc_main!(2021 day 12, generator gen, part1 [solve::<0>], part2 [solve::<1>], example part1 EG => 10, example part2 EG => 36, example part2 EG2 => 103);

const EG: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
const EG2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

struct State<'a> {
    map: &'a HashMap<String, Vec<String>>,
    visited: HashSet<&'a str>,
    remaining_small_visits: usize,
    pos: &'a str,
}
fn is_lower(s: &str) -> bool {
    s.chars().next().unwrap().is_ascii_lowercase()
}
impl<'a> State<'a> {
    fn new(map: &'a HashMap<String, Vec<String>>, remaining_small_visits: usize) -> Self {
        Self {
            map,
            visited: HashSet::new(),
            remaining_small_visits,
            pos: "start",
        }
    }
    fn options(&'a self) -> impl Iterator<Item = &'a str> {
        const EMPTY: &Vec<String> = &Vec::new();
        self.map
            .get(self.pos)
            .unwrap_or(EMPTY)
            .iter()
            .map(|x| &x[..])
            .filter(|x| {
                !is_lower(x) || !self.visited.contains(x) || self.remaining_small_visits > 0
            })
    }
    fn step(&'a self, to: &'a str) -> Self {
        let mut visited = self.visited.clone();
        let small_visit_delta: usize = (is_lower(to) && !visited.insert(to)).into();
        Self {
            map: self.map,
            visited,
            remaining_small_visits: self.remaining_small_visits - small_visit_delta,
            pos: to,
        }
    }
    fn explore(&'a mut self) -> usize {
        self.options()
            .map(|o| match o {
                "end" => 1,
                "start" => 0,
                _ => self.step(o).explore(),
            })
            .sum()
    }
}
fn gen(input: &str) -> HashMap<String, Vec<String>> {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for l in input.lines() {
        let mut i = l.split('-');
        let a = i.next().unwrap().to_string();
        let b = i.next().unwrap().to_string();
        connections.entry(a.clone()).or_default().push(b.clone());
        connections.entry(b).or_default().push(a);
    }
    connections
}

fn solve<const SMALL_VISITS: usize>(connections: &HashMap<String, Vec<String>>) -> usize {
    State::new(connections, SMALL_VISITS).explore()
}
