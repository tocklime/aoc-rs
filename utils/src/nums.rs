use modinverse::modinverse;
use num::{CheckedAdd, CheckedMul, CheckedSub, Integer, Num, One, Signed, Zero};
use std::{
    convert::TryInto,
    fmt::Debug,
    iter::{Product, Sum},
    ops::{
        Add, AddAssign, BitAnd, BitOr, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, Shr,
    },
};

#[must_use]
pub fn int_to_digits_big_endian<const SIZE: usize>(mut i: usize) -> [u8; SIZE] {
    let mut ans = [0_u8; SIZE];
    let mut pos = SIZE;
    #[allow(clippy::cast_possible_truncation)] //we mod 10 it, it's going to fit in u8.
    while i > 0 && pos > 0 {
        pos -= 1;
        ans[pos] = (i % 10) as u8;
        i /= 10;
    }
    ans
}

pub fn digits<T>(mut n: T) -> impl Iterator<Item = T>
where
    T: Num + RemAssign<T> + DivAssign<T> + From<u8> + MulAssign<T> + PartialOrd + Copy,
{
    let ten: T = 10.into();
    let mut div = T::one();
    while n >= div * ten {
        div *= ten;
    }
    std::iter::from_fn(move || {
        if div == T::zero() {
            None
        } else {
            let v = n / div;
            n %= div;
            div /= ten;
            Some(v)
        }
    })
}

pub fn exp_by_squares<N>(base: &N, mut exp: usize) -> N
where
    N: Mul<Output = N> + Clone + MulAssign,
{
    // Exponentiation by squares.
    let mut ans = base.clone();
    let mut multiplier = base.clone();
    let mut buf: N;
    while exp != 0 {
        if exp % 2 == 1 {
            ans *= multiplier.clone();
        }
        exp /= 2;
        buf = multiplier.clone();
        multiplier *= buf;
    }
    ans
}

pub fn mod_pow<T>(mut base: T, mut exp: T, modulus: T) -> T
where
    T: Num + Copy + Shr<Output = T> + From<u8> + PartialOrd,
{
    if modulus == T::one() {
        return T::zero();
    }
    if exp == T::zero() {
        T::one()
    } else {
        let mut result = T::one();
        base = base % modulus;
        loop {
            if exp % 2.into() == T::one() {
                result = result * base % modulus;
            }
            exp = exp >> T::one();
            if exp == T::zero() {
                break;
            }
            base = base * base % modulus;
        }
        result
    }
}

pub fn add_i_mod<T: Num + Signed + TryInto<usize>>(u: usize, i: &T, modulo: usize) -> usize {
    let i_as_u: usize = i.abs().try_into().ok().unwrap() % modulo;
    if i.is_negative() {
        if u >= i_as_u {
            u - i_as_u
        } else {
            u + modulo - i_as_u
        }
    } else {
        (u + i_as_u) % modulo
    }
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
    #[must_use]
    fn with_set_bit(self, bit_ix: u8, bit_value: bool) -> Self;
    fn set_bit(&mut self, bit_ix: u8, bit_value: bool);
    fn get_bit(self, bit_ix: u8) -> bool;
}
impl<N> NumBitExt for N
where
    N: Copy
        + Num
        + BitOr<Output = N>
        + BitAnd<Output = N>
        + Shl<usize, Output = N>
        + Not<Output = N>,
{
    #[inline]
    fn with_set_bit(self, bit_ix: u8, bit_value: bool) -> Self {
        if bit_value {
            self | (N::one() << bit_ix.into())
        } else {
            self & !(N::one() << bit_ix.into())
        }
    }

    #[inline]
    fn get_bit(self, bit_ix: u8) -> bool {
        (self & (N::one() << bit_ix.into())) != N::zero()
    }

    #[inline]
    fn set_bit(&mut self, bit_ix: u8, bit_value: bool) {
        *self = self.with_set_bit(bit_ix, bit_value);
    }
}

#[must_use]
pub fn is_sorted(i: &[u8]) -> bool {
    i.iter().zip(i.iter().skip(1)).all(|(a, b)| a <= b)
}

pub fn de_prefixsum<T: AddAssign + Default + Copy>(input: &[T]) -> Vec<T> {
    let mut total: T = Default::default();
    let mut ans = Vec::with_capacity(input.len());
    for i in input {
        total += *i;
        ans.push(total);
    }
    ans
}

pub fn find_upper<T: Integer + Copy>(func: &impl Fn(T) -> T, target: T) -> T {
    let mut upper = T::one();
    loop {
        let output = func(upper);
        if output >= target {
            return upper;
        }
        upper = upper + upper;
    }
}
pub fn bin_search<T: Integer + Copy>(func: &impl Fn(T) -> T, target: T, upper: T, lower: T) -> T {
    let candidate = (upper + lower) / (T::one() + T::one());
    if candidate == lower {
        return lower;
    }
    let val = func(candidate);
    if val >= target {
        bin_search(func, target, candidate, lower)
    } else {
        bin_search(func, target, upper, candidate)
    }
}
pub fn unbounded_bin_search<T: Integer + Copy>(func: impl Fn(T) -> T, target: T) -> T {
    let upper = find_upper(&func, target);
    bin_search(&func, target, upper, upper / (T::one() + T::one()))
}
pub fn mod_mul<T>(a: &T, b: &T, m: T) -> T
where
    T: CheckedMul + Rem<Output = T> + Debug,
{
    match a.checked_mul(b) {
        None => panic!("mod_mul overflowed with {a:?}x{b:?}%{m:?}"),
        Some(ab) => ab % m,
    }
}
pub fn mod_add<T>(a: &T, b: &T, m: T) -> T
where
    T: CheckedAdd + Rem<Output = T> + Debug,
{
    match a.checked_add(b) {
        None => panic!("mod_add overflowed with {a:?}+{b:?}%{m:?}"),
        Some(ab) => ab % m,
    }
}
pub fn mod_sub<T>(a: &T, b: &T, m: T) -> T
where
    T: CheckedSub + Rem<Output = T> + Debug,
{
    match a.checked_sub(b) {
        None => panic!("mod_sub underflowed with {a:?}-{b:?}%{m:?}"),
        Some(ab) => ab % m,
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;

    use super::*;

    #[test]
    pub fn test_mod_pow() {
        //2^5 = 32.
        for m in 1..100 {
            assert_eq!(mod_pow(2, 5, m), 32 % m);
        }

        assert_eq!(mod_pow(6, 25, 20), 16);
        assert_eq!(mod_pow(2, 64, 2), 0);
        assert_eq!(mod_pow(23895, 15, 14189), 344);
        assert_eq!(
            mod_pow(6_547_890_621_u128, 4_532_415_u128, 76_543_278_906_u128),
            1_039_609_179_u128
        );
    }
    #[quickcheck]
    fn mod_pow_works_for_checkable_values(a: u64, b: u32, m: u64) -> TestResult {
        if m == 0 || a.checked_pow(b).is_none() {
            return TestResult::discard();
        }
        eprintln!("{} ^ {} % {} == {}", a, b, m, a.pow(b) % m);
        TestResult::from_bool(mod_pow(a, b.into(), m) == a.pow(b) % m)
    }
    proptest! {
        #[test]
        fn mod_pow_proptest(a in 0u64..100u64, b in 0u32..32u32, m in 1u64..100u64) {
            eprintln!("{a} ^ {b} % {m}...");
            if m > 0 && a.checked_pow(b).is_some() {
                eprintln!("{} ^ {} % {} == {}",a,b,m,a.pow(b) % m);
                prop_assert_eq!(mod_pow(a, b.into(), m) , a.pow(b) % m);
            }
        }
    }
}
