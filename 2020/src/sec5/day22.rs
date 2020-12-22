use std::{collections::{HashSet, VecDeque, hash_map::DefaultHasher}, hash::{Hash, Hasher}, num::ParseIntError, str::FromStr};

type Deck = VecDeque<usize>;
#[derive(Debug, Clone,Hash)]
pub struct Game {
    hands: [Deck; 2],
    print_log: bool,
}

use itertools::Itertools;

impl Game {
    pub fn draw_cards(&mut self) -> Vec<usize> {
        self.hands.iter_mut().map(|x| x.pop_front().unwrap()).collect()
    }
    pub fn replace_cards(&mut self, winner: usize, cards: &[usize]) {
        match winner {
            0 => {
                self.hands[0].push_back(cards[0]);
                self.hands[0].push_back(cards[1]);
            }
            1 => {
                self.hands[1].push_back(cards[1]);
                self.hands[1].push_back(cards[0]);
            }
            _ => panic!("Unknown player"),
        }
    }
    pub fn basic_turn(&mut self) {
        let cards = self.draw_cards();
        let winner = if cards[0] > cards[1] { 0 } else { 1 };
        self.replace_cards(winner, &cards);
    }
    pub fn winner(&self) -> Option<usize> {
        match (self.hands[0].is_empty(), self.hands[1].is_empty()) {
            (false, false) => None,
            (false, true) => Some(0),
            (true, false) => Some(1),
            (true, true) => panic!("Both players have empty decks"),
        }
    }
    pub fn winning_deck(&self) -> Option<&VecDeque<usize>> {
        self.winner().map(|w| &self.hands[w])
    }
    pub fn basic_game(&mut self) {
        while self.winner().is_none() {
            self.basic_turn();
        }
    }
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
    pub fn recursive_game(&mut self, game_counter: &mut usize) -> usize {
        let mut memory = HashSet::new();

        *game_counter += 1;
        let my_game_num = *game_counter;
        let print_log = self.print_log;
        if self.print_log {
            println!("=== Game {} ===\n", my_game_num);
        }
        let mut turn_count = 0;
        while self.winner().is_none() {
            turn_count += 1;
            if print_log {
                println!("-- Round {} (Game {}) --", turn_count, my_game_num);
                for (ix, h) in self.hands.iter().enumerate() {
                    println!("Player {}'s deck: {}", ix + 1, h.iter().map(ToString::to_string).join(", "));
                }
            }
            if !memory.insert(self.get_hash()) {
                if print_log {
                    println!("Seen before, p1 win");
                }
                return 0;
            }
            let cs: Vec<usize> = self.draw_cards();
            if print_log {
                for (c, p) in cs.iter().zip(1..) {
                    println!("Player {} plays: {}", p, c);
                }
            }
            let winner = if cs[0] <= self.hands[0].len() && cs[1] <= self.hands[1].len() {
                if print_log {
                    println!("Playing a sub-game to determine the winner...\n");
                }
                let p1_deck = self.hands[0].iter().take(cs[0]).copied().collect();
                let p2_deck = self.hands[1].iter().take(cs[1]).copied().collect();
                let mut sub_game = Self {
                    hands: [p1_deck, p2_deck],
                    print_log,
                };
                sub_game.recursive_game(game_counter)
            } else if cs[0] > cs[1] {
                0
            } else {
                1
            };
            if print_log {
                println!("Player {} wins round {} of game {}!\n", winner + 1, turn_count, my_game_num);
            }
            self.replace_cards(winner, &cs);
        }
        let winner = self.winner().unwrap();
        if print_log {
            println!("The winner of game {} is player {}!", my_game_num, winner + 1);
        }
        winner
    }
}
impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ps = s.split("\n\n");
        let p1 = ps.next().unwrap().lines().skip(1).map(str::parse).collect::<Result<_, _>>()?;
        let p2 = ps.next().unwrap().lines().skip(1).map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self {
            hands: [p1, p2],
            print_log: false,
        })
    }
}

pub fn basic_turn(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) {
    let c1 = p1.pop_front().unwrap();
    let c2 = p2.pop_front().unwrap();
    assert_ne!(c1, c2);
    if c1 > c2 {
        p1.push_back(c1);
        p1.push_back(c2);
    } else {
        p2.push_back(c2);
        p2.push_back(c1);
    }
}

pub fn basic_game(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) {
    while !p1.is_empty() && !p2.is_empty() {
        basic_turn(p1, p2);
    }
}
#[aoc_generator(day22)]
pub fn gen(input: &str) -> Game {
    input.parse().unwrap()
}

#[aoc(day22, part1)]
pub fn p1(g: &Game) -> usize {
    let mut g = g.clone();
    g.basic_game();
    g.winning_deck().unwrap().iter().rev().zip(1..).map(|(a, b)| a * b).sum()
}

#[aoc(day22, part2)]
pub fn p2(g: &Game) -> usize {
    let mut g = g.clone();
    let mut game_count = 0;
    let a = g.recursive_game(&mut game_count);
    g.hands[a].iter().rev().zip(1..).map(|(a, b)| a * b).sum()
}
