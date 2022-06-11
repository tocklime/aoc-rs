aoc_harness::aoc_main!(2018 day 21, part1 [p1], part2 [p2]);
use utils::comp::*;
use itertools::Itertools;
use reformation::Reformation;
use std::collections::HashSet;


fn p1(input: &str) -> i64 {
    let Macro::SetIp(ip) = Macro::parse(input.lines().next().unwrap()).unwrap();
    let mut d = Device::new(6);
    d.ip = Some(ip);
    let prog: Vec<Insn> = input
        .lines()
        .skip(1)
        .map(|l| Insn::parse(l).unwrap())
        .collect_vec();
    while d.run_to_fn(&prog, |x| x == 18 || x == 28) {
        match d.get_r(2) {
            18 => {
                //now find value for r3 such that (r3+1) * 256 > r1.
                let r1 = d.get_r(1);
                let mut t = r1 / 256;
                while (t + 1) * 256 <= r1 {
                    t += 1;
                }
                d.set(3, t);
            }
            28 => {
                break;
            }
            _ => {}
        }
    }
    d.regs[4]
}

fn p2(input: &str) -> i64 {
    let Macro::SetIp(ip) = Macro::parse(input.lines().next().unwrap()).unwrap();
    let mut d = Device::new(6);
    d.ip = Some(ip);
    let prog: Vec<Insn> = input
        .lines()
        .skip(1)
        .map(|l| Insn::parse(l).unwrap())
        .collect_vec();
    let mut seen = HashSet::new();
    let mut last_insert = None;
    while d.run_to_fn(&prog, |x| x == 18 || x == 28) {
        match d.get_r(2) {
            18 => {
                //now find value for r3 such that (r3+1) * 256 > r1.
                let r1 = d.get_r(1);
                let mut t = r1 / 256;
                while (t + 1) * 256 <= r1 {
                    t += 1;
                }
                d.set(3, t);
            }
            28 => {
                if !seen.insert(d.regs[4]) {
                    break;
                } else {
                    last_insert = Some(d.regs[4]);
                }
            }
            _ => {}
        }
    }
    last_insert.unwrap()
}

/*

#ip 2
0 seti 123 0 4      r4 = 123
1 bani 4 456 4      r4 = 123 & 456
2 eqri 4 72 4       r4 = r4 == 72
3 addr 4 2 2        r2 += r4
4 seti 0 0 2        r2 = 0 # inf loop.
5 seti 0 1 4        r4 = 0 # reset.

6 bori 4 65536 1        r1 = r4 | 0x1000
7 seti 16031208 7 4     r4 = 16031208 (0xF49DE8)

8 bani 1 255 3          r3 = r1 & 255 (0xFF)
9 addr 4 3 4            r4 = r4 + r3 (r4 += r1 & 0xFF + 0xF49DE8)
10 bani 4 16777215 4    r4 = r4 & 0xFFFFFF (modulo)
11 muli 4 65899 4       r4 *= 65899 (0x1016B)
12 bani 4 16777215 4    r4 &= 0xFFFFFF (modulo)   / r4 += (((r1 & 0xFF + 0xF49DE8) & 0xFFFFFF) * 0x1016B) & 0xFFFFFF
13 gtir 256 1 3         r3 = 256 > r1
14 addr 3 2 2           r2 += r3             if 256 > r1
15 addi 2 1 2           r2 += 1                 END if r4 == r0
16 seti 27 3 2          r2 = 27 GOTO 28!        GOTO 6

17 seti 0 9 3           r3 = 0

18 addi 3 1 5           r5 = r3 + 1         loop:
19 muli 5 256 5         r5 *= 256
20 gtrr 5 1 5           r5 = r5 > r1            if (r3 + 1) * 256 > r1:
21 addr 5 2 2           r2 += r5                     r1 = r3
22 addi 2 1 2           r2 += 2                      GOTO 8
23 seti 25 7 2          GOTO 26                 else:
                                                     r3 += 1
24 addi 3 1 3           r3 += 1
25 seti 17 4 2          GOTO 18

26 setr 3 1 1           r1 = r3
27 seti 7 5 2           r2 = 7  GOTO 8

28 eqrr 4 0 3           r3 = r4 == r0 end if r4 == r0
29 addr 3 2 2           r2 += r3   end if r3 > 0
30 seti 5 1 2           r2 = 5 GOTO 6.



*/
