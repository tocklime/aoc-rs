#![allow(clippy::redundant_pattern_matching)]
use reformation::Reformation;
use std::convert::TryInto;

#[derive(Reformation, Eq, PartialEq, Copy, Clone,Debug)]
enum Value {
    #[reformation("{}")]
    Lit(i64),
    #[reformation("{}")]
    Reg(char),
}

#[derive(Reformation, Eq, PartialEq, Copy, Clone,Debug)]
enum Instr {
    #[reformation("cpy {} {}")]
    Cpy(Value, char),
    #[reformation("inc {}")]
    Inc(char),
    #[reformation("dec {}")]
    Dec(char),
    #[reformation("jnz {} {}")]
    Jnz(Value, i64),
}

#[derive(Debug)]
struct Computer {
    registers: [i64; 4],
    instruction_pointer: usize,
    memory: Vec<Instr>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        Self {
            registers: [0, 0, 0, 0],
            instruction_pointer: 0,
            memory: input.lines().map(|x| Instr::parse(x).unwrap()).collect(),
        }
    }
    fn set_reg(&mut self, c: char, i: i64) {
        self.registers[((c as u8) - b'a') as usize] = i;
    }
    fn get_reg(&self, c: char) -> i64 {
        self.registers[((c as u8) - b'a') as usize]
    }
    fn step(&mut self) {
        let mut next_pc:i64 = (self.instruction_pointer + 1).try_into().unwrap();
        match self.memory[self.instruction_pointer] {
            Instr::Cpy(Value::Lit(a), r) => self.set_reg(r,a),
            Instr::Cpy(Value::Reg(a), r) => self.set_reg(r,self.get_reg(a)),
            Instr::Inc(r) => self.set_reg(r,1+self.get_reg(r)),
            Instr::Dec(r) => self.set_reg(r,self.get_reg(r)-1),
            Instr::Jnz(Value::Lit(a), x) => if a != 0 {next_pc += x - 1},
            Instr::Jnz(Value::Reg(a), x) => if self.get_reg(a) != 0 {next_pc += x - 1},
        }
        self.instruction_pointer = next_pc.try_into().unwrap();
    }
    fn run(&mut self) {
        while self.instruction_pointer < self.memory.len() {
            self.step();
        }
    }
}

#[aoc(day12, part1)]
#[post(ret == 318_077)]
fn p1(input: &str) -> i64 {
    let mut c = Computer::parse(input);
    c.run();
    c.get_reg('a')
}

#[aoc(day12, part2)]
#[post(ret == 9_227_731)]
fn p2(input: &str) -> i64 {
    let mut c = Computer::parse(input);
    c.set_reg('c',1);
    c.run();
    c.get_reg('a')
}
