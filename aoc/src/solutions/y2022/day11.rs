use std::{cmp::Reverse, collections::VecDeque};

use aoc_harness::*;

aoc_main!(2022 day 11, generator gen, part1 [p1::<3, 20>] => 110885, part2 [p1::<1, 10000>] => 25272176808, example both EG => (10605, 2713310158));

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

#[derive(Debug,Clone)]
enum Operation {
    AddI(usize),
    MulI(usize),
    Double,
    Square
}
#[derive(Debug,Clone)]
struct Monkey {
    held: VecDeque<usize>,
    alter: Operation,
    test_div: usize,
    true_target: usize,
    false_target: usize,
}


fn gen(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();
    for m_input in input.split("\n\n") {
        let mut l = m_input.lines().skip(1);
        let held = l.next().unwrap()["  Starting items: ".len()..]
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();
        let alter = match l.next().unwrap()
            ["  Operation: new = old ".len()..]
            .split_once(' ')
            .unwrap()
        {
            ("+", "old") => Operation::Double,
            ("+", x) => {
                let x: usize = x.parse().unwrap();
                Operation::AddI(x)
            }
            ("*", "old") => Operation::Square,
            ("*", x) => {
                let x: usize = x.parse().unwrap();
                Operation::MulI(x)
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
        });
    }
    monkeys
}

fn p1<const DIV: usize, const ROUNDS: usize>(input: &[Monkey]) -> usize {
    let mut monkeys : Vec<Monkey> = input.to_vec();
    let mut inspection_counts = vec![0; monkeys.len()];
    let big_modulo : usize = monkeys.iter().map(|m| m.test_div).product();
    for _round in 0..ROUNDS {
        for m_ix in 0..monkeys.len() {
            inspection_counts[m_ix] += monkeys[m_ix].held.len();
            while let Some(i) = monkeys[m_ix].held.pop_front() {
                let me = &mut monkeys[m_ix];
                let new_i = match me.alter {
                    Operation::AddI(x) => i + x,
                    Operation::MulI(x) => i * x,
                    Operation::Double => 2 * i,
                    Operation::Square => i * i,
                } / DIV % big_modulo;
                let new_t = if new_i % me.test_div == 0 {
                    me.true_target
                } else {
                    me.false_target
                };
                monkeys[new_t].held.push_back(new_i);
            }
        }
    }
    inspection_counts
        .into_iter()
        .map(Reverse)
        .k_smallest(2)
        .map(|x| x.0)
        .product()
}
