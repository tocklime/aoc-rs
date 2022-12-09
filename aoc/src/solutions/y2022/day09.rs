use std::collections::HashSet;

use aoc_harness::*;
use utils::{
    cartesian::{Dir, Point},
};

aoc_main!(2022 day 9, part1 [solve::<2>] => 6357, part2 [solve::<10>] => 2627, both [both], example both EG => (13,1), example both EG2 => (88, 36));

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
fn step_knot(head:Point<isize>, mut tail: Point<isize>) -> Point::<isize> {
    let diff = head - tail;
    if diff.x.abs() > 1 || diff.y.abs() > 1 {
        //take a (possibly diagonal) step toward head.
        tail += Dir::Right * diff.x.signum() + Dir::Up * diff.y.signum();
    }
    tail
}
fn solve<const SIZE: usize>(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut rope = [Point::<isize>::new(0, 0); SIZE];
    for l in input.lines() {
        let count = l[2..].parse().unwrap();
        let dir = Dir::from_x("UDLR", l.chars().next().unwrap());
        for _ in 0..count {
            rope[0] = rope[0].step(dir);
            for ix in 1..SIZE {
                rope[ix] = step_knot(rope[ix-1], rope[ix]);
            }
            visited.insert(rope[SIZE - 1]);
        }
    }
    visited.len()
}

fn both(input: &str) -> (usize, usize) {
    let mut p1 = HashSet::new();
    let mut p2 = HashSet::new();
    const SIZE: usize = 10;
    let mut rope = [Point::<isize>::new(0, 0); SIZE];
    for l in input.lines() {
        let count = l[2..].parse().unwrap();
        let dir = Dir::from_x("UDLR", l.chars().next().unwrap());
        for _ in 0..count {
            rope[0] = rope[0].step(dir);
            for ix in 1..10 {
                rope[ix] = step_knot(rope[ix-1], rope[ix]);
            }
            p1.insert(rope[1]);
            p2.insert(rope[SIZE - 1]);
        }
    }
    (p1.len(), p2.len())
}
