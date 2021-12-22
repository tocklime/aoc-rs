use std::{cmp::max, fmt::Display};

use itertools::Itertools;

use crate::span::Span;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cube {
    s: [Span<isize>; 3],
}
impl std::fmt::Debug for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "x={}..{},y={}..{},z={}..{} ({})",
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

impl Cube {
    pub fn new(s: [Span<isize>; 3]) -> Self {
        Self { s }
    }
    pub fn size(&self) -> isize {
        self.s.iter().map(|z| max(0, z.end - z.start)).product()
    }
    pub fn intersects(&self, other: &Self) -> bool {
        self.intersection(other).is_some()
    }
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        Some(Self {
            s: [
                self.s[0].intersection(&other.s[0])?,
                self.s[1].intersection(&other.s[1])?,
                self.s[2].intersection(&other.s[2])?,
            ],
        })
    }
    pub fn subtract(&self, other: &Self) -> Vec<Self> {
        let mut ans = Vec::new();
        if self.intersects(other) {
            for v in (0..3)
                .map(|x| self.s[x].cut_by(&other.s[x]))
                .multi_cartesian_product()
            {
                if v.iter().zip(&other.s).any(|(a, b)| a.is_disjoint(b)) {
                    ans.push(Self {
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
