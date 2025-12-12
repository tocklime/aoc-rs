use std::collections::BTreeSet;

use bitvec::{order::Lsb0, view::BitView};
use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::{
        complete::{self, newline},
        one_of,
    },
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::delimited,
};
use num::Rational64;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utils::{collections::VecLookup, linear_programming, nom::NomError};
use z3::{Optimize, SatResult, ast::Int};

aoc_harness::aoc_main!(2025 day 10, generator generate, part1 [p1] => 466, part2 [p2_z3, p2_simplex, p2_clear_parity_and_half] => 17214, example part1 EG => 7, example part2 EG => 33);

#[derive(Debug)]
struct Prob {
    target: u32,
    buttons: Vec<u32>,
    #[allow(dead_code)]
    joltage: Vec<u32>,
    constraints: Vec<Constraint>,
}
struct Constraint {
    buttons: u32,
    sum: u32,
}
impl std::fmt::Debug for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Constraint")
            .field("buttons", &format!("0b{:b}", self.buttons))
            .field("sum", &self.sum)
            .finish()
    }
}

impl Prob {
    fn new(target: u32, buttons: Vec<u32>, joltage: Vec<u32>) -> Self {
        let constraints = joltage
            .iter()
            .enumerate()
            .map(|(ix, sum)| {
                //which buttons increment this one?
                let buttons = buttons
                    .iter()
                    .enumerate()
                    .filter(|&(_, b)| (*b & (1 << ix)) > 0)
                    .fold(0u32, |a, (ix, _)| a | 1 << ix);
                Constraint { buttons, sum: *sum }
            })
            .collect();
        Self {
            target,
            buttons,
            joltage,
            constraints,
        }
    }
    fn parser<'a>() -> impl Parser<&'a str, Output = Self, Error = NomError<'a>> {
        (
            nom::sequence::delimited(tag("["), many1(one_of(".#")), tag("] ")).map(|n| {
                n.iter()
                    .enumerate()
                    .map(|(ix, c)| if c == &'#' { 1u32 << ix } else { 0 })
                    .sum::<u32>()
            }),
            many1(
                delimited(
                    tag("("),
                    separated_list1(tag(","), complete::u32),
                    tag(") "),
                )
                .map(|v| v.iter().map(|x| 1 << x).sum()),
            ),
            delimited(tag("{"), separated_list1(tag(","), complete::u32), tag("}")),
        )
            .map(|(target, buttons, joltage)| Self::new(target, buttons, joltage))
    }
    fn solve_parity(&self, p: u32) -> usize {
        //find selection of buttons which xor together to make target.
        //each button is in or out.
        for button_count in 1..=self.buttons.len() {
            //iterate over all numbers with `button_count` buttons pressed.
            for buttons in (0..self.buttons.len()).combinations(button_count) {
                let value = buttons
                    .iter()
                    .map(|&ix| self.buttons[ix])
                    .fold(0, |a, b| a ^ b);
                if value == p {
                    // println!("Pressing {buttons:?} of {self:?} yields {value}. Thats {button_count} buttons");
                    return button_count;
                }
            }
        }
        unreachable!()
    }
    fn power_up(&self) -> usize {
        self.solve_parity(self.target)
    }
    fn solve_joltage_with_clear_parity_and_half(&self) -> u32 {
        let mut todo : BTreeSet<(u32, u32, Vec<u32>)> = [(0, 1, self.joltage.clone())].into_iter().collect();
        let mut parity_options: VecLookup<Vec<(u32, Vec<u32>)>> = VecLookup::new();
        for b_mix in 0u32..(1<<self.buttons.len()) {
            let b_effect = b_mix.view_bits::<Lsb0>().iter_ones().fold(vec![0;self.joltage.len()], |mut p, a| {
                self.buttons[a].view_bits::<Lsb0>().iter_ones().for_each(|j| p[j] += 1);
                p
            });
            let effect_parity = b_effect.iter().enumerate().map(|(ix, b)| if b % 2 == 0 { 0 } else { 1 << ix}).sum::<u32>();
            parity_options.entry(effect_parity as usize).or_default().push((b_mix.count_ones(), b_effect));
        }
        while let Some((count, move_cost, pattern)) = todo.pop_first() {
            if pattern.iter().all(|x| x == &0) {
                return count;
            }
            let current_parity = pattern.iter().enumerate().map(|(ix, x)| if x % 2 == 0 { 0} else {1<<ix}).sum::<u32>();
            for (cost, effect) in parity_options.get(current_parity as usize).iter().copied().flatten() {
                if let Some(new_pattern) = pattern.iter().zip(effect).map(|(a,b)| a.checked_sub(*b).map(|n| n/2)).collect() {
                    todo.insert((count + cost * move_cost, move_cost*2, new_pattern));
                }
            }
        }
        unreachable!()
    }
    fn solve_joltage_with_simplex(&self) -> u64 {
        let mut p = linear_programming::SimplexProb::new();
        for ix in 0..self.buttons.len() {
            p.declare_var((b'a' + ix as u8) as char);
        }
        for c in &self.constraints {
            let lhs: Vec<(i64, char)> = c
                .buttons
                .view_bits::<Lsb0>()
                .iter_ones()
                .map(|ix| (1i64, (b'a' + (ix as u8)) as char))
                .collect();
            p.add_eq_constraint(&lhs, c.sum as i64);
        }
        let all_buttons: Vec<(i64, char)> = (0..self.buttons.len())
            .map(|x| (-1, (b'a' + (x as u8)) as char))
            .collect();
        p.set_objective(&all_buttons);
        let Some(s) = p.solve_integer(false) else {
            p.solve_integer(true);
            panic!()
        };
        assert!(s < Rational64::ZERO);
        (-s).to_integer() as u64
    }
    fn solve_joltage_with_z3(&self, print: bool) -> u64 {
        let buttons = (0..self.buttons.len())
            .map(|_x| Int::fresh_const(&format!("B{_x}")))
            .collect_vec();
        let o = Optimize::new();
        for b in &buttons {
            o.assert(&b.ge(0));
        }
        for c in &self.constraints {
            let ns: Int = c
                .buttons
                .view_bits::<Lsb0>()
                .iter_ones()
                .map(|ix| &buttons[ix])
                .fold(0.into(), |a, i| i + a);
            o.assert(&ns.eq(c.sum));
        }
        let all_buttons: Int = buttons.iter().fold(0.into(), |a, b| a + b);
        o.minimize(&all_buttons);
        assert_eq!(o.check(&[]), SatResult::Sat);
        let ans = o.get_model().unwrap();
        if print {
            println!("{ans:?}");
        }
        buttons
            .iter()
            .map(|b| ans.get_const_interp(b).unwrap().as_u64().unwrap())
            .sum::<u64>()
    }
}

fn generate(input: &str) -> Vec<Prob> {
    all_consuming(separated_list1(newline, Prob::parser())).parse(input.trim()).unwrap().1
}
fn p1(d: &[Prob]) -> usize {
    d.par_iter().map(Prob::power_up).sum::<usize>()
}
fn p2_z3(d: &[Prob]) -> u64 {
    d.par_iter()
        .map(|p| p.solve_joltage_with_z3(false))
        .sum::<u64>()
}
fn p2_simplex(d: &[Prob]) -> u64 {
    d.par_iter().map(Prob::solve_joltage_with_simplex).sum::<u64>()
}

fn p2_clear_parity_and_half(d: &[Prob]) -> u32 {
    d.par_iter().map(Prob::solve_joltage_with_clear_parity_and_half).sum::<u32>()
}

const EG: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
