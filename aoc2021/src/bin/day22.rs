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
    x: Span<isize>,
    y: Span<isize>,
    z: Span<isize>,
}

impl Display for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} x={}..{},y={}..{},z={}..{} ({})",
            if self.target_state { "on" } else { "off" },
            self.x.start,
            self.x.end,
            self.y.start,
            self.y.end,
            self.z.start,
            self.z.end,
            self.size()
        ))
    }
}

impl X {
    fn size(&self) -> isize {
        [
            self.z.end - self.z.start,
            self.y.end - self.y.start,
            self.x.end - self.x.start,
        ]
        .iter()
        .product()
    }
    fn subtract(&self, other: &Self) -> Vec<Self> {
        //subtract
        let mut ans = Vec::new();
        if self.x.intersects(&other.x) && self.y.intersects(&other.y) && self.z.intersects(&other.z)
        {
            for x in self.x.cut_by(&other.x) {
                for y in self.y.cut_by(&other.y) {
                    for z in self.z.cut_by(&other.z) {
                        if x.is_entirely_within(&other.x)
                            && y.is_entirely_within(&other.y)
                            && z.is_entirely_within(&other.z)
                        {
                            //no.
                        } else {
                            ans.push(Self {
                                target_state: true,
                                x,
                                y,
                                z,
                            })
                        }
                    }
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
            x: Span::new(x1, x2 + 1),
            y: Span::new(y1, y2 + 1),
            z: Span::new(z1, z2 + 1),
        })
    }
}

fn solve<const ONLY_SMALL: bool> (input: &[X]) -> isize {
    let mut ons: Vec<X> = Vec::new();
    for i in input {
        if ONLY_SMALL && i.x.start.abs() > 50 {
            continue;
        }
        if i.target_state {
            //on. we want this, and everything in ons, that doesn't intersect this.
            let mut new_ons = vec![*i];
            for o in ons {
                new_ons.extend(o.subtract(i));
            }
            ons = new_ons;
        } else {
            //off. take this away from everything.
            ons = ons.into_iter().flat_map(|p: X| p.subtract(i)).collect();
        }
    }
    ons.into_iter().map(|x| x.size()).sum()
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sub() {
        let a: X = "on x=10..11,y=11..13,z=11..13".parse().unwrap();
        let b: X = "off x=9..12,y=9..12,z=9..12".parse().unwrap();
        let s = a.subtract(&b);
        dbg!(&a, &b, &s);
        assert!(s.len() > 0);
        for x in &s {
            println!("{}", x);
        }
        assert!(false);
    }
}
/*
on x=10..11,y=11..13,z=11..13 (4)
subtract
off x=9..12,y=9..12,z=9..12 (27)
*/
