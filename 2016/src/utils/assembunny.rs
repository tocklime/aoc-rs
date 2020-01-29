#![allow(clippy::redundant_pattern_matching)]
use reformation::Reformation;
use std::convert::TryInto;

#[derive(Reformation, Eq, PartialEq, Copy, Clone,Debug)]
pub enum Value {
    #[reformation("{}")]
    Lit(i64),
    #[reformation("{}")]
    Reg(char),
}

#[derive(Reformation, Eq, PartialEq, Copy, Clone,Debug)]
pub enum Instr {
    #[reformation("cpy {} {}")]
    Cpy(Value, Value),
    #[reformation("inc {}")]
    Inc(Value),
    #[reformation("dec {}")]
    Dec(Value),
    #[reformation("jnz {} {}")]
    Jnz(Value, Value),
    #[reformation("tgl {}")]
    Tgl(Value)
}

#[derive(Debug)]
pub struct Computer {
    registers: [i64; 4],
    pub(crate) instruction_pointer: usize,
    memory: Vec<Instr>,
}

impl Computer {
    pub fn parse(input: &str) -> Self {
        Self {
            registers: [0, 0, 0, 0],
            instruction_pointer: 0,
            memory: input.lines().map(|x| Instr::parse(x).unwrap()).collect(),
        }
    }
    pub fn set_reg(&mut self, c: char, i: i64) {
        self.registers[((c as u8) - b'a') as usize] = i;
    }
    pub fn get_reg(&self, c: char) -> i64 {
        self.registers[((c as u8) - b'a') as usize]
    }
    pub fn get(&self, v: Value) -> i64 {
        match v {
            Value::Lit(a) => a,
            Value::Reg(r) => self.get_reg(r),
        }
    }
    pub fn set_i(&mut self, r: Value, i: i64)  {
        self.set(r,Value::Lit(i));
    }
    pub fn set(&mut self, r: Value, i: Value)  {
        if let Value::Reg(c) = r {
            self.set_reg(c,self.get(i));
        }
    }
    pub fn step(&mut self) {
        let instr_as_i64 : i64 = self.instruction_pointer.try_into().unwrap();
        let mut next_pc = instr_as_i64 + 1;
        match self.memory[self.instruction_pointer] {
            Instr::Cpy(v, r) => self.set(r,v),
            Instr::Inc(r) => self.set_i(r,self.get(r)+1),
            Instr::Dec(r) => self.set_i(r,self.get(r)-1),
            Instr::Jnz(v,x) => if self.get(v) != 0 {next_pc += self.get(x) - 1},
            Instr::Tgl(v) => {
                let i = self.get(v);
                let ix : usize = (i + instr_as_i64).try_into().unwrap();
                if let Some(curr) = self.memory.get_mut(ix) {
                    *curr = match curr {
                        Instr::Jnz(a,b) => Instr::Cpy(*a,*b),
                        Instr::Cpy(a,b) => Instr::Jnz(*a,*b),
                        Instr::Inc(a) => Instr::Dec(*a),
                        Instr::Dec(a) => Instr::Inc(*a),
                        Instr::Tgl(a) => Instr::Inc(*a),
                    };
                }
            }
        }
        self.instruction_pointer = next_pc.try_into().unwrap();
    }
    pub fn running(&self) -> bool {
        self.instruction_pointer < self.memory.len()
    }
    pub fn run(&mut self) {
        while self.running() {
            self.step();
        }
    }
}
