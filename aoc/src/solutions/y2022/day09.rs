use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use aoc_harness::*;
use utils::cartesian::{render_char_map_w, Dir, Point};

aoc_main!(2022 day 9, part1 [p1::<2>], part2 [p1::<10>], example part1 EG => 13, example part2 EG2 => 36);

const EG: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
const EG2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
fn step_knot(head: &Point<isize>, tail: &Point<isize>) -> Point<isize>{
    let diff = *head - *tail;
    let mut tail = *tail;
    if diff.x.abs() > 1 || diff.y.abs() > 1 {
        tail = match head.x.cmp(&tail.x) {
            Ordering::Equal => tail,
            Ordering::Less =>  tail.step(Dir::Left),
            Ordering::Greater =>  tail.step(Dir::Right),
        };
        tail = match head.y.cmp(&tail.y) {
            Ordering::Equal => tail,
            Ordering::Less =>  tail.step(Dir::Down),
            Ordering::Greater =>  tail.step(Dir::Up),
        };
    }
    tail
}
fn p1<const SIZE: usize>(input: &str) -> usize {
    // let mut head = Point::<isize>::new(0, 0);
    // let mut tail = Point::<isize>::new(0, 0);
    let mut visited = HashSet::new();
    let mut map = HashMap::new();
    let mut rope = [Point::<isize>::new(0,0); SIZE];
    for l in input.lines() {
        let (dir, count) = l.split_once(' ').unwrap();
        let count: usize = count.parse().unwrap();
        for _ in 0..count {
            rope[0] = rope[0].follow_x("UDLR", dir.chars().next().unwrap());
            for ix in 1..SIZE {
                rope[ix] = step_knot(&rope[ix-1], &rope[ix]);
            }
            map.insert(rope[SIZE-1], '#');
            visited.insert(rope[SIZE-1]);
            // map.insert(head, 'H');
            // map.insert(tail, 'T');
            // println!("{}", render_char_map_w(&map, 1, ".", true));
        }
        println!("{} {:?}", l, rope);
    }
    visited.len()
}
