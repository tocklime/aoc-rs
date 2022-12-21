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

aoc_main!(2022 day 19, generator gen, part1 [p1] => 1199, part2 [p2] => 3510, example part1 EG => 33);

const EG: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;
const RESOURCE_COUNT: usize = 4;
type Resource = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ResourceCount([u32; RESOURCE_COUNT]);

impl ResourceCount {
    const fn new() -> Self {
        ResourceCount([0; RESOURCE_COUNT])
    }
}
impl std::ops::Deref for ResourceCount {
    type Target = [u32; RESOURCE_COUNT];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for ResourceCount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[derive(Debug)]
struct Blueprint {
    id: u32,
    costs: [ResourceCount; RESOURCE_COUNT],
    maximum_demand: ResourceCount,
}
fn parse_resource(input: &str) -> IResult<&str, Resource> {
    alt((
        value(ORE, tag("ore")),
        value(CLAY, tag("clay")),
        value(OBSIDIAN, tag("obsidian")),
        value(GEODE, tag("geode")),
    ))(input)
}
fn parse_cost(input: &str) -> IResult<&str, (Resource, ResourceCount)> {
    //Each XXX robot costs ....
    let (input, output) = delimited(tag("Each "), parse_resource, tag(" robot costs "))(input)?;
    //2 ore and 3 clay
    let (input, cost_vec) = separated_list1(
        tag(" and "),
        separated_pair(complete::u32, tag(" "), parse_resource),
    )(input)?;
    let mut costs = ResourceCount::new();
    for (count, res) in cost_vec {
        costs[res] = count;
    }
    Ok((input, (output, costs)))
}
fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = delimited(tag("Blueprint "), complete::u32, tag(": "))(input)?;
    let (input, costs_vec) = separated_list1(tag(". "), parse_cost)(input)?;
    let (input, _) = tag(".")(input)?;
    let mut maximum_demand = ResourceCount::new();
    for res in 0..GEODE {
        maximum_demand[res] = costs_vec.iter().map(|x| x.1[res]).max().unwrap();
    }
    maximum_demand[GEODE] = u32::MAX;
    let mut costs = [ResourceCount::new(); RESOURCE_COUNT];
    for (res, cost) in costs_vec {
        costs[res] = cost;
    }
    Ok((
        input,
        Blueprint {
            id,
            costs,
            maximum_demand,
        },
    ))
}
#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
struct State {
    robots: ResourceCount,
    resources: ResourceCount,
    time_left: u32,
}

impl State {
    fn start_state(t: u32) -> Self {
        Self {
            robots: ResourceCount([1, 0, 0, 0]),
            resources: ResourceCount([0, 0, 0, 0]),
            time_left: t,
        }
    }
    fn geode_heuristic(&self) -> u32 {
        //if we built a geode robot for every remaining turn, how many geodes would we get?
        let geodes_now = self.resources[GEODE];
        if self.time_left == 0 {
            return geodes_now;
        }
        let geode_robots = self.robots[GEODE];
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
        for r in 0..RESOURCE_COUNT {
            // for (&r, &count) in &state.robots {
            new_state.resources[r] += time * state.robots[r];
        }
        new_state.time_left -= time;
        new_state
    }
    fn try_build(&self, state: &State, robot: Resource) -> Option<State> {
        let mut time_to_enough_resource = 0;
        //no point in building more robots than we can possibly use in one turn!
        let max_need = self.maximum_demand[robot];
        if state.robots[robot] >= max_need {
            return None;
        }
        for c in 0..RESOURCE_COUNT {
            // for (c, amount) in &self.costs[robot] {
            let amount = self.costs[robot][c];
            let have_now = state.resources[c];
            if amount <= have_now {
                continue;
            }
            let need = amount - have_now;
            let rate = state.robots[c];
            //in X turns I have X * rate more.
            //what is smallest int s.t. X*rate >= need.
            if rate == 0 {
                //we can't build this yet.
                return None;
            }
            let (d, m) = (need / rate, need % rate);
            time_to_enough_resource = time_to_enough_resource.max(d + u32::from(m > 0));
        }
        //need to wait time_to_done steps, then 1 more step while I build the new robot.
        let wait_time = time_to_enough_resource + 1;
        if wait_time > state.time_left {
            return None;
        }
        let mut new_state = self.wait(state, wait_time);
        //produce 1 robot!
        for r in 0..RESOURCE_COUNT {
            let count = self.costs[robot][r];
            // dbg!(state, robot, &new_state, r, count, wait_time, self);
            new_state.resources[r] -= count;
        }
        new_state.robots[robot] += 1;
        Some(new_state)
    }

    fn most_geodes_in(&self, time_left: u32) -> u32 {
        self.try_most_geodes_in(State::start_state(time_left))
    }
    fn try_most_geodes_in(&self, state: State) -> u32 {
        let mut best_known = 0;
        let mut stack = vec![state];
        while let Some(state) = stack.pop() {
            if state.geode_heuristic() < best_known {
                continue;
            }
            let next_states = [GEODE, OBSIDIAN, CLAY, ORE]
                .iter()
                .flat_map(|r| self.try_build(&state, *r))
                .collect::<Vec<_>>();
            if next_states.is_empty() {
                //nothing worth building, ever again. just wait
                let final_state = self.wait(&state, state.time_left);
                let ans = final_state.resources[GEODE];
                if ans > best_known {
                    best_known = ans;
                }
            } else {
                stack.extend(next_states);
            }
        }
        best_known
    }
}

fn gen(input: &str) -> Vec<Blueprint> {
    separated_list1(newline, parse_blueprint)(input).unwrap().1
}
fn p1(blueprints: &[Blueprint]) -> u32 {
    blueprints.iter().map(|b| b.most_geodes_in(24) * b.id).sum()
}

fn p2(blueprints: &[Blueprint]) -> u32 {
    blueprints[0..3]
        .iter()
        .map(|b| b.most_geodes_in(32))
        .product()
}
