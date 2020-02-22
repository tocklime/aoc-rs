use num::Num;
use std::ops::Shr;

pub fn mod_pow<T>(mut base: T, mut exp: T, modulus: T) -> T
    where T: Num + Copy + Shr<Output = T> + From<u8> + PartialOrd
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