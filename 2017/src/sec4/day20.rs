use regex::Regex;
use num::abs;
use itertools::Itertools;
use crate::utils::collections::ToLookup;
use std::iter::Iterator;

struct Particle {
    id: usize,
    pos: [i64;3],
    vel: [i64;3],
    acc: [i64;3],
}
impl Particle {
    fn new(id: usize, input: &str) -> Self{
        let re = Regex::new(r"^p=<([^>]*)>, v=<([^>]*)>, a=<([^>]*)>$").expect("regex comp");
        let m = re.captures_iter(input).nth(0).expect("re match");
        let p = &m[1].split(',').map(|x| x.parse::<i64>().unwrap()).collect_vec();
        let v = &m[2].split(',').map(|x| x.parse::<i64>().unwrap()).collect_vec();
        let a = &m[3].split(',').map(|x| x.parse::<i64>().unwrap()).collect_vec();
        Self {
            id,
            pos: [p[0],p[1],p[2]],
            vel: [v[0],v[1],v[2]],
            acc: [a[0],a[1],a[2]],
        }
    }
    fn manhattan(&self) -> i64 {
        self.pos.iter().map(|&x| abs(x)).sum()
    }
    fn step(&mut self) {
        for d in 0..3 {
            self.vel[d] += self.acc[d];
            self.pos[d] += self.vel[d]
        }
    }
}

#[aoc(day20,part1)]
fn p1(input: &str) -> usize {
    let mut ps = input.lines().enumerate().map(|(x,l)| Particle::new(x,l)).collect_vec();
    let mut closest = 0;
    for _ in 0..500 {
        let new_closest = ps.iter().map(|p| (p.manhattan(),p.id)).min().expect("closest").1;
        if closest != new_closest {
            closest = new_closest;
        }
        ps.iter_mut().for_each(|p|p.step());
    }
    closest
}

#[aoc(day20,part2)]
fn p2(input: &str) -> usize {
    let mut ps = input.lines().enumerate().map(|(x,l)| Particle::new(x,l)).collect_vec();
    for _ in 0..50 {
        let lu = ps.iter().map(|p| (p.pos,p.id)).collect_lookup();
        for (p,i) in lu.iter().filter(|x| x.1.len() > 1) {
            for &id in i {
                ps.remove(ps.iter().position(|x| x.id == id).unwrap());
            }
        }
        ps.iter_mut().for_each(|p|p.step());
    }
    ps.len()
}