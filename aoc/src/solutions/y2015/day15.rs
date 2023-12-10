

aoc_harness::aoc_main!(2015 day 15, part1 [p1] => 18965440, part2 [p2] => 15862900);

use counter::Counter;
use itertools::Itertools;
use reformation::Reformation;
use std::cmp::max;

#[derive(Reformation, Debug, Hash, PartialEq, Eq)]
#[reformation(r"{name}: capacity {capacity}, durability {durability}, flavor {flavor}, texture {texture}, calories {calories}")]
struct Line<'a> {
    name: &'a str,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn evaluate(recipe: &Counter<&&Line>, calorie_target: Option<i32>) -> i32 {
    if let Some(t) = calorie_target {
        let cal: i32 = recipe
            .iter()
            .map(|(l, q)| l.calories * (i32::try_from(*q).unwrap()))
            .sum();
        if cal != t {
            return 0;
        }
    }
    let cap: i32 = recipe
        .iter()
        .map(|(l, q)| l.capacity * (i32::try_from(*q).unwrap()))
        .sum();
    let dur: i32 = recipe
        .iter()
        .map(|(l, q)| l.durability * (i32::try_from(*q).unwrap()))
        .sum();
    let fla: i32 = recipe
        .iter()
        .map(|(l, q)| l.flavor * (i32::try_from(*q).unwrap()))
        .sum();
    let tex: i32 = recipe
        .iter()
        .map(|(l, q)| l.texture * (i32::try_from(*q).unwrap()))
        .sum();
    [cap, dur, fla, tex].iter().map(|&x| max(0, x)).product()
}

//
fn p1(input: &str) -> i32 {
    let lines = input.lines().map(|l| Line::parse(l).unwrap()).collect_vec();
    lines
        .iter()
        .combinations_with_replacement(100)
        .map(|candidate| evaluate(&candidate.iter().collect(), None))
        .max()
        .unwrap()
}

//
fn p2(input: &str) -> i32 {
    let lines = input.lines().map(|l| Line::parse(l).unwrap()).collect_vec();
    lines
        .iter()
        .combinations_with_replacement(100)
        .map(|candidate| evaluate(&candidate.iter().collect(), Some(500)))
        .max()
        .unwrap()
}
