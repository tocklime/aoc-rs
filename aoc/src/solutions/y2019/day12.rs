use utils::iter::all_ix_pairs;
use aoc_harness::aoc_main;
use num::integer::lcm;
use regex::Regex;
use std::str::FromStr;

aoc_main!(2019 day 12, generator gen, part1 [p1] => 7098, part2 [p2] => 400128139852752);

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Moon {
    pos: Vec<i64>,
    vel: Vec<i64>,
}

impl FromStr for Moon {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
        if let Some(m) = re.captures(s) {
            Ok(Self {
                pos: (0..3).map(|x| m[x + 1].parse().unwrap()).collect(),
                vel: vec![0, 0, 0],
            })
        } else {
            Err("No regex match".to_string())
        }
    }
}
impl Moon {
    fn step(&mut self) {
        for (i, p) in self.pos.iter_mut().enumerate() {
            *p += self.vel[i];
        }
    }
    fn energy(&self) -> i64 {
        let pot: i64 = self.pos.iter().map(|d| d.abs()).sum();
        let kin: i64 = self.vel.iter().map(|d| d.abs()).sum();
        pot * kin
    }
}

pub fn gen(input: &str) -> Vec<Moon> {
    input.lines().map(|l| l.parse::<Moon>().unwrap()).collect()
}

fn do_gravity(moons: &mut [Moon], dimension: usize) {
    all_ix_pairs(moons.len()).for_each(|(m1ix, m2ix)| {
        let s = (moons[m1ix].pos[dimension] - moons[m2ix].pos[dimension]).signum();
        moons.get_mut(m1ix).unwrap().vel[dimension] -= s;
        moons.get_mut(m2ix).unwrap().vel[dimension] += s;
    })
}

pub fn p1(input: &[Moon]) -> i64 {
    let mut moons = input.to_vec();
    for _ in 0..1000 {
        (0..3).for_each(|d| do_gravity(&mut moons, d));
        moons.iter_mut().for_each(Moon::step);
    }
    moons.iter().map(Moon::energy).sum()
}

#[allow(clippy::maybe_infinite_iter)]
pub fn p2(input: &[Moon]) -> usize {
    let periods = (0..3).map(|d| {
        let mut moons = input.to_vec();
        (1..)
            .find(|_| {
                do_gravity(&mut moons, d);
                moons.iter_mut().for_each(Moon::step);
                moons == input
            })
            .unwrap()
    });
    periods.fold(1, lcm)
}
