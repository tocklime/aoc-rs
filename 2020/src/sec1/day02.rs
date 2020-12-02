#![warn(clippy::all)]
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(FromStr, Display, Debug)]
#[display(r"{min}-{max} {letter}: {password}")]
pub struct Password {
    min : usize,
    max : usize,
    letter: char,
    password: String,
}

impl Password {
    pub fn is_valid_1(&self) -> bool {
        let mut hm: HashMap<char, usize> = HashMap::new();
        for c in self.password.chars() {
            *hm.entry(c).or_default() += 1;
        }
        let c = *hm.entry(self.letter).or_default();
        c >= self.min && c <= self.max
    }
    pub fn is_valid_2(&self) -> bool {
        [self.min,self.max].iter()
            .map(|n| self.password.chars().nth(n-1))
            .filter(|&c| c == Some(self.letter))
            .count() == 1
    }
}

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<Password> {
    input.trim().lines().map(|l| l.parse().unwrap()).collect()
}
#[aoc(day2,part1)]
pub fn p1(input: &[Password]) -> usize {
    input.iter().filter(|x| x.is_valid_1()).count()
}
#[aoc(day2,part2)]
pub fn p2(input: &[Password]) -> usize {
    input.iter().filter(|x| x.is_valid_2()).count()
}