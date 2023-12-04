use std::ops::{BitAnd, BitOr, BitOrAssign, Shl, Sub};

use num::Integer;

use crate::nums::NumBitExt;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct NumSet<const BYTES: usize> {
    n: [u8; BYTES],
}
impl<const BYTES: usize> Default for NumSet<BYTES> {
    fn default() -> Self {
        Self { n: [0; BYTES] }
    }
}

impl<const BYTES: usize> std::fmt::Debug for NumSet<BYTES> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vals: Vec<usize> = self.iter().collect();
        f.debug_struct("NumSet")
            .field("n", &self.n)
            .field("elements", &vals)
            .finish()
    }
}
macro_rules! from_impls {
    ($BYTES:expr, $ty:tt) => {
        impl From<NumSet<$BYTES>> for $ty {
            fn from(n: NumSet<$BYTES>) -> Self {
                $ty::from_le_bytes(n.n)
            }
        }
        impl From<$ty> for NumSet<$BYTES> {
            fn from(n: $ty) -> Self {
                Self { n: n.to_le_bytes() }
            }
        }
    };
}

from_impls!(1, u8);
from_impls!(2, u16);
from_impls!(4, u32);
from_impls!(8, u64);
from_impls!(8, usize);
from_impls!(16, u128);

impl<const BYTES: usize> NumSet<BYTES> {
    const MAX: usize = BYTES * 8 - 1;

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_bit(&mut self, n: usize, value: bool) {
        let (byte, bit) = n.div_rem(&8);
        assert!(byte < BYTES);
        self.n[byte].set_bit(u8::try_from(bit).unwrap(), value);
    }
    pub fn insert<T: Into<usize>>(&mut self, n: T) -> bool {
        let as_usize: usize = n.into();
        let was_in = self.contains(as_usize);
        self.set_bit(as_usize, true);
        !was_in
    }
    #[must_use]
    pub fn with(&self, n: usize) -> Self {
        let mut a = *self;
        a.insert(n);
        a
    }
    #[must_use]
    pub fn is_subset(&self, other: &Self) -> bool {
        self.n.iter().zip(other.n.iter()).all(|(&a, &b)| a & b == a)
    }
    #[must_use]
    pub fn contains<T: Into<usize>>(&self, n: T) -> bool {
        let u: usize = n.into();
        let (byte, bit) = u.div_rem(&8);
        assert!(
            byte < BYTES,
            "Looking for value {byte} in set with {BYTES} bytes"
        );
        self.n[byte].get_bit(u8::try_from(bit).unwrap())
    }
    #[must_use]
    pub fn iter(&self) -> NumSetIter<BYTES> {
        NumSetIter {
            n: *self,
            pow_fwd: 0,
            pow_back: Self::MAX,
        }
    }
    #[must_use]
    pub fn len(&self) -> u32 {
        self.n.iter().map(|n| n.count_ones()).sum()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.n.iter().all(|&x| x == 0)
    }
}

impl Shl<u8> for NumSet<1> {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self {
            n: [self.n[0] << rhs],
        }
    }
}

impl<const BYTES: usize> BitOrAssign for NumSet<BYTES> {
    fn bitor_assign(&mut self, rhs: Self) {
        let mut ix = 0;
        self.n = self.n.map(|x| {
            let ans = x | rhs.n[ix];
            ix += 1;
            ans
        });
    }
}
impl<const BYTES: usize> BitOr for NumSet<BYTES> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut ix = 0;
        let n = self.n.map(|x| {
            let ans = x | rhs.n[ix];
            ix += 1;
            ans
        });
        Self { n }
    }
}
impl<const BYTES: usize> BitAnd for NumSet<BYTES> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut ix = 0;
        let n = self.n.map(|x| {
            let ans = x & rhs.n[ix];
            ix += 1;
            ans
        });
        Self { n }
    }
}
impl<const BYTES: usize> std::ops::Not for NumSet<BYTES> {
    type Output = Self;

    fn not(self) -> Self::Output {
        let n = self.n.map(|x| !x);
        Self { n }
    }
}
impl<const BYTES: usize> Sub for NumSet<BYTES> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self & !rhs
    }
}

impl<T, const BYTES: usize> FromIterator<T> for NumSet<BYTES>
where
    T: Into<usize>,
{
    fn from_iter<TIter: IntoIterator<Item = T>>(iter: TIter) -> Self {
        let mut s = Self::new();
        for x in iter {
            s.insert(x);
        }
        s
    }
}

#[derive(Debug)]
pub struct NumSetIter<const BYTES: usize> {
    n: NumSet<BYTES>,
    pow_fwd: usize,
    pow_back: usize,
}
impl<const BYTES: usize> Iterator for NumSetIter<BYTES> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.n.contains(self.pow_fwd)
            && self.pow_fwd <= self.pow_back
            && self.pow_fwd < NumSet::<BYTES>::MAX
        {
            self.pow_fwd += 1;
        }
        self.n.contains(self.pow_fwd).then(|| {
            let ans = self.pow_fwd;
            self.pow_fwd += 1;
            ans
        })
    }
}
impl<const BYTES: usize> DoubleEndedIterator for NumSetIter<BYTES> {
    fn next_back(&mut self) -> Option<Self::Item> {
        while !self.n.contains(self.pow_back) && self.pow_back >= self.pow_fwd && self.pow_back > 0
        {
            self.pow_back -= 1;
        }
        self.n.contains(self.pow_back).then(|| {
            let ans = self.pow_back;
            self.pow_back += 1;
            ans
        })
    }
}

impl<TN, const BYTES: usize> Extend<TN> for NumSet<BYTES>
where
    TN: Into<usize>,
{
    fn extend<T: IntoIterator<Item = TN>>(&mut self, iter: T) {
        for x in iter {
            self.insert(x);
        }
    }
}
