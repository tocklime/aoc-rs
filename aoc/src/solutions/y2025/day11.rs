use std::collections::{HashMap, VecDeque, hash_map::Entry};

use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
};
use utils::{collections::VecLookup, nom::NomError};

aoc_harness::aoc_main!(2025 day 11, generator generate, part1 [p1] => 571, part2 [p2, p2a] => 511_378_159_390_560, example part1 EG => 5, example part2 EG2 => 2);

struct Prob<'input> {
    name_to_id: HashMap<&'input str, usize>,
    map: VecLookup<Vec<usize>>,
    topo_order: Vec<usize>,
    id_to_topo_ix: VecLookup<usize>,
}
impl<'input> Prob<'input> {
    fn parser() -> impl Parser<&'input str, Output = Self, Error = NomError<'input>> {
        all_consuming(separated_list1(
            newline::<_, NomError>,
            separated_pair(alpha1, tag(": "), separated_list1(tag(" "), alpha1)),
        ))
        .map(Self::new)
    }
    fn new(d: Vec<(&'input str, Vec<&'input str>)>) -> Self {
        let mut name_to_id = HashMap::new();
        let mut map: VecLookup<Vec<usize>> = VecLookup::default();
        let mut indegree: VecLookup<usize> = VecLookup::default();
        let mut next_id = 0;
        let mut id_to_topo_ix = VecLookup::default();

        let mut get_id = |s| match name_to_id.entry(s) {
            Entry::Occupied(occupied_entry) => *occupied_entry.get(),
            Entry::Vacant(vacant_entry) => {
                let new_id = next_id;
                vacant_entry.insert(new_id);
                next_id += 1;
                new_id
            }
        };
        for (n, nexts) in d {
            let id = get_id(n);
            let e = map.entry(id).or_default();
            for b in nexts {
                let b_id = get_id(b);
                e.push(b_id);
                *indegree.entry(b_id).or_default() += 1;
            }
        }
        let mut q: VecDeque<usize> = (0..next_id)
            .filter(|a| !indegree.contains_key(*a))
            .collect();
        let mut topo_order = Vec::new();
        while let Some(n) = q.pop_front() {
            id_to_topo_ix.insert(n, topo_order.len());
            topo_order.push(n);
            for &next in map.get(n).into_iter().flatten() {
                let e = indegree.entry(next).or_default();
                assert!(*e > 0);
                *e -= 1;
                if *e == 0 {
                    q.push_back(next);
                }
            }
        }
        Self {
            name_to_id,
            topo_order,
            id_to_topo_ix,
            map,
        }
    }
    fn routes_from(&self, from_n: &str) -> VecLookup<usize> {
        let from = self.name_to_id[from_n];
        let mut route_counts: VecLookup<usize> = [(from, 1)].into_iter().collect();
        let start = self.id_to_topo_ix[from];
        for &n in &self.topo_order[start..] {
            let count = route_counts.get(n).copied().unwrap_or_default();
            for &next in self.map.get(n).into_iter().flatten() {
                *route_counts.entry(next).or_default() += count;
            }
        }
        route_counts
    }
    fn routes_from_to(&self, from_n: &str, to_n: &str) -> usize {
        let from = self.name_to_id[from_n];
        let to = self.name_to_id[to_n];
        let mut route_counts: VecLookup<usize> = [(from, 1)].into_iter().collect();
        let start = self.id_to_topo_ix[from];
        for &n in &self.topo_order[start..] {
            let count = route_counts.get(n).copied().unwrap_or_default();
            if n == to {
                return count;
            }
            for &next in self.map.get(n).into_iter().flatten() {
                *route_counts.entry(next).or_default() += count;
            }
        }
        0
    }
}

fn generate<'a>(input: &'a str) -> Prob<'a> {
    Prob::parser().parse(input.trim()).unwrap().1
}

fn p1(prob: &Prob) -> usize {
    prob.routes_from("you")[prob.name_to_id["out"]]
}
fn p2a(prob: &Prob) -> usize {
    [["svr", "dac", "fft", "out"], ["svr", "fft", "dac", "out"]]
        .iter()
        .map(|r| {
            r.iter()
                .tuple_windows()
                .map(|(&a, &b)| prob.routes_from_to(a, b))
                .try_fold(1, |a, b| (b > 0).then_some(a * b))
                .unwrap_or_default()
        })
        .sum()
}
fn p2(prob: &Prob) -> usize {
    let routes = [["svr", "dac", "fft", "out"], ["svr", "fft", "dac", "out"]];
    let mut routes_from_map = HashMap::new();
    let mut total = 0;
    for r in routes {
        let mut count = 1;
        for (&a, &b) in r.iter().tuple_windows() {
            let map = routes_from_map
                .entry(a)
                .or_insert_with(|| prob.routes_from(a));
            count *= map.get(prob.name_to_id[b]).copied().unwrap_or_default();
            if count == 0 {
                break;
            }
        }
        total += count;
    }
    total
}

const EG: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

const EG2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
