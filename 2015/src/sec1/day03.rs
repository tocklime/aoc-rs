use crate::utils::cartesian::Point;
use std::collections::HashSet;
use itertools::Itertools;

#[aoc(day3, part1)]
pub fn p1(input: &str) -> usize {
    let mut pos = Point::new(0, 0);
    let mut visited: HashSet<Point<i32>> = HashSet::new();
    visited.insert(pos);
    for c in input.chars() {
        pos = pos.follow_arrow(c);
        visited.insert(pos);
    }
    visited.len()
}

#[aoc(day3, part2)]
pub fn p2(input: &str) -> usize {
    let mut santa = Point::new(0, 0);
    let mut robosanta = Point::new(0, 0);
    let mut visited: HashSet<Point<i32>> = HashSet::new();
    visited.insert(santa);
    for mut cs in &input.chars().chunks(2) {
        santa = santa.follow_arrow(cs.next().unwrap());
        visited.insert(santa);
        robosanta = robosanta.follow_arrow(cs.next().unwrap());
        visited.insert(robosanta);
    }
    visited.len()
}
