#![warn(clippy::all)]
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::utils::cartesian::Point;
use crate::utils::aabb::Aabb;
use parse_display::{Display, FromStr};

#[derive(FromStr, Display, Debug)]
#[display(r"#{id} @ {pos.x},{pos.y}: {size.x}x{size.y}")]
//input like: #1 @ 7,589: 24x11
pub struct Claim {
    id : usize,
    #[from_str(default)]
    pos: Point<usize>,
    #[from_str(default)]
    size: Point<usize>,
}
impl Claim {
    pub fn bb(&self) -> Aabb<usize> {
        Aabb::new(self.pos).extend(self.pos + self.size - Point::new(1,1))
    }
}
#[aoc_generator(day3)]
pub fn gen(input: &str) -> Vec<Claim> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day3,part1)]
pub fn p1(input: &[Claim]) -> usize {
    let mut map : HashMap<Point<usize>,usize> = HashMap::new();
    for c in input {
        let bb = Aabb::new(c.pos).extend(c.pos + c.size - Point::new(1,1));
        for p in bb.all_points() {
            *map.entry(p).or_default() += 1;
        }
    }
    map.values().filter(|&&x| x > 1).count()
}

#[aoc(day3,part2,fullmap)]
pub fn p2(input: &[Claim]) -> usize {
    //find id of claim which doesn't overlap.
    let mut map : HashMap<Point<usize>,HashSet<usize>> = HashMap::new();
    let mut uncontested_claims: HashSet<usize> = HashSet::new();
    for c in input {
        uncontested_claims.insert(c.id);
        for p in c.bb().all_points() {
            let here = map.entry(p).or_default();
            here.insert(c.id);
            if here.len() > 1 {
                for c in here.iter() {
                    uncontested_claims.remove(c);
                }
            }
        }
    }
    *uncontested_claims.iter().next().unwrap()
}

#[aoc(day3,part2,claimmap)]
pub fn p2_claimmap(input: &[Claim]) -> usize {
    let mut map = HashMap::new();
    let mut uncontested_claims: HashSet<usize> = HashSet::new();
    for c in input {
        uncontested_claims.insert(c.id);
        for p in c.bb().all_points() {
            match map.get(&p) {
                Some(o_id) => {
                    uncontested_claims.remove(&o_id);
                    uncontested_claims.remove(&c.id);
                }
                None => {
                    map.insert(p, c.id);
                }
            }
        }
    }
    *uncontested_claims.iter().next().unwrap()
}

#[aoc(day3,part2,pairwise)]
pub fn p2_pairwise(input: &[Claim]) -> usize {
    let mut candidates : HashSet<usize> = input.iter().map(|x| x.id).collect();
    for p in input.iter().combinations(2) {
        if p[0].bb().intersect(p[1].bb()).is_valid() {
            candidates.remove(&p[0].id);
            candidates.remove(&p[1].id);
        }
    }
    *candidates.iter().next().unwrap()
}