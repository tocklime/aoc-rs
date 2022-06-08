use aoc_harness::aoc_main;

aoc_main!(2015 day 3, part1 [p1], part2 [p2]);
use utils::cartesian::Point;
use std::collections::HashSet;
use itertools::Itertools;


fn p1(input: &str) -> usize {
    let mut pos = Point::new(0, 0);
    let mut visited: HashSet<Point<i32>> = HashSet::new();
    visited.insert(pos);
    for c in input.chars() {
        pos = pos.follow_arrow(c);
        visited.insert(pos);
    }
    visited.len()
}


fn p2(input: &str) -> usize {
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
