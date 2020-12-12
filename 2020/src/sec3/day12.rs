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
    input
        .iter()
        .fold((Point::new(0, 0), Dir::Right), |(ship, dir), &l| match l {
            (c, d) if "NSWE".contains(c) => (ship + Dir::from_x("NSWE", c).as_point_step() * d, dir),
            ('F', d) => (ship + dir.as_point_step() * d, dir),
            ('L', d) => (ship, (0..d/90).fold(dir,|d,_|d.turn_left())),
            ('R', d) => (ship, (0..d/90).fold(dir,|d,_|d.turn_right())),
            _ => panic!("Unknown instruction {:?}", l),
        })
        .0
        .manhattan()
}
#[aoc(day12, part2)]
pub fn p2(input: &[(char, isize)]) -> isize {
    input
        .iter()
        .fold((Point::new(0, 0), Point::new(10, 1)), |(ship, way), &l| match l {
            (c, d) if "NSWE".contains(c) => (ship, way + Dir::from_x("NSWE", c).as_point_step() * d),
            ('F', d) => (ship + way * d, way),
            ('L', d) => (ship, (0..d / 90).fold(way, |w, _| w.rotate_left_about_origin())),
            ('R', d) => (ship, (0..d / 90).fold(way, |w, _| w.rotate_right_about_origin())),
            _ => panic!("Unknown instruction {:?}", l),
        })
        .0
        .manhattan()
}
