use std::fmt::Debug;

use aoc_harness::*;
use pathfinding::prelude::{dfs, dijkstra};

aoc_main!(2021 day 24, part1 [p1]);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Alu {
    wxyz: [isize; 4],
    line: usize,
    next_input: Option<isize>,
    state: AluState,
}

fn to_reg_ix(c: &str) -> usize {
    match c {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => unreachable!(),
    }
}
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum AluState {
    NeedsInput,
    Done,
    Crashed,
    Going,
}
impl Alu {
    fn read(&self, x: &str) -> isize {
        match x {
            "w" => self.wxyz[0],
            "x" => self.wxyz[1],
            "y" => self.wxyz[2],
            "z" => self.wxyz[3],
            _ => x.parse().unwrap(),
        }
    }
    fn run_to_input(&mut self, code: &[&str]) -> AluState {
        loop {
            if self.line == code.len() {
                return AluState::Done;
            }
            let line = code[self.line];
            match self.execute(line) {
                AluState::NeedsInput => return AluState::NeedsInput,
                AluState::Done => return AluState::Done,
                AluState::Crashed => return AluState::Crashed,
                AluState::Going => {
                    self.line += 1;
                }
            }
        }
    }
    fn execute(&mut self, input: &str) -> AluState {
        let s = input.split(' ').collect_vec();
        match s[0] {
            "inp" => {
                self.wxyz[to_reg_ix(s[1])] = match self.next_input.take() {
                    Some(x) => x,
                    None => return AluState::NeedsInput,
                }
            }
            "add" => self.wxyz[to_reg_ix(s[1])] = self.read(s[1]) + self.read(s[2]),
            "mul" => self.wxyz[to_reg_ix(s[1])] = self.read(s[1]) * self.read(s[2]),
            "div" => {
                self.wxyz[to_reg_ix(s[1])] = {
                    let b = self.read(s[2]);
                    if b == 0 {
                        return AluState::Crashed;
                    }
                    self.read(s[1]) / b
                }
            }
            "mod" => {
                self.wxyz[to_reg_ix(s[1])] = {
                    let a = self.read(s[1]);
                    let b = self.read(s[2]);
                    if a < 0 || b <= 0 {
                        return AluState::Crashed;
                    }
                    a % b
                }
            }
            "eql" => {
                self.wxyz[to_reg_ix(s[1])] = if self.read(s[1]) == self.read(s[2]) {
                    1
                } else {
                    0
                }
            }
            _ => panic!("wat is {}", input),
        }
        AluState::Going
    }
}

fn render_z(mut z: isize) -> String {
    let mut s = String::new();
    while z > 0 {
        s += &format!("{} ", z % 26);
        // s.push(('a' as u32 + (z % 26) as u32) as u8 as char);
        z /= 26;
    }
    s
}
fn p1(input: &str) -> usize {
    let s = Alu {
        wxyz: [0; 4],
        line: 0,
        next_input: None,
        state: AluState::Going,
    };
    let code = input.lines().collect_vec();
    let mut a = s.clone();
    //               1 2 3 5 7 8 9
    //let s = [6, 9, 9, 4, 9, 9, 9];
    let s = [1, 4, 9, 1, 7, 5, 3];
    let full_seq = [
        s[0],
        s[1],
        s[2],
        s[2] - 8,
        s[3],
        s[3] + 5,
        s[4],
        s[5],
        s[6],
        s[6] - 2,
        s[5] - 4,
        s[4] - 6,
        s[1] - 3,
        s[0] + 3,
    ];
    for (ix, i) in full_seq.iter().enumerate() {
        if *i >= 1 && *i <= 9 {
            a.next_input = Some(*i);
            a.run_to_input(&code);
            println!("After {}: {}", i, render_z(a.read("z")));
        } else {
            panic!("Out of range at ix {}", ix);
        }
    }
    dbg!(&full_seq);

    let x = dijkstra(
        &(0, 0),
        |(z, line)| {
            let mut v: Vec<((isize, usize), usize)> = (1..=9)
                .rev()
                .filter_map(|i| {
                    // println!("{} {} {}", i, z, render_z(z));
                    let mut a = Alu {
                        wxyz: [0, 0, 0, *z],
                        line: *line,
                        next_input: Some(i),
                        state: AluState::Going,
                    };
                    a.state = a.run_to_input(&code);
                    match a.state {
                        AluState::NeedsInput | AluState::Done => Some(((a.read("z"), a.line), 1)),
                        AluState::Crashed => None,
                        AluState::Going => unreachable!(),
                    }
                })
                .collect_vec();
            //prefer ones where z is smaller.
            v.sort_by_key(|x| x.0 .0);
            v
        },
        |(z, line)| {
            if *line == code.len() {
                // dbg!(choices, z, line);
                // println!("z: {}", z);
                *z == 0
            } else {
                false
            }
        },
    );
    dbg!(x);
    0
}
