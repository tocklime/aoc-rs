use std::{collections::HashMap, str::FromStr};

use aoc_harness::*;
use nom::{
    bytes::{
        complete::{tag, take_until},
        streaming::take,
    },
    character::complete::{self, newline},
    combinator::all_consuming,
    multi::{many1, many_m_n, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};
use utils::{numset::NumSet, collections::VecLookup};

aoc_main!(2022 day 16, both [p2a] => (1728, 2304), example both EG => (1651,1707));

const EG: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

#[derive(Debug)]
struct Valve {
    name: String,
    id: u8,
    rate: u32,
    connections: Vec<String>,
}
//Valve JJ has flow rate=21; tunnel leads to valve II
fn parse_line(input: &str) -> IResult<&str, Valve> {
    let (input, (_, name, _, rate, _, connections)) = tuple((
        tag("Valve "),
        take(2_usize),
        tag(" has flow rate="),
        complete::u32,
        many_m_n(5, 5, tuple((take_until(" "), tag(" ")))),
        separated_list1(tag(", "), take(2_usize)),
    ))(input)?;
    Ok((
        input,
        Valve {
            name: name.to_string(),
            id: 0,
            rate,
            connections: connections.into_iter().map(|x| x.to_string()).collect(),
        },
    ))
}

struct X {
    valves: Vec<Valve>,
    good_moves: VecLookup<VecLookup<u32>>,
    max_flow: u32,
    aa_id: u8,
}
impl FromStr for X {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, mut valves) = all_consuming(many1(terminated(parse_line, newline)))(s).unwrap();
        let mut aa_id = 0;
        for (ix, v) in valves.iter_mut().enumerate() {
            v.id = ix as u8;
            if v.name == "AA" {
                aa_id = ix as u8;
            }
        }
        let map: HashMap<String, u8> = valves.iter().map(|v| (v.name.clone(), v.id)).collect();

        //need a graph of shortest paths from start and interesting vents to interesting vents.
        //vents: BB, CC, DD, EE, HH, JJ
        let targets = valves
            .iter()
            .filter(|v| v.rate > 0)
            .map(|x| x.id)
            .collect_vec();
        let mut good_moves: VecLookup<VecLookup<u32>> = VecLookup::default();
        let min_path = pathfinding::directed::dijkstra::dijkstra_all(&aa_id, |&l| {
            valves[l as usize]
                .connections
                .iter()
                .map(|t| (map[&t[..]], 1))
        });
        let p = min_path
            .into_iter()
            .filter_map(|(t, (_, cost))| {
                if targets.contains(&t) {
                    Some((t as usize, cost))
                } else {
                    None
                }
            })
            .collect();
        good_moves.insert(aa_id as usize, p);
        for &start in &targets {
            let min_path = pathfinding::directed::dijkstra::dijkstra_all(&start, |&l| {
                valves[l as usize]
                    .connections
                    .iter()
                    .map(|t| (map[&t[..]], 1))
            });
            let p = min_path
                .into_iter()
                .filter_map(|(t, (_, cost))| {
                    if targets.contains(&t) {
                        Some((t as usize, cost))
                    } else {
                        None
                    }
                })
                .collect();
            good_moves.insert(start as usize, p);
        }
        let max_flow = valves.iter().map(|x| x.rate).sum();
        Ok(Self {
            valves,
            good_moves,
            max_flow,
            aa_id
        })
    }
}
//DpType maps location -> cooldown_remaining -> Open valves -> flow.
type DpType<'a> = HashMap<(u8, u32, NumSet<u64>), u32>;
impl X {
    fn get_flow_for_minute(&self, open: NumSet<u64>) -> u32 {
        open.iter()
            .map(|ix| self.valves[ix as usize].rate)
            .sum::<u32>()
    }

    fn do_time_step<'a, 'b>(&'a self, old_dp: &'b DpType<'a>) -> DpType<'a> {
        let mut new_dp: DpType = HashMap::new();

        macro_rules! update {
            ($loc:expr, $cooldown:expr, $oldflow:expr, $oldopen:expr, $newopen:expr) => {
                let e = new_dp.entry(($loc, $cooldown, $newopen)).or_default();
                let flow = self.get_flow_for_minute($oldopen) + $oldflow;
                if flow > *e {
                    *e = flow;
                }
            };
        }

        for (&(loc, cooldown, open), &old_flow) in old_dp {
            //what can we do from here?
            if cooldown > 0 {
                //only wait for cooldown (travel time).
                update!(loc, cooldown - 1, old_flow, open, open);
            } else if self.get_flow_for_minute(open) == self.max_flow {
                //if everything is open, then stand still.
                update!(loc, 0, old_flow, open, open);
            } else {
                //can we open?
                let id = loc;
                if !open.contains(id) && self.valves[id as usize].rate > 0 {
                    update!(loc, 0, old_flow, open, open.with(id));
                }
                //move.
                for (target, &cost) in &self.good_moves[loc as usize] {
                    update!(target as u8, cost - 1, old_flow, open, open);
                }
            }
        }
        new_dp
    }
}

fn p2a(input: &str) -> (u32, u32) {
    let input: X = input.parse().unwrap();
    let mut dp: DpType = HashMap::new();
    dp.insert((input.aa_id, 0, NumSet::new()), 0);
    let dp26 = (1..=26).fold(dp, |old_dp, _| input.do_time_step(&old_dp));
    let dp30 = (27..=30).fold(dp26.clone(), |old_dp, _| input.do_time_step(&old_dp));

    let p1 = *dp30.values().max().unwrap();
    let mut p2 = 0;
    let mut bests = HashMap::new();
    for (k, v) in dp26 {
        let e = bests.entry(k.2).or_default();
        if v > *e {
            *e = v;
        }
    }
    for (a, b) in bests.iter().cartesian_product(bests.iter()) {
        if *a.0 & *b.0 == NumSet::new() && a.1 + b.1 > p2 {
            p2 = a.1 + b.1;
        }
    }
    (p1, p2)
}
