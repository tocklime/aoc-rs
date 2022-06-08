use reformation::Reformation;
use std::collections::HashMap;
use itertools::Itertools;
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
    #[reformation("set {} {}")]
    Set(char, Val),
    #[reformation("sub {} {}")]
    Sub(char, Val),
    #[reformation("mul {} {}")]
    Mul(char, Val),
    #[reformation("jnz {} {}")]
    Jnz(Val, Val),
}

struct Duet<'a> {
    mem: &'a [Op],
    ip: i64,
    regs: HashMap<char, i64>,
    mul_count: usize,
}

#[derive(PartialEq, Eq)]
enum StepResult {
    Halt,
    Running,
}

impl<'a> Duet<'a> {
    fn new(mem: &'a [Op]) -> Self {
        Self {
            mem,
            ip: 0,
            regs: HashMap::new(),
            mul_count: 0,
        }
    }
    fn get(&self, c: char) -> i64 {
        *self.regs.get(&c).unwrap_or(&0)
    }
    fn lookup(&self, v: Val) -> i64 {
        match v {
            Val::Literal(x) => x,
            Val::Ref(c) => self.get(c)
        }
    }
    fn set(&mut self, c: char, i: i64) {
        self.regs.insert(c, i);
    }

    fn step_1(&mut self) -> StepResult {
        let mut next_ip = self.ip + 1;
        match self.get_op() {
            None => return StepResult::Halt,
            Some(Op::Set(x, y)) => self.set(x, self.lookup(y)),
            Some(Op::Sub(c, v)) => self.set(c, self.get(c) - self.lookup(v)),
            Some(Op::Mul(c, v)) => {
                self.set(c, self.get(c) * self.lookup(v));
                self.mul_count += 1;
            }
            Some(Op::Jnz(v, w)) => if self.lookup(v) != 0 {
                next_ip = self.ip + self.lookup(w);
            }
        }
        self.ip = next_ip;
        StepResult::Running
    }

    fn get_op(&self) -> Option<Op> {
        let as_u: Option<usize> = self.ip.try_into().ok();
        as_u.and_then(|i| self.mem.get(i).cloned())
    }
}


fn p1(input: &str) -> usize {
    let m = input.lines().map(|x| Op::parse(x).unwrap()).collect_vec();
    let mut c = Duet::new(&m);
    while c.step_1() != StepResult::Halt {}
    c.mul_count
}


fn p2(input: &str) -> usize {
    let m = input.lines().map(|x| Op::parse(x).unwrap()).collect_vec();
    let mut c = Duet::new(&m);
    c.set('a', 1);
    while c.ip < 8 {
        c.step_1();
    }
    let from = c.get('b').try_into().unwrap();
    let to = c.get('c').try_into().unwrap();
    let s = primal::Sieve::new(to);
    (from..=to).step_by(17).filter(|x| !s.is_prime(*x)).count()
}
