use std::ops::{BitAnd, BitOr, Not, Shl, Shr, Sub};

use num::{Num, PrimInt};

use crate::nums::NumBitExt;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct NumSet<T> {
    n: T,
}
impl<T: Into<usize>> From<NumSet<T>> for usize {
    fn from(n: NumSet<T>) -> Self {
        n.n.into()
    }
}

impl<T> NumSet<T>
where
    T: BitOr<Output = T>
        + BitAnd<Output = T>
        + Shl<usize, Output = T>
        + Not<Output = T>
        + Num
        + Copy,
{
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
    pub fn insert(&mut self, n: usize) {
        self.n.set_bit(n, true);
    }
    pub fn is_subset(&self, other: &NumSet<T>) -> bool {
        (self.n & other.n) == self.n
    }
    pub fn contains(&self, n: usize) -> bool {
        self.n.get_bit(n)
    }
    pub fn iter(self) -> NumSetIter<T> {
        NumSetIter { n: self, pow: 0 }
    }
}
impl<T: BitOr<Output = T>> BitOr for NumSet<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        NumSet { n: self.n | rhs.n }
    }
}
impl<T> Sub for NumSet<T>
where
    T: BitAnd<Output = T> + Not<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        NumSet { n: self.n & !rhs.n }
    }
}
impl<T: PrimInt> NumSet<T> {
    pub fn len(&self) -> u32 {
        self.n.count_ones()
    }
    pub fn is_empty(&self) -> bool {
        self.n == T::zero()
    }
}
impl<T: Num + Copy> Default for NumSet<T>
where
    T: BitOr<Output = T> + BitAnd<Output = T> + Shl<usize, Output = T> + Not<Output = T>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<u8> for NumSet<T>
where
    T: BitOr<Output = T>
        + BitAnd<Output = T>
        + Shl<usize, Output = T>
        + Not<Output = T>
        + Copy
        + Num,
{
    fn from_iter<TIter: IntoIterator<Item = u8>>(iter: TIter) -> Self {
        let mut s = Self::new();
        for x in iter {
            s.insert(x.into());
        }
        s
    }
}

#[derive(Debug)]
pub struct NumSetIter<T> {
    n: NumSet<T>,
    pow: usize,
}
impl<T> Iterator for NumSetIter<T>
where
    T: PrimInt
        + std::fmt::Debug
        + BitAnd<Output = T>
        + Shr<usize, Output = T>
        + Shl<usize, Output = T>,
{
    type Item = usize;

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
        // dbg!(&self, &ans);
        Some(ans)
    }
}
