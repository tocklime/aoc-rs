use std::cmp::Ordering;

use itertools::Itertools;
use utils::{collections::VecLookup, inputs::parse_input_from_str_sep_by, numset::NumSet};

aoc_harness::aoc_main!(2024 day 5, generator gen, both [checks,sort] => (5329, 5833), example both EG => (143,123));

fn check_line(line: &[u8], rules: &Rules) -> bool {
    for (ix, n) in line.iter().enumerate() {
        if let Some(after) = rules.get(usize::from(*n)) {
            //if after appear in the list, they must come after.
            //or, they must NOT come before.
            if after.iter().any(|x| line[0..ix].contains(&x)) {
                return false;
            }
        }
    }
    true
}
type Rules = VecLookup<NumSet<u128>>;
fn find_mid(line: &[u8], rules: &Rules) -> u8 {
    let mut remain = line.to_vec();
    let mut placed = 0;
    while !remain.is_empty() {
        let x = remain
            .iter()
            .enumerate()
            .find(|(_, &n)| {
                remain
                    .iter()
                    .all(|o| rules.get(usize::from(*o)).map(|x| !x.contains(n)).unwrap_or(true))
            })
            .unwrap();
        if placed == line.len()/2 {
            return *x.1;
        }
        placed += 1;
        remain.swap_remove(x.0);
    }
    unreachable!()
}

struct P {
    rules: Rules,
    updates: Vec<Vec<u8>>
}
impl P {
    fn cmp_bool(&self) -> impl (Fn(&u8,&u8) -> bool) + use<'_> {
        |&a,&b| {
            self.rules.get(usize::from(a)).map(|x| x.contains(b)).unwrap_or_default()
        }
    }
    fn cmp_ord(&self) -> impl (Fn(&&u8,&&u8) -> Ordering) + use<'_> {
        |&&a,&&b| {
            if self.rules.get(usize::from(a)).map(|x| x.contains(b)).unwrap_or_default() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

fn gen(input: &str) -> P {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let mut rules_map: Rules = Rules::default();
    for r in rules.lines() {
        let (a, b) = r.split_once('|').unwrap();
        let a= a.parse().unwrap();
        let b= b.parse().unwrap();
        rules_map.entry(a).or_default().insert(b);
    }
    let updates = updates.lines().map(|l| parse_input_from_str_sep_by(l, ",")).collect();
    P {rules: rules_map, updates}

}


fn checks(input: &P) -> (usize, usize) {
    input.updates.iter().fold((0,0), |(p1,p2), line| {
        if check_line(line, &input.rules) {
            (p1 + usize::from(line[line.len() / 2]),p2)
        } else {
            (p1,p2+usize::from(find_mid(line, &input.rules)))
        }
    })
}
fn sort(input: &P) -> (usize, usize) {
    input.updates.iter().fold((0,0), |(p1,p2), line| {
        if line.is_sorted_by(input.cmp_bool()) {
            (p1 + usize::from(line[line.len() / 2]),p2)
        } else {
            let sorted_mid = *line.iter().sorted_by(input.cmp_ord()).nth(line.len()/2).unwrap();
            (p1,p2+usize::from(sorted_mid))
        }
    })
}

const EG: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
