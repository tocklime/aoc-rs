#![allow(clippy::inline_always)]
use std::ops::{BitOr, Sub};

use crate::nums::NumBitExt;
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NumSet {
    n: usize,
}
impl From<NumSet> for usize {
    #[inline(always)]
    fn from(n: NumSet) -> Self {
        n.n
    }
}

impl NumSet {
    #[must_use]
    #[inline(always)]
    pub fn inner(self) -> usize {
        self.n
    }
    #[inline(always)]
    #[must_use]
    pub const fn new() -> Self {
        Self { n: 0 }
    }
    #[must_use]
    pub fn from(n: usize) -> Self {
        Self { n }
    }
    #[inline(always)]
    pub fn insert(&mut self, n: u8) -> bool {
        let was_in = self.n.get_bit(n);
        self.n.set_bit(n, true);
        !was_in
    }
    #[must_use]
    #[inline(always)]
    pub fn is_subset(&self, other: &NumSet) -> bool {
        (self.n & other.n) == self.n
    }
    #[must_use]
    #[inline(always)]
    pub fn contains(&self, n: u8) -> bool {
        self.n.get_bit(n)
    }
    #[must_use]
    #[inline(always)]
    pub fn iter(self) -> NumSetIter {
        NumSetIter { n: self, pow: 0 }
    }
    #[must_use]
    #[inline(always)]
    pub fn len(&self) -> u32 {
        self.n.count_ones()
    }
    #[must_use]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }
}
impl BitOr for NumSet {
    type Output = Self;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        NumSet { n: self.n | rhs.n }
    }
}
impl Sub for NumSet {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        NumSet { n: self.n & !rhs.n }
    }
}
impl Default for NumSet {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<u8> for NumSet {
    #[inline(always)]
    fn from_iter<TIter: IntoIterator<Item = u8>>(iter: TIter) -> Self {
        let mut s = Self::new();
        for x in iter {
            s.insert(x.into());
        }
        s
    }
}

#[derive(Debug)]
pub struct NumSetIter {
    n: NumSet,
    pow: u8,
}
impl Iterator for NumSetIter {
    type Item = u8;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        while self.n.n & 1 != 1 {
            if self.n.n == 0 {
                return None;
            }
            self.n.n >>= 1;
            self.pow += 1;
        }
        let ans = self.pow;
        self.n.n >>= 1;
        self.pow += 1;
        Some(ans)
    }
}
