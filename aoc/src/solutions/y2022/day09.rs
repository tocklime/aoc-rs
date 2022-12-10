use std::collections::HashSet;

use aoc_harness::*;
use utils::{
    aabb::Aabb,
    cartesian::{Dir, Point},
    grid2d::Grid2d,
};

aoc_main!(2022 day 9, both [both], part1 [solve::<2>] => 6357, part2 [solve::<10>] => 2627,  example both EG => (13,0), example both EG2 => (88, 36));

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
fn step_knot(head: Point<isize>, tail: Point<isize>) -> Option<Point<isize>> {
    let diff = head - tail;
    if diff.x.abs() > 1 || diff.y.abs() > 1 {
        //take a (possibly diagonal) step toward head.
        Some(tail + Dir::Right * diff.x.signum() + Dir::Up * diff.y.signum())
    } else {
        None
    }
}

fn solve<const SIZE: usize>(input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut rope = [Point::<isize>::new(0, 0); SIZE];
    for l in input.lines() {
        let count = l[2..].parse().unwrap();
        let dir = Dir::from_x("UDLR", l.chars().next().unwrap());
        for _ in 0..count {
            rope[0] = rope[0].step(dir);
            let move_count = (1..SIZE).find(|&ix| {
                match step_knot(rope[ix - 1], rope[ix]) {
                    Some(x) => {
                        rope[ix] = x;
                        false
                    }
                    None => {
                        //this knot didn't move, so we don't expect the rest to.
                        true
                    }
                }
            });
            if move_count.is_none() || move_count >= Some(SIZE - 1) {
                visited.insert(rope[SIZE - 1]);
            }
        }
    }
    visited.len()
}

fn get_bb(input: &str) -> Aabb<isize> {
    let a = input
        .lines()
        .scan(Point::new(0, 0), |p, l| {
            let count: isize = l[2..].parse().unwrap();
            let dir = Dir::from_x("UDLR", l.chars().next().unwrap());
            *p += dir * count;
            Some(*p)
        })
        .collect();
    a
}
fn both(input: &str) -> (usize, usize) {
    let bb = get_bb(input);
    let mut p1 = 0;
    let mut p1_grid = Grid2d::from_elem((bb.height(), bb.width()), false);
    let mut p2 = 0;
    let mut p2_grid = p1_grid.clone();
    const SIZE: usize = 10;
    let mut rope = [Point::<isize>::new(0, 0); SIZE];
    for l in input.lines() {
        let count = l[2..].parse().unwrap();
        let dir = Dir::from_x("UDLR", l.chars().next().unwrap());
        for _ in 0..count {
            rope[0] = rope[0].step(dir);
            let move_count = (1..SIZE)
                .find(|&ix| {
                    match step_knot(rope[ix - 1], rope[ix]) {
                        Some(x) => {
                            rope[ix] = x;
                            false
                        }
                        None => {
                            //this knot didn't move, so we don't expect the rest to.
                            true
                        }
                    }
                })
                .unwrap_or(SIZE);
            if move_count >= 1 {
                let p = rope[1] - bb.bottom_left;
                if p1_grid.insert((p.y as usize, p.x as usize), true) {
                    p1 += 1;
                }
            }
            if move_count >= SIZE - 1 {
                let p = rope[SIZE - 1] - bb.bottom_left;
                if p2_grid.insert((p.y as usize, p.x as usize), true) {
                    p2 += 1;
                }
            }
        }
    }
    (p1, p2)
}
