use std::ops::{BitAnd, BitOr, BitOrAssign, Shl, Sub};

use num::PrimInt;

use crate::nums::NumBitExt;
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct NumSet<T: PrimInt> {
    n: T,
}

impl<T: PrimInt + std::fmt::Debug> std::fmt::Debug for NumSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vals: Vec<u8> = self.iter().collect();
        f.debug_struct("NumSet")
            .field("n", &self.n)
            .field("elements", &vals)
            .finish()
    }
}
impl<T: PrimInt + Into<usize>> From<NumSet<T>> for usize {
    fn from(n: NumSet<T>) -> Self {
        n.n.into()
    }
}
impl<T: PrimInt + From<usize>> From<usize> for NumSet<T> {
    fn from(n: usize) -> Self {
        Self { n: n.into() }
    }
}

impl<T: PrimInt> NumSet<T> {
    #[must_use]
    pub fn inner(self) -> T {
        self.n
    }
    #[must_use]
    pub fn new() -> Self {
        Self { n: T::zero() }
    }
    pub fn from(n: T) -> Self {
        Self { n }
    }
    pub fn insert(&mut self, n: u8) -> bool {
        let was_in = self.n.get_bit(n);
        self.n.set_bit(n, true);
        !was_in
    }
    #[must_use]
    pub fn with(&self, n: u8) -> Self {
        let mut a = *self;
        a.insert(n);
        a
    }
    #[must_use]
    pub fn is_subset(&self, other: &NumSet<T>) -> bool {
        (self.n & other.n) == self.n
    }
    #[must_use]
    pub fn contains(&self, n: u8) -> bool {
        self.n.get_bit(n)
    }
    #[must_use]
    pub fn iter(self) -> NumSetIter<T> {
        NumSetIter { n: self, pow: 0 }
    }
    #[must_use]
    pub fn len(&self) -> u32 {
        self.n.count_ones()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.n == T::zero()
    }
}
impl<S, T: PrimInt + Shl<S, Output = T>> Shl<S> for NumSet<T> {
    type Output = Self;

    fn shl(self, rhs: S) -> Self::Output {
        Self { n: self.n << rhs }
    }
}
impl<T: PrimInt + BitOrAssign> BitOrAssign for NumSet<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.n |= rhs.n;
    }
}
impl<T: PrimInt> BitOr for NumSet<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        NumSet { n: self.n | rhs.n }
    }
}
impl<T: PrimInt> BitAnd for NumSet<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        NumSet { n: self.n & rhs.n }
    }
}
impl<T: PrimInt> Sub for NumSet<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        NumSet { n: self.n & !rhs.n }
    }
}
impl<T: PrimInt> Default for NumSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PrimInt> FromIterator<u8> for NumSet<T> {
    fn from_iter<TIter: IntoIterator<Item = u8>>(iter: TIter) -> Self {
        let mut s = Self::new();
        for x in iter {
            s.insert(x);
        }
        s
    }
}

#[derive(Debug)]
pub struct NumSetIter<T: PrimInt> {
    n: NumSet<T>,
    pow: u8,
}
impl<T: PrimInt> Iterator for NumSetIter<T> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.n.n & T::one() != T::one() {
            if self.n.n == T::zero() {
                return None;
            }
            self.n.n = self.n.n >> 1;
            self.pow += 1;
        }
        let ans = self.pow;
        self.n.n = self.n.n >> 1;
        self.pow += 1;
        Some(ans)
    }
}

impl Extend<u8> for NumSet<u128> {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        for x in iter {
            self.insert(x);
        }
    }
}
