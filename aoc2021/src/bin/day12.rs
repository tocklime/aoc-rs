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
    map: [NumSet<u32>; 256],
    lower_case_set: NumSet<u32>,
    start: u8,
    end: u8,
}
struct Pos {
    visited: NumSet<u32>,
    remaining_small_visits: usize,
    pos: u8,
}

impl State {
    fn options<'a>(&'a self, p: &'a Pos) -> impl Iterator<Item = u8> + '_ {
        self.map[p.pos as usize]
            .iter()
            .filter(|&x| {
                x != self.start && (
                !self.lower_case_set.contains(x)
                    || !p.visited.contains(x)
                    || p.remaining_small_visits > 0)
            })
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
    fn explore_iter(&self, p: Pos) -> usize {
        let mut stack = vec![p];
        let mut solutions = 0;
        while let Some(p) = stack.pop() {
            if p.pos == self.end {
                solutions += 1;
            } else {
                stack.extend(self.options(&p).map(|to| self.step(&p,to)))
            }
        }
        solutions
    }
}
fn gen(input: &str) -> State {
    let mut map: [NumSet<u32>;256] = [NumSet::new(); 256];
    let mut str_to_num: HashMap<String, u8> = HashMap::new();
    let mut get_num = |x| {
        let l = str_to_num.len().try_into().unwrap();
        *str_to_num.entry(x).or_insert(l)
    };
    for l in input.lines() {
        let mut i = l.split('-');
        let a = get_num(i.next().unwrap().to_string());
        let b = get_num(i.next().unwrap().to_string());
        map[a as usize].insert(b);
        map[b as usize].insert(a);
    }
    let lower_case_set: NumSet<u32> = str_to_num
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
    state.explore_iter(start)
}
