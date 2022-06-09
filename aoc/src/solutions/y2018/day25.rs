use itertools::Itertools;
use num::abs;
use std::collections::{HashMap, HashSet};

fn manhattan(a: &[i64], b: &[i64]) -> i64 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(a, b)| abs(*a - *b)).sum()
}

fn p1(input: &str) -> usize {
    let neighbours = gen(input);
    let mut root_to_members: HashMap<usize, HashSet<usize>> = (0..neighbours.len())
        .map(|x| (x, [x].iter().cloned().collect::<HashSet<usize>>()))
        .collect();
    loop {
        let found_to_join: Option<(usize, usize)> =
            root_to_members.iter().combinations(2).find_map(|v| {
                if v[0]
                    .1
                    .iter()
                    .any(|&s| neighbours[s].intersection(v[1].1).next().is_some())
                {
                    Some((*v[0].0, *v[1].0))
                } else {
                    None
                }
            });
        match found_to_join {
            None => {
                break;
            }
            Some((a, b)) => {
                let xs = root_to_members.remove(&b).unwrap();
                root_to_members.get_mut(&a).unwrap().extend(xs.iter());
                //println!("Joined ids {} and {}, now there's {} constellations",a,b,root_to_members.len());
            }
        }
    }
    root_to_members.len()
}
fn gen(input: &str) -> Vec<HashSet<usize>> {
    let coords: Vec<Vec<i64>> = input
        .lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    let mut neighbours: Vec<HashSet<usize>> = vec![HashSet::new(); coords.len()];
    for v in (0..coords.len()).combinations(2) {
        if manhattan(&coords[v[0]], &coords[v[1]]) <= 3 {
            neighbours.get_mut(v[0]).unwrap().insert(v[1]);
            neighbours.get_mut(v[1]).unwrap().insert(v[0]);
        }
    }
    neighbours
}

fn p1b(input: &str) -> usize {
    let neighbours = gen(input);
    let mut visited = HashSet::new();
    let mut count = 0;
    for c in 0..neighbours.len() {
        if visited.contains(&c) {
            continue;
        }
        count += 1;
        visited.insert(c);
        let mut fringe = neighbours[c].clone();
        while !fringe.is_empty() {
            visited.extend(fringe.iter());
            fringe = fringe
                .iter()
                .flat_map(|x| neighbours[*x].iter().filter(|x| !visited.contains(x)))
                .cloned()
                .collect();
        }
    }
    count
}
