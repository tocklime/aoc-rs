use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    str::FromStr,
};

use itertools::Itertools;

use crate::utils::inputs::try_parse_many;

#[derive(Debug)]
pub struct Restriction {
    name: String,
    ranges: Vec<(u32, u32)>,
}
impl Restriction {
    pub fn in_range(&self, n: u32) -> bool {
        self.ranges.iter().any(|&(l, h)| (l..=h).contains(&n))
    }
}

impl FromStr for Restriction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.trim().split(':').collect_vec();
        let name = x[0].into();
        let mut ranges = Vec::new();
        for r in x[1].split(" or ") {
            let rs = r.split('-').map(|n| n.trim().parse()).collect::<Result<Vec<u32>, _>>()?;
            ranges.push((rs[0], rs[1]));
        }
        Ok(Self { name, ranges })
    }
}
#[derive(Debug)]
pub struct Prob {
    restrictions: Vec<Restriction>,
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
}
impl FromStr for Prob {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let secs = s.split("\n\n").collect_vec();
        let restrictions = try_parse_many(secs[0], "\n")?;
        let my_ticket = try_parse_many(secs[1].lines().nth(1).unwrap(), ",")?;
        let nearby_tickets = secs[2]
            .lines()
            .skip(1)
            .map(|l| try_parse_many(l, ","))
            .collect::<Result<Vec<Vec<u32>>, _>>()?;
        Ok(Self {
            restrictions,
            my_ticket,
            nearby_tickets,
        })
    }
}
#[aoc_generator(day16)]
pub fn gen(input: &str) -> Prob {
    input.trim().parse().expect("Bad input")
}
#[aoc(day16, part1)]
pub fn p1(input: &Prob) -> u32 {
    input
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|&&v| input.restrictions.iter().all(|r| !r.in_range(v)))
        .sum()
}
#[aoc(day16, part2)]
pub fn p2(input: &Prob) -> u64 {
    let valid_nearbys = input
        .nearby_tickets
        .iter()
        .filter(|t| t.iter().all(|&v| input.restrictions.iter().any(|r| r.in_range(v))))
        .collect_vec();
    let field_count = input.restrictions.len();
    let mut possible: HashMap<String, HashSet<usize>> = input
        .restrictions
        .iter()
        .map(|r| (r.name.clone(), (0..field_count).collect()))
        .collect();
    //Phase 1: eliminate possibilities based on field values.
    for t in &valid_nearbys {
        for (ix, &v) in t.iter().enumerate() {
            for r in &input.restrictions {
                if !r.in_range(v) {
                    possible.get_mut(&r.name).unwrap().remove(&ix);
                }
            }
        }
    }
    //Phase 2: eliminate possibilities where another field must have a given index.
    while possible.values().any(|hs| hs.len() > 1) {
        let uniqued: Vec<usize> = possible
            .values()
            .filter_map(|hs| if hs.len() == 1 { hs.iter().next() } else { None })
            .copied()
            .collect();
        for x in uniqued {
            for hs in possible.values_mut() {
                if hs.len() > 1 {
                    hs.remove(&x);
                }
            }
        }
    }
    possible
        .iter()
        .filter_map(|(k, v)| {
            if k.starts_with("departure") {
                Some(u64::from(input.my_ticket[(*v.iter().next().unwrap())]))
            } else {
                None
            }
        })
        .product()
}
