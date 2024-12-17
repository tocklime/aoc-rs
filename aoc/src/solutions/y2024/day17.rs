use std::collections::{BTreeSet, HashSet};

use itertools::Itertools;
use utils::inputs;

aoc_harness::aoc_main!(2024 day 17, part1 [p1], part2 [p2],
    example part1 EG => "4,6,3,5,6,3,5,2,1,0"
//   example part2 EG2 => 117_440
);

#[derive(Debug, Clone)]
struct Machine {
    regs: [i64; 3],
    ip: usize,
    program: Vec<u8>,
    output: Vec<u8>,
    force_quine: bool,
}

impl Machine {
    fn from_str(s: &str) -> Self {
        let (regs, prog) = s.trim().split_once("\n\n").unwrap();
        let mut r = inputs::find_things(regs, nom::character::complete::i64);
        let stripped = prog.strip_prefix("Program: ").unwrap();
        // dbg!(regs,prog,stripped);
        let program =
            inputs::input_from_str_sep_by(stripped, ",", |x| x.parse().expect("read prog int"));
        Self {
            regs: [r.next().unwrap(), r.next().unwrap(), r.next().unwrap()],
            ip: 0,
            program,
            output: Vec::new(),
            force_quine: false,
        }
    }
    fn read_lit(&self) -> Option<i64> {
        self.program.get(self.ip + 1).map(|&x| x.into())
    }
    fn read_combo(&self) -> Option<i64> {
        let n = self.read_lit()?;
        match n {
            0..4 => Some(n),
            4..7 => Some(self.regs[(n - 4) as usize]),
            _ => None,
        }
    }
    fn step(&mut self) -> Option<()> {
        let op = self.program.get(self.ip)?;
        let mut did_jump = false;
        match *op {
            0 /* adz */ => { //A/2^combo
                let a = self.regs[0];
                let b = 2i64.pow(self.read_combo()?.try_into().unwrap());
                self.regs[0] = a/b;
            }
            1 /* bxl */=> {
                self.regs[1] ^= self.read_lit()?;
            }
            2 => {
                self.regs[1] = self.read_combo()? % 8;
            }
            3 => {
                if self.regs[0] != 0 {
                    self.ip = self.read_lit()? as usize;
                    did_jump = true;
                }
            }
            4 => {
                self.regs[1] ^= self.regs[2];
            }
            5 => {
                let n = self.read_combo()?.rem_euclid(8) as u8;
                if self.force_quine {
                    match self.program.get(self.output.len()) {
                        None => return None, //output too long.
                        Some(&x) if x != n => return None, //output value wrong.
                        _ => (),
                    }
                }
                self.output.push(n);
            }
            6 => {
                let a = self.regs[0];
                let b = 2i64.pow(self.read_combo()?.try_into().unwrap());
                self.regs[1] = a/b;
            }
            7 =>
            {
                let a = self.regs[0];
                let b = 2i64.pow(self.read_combo()?.try_into().unwrap());
                self.regs[2] = a/b;
            }
            _ => unreachable!()
        }
        if !did_jump {
            self.ip += 2;
        }
        Some(())
    }
    fn run(&mut self) {
        while self.step().is_some() {}
    }
    fn quine_score(&self, init_a: i64) -> usize {
        let mut m = self.clone();
        // m.force_quine = true;
        m.regs[0] = init_a;
        m.run();
        let matching = m
            .output
            .iter()
            .zip(m.program.iter())
            .take_while(|(a, b)| a == b)
            .count();
        // println!(
        //     "   Running with {init_a:o} gets {:?}, which matches first {matching} of program {:?}",
        //     &m.output, &m.program
        // );
        matching
    }
}

fn p1(input: &str) -> String {
    let mut m = Machine::from_str(input);
    m.run();
    m.output
        .iter()
        .map(std::string::ToString::to_string)
        .join(",")
}
fn p2(input: &str) -> i64 {
    let m = Machine::from_str(input);
    // let mut sets_of_3_bits: Vec<i64> = vec![];
    // let mut best_score = 0;
    // let mut loops_since_score_increase = 0;
    let mut fringe: HashSet<i64> = [0].into_iter().collect();
    let mut correct : BTreeSet<i64> = BTreeSet::new();
    let mr = &m;
    dbg!(m.program.len());
    for step_ix in 0..m.program.len()-1 {
        let new_fringe : HashSet<i64> = fringe.iter().flat_map(move |s| {
            (0..0o1000).filter_map(move |x| {
                let candidate = s | x << (3 * step_ix);
                let res = mr.quine_score(candidate);
                // println!("Trying {candidate:o}, got score {res}");
                if res == mr.program.len() {
                    // correct.insert(candidate);
                    println!("Candidate: {candidate}");
                    None
                } else if res >= step_ix {
                    Some(candidate)
                } else {
                    None
                }
            })
        }).collect();
        println!("Step {} has {} options", step_ix, new_fringe.len());
        if new_fringe.is_empty() {
            println!("No possible way to get to step {step_ix}.");
            for a in &fringe {
                println!("  Opt: {a:o} => {step_ix}");
            }
        }
        fringe = new_fringe;
    }
    dbg!(fringe.len());
    correct.pop_first().unwrap()
    // fringe.into_iter().next().unwrap()
    //38886110969332 is an answer?

    // while let Some((s, n)) = stack.pop() {
    //     //we have a value which outputs the first n outputs correctly. What are the next bits?
    //     //search to find the next output value.
    //     if n == m.program.len() {
    //         return s;
    //     }
    //     stack.extend(next);
    // }
    // 0

    // loop {
    //     let (score, _, n) = (0..0o1000).map(|x| {
    //         let n = sets_of_3_bits.iter().chain(&[x]).rev().copied().reduce(|a,b| a<<3|b).unwrap();
    //         let s = m.quine_score(n);
    //         println!("  trying to add bits of {x} to {:?}, n is now {n} which is {n:o} in octal. Score is {s}.", sets_of_3_bits);
    //         (s,8-x, x)
    //     }).max().unwrap();
    //     if score > best_score {
    //         loops_since_score_increase = 0;
    //         best_score = score;
    //     } else {
    //         loops_since_score_increase += 1;
    //         assert!(loops_since_score_increase < 3);
    //     }
    //     if score == m.program.len() {
    //         println!("Found perfect x {n}");
    //         sets_of_3_bits.push(n&0b111);
    //         sets_of_3_bits.push((n>>3)&0b111);
    //         println!("{sets_of_3_bits:?}");
    //         return sets_of_3_bits.into_iter().rev().reduce(|a,b| a<<3|b).unwrap();
    //     }else {
    //         println!("Found good x {n} ({n:o}), {}", n&0b111);
    //         sets_of_3_bits.push(n&0b111);
    //         println!("known bits now: {sets_of_3_bits:?}");
    //     }
    //     // panic!()
    // }
}

const EG: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

const EG2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";
