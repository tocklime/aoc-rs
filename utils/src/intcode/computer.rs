use super::compmem::CompMem;
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};

use super::enums::*;
use super::oparg::Arg;
use super::opcode::OpCode;

#[derive(Debug)]
pub struct Computer<MemType = i32> {
    name: String,
    initial_mem: Vec<MemType>,
    memory: HashMap<isize, MemType>,
    instruction_pointer: isize,
    state: ComputerState,
    fixed_input: Vec<MemType>,
    input_chan: Option<Receiver<MemType>>,
    input_arc: Option<Arc<Mutex<MemType>>>,
    output: Vec<MemType>,
    output_chan: Option<Sender<MemType>>,
    relative_base: isize,
    default_input: Option<MemType>,
    ips_since_last_mem_edit: HashSet<isize>,
}

impl<MT> Clone for Computer<MT>
where
    MT: CompMem,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            initial_mem: self.initial_mem.clone(),
            memory: self.memory.clone(),
            instruction_pointer: self.instruction_pointer,
            state: self.state,
            relative_base: self.relative_base,
            fixed_input: self.fixed_input.clone(),
            input_chan: None,
            input_arc: None,
            output: self.output.clone(),
            output_chan: None,
            default_input: self.default_input,
            ips_since_last_mem_edit: self.ips_since_last_mem_edit.clone(),
        }
    }
}

impl<MemType> FromStr for Computer<MemType>
where
    MemType: FromStr + CompMem,
{
    type Err = <MemType as std::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is: Result<Vec<_>, _> = s.trim().split(',').map(str::parse).collect();
        Ok(Self::new(&is?))
    }
}
impl<MemType> Computer<MemType>
where
    MemType: CompMem,
{
    pub fn new(initial_mem: &[MemType]) -> Self {
        let mut c = Self {
            initial_mem: Vec::from(initial_mem),
            name: String::from("COMP"),
            memory: HashMap::new(),
            instruction_pointer: 0,
            state: ComputerState::Running,
            fixed_input: vec![],
            input_chan: None,
            input_arc: None,
            output: vec![],
            output_chan: None,
            relative_base: 0,
            default_input: None,
            ips_since_last_mem_edit: HashSet::new(),
        };
        c.reset();
        c
    }
    pub fn with_name(&mut self, n: String) -> &mut Self {
        self.name = n;
        self
    }
    pub fn disassembly(&self) -> String {
        let mut ip = 0;
        let mut output = String::new();
        let max_mem: usize = cmp::max(
            self.initial_mem.len(),
            self.memory
                .keys()
                .max()
                .copied()
                .and_then(|x| x.try_into().ok())
                .unwrap_or(0_usize),
        );
        while ip < max_mem {
            let a = self.get_args(ip);
            match Op::try_from_mem_slice(&a) {
                Some(o) => {
                    output.push_str(&format!("{: >4}: {}\n", ip, o));
                    ip += 1 + o.op.arg_count();
                }
                None => {
                    for i in &a {
                        output.push_str(&format!("{: >4}: {}\n", ip, i));
                        ip += 1;
                    }
                }
            }
        }
        output
    }

    pub fn get_args(&self, ip: usize) -> [MemType; 4] {
        let mut ans: [MemType; 4] = Default::default();
        for (i, a) in ans.iter_mut().enumerate() {
            *a = self.abs_load((ip + i).try_into().unwrap());
        }
        ans
    }
    pub fn get_last_output(&self) -> MemType {
        *self.get_output().last().unwrap()
    }
    pub fn get_output(&self) -> &[MemType] {
        &self.output
    }
    pub fn clear_output(&mut self) -> &mut Self {
        self.output.clear();
        self
    }
    pub fn take_output(&mut self) -> Vec<MemType> {
        std::mem::take(&mut self.output)
    }
    pub fn give_input(&mut self, input: Vec<MemType>) -> &mut Self {
        self.fixed_input = input;
        self
    }
    pub fn with_string_input(&mut self, input: &str) -> &mut Self {
        self.give_input(input.bytes().map(|x| x.into()).collect());
        self
    }
    pub fn output_as_string(&self) -> String {
        self.output.iter().map(|&x| x.as_char()).collect()
    }
    pub fn with_input(&mut self, x: MemType) -> &mut Self {
        self.fixed_input.push(x);
        self
    }
    pub fn with_default_input(&mut self, x: MemType) -> &mut Self {
        self.default_input = Some(x);
        self
    }
    pub fn connect_output_from(
        &mut self,
        other: &mut Self,
        initial_input: &[MemType],
    ) -> &mut Self {
        let tx = self.make_input_chan();
        for &v in initial_input {
            tx.send(v).expect("Failed to send initial value");
        }
        other.with_chan_output(tx);
        self
    }
    pub fn with_chan_input(&mut self, x: Receiver<MemType>) -> &mut Self {
        self.input_chan = Some(x);
        self
    }
    pub fn with_chan_output(&mut self, x: Sender<MemType>) -> &mut Self {
        self.output_chan = Some(x);
        self
    }
    pub fn make_input_arc(&mut self) -> Arc<Mutex<MemType>> {
        let x = Arc::new(Mutex::new(Default::default()));
        self.input_arc = Some(x.clone());
        x
    }
    pub fn make_input_chan(&mut self) -> Sender<MemType> {
        let (tx, rx) = mpsc::channel();
        self.with_chan_input(rx);
        tx
    }
    pub fn make_output_chan(&mut self) -> Receiver<MemType> {
        let (tx, rx) = mpsc::channel();
        self.with_chan_output(tx);
        rx
    }
    pub fn make_io_chans(&mut self) -> (Sender<MemType>, Receiver<MemType>) {
        (self.make_input_chan(), self.make_output_chan())
    }
    pub fn reset(&mut self) -> &mut Self {
        self.memory = HashMap::new();
        self.instruction_pointer = 0;
        self.state = ComputerState::Running;
        self.relative_base = 0;
        self.fixed_input = vec![];
        self.ips_since_last_mem_edit.clear();
        self
    }
    pub fn current_op_with_args(&self) -> Op<MemType> {
        let ms = self.get_args(self.instruction_pointer.try_into().unwrap());
        Op::from_mem_slice(&ms)
    }
    pub fn abs_load(&self, pos: isize) -> MemType {
        self.memory.get(&pos).copied().unwrap_or_else(|| {
            pos.try_into()
                .ok()
                .and_then(|p: usize| self.initial_mem.get(p))
                .copied()
                .unwrap_or_default()
        })
    }
    pub fn rel_load(&self, offset: isize) -> MemType {
        self.abs_load(self.relative_base + offset)
    }
    pub fn rel_offset(&self, offset: isize) -> isize {
        self.relative_base + offset
    }
    pub fn load(&self, offset: isize) -> MemType {
        self.abs_load(self.instruction_pointer + offset)
    }
    pub fn store(&mut self, offset: isize, value: MemType) {
        self.abs_store(self.instruction_pointer + offset, value);
    }
    pub fn abs_store(&mut self, offset: isize, value: MemType) {
        if self.memory.get(&offset) == Some(&value) {
            return;
        }
        self.ips_since_last_mem_edit.clear();
        *self.memory.entry(offset).or_insert_with(Default::default) = value;
    }
    pub fn inc_ip(&mut self, offset: isize) {
        self.instruction_pointer += offset;
    }
    pub fn run(&mut self) -> &Self {
        loop {
            match self.step().state {
                ComputerState::Halted => {
                    return self;
                }
                ComputerState::Running => (),
            }
        }
    }
    pub fn run_to_input(&mut self) -> bool {
        self.step();
        loop {
            let op = self.current_op_with_args();
            if self.state == ComputerState::Halted {
                return false;
            }
            if op.op == OpCode::Input && self.fixed_input.is_empty() {
                return true;
            }
            op.execute(self);
        }
    }
    pub fn is_halted(&self) -> bool {
        self.state() == ComputerState::Halted
    }
    pub fn step(&mut self) -> &mut Self {
        self.current_op_with_args().execute(self);
        self
    }
    pub fn seems_to_be_looping(&self) -> bool {
        self.ips_since_last_mem_edit
            .contains(&self.instruction_pointer)
    }
    pub fn state(&self) -> ComputerState {
        self.state
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Op<MemType> {
    op: OpCode,
    args: [Arg<MemType>; 3],
}

impl<MemType> Op<MemType>
where
    MemType: CompMem,
{
    pub fn try_from_mem_slice(m: &[MemType; 4]) -> Option<Self> {
        let as_int = m[0].try_into().ok()?;
        let ps = as_int / 100;
        let op1 = ps % 10;
        let op2 = (ps / 10) % 10;
        let op3 = (ps / 100) % 10;
        Some(Self {
            op: OpCode::try_from(as_int % 100).ok()?,
            args: [
                Arg::new(m[1], ParameterMode::try_from(op1).ok()?),
                Arg::new(m[2], ParameterMode::try_from(op2).ok()?),
                Arg::new(m[3], ParameterMode::try_from(op3).ok()?),
            ],
        })
    }
    pub fn from_mem_slice(m: &[MemType; 4]) -> Self {
        Self::try_from_mem_slice(m).unwrap()
    }
    pub fn execute(&self, c: &mut Computer<MemType>) {
        c.ips_since_last_mem_edit.insert(c.instruction_pointer);
        let op_count = self.op.arg_count();
        let ps = self.args;
        let mut do_ip_inc = true;
        match self.op {
            OpCode::Add => c.abs_store(ps[2].ptr(c), ps[0].get(c) + ps[1].get(c)),
            OpCode::Mult => c.abs_store(ps[2].ptr(c), ps[0].get(c) * ps[1].get(c)),
            OpCode::LessThan => c.abs_store(ps[2].ptr(c), (ps[0].get(c) < ps[1].get(c)).into()),
            OpCode::Equals => c.abs_store(ps[2].ptr(c), (ps[0].get(c) == ps[1].get(c)).into()),
            OpCode::Input => {
                let i = if !c.fixed_input.is_empty() {
                    c.fixed_input.remove(0)
                } else if let Some(r) = &c.input_chan {
                    match c.default_input {
                        Some(d) => r.try_recv().unwrap_or(d),
                        None => r.recv().expect("No value on receiver"),
                    }
                } else if let Some(a) = &c.input_arc {
                    *a.lock().unwrap()
                } else {
                    panic!("No input")
                };
                c.abs_store(ps[0].ptr(c), i);
            }
            OpCode::Output => {
                let o = ps[0].get(c);
                c.output.push(o);
                if let Some(ch) = &c.output_chan {
                    ch.send(o).expect("Could not send");
                }
            }
            OpCode::JumpIfTrue | OpCode::JumpIfFalse => {
                if (ps[0].get(c) != Default::default()) == (self.op == OpCode::JumpIfTrue) {
                    c.instruction_pointer = ps[1].get(c).as_isize();
                    do_ip_inc = false;
                }
            }
            OpCode::MoveRelativeBase => {
                c.relative_base += ps[0].get(c).as_isize();
                c.ips_since_last_mem_edit.clear();
            }
            OpCode::Halt => {
                c.state = ComputerState::Halted;
                do_ip_inc = false;
            }
        }
        if do_ip_inc {
            c.inc_ip((1 + op_count).try_into().unwrap());
        }
    }
}
impl<MemType> fmt::Display for Op<MemType>
where
    MemType: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{: <20}", self.op)?;
        for i in 0..self.op.arg_count() {
            write!(f, "{: <5} ", self.args[i])?;
        }
        fmt::Result::Ok(())
    }
}
