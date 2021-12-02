use std::str::FromStr;

use aoc_harness::*;
use scan_fmt::scan_fmt;
use utils::cartesian::{Dir, Point};

aoc_main!(2021 day 2, generator lines::<Instruction>, [p1] => 1868935, [p2] => 1965970888);

struct Instruction {
    dir: Dir,
    distance: isize,
}
impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, distance) = scan_fmt!(s, "{} {}", String, isize).unwrap();
        let dir = match dir.as_ref() {
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
        .fold((Point::default(), 0), |(pos, aim), ins| match ins.dir {
            Dir::Right => (
                pos + (Dir::Right * ins.distance) + Dir::Up * aim * ins.distance,
                aim,
            ),
            Dir::Down => (pos, aim - ins.distance),
            Dir::Up => (pos, aim + ins.distance),
            _ => unreachable!(),
        })
        .0
        .area()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
    #[test]
    fn test_eg1_p1() {
        assert_eq!(p1(&lines::<Instruction>(EXAMPLE)), 150);
    }
    #[test]
    fn test_eg1_p2() {
        assert_eq!(p2(&lines::<Instruction>(EXAMPLE)), 900);
    }
}
