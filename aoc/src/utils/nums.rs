use num::{CheckedAdd, CheckedMul, CheckedSub, Integer, Num};
use std::fmt::Debug;
use std::ops::{AddAssign, Rem, Shr};

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
pub fn mod_mul<T>(a: &T, b: &T, m: T) -> T
where
    T: CheckedMul + Rem<Output = T> + Debug,
{
    match a.checked_mul(b) {
        None => panic!("mod_mul overflowed with {:?}x{:?}%{:?}", a, b, m),
        Some(ab) => ab % m,
    }
}
pub fn mod_add<T>(a: &T, b: &T, m: T) -> T
where
    T: CheckedAdd + Rem<Output = T> + Debug,
{
    match a.checked_add(b) {
        None => panic!("mod_add overflowed with {:?}+{:?}%{:?}", a, b, m),
        Some(ab) => ab % m,
    }
}
pub fn mod_sub<T>(a: &T, b: &T, m: T) -> T
where
    T: CheckedSub + Rem<Output = T> + Debug,
{
    match a.checked_sub(b) {
        None => panic!("mod_sub underflowed with {:?}-{:?}%{:?}", a, b, m),
        Some(ab) => ab % m,
    }
}

pub fn mod_inv<T>(base: T, modulus: T) -> T
where
    T: Num + Copy + Shr<Output = T> + From<u8> + PartialOrd,
{
    mod_pow(base, modulus - 2.into(), modulus)
}
