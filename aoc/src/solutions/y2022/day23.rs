use std::{
    collections::{HashMap, HashSet},
    ops::RangeBounds,
};

use aoc_harness::*;
use utils::cartesian::{self, point_map_bounding_box, render_char_map_w, render_set_w, Point};

aoc_main!(2022 day 23, part1 [p1] => 3990, part2 [p2] => 1057, example part1 EG0 => 25, example both EG => (110,20));

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

struct Elf {
    id: usize,
    next_choice: u8,
}

fn step_world(world: &mut HashMap<Point<i64>, u64>, round_num: usize) -> usize {
    let mut proposals: HashMap<Point<i64>, Vec<(Point<i64>, u64)>> = HashMap::new();
    let choices = [0, 1, 2, 3]
        .into_iter()
        .cycle()
        .skip(round_num % 4)
        .take(4)
        .collect_vec();
    for (loc, elf_id) in world.iter() {
        //clockwise from up.
        let n_map = loc
            .neighbours_with_diagonals()
            .map(|p| world.contains_key(&p));
        if n_map.iter().any(|x| *x) {
            let choice = choices.iter().find_map(|c| {
                match c {
                    0 =>
                    //up
                    {
                        [loc.up(), loc.up().left(), loc.up().right()]
                            .into_iter()
                            .all(|x| !world.contains_key(&x))
                            .then_some(loc.up())
                    }
                    1 =>
                    //down
                    {
                        [loc.down(), loc.down().left(), loc.down().right()]
                            .into_iter()
                            .all(|x| !world.contains_key(&x))
                            .then_some(loc.down())
                    }
                    2 =>
                    //left
                    {
                        [loc.left(), loc.left().up(), loc.left().down()]
                            .into_iter()
                            .all(|x| !world.contains_key(&x))
                            .then_some(loc.left())
                    }
                    3 =>
                    //right
                    {
                        [loc.right(), loc.right().up(), loc.right().down()]
                            .into_iter()
                            .all(|x| !world.contains_key(&x))
                            .then_some(loc.right())
                    }
                    _ => panic!(),
                }
            });
            if let Some(choice) = choice {
                proposals.entry(choice).or_default().push((*loc, *elf_id));
            }
        }
    }
    let mut moves = 0;
    for (p, list) in proposals {
        if list.len() == 1 {
            moves += 1;
            let (from, elf) = list[0];
            world.remove(&from);
            world.insert(p, elf);
        }
    }
    moves
}
fn p1(input: &str) -> usize {
    let world = cartesian::as_point_map::<i64>(input, true);
    let mut world = world
        .iter()
        .enumerate()
        .filter_map(|(id, (p, c))| (c == &'#').then_some((*p, id as u64)))
        .collect();
    for r in 0..10 {
        let x = step_world(&mut world, r);
        let w: HashSet<Point<i64>> = world.keys().copied().collect();
        println!(
            "{} ({}):\n{}\n\n",
            r + 1,
            x,
            render_set_w(&w, '#', '.', true)
        );
    }
    let bb = point_map_bounding_box(&world);
    bb.area() - world.len()
}

fn p2(input: &str) -> usize {
    let world = cartesian::as_point_map::<i64>(input, true);
    let mut world = world
        .iter()
        .enumerate()
        .filter_map(|(id, (p, c))| (c == &'#').then_some((*p, id as u64)))
        .collect();
    (0..)
        .find(|r| {
            let x = step_world(&mut world, *r);
            x == 0
        })
        .unwrap()
        + 1
}
