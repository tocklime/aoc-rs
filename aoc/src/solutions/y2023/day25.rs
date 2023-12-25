use std::collections::HashMap;

use itertools::Itertools;
use petgraph::{
    prelude::*,
    visit::{depth_first_search, Control, DfsEvent},
};
use rand::Rng;

aoc_harness::aoc_main!(2023 day 25, part1 [p1] => 538560);

fn find_articulation_point(input: &Graph<&str, (), Undirected>) -> Option<EdgeIndex> {
    let mut depths = HashMap::new();
    let mut current_depth = 0;
    let mut low = HashMap::new();
    let mut is_articulation = false;
    let mut parents = HashMap::new();
    let x = depth_first_search(input, input.node_indices().take(1), |n| {
        match n {
            DfsEvent::Discover(a, b) => {
                depths.insert(a, current_depth);
                low.insert(a, current_depth);
                current_depth += 1;
            }
            DfsEvent::TreeEdge(a, b) => {
                parents.insert(a, b);
            }
            DfsEvent::BackEdge(a, b) | DfsEvent::CrossForwardEdge(a, b) => {}
            DfsEvent::Finish(a, t) => {
                //we've visited all children of a.
                let mut l = current_depth;
                let mut child_count = 0;
                for child in input.edges(a) {
                    let b = child.target();
                    if Some(&b) != parents.get(&a) {
                        l = l.min(low[&b]);
                        child_count += 1;
                    }
                    if low[&b] >= depths[&a] {
                        is_articulation = true;
                    }
                }
                low.insert(a, l);
                current_depth -= 1;
                if parents.contains_key(&a) && is_articulation
                    || !parents.contains_key(&a) && child_count > 1
                {
                    return Control::Break(a);
                }
            }
        }
        Control::Continue
    });
    // x.break_value()
    todo!()
}
fn try_find_min_cut(g: &Graph<&str, (), Undirected>) -> (usize, usize, usize) {
    let mut g = g.clone();
    let mut rng = rand::thread_rng();
    let mut weights: HashMap<&str, usize> = HashMap::new();
    while g.node_count() > 2 {
        //find random edge
        let r = EdgeIndex::new(rng.gen_range(0..g.edge_count()));
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
    // let d = petgraph::dot::Dot::new(&g);
    // println!("{d:?}");
    // dbg!(&weights);
    for (a, b) in g.node_indices().tuple_windows() {
        let a_str = g.node_weight(a).unwrap();
        let b_str = g.node_weight(b).unwrap();
        return (g.edge_count(), *weights.get(a_str).unwrap_or(&1), *weights.get(b_str).unwrap_or(&1));
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
    assert_eq!(petgraph::algo::connected_components(&g), 1);
    println!("{} edges", edges.len());
    loop {
        let (a, b, c) = try_find_min_cut(&g);
        if a == 3 {
            return b * c
        }
    }
    // {
    // // let x = petgraph::dot::Dot::with_config(&g, &[Config::EdgeNoLabel]);
    // // let file = std::fs::write("day24.dot", format!("{x:?}")).expect("dot write");
    // }
    // for chosen in edges.into_iter().combinations(2) {
    //     let mut new_g = g.clone();
    //     for x in &chosen {
    //         new_g.remove_edge(*x);
    //     }
    //     if let Some(x) = find_articulation_point(&new_g) {
    //         println!("Choose! {chosen:?} and {x:?}");
    //     }
    // }
    // todo!()
}

// const EG: &str = "jqt: rhn xhk nvd
// rsh: frs pzl lsr
// xhk: hfx
// cmg: qnr nvd lhk bvb
// rhn: xhk bvb hfx
// bvb: xhk hfx
// pzl: lsr hfx nvd
// qnr: nvd
// ntq: jqt hfx bvb xhk
// nvd: lhk
// lsr: lhk
// rzs: qnr cmg lsr rsh
// frs: qnr lhk lsr
// ";
