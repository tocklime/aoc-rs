use utils::cartesian::{as_point_map, Point};
use itertools::Itertools;
use pathfinding::prelude::astar;
use std::collections::HashMap;
use std::convert::TryInto;


fn gen_dists(input: &str) -> HashMap<char,HashMap<char,usize>> {
    let map = as_point_map(input);
    let locs = map.iter().filter_map(|(&k, &v): (&Point<i32>, &char)| {
        if v.is_numeric() {
            Some((v, k))
        } else { None }
    }).collect_vec();

    let mut ans = HashMap::new();

    for v in locs.iter().combinations(2) {
        let a = v[0].1;
        let b = v[1].1;
        let a_to_b = astar::<Point<i32>, usize, _, _, _, _>(
            &a,
            |p| {
                p.neighbours().iter().filter_map(|p|
                    {
                        match map.get(p) {
                            Some(c) if *c != '#' => Some((*p, 1)),
                            _ => None
                        }
                    }
                ).collect_vec()
            },
            |&p| (b - p).manhattan().try_into().unwrap(),
            |&p| b == p,
        ).expect(&format!("No route: {:?}",&v)).1;
        ans.entry(v[0].0).or_insert_with(HashMap::new).insert(v[1].0, a_to_b);
        ans.entry(v[1].0).or_insert_with(HashMap::new).insert(v[0].0, a_to_b);
    }
    ans
}

fn p1(input: &str) -> usize {
    let dists = gen_dists(input);

    let x = dists.keys().filter(|&&x| x != '0')
        .permutations(dists.len() - 1)
        .map(|v| {
            (v.iter().fold((0,'0'), |(dist,pos), &next_c|
                (dists[&pos][next_c] + dist,*next_c)
            ),v)
        })
        .min();
    (x.unwrap().0).0
}

fn p2(input: &str) -> usize {
    let dists = gen_dists(input);

    let x = dists.keys().filter(|&&x| x != '0')
        .permutations(dists.len() - 1)
        .map(|mut v| {
            v.push(&'0');
            (v.iter().fold((0,'0'), |(dist,pos), &next_c|
                (dists[&pos][next_c] + dist,*next_c)
            ),v)
        })
        .min();
    (x.unwrap().0).0
}
