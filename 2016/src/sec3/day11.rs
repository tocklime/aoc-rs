#![allow(clippy::redundant_pattern_matching)]

use reformation::Reformation;
use pathfinding::directed::astar::astar;
use num::traits::ToPrimitive;
use bitintr::*;

#[derive(Reformation, Copy, Clone, Eq, Hash, PartialEq, Debug, PartialOrd, Ord, ToPrimitive)]
enum Element {
    #[reformation("promethium")]
    Promethium,
    #[reformation("cobalt")]
    Cobalt,
    #[reformation("curium")]
    Curium,
    #[reformation("ruthenium")]
    Ruthenium,
    #[reformation("plutonium")]
    Plutonium,
    #[reformation("elerium")]
    Elerium,
    #[reformation("dilithium")]
    Dilithium,
}

#[derive(Reformation, Copy, Clone, Eq, Hash, PartialEq, Debug, PartialOrd, Ord)]
enum Item {
    #[reformation("{}-compatible microchip.*")]
    Microchip(Element),
    #[reformation("{} generator.*")]
    Generator(Element),
}


#[derive(Reformation, Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum GiveTarget {
    #[reformation("output {}")]
    Output(usize),
    #[reformation("bot {}")]
    Bot(usize),
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct World {
    elevator: usize,
    chips: [u8; 4],
    gens: [u8; 4],
}

impl World {
    fn add(&mut self, i: Item, f: usize) {
        match i {
            Item::Microchip(i) => { self.chips[f] |= 1 << i.to_u8().unwrap() }
            Item::Generator(i) => { self.gens[f] |= 1 << i.to_u8().unwrap() }
        }
    }

    fn is_done(&self) -> bool {
        self.heuristic() == 0
    }
    fn is_safe(&self) -> bool {
        (0..=3).all(|f| {
            self.gens[f] == 0 || self.gens[f].andn(self.chips[f]) == 0
        })
    }
    fn heuristic(&self) -> usize {
        //for each item, 3-floor num.
        (0..=3).map::<usize, _>(|f| {
            let pc: usize = (self.chips[f].popcnt() + self.gens[f].popcnt()).into();
            (3 - f) * pc
        }).sum()
    }

    fn step_world(&self, new_f: usize, with_chips: u8, with_gens: u8) -> World {
        debug_assert!(with_chips.popcnt() + with_gens.popcnt() <= 2);
        debug_assert!(with_chips.popcnt() + with_gens.popcnt() > 0);
        debug_assert!(new_f < self.chips.len());
        debug_assert_eq!(self.chips[self.elevator] & with_chips, with_chips);
        debug_assert_eq!(self.gens[self.elevator] & with_gens, with_gens);
        let mut w = World {
            elevator: new_f,
            chips: self.chips.clone(),
            gens: self.gens.clone(),
        };
        w.chips[self.elevator] = with_chips.andn(w.chips[self.elevator]);
        w.chips[w.elevator] |= with_chips;
        w.gens[self.elevator] = with_gens.andn(w.gens[self.elevator]);
        w.gens[w.elevator] |= with_gens;
        w
    }

    //options to take
    //1: any chip or gen
    //2: any pair of lone chips or lone gens (not one of each)
    //2: Any arbitrary... matching chip/gen pair.

    fn neighbours(&self) -> Vec<(World, usize)> {
        let f = self.elevator;
        let this_pairs = self.chips[f] & self.gens[f];
        let mut opts = Vec::new();
        //any chip
        let mut x = self.chips[f];
        while x != 0 {
            let opt = x.blsi();
            x = opt.andn(x);
            opts.push((opt,0));
        }
        //any gen
        let mut x = self.gens[f];
        while x != 0 {
            let opt = x.blsi();
            x = opt.andn(x);
            opts.push((0,opt));
        }
        //an arbitrary pair.
        if this_pairs != 0 {
            let opt = this_pairs.blsi();
            opts.push((opt,opt));
        }
        //any pair of lone chips
        let mut x = self.chips[f];
        while x != 0 {
            let opt = x.blsi();
            x = opt.andn(x);
            let mut y = x;
            while y != 0 {
                let opt2 = y.blsi();
                y = opt2.andn(y);
                opts.push((opt|opt2, 0));
            }
        }
        //any pair of lone gens
        let mut x = self.gens[f];
        while x != 0 {
            let opt = x.blsi();
            x = opt.andn(x);
            let mut y = x;
            while y != 0 {
                let opt2 = y.blsi();
                y = opt2.andn(y);
                opts.push((0,opt|opt2));
            }
        }
        let mut ans = Vec::new();
        if f > 0 {
            for (chs, gs) in &opts {
                ans.push(self.step_world(f-1,*chs,*gs));
            }
        }
        if f < 3 {
            for (chs, gs) in &opts {
                ans.push(self.step_world(f+1,*chs,*gs));
            }
        }
        (ans.into_iter()
            .filter_map(|w| if w.is_safe() { Some((w, 1)) } else { None }).collect())
    }
}

fn gen(input: &str) -> World {
    let fs: Vec<Vec<Item>> = input.lines().map(|f|
        f.split(" a ").filter_map(|x|
            Item::parse(x).ok()
        ).collect()
    ).collect();
    let mut w = World {
        elevator: 0,
        chips: [0, 0, 0, 0],
        gens: [0, 0, 0, 0],
    };
    fs.iter().enumerate().for_each(|(f, i_vec)|
        i_vec.into_iter().for_each(|i| w.add(*i, f))
    );
    w
}

#[aoc(day11, part1)]
#[post(ret == 33)]
fn p1(input: &str) -> usize {
    let w = gen(input);
    astar(&w, |s| s.neighbours(), |s| s.heuristic(), |s| s.is_done())
        .unwrap().0.len() - 1
}

#[aoc(day11, part2)]
#[post(ret == 57)]
fn p2(input: &str) -> usize {
    let mut w = gen(input);
    w.add(Item::Generator(Element::Elerium), 0);
    w.add(Item::Microchip(Element::Elerium), 0);
    w.add(Item::Generator(Element::Dilithium), 0);
    w.add(Item::Microchip(Element::Dilithium), 0);
    astar(&w, |s| s.neighbours(), |s| s.heuristic(), |s| s.is_done())
        .unwrap().0.len() - 1
}
