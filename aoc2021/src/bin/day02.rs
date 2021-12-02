use std::str::FromStr;

use aoc_harness::*;
use sscanf::scanf;
use utils::cartesian::{Dir, Point};

aoc_main!(2021 day 2, [p1], []);

struct X {
    dir: Dir,
}
impl FromStr for X {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max, letter, password) = scanf!(s, "{}-{} {}: {}", usize, usize, char, String)
            .ok_or_else(|| format!("Bad input: {}", s))?;
        todo!()
    }
}
fn p1(input: &str) -> isize {
    let mut pos: Point<isize> = Point::new(0, 0);
    let mut aim: isize = 0;
    for ins in input.lines() {
        let (dir, x) = scanf!(ins, "{} {}", String, isize).unwrap();
        match dir.as_ref() {
            "forward" => {
                pos += Dir::Right * x + Dir::Up * aim * x;
            }
            "up" => {
                aim -= x;
            }
            "down" => {
                aim += x;
            }
            _ => unreachable!(),
        };
        dbg!(dir, x, aim, pos);
    }
    pos.x * pos.y
}

#[test]
fn test_eg1() {
    let input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
    assert_eq!(p1(input), 900);
}
