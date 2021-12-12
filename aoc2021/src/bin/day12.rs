use std::collections::{HashMap, HashSet};

use aoc_harness::*;

aoc_main!(2021 day 12, part1 [p1], part2 [p2], example part1 EG => 10, example part2 EG => 36, example part2 EG2 => 103);

const EG: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
const EG2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

fn explore<'a>(
    connections: &'a HashMap<String, Vec<String>>,
    remaining_small_visits: usize,
    paths: &'_ mut HashSet<Vec<&'a String>>,
    path: &'_ mut Vec<&'a String>,
) {
    let pos = path.last().unwrap();
    if let Some(next) = connections.get(*pos) {
        let valid_targets = next
            .iter()
            .filter(|x| {
                x.chars().next().unwrap().is_ascii_uppercase()
                    || !path.contains(x)
                    || remaining_small_visits > 0
            })
            .collect_vec();
        // println!("Gone {:?}, valid targets: {:?}", &path, &valid_targets);
        for n in valid_targets {
            let mut new_path = path.clone();
            new_path.push(n);
            if n == "end" {
                // println!("Gone {:?}, found new path {:?}", &path, &new_path);
                paths.insert(new_path);
            } else {
                // println!("Gone {:?}, considering {}", &path, n);
                let is_small = n.chars().next().unwrap().is_ascii_lowercase();
                if is_small && path.contains(&n) {
                    explore(
                        connections,
                        remaining_small_visits - 1,
                        paths,
                        &mut new_path,
                    );
                } else {
                    explore(connections, remaining_small_visits, paths, &mut new_path);
                }
            }
        }
    } else {
        //dead end.
    }
}
fn p1(input: &str) -> usize {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for l in input.lines() {
        let mut i = l.split('-');
        let a = i.next().unwrap().to_string();
        let b = i.next().unwrap().to_string();
        connections.entry(a.clone()).or_default().push(b.clone());
        connections.entry(b).or_default().push(a);
    }
    let mut paths = HashSet::new();
    let start = connections
        .keys()
        .find(|&x| <String as AsRef<str>>::as_ref(x) == "start")
        .unwrap();
    let mut path = vec![start];
    explore(&connections, 0, &mut paths, &mut path);
    paths.len()
}

fn p2(input: &str) -> usize {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    for l in input.lines() {
        let mut i = l.split('-');
        let a = i.next().unwrap().to_string();
        let b = i.next().unwrap().to_string();
        if b != "start" {
            connections.entry(a.clone()).or_default().push(b.clone());
        }
        if a != "start" {
            connections.entry(b).or_default().push(a);
        }
    }
    let mut paths = HashSet::new();
    let start = connections
        .keys()
        .find(|&x| <String as AsRef<str>>::as_ref(x) == "start")
        .unwrap();
    let mut path = vec![start];
    explore(&connections, 1, &mut paths, &mut path);
    paths.len()
}
