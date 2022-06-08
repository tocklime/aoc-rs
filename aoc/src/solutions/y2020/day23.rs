use aoc_harness::aoc_main;

aoc_main!(2020 day 23, generator gen, part1 [p1], part2 [p2]);
use std::convert::TryInto;
struct Game {
    /// next_map is a lookup table mapping cups to their next neighbour on the right.
    /// that is, next_map[a] = b where the cup clockwise from a is b.
    next_map: Vec<u32>,
    current: u32,
    max_num: u32,
}
struct GameIter<'a> {
    g: &'a Game,
    cursor: u32,
    start: u32,
    done: bool,
}
impl Game {
    #[allow(dead_code)]
    fn print(&self) {
        for x in self {
            print!("{} ", x);
        }
        println!();
    }
    fn step(&mut self) {
        let a = self.next_map[self.current as usize];
        let b = self.next_map[a as usize];
        let c = self.next_map[b as usize];
        let new_current = self.next_map[c as usize];
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
        let next = self.next_map[destination as usize];
        self.next_map[self.current as usize] = new_current;
        self.next_map[destination as usize] = a;
        self.next_map[c as usize] = next;
        self.current = new_current;
    }
    fn make_game(seed: &[u32], max_num: u32) -> Self {
        let mut next_map: Vec<u32> = vec![0;(max_num as usize)+1];
        //populate with the seed.
        for v in seed.windows(2) {
            next_map[v[0] as usize] = v[1];
        }
        let max_seed : u32 = seed.len().try_into().unwrap();
        let mut last = seed[seed.len() - 1];
        //now fill in the rest (if any) with ascending numbers.
        for x in max_seed..=max_num {
            next_map[last as usize] = x;
            last = x;
        }
        //now link the final element to the first one.
        next_map[last as usize] = seed[0];
        Self {
            next_map,
            current: seed[0],
            max_num,
        }
    }
    fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }
    pub const fn iterate_from(&self, from: u32) -> GameIter {
        GameIter {
            g: self,
            cursor: from,
            start: from,
            done: false
        }
    }
}
impl<'a> IntoIterator for &'a Game {
    type Item = u32;

    type IntoIter = GameIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iterate_from(self.current)
    }
}
impl<'a> Iterator for GameIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        }else {
            let a = self.cursor;
            self.cursor = self.g.next_map[self.cursor as usize];
            self.done = self.cursor == self.start;
            Some(a)
        }
    }
}

fn gen(input: &str) -> Vec<u32> {
    input.trim().chars().map(|x| x.to_digit(10).unwrap()).collect()
}
fn p1(input: &[u32]) -> String {
    let mut g = Game::make_game(input, 9);
    g.run(100);
    let as_vec = g.iterate_from(1).skip(1).map(|x| x.to_string()).collect::<Vec<_>>();
    as_vec.join("")
}
fn p2(input: &[u32]) -> u64 {
    let mut g = Game::make_game(input, 1_000_000);
    g.run(10_000_000);
    g.iterate_from(1).skip(1).take(2).map(u64::from).product()
}
