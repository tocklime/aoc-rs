use std::{
    collections::{BTreeSet, HashMap},
    hash::{Hash, Hasher},
};

use itertools::Itertools;
use utils::numset::NumSet;

aoc_harness::aoc_main!(2024 day 24, part1 [p1] => 43_559_017_878_162, part2 [p2] => "fhc,ggt,hqk,mwh,qhj,z06,z11,z35", example part1 EG => 4);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Wire {
    ins: [String; 2],
    op: String,
    out: String,
}
impl Hash for Wire {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ins.hash(state);
        self.op.hash(state);
    }
}

impl Wire {
    fn new(a: &str, b: &str, op: &str) -> Self {
        Self {
            ins: [a.to_string(), b.to_string()],
            op: op.to_string(),
            out: String::new(),
        }
    }
    fn parse(s: &str) -> Self {
        let (lhs, rhs) = s.split_once(" -> ").unwrap();
        let ins: Vec<&str> = lhs.split(" ").collect();
        Wire {
            ins: [ins[0].to_owned(), ins[2].to_owned()],
            op: ins[1].to_owned(),
            out: rhs.to_owned(),
        }
    }
    fn run(&self, known: &HashMap<String, usize>) -> Option<usize> {
        if let (Some(a), Some(b)) = (known.get(&self.ins[0][..]), known.get(&self.ins[1][..])) {
            match &self.op[..] {
                "AND" => Some(a & b),
                "OR" => Some(a | b),
                "XOR" => Some(a ^ b),
                _ => unreachable!(),
            }
        } else {
            None
        }
    }
}
impl std::fmt::Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} -> {}",
            self.ins[0], self.op, self.ins[1], self.out
        )
    }
}
#[derive(Clone)]
struct Wires {
    known: HashMap<String, usize>,
    rules: Vec<Wire>,
}
impl Wires {
    fn swap(&mut self, a: &str, b: &str) {
        for r in &mut self.rules {
            if r.out == a {
                r.out = b.to_owned();
            } else if r.out == b {
                r.out = a.to_owned();
            }
        }
    }
    fn new(input: &str) -> Self {
        let mut known: HashMap<String, usize> = HashMap::new();
        let (start_values, rules) = input.split_once("\n\n").unwrap();
        for l in start_values.lines() {
            let (k, v) = l.split_once(": ").unwrap();
            known.insert(k.to_owned(), v.parse().unwrap());
        }
        let rules = rules.lines().map(Wire::parse).collect::<Vec<_>>();
        Self { known, rules }
    }
    fn read_val(&self, val: char) -> usize {
        (0..)
            .map(|x| {
                let key = format!("{val}{:02}", x);
                self.known.get(&key[..])
            })
            .take_while(std::option::Option::is_some)
            .map(|x| x.unwrap())
            .enumerate()
            .fold(0, |n, (pow, x)| n | (x << pow))
    }
    fn run(&mut self) -> usize {
        while !self.rules.is_empty() {
            self.rules.retain(|l| {
                if let Some(v) = l.run(&self.known) {
                    self.known.insert(l.out.to_owned(), v);
                    false
                } else {
                    true
                }
            });
        }
        self.read_val('z')
    }
    fn find_wire(&self, in_1: &str, in_2: &str, op: &str) -> Option<&Wire> {
        self.rules.iter().find(|x| {
            ((x.ins[0] == in_1 && x.ins[1] == in_2) || (x.ins[0] == in_2 && x.ins[1] == in_1))
                && x.op == op
        })
    }
    fn find_wire_one(&self, in_1: &str, op: &str) -> Option<&Wire> {
        self.rules
            .iter()
            .find(|x| ((x.ins[0] == in_1) || (x.ins[1] == in_1)) && x.op == op)
    }

    fn find_next_spine_wire(
        &self,
        c: &str,
        spine_gate: &str,
        input_gate: &Wire,
    ) -> Result<&Wire, (String, String)> {
        let in_gate = self
            .find_wire(&input_gate.ins[0], &input_gate.ins[1], &input_gate.op);
        let in_gate = match in_gate {
            Some(x) => x,
            None => {
                panic!("Cannot find input gate: {}", input_gate);
            }
        };
        //if we can't find in_Gate, then x and y gates are missing. that won't happen, so safe to unwrap it.
        let p = self.find_wire(c, &in_gate.out, spine_gate);
        //if we didn't find p, then either in_gate.out is wrong, or c.out is wrong.
        match p {
            Some(p) => Ok(p),
            None => {
                if let Some(a) = self.find_wire_one(&in_gate.out, spine_gate) {
                    //c.out is wrong, should be whichever input of a isn't in_gate.out
                    let correct = a.ins.iter().find(|x| *x != &in_gate.out).unwrap().clone();
                    Err((c.to_string(), correct))
                } else if let Some(a) = self.find_wire_one(c, spine_gate) {
                    //in_Gate.out is wrong, should be whichever input of a isn't c
                    let correct = a.ins.iter().find(|x| x != &c).unwrap().clone();
                    Err((in_gate.out.clone(), correct))
                } else {
                    unreachable!()
                }
            }
        }
    }
    fn find_next_spine_wire_fix_swaps(
        &mut self,
        swaps: &mut BTreeSet<String>,
        c: &str,
        spine_gate: &str,
        input_gate: &Wire,
    ) -> String {
        let f = self.find_next_spine_wire(c, spine_gate, input_gate);
        match f {
            Ok(x) => x.out.clone(),
            Err((a, b)) => {
                swaps.insert(a.clone());
                swaps.insert(b.clone());
                self.swap(&a, &b);
                let ans = self
                    .find_next_spine_wire(c, spine_gate, input_gate)
                    .unwrap()
                    .out
                    .clone();
                ans
            }
        }
    }

    fn validate_bit(&self, n: u8) -> bool {
        for x in 0..2 {
            for y in 0..2 {
                for carry in 0..2 {
                    let known = make_inputs(n, x == 1, y == 1, carry == 1);
                    let mut wires = Wires {
                        known,
                        rules: self.rules.clone(),
                    };
                    let x = wires.read_val('x');
                    let y = wires.read_val('y');
                    let z = wires.run();
                    if z != x + y {
                        return false;
                    }
                }
            }
        }
        true
    }
}

fn p1(input: &str) -> usize {
    Wires::new(input).run()
}

const INPUT_SIZE: u8 = 45;
fn make_inputs(n: u8, x: bool, y: bool, carry: bool) -> HashMap<String, usize> {
    let mut known = HashMap::new();
    for i in 0..INPUT_SIZE {
        let (x, y) = if i + 1 == n {
            (carry, carry)
        } else if i == n {
            (x, y)
        } else {
            (false, false)
        };
        known.insert(format!("x{:02}", i), x.into());
        known.insert(format!("y{:02}", i), y.into());
    }
    known
}

fn p2(input: &str) -> String {
    let (_, rules) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().map(Wire::parse).collect::<Vec<_>>();
    let mut wires = Wires {
        known: HashMap::new(),
        rules,
    };

    //now need to isolate rules with problems.
    let mut suspect_bits: NumSet<u64> = NumSet::new();
    let mut last_was_good = true;
    for bit_pos in 0..INPUT_SIZE {
        if wires.validate_bit(bit_pos) {
            last_was_good = true;
        } else {
            if last_was_good {
                suspect_bits.insert(bit_pos);
            }
            last_was_good = false;
        }
    }
    //z00 bit is just x00 XOR y00.
    let mut swaps = BTreeSet::<String>::new();
    let c00 = wires.find_wire("x00", "y00", "AND").unwrap();
    let mut c_last = c00.out.clone();
    for n in 2..INPUT_SIZE {
        let x_prev = format!("x{:02}", n - 1);
        let y_prev = format!("y{:02}", n - 1);
        let x = format!("x{:02}", n);
        let y = format!("y{:02}", n);
        let z = format!("z{:02}", n);
        c_last = wires.find_next_spine_wire_fix_swaps(
            &mut swaps,
            &c_last,
            "AND",
            &Wire::new(&x_prev, &y_prev, "XOR"),
        );
        c_last = wires.find_next_spine_wire_fix_swaps(
            &mut swaps,
            &c_last,
            "OR",
            &Wire::new(&x_prev, &y_prev, "AND"),
        );
        let z_now = wires.find_next_spine_wire_fix_swaps(
            &mut swaps,
            &c_last,
            "XOR",
            &Wire::new(&x, &y, "XOR"),
        );
        if z_now != z {
            swaps.insert(z_now.clone());
            swaps.insert(z.clone());
            wires.swap(&z_now, &z);
        }
    }
    swaps.iter().join(",")
}

const EG: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";
