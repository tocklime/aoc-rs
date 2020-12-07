use crate::utils::collections::ToLookupSet;
use itertools::Itertools;
use petgraph::{
    graphmap::DiGraphMap,
    visit::{Dfs, Reversed, Walker},
};
use std::collections::{HashMap, HashSet};
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Bags {
    n: usize,
    colour: String,
}

pub fn gen(input: &str) -> Vec<(String, Vec<Bags>)> {
    input
        .lines()
        .map(|l| {
            let (n, bags): (&str, &str) = l.split(" bags contain ").next_tuple().unwrap();
            let inners = bags
                .trim_end_matches('.')
                .split(", ")
                .filter_map(|i| {
                    let (c, a) = i.splitn(2, ' ').next_tuple().unwrap();
                    if let Ok(x) = c.parse() {
                        let t = a.trim_end_matches(" bag").trim_end_matches(" bags");
                        Some(Bags {
                            n: x,
                            colour: t.to_string(),
                        })
                    } else {
                        None
                    }
                })
                .collect();
            (n.to_string(), inners)
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn p1(input: &str) -> usize {
    let v = gen(input);
    let cs = v
        .iter()
        .flat_map(|(n, bags)| bags.iter().map(move |b| (b.colour.as_ref(), n.as_ref())))
        .collect_lookup_set();
    let mut to_explore = vec!["shiny gold"];
    let mut seen = HashSet::new();
    while !to_explore.is_empty() {
        let c = to_explore.pop().unwrap();
        seen.insert(c);
        to_explore.extend(cs.get(c).unwrap_or(&HashSet::new()) - &seen);
    }
    seen.len() - 1
}

pub fn count_bags(m: &HashMap<&str, HashSet<&Bags>>, name: &str) -> usize {
    m.get(name).map_or(0, |x| {
        x.iter().map(|b| b.n * (1 + count_bags(m, &b.colour))).sum()
    })
}

#[aoc(day7, part2)]
pub fn p2(input: &str) -> usize {
    let v = gen(input);
    let cs = v
        .iter()
        .flat_map(|(n, bags)| bags.iter().map(move |b| (n.as_ref(), b)))
        .collect_lookup_set();

    count_bags(&cs, "shiny gold")
}

fn gen2(inp: &str) -> DiGraphMap<&str, usize> {
    let mut g: DiGraphMap<&str, usize> = DiGraphMap::new();
    for l in inp.lines() {
        let (container, bags): (&str, &str) = l.split(" bags contain ").next_tuple().unwrap();
        bags.trim_end_matches('.').split(", ").for_each(|i| {
            let (w_str, content) = i.splitn(2, ' ').next_tuple().unwrap();
            if let Ok(weight) = w_str.parse() {
                let content = content.trim_end_matches(" bag").trim_end_matches(" bags");
                g.add_edge(container, content, weight);
            }
        });
    }
    g
}

#[aoc(day7, part1, graph)]
pub fn p1_graph(input: &str) -> usize {
    let g = gen2(input);
    Dfs::new(&g, "shiny gold").iter(Reversed(&g)).count() - 1
}

pub fn count_bags_from_graph(g: &DiGraphMap<&str, usize>, name: &str) -> usize {
    g.edges(name)
        .map(|(_, t, w)| w * (1 + count_bags_from_graph(g, t)))
        .sum()
}

#[aoc(day7, part2, graph)]
pub fn p2_graph(input: &str) -> usize {
    let g = gen2(input);
    count_bags_from_graph(&g, "shiny gold")
}
