use crate::utils::nums::add_isize;
use parse_display::{Display, FromStr};
use std::collections::HashSet;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "lowercase")]
pub enum Op {
    Jmp,
    Acc,
    Nop,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{op} {n}")]
pub struct Inst {
    op: Op,
    n: isize,
}

pub fn go(prog: &[Inst], change: Option<usize>) -> (isize, bool) {
    let mut acc: isize = 0;
    let mut visited = HashSet::new();
    let mut pc = 0;
    while pc < prog.len() && !visited.contains(&pc) {
        let inst = &prog[pc];
        visited.insert(pc);
        let mut pc_delta = 1;
        match (&inst.op, Some(pc) == change) {
            (Op::Acc, _) => acc += inst.n,
            (Op::Jmp, false) | (Op::Nop, true) => pc_delta = inst.n,
            (Op::Nop, false) | (Op::Jmp, true) => {}
        }
        pc = add_isize(pc, pc_delta);
    }
    (acc, pc == prog.len())
}

#[aoc_generator(day8)]
pub fn gen(input: &str) -> Vec<Inst> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}
#[aoc(day8, part1)]
pub fn p1(input: &[Inst]) -> isize {
    go(input, None).0
}

#[aoc(day8, part2)]
pub fn p2(input: &[Inst]) -> Option<isize> {
    (0..input.len()).find_map(|i| {
        if input[i].op == Op::Acc {
            None
        } else {
            let (acc, fin) = go(input, Some(i));
            if fin {
                Some(acc)
            } else {
                None
            }
        }
    })
}
