use reformation::Reformation;
use itertools::Itertools;
use nom::lib::std::collections::HashMap;

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
    for s in input.lines().map(|x| Step::parse(x).unwrap()) {
        dist_map.entry(s.from).or_insert_with(HashMap::new).insert(s.to, s.distance);
        dist_map.entry(s.to).or_insert_with(HashMap::new).insert(s.from, s.distance);
    }
    dist_map
}

fn all_dists<'a>(dist_map: &'a DistMap<'a>) -> impl Iterator<Item=u32> + 'a {
    dist_map.keys().into_iter().permutations(dist_map.len())
        .map(move |p| {
            p.into_iter().tuple_windows().map(|(a, b)| dist_map[a][b]).sum()
        })
}

#[aoc(day9, part1)]
fn p1(input: &str) -> u32 {
    all_dists(&gen(input)).min().unwrap()
}

#[aoc(day9, part2)]
fn p2(input: &str) -> u32 {
    all_dists(&gen(input)).max().unwrap()
}

