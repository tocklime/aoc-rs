use itertools::Itertools;
use std::collections::HashSet;
use pathfinding::directed::astar::astar;

struct Rule<'a> {
    lhs: &'a str,
    rhs: &'a str,
}

impl<'a> Rule<'a> {
    fn parse(s: &'a str) -> Self {
        let v = s.split(" => ").collect_vec();
        Self { lhs: v[0], rhs: v[1] }
    }
}

fn steps(s: &str, rules: &Vec<Rule>) -> HashSet<String> {
    let mut ans = HashSet::new();
    for pos in 0..s.len() {
        for rule in rules {
            if s[pos..].starts_with(rule.lhs) {
                let new_str: String = format!("{}{}{}", &s[0..pos], rule.rhs, &s[pos + rule.lhs.len()..]);
                ans.insert(new_str);
            }
        }
    }
    ans
}

#[aoc(day19, part1)]
fn p1(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    let state = *lines.last().unwrap();
    let rules: Vec<Rule> = lines[0..lines.len() - 2].iter().cloned().map(Rule::parse).collect_vec();
    steps(state, &rules).len()
}

fn steps_rev(s: &str, rules: &Vec<Rule>) -> HashSet<String> {
    let mut ans = HashSet::new();
    for pos in 0..s.len() {
        for rule in rules {
            if s[pos..].starts_with(rule.rhs) {
                let new_str: String = format!("{}{}{}", &s[0..pos], rule.lhs, &s[pos + rule.rhs.len()..]);
                ans.insert(new_str);
            }
        }
    }
    ans
}

#[aoc(day19, part2)]
fn p2(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    let target = *lines.last().unwrap();
    let rules: Vec<Rule> = lines[0..lines.len() - 2].iter().cloned().map(Rule::parse).collect_vec();
    let path = astar(
        &target.to_owned(),
        |s| steps_rev(s, &rules).into_iter().map(|x| (x, 1)),
        |s| s.len() - 1,
        |s| *s == "e").unwrap();
    path.1
}