use bitvec::{order::Lsb0, prelude::{BitField, BitVec}, view::BitView};
use std::{collections::HashMap, num::ParseIntError, str::FromStr};

pub struct Mask {
    lo : u64,
    mask: u64
}
impl Mask {
    #[allow(clippy::missing_const_for_fn)]
    pub fn mask_size(&self) -> u64 {
        u64::pow(2, self.mask.count_ones())
    }
}
pub enum Line {
    SetMask(Mask),
    SetMem(u64, u64),
}
impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(m) = s.strip_prefix("mask = ") {
            let lo =  m.chars().rev().map(|x| x == '1').collect::<BitVec>().load(); //0,X=> 0, 1=>1
            let mask = m.chars().rev().map(|x| x == 'X').collect::<BitVec>().load(); //0,1=>0,X=> 1
            Ok(Self::SetMask(Mask{lo,mask}))
        }else {
            let sp :Vec<&str> = s.split(|c| "[] =".contains(c)).collect();
            let addr = sp[1].parse()?;
            let val = sp[5].parse()?;
            Ok(Self::SetMem(addr,val))
        }
    }
}

#[aoc_generator(day14)]
pub fn gen(input: &str) -> Vec<Line> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day14, part1)]
pub fn p1(input: &[Line]) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask  = None;
    for l in input {
        match l {
            Line::SetMask(m) => {
                mask = Some(m);
            }
            &Line::SetMem(target, value) => {
                let v = (value & mask.unwrap().mask) | mask.unwrap().lo;
                mem.insert(target, v);
            }
        }
    }
    mem.values().sum()
}

fn get_mem_vals(mask: &Mask, value: u64) -> Vec<u64> {
    mask.mask.view_bits::<Lsb0>().iter()
    .zip(mask.lo.view_bits::<Lsb0>())
    .zip(value.view_bits::<Lsb0>())
    .enumerate()
    .fold(vec![0], |acc,(ix,((mask_x,mask_01),value_bit))|{
        if *mask_x {
            (0..=1).flat_map(|new_bit| acc.iter().map(move |x| (new_bit << ix) | x)).collect()
        } else {
            let v = *mask_01 || *value_bit;
            acc.iter().map(|x| ((v as u64) << ix) | x).collect()
        }
    })
}

#[aoc(day14, part2)]
pub fn p2(input: &[Line]) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = None;
    for l in input {
        match l {
            Line::SetMask(m) => {
                mask = Some(m);
            }
            &Line::SetMem(target, value) => {
                let ts = get_mem_vals(mask.unwrap(), target);
                for t in ts {
                    mem.insert(t, value);
                }
            }
        }
    }
    mem.values().sum()
}