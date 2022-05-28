use parse_display::{Display, FromStr};

use aoc_harness::aoc_main;

aoc_main!(2020 day 2, generator gen, part1 [p1] => 620, part2 [p2] => 727);

use utils::inputs::parse_input_from_str_sep_by;

#[derive(FromStr, Display, Debug)]
#[display(r"{min}-{max} {letter}: {password}")]
pub struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Password {
    pub fn is_valid_1(&self) -> bool {
        (self.min..=self.max).contains(&self.password.chars().filter(|&x| x == self.letter).count())
    }
    pub fn is_valid_2(&self) -> bool {
        self.password
            .chars()
            //decorate with indices
            .zip(1..)
            //limit to self.max (as we're only interested in the characters at min and max)
            .take(self.max)
            .filter(|(c, ix)| *c == self.letter && [self.min, self.max].contains(ix))
            .count()
            == 1
    }
}

pub fn gen(input: &str) -> Vec<Password> {
    parse_input_from_str_sep_by(input, "\n")
}
pub fn p1(input: &[Password]) -> usize {
    input.iter().filter(|x| x.is_valid_1()).count()
}
pub fn p2(input: &[Password]) -> usize {
    input.iter().filter(|x| x.is_valid_2()).count()
}
