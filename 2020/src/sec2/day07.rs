use crate::utils::collections::ToLookupSet;
use itertools::Itertools;
use parse_display::FromStr;
use std::collections::{HashMap, HashSet};

#[derive(FromStr, Debug, Hash, PartialEq, Eq, Clone)]
#[from_str(regex = r"(?P<n>[0-9]+) (?P<colour>[a-z ]*) bags?")]
pub struct Bags {
    n: usize,
    colour: String,
}

pub fn all_containers<'a>(
    m: &'a HashMap<&'a str, HashSet<&'a str>>,
    name: &'a str,
) -> HashSet<&'a str> {
    let mut to_explore = vec![name];
    let mut seen = HashSet::new();
    while !to_explore.is_empty() {
        let c = to_explore.pop().unwrap();
        seen.insert(c);
        to_explore.extend(m.get(c).unwrap_or(&HashSet::new()) - &seen);
    }
    seen
}

pub fn count_bags(m: &HashMap<&str, HashSet<&Bags>>, name: &str) -> usize {
    m.get(name).map_or(0, |x| {
        x.iter().map(|b| b.n * (1 + count_bags(m, &b.colour))).sum()
    })
}
#[aoc_generator(day7)]
pub fn gen(input: &str) -> Vec<(String, Vec<Bags>)> {
    input
        .lines()
        .map(|l| {
            let x = l.split(" bags contain ").collect::<Vec<_>>();
            let n = x[0].to_string();
            let inners = x[1]
                .trim_end_matches('.')
                .split(", ")
                .map(|i| i.parse::<Bags>().ok())
                .filter_map(|x| x)
                .collect_vec();
            (n, inners)
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn p1(input: &[(String, Vec<Bags>)]) -> usize {
    let cs = input
        .iter()
        .flat_map(|(n, bags)| bags.iter().map(move |b| (b.colour.as_ref(), n.as_ref())))
        .collect_lookup_set();
    all_containers(&cs, "shiny gold").len() - 1
}
#[aoc(day7, part2)]
pub fn p2(input: &[(String, Vec<Bags>)]) -> usize {
    let cs = input
        .iter()
        .flat_map(|(n, bags)| bags.iter().map(move |b| (n.as_ref(), b)))
        .collect_lookup_set();

    count_bags(&cs, "shiny gold")
}
