use aoc_harness_macros::aoc_main;

use sscanf::scanf;
use utils::inputs::parse_input_from_str_sep_by;

#[derive(Debug)]
pub struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}
impl std::str::FromStr for Password {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max, letter, password) = scanf!(s, "{}-{} {}: {}", usize, usize, char, String)
            .ok_or_else(|| format!("Bad input: {}", s))?;
        Ok(Self {
            min,
            max,
            letter,
            password,
        })
    }
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

#[test]
fn test_parse() {
    let a: Password = "1-2 a: aaaa".parse().unwrap();
    assert_eq!(a.min, 1);
    assert_eq!(a.max, 2);
    assert_eq!(a.letter, 'a');
    assert_eq!(a.password, "aaaa");
}

aoc_main!(2020 day 2, generator gen, [p1] => 620, [p2] => 727);
