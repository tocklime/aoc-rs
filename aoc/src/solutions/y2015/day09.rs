use aoc_harness::aoc_main;

aoc_main!(2015 day 9, generator gen, part1 [p1], part2 [p2]);

use reformation::Reformation;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Reformation, Debug)]
#[reformation(r"{from} to {to} = {distance}")]
struct Step<'a> {
    from: &'a str,
    to: &'a str,
    distance: u32,
}

type DistMap<'a> = HashMap<&'a str, HashMap<&'a str, u32>>;

fn gen(input: &str) -> DistMap {
    let mut dist_map: DistMap = HashMap::new();
    for s in input.trim().lines().map(|x| Step::parse(x).unwrap()) {
        dist_map.entry(s.from).or_insert_with(HashMap::new).insert(s.to, s.distance);
        dist_map.entry(s.to).or_insert_with(HashMap::new).insert(s.from, s.distance);
    }
    dist_map
}

fn all_dists<'a>(dist_map: &'a DistMap<'a>) -> impl Iterator<Item=u32> + 'a {
    dist_map.keys().permutations(dist_map.len())
        .map(move |p| {
            p.into_iter().tuple_windows().map(|(a, b)| dist_map[a][b]).sum()
        })
}


fn p1(input: &DistMap) -> u32 {
    all_dists(input).min().unwrap()
}


fn p2(input: &DistMap) -> u32 {
    all_dists(input).max().unwrap()
}

