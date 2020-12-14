use bitvec::{order::Lsb0, view::BitView};
use std::collections::HashMap;
use EitherOrBoth::{Both, Left, Right};

use itertools::{EitherOrBoth, Itertools};
use parse_display::{Display, FromStr};

#[derive(FromStr, Display, Debug)]
pub enum Line {
    #[display(r"mask = {0}")]
    SetMask(String),
    #[display(r"mem[{0}] = {1}")]
    SetMem(usize, usize),
}

#[aoc_generator(day14)]
pub fn gen(input: &str) -> Vec<Line> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day14, part1)]
pub fn p1(input: &[Line]) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask: Vec<char> = Vec::new();
    for l in input {
        match l {
            Line::SetMask(m) => {
                mask = m.chars().rev().collect();
            }
            &Line::SetMem(target, value) => {
                let masked_val: usize = value
                    .view_bits::<Lsb0>()
                    .iter()
                    .zip_longest(&mask)
                    .enumerate()
                    .fold(0, |acc, (ix, e)| {
                        let bit = match e {
                            Both(&x, 'X') | Left(&x) => x,
                            Both(_, c) | Right(c) => *c == '1',
                        };
                        acc | (if bit { 1 } else { 0 } << ix)
                    });
                mem.insert(target, masked_val);
            }
        }
    }
    mem.values().sum()
}

fn get_mem_vals(mask: &[char], value: usize) -> Vec<usize> {
    mask.iter().enumerate().fold(vec![0], |opts, (ix, c)| {
        let value_bit = value & (1 << ix);
        match *c {
            '0' => opts.iter().map(|x| (value_bit) | x).collect(),
            '1' => opts.iter().map(|x| (1 << ix) | x).collect(),
            'X' => (0..=1)
                .flat_map(|new_bit| opts.iter().map(move |x| (new_bit << ix) | x))
                .collect(),
            _ => unreachable!(),
        }
    })
}

#[aoc(day14, part2)]
pub fn p2(input: &[Line]) -> usize {
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask: Vec<char> = Vec::new();
    for l in input {
        match l {
            Line::SetMask(m) => {
                mask = m.chars().rev().collect();
            }
            &Line::SetMem(target, value) => {
                let ts = get_mem_vals(&mask, target);
                for t in ts {
                    mem.insert(t, value);
                }
            }
        }
    }
    mem.values().sum()
}

//850308481936 too low
//1359192470831 too low
//3564822193820 is right.

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn expanding() {
        assert_eq!(vec![0, 1], get_mem_vals(&['X'], 0));
        assert_eq!(vec![1], get_mem_vals(&['0'], 1));
        assert_eq!(vec![1], get_mem_vals(&['1'], 0));
        assert_eq!(vec![58, 59], get_mem_vals(&['X', '1', '0', '0', '1', '0'], 42));
        assert_eq!(vec![26, 27, 58, 59], get_mem_vals(&['X', '1', '0', '0', '1', 'X'], 42));
    }
}
