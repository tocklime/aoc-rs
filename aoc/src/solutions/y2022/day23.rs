use std::option::Option;

use aoc_harness::*;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
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

fn step_world(world: &mut HashSet<Point<i64>>, round_num: usize) -> usize {
    const CHOICES: [[usize; 3]; 4] = [[7, 0, 1], [3, 4, 5], [5, 6, 7], [1, 2, 3]];
    let mut proposals: HashMap<Point<i64>, Option<Point<i64>>> = HashMap::default();
    //we get neighbours from `neighbours_with_diagonals clockwise from up.
    // 701
    // 6#2
    // 543
    for loc in world.iter() {
        let neighbours = loc.neighbours_with_diagonals();
        let n_map = neighbours.map(|p| world.get(&p));
        if n_map.iter().any(Option::is_some) {
            let choice = (0..4).find_map(|ix| {
                let ch = &CHOICES[(ix + round_num) % 4];
                ch.iter()
                    .all(|x| n_map[*x].is_none())
                    .then_some(neighbours[ch[1]])
            });
            if let Some(choice) = choice {
                proposals
                    .entry(choice)
                    .and_modify(|x| *x = None)
                    .or_insert(Some(*loc));
            }
        }
    }
    let mut moves = 0;
    for (p, list) in proposals {
        if let Some(from) = list {
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
    (0..10).for_each(|r| _ = step_world(&mut world, r));
    let bb: Aabb<i64> = world.iter().collect();
    bb.area() - world.len()
}

fn p2(input: &HashSet<Point<i64>>) -> usize {
    let mut world = input.clone();
    (0..).find(|r| step_world(&mut world, *r) == 0).unwrap() + 1
}
