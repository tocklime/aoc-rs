#![allow(clippy::redundant_pattern_matching)]

use reformation::Reformation;
use itertools::Itertools;
use std::cmp::max;
use counter::Counter;

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

fn evaluate(recipe: Counter<&&Line>, calorie_target: Option<i32>) -> i32 {
    if let Some(t) = calorie_target {
        let cal: i32 = recipe.iter().map(|(l, q)| l.calories * *q as i32).sum();
        if cal != t {
            return 0;
        }
    }
    let cap: i32 = recipe.iter().map(|(l, q)| l.capacity * *q as i32).sum();
    let dur: i32 = recipe.iter().map(|(l, q)| l.durability * *q as i32).sum();
    let fla: i32 = recipe.iter().map(|(l, q)| l.flavor * *q as i32).sum();
    let tex: i32 = recipe.iter().map(|(l, q)| l.texture * *q as i32).sum();
    [cap,dur,fla,tex].iter().map(|&x| max(0,x)).product()
}

#[aoc(day15, part1)]
#[post(ret == 367_777)]
fn p1(input: &str) -> i32 {
    let lines = input.lines().map(|l| Line::parse(l).unwrap()).collect_vec();
    lines.iter().combinations_with_replacement(100).map(|candidate| {
        evaluate(candidate.iter().collect(),None)
    }).max().unwrap()
}

#[aoc(day15, part2)]
#[post(ret == 3_903_937)]
fn p2(input: &str) -> i32 {
    let lines = input.lines().map(|l| Line::parse(l).unwrap()).collect_vec();
    lines.iter().combinations_with_replacement(100).map(|candidate| {
        evaluate(candidate.iter().collect(),Some(500))
    }).max().unwrap()
}
