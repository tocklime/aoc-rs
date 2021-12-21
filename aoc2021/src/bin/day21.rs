use std::{collections::HashMap, str::FromStr};

use aoc_harness::*;

aoc_main!(2021 day 21, generator whole_input_is::<Game>, part1 [p1] => 598416, part2 [p2], example part1 EG => 739_785, example part2 EG => 444_356_092_776_315);

const EG: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Game {
    turn: usize,
    scores: [u16; 2],
    positions: [u16; 2],
    target_score: u16,
}
impl Game {
    fn new(a: u16, b: u16) -> Self {
        Self {
            turn: 0,
            scores: [0, 0],
            positions: [a, b],
            target_score: 0,
        }
    }
    fn increase(val: &mut u16, increase: u16, modulo: u16) -> u16 {
        *val = (*val + increase) % modulo;
        *val
    }
    fn take_turn(&mut self, roll: u16) -> Option<(usize, usize)> {
        let p_ix = self.turn % 2;
        self.turn += 1;
        let pos = Self::increase(&mut self.positions[p_ix], roll, 10);
        let score = if pos == 0 { 10 } else { pos };
        self.scores[p_ix] += u16::from(score);
        if self.scores[p_ix] >= self.target_score {
            Some((p_ix, 3 * self.turn * usize::from(self.scores[1 - p_ix])))
        } else {
            None
        }
    }
}
impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p: Vec<u16> = s
            .lines()
            .map(|x| x.split(": ").nth(1).unwrap().parse().unwrap())
            .collect_vec();
        Ok(Self::new(p[0], p[1]))
    }
}
fn p1(input: &Game) -> usize {
    let mut g = input.clone();
    g.target_score = 1000;
    let all_rolls = (1..=100).cycle().chunks(3);
    let mut die = all_rolls.into_iter().map(std::iter::Iterator::sum);
    loop {
        let roll = die.next().unwrap();

        if let Some(x) = g.take_turn(roll) {
            return x.1;
        }
    }
}

//outcome of 3 dice:
/*
3 1
4 3
5 6
6 7
7 6
8 3
9 1
*/
const ROLLS: [(u16, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
fn explore_from(g: &Game, weight: usize, cache: &mut HashMap<Game, [usize; 2]>) -> [usize; 2] {
    if let Some(w) = cache.get(g) {
        return [w[0] * weight, w[1] * weight];
    }
    let mut wins = [0, 0];
    for (r, w) in ROLLS {
        let mut g = g.clone();
        if let Some((p, _)) = g.take_turn(r) {
            wins[p] += w;
        } else {
            let w = explore_from(&g, w, cache);
            wins[0] += w[0];
            wins[1] += w[1];
        }
    }
    cache.insert(g.clone(), wins);
    [wins[0] * weight, wins[1] * weight]
}
fn p2(input: &Game) -> usize {
    let mut g = input.clone();
    g.target_score = 21;
    let mut cache = HashMap::new();
    let [a, b] = explore_from(&g, 1, &mut cache);
    std::cmp::max(a, b)
}
