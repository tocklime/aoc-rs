use num::Integer;
use rustc_hash::FxHashMap;
use rayon::prelude::*;

aoc_harness::aoc_main!(2023 day 8, generator gen, part1 [p1] => 13019, part2 [p2] => 13_524_038_372_771,
    example part1 EG => 2, example part1 EG2 => 6, example part2 EG3 => 6);

struct Prob {
    directions: String,
    map: FxHashMap<String, (String, String)>,
}
impl Prob {
    fn solve_from(&self, start: &str) -> usize {
        self.directions
            .chars()
            .cycle()
            .try_fold((0, start), |(ix, pos), dir| {
                if pos.ends_with('Z') {
                    return Err(ix);
                }
                let pos = match dir {
                    'R' => &self.map[pos].1,
                    'L' => &self.map[pos].0,
                    _ => panic!("Unknown dir {dir}"),
                };
                Ok((ix + 1, pos))
            })
            .unwrap_err()
    }
}

fn gen(input: &str) -> Prob {
    let mut l = input.lines();
    let rls = l.next().unwrap();
    let _ = l.next().unwrap();
    let map = l
        .map(|l| {
            let (from, to) = l.split_once(" = ").unwrap();
            let (l, r) = to[1..to.len() - 1].split_once(", ").unwrap();
            (from.to_owned(), (l.to_owned(), r.to_owned()))
        })
        .collect();
    Prob {
        directions: rls.to_owned(),
        map,
    }
}
fn p1(input: &Prob) -> usize {
    input.solve_from("AAA")
}

fn p2(input: &Prob) -> usize {
    input
        .map
        .par_iter()
        .filter_map(|x| x.0.ends_with('A').then_some(x.0))
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
