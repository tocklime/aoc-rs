use itertools::Itertools;
use utils::inputs;

aoc_harness::aoc_main!(2024 day 17, 
    generator Machine::from_str, 
    part1 [p1] => "2,7,4,7,2,1,7,5,1", 
    part2 [p2_bfs, p2_dfs,p2_dfs_2] => 37_221_274_271_220,
    example part1 EG => "4,6,3,5,6,3,5,2,1,0",
    example part2 EG2 => 117_440
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
        if m.output.len() > m.program.len() {
            return 0;
        }
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

fn p1(input: &Machine) -> String {
    let mut m = input.clone();
    m.run();
    m.output
        .iter()
        .map(std::string::ToString::to_string)
        .join(",")
}
fn p2_bfs(m: &Machine) -> i64 {
    let lookahead = if m.program.len() == 6 { 0 } else { 4 };
    let mut fringe: Vec<i64> = [0].into_iter().collect();
    for step_ix in 0..m.program.len() {
        fringe = fringe.into_iter().flat_map(move |s| {
            (0..8).filter_map(move |x| {
                //considering adding `x` as new octal digit on the front of the value.
                //it's ok if by also adding any possible value between 0 and 2^lookahead 
                //gives a new correct digit.
                let with_la = (0..2_i64.pow(lookahead)).any(|la| {
                    let f = la << 3 | x;
                    let candidate = s | f << (3*step_ix);
                    m.quine_score(candidate) >= step_ix
                });
                (with_la).then_some(s | x << (3*step_ix))
            })
        }).collect();
        // println!("Fringe at step {step_ix} now has {}", fringe.len());
    }
    fringe.into_iter().min().unwrap()
}
fn p2_dfs(m: &Machine) -> i64 {
    let lookahead = if m.program.len() == 6 { 0 } else { 4 };
    let mut stack: Vec<(i64, usize)> = [(0, 0)].into_iter().collect();
    while let Some((s, step_ix)) = stack.pop() {
        if step_ix == m.program.len() {
            return s;
        }
        //do in reverse, so that lower numbers are TOS, so we'll find the smallest solution first.
        let next = (0..8).rev().filter_map(move |x| {
            //considering adding `x` as new octal digit on the front of the value.
            //it's ok if by also adding any possible value between 0 and 2^lookahead
            //gives a new correct digit.
            let with_la = (0..2_i64.pow(lookahead)).any(|la| {
                let f = la << 3 | x;
                let candidate = s | f << (3 * step_ix);
                m.quine_score(candidate) >= step_ix
            });
            (with_la).then_some((s | x << (3 * step_ix), step_ix + 1))
        });
        stack.extend(next);
    }
    0
}
fn p2_dfs_2(m: &Machine) -> i64 {
    let lookahead = if m.program.len() == 6 { 0 } else { 4 };
    let mut stack: Vec<(i64, i64, usize)> = [(0, 0, 0)].into_iter().collect();
    while let Some((s, next_digit_to_try, step_ix)) = stack.pop() {
        if step_ix == m.program.len() {
            return s;
        }
        if next_digit_to_try < 7 {
            stack.push((s,next_digit_to_try+1,step_ix));
        }
        //considering adding `next_digit_to_try` as new octal digit on the front of the value.
        //it's ok if by also adding any possible value between 0 and 2^lookahead
        //gives a new correct digit.
        let with_la = (0..2_i64.pow(lookahead)).any(|la| {
            let f = la << 3 | next_digit_to_try;
            let candidate = s | f << (3 * step_ix);
            m.quine_score(candidate) >= step_ix
        });
        if with_la {
            stack.push((s|next_digit_to_try << (3*step_ix),0,step_ix+1));
        }
    }
    0
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
