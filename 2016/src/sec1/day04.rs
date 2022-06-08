#![allow(clippy::redundant_pattern_matching)]

use reformation::Reformation;
use counter::Counter;
use std::convert::TryInto;

#[derive(Reformation, Debug)]
#[reformation(r"{name}-{id}\[{checksum}\]")]
struct Room<'a> {
    name: &'a str,
    id: usize,
    checksum: &'a str,
}

impl Room<'_> {
    fn mk_checksum(&self) -> String {
        let counts: Counter<_> = self.name.chars().filter(|c| c.is_alphabetic()).collect();
        counts.most_common_ordered()[0..5].iter().map(|(a, _)| a).collect()
    }
    fn is_valid(&self) -> bool {
        self.checksum == self.mk_checksum()
    }
    fn decrypt_name(&self) -> String {
        let key: u8 = (self.id % 26).try_into().unwrap();
        self.name.chars().map(|c| {
            if c.is_alphabetic() {
                let ix = c as u8 - b'a';
                let new_ix = (ix + key) % 26 + b'a';
                new_ix as char
            } else {
                ' '
            }
        }).collect()
    }
}

fn gen(input: &str) -> Vec<Room> {
    input.lines().map(|l| Room::parse(l).unwrap()).collect()
}

#[aoc(day4, part1)]

fn p1(input: &str) -> usize {
    gen(input).into_iter().filter_map(|r| {
        if r.is_valid() { Some(r.id) } else { None }
    }).sum()
}

#[aoc(day4, part2)]

fn p2(input: &str) -> usize {
    gen(input).into_iter().find(|r|
        r.is_valid() && r.decrypt_name() == "northpole object storage"
    ).unwrap().id
}