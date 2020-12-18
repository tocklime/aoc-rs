#[derive(Debug,PartialEq,Eq)]
pub enum Op {
    Add,
    Mul,
    None
}
#[derive(Debug,PartialEq,Eq)]
struct StackFrame {
    val : i64,
    op: Op,
    hard_brackets: bool
}
impl StackFrame {
    pub const fn new(val:i64,is_hard:bool) -> Self {
        Self {val,op: Op::None, hard_brackets:is_hard}
    }
    pub fn collapse_to(self, other : &mut Self) {
        assert_eq!(self.op,Op::None);
        other.apply(self.val);
    }
    pub fn apply(&mut self, val: i64) {
        match self.op {
            Op::None => {
                if self.val == 0 {
                    self.val = val;
                } else {
                    panic!("Can't collapse onto frame with no pending operation");
                }
            }
            Op::Add => { self.val += val; }
            Op::Mul => { self.val *= val; }
        }
        self.op = Op::None;
    }
    pub fn pending_multiply(&mut self) {
        assert_eq!(self.op,Op::None);
        self.op = Op::Mul;
    }
    pub fn pending_addition(&mut self) {
        assert_eq!(self.op,Op::None);
        self.op = Op::Add;
    }
}
#[derive(PartialEq,Eq,Clone, Copy)]
pub enum Mode {Even, AddsFirst}
pub fn solve(input: &str, mode: Mode) -> i64 {
    input
        .lines()
        .map(|l| {
            let mut stack :Vec<StackFrame> = vec![StackFrame::new(0,true)];
            for c in l.chars() {
                match c {
                    ' ' => {}
                    '+' => {
                        stack.last_mut().unwrap().pending_addition();
                    }
                    '*' => {
                        stack.last_mut().unwrap().pending_multiply();
                        if mode == Mode::AddsFirst {
                            stack.push(StackFrame::new(0,false));
                        }
                    }
                    '(' => {
                        stack.push(StackFrame::new(0,true));
                    }
                    ')' => {
                        let mut done = false;
                        while !done {
                            let p = stack.pop().unwrap();
                            done = p.hard_brackets;
                            p.collapse_to(stack.last_mut().unwrap());
                        }
                    }
                    _ => {
                        let d = c.to_string().parse::<i64>().unwrap();
                        stack.last_mut().unwrap().apply(d);
                    }
                }
            }
            while stack.len() > 1 {
                stack.pop().unwrap().collapse_to(stack.last_mut().unwrap());
            }
            stack[0].val
        })
        .sum()
}
#[aoc(day18, part2)]
pub fn p2(input: &str) -> i64 {
    solve(input, Mode::AddsFirst)
}
#[aoc(day18, part1)]
pub fn p1(input: &str) -> i64 {
    solve(input, Mode::Even)
}
