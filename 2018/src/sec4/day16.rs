use crate::utils::comp::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

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
        let before = Device::with_regs(vec![
            b[1].parse().unwrap(),
            b[2].parse().unwrap(),
            b[3].parse().unwrap(),
            b[4].parse().unwrap(),
        ]);
        let a = state_re.captures(ch[2]).unwrap();
        let after = Device::with_regs(vec![
            a[1].parse().unwrap(),
            a[2].parse().unwrap(),
            a[3].parse().unwrap(),
            a[4].parse().unwrap(),
        ]);
        let insn = Insn::parse_basic(ch[1]);
        let c = ops
            .iter()
            .filter(|o| {
                let i = Insn::Op(**o, insn.1[0], insn.1[1], insn.1[2]);
                let mut x = before.clone();
                x.eval(i);
                x == after
            })
            .count();
        if c >= 3 {
            match_count += 1;
        }
    }
    match_count
}

#[aoc(day16, part2)]
fn p2(input: &str) -> N {
    let input = input.replace("\r", "");
    let secs: Vec<&str> = input.split("\n\n\n\n").collect();
    let lines = secs[0].lines().collect_vec();
    let ops = Op::all();
    let mut options = HashMap::new();
    let state_re = Regex::new(r"\w+:\s+\[(\d+), (\d+), (\d+), (\d+)\]").unwrap();
    for ch in lines.chunks(4) {
        let b = state_re.captures(ch[0]).unwrap();
        let before = Device::with_regs(vec![
            b[1].parse().unwrap(),
            b[2].parse().unwrap(),
            b[3].parse().unwrap(),
            b[4].parse().unwrap(),
        ]);
        let a = state_re.captures(ch[2]).unwrap();
        let after = Device::with_regs(vec![
            a[1].parse().unwrap(),
            a[2].parse().unwrap(),
            a[3].parse().unwrap(),
            a[4].parse().unwrap(),
        ]);
        let insn = Insn::parse_basic(ch[1]);
        let opts: HashSet<Op> = ops
            .iter()
            .filter(|o| {
                let i = Insn::Op(**o, insn.1[0], insn.1[1], insn.1[2]);
                let mut x = before.clone();
                x.eval(i);
                x == after
            })
            .cloned()
            .collect();
        options
            .entry(insn.0)
            .and_modify(|x: &mut HashSet<Op>| *x = x.intersection(&opts).cloned().collect())
            .or_insert(opts);
    }
    let mut known = HashMap::new();
    loop {
        let singleton = options.iter().find(|(_, v)| v.len() == 1);
        match singleton {
            None => break,
            Some((&n, single)) => {
                let op = *single.iter().next().unwrap();
                known.insert(n, op);
                options.remove(&n);
                for hs in options.values_mut() {
                    hs.remove(&op);
                }
            }
        }
    }
    assert_eq!(options.len(), 0);
    //now, evaluate the rest.
    let mut state = Device::new(4);
    for l in secs[1].lines() {
        let p = Insn::parse_basic(l);
        let insn = Insn::Op(known[&p.0], p.1[0], p.1[1], p.1[2]);
        state.eval(insn);
    }
    state.regs[0]
}
