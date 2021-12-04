use std::ops::{BitAnd, BitOr, Not, Shl};

use num::Num;

use crate::nums::NumBitExt;

#[derive(Debug, Clone, Copy)]
pub struct NumSet<T> {
    n: T,
}
impl<T: Num + Copy> NumSet<T>
where
    T: BitOr<Output = T>,
    T: BitAnd<Output = T>,
    T: Shl<usize, Output = T>,
    T: Not<Output = T>,
{
    pub fn new() -> Self {
        Self { n: T::zero() }
    }
    pub fn from(n: T) -> Self {
        Self { n }
    }
    pub fn insert(&mut self, n: usize) {
        self.n.set_bit(n.into(), true)
    }
    pub fn is_subset(&self, other: &NumSet<T>) -> bool {
        (self.n & other.n) == self.n
    }
    pub fn contains(&self, n: usize) -> bool {
        self.n.get_bit(n.into())
    }
}
impl FromIterator<u8> for NumSet<u128> {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let mut s = Self::new();
        for x in iter {
            s.insert(x.into());
        }
        s
    }
}
