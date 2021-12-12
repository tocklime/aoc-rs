use std::collections::HashMap;

use aoc_harness::*;
use utils::numset::NumSet;

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

struct State {
    map: HashMap<u8, NumSet>,
    lower_case_set: NumSet,
    start: u8,
    end: u8,
}
struct Pos {
    visited: NumSet,
    remaining_small_visits: usize,
    pos: u8,
}

impl State {
    fn options(&self, p: &Pos) -> NumSet {
        self.map
            .get(&p.pos)
            .unwrap()
            .iter()
            .map(|x| x as u8)
            .filter(|x| {
                !self.lower_case_set.contains(*x)
                    || !p.visited.contains(*x)
                    || p.remaining_small_visits > 0
            })
            .collect()
    }
    fn step(&self, from: &Pos, to: u8) -> Pos {
        let mut visited = from.visited;
        let small_visit_delta: usize =
            (self.lower_case_set.contains(to) && !visited.insert(to)).into();
        Pos {
            visited,
            remaining_small_visits: from.remaining_small_visits - small_visit_delta,
            pos: to,
        }
    }
    fn explore(&self, p: &Pos) -> usize {
        self.options(p)
            .iter()
            .map(|o| match o as u8 {
                x if x == self.end => 1,
                x if x == self.start => 0,
                x => self.explore(&self.step(&p, x)),
            })
            .sum()
    }
}
fn gen(input: &str) -> State {
    let mut map: HashMap<u8, NumSet> = HashMap::new();
    let mut str_to_num: HashMap<String, u8> = HashMap::new();
    let mut get_num = |x| {
        let l = str_to_num.len().try_into().unwrap();
        *str_to_num.entry(x).or_insert(l)
    };
    for l in input.lines() {
        let mut i = l.split('-');
        let a = get_num(i.next().unwrap().to_string());
        let b = get_num(i.next().unwrap().to_string());
        map.entry(a).or_default().insert(b);
        map.entry(b).or_default().insert(a);
    }
    let lower_case_set: NumSet = str_to_num
        .iter()
        .filter(|(a, _)| a.chars().next().unwrap().is_ascii_lowercase())
        .map(|a| *a.1)
        .collect();
    let start = *str_to_num.get("start").unwrap();
    let end = *str_to_num.get("end").unwrap();
    State {
        map,
        lower_case_set,
        start,
        end,
    }
}

fn solve<const SMALL_VISITS: usize>(state: &State) -> usize {
    let start = Pos {
        visited: NumSet::new(),
        remaining_small_visits: SMALL_VISITS,
        pos: state.start,
    };
    state.explore(&start)
}
