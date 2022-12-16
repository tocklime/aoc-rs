use std::collections::HashMap;

use aoc_harness::*;
use nom::{
    bytes::{complete::tag, streaming::take},
    character::complete::{self, newline},
    combinator::{all_consuming, opt},
    multi::{many1, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};
use utils::numset::NumSet;

aoc_main!(2022 day 16, part1 [p1] => 1728, part2 [p2a] => 2304, example both EG => (1651,1707));

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
fn parse_line(input: &str) -> IResult<&str, Valve> {
    let (input, (_, name, _, rate, _, _, _, _, _, _, _, connections)) = tuple((
        tag("Valve "),
        take(2_usize),
        tag(" has flow rate="),
        complete::u32,
        tag("; tunnel"),
        opt(tag("s")),
        tag(" lead"),
        opt(tag("s")),
        tag(" to valve"),
        opt(tag("s")),
        tag(" "),
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
fn p1(input: &str) -> u32 {
    let (_, mut valves) = all_consuming(many1(terminated(parse_line, newline)))(input).unwrap();
    for (ix, v) in valves.iter_mut().enumerate() {
        v.id = ix as u8;
    }
    let map: HashMap<&str, &Valve> = valves.iter().map(|v| (&v.name[..], v)).collect();
    //need a graph of shortest paths from start and interesting vents to interesting vents.
    //vents: BB, CC, DD, EE, HH, JJ
    let targets = valves
        .iter()
        .filter(|v| v.rate > 0)
        .map(|x| &x.name[..])
        .collect_vec();
    let mut paths: HashMap<&str, HashMap<&str, u32>> = HashMap::new();
    let min_path = pathfinding::directed::dijkstra::dijkstra_all(&"AA", |l| {
        map[l].connections.iter().map(|t| (&t[..], 1))
    });
    let p = min_path
        .into_iter()
        .filter_map(|(t, (_, cost))| {
            if targets.contains(&t) {
                Some((t, cost))
            } else {
                None
            }
        })
        .collect();
    paths.insert("AA", p);

    for &start in &targets {
        let min_path = pathfinding::directed::dijkstra::dijkstra_all(&start, |l| {
            map[l].connections.iter().map(|t| (&t[..], 1))
        });
        let p = min_path
            .into_iter()
            .filter_map(|(t, (_, cost))| {
                if targets.contains(&t) {
                    Some((t, cost))
                } else {
                    None
                }
            })
            .collect();
        paths.insert(start, p);
    }
    let start = ("AA", 30, NumSet::<u64>::new(), 0);
    let all = pathfinding::directed::dijkstra::dijkstra_all(
        &start,
        move |&(loc, time, on_valves, vented)| {
            let here = map[loc];
            let targets = &paths[loc];
            let mut options = Vec::new();
            if time > 0 {
                if !on_valves.contains(here.id) && here.rate > 0 {
                    let mut new_on = on_valves;
                    new_on.insert(here.id);
                    options.push(((loc, time - 1, new_on, vented + here.rate * (time - 1)), 1));
                }
                for (&t, &c) in targets.iter() {
                    if time > c {
                        options.push(((t, time - c, on_valves, vented), 1));
                    }
                }
            }
            options
        },
    );
    let max = all.iter().max_by_key(|x| x.0 .3);
    max.unwrap().0 .3
}

fn p2a(input: &str) -> u32 {
    let (_, mut valves) = all_consuming(many1(terminated(parse_line, newline)))(input).unwrap();
    for (ix, v) in valves.iter_mut().enumerate() {
        v.id = ix as u8;
    }
    let map: HashMap<&str, &Valve> = valves.iter().map(|v| (&v.name[..], v)).collect();
    let mut dp: HashMap<(&str, NumSet<u64>), u32> = HashMap::new();
    dp.insert(("AA", NumSet::new()), 0);
    let get_flow_for_minute =
        |open: NumSet<u64>| open.iter().map(|ix| valves[ix as usize].rate).sum::<u32>();
    let dp = (1..=26).fold(dp, |old_dp, _| {
        let mut new_dp = HashMap::new();
        for (loc, v) in &map {
            let id = map[loc].id;
            //what is doable at time t location v?
            //move from locations, keep the same rate.
            for a in &map[loc].connections {
                let prevs = old_dp
                    .iter()
                    .filter(|((ol, _), _)| ol == a)
                    .map(|((_, open), flow)| (*open, *flow))
                    .collect_vec();
                for (open, oldflow) in prevs {
                    let x = new_dp.entry((&v.name[..], open)).or_default();
                    let flow_now = get_flow_for_minute(open);
                    let flow = flow_now + oldflow;
                    if flow > *x {
                        *x = flow;
                    }
                }
            }
            //open
            if map[loc].rate > 0 {
                let prevs = old_dp
                    .iter()
                    .filter(|((ol, open), _)| {
                        ol == loc && !open.contains(id)
                    })
                    .map(|((_, open), flow)| (*open, *flow))
                    .collect_vec();
                for (open, oldflow) in prevs {
                    let mut newopen = open;
                    newopen.insert(id);
                    let x = new_dp.entry((&v.name[..], newopen)).or_default();
                    let flow_now = get_flow_for_minute(open);
                    let flow = flow_now + oldflow;
                    if flow > *x {
                        *x = flow;
                    }
                }
            }
        }
        new_dp
    });
    let mut p2 = 0;
    let mut bests = HashMap::new();
    for (k,v) in dp {
        let e = bests.entry(k.1).or_default();
        if v > *e {
            *e = v;
        }
    }
    for (a, b) in bests.iter().cartesian_product(bests.iter()) {
        if *a.0 & *b.0 == NumSet::new() && a.1 + b.1 > p2 {
            p2 = a.1 + b.1;
        }
    }
    p2
}
