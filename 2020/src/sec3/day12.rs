use crate::utils::cartesian::{Dir, Point};

#[aoc_generator(day12)]
pub fn gen(input: &str) -> Vec<(char, isize)> {
    input
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[1..].parse().unwrap()))
        .collect()
}
#[aoc(day12, part1)]
pub fn p1(input: &[(char, isize)]) -> isize {
    let mut ship: Point<isize> = Point::new(0, 0);
    let mut dir = Dir::Right;
    for &l in input {
        match l {
            (c, d) if "NSWE".contains(c) => ship += Dir::from_x("NSWE", c).as_point_step() * d,
            ('F', d) => ship += dir.as_point_step() * d,
            ('L', 90) | ('R', 270) => dir = dir.turn_left(),
            ('R', 90) | ('L', 270) => dir = dir.turn_right(),
            ('L', 180) | ('R', 180) => dir = dir.turn_about(),
            _ => panic!("Unknown instruction {:?}", l),
        }
    }
    ship.manhattan()
}
#[aoc(day12, part2)]
pub fn p2(input: &[(char, isize)]) -> isize {
    let mut ship: Point<isize> = Point::new(0, 0);
    let mut waypoint: Point<isize> = Point::new(10, 1);
    for &l in input {
        match l {
            (c, d) if "NWES".contains(c) => waypoint += Dir::from_x("NSWE", c).as_point_step() * d,
            ('F', d) => ship += waypoint * d,
            ('L', 90) | ('R', 270) => waypoint = waypoint.rotate_left_about_origin(),
            ('R', 90) | ('L', 270) => waypoint = waypoint.rotate_right_about_origin(),
            ('L', 180) | ('R', 180) => waypoint = waypoint.rotate_180_about_origin(),
            _ => panic!("Unknown instruction {:?}", l),
        }
    }
    ship.manhattan()
}
