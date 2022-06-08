use aoc_harness::aoc_main;

aoc_main!(2015 day 16, part1 [p1], part2 [p2]);
use itertools::Itertools;
use nom::lib::std::collections::HashMap;
use std::cmp::Ordering;
//Sue 1: goldfish: 6, trees: 9, akitas: 0

/* Spec:
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
*/

fn p1(input: &str) -> String {
    let spec: HashMap<&str, u32> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .iter()
    .cloned()
    .collect();
    for f in input.lines() {
        let split_1 = f.splitn(2, ':').collect_vec();
        let is_match = split_1[1].split(',').all(|i| {
            let nv = i.split(':').collect_vec();
            let val = nv[1].trim().parse::<u32>().unwrap();
            spec[nv[0].trim()] == val
        });
        if is_match {
            return split_1[0].to_owned();
        }
    }
    "Unknown Sue".to_owned()
}

fn p2(input: &str) -> String {
    let spec: HashMap<&str, (u32, Ordering)> = [
        ("children", (3, Ordering::Equal)),
        ("cats", (7, Ordering::Greater)),
        ("samoyeds", (2, Ordering::Equal)),
        ("pomeranians", (3, Ordering::Less)),
        ("akitas", (0, Ordering::Equal)),
        ("vizslas", (0, Ordering::Equal)),
        ("goldfish", (5, Ordering::Less)),
        ("trees", (3, Ordering::Greater)),
        ("cars", (2, Ordering::Equal)),
        ("perfumes", (1, Ordering::Equal)),
    ]
    .iter()
    .cloned()
    .collect();
    for f in input.lines() {
        let split_1 = f.splitn(2, ':').collect_vec();
        let is_match = split_1[1].split(',').all(|i| {
            let nv = i.split(':').collect_vec();
            let val = nv[1].trim().parse::<u32>().unwrap();
            match spec[nv[0].trim()] {
                (n, Ordering::Equal) => val == n,
                (n, Ordering::Greater) => val > n,
                (n, Ordering::Less) => val < n,
            }
        });
        if is_match {
            return split_1[0].to_owned();
        }
    }
    "Unknown Sue".to_owned()
}
