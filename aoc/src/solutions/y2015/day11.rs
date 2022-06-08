use aoc_harness::aoc_main;

aoc_main!(2015 day 11, part1 [p1], part2 [p2]);
use itertools::Itertools;
use std::collections::HashSet;
use std::char;

#[derive(Debug, Clone)]
struct Password(Vec<char>);

impl<'a> Iterator for Password {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let mut pos_to_inc = self.0.len() - 1;
        while self.0[pos_to_inc] == 'z' {
            self.0[pos_to_inc] = 'a';
            pos_to_inc -= 1;
        }
        let curr: u32 = self.0[pos_to_inc].into();
        let next: char = char::from_u32(curr + 1).unwrap();
        self.0[pos_to_inc] = next;
        Some(self.clone())
    }
}
impl From<&str> for Password {
    fn from(s: &str) -> Self {
        Self(s.chars().collect())
    }
}
impl From<Password> for String {
    fn from(p: Password) -> Self {
        p.0.iter().collect()
    }
}

impl Password {
    fn has_straight(&self) -> bool {
        for (&a, &b, &c) in self.0.iter().tuple_windows() {
            let x: u32 = a.into();
            let y: u32 = b.into();
            let z: u32 = c.into();
            if y == x + 1 && z == x + 2 {
                return true;
            }
        }
        false
    }
    fn no_forbidden_letters(&self) -> bool {
        !self.0.contains(&'i')
            && !self.0.contains(&'o')
            && !self.0.contains(&'l')
    }
    fn two_pair(&self) -> bool {
        let pairs: HashSet<_> = self.0.iter().tuple_windows().filter(|(a, b)| a == b).collect();
        pairs.len() >= 2
    }
    fn acceptable(&self) -> bool {
        self.has_straight() && self.no_forbidden_letters() && self.two_pair()
    }
}



fn p1(input: &str) -> String {
    let pw: Password = input.into();
    pw.filter(Password::acceptable).nth(0).unwrap().into()
}


fn p2(input: &str) -> String {
    let pw: Password = input.into();
    pw.filter(Password::acceptable).nth(1).unwrap().into()
}
