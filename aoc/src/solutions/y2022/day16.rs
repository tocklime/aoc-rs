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

aoc_main!(2022 day 16, part2 [p2a], /*part1 [p1], example part1 EG => 1651*/ example part2 EG => 1707);

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
    dbg!(&paths);
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
    dbg!(max);
    max.unwrap().0 .3
}

fn p2(input: &str) -> u32 {
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
    dbg!(&paths);

    let total_rate: u32 = valves.iter().map(|x| x.rate).sum();
    let all_on: NumSet<u64> = targets.iter().map(|x| map[x].id).collect();
    let start = ([("AA", 0), ("AA", 0)], 26, NumSet::<u64>::new(), 0);
    let all = pathfinding::directed::dijkstra::dijkstra(
        &start,
        |&(locations, time, on_valves, vented)| {
            let mut opts = Vec::new();
            if time > 0 {
                let mut c = Vec::new();
                for (l, cooldown) in locations {
                    let mut my_opts: Vec<(_, Option<u8>)> = Vec::new();
                    if cooldown > 0 {
                        my_opts.push(((l, cooldown - 1), None));
                    } else {
                        let here = map[l];
                        if !on_valves.contains(here.id) && here.rate > 0 {
                            my_opts.push(((l, 0), Some(here.id)));
                        }
                        for (&t, &c) in paths[l].iter() {
                            if time > c {
                                my_opts.push(((t, c - 1), None));
                            }
                        }
                    }
                    c.push(my_opts);
                }
                for (me, ele) in c[0].iter().cartesian_product(&c[1]) {
                    //cost is how much we've not vented.
                    let mut vents = on_valves;
                    if let Some(x) = me.1 {
                        vents.insert(x);
                    }
                    if let Some(x) = ele.1 {
                        vents.insert(x);
                    }
                    let vented_this_turn = valves
                        .iter()
                        .filter(|v| vents.contains(v.id))
                        .map(|x| x.rate)
                        .sum::<u32>();
                    opts.push((
                        ([me.0, ele.0], time - 1, vents, vented + vented_this_turn),
                        total_rate - vented_this_turn,
                    ));
                }
            }
            opts
        },
        |(_, t, v, _)| *t == 0 || *v == all_on,
    )
    .unwrap();
    for (locs, a, b, v) in &all.0 {
        let opened = valves
            .iter()
            .filter(|x| b.contains(x.id))
            .map(|x| &x.name[..])
            .collect::<Vec<_>>();
        println!(
            "At time {}, I am at {:?} and ele at {:?}, opened are {:?}. {}",
            a,
            locs[0],
            locs[1],
            opened.join(", "),
            v
        );
    }

    let last = all.0[all.0.len() - 1];
    let last_but_one = all.0[all.0.len() - 2];
    let delta_on_last_turn = last.3 - last_but_one.3;
    let ans = last.3 + (last.1 - 1) * delta_on_last_turn;

    ans
}

fn p2a(input: &str) -> u32 {
    let (_, mut valves) = all_consuming(many1(terminated(parse_line, newline)))(input).unwrap();
    for (ix, v) in valves.iter_mut().enumerate() {
        v.id = ix as u8;
    }
    let map: HashMap<&str, &Valve> = valves.iter().map(|v| (&v.name[..], v)).collect();
    let mut dp: HashMap<(u64, &str, NumSet<u64>), u32> = HashMap::new();
    dp.insert((0, "AA", NumSet::new()), 0);
    let get_flow_for_minute =
        |open: NumSet<u64>| open.iter().map(|ix| valves[ix as usize].rate).sum::<u32>();
    for t in 1..=26 {
        for (loc, v) in &map {
            let id = map[loc].id;
            //what is doable at time t location v?
            //move from locations, keep the same rate.
            for a in &map[loc].connections {
                let prevs = dp
                    .iter()
                    .filter(|((old_t, ol, _), _)| *old_t == t - 1 && ol == a)
                    .map(|((_, _, open), flow)| (*open, *flow))
                    .collect_vec();
                for (open, oldflow) in prevs {
                    let x = dp.entry((t, &v.name[..], open)).or_default();
                    let flow_now = get_flow_for_minute(open);
                    let flow = flow_now + oldflow;
                    if flow > *x {
                        *x = flow;
                    }
                }
            }
            //open
            if map[loc].rate > 0 {
                let prevs = dp
                    .iter()
                    .filter(|((old_t, ol, open), _)| {
                        *old_t == t - 1 && ol == loc && !open.contains(id)
                    })
                    .map(|((_, _, open), flow)| (*open, *flow))
                    .collect_vec();
                for (open, oldflow) in prevs {
                    let mut newopen = open;
                    newopen.insert(id);
                    let x = dp.entry((t, &v.name[..], newopen)).or_default();
                    let flow_now = get_flow_for_minute(open);
                    let flow = flow_now + oldflow;
                    if flow > *x {
                        *x = flow;
                    }
                }
            }
        }
    }
    let mut p2 = 0;
    let render_numset = |ns : NumSet<u64>| {
        valves
        .iter()
        .filter(|x| ns.contains(x.id))
        .map(|x| &x.name[..])
        .collect::<Vec<_>>().join(", ")
    };
    for (a, b) in dp.iter().cartesian_product(dp.iter()) {
        if a.0 .2 & b.0 .2 == NumSet::new() && a.1 + b.1 > p2 {
            dbg!(a,b);
            println!(
                "I am at {:?} and ele at {:?}, opened are {:?} // {:?}. {}",
                a, b, 
                render_numset(a.0.2),
                render_numset(b.0.2),
                a.1 + b.1
            );
            p2 = a.1 + b.1;
        }
    }
    p2
}
