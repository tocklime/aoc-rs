use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
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

aoc_main!(2022 day 19, part2 [p2] => 3510, part1 [p1] => 1199, example part1 EG => 33);

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
use Resource::*;

#[derive(Debug)]
struct Blueprint {
    id: u32,
    costs: HashMap<Resource, HashMap<Resource, u32>>,
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
    Ok((
        input,
        Blueprint {
            id,
            costs: costs.into_iter().collect(),
        },
    ))
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
struct State {
    robots: BTreeMap<Resource, u32>,
    resources: BTreeMap<Resource, u32>,
    must_not_build: BTreeSet<Resource>,
    time_left: u32,
}

impl State {
    fn start_state(t: u32) -> Self {
        Self {
            robots: [(Ore, 1)].into_iter().collect(),
            resources: Default::default(),
            must_not_build: Default::default(),
            time_left: t,
        }
    }
}

impl Blueprint {
    fn try_build(&self, state: &State, r: Resource) -> Option<BTreeMap<Resource, u32>> {
        let mut ans = state.resources.clone();
        for (r, c) in &self.costs[&r] {
            let r = ans.get_mut(r)?;
            *r = r.checked_sub(*c)?;
        }
        Some(ans)
    }
    fn do_time(&self, state: &State, new_robot: Option<Resource>) -> Option<State> {
        let mut new_state = state.clone();
        new_state.time_left -= 1;
        if new_robot != Some(Geode) && self.try_build(state, Geode).is_some() {
            return None;
        }
        if let Some(r) = new_robot {
            if state.must_not_build.contains(&r) {
                return None;
            }
            if r != Geode {
                let max_cost = self
                    .costs
                    .values()
                    .map(|x| *x.get(&r).unwrap_or(&0))
                    .max()
                    .unwrap();
                if state.robots.get(&r).unwrap_or(&0) >= &max_cost {
                    return None;
                }
            }
            new_state.resources = self.try_build(state, r)?;
            *new_state.robots.entry(r).or_default() += 1;
            new_state.must_not_build.clear();
        } else {
            //I chose not to build. I must not build things I can currently afford next time!
            let chose_not_to_build = [Ore, Clay, Obsidian, Geode]
                .into_iter()
                .filter(|r| self.try_build(state, *r).is_some());
            new_state.must_not_build.extend(chose_not_to_build);
        }

        for (r, count) in &state.robots {
            *new_state.resources.entry(*r).or_default() += *count;
        }
        Some(new_state)
    }

    fn most_geodes_in(&self, time_left: u32) -> u32 {
        let mut cache = HashMap::new();
        self.try_most_geodes_in(State::start_state(time_left), &mut cache)
    }
    fn try_most_geodes_in(&self, state: State, cache: &mut HashMap<State, u32>) -> u32 {
        if let Some(x) = cache.get(&state) {
            return *x;
        }
        if state.time_left == 0 {
            let ans = *state.resources.get(&Geode).unwrap_or(&0);
            cache.insert(state, ans);
            return ans;
        }
        let mut answers = Vec::new();
        //choose! what can I build?
        answers.extend(
            [None, Some(Geode), Some(Obsidian), Some(Clay), Some(Ore)]
                .iter()
                .filter_map(|r| {
                    let new_s = self.do_time(&state, *r)?;
                    // println!("Building {:?} goes {:?} to {:?}", r, &state, &new_s);
                    let best = self.try_most_geodes_in(new_s, cache);
                    Some((r, best))
                }),
        );
        //what was the best option?
        let (_res, geodes) = answers.into_iter().max_by_key(|(_, b)| *b).unwrap();
        cache.insert(state.clone(), geodes);
        geodes
    }
}

fn p1(input: &str) -> u32 {
    let (_, blueprints) = separated_list1(newline, parse_blueprint)(input).unwrap();
    blueprints.iter().map(|b| b.most_geodes_in(24) * b.id).sum()
}

fn p2(input: &str) -> u32 {
    let (_, mut blueprints) = separated_list1(newline, parse_blueprint)(input).unwrap();
    blueprints.truncate(3);
    blueprints
        .iter()
        .map(|b| {
            let a = b.most_geodes_in(32);
            dbg!(b.id, a, a * b.id);
            a
        })
        .product()
}
