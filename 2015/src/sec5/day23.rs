use nom::lib::std::collections::HashMap;
use reformation::Reformation;

#[derive(Reformation, Debug)]
enum Instr {
    #[reformation(r"hlf {}")]
    Hlf(char),
    #[reformation(r"tpl {}")]
    Tpl(char),
    #[reformation(r"inc {}")]
    Inc(char),
    #[reformation(r"jmp {}")]
    Jmp(i32),
    #[reformation(r"jie {}, {}")]
    Jie(char, i32),
    #[reformation(r"jio {}, {}")]
    Jio(char, i32),
}

struct Comp {
    regs: HashMap<char, usize>,
    ip: i32,
    program: HashMap<i32, Instr>,
}

impl Comp {
    fn parse(input: &str) -> Self {
        Comp {
            ip: 0,
            regs: [('a', 0), ('b', 0)].iter().cloned().collect(),
            program: (0..).zip(input.lines().map(|x| Instr::parse(x).unwrap())).collect(),
        }
    }
    fn is_running(&self) -> bool {
        self.program.contains_key(&self.ip)
    }
    fn step(&mut self) {
        let mut new_ip = self.ip + 1;
        match self.program[&self.ip] {
            Instr::Hlf(c) => { self.regs.entry(c).and_modify(|x| *x /= 2); }
            Instr::Tpl(c) => { self.regs.entry(c).and_modify(|x| *x *= 3); }
            Instr::Inc(c) => { self.regs.entry(c).and_modify(|x| *x += 1); }
            Instr::Jmp(i) => new_ip = self.ip + i,
            Instr::Jie(c, i) => if self.regs[&c] % 2 == 0 { new_ip = self.ip + i }
            Instr::Jio(c, i) => if self.regs[&c] == 1 { new_ip = self.ip + i }
        }
        self.ip = new_ip;
    }
    fn run_and_return_b(&mut self) -> usize {
        while self.is_running(){
            self.step();
        }
        self.regs[&'b']
    }
}


#[aoc(day23, part1)]
fn p1(input: &str) -> usize {
    let mut c = Comp::parse(input);
    c.run_and_return_b()
}
#[aoc(day23, part2)]
fn p2(input: &str) -> usize {
    let mut c = Comp::parse(input);
    c.regs.insert('a', 1);
    c.run_and_return_b()
}
