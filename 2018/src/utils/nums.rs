use num::Num;
use std::ops::{DivAssign, MulAssign, RemAssign, Shr};

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
