#![warn(clippy::all)]
use regex::Regex;
use std::collections::{HashMap, HashSet};

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

#[aoc_generator(day4)]
pub fn gen(input: &str) -> Vec<HashMap<String, String>> {
    input
        .split("\n\n")
        .map(|p| {
            p.split(|c| c == ' ' || c == '\n')
                .map(|x| {
                    let a: Vec<&str> = x.split(':').collect();
                    (a[0].to_string(), a[1].to_string())
                })
                .collect()
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn p1(input: &[HashMap<String, String>]) -> usize {
    input
        .iter()
        .filter(|&fns| REQUIRED_SET.iter().all(|&k| fns.contains_key(k)))
        .count()
}

#[aoc(day4, part2)]
pub fn p2(input: &[HashMap<String, String>]) -> usize {
    input
        .iter()
        .filter(|&fns| {
            REQUIRED_SET
                .iter()
                .all(|&k| fns.get(k).and_then(|x| validate(k,x)) == Some(true))
        })
        .count()
}
