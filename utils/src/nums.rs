use modinverse::modinverse;
use num::{Integer, Num, One, Signed, Zero};
use std::{
    convert::TryInto,
    iter::{Product, Sum},
    ops::{Add, BitAnd, BitOr, Not, Shl, Shr},
};

pub fn mod_pow<T>(mut base: T, mut exp: T, modulus: T) -> T
where
    T: Num + Copy + Shr<Output = T> + From<u8> + PartialOrd,
{
    if modulus == T::one() {
        return T::zero();
    }
    let mut result = T::one();
    base = base % modulus;
    while exp > T::zero() {
        if exp % 2.into() == T::one() {
            result = result * base % modulus;
        }
        exp = exp >> T::one();
        base = base * base % modulus
    }
    result
}

pub fn add_i<T: Num + Signed + TryInto<usize>>(u: usize, i: &T) -> usize {
    let i_as_u: usize = i.abs().try_into().ok().unwrap();
    if i.is_negative() {
        u - i_as_u
    } else {
        u + i_as_u
    }
}
pub fn add_assign_i<T: Num + Signed + TryInto<usize>>(u: &mut usize, i: &T) {
    let i_as_u: usize = i.abs().try_into().ok().unwrap();
    if i.is_negative() {
        *u -= i_as_u;
    } else {
        *u += i_as_u;
    }
}

pub fn chinese_remainder_theorem<T>(list: &[(T, T)]) -> T
where
    T: Num + Product + Sum + Integer + Copy,
{
    let m_prod: T = list.iter().map(|x| x.1).product();
    list.iter()
        .map(|&(x, m)| x * (m_prod / m) * modinverse(m_prod / m, m).unwrap())
        .sum::<T>()
        % m_prod
}

pub trait NumExt {
    fn applications_of<T, F: Fn(T) -> T>(self, initial: T, step: F) -> T;
    fn applications_of_ref<T, F: Fn(&T) -> T>(self, initial: T, step: F) -> T;
}

impl<N: Num> NumExt for N
where
    N: Add<N, Output = N> + PartialOrd + Clone + One + Zero,
{
    fn applications_of_ref<T, F: Fn(&T) -> T>(self, initial: T, step: F) -> T {
        let mut x = initial;
        let mut s = Self::zero();
        while s < self {
            x = step(&x);
            s = s + Self::one();
        }
        x
    }
    fn applications_of<T, F: Fn(T) -> T>(self, initial: T, step: F) -> T {
        let mut x = initial;
        let mut s = Self::zero();
        while s < self {
            x = step(x);
            s = s + Self::one();
        }
        x
    }
}
pub trait NumBitExt {
    fn with_set_bit(self, bit_ix: usize, bit_value: bool) -> Self;
    fn set_bit(&mut self, bit_ix: usize, bit_value: bool);
    fn get_bit(self, bit_ix: usize) -> bool;
}
impl<N: Num + Copy> NumBitExt for N
where
    N: Num,
    N: BitOr<Output = N>,
    N: BitAnd<Output = N>,
    N: Shl<usize, Output = N>,
    N: Not<Output = N>,
{
    fn with_set_bit(self, bit_ix: usize, bit_value: bool) -> Self {
        if bit_value {
            self | (N::one() << bit_ix)
        } else {
            self & !(N::one() << bit_ix)
        }
    }

    fn get_bit(self, bit_ix: usize) -> bool {
        (self & (N::one() << bit_ix)) != N::zero()
    }

    fn set_bit(&mut self, bit_ix: usize, bit_value: bool) {
        *self = self.with_set_bit(bit_ix, bit_value);
    }
}
