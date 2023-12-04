use itertools::Itertools;

use crate::span::Span;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Clone, Copy, Eq, Hash)]
#[must_use]
#[display("x={x},y={y},z={z}")]

pub struct Cube {
    x: Span<isize>,
    y: Span<isize>,
    z: Span<isize>,
}
impl std::fmt::Debug for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self}"))
    }
}

impl Cube {
    pub fn new(s: [Span<isize>; 3]) -> Self {
        Self {
            x: s[0],
            y: s[1],
            z: s[2],
        }
    }
    pub fn make_upper_inclusive(&mut self) {
        self.x.make_upper_inclusive();
        self.y.make_upper_inclusive();
        self.z.make_upper_inclusive();
    }
    pub fn as_array(&self) -> [Span<isize>; 3] {
        [self.x, self.y, self.z]
    }
    #[must_use]
    pub fn size(&self) -> isize {
        (self.x.end - self.x.start) * (self.y.end - self.y.start) * (self.z.end - self.z.start)
    }
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool {
        self.intersection(other).is_some()
    }
    #[must_use]
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        Some(Self {
            x: self.x.intersection(&other.x)?,
            y: self.y.intersection(&other.y)?,
            z: self.z.intersection(&other.z)?,
        })
    }
    #[must_use]
    /// subtract other from self - will return a set of cubes that cover the remainder.
    pub fn subtract(&self, other: &Self) -> Vec<Self> {
        let mut ans = Vec::new();
        if self.intersects(other) {
            let arr = self.as_array();
            let oth = other.as_array();
            for v in (0..3)
                .map(|x| arr[x].cut_by(&oth[x]))
                .multi_cartesian_product()
            {
                if v.iter().zip(&oth).any(|(a, b)| a.is_disjoint(b)) {
                    ans.push(Self::new([v[0], v[1], v[2]]));
                }
            }
        } else {
            ans.push(*self);
        }
        ans
    }
}
