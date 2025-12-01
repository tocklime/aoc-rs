use std::{cmp::Reverse, collections::HashMap, str::FromStr, string::ToString};

use aoc_harness::*;
use nom::{
    IResult, Parser, bytes::{
        complete::{tag, take_until},
        streaming::take,
    }, character::complete::{self, newline}, combinator::all_consuming, multi::{many_m_n, many1, separated_list1}, sequence::terminated
};
use utils::{collections::VecLookup, numset::NumSet};

aoc_harness::aoc_main!(2022 day 16, generator whole_input_is::<X>, both [p2a] => (1728, 2304), example both EG => (1651,1707));

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
    let (input, (_, name, _, rate, _, connections)) = (
        tag("Valve "),
        take(2_usize),
        tag(" has flow rate="),
        complete::u32,
        many_m_n(5, 5, (take_until(" "), tag(" "))),
        separated_list1(tag(", "), take(2_usize)),
    ).parse(input)?;
    Ok((
        input,
        Valve {
            name: name.to_string(),
            id: 0,
            rate,
            connections: connections.into_iter().map(ToString::to_string).collect(),
        },
    ))
}

#[derive(Debug)]
struct X {
    valves: Vec<Valve>,
    connections: VecLookup<VecLookup<usize>>,
    max_flow: u32,
}
impl FromStr for X {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, mut valves) = all_consuming(many1(terminated(parse_line, newline))).parse(s).unwrap();
        valves.sort_by_key(|v| (v.name != "AA", Reverse(v.rate)));
        for (ix, v) in valves.iter_mut().enumerate() {
            v.id = ix as u8;
        }
        let map: HashMap<String, u8> = valves.iter().map(|v| (v.name.clone(), v.id)).collect();

        //need a graph of shortest paths from start and interesting vents to interesting vents.
        //vents: BB, CC, DD, EE, HH, JJ
        let targets = valves
            .iter()
            .filter(|v| v.rate > 0)
            .map(|x| x.id)
            .collect_vec();
        let mut connections: VecLookup<VecLookup<usize>> = VecLookup::default();
        let min_path = pathfinding::directed::dijkstra::dijkstra_all(&0, |&l| {
            valves[l as usize]
                .connections
                .iter()
                .map(|t| (map[&t[..]], 1_usize))
        });
        let p = min_path
            .into_iter()
            .filter_map(|(t, (_, cost))| targets.contains(&t).then_some((t as usize, cost)))
            .collect();
        connections.insert(0, p);
        for &start in &targets {
            let min_path = pathfinding::directed::dijkstra::dijkstra_all(&start, |&l| {
                valves[l as usize]
                    .connections
                    .iter()
                    .map(|t| (map[&t[..]], 1))
            });
            let p = min_path
                .into_iter()
                .filter_map(|(t, (_, cost))| targets.contains(&t).then_some((t as usize, cost)))
                .collect();
            connections.insert(start as usize, p);
        }
        let max_flow = valves.iter().map(|x| x.rate).sum();
        Ok(Self {
            valves,
            connections,
            max_flow,
        })
    }
}
//DpType maps open valves -> location -> cooldown_remaining -> flow.
//sizes:           65535     15          10?
type DpType<'a> = VecLookup<VecLookup<VecLookup<u32>>>;
impl X {
    fn get_flow_for_minute(&self, open: NumSet<u64>) -> u32 {
        open.iter()
            .map(|ix| self.valves[ix as usize].rate)
            .sum::<u32>()
    }

    fn do_time_step<'a>(&'a self, old_dp: &DpType<'a>) -> DpType<'a> {
        let mut new_dp: DpType = VecLookup::default();

        macro_rules! update {
            ($loc:expr, $cooldown:expr, $oldflow:expr, $oldopen:expr, $newopen:expr) => {
                let newopen_usize = $newopen.inner() as usize;
                let flow = self.get_flow_for_minute($oldopen) + $oldflow;
                let e = new_dp
                    .entry(newopen_usize)
                    .or_default()
                    .entry($loc)
                    .or_default()
                    .entry($cooldown)
                    .or_default();
                if flow > *e {
                    *e = flow;
                }
            };
        }

        for (open, stuff) in old_dp {
            for (loc, stuff) in stuff {
                for (cooldown, &old_flow) in stuff {
                    let open = NumSet::from(open as u64);
                    //what can we do from here?
                    if cooldown > 0 {
                        //only wait for cooldown (travel time).
                        update!(loc, cooldown - 1, old_flow, open, open);
                    } else if self.get_flow_for_minute(open) == self.max_flow {
                        //if everything is open, then stand still.
                        update!(loc, 0, old_flow, open, open);
                    } else {
                        //can we open?
                        if !open.contains(loc as u8) && self.valves[loc].rate > 0 {
                            update!(loc, 0, old_flow, open, open.with(loc as u8));
                        }
                        //move.
                        for (target, &cost) in &self.connections[loc] {
                            if !open.contains(target as u8) {
                                update!(target, cost - 1, old_flow, open, open);
                            }
                        }
                    }
                }
            }
        }
        new_dp
    }
}

fn p2a(input: &X) -> (u32, u32) {
    let mut dp: DpType = VecLookup::with_capacity(65535);
    dp.entry(0).or_default().entry(0).or_default().insert(0, 0);
    let dp26 = (1..=26).fold(dp, |old_dp, _| input.do_time_step(&old_dp));

    let bests: Vec<(usize, u32)> = dp26
        .iter()
        .map(|(b, x)| (b, *x.values().flat_map(VecLookup::values).max().unwrap()))
        .collect();

    let p2 = bests
        .iter()
        .cartesian_product(bests.iter())
        .filter_map(|((a, b), (c, d))| (*a & *c == 0).then_some(b + d))
        .max()
        .unwrap();

    let dp30 = (27..=30).fold(dp26, |old_dp, _| input.do_time_step(&old_dp));

    let p1 = *dp30
        .values()
        .flat_map(VecLookup::values)
        .flat_map(VecLookup::values)
        .max()
        .unwrap();
    (p1, p2)
}
