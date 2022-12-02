#![allow(clippy::redundant_pattern_matching)]
use itertools::Itertools;
use reformation::Reformation;
use std::convert::TryInto;

pub type N = i64;

#[derive(Reformation, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    #[reformation("i")]
    I,
    #[reformation("r")]
    R,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reformation)]
pub enum Op {
    #[reformation("add{}")]
    Add(Mode),
    #[reformation("mul{}")]
    Mul(Mode),
    #[reformation("ban{}")]
    Ban(Mode),
    #[reformation("bor{}")]
    Bor(Mode),
    #[reformation("set{}")]
    Set(Mode),
    #[reformation("gt{}{}")]
    Gt(Mode, Mode),
    #[reformation("eq{}{}")]
    Eq(Mode, Mode),
}

impl Op {
    #[must_use]
    pub fn all() -> [Self; 16] {
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
pub struct Device {
    pub regs: Vec<N>,
    pub ip: Option<usize>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Reformation)]
pub enum Macro {
    #[reformation("#ip {}")]
    SetIp(usize),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Reformation)]
pub enum Insn {
    #[reformation("{} {} {} {}")]
    Op(Op, N, N, N),
    //  #[reformation("{}")]
    //  Macro(Macro),
}

impl Insn {
    #[must_use]
    pub fn parse_basic(input: &str) -> (N, [N; 3]) {
        let v = input
            .split(' ')
            .map(|x| x.trim().parse::<N>().expect("parse_basic"))
            .collect_vec();
        (v[0], [v[1], v[2], v[3]])
    }
}

impl Device {
    #[must_use]
    pub fn new(reg_count: usize) -> Self {
        Self {
            regs: vec![0; reg_count],
            ip: None,
        }
    }
    #[must_use]
    pub fn with_regs(regs: Vec<N>) -> Self {
        Self { regs, ip: None }
    }
    #[must_use]
    pub fn get_r(&self, reg: N) -> N {
        let u: usize = reg.try_into().unwrap();
        self.regs[u]
    }
    #[must_use]
    pub fn get(&self, reg: N, m: Mode) -> N {
        match m {
            Mode::I => reg,
            Mode::R => self.get_r(reg),
        }
    }
    pub fn set(&mut self, reg: N, val: N) {
        let u: usize = reg.try_into().unwrap();
        self.regs[u] = val;
    }
    #[allow(clippy::many_single_char_names)]
    pub fn eval(&mut self, i: Insn) {
        match i {
            Insn::Op(Op::Add(m), a, b, c) => {
                self.set(c, self.get_r(a) + self.get(b, m));
            }
            Insn::Op(Op::Mul(m), a, b, c) => {
                self.set(c, self.get_r(a) * self.get(b, m));
            }
            Insn::Op(Op::Ban(m), a, b, c) => {
                self.set(c, self.get_r(a) & self.get(b, m));
            }
            Insn::Op(Op::Bor(m), a, b, c) => {
                self.set(c, self.get_r(a) | self.get(b, m));
            }
            Insn::Op(Op::Set(m), a, _, c) => {
                self.set(c, self.get(a, m));
            }
            Insn::Op(Op::Gt(m, n), a, b, c) => {
                self.set(c, i64::from(self.get(a, m) > self.get(b, n)));
            }
            Insn::Op(Op::Eq(m, n), a, b, c) => {
                self.set(c, i64::from(self.get(a, m) == self.get(b, n)));
            }
        }
    }
    pub fn run_to_fn<F>(&mut self, prog: &[Insn], breaks: F) -> bool
    where
        F: Fn(i64) -> bool,
    {
        let ip = self.ip.unwrap();
        loop {
            let as_u: Option<usize> = self.regs[ip].try_into().ok();
            match as_u.and_then(|l| prog.get(l)) {
                None => return false,
                Some(m) => {
                    self.eval(*m);
                    self.regs[ip] += 1;
                    if breaks(self.regs[ip]) {
                        return true;
                    }
                }
            }
        }
    }
    pub fn run_to_ip(&mut self, prog: &[Insn], target: usize) {
        let ip = self.ip.unwrap();
        loop {
            let as_u: Option<usize> = self.regs[ip].try_into().ok();
            if as_u == Some(target) {
                break;
            }
            match as_u.and_then(|l| prog.get(l)) {
                None => break,
                Some(m) => {
                    self.eval(*m);
                    self.regs[ip] += 1;
                }
            }
        }
    }
    pub fn run(&mut self, prog: &[Insn]) {
        let ip = self.ip.unwrap();
        loop {
            let as_u: Option<usize> = self.regs[ip].try_into().ok();
            match as_u.and_then(|l| prog.get(l)) {
                None => break,
                Some(m) => {
                    self.eval(*m);
                    self.regs[ip] += 1;
                }
            }
        }
    }
}
