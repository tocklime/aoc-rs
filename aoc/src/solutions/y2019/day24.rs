use utils::algorithms::{automata_step, automata_step_mut};
use utils::points::Point;

use itertools::iterate;
use num::pow;
use std::collections::HashSet;
use std::convert::TryInto;

aoc_harness::aoc_main!(2019 day 24, generator gen_, part1 [p1] => 32_509_983, part2 [p2m,p2c] => 2012);

#[must_use]
pub fn gen_(input: &str) -> HashSet<Point> {
    let hm = utils::points::as_point_map(input);
    hm.iter()
        .filter_map(|(a, b)| if b == &'#' { Some(*a) } else { None })
        .collect()
}

pub fn p1(input: &HashSet<Point>) -> usize
where
{
    let mut seen = HashSet::new();
    iterate(input.clone(), |g| automata_step(g, flat_neighbours, lives))
        .map(|x| biodiversity(&x))
        .find(|&x| !seen.insert(x))
        .unwrap()
}

pub fn p2m(input: &HashSet<Point>) -> usize {
    let mut g: HashSet<(Point, i32)> = input.iter().map(|a| (*a, 0)).collect();
    for _ in 0..200 {
        automata_step_mut(&mut g, recur_neighbours, lives);
    }
    g.len()
}

pub fn p2c(input: &HashSet<Point>) -> usize {
    let mut g: HashSet<(Point, i32)> = input.iter().map(|a| (*a, 0)).collect();
    for _ in 0..200 {
        g = automata_step(&g, recur_neighbours, lives);
    }
    g.len()
}

pub fn flat_neighbours(p: Point) -> Vec<Point> {
    p.neighbours()
        .iter()
        .copied()
        .filter(|p| p.0 >= 0 && p.1 >= 0 && p.0 < 5 && p.1 < 5)
        .collect()
}

pub fn recur_neighbours(p: (Point, i32)) -> Vec<(Point, i32)> {
    let mut ans = Vec::with_capacity(8);
    flat_neighbours(p.0)
        .into_iter()
        .filter(|x| x != &Point(2, 2))
        .for_each(|x| ans.push((x, p.1)));
    match (p.0).0 {
        0 => ans.push((Point(1, 2), p.1 + 1)),
        4 => ans.push((Point(3, 2), p.1 + 1)),
        _ => (),
    };
    match (p.0).1 {
        0 => ans.push((Point(2, 1), p.1 + 1)),
        4 => ans.push((Point(2, 3), p.1 + 1)),
        _ => (),
    };
    match p.0 {
        Point(2, 1) => (0..5).for_each(|x| ans.push((Point(x, 0), p.1 - 1))),
        Point(1, 2) => (0..5).for_each(|x| ans.push((Point(0, x), p.1 - 1))),
        Point(3, 2) => (0..5).for_each(|x| ans.push((Point(4, x), p.1 - 1))),
        Point(2, 3) => (0..5).for_each(|x| ans.push((Point(x, 4), p.1 - 1))),
        _ => (),
    };
    ans
}

#[must_use]
pub fn lives(is_alive: bool, neighbour_count: usize) -> bool {
    neighbour_count == 1 || (!is_alive && neighbour_count == 2)
}

#[test]
pub fn d24p2() {
    assert_eq!(recur_neighbours((Point(0, 0), 0)).len(), 4);
}

pub fn biodiversity(g: &HashSet<Point>) -> usize
{
    g.iter()
        .map(|&p| pow(2, (p.0 + p.1 * 5).try_into().unwrap()))
        .sum()
}

#[test]
pub fn d24p1() {
    let i = ".....
.....
.....
#....
.#...
";
    let btm = gen_(i);
    assert_eq!(biodiversity(&btm), 2_129_920);
}
