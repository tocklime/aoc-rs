use std::{num::ParseIntError, str::FromStr};

use pathfinding::prelude::dfs;

#[derive(Debug, Clone)]
pub enum Match {
    Lit(char),
    RulesSeq(Vec<usize>),
}
#[derive(Debug, Clone)]
pub struct Rule {
    id: usize,
    matches: Vec<Match>,
}

impl FromStr for Match {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('"') {
            Ok(Self::Lit(s.chars().nth(1).unwrap()))
        } else {
            Ok(Self::RulesSeq(
                s.split(' ').map(str::parse).collect::<Result<Vec<usize>, _>>()?,
            ))
        }
    }
}
impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split(": ");
        let id = sp.next().unwrap().parse()?;
        let matches = sp
            .next()
            .unwrap()
            .split(" | ")
            .map(str::parse)
            .collect::<Result<Vec<Match>, _>>()?;
        Ok(Self { id, matches })
    }
}

pub fn matches(s: &str, rules: &[Rule]) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let start = (vec![0], 0);
    dfs::<(Vec<usize>, usize), _, _, _>(
        start,
        |(pending_rules, str_pos)| {
            if let Some((r, rest)) = pending_rules.split_first() {
                let rule: &Rule = rules.get(*r).unwrap();
                rule.matches
                    .iter()
                    .filter_map(|m| -> Option<(Vec<usize>, usize)> {
                        match m {
                            Match::Lit(c) if chars.get(*str_pos) == Some(c) => Some((rest.to_vec(), str_pos + 1)),
                            Match::RulesSeq(rs) => {
                                let mut from_here = rs.clone();
                                from_here.extend(rest);
                                Some((from_here, *str_pos))
                            }
                            _ => None,
                        }
                    })
                    .collect::<Vec<_>>()
            } else {
                Vec::new()
            }
        },
        |(rs, s)| rs.is_empty() && *s == chars.len(),
    )
    .is_some()
}

#[aoc_generator(day19)]
pub fn gen(input: &str) -> (Vec<Rule>, Vec<String>) {
    let mut sp = input.split("\n\n");
    let mut rules: Vec<Rule> = sp.next().unwrap().lines().map(|l| l.parse::<Rule>().unwrap()).collect();
    rules.sort_by_key(|x| x.id);
    for (ix, r) in rules.iter().enumerate() {
        assert_eq!(ix, r.id);
    }

    let candidates = sp.next().unwrap().lines().map(ToString::to_string).collect();

    (rules, candidates)
}

#[aoc(day19, part1)]
pub fn p1((rules, input): &(Vec<Rule>, Vec<String>)) -> usize {
    input.iter().filter(|l| matches(l, rules)).count()
}

#[aoc(day19, part2)]
pub fn p2((rules, input): &(Vec<Rule>, Vec<String>)) -> usize {
    let mut rules = rules.clone();
    let patch_inp = "8: 42 | 42 8\n11: 42 31 | 42 11 31";
    for r in patch_inp.lines().map(|x| x.parse::<Rule>().unwrap()) {
        let id = r.id;
        rules[id] = r;
    }
    input.iter().filter(|l| matches(l, &rules)).count()
}
