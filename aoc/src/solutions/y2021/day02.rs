use std::str::FromStr;

use aoc_harness::*;
use utils::cartesian::{Dir, Point};

aoc_harness::aoc_main!(2021 day 2, generator lines::<Instruction>, part1 [p1] => 1_868_935, part2 [p2] => 1_965_970_888,
          example both EG => (150, 900));

struct Instruction {
    dir: Dir,
    distance: isize,
}
impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, distance_str) = s.split_once(' ').unwrap();
        let distance = distance_str.parse().unwrap();
        let dir = match dir {
            "forward" => Dir::Right,
            "up" => Dir::Down,
            "down" => Dir::Up,
            _ => unreachable!(),
        };
        Ok(Self { dir, distance })
    }
}

fn p1(input: &[Instruction]) -> isize {
    input
        .iter()
        .fold(Point::<isize>::new(0, 0), |pos, ins| {
            pos + ins.dir * ins.distance
        })
        .area()
}

fn p2(input: &[Instruction]) -> isize {
    input
        .iter()
        .fold((Point::<isize>::default(), 0), |(pos, aim), ins| match ins.dir {
            Dir::Right => (
                pos + (Dir::Right * ins.distance) + Dir::Up * aim * ins.distance,
                aim,
            ),
            Dir::Down => (pos, aim - ins.distance),
            Dir::Up => (pos, aim + ins.distance),
            Dir::Left => unreachable!(),
        })
        .0
        .area()
}

const EG: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
