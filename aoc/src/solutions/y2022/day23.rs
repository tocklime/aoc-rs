use std::collections::{HashMap, HashSet};

use aoc_harness::*;
use utils::{
    aabb::Aabb,
    cartesian::{as_point_map, Point},
};

aoc_main!(2022 day 23, generator gen, part1 [p1] => 3990, part2 [p2] => 1057, example both EG0 => (25,4), example both EG => (110,20));

const EG: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

const EG0: &str = ".....
..##.
..#..
.....
..##.
.....
";

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
enum Solo<T> {
    #[default]
    None,
    One(T),
    Many,
}
impl<T> Solo<T> {
    fn push(&mut self, t: T) {
        *self = match self {
            Solo::None => Solo::One(t),
            _ => Solo::Many,
        };
    }
}

fn step_world(world: &mut HashSet<Point<i64>>, round_num: usize) -> usize {
    let mut proposals: HashMap<Point<i64>, Solo<Point<i64>>> = HashMap::new();
    const CHOICES: [[usize; 3]; 4] = [[7, 0, 1], [3, 4, 5], [5, 6, 7], [1, 2, 3]];
    for loc in world.iter() {
        //clockwise from up.
        let neighbours = loc.neighbours_with_diagonals();
        let n_map = loc.neighbours_with_diagonals().map(|p| world.get(&p));
        // 701
        // 6#2
        // 543
        if n_map.iter().any(|x| x.is_some()) {
            let choice = (0..4).find_map(|ix| {
                let ch = &CHOICES[(ix + round_num) % 4];
                ch.iter()
                    .all(|x| n_map[*x].is_none())
                    .then_some(neighbours[ch[1]])
            });
            if let Some(choice) = choice {
                proposals.entry(choice).or_default().push(*loc);
            }
        }
    }
    let mut moves = 0;
    for (p, list) in proposals {
        if let Solo::One(from) = list {
            moves += 1;
            world.remove(&from);
            world.insert(p);
        }
    }
    moves
}

fn gen(input: &str) -> HashSet<Point<i64>> {
    as_point_map::<i64>(input, true)
        .into_iter()
        .filter_map(|(p, c)| (c == '#').then_some(p))
        .collect()
}
fn p1(input: &HashSet<Point<i64>>) -> usize {
    let mut world = input.clone();
    let _: usize = (0..10).map(|r| step_world(&mut world, r)).sum();
    let bb: Aabb<i64> = world.iter().collect();
    bb.area() - world.len()
}

fn p2(input: &HashSet<Point<i64>>) -> usize {
    let mut world = input.clone();
    (0..).find(|r| step_world(&mut world, *r) == 0).unwrap() + 1
}
