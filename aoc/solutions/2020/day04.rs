use aoc_harness::aoc_main;

aoc_main!(2020 day 4, generator gen, part1 [p1] => 233, part2 [p2] => 111);
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

use utils::inputs::input_from_str_sep_by;
use lazy_static::lazy_static;

lazy_static! {
    static ref HEIGHT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    static ref COL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref PASS_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref REQUIRED_SET: HashSet<&'static str> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .cloned()
            .collect();
}
fn validate(k: &str, v: &str) -> Option<bool> {
    let a = match k {
        "byr" => (1920..=2002).contains(&v.parse::<usize>().ok()?),
        "iyr" => (2010..=2020).contains(&v.parse::<usize>().ok()?),
        "eyr" => (2020..=2030).contains(&v.parse::<usize>().ok()?),
        "hgt" => {
            let x = HEIGHT_RE.captures(v)?;
            match x.get(2)?.as_str() {
                "cm" => (150..=193).contains(&x.get(1)?.as_str().parse::<usize>().ok()?),
                "in" => (59..=76).contains(&x.get(1)?.as_str().parse::<usize>().ok()?),
                _ => false,
            }
        }
        "hcl" => COL_RE.is_match(v),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v),
        "pid" => PASS_RE.is_match(v),
        _ => true,
    };
    Some(a)
}

pub fn gen(input: &str) -> Vec<HashMap<&str, &str>> {
    input_from_str_sep_by(input, "\n\n", |p| {
        p.split_whitespace()
                .flat_map(move |p| p.split(':')).tuples()
                .collect()
    })
}

pub fn p1(input: &[HashMap<&str, &str>]) -> usize {
    input
        .iter()
        .filter(|&fns| REQUIRED_SET.iter().all(|&k| fns.contains_key(k)))
        .count()
}

pub fn p2(input: &[HashMap<&str, &str>]) -> usize {
    input
        .iter()
        .filter(|&fns| {
            REQUIRED_SET
                .iter()
                .all(|&k| fns.get(k).and_then(|x| validate(k,x)) == Some(true))
        })
        .count()
}
