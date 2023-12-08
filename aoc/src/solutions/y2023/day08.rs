use std::collections::HashMap;

use itertools::{FoldWhile, Itertools};
use num::Integer;
use rayon::prelude::*;

aoc_harness::aoc_main!(2023 day 8, generator gen, part1 [p1] => 13019, part2 [p2] => 13_524_038_372_771,
    example part1 EG => 2, example part1 EG2 => 6, example part2 EG3 => 6);

struct Prob {
    directions: String,
    minimum_mid: usize,
    minimum_end: usize,
    map: Vec<(usize, usize)>,
}
impl Prob {
    fn is_end(&self, p: usize) -> bool {
        p >= self.minimum_end
    }
    fn solve_from(&self, start: usize) -> usize {
        self.directions
            .chars()
            .cycle()
            .fold_while((0, start), |(ix, pos), dir| {
                let pos = match dir {
                    'R' => self.map[pos].1,
                    'L' => self.map[pos].0,
                    _ => panic!("Unknown dir {dir}"),
                };
                if self.is_end(pos) {
                    FoldWhile::Done((ix + 1, pos))
                } else {
                    FoldWhile::Continue((ix + 1, pos))
                }
            })
            .into_inner()
            .0
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum NodeType {
    Start,
    Mid,
    End,
}

fn gen(input: &str) -> Prob {
    let mut l = input.lines();
    let rls = l.next().unwrap();
    let _ = l.next().unwrap();
    let get_type = |x: &str| {
        if x.ends_with('A') {
            NodeType::Start
        } else if x.ends_with('Z') {
            NodeType::End
        } else {
            NodeType::Mid
        }
    };
    let name_to_targets = l
        .map(|l| {
            let (from, to) = l.split_once(" = ").unwrap();
            let (l, r) = to[1..to.len() - 1].split_once(", ").unwrap();
            (get_type(from), from, (l, r))
        })
        .sorted()
        .collect::<Vec<_>>();
    let mut minimums = HashMap::new();

    let name_to_ix = name_to_targets
        .iter()
        .zip(0usize..)
        .map(|((typ, name, _), ix)| {
            minimums.entry(typ).or_insert(ix);
            (name, ix)
        })
        .collect::<HashMap<_, _>>();
    let map = name_to_targets
        .iter()
        .map(|(_, _, (l, r))| (name_to_ix[&l], name_to_ix[&r]))
        .collect();
    Prob {
        directions: rls.to_owned(),
        map,
        minimum_mid: minimums[&NodeType::Mid],
        minimum_end: minimums[&NodeType::End],
    }
}
fn p1(input: &Prob) -> usize {
    input.solve_from(0)
}

fn p2(input: &Prob) -> usize {
    (0..input.minimum_mid)
        .into_par_iter()
        .map(|p| input.solve_from(p))
        .reduce(|| 1, |a, b| b.lcm(&a))
}
const EG: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

const EG2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
const EG3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
