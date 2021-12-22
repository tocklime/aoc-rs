use std::{cmp::max, collections::HashMap, fmt::Display, str::FromStr};

use aoc_harness::*;
use scan_fmt::scan_fmt;
use utils::span::Span;

aoc_main!(2021 day 22, generator lines::<X>, 
    part1 [by_block::<20>, construction::<true>, region_weights::<20>] => 582644, example part1 EG => 39, 
    part2 [by_block::<420>, construction::<false>, region_weights::<420>] => 1263804707062415);

const EG: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct X {
    target_state: bool,
    s: [Span<isize>; 3],
}
impl std::fmt::Debug for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl Display for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} x={}..{},y={}..{},z={}..{} ({})",
            if self.target_state { "on" } else { "off" },
            self.s[0].start,
            self.s[0].end,
            self.s[1].start,
            self.s[1].end,
            self.s[2].start,
            self.s[2].end,
            self.size()
        ))
    }
}

impl X {
    fn size(&self) -> isize {
        self.s.iter().map(|z| max(0, z.end - z.start)).product()
    }
    fn intersects(&self, other: &Self) -> bool {
        self.s.iter().zip(&other.s).all(|(a, b)| a.intersects(b))
    }
    fn intersection(&self, other: &Self) -> Option<Self> {
        Some(Self {
            target_state: other.target_state,
            s: [
                self.s[0].intersection(&other.s[0])?,
                self.s[1].intersection(&other.s[1])?,
                self.s[2].intersection(&other.s[2])?,
            ],
        })
    }
    fn subtract(&self, other: &Self) -> Vec<Self> {
        let mut ans = Vec::new();
        if self.intersects(other) {
            for v in (0..3)
                .map(|x| self.s[x].cut_by(&other.s[x]))
                .multi_cartesian_product()
            {
                if v.iter().zip(&other.s).any(|(a, b)| a.is_disjoint(b)) {
                    ans.push(Self {
                        target_state: true,
                        s: [v[0], v[1], v[2]],
                    });
                }
            }
        } else {
            ans.push(*self);
        }
        ans
    }
}

impl FromStr for X {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, x1, x2, y1, y2, z1, z2) = scan_fmt!(
            s,
            "{} x={}..{},y={}..{},z={}..{}",
            String,
            isize,
            isize,
            isize,
            isize,
            isize,
            isize
        )
        .unwrap();
        Ok(Self {
            target_state: s == "on",
            s: [
                Span::new(x1, x2 + 1),
                Span::new(y1, y2 + 1),
                Span::new(z1, z2 + 1),
            ],
        })
    }
}

fn volume_remaining(me: &X, future: &[X], depth: usize) -> isize {
    let mut my_vol = me.size();
    let filtered_future = future
        .iter()
        .filter_map(|x| me.intersection(x))
        .collect_vec();
    for ix in 0..filtered_future.len() {
        // println!("{} {} {}",depth, ix, filtered_future.len());
        my_vol -= volume_remaining(&filtered_future[ix], &filtered_future[ix + 1..], depth + 1);
    }
    my_vol
}

fn by_block<const N: usize>(input: &[X]) -> isize {
    let mut ans = 0;
    for (ix, i) in input.iter().enumerate().take(N) {
        if i.target_state {
            let x = volume_remaining(i, &input[ix + 1..], 1);
            ans += x;
        }
    }
    ans
}

fn region_weights<const N: usize>(input: &[X]) -> isize {
    let mut weights: HashMap<X, isize> = HashMap::new();
    for i in input.iter().take(N) {
        // println!("{} {}", i, weights.len());
        let mut new_weights = weights.clone();
        for (k, w) in weights {
            if let Some(intersection) = k.intersection(i) {
                *new_weights.entry(intersection).or_default() -= w;
            }
        }
        if i.target_state {
            *new_weights.entry(*i).or_default() += 1;
        }
        new_weights.retain(|_, v| *v != 0);
        weights = new_weights;
    }
    // dbg!(&weights);
    weights.into_iter().map(|(c, s)| c.size() * s).sum()
}

fn construction<const ONLY_SMALL: bool>(input: &[X]) -> isize {
    let mut ons: Vec<X> = Vec::new();
    for i in input {
        if ONLY_SMALL && i.s[0].start.abs() > 50 {
            continue;
        }
        let mut new_ons = Vec::with_capacity(ons.len());
        for o in ons {
            //these are sorted by x start.
            if o.intersects(i) {
                new_ons.extend(o.subtract(i));
            } else {
                new_ons.push(o);
            }
        }
        if i.target_state {
            new_ons.push(*i);
        }
        ons = new_ons;
    }
    ons.into_iter().map(|x| x.size()).sum()
}
