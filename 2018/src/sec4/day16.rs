use std::convert::TryInto;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashSet, HashMap};

type N = i64;


#[derive(Debug, Copy, Clone, PartialEq, Eq,Hash)]
enum Mode {
    I,
    R,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq,Hash)]
enum Op {
    Add(Mode),
    Mul(Mode),
    Ban(Mode),
    Bor(Mode),
    Set(Mode),
    Gt(Mode, Mode),
    Eq(Mode, Mode),
}

impl Op {
    fn all() -> [Self; 16] {
        [
            Op::Add(Mode::I),
            Op::Add(Mode::R),
            Op::Mul(Mode::I),
            Op::Mul(Mode::R),
            Op::Ban(Mode::I),
            Op::Ban(Mode::R),
            Op::Bor(Mode::I),
            Op::Bor(Mode::R),
            Op::Set(Mode::I),
            Op::Set(Mode::R),
            Op::Gt(Mode::I, Mode::R),
            Op::Gt(Mode::R, Mode::I),
            Op::Gt(Mode::R, Mode::R),
            Op::Eq(Mode::I, Mode::R),
            Op::Eq(Mode::R, Mode::I),
            Op::Eq(Mode::R, Mode::R),
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Device {
    regs: [N; 4],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Insn {
    op: Op,
    args: [N; 3],
}

impl Insn {
    fn parse_basic(input: &str) -> (N, [N; 3]) {
        let v = input.split(' ').map(|x| x.trim().parse::<N>().expect("parse_basic")).collect_vec();
        (v[0], [v[1], v[2], v[3]])
    }
}

impl Device {
    fn get_r(&self, reg: N) -> N {
        let u: usize = reg.try_into().unwrap();
        self.regs[u]
    }
    fn get(&self, reg: N, m: Mode) -> N {
        match m {
            Mode::I => reg,
            Mode::R => self.get_r(reg)
        }
    }
    fn set(&mut self, reg: N, val: N) {
        let u: usize = reg.try_into().unwrap();
        self.regs[u] = val;
    }
    fn eval(&mut self, i: Insn) {
        match i.op {
            Op::Add(m) => {
                self.set(i.args[2], self.get_r(i.args[0]) + self.get(i.args[1], m));
            }
            Op::Mul(m) => {
                self.set(i.args[2], self.get_r(i.args[0]) * self.get(i.args[1], m));
            }
            Op::Ban(m) => {
                self.set(i.args[2], self.get_r(i.args[0]) & self.get(i.args[1], m));
            }
            Op::Bor(m) => {
                self.set(i.args[2], self.get_r(i.args[0]) | self.get(i.args[1], m));
            }
            Op::Set(m) => {
                self.set(i.args[2], self.get(i.args[0], m));
            }
            Op::Gt(m, n) => {
                self.set(i.args[2], if self.get(i.args[0], m) > self.get(i.args[1], n) { 1 } else { 0 });
            }
            Op::Eq(m, n) => {
                self.set(i.args[2], if self.get(i.args[0], m) == self.get(i.args[1], n) { 1 } else { 0 });
            }
        }
    }
}

#[aoc(day16, part1)]
fn p1(input: &str) -> usize {
    let input = input.replace("\r", "");
    let secs: Vec<&str> = input.split("\n\n\n").collect();
    let mut match_count = 0;
    let lines = secs[0].lines().collect_vec();
    let ops = Op::all();
    let state_re = Regex::new(r"\w+:\s+\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    for ch in lines.chunks(4) {
        let b = state_re.captures(ch[0]).unwrap();
        let before = Device {
            regs: [b[1].parse().unwrap(),
                b[2].parse().unwrap(),
                b[3].parse().unwrap(),
                b[4].parse().unwrap()]
        };
        let a = state_re.captures(ch[2]).unwrap();
        let after = Device {
            regs: [a[1].parse().unwrap(),
                a[2].parse().unwrap(),
                a[3].parse().unwrap(),
                a[4].parse().unwrap()]
        };
        let insn = Insn::parse_basic(ch[1]);
        let c = ops.iter().filter(|o| {
            let i = Insn { op: **o, args: insn.1 };
            let mut x = before.clone();
            x.eval(i);
            x == after
        }).count();
        if c >= 3 {
            match_count += 1;
        }
    }
    match_count
}
#[aoc(day16,part2)]
fn p2(input: &str) -> N {
    let input = input.replace("\r", "");
    let secs: Vec<&str> = input.split("\n\n\n\n").collect();
    let lines = secs[0].lines().collect_vec();
    let ops = Op::all();
    let mut options = HashMap::new();
    let state_re = Regex::new(r"\w+:\s+\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    for ch in lines.chunks(4) {
        let b = state_re.captures(ch[0]).unwrap();
        let before = Device {
            regs: [b[1].parse().unwrap(),
                b[2].parse().unwrap(),
                b[3].parse().unwrap(),
                b[4].parse().unwrap()]
        };
        let a = state_re.captures(ch[2]).unwrap();
        let after = Device {
            regs: [a[1].parse().unwrap(),
                a[2].parse().unwrap(),
                a[3].parse().unwrap(),
                a[4].parse().unwrap()]
        };
        let insn = Insn::parse_basic(ch[1]);
        let opts : HashSet::<Op> = ops.iter().filter(|o| {
            let i = Insn { op: **o, args: insn.1 };
            let mut x = before.clone();
            x.eval(i);
            x == after
        }).cloned().collect();
        options.entry(insn.0).and_modify(|x: &mut HashSet<Op>| *x = x.intersection(&opts).cloned().collect()).or_insert(opts);
    }
    let mut known = HashMap::new();
    loop {
        let singleton = options.iter().find(|(_,v)| v.len() == 1);
        match singleton {
            None => break,
            Some((&n,single)) => {
                let op = *single.iter().nth(0).unwrap();
                known.insert(n,op);
                options.remove(&n);
                for hs in options.values_mut() {
                    hs.remove(&op);
                }
            }
        }
    }
    assert_eq!(options.len(),0);
    //now, evaluate the rest.
    let mut state = Device {
        regs : [0,0,0,0]
    };
    for l in secs[1].lines() {
        let p = Insn::parse_basic(l);
        let insn = Insn {
            args : p.1,
            op : known[&p.0]
        };
        state.eval(insn);
    }
    state.regs[0]
}