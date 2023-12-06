

aoc_harness::aoc_main!(2020 day 8, generator gen, part1 [p1], part2 [p2, p2d, p2_optimal]);
use parse_display::{Display, FromStr};
use pathfinding::prelude::bfs;
use std::collections::HashSet;

use utils::nums::add_i;

#[derive(Display, FromStr, PartialEq, Eq, Debug, Clone, Copy)]
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

#[derive(Display, FromStr, PartialEq, Eq, Debug, Clone, Copy)]
#[display("{op} {n}")]
pub struct Inst {
    op: Op,
    n: i64,
}

#[derive(Debug, Clone, Copy, Eq)]
pub struct State {
    pc: usize,
    acc: i64,
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pc.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pc == other.pc
    }
}
impl State {
    fn step(&self, inst: &Inst) -> Self {
        let mut new = *self;
        let mut pc_delta = 1;
        match &inst.op {
            Op::Acc => new.acc += inst.n,
            Op::Jmp => pc_delta = inst.n,
            Op::Nop => {}
        }
        new.pc = add_i(self.pc, &pc_delta);
        new
    }
}

fn go(prog: &[Inst], change: Option<usize>) -> (i64, bool) {
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

fn gen(input: &str) -> Vec<Inst> {
    input.trim().lines().map(|x| x.parse().unwrap()).collect()
}
fn p1(input: &[Inst]) -> i64 {
    go(input, None).0
}

fn p2(input: &[Inst]) -> Option<i64> {
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

fn p2d(input: &[Inst]) -> Option<i64> {
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

fn explore(
    input: &[Inst],
    mut state: State,
    visited: &mut HashSet<(bool, usize)>,
    allow_flip: bool,
) -> Option<i64> {
    //in visited, we store the state and whether there had been a flip or not. We may get to the same state via
    //different means, and this nicely prunes those after the first exploration.
    while !visited.contains(&(allow_flip, state.pc)) {
        if state.pc == input.len() {
            return Some(state.acc);
        }
        let inst = input[state.pc];
        visited.insert((allow_flip, state.pc));
        if allow_flip && inst.op != Op::Acc {
            //try flipping it and running to the end.
            let flipped = Inst {
                op: inst.op.switch(),
                n: inst.n,
            };
            if let Some(x) = explore(input, state.step(&flipped), visited, false) {
                return Some(x);
            }
            //nope, that didn't work. carry on with the normal one.
        }
        state = state.step(&inst);
    }
    None
}

fn p2_optimal(input: &[Inst]) -> Option<i64> {
    let state = State { pc: 0, acc: 0 };
    let mut visited = HashSet::new();
    explore(input, state, &mut visited, true)
}
