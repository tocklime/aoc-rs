use aoc_harness::aoc_main;

aoc_main!(2015 day 13, part1 [p1], part2 [p2]);
//Eric would lose 65 happiness units by sitting next to Alice.
//Eric would gain 24 happiness units by sitting next to Bob.

use reformation::Reformation;
use itertools::Itertools;
use nom::lib::std::collections::HashMap;
use std::iter;

#[derive(Reformation, Debug)]
#[reformation(r"{subject} would {direction} {value} happiness units by sitting next to {object}.")]
struct Line<'a> {
    subject: &'a str,
    object: &'a str,
    value: i32,
    direction: &'a str,
}

fn gen(input: &str) -> HashMap<&str, HashMap<&str, i32>> {
    let lines = input.lines().map(|l| Line::parse(l).unwrap()).collect_vec();
    let mut info: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for l in lines {
        info.entry(l.subject).or_insert_with(HashMap::new)
            .insert(l.object, if l.direction == "gain" { l.value } else { -l.value });
    }
    info
}


fn p1(input: &str) -> i32 {
    let info = gen(input);
    info.keys().permutations(info.keys().len())
        .map(|p| {
            p.iter().chain(iter::once(&p[0]))
                .tuple_windows().map(|(a, b)| info[*a][*b] + info[*b][*a]).sum()
        }).max().unwrap()
}


fn p2(input: &str) -> i32 {
    let mut info = gen(input);
    let mut my_prefs: HashMap<&str, i32> = HashMap::new();
    for (k, v) in info.iter_mut() {
        my_prefs.insert(k, 0);
        v.insert("Me", 0);
    }
    info.insert("Me", my_prefs);
    info.keys().permutations(info.keys().len())
        .map(|p| {
            p.iter().chain(iter::once(&p[0]))
                .tuple_windows().map(|(a, b)| info[*a][*b] + info[*b][*a]).sum()
        }).max().unwrap()
}
