// Step P must be finished before step Z can begin.
use std::collections::{HashSet, BTreeMap, HashMap};
#[derive(Debug)]
struct Step {
    pre : char,
    post: char
}
#[aoc_generator(day7)]
fn gen(input: &str) -> Vec<Step> {
    input.trim().lines().map(|l| 
        Step {
            pre: l.chars().nth(5).unwrap(),
            post: l.chars().nth(36).unwrap()
        }
    ).collect()
}

#[aoc(day7, part1)]
fn p1(input: &[Step]) -> String {
    let mut map = BTreeMap::new();
    for i in input {
        map.entry(i.pre).or_insert_with(HashSet::new);
        map.entry(i.post).or_insert_with(HashSet::new).insert(i.pre);
    }
    let mut ans = String::new();
    let mut done: HashSet<char> = HashSet::new();
    while !map.is_empty() {
        let &next = map.iter().find(|x| 
            (x.1 - &done).is_empty()
        ).expect("something to do").0;
        map.remove(&next);
        done.insert(next);
        ans.push(next);
    }
    ans
}

fn duration(c : char) -> u32 {
    61 + (c as u32) - ('A' as u32)
}
#[aoc(day7, part2)]
fn p2(input: &[Step]) -> u32 {
    let mut map = BTreeMap::new();
    for i in input {
        map.entry(i.pre).or_insert_with(HashSet::new);
        map.entry(i.post).or_insert_with(HashSet::new).insert(i.pre);
    }
    let mut in_progress: HashMap<char,u32> = HashMap::new();
    let mut done : HashMap<char,u32> = HashMap::new();
    let workers = 5;
    while done.len() < map.len() {
        let time = *in_progress.values().min().unwrap_or(&0);
        let mut new_in_progress = HashMap::new();
        for (&c,&t) in &in_progress {
            if t == time {
                done.insert(c,t);
            } else {
                new_in_progress.insert(c,t);
            }
        }
        let available = workers - new_in_progress.len();
        map.iter()
        .filter_map(|(&c,pre)|
            if pre.iter().all(|x| done.contains_key(x))
                && !done.contains_key(&c) 
                && !in_progress.contains_key(&c) { 
                Some((c,time + duration(c)))
            } else {None})
        .take(available)
        .for_each(|x| {
            new_in_progress.insert(x.0, x.1);
        });
        in_progress = new_in_progress;
    }
    *done.values().max().unwrap()
}