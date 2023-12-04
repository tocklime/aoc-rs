use aoc_harness::*;
use utils::{
    aabb::Aabb,
    cartesian::{self, Dir, Point},
    collections::VecLookup,
};

aoc_main!(2022 day 24, both [solve] => (373,997), example both EG => (18,54));

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
    blizzards: [VecLookup<Vec<bool>>; 4],
}

impl World {
    fn blizzard_location(blizzard_start: i32, width: i32, direction: i32, time: i32) -> i32 {
        ((blizzard_start + direction * (time % width) - 1).rem_euclid(width)) + 1
    }
    fn space_at(&self, p: Point<i32>, t: i32) -> char {
        let mod_w = self.target.x;
        let mod_h = self.start.y - 1;
        if p == self.target || p == self.start {
            return '.';
        }
        if p.x <= 0 || p.x > self.target.x || p.y <= 0 || p.y >= self.start.y {
            return '#';
        }
        //here, instead of moving every blizzard t steps, we move ourself t steps back to t=0 in each direction,
        let bliz_locs = [
            (Dir::Left, p.y, Self::blizzard_location(p.x, mod_w, 1, t)),
            (Dir::Right, p.y, Self::blizzard_location(p.x, mod_w, -1, t)),
            (Dir::Down, p.x, Self::blizzard_location(p.y, mod_h, 1, t)),
            (Dir::Up, p.x, Self::blizzard_location(p.y, mod_h, -1, t)),
        ];
        //then we check if a blizzard of the necessary direction started in that spot.
        let blizzs: Vec<bool> = (0..4)
            .map(|ix| {
                let (_dir, fixed, p) = bliz_locs[ix];
                let blizz = &self.blizzards[ix];
                blizz
                    .get(fixed as usize)
                    .map(|x| x[p as usize])
                    .unwrap_or(false)
            })
            .collect();
        match &blizzs[..] {
            &[false, false, false, false] => '.',
            &[true, false, false, false] => '<',
            &[false, true, false, false] => '>',
            &[false, false, true, false] => '^',
            &[false, false, false, true] => 'v',
            x => (x.iter().filter(|x| **x).count() as u8 + b'0') as char,
        }
    }
    #[allow(dead_code)]
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

fn solve(input: &str) -> (i32, i32) {
    let world = cartesian::as_point_map(input, true);
    let bb: Aabb<i32> = world.keys().collect();
    let start = Point::new(1, bb.top_right.y);
    let target = Point::new(bb.top_right.x - 1, 0);
    let mut blizzards: [VecLookup<Vec<bool>>; 4] = Default::default();
    for (point, char) in world.into_iter() {
        if let Some(d) = Dir::try_from_x("^v<>", char) {
            let ix = match d {
                Dir::Left => 0,
                Dir::Right => 1,
                Dir::Down => 2,
                Dir::Up => 3,
            };
            let a = &mut blizzards[ix];
            match d {
                Dir::Up | Dir::Down => {
                    a.entry(point.x as usize)
                        .or_insert_with(|| vec![false; bb.top_right.y as usize])
                        [point.y as usize] = true;
                }
                Dir::Left | Dir::Right => {
                    a.entry(point.y as usize)
                        .or_insert_with(|| vec![false; bb.top_right.x as usize])
                        [point.x as usize] = true;
                }
            }
        }
    }
    let world = World {
        start,
        target,
        blizzards,
    };
    let there = world.shortest(0, world.start, world.target);
    let back = world.shortest(there, world.target, world.start);
    let there_again = world.shortest(back, world.start, world.target);

    (there, there_again)
}
