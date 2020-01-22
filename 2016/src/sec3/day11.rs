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

impl Item {
    fn is_generator(&self) -> bool {
        match self {
            Item::Microchip(_) => false,
            Item::Generator(_) => true,
        }
    }
    fn is_compatible_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Microchip(a), Item::Generator(b)) => a == b,
            (Item::Generator(a), Item::Microchip(b)) => a == b,
            _ => true
        }
    }
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
        (0..3).all(|f| self.chips[f] == 0 && self.gens[f] == 0)
    }
    fn is_safe(&self) -> bool {
        (0..=3).all(|f| {
            self.gens[f] == 0 || self.gens[f].andn(self.chips[f]) == 0
        })
    }
    fn heuristic(&self) -> usize {
        //for each item, 3-floor num.
        (0..=3).map::<usize,_>(|f| {
            let pc:usize = (self.chips[f].popcnt() + self.gens[f].popcnt()).into();
            (3 - f) * pc
        }).sum()
    }
    fn adjacent_floors(&self) -> Vec<usize> {
        let mut ans = Vec::new();
        if self.elevator > 0 { ans.push(self.elevator - 1); }
        if self.elevator < 3 { ans.push(self.elevator + 1); }
        ans
    }

    fn step_world(&self, new_f: usize, with_chips: u8, with_gens: u8) -> World {
        assert!(with_chips.popcnt() + with_gens.popcnt() <= 2);
        assert!(with_chips.popcnt() + with_gens.popcnt() > 0);
        assert!(new_f < self.chips.len());
        assert_eq!(self.chips[self.elevator] & with_chips, with_chips);
        assert_eq!(self.gens[self.elevator] & with_gens, with_gens);
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
    //1: any lone chip or gen
    //2: any pair of lone chips or lone gens (not one of each)
    //2: Any arbitrary... matching chip/gen pair.

    fn neighbours(&self) -> Vec<(World, usize)> {
        //take 1 thing.
        let f = self.elevator;
        let this_pairs = self.chips[f] & self.gens[f];
        let this_lone_gens = self.gens[f] & !self.chips[f];
        let this_lone_chips = self.chips[f] & !self.gens[f];
        let next_fs = self.adjacent_floors();
        let mut ans = Vec::new();
        //any chip
        let mut x = self.chips[f];
        while x != 0 {
            let opt = 1 << x.tzcnt(); //get ix of least sig 1 bit.
            x = opt.andn(x);
            if f > 0 {
                ans.push(self.step_world(f - 1, opt, 0));
            }
            if f < 3 {
                ans.push(self.step_world(f + 1, opt, 0));
            }
        }
        //any gen
        let mut x = self.gens[f];
        while x != 0 {
            let opt = 1 << x.tzcnt();
            x = opt.andn(x);
            if f > 0 {
                ans.push(self.step_world(f - 1, 0, opt));
            }
            if f < 3 {
                ans.push(self.step_world(f + 1, 0, opt));
            }
        }
        //an arbitrary pair.
        let mut x = this_pairs;
        if x != 0 {
            let opt = 1 << x.tzcnt();
            x = opt.andn(x);
            if f > 0 {
                ans.push(self.step_world(f - 1, opt, opt));
            }
            if f < 3 {
                ans.push(self.step_world(f + 1, opt, opt));
            }
        }
        //any pair of lone chips
        let mut x = self.chips[f];
        while x != 0 {
            let opt = 1 << x.tzcnt();
            x = opt.andn(x);
            let mut y = x;
            while y != 0 {
                let opt2 = 1 << y.tzcnt();
                y = opt2.andn(y);
                if f > 0 {
                    ans.push(self.step_world(f - 1, opt | opt2, 0));
                }
                if f < 3 {
                    ans.push(self.step_world(f + 1, opt | opt2, 0));
                }
            }
        }
        //any pair of lone gens
        let mut x = self.gens[f];
        while x != 0 {
            let opt = 1 << x.tzcnt();
            x = opt.andn(x);
            let mut y = x;
            while y != 0 {
                let opt2 = 1 << y.tzcnt();
                y = opt2.andn(y);
                if f > 0 {
                    ans.push(self.step_world(f - 1, 0, opt | opt2));
                }
                if f < 3 {
                    ans.push(self.step_world(f + 1, 0, opt | opt2));
                }
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
