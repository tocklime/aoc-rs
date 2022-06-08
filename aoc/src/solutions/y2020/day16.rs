use aoc_harness::aoc_main;

aoc_main!(2020 day 16, generator gen, part1 [p1], part2 [p2]);
use itertools::Itertools;
use std::{num::ParseIntError, str::FromStr};
use utils::inputs::try_parse_many;

#[derive(Debug)]
pub struct Restriction {
    _name: String,
    ranges: Vec<(u32, u32)>,
}
impl Restriction {
    fn in_range(&self, n: u32) -> bool {
        (self.ranges[0].0 <= n && n <= self.ranges[0].1) || (self.ranges[1].0 <= n && n <= self.ranges[1].1)
    }
}

impl FromStr for Restriction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.split(':').collect_vec();
        let _name = x[0].into();
        let mut ranges = Vec::new();
        for r in x[1].split(" or ") {
            let rs = r.split('-').map(|n| n.trim().parse()).collect::<Result<Vec<u32>, _>>()?;
            ranges.push((rs[0], rs[1]));
        }
        Ok(Self { _name, ranges })
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
        let nearby_tickets = secs[2]
            .lines()
            .skip(1)
            .map(|l| try_parse_many(l, ","))
            .collect::<Result<Vec<Vec<u32>>, _>>()?;
        Ok(Self {
            restrictions: try_parse_many(secs[0], "\n")?,
            my_ticket: try_parse_many(secs[1].lines().nth(1).unwrap(), ",")?,
            nearby_tickets,
        })
    }
}
fn gen(input: &str) -> Prob {
    input.replace("\r", "").trim().parse().expect("Bad input")
}
fn p1(input: &Prob) -> u32 {
    input
        .nearby_tickets
        .iter()
        .flatten()
        .filter(|&&v| input.restrictions.iter().all(|r| !r.in_range(v)))
        .sum()
}
fn get_set_bit(n: u32) -> Option<u32> {
    (0..=32).find(|x| n & (1 << x) != 0)
}
fn p2(input: &Prob) -> u64 {
    let valid_nearbys = input
        .nearby_tickets
        .iter()
        .filter(|t| t.iter().all(|&v| input.restrictions.iter().any(|r| r.in_range(v))))
        .collect_vec();
    let field_count = input.restrictions.len();
    assert!(field_count <= 32);
    //theres <32 fields, so lets use a u32 as a bitfield of possibilities..
    //Phase 1: create possibilities based on all field values being valid.
    let mut possible: Vec<u32> = input
        .restrictions
        .iter()
        .map(|r| {
            let mut i = 0;
            for ix in 0..field_count {
                if valid_nearbys.iter().all(|x| r.in_range(x[ix])) {
                    i |= 1 << ix;
                }
            }
            i
        })
        .collect();
    //Phase 2: eliminate possibilities where another field must have a given index.
    let mut ans = vec![0; field_count];
    while let Some(&x) = possible.iter().find(|&p| p.count_ones() == 1) {
        for (ix, p) in possible.iter_mut().enumerate() {
            if *p == x {
                ans[ix] = get_set_bit(x).unwrap();
            }
            *p &= !x;
        }
    }
    ans[0..6].iter().map(|&v| u64::from(input.my_ticket[v as usize])).product()
}
