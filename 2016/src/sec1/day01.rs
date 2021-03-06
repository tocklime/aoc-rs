use regex::Regex;
use crate::utils::cartesian::{Point, Dir};
use std::collections::HashSet;
use itertools::Itertools;

#[aoc_generator(day1)]
fn gen(input: &str) -> Vec<Point<i32>> {
    let re = Regex::new(r"([RL])(\d+)").unwrap();
    re.captures_iter(input).scan((Dir::Up, Point::new(0, 0)), |a, b| {
        a.0 = match &b[1] {
            "R" => a.0.turn_right(),
            "L" => a.0.turn_left(),
            _ => panic!("Unknown turn instr {}", &b[1])
        };
        let steps = b[2].parse::<i32>().unwrap();
        let points_on_line = (1..=steps).map(|n| a.1 + a.0.as_point_step() * n).collect_vec();
        a.1 += a.0.as_point_step() * steps;
        Some(points_on_line)
    }).flatten().collect()
}

#[aoc(day1, part1)]
#[post(ret == 236)]
fn p1(input: &[Point<i32>]) -> i32 {
    input.last().unwrap().manhattan()
}

#[aoc(day1, part2)]
#[post(ret == Some(182))]
fn p2(input: &[Point<i32>]) -> Option<i32> {
    let mut seen = HashSet::new();
    input.iter().find_map(|x| {
        if seen.insert(x.clone()) {
            None
        } else {
            Some(x.manhattan())
        }
    })
}
