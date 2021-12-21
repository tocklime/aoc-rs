use std::str::FromStr;

use aoc_harness::*;
use intmap::IntMap;

aoc_main!(2021 day 21, generator whole_input_is::<Game>, part1 [p1] => 598_416, part2 [p2] => 27_674_034_218_179, example part1 EG => 739_785, example part2 EG => 444_356_092_776_315);

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
    fn as_int(&self) -> u16 {
        //for p2, scores need 6 bits (16<21<32)
        //and positions need 5 bits (8<10<16)
        let n1 = self.scores[0] * 10 + (self.positions[0]);
        let n2 = self.scores[1] * 10 + (self.positions[1]);
        n1 | n2 << 8
    }
    fn take_turn(&mut self, roll: u16) -> Option<(usize, usize)> {
        let p_ix = 0;
        self.turn += 1;
        self.positions[p_ix] = (self.positions[p_ix] + roll) % 10;
        let pos = self.positions[p_ix];
        let score = if pos == 0 { 10 } else { pos };
        self.scores[p_ix] += score;
        if self.scores[p_ix] >= self.target_score {
            Some((p_ix, 3 * self.turn * usize::from(self.scores[1 - p_ix])))
        } else {
            self.scores.swap(0, 1);
            self.positions.swap(0, 1);
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

const ROLLS: [(u16, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
fn explore_from(g: &Game, weight: usize, cache: &mut IntMap<[usize; 2]>) -> [usize; 2] {
    let n: u64 = g.as_int().into();
    let mut wins = [0, 0];
    if let Some(w) = cache.get(n) {
        wins = *w;
    } else {
        for (r, w) in ROLLS {
            let mut g = g.clone();
            if let Some((p, _)) = g.take_turn(r) {
                wins[p] += w;
            } else {
                let w = explore_from(&g, w, cache);
                wins[0] += w[1];
                wins[1] += w[0];
            }
        }
        cache.insert(n, wins);
    }
    [wins[0] * weight, wins[1] * weight]
}

fn p2(input: &Game) -> usize {
    let mut g = input.clone();
    g.target_score = 21;
    let mut cache = IntMap::new();
    let ws = explore_from(&g, 1, &mut cache);
    ws.into_iter().max().unwrap()
}
