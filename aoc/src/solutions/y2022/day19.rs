use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use aoc_harness::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::value,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

aoc_main!(2022 day 19, part1 [p1] => 1199, part2 [p2] => 3510, example part1 EG => 33);

const EG: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Copy, Clone)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
use num::Integer;
use Resource::*;

#[derive(Debug)]
struct Blueprint {
    id: u32,
    costs: HashMap<Resource, HashMap<Resource, u32>>,
    maximum_demand: BTreeMap<Resource, u32>,
}
fn parse_resource(input: &str) -> IResult<&str, Resource> {
    alt((
        value(Ore, tag("ore")),
        value(Clay, tag("clay")),
        value(Obsidian, tag("obsidian")),
        value(Geode, tag("geode")),
    ))(input)
}
fn parse_cost(input: &str) -> IResult<&str, (Resource, HashMap<Resource, u32>)> {
    //Each XXX robot costs ....
    let (input, output) = delimited(tag("Each "), parse_resource, tag(" robot costs "))(input)?;
    let (input, costs) = separated_list1(
        tag(" and "),
        separated_pair(complete::u32, tag(" "), parse_resource),
    )(input)?;
    Ok((
        input,
        (output, costs.into_iter().map(|(a, b)| (b, a)).collect()),
    ))
}
fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = delimited(tag("Blueprint "), complete::u32, tag(": "))(input)?;
    let (input, costs) = separated_list1(tag(". "), parse_cost)(input)?;
    let (input, _) = tag(".")(input)?;
    let mut maximum_demand = BTreeMap::new();
    maximum_demand.insert(Geode, u32::MAX);
    for (_, cs) in &costs {
        for (&c, &val) in cs {
            let x: &mut u32 = maximum_demand.entry(c).or_default();
            *x = (*x).max(val);
        }
    }
    Ok((
        input,
        Blueprint {
            id,
            costs: costs.into_iter().collect(),
            maximum_demand,
        },
    ))
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
struct State {
    robots: BTreeMap<Resource, u32>,
    resources: BTreeMap<Resource, u32>,
    time_left: u32,
}

impl State {
    fn start_state(t: u32) -> Self {
        Self {
            robots: [(Ore, 1)].into_iter().collect(),
            resources: Default::default(),
            time_left: t,
        }
    }
    fn geode_heuristic(&self) -> u32 {
        //if we built a geode robot for every remaining turn, how many geodes would we get?
        let geodes_now = self.resources.get(&Geode).copied().unwrap_or_default();
        if self.time_left == 0 {
            return geodes_now;
        }
        let geode_robots = self.robots.get(&Geode).copied().unwrap_or_default();
        let existing_robot_production = geode_robots * self.time_left;
        let time_left = self.time_left;
        let new_robot_production = (time_left * (time_left - 1)) / 2;
        geodes_now + existing_robot_production + new_robot_production
    }
}

impl Blueprint {
    fn wait(&self, state: &State, time: u32) -> State {
        let mut new_state = state.clone();
        //harvest resources!
        for (&r, &count) in &state.robots {
            *new_state.resources.entry(r).or_default() += time * count;
        }
        new_state.time_left -= time;
        new_state
    }
    fn try_build(&self, state: &State, robot: Resource) -> Option<State> {
        let mut time_to_enough_resource = 0;
        //no point in building more robots than we can possibly use in one turn!
        let max_need = self.maximum_demand[&robot];
        if state.robots.get(&robot).copied().unwrap_or_default() >= max_need {
            return None;
        }
        for (c, amount) in &self.costs[&robot] {
            let have_now = state.resources.get(c).copied().unwrap_or_default();
            if *amount < have_now {
                continue;
            }
            let need = amount - have_now;
            let rate = *state.robots.get(c)?;
            //in X turns I have X * rate more.
            //what is smallest int s.t. X*rate >= need.
            if rate == 0 {
                //we can't build this yet.
                return None;
            }
            let (d, m) = need.div_mod_floor(&rate);
            time_to_enough_resource = time_to_enough_resource.max(d + u32::from(m > 0));
        }
        //need to wait time_to_done steps, then 1 more step while I build the new robot.
        let wait_time = time_to_enough_resource + 1;
        if wait_time > state.time_left {
            return None;
        }
        let mut new_state = self.wait(state, wait_time);
        //produce 1 robot!
        for (&r, &count) in &self.costs[&robot] {
            // dbg!(state, robot, &new_state, r, count, wait_time, self);
            *new_state.resources.entry(r).or_default() -= count;
        }
        *new_state.robots.entry(robot).or_default() += 1;
        Some(new_state)
    }

    fn most_geodes_in(&self, time_left: u32) -> u32 {
        let mut cache = HashMap::new();
        let mut best_known = 0;
        self.try_most_geodes_in(State::start_state(time_left), &mut best_known, &mut cache)
    }
    fn try_most_geodes_in(
        &self,
        state: State,
        best_known: &mut u32,
        cache: &mut HashMap<State, u32>,
    ) -> u32 {
        if let Some(x) = cache.get(&state) {
            return *x;
        }
        let ans = if state.geode_heuristic() < *best_known {
            0
        } else {
            let results = [Geode, Obsidian, Clay, Ore]
                .iter()
                .flat_map(|r| self.try_build(&state, *r))
                //     .collect::<Vec<_>>();
                // let results = results
                //     .into_iter()
                .map(|s| self.try_most_geodes_in(s, best_known, cache))
                .max();

            match results {
                Some(x) => x,
                None => {
                    //nothing worth building, ever again. just wait
                    let final_state = self.wait(&state, state.time_left);
                    let geodes = final_state
                        .resources
                        .get(&Geode)
                        .copied()
                        .unwrap_or_default();
                    geodes
                }
            }
        };
        cache.insert(state, ans);
        if ans > *best_known {
            *best_known = ans;
        }
        ans
    }
}

fn p1(input: &str) -> u32 {
    let (_, blueprints) = separated_list1(newline, parse_blueprint)(input).unwrap();
    blueprints.iter().map(|b| b.most_geodes_in(24) * b.id).sum()
}

fn p2(input: &str) -> u32 {
    let (_, blueprints) = separated_list1(newline, parse_blueprint)(input).unwrap();
    blueprints
        .iter()
        .take(3)
        .map(|b| b.most_geodes_in(32))
        .product()
}
