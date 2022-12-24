use std::collections::{HashMap, HashSet};

use aoc_harness::*;
use pathfinding::directed::dijkstra;
use utils::{
    aabb::Aabb,
    cartesian::{self, Dir, Point},
};

aoc_main!(2022 day 24, part1 [p1], part2 [p2], example both EG => (18,54));

const EG: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

#[derive(Debug)]
struct World {
    start: Point<i32>,
    target: Point<i32>,
    blizzards: HashMap<Dir, HashSet<Point<i32>>>,
}

impl World {
    fn space_at(&self, p: Point<i32>, t: i32) -> char {
        let mod_w = self.target.x;
        let mod_h = self.start.y - 1;
        if p == self.target || p == self.start {
            return '.';
        }
        if p.x == 0 || p.x > self.target.x || p.y == 0 || p.y >= self.start.y {
            return '#';
        }
        // dbg!(&self.blizzards[&Dir::Left], mod_w);
        let blizzards = [
            self.blizzards[&Dir::Left]
                .iter()
                .any(|x| x.y == p.y && p.x == ((mod_w + x.x - (t % mod_w) - 1) % mod_w) + 1),
            self.blizzards[&Dir::Right]
                .iter()
                .any(|x| x.y == p.y && p.x == ((x.x + t - 1) % mod_w) + 1),
            self.blizzards[&Dir::Up]
                .iter()
                .any(|x| x.x == p.x && p.y == ((x.y + t - 1) % mod_h) + 1),
            self.blizzards[&Dir::Down]
                .iter()
                .any(|x| x.x == p.x && p.y == ((mod_h + x.y - (t % mod_h) - 1) % mod_h) + 1),
        ];
        match blizzards {
            [false, false, false, false] => '.',
            [true, false, false, false] => '<',
            [false, true, false, false] => '>',
            [false, false, true, false] => '^',
            [false, false, false, true] => 'v',
            x => x
                .into_iter()
                .map(u32::from)
                .sum::<u32>()
                .to_string()
                .chars()
                .next()
                .unwrap(),
        }
    }
    fn render_world_at(&self, t: i32) -> String {
        let mut a = String::new();
        for y in (0..=self.start.y).rev() {
            for x in 0..=self.target.x + 1 {
                let c = self.space_at(Point::new(x, y), t);
                a.push(c);
            }
            a.push('\n');
        }
        a
    }
    fn shortest(&self, start_time: i32, start_point: Point<i32>, target: Point<i32>) -> i32 {
        let x = pathfinding::directed::astar::astar(
            &(start_time, start_point),
            |&(t, p)| {
                let my_t = t;
                let world = &self;
                p.neighbours_and_self()
                    .into_iter()
                    .filter(move |p| world.space_at(*p, my_t + 1) == '.')
                    .map(move |p| ((t + 1, p), 1))
            },
            |(_, p)| (target - *p).manhattan(),
            |(_, p)| *p == target,
        );
        x.unwrap().1 + start_time
    }
}

fn p1(input: &str) -> usize {
    let world = cartesian::as_point_map(input, true);
    let bb: Aabb<i32> = world.keys().collect();
    let start = Point::new(1, bb.top_right.y);
    let target = Point::new(bb.top_right.x - 1, 0);
    let mut blizzards: HashMap<Dir, HashSet<Point<i32>>> = Default::default();
    for (point, char) in world.into_iter() {
        if let Some(d) = Dir::try_from_x("^v<>", char) {
            blizzards.entry(d).or_default().insert(point);
        }
    }
    let world = World {
        start,
        target,
        blizzards,
    };
    // dbg!(&world);
    // dbg!(world.space_at(world.start.down(), 1));
    // println!("{}", world.render_world_at(1));
    // todo!();
    let x = pathfinding::directed::astar::astar(
        &(0, world.start),
        |&(t, p)| {
            let my_t = t;
            let world = &world;
            p.neighbours_and_self()
                .into_iter()
                .filter(move |p| world.space_at(*p, my_t + 1) == '.')
                .map(move |p| ((t + 1, p), 1))
        },
        |(_, p)| (*p - world.target).manhattan(),
        |(_, p)| *p == world.target,
    );

    x.unwrap().1 as usize
}

fn p2(input: &str) -> usize {
    let world = cartesian::as_point_map(input, true);
    let bb: Aabb<i32> = world.keys().collect();
    let start = Point::new(1, bb.top_right.y);
    let target = Point::new(bb.top_right.x - 1, 0);
    let mut blizzards: HashMap<Dir, HashSet<Point<i32>>> = Default::default();
    for (point, char) in world.into_iter() {
        if let Some(d) = Dir::try_from_x("^v<>", char) {
            blizzards.entry(d).or_default().insert(point);
        }
    }
    let world = World {
        start,
        target,
        blizzards,
    };
    // dbg!(&world);
    // dbg!(world.space_at(world.start.down(), 1));
    // println!("{}", world.render_world_at(1));
    // todo!();
    let there = world.shortest(0, world.start, world.target);
    dbg!(there);
    let back = world.shortest(there, world.target, world.start);
    dbg!(back);
    let there_again = world.shortest(back, world.start, world.target);
    dbg!(there_again);

    there_again as usize
}
