use std::mem;

use aoc_harness::*;

aoc_main!(2022 day 11, part1 [p1::<3, 20>] => 110885, part2 [p1::<1, 10000>] => 25272176808, example both EG => (10605, 2713310158));

const EG: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

struct Monkey {
    held: Vec<usize>,
    alter: Box<dyn Fn(usize) -> usize>,
    test_div: usize,
    true_target: usize,
    false_target: usize,
    inspection_count: usize,
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("held", &self.held)
            .field("test_div", &self.test_div)
            .field("true_target", &self.true_target)
            .field("false_target", &self.false_target)
            .field("inspection_count", &self.inspection_count)
            .finish()
    }
}

fn p1<const DIV: usize, const ROUNDS: usize>(input: &str) -> usize {
    let mut monkeys = Vec::new();
    for (m_ix, m_input) in input.split("\n\n").enumerate() {
        let mut l = m_input.lines().skip(1);
        let held = l.next().unwrap()["  Starting items: ".len()..]
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();
        let alter: Box<dyn Fn(usize) -> usize> = match l.next().unwrap()
            ["  Operation: new = old ".len()..]
            .split_once(' ')
            .unwrap()
        {
            ("+", "old") => Box::new(|i| i + i),
            ("+", x) => {
                let x: usize = x.parse().unwrap();
                Box::new(move |i| i + x)
            }
            ("*", "old") => Box::new(|i| i * i),
            ("*", x) => {
                let x: usize = x.parse().unwrap();
                Box::new(move |i| x * i)
            }
            (a, b) => panic!("{} {}??", a, b),
        };
        let test_div: usize = l.next().unwrap()["  Test: divisible by ".len()..]
            .parse()
            .unwrap();
        let true_target: usize = l.next().unwrap()["    If true: throw to monkey ".len()..]
            .parse()
            .unwrap();
        let false_target: usize = l.next().unwrap()["    If false: throw to monkey ".len()..]
            .parse()
            .unwrap();
        monkeys.push(Monkey {
            held,
            alter,
            test_div,
            true_target,
            false_target,
            inspection_count: 0,
        });
    }
    let big_modulo : usize = monkeys.iter().map(|m| m.test_div).product();
    for _round in 0..ROUNDS {
        for m_ix in 0..monkeys.len() {
            let mut monkey_held = Vec::new();
            mem::swap(&mut monkeys[m_ix].held, &mut monkey_held);
            monkeys[m_ix].inspection_count += monkey_held.len();
            for i in monkey_held.into_iter() {
                let new_i = (*monkeys[m_ix].alter)(i) / DIV % big_modulo;
                let new_t = if new_i % monkeys[m_ix].test_div == 0 {
                    monkeys[m_ix].true_target
                } else {
                    monkeys[m_ix].false_target
                };
                // println!(
                //     "Monkey {} changes {} to {} and throws to {}",
                //     m_ix, i, new_i, new_t
                // );
                monkeys[new_t].held.push(new_i);
            }
            // dbg!(&monkeys);
        }
    }
    let counts: Vec<usize> = monkeys
        .into_iter()
        .map(|m| m.inspection_count)
        .sorted()
        .collect();
    counts[counts.len() - 1] * counts[counts.len() - 2]
}
