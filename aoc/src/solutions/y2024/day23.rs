aoc_harness::aoc_main!(2024 day 23, part1 [p1] => 1119, part2 [p2] => "av,fr,gj,hk,ii,je,jo,lq,ny,qd,uq,wq,xc", example part1 EG => 7, example part2 EG => "co,de,ka,ta");
use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use petgraph::prelude::*;

type G<'a> = Graph<&'a str, (), Undirected>;

fn p1(input: &str) -> usize {
    let mut g : G = UnGraph::new_undirected();
    let mut node_lookup : HashMap<&str, NodeIndex> = HashMap::new();

    for l in input.lines() {
        let (a,b) = l.split_once('-').unwrap();
        let a_ix = *node_lookup.entry(a).or_insert_with(|| g.add_node(a));
        let b_ix = *node_lookup.entry(b).or_insert_with(|| g.add_node(b));
        g.add_edge(a_ix, b_ix, ());
    }
    g.node_indices().map(|n| {
        g.edges(n).tuple_combinations().filter(|(a,b)|{
            let a = if a.source() == n { a.target()} else {a.source() };
            let b = if b.source() == n { b.target()} else {b.source() };
            g.find_edge(a, b).is_some() && ([n,a,b].iter().any(|n| g[*n].starts_with("t")))
        }).count()
    }).sum::<usize>() / 3usize
}

fn bron_kerbosch_rec<F: FnMut(HashSet<NodeIndex>)>(g: &G, report: &mut F, r: HashSet<NodeIndex>, mut p: HashSet<NodeIndex>, mut x: HashSet<NodeIndex>) {
    if p.is_empty() && x.is_empty() {
        report(r.clone());
    }
    while let Some(&v) = p.iter().next() {
        let mut new_r = r.clone();
        new_r.insert(v);
        let new_p = g.neighbors(v).filter(|n| p.contains(n)).collect();
        let new_x = g.neighbors(v).filter(|n| x.contains(n)).collect();
        bron_kerbosch_rec(g, report, new_r, new_p, new_x);
        p.remove(&v);
        x.insert(v);
    }
}
fn bron_kerbosch<F: FnMut(HashSet<NodeIndex>)>(g: &G, report: &mut F) {
    let p = g.node_indices().collect();
    bron_kerbosch_rec(g, report, HashSet::new(), p, HashSet::new())
}

fn p2(input: &str) -> String {
    let mut g : G = UnGraph::new_undirected();
    let mut node_lookup : HashMap<&str, NodeIndex> = HashMap::new();

    for l in input.lines() {
        let (a,b) = l.split_once('-').unwrap();
        let a_ix = *node_lookup.entry(a).or_insert_with(|| g.add_node(a));
        let b_ix = *node_lookup.entry(b).or_insert_with(|| g.add_node(b));
        g.add_edge(a_ix, b_ix, ());
    }
    let mut best_clique : HashSet<NodeIndex> = HashSet::new();
    bron_kerbosch(&g, &mut |c: HashSet<NodeIndex>| if c.len() > best_clique.len() {best_clique = c; });
    best_clique.into_iter().map(|x| g[x]).sorted().join(",")
}

const EG: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";