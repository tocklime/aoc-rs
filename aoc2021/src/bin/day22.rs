use std::{fmt::Display, str::FromStr};

use aoc_harness::*;
use scan_fmt::scan_fmt;
use utils::span::Span;

aoc_main!(2021 day 22, generator lines::<X>, part1 [solve::<true>] => 582644, example part1 EG => 39, part2 [solve::<false>] => 1263804707062415);

const EG: &str = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

#[derive(Debug, Clone, Copy)]
struct X {
    target_state: bool,
    s: [Span<isize>; 3],
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
        [
            self.s[2].end - self.s[2].start,
            self.s[1].end - self.s[1].start,
            self.s[0].end - self.s[0].start,
        ]
        .iter()
        .product()
    }
    fn intersects(&self, other: &Self) -> bool {
        self.s.iter().zip(&other.s).all(|(a, b)| a.intersects(b))
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

fn solve<const ONLY_SMALL: bool>(input: &[X]) -> isize {
    let mut ons: Vec<X> = Vec::new();
    for i in input {
        if ONLY_SMALL && i.s[0].start.abs() > 50 {
            continue;
        }
        ons = ons.into_iter().flat_map(|x| x.subtract(i)).collect();
        if i.target_state {
            ons.push(*i);
        }
    }
    ons.into_iter().map(|x| x.size()).sum()
}
