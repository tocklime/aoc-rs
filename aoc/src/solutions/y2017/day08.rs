use aoc_harness::aoc_main;

aoc_main!(2017 day 8, part1 [p1], part2 [p2]);
use reformation::Reformation;
use std::cmp::max;
use std::collections::HashMap;

#[derive(Reformation, Debug)]
enum Cmp {
    #[reformation(">")]
    Gt,
    #[reformation(">=")]
    Gte,
    #[reformation("<")]
    Lt,
    #[reformation("<=")]
    Lte,
    #[reformation("==")]
    Eq,
    #[reformation("!=")]
    Neq,
}

#[derive(Reformation, Debug)]
enum Op {
    #[reformation("inc")]
    Inc,
    #[reformation("dec")]
    Dec,
}

#[derive(Debug, Reformation)]
#[reformation("{target} {op} {op_size} if {cnd_target} {cnd_cmp} {cnd_val}")]
struct Line<'a> {
    target: &'a str,
    op: Op,
    op_size: i32,
    cnd_target: &'a str,
    cnd_cmp: Cmp,
    cnd_val: i32,
}

fn p1(input: &str) -> i32 {
    let mut regs: HashMap<&str, i32> = HashMap::new();
    for l in input.lines() {
        let l = Line::parse(l).unwrap();
        let go = match (regs.get(l.cnd_target).unwrap_or(&0), l.cnd_cmp, &l.cnd_val) {
            (a, Cmp::Gt, b) => a > b,
            (a, Cmp::Gte, b) => a >= b,
            (a, Cmp::Lt, b) => a < b,
            (a, Cmp::Lte, b) => a <= b,
            (a, Cmp::Eq, b) => a == b,
            (a, Cmp::Neq, b) => a != b,
        };
        if go {
            let e = regs.entry(l.target).or_default();
            match l.op {
                Op::Inc => *e += l.op_size,
                Op::Dec => *e -= l.op_size,
            }
        }
    }
    *regs.values().max().unwrap()
}

fn p2(input: &str) -> i32 {
    let mut regs: HashMap<&str, i32> = HashMap::new();
    let mut highest = 0;
    for l in input.lines() {
        let l = Line::parse(l).unwrap();
        let go = match (regs.get(l.cnd_target).unwrap_or(&0), l.cnd_cmp, &l.cnd_val) {
            (a, Cmp::Gt, b) => a > b,
            (a, Cmp::Gte, b) => a >= b,
            (a, Cmp::Lt, b) => a < b,
            (a, Cmp::Lte, b) => a <= b,
            (a, Cmp::Eq, b) => a == b,
            (a, Cmp::Neq, b) => a != b,
        };
        if go {
            let e = regs.entry(l.target).or_default();
            match l.op {
                Op::Inc => *e += l.op_size,
                Op::Dec => *e -= l.op_size,
            }
            highest = max(highest, *e);
        }
    }
    highest
}
