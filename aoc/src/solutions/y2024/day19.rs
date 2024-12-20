use core::str;
use petgraph::prelude::*;

use utils::collections::VecLookup;

aoc_harness::aoc_main!(2024 day 19, part1 [p1] => 228, part2 [p2,p2_graph,p2_vec,p2_graph2] => 584_553_405_070_389, example both EG => (6,16));

fn p1(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let trie: qp_trie::Trie<_, _> = towels.split(", ").map(|x| (x.as_bytes(), ())).collect();

    patterns
        .lines()
        .map(|l| {
            let bs = l.as_bytes();
            let top = trie.subtrie(&[][..]);
            let mut stack = vec![(0, top, 0)];
            let mut found = 0;
            while let Some((ix, subtrie, word_len)) = stack.pop() {
                let key = &bs[ix - word_len..=ix];
                if subtrie.get(key).is_some() {
                    //this is a valid end point, so can start from next at top.
                    if ix < bs.len() - 1 {
                        stack.push((ix + 1, trie.subtrie(&[][..]), 0));
                    } else {
                        found += 1;
                        break;
                    }
                }
                let next_subtrie = subtrie.subtrie(key);
                if !next_subtrie.is_empty() && ix < bs.len() - 1 {
                    stack.push((ix + 1, next_subtrie, word_len + 1));
                }
            }
            found
        })
        .sum::<usize>()
}
fn p2_graph(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let mut g: Graph<bool, char, Directed, usize> = petgraph::prelude::DiGraph::default();
    let entry = g.add_node(false);
    for t in towels.split(", ") {
        let mut ni = entry;
        for c in t.chars() {
            if let Some(x) = g.edges(ni).find(|e| e.weight() == &c) {
                ni = x.target();
            } else {
                let nni = g.add_node(false);
                g.add_edge(ni, nni, c);
                ni = nni;
            }
        }
        g[ni] = true;
    }
    let mut total = 0;
    for l in patterns.lines() {
        // println!("\n\n{l}:");
        let mut options: VecLookup<usize> = VecLookup::default(); //map of node-index to count.
        options.insert(entry.index(), 1);
        for c in l.chars() {
            let mut new_options = VecLookup::default();
            for (node_index, &count) in &options {
                if let Some(n) = g.edges(node_index.into()).find(|x| x.weight() == &c) {
                    let target = n.target();
                    if g[target] {
                        *new_options.entry(entry.index()).or_default() += count;
                    }
                    *new_options.entry(target.index()).or_default() += count;
                }
            }
            options = new_options;
        }
        //now have dp of counts of ways to get to n-letters through a word at the end of the string.
        //we only care about those that are 0 characters through a word (so, on a word boundary).
        total += options.get(entry.index()).copied().unwrap_or_default();
    }
    total
}
fn p2_graph2(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let mut g: Graph<bool, String, Directed, usize> = petgraph::prelude::DiGraph::default();
    let entry = g.add_node(false);
    for t in towels.split(", ") {
        let mut ni = entry;
        for c in t.chars() {
            if let Some(x) = g.edges(ni).find(|e| e.weight() == &c.to_string()) {
                ni = x.target();
            } else {
                let nni = g.add_node(false);
                g.add_edge(ni, nni, c.to_string());
                ni = nni;
            }
        }
        g[ni] = true;
    }
    //now squash it. for g is a tree, so every node has one parent. Find nodes with only one child and merge them.
    let mut done = false;
    // let mut count = 1;
    // std::fs::write("graph0.dot", format!("{}",petgraph::dot::Dot::with_config(&g, &[]))).unwrap();
    while !done {
        done = true;
        for ix in g.node_indices() {
            if g.edges(ix).count() == 1 && !g[ix] {
                let incoming = g.edges_directed(ix, Direction::Incoming).next().unwrap();
                let parent = incoming.source();
                let label = g.remove_edge(incoming.id()).unwrap();
                let child = g.edges(ix).next().unwrap();
                let next = child.target();
                let child_label = g.remove_edge(child.id()).unwrap();
                g.add_edge(parent, next, format!("{}{}",label,child_label));
                g.remove_node(ix);
                // std::fs::write(format!("graph{count}.dot"), format!("{}",petgraph::dot::Dot::with_config(&g, &[]))).unwrap();
                // count +=1;
                done = false;
                break;
            }
        }
    }
    // std::fs::write("graph.dot", format!("{}",petgraph::dot::Dot::with_config(&g, &[]))).unwrap();

    let mut total = 0;
    for l in patterns.lines() {
        // println!("\n\n{l}:");
        let mut options: VecLookup<VecLookup<usize>> = VecLookup::default(); //map of pattern index to node index to count.
        options.entry(0).or_default().insert(entry.index(), 1);
        for ix in 0..l.len() {
            let opts = options.remove(ix);
            if let Some(opts) = opts {
                for (ni, &count) in &opts {
                    //there are `count` ways of being at `ix` and node `ni`.
                    for e in g.edges(ni.into()) {
                        if l[ix..].strip_prefix(e.weight()).is_some() {
                            let x = options.entry(ix + e.weight().len()).or_default();
                            if g[e.target()] {
                                *x.entry(entry.index()).or_default() += count;
                            }
                            *x.entry(e.target().index()).or_default() += count;
                            // println!("{l} during {ix}/{}: {options:?}",e.weight());
                        }
                    }
                }
            }
            // println!("{l} after {ix}: {options:?}");
        }
        //now have dp of counts of ways to get to n-letters through a word at the end of the string.
        //we only care about those that are 0 characters through a word (so, on a word boundary).
        let this_pattern = *options
            .entry(l.len())
            .or_default()
            .entry(entry.index())
            .or_default();
        // println!("{l}: {this_pattern}");
        total += this_pattern;
    }
    total
}
fn p2_vec(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels: Vec<&[u8]> = towels.split(", ").map(str::as_bytes).collect();
    let mut total = 0;
    for l in patterns.lines() {
        let bs = l.as_bytes();
        let mut options: VecLookup<usize> = VecLookup::default();
        options.insert(0, 1); //key is bs index, value is count of ways of getting there.
        for ix in 0..bs.len() {
            let to_here = options.get(ix).copied().unwrap_or_default(); //we are scanning through, so this must be the correct count.
            if to_here > 0 {
                towels
                    .iter()
                    .filter_map(|&x| bs[ix..].strip_prefix(x).map(|_| x))
                    .for_each(|towel| {
                        *options.entry(ix + (towel.len())).or_default() += to_here;
                    });
            }
        }
        let a = options.get(bs.len()).copied().unwrap_or_default();
        total += a;
    }
    total
}

fn p2(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let trie: qp_trie::Trie<_, _> = towels.split(", ").map(|x| (x.as_bytes(), ())).collect();
    let mut total = 0;

    for l in patterns.lines() {
        // println!("\n\n{l}:");
        let bs = l.as_bytes();
        let mut options: VecLookup<usize> = VecLookup::default(); //map of partial-word to count.
        options.insert(0, 1);
        for (ix, _c) in l.chars().enumerate() {
            let mut new_options = VecLookup::default();
            for (progress, &count) in &options {
                let word_so_far = &bs[ix - progress..=ix];
                if trie.contains_key(word_so_far) {
                    //could end here.
                    *new_options.entry(0).or_default() += count;
                }
                //worth keeping looking as a potential current word?
                if !trie.subtrie(word_so_far).is_empty() {
                    *new_options.entry(progress + 1).or_default() += count;
                }
            }
            options = new_options;
        }
        //now have dp of counts of ways to get to n-letters through a word at the end of the string.
        //we only care about those that are 0 characters through a word (so, on a word boundary).
        total += options.get(0).copied().unwrap_or_default();
    }

    total
}

const EG: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
