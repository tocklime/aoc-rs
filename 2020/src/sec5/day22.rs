use std::{
    collections::{
        hash_map::DefaultHasher,
        hash_map::Entry::{Occupied, Vacant},
        VecDeque,
    },
    hash::{Hash, Hasher},
    num::ParseIntError,
    str::FromStr,
};

type Card = u8;
type Player = u8;
type Deck = VecDeque<Card>;
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Game {
    hands: [Deck; 2],
    print_log: bool,
}

use itertools::Itertools;
use nohash_hasher::{IntMap, IntSet};

impl Game {
    #[inline]
    pub fn draw_cards(&mut self) -> Option<[Card; 2]> {
        if self.hands[0].is_empty() || self.hands[1].is_empty() {
            None
        } else {
            Some([self.hands[0].pop_front().unwrap(), self.hands[1].pop_front().unwrap()])
        }
    }
    #[inline]
    pub fn replace_cards(&mut self, winner: Player, cards: &[Card]) {
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
    #[inline]
    pub fn winner(&self) -> Option<Player> {
        match (self.hands[0].is_empty(), self.hands[1].is_empty()) {
            (false, false) => None,
            (false, true) => Some(0),
            (true, false) => Some(1),
            (true, true) => panic!("Both players have empty decks"),
        }
    }
    pub fn score_player(&self, player: Player) -> u32 {
        self.hands[player as usize]
            .iter()
            .rev()
            .zip(1..)
            .map(|(a, b)| u32::from(*a) * b)
            .sum()
    }
    pub fn winning_deck(&self) -> Option<&Deck> {
        self.winner().map(|w| &self.hands[w as usize])
    }
    pub fn basic_game(&mut self) -> Player {
        while let Some(cards) = self.draw_cards() {
            let winner = (0_u8..=1).max_by_key::<u8, _>(|&p| cards[p as usize]).unwrap();
            self.replace_cards(winner, &cards);
        }
        self.winner().unwrap()
    }
    #[inline]
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hands.hash(&mut hasher);
        hasher.finish()
    }
    pub fn recursive_game(&mut self, game_counter: &mut usize) -> Player {
        let mut memory: IntSet<u64> = IntSet::default();
        *game_counter += 1;
        let my_game_num = *game_counter;
        if self.print_log {
            println!("=== Game {} ===\n", my_game_num);
        }
        let mut turn_count = 0;
        while let Some(cs) = self.draw_cards() {
            turn_count += 1;
            if self.print_log {
                println!("-- Round {} (Game {}) --", turn_count, my_game_num);
                for (ix, h) in self.hands.iter().enumerate() {
                    println!("Player {}'s deck: {}", ix + 1, h.iter().map(ToString::to_string).join(", "));
                }
            }
            if turn_count % 2 == 0 && !memory.insert(self.get_hash()) {
                if self.print_log {
                    println!("Game {}, Loop detected on turn {}. P1 win", my_game_num, turn_count);
                }
                return 0;
            }
            if self.print_log {
                for (c, p) in cs.iter().zip(1..) {
                    println!("Player {} plays: {}", p, c);
                }
            }
            let winner = if (cs[0] as usize) <= self.hands[0].len() && (cs[1] as usize) <= self.hands[1].len() {
                if self.print_log {
                    println!("Playing a sub-game to determine the winner...\n");
                }
                let mut it = (0..=1).map(|p| self.hands[p].iter().take(cs[p].into()).copied().collect());
                let mut sub_game = Self {
                    hands: [it.next().unwrap(), it.next().unwrap()],
                    print_log: self.print_log,
                };
                sub_game.recursive_game(game_counter)
            } else {
                (0..=1).max_by_key(|&x| cs[x as usize]).unwrap()
            };
            if self.print_log {
                println!("Player {} wins round {} of game {}!\n", winner + 1, turn_count, my_game_num);
            }
            self.replace_cards(winner, &cs);
        }
        let winner = self.winner().unwrap();
        if self.print_log {
            println!("The winner of game {} is player {}!", my_game_num, winner + 1);
        }
        winner
    }
}
impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ps = s.split("\n\n");
        let mut players = ps.map(|p| p.lines().skip(1).map(str::parse).collect::<Result<_, _>>());
        Ok(Self {
            hands: [players.next().unwrap()?, players.next().unwrap()?],
            print_log: false,
        })
    }
}

#[aoc(day22, part1)]
pub fn p1(input: &str) -> u32 {
    let mut g: Game = input.parse().unwrap();
    let a = g.basic_game();
    g.score_player(a)
}

#[aoc(day22, part2)]
pub fn p2(input: &str) -> u32 {
    let mut g: Game = input.parse().unwrap();
    let mut game_count = 0;
    g.print_log = false;
    let a = g.recursive_game(&mut game_count);
    g.score_player(a)
}
