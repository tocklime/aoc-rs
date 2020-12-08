use itertools::Itertools;
use petgraph::{
//    dot::Dot,
    graphmap::DiGraphMap,
    visit::{Dfs, Reversed, Walker},
};
type G<'a> = DiGraphMap<&'a str,usize>;

fn gen(inp: &str) -> G {
    let mut g: DiGraphMap<&str, usize> = DiGraphMap::new();
    for l in inp.lines() {
        let (container, bags): (&str, &str) = l.split(" bags contain ").next_tuple().unwrap();
        bags.trim_end_matches('.').split(", ").for_each(|i| {
            let (w_str, content) = i.splitn(2, ' ').next_tuple().unwrap();
            if let Ok(weight) = w_str.parse() {
                let content = content.trim_end_matches('s').trim_end_matches(" bag");
                g.add_edge(container, content, weight);
            }
        });
    }
    g
}

#[aoc(day7, part1)]
pub fn p1(input: &str) -> usize {
    let input = gen(input);
    Dfs::new(&input, "shiny gold").iter(Reversed(&input)).count() - 1
}

pub fn count_bags_from_graph(g: &G, name: &str) -> usize {
    g.edges(name)
        .map(|(_, t, w)| w * (1 + count_bags_from_graph(g, t)))
        .sum()
}

#[aoc(day7, part2)]
pub fn p2_graph(input: &str) -> usize {
    let input = gen(input);
    //let r = Dot::new(&input);
    //std::fs::write("d07-bags.dot", format!(r#"{:?}"#,r)).expect("Failed to write graph file");
    count_bags_from_graph(&input, "shiny gold")
}
