aoc_harness::aoc_main!(2018 day 19, part1 [p1], part2 [p2]);
use utils::comp::*;
use itertools::Itertools;
use reformation::Reformation;


fn p1(input: &str) -> i64 {
    let Macro::SetIp(ip) = Macro::parse(input.lines().next().unwrap()).unwrap();
    let mut d = Device::new(6);
    d.ip = Some(ip);
    let prog: Vec<Insn> = input
        .lines()
        .skip(1)
        .map(|l| Insn::parse(l).unwrap())
        .collect_vec();
    d.run(&prog);
    d.regs[0]
}

fn p2(input: &str) -> i64 {
    let Macro::SetIp(ip) = Macro::parse(input.lines().next().unwrap()).unwrap();
    let mut d = Device::new(6);
    d.ip = Some(ip);
    d.set(0, 1);
    let prog: Vec<Insn> = input
        .lines()
        .skip(1)
        .map(|l| Insn::parse(l).unwrap())
        .collect_vec();
    d.run_to_ip(&prog, 1);
    let t = d.regs[3];
    //now find sum of divisors of t.
    (1..=t).filter(|x| t % x == 0).sum()
}
