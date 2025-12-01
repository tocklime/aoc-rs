use std::collections::HashMap;

use itertools::Itertools;
use petgraph::prelude::*;
use rand::Rng;

aoc_harness::aoc_main!(2023 day 25, part1 [p1] => 538_560);

fn karger(g: &Graph<&str, (), Undirected>) -> (usize, usize, usize) {
    let mut g = g.clone();
    let mut rng = rand::rng();
    let mut weights: HashMap<&str, usize> = HashMap::new();
    while g.node_count() > 2 {
        //find random edge
        let r = EdgeIndex::new(rng.random_range(0..g.edge_count()));
        let (a, b) = g.edge_endpoints(r).unwrap();
        let mut walker = g.neighbors(b).detach();
        while let Some((_, t)) = walker.next(&g) {
            if t != a {
                g.add_edge(a, t, ());
            }
        }
        let b_str = g.node_weight(b).unwrap();
        let b_weight = weights.remove(b_str).unwrap_or(1);
        //folding b into a;
        let a_str = g.node_weight(a).unwrap();
        *weights.entry(*a_str).or_insert(1) += b_weight;
        g.remove_node(b);
    }
    if let Some((a,b)) = g.node_indices().tuple_windows().next() {
        let a_str = g.node_weight(a).unwrap();
        let b_str = g.node_weight(b).unwrap();
        return (
            g.edge_count(),
            *weights.get(a_str).unwrap_or(&1),
            *weights.get(b_str).unwrap_or(&1),
        );
    }
    unreachable!()
}
fn p1(input: &str) -> usize {
    let mut g: Graph<&str, (), Undirected> = Graph::default();
    let mut nodes = HashMap::new();
    let mut edges: Vec<petgraph::prelude::EdgeIndex> = Vec::new();
    for l in input.lines() {
        let (from, tos) = l.split_once(": ").unwrap();
        let from_n = *nodes.entry(from).or_insert_with(|| g.add_node(from));
        for to in tos.split_ascii_whitespace() {
            let to_n = *nodes.entry(to).or_insert_with(|| g.add_node(to));
            edges.push(g.add_edge(from_n, to_n, ()));
        }
    }
    loop {
        //keep trying karger until we find a min cut of size 3.
        let (min_cut_size, b, c) = karger(&g);
        if min_cut_size == 3 {
            return b * c;
        }
    }
}
