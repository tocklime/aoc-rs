use aoc_harness::aoc_main;

aoc_main!(2017 day 18, part1 [p1], part2 [p2]);
use itertools::Itertools;
use reformation::Reformation;
use std::collections::{HashMap, VecDeque};
use std::convert::TryInto;

#[derive(Debug, Reformation, Copy, Clone)]
enum Val {
    #[reformation("{}")]
    Literal(i64),
    #[reformation("{}")]
    Ref(char),
}

#[derive(Debug, Reformation, Copy, Clone)]
enum Op {
    #[reformation("snd {}")]
    Snd(char),
    #[reformation("set {} {}")]
    Set(char, Val),
    #[reformation("add {} {}")]
    Add(char, Val),
    #[reformation("mul {} {}")]
    Mul(char, Val),
    #[reformation("mod {} {}")]
    Mod(char, Val),
    #[reformation("rcv {}")]
    Rcv(char),
    #[reformation("jgz {} {}")]
    Jgz(Val, Val),
}

struct Duet<'a> {
    mem: &'a [Op],
    ip: i64,
    last_sound: Option<i64>,
    last_rcv: Option<i64>,
    input_queue: VecDeque<i64>,
    regs: HashMap<char, i64>,
}

#[derive(PartialEq, Eq)]
enum StepResult {
    Halt,
    Send(i64),
    NeedInput,
    Running,
}

impl<'a> Duet<'a> {
    fn new(mem: &'a [Op]) -> Self {
        Self {
            mem,
            ip: 0,
            last_sound: None,
            last_rcv: None,
            input_queue: VecDeque::new(),
            regs: HashMap::new(),
        }
    }
    fn get(&self, c: char) -> i64 {
        *self.regs.get(&c).unwrap_or(&0)
    }
    fn lookup(&self, v: Val) -> i64 {
        match v {
            Val::Literal(x) => x,
            Val::Ref(c) => self.get(c),
        }
    }
    fn set(&mut self, c: char, i: i64) {
        self.regs.insert(c, i);
    }

    fn step_1(&mut self) -> StepResult {
        let mut next_ip = self.ip + 1;
        match self.get_op() {
            None => return StepResult::Halt,
            Some(Op::Snd(c)) => self.last_sound = Some(self.get(c)),
            Some(Op::Set(x, y)) => self.set(x, self.lookup(y)),
            Some(Op::Add(c, v)) => self.set(c, self.get(c) + self.lookup(v)),
            Some(Op::Mul(c, v)) => self.set(c, self.get(c) * self.lookup(v)),
            Some(Op::Mod(c, v)) => self.set(c, self.get(c) % self.lookup(v)),
            Some(Op::Rcv(c)) => {
                if self.get(c) != 0 {
                    self.last_rcv = self.last_sound;
                }
            }
            Some(Op::Jgz(v, w)) => {
                if self.lookup(v) > 0 {
                    next_ip = self.ip + self.lookup(w);
                }
            }
        }
        self.ip = next_ip;
        StepResult::Running
    }

    fn get_op(&self) -> Option<Op> {
        let as_u: Option<usize> = self.ip.try_into().ok();
        as_u.and_then(|i| self.mem.get(i).cloned())
    }
    fn is_blocked(&self) -> bool {
        match self.get_op() {
            Some(Op::Rcv(_)) => self.input_queue.is_empty(),
            None => true,
            _ => false,
        }
    }

    fn step_2(&mut self) -> StepResult {
        let mut next_ip = self.ip + 1;
        let mut res = StepResult::Running;
        match self.get_op() {
            None => return StepResult::Halt,
            Some(Op::Snd(c)) => res = StepResult::Send(self.get(c)),
            Some(Op::Set(x, y)) => self.set(x, self.lookup(y)),
            Some(Op::Add(c, v)) => self.set(c, self.get(c) + self.lookup(v)),
            Some(Op::Mul(c, v)) => self.set(c, self.get(c) * self.lookup(v)),
            Some(Op::Mod(c, v)) => self.set(c, self.get(c) % self.lookup(v)),
            Some(Op::Rcv(c)) => match self.input_queue.pop_front() {
                None => {
                    res = StepResult::NeedInput;
                    next_ip = self.ip;
                }
                Some(x) => self.set(c, x),
            },
            Some(Op::Jgz(v, w)) => {
                if self.lookup(v) > 0 {
                    next_ip = self.ip + self.lookup(w);
                }
            }
        }
        self.ip = next_ip;
        res
    }
}

fn p1(input: &str) -> i64 {
    let m = input.lines().map(|x| Op::parse(x).unwrap()).collect_vec();
    let mut c = Duet::new(&m);
    loop {
        if c.step_1() == StepResult::Halt {
            break;
        }
        if let Some(x) = c.last_rcv {
            return x;
        }
    }
    0
}

fn p2(input: &str) -> i64 {
    let m = input.lines().map(|x| Op::parse(x).unwrap()).collect_vec();
    let mut a = Duet::new(&m);
    let mut b = Duet::new(&m);
    a.set('p', 0);
    b.set('p', 1);
    let mut b_send_count = 0;
    while !(a.is_blocked() && b.is_blocked()) {
        while !a.is_blocked() {
            //a
            if let StepResult::Send(x) = a.step_2() {
                b.input_queue.push_back(x)
            }
        }
        while !b.is_blocked() {
            if let StepResult::Send(x) = b.step_2() {
                b_send_count += 1;
                a.input_queue.push_back(x)
            }
        }
    }
    b_send_count
}
