/*The first floor contains a promethium generator and a promethium-compatible microchip.
The second floor contains a cobalt generator, a curium generator, a ruthenium generator, and a plutonium generator.
The third floor contains a cobalt-compatible microchip, a curium-compatible microchip, a ruthenium-compatible microchip, and a plutonium-compatible microchip.
The fourth floor contains nothing relevant.
*/
/*
F4 .
F3 . CoM CuM RM PlG
F2 . CoG CuG RG PlG
F1 E PrG PrM

*/
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Copy,Clone,Eq,Hash,PartialEq)]
enum Item<'a> {
    Microchip(&'a str),
    Generator(&'a str),
}

impl<'a> Item<'a> {
    fn is_compatible_with(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Microchip(a), Item::Generator(b)) => a == b,
            (Item::Generator(a), Item::Microchip(b)) => a == b,
            _ => true
        }
    }
}

#[derive(Clone)]
struct World<'a> {
    elevator: usize,
    floors: [HashSet<Item<'a>>; 4],
}

impl<'a> World<'a> {
    fn is_done(&self) -> bool {
        (0..3).all(|f| self.floors[f].len() == 0)
    }
    fn is_safe(&self) -> bool {
        self.floors.iter().all(|f| f.iter().combinations(2).all(|c| c[0].is_compatible_with(c[1])))
    }
    fn adjacent_floors(&self) ->Vec<usize> {
        let mut ans = Vec::new();
        if self.elevator > 0 {ans.push(self.elevator - 1);}
        if self.elevator < 3 {ans.push(self.elevator + 1);}
        ans
    }
    fn neighbours(&self) -> Vec<World<'a>> {
        //take 1 thing.
        let this_f = &self.floors[self.elevator];
        let next_fs= self.adjacent_floors();
        (1..3)
            .flat_map(|i| this_f.iter().combinations(i) )
            .filter(|c| c.len() < 2 || (c[0].is_compatible_with(c[1])))
            .flat_map(|c| {
                next_fs.iter()
                    .map(move |&new_f| {
                        let mut new_w = self.clone();
                        for &i in &c {
                            new_w.floors[new_w.elevator].remove(i);
                            new_w.floors[new_f].insert(*i);
                        }
                        new_w.elevator = new_f;
                        new_w
                    })
            }).collect()
    }
}

#[aoc(day11,part1)]
fn p1(input: &str) -> usize {
    unimplemented!("TODO: Parse input...");
}