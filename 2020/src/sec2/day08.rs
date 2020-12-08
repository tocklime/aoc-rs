use crate::utils::nums::add_isize;
use parse_display::{Display, FromStr};
use pathfinding::prelude::bfs;
use std::collections::HashSet;

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display(style = "lowercase")]
pub enum Op {
    Jmp,
    Acc,
    Nop,
}
impl Op {
    const fn switch(self) -> Self {
        match self {
            Self::Jmp => Self::Nop,
            Self::Acc => Self::Acc,
            Self::Nop => Self::Jmp,
        }
    }
}

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy)]
#[display("{op} {n}")]
pub struct Inst {
    op: Op,
    n: isize,
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct State {
    pc: usize,
    acc: isize,
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pc.hash(state)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pc == other.pc
    }
}
impl State {
    pub fn step(&self, inst: &Inst) -> Self {
        let mut new = *self;
        let mut pc_delta = 1;
        match &inst.op {
            Op::Acc => new.acc += inst.n,
            Op::Jmp => pc_delta = inst.n,
            Op::Nop => {}
        }
        new.pc = add_isize(self.pc, pc_delta);
        new
    }
}

pub fn go(prog: &[Inst], change: Option<usize>) -> (isize, bool) {
    let mut visited = HashSet::new();
    let mut state = State { pc: 0, acc: 0 };
    while state.pc < prog.len() && !visited.contains(&state.pc) {
        let mut inst = prog[state.pc];
        visited.insert(state.pc);
        if Some(state.pc) == change {
            inst.op = inst.op.switch();
        }
        state = state.step(&inst);
    }
    (state.acc, state.pc == prog.len())
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

#[aoc(day8, part2, bfs)]
pub fn p2d(input: &[Inst]) -> Option<isize> {
    let start = (false, State { pc: 0, acc: 0 });
    let d = bfs(
        &start,
        //sucessors function
        |&(have_switched, s)| {
            let inst = input[s.pc];
            let mut v = vec![(have_switched, s.step(&inst))];
            if !have_switched {
                let switched = Inst {
                    op: inst.op.switch(),
                    n: inst.n,
                };
                v.push((true, s.step(&switched)));
            }
            v
        },
        //success function
        |s| s.1.pc == input.len(),
    );
    d.and_then(|x| x.last().copied()).map(|x| x.1.acc)
}
