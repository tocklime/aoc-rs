use std::collections::HashMap;

use itertools::Itertools;
use nohash_hasher::IntMap;

struct Game {
    next_map: IntMap<u32, u32>,
    current: u32,
    max_num: u32,
}
struct GameIter<'a> {
    g: &'a Game,
    cursor: u32,
    start: u32,
}
impl Game {
    pub fn print(&self) {
        let mut x = self.current;
        loop {
            print!("{} ", x);
            x = self.next_map[&x];
            if self.current == x {
                break;
            }
        }
        println!();
    }
    pub fn step(&mut self) {
        let a = self.next_map[&self.current];
        let b = self.next_map[&a];
        let c = self.next_map[&b];
        let new_current = self.next_map[&c];
        let mut destination = self.current - 1;
        if destination == 0 {
            destination = self.max_num
        };
        while [a, b, c].contains(&destination) {
            destination -= 1;
            if destination == 0 {
                destination = self.max_num
            };
        }
        let next = self.next_map[&destination];
        self.next_map.insert(self.current, new_current);
        self.next_map.insert(destination, a);
        self.next_map.insert(c, next);
        self.current = new_current;
    }
    pub fn make_game(seed: &[u32], max_num: u32) -> Self {
        let mut i = seed.to_vec();
        for x in 10..=max_num {
            i.push(x);
        }
        //current cup is top of list.
        let mut next_map: IntMap<u32, u32> = i.iter().copied().tuple_windows().collect();
        next_map.insert(i[i.len() - 1], i[0]);
        Self {
            next_map,
            current: i[0],
            max_num,
        }
    }
    pub fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }
    pub fn iterate_from(&self, from: u32) -> GameIter {
        GameIter {
            g: self,
            cursor: from,
            start: from
        }
    }
}
impl<'a> Iterator for GameIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor = self.g.next_map[&self.cursor];
        if self.cursor == self.start {
            None
        }else {
            Some(self.cursor)
        }
    }
}

#[aoc_generator(day23)]
pub fn gen(input: &str) -> Vec<u32> {
    input.trim().chars().map(|x| x.to_digit(10).unwrap()).collect()
}
#[aoc(day23, part1)]
pub fn p1(input: &[u32]) -> String {
    let mut g = Game::make_game(input, 9);
    g.run(100);
    g.iterate_from(1).map(|x| x.to_string()).join("")
}
#[aoc(day23, part2)]
pub fn p2(input: &[u32]) -> u64 {
    let mut g = Game::make_game(input, 1_000_000);
    g.run(10_000_000);
    g.iterate_from(1).take(2).map(u64::from).product()
}
