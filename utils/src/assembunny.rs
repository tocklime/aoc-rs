use reformation::Reformation;
use std::convert::TryInto;

#[derive(Reformation, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Value {
    #[reformation("{}")]
    Lit(i64),
    #[reformation("{}")]
    Reg(char),
}

#[derive(Reformation, Eq, PartialEq, Copy, Clone, Debug)]
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
    Tgl(Value),
    #[reformation("out {}")]
    Out(Value),
}

#[derive(Debug, Clone)]
pub struct Computer {
    registers: [i64; 4],
    instruction_pointer: usize,
    memory: Vec<Instr>,
    pub(crate) output: Vec<i64>,
}

impl Computer {
    #[must_use]
    pub fn parse(input: &str) -> Self {
        Self {
            registers: [0, 0, 0, 0],
            instruction_pointer: 0,
            memory: input.lines().map(|x| Instr::parse(x).unwrap()).collect(),
            output: Vec::new(),
        }
    }
    pub fn set_reg(&mut self, c: char, i: i64) {
        self.registers[((c as u8) - b'a') as usize] = i;
    }
    #[must_use]
    pub fn get_reg(&self, c: char) -> i64 {
        self.registers[((c as u8) - b'a') as usize]
    }
    #[must_use]
    pub fn get(&self, v: Value) -> i64 {
        match v {
            Value::Lit(a) => a,
            Value::Reg(r) => self.get_reg(r),
        }
    }
    #[must_use]
    pub fn get_instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }
    pub fn set_instruction_pointer(&mut self, ip: usize) {
        self.instruction_pointer = ip;
    }
    #[must_use]
    pub fn output_is_empty(&self) -> bool {
        self.output.is_empty()
    }
    pub fn take_output(&mut self) -> Vec<i64> {
        std::mem::take(&mut self.output)
    }
    #[must_use]
    pub fn first_output(&self) -> Option<i64> {
        self.output.get(0).copied()
    }
    pub fn clear_output(&mut self) {
        self.output.clear();
    }
    pub fn set_i(&mut self, r: Value, i: i64) {
        self.set(r, Value::Lit(i));
    }
    pub fn set(&mut self, r: Value, i: Value) {
        if let Value::Reg(c) = r {
            self.set_reg(c, self.get(i));
        }
    }
    pub fn step(&mut self) {
        let instr_as_i64: i64 = self.instruction_pointer.try_into().unwrap();
        let mut next_pc = instr_as_i64 + 1;
        match self.memory[self.instruction_pointer] {
            Instr::Cpy(v, r) => self.set(r, v),
            Instr::Inc(r) => self.set_i(r, self.get(r) + 1),
            Instr::Dec(r) => self.set_i(r, self.get(r) - 1),
            Instr::Jnz(v, x) => {
                if self.get(v) != 0 {
                    next_pc += self.get(x) - 1
                }
            }
            Instr::Tgl(v) => {
                let i = self.get(v);
                let ix: usize = (i + instr_as_i64).try_into().unwrap();
                if let Some(curr) = self.memory.get_mut(ix) {
                    *curr = match curr {
                        Instr::Jnz(a, b) => Instr::Cpy(*a, *b),
                        Instr::Cpy(a, b) => Instr::Jnz(*a, *b),
                        Instr::Inc(a) => Instr::Dec(*a),
                        Instr::Dec(a) => Instr::Inc(*a),
                        Instr::Tgl(a) => Instr::Inc(*a),
                        Instr::Out(a) => Instr::Inc(*a),
                    };
                }
            }
            Instr::Out(x) => self.output.push(self.get(x)),
        }
        self.instruction_pointer = next_pc.try_into().unwrap();
    }
    #[must_use]
    pub fn running(&self) -> bool {
        self.instruction_pointer < self.memory.len()
    }
    pub fn run(&mut self) {
        while self.running() {
            self.step();
        }
    }
    pub fn run_to_output(&mut self) -> Option<i64> {
        while self.running() && self.output.is_empty() {
            self.step();
        }
        let out = self.output.get(0).cloned();
        self.output.clear();
        out
    }
}
