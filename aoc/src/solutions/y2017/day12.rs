use aoc_harness::aoc_main;

aoc_main!(2017 day 12, generator gen, part1 [p1], part2 [p2]);
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Prob = HashMap<usize, HashSet<usize>>;

fn gen(input: &str) -> Prob {
    let mut hm: Prob = HashMap::new();
    for l in input.lines() {
        let s: Vec<&str> = l.split(" <-> ").collect_vec();
        let ns = s[1].split(", ").map(|x| x.parse::<usize>().unwrap());
        let me = s[0].parse::<usize>().unwrap();
        hm.insert(me, ns.collect::<HashSet<usize>>());
    }
    hm
}

fn p1(input: &Prob) -> usize {
    find_group(input, 0).len()
}

fn p2(input: &Prob) -> usize {
    let mut handled: HashSet<usize> = HashSet::new();
    let mut set_count = 0;
    loop {
        match input.keys().find(|x| !handled.contains(x)) {
            None => break,
            Some(&x) => {
                let s = find_group(input, x);
                set_count += 1;
                handled.extend(&s);
            }
        }
    }
    set_count
}

fn find_group(p: &Prob, member: usize) -> HashSet<usize> {
    let mut done: HashSet<usize> = HashSet::new();
    let mut group: HashSet<usize> = vec![member].into_iter().collect();
    while !group.is_empty() {
        let next_fringe: HashSet<usize> = group.iter().flat_map(|i| &p[i]).copied().collect();
        done.extend(&group);
        group = next_fringe.difference(&done).copied().collect();
    }
    done
}
