#![allow(clippy::redundant_pattern_matching)]

use reformation::Reformation;
use nom::lib::std::collections::{HashMap, VecDeque};
use itertools::Itertools;

#[derive(Reformation, Debug, Hash, PartialEq, Eq,Clone,Copy)]
enum GiveTarget {
    #[reformation("output {}")]
    Output(usize),
    #[reformation("bot {}")]
    Bot(usize),
}
impl GiveTarget {
    fn bot_value(self) -> Option<usize> {
        match self {
            Self::Output(_) => None,
            Self::Bot(v) => Some(v),
        }
    }
}

#[derive(Reformation, Debug)]
enum Line {
    #[reformation("value {value} goes to bot {bot}")]
    Input {
        value: usize,
        bot: usize,
    },
    #[reformation("bot {bot} gives low to {low} and high to {high}")]
    Give {
        bot: usize,
        low: GiveTarget,
        high: GiveTarget,
    },
}

#[aoc_generator(day10)]
fn gen(input: &str) -> Vec<Line> {
    input.lines().map(|x| Line::parse(x).unwrap()).collect()
}

fn process(input: &[Line]) -> HashMap<GiveTarget, Vec<usize>> {
    let mut known_handlings: HashMap<GiveTarget, Vec<usize>> = HashMap::new();
    let mut to_handle: VecDeque<&Line> = input.iter().collect();
    while !to_handle.is_empty() {
        let item = to_handle.pop_front().unwrap();
        match item {
            Line::Input { bot, value } => known_handlings.entry(GiveTarget::Bot(*bot)).or_insert_with(Vec::new).push(*value),
            Line::Give { bot, low, high } => {
                match known_handlings.get(&GiveTarget::Bot(*bot)) {
                    Some(v) if v.len() >= 2 => {
                        let (&lo, &hi) = v.iter().minmax().into_option().unwrap();
                        known_handlings.entry(*low).or_insert_with(Vec::new).push(lo);
                        known_handlings.entry(*high).or_insert_with(Vec::new).push(hi);
                    }
                    _ => {
                        //don't know yet, put it back.
                        to_handle.push_back(item);
                    }
                }
            }
        }
    }
    known_handlings
}

#[aoc(day10, part1)]

fn p1(input: &[Line]) -> Option<usize> {
    let known_handlings = process(input);
    //find bot which handles 61 and 17
    known_handlings.iter().find(|(_,v)|
        v.contains(&61) && v.contains(&17)
    ).unwrap().0.bot_value()
}

#[aoc(day10,part2)]

fn p2(input: &[Line]) -> usize {
    let known_handlings = process(input);
    (0..3).map(|i| known_handlings[&GiveTarget::Output(i)][0]).product()
}