use std::{
    cmp::{max, min},
    fmt::Debug,
};

use aoc_harness::*;

aoc_main!(2021 day 24, both [both] => (69914999975369, 14911675311114));

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

#[allow(dead_code)]
fn render_z(mut z: isize) -> String {
    let mut s = String::new();
    while z > 0 {
        s += &format!("{} ", z % 26);
        // s.push(('a' as u32 + (z % 26) as u32) as u8 as char);
        z /= 26;
    }
    s
}
fn read_constraints(input: &str) -> Vec<(usize, usize, isize)> {
    let blocks = input.split("inp w");
    let mut stack = Vec::new();
    let mut ans = Vec::new();
    for (ix, b) in blocks.filter(|x| !x.is_empty()).enumerate() {
        let lines = b.lines().collect_vec();
        if lines.is_empty() {
            continue;
        }
        //is this a push or pop?
        if lines[4] == "div z 1" {
            //what difference do we push?
            let diff: isize = lines[15].split(' ').nth(2).unwrap().parse().unwrap();
            stack.push((ix, diff));
        } else {
            let diff2: isize = lines[5].split(' ').nth(2).unwrap().parse().unwrap();
            let (ix1, diff1) = stack.pop().unwrap();
            ans.push((ix, ix1, diff1 + diff2));
        }
    }
    ans
}
fn both(input: &str) -> (isize, isize) {
    let s = Alu {
        wxyz: [0; 4],
        line: 0,
        next_input: None,
        state: AluState::Going,
    };
    let code = input.lines().collect_vec();
    let constraints = read_constraints(input);
    let min_max = (0..14)
        .map(|ix| {
            for &(a, b, diff) in &constraints {
                if a == ix {
                    return (max(1, 1 + diff), min(9, 9 + diff));
                } else if b == ix {
                    return (max(1, 1 - diff), min(9, 9 - diff));
                }
            }
            panic!("unconstrained ix: {}", ix);
        })
        .collect_vec();
    {
        let mut min_alu = s.clone();
        let mut max_alu = s;
        for (min, max) in &min_max {
            min_alu.next_input = Some(*min);
            max_alu.next_input = Some(*max);
            min_alu.run_to_input(&code);
            max_alu.run_to_input(&code);
        }
        assert_eq!(min_alu.read("z"), 0);
        assert_eq!(max_alu.read("z"), 0);
    }
    min_max.iter().fold((0, 0), |(max, min), &v| {
        let min = min * 10 + v.0;
        let max = max * 10 + v.1;
        (max, min)
    })
}
