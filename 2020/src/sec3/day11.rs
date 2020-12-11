use itertools::{iterate, Itertools};
use std::collections::HashMap;

use crate::utils::{
    aabb::Aabb,
    cartesian::{as_point_map, Point},
};

type M = HashMap<Point<isize>, bool>;

#[aoc_generator(day11)]
pub fn gen(input: &str) -> M {
    as_point_map::<isize>(input, false)
        .iter()
        .filter_map(|(&k, v)| match v {
            'L' => Some((k, false)),
            '#' => Some((k, true)),
            _ => None,
        })
        .collect()
}

pub fn step(input: &M) -> M {
    let mut ans: M = HashMap::new();
    for (&p, &c) in input {
        let count = p
            .neighbours_with_diagonals()
            .iter()
            .filter(|&x| input.get(x) == Some(&true))
            .count();
        //is occupied if no visible occupied, or is already and not too many visible occupied.
        ans.insert(p, count == 0 || c && (1..4).contains(&count));
    }
    ans
}

#[aoc(day11, part1)]
pub fn p1(input: &M) -> Option<usize> {
    let first_duplicate = iterate(input.clone(), step)
        .tuple_windows()
        .find(|(a, b)| a == b)?
        .0;
    Some(first_duplicate.values().filter(|&&x| x).count())
}

pub fn visible_neighbours(input: &M, p: Point<isize>, bounds: &Aabb<isize>) -> usize {
    //find first visible seat in each direction.
    //for each direction, keep going in a direction until you get a chair. Stop when you leave bounds.
    //count the occupied chairs found.
    Point::new(0, 0)
        .neighbours_with_diagonals()
        .iter()
        .filter(|&&d| {
            iterate(p+d,|&p|p+d)
                .take_while(|p| bounds.contains(p))
                .find_map(|p| input.get(&p))
                .copied()
                .unwrap_or(false)
        })
        .count()
}

pub fn step2(input: &M, bounds: &Aabb<isize>) -> M {
    let mut ans: M = HashMap::new();
    for (&p, &c) in input {
        let count = visible_neighbours(input, p, bounds);
        //is occupied if no visible neighbours, or currently occupied and not too many neighbours.
        ans.insert(p, count == 0 || c && (1..5).contains(&count));
    }
    ans
}

#[aoc(day11, part2)]
pub fn p2(input: &M) -> Option<usize> {
    let bounds = input.keys().collect();
    let first_duplicate = iterate(input.clone(), |x| step2(x, &bounds))
        .tuple_windows()
        .find(|(a, b)| a == b)?
        .0;
    Some(first_duplicate.values().filter(|&&x| x).count())
}
