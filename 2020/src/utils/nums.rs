use num::{Integer, Num, Signed};
use std::{convert::TryInto, iter::{Product, Sum}, ops::Shr};
use modinverse::modinverse;

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

pub fn add_i<T : Num + Signed + TryInto<usize>>(u : usize, i: &T) -> usize {
    let i_as_u : usize = i.abs().try_into().ok().unwrap();
    if i.is_negative() {
        u - i_as_u
    } else {
        u + i_as_u
    }
}
pub fn add_assign_i<T : Num + Signed + TryInto<usize>>(u : &mut usize, i: &T) {
    let i_as_u : usize = i.abs().try_into().ok().unwrap();
    if i.is_negative() {
        *u -= i_as_u;
    } else {
        *u += i_as_u;
    }
}

pub fn chinese_remainder_theorem<T>(list : &[(T,T)]) -> T 
    where T : Num + Product + Sum + Integer + Copy,
{
    let m_prod: T = list.iter().map(|x| x.1).product();
    list.iter().map(|&(x,m)| x * (m_prod/m) * modinverse(m_prod/m,m).unwrap()).sum::<T>() % m_prod
}