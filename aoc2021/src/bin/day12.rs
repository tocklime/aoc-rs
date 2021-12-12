use std::{collections::HashMap, str::FromStr};

use aoc_harness::*;
use utils::numset::NumSet;

aoc_main!(2021 day 12, generator whole_input_is::<State>, part1 [solve::<0>], part2 [solve::<1>], example part1 EG => 10, example part2 EG => 36, example part2 EG2 => 103);

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

#[derive(Debug)]
struct State {
    map: Vec<Vec<u8>>,
    start: u8,
    end: u8,
}
struct Pos {
    visited: NumSet<u32>,
    remaining_revisits: usize,
    pos: u8,
}

impl State {
    fn neighbours<'a>(&'a self, p: &'a Pos) -> impl Iterator<Item = Pos> + '_ {
        self.map[p.pos as usize].iter().copied().filter_map(|x| {
            let mut visited = p.visited;
            let small_visit_delta: usize = (!visited.insert(x)).into();
            Some(Pos {
                visited,
                remaining_revisits: p.remaining_revisits.checked_sub(small_visit_delta)?,
                pos: x,
            })
        })
    }
    fn explore(&self, p: Pos) -> usize {
        let mut ans = 0;
        for n in self.neighbours(&p) {
            if n.pos == self.end {
                ans += 1;
            } else {
                ans += self.explore(n);
            }
        }
        ans
    }
}

impl FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut full_map: Vec<Vec<u8>> = Vec::new();
        let mut str_to_num: HashMap<String, u8> = HashMap::new();
        let mut room_count = 0;
        for l in s.lines() {
            let i = l
                .split('-')
                .map(|n| {
                    {
                        *str_to_num.entry(n.to_string()).or_insert_with(|| {
                            let a = room_count;
                            room_count += 1;
                            full_map.push(Vec::new());
                            a
                        })
                    }
                })
                .collect_vec();
            full_map[i[0] as usize].push(i[1]);
            full_map[i[1] as usize].push(i[0]);
        }
        let lower_case_set: NumSet<u32> = str_to_num
            .iter()
            .filter(|(a, _)| a.chars().next().unwrap().is_ascii_lowercase())
            .map(|a| *a.1)
            .collect();

        //for every a->B, for every B->c, map a-c and remove a->B.
        let mut removed_bigs_map: Vec<Vec<u8>> = vec![Vec::new(); full_map.len()];
        let start = *str_to_num.get("start").unwrap();
        let end = *str_to_num.get("end").unwrap();
        for from in 0..str_to_num.len() {
            if lower_case_set.contains(from as u8) {
                let mut targets = Vec::new();
                for &to in full_map[from].iter() {
                    if lower_case_set.contains(to) {
                        targets.push(to);
                    } else {
                        targets.extend(&full_map[to as usize]);
                    }
                }
                targets.retain(|&x| x != start);
                removed_bigs_map[from] = targets
            }
        }

        Ok(State {
            map: removed_bigs_map,
            start,
            end,
        })
    }
}

fn solve<const SMALL_VISITS: usize>(state: &State) -> usize {
    state.explore(Pos {
        visited: NumSet::new(),
        remaining_revisits: SMALL_VISITS,
        pos: state.start,
    })
}
