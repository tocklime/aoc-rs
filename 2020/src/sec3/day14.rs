use bitvec::{
    order::Lsb0,
    prelude::{BitField, BitVec},
    view::BitView,
};
use std::{collections::HashMap, num::ParseIntError, str::FromStr};

pub struct Mask {
    lo: u64,
    xs: u64,
    x_locations: Vec<usize>
}
pub enum Line {
    SetMask(Mask),
    SetMem(u64, u64),
}
impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(m) = s.strip_prefix("mask = ") {
            let lo = m.chars().rev().map(|x| x == '1').collect::<BitVec>().load();
            let xs_vec = m.chars().rev().map(|x| x == 'X').collect::<BitVec>();
            let xs = xs_vec.load();
            let x_locations = xs_vec.iter().enumerate().filter_map(|(ix,&x)| if x {Some(ix)}else {None}).collect();
            Ok(Self::SetMask(Mask { lo, xs, x_locations }))
        } else {
            let sp: Vec<&str> = s.split(|c| "[] =".contains(c)).collect();
            let addr = sp[1].parse()?;
            let val = sp[5].parse()?;
            Ok(Self::SetMem(addr, val))
        }
    }
}

#[aoc_generator(day14)]
pub fn gen(input: &str) -> Vec<Line> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

pub fn run<F> (input: &[Line],set_mem: F) -> u64
where F : Fn(&mut HashMap<u64,u64>,&Mask,u64,u64) {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = None;
    for l in input {
        match l {
            Line::SetMask(m) => {
                mask = Some(m);
            }
            &Line::SetMem(target, value) => {
                set_mem(&mut mem, mask.expect("Mask use before set"),target,value);
            }
        }
    }
    mem.values().sum()

}
#[aoc(day14, part1)]
pub fn p1(input: &[Line]) -> u64 {
    run(input, |mem,mask,target,value| {
        mem.insert(target, (value & mask.xs) | mask.lo);
    })
}

fn get_mem_vals(mask: &Mask, value: u64) -> Vec<u64> {
    let mask_bits = mask.xs.view_bits::<Lsb0>();
    let val_mask = mask.lo | value;
    mask_bits.iter().enumerate().fold(vec![0], |acc, (ix, &mask_bit)| {
        if mask_bit {
            (0..=1)
                .flat_map(|new_bit| acc.iter().map(move |x| (new_bit << ix) | x))
                .collect()
        } else {
            acc.iter().map(|x| ((1 << ix) & val_mask) | x).collect()
        }
    })
}

#[aoc(day14, part2)]
pub fn p2(input: &[Line]) -> u64 {
    run(input, |mem,mask,target,value| {
        let mut val = mask.lo | target;
        let this = val.view_bits_mut::<Lsb0>();
        for n in 0..(1<<mask.x_locations.len()) {
            for x in &mask.x_locations{
                this.set(*x,(1<<x) & n > 0);
            }
            mem.insert(this.load(), value);
        }
    })
}
